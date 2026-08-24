//! Agent-owned lifetime boundary for one connected authenticated remote application session.
//!
//! C03e-I selected this outer ownership shape. C03e-K materializes the by-value owner that retains
//! the already-authenticated live peer together with the C03e-J capability owner. C03e-L adds the
//! post-authentication binding/composition transaction, C03e-O adds exactly one serialized
//! capability request transaction over the C03e-N bridge-owned wire adapter, C03e-Q adds the
//! C03e-P-selected borrowed serial request loop, and C03e-S adds the C03e-R-selected executor-neutral
//! cancellation-aware single-worker seam. It does not spawn tasks, publish readiness, retry/reconnect,
//! or wire the Agent binary.

use std::{
    fmt,
    future::{Future, poll_fn},
    ops::Range,
    task::Poll,
};

use prw_core::DeviceId;
use prw_policy::PolicyEvaluator;
use prw_remote_bridge::{
    CapabilityBridge, CapabilityDispatcher, RemoteBridgeError,
    authorized_request_dispatch::dispatch_authorized_request,
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

use super::{RemoteSessionCapabilityRuntimeOwner, SharedCurrentCapabilityAuthority};

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
const REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_CODE: u32 = 3;
const REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_REASON: &[u8] =
    b"remote capability session terminated";
const REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_CODE: u32 = 4;
const REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_REASON: &[u8] =
    b"remote capability session shutdown";

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

/// Terminal result of the executor-neutral C03e-S single-worker seam.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthenticatedRemoteSessionWorkerStop {
    /// External worker cancellation won and the retained peer was closed with the code-4 diagnostic.
    Cancelled,
    /// The existing C03e-Q loop failed first and preserved its original typed transaction failure.
    Failed(AuthenticatedRemoteSessionCapabilityTransactionError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthenticatedRemoteSessionWorkerRaceOutcome {
    Cancelled,
    Failed(AuthenticatedRemoteSessionCapabilityTransactionError),
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

    /// Returns the authenticated logical DeviceId retained by this runtime owner.
    ///
    /// This Agent-internal accessor derives identity only from the already-bound authenticated
    /// session. It performs no I/O, registry lookup, policy evaluation or transport selection.
    #[must_use]
    pub(super) const fn logical_device_id(&self) -> &DeviceId {
        self.capability_owner.bound_session.session().device_id()
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
    pub async fn process_one_capability_request<
        P: PolicyEvaluator + Send + Sync,
        D: CapabilityDispatcher + Send,
    >(
        &mut self,
        authority: &SharedCurrentCapabilityAuthority<P>,
        now_unix_seconds: u64,
        dispatcher: &mut D,
    ) -> Result<(), AuthenticatedRemoteSessionCapabilityTransactionError> {
        let mut stream = self.peer.accept_control_stream().await?;
        let request = receive_capability_request_frame(&mut stream).await?;
        let bound_session = &self.capability_owner.bound_session;
        let authorized = authority
            .with_current_authority(|registry, policy| {
                let bridge = CapabilityBridge::new(registry, policy);
                bound_session.authorize(&bridge, now_unix_seconds, &request)
            })
            .await?;
        let response = dispatch_authorized_request(&authorized, dispatcher)?;
        send_capability_response_frame(&mut stream, &response).await?;
        Ok(())
    }

    /// Runs the C03e-P-selected serial capability-session request loop.
    ///
    /// The verifier-time provider is sampled exactly once immediately before each existing C03e-O
    /// transaction. Only a successful transaction reaches the next iteration. The first transaction
    /// failure closes the same retained peer exactly once with the fixed capability-session
    /// termination diagnostic and returns that original typed failure unchanged.
    ///
    /// This borrowed loop owns no task, cancellation token, drain deadline, join handle, retry,
    /// replacement session, concurrent request admission or readiness state.
    ///
    /// # Errors
    ///
    /// Returns the first [`AuthenticatedRemoteSessionCapabilityTransactionError`] emitted by the
    /// existing one-request transaction after explicitly closing the retained peer.
    pub async fn run_capability_request_loop<
        P: PolicyEvaluator + Send + Sync,
        D: CapabilityDispatcher + Send,
        T: FnMut() -> u64 + Send,
    >(
        &mut self,
        authority: &SharedCurrentCapabilityAuthority<P>,
        mut verifier_time_unix_seconds: T,
        dispatcher: &mut D,
    ) -> Result<(), AuthenticatedRemoteSessionCapabilityTransactionError> {
        loop {
            let now_unix_seconds = verifier_time_unix_seconds();
            if let Err(error) = self
                .process_one_capability_request(authority, now_unix_seconds, dispatcher)
                .await
            {
                self.peer.close(
                    REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_CODE,
                    REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_REASON,
                );
                return Err(error);
            }
        }
    }

    /// Runs one cancellation-aware remote-session worker body without spawning a task.
    ///
    /// The caller supplies an executor-neutral cancellation future. This method polls the existing
    /// C03e-Q loop before cancellation on each wake, so an already-ready terminal Q failure retains
    /// its original code-3 close/error classification. Cancellation wins only while Q remains
    /// pending. When cancellation wins, the in-flight Q future is dropped first; after that mutable
    /// owner borrow is released, this method closes the same retained peer exactly once with the
    /// fixed code-4 shutdown diagnostic and returns [`AuthenticatedRemoteSessionWorkerStop::Cancelled`].
    ///
    /// A clean `Ok(())` return from the current Q loop is not a selected lifecycle completion. If a
    /// future Q implementation ever produces it, this seam stops polling that completed loop and
    /// remains pending solely on the caller-owned cancellation future rather than fabricating a
    /// success or failure classification.
    ///
    /// This method creates no channel, task, runtime, join handle, registry entry, retry or readiness
    /// state and does not expose the retained peer.
    pub async fn run_capability_request_worker<
        P: PolicyEvaluator + Send + Sync,
        D: CapabilityDispatcher + Send,
        T: FnMut() -> u64 + Send,
        C: Future<Output = ()> + Send,
    >(
        &mut self,
        authority: &SharedCurrentCapabilityAuthority<P>,
        verifier_time_unix_seconds: T,
        dispatcher: &mut D,
        cancellation: C,
    ) -> AuthenticatedRemoteSessionWorkerStop {
        let outcome = {
            let mut request_loop = Box::pin(self.run_capability_request_loop(
                authority,
                verifier_time_unix_seconds,
                dispatcher,
            ));
            let mut cancellation = Box::pin(cancellation);
            let mut request_loop_completed_cleanly = false;

            poll_fn(|context| {
                if !request_loop_completed_cleanly {
                    match request_loop.as_mut().poll(context) {
                        Poll::Ready(Ok(())) => request_loop_completed_cleanly = true,
                        Poll::Ready(Err(error)) => {
                            return Poll::Ready(
                                AuthenticatedRemoteSessionWorkerRaceOutcome::Failed(error),
                            );
                        }
                        Poll::Pending => {}
                    }
                }

                match cancellation.as_mut().poll(context) {
                    Poll::Ready(()) => {
                        Poll::Ready(AuthenticatedRemoteSessionWorkerRaceOutcome::Cancelled)
                    }
                    Poll::Pending => Poll::Pending,
                }
            })
            .await
        };

        match outcome {
            AuthenticatedRemoteSessionWorkerRaceOutcome::Cancelled => {
                self.peer.close(
                    REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_CODE,
                    REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_REASON,
                );
                AuthenticatedRemoteSessionWorkerStop::Cancelled
            }
            AuthenticatedRemoteSessionWorkerRaceOutcome::Failed(error) => {
                AuthenticatedRemoteSessionWorkerStop::Failed(error)
            }
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
        AuthenticatedRemoteSessionRuntimeOwner, AuthenticatedRemoteSessionWorkerStop,
        REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_CODE,
        REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_REASON,
        REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_CODE,
        REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_REASON,
        REMOTE_SESSION_BINDING_FAILURE_CLOSE_CODE, REMOTE_SESSION_BINDING_FAILURE_CLOSE_REASON,
        compose_authenticated_remote_session,
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
    fn capability_session_termination_close_diagnostic_is_fixed_nonzero_and_nonempty() {
        assert_eq!(REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_CODE, 3);
        assert!(!REMOTE_CAPABILITY_SESSION_TERMINATION_CLOSE_REASON.is_empty());
    }

    #[test]
    fn capability_session_shutdown_close_diagnostic_is_fixed_nonzero_and_nonempty() {
        assert_eq!(REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_CODE, 4);
        assert!(!REMOTE_CAPABILITY_SESSION_SHUTDOWN_CLOSE_REASON.is_empty());
    }

    #[test]
    fn bridge_failure_classification_is_preserved() {
        let error = RemoteBridgeError::SessionExpired;
        assert_eq!(
            AuthenticatedRemoteSessionCapabilityTransactionError::from(error),
            AuthenticatedRemoteSessionCapabilityTransactionError::Bridge(error)
        );
    }

    #[test]
    fn worker_failure_preserves_exact_transaction_error() {
        let error = AuthenticatedRemoteSessionCapabilityTransactionError::Bridge(
            RemoteBridgeError::SessionExpired,
        );
        let stop = AuthenticatedRemoteSessionWorkerStop::Failed(error);

        match stop {
            AuthenticatedRemoteSessionWorkerStop::Failed(observed) => {
                assert_eq!(observed, error);
            }
            AuthenticatedRemoteSessionWorkerStop::Cancelled => {
                panic!("worker failure must not be reclassified as cancellation");
            }
        }
    }
}
