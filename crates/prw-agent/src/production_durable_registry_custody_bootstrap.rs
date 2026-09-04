//! Agent-owned production durable-registry systemd custody composition seam.
//!
//! C03e-JB materializes only the C03e-JA-selected join between the existing fixed production
//! durable-registry systemd credential loader, the existing control-plane provider bootstrap, and
//! the existing semantic durable-registry store. The facade accepts no endpoint, trust,
//! certificate, private-key, credential-directory, raw provider-client, request-id, or IP input.
//!
//! Calling the async facade reads the existing fixed registry service credentials and, only after
//! successful custody validation, performs the existing one-shot provider bootstrap before wrapping
//! the bounded executor in the existing semantic store. It performs no registry semantic operation,
//! creates no retry/fallback/runtime task, wires no startup/readiness callsite, deploys nothing, and
//! creates or mutates no production registry record.
//!
//! C03e-KE adds only the C03e-KD-selected production durable capability-authority population
//! composition. It performs the existing durable-registry bootstrap exactly once and then applies the
//! existing infallible runtime-custody and deny-all capability-authority ownership adaptations. It
//! performs no registry semantic read, mutex acquisition, authorization, policy evaluation, runtime
//! activation or executable wiring.

use std::fmt;

use prw_control_plane::durable_registry_etcd_bootstrap::{
    DurableRegistryProductionEtcdBootstrapError, bootstrap_durable_registry_production_executor,
};
use prw_reachability_custody::durable_registry_custody::{
    DurableRegistryCustodyError,
    load_durable_registry_production_etcd_bootstrap_config_from_systemd_credentials,
};
use prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStore;

use crate::production_durable_registry_runtime_custody::{
    ProductionDurableCapabilityAuthority, ProductionDurableRegistryRuntimeCustody,
};

/// Bounded Agent-level failure while joining durable-registry custody to provider composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductionDurableRegistryCustodyBootstrapError {
    /// The fixed production registry service-credential set failed custody validation.
    Custody(DurableRegistryCustodyError),
    /// The validated opaque registry config reached control-plane but provider bootstrap failed.
    ProviderBootstrap(DurableRegistryProductionEtcdBootstrapError),
}

impl fmt::Display for ProductionDurableRegistryCustodyBootstrapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custody(_) => {
                formatter.write_str("production durable registry custody bootstrap failed")
            }
            Self::ProviderBootstrap(_) => {
                formatter.write_str("production durable registry provider bootstrap failed")
            }
        }
    }
}

impl std::error::Error for ProductionDurableRegistryCustodyBootstrapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Custody(error) => Some(error),
            Self::ProviderBootstrap(error) => Some(error),
        }
    }
}

impl From<DurableRegistryCustodyError> for ProductionDurableRegistryCustodyBootstrapError {
    fn from(error: DurableRegistryCustodyError) -> Self {
        Self::Custody(error)
    }
}

impl From<DurableRegistryProductionEtcdBootstrapError>
    for ProductionDurableRegistryCustodyBootstrapError
{
    fn from(error: DurableRegistryProductionEtcdBootstrapError) -> Self {
        Self::ProviderBootstrap(error)
    }
}

/// Loads fixed production registry credentials, bootstraps the provider, and returns the semantic
/// durable-registry store without performing any registry operation.
///
/// Custody is attempted exactly once through the existing fixed systemd loader. Only its validated
/// opaque [`prw_control_plane::durable_registry_etcd_bootstrap::DurableRegistryProductionEtcdBootstrapConfig`]
/// is moved into the existing control-plane provider bootstrap. The returned bounded executor is
/// moved directly into [`DurableRegistryEtcdStore::new`]. No raw provider client, endpoint, trust,
/// certificate, private-key or credential-directory value enters this Agent facade.
///
/// Calling this function performs the already-existing credential-file reads and provider network
/// bootstrap. It performs no registry Get/Txn/Put, adds no retry/fallback/reconnect/background task,
/// and creates no startup/readiness/runtime callsite or production registry record.
///
/// # Errors
///
/// Returns [`ProductionDurableRegistryCustodyBootstrapError::Custody`] when bounded systemd custody
/// fails before provider I/O, or
/// [`ProductionDurableRegistryCustodyBootstrapError::ProviderBootstrap`] when the existing provider
/// bootstrap fails. No partial, degraded, cached, or in-memory fallback store is returned.
pub async fn bootstrap_production_durable_registry_from_systemd_credentials()
-> Result<DurableRegistryEtcdStore, ProductionDurableRegistryCustodyBootstrapError> {
    let config = load_durable_registry_production_etcd_bootstrap_config_from_systemd_credentials()
        .map_err(ProductionDurableRegistryCustodyBootstrapError::Custody)?;

    let executor = bootstrap_durable_registry_production_executor(config)
        .await
        .map_err(ProductionDurableRegistryCustodyBootstrapError::ProviderBootstrap)?;

    Ok(DurableRegistryEtcdStore::new(executor))
}

/// Populates one dormant production durable capability authority from the fixed production
/// durable-registry systemd custody/provider bootstrap.
///
/// The existing durable-registry bootstrap is awaited exactly once. On success, the returned semantic
/// store is consumed exactly once into [`ProductionDurableRegistryRuntimeCustody`] and that custody is
/// then consumed exactly once into [`ProductionDurableCapabilityAuthority`]. The authority retains the
/// existing production deny-all policy baseline through its existing constructor.
///
/// Population performs no registry semantic read, mutex acquisition, session or transport validation,
/// policy evaluation, authorization, request decoding, dispatcher invocation, response I/O, task
/// spawn, readiness publication or runtime/network activation.
///
/// # Errors
///
/// Propagates [`ProductionDurableRegistryCustodyBootstrapError`] unchanged from the existing one-shot
/// production durable-registry bootstrap. No retry, fallback, synthetic authority or partial custody
/// is returned.
pub(crate) async fn bootstrap_production_durable_capability_authority_from_systemd_credentials()
-> Result<ProductionDurableCapabilityAuthority, ProductionDurableRegistryCustodyBootstrapError> {
    let store = bootstrap_production_durable_registry_from_systemd_credentials().await?;
    let registry_custody = ProductionDurableRegistryRuntimeCustody::from_store(store);
    Ok(ProductionDurableCapabilityAuthority::from_registry_custody(
        registry_custody,
    ))
}

#[cfg(test)]
mod tests {
    use std::{error::Error, future::Future};

    use prw_control_plane::durable_registry_etcd_bootstrap::DurableRegistryProductionEtcdBootstrapError;
    use prw_reachability_custody::durable_registry_custody::DurableRegistryCustodyError;
    use prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStore;

    use super::{
        ProductionDurableRegistryCustodyBootstrapError,
        bootstrap_production_durable_capability_authority_from_systemd_credentials,
        bootstrap_production_durable_registry_from_systemd_credentials,
    };
    use crate::production_durable_registry_runtime_custody::ProductionDurableCapabilityAuthority;

    fn assert_bootstrap_signature() {
        fn assert_future<F>(_future: F)
        where
            F: Future<
                Output = Result<
                    DurableRegistryEtcdStore,
                    ProductionDurableRegistryCustodyBootstrapError,
                >,
            >,
        {
        }

        assert_future(bootstrap_production_durable_registry_from_systemd_credentials());
    }

    fn assert_capability_authority_bootstrap_signature() {
        fn assert_future<F>(_future: F)
        where
            F: Future<
                Output = Result<
                    ProductionDurableCapabilityAuthority,
                    ProductionDurableRegistryCustodyBootstrapError,
                >,
            >,
        {
        }

        assert_future(
            bootstrap_production_durable_capability_authority_from_systemd_credentials(),
        );
    }

    #[test]
    fn production_registry_custody_bootstrap_has_zero_argument_store_return_shape() {
        let _ = assert_bootstrap_signature as fn();
    }

    #[test]
    fn production_capability_authority_bootstrap_has_selected_zero_argument_return_shape() {
        let _ = assert_capability_authority_bootstrap_signature as fn();
    }

    #[test]
    fn custody_failure_is_wrapped_without_secret_detail_in_display() {
        let error = ProductionDurableRegistryCustodyBootstrapError::from(
            DurableRegistryCustodyError::CredentialsDirectoryMissing,
        );
        assert!(matches!(
            error,
            ProductionDurableRegistryCustodyBootstrapError::Custody(
                DurableRegistryCustodyError::CredentialsDirectoryMissing
            )
        ));
        assert_eq!(
            error.to_string(),
            "production durable registry custody bootstrap failed"
        );
        assert!(error.source().is_some());
    }

    #[test]
    fn provider_bootstrap_failure_is_wrapped_without_provider_detail_in_display() {
        let error = ProductionDurableRegistryCustodyBootstrapError::from(
            DurableRegistryProductionEtcdBootstrapError::RegistryConnect,
        );
        assert!(matches!(
            error,
            ProductionDurableRegistryCustodyBootstrapError::ProviderBootstrap(
                DurableRegistryProductionEtcdBootstrapError::RegistryConnect
            )
        ));
        assert_eq!(
            error.to_string(),
            "production durable registry provider bootstrap failed"
        );
        assert!(error.source().is_some());
    }
}
