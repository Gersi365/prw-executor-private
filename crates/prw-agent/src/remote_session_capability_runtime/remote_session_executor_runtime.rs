//! Agent-owned executor custody for the staged remote-session runtime path.
//!
//! C03e-T selected the narrow first executor shape: one explicit non-cloneable Tokio
//! current-thread runtime owner. C03e-U materializes that construction/custody boundary, C03e-V
//! adds one borrowed domain-specific drive seam for the existing C03e-S worker body, and C03e-AB
//! adds one lexically-contained spawned-and-joined worker seam. It does not bind remote transport,
//! retain persistent workers, publish readiness, or wire the Agent binary.

use std::{fmt, future::Future};

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::CapabilityDispatcher;
use tokio::runtime::{Builder, Runtime};

use super::{
    SharedCurrentCapabilityAuthority,
    authenticated_remote_session_runtime::{
        AuthenticatedRemoteSessionRuntimeOwner, AuthenticatedRemoteSessionWorkerStop,
    },
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

/// Explicit Agent-owned Tokio custody for remote transport/session async work.
///
/// The raw Tokio runtime remains private. C03e-V exposes the borrowed
/// [`Self::drive_capability_request_worker`] seam and C03e-AB adds the bounded
/// [`Self::drive_spawned_capability_request_worker`] seam. No generic `block_on`, runtime handle,
/// persistent task handle, network bind, peer acceptance, concurrent session admission,
/// cancellation-controller creation, readiness or production activation surface is exposed.
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
}

#[cfg(test)]
mod tests {
    use super::{
        AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionExecutorRuntime,
        RemoteSessionExecutorRuntimeCreateError,
    };

    fn assert_constructor_signature(
        constructor: fn() -> Result<
            RemoteSessionExecutorRuntime,
            RemoteSessionExecutorRuntimeCreateError,
        >,
    ) {
        let _ = constructor;
    }

    fn assert_send_static<T: Send + 'static>() {}

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
}
