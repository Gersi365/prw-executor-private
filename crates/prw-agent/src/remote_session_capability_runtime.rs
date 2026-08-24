//! Agent-owned lifetime boundary for one bound remote-session capability context.
//!
//! C03f selected this ownership boundary. C03e-J materializes only the source-level by-value
//! wrapper on the current post-authentication C03e-I lineage. C03e-U adds only the separately
//! selected executor-custody owner for future remote async work. This module still does not wire
//! the Agent binary, publish readiness, spawn tasks, or activate remote transport.

mod authenticated_remote_session_runtime;
mod remote_session_executor_runtime;

pub use authenticated_remote_session_runtime::AuthenticatedRemoteSessionRuntimeOwner;
pub use remote_session_executor_runtime::{
    RemoteSessionExecutorRuntime, RemoteSessionExecutorRuntimeCreateError,
};

use prw_remote_bridge::remote_session_binding::BoundRemoteSession;

/// Agent-owned lifetime boundary for one already-bound remote session.
///
/// Construction performs ownership composition only. The retained binding remains private until a
/// separately gated Agent-internal operation seam is selected and materialized.
pub struct RemoteSessionCapabilityRuntimeOwner {
    #[allow(
        dead_code,
        reason = "C03e-J retains the bound session for a separately gated operation seam"
    )]
    bound_session: BoundRemoteSession,
}

impl RemoteSessionCapabilityRuntimeOwner {
    /// Consumes one existing bound remote session without performing I/O or authorization.
    #[must_use]
    pub const fn new(bound_session: BoundRemoteSession) -> Self {
        Self { bound_session }
    }
}

#[cfg(test)]
mod tests {
    use prw_remote_bridge::remote_session_binding::BoundRemoteSession;

    use super::RemoteSessionCapabilityRuntimeOwner;

    fn assert_constructor_signature(
        constructor: fn(BoundRemoteSession) -> RemoteSessionCapabilityRuntimeOwner,
    ) {
        let _ = constructor;
    }

    #[test]
    fn runtime_owner_consumes_exact_bound_remote_session_shape() {
        assert_constructor_signature(RemoteSessionCapabilityRuntimeOwner::new);
    }
}
