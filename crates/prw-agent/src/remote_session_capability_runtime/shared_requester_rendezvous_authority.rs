//! Shared Agent-owned synchronization for requester/rendezvous authority.
//!
//! C03e-FP materializes only the C03e-FO-selected cloneable Tokio-async-mutex wrapper around one
//! existing process-local requester/rendezvous runtime owner. The wrapper serializes the exact
//! requester DR registration critical section, nests the existing shared-current registry/policy
//! read only after requester-authority lock acquisition, and releases requester-authority custody
//! before requester acknowledgement framing or response I/O. It exposes no raw provider, mutex, or
//! guard and does not spawn tasks, activate persistent workers, close peers, select reachability,
//! dial targets, publish readiness, deploy, restart/recover, or merge.

use std::sync::Arc;

use prw_policy::PolicyEvaluator;
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use prw_remote_bridge::requester_rendezvous_in_memory_provider::InMemoryRequesterRendezvousAuthorityProvider;

    use super::SharedRequesterRendezvousAuthority;
    use crate::candidate_publication_requester_rendezvous_runtime::CandidatePublicationRequesterRendezvousRuntimeOwner;

    fn assert_clone_send_sync<T: Clone + Send + Sync>() {}

    #[test]
    fn shared_authority_is_clone_send_sync() {
        assert_clone_send_sync::<SharedRequesterRendezvousAuthority>();
    }

    #[test]
    fn clone_shares_one_runtime_owner_allocation() {
        let provider = InMemoryRequesterRendezvousAuthorityProvider::new(2)
            .expect("explicit non-zero provider capacity");
        let runtime_owner = CandidatePublicationRequesterRendezvousRuntimeOwner::new(provider);
        let authority = SharedRequesterRendezvousAuthority::new(runtime_owner);
        let clone = authority.clone();

        assert!(Arc::ptr_eq(&authority.runtime_owner, &clone.runtime_owner));
    }
}
