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
//! This module does not load credentials, bootstrap a provider, expose a generic store/executor
//! extraction seam, register global state, publish readiness, wire startup, create a background task,
//! activate networking, deploy, mutate registry state, or mutate production state.

use prw_connectivity::PeerConnectivityIdentity;
use prw_core::DeviceId;
use prw_registry::durable_registry_etcd_store::{
    DurableRegistryEtcdStore, DurableRegistryEtcdStoreError,
};

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

#[cfg(test)]
mod tests {
    use prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStore;

    use super::ProductionDurableRegistryRuntimeCustody;

    fn assert_constructor_signature(
        constructor: fn(DurableRegistryEtcdStore) -> ProductionDurableRegistryRuntimeCustody,
    ) {
        let _ = constructor;
    }

    #[test]
    fn runtime_custody_constructor_consumes_exact_store_by_value() {
        assert_constructor_signature(ProductionDurableRegistryRuntimeCustody::from_store);
    }
}
