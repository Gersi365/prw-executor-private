//! Dormant process-level custody for the production durable capability authority.
//!
//! C03e-LD materializes only the C03e-LC-selected modular source layout for the C03e-LB
//! higher-owner custody semantics. One existing production/reachability/requester-rendezvous
//! aggregate is retained by value beside exactly one outer `Arc<ProductionDurableCapabilityAuthority>`.
//! Construction performs only the selected ownership adaptation and activates no runtime behavior.

#![allow(clippy::redundant_pub_crate)]

use std::sync::Arc;

use prw_core::DeviceId;
use prw_policy::{PolicyEvaluator, ProductionRemoteCapabilityDenyAllPolicy};
use prw_registry::WorkspaceDeviceRegistry;
use prw_remote_bridge::{
    CapabilityDispatcher,
    requester_rendezvous_in_memory_provider::{
        InMemoryRequesterRendezvousAuthorityProvider, RequesterRendezvousLifecycleError,
    },
};
use prw_session::SessionAuthenticationService;
use tokio::sync::mpsc;

use crate::candidate_publication_requester_rendezvous_runtime::CandidatePublicationRequesterRendezvousRuntimeOwner;
use crate::candidate_publication_requester_rendezvous_start_intent::policy_source::BoundedRequesterRendezvousStartPolicySource;
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

/// Populates one dormant pre-requester durable owner with one fresh process-local session service.
///
/// This wrapper constructs exactly one existing empty fail-closed [`SessionAuthenticationService`]
/// and moves that exact value by value into the existing C03e-MD production population helper
/// exactly once. It does not begin a session, generate challenge randomness, select authentication
/// timing, construct a service per admission/worker/session, clone or share the service, choose
/// capability-policy provenance, create an expected-request channel, construct requester/rendezvous
/// custody, add an invocation site, or activate runtime/listener/network behavior.
///
/// # Errors
///
/// Returns the existing C03e-MD population error unchanged. Service construction itself is
/// infallible, so this wrapper adds no error variant, retry, fallback or recovery path.
#[allow(
    clippy::future_not_send,
    clippy::type_complexity,
    dead_code,
    reason = "C03e-MF materializes the ME-selected dormant session-authentication population wrapper before separately gated remaining production provenance and caller wiring"
)]
pub(crate) async fn linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_session_authentication<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    capability_authority: SharedCurrentCapabilityAuthority<P>,
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<
    LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError,
> {
    let session_authentication = SessionAuthenticationService::new();
    linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources(
        capability_authority,
        session_authentication,
        expected_requests,
        admission_timing,
        on_completion,
        on_rejection,
        on_admission_failure,
    )
    .await
}

/// Populates one dormant pre-requester durable owner with a fail-closed current capability authority.
///
/// This wrapper constructs exactly one empty [`WorkspaceDeviceRegistry`] and exactly one
/// [`ProductionRemoteCapabilityDenyAllPolicy`], composes those exact values into one
/// [`SharedCurrentCapabilityAuthority`], and moves that exact authority by value into the existing
/// C03e-MF session-authentication population wrapper exactly once. The resulting current authority
/// therefore contains no populated membership/device state and grants no represented capability.
///
/// This helper does not hydrate or synchronize the current registry, load an allow-bearing policy,
/// adapt durable-registry custody into current authority, create an expected-request channel,
/// select admission timing or callbacks, construct requester/rendezvous custody, add an invocation
/// site, or activate runtime/listener/network behavior.
///
/// # Errors
///
/// Returns the existing C03e-MD population error unchanged. Empty-registry, deny-all-policy and
/// shared-current-authority construction are infallible, so no new error variant, retry, fallback or
/// recovery path is added.
#[allow(
    clippy::future_not_send,
    clippy::type_complexity,
    dead_code,
    reason = "C03e-MH materializes the MG-selected dormant fail-closed current capability-authority population wrapper before separately gated remaining production provenance and caller wiring"
)]
pub(crate) async fn linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_fail_closed_current_capability_authority<
    D,
    T,
    F,
    C,
    R,
    E,
>(
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<
    LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<
        ProductionRemoteCapabilityDenyAllPolicy,
        D,
        T,
        F,
        C,
        R,
        E,
    >,
    LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError,
> {
    let capability_authority = SharedCurrentCapabilityAuthority::new(
        WorkspaceDeviceRegistry::new(),
        ProductionRemoteCapabilityDenyAllPolicy,
    );
    linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_session_authentication(
        capability_authority,
        expected_requests,
        admission_timing,
        on_completion,
        on_rejection,
        on_admission_failure,
    )
    .await
}

/// Non-cloneable dormant custody joining one populated pre-requester durable owner with one exact
/// fail-closed requester/rendezvous start policy source.
#[allow(
    dead_code,
    reason = "C03e-MJ materializes the MI-selected private empty requester-policy custody before separately gated provider/runtime join"
)]
pub(crate) struct LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs<
    D,
    T,
    F,
    C,
    R,
    E,
> {
    production_inputs: LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<
        ProductionRemoteCapabilityDenyAllPolicy,
        D,
        T,
        F,
        C,
        R,
        E,
    >,
    requester_policy_source: BoundedRequesterRendezvousStartPolicySource,
}

/// Populates one dormant pre-requester owner plus one explicit empty requester policy source.
///
/// This wrapper first invokes the existing C03e-MH population helper exactly once. Only after that
/// succeeds does it construct exactly one empty [`BoundedRequesterRendezvousStartPolicySource`]
/// through `Default`. The exact successful MH owner and exact empty source are then moved by value
/// into one private non-cloneable carrier. No requester binding is added and no policy lookup is
/// performed during population.
///
/// This helper constructs no requester/rendezvous provider or runtime owner, selects no provider
/// capacity, performs no provider registration, constructs no final requester/rendezvous aggregate,
/// adds no invocation site, and activates no runtime/listener/network behavior.
///
/// # Errors
///
/// Returns the existing C03e-MD population error unchanged. Empty requester-policy source
/// construction is infallible, so no new error variant, retry, fallback or recovery path is added.
#[allow(
    clippy::future_not_send,
    clippy::type_complexity,
    dead_code,
    reason = "C03e-MJ materializes the MI-selected dormant empty requester-policy population wrapper before separately gated provider/runtime provenance and final join"
)]
pub(crate) async fn linux_agent_production_durable_reachability_requester_policy_remote_process_operation_inputs_from_production_sources<
    D,
    T,
    F,
    C,
    R,
    E,
>(
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<
    LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs<
        D,
        T,
        F,
        C,
        R,
        E,
    >,
    LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError,
> {
    let production_inputs = linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_fail_closed_current_capability_authority(
        expected_requests,
        admission_timing,
        on_completion,
        on_rejection,
        on_admission_failure,
    )
    .await?;
    let requester_policy_source = BoundedRequesterRendezvousStartPolicySource::default();
    Ok(
        LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs {
            production_inputs,
            requester_policy_source,
        },
    )
}

/// Populates one dormant requester/rendezvous runtime owner from one explicit caller-owned capacity.
///
/// The helper invokes the existing bounded in-memory provider constructor exactly once. Only after
/// successful provider construction does it move that exact provider by value into the existing
/// runtime-owner constructor exactly once. It performs no requester-policy evaluation, provider
/// registration, current-grant selection, cleanup, final requester/rendezvous join, caller wiring,
/// listener/readiness publication, or runtime/network activation.
///
/// # Errors
///
/// Returns the existing [`RequesterRendezvousLifecycleError`] unchanged. Zero capacity therefore
/// remains invalid through the provider constructor. No defaulting, clamping, retry, fallback,
/// alternate capacity, or partial runtime owner is produced.
#[allow(
    dead_code,
    reason = "C03e-ML materializes the MK-selected dormant explicit-capacity requester/rendezvous provider-runtime population before separately gated final join and caller wiring"
)]
pub(crate) fn linux_agent_production_requester_rendezvous_runtime_owner_from_explicit_nonzero_capacity(
    max_records: usize,
) -> Result<CandidatePublicationRequesterRendezvousRuntimeOwner, RequesterRendezvousLifecycleError>
{
    let provider = InMemoryRequesterRendezvousAuthorityProvider::new(max_records)?;
    Ok(CandidatePublicationRequesterRendezvousRuntimeOwner::new(
        provider,
    ))
}

/// Joins existing requester-policy and requester/rendezvous runtime custody into the existing
/// production durable requester/rendezvous owner without invoking either authority lane.
///
/// This helper consumes the exact C03e-MJ requester-policy carrier and one already-populated
/// requester/rendezvous runtime owner by value. It destructures only existing private custody,
/// constructs the existing inner production requester/rendezvous owner exactly once, then moves that
/// exact inner owner and the exact raw durable capability authority into the existing outer durable
/// higher-owner constructor exactly once. It performs no I/O, policy evaluation, provider mutation,
/// capacity selection, registration, current-grant selection, cleanup, caller wiring, listener or
/// runtime/network activation.
#[allow(
    dead_code,
    reason = "C03e-MN materializes the MM-selected dormant by-value requester/rendezvous custody join before separately gated combined production population and executable caller wiring"
)]
pub(crate) fn linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_existing_custody<
    D,
    T,
    F,
    C,
    R,
    E,
>(
    requester_policy_inputs:
        LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs<
            D, T, F, C, R, E,
        >,
    requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
) -> LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    ProductionRemoteCapabilityDenyAllPolicy,
    D,
    T,
    F,
    C,
    R,
    E,
> {
    let LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs {
        production_inputs,
        requester_policy_source,
    } = requester_policy_inputs;
    let LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs {
        production_inputs,
        capability_authority,
    } = production_inputs;
    let requester_rendezvous_inputs =
        LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(
            production_inputs,
            requester_policy_source,
            requester_rendezvous_runtime_owner,
        );
    LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(
        requester_rendezvous_inputs,
        capability_authority,
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
