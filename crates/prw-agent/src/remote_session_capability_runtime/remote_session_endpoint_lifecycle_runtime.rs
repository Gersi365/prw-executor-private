//! Agent-owned startup and shutdown-control composition for one remote endpoint lifecycle.
//!
//! C03e-AO selected executor-before-bind startup, recoverable reachability-authority custody on
//! startup failure, one remote-specific explicit supervisor-shutdown pair, and delegation to the
//! existing C03e-AN endpoint lifecycle. C03e-AP materializes only those source seams. C03e-AR adds
//! the separately selected Agent-internal path that consumes an already-created executor before
//! the same existing endpoint bind. C03e-BB adds only the BA-selected read-only observation of the
//! exact local address reported by that already-bound retained endpoint. This module does not wire
//! Agent `main.rs`, publish readiness, consume process signals, retry startup, or activate an
//! endpoint from an executable path.

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

use super::requester_rendezvous_retained_custody_dr_continuation::{
    RequesterRendezvousPostTerminalResponseSerialLifecycleError,
    RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
};
use super::{
    RemoteSessionExecutorRuntime, RemoteSessionExecutorRuntimeCreateError,
    RemoteSessionExpectedDeviceAdmissionRejection,
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest, RemoteSessionPersistentCollectionConfigError,
    RemoteSessionRealAdmissionError, RemoteSessionRealAdmissionTiming,
    RemoteSessionRegisteredWorkerCompletion, RemoteSessionRepeatedAdmissionFailure,
    RemoteSessionSpawnedWorkerJoinError, SharedCurrentCapabilityAuthority,
    SharedRequesterRendezvousAuthority,
};
use crate::{
    candidate_publication_requester_rendezvous_start_intent::policy_source::RequesterRendezvousStartPolicySource,
    production_durable_registry_runtime_custody::ProductionDurableCapabilityAuthority,
    reachability_authority_admission::ReachabilityAuthorityRuntimeOwner,
    remote_transport_runtime::{AgentRemoteTransportBindError, AgentRemoteTransportRuntime},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndpointStartupCompositionError<ExecutorError, TransportError> {
    Executor(ExecutorError),
    Transport(TransportError),
}

type EndpointStartupCompositionFailure<Authority, ExecutorError, TransportError> = (
    Box<Authority>,
    EndpointStartupCompositionError<ExecutorError, TransportError>,
);

type EndpointStartupCompositionResult<
    Authority,
    Executor,
    Transport,
    ExecutorError,
    TransportError,
> = Result<
    (Executor, Transport),
    EndpointStartupCompositionFailure<Authority, ExecutorError, TransportError>,
>;

type EndpointBindCompositionResult<Authority, Executor, Transport, TransportError> =
    Result<(Executor, Transport), (Box<Authority>, TransportError)>;

fn compose_endpoint_bind_with_executor<Authority, Executor, Transport, TransportError>(
    executor: Executor,
    authority: Authority,
    bind_transport: impl FnOnce(Authority) -> Result<Transport, (Box<Authority>, TransportError)>,
) -> EndpointBindCompositionResult<Authority, Executor, Transport, TransportError> {
    let transport = bind_transport(authority)?;
    Ok((executor, transport))
}

fn compose_endpoint_startup<Authority, Executor, Transport, ExecutorError, TransportError>(
    authority: Authority,
    construct_executor: impl FnOnce() -> Result<Executor, ExecutorError>,
    bind_transport: impl FnOnce(Authority) -> Result<Transport, (Box<Authority>, TransportError)>,
) -> EndpointStartupCompositionResult<Authority, Executor, Transport, ExecutorError, TransportError>
{
    let executor = match construct_executor() {
        Ok(executor) => executor,
        Err(error) => {
            return Err((
                Box::new(authority),
                EndpointStartupCompositionError::Executor(error),
            ));
        }
    };

    compose_endpoint_bind_with_executor(executor, authority, bind_transport).map_err(
        |(authority, error)| (authority, EndpointStartupCompositionError::Transport(error)),
    )
}

fn map_bound_addr_observation<E>(
    observation: Result<SocketAddr, E>,
) -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError> {
    observation.map_err(|_| RemoteSessionEndpointBoundAddressError::Unavailable)
}

/// Stable failure class while observing the exact local address of one already-bound endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteSessionEndpointBoundAddressError {
    /// The retained lower transport could not report its already-bound local address.
    Unavailable,
}

impl fmt::Display for RemoteSessionEndpointBoundAddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable => formatter.write_str("remote endpoint bound address unavailable"),
        }
    }
}

impl std::error::Error for RemoteSessionEndpointBoundAddressError {}

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

/// Bounded crate-visible terminal family for one requester-aware endpoint worker completion.
#[allow(
    dead_code,
    reason = "C03e-LP materializes the LO-reselected completion projection before separately gated higher-owner caller migration"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection {
    /// Caller-owned cancellation won at one selected requester lifecycle cancellation boundary.
    Cancelled,
    /// The existing requester-aware serial lifecycle stopped on one ingress failure.
    IngressFailure,
    /// The existing requester-aware serial lifecycle stopped on one requester response failure.
    RequesterResponseFailure,
    /// Tokio reported abnormal completion for the retained requester-aware worker task.
    AbnormalTaskCompletion,
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
    const fn new(
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

    /// Binds the existing real remote endpoint using an already-created private executor.
    ///
    /// This Agent-internal seam consumes the exact supplied executor and admitted reachability
    /// authority. It attempts the existing fixed-credential/TLS/socket bind once and creates the
    /// existing supervisor-shutdown pair only after bind succeeds. It never constructs a replacement
    /// executor.
    ///
    /// # Errors
    ///
    /// Returns the existing AP startup-failure owner with the exact reachability authority retained
    /// and the existing transport failure classification. No retry, alternate bind, executor
    /// replacement, reachability re-bootstrap or readiness publication is performed.
    #[allow(
        dead_code,
        reason = "C03e-AR materializes the AQ-selected source seam for a separately gated process consumer"
    )]
    pub(crate) fn bind_with_executor_from_systemd_credentials(
        executor: RemoteSessionExecutorRuntime,
        authority_owner: ReachabilityAuthorityRuntimeOwner,
        bind_addr: SocketAddr,
    ) -> Result<
        (Self, RemoteSessionSupervisorShutdownController),
        RemoteSessionEndpointLifecycleStartupFailure,
    > {
        let startup =
            compose_endpoint_bind_with_executor(executor, authority_owner, |authority_owner| {
                AgentRemoteTransportRuntime::bind_from_systemd_credentials(
                    authority_owner,
                    bind_addr,
                )
                .map_err(|failure| {
                    let error = failure.error();
                    let authority_owner = failure.into_authority_owner();
                    (Box::new(authority_owner), error)
                })
            });

        let (executor, transport) = match startup {
            Ok(parts) => parts,
            Err((authority_owner, error)) => {
                return Err(RemoteSessionEndpointLifecycleStartupFailure::new(
                    authority_owner,
                    RemoteSessionEndpointLifecycleStartupError::Transport(error),
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

    /// Returns the exact local socket address reported by the retained already-bound endpoint.
    ///
    /// This is a synchronous read-only observation. It does not use the original bind input as an
    /// authoritative substitute, create a connectivity candidate, publish reachability, retry,
    /// rebind, close the endpoint, request shutdown, or mutate lifecycle ownership.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSessionEndpointBoundAddressError::Unavailable`] when the existing retained
    /// lower transport cannot report its already-bound local address.
    pub fn bound_addr(&self) -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError> {
        map_bound_addr_observation(self.transport.local_addr())
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

    /// Consumes this startup owner and delegates one dormant production-durable repeated-admission
    /// endpoint lifecycle to the exact C03e-LK executor boundary.
    ///
    /// The retained endpoint transport is borrowed only for that one executor invocation, and the
    /// retained supervisor-shutdown signal is converted exactly once. The distinct requester-DR and
    /// production durable capability authorities are forwarded unchanged. Endpoint close and idle
    /// drain remain owned exclusively by the LK executor method.
    ///
    /// This seam performs no durable-authority bootstrap or population, callback projection,
    /// requester-lifecycle visibility widening, executable caller migration, runtime activation,
    /// endpoint bind, retry, merge, or deployment.
    #[allow(
        dead_code,
        reason = "C03e-LM materializes the LL-selected dormant durable endpoint-owner caller adaptation before separately gated higher-owner projection and production authority population"
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "C03e-LM forwards the exact LL-selected durable executor boundary inputs without introducing a new aggregate"
    )]
    pub(super) fn drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability<
        P,
        D,
        T,
        PS,
        F,
        C,
        R,
        E,
    >(
        self,
        max_active_workers: NonZeroUsize,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
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
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        C: FnMut(
            DeviceId,
            Result<
                RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
                RemoteSessionSpawnedWorkerJoinError,
            >,
        ),
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        let Self {
            mut executor,
            transport,
            supervisor_shutdown,
        } = self;

        executor
            .drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability(
                max_active_workers,
                &transport,
                authority,
                capability_authority,
                policy_source,
                requester_rendezvous_authority,
                session_authentication,
                expected_requests,
                supervisor_shutdown.into_shutdown(),
                admission_timing,
                on_completion,
                on_rejection,
                on_admission_failure,
            )
    }

    /// Consumes this endpoint owner and exposes only the bounded C03e-LO-selected terminal family.
    ///
    /// The existing C03e-LM durable endpoint method remains the sole owner of endpoint/executor
    /// lifecycle behavior. This adapter invokes it exactly once, forwards all non-completion inputs
    /// and callbacks unchanged, and projects requester-private terminal payloads into four bounded
    /// crate-visible families without widening those private types.
    ///
    /// # Errors
    ///
    /// Returns the existing persistent-collection configuration error unchanged.
    #[allow(
        dead_code,
        reason = "C03e-LP materializes the LO-reselected dormant projection adapter before separately gated higher-owner caller migration"
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "C03e-LP preserves the exact C03e-LM durable endpoint inputs while projecting only completion"
    )]
    pub(crate) fn drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection<
        P,
        D,
        T,
        PS,
        F,
        C,
        R,
        E,
    >(
        self,
        max_active_workers: NonZeroUsize,
        authority: &SharedCurrentCapabilityAuthority<P>,
        capability_authority: Arc<ProductionDurableCapabilityAuthority>,
        policy_source: Arc<PS>,
        requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        admission_timing: F,
        mut on_completion: C,
        on_rejection: R,
        on_admission_failure: E,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError>
    where
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
        F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
        C: FnMut(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection),
        R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ),
        E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
    {
        self.drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability(
            max_active_workers,
            authority,
            capability_authority,
            policy_source,
            requester_rendezvous_authority,
            session_authentication,
            expected_requests,
            admission_timing,
            |device_id, completion_result| {
                let projection = match completion_result {
                    Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled) => {
                        RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection::Cancelled
                    }
                    Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(
                        RequesterRendezvousPostTerminalResponseSerialLifecycleError::Ingress(_),
                    )) => {
                        RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection::IngressFailure
                    }
                    Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(
                        RequesterRendezvousPostTerminalResponseSerialLifecycleError::RequesterResponse(_),
                    )) => {
                        RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection::RequesterResponseFailure
                    }
                    Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion) => {
                        RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection::AbnormalTaskCompletion
                    }
                };
                on_completion(device_id, projection);
            },
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
        EndpointStartupCompositionError, RemoteSessionEndpointBoundAddressError,
        RemoteSessionEndpointLifecycleRuntime, RemoteSessionEndpointLifecycleStartupFailure,
        RemoteSessionExecutorRuntime, RemoteSessionSupervisorShutdownController,
        compose_endpoint_bind_with_executor, compose_endpoint_startup, map_bound_addr_observation,
        remote_session_supervisor_shutdown_pair,
    };
    use crate::{
        reachability_authority_admission::ReachabilityAuthorityRuntimeOwner,
        reachability_authority_custody_bootstrap::ReachabilityAuthorityCustodyBootstrapError,
    };

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

    fn assert_bound_addr_signature(
        observation: fn(
            &RemoteSessionEndpointLifecycleRuntime,
        ) -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError>,
    ) {
        let _ = observation;
    }

    #[expect(
        clippy::type_complexity,
        reason = "C03e-AR test intentionally states the exact Agent-internal same-executor constructor shape"
    )]
    fn assert_same_executor_constructor_signature(
        constructor: fn(
            RemoteSessionExecutorRuntime,
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

    fn assert_reachability_bootstrap_signature(
        bootstrap: fn(
            &RemoteSessionExecutorRuntime,
        ) -> Result<
            ReachabilityAuthorityRuntimeOwner,
            ReachabilityAuthorityCustodyBootstrapError,
        >,
    ) {
        let _ = bootstrap;
    }

    #[test]
    fn bound_addr_mapping_preserves_exact_socket_addr() {
        let bound_addr = SocketAddr::from(([127, 0, 0, 1], 43_121));

        assert_eq!(
            map_bound_addr_observation::<()>(Ok(bound_addr)),
            Ok(bound_addr)
        );
    }

    #[test]
    fn bound_addr_mapping_collapses_lower_error_to_unavailable() {
        assert_eq!(
            map_bound_addr_observation::<&'static str>(Err("lower address unavailable")),
            Err(RemoteSessionEndpointBoundAddressError::Unavailable)
        );
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
    fn supplied_executor_is_preserved_through_successful_fake_bind() {
        let bind_calls = Cell::new(0_u8);

        let result = compose_endpoint_bind_with_executor(31_u8, 37_u8, |authority| {
            bind_calls.set(bind_calls.get() + 1);
            Ok::<_, (Box<u8>, &'static str)>((authority, 41_u8))
        });

        assert_eq!(bind_calls.get(), 1);
        assert_eq!(result, Ok((31_u8, (37_u8, 41_u8))));
    }

    #[test]
    fn supplied_executor_bind_failure_retains_exact_authority_without_retry() {
        let bind_calls = Cell::new(0_u8);

        let result = compose_endpoint_bind_with_executor(43_u8, 47_u8, |authority| {
            bind_calls.set(bind_calls.get() + 1);
            Err::<(u8, u8), _>((Box::new(authority), "bind failed"))
        });

        assert_eq!(bind_calls.get(), 1);
        assert_eq!(result, Err((Box::new(47_u8), "bind failed")));
    }

    #[test]
    fn production_constructors_and_bootstrap_have_exact_selected_shapes() {
        assert_constructor_signature(
            RemoteSessionEndpointLifecycleRuntime::bind_from_systemd_credentials,
        );
        assert_same_executor_constructor_signature(
            RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials,
        );
        assert_reachability_bootstrap_signature(
            RemoteSessionExecutorRuntime::bootstrap_reachability_authority_from_systemd_credentials,
        );
        assert_bound_addr_signature(RemoteSessionEndpointLifecycleRuntime::bound_addr);
    }
}
