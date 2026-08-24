//! Agent-owned executor custody for the staged remote-session runtime path.
//!
//! C03e-T selected the narrow first executor shape: one explicit non-cloneable Tokio
//! current-thread runtime owner. C03e-U materializes that construction/custody boundary, C03e-V
//! adds one borrowed domain-specific drive seam for the existing C03e-S worker body, C03e-AB
//! adds one lexically-contained spawned-and-joined worker seam, and C03e-AF adds one bounded
//! current-thread single-worker supervisor. It does not bind remote transport, retain persistent
//! workers, publish readiness, or wire the Agent binary.

use std::{
    fmt,
    future::{Future, poll_fn},
    pin::Pin,
    task::Poll,
};

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::CapabilityDispatcher;
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

use super::{
    RemoteSessionWorkerCancellationController, SharedCurrentCapabilityAuthority,
    authenticated_remote_session_runtime::{
        AuthenticatedRemoteSessionRuntimeOwner, AuthenticatedRemoteSessionWorkerStop,
    },
    remote_session_worker_cancellation_pair,
};

/// Failure while constructing the Agent-owned remote-session executor runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteSessionExecutorRuntimeCreateError {
    /// Tokio could not construct the selected current-thread runtime with I/O/time drivers.
    Construction,
}

impl fmt::Display for RemoteSessionExecutorRuntimeCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("failed to construct remote session executor runtime")
    }
}

impl std::error::Error for RemoteSessionExecutorRuntimeCreateError {}

/// Failure while joining the one lexically-contained spawned remote-session worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteSessionSpawnedWorkerJoinError {
    /// Tokio reported abnormal completion for the one local worker task.
    AbnormalTaskCompletion,
}

impl fmt::Display for RemoteSessionSpawnedWorkerJoinError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("spawned remote session worker completed abnormally")
    }
}

impl std::error::Error for RemoteSessionSpawnedWorkerJoinError {}

async fn await_supervised_worker<T, S>(
    mut worker_handle: JoinHandle<T>,
    cancellation_controller: RemoteSessionWorkerCancellationController,
    supervisor_shutdown: S,
) -> Result<T, RemoteSessionSpawnedWorkerJoinError>
where
    S: Future<Output = ()> + Send,
{
    let mut supervisor_shutdown = Box::pin(supervisor_shutdown);

    let completed_worker = poll_fn(|context| {
        if let Poll::Ready(worker_result) = Pin::new(&mut worker_handle).poll(context) {
            return Poll::Ready(Some(worker_result));
        }

        if let Poll::Ready(()) = supervisor_shutdown.as_mut().poll(context) {
            return Poll::Ready(None);
        }

        Poll::Pending
    })
    .await;

    if let Some(worker_result) = completed_worker {
        return worker_result
            .map_err(|_| RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion);
    }

    cancellation_controller.request_cancellation();
    worker_handle
        .await
        .map_err(|_| RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)
}

/// Explicit Agent-owned Tokio custody for remote transport/session async work.
///
/// The raw Tokio runtime remains private. C03e-V exposes the borrowed
/// [`Self::drive_capability_request_worker`] seam, C03e-AB adds the bounded
/// [`Self::drive_spawned_capability_request_worker`] seam, and C03e-AF adds the bounded
/// [`Self::drive_supervised_capability_request_worker`] seam. No generic `block_on`, runtime
/// handle, persistent task handle, network bind, peer acceptance, concurrent session admission,
/// readiness or production activation surface is exposed.
pub struct RemoteSessionExecutorRuntime {
    runtime: Runtime,
}

impl RemoteSessionExecutorRuntime {
    /// Constructs the C03e-T-selected current-thread Tokio runtime with I/O/time drivers enabled.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSessionExecutorRuntimeCreateError::Construction`] when Tokio rejects runtime
    /// construction. No retry, process exit, network bind, task spawn or readiness publication is
    /// performed by this constructor.
    pub fn new() -> Result<Self, RemoteSessionExecutorRuntimeCreateError> {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .map(|runtime| Self { runtime })
            .map_err(|_| RemoteSessionExecutorRuntimeCreateError::Construction)
    }

    /// Drives exactly one borrowed C03e-S remote-session worker body to its terminal stop.
    ///
    /// The executor owner and authenticated-session owner are both mutably borrowed for the whole
    /// synchronous drive call. The existing C03e-S worker remains the sole authority for the race
    /// between the C03e-Q request loop and caller-supplied cancellation, including its existing
    /// code-3 failure close, code-4 cancellation close and exact terminal classification.
    ///
    /// This seam intentionally uses the private Tokio runtime only through one internal `block_on`
    /// call. It does not expose a generic future-driving API, spawn a task, construct a cancellation
    /// controller, clone a runtime handle, retain a join handle, admit a second session, bind remote
    /// transport, wire `main.rs`, or publish readiness.
    pub fn drive_capability_request_worker<
        P: PolicyEvaluator + Send + Sync,
        D: CapabilityDispatcher + Send,
        T: FnMut() -> u64 + Send,
        C: Future<Output = ()> + Send,
    >(
        &mut self,
        session_owner: &mut AuthenticatedRemoteSessionRuntimeOwner,
        authority: &SharedCurrentCapabilityAuthority<P>,
        verifier_time_unix_seconds: T,
        dispatcher: &mut D,
        cancellation: C,
    ) -> AuthenticatedRemoteSessionWorkerStop {
        self.runtime
            .block_on(session_owner.run_capability_request_worker(
                authority,
                verifier_time_unix_seconds,
                dispatcher,
                cancellation,
            ))
    }

    /// Spawns and joins exactly one owned remote-session worker under this private runtime.
    ///
    /// The authenticated-session owner, dispatcher, verifier-time provider and cancellation future
    /// move into one `async move` task. Exactly one clone of the shared-current authority moves into
    /// the same task. The one local join handle is awaited before this bounded drive returns; it is
    /// never returned, stored, detached, aborted or inserted into a collection.
    ///
    /// The task delegates to the existing C03e-S/Z worker body, preserving fresh shared-current
    /// authorization and the existing request-loop/cancellation ordering. The executor remains
    /// mutably borrowed for the entire synchronous drive, so this seam does not admit a second
    /// concurrent worker through the same owner.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`] if Tokio reports an
    /// abnormal join result. Raw Tokio join errors, panic payloads and task/runtime identifiers are
    /// not exposed. No retry, replacement task or replacement session is attempted.
    pub fn drive_spawned_capability_request_worker<
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        C: Future<Output = ()> + Send + 'static,
    >(
        &mut self,
        session_owner: AuthenticatedRemoteSessionRuntimeOwner,
        authority: &SharedCurrentCapabilityAuthority<P>,
        verifier_time_unix_seconds: T,
        dispatcher: D,
        cancellation: C,
    ) -> Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError> {
        let authority = (*authority).clone();

        self.runtime.block_on(async move {
            let worker_handle = tokio::spawn(async move {
                let mut session_owner = session_owner;
                let mut dispatcher = dispatcher;

                session_owner
                    .run_capability_request_worker(
                        &authority,
                        verifier_time_unix_seconds,
                        &mut dispatcher,
                        cancellation,
                    )
                    .await
            });

            worker_handle
                .await
                .map_err(|_| RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)
        })
    }

    /// Supervises exactly one spawned remote-session worker under this private current-thread
    /// runtime until worker completion or orderly supervisor shutdown.
    ///
    /// The supervisor creates one C03e-AD cancellation pair and retains the controller beside the
    /// one local worker join handle. The paired signal is moved into the worker. Worker completion
    /// is polled before supervisor shutdown on every race wake. If shutdown wins while the worker is
    /// still pending, the supervisor requests cancellation once and then continues driving the same
    /// worker handle to terminal completion before returning.
    ///
    /// Supervisor shutdown readiness does not fabricate a cancelled worker result. The existing
    /// C03e-S worker remains authoritative for whether orderly cancellation or a real request-loop
    /// failure wins, including its existing code-4/code-3 peer-close behavior.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`] only when Tokio
    /// reports abnormal completion for the one supervised worker task. No raw join error, task ID,
    /// panic payload or runtime identity is exposed.
    pub fn drive_supervised_capability_request_worker<
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        S: Future<Output = ()> + Send,
    >(
        &mut self,
        session_owner: AuthenticatedRemoteSessionRuntimeOwner,
        authority: &SharedCurrentCapabilityAuthority<P>,
        verifier_time_unix_seconds: T,
        dispatcher: D,
        supervisor_shutdown: S,
    ) -> Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError> {
        let authority = (*authority).clone();

        self.runtime.block_on(async move {
            let (cancellation_controller, cancellation_signal) =
                remote_session_worker_cancellation_pair();

            let worker_handle = tokio::spawn(async move {
                let mut session_owner = session_owner;
                let mut dispatcher = dispatcher;

                session_owner
                    .run_capability_request_worker(
                        &authority,
                        verifier_time_unix_seconds,
                        &mut dispatcher,
                        cancellation_signal.into_cancelled(),
                    )
                    .await
            });

            await_supervised_worker(worker_handle, cancellation_controller, supervisor_shutdown)
                .await
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        future::{Future, pending, ready},
        sync::Arc,
        task::{Context, Poll, Wake, Waker},
    };

    use tokio::runtime::{Builder, Runtime};

    use super::{
        AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionExecutorRuntime,
        RemoteSessionExecutorRuntimeCreateError, RemoteSessionSpawnedWorkerJoinError,
        await_supervised_worker, remote_session_worker_cancellation_pair,
    };

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    fn assert_constructor_signature(
        constructor: fn() -> Result<
            RemoteSessionExecutorRuntime,
            RemoteSessionExecutorRuntimeCreateError,
        >,
    ) {
        let _ = constructor;
    }

    fn assert_send_static<T: Send + 'static>() {}

    fn current_thread_runtime() -> Runtime {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("current-thread runtime constructs")
    }

    #[test]
    fn executor_owner_constructor_has_exact_fallible_shape() {
        assert_constructor_signature(RemoteSessionExecutorRuntime::new);
    }

    #[test]
    fn selected_current_thread_runtime_constructs_without_running_remote_work() {
        let runtime =
            RemoteSessionExecutorRuntime::new().expect("current-thread runtime constructs");
        drop(runtime);
    }

    #[test]
    fn authenticated_remote_session_owner_is_spawn_compatible() {
        assert_send_static::<AuthenticatedRemoteSessionRuntimeOwner>();
    }

    #[test]
    fn supervisor_shutdown_cancels_pending_worker_and_joins_same_handle() {
        let runtime = current_thread_runtime();

        let result = runtime.block_on(async {
            let (controller, signal) = remote_session_worker_cancellation_pair();
            let worker_handle = tokio::spawn(async move {
                signal.into_cancelled().await;
                7_u8
            });

            await_supervised_worker(worker_handle, controller, ready(())).await
        });

        assert_eq!(result, Ok(7_u8));
    }

    #[test]
    fn completed_worker_wins_while_supervisor_shutdown_remains_pending() {
        let runtime = current_thread_runtime();

        let result = runtime.block_on(async {
            let (controller, _signal) = remote_session_worker_cancellation_pair();
            let worker_handle = tokio::spawn(async { 11_u8 });

            await_supervised_worker(worker_handle, controller, pending()).await
        });

        assert_eq!(result, Ok(11_u8));
    }

    #[test]
    fn already_completed_worker_wins_same_poll_tie_without_requesting_cancellation() {
        let runtime = current_thread_runtime();

        let (result, signal) = runtime.block_on(async {
            let (controller, signal) = remote_session_worker_cancellation_pair();
            let worker_handle = tokio::spawn(async { 13_u8 });
            tokio::task::yield_now().await;

            let result = await_supervised_worker(worker_handle, controller, ready(())).await;
            (result, signal)
        });

        assert_eq!(result, Ok(13_u8));

        let mut cancellation = Box::pin(signal.into_cancelled());
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        assert_eq!(cancellation.as_mut().poll(&mut context), Poll::Pending);
    }

    #[test]
    fn abnormal_supervised_worker_completion_maps_to_bounded_join_error() {
        let runtime = current_thread_runtime();

        let result = runtime.block_on(async {
            let (controller, _signal) = remote_session_worker_cancellation_pair();
            let worker_handle = tokio::spawn(async {
                panic!("intentional supervised-worker test panic");
            });

            await_supervised_worker(worker_handle, controller, pending()).await
        });

        assert_eq!(
            result,
            Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)
        );
    }
}
