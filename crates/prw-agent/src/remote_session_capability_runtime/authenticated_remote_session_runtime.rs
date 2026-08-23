//! Agent-owned lifetime boundary for one connected authenticated remote application session.
//!
//! C03e-I selected this outer ownership shape. C03e-K materializes only the by-value owner that
//! retains the already-authenticated live peer together with the C03e-J capability owner. It does
//! not construct a bound session, perform transport I/O, run a request loop, spawn tasks, publish
//! readiness, retry/reconnect, or wire the Agent binary.

use prw_remote_bridge::remote_server_transport_runtime::AuthenticatedRemotePeerConnection;

use super::RemoteSessionCapabilityRuntimeOwner;

/// Retains one authenticated peer and its bound capability lifetime under one Agent owner.
pub struct AuthenticatedRemoteSessionRuntimeOwner {
    #[allow(
        dead_code,
        reason = "C03e-K retains the live peer for a separately gated session-operation seam"
    )]
    peer: AuthenticatedRemotePeerConnection,
    #[allow(
        dead_code,
        reason = "C03e-K retains the capability owner for a separately gated session-operation seam"
    )]
    capability_owner: RemoteSessionCapabilityRuntimeOwner,
}

impl AuthenticatedRemoteSessionRuntimeOwner {
    /// Composes ownership only; construction performs no I/O or authorization.
    #[must_use]
    pub const fn new(
        peer: AuthenticatedRemotePeerConnection,
        capability_owner: RemoteSessionCapabilityRuntimeOwner,
    ) -> Self {
        Self {
            peer,
            capability_owner,
        }
    }
}

#[cfg(test)]
mod tests {
    use prw_remote_bridge::remote_server_transport_runtime::AuthenticatedRemotePeerConnection;

    use super::AuthenticatedRemoteSessionRuntimeOwner;
    use crate::remote_session_capability_runtime::RemoteSessionCapabilityRuntimeOwner;

    fn assert_constructor_signature(
        constructor: fn(
            AuthenticatedRemotePeerConnection,
            RemoteSessionCapabilityRuntimeOwner,
        ) -> AuthenticatedRemoteSessionRuntimeOwner,
    ) {
        let _ = constructor;
    }

    #[test]
    fn outer_owner_consumes_exact_peer_and_capability_owner_shape() {
        assert_constructor_signature(AuthenticatedRemoteSessionRuntimeOwner::new);
    }
}
