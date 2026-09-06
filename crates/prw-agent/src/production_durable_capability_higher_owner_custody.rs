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
use prw_session::SessionAuthenticationService;
use tokio::sync::mpsc;

use crate::linux_bootstrap::{
    LinuxAgentBootstrapStartFailure, LinuxAgentBootstrapWithRemoteReport,
    LinuxAgentProductionReachabilityRemoteProcessOperationInputs,
    LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs,
    LinuxAgentProductionRemoteProcessInputPopulationError, LinuxAgentRemotePeerDeviceSourceError,
    LinuxAgentRemoteSupervisorShutdownPublisher,
    linux_agent_production_reachability_requester_rendezvous_remote_process_operation,
    linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection,
    linux_agent_remote_process_operation_inputs_from_production_worker_limit,
    load_linux_agent_remote_peer_device_id_from_env, run_with_remote_process_companion,
};
use crate::production_durable_registry_custody_bootstrap::{
    ProductionDurablePeerCapabilityAuthorityPopulationError,
    bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials,
};
use crate::production_durable_registry_runtime_custody::ProductionDurableCapabilityAuthority;
use crate::remote_session_capability_runtime::{
    RemoteSessionExpectedDeviceAdmissionRejection,
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest, RemoteSessionRealAdmissionError,
    RemoteSessionRealAdmissionTiming, RemoteSessionRegisteredWorkerCompletion,
    RemoteSessionRepeatedAdmissionFailure,
    RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection,
    SharedCurrentCapabilityAuthority,
};

/// Non-cloneable dormant pre-requester owner for one same-custody production reachability pair.
#[allow(
    dead_code,
    reason = "C03e-MB materializes the MA-selected pre-requester same-custody ownership carrier before separately gated population and requester/rendezvous join"
)]
pub(crate) struct LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
> {
    production_inputs:
        LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    capability_authority: ProductionDurableCapabilityAuthority,
}

/// Bounded Agent-local failure while populating one pre-requester same-custody durable owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "C03e-MD materializes the MC-selected bounded population error before separately gated caller wiring"
)]
pub(crate) enum LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError {
    /// Existing production worker-limit/bind input population failed.
    RemoteProcessInputs(LinuxAgentProductionRemoteProcessInputPopulationError),
    /// Fixed process logical-peer source failed before durable-registry/provider work.
    PeerDeviceSource(LinuxAgentRemotePeerDeviceSourceError),
    /// Single-custody peer plus durable capability-authority population failed.
    SameCustodyPeerCapabilityAuthority(ProductionDurablePeerCapabilityAuthorityPopulationError),
}

impl std::fmt::Display
    for LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::RemoteProcessInputs(_) => {
                "production durable reachability remote-process input population failed"
            }
            Self::PeerDeviceSource(_) => {
                "production durable reachability peer-device source failed"
            }
            Self::SameCustodyPeerCapabilityAuthority(_) => {
                "production durable reachability same-custody peer capability authority population failed"
            }
        })
    }
}

impl std::error::Error
    for LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RemoteProcessInputs(error) => Some(error),
            Self::PeerDeviceSource(error) => Some(error),
            Self::SameCustodyPeerCapabilityAuthority(error) => Some(error),
        }
    }
}

impl From<LinuxAgentProductionRemoteProcessInputPopulationError>
    for LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError
{
    fn from(error: LinuxAgentProductionRemoteProcessInputPopulationError) -> Self {
        Self::RemoteProcessInputs(error)
    }
}

impl From<LinuxAgentRemotePeerDeviceSourceError>
    for LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError
{
    fn from(error: LinuxAgentRemotePeerDeviceSourceError) -> Self {
        Self::PeerDeviceSource(error)
    }
}

impl From<ProductionDurablePeerCapabilityAuthorityPopulationError>
    for LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError
{
    fn from(error: ProductionDurablePeerCapabilityAuthorityPopulationError) -> Self {
        Self::SameCustodyPeerCapabilityAuthority(error)
    }
}

/// Populates one dormant pre-requester durable owner from existing production sources.
///
/// The helper first reuses the existing production worker-limit/bind input population. Only after
/// that succeeds does it load the fixed logical peer `DeviceId`. Only after both stages succeed does
/// it invoke the C03e-LZ same-custody helper exactly once. The returned current peer and raw durable
/// capability authority therefore share one exact durable-registry/runtime-custody lineage. The peer
/// is moved once into the existing production reachability owner, and that owner plus the exact raw
/// authority are moved directly into the C03e-MB pre-requester owner.
///
/// This helper does not invoke the legacy production peer helper or JR worker+peer composite helper,
/// performs no second durable-registry/provider bootstrap, constructs no requester/rendezvous
/// custody, wraps no durable authority in `Arc`, adds no invocation site, and activates no runtime,
/// listener, readiness, endpoint or network behavior.
///
/// # Errors
///
/// Fails before the next stage on production worker-limit/bind population failure, logical-peer
/// source failure, or same-custody peer/capability-authority population failure. Underlying errors
/// remain available through the bounded composite source chain. No retry, fallback, alternate peer,
/// cache, synthetic authority, degraded authority or partial owner is returned.
#[allow(
    clippy::future_not_send,
    clippy::type_complexity,
    dead_code,
    reason = "C03e-MD materializes the MC-selected dormant pre-requester same-custody owner population before separately gated requester/rendezvous and caller wiring"
)]
pub(crate) async fn linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    capability_authority: SharedCurrentCapabilityAuthority<P>,
    session_authentication: SessionAuthenticationService,
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<
    LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError,
> {
    let remote_process_inputs =
        linux_agent_remote_process_operation_inputs_from_production_worker_limit(
            capability_authority,
            session_authentication,
            expected_requests,
            admission_timing,
            on_completion,
            on_rejection,
            on_admission_failure,
        )?;
    let device_id = load_linux_agent_remote_peer_device_id_from_env()?;
    let (peer, capability_authority) =
        bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials(
            device_id,
        )
        .await?;
    let production_inputs = LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(
        peer,
        remote_process_inputs,
    );
    Ok(
        LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs {
            production_inputs,
            capability_authority,
        },
    )
}

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

/// Runs the existing Linux remote companion with one dormant higher-owner durable projection operation.
///
/// This crate-private assembly consumes only already-typed higher-owner inputs. It constructs the
/// existing C03e-LV projection-capable one-shot operation exactly once and passes that operation
/// directly to the existing generic Linux remote-companion runner exactly once. It adds no concrete
/// production input population, callback policy, executable invocation site or runtime activation.
#[allow(
    dead_code,
    reason = "C03e-LX materializes the LW-selected dormant higher-owner projection companion assembly before separately gated executable caller population"
)]
pub(crate) fn run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_with_production_durable_capability_projection<
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
) -> Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>
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
    let operation =
        linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(
            inputs,
        );
    run_with_remote_process_companion(operation)
}
