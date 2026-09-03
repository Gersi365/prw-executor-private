//! Agent-owned runtime custody for one production durable-registry semantic store.
//!
//! C03e-JD materializes only the C03e-JC-selected side-effect-free ownership adaptation. One
//! already-produced [`DurableRegistryEtcdStore`] is consumed exactly once and retained privately by
//! Agent runtime custody for a later separately gated operation-specific use site.
//!
//! This module does not load credentials, bootstrap a provider, issue registry semantic operations,
//! expose a generic store/executor extraction seam, register global state, publish readiness, wire
//! startup, create a background task, activate networking, deploy, or mutate production state.

use prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStore;

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
    pub fn from_store(store: DurableRegistryEtcdStore) -> Self {
        Self { store }
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
