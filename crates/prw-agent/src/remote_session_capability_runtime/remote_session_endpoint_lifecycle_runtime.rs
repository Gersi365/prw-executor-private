//! Agent-owned startup and shutdown-control composition for one remote endpoint lifecycle.
//!
//! C03e-AO selected executor-before-bind startup, recoverable reachability-authority custody on
//! startup failure, one remote-specific explicit supervisor-shutdown pair, and delegation to the
//! existing C03e-AN endpoint lifecycle. C03e-AP materializes only those source seams. This module
//! does not wire Agent `main.rs`, publish readiness, consume process signals, retry startup, or
//! activate an endpoint from an executable path.

use std::{
    fmt,
    net::SocketAddr,
    num::NonZeroUsize,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use prw_core::DeviceId;
use prw_policy::PolicyEvaluator;
use prw_remote_bridge::CapabilityDispatcher;
use prw_session::SessionAuthenticationService;
use tokio::sync::{Notify, mpsc};

use super::{
    RemoteSessionExecutorRuntime, RemoteSessionExecutorRuntimeCreateError,
    RemoteSessionExpectedDeviceAdmissionRejection, RemoteSessionExpectedDeviceAdmissionRequest,
    RemoteSessionPersistentCollectionConfigError, RemoteSessionRealAdmissionTiming,
    RemoteSessionRegisteredWorkerCompletion, RemoteSessionRepeatedAdmissionFailure,
    SharedCurrentCapabilityAuthority,
};
use crate::{
    reachability_authority_admission::ReachabilityAuthorityRuntimeOwner,
    remote_transport_runtime::{AgentRemoteTransportBindError, AgentRemoteTransportRuntime},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndpointStartupCompositionError<ExecutorError, TransportError> {
    Executor(ExecutorError),
    Transport(TransportError),
}

fn compose_endpoint_startup<Authority, Executor, Transport, ExecutorError, TransportError>(
    authority: Authority,
    construct_executor: impl FnOnce() -> Result<Executor, ExecutorError>,
    bind_transport: impl FnOnce(Authority) -> Result<Transport, (Box<Authority>, TransportError)>,
) -> Result<
    (Executor, Transport),
    (
        Box<Authority>,
        EndpointStartupCompositionError<ExecutorError, TransportError>,
    ),
> {
    let executor = match construct_executor() {
        Ok(executor) => executor,
        Err(error) => {
            return Err((
                Box::new(authority),
                EndpointStartupCompositionError::Executor(error),
            ));
        }
    };

    let transport = match bind_transport(authority) {
        Ok(transport) => transport,
        Err((authority, error)) => {
            return Err((authority, EndpointStartupCompositionError::Transport(error)));
        }
    };

    Ok((executor, transport))
}

/// Stable failure class while composing one Agent-owned remote endpoint lifecycle runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteSessionEndpointLifecycleStartupError {
    /// Construction of the existing private current-thread executor failed before endpoint bind.
    Executor(RemoteSessionExecutorRuntimeCreateError),
    /// Existing fixed-credential/TLS/socket endpoint bind failed after executor construction.
    Transport(AgentRemoteTransportBindError),
}

impl fmt::Display for RemoteSessionEndpointLifecycleStartupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Executor(_) => {
                formatter.write_str("remote endpoint executor construction failed")
            }
            Self::Transport(_) => formatter.write_str("remote endpoint bind failed"),
        }
    }
}

impl std::error::Error for RemoteSessionEndpointLifecycleStartupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Executor(error) => Some(error),
            Self::Transport(error) => Some(error),
        }
    }
}

/// Recoverable failed startup transaction retaining the exact admitted reachability authority.
pub struct RemoteSessionEndpointLifecycleStartupFailure {
    authority_owner: Box<ReachabilityAuthorityRuntimeOwner>,
    error: RemoteSessionEndpointLifecycleStartupError,
}

impl fmt::Debug for RemoteSessionEndpointLifecycleStartupFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RemoteSessionEndpointLifecycleStartupFailure")
            .field("authority_owner", &"<retained>")
            .field("error", &self.error)
            .finish()
    }
}

impl fmt::Display for RemoteSessionEndpointLifecycleStartupFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for RemoteSessionEndpointLifecycleStartupFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl RemoteSessionEndpointLifecycleStartupFailure {
    fn new(
        authority_owner: Box<ReachabilityAuthorityRuntimeOwner>,
        error: RemoteSessionEndpointLifecycleStartupError,
    ) -> Self {
        Self {
            authority_owner,
            error,
        }
    }

    /// Returns the stable bounded startup failure without exposing authority internals.
    #[must_use]
    pub const fn error(&self) -> RemoteSessionEndpointLifecycleStartupError {
        self.error
    }

    /// Recovers the exact admitted reachability-authority owner after failed startup.
    #[must_use]
    pub fn into_authority_owner(self) -> ReachabilityAuthorityRuntimeOwner {
        *self.authority_owner
    }
}

struct RemoteSessionSupervisorShutdownState {
    requested: AtomicBool,
    wake: Notify,
}

/// Explicit non-cloneable authority for requesting orderly shutdown of one remote supervisor.
pub struct RemoteSessionSupervisorShutdownController {
    state: Arc<RemoteSessionSupervisorShutdownState>,
}

struct RemoteSessionSupervisorShutdownSignal {
    state: Arc<RemoteSessionSupervisorShutdownState>,
}

fn remote_session_supervisor_shutdown_pair() -> (
    RemoteSessionSupervisorShutdownController,
    RemoteSessionSupervisorShutdownSignal,
) {
    let state = Arc::new(RemoteSessionSupervisorShutdownState {
        requested: AtomicBool::new(false),
        wake: Notify::new(),
    });

    (
        RemoteSessionSupervisorShutdownController {
            state: Arc::clone(&state),
        },
        RemoteSessionSupervisorShutdownSignal { state },
    )
}

impl RemoteSessionSupervisorShutdownController {
    /// Requests orderly shutdown of the paired remote-session supervisor.
    ///
    /// The request is monotonic and idempotent. This method only makes the paired supervisor future
    /// ready; it does not close the endpoint, cancel workers directly, abort tasks, mutate authority
    /// state, or publish readiness.
    pub fn request_shutdown(&self) {
        self.state.requested.store(true, Ordering::Release);
        self.state.wake.notify_one();
    }
}

impl RemoteSessionSupervisorShutdownSignal {
    async fn into_shutdown(self) {
        while !self.state.requested.load(Ordering::Acquire) {
            self.state.wake.notified().await;
        }
    }
}

/// Agent-owned lifecycle composition for one already-authorized real remote endpoint startup.
pub struct RemoteSessionEndpointLifecycleRuntime {
    executor: RemoteSessionExecutorRuntime,
    transport: AgentRemoteTransportRuntime,
    supervisor_shutdown: RemoteSessionSupervisorShutdownSignal,
}

impl RemoteSessionEndpointLifecycleRuntime {
    /// Constructs the private executor before attempting the existing real remote endpoint bind.
    ///
    /// The admitted reachability-authority owner is retained in the returned failure for both
    /// executor-construction and endpoint-bind failure. Successful startup creates one private
    /// supervisor-shutdown signal and returns its separate non-cloneable controller beside the
    /// lifecycle owner.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSessionEndpointLifecycleStartupFailure`] when executor construction or the
    /// existing fixed-credential/TLS/socket bind transaction fails. No retry, fallback runtime,
    /// alternate bind, or automatic reachability re-bootstrap is attempted.
    pub fn bind_from_systemd_credentials(
        authority_owner: ReachabilityAuthorityRuntimeOwner,
        bind_addr: SocketAddr,
    ) -> Result<
        (Self, RemoteSessionSupervisorShutdownController),
        RemoteSessionEndpointLifecycleStartupFailure,
    > {
        let startup = compose_endpoint_startup(
            authority_owner,
            RemoteSessionExecutorRuntime::new,
            |authority_owner| {
                AgentRemoteTransportRuntime::bind_from_systemd_credentials(
                    authority_owner,
                    bind_addr,
                )
                .map_err(|failure| {
                    let error = failure.error();
                    let authority_owner = failure.into_authority_owner();
                    (Box::new(authority_owner), error)
                })
            },
        );

        let (executor, transport) = match startup {
            Ok(parts) => parts,
            Err((authority_owner, error)) => {
                let error = match error {
                    EndpointStartupCompositionError::Executor(error) => {
                        RemoteSessionEndpointLifecycleStartupError::Executor(error)
                    }
                    EndpointStartupCompositionError::Transport(error) => {
                        RemoteSessionEndpointLifecycleStartupError::Transport(error)
                    }
                };
                return Err(RemoteSessionEndpointLifecycleStartupFailure::new(
                    authority_owner,
                    error,
                ));
            }
        };

        let (shutdown_controller, supervisor_shutdown) = remote_session_supervisor_shutdown_pair();

        Ok((
            Self {
                executor,
                transport,
                supervisor_shutdown,
            },
            shutdown_controller,
        ))
    }

    /// Consumes this startup owner and drives exactly one repeated-admission endpoint lifecycle.
    ///
    /// The stored supervisor-shutdown signal is consumed exactly once. All admission, worker,
    /// shutdown, endpoint-close and idle-drain behavior is delegated to the existing C03e-AN
    /// executor lifecycle; AP does not copy or replace those state machines.
    ///
    /// # Errors
    ///
    /// Returns the existing persistent-collection configuration error unchanged after the C03e-AN
    /// lifecycle has still closed the bound endpoint and driven it idle.
    #[expect(
        clippy::too_many_arguments,
        reason = "C03e-AP forwards the exact existing C03e-AN lifecycle inputs"
    )]
    pub fn drive_repeated_real_remote_admission_endpoint_lifecycle<P, D, T, F, C, R, E>(
        self,
        max_active_workers: NonZeroUsize,
        authority: &SharedCurrentCapabilityAuthority<P>,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        admission_timing: F,
        on_completion: C,
        on_rejection: R,
        on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        C: FnMut(RemoteSessionRegisteredWorkerCompletion),
        R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>),
        E: FnMut(RemoteSessionRepeatedAdmissionFailure),
    {
        let Self {
            mut executor,
            transport,
            supervisor_shutdown,
        } = self;

        executor.drive_repeated_real_remote_admission_endpoint_lifecycle(
            max_active_workers,
            &transport,
            authority,
            session_authentication,
            expected_requests,
            supervisor_shutdown.into_shutdown(),
            admission_timing,
            on_completion,
            on_rejection,
            on_admission_failure,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        future::Future,
        net::SocketAddr,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        task::{Context, Poll, Wake, Waker},
    };

    use super::{
        EndpointStartupCompositionError, RemoteSessionEndpointLifecycleRuntime,
        RemoteSessionEndpointLifecycleStartupFailure, RemoteSessionSupervisorShutdownController,
        compose_endpoint_startup, remote_session_supervisor_shutdown_pair,
    };
    use crate::reachability_authority_admission::ReachabilityAuthorityRuntimeOwner;

    #[derive(Default)]
    struct WakeFlag {
        woken: AtomicBool,
    }

    impl Wake for WakeFlag {
        fn wake(self: Arc<Self>) {
            self.woken.store(true, Ordering::SeqCst);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.woken.store(true, Ordering::SeqCst);
        }
    }

    fn test_context() -> (Arc<WakeFlag>, Waker) {
        let flag = Arc::new(WakeFlag::default());
        let waker = Waker::from(Arc::clone(&flag));
        (flag, waker)
    }

    fn assert_send_static_shutdown_future<F>(future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        drop(future);
    }

    fn assert_constructor_signature(
        constructor: fn(
            ReachabilityAuthorityRuntimeOwner,
            SocketAddr,
        ) -> Result<
            (
                RemoteSessionEndpointLifecycleRuntime,
                RemoteSessionSupervisorShutdownController,
            ),
            RemoteSessionEndpointLifecycleStartupFailure,
        >,
    ) {
        let _ = constructor;
    }

    #[test]
    fn shutdown_requested_before_poll_completes_from_durable_state() {
        let (controller, signal) = remote_session_supervisor_shutdown_pair();
        controller.request_shutdown();

        let mut future = Box::pin(signal.into_shutdown());
        let (_wake_flag, waker) = test_context();
        let mut context = Context::from_waker(&waker);

        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
    }

    #[test]
    fn pending_shutdown_signal_is_woken_and_then_completes() {
        let (controller, signal) = remote_session_supervisor_shutdown_pair();
        let mut future = Box::pin(signal.into_shutdown());
        let (wake_flag, waker) = test_context();
        let mut context = Context::from_waker(&waker);

        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        assert!(!wake_flag.woken.load(Ordering::SeqCst));

        controller.request_shutdown();

        assert!(wake_flag.woken.load(Ordering::SeqCst));
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
    }

    #[test]
    fn repeated_shutdown_requests_are_idempotent() {
        let (controller, signal) = remote_session_supervisor_shutdown_pair();
        controller.request_shutdown();
        controller.request_shutdown();

        let mut future = Box::pin(signal.into_shutdown());
        let (_wake_flag, waker) = test_context();
        let mut context = Context::from_waker(&waker);

        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
    }

    #[test]
    fn dropping_controller_without_request_leaves_signal_pending() {
        let (controller, signal) = remote_session_supervisor_shutdown_pair();
        let mut future = Box::pin(signal.into_shutdown());
        let (_wake_flag, waker) = test_context();
        let mut context = Context::from_waker(&waker);

        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        drop(controller);
        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    }

    #[test]
    fn shutdown_future_matches_existing_supervisor_bound() {
        let (_controller, signal) = remote_session_supervisor_shutdown_pair();
        assert_send_static_shutdown_future(signal.into_shutdown());
    }

    #[test]
    fn startup_composition_constructs_executor_before_bind() {
        let events = RefCell::new(Vec::<&'static str>::new());

        let result = compose_endpoint_startup(
            7_u8,
            || {
                events.borrow_mut().push("executor");
                Ok::<_, &'static str>(11_u8)
            },
            |authority| {
                events.borrow_mut().push("bind");
                Ok::<_, (Box<u8>, &'static str)>((authority, 13_u8))
            },
        );

        assert_eq!(result, Ok((11_u8, (7_u8, 13_u8))));
        assert_eq!(*events.borrow(), vec!["executor", "bind"]);
    }

    #[test]
    fn executor_failure_retains_authority_and_prevents_bind_attempt() {
        let bind_called = Cell::new(false);

        let result = compose_endpoint_startup(
            17_u8,
            || Err::<u8, _>("executor failed"),
            |authority| {
                bind_called.set(true);
                Ok::<_, (Box<u8>, &'static str)>((authority, 19_u8))
            },
        );

        assert!(!bind_called.get());
        assert_eq!(
            result,
            Err((
                Box::new(17_u8),
                EndpointStartupCompositionError::Executor("executor failed")
            ))
        );
    }

    #[test]
    fn bind_failure_retains_exact_authority_without_retry() {
        let bind_calls = Cell::new(0_u8);

        let result = compose_endpoint_startup(
            23_u8,
            || Ok::<_, &'static str>(29_u8),
            |authority| {
                bind_calls.set(bind_calls.get() + 1);
                Err::<(u8, u8), _>((Box::new(authority), "bind failed"))
            },
        );

        assert_eq!(bind_calls.get(), 1);
        assert_eq!(
            result,
            Err((
                Box::new(23_u8),
                EndpointStartupCompositionError::Transport("bind failed")
            ))
        );
    }

    #[test]
    fn production_constructor_has_exact_selected_shape() {
        assert_constructor_signature(
            RemoteSessionEndpointLifecycleRuntime::bind_from_systemd_credentials,
        );
    }
}
