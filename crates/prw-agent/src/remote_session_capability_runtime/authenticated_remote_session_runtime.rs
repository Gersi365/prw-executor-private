//! Agent-owned lifetime boundary for one connected authenticated remote application session.
//!
//! C03e-I selected this outer ownership shape. C03e-K materializes the by-value owner that retains
//! the already-authenticated live peer together with the C03e-J capability owner. C03e-L adds the
//! post-authentication binding/composition transaction, while C03e-O adds exactly one serialized
//! capability request transaction over the C03e-N bridge-owned wire adapter. It does not run a
//! request loop, spawn tasks, publish readiness, retry/reconnect, or wire the Agent binary.

use std::{fmt, ops::Range};

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::{
    CapabilityBridge, CapabilityDispatcher, RemoteBridgeError,
    capability_request_wire::{
        CapabilityRequestWireError, receive_capability_request_frame,
        send_capability_response_frame,
    },
    remote_server_transport_runtime::{
        AuthenticatedRemotePeerConnection, RemoteServerTransportRuntimeError,
    },
    remote_session_binding::BoundRemoteSession,
};
use prw_session::AuthenticatedDeviceSession;

use super::RemoteSessionCapabilityRuntimeOwner;

#[allow(
    dead_code,
    reason = "C03e-L stages the binding composition seam before separately gated operation-surface exposure"
)]
const REMOTE_SESSION_BINDING_FAILURE_CLOSE_CODE: u32 = 2;
#[allow(
    dead_code,
    reason = "C03e-L stages the binding composition seam before separately gated operation-surface exposure"
)]
const REMOTE_SESSION_BINDING_FAILURE_CLOSE_REASON: &[u8] = b"remote session binding failed";

/// Failure while processing exactly one capability request on one authenticated remote session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthenticatedRemoteSessionCapabilityTransactionError {
    /// Accepting the next bounded control stream from the retained authenticated peer failed.
    Accept(RemoteServerTransportRuntimeError),
    /// Receiving or sending the one bounded PRWM frame failed.
    Wire(CapabilityRequestWireError),
    /// Current bound-session authorization or capability dispatch failed.
    Bridge(RemoteBridgeError),
}

impl fmt::Display for AuthenticatedRemoteSessionCapabilityTransactionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept(_) => formatter.write_str("remote capability stream acceptance failed"),
            Self::Wire(_) => formatter.write_str("remote capability wire transaction failed"),
            Self::Bridge(_) => formatter.write_str("remote capability bridge transaction failed"),
        }
    }
}

impl std::error::Error for AuthenticatedRemoteSessionCapabilityTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Accept(error) => Some(error),
            Self::Wire(error) => Some(error),
            Self::Bridge(error) => Some(error),
        }
    }
}

impl From<RemoteServerTransportRuntimeError>
    for AuthenticatedRemoteSessionCapabilityTransactionError
{
    fn from(error: RemoteServerTransportRuntimeError) -> Self {
        Self::Accept(error)
    }
}

impl From<CapabilityRequestWireError> for AuthenticatedRemoteSessionCapabilityTransactionError {
    fn from(error: CapabilityRequestWireError) -> Self {
        Self::Wire(error)
    }
}

impl From<RemoteBridgeError> for AuthenticatedRemoteSessionCapabilityTransactionError {
    fn from(error: RemoteBridgeError) -> Self {
        Self::Bridge(error)
    }
}

/// Retains one authenticated peer and its bound capability lifetime under one Agent owner.
pub struct AuthenticatedRemoteSessionRuntimeOwner {
    peer: AuthenticatedRemotePeerConnection,
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

    /// Processes exactly one capability request on exactly one newly accepted control stream.
    ///
    /// The mutable owner borrow deliberately serializes this operation boundary. The retained peer
    /// accepts one stream, the C03e-N adapter receives one bounded PRWM frame, and the retained
    /// bound session delegates exactly once to the current [`CapabilityBridge`] using caller-supplied
    /// verifier time and mutable dispatcher. Only bridge success is sent as one response frame on
    /// the same stream.
    ///
    /// No transport identity, logical identity, lease, registry result or policy result is selected
    /// by this method. The retained [`BoundRemoteSession`] continues to supply its bound transport
    /// identity and lease internally, while the bridge performs current registry/policy validation.
    ///
    /// # Errors
    ///
    /// Returns the existing bounded stream-accept failure, C03e-N wire failure or existing
    /// [`RemoteBridgeError`] through [`AuthenticatedRemoteSessionCapabilityTransactionError`].
    /// Failure produces no fabricated success response, retry, replacement stream/session/lease,
    /// pending-session abort, authenticated-session deletion or automatic whole-peer close.
    pub async fn process_one_capability_request<P: PolicyEvaluator, D: CapabilityDispatcher>(
        &mut self,
        bridge: &CapabilityBridge<'_, P>,
        now_unix_seconds: u64,
        dispatcher: &mut D,
    ) -> Result<(), AuthenticatedRemoteSessionCapabilityTransactionError> {
        let mut stream = self.peer.accept_control_stream().await?;
        let request = receive_capability_request_frame(&mut stream).await?;
        let response = self.capability_owner.bound_session.process_request(
            bridge,
            now_unix_seconds,
            &request,
            dispatcher,
        )?;
        send_capability_response_frame(&mut stream, &response).await?;
        Ok(())
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
#[allow(
    dead_code,
    reason = "C03e-L stages the binding composition seam before separately gated operation-surface exposure"
)]
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
        RemoteBridgeError, remote_server_transport_runtime::AuthenticatedRemotePeerConnection,
    };
    use prw_session::AuthenticatedDeviceSession;

    use super::{
        AuthenticatedRemoteSessionCapabilityTransactionError,
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
        )
            -> Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteBridgeError>,
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

    #[test]
    fn bridge_failure_classification_is_preserved() {
        let error = RemoteBridgeError::SessionExpired;
        assert_eq!(
            AuthenticatedRemoteSessionCapabilityTransactionError::from(error),
            AuthenticatedRemoteSessionCapabilityTransactionError::Bridge(error)
        );
    }
}
