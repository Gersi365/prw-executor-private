//! Agent-owned custody for one recovered production reachability owner.
//!
//! C03e-GG materializes only the C03e-GF-selected fail-closed recovery/custody boundary around
//! the existing `ProductionReachabilityOwner`. Construction performs one authoritative durable
//! recovery for one exact peer lifecycle. The retained owner is not cloneable or exposed directly,
//! and this module performs no candidate execution, requester mutation, response I/O, traversal
//! activation, listener/readiness publication, dialing, deployment, or process recovery.

use prw_connectivity::PeerConnectivityIdentity;
use prw_remote_bridge::reachability_owner::{
    CandidatePublicationFreshnessTokenSource, ProductionReachabilityOwner,
    ReachabilityDurableStore, ReachabilityOwnerError,
};

/// Agent-owned exclusive custody of one recovered production reachability owner.
///
/// The wrapper intentionally has no `Clone` implementation. Store and verifier token-source
/// ownership remain encapsulated inside the existing production owner. Later separately gated
/// composition may operate through the bounded closure seam without obtaining a raw owner handle.
pub(crate) struct ProductionReachabilityOwnerCustody<S, T> {
    owner: ProductionReachabilityOwner<S, T>,
}

impl<S, T> ProductionReachabilityOwnerCustody<S, T>
where
    S: ReachabilityDurableStore,
    T: CandidatePublicationFreshnessTokenSource,
{
    /// Recovers and retains exactly one production owner for `peer`.
    ///
    /// This delegates once to the existing authoritative durable recovery law. Missing,
    /// ambiguous, mismatched, recovery-required, or retired durable state is preserved exactly as
    /// classified by `ProductionReachabilityOwner::recover`; no default/rebaseline owner exists.
    ///
    /// # Errors
    ///
    /// Returns the exact existing [`ReachabilityOwnerError`] produced by authoritative recovery.
    pub(crate) fn recover(
        store: S,
        token_source: T,
        peer: &PeerConnectivityIdentity,
    ) -> Result<Self, ReachabilityOwnerError> {
        let owner = ProductionReachabilityOwner::recover(store, token_source, peer)?;
        Ok(Self { owner })
    }

    /// Runs one bounded synchronous operation with exclusive mutable access to the retained owner.
    ///
    /// The higher-ranked closure cannot return a reference tied to the owner borrow, so mutable
    /// owner custody cannot escape this lexical call. This seam performs no operation by itself;
    /// the caller-provided operation remains separately gated.
    pub(crate) fn with_owner_mut<R>(
        &mut self,
        operation: impl for<'owner> FnOnce(&'owner mut ProductionReachabilityOwner<S, T>) -> R,
    ) -> R {
        operation(&mut self.owner)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use prw_connectivity::{PeerConnectivityIdentity, PeerConnectivityPlan, TransportIdentity};
    use prw_core::DeviceId;
    use prw_remote_bridge::{
        candidate_publication_freshness::{
            CandidatePublicationFreshnessRecord, CandidatePublicationFreshnessToken,
        },
        reachability_owner::{
            CandidatePublicationFreshnessTokenSource, FreshnessTokenSourceError,
            ReachabilityDurableSnapshot, ReachabilityDurableStore, ReachabilityOwnerError,
            ReachabilityOwnerMode, ReachabilityPersistenceCommit, ReachabilityPersistenceError,
        },
    };

    use super::ProductionReachabilityOwnerCustody;

    struct CountingStore {
        expected_peer: PeerConnectivityIdentity,
        snapshot: Option<ReachabilityDurableSnapshot>,
        load_error: Option<ReachabilityPersistenceError>,
        load_calls: Arc<AtomicUsize>,
        commit_calls: Arc<AtomicUsize>,
    }

    impl ReachabilityDurableStore for CountingStore {
        fn load_current(
            &mut self,
            peer: &PeerConnectivityIdentity,
        ) -> Result<Option<ReachabilityDurableSnapshot>, ReachabilityPersistenceError> {
            assert_eq!(peer, &self.expected_peer);
            self.load_calls.fetch_add(1, Ordering::SeqCst);
            match self.load_error {
                Some(error) => Err(error),
                None => Ok(self.snapshot.clone()),
            }
        }

        fn compare_and_commit(
            &mut self,
            _expected_current: CandidatePublicationFreshnessToken,
            _replacement: &ReachabilityDurableSnapshot,
        ) -> Result<ReachabilityPersistenceCommit, ReachabilityPersistenceError> {
            self.commit_calls.fetch_add(1, Ordering::SeqCst);
            Ok(ReachabilityPersistenceCommit::Committed)
        }
    }

    struct CountingTokenSource {
        issue_calls: Arc<AtomicUsize>,
    }

    impl CandidatePublicationFreshnessTokenSource for CountingTokenSource {
        fn issue_token(
            &mut self,
        ) -> Result<CandidatePublicationFreshnessToken, FreshnessTokenSourceError> {
            self.issue_calls.fetch_add(1, Ordering::SeqCst);
            Err(FreshnessTokenSourceError::Unavailable)
        }
    }

    fn test_peer() -> PeerConnectivityIdentity {
        PeerConnectivityIdentity::new(
            DeviceId::new("gg-production-reachability-peer").expect("valid device id"),
            TransportIdentity::new([0x31; 32]).expect("non-zero transport identity"),
        )
    }

    fn established_snapshot(peer: &PeerConnectivityIdentity) -> ReachabilityDurableSnapshot {
        let plan =
            PeerConnectivityPlan::new(peer.clone(), Vec::new()).expect("empty plan is valid");
        let freshness = CandidatePublicationFreshnessRecord::established(
            peer.clone(),
            CandidatePublicationFreshnessToken::new([0x47; 32]).expect("non-zero freshness token"),
        );
        ReachabilityDurableSnapshot::new(plan, freshness).expect("peer-consistent snapshot")
    }

    fn counts() -> (Arc<AtomicUsize>, Arc<AtomicUsize>, Arc<AtomicUsize>) {
        (
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
        )
    }

    #[test]
    fn successful_custody_construction_recovers_once_without_semantic_execution() {
        let peer = test_peer();
        let (load_calls, commit_calls, issue_calls) = counts();
        let store = CountingStore {
            expected_peer: peer.clone(),
            snapshot: Some(established_snapshot(&peer)),
            load_error: None,
            load_calls: Arc::clone(&load_calls),
            commit_calls: Arc::clone(&commit_calls),
        };
        let token_source = CountingTokenSource {
            issue_calls: Arc::clone(&issue_calls),
        };

        let mut custody = ProductionReachabilityOwnerCustody::recover(store, token_source, &peer)
            .unwrap_or_else(|error| panic!("authoritative recovery should succeed: {error}"));

        assert_eq!(load_calls.load(Ordering::SeqCst), 1);
        assert_eq!(commit_calls.load(Ordering::SeqCst), 0);
        assert_eq!(issue_calls.load(Ordering::SeqCst), 0);
        assert_eq!(
            custody.with_owner_mut(|owner| owner.mode()),
            ReachabilityOwnerMode::Current
        );
        assert_eq!(
            custody.with_owner_mut(|owner| owner.plan().peer().clone()),
            peer
        );
        assert_eq!(load_calls.load(Ordering::SeqCst), 1);
        assert_eq!(commit_calls.load(Ordering::SeqCst), 0);
        assert_eq!(issue_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn missing_durable_state_is_preserved_exactly() {
        let peer = test_peer();
        let (load_calls, commit_calls, issue_calls) = counts();
        let result = ProductionReachabilityOwnerCustody::recover(
            CountingStore {
                expected_peer: peer.clone(),
                snapshot: None,
                load_error: None,
                load_calls: Arc::clone(&load_calls),
                commit_calls: Arc::clone(&commit_calls),
            },
            CountingTokenSource {
                issue_calls: Arc::clone(&issue_calls),
            },
            &peer,
        );

        assert!(matches!(
            result,
            Err(ReachabilityOwnerError::DurableStateMissing)
        ));
        assert_eq!(load_calls.load(Ordering::SeqCst), 1);
        assert_eq!(commit_calls.load(Ordering::SeqCst), 0);
        assert_eq!(issue_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn ambiguous_durable_load_is_preserved_exactly() {
        let peer = test_peer();
        let (load_calls, commit_calls, issue_calls) = counts();
        let result = ProductionReachabilityOwnerCustody::recover(
            CountingStore {
                expected_peer: peer.clone(),
                snapshot: None,
                load_error: Some(ReachabilityPersistenceError::UnavailableOrAmbiguous),
                load_calls: Arc::clone(&load_calls),
                commit_calls: Arc::clone(&commit_calls),
            },
            CountingTokenSource {
                issue_calls: Arc::clone(&issue_calls),
            },
            &peer,
        );

        assert!(matches!(
            result,
            Err(ReachabilityOwnerError::Persistence(
                ReachabilityPersistenceError::UnavailableOrAmbiguous
            ))
        ));
        assert_eq!(load_calls.load(Ordering::SeqCst), 1);
        assert_eq!(commit_calls.load(Ordering::SeqCst), 0);
        assert_eq!(issue_calls.load(Ordering::SeqCst), 0);
    }
}
