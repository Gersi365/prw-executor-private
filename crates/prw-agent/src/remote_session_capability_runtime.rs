//! Agent-owned lifetime boundary for one bound remote-session capability context.
//!
//! C03f selected this ownership boundary. C03e-J materializes only the source-level by-value
//! wrapper on the current post-authentication C03e-I lineage. C03e-U adds the separately selected
//! executor-custody owner, C03e-X adds the C03e-W-selected shared-current registry/policy authority
//! owner, C03e-AB adds one lexically-contained spawned-and-joined worker seam, and C03e-AD adds one
//! single-worker orderly cancellation pair. C03e-AH adds the first pre-listener persistent
//! current-thread worker collection seam, C03e-AJ adds one bounded expected-device real remote
//! admission transaction, C03e-AL composes repeated expected-device admission with that persistent
//! collection under the same private current-thread runtime, and C03e-AP adds the AO-selected
//! executor-before-bind endpoint lifecycle startup plus explicit remote-supervisor shutdown control.
//! This module still does not wire the Agent binary, publish readiness, or activate a production
//! listener lifecycle.

mod authenticated_remote_session_runtime;
mod real_remote_admission_transaction;
mod remote_session_endpoint_lifecycle_runtime;
mod remote_session_executor_runtime;
mod remote_session_worker_cancellation;
mod shared_current_capability_authority;

pub use authenticated_remote_session_runtime::AuthenticatedRemoteSessionRuntimeOwner;
pub use real_remote_admission_transaction::{
    RemoteSessionRealAdmissionError, admit_expected_remote_device_session,
};
pub use remote_session_endpoint_lifecycle_runtime::{
    RemoteSessionEndpointLifecycleRuntime, RemoteSessionEndpointLifecycleStartupError,
    RemoteSessionEndpointLifecycleStartupFailure, RemoteSessionSupervisorShutdownController,
};
pub use remote_session_executor_runtime::{
    RemoteSessionExecutorRuntime, RemoteSessionExecutorRuntimeCreateError,
    RemoteSessionExpectedDeviceAdmissionRejection,
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest, RemoteSessionPersistentCollectionConfigError,
    RemoteSessionRealAdmissionTiming, RemoteSessionRegisteredWorkerCompletion,
    RemoteSessionRepeatedAdmissionFailure, RemoteSessionSpawnedWorkerJoinError,
    RemoteSessionWorkerAdmission, RemoteSessionWorkerAdmissionRejection,
    RemoteSessionWorkerAdmissionRejectionReason,
};
pub use remote_session_worker_cancellation::{
    RemoteSessionWorkerCancellationController, RemoteSessionWorkerCancellationSignal,
    remote_session_worker_cancellation_pair,
};
pub use shared_current_capability_authority::SharedCurrentCapabilityAuthority;

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
