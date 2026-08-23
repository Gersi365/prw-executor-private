//! Authority-gated real remote transport owner for the Ubuntu PRW Agent.
//!
//! C03e-C is the first Agent-owned composition that can bind a real mesh QUIC server endpoint. The
//! constructor requires the opaque reachability-authority runtime owner and retains it for the full
//! endpoint lifetime. This module does not accept peers, authenticate logical sessions, dispatch
//! capabilities, spawn tasks, or publish remote readiness.

use std::{fmt, net::SocketAddr};

use prw_reachability_custody::mesh_transport_custody::{
    MeshTransportCustodyError, load_mesh_transport_credentials_from_systemd,
};
use prw_remote_bridge::remote_server_transport_runtime::{
    RemoteServerTransportRuntime, RemoteServerTransportRuntimeError,
};

use crate::reachability_authority_admission::ReachabilityAuthorityRuntimeOwner;

/// Stable failure class while constructing the authority-gated Agent remote endpoint owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AgentRemoteTransportBindError {
    /// Fixed systemd mesh credential custody failed before transport construction.
    Credential(MeshTransportCustodyError),
    /// Locked TLS construction or the real UDP/QUIC endpoint bind failed.
    Transport(RemoteServerTransportRuntimeError),
}

impl fmt::Display for AgentRemoteTransportBindError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credential(_) => {
                formatter.write_str("Agent mesh transport credential load failed")
            }
            Self::Transport(_) => formatter.write_str("Agent remote transport bind failed"),
        }
    }
}

impl std::error::Error for AgentRemoteTransportBindError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Credential(error) => Some(error),
            Self::Transport(error) => Some(error),
        }
    }
}

impl From<MeshTransportCustodyError> for AgentRemoteTransportBindError {
    fn from(error: MeshTransportCustodyError) -> Self {
        Self::Credential(error)
    }
}

impl From<RemoteServerTransportRuntimeError> for AgentRemoteTransportBindError {
    fn from(error: RemoteServerTransportRuntimeError) -> Self {
        Self::Transport(error)
    }
}

/// Failed bind transaction that retains the already-admitted authority owner.
pub struct AgentRemoteTransportBindFailure {
    authority_owner: ReachabilityAuthorityRuntimeOwner,
    error: AgentRemoteTransportBindError,
}

impl fmt::Debug for AgentRemoteTransportBindFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentRemoteTransportBindFailure")
            .field("authority_owner", &"<retained>")
            .field("error", &self.error)
            .finish()
    }
}

impl fmt::Display for AgentRemoteTransportBindFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for AgentRemoteTransportBindFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl AgentRemoteTransportBindFailure {
    const fn new(
        authority_owner: ReachabilityAuthorityRuntimeOwner,
        error: AgentRemoteTransportBindError,
    ) -> Self {
        Self {
            authority_owner,
            error,
        }
    }

    /// Returns the stable underlying bind failure without exposing authority internals.
    #[must_use]
    pub const fn error(&self) -> AgentRemoteTransportBindError {
        self.error
    }

    /// Recovers the exact admitted authority owner after a failed endpoint transaction.
    #[must_use]
    pub fn into_authority_owner(self) -> ReachabilityAuthorityRuntimeOwner {
        self.authority_owner
    }
}

/// Agent-owned proof that authority admission and one real QUIC server bind both succeeded.
pub struct AgentRemoteTransportRuntime {
    authority_owner: ReachabilityAuthorityRuntimeOwner,
    transport: RemoteServerTransportRuntime,
}

impl AgentRemoteTransportRuntime {
    /// Loads the fixed mesh credentials and binds one real server endpoint behind authority admission.
    ///
    /// The admitted authority owner is consumed on entry. Any custody/TLS/socket failure returns it
    /// inside [`AgentRemoteTransportBindFailure`], so failure never fabricates a runtime and does not
    /// require an automatic authority re-bootstrap. The private PKCS#8 key is moved, not cloned, out
    /// of its C03e-B zeroizing transfer container immediately before transport-owned TLS conversion.
    ///
    /// # Errors
    ///
    /// Returns [`AgentRemoteTransportBindFailure`] on fixed credential custody failure, locked TLS
    /// construction failure, or real UDP/QUIC endpoint bind failure.
    pub fn bind_from_systemd_credentials(
        authority_owner: ReachabilityAuthorityRuntimeOwner,
        bind_addr: SocketAddr,
    ) -> Result<Self, AgentRemoteTransportBindFailure> {
        let credentials = match load_mesh_transport_credentials_from_systemd() {
            Ok(credentials) => credentials,
            Err(error) => {
                return Err(AgentRemoteTransportBindFailure::new(
                    authority_owner,
                    AgentRemoteTransportBindError::Credential(error),
                ));
            }
        };
        let (root_certificate_der, certificate_der, mut private_key_pkcs8_der) =
            credentials.into_transport_tls_der();
        let private_key_pkcs8_der = std::mem::take(&mut *private_key_pkcs8_der);
        let transport = match RemoteServerTransportRuntime::bind_from_der(
            bind_addr,
            root_certificate_der,
            certificate_der,
            private_key_pkcs8_der,
        ) {
            Ok(transport) => transport,
            Err(error) => {
                return Err(AgentRemoteTransportBindFailure::new(
                    authority_owner,
                    AgentRemoteTransportBindError::Transport(error),
                ));
            }
        };
        Ok(Self {
            authority_owner,
            transport,
        })
    }

    /// Returns the retained admitted authority owner by immutable reference.
    #[must_use]
    pub const fn authority_owner(&self) -> &ReachabilityAuthorityRuntimeOwner {
        &self.authority_owner
    }

    /// Returns the kernel-selected local UDP address of the real mesh endpoint.
    ///
    /// # Errors
    ///
    /// Propagates the existing bridge/transport endpoint-address failure.
    pub fn local_addr(&self) -> Result<SocketAddr, AgentRemoteTransportBindError> {
        self.transport
            .local_addr()
            .map_err(AgentRemoteTransportBindError::Transport)
    }

    /// Explicitly closes the remote endpoint without affecting existing local Agent readiness.
    pub fn close(&self, code: u32, reason: &[u8]) {
        self.transport.close(code, reason);
    }

    /// Waits until all remote endpoint connections are idle.
    pub async fn wait_idle(&self) {
        self.transport.wait_idle().await;
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::{
        AgentRemoteTransportBindError, AgentRemoteTransportBindFailure, AgentRemoteTransportRuntime,
    };
    use crate::reachability_authority_admission::ReachabilityAuthorityRuntimeOwner;

    fn assert_constructor_signature(
        constructor: fn(
            ReachabilityAuthorityRuntimeOwner,
            SocketAddr,
        )
            -> Result<AgentRemoteTransportRuntime, AgentRemoteTransportBindFailure>,
    ) {
        let _ = constructor;
    }

    fn assert_failure_accessors(
        error: fn(&AgentRemoteTransportBindFailure) -> AgentRemoteTransportBindError,
        recover: fn(AgentRemoteTransportBindFailure) -> ReachabilityAuthorityRuntimeOwner,
    ) {
        let _ = (error, recover);
    }

    #[test]
    fn remote_endpoint_constructor_requires_exact_authority_owner() {
        assert_constructor_signature(AgentRemoteTransportRuntime::bind_from_systemd_credentials);
    }

    #[test]
    fn failed_bind_surface_preserves_recoverable_authority_ownership() {
        assert_failure_accessors(
            AgentRemoteTransportBindFailure::error,
            AgentRemoteTransportBindFailure::into_authority_owner,
        );
    }
}
