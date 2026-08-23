//! Agent-owned lifetime boundary for one bound remote-session capability context.
//!
//! Phase 152 C03f selected the ownership boundary. C03g materializes only the source-level
//! by-value wrapper on the authoritative C03e-C integration line. It does not accept peers,
//! authenticate sessions, authorize or dispatch capabilities, spawn tasks, publish remote
//! readiness, or perform transport I/O.

use prw_remote_bridge::remote_session_binding::BoundRemoteSession;

/// Agent-owned lifetime boundary for one already-bound remote session.
///
/// Construction performs ownership composition only. The retained binding remains private until a
/// separately gated Agent-internal operation seam is selected and materialized.
pub struct RemoteSessionCapabilityRuntimeOwner {
    #[allow(
        dead_code,
        reason = "C03g retains the bound session for a separately gated operation seam"
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
