//! Phase 152 C02f-CI Agent-owned reachability authority admission seam.
//!
//! C02f-CH preserves the existing local Agent readiness meaning and requires successful
//! reachability authority construction before future authority-dependent remote admission. This
//! module materializes only that source-level admission token and callable bootstrap/admission
//! seam. It does not wire the operation into `main.rs`, runtime readiness, remote transport, or
//! any background/retry lifecycle.

use prw_remote_bridge::reachability_live_owner_async::ReachabilityLiveOwnerComposedAsyncAuthority;

use crate::reachability_authority_custody_bootstrap::{
    ReachabilityAuthorityCustodyBootstrapError,
    bootstrap_reachability_live_owner_authority_from_systemd_credentials,
};

/// Opaque Agent-owned proof that reachability authority bootstrap completed successfully.
pub struct ReachabilityLiveOwnerAuthorityAdmission {
    authority: ReachabilityLiveOwnerComposedAsyncAuthority,
}

impl ReachabilityLiveOwnerAuthorityAdmission {
    const fn from_authority(authority: ReachabilityLiveOwnerComposedAsyncAuthority) -> Self {
        Self { authority }
    }

    /// Returns the admitted bridge-owned composed authority by immutable reference.
    #[must_use]
    pub const fn authority(&self) -> &ReachabilityLiveOwnerComposedAsyncAuthority {
        &self.authority
    }
}

/// Bootstraps and admits the reachability live-owner authority through the existing C02f-CG seam.
///
/// Calling this function performs the fixed C02f-CE credential reads and subsequent provider
/// network I/O already owned by C02f-CG/CF. The existing bounded C02f-CG error is propagated
/// unchanged. Only successful authority construction can produce the opaque admission token.
///
/// C02f-CI does not invoke this function from the Agent binary or runtime and does not add retry,
/// background work, remote transport readiness, recovery, PRWF initialization, or R1-R4 effects.
///
/// # Errors
///
/// Returns the existing [`ReachabilityAuthorityCustodyBootstrapError`] when fixed credential
/// custody or provider bootstrap fails. Failure never produces an admission token.
pub async fn bootstrap_and_admit_reachability_live_owner_authority_from_systemd_credentials()
-> Result<ReachabilityLiveOwnerAuthorityAdmission, ReachabilityAuthorityCustodyBootstrapError> {
    bootstrap_reachability_live_owner_authority_from_systemd_credentials()
        .await
        .map(ReachabilityLiveOwnerAuthorityAdmission::from_authority)
}

#[cfg(test)]
mod tests {
    use std::future::Future;

    use super::{
        ReachabilityLiveOwnerAuthorityAdmission,
        bootstrap_and_admit_reachability_live_owner_authority_from_systemd_credentials,
    };
    use crate::reachability_authority_custody_bootstrap::ReachabilityAuthorityCustodyBootstrapError;

    fn assert_admission_signature<F, Fut>(_bootstrap: F)
    where
        F: FnOnce() -> Fut,
        Fut: Future<
            Output = Result<
                ReachabilityLiveOwnerAuthorityAdmission,
                ReachabilityAuthorityCustodyBootstrapError,
            >,
        >,
    {
    }

    #[test]
    fn admission_bootstrap_has_exact_no_argument_to_admission_shape() {
        assert_admission_signature(
            bootstrap_and_admit_reachability_live_owner_authority_from_systemd_credentials,
        );
    }
}
