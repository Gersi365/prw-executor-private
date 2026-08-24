//! Agent-owned executor custody for the staged remote-session runtime path.
//!
//! C03e-T selected the narrow first executor shape: one explicit non-cloneable Tokio
//! current-thread runtime owner. C03e-U materializes that construction/custody boundary, and C03e-V
//! adds only one borrowed domain-specific drive seam for the existing C03e-S worker body. It does
//! not bind remote transport, spawn a task, publish readiness, or wire the Agent binary.

use std::{fmt, future::Future};

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::{CapabilityBridge, CapabilityDispatcher};
use tokio::runtime::{Builder, Runtime};

use super::authenticated_remote_session_runtime::{
    AuthenticatedRemoteSessionRuntimeOwner, AuthenticatedRemoteSessionWorkerStop,
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

/// Explicit Agent-owned Tokio custody for remote transport/session async work.
///
/// The raw Tokio runtime remains private. C03e-V exposes only the bounded
/// [`Self::drive_capability_request_worker`] seam; no generic `block_on`, task spawn, handle clone,
/// network bind, peer acceptance, concurrent session admission, cancellation-controller creation,
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
        P: PolicyEvaluator + Sync,
        D: CapabilityDispatcher + Send,
        T: FnMut() -> u64 + Send,
        C: Future<Output = ()> + Send,
    >(
        &mut self,
        session_owner: &mut AuthenticatedRemoteSessionRuntimeOwner,
        bridge: &CapabilityBridge<'_, P>,
        verifier_time_unix_seconds: T,
        dispatcher: &mut D,
        cancellation: C,
    ) -> AuthenticatedRemoteSessionWorkerStop {
        self.runtime
            .block_on(session_owner.run_capability_request_worker(
                bridge,
                verifier_time_unix_seconds,
                dispatcher,
                cancellation,
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::{RemoteSessionExecutorRuntime, RemoteSessionExecutorRuntimeCreateError};

    fn assert_constructor_signature(
        constructor: fn() -> Result<
            RemoteSessionExecutorRuntime,
            RemoteSessionExecutorRuntimeCreateError,
        >,
    ) {
        let _ = constructor;
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
}
