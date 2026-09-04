//! Agent-owned runtime custody for one production durable-registry semantic store.
//!
//! C03e-JD materializes only the C03e-JC-selected side-effect-free ownership adaptation. One
//! already-produced [`DurableRegistryEtcdStore`] is consumed exactly once and retained privately by
//! Agent runtime custody for a later separately gated operation-specific use site.
//!
//! C03e-JF adds only the C03e-JE-selected operation-specific production peer-identity lookup. It
//! resolves one explicitly supplied logical [`DeviceId`] through the privately held durable registry's
//! authoritative current same-device transport binding and returns one [`PeerConnectivityIdentity`].
//!
//! C03e-KA adds only the C03e-JZ-selected dormant production durable capability-authority custody.
//! One existing durable-registry runtime custody is shared through one outer `Arc<Mutex<_>>` beside
//! the fixed fail-closed [`ProductionRemoteCapabilityDenyAllPolicy`]. No authorization method or
//! durable-bridge invocation is materialized here.
//!
//! This module does not load credentials, bootstrap a provider, expose a generic store/executor
//! extraction seam, register global state, publish readiness, wire startup, create a background task,
//! activate networking, deploy, mutate registry state, or mutate production state.

use std::sync::Arc;

use prw_connectivity::PeerConnectivityIdentity;
use prw_core::DeviceId;
use prw_policy::ProductionRemoteCapabilityDenyAllPolicy;
use prw_registry::durable_registry_etcd_store::{
    DurableRegistryEtcdStore, DurableRegistryEtcdStoreError,
};
use tokio::sync::Mutex;

/// Agent-owned dormant runtime custody of one production durable-registry semantic store.
///
/// The contained store remains private and non-cloneable. Later separately gated checkpoints may add
/// only operation-specific methods needed by exact runtime use cases; this owner intentionally exposes
/// no generic store getter or extraction path.
pub struct ProductionDurableRegistryRuntimeCustody {
    store: DurableRegistryEtcdStore,
}

impl ProductionDurableRegistryRuntimeCustody {
    /// Consumes one already-composed durable-registry semantic store into Agent runtime custody.
    ///
    /// This method performs ownership adaptation only. It does not read credentials, perform network
    /// I/O, issue etcd operations, validate registry state, publish readiness, register global state,
    /// or create a runtime task.
    #[must_use]
    pub const fn from_store(store: DurableRegistryEtcdStore) -> Self {
        Self { store }
    }

    /// Resolves one production peer identity from the durable registry's current same-device binding.
    ///
    /// The caller supplies only the logical device identity. The current transport identity is read
    /// exactly once from the privately held semantic store and cannot be supplied or substituted by
    /// the caller. Construction of the returned peer occurs only after that authoritative read
    /// succeeds.
    ///
    /// # Errors
    ///
    /// Propagates the existing provider-neutral durable-registry semantic/read authority failure
    /// unchanged. No fallback peer, retry, stale cache, alternate device or alternate transport source
    /// is used.
    pub(crate) async fn peer_connectivity_identity(
        &mut self,
        device_id: DeviceId,
    ) -> Result<PeerConnectivityIdentity, DurableRegistryEtcdStoreError> {
        let current_transport = self.store.current_transport_identity(&device_id).await?;
        Ok(PeerConnectivityIdentity::new(device_id, current_transport))
    }
}

/// Dormant Agent custody for the production durable capability-authority prerequisites.
///
/// The durable-registry runtime custody remains private behind one shared asynchronous mutex. The
/// policy is the concrete production deny-all baseline and therefore cannot carry positive grants.
/// This owner performs no authorization itself and exposes no inner-store access seam.
pub struct ProductionDurableCapabilityAuthority {
    registry_custody: Arc<Mutex<ProductionDurableRegistryRuntimeCustody>>,
    policy: ProductionRemoteCapabilityDenyAllPolicy,
}

impl ProductionDurableCapabilityAuthority {
    /// Consumes one exact durable-registry custody into dormant production capability-authority
    /// ownership.
    ///
    /// Construction is synchronous and side-effect-free: it performs no lock acquisition, provider
    /// I/O, registry operation, policy lookup, authorization, task spawn or runtime activation.
    #[must_use]
    pub fn from_registry_custody(
        registry_custody: ProductionDurableRegistryRuntimeCustody,
    ) -> Self {
        Self {
            registry_custody: Arc::new(Mutex::new(registry_custody)),
            policy: ProductionRemoteCapabilityDenyAllPolicy,
        }
    }
}

#[cfg(test)]
mod tests {
    use prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStore;

    use super::{ProductionDurableCapabilityAuthority, ProductionDurableRegistryRuntimeCustody};

    fn assert_constructor_signature(
        constructor: fn(DurableRegistryEtcdStore) -> ProductionDurableRegistryRuntimeCustody,
    ) {
        let _ = constructor;
    }

    fn assert_capability_authority_constructor_signature(
        constructor: fn(
            ProductionDurableRegistryRuntimeCustody,
        ) -> ProductionDurableCapabilityAuthority,
    ) {
        let _ = constructor;
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn runtime_custody_constructor_consumes_exact_store_by_value() {
        assert_constructor_signature(ProductionDurableRegistryRuntimeCustody::from_store);
    }

    #[test]
    fn capability_authority_constructor_consumes_exact_registry_custody_by_value() {
        assert_capability_authority_constructor_signature(
            ProductionDurableCapabilityAuthority::from_registry_custody,
        );
    }

    #[test]
    fn capability_authority_custody_is_send_sync() {
        assert_send_sync::<ProductionDurableCapabilityAuthority>();
    }
}
