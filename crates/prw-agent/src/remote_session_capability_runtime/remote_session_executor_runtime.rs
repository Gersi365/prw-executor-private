//! Agent-owned executor custody for the staged remote-session runtime path.
//!
//! C03e-T selected the narrow first executor shape: one explicit non-cloneable Tokio
//! current-thread runtime owner. C03e-U materializes only that construction/custody boundary.
//! It does not bind remote transport, drive a future, spawn a task, publish readiness, or wire
//! the Agent binary.

use std::fmt;

use tokio::runtime::{Builder, Runtime};

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

/// Explicit Agent-owned Tokio custody for future remote transport/session async work.
///
/// The raw Tokio runtime remains private. C03e-U intentionally exposes no `block_on`, task spawn,
/// handle clone, network bind, peer acceptance, session admission, cancellation channel, readiness,
/// or production activation surface.
pub struct RemoteSessionExecutorRuntime {
    #[allow(
        dead_code,
        reason = "C03e-U materializes executor custody before the separately gated borrowed drive seam"
    )]
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
