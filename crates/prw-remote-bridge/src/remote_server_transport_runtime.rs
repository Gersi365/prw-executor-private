//! Bridge-owned real QUIC server endpoint wrapper for the authority-gated Agent runtime.
//!
//! C03e-C composes the existing C03e-B DER helper and C03c real-socket endpoint behind one narrow
//! server-runtime owner. It does not accept peers, authenticate logical sessions, grant
//! capabilities, or publish Agent readiness.

use std::{fmt, net::SocketAddr};

use prw_remote_transport::runtime::{
    MeshQuicEndpoint, MeshQuicRuntimeError, build_server_config_from_der,
};

/// Failure while constructing or querying one bridge-owned real server endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteServerTransportRuntimeError {
    /// Existing C03c/C03e-B transport validation, TLS construction, or socket runtime failed.
    Transport(MeshQuicRuntimeError),
}

impl fmt::Display for RemoteServerTransportRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(_) => formatter.write_str("remote server transport runtime failed"),
        }
    }
}

impl std::error::Error for RemoteServerTransportRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
        }
    }
}

impl From<MeshQuicRuntimeError> for RemoteServerTransportRuntimeError {
    fn from(error: MeshQuicRuntimeError) -> Self {
        Self::Transport(error)
    }
}

/// One real bound QUIC server endpoint owned at the remote-bridge boundary.
#[derive(Debug)]
pub struct RemoteServerTransportRuntime {
    endpoint: MeshQuicEndpoint,
}

impl RemoteServerTransportRuntime {
    /// Builds the locked server TLS profile from owned DER and binds one real UDP/QUIC endpoint.
    ///
    /// TLS construction occurs before socket binding. The supplied address is explicit; this path
    /// performs no DNS discovery, reachability lookup, retry, peer acceptance, or readiness
    /// publication.
    ///
    /// # Errors
    ///
    /// Propagates the existing C03c/C03e-B failure through
    /// [`RemoteServerTransportRuntimeError::Transport`].
    pub fn bind_from_der(
        bind_addr: SocketAddr,
        root_certificate_der: Vec<u8>,
        certificate_der: Vec<u8>,
        private_key_pkcs8_der: Vec<u8>,
    ) -> Result<Self, RemoteServerTransportRuntimeError> {
        let server_config = build_server_config_from_der(
            root_certificate_der,
            certificate_der,
            private_key_pkcs8_der,
        )?;
        let endpoint = MeshQuicEndpoint::bind_server(bind_addr, server_config)?;
        Ok(Self { endpoint })
    }

    /// Returns the kernel-selected local UDP address of the bound endpoint.
    ///
    /// # Errors
    ///
    /// Propagates the existing C03c endpoint-address failure classification.
    pub fn local_addr(&self) -> Result<SocketAddr, RemoteServerTransportRuntimeError> {
        self.endpoint.local_addr().map_err(Into::into)
    }

    /// Explicitly closes this endpoint and all live connections.
    pub fn close(&self, code: u32, reason: &[u8]) {
        self.endpoint.close(code, reason);
    }

    /// Waits until the underlying real QUIC endpoint becomes idle.
    pub async fn wait_idle(&self) {
        self.endpoint.wait_idle().await;
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use prw_remote_transport::{
        RemoteTransportError,
        runtime::MeshQuicRuntimeError,
    };

    use super::{RemoteServerTransportRuntime, RemoteServerTransportRuntimeError};

    const fn loopback_any() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)
    }

    #[test]
    fn malformed_tls_material_fails_before_server_runtime_exists() {
        let result = RemoteServerTransportRuntime::bind_from_der(
            loopback_any(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(RemoteServerTransportRuntimeError::Transport(
                MeshQuicRuntimeError::Transport(RemoteTransportError::InvalidTrustRoots)
            ))
        ));
    }
}
