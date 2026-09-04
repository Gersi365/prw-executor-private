//! Narrow public Linux Agent binary-bootstrap facade.
//!
//! Phase 102 keeps the internal Linux lifecycle/readiness/signal/worker graph
//! crate-private. This module exposes only the fixed initial bootstrap profile,
//! bounded startup/terminal classifications, and one call into the already-
//! validated Phase 098 signal-aware runtime.

use std::{
    ffi::OsString,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::{NonZeroU16, NonZeroUsize},
    time::Duration,
};

use prw_connectivity::PeerConnectivityIdentity;
use prw_core::DeviceId;
use prw_network::PrivateDnsConfig;
use prw_policy::{BoundedLocalReadPolicy, PolicyEvaluator};
use prw_remote_bridge::CapabilityDispatcher;
use prw_session::SessionAuthenticationService;
use tokio::sync::mpsc;

use crate::candidate_publication_requester_rendezvous_runtime::CandidatePublicationRequesterRendezvousRuntimeOwner;
use crate::candidate_publication_requester_rendezvous_start_intent::policy_source::BoundedRequesterRendezvousStartPolicySource;
use crate::linux_identity::deadline_io::LocalLinuxIoBudget;
use crate::linux_identity::production_lifecycle::LocalLinuxProductionLifecycleAssemblyError;
use crate::linux_identity::production_runtime_loop::LocalLinuxProductionRuntimeInputs;
use crate::linux_identity::production_runtime_types::{
    LocalLinuxProductionRuntimeCleanup, LocalLinuxProductionRuntimeConfig,
    LocalLinuxProductionRuntimeCounters,
};
use crate::linux_identity::signal_aware_runtime::{
    LocalLinuxSignalAwareRuntimeStartError, LocalLinuxSignalAwareRuntimeTerminalReason,
    run_signal_aware_linux_production_runtime_from_env,
    run_signal_aware_linux_production_runtime_from_env_with_companion,
};
use crate::linux_identity::termination_signal::{
    LocalLinuxTerminationSignal, LocalLinuxTerminationSignalMaskRestore,
    LocalLinuxTerminationSignalSourceCreateError,
};
use crate::linux_identity::xdg_runtime_root::prw_runtime_directory::agent_instance_lock::AgentInstanceLockError;
use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
use crate::local_commands::status_snapshot::{LocalAgentRuntimeState, LocalAgentStatusSnapshot};
use crate::remote_session_capability_runtime::{
    RemoteSessionEndpointLifecycleRuntime, RemoteSessionExecutorRuntime,
    RemoteSessionExpectedDeviceAdmissionRejection, RemoteSessionExpectedDeviceAdmissionRequest,
    RemoteSessionRealAdmissionTiming, RemoteSessionRegisteredWorkerCompletion,
    RemoteSessionRepeatedAdmissionFailure, RemoteSessionSupervisorShutdownController,
    SharedCurrentCapabilityAuthority,
    remote_session_process_lifecycle_control::{
        RemoteSessionProcessControllerFinalization, RemoteSessionProcessLifecycleFinalization,
        RemoteSessionProcessLifecycleOwner, RemoteSessionProcessLifecycleSpawnError,
        RemoteSessionProcessThreadFinalization, RemoteSessionSupervisorShutdownPublish,
        RemoteSessionSupervisorShutdownPublisher,
    },
};

/// Fixed non-secret process configuration name for the production remote endpoint bind address.
pub const PRW_REMOTE_BIND_ADDR_ENV: &str = "PRW_REMOTE_BIND_ADDR";

/// Fixed non-secret process configuration name for the production remote peer logical device.
pub const PRW_REMOTE_PEER_DEVICE_ID_ENV: &str = "PRW_REMOTE_PEER_DEVICE_ID";

/// Fixed non-secret process configuration name for the production remote active-worker bound.
pub const PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV: &str = "PRW_REMOTE_MAX_ACTIVE_WORKERS";

/// Stable failure while acquiring or validating production remote bind-address configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LinuxAgentRemoteBindAddressSourceError {
    /// The fixed configuration value is absent or empty.
    Unavailable,
    /// The operating-system value is not valid Unicode.
    EncodingInvalid,
    /// The configured value is not an exact `SocketAddr`.
    SocketAddressInvalid,
    /// The parsed address is not eligible for this explicit bind-and-observe lane.
    AddressNotBindAdvertisable,
}

impl std::fmt::Display for LinuxAgentRemoteBindAddressSourceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Unavailable => "remote bind-address configuration unavailable",
            Self::EncodingInvalid => "remote bind-address configuration encoding invalid",
            Self::SocketAddressInvalid => "remote bind-address socket address invalid",
            Self::AddressNotBindAdvertisable => "remote bind-address is not bind-advertisable",
        })
    }
}

impl std::error::Error for LinuxAgentRemoteBindAddressSourceError {}

fn parse_linux_agent_remote_bind_addr_value(
    value: Option<OsString>,
) -> Result<SocketAddr, LinuxAgentRemoteBindAddressSourceError> {
    let value = value.ok_or(LinuxAgentRemoteBindAddressSourceError::Unavailable)?;
    if value.is_empty() {
        return Err(LinuxAgentRemoteBindAddressSourceError::Unavailable);
    }
    let value = value
        .into_string()
        .map_err(|_| LinuxAgentRemoteBindAddressSourceError::EncodingInvalid)?;
    let address = value
        .parse::<SocketAddr>()
        .map_err(|_| LinuxAgentRemoteBindAddressSourceError::SocketAddressInvalid)?;
    let ip = address.ip();
    if ip.is_unspecified()
        || ip.is_multicast()
        || matches!(ip, IpAddr::V4(ipv4) if ipv4 == Ipv4Addr::BROADCAST)
    {
        return Err(LinuxAgentRemoteBindAddressSourceError::AddressNotBindAdvertisable);
    }
    Ok(address)
}

/// Loads the explicitly configured production remote bind address from the process environment.
///
/// The fixed value is parsed directly as [`SocketAddr`]. This function performs no DNS lookup,
/// interface enumeration, route inspection, public-address discovery, socket bind or fallback.
/// Port `0` remains valid pre-bind so the retained endpoint may report the kernel-selected port
/// through the separately materialized bound-address observation after a successful bind.
///
/// Configuration validity is not identity, authentication, authorization, readiness, reachability,
/// publication provenance or public-routability evidence.
///
/// # Errors
///
/// Fails closed when the fixed configuration is absent/empty, non-Unicode, malformed, unspecified,
/// multicast, or IPv4 limited broadcast. The error classification does not expose the configured
/// value.
pub fn load_linux_agent_remote_bind_addr_from_env()
-> Result<SocketAddr, LinuxAgentRemoteBindAddressSourceError> {
    parse_linux_agent_remote_bind_addr_value(std::env::var_os(PRW_REMOTE_BIND_ADDR_ENV))
}

/// Stable failure while acquiring or validating the production remote peer logical device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LinuxAgentRemotePeerDeviceSourceError {
    /// The fixed configuration value is absent.
    Missing,
    /// The operating-system value is not valid Unicode.
    NonUnicode,
    /// The configured value does not satisfy the existing `DeviceId` contract.
    InvalidIdentifier,
}

impl std::fmt::Display for LinuxAgentRemotePeerDeviceSourceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Missing => "remote peer-device configuration missing",
            Self::NonUnicode => "remote peer-device configuration encoding invalid",
            Self::InvalidIdentifier => "remote peer-device identifier invalid",
        })
    }
}

impl std::error::Error for LinuxAgentRemotePeerDeviceSourceError {}

fn parse_linux_agent_remote_peer_device_id_value(
    value: Option<OsString>,
) -> Result<DeviceId, LinuxAgentRemotePeerDeviceSourceError> {
    let value = value.ok_or(LinuxAgentRemotePeerDeviceSourceError::Missing)?;
    let value = value
        .into_string()
        .map_err(|_| LinuxAgentRemotePeerDeviceSourceError::NonUnicode)?;
    DeviceId::new(value).map_err(|error| match error {
        prw_core::IdentifierError::Empty => {
            LinuxAgentRemotePeerDeviceSourceError::InvalidIdentifier
        }
    })
}

/// Loads the explicitly configured production remote peer logical device from the process environment.
///
/// The exact Unicode value is passed directly to [`DeviceId::new`] without trimming,
/// normalization, case conversion, delimiter parsing or endpoint interpretation. This source
/// performs no registry/provider I/O and does not construct a [`PeerConnectivityIdentity`].
///
/// Configuration validity is process peer intent only; current same-device transport authority
/// remains the responsibility of the separately materialized durable-registry lookup.
///
/// # Errors
///
/// Fails closed when the fixed configuration is missing, non-Unicode, empty, or whitespace-only.
/// The bounded error surface does not expose the configured identifier value.
pub fn load_linux_agent_remote_peer_device_id_from_env()
-> Result<DeviceId, LinuxAgentRemotePeerDeviceSourceError> {
    parse_linux_agent_remote_peer_device_id_value(std::env::var_os(PRW_REMOTE_PEER_DEVICE_ID_ENV))
}

/// Stable failure while acquiring or validating the production remote active-worker bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LinuxAgentRemoteMaxActiveWorkersSourceError {
    /// The fixed configuration value is absent.
    Missing,
    /// The operating-system value is not valid Unicode.
    NonUnicode,
    /// The configured value is not a strictly-positive target-`usize` ASCII decimal integer.
    InvalidValue,
}

impl std::fmt::Display for LinuxAgentRemoteMaxActiveWorkersSourceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Missing => "remote max-active-workers configuration missing",
            Self::NonUnicode => "remote max-active-workers configuration encoding invalid",
            Self::InvalidValue => "remote max-active-workers configuration invalid",
        })
    }
}

impl std::error::Error for LinuxAgentRemoteMaxActiveWorkersSourceError {}

fn parse_linux_agent_remote_max_active_workers_value(
    value: Option<OsString>,
) -> Result<NonZeroUsize, LinuxAgentRemoteMaxActiveWorkersSourceError> {
    let value = value.ok_or(LinuxAgentRemoteMaxActiveWorkersSourceError::Missing)?;
    let value = value
        .into_string()
        .map_err(|_| LinuxAgentRemoteMaxActiveWorkersSourceError::NonUnicode)?;
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue);
    }
    let parsed = value
        .parse::<usize>()
        .map_err(|_| LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue)?;
    NonZeroUsize::new(parsed).ok_or(LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue)
}

/// Loads the explicitly configured production remote active-worker bound from the process environment.
///
/// The exact Unicode value must contain ASCII decimal digits only and is converted fail-closed into
/// the existing [`NonZeroUsize`] input domain. This source performs no trimming, normalization,
/// fallback, retry, alternate-variable lookup, dynamic refresh, or host-derived auto-sizing.
///
/// # Errors
///
/// Fails closed when the fixed configuration is missing, non-Unicode, empty, malformed, zero, or
/// out of range for target `usize`. The bounded error surface does not expose the configured value.
pub fn load_linux_agent_remote_max_active_workers_from_env()
-> Result<NonZeroUsize, LinuxAgentRemoteMaxActiveWorkersSourceError> {
    parse_linux_agent_remote_max_active_workers_value(std::env::var_os(
        PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV,
    ))
}

/// Stable high-level terminal class exposed to the Agent binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentBootstrapTerminal {
    /// Programmatic monotonic shutdown completed the runtime loop.
    ProgrammaticShutdown,
    /// A handled `SIGTERM` initiated orderly shutdown.
    SigTerm,
    /// A handled `SIGINT` initiated orderly shutdown.
    SigInt,
    /// Signal-aware readiness failed closed.
    ReadinessFatal,
    /// Runtime scheduling failed under the locked fail-stop policy.
    RuntimeFatal,
}

impl LinuxAgentBootstrapTerminal {
    /// Returns the bounded token used by the initial stderr summary contract.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::ProgrammaticShutdown => "programmatic_shutdown",
            Self::SigTerm => "sigterm",
            Self::SigInt => "sigint",
            Self::ReadinessFatal => "readiness_fatal",
            Self::RuntimeFatal => "runtime_fatal",
        }
    }

    const fn is_normal(self) -> bool {
        matches!(
            self,
            Self::ProgrammaticShutdown | Self::SigTerm | Self::SigInt
        )
    }
}

/// Listener/socket cleanup class exposed to the Agent binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentBootstrapCleanup {
    /// Exact validated listener/socket cleanup completed.
    Clean,
    /// Listener/socket cleanup failed after the terminal cause was established.
    Failed,
}

impl LinuxAgentBootstrapCleanup {
    /// Returns the bounded token used by the initial stderr summary contract.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Failed => "failed",
        }
    }
}

/// Signal-mask restoration evidence exposed to the Agent binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentBootstrapSignalMaskRestore {
    /// No signal mask had been changed on this failure path.
    NotApplicable,
    /// The exact prior calling-thread signal mask was restored.
    Restored,
    /// Restoring the prior calling-thread signal mask failed.
    Failed,
}

impl LinuxAgentBootstrapSignalMaskRestore {
    /// Returns the bounded token used by the initial stderr failure contract.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Restored => "restored",
            Self::Failed => "failed",
        }
    }
}

/// Memory-bounded process-lifetime counters exposed by the bootstrap facade.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LinuxAgentBootstrapCounters {
    readiness_steps: u64,
    listener_armed_steps: u64,
    runtime_wakes: u64,
    wait_interruptions: u64,
    scheduling_attempts: u64,
    workers_registered: u64,
    worker_completions: u64,
    peer_rejections: u64,
}

impl LinuxAgentBootstrapCounters {
    /// Returns completed readiness steps.
    #[must_use]
    pub const fn readiness_steps(self) -> u64 {
        self.readiness_steps
    }

    /// Returns readiness steps that armed listener interest.
    #[must_use]
    pub const fn listener_armed_steps(self) -> u64 {
        self.listener_armed_steps
    }

    /// Returns processed runtime wake outcomes.
    #[must_use]
    pub const fn runtime_wakes(self) -> u64 {
        self.runtime_wakes
    }

    /// Returns surfaced wait interruptions.
    #[must_use]
    pub const fn wait_interruptions(self) -> u64 {
        self.wait_interruptions
    }

    /// Returns total bounded scheduling attempts.
    #[must_use]
    pub const fn scheduling_attempts(self) -> u64 {
        self.scheduling_attempts
    }

    /// Returns total workers registered during the runtime lifetime.
    #[must_use]
    pub const fn workers_registered(self) -> u64 {
        self.workers_registered
    }

    /// Returns total classified worker completions.
    #[must_use]
    pub const fn worker_completions(self) -> u64 {
        self.worker_completions
    }

    /// Returns same-UID peer authorization rejections handled connection-locally.
    #[must_use]
    pub const fn peer_rejections(self) -> u64 {
        self.peer_rejections
    }
}

/// Final bounded report returned to the standalone Agent binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxAgentBootstrapReport {
    terminal: LinuxAgentBootstrapTerminal,
    counters: LinuxAgentBootstrapCounters,
    cleanup: LinuxAgentBootstrapCleanup,
    signal_mask_restore: LinuxAgentBootstrapSignalMaskRestore,
}

impl LinuxAgentBootstrapReport {
    /// Returns the high-level terminal class.
    #[must_use]
    pub const fn terminal(self) -> LinuxAgentBootstrapTerminal {
        self.terminal
    }

    /// Returns memory-bounded lifetime counters.
    #[must_use]
    pub const fn counters(self) -> LinuxAgentBootstrapCounters {
        self.counters
    }

    /// Returns listener/socket cleanup evidence.
    #[must_use]
    pub const fn cleanup(self) -> LinuxAgentBootstrapCleanup {
        self.cleanup
    }

    /// Returns exact signal-mask restoration evidence.
    #[must_use]
    pub const fn signal_mask_restore(self) -> LinuxAgentBootstrapSignalMaskRestore {
        self.signal_mask_restore
    }

    /// Returns whether the locked Phase 101 binary exit contract classifies this report as success.
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.terminal.is_normal()
            && matches!(self.cleanup, LinuxAgentBootstrapCleanup::Clean)
            && matches!(
                self.signal_mask_restore,
                LinuxAgentBootstrapSignalMaskRestore::Restored
            )
    }
}

/// Bounded startup-failure class exposed to the standalone Agent binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentBootstrapStartKind {
    /// Default private-DNS config could not project into the bounded local snapshot.
    PrivateDnsSnapshot,
    /// Safe `SIGTERM`/`SIGINT` signal source could not be established.
    SignalSource,
    /// `$XDG_RUNTIME_DIR` validation failed.
    RuntimeRoot,
    /// The fixed PRW runtime child could not be safely prepared.
    RuntimeDirectory,
    /// Another conforming Agent instance already holds the instance lock.
    AlreadyRunning,
    /// The instance lock failed for a reason other than an existing Agent.
    InstanceLock,
    /// The validated local Agent socket could not be bound.
    Bind,
    /// The bound socket could not enter listening state.
    Listen,
    /// The listener could not enter verified nonblocking accept-ready state.
    AcceptReady,
    /// The shared runtime wake descriptor could not be created.
    RuntimeWake,
}

impl LinuxAgentBootstrapStartKind {
    /// Returns the bounded token used by the initial stderr failure contract.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::PrivateDnsSnapshot => "private_dns_snapshot",
            Self::SignalSource => "signal_source",
            Self::RuntimeRoot => "runtime_root",
            Self::RuntimeDirectory => "runtime_directory",
            Self::AlreadyRunning => "already_running",
            Self::InstanceLock => "instance_lock",
            Self::Bind => "bind",
            Self::Listen => "listen",
            Self::AcceptReady => "accept_ready",
            Self::RuntimeWake => "runtime_wake",
        }
    }
}

/// Startup failure plus any signal-mask rollback evidence available on that path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxAgentBootstrapStartFailure {
    kind: LinuxAgentBootstrapStartKind,
    signal_mask_restore: LinuxAgentBootstrapSignalMaskRestore,
}

impl LinuxAgentBootstrapStartFailure {
    const fn new(
        kind: LinuxAgentBootstrapStartKind,
        signal_mask_restore: LinuxAgentBootstrapSignalMaskRestore,
    ) -> Self {
        Self {
            kind,
            signal_mask_restore,
        }
    }

    /// Returns the bounded startup-failure class.
    #[must_use]
    pub const fn kind(self) -> LinuxAgentBootstrapStartKind {
        self.kind
    }

    /// Returns signal-mask rollback evidence for the startup-failure path.
    #[must_use]
    pub const fn signal_mask_restore(self) -> LinuxAgentBootstrapSignalMaskRestore {
        self.signal_mask_restore
    }
}

/// Bounded result of publishing the existing one-shot remote supervisor shutdown controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentRemoteSupervisorShutdownPublish {
    /// The exact controller moved to process-side ownership.
    Published,
    /// Process-side ownership disappeared and orderly shutdown was requested on the recovered controller.
    ReceiverGoneShutdownRequested,
}

/// Non-cloneable bootstrap-facing one-shot publisher for the existing remote shutdown controller.
pub struct LinuxAgentRemoteSupervisorShutdownPublisher {
    publisher: RemoteSessionSupervisorShutdownPublisher,
}

impl LinuxAgentRemoteSupervisorShutdownPublisher {
    /// Consumes this wrapper and publishes the exact existing remote supervisor shutdown controller.
    #[must_use]
    pub fn publish(
        self,
        controller: RemoteSessionSupervisorShutdownController,
    ) -> LinuxAgentRemoteSupervisorShutdownPublish {
        map_remote_shutdown_publish(self.publisher.publish(controller))
    }
}

fn run_remote_process_operation_composition<
    Executor,
    Authority,
    Endpoint,
    Controller,
    Publication,
    ExecutorError,
    BootstrapError,
    EndpointError,
>(
    construct_executor: impl FnOnce() -> Result<Executor, ExecutorError>,
    bootstrap_authority: impl FnOnce(&Executor) -> Result<Authority, BootstrapError>,
    start_endpoint: impl FnOnce(Executor, Authority) -> Result<(Endpoint, Controller), EndpointError>,
    publish_controller: impl FnOnce(Controller) -> Publication,
    drive_lifecycle: impl FnOnce(Endpoint, Publication),
) -> bool {
    let Ok(executor) = construct_executor() else {
        return false;
    };
    let Ok(authority) = bootstrap_authority(&executor) else {
        return false;
    };
    let Ok((endpoint, controller)) = start_endpoint(executor, authority) else {
        return false;
    };
    let publication = publish_controller(controller);
    drive_lifecycle(endpoint, publication);
    true
}

/// Injected values required to build one library-owned remote process operation.
///
/// Construction owns only already-typed inputs. It performs no credential read, provider I/O,
/// endpoint bind, authentication, authorization, task spawn, readiness publication or process
/// lifecycle mutation. The owner is intentionally non-cloneable.
pub struct LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    bind_addr: SocketAddr,
    max_active_workers: NonZeroUsize,
    capability_authority: SharedCurrentCapabilityAuthority<P>,
    session_authentication: SessionAuthenticationService,
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
}

impl<P, D, T, F, C, R, E> LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    /// Consumes the exact injected remote-operation values without starting remote work.
    #[must_use]
    #[expect(
        clippy::too_many_arguments,
        reason = "C03e-AZ keeps the selected injected remote-operation inputs explicit and typed"
    )]
    pub const fn new(
        bind_addr: SocketAddr,
        max_active_workers: NonZeroUsize,
        capability_authority: SharedCurrentCapabilityAuthority<P>,
        session_authentication: SessionAuthenticationService,
        expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
        admission_timing: F,
        on_completion: C,
        on_rejection: R,
        on_admission_failure: E,
    ) -> Self {
        Self {
            bind_addr,
            max_active_workers,
            capability_authority,
            session_authentication,
            expected_requests,
            admission_timing,
            on_completion,
            on_rejection,
            on_admission_failure,
        }
    }
}

/// Populates the existing remote-operation owner from the selected production bind-address source.
///
/// This crate-private helper performs exactly one existing process-environment bind-address load and
/// otherwise only moves already-typed remote-operation inputs into the existing owner constructor.
/// It does not construct peer identity, requester/rendezvous custody or any executable caller.
#[allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    dead_code,
    reason = "C03e-IM materializes the IL-selected production bind-address input population before separately gated remaining production provenance"
)]
pub(crate) fn linux_agent_remote_process_operation_inputs_from_production_bind_addr<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    max_active_workers: NonZeroUsize,
    capability_authority: SharedCurrentCapabilityAuthority<P>,
    session_authentication: SessionAuthenticationService,
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<
    LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    LinuxAgentRemoteBindAddressSourceError,
> {
    let bind_addr = load_linux_agent_remote_bind_addr_from_env()?;
    Ok(LinuxAgentRemoteProcessOperationInputs::new(
        bind_addr,
        max_active_workers,
        capability_authority,
        session_authentication,
        expected_requests,
        admission_timing,
        on_completion,
        on_rejection,
        on_admission_failure,
    ))
}

/// Bounded Agent-local failure while populating production worker-limit and bind-address inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinuxAgentProductionRemoteProcessInputPopulationError {
    /// The fixed production worker-limit source failed before bind-address acquisition.
    WorkerLimitSource(LinuxAgentRemoteMaxActiveWorkersSourceError),
    /// The existing production bind-address source failed after worker-limit acquisition.
    BindAddressSource(LinuxAgentRemoteBindAddressSourceError),
}

impl std::fmt::Display for LinuxAgentProductionRemoteProcessInputPopulationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::WorkerLimitSource(_) => "production worker-limit source failed",
            Self::BindAddressSource(_) => "production bind-address source failed",
        })
    }
}

impl std::error::Error for LinuxAgentProductionRemoteProcessInputPopulationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WorkerLimitSource(error) => Some(error),
            Self::BindAddressSource(error) => Some(error),
        }
    }
}

impl From<LinuxAgentRemoteMaxActiveWorkersSourceError>
    for LinuxAgentProductionRemoteProcessInputPopulationError
{
    fn from(error: LinuxAgentRemoteMaxActiveWorkersSourceError) -> Self {
        Self::WorkerLimitSource(error)
    }
}

impl From<LinuxAgentRemoteBindAddressSourceError>
    for LinuxAgentProductionRemoteProcessInputPopulationError
{
    fn from(error: LinuxAgentRemoteBindAddressSourceError) -> Self {
        Self::BindAddressSource(error)
    }
}

/// Populates the existing remote-process owner from production worker-limit and bind sources.
///
/// The helper loads the fixed worker limit exactly once, then delegates exactly once to the existing
/// bind-address population helper with that exact `NonZeroUsize`. All remaining typed inputs move
/// unchanged. It performs no peer lookup, operation construction, runtime activation or fallback.
///
/// # Errors
///
/// Fails closed on worker-limit source failure before bind-address acquisition, or on the existing
/// bind-address source failure after worker-limit acquisition. The bounded stage error preserves the
/// exact underlying source error without exposing configured values.
#[allow(
    clippy::type_complexity,
    dead_code,
    reason = "C03e-JP materializes the JO-selected production worker-limit input population before separately gated remaining production provenance"
)]
pub(crate) fn linux_agent_remote_process_operation_inputs_from_production_worker_limit<
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
    LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    LinuxAgentProductionRemoteProcessInputPopulationError,
> {
    let max_active_workers = load_linux_agent_remote_max_active_workers_from_env()?;
    let inputs = linux_agent_remote_process_operation_inputs_from_production_bind_addr(
        max_active_workers,
        capability_authority,
        session_authentication,
        expected_requests,
        admission_timing,
        on_completion,
        on_rejection,
        on_admission_failure,
    )?;
    Ok(inputs)
}

/// Crate-private production process-operation inputs selected by C03e-IF.
///
/// This owner retains one typed logical peer identity beside the existing injected remote-process
/// inputs. Construction is side-effect-free: it performs no credential read, provider I/O,
/// endpoint bind, listener activation, readiness publication or durable-owner mutation.
#[allow(
    dead_code,
    reason = "C03e-IG materializes the IF-selected production process-operation input owner before separately gated executable assembly"
)]
pub(crate) struct LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>
{
    peer: PeerConnectivityIdentity,
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
}

impl<P, D, T, F, C, R, E>
    LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>
{
    /// Consumes the exact typed peer identity and existing remote-process inputs without starting work.
    #[must_use]
    #[allow(
        dead_code,
        reason = "C03e-IG materializes the IF-selected production process-operation input owner before separately gated executable assembly"
    )]
    pub(crate) const fn new(
        peer: PeerConnectivityIdentity,
        remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    ) -> Self {
        Self {
            peer,
            remote_process_inputs,
        }
    }
}

/// Bounded Agent-local failure while populating one production peer input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinuxAgentProductionPeerInputPopulationError {
    /// The fixed process logical-peer source failed before provider/bootstrap work.
    PeerDeviceSource(LinuxAgentRemotePeerDeviceSourceError),
    /// Existing production durable-registry custody/provider bootstrap failed.
    DurableRegistryBootstrap(
        crate::production_durable_registry_custody_bootstrap::ProductionDurableRegistryCustodyBootstrapError,
    ),
    /// Existing current same-device durable-registry peer lookup failed.
    DurableRegistryLookup(prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError),
}

impl std::fmt::Display for LinuxAgentProductionPeerInputPopulationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::PeerDeviceSource(_) => "production peer-device source failed",
            Self::DurableRegistryBootstrap(_) => "production durable-registry bootstrap failed",
            Self::DurableRegistryLookup(_) => "production durable-registry peer lookup failed",
        })
    }
}

impl std::error::Error for LinuxAgentProductionPeerInputPopulationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PeerDeviceSource(error) => Some(error),
            Self::DurableRegistryBootstrap(error) => Some(error),
            Self::DurableRegistryLookup(error) => Some(error),
        }
    }
}

impl From<LinuxAgentRemotePeerDeviceSourceError> for LinuxAgentProductionPeerInputPopulationError {
    fn from(error: LinuxAgentRemotePeerDeviceSourceError) -> Self {
        Self::PeerDeviceSource(error)
    }
}

impl From<
    crate::production_durable_registry_custody_bootstrap::ProductionDurableRegistryCustodyBootstrapError,
> for LinuxAgentProductionPeerInputPopulationError
{
    fn from(
        error: crate::production_durable_registry_custody_bootstrap::ProductionDurableRegistryCustodyBootstrapError,
    ) -> Self {
        Self::DurableRegistryBootstrap(error)
    }
}

impl From<prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError>
    for LinuxAgentProductionPeerInputPopulationError
{
    fn from(
        error: prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError,
    ) -> Self {
        Self::DurableRegistryLookup(error)
    }
}

/// Populates only the existing production reachability `peer` field from current registry authority.
///
/// The helper loads one fixed process logical [`DeviceId`], bootstraps the existing production
/// durable-registry store once, adapts that exact store into the existing Agent runtime custody,
/// resolves one current same-device [`PeerConnectivityIdentity`], and then moves the already-built
/// remote-process inputs unchanged into the existing production owner. It selects no caller and
/// performs no reachability recovery, endpoint bind, readiness publication or remote lifecycle work.
///
/// # Errors
///
/// Fails before the next stage on peer-device source, durable-registry bootstrap or current-peer
/// lookup failure. No retry, fallback, alternate peer, cache or degraded owner is produced.
#[allow(
    clippy::future_not_send,
    dead_code,
    reason = "C03e-JK materializes the JJ-selected production peer input population before separately gated remaining production provenance"
)]
pub(crate) async fn linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
) -> Result<
    LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    LinuxAgentProductionPeerInputPopulationError,
> {
    let device_id = load_linux_agent_remote_peer_device_id_from_env()?;
    let store = crate::production_durable_registry_custody_bootstrap::bootstrap_production_durable_registry_from_systemd_credentials().await?;
    let mut registry_custody =
        crate::production_durable_registry_runtime_custody::ProductionDurableRegistryRuntimeCustody::from_store(store);
    let peer = registry_custody
        .peer_connectivity_identity(device_id)
        .await?;
    Ok(
        LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(
            peer,
            remote_process_inputs,
        ),
    )
}

/// Builds one side-effect-free injected remote operation compatible with the AX bootstrap facade.
///
/// Factory construction performs ownership composition only. Remote credential/provider I/O and
/// endpoint startup occur only if a caller later invokes the returned closure. The operation uses
/// one exact private executor for reachability bootstrap and the complete endpoint/session lifecycle.
/// It does not select a production bind-address source, expected-device producer, dispatcher,
/// registry/policy source, timing source, readiness policy or executable process-exit policy.
pub fn linux_agent_remote_process_operation<P, D, T, F, C, R, E>(
    inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
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
    move |publisher| {
        let LinuxAgentRemoteProcessOperationInputs {
            bind_addr,
            max_active_workers,
            capability_authority,
            mut session_authentication,
            expected_requests,
            admission_timing,
            on_completion,
            on_rejection,
            on_admission_failure,
        } = inputs;

        let _ = run_remote_process_operation_composition(
            RemoteSessionExecutorRuntime::new,
            RemoteSessionExecutorRuntime::bootstrap_reachability_authority_from_systemd_credentials,
            move |executor, authority_owner| {
                RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials(
                    executor,
                    authority_owner,
                    bind_addr,
                )
            },
            move |controller| publisher.publish(controller),
            move |lifecycle, _publication| {
                let _ = lifecycle.drive_repeated_real_remote_admission_endpoint_lifecycle(
                    max_active_workers,
                    &capability_authority,
                    &mut session_authentication,
                    expected_requests,
                    admission_timing,
                    on_completion,
                    on_rejection,
                    on_admission_failure,
                );
            },
        );
    }
}

/// Builds one dormant production-reachability remote process operation selected by C03e-IF.
///
/// Factory construction only moves already-typed values into a one-shot closure. Credential/provider
/// bootstrap, endpoint bind, shutdown-controller publication and endpoint drive occur only if a
/// separately gated caller later invokes that closure. The operation preserves one exact executor
/// across production bootstrap and endpoint startup and retains durable production-owner custody
/// through the production endpoint wrapper's complete lifecycle drive.
#[allow(
    dead_code,
    reason = "C03e-IG materializes the IF-selected production process operation before separately gated executable assembly"
)]
pub(crate) fn linux_agent_production_reachability_remote_process_operation<P, D, T, F, C, R, E>(
    inputs: LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
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
    move |publisher| {
        let LinuxAgentProductionReachabilityRemoteProcessOperationInputs {
            peer,
            remote_process_inputs,
        } = inputs;
        let LinuxAgentRemoteProcessOperationInputs {
            bind_addr,
            max_active_workers,
            capability_authority,
            mut session_authentication,
            expected_requests,
            admission_timing,
            on_completion,
            on_rejection,
            on_admission_failure,
        } = remote_process_inputs;

        let _ = run_remote_process_operation_composition(
            RemoteSessionExecutorRuntime::new,
            move |executor| {
                executor.bootstrap_production_reachability_runtime_custody_from_systemd_credentials(
                    &peer,
                )
            },
            move |executor, runtime_custody| {
                runtime_custody.bind_remote_endpoint_with_executor_from_systemd_credentials(
                    executor, bind_addr,
                )
            },
            move |controller| publisher.publish(controller),
            move |lifecycle, _publication| {
                let _ = lifecycle.drive_repeated_real_remote_admission_endpoint_lifecycle(
                    max_active_workers,
                    &capability_authority,
                    &mut session_authentication,
                    expected_requests,
                    admission_timing,
                    on_completion,
                    on_rejection,
                    on_admission_failure,
                );
            },
        );
    }
}

/// Crate-private production requester/rendezvous process-operation lifetime custody selected by C03e-IH.
///
/// Construction joins only already-typed, already-owned values. It performs no requester-policy
/// evaluation, requester/rendezvous provider mutation, credential read, endpoint bind, listener
/// activation, readiness publication, candidate publication, traversal, dialing, or durable-owner
/// mutation. The owner is intentionally non-cloneable.
#[allow(
    dead_code,
    reason = "C03e-II materializes the IH-selected production/requester-rendezvous custody join before separately gated executable assembly"
)]
pub(crate) struct LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
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
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
}

impl<P, D, T, F, C, R, E>
    LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
        P,
        D,
        T,
        F,
        C,
        R,
        E,
    >
{
    /// Consumes the exact production operation inputs and requester/rendezvous custody values.
    #[must_use]
    #[allow(
        dead_code,
        reason = "C03e-II materializes the IH-selected production/requester-rendezvous custody join before separately gated executable assembly"
    )]
    pub(crate) const fn new(
        production_inputs: LinuxAgentProductionReachabilityRemoteProcessOperationInputs<
            P,
            D,
            T,
            F,
            C,
            R,
            E,
        >,
        requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
        requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
    ) -> Self {
        Self {
            production_inputs,
            requester_rendezvous_start_policy_source,
            requester_rendezvous_runtime_owner,
        }
    }
}

/// Builds one dormant production operation that retains requester/rendezvous custody by value.
///
/// Factory construction delegates exactly once to the existing C03e-IG production operation and
/// otherwise performs ownership composition only. Requester-policy and requester/rendezvous
/// provider behavior remain uninvoked; the returned one-shot closure explicitly releases those
/// custody values immediately before delegating to the unchanged production operation.
#[allow(
    dead_code,
    reason = "C03e-II materializes the IH-selected production/requester-rendezvous custody join before separately gated executable assembly"
)]
pub(crate) fn linux_agent_production_reachability_requester_rendezvous_remote_process_operation<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    inputs: LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
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
    let LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs {
        production_inputs,
        requester_rendezvous_start_policy_source,
        requester_rendezvous_runtime_owner,
    } = inputs;
    let operation = linux_agent_production_reachability_remote_process_operation(production_inputs);

    move |publisher| {
        drop(requester_rendezvous_runtime_owner);
        drop(requester_rendezvous_start_policy_source);
        operation(publisher);
    }
}

/// Crate-private process-operation lifetime custody for one concrete requester-aware policy source.
#[allow(
    dead_code,
    reason = "C03e-EB materializes requester-policy custody before separately gated production assembly"
)]
pub(crate) struct LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
}

impl<P, D, T, F, C, R, E>
    LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>
{
    /// Owns the existing remote-process inputs and one already-constructed requester-policy source.
    #[must_use]
    #[allow(
        dead_code,
        reason = "C03e-EB materializes requester-policy custody before separately gated production assembly"
    )]
    pub(crate) const fn new(
        remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
        requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
        requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
    ) -> Self {
        Self {
            remote_process_inputs,
            requester_rendezvous_start_policy_source,
            requester_rendezvous_runtime_owner,
        }
    }
}

/// Builds one crate-private remote operation that retains requester-policy source custody.
#[allow(
    dead_code,
    reason = "C03e-EB materializes requester-policy custody before separately gated production assembly"
)]
pub(crate) fn linux_agent_requester_rendezvous_remote_process_operation<P, D, T, F, C, R, E>(
    inputs: LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
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
    let LinuxAgentRequesterRendezvousRemoteProcessOperationInputs {
        remote_process_inputs,
        requester_rendezvous_start_policy_source,
        requester_rendezvous_runtime_owner,
    } = inputs;
    let operation = linux_agent_remote_process_operation(remote_process_inputs);

    move |publisher| {
        drop(requester_rendezvous_runtime_owner);
        drop(requester_rendezvous_start_policy_source);
        operation(publisher);
    }
}

/// Bounded process-side controller finalization evidence for the injected remote companion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentRemoteProcessControllerFinalization {
    /// The handed-off controller received the orderly shutdown request.
    ShutdownRequested,
    /// The remote lane ended before publishing a shutdown controller.
    UnavailableBeforeEndpointStartup,
}

/// Bounded OS-thread finalization evidence for the injected remote companion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentRemoteProcessThreadFinalization {
    /// The exact join-owned remote thread returned normally.
    Joined,
    /// The exact join-owned remote thread panicked; payload and thread identity were discarded.
    Panicked,
}

/// Secondary bounded finalization evidence for the injected remote companion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentRemoteProcessCompanionFinalization {
    /// The remote process thread could not be created; local bootstrap semantics remain primary.
    SpawnFailed,
    /// The existing AT process owner finalized the controller handoff and exact join-owned thread.
    Finalized {
        /// Bounded process-side controller finalization evidence.
        controller: LinuxAgentRemoteProcessControllerFinalization,
        /// Bounded exact-thread join evidence.
        thread: LinuxAgentRemoteProcessThreadFinalization,
    },
}

/// Existing local bootstrap report plus secondary injected-remote-companion evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxAgentBootstrapWithRemoteReport {
    local: LinuxAgentBootstrapReport,
    remote: LinuxAgentRemoteProcessCompanionFinalization,
}

impl LinuxAgentBootstrapWithRemoteReport {
    /// Returns the existing primary local bootstrap report unchanged.
    #[must_use]
    pub const fn local(self) -> LinuxAgentBootstrapReport {
        self.local
    }

    /// Returns bounded secondary remote companion finalization evidence.
    #[must_use]
    pub const fn remote(self) -> LinuxAgentRemoteProcessCompanionFinalization {
        self.remote
    }
}

fn with_initial_runtime_inputs<R>(
    operation: impl FnOnce(
        LocalLinuxProductionRuntimeInputs<'_>,
    ) -> Result<R, LinuxAgentBootstrapStartFailure>,
) -> Result<R, LinuxAgentBootstrapStartFailure> {
    let private_dns_config = PrivateDnsConfig::default();
    let private_dns_snapshot = LocalPrivateDnsSnapshot::try_from_config(&private_dns_config)
        .map_err(|_| {
            LinuxAgentBootstrapStartFailure::new(
                LinuxAgentBootstrapStartKind::PrivateDnsSnapshot,
                LinuxAgentBootstrapSignalMaskRestore::NotApplicable,
            )
        })?;

    let inputs = LocalLinuxProductionRuntimeInputs::new(
        initial_runtime_config(),
        BoundedLocalReadPolicy::allow_local_reads(),
        LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready),
        &private_dns_snapshot,
    );

    operation(inputs)
}

/// Runs the fixed initial standalone Linux Agent bootstrap profile.
///
/// The facade builds only the Phase 101 locked immutable profile and delegates
/// to the already-validated Phase 098 signal-aware runtime. It performs no
/// systemd installation/activation and opens no public or remote listener.
///
/// # Errors
///
/// Returns a bounded startup failure when snapshot construction, safe signal
/// setup, or descriptor-anchored local lifecycle assembly cannot complete.
pub fn run() -> Result<LinuxAgentBootstrapReport, LinuxAgentBootstrapStartFailure> {
    with_initial_runtime_inputs(|inputs| {
        run_signal_aware_linux_production_runtime_from_env(inputs, |_| {})
            .map(|report| map_terminal_report(&report))
            .map_err(map_start_failure)
    })
}

/// Runs the same fixed initial Linux Agent profile with one injected remote process companion.
///
/// The remote operation remains caller-supplied. This facade creates no production remote inputs,
/// selects no bind address, performs no reachability bootstrap, and does not define executable exit
/// policy for the returned secondary remote evidence.
///
/// # Errors
///
/// Returns the existing local bootstrap startup failure unchanged. Remote process-thread spawn
/// failure remains secondary evidence on successful local lifecycle completion.
pub fn run_with_remote_process_companion<F>(
    operation: F,
) -> Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>
where
    F: FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static,
{
    with_initial_runtime_inputs(|inputs| {
        run_with_remote_process_companion_inputs(inputs, operation)
            .map(|(local, remote)| LinuxAgentBootstrapWithRemoteReport { local, remote })
    })
}

/// Runs the fixed local profile with the already-typed production/requester-rendezvous companion.
///
/// This crate-private assembly boundary only composes the existing C03e-II operation factory with
/// the existing injected remote-companion runner. It constructs no real production inputs and is
/// not invoked by `run()` or the Agent executable.
#[allow(
    dead_code,
    reason = "C03e-IK materializes the IJ-selected dormant executable assembly before separately gated caller/input assembly"
)]
pub(crate) fn run_with_production_reachability_requester_rendezvous_remote_process_companion<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
>(
    inputs: LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
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
    C: FnMut(RemoteSessionRegisteredWorkerCompletion) + Send + 'static,
    R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>) + Send + 'static,
    E: FnMut(RemoteSessionRepeatedAdmissionFailure) + Send + 'static,
{
    let operation =
        linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs);
    run_with_remote_process_companion(operation)
}

fn run_with_remote_process_companion_inputs<F>(
    inputs: LocalLinuxProductionRuntimeInputs<'_>,
    operation: F,
) -> Result<
    (
        LinuxAgentBootstrapReport,
        LinuxAgentRemoteProcessCompanionFinalization,
    ),
    LinuxAgentBootstrapStartFailure,
>
where
    F: FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static,
{
    let mut remote_finalization = None;
    let report = run_signal_aware_linux_production_runtime_from_env_with_companion(
        inputs,
        |_| {},
        || {
            RemoteSessionProcessLifecycleOwner::spawn(move |publisher| {
                operation(LinuxAgentRemoteSupervisorShutdownPublisher { publisher });
            })
        },
        |companion| {
            remote_finalization = Some(finalize_remote_process_companion(companion));
        },
    )
    .map_err(map_start_failure)?;

    let remote_finalization = remote_finalization
        .expect("signal-aware companion finalizer runs before successful bootstrap return");

    Ok((map_terminal_report(&report), remote_finalization))
}

const fn map_remote_shutdown_publish(
    publish: RemoteSessionSupervisorShutdownPublish,
) -> LinuxAgentRemoteSupervisorShutdownPublish {
    match publish {
        RemoteSessionSupervisorShutdownPublish::Published => {
            LinuxAgentRemoteSupervisorShutdownPublish::Published
        }
        RemoteSessionSupervisorShutdownPublish::ReceiverGoneShutdownRequested => {
            LinuxAgentRemoteSupervisorShutdownPublish::ReceiverGoneShutdownRequested
        }
    }
}

const fn map_remote_process_controller_finalization(
    controller: RemoteSessionProcessControllerFinalization,
) -> LinuxAgentRemoteProcessControllerFinalization {
    match controller {
        RemoteSessionProcessControllerFinalization::ShutdownRequested => {
            LinuxAgentRemoteProcessControllerFinalization::ShutdownRequested
        }
        RemoteSessionProcessControllerFinalization::UnavailableBeforeEndpointStartup => {
            LinuxAgentRemoteProcessControllerFinalization::UnavailableBeforeEndpointStartup
        }
    }
}

const fn map_remote_process_thread_finalization(
    thread: RemoteSessionProcessThreadFinalization,
) -> LinuxAgentRemoteProcessThreadFinalization {
    match thread {
        RemoteSessionProcessThreadFinalization::Joined => {
            LinuxAgentRemoteProcessThreadFinalization::Joined
        }
        RemoteSessionProcessThreadFinalization::Panicked => {
            LinuxAgentRemoteProcessThreadFinalization::Panicked
        }
    }
}

const fn map_remote_process_finalization(
    finalization: RemoteSessionProcessLifecycleFinalization,
) -> LinuxAgentRemoteProcessCompanionFinalization {
    LinuxAgentRemoteProcessCompanionFinalization::Finalized {
        controller: map_remote_process_controller_finalization(finalization.controller()),
        thread: map_remote_process_thread_finalization(finalization.thread()),
    }
}

fn finalize_remote_process_companion(
    companion: Result<RemoteSessionProcessLifecycleOwner, RemoteSessionProcessLifecycleSpawnError>,
) -> LinuxAgentRemoteProcessCompanionFinalization {
    companion.map_or(
        LinuxAgentRemoteProcessCompanionFinalization::SpawnFailed,
        |owner| map_remote_process_finalization(owner.finalize()),
    )
}

fn initial_runtime_config() -> LocalLinuxProductionRuntimeConfig {
    LocalLinuxProductionRuntimeConfig::new(
        NonZeroUsize::new(2).expect("Phase 101 worker capacity is non-zero"),
        NonZeroU16::new(8).expect("Phase 101 listener backlog is non-zero"),
        NonZeroUsize::new(2).expect("Phase 101 scheduling budget is non-zero"),
        NonZeroUsize::new(1).expect("Phase 101 request budget is non-zero"),
        LocalLinuxIoBudget::try_new(Duration::from_secs(2))
            .expect("Phase 101 read I/O budget is non-zero"),
        LocalLinuxIoBudget::try_new(Duration::from_secs(2))
            .expect("Phase 101 write I/O budget is non-zero"),
    )
}

const fn map_terminal_report(
    report: &crate::linux_identity::signal_aware_runtime::LocalLinuxSignalAwareRuntimeTerminalReport,
) -> LinuxAgentBootstrapReport {
    LinuxAgentBootstrapReport {
        terminal: match report.reason() {
            LocalLinuxSignalAwareRuntimeTerminalReason::ProgrammaticShutdown => {
                LinuxAgentBootstrapTerminal::ProgrammaticShutdown
            }
            LocalLinuxSignalAwareRuntimeTerminalReason::TerminationSignal(
                LocalLinuxTerminationSignal::SigTerm,
            ) => LinuxAgentBootstrapTerminal::SigTerm,
            LocalLinuxSignalAwareRuntimeTerminalReason::TerminationSignal(
                LocalLinuxTerminationSignal::SigInt,
            ) => LinuxAgentBootstrapTerminal::SigInt,
            LocalLinuxSignalAwareRuntimeTerminalReason::ReadinessFatal(_) => {
                LinuxAgentBootstrapTerminal::ReadinessFatal
            }
            LocalLinuxSignalAwareRuntimeTerminalReason::RuntimeFatal(_) => {
                LinuxAgentBootstrapTerminal::RuntimeFatal
            }
        },
        counters: map_counters(report.counters()),
        cleanup: map_cleanup(report.cleanup()),
        signal_mask_restore: map_signal_mask_restore(report.mask_restore()),
    }
}

const fn map_counters(
    counters: LocalLinuxProductionRuntimeCounters,
) -> LinuxAgentBootstrapCounters {
    LinuxAgentBootstrapCounters {
        readiness_steps: counters.readiness_steps(),
        listener_armed_steps: counters.listener_armed_steps(),
        runtime_wakes: counters.runtime_wakes(),
        wait_interruptions: counters.wait_interruptions(),
        scheduling_attempts: counters.scheduling_attempts(),
        workers_registered: counters.workers_registered(),
        worker_completions: counters.worker_completions(),
        peer_rejections: counters.peer_rejections(),
    }
}

const fn map_cleanup(cleanup: LocalLinuxProductionRuntimeCleanup) -> LinuxAgentBootstrapCleanup {
    match cleanup {
        LocalLinuxProductionRuntimeCleanup::Clean => LinuxAgentBootstrapCleanup::Clean,
        LocalLinuxProductionRuntimeCleanup::Failed(_) => LinuxAgentBootstrapCleanup::Failed,
    }
}

const fn map_signal_mask_restore(
    restore: LocalLinuxTerminationSignalMaskRestore,
) -> LinuxAgentBootstrapSignalMaskRestore {
    match restore {
        LocalLinuxTerminationSignalMaskRestore::Restored => {
            LinuxAgentBootstrapSignalMaskRestore::Restored
        }
        LocalLinuxTerminationSignalMaskRestore::Failed => {
            LinuxAgentBootstrapSignalMaskRestore::Failed
        }
    }
}

const fn map_start_failure(
    error: LocalLinuxSignalAwareRuntimeStartError,
) -> LinuxAgentBootstrapStartFailure {
    match error {
        LocalLinuxSignalAwareRuntimeStartError::SignalSource(error) => match error {
            LocalLinuxTerminationSignalSourceCreateError::MaskBlockFailed => {
                LinuxAgentBootstrapStartFailure::new(
                    LinuxAgentBootstrapStartKind::SignalSource,
                    LinuxAgentBootstrapSignalMaskRestore::NotApplicable,
                )
            }
            LocalLinuxTerminationSignalSourceCreateError::DescriptorCreateFailed {
                mask_restore,
            } => LinuxAgentBootstrapStartFailure::new(
                LinuxAgentBootstrapStartKind::SignalSource,
                map_signal_mask_restore(mask_restore),
            ),
        },
        LocalLinuxSignalAwareRuntimeStartError::Lifecycle {
            error,
            mask_restore,
        } => LinuxAgentBootstrapStartFailure::new(
            map_lifecycle_start_kind(error),
            map_signal_mask_restore(mask_restore),
        ),
    }
}

const fn map_lifecycle_start_kind(
    error: LocalLinuxProductionLifecycleAssemblyError,
) -> LinuxAgentBootstrapStartKind {
    match error {
        LocalLinuxProductionLifecycleAssemblyError::RuntimeRoot(_) => {
            LinuxAgentBootstrapStartKind::RuntimeRoot
        }
        LocalLinuxProductionLifecycleAssemblyError::RuntimeDirectory(_) => {
            LinuxAgentBootstrapStartKind::RuntimeDirectory
        }
        LocalLinuxProductionLifecycleAssemblyError::InstanceLock(
            AgentInstanceLockError::AlreadyRunning,
        ) => LinuxAgentBootstrapStartKind::AlreadyRunning,
        LocalLinuxProductionLifecycleAssemblyError::InstanceLock(_) => {
            LinuxAgentBootstrapStartKind::InstanceLock
        }
        LocalLinuxProductionLifecycleAssemblyError::Bind(_) => LinuxAgentBootstrapStartKind::Bind,
        LocalLinuxProductionLifecycleAssemblyError::Listen { .. } => {
            LinuxAgentBootstrapStartKind::Listen
        }
        LocalLinuxProductionLifecycleAssemblyError::AcceptReady { .. } => {
            LinuxAgentBootstrapStartKind::AcceptReady
        }
        LocalLinuxProductionLifecycleAssemblyError::RuntimeWake { .. } => {
            LinuxAgentBootstrapStartKind::RuntimeWake
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        ffi::OsString,
        net::{Ipv4Addr, Ipv6Addr, SocketAddr},
        num::NonZeroUsize,
    };

    #[cfg(unix)]
    use std::os::unix::ffi::OsStringExt;

    use prw_connectivity::{PeerConnectivityIdentity, TransportIdentity};
    use prw_core::DeviceId;
    use prw_policy::{BoundedLocalReadPolicy, Capability, Decision, PolicyEvaluator};
    use prw_registry::WorkspaceDeviceRegistry;
    use prw_remote_bridge::{AuthorizedCapabilityRequest, CapabilityDispatcher};
    use prw_session::SessionAuthenticationService;
    use tokio::sync::mpsc;

    use super::{
        LinuxAgentBootstrapCleanup, LinuxAgentBootstrapCounters, LinuxAgentBootstrapReport,
        LinuxAgentBootstrapSignalMaskRestore, LinuxAgentBootstrapStartFailure,
        LinuxAgentBootstrapStartKind, LinuxAgentBootstrapTerminal,
        LinuxAgentBootstrapWithRemoteReport,
        LinuxAgentProductionReachabilityRemoteProcessOperationInputs,
        LinuxAgentRemoteBindAddressSourceError, LinuxAgentRemoteMaxActiveWorkersSourceError,
        LinuxAgentRemotePeerDeviceSourceError, LinuxAgentRemoteProcessCompanionFinalization,
        LinuxAgentRemoteProcessControllerFinalization, LinuxAgentRemoteProcessOperationInputs,
        LinuxAgentRemoteProcessThreadFinalization, LinuxAgentRemoteSupervisorShutdownPublish,
        LinuxAgentRemoteSupervisorShutdownPublisher, PRW_REMOTE_BIND_ADDR_ENV,
        PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV, PRW_REMOTE_PEER_DEVICE_ID_ENV,
        finalize_remote_process_companion, initial_runtime_config,
        linux_agent_production_reachability_remote_process_operation,
        linux_agent_remote_process_operation, load_linux_agent_remote_bind_addr_from_env,
        load_linux_agent_remote_max_active_workers_from_env,
        load_linux_agent_remote_peer_device_id_from_env, map_lifecycle_start_kind,
        map_remote_shutdown_publish, parse_linux_agent_remote_bind_addr_value,
        parse_linux_agent_remote_max_active_workers_value,
        parse_linux_agent_remote_peer_device_id_value, run,
        run_remote_process_operation_composition, run_with_remote_process_companion,
    };
    use crate::linux_identity::production_lifecycle::LocalLinuxProductionLifecycleAssemblyError;
    use crate::linux_identity::worker_capacity::LocalLinuxWorkerCapacity;
    use crate::linux_identity::xdg_runtime_root::prw_runtime_directory::agent_instance_lock::AgentInstanceLockError;
    use crate::remote_session_capability_runtime::{
        RemoteSessionExpectedDeviceAdmissionRejection, RemoteSessionExpectedDeviceAdmissionRequest,
        RemoteSessionRealAdmissionTiming, RemoteSessionRegisteredWorkerCompletion,
        RemoteSessionRepeatedAdmissionFailure, RemoteSessionSupervisorShutdownController,
        SharedCurrentCapabilityAuthority,
        remote_session_process_lifecycle_control::{
            RemoteSessionProcessLifecycleOwner, RemoteSessionProcessLifecycleSpawnError,
            RemoteSessionSupervisorShutdownPublish,
        },
    };

    struct TestDispatcher;

    impl CapabilityDispatcher for TestDispatcher {
        type Error = ();

        fn dispatch(
            &mut self,
            _request: &AuthorizedCapabilityRequest,
        ) -> Result<Vec<u8>, Self::Error> {
            Ok(Vec::new())
        }
    }

    type TestExpectedRequest =
        RemoteSessionExpectedDeviceAdmissionRequest<TestDispatcher, fn() -> u64>;
    type TestExpectedRejection =
        RemoteSessionExpectedDeviceAdmissionRejection<TestDispatcher, fn() -> u64>;

    fn test_verifier_time() -> u64 {
        1
    }

    fn test_admission_timing(_device_id: &DeviceId) -> RemoteSessionRealAdmissionTiming {
        RemoteSessionRealAdmissionTiming::new(1..2, 1, 1..2)
    }

    fn test_completion(_completion: RemoteSessionRegisteredWorkerCompletion) {}

    fn test_rejection(_rejection: TestExpectedRejection) {}

    fn test_admission_failure(_failure: RemoteSessionRepeatedAdmissionFailure) {}

    fn assert_remote_operation_shape<F>(operation: F)
    where
        F: FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static,
    {
        drop(operation);
    }

    #[test]
    fn remote_bind_source_public_reader_has_exact_selected_shape() {
        fn assert_signature(
            reader: fn() -> Result<SocketAddr, LinuxAgentRemoteBindAddressSourceError>,
        ) {
            let _ = reader;
        }

        assert_eq!(PRW_REMOTE_BIND_ADDR_ENV, "PRW_REMOTE_BIND_ADDR");
        assert_signature(load_linux_agent_remote_bind_addr_from_env);
    }

    #[test]
    fn remote_bind_source_rejects_missing_empty_and_malformed_values() {
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(None),
            Err(LinuxAgentRemoteBindAddressSourceError::Unavailable)
        );
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(Some(OsString::new())),
            Err(LinuxAgentRemoteBindAddressSourceError::Unavailable)
        );
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(Some(OsString::from("example.invalid:4433"))),
            Err(LinuxAgentRemoteBindAddressSourceError::SocketAddressInvalid)
        );
    }

    #[cfg(unix)]
    #[test]
    fn remote_bind_source_rejects_non_unicode_value() {
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(Some(OsString::from_vec(vec![0xff]))),
            Err(LinuxAgentRemoteBindAddressSourceError::EncodingInvalid)
        );
    }

    #[test]
    fn remote_bind_source_preserves_exact_ipv4_ipv6_loopback_and_port_zero() {
        let ipv4 = SocketAddr::from(([192, 0, 2, 10], 4433));
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(Some(OsString::from(ipv4.to_string()))),
            Ok(ipv4)
        );

        let ipv6 = SocketAddr::from((Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 10), 4434));
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(Some(OsString::from(ipv6.to_string()))),
            Ok(ipv6)
        );

        let port_zero = SocketAddr::from(([192, 0, 2, 10], 0));
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(Some(OsString::from(port_zero.to_string()))),
            Ok(port_zero)
        );

        let loopback = SocketAddr::from(([127, 0, 0, 1], 4435));
        assert_eq!(
            parse_linux_agent_remote_bind_addr_value(Some(OsString::from(loopback.to_string()))),
            Ok(loopback)
        );
    }

    #[test]
    fn remote_bind_source_rejects_non_advertisable_address_classes() {
        for rejected in [
            SocketAddr::from(([0, 0, 0, 0], 4433)),
            SocketAddr::from((Ipv6Addr::UNSPECIFIED, 4433)),
            SocketAddr::from(([224, 0, 0, 1], 4433)),
            SocketAddr::from((Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1), 4433)),
            SocketAddr::from((Ipv4Addr::BROADCAST, 4433)),
        ] {
            assert_eq!(
                parse_linux_agent_remote_bind_addr_value(Some(OsString::from(
                    rejected.to_string()
                ))),
                Err(LinuxAgentRemoteBindAddressSourceError::AddressNotBindAdvertisable)
            );
        }
    }

    #[test]
    fn remote_peer_device_source_public_reader_has_exact_selected_shape() {
        fn assert_signature(
            reader: fn() -> Result<DeviceId, LinuxAgentRemotePeerDeviceSourceError>,
        ) {
            let _ = reader;
        }

        assert_eq!(PRW_REMOTE_PEER_DEVICE_ID_ENV, "PRW_REMOTE_PEER_DEVICE_ID");
        assert_signature(load_linux_agent_remote_peer_device_id_from_env);
    }

    #[test]
    fn remote_peer_device_source_rejects_missing_empty_and_whitespace_values() {
        assert_eq!(
            parse_linux_agent_remote_peer_device_id_value(None),
            Err(LinuxAgentRemotePeerDeviceSourceError::Missing)
        );
        assert_eq!(
            parse_linux_agent_remote_peer_device_id_value(Some(OsString::new())),
            Err(LinuxAgentRemotePeerDeviceSourceError::InvalidIdentifier)
        );
        assert_eq!(
            parse_linux_agent_remote_peer_device_id_value(Some(OsString::from("   "))),
            Err(LinuxAgentRemotePeerDeviceSourceError::InvalidIdentifier)
        );
    }

    #[cfg(unix)]
    #[test]
    fn remote_peer_device_source_rejects_non_unicode_value() {
        assert_eq!(
            parse_linux_agent_remote_peer_device_id_value(Some(OsString::from_vec(vec![0xff]))),
            Err(LinuxAgentRemotePeerDeviceSourceError::NonUnicode)
        );
    }

    #[test]
    fn remote_peer_device_source_preserves_exact_non_empty_identifier() {
        let ordinary =
            parse_linux_agent_remote_peer_device_id_value(Some(OsString::from("peer-device-1")))
                .expect("ordinary peer device identifier");
        assert_eq!(ordinary.as_str(), "peer-device-1");

        let spaced = parse_linux_agent_remote_peer_device_id_value(Some(OsString::from(
            "  peer-device-1  ",
        )))
        .expect("non-empty spaced peer device identifier");
        assert_eq!(spaced.as_str(), "  peer-device-1  ");
    }

    #[test]
    fn remote_max_active_workers_source_public_reader_has_exact_selected_shape() {
        fn assert_signature(
            reader: fn() -> Result<NonZeroUsize, LinuxAgentRemoteMaxActiveWorkersSourceError>,
        ) {
            let _ = reader;
        }

        assert_eq!(
            PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV,
            "PRW_REMOTE_MAX_ACTIVE_WORKERS"
        );
        assert_signature(load_linux_agent_remote_max_active_workers_from_env);
    }

    #[test]
    fn remote_max_active_workers_source_rejects_missing_empty_and_malformed_values() {
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(None),
            Err(LinuxAgentRemoteMaxActiveWorkersSourceError::Missing)
        );
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::new())),
            Err(LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue)
        );

        for malformed in [" 1", "1 ", "+1", "-1", "1.0", "1_0", "1e1", "1a"] {
            assert_eq!(
                parse_linux_agent_remote_max_active_workers_value(Some(OsString::from(malformed))),
                Err(LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue)
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn remote_max_active_workers_source_rejects_non_unicode_value() {
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::from_vec(vec![0xff]))),
            Err(LinuxAgentRemoteMaxActiveWorkersSourceError::NonUnicode)
        );
    }

    #[test]
    fn remote_max_active_workers_source_rejects_zero_and_target_usize_overflow() {
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::from("0"))),
            Err(LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue)
        );
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::from("0000"))),
            Err(LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue)
        );

        let overflow = format!("{}0", usize::MAX);
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::from(overflow))),
            Err(LinuxAgentRemoteMaxActiveWorkersSourceError::InvalidValue)
        );
    }

    #[test]
    fn remote_max_active_workers_source_preserves_positive_magnitude_and_leading_zeroes() {
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::from("17")))
                .expect("positive worker bound")
                .get(),
            17
        );
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::from("00017")))
                .expect("leading-zero positive worker bound")
                .get(),
            17
        );
        assert_eq!(
            parse_linux_agent_remote_max_active_workers_value(Some(OsString::from(
                usize::MAX.to_string(),
            )))
            .expect("target-usize maximum is a valid positive worker bound")
            .get(),
            usize::MAX
        );
    }

    #[test]
    fn production_remote_process_input_population_error_preserves_selected_stage_types() {
        fn assert_worker_source_conversion(
            convert: fn(
                LinuxAgentRemoteMaxActiveWorkersSourceError,
            ) -> super::LinuxAgentProductionRemoteProcessInputPopulationError,
        ) {
            let _ = convert;
        }

        fn assert_bind_source_conversion(
            convert: fn(
                LinuxAgentRemoteBindAddressSourceError,
            ) -> super::LinuxAgentProductionRemoteProcessInputPopulationError,
        ) {
            let _ = convert;
        }

        assert_worker_source_conversion(
            super::LinuxAgentProductionRemoteProcessInputPopulationError::from,
        );
        assert_bind_source_conversion(
            super::LinuxAgentProductionRemoteProcessInputPopulationError::from,
        );

        let worker = super::LinuxAgentProductionRemoteProcessInputPopulationError::from(
            LinuxAgentRemoteMaxActiveWorkersSourceError::Missing,
        );
        assert!(matches!(
            worker,
            super::LinuxAgentProductionRemoteProcessInputPopulationError::WorkerLimitSource(
                LinuxAgentRemoteMaxActiveWorkersSourceError::Missing
            )
        ));
        assert_eq!(worker.to_string(), "production worker-limit source failed");
        assert!(std::error::Error::source(&worker).is_some());

        let bind = super::LinuxAgentProductionRemoteProcessInputPopulationError::from(
            LinuxAgentRemoteBindAddressSourceError::Unavailable,
        );
        assert!(matches!(
            bind,
            super::LinuxAgentProductionRemoteProcessInputPopulationError::BindAddressSource(
                LinuxAgentRemoteBindAddressSourceError::Unavailable
            )
        ));
        assert_eq!(bind.to_string(), "production bind-address source failed");
        assert!(std::error::Error::source(&bind).is_some());
    }

    #[test]
    fn production_worker_limit_input_population_helper_has_selected_type_shape() {
        #[allow(clippy::type_complexity)]
        fn assert_signature(
            entry: fn(
                SharedCurrentCapabilityAuthority<BoundedLocalReadPolicy>,
                SessionAuthenticationService,
                mpsc::Receiver<TestExpectedRequest>,
                fn(&DeviceId) -> RemoteSessionRealAdmissionTiming,
                fn(RemoteSessionRegisteredWorkerCompletion),
                fn(TestExpectedRejection),
                fn(RemoteSessionRepeatedAdmissionFailure),
            ) -> Result<
                LinuxAgentRemoteProcessOperationInputs<
                    BoundedLocalReadPolicy,
                    TestDispatcher,
                    fn() -> u64,
                    fn(&DeviceId) -> RemoteSessionRealAdmissionTiming,
                    fn(RemoteSessionRegisteredWorkerCompletion),
                    fn(TestExpectedRejection),
                    fn(RemoteSessionRepeatedAdmissionFailure),
                >,
                super::LinuxAgentProductionRemoteProcessInputPopulationError,
            >,
        ) {
            let _ = entry;
        }

        assert_signature(
            super::linux_agent_remote_process_operation_inputs_from_production_worker_limit::<
                BoundedLocalReadPolicy,
                TestDispatcher,
                fn() -> u64,
                fn(&DeviceId) -> RemoteSessionRealAdmissionTiming,
                fn(RemoteSessionRegisteredWorkerCompletion),
                fn(TestExpectedRejection),
                fn(RemoteSessionRepeatedAdmissionFailure),
            >,
        );
    }

    #[test]
    fn production_peer_input_population_error_preserves_selected_stage_types() {
        fn assert_peer_source_conversion(
            convert: fn(
                LinuxAgentRemotePeerDeviceSourceError,
            ) -> super::LinuxAgentProductionPeerInputPopulationError,
        ) {
            let _ = convert;
        }

        fn assert_bootstrap_conversion(
            convert: fn(
                crate::production_durable_registry_custody_bootstrap::ProductionDurableRegistryCustodyBootstrapError,
            ) -> super::LinuxAgentProductionPeerInputPopulationError,
        ) {
            let _ = convert;
        }

        fn assert_lookup_conversion(
            convert: fn(
                prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError,
            ) -> super::LinuxAgentProductionPeerInputPopulationError,
        ) {
            let _ = convert;
        }

        assert_peer_source_conversion(super::LinuxAgentProductionPeerInputPopulationError::from);
        assert_bootstrap_conversion(super::LinuxAgentProductionPeerInputPopulationError::from);
        assert_lookup_conversion(super::LinuxAgentProductionPeerInputPopulationError::from);

        let peer_source = super::LinuxAgentProductionPeerInputPopulationError::from(
            LinuxAgentRemotePeerDeviceSourceError::Missing,
        );
        assert!(matches!(
            peer_source,
            super::LinuxAgentProductionPeerInputPopulationError::PeerDeviceSource(
                LinuxAgentRemotePeerDeviceSourceError::Missing
            )
        ));
        assert_eq!(
            peer_source.to_string(),
            "production peer-device source failed"
        );
        assert!(std::error::Error::source(&peer_source).is_some());

        let lookup = super::LinuxAgentProductionPeerInputPopulationError::from(
            prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError::ReadUnavailable,
        );
        assert!(matches!(
            lookup,
            super::LinuxAgentProductionPeerInputPopulationError::DurableRegistryLookup(
                prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError::ReadUnavailable
            )
        ));
        assert_eq!(
            lookup.to_string(),
            "production durable-registry peer lookup failed"
        );
        assert!(std::error::Error::source(&lookup).is_some());
    }

    #[test]
    fn production_peer_input_population_helper_future_is_dormant_until_polled() {
        let (_sender, receiver) = mpsc::channel::<TestExpectedRequest>(1);
        let remote_process_inputs = LinuxAgentRemoteProcessOperationInputs::new(
            SocketAddr::from(([127, 0, 0, 1], 0)),
            NonZeroUsize::new(1).expect("nonzero test worker bound"),
            SharedCurrentCapabilityAuthority::new(
                WorkspaceDeviceRegistry::new(),
                BoundedLocalReadPolicy::allow_local_reads(),
            ),
            SessionAuthenticationService::new(),
            receiver,
            test_admission_timing as fn(&DeviceId) -> RemoteSessionRealAdmissionTiming,
            test_completion as fn(RemoteSessionRegisteredWorkerCompletion),
            test_rejection as fn(TestExpectedRejection),
            test_admission_failure as fn(RemoteSessionRepeatedAdmissionFailure),
        );

        let future = super::linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(
            remote_process_inputs,
        );
        drop(future);
    }

    #[test]
    fn initial_profile_matches_phase_101_lock() {
        let config = initial_runtime_config();
        assert_eq!(config.worker_capacity().get(), 2);
        assert_eq!(config.listener_backlog().get(), 8);
        assert_eq!(config.scheduling_attempt_budget().get(), 2);
        assert_eq!(config.worker_config().request_budget().get(), 1);
        assert_eq!(config.worker_config().read_budget().duration().as_secs(), 2);
        assert_eq!(
            config.worker_config().write_budget().duration().as_secs(),
            2
        );
        let capacity = LocalLinuxWorkerCapacity::new(config.worker_capacity());
        assert_eq!(capacity.max_workers(), 2);
    }

    #[test]
    fn phase_101_policy_allows_only_existing_local_reads() {
        let policy = BoundedLocalReadPolicy::allow_local_reads();
        assert_eq!(
            policy.evaluate(Capability::AgentStatusRead),
            Decision::Allow
        );
        assert_eq!(
            policy.evaluate(Capability::PrivateDnsConfigRead),
            Decision::Allow
        );
        for denied in [
            Capability::TerminalOpen,
            Capability::TerminalExec,
            Capability::FilesRead,
            Capability::FilesWrite,
            Capability::FilesDelete,
            Capability::ForwardingCreate,
            Capability::DeviceManage,
            Capability::PolicyManage,
        ] {
            assert_eq!(policy.evaluate(denied), Decision::Deny);
        }
    }

    #[test]
    fn success_requires_normal_terminal_clean_cleanup_and_restored_mask() {
        let success = LinuxAgentBootstrapReport {
            terminal: LinuxAgentBootstrapTerminal::SigTerm,
            counters: LinuxAgentBootstrapCounters::default(),
            cleanup: LinuxAgentBootstrapCleanup::Clean,
            signal_mask_restore: LinuxAgentBootstrapSignalMaskRestore::Restored,
        };
        assert!(success.is_success());

        for report in [
            LinuxAgentBootstrapReport {
                terminal: LinuxAgentBootstrapTerminal::RuntimeFatal,
                ..success
            },
            LinuxAgentBootstrapReport {
                cleanup: LinuxAgentBootstrapCleanup::Failed,
                ..success
            },
            LinuxAgentBootstrapReport {
                signal_mask_restore: LinuxAgentBootstrapSignalMaskRestore::Failed,
                ..success
            },
        ] {
            assert!(!report.is_success());
        }
    }

    #[test]
    fn second_instance_maps_to_stable_already_running_class() {
        assert_eq!(
            map_lifecycle_start_kind(LocalLinuxProductionLifecycleAssemblyError::InstanceLock(
                AgentInstanceLockError::AlreadyRunning,
            )),
            LinuxAgentBootstrapStartKind::AlreadyRunning
        );
    }

    #[test]
    fn public_run_retains_exact_no_companion_signature() {
        fn assert_run_signature(
            entry: fn() -> Result<LinuxAgentBootstrapReport, LinuxAgentBootstrapStartFailure>,
        ) {
            let _ = entry;
        }

        assert_run_signature(run);
    }

    #[test]
    fn public_remote_companion_facade_has_exact_injected_operation_shape() {
        type RemoteCompanionEntry =
            fn(
                fn(LinuxAgentRemoteSupervisorShutdownPublisher),
            )
                -> Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>;

        fn operation(_: LinuxAgentRemoteSupervisorShutdownPublisher) {}
        fn assert_signature(entry: RemoteCompanionEntry) {
            let _ = entry;
        }

        assert_signature(
            run_with_remote_process_companion::<fn(LinuxAgentRemoteSupervisorShutdownPublisher)>,
        );
        let _ = operation;
    }

    #[test]
    fn public_shutdown_publisher_has_exact_consuming_method_shape() {
        fn assert_signature(
            publish: fn(
                LinuxAgentRemoteSupervisorShutdownPublisher,
                RemoteSessionSupervisorShutdownController,
            ) -> LinuxAgentRemoteSupervisorShutdownPublish,
        ) {
            let _ = publish;
        }

        assert_signature(LinuxAgentRemoteSupervisorShutdownPublisher::publish);
    }

    #[test]
    fn injected_operation_factory_construction_is_side_effect_free_and_send_static() {
        let (_sender, receiver) = mpsc::channel::<TestExpectedRequest>(1);
        let inputs = LinuxAgentRemoteProcessOperationInputs::new(
            SocketAddr::from(([127, 0, 0, 1], 0)),
            NonZeroUsize::new(1).expect("nonzero test worker bound"),
            SharedCurrentCapabilityAuthority::new(
                WorkspaceDeviceRegistry::new(),
                BoundedLocalReadPolicy::allow_local_reads(),
            ),
            SessionAuthenticationService::new(),
            receiver,
            test_admission_timing as fn(&DeviceId) -> RemoteSessionRealAdmissionTiming,
            test_completion as fn(RemoteSessionRegisteredWorkerCompletion),
            test_rejection as fn(TestExpectedRejection),
            test_admission_failure as fn(RemoteSessionRepeatedAdmissionFailure),
        );

        let operation = linux_agent_remote_process_operation(inputs);
        assert_remote_operation_shape(operation);
        let _ = test_verifier_time as fn() -> u64;
    }

    #[test]
    fn production_operation_factory_construction_is_side_effect_free_and_send_static() {
        let peer = PeerConnectivityIdentity::new(
            DeviceId::new("c03e-ig-peer").expect("device"),
            TransportIdentity::new([0x49; 32]).expect("transport"),
        );
        let (_sender, receiver) = mpsc::channel::<TestExpectedRequest>(1);
        let remote_process_inputs = LinuxAgentRemoteProcessOperationInputs::new(
            SocketAddr::from(([127, 0, 0, 1], 0)),
            NonZeroUsize::new(1).expect("nonzero test worker bound"),
            SharedCurrentCapabilityAuthority::new(
                WorkspaceDeviceRegistry::new(),
                BoundedLocalReadPolicy::allow_local_reads(),
            ),
            SessionAuthenticationService::new(),
            receiver,
            test_admission_timing as fn(&DeviceId) -> RemoteSessionRealAdmissionTiming,
            test_completion as fn(RemoteSessionRegisteredWorkerCompletion),
            test_rejection as fn(TestExpectedRejection),
            test_admission_failure as fn(RemoteSessionRepeatedAdmissionFailure),
        );
        let inputs = LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(
            peer,
            remote_process_inputs,
        );

        let operation = linux_agent_production_reachability_remote_process_operation(inputs);
        assert_remote_operation_shape(operation);
    }

    #[test]
    fn production_requester_rendezvous_join_factory_construction_is_side_effect_free_and_send_static()
     {
        let peer = PeerConnectivityIdentity::new(
            DeviceId::new("c03e-ii-peer").expect("device"),
            TransportIdentity::new([0x52; 32]).expect("transport"),
        );
        let (_sender, receiver) = mpsc::channel::<TestExpectedRequest>(1);
        let remote_process_inputs = LinuxAgentRemoteProcessOperationInputs::new(
            SocketAddr::from(([127, 0, 0, 1], 0)),
            NonZeroUsize::new(1).expect("nonzero test worker bound"),
            SharedCurrentCapabilityAuthority::new(
                WorkspaceDeviceRegistry::new(),
                BoundedLocalReadPolicy::allow_local_reads(),
            ),
            SessionAuthenticationService::new(),
            receiver,
            test_admission_timing as fn(&DeviceId) -> RemoteSessionRealAdmissionTiming,
            test_completion as fn(RemoteSessionRegisteredWorkerCompletion),
            test_rejection as fn(TestExpectedRejection),
            test_admission_failure as fn(RemoteSessionRepeatedAdmissionFailure),
        );
        let production_inputs = LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(
            peer,
            remote_process_inputs,
        );
        let requester_rendezvous_start_policy_source =
            super::BoundedRequesterRendezvousStartPolicySource::default();
        let requester_rendezvous_runtime_owner =
            super::CandidatePublicationRequesterRendezvousRuntimeOwner::new(
                prw_remote_bridge::requester_rendezvous_in_memory_provider::InMemoryRequesterRendezvousAuthorityProvider::new(1)
                    .expect("explicit non-zero requester/rendezvous provider capacity"),
            );
        let inputs = super::LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(
            production_inputs,
            requester_rendezvous_start_policy_source,
            requester_rendezvous_runtime_owner,
        );

        let operation = super::linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs);
        assert_remote_operation_shape(operation);
    }

    #[test]
    fn synthetic_composition_preserves_same_executor_and_exact_stage_order() {
        let events = RefCell::new(Vec::new());

        let completed = run_remote_process_operation_composition(
            || {
                events.borrow_mut().push("executor");
                Ok::<_, ()>(31_u8)
            },
            |executor| {
                events.borrow_mut().push("bootstrap");
                assert_eq!(*executor, 31);
                Ok::<_, ()>(37_u8)
            },
            |executor, authority| {
                events.borrow_mut().push("endpoint");
                assert_eq!(executor, 31);
                assert_eq!(authority, 37);
                Ok::<_, ()>((41_u8, 43_u8))
            },
            |controller| {
                events.borrow_mut().push("publish");
                assert_eq!(controller, 43);
                LinuxAgentRemoteSupervisorShutdownPublish::Published
            },
            |endpoint, publication| {
                events.borrow_mut().push("lifecycle");
                assert_eq!(endpoint, 41);
                assert_eq!(
                    publication,
                    LinuxAgentRemoteSupervisorShutdownPublish::Published
                );
            },
        );

        assert!(completed);
        assert_eq!(
            *events.borrow(),
            ["executor", "bootstrap", "endpoint", "publish", "lifecycle"]
        );
    }

    #[test]
    fn synthetic_failures_suppress_all_later_stages() {
        let later = Cell::new(0_u8);
        let completed = run_remote_process_operation_composition(
            || Err::<u8, _>(()),
            |_executor| {
                later.set(later.get() + 1);
                Ok::<_, ()>(2_u8)
            },
            |_executor, _authority| {
                later.set(later.get() + 1);
                Ok::<_, ()>((3_u8, 4_u8))
            },
            |_controller| {
                later.set(later.get() + 1);
                5_u8
            },
            |_endpoint, _publication| later.set(later.get() + 1),
        );
        assert!(!completed);
        assert_eq!(later.get(), 0);

        let later = Cell::new(0_u8);
        let completed = run_remote_process_operation_composition(
            || Ok::<_, ()>(1_u8),
            |_executor| Err::<u8, _>(()),
            |_executor, _authority| {
                later.set(later.get() + 1);
                Ok::<_, ()>((3_u8, 4_u8))
            },
            |_controller| {
                later.set(later.get() + 1);
                5_u8
            },
            |_endpoint, _publication| later.set(later.get() + 1),
        );
        assert!(!completed);
        assert_eq!(later.get(), 0);

        let later = Cell::new(0_u8);
        let completed = run_remote_process_operation_composition(
            || Ok::<_, ()>(1_u8),
            |_executor| Ok::<_, ()>(2_u8),
            |_executor, _authority| Err::<(u8, u8), _>(()),
            |_controller| {
                later.set(later.get() + 1);
                5_u8
            },
            |_endpoint, _publication| later.set(later.get() + 1),
        );
        assert!(!completed);
        assert_eq!(later.get(), 0);
    }

    #[test]
    fn receiver_gone_publication_equivalent_still_drives_same_lifecycle_stage() {
        let lifecycle_called = Cell::new(false);

        let completed = run_remote_process_operation_composition(
            || Ok::<_, ()>(1_u8),
            |_executor| Ok::<_, ()>(2_u8),
            |_executor, _authority| Ok::<_, ()>((3_u8, 4_u8)),
            |_controller| LinuxAgentRemoteSupervisorShutdownPublish::ReceiverGoneShutdownRequested,
            |_endpoint, publication| {
                assert_eq!(
                    publication,
                    LinuxAgentRemoteSupervisorShutdownPublish::ReceiverGoneShutdownRequested
                );
                lifecycle_called.set(true);
            },
        );

        assert!(completed);
        assert!(lifecycle_called.get());
    }

    #[test]
    fn internal_publication_outcomes_map_to_bounded_public_classes() {
        assert_eq!(
            map_remote_shutdown_publish(RemoteSessionSupervisorShutdownPublish::Published),
            LinuxAgentRemoteSupervisorShutdownPublish::Published
        );
        assert_eq!(
            map_remote_shutdown_publish(
                RemoteSessionSupervisorShutdownPublish::ReceiverGoneShutdownRequested
            ),
            LinuxAgentRemoteSupervisorShutdownPublish::ReceiverGoneShutdownRequested
        );
    }

    #[test]
    fn synthetic_remote_process_spawn_failure_remains_secondary_evidence() {
        assert_eq!(
            finalize_remote_process_companion(Err(RemoteSessionProcessLifecycleSpawnError)),
            LinuxAgentRemoteProcessCompanionFinalization::SpawnFailed
        );
    }

    #[test]
    fn injected_remote_process_owner_maps_to_bounded_public_join_evidence() {
        let owner = RemoteSessionProcessLifecycleOwner::spawn(drop)
            .expect("injected non-networking remote process thread spawns");

        assert_eq!(
            finalize_remote_process_companion(Ok(owner)),
            LinuxAgentRemoteProcessCompanionFinalization::Finalized {
                controller:
                    LinuxAgentRemoteProcessControllerFinalization::UnavailableBeforeEndpointStartup,
                thread: LinuxAgentRemoteProcessThreadFinalization::Joined,
            }
        );
    }
}
