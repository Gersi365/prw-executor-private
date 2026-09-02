//! Agent-owned endpoint lifecycle custody for production reachability.
//!
//! C03e-IA materializes only the C03e-HZ-selected endpoint-startup custody transaction. The
//! wrapper retains the existing remote endpoint lifecycle beside the recovered durable production
//! reachability owner. A startup failure retains the complete pre-bind production runtime custody.
//!
//! This module adds no executable callsite, runtime drive, readiness publication, candidate
//! publication, traversal activation, peer dial, retry, provider re-bootstrap, systemd unit/package
//! mutation, deployment, or production-state mutation.

use std::{fmt, net::SocketAddr};

use crate::{
    production_reachability_owner_composition::ProductionReachabilityEtcdOwnerCustody,
    production_reachability_runtime_custody::ProductionReachabilityRuntimeCustody,
    remote_session_capability_runtime::{
        RemoteSessionEndpointBoundAddressError, RemoteSessionEndpointLifecycleRuntime,
        RemoteSessionEndpointLifecycleStartupError,
    },
};

type EndpointStartupCustodyResult<Authority, Durable, Endpoint, Controller, Error> = Result<
    ((Endpoint, Durable), Controller),
    ((Authority, Durable), Error),
>;

pub(super) fn compose_endpoint_startup_custody<Authority, Durable, Endpoint, Controller, Error>(
    authority: Authority,
    durable: Durable,
    bind: impl FnOnce(Authority) -> Result<(Endpoint, Controller), (Authority, Error)>,
) -> EndpointStartupCustodyResult<Authority, Durable, Endpoint, Controller, Error> {
    match bind(authority) {
        Ok((endpoint, controller)) => Ok(((endpoint, durable), controller)),
        Err((authority, error)) => Err(((authority, durable), error)),
    }
}

/// Joint production custody for one successfully started remote endpoint lifecycle.
///
/// The existing endpoint lifecycle retains the live reachability authority. The recovered durable
/// production-owner custody remains retained beside it and is not cloneable or independently
/// extractable from this wrapper.
pub struct ProductionReachabilityEndpointLifecycleRuntime {
    endpoint: RemoteSessionEndpointLifecycleRuntime,
    owner_custody: ProductionReachabilityEtcdOwnerCustody,
}

impl ProductionReachabilityEndpointLifecycleRuntime {
    #[must_use]
    pub(super) const fn new(
        endpoint: RemoteSessionEndpointLifecycleRuntime,
        owner_custody: ProductionReachabilityEtcdOwnerCustody,
    ) -> Self {
        Self {
            endpoint,
            owner_custody,
        }
    }

    /// Returns the exact local socket address reported by the retained already-bound endpoint.
    ///
    /// This delegates to the existing read-only endpoint observation and performs no candidate
    /// construction/publication, retry, bind, rebind, close or readiness mutation.
    ///
    /// # Errors
    ///
    /// Returns the existing bounded endpoint observation error unchanged.
    pub fn bound_addr(&self) -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError> {
        self.endpoint.bound_addr()
    }
}

/// Recoverable production endpoint-startup failure retaining complete pre-bind runtime custody.
pub struct ProductionReachabilityEndpointLifecycleStartupFailure {
    runtime_custody: ProductionReachabilityRuntimeCustody,
    error: RemoteSessionEndpointLifecycleStartupError,
}

impl fmt::Debug for ProductionReachabilityEndpointLifecycleStartupFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProductionReachabilityEndpointLifecycleStartupFailure")
            .field("runtime_custody", &"<retained>")
            .field("error", &self.error)
            .finish()
    }
}

impl fmt::Display for ProductionReachabilityEndpointLifecycleStartupFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for ProductionReachabilityEndpointLifecycleStartupFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl ProductionReachabilityEndpointLifecycleStartupFailure {
    #[must_use]
    pub(super) const fn new(
        runtime_custody: ProductionReachabilityRuntimeCustody,
        error: RemoteSessionEndpointLifecycleStartupError,
    ) -> Self {
        Self {
            runtime_custody,
            error,
        }
    }

    /// Returns the existing bounded endpoint startup classification.
    #[must_use]
    pub const fn error(&self) -> RemoteSessionEndpointLifecycleStartupError {
        self.error
    }

    /// Recovers the complete production runtime custody after failed endpoint startup.
    #[must_use]
    pub fn into_runtime_custody(self) -> ProductionReachabilityRuntimeCustody {
        self.runtime_custody
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, net::SocketAddr};

    use super::{
        ProductionReachabilityEndpointLifecycleRuntime, compose_endpoint_startup_custody,
    };
    use crate::remote_session_capability_runtime::RemoteSessionEndpointBoundAddressError;

    fn assert_bound_addr_signature(
        observation: fn(
            &ProductionReachabilityEndpointLifecycleRuntime,
        ) -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError>,
    ) {
        let _ = observation;
    }

    #[test]
    fn success_routes_endpoint_and_durable_custody_without_retry() {
        let calls = Cell::new(0_u8);
        let result = compose_endpoint_startup_custody(11_u8, 22_u8, |authority| {
            calls.set(calls.get() + 1);
            assert_eq!(authority, 11);
            Ok::<_, (u8, u8)>((33_u8, 44_u8))
        });

        assert_eq!(result, Ok(((33, 22), 44)));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn failure_recombines_authority_and_durable_custody_without_retry() {
        let calls = Cell::new(0_u8);
        let result = compose_endpoint_startup_custody(11_u8, 22_u8, |authority| {
            calls.set(calls.get() + 1);
            Err::<(u8, u8), _>((authority, 55_u8))
        });

        assert_eq!(result, Err(((11, 22), 55)));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn production_endpoint_bound_address_has_exact_read_only_shape() {
        assert_bound_addr_signature(ProductionReachabilityEndpointLifecycleRuntime::bound_addr);
    }
}
