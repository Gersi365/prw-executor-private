//! Agent-owned runtime custody for one production reachability bootstrap composition.
//!
//! C03e-HY materializes only the C03e-HX-selected side-effect-free ownership adaptation. One
//! already-produced [`ProductionReachabilityBootstrapComposition`] is consumed exactly once; its
//! live composed authority is adapted into the existing Agent runtime-authority owner while its
//! recovered durable production owner remains retained beside that authority owner.
//!
//! This module reads no credentials, performs no provider or durable I/O, creates no runtime/task,
//! binds no endpoint, publishes no readiness, activates no candidate publication or traversal,
//! dials no peer, mutates no startup/shutdown path, and exposes no generic extraction seam.

use crate::{
    production_reachability_bootstrap::ProductionReachabilityBootstrapComposition,
    production_reachability_owner_composition::ProductionReachabilityEtcdOwnerCustody,
    reachability_authority_admission::ReachabilityAuthorityRuntimeOwner,
};

/// Agent-owned joint custody of the production live authority and recovered durable owner.
///
/// Both semantic values remain private and non-cloneable. A later separately gated endpoint/process
/// transaction may consume this owner without first exposing or dropping either component.
pub struct ProductionReachabilityRuntimeCustody {
    authority_owner: ReachabilityAuthorityRuntimeOwner,
    owner_custody: ProductionReachabilityEtcdOwnerCustody,
}

impl ProductionReachabilityRuntimeCustody {
    /// Consumes one production bootstrap composition into joint runtime custody.
    ///
    /// This method performs ownership adaptation only. The bootstrap composition has already
    /// completed all credential/provider/durable work before entry. Its two semantic parts are
    /// immediately re-owned here; neither part is returned or registered globally.
    #[must_use]
    pub fn from_bootstrap_composition(
        composition: ProductionReachabilityBootstrapComposition,
    ) -> Self {
        let (live_owner_authority, owner_custody) = composition.into_parts();
        let authority_owner =
            ReachabilityAuthorityRuntimeOwner::from_composed_authority(live_owner_authority);

        Self {
            authority_owner,
            owner_custody,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProductionReachabilityRuntimeCustody;
    use crate::production_reachability_bootstrap::ProductionReachabilityBootstrapComposition;

    fn assert_constructor_signature(
        constructor: fn(
            ProductionReachabilityBootstrapComposition,
        ) -> ProductionReachabilityRuntimeCustody,
    ) {
        let _ = constructor;
    }

    #[test]
    fn runtime_custody_constructor_consumes_exact_bootstrap_composition_shape() {
        assert_constructor_signature(ProductionReachabilityRuntimeCustody::from_bootstrap_composition);
    }
}
