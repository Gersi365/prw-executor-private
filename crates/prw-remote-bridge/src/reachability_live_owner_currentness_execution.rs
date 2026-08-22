//! Phase 152 C02f-BF authoritative currentness provider execution composition.
//!
//! C02f-BE selected the exact read-only currentness sequence from one semantic live-owner grant
//! through the already-materialized C02f-AD linearizable provider currentness primitive into the
//! provider-neutral semantic currentness result. This module materializes only that sequence.
//!
//! The caller supplies an already-created mutable `ReachabilityLiveOwnerEtcdStore`; this module does
//! not select endpoints, construct an etcd client, configure TLS/auth/RBAC, create a runtime/task,
//! allocate or reissue fences, generate attempt IDs, compose acquisition/release, activate R1-R4
//! effects, deploy, or merge anything. Provider I/O occurs only when the returned future is polled.

#![allow(clippy::manual_async_fn)]

use std::{future::Future, num::NonZeroU128};

use prw_control_plane::{
    reachability_live_owner_etcd::ReachabilityLiveOwnerEtcdStore,
    reachability_live_owner_txn::LiveOwnerProviderCurrentness,
};

use crate::reachability_live_owner::{
    ReachabilityLiveOwnerAuthorityError, ReachabilityLiveOwnerCurrentness,
    ReachabilityLiveOwnerGrant,
};

/// Executes the selected authoritative provider currentness check for one exact semantic grant.
///
/// The exact peer and fence are derived only from `grant`. C02f-AD performs exactly one default-
/// linearizable exact-key observation and C02f-AB exact peer/fence classification through
/// `ReachabilityLiveOwnerEtcdStore::currentness`. No extra read, cache fallback, Watch inference,
/// reconciliation, mutation or retry is introduced here.
///
/// A successful `Current` result is a point-in-time authority proof only. It does not replace the
/// separately required R1-R4 effect-side stale-fence rejection.
///
/// The explicit `Future + Send` return type preserves the C02f-Y production async contract without
/// transferring runtime ownership into this module.
///
/// # Errors
///
/// Returns [`ReachabilityLiveOwnerAuthorityError::FenceExhausted`] only if the semantic fence cannot
/// be represented as the required non-zero provider fence. Every C02f-AD provider/classification
/// error maps fail-closed to [`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`].
pub fn execute_live_owner_currentness<'a>(
    store: &'a mut ReachabilityLiveOwnerEtcdStore,
    grant: &'a ReachabilityLiveOwnerGrant,
) -> impl Future<Output = Result<ReachabilityLiveOwnerCurrentness, ReachabilityLiveOwnerAuthorityError>>
+ Send
+ 'a {
    async move {
        let raw_fence = NonZeroU128::new(grant.fence().get())
            .ok_or(ReachabilityLiveOwnerAuthorityError::FenceExhausted)?;

        let provider_currentness = store
            .currentness(grant.peer(), raw_fence)
            .await
            .map_err(map_provider_failure)?;

        Ok(map_provider_currentness(provider_currentness))
    }
}

const fn map_provider_currentness(
    currentness: LiveOwnerProviderCurrentness,
) -> ReachabilityLiveOwnerCurrentness {
    match currentness {
        LiveOwnerProviderCurrentness::Current => ReachabilityLiveOwnerCurrentness::Current,
        LiveOwnerProviderCurrentness::Stale => ReachabilityLiveOwnerCurrentness::Stale,
    }
}

fn map_provider_failure<T>(_error: T) -> ReachabilityLiveOwnerAuthorityError {
    ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_provider_current_maps_one_to_one() {
        assert_eq!(
            map_provider_currentness(LiveOwnerProviderCurrentness::Current),
            ReachabilityLiveOwnerCurrentness::Current
        );
    }

    #[test]
    fn exact_provider_stale_maps_one_to_one() {
        assert_eq!(
            map_provider_currentness(LiveOwnerProviderCurrentness::Stale),
            ReachabilityLiveOwnerCurrentness::Stale
        );
    }

    #[test]
    fn provider_failure_maps_fail_closed() {
        assert_eq!(
            map_provider_failure(()),
            ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous
        );
    }
}
