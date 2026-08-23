//! Agent-owned lifetime boundary for one connected authenticated remote application session.
//!
//! C03e-I selected this outer ownership shape. C03e-K materializes the by-value owner that retains
//! the already-authenticated live peer together with the C03e-J capability owner. C03e-L adds only
//! the post-authentication binding/composition transaction that creates that owner from an existing
//! authenticated logical session and a separately verifier-owned application lease interval. It
//! does not run a request loop, spawn tasks, publish readiness, retry/reconnect, or wire the Agent
//! binary.

use std::ops::Range;

use prw_remote_bridge::{
    RemoteBridgeError,
    remote_server_transport_runtime::AuthenticatedRemotePeerConnection,
    remote_session_binding::BoundRemoteSession,
};
use prw_session::AuthenticatedDeviceSession;

use super::RemoteSessionCapabilityRuntimeOwner;

const REMOTE_SESSION_BINDING_FAILURE_CLOSE_CODE: u32 = 2;
const REMOTE_SESSION_BINDING_FAILURE_CLOSE_REASON: &[u8] = b"remote session binding failed";

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

/// Binds one already-authenticated logical session to its same live peer and application lease.
///
/// The peer's already-revalidated [`prw_remote_bridge::remote_server_transport_runtime::TransportIdentity`]
/// is snapshotted exactly once before delegating lease validation and binding construction to the
/// existing [`BoundRemoteSession::new`] implementation. The verifier supplies the application lease
/// interval independently of authentication-challenge timing.
///
/// On binding failure, the same peer is explicitly closed with a fixed non-secret diagnostic and the
/// existing [`RemoteBridgeError`] is returned unchanged. Pending-session abort is intentionally not
/// invoked because successful authentication already consumed the pending challenge.
///
/// # Errors
///
/// Returns the existing [`RemoteBridgeError`] produced by [`BoundRemoteSession::new`], including its
/// current invalid-lease classification. No retry, replacement session, replacement lease, or
/// authenticated-session deletion is attempted.
pub fn compose_authenticated_remote_session(
    peer: AuthenticatedRemotePeerConnection,
    session: AuthenticatedDeviceSession,
    application_lease_unix_seconds: Range<u64>,
) -> Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteBridgeError> {
    let transport_identity = peer.transport_identity();
    let bound_session = match BoundRemoteSession::new(
        transport_identity,
        session,
        application_lease_unix_seconds.start,
        application_lease_unix_seconds.end,
    ) {
        Ok(bound_session) => bound_session,
        Err(error) => {
            peer.close(
                REMOTE_SESSION_BINDING_FAILURE_CLOSE_CODE,
                REMOTE_SESSION_BINDING_FAILURE_CLOSE_REASON,
            );
            return Err(error);
        }
    };
    let capability_owner = RemoteSessionCapabilityRuntimeOwner::new(bound_session);
    Ok(AuthenticatedRemoteSessionRuntimeOwner::new(
        peer,
        capability_owner,
    ))
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use prw_remote_bridge::{
        RemoteBridgeError,
        remote_server_transport_runtime::AuthenticatedRemotePeerConnection,
    };
    use prw_session::AuthenticatedDeviceSession;

    use super::{
        AuthenticatedRemoteSessionRuntimeOwner, REMOTE_SESSION_BINDING_FAILURE_CLOSE_CODE,
        REMOTE_SESSION_BINDING_FAILURE_CLOSE_REASON, compose_authenticated_remote_session,
    };
    use crate::remote_session_capability_runtime::RemoteSessionCapabilityRuntimeOwner;

    fn assert_constructor_signature(
        constructor: fn(
            AuthenticatedRemotePeerConnection,
            RemoteSessionCapabilityRuntimeOwner,
        ) -> AuthenticatedRemoteSessionRuntimeOwner,
    ) {
        let _ = constructor;
    }

    fn assert_composition_signature(
        composition: fn(
            AuthenticatedRemotePeerConnection,
            AuthenticatedDeviceSession,
            Range<u64>,
        ) -> Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteBridgeError>,
    ) {
        let _ = composition;
    }

    #[test]
    fn outer_owner_consumes_exact_peer_and_capability_owner_shape() {
        assert_constructor_signature(AuthenticatedRemoteSessionRuntimeOwner::new);
    }

    #[test]
    fn post_auth_composition_requires_peer_session_and_separate_lease_interval() {
        assert_composition_signature(compose_authenticated_remote_session);
    }

    #[test]
    fn binding_failure_peer_close_diagnostic_is_fixed_nonzero_and_nonempty() {
        assert_ne!(REMOTE_SESSION_BINDING_FAILURE_CLOSE_CODE, 0);
        assert!(!REMOTE_SESSION_BINDING_FAILURE_CLOSE_REASON.is_empty());
    }
}
