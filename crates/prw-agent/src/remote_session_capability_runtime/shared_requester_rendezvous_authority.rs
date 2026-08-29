//! Shared Agent-owned synchronization for requester/rendezvous authority.
//!
//! C03e-FP materializes the C03e-FO-selected cloneable Tokio-async-mutex wrapper around one
//! existing process-local requester/rendezvous runtime owner. The wrapper serializes the exact
//! requester DR registration critical section, nests the existing shared-current registry/policy
//! read only after requester-authority lock acquisition, and releases requester-authority custody
//! before requester acknowledgement framing or response I/O. C03e-FY adds only the FX-selected
//! candidate-publication grant/commit/cleanup custody split: candidate admission occurs before the
//! requester lock, one grant is selected under the lock, the lock is released before durable
//! reachability commit, and exact record cleanup reacquires the lock only after definite commit
//! success. It exposes no raw provider, mutex, or guard and activates no listener/network runtime.

use std::sync::Arc;

use prw_core::{DeviceId, SessionId};
use prw_policy::PolicyEvaluator;
use prw_registry::WorkspaceDeviceRegistry;
use prw_remote_bridge::{
    candidate_publication_control_frame::CandidatePublicationControlFrame,
    candidate_publication_execution::CandidatePublicationExecutionError,
    candidate_reachability::publish_current_candidates,
    prwc_connection_authentication::AuthenticatedPrwcConnection,
    reachability_owner::{
        CandidatePublicationFreshnessTokenSource, ProductionReachabilityOwner,
        ReachabilityCommitOutcome, ReachabilityDurableStore,
    },
    requester_rendezvous_authority::{
        AuthorizedRequesterRendezvous, RequesterRendezvousAuthorityError,
    },
    requester_rendezvous_in_memory_provider::RequesterRendezvousLifecycleError,
};
use tokio::sync::Mutex;

use super::SharedCurrentCapabilityAuthority;
use crate::{
    candidate_publication_requester_rendezvous_runtime::CandidatePublicationRequesterRendezvousRuntimeOwner,
    candidate_publication_requester_rendezvous_start_intent::{
        RequesterRendezvousStartIntent,
        composition::{
            RequesterRendezvousStartCompositionError,
            validate_authorize_and_register_requester_rendezvous_start,
        },
        policy_source::RequesterRendezvousStartPolicySource,
    },
};

/// Non-authorizing exact lifecycle identity preserved from one selected requester grant.
///
/// Possession of this value cannot authorize candidate publication, requester registration,
/// reconnect, retry, dialing, or any new session. It exists only so a definite successful durable
/// publication can retire and remove the same exact requester record afterward.
#[derive(Debug, PartialEq, Eq)]
struct RequesterRendezvousCommittedCleanupIdentity {
    requester_session_id: SessionId,
    expected_publisher_device_id: DeviceId,
}

impl RequesterRendezvousCommittedCleanupIdentity {
    fn from_grant(grant: &AuthorizedRequesterRendezvous) -> Self {
        Self {
            requester_session_id: grant.requester_session().session_id().clone(),
            expected_publisher_device_id: grant.expected_publisher_device_id().clone(),
        }
    }
}

/// Definite durable candidate-publication success plus independent post-commit cleanup disposition.
///
/// A cleanup error never rewrites the already-committed publication into
/// [`CandidatePublicationExecutionError`]. Callers can project the exact successful reachability
/// outcome for existing Accepted response framing while handling cleanup disposition separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CandidatePublicationPostCommitRequesterCleanupOutcome {
    reachability_commit: ReachabilityCommitOutcome,
    cleanup: Result<(), RequesterRendezvousLifecycleError>,
}

impl CandidatePublicationPostCommitRequesterCleanupOutcome {
    /// Returns the definite successful durable reachability commit outcome.
    #[must_use]
    pub(crate) const fn reachability_commit(self) -> ReachabilityCommitOutcome {
        self.reachability_commit
    }

    /// Returns the independent exact-record cleanup disposition.
    #[must_use]
    pub(crate) const fn cleanup_result(self) -> Result<(), RequesterRendezvousLifecycleError> {
        self.cleanup
    }

    /// Transfers both post-commit result components without flattening either classification.
    #[must_use]
    pub(crate) const fn into_parts(
        self,
    ) -> (
        ReachabilityCommitOutcome,
        Result<(), RequesterRendezvousLifecycleError>,
    ) {
        (self.reachability_commit, self.cleanup)
    }
}

/// Cloneable handle to exactly one process-local requester/rendezvous runtime owner.
///
/// Clones share only the outer [`Arc`]. The existing runtime owner and its provider state are never
/// cloned or snapshotted. Operation callers cannot obtain the raw mutex, guard, runtime owner, or
/// provider.
pub struct SharedRequesterRendezvousAuthority {
    runtime_owner: Arc<Mutex<CandidatePublicationRequesterRendezvousRuntimeOwner>>,
}

impl Clone for SharedRequesterRendezvousAuthority {
    fn clone(&self) -> Self {
        Self {
            runtime_owner: Arc::clone(&self.runtime_owner),
        }
    }
}

impl SharedRequesterRendezvousAuthority {
    /// Takes by-value custody of the exact existing requester/rendezvous runtime owner.
    ///
    /// Construction performs no registration, authorization, I/O, task creation, readiness
    /// publication, peer disposition, or provider cloning.
    #[must_use]
    pub fn new(runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner) -> Self {
        Self {
            runtime_owner: Arc::new(Mutex::new(runtime_owner)),
        }
    }

    /// Runs the exact DI -> DP -> DK -> DN requester-start composition under FO lock ordering.
    ///
    /// The requester/rendezvous mutex is acquired first. While that guard remains held, the exact
    /// existing shared-current registry/policy read is acquired and the synchronous existing DR
    /// composition runs once. The current-authority guard is released by
    /// `with_current_authority(...)`, then this method releases the requester-authority guard before
    /// returning to FB. Consequently FD/FH response framing and I/O occur after both authority
    /// guards have been released.
    ///
    /// # Errors
    ///
    /// Returns the exact existing requester/rendezvous start composition error without translation,
    /// retry, fallback, replacement registration, provider reset, or peer close.
    pub async fn validate_authorize_and_register_requester_rendezvous_start<
        P: PolicyEvaluator + Send + Sync,
        S: RequesterRendezvousStartPolicySource + Sync + ?Sized,
    >(
        &self,
        authority: &SharedCurrentCapabilityAuthority<P>,
        policy_source: &S,
        intent: RequesterRendezvousStartIntent,
    ) -> Result<(), RequesterRendezvousStartCompositionError> {
        let mut runtime_owner = self.runtime_owner.lock().await;

        authority
            .with_current_authority(|registry, _current_capability_policy| {
                validate_authorize_and_register_requester_rendezvous_start(
                    registry,
                    policy_source,
                    &mut runtime_owner,
                    intent,
                )
            })
            .await
    }

    /// Selects one current requester/rendezvous grant under bounded shared-authority custody.
    ///
    /// The requester mutex is held only for the synchronous provider authorization call and is
    /// released before this async method returns. No durable reachability work or response I/O is
    /// executed while the guard exists.
    async fn authorize_current_for_publisher(
        &self,
        publisher_device_id: &DeviceId,
    ) -> Result<AuthorizedRequesterRendezvous, RequesterRendezvousAuthorityError> {
        let mut runtime_owner = self.runtime_owner.lock().await;
        runtime_owner.authorize_current_requester_rendezvous_for_publisher(publisher_device_id)
    }

    /// Reacquires shared requester authority only to retire and remove one exact committed record.
    ///
    /// This operation is called only after definite durable reachability commit success. It holds
    /// the requester mutex across the synchronous exact `retire -> remove_retired` pair and releases
    /// it before returning. It performs no retry, rollback, response I/O, or publisher peer action.
    async fn cleanup_committed_requester_rendezvous_record(
        &self,
        identity: RequesterRendezvousCommittedCleanupIdentity,
    ) -> Result<(), RequesterRendezvousLifecycleError> {
        let mut runtime_owner = self.runtime_owner.lock().await;
        runtime_owner.cleanup_committed_requester_rendezvous_record(
            &identity.requester_session_id,
            &identity.expected_publisher_device_id,
        )
    }

    async fn commit_then_cleanup<T, E, C>(
        &self,
        identity: RequesterRendezvousCommittedCleanupIdentity,
        commit: C,
    ) -> Result<(T, Result<(), RequesterRendezvousLifecycleError>), E>
    where
        C: FnOnce() -> Result<T, E>,
    {
        let committed = commit()?;
        let cleanup = self
            .cleanup_committed_requester_rendezvous_record(identity)
            .await;
        Ok((committed, cleanup))
    }

    /// Executes one candidate-publication semantic attempt with FX-selected post-commit cleanup.
    ///
    /// Ordering is exact and fail-closed:
    ///
    /// 1. authenticated publisher/session/transport/candidate admission occurs without requester
    ///    mutex custody;
    /// 2. the requester mutex is acquired only long enough to select one exact current grant;
    /// 3. exact expected-publisher equality is checked after the grant-selection guard is released;
    /// 4. the existing durable reachability owner commits while no requester mutex guard exists;
    /// 5. only after definite commit success is requester authority reacquired for exact
    ///    `retire -> remove_retired` cleanup;
    /// 6. the committed reachability outcome and cleanup disposition are returned separately.
    ///
    /// Candidate admission, requester authorization, expected-publisher mismatch, or reachability
    /// commit failure returns the existing [`CandidatePublicationExecutionError`] and performs no
    /// cleanup. Cleanup failure after commit remains inside the successful post-commit outcome and
    /// therefore cannot become a generic wire-level Rejected result for an already-committed
    /// publication.
    ///
    /// This seam reads/writes no frame, allocates no request ID, spawns no task, starts no command
    /// loop, selects no target dialing, and activates no listener/network runtime.
    ///
    /// # Errors
    ///
    /// Returns the existing candidate-publication execution error classes only for failures before
    /// or at durable commit. Exact post-commit cleanup errors are preserved separately in the
    /// successful return value.
    pub(crate) async fn execute_authenticated_candidate_publication_with_post_commit_cleanup<S, T>(
        &self,
        connection: &AuthenticatedPrwcConnection,
        command: &CandidatePublicationControlFrame,
        registry: &WorkspaceDeviceRegistry,
        owner: &mut ProductionReachabilityOwner<S, T>,
    ) -> Result<
        CandidatePublicationPostCommitRequesterCleanupOutcome,
        CandidatePublicationExecutionError,
    >
    where
        S: ReachabilityDurableStore,
        T: CandidatePublicationFreshnessTokenSource,
    {
        let submission = command.submission();
        let publication = publish_current_candidates(
            registry,
            connection.session(),
            submission.presented_transport_identity(),
            submission.candidates().to_vec(),
        )
        .map_err(CandidatePublicationExecutionError::Candidate)?;

        let publisher_device_id = publication.peer().device_id();
        let grant = self
            .authorize_current_for_publisher(publisher_device_id)
            .await
            .map_err(CandidatePublicationExecutionError::RequesterAuthority)?;
        if grant.expected_publisher_device_id() != publisher_device_id {
            return Err(CandidatePublicationExecutionError::ExpectedPublisherMismatch);
        }

        let cleanup_identity = RequesterRendezvousCommittedCleanupIdentity::from_grant(&grant);
        let (reachability_commit, cleanup) = self
            .commit_then_cleanup(cleanup_identity, || {
                owner
                    .commit_candidate_publication(
                        registry,
                        grant.requester_session(),
                        &publication,
                        submission.presented_freshness(),
                    )
                    .map_err(CandidatePublicationExecutionError::Reachability)
            })
            .await?;

        Ok(CandidatePublicationPostCommitRequesterCleanupOutcome {
            reachability_commit,
            cleanup,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, sync::Arc};

    use prw_core::{DeviceId, SessionId};
    use prw_remote_bridge::{
        requester_rendezvous_authority::RequesterRendezvousAuthorityError,
        requester_rendezvous_in_memory_provider::{
            InMemoryRequesterRendezvousAuthorityProvider, RequesterRendezvousLifecycleError,
        },
    };
    use tokio::runtime::Builder;

    use super::{RequesterRendezvousCommittedCleanupIdentity, SharedRequesterRendezvousAuthority};
    use crate::candidate_publication_requester_rendezvous_runtime::CandidatePublicationRequesterRendezvousRuntimeOwner;

    fn block_on<F: Future>(future: F) -> F::Output {
        Builder::new_current_thread()
            .build()
            .expect("construct FY test runtime")
            .block_on(future)
    }

    fn authority_with_capacity(capacity: usize) -> SharedRequesterRendezvousAuthority {
        let provider = InMemoryRequesterRendezvousAuthorityProvider::new(capacity)
            .expect("explicit non-zero provider capacity");
        SharedRequesterRendezvousAuthority::new(
            CandidatePublicationRequesterRendezvousRuntimeOwner::new(provider),
        )
    }

    fn unknown_cleanup_identity() -> RequesterRendezvousCommittedCleanupIdentity {
        RequesterRendezvousCommittedCleanupIdentity {
            requester_session_id: SessionId::new("fy-cleanup-requester-unknown")
                .expect("valid requester session id"),
            expected_publisher_device_id: DeviceId::new("fy-cleanup-publisher-unknown")
                .expect("valid publisher device id"),
        }
    }

    fn assert_clone_send_sync<T: Clone + Send + Sync>() {}

    #[test]
    fn shared_authority_is_clone_send_sync() {
        assert_clone_send_sync::<SharedRequesterRendezvousAuthority>();
    }

    #[test]
    fn clone_shares_one_runtime_owner_allocation() {
        let authority = authority_with_capacity(2);
        let clone = authority.clone();

        assert!(Arc::ptr_eq(&authority.runtime_owner, &clone.runtime_owner));
    }

    #[test]
    fn current_grant_selection_releases_requester_mutex_before_return() {
        let authority = authority_with_capacity(1);
        let publisher_device_id =
            DeviceId::new("fy-missing-publisher").expect("valid publisher device id");

        assert_eq!(
            block_on(authority.authorize_current_for_publisher(&publisher_device_id)),
            Err(RequesterRendezvousAuthorityError::Missing)
        );
        assert!(authority.runtime_owner.try_lock().is_ok());
    }

    #[test]
    fn commit_runs_without_requester_guard_and_cleanup_failure_preserves_committed_value() {
        let authority = authority_with_capacity(1);
        let identity = unknown_cleanup_identity();

        let result = block_on(authority.commit_then_cleanup(identity, || {
            assert!(authority.runtime_owner.try_lock().is_ok());
            Ok::<u8, ()>(7)
        }));

        assert_eq!(
            result,
            Ok((7, Err(RequesterRendezvousLifecycleError::RecordUnknown)))
        );
        assert!(authority.runtime_owner.try_lock().is_ok());
    }

    #[test]
    fn commit_failure_returns_before_cleanup_phase() {
        let authority = authority_with_capacity(1);
        let identity = unknown_cleanup_identity();

        let result = block_on(authority.commit_then_cleanup(identity, || {
            assert!(authority.runtime_owner.try_lock().is_ok());
            Err::<u8, _>("commit failed")
        }));

        assert_eq!(result, Err("commit failed"));
        assert!(authority.runtime_owner.try_lock().is_ok());
    }
}
