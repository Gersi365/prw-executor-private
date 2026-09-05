//! Dormant process-level custody for the production durable capability authority.
//!
//! C03e-LD materializes only the C03e-LC-selected modular source layout for the C03e-LB
//! higher-owner custody semantics. One existing production/reachability/requester-rendezvous
//! aggregate is retained by value beside exactly one outer `Arc<ProductionDurableCapabilityAuthority>`.
//! Construction performs only the selected ownership adaptation and activates no runtime behavior.

#![allow(clippy::redundant_pub_crate)]

use std::sync::Arc;

use prw_core::DeviceId;
use prw_policy::PolicyEvaluator;
use prw_remote_bridge::CapabilityDispatcher;

use crate::linux_bootstrap::{
    LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs,
    LinuxAgentRemoteSupervisorShutdownPublisher,
    linux_agent_production_reachability_requester_rendezvous_remote_process_operation,
    linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection,
};
use crate::production_durable_registry_runtime_custody::ProductionDurableCapabilityAuthority;
use crate::remote_session_capability_runtime::{
    RemoteSessionExpectedDeviceAdmissionRejection,
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest,
    RemoteSessionRealAdmissionError,
    RemoteSessionRealAdmissionTiming,
    RemoteSessionRegisteredWorkerCompletion,
    RemoteSessionRepeatedAdmissionFailure,
    RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection,
};

/// Non-cloneable dormant process-lifetime owner for one production durable capability authority.
pub(crate) struct LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
> {
    requester_rendezvous_inputs:
        LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
            P,
            D,
            T,
            F,
            C,
            R,
            E,
        >,
    capability_authority: Arc<ProductionDurableCapabilityAuthority>,
}

impl<P, D, T, F, C, R, E>
    LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
        P,
        D,
        T,
        F,
        C,
        R,
        E,
    >
{
    /// Consumes the existing production aggregate and one raw durable authority into dormant custody.
    #[must_use]
    pub(crate) fn new(
        requester_rendezvous_inputs:
            LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
                P,
                D,
                T,
                F,
                C,
                R,
                E,
            >,
        capability_authority: ProductionDurableCapabilityAuthority,
    ) -> Self {
        Self {
            requester_rendezvous_inputs,
            capability_authority: Arc::new(capability_authority),
        }
    }
}

/// Builds one dormant production operation that retains durable capability-authority custody.
#[allow(
    dead_code,
    reason = "C03e-LF materializes the LE-selected operation-boundary durable-authority lifetime custody before separately gated caller migration and propagation"
)]
pub(crate) fn linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    inputs: LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
        P,
        D,
        T,
        F,
        C,
        R,
        E,
    >,
) -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
where
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> u64 + Send + 'static,
    F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static,
    C: FnMut(RemoteSessionRegisteredWorkerCompletion) + Send + 'static,
    R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>) + Send + 'static,
    E: FnMut(RemoteSessionRepeatedAdmissionFailure) + Send + 'static,
{
    let LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs {
        requester_rendezvous_inputs,
        capability_authority,
    } = inputs;
    let operation =
        linux_agent_production_reachability_requester_rendezvous_remote_process_operation(
            requester_rendezvous_inputs,
        );

    move |publisher| {
        operation(publisher);
        drop(capability_authority);
    }
}

/// Builds one dormant higher-owner production operation that transfers durable-authority custody
/// into the existing production durable callback projection operation.
///
/// Factory construction performs ownership transfer only. The existing requester/rendezvous
/// production aggregate and the one retained outer durable-capability `Arc` are consumed by value
/// and delegated exactly once to the C03e-LT projection-capable Linux operation. No additional Arc
/// construction/clone, callback translation, provider I/O, endpoint bind, publication, retry,
/// reconnect, executable caller or runtime activation is added.
#[allow(
    dead_code,
    reason = "C03e-LV materializes the LU-selected dormant higher-owner callback projection caller migration before separately gated executable assembly"
)]
pub(crate) fn linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    inputs: LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
        P,
        D,
        T,
        F,
        C,
        R,
        E,
    >,
) -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
where
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> u64 + Send + 'static,
    F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static,
    C: FnMut(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)
        + Send
        + 'static,
    R: FnMut(
            RemoteSessionExpectedDeviceAdmissionRejectionReason,
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
        ) + Send
        + 'static,
    E: FnMut(DeviceId, RemoteSessionRealAdmissionError) + Send + 'static,
{
    let LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs {
        requester_rendezvous_inputs,
        capability_authority,
    } = inputs;

    linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(
        requester_rendezvous_inputs,
        capability_authority,
    )
}
