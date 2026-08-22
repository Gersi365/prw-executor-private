//! Phase 152 C02f-BQ reconciled replacement-acquisition provider execution composition.
//!
//! C02f-AW selected the exact evidence-continuity sequence from one retained C02f-AS acquisition
//! handoff through the already-materialized C02f-AE bounded provider reconciliation into the
//! validated C02f-AV semantic mapper. This module materializes only that sequence.
//!
//! The caller supplies an already-created mutable `ReachabilityLiveOwnerEtcdStore` and one exact
//! retained `FenceSequenceLiveOwnerAcquisitionHandoff`; this module does not select endpoints,
//! construct an etcd client, configure TLS/auth/RBAC, create a runtime/task, allocate fences or
//! attempt IDs, prepare a handoff, compose first-owner/currentness/release/full authority behavior,
//! activate R1-R4 effects, deploy, or merge anything. Provider I/O occurs only when the returned
//! future is polled.

#![allow(clippy::manual_async_fn)]

use std::future::Future;

use prw_control_plane::{
    reachability_acquisition_evidence::FenceSequenceLiveOwnerAcquisitionHandoff,
    reachability_live_owner_etcd::ReachabilityLiveOwnerEtcdStore,
};

use crate::{
    reachability_live_owner::{
        ReachabilityLiveOwnerAcquisition, ReachabilityLiveOwnerAuthorityError,
    },
    reachability_live_owner_reconciled_acquisition::map_reconciled_live_owner_acquisition,
};

/// Executes the selected reconciled replacement-acquisition sequence for one exact retained handoff.
///
/// `before` is cloned only from `handoff.observation()` and `successor` only from the exact retained
/// transaction successor. Those values are passed directly to C02f-AE
/// `execute_acquisition_with_reconciliation`; its exact terminal provider evidence is then passed
/// with the same original handoff directly to C02f-AV `map_reconciled_live_owner_acquisition`.
///
/// This composition performs no replanning, direct `execute`, extra provider re-observation,
/// outer retry/reissue loop, evidence reconstruction or semantic result manufacture.
///
/// The explicit `Future + Send` return type preserves the C02f-Y production async contract without
/// transferring runtime ownership into this module.
///
/// # Errors
///
/// Every C02f-AE provider/reconciliation failure maps fail-closed to
/// [`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`]. C02f-AV semantic mapping errors,
/// including its reserved `FenceExhausted` conversion failure, are returned unchanged.
pub fn execute_reconciled_live_owner_acquisition<'a>(
    store: &'a mut ReachabilityLiveOwnerEtcdStore,
    handoff: &'a FenceSequenceLiveOwnerAcquisitionHandoff,
) -> impl Future<Output = Result<ReachabilityLiveOwnerAcquisition, ReachabilityLiveOwnerAuthorityError>>
+ Send
+ 'a {
    async move {
        let before = handoff.observation().clone();
        let successor = handoff.acquisition().transaction().successor().clone();

        let resolved = store
            .execute_acquisition_with_reconciliation(before, successor)
            .await
            .map_err(map_provider_failure)?;

        map_reconciled_live_owner_acquisition(handoff, &resolved)
    }
}

fn map_provider_failure<T>(_error: T) -> ReachabilityLiveOwnerAuthorityError {
    ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_failure_maps_fail_closed() {
        assert_eq!(
            map_provider_failure(()),
            ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous
        );
    }
}
