//! Agent-owned executor custody for the staged remote-session runtime path.
//!
//! C03e-T selected the narrow first executor shape: one explicit non-cloneable Tokio
//! current-thread runtime owner. C03e-U materializes that construction/custody boundary, C03e-V
//! adds one borrowed domain-specific drive seam for the existing C03e-S worker body, C03e-AB
//! adds one lexically-contained spawned-and-joined worker seam, C03e-AF adds one bounded
//! current-thread single-worker supervisor, and C03e-AH adds the first pre-listener persistent
//! current-thread worker collection. It does not bind remote transport, publish readiness, or wire
//! the Agent binary.

use std::{
    collections::{HashMap, hash_map::Entry as HashMapEntry},
    fmt,
    future::{Future, poll_fn},
    hash::Hash,
    num::NonZeroUsize,
    pin::Pin,
    task::{Context, Poll},
};

use prw_core::DeviceId;
use prw_policy::PolicyEvaluator;
use prw_registry::MAX_REGISTERED_DEVICES;
use prw_remote_bridge::CapabilityDispatcher;
use tokio::{
    runtime::{Builder, Runtime},
    sync::mpsc,
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

/// Invalid configuration for the bounded persistent remote-worker collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteSessionPersistentCollectionConfigError {
    /// The configured active-worker bound exceeds the existing registered-device ceiling.
    CapacityExceedsRegisteredDeviceLimit,
}

impl fmt::Display for RemoteSessionPersistentCollectionConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("remote session worker capacity exceeds registered device limit")
    }
}

impl std::error::Error for RemoteSessionPersistentCollectionConfigError {}

/// One already-authenticated candidate for the pre-listener persistent worker collection.
pub struct RemoteSessionWorkerAdmission<D, T> {
    session_owner: AuthenticatedRemoteSessionRuntimeOwner,
    dispatcher: D,
    verifier_time_unix_seconds: T,
}

impl<D, T> RemoteSessionWorkerAdmission<D, T> {
    /// Creates one ownership-only admission item with no caller-supplied logical identity.
    #[must_use]
    pub const fn new(
        session_owner: AuthenticatedRemoteSessionRuntimeOwner,
        dispatcher: D,
        verifier_time_unix_seconds: T,
    ) -> Self {
        Self {
            session_owner,
            dispatcher,
            verifier_time_unix_seconds,
        }
    }

    fn logical_device_id(&self) -> &DeviceId {
        self.session_owner.logical_device_id()
    }

    /// Recovers the untouched owned candidate parts for explicit caller cleanup or custody.
    #[must_use]
    pub fn into_parts(self) -> (AuthenticatedRemoteSessionRuntimeOwner, D, T) {
        (
            self.session_owner,
            self.dispatcher,
            self.verifier_time_unix_seconds,
        )
    }
}

/// Bounded reason that an already-authenticated worker candidate was not admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteSessionWorkerAdmissionRejectionReason {
    /// One active worker already owns the same authenticated logical DeviceId key.
    DuplicateActiveDevice,
}

/// Owns one rejected already-authenticated candidate together with its bounded reason.
pub struct RemoteSessionWorkerAdmissionRejection<D, T> {
    reason: RemoteSessionWorkerAdmissionRejectionReason,
    admission: RemoteSessionWorkerAdmission<D, T>,
}

impl<D, T> RemoteSessionWorkerAdmissionRejection<D, T> {
    /// Returns the bounded rejection reason.
    #[must_use]
    pub const fn reason(&self) -> RemoteSessionWorkerAdmissionRejectionReason {
        self.reason
    }

    /// Returns the untouched rejected admission item.
    #[must_use]
    pub const fn admission(&self) -> &RemoteSessionWorkerAdmission<D, T> {
        &self.admission
    }

    /// Returns ownership of the untouched rejected admission item.
    #[must_use]
    pub fn into_admission(self) -> RemoteSessionWorkerAdmission<D, T> {
        self.admission
    }
}

/// One explicitly reaped persistent-worker completion associated with its logical DeviceId.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteSessionRegisteredWorkerCompletion {
    device_id: DeviceId,
    result: Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>,
}

impl RemoteSessionRegisteredWorkerCompletion {
    /// Returns the authenticated logical DeviceId that keyed the retained worker entry.
    #[must_use]
    pub const fn device_id(&self) -> &DeviceId {
        &self.device_id
    }

    /// Returns the existing bounded worker/join terminal result.
    #[must_use]
    pub const fn result(
        &self,
    ) -> Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError> {
        self.result
    }
}

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

        if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
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

struct RemoteSessionPersistentWorkerEntry<T> {
    cancellation_controller: RemoteSessionWorkerCancellationController,
    worker_handle: JoinHandle<T>,
}

enum RemoteSessionPersistentSupervisorEvent<C> {
    Shutdown,
    Admission(C),
}

fn validate_persistent_worker_capacity(
    max_active_workers: NonZeroUsize,
) -> Result<usize, RemoteSessionPersistentCollectionConfigError> {
    let max_active_workers = max_active_workers.get();
    if max_active_workers > MAX_REGISTERED_DEVICES {
        return Err(
            RemoteSessionPersistentCollectionConfigError::CapacityExceedsRegisteredDeviceLimit,
        );
    }
    Ok(max_active_workers)
}

fn map_worker_join_result<T>(
    result: Result<T, tokio::task::JoinError>,
) -> Result<T, RemoteSessionSpawnedWorkerJoinError> {
    result.map_err(|_| RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)
}

fn reap_ready_persistent_workers<K, T, C>(
    active: &mut HashMap<K, RemoteSessionPersistentWorkerEntry<T>>,
    context: &mut Context<'_>,
    on_completion: &mut C,
) where
    K: Eq + Hash + Clone,
    C: FnMut(K, Result<T, RemoteSessionSpawnedWorkerJoinError>),
{
    active.retain(
        |key, entry| match Pin::new(&mut entry.worker_handle).poll(context) {
            Poll::Ready(result) => {
                on_completion(key.clone(), map_worker_join_result(result));
                false
            }
            Poll::Pending => true,
        },
    );
}

async fn run_persistent_worker_collection<K, Candidate, T, S, Kf, Sf, Cf, Rf>(
    max_active_workers: usize,
    mut admissions: mpsc::Receiver<Candidate>,
    supervisor_shutdown: S,
    mut key_for_candidate: Kf,
    mut spawn_worker: Sf,
    mut on_completion: Cf,
    mut on_rejection: Rf,
) where
    K: Eq + Hash + Clone,
    S: Future<Output = ()>,
    Kf: FnMut(&Candidate) -> K,
    Sf: FnMut(Candidate) -> (RemoteSessionWorkerCancellationController, JoinHandle<T>),
    Cf: FnMut(K, Result<T, RemoteSessionSpawnedWorkerJoinError>),
    Rf: FnMut(RemoteSessionWorkerAdmissionRejectionReason, Candidate),
{
    let mut active = HashMap::<K, RemoteSessionPersistentWorkerEntry<T>>::new();
    let mut supervisor_shutdown = Box::pin(supervisor_shutdown);
    let mut admission_open = true;

    loop {
        let event = poll_fn(|context| {
            reap_ready_persistent_workers(&mut active, context, &mut on_completion);

            if supervisor_shutdown.as_mut().poll(context) == Poll::Ready(()) {
                return Poll::Ready(RemoteSessionPersistentSupervisorEvent::Shutdown);
            }

            if admission_open && active.len() < max_active_workers {
                match Pin::new(&mut admissions).poll_recv(context) {
                    Poll::Ready(Some(candidate)) => {
                        return Poll::Ready(RemoteSessionPersistentSupervisorEvent::Admission(
                            candidate,
                        ));
                    }
                    Poll::Ready(None) => admission_open = false,
                    Poll::Pending => {}
                }
            }

            Poll::Pending
        })
        .await;

        match event {
            RemoteSessionPersistentSupervisorEvent::Shutdown => break,
            RemoteSessionPersistentSupervisorEvent::Admission(candidate) => {
                let key = key_for_candidate(&candidate);
                match active.entry(key) {
                    HashMapEntry::Occupied(_) => on_rejection(
                        RemoteSessionWorkerAdmissionRejectionReason::DuplicateActiveDevice,
                        candidate,
                    ),
                    HashMapEntry::Vacant(slot) => {
                        let (cancellation_controller, worker_handle) = spawn_worker(candidate);
                        slot.insert(RemoteSessionPersistentWorkerEntry {
                            cancellation_controller,
                            worker_handle,
                        });
                    }
                }
            }
        }
    }

    for entry in active.values() {
        entry.cancellation_controller.request_cancellation();
    }

    poll_fn(|context| {
        reap_ready_persistent_workers(&mut active, context, &mut on_completion);
        if active.is_empty() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await;
}

/// Explicit Agent-owned Tokio custody for remote transport/session async work.
///
/// The raw Tokio runtime remains private. C03e-V exposes the borrowed
/// [`Self::drive_capability_request_worker`] seam, C03e-AB adds the bounded
/// [`Self::drive_spawned_capability_request_worker`] seam, C03e-AF adds the bounded
/// [`Self::drive_supervised_capability_request_worker`] seam, and C03e-AH adds the pre-listener
/// [`Self::drive_persistent_remote_worker_collection`] seam. No generic `block_on`, runtime handle,
/// network bind, peer acceptance/authentication, readiness or production activation surface is
/// exposed.
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

    /// Drives the first bounded persistent remote-worker collection on the private current-thread
    /// runtime using an injected already-authenticated admission source.
    ///
    /// The complete collection lifetime remains inside one private `Runtime::block_on`. Active
    /// workers are keyed only by the DeviceId derived from each authenticated runtime owner. The
    /// injected bounded mpsc receiver is not polled while the collection is at capacity. Ready
    /// worker completions are reaped before shutdown and admission work on every supervisor poll.
    ///
    /// Duplicate DeviceId candidates are rejected before worker spawn and returned intact through
    /// `on_rejection`. Reaped worker results are reported immediately through `on_completion` and
    /// are not accumulated by the supervisor. When orderly shutdown wins, admission stops, every
    /// retained controller is asked to cancel before any remaining handle is drained, and the same
    /// current-thread runtime stays driven until the active map is empty.
    ///
    /// Closing the injected admission source does not cancel active workers or end this method; the
    /// separately supplied supervisor-shutdown future remains authoritative for orderly shutdown.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSessionPersistentCollectionConfigError::CapacityExceedsRegisteredDeviceLimit`]
    /// before runtime work when `max_active_workers` exceeds [`MAX_REGISTERED_DEVICES`].
    pub fn drive_persistent_remote_worker_collection<
        P: PolicyEvaluator + Send + Sync + 'static,
        D: CapabilityDispatcher + Send + 'static,
        T: FnMut() -> u64 + Send + 'static,
        S: Future<Output = ()> + Send,
        C: FnMut(RemoteSessionRegisteredWorkerCompletion),
        R: FnMut(RemoteSessionWorkerAdmissionRejection<D, T>),
    >(
        &mut self,
        max_active_workers: NonZeroUsize,
        authority: &SharedCurrentCapabilityAuthority<P>,
        admissions: mpsc::Receiver<RemoteSessionWorkerAdmission<D, T>>,
        supervisor_shutdown: S,
        mut on_completion: C,
        mut on_rejection: R,
    ) -> Result<(), RemoteSessionPersistentCollectionConfigError> {
        let max_active_workers = validate_persistent_worker_capacity(max_active_workers)?;

        self.runtime.block_on(run_persistent_worker_collection(
            max_active_workers,
            admissions,
            supervisor_shutdown,
            |admission| admission.logical_device_id().clone(),
            |admission| {
                let authority = (*authority).clone();
                let (mut session_owner, mut dispatcher, verifier_time_unix_seconds) =
                    admission.into_parts();
                let (cancellation_controller, cancellation_signal) =
                    remote_session_worker_cancellation_pair();
                let worker_handle = tokio::spawn(async move {
                    session_owner
                        .run_capability_request_worker(
                            &authority,
                            verifier_time_unix_seconds,
                            &mut dispatcher,
                            cancellation_signal.into_cancelled(),
                        )
                        .await
                });
                (cancellation_controller, worker_handle)
            },
            |device_id, result| {
                on_completion(RemoteSessionRegisteredWorkerCompletion { device_id, result });
            },
            |reason, admission| {
                on_rejection(RemoteSessionWorkerAdmissionRejection { reason, admission });
            },
        ));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        future::{Future, pending, ready},
        num::NonZeroUsize,
        task::{Context, Poll, Waker},
    };

    use prw_registry::MAX_REGISTERED_DEVICES;
    use tokio::{
        runtime::{Builder, Runtime},
        sync::{mpsc, oneshot},
    };

    use super::{
        AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionExecutorRuntime,
        RemoteSessionExecutorRuntimeCreateError, RemoteSessionPersistentCollectionConfigError,
        RemoteSessionSpawnedWorkerJoinError, RemoteSessionWorkerAdmissionRejectionReason,
        await_supervised_worker, remote_session_worker_cancellation_pair,
        run_persistent_worker_collection, validate_persistent_worker_capacity,
    };

    #[derive(Debug)]
    struct TestAdmission {
        key: u8,
        value: u8,
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
        let mut context = Context::from_waker(Waker::noop());
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

    #[test]
    fn persistent_collection_rejects_duplicate_before_second_spawn() {
        let runtime = current_thread_runtime();

        let (spawn_count, rejected, completions) = runtime.block_on(async {
            let (sender, receiver) = mpsc::channel(4);
            assert!(sender.try_send(TestAdmission { key: 1, value: 10 }).is_ok());
            assert!(sender.try_send(TestAdmission { key: 1, value: 11 }).is_ok());
            drop(sender);

            let (shutdown_sender, shutdown_receiver) = oneshot::channel();
            let mut shutdown_sender = Some(shutdown_sender);
            let mut spawn_count = 0_usize;
            let mut rejected = Vec::new();
            let mut completions = Vec::new();

            run_persistent_worker_collection(
                2,
                receiver,
                async move {
                    let _ = shutdown_receiver.await;
                },
                |candidate: &TestAdmission| candidate.key,
                |candidate| {
                    spawn_count += 1;
                    let (controller, signal) = remote_session_worker_cancellation_pair();
                    let handle = tokio::spawn(async move {
                        signal.into_cancelled().await;
                        candidate.value
                    });
                    (controller, handle)
                },
                |key, result| completions.push((key, result)),
                |reason, candidate| {
                    rejected.push((reason, candidate.value));
                    if let Some(sender) = shutdown_sender.take() {
                        let _ = sender.send(());
                    }
                },
            )
            .await;

            (spawn_count, rejected, completions)
        });

        assert_eq!(spawn_count, 1);
        assert_eq!(
            rejected,
            vec![(
                RemoteSessionWorkerAdmissionRejectionReason::DuplicateActiveDevice,
                11
            )]
        );
        assert_eq!(completions, vec![(1, Ok(10))]);
    }

    #[test]
    fn persistent_collection_reaps_completed_key_before_same_device_readmission() {
        let runtime = current_thread_runtime();

        let (spawn_count, rejection_count, completions) = runtime.block_on(async {
            let (sender, receiver) = mpsc::channel(4);
            assert!(sender.try_send(TestAdmission { key: 2, value: 20 }).is_ok());
            assert!(sender.try_send(TestAdmission { key: 2, value: 21 }).is_ok());
            drop(sender);

            let (shutdown_sender, shutdown_receiver) = oneshot::channel();
            let mut shutdown_sender = Some(shutdown_sender);
            let mut spawn_count = 0_usize;
            let mut rejection_count = 0_usize;
            let mut completions = Vec::new();

            run_persistent_worker_collection(
                1,
                receiver,
                async move {
                    let _ = shutdown_receiver.await;
                },
                |candidate: &TestAdmission| candidate.key,
                |candidate| {
                    spawn_count += 1;
                    let (controller, signal) = remote_session_worker_cancellation_pair();
                    let handle = tokio::spawn(async move {
                        drop(signal);
                        candidate.value
                    });
                    (controller, handle)
                },
                |key, result| {
                    completions.push((key, result));
                    if completions.len() == 2
                        && let Some(sender) = shutdown_sender.take()
                    {
                        let _ = sender.send(());
                    }
                },
                |_reason, _candidate| rejection_count += 1,
            )
            .await;

            (spawn_count, rejection_count, completions)
        });

        assert_eq!(spawn_count, 2);
        assert_eq!(rejection_count, 0);
        assert_eq!(completions, vec![(2, Ok(20)), (2, Ok(21))]);
    }

    #[test]
    fn persistent_collection_shutdown_wins_before_prequeued_admission() {
        let runtime = current_thread_runtime();

        let (spawn_count, rejection_count, completion_count) = runtime.block_on(async {
            let (sender, receiver) = mpsc::channel(1);
            assert!(sender.try_send(TestAdmission { key: 3, value: 30 }).is_ok());
            drop(sender);

            let mut spawn_count = 0_usize;
            let mut rejection_count = 0_usize;
            let mut completion_count = 0_usize;

            run_persistent_worker_collection(
                1,
                receiver,
                ready(()),
                |candidate: &TestAdmission| candidate.key,
                |candidate| {
                    spawn_count += 1;
                    let (controller, signal) = remote_session_worker_cancellation_pair();
                    let handle = tokio::spawn(async move {
                        signal.into_cancelled().await;
                        candidate.value
                    });
                    (controller, handle)
                },
                |_key, _result| completion_count += 1,
                |_reason, _candidate| rejection_count += 1,
            )
            .await;

            (spawn_count, rejection_count, completion_count)
        });

        assert_eq!(spawn_count, 0);
        assert_eq!(rejection_count, 0);
        assert_eq!(completion_count, 0);
    }

    #[test]
    fn persistent_collection_shutdown_cancels_and_drains_all_retained_workers() {
        let runtime = current_thread_runtime();

        let (spawn_count, rejection_count, mut completions) = runtime.block_on(async {
            let (sender, receiver) = mpsc::channel(4);
            assert!(sender.try_send(TestAdmission { key: 4, value: 40 }).is_ok());
            assert!(sender.try_send(TestAdmission { key: 5, value: 50 }).is_ok());
            drop(sender);

            let (shutdown_sender, shutdown_receiver) = oneshot::channel();
            let mut shutdown_sender = Some(shutdown_sender);
            let mut spawn_count = 0_usize;
            let mut rejection_count = 0_usize;
            let mut completions = Vec::new();

            run_persistent_worker_collection(
                2,
                receiver,
                async move {
                    let _ = shutdown_receiver.await;
                },
                |candidate: &TestAdmission| candidate.key,
                |candidate| {
                    spawn_count += 1;
                    let (controller, signal) = remote_session_worker_cancellation_pair();
                    let handle = tokio::spawn(async move {
                        signal.into_cancelled().await;
                        candidate.value
                    });
                    if spawn_count == 2
                        && let Some(sender) = shutdown_sender.take()
                    {
                        let _ = sender.send(());
                    }
                    (controller, handle)
                },
                |key, result| completions.push((key, result)),
                |_reason, _candidate| rejection_count += 1,
            )
            .await;

            (spawn_count, rejection_count, completions)
        });

        completions.sort_by_key(|(key, _)| *key);
        assert_eq!(spawn_count, 2);
        assert_eq!(rejection_count, 0);
        assert_eq!(completions, vec![(4, Ok(40)), (5, Ok(50))]);
    }

    #[test]
    fn closed_admission_source_does_not_end_persistent_supervisor() {
        let runtime = current_thread_runtime();

        runtime.block_on(async {
            let (sender, receiver) = mpsc::channel::<TestAdmission>(1);
            drop(sender);
            let (shutdown_sender, shutdown_receiver) = oneshot::channel();

            let supervisor = tokio::spawn(run_persistent_worker_collection(
                1,
                receiver,
                async move {
                    let _ = shutdown_receiver.await;
                },
                |candidate: &TestAdmission| candidate.key,
                |candidate| {
                    let (controller, signal) = remote_session_worker_cancellation_pair();
                    let handle = tokio::spawn(async move {
                        signal.into_cancelled().await;
                        candidate.value
                    });
                    (controller, handle)
                },
                |_key, _result| {},
                |_reason, _candidate| {},
            ));

            tokio::task::yield_now().await;
            assert!(!supervisor.is_finished());
            assert!(shutdown_sender.send(()).is_ok());
            assert!(supervisor.await.is_ok());
        });
    }

    #[test]
    fn persistent_collection_capacity_rejects_only_values_above_registry_ceiling() {
        let maximum = NonZeroUsize::new(MAX_REGISTERED_DEVICES).expect("nonzero registry limit");
        assert_eq!(
            validate_persistent_worker_capacity(maximum),
            Ok(MAX_REGISTERED_DEVICES)
        );

        let above_maximum =
            NonZeroUsize::new(MAX_REGISTERED_DEVICES + 1).expect("nonzero larger limit");
        assert_eq!(
            validate_persistent_worker_capacity(above_maximum),
            Err(RemoteSessionPersistentCollectionConfigError::CapacityExceedsRegisteredDeviceLimit)
        );
    }
}
