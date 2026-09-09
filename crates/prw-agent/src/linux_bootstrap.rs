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
    sync::Arc,
    time::Duration,
};

use prw_connectivity::PeerConnectivityIdentity;
use prw_core::DeviceId;
use prw_network::PrivateDnsConfig;
use prw_policy::{BoundedLocalReadPolicy, PolicyEvaluator};
use prw_remote_bridge::{AuthorizedCapabilityRequest, BridgeCommand, CapabilityDispatcher};
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
use crate::local_commands::status_snapshot::{
    LocalAgentRuntimeState, LocalAgentStatusSnapshot, codec::encode_status_snapshot,
};
use crate::production_durable_registry_runtime_custody::ProductionDurableCapabilityAuthority;
use crate::remote_session_capability_runtime::{
    RemoteSessionEndpointLifecycleRuntime, RemoteSessionExecutorRuntime,
    RemoteSessionExpectedDeviceAdmissionRejection,
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest, RemoteSessionRealAdmissionError,
    RemoteSessionRealAdmissionTiming, RemoteSessionRegisteredWorkerCompletion,
    RemoteSessionRepeatedAdmissionFailure,
    RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection,
    RemoteSessionSupervisorShutdownController, SharedCurrentCapabilityAuthority,
    SharedRequesterRendezvousAuthority,
    remote_session_process_lifecycle_control::{
        RemoteSessionProcessControllerFinalization, RemoteSessionProcessLifecycleFinalization,
        RemoteSessionProcessLifecycleOwner, RemoteSessionProcessLifecycleSpawnError,
        RemoteSessionProcessThreadFinalization, RemoteSessionSupervisorShutdownPublish,
        RemoteSessionSupervisorShutdownPublisher,
    },
};

/// Dormant owned status-only adapter selected by C03e-NA.
#[allow(
    dead_code,
    reason = "C03e-NB materializes the owned dispatcher before separately gated caller composition"
)]
pub(crate) struct LinuxAgentProductionRemoteCapabilityDispatcher {
    status_snapshot: LocalAgentStatusSnapshot,
}

impl LinuxAgentProductionRemoteCapabilityDispatcher {
    #[allow(
        dead_code,
        reason = "production dispatcher construction remains deferred after C03e-NB"
    )]
    pub(crate) const fn new(status_snapshot: LocalAgentStatusSnapshot) -> Self {
        Self { status_snapshot }
    }

    // Keep the pure command projection testable without constructing transport/session authority.
    #[allow(
        dead_code,
        reason = "C03e-NB command projection remains dormant until separately gated caller composition"
    )]
    fn dispatch_command(
        &self,
        command: &BridgeCommand,
    ) -> Result<Vec<u8>, LinuxAgentProductionRemoteCapabilityDispatchError> {
        match command {
            BridgeCommand::AgentStatus => Ok(encode_status_snapshot(self.status_snapshot).to_vec()),
            BridgeCommand::FileList(_)
            | BridgeCommand::FileStat(_)
            | BridgeCommand::FileCreate { .. }
            | BridgeCommand::DirectoryCreate(_)
            | BridgeCommand::UploadBegin(_)
            | BridgeCommand::UploadResume(_)
            | BridgeCommand::UploadChunk { .. }
            | BridgeCommand::UploadFinalize(_)
            | BridgeCommand::UploadAbort(_)
            | BridgeCommand::DownloadChunk { .. }
            | BridgeCommand::TerminalOpen { .. }
            | BridgeCommand::TerminalInput { .. }
            | BridgeCommand::TerminalResize { .. }
            | BridgeCommand::TerminalRead { .. }
            | BridgeCommand::TerminalClose(_)
            | BridgeCommand::ForwardOpen { .. }
            | BridgeCommand::ForwardClose(_) => {
                Err(LinuxAgentProductionRemoteCapabilityDispatchError::UnsupportedProviderFamily)
            }
        }
    }
}

/// Bounded zero-data failure for provider-backed commands outside the selected adapter surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "C03e-NB dispatch errors remain dormant until separately gated caller composition"
)]
pub(crate) enum LinuxAgentProductionRemoteCapabilityDispatchError {
    UnsupportedProviderFamily,
}

impl std::fmt::Display for LinuxAgentProductionRemoteCapabilityDispatchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("remote capability provider family unsupported")
    }
}

impl std::error::Error for LinuxAgentProductionRemoteCapabilityDispatchError {}

impl CapabilityDispatcher for LinuxAgentProductionRemoteCapabilityDispatcher {
    type Error = LinuxAgentProductionRemoteCapabilityDispatchError;

    fn dispatch(&mut self, request: &AuthorizedCapabilityRequest) -> Result<Vec<u8>, Self::Error> {
        self.dispatch_command(request.command())
    }
}

/// Fixed non-secret process configuration name for the production remote endpoint bind address.
pub const PRW_REMOTE_BIND_ADDR_ENV: &str = "PRW_REMOTE_BIND_ADDR";

/// Fixed non-secret process configuration name for the production remote peer logical device.
pub const PRW_REMOTE_PEER_DEVICE_ID_ENV: &str = "PRW_REMOTE_PEER_DEVICE_ID";

/// Fixed non-secret process configuration name for the production remote active-worker bound.
pub const PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV: &str = "PRW_REMOTE_MAX_ACTIVE_WORKERS";

/// Fixed non-secret process configuration name for the production requester/rendezvous record bound.
#[allow(
    dead_code,
    reason = "C03e-MR materializes the MQ-selected fixed requester/rendezvous max-records environment source before separately gated population composition"
)]
pub(crate) const PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV: &str =
    "PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS";

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

/// Bounded failure while acquiring or validating requester/rendezvous record capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "C03e-MR materializes the MQ-selected fixed requester/rendezvous max-records environment source before separately gated population composition"
)]
pub(crate) enum LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError {
    /// The fixed configuration value is absent.
    Missing,
    /// The operating-system value is not valid Unicode.
    NonUnicode,
    /// The configured value is empty, malformed, or outside target `usize`.
    InvalidValue,
}

impl std::fmt::Display for LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Missing => "remote requester/rendezvous max-records configuration missing",
            Self::NonUnicode => {
                "remote requester/rendezvous max-records configuration encoding invalid"
            }
            Self::InvalidValue => "remote requester/rendezvous max-records configuration invalid",
        })
    }
}

impl std::error::Error for LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError {}

#[allow(
    dead_code,
    reason = "C03e-MR materializes the MQ-selected strict ASCII-decimal requester/rendezvous max-records parser before separately gated population composition"
)]
fn parse_linux_agent_remote_requester_rendezvous_max_records_value(
    value: Option<OsString>,
) -> Result<usize, LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError> {
    let value = value.ok_or(LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError::Missing)?;
    let value = value
        .into_string()
        .map_err(|_| LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError::NonUnicode)?;
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError::InvalidValue);
    }
    value
        .parse::<usize>()
        .map_err(|_| LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError::InvalidValue)
}

/// Loads the explicitly configured production requester/rendezvous record bound.
///
/// The exact Unicode value must contain ASCII decimal digits only and is converted fail-closed to
/// target `usize`. Zero is returned unchanged; the existing requester/rendezvous provider
/// constructor remains the sole semantic authority for the non-zero capacity invariant. This
/// source performs no trimming, fallback, retry, alternate-variable lookup, worker-limit aliasing,
/// cache, refresh, provider construction, population composition, or runtime activation.
///
/// # Errors
///
/// Fails closed when the fixed configuration is missing, non-Unicode, empty, malformed, or outside
/// target `usize`. The bounded error surface does not expose the configured value.
#[allow(
    dead_code,
    reason = "C03e-MR materializes the MQ-selected fixed requester/rendezvous max-records environment loader before separately gated population composition"
)]
pub(crate) fn load_linux_agent_remote_requester_rendezvous_max_records_from_env()
-> Result<usize, LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError> {
    parse_linux_agent_remote_requester_rendezvous_max_records_value(std::env::var_os(
        PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV,
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
    #[must_use]
    pub const fn readiness_steps(self) -> u64 { self.readiness_steps }
    #[must_use]
    pub const fn listener_armed_steps(self) -> u64 { self.listener_armed_steps }
    #[must_use]
    pub const fn runtime_wakes(self) -> u64 { self.runtime_wakes }
    #[must_use]
    pub const fn wait_interruptions(self) -> u64 { self.wait_interruptions }
    #[must_use]
    pub const fn scheduling_attempts(self) -> u64 { self.scheduling_attempts }
    #[must_use]
    pub const fn workers_registered(self) -> u64 { self.workers_registered }
    #[must_use]
    pub const fn worker_completions(self) -> u64 { self.worker_completions }
    #[must_use]
    pub const fn peer_rejections(self) -> u64 { self.peer_rejections }
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
    #[must_use]
    pub const fn terminal(self) -> LinuxAgentBootstrapTerminal { self.terminal }
    #[must_use]
    pub const fn counters(self) -> LinuxAgentBootstrapCounters { self.counters }
    #[must_use]
    pub const fn cleanup(self) -> LinuxAgentBootstrapCleanup { self.cleanup }
    #[must_use]
    pub const fn signal_mask_restore(self) -> LinuxAgentBootstrapSignalMaskRestore { self.signal_mask_restore }
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.terminal.is_normal()
            && matches!(self.cleanup, LinuxAgentBootstrapCleanup::Clean)
            && matches!(self.signal_mask_restore, LinuxAgentBootstrapSignalMaskRestore::Restored)
    }
}

/// Bounded startup-failure class exposed to the standalone Agent binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentBootstrapStartKind {
    PrivateDnsSnapshot,
    SignalSource,
    RuntimeRoot,
    RuntimeDirectory,
    AlreadyRunning,
    InstanceLock,
    Bind,
    Listen,
    AcceptReady,
    RuntimeWake,
}

impl LinuxAgentBootstrapStartKind {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxAgentBootstrapStartFailure {
    kind: LinuxAgentBootstrapStartKind,
    signal_mask_restore: LinuxAgentBootstrapSignalMaskRestore,
}

impl LinuxAgentBootstrapStartFailure {
    const fn new(kind: LinuxAgentBootstrapStartKind, signal_mask_restore: LinuxAgentBootstrapSignalMaskRestore) -> Self {
        Self { kind, signal_mask_restore }
    }
    #[must_use]
    pub const fn kind(self) -> LinuxAgentBootstrapStartKind { self.kind }
    #[must_use]
    pub const fn signal_mask_restore(self) -> LinuxAgentBootstrapSignalMaskRestore { self.signal_mask_restore }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAgentRemoteSupervisorShutdownPublish {
    Published,
    ReceiverGoneShutdownRequested,
}

pub struct LinuxAgentRemoteSupervisorShutdownPublisher {
    publisher: RemoteSessionSupervisorShutdownPublisher,
}

impl LinuxAgentRemoteSupervisorShutdownPublisher {
    #[must_use]
    pub fn publish(self, controller: RemoteSessionSupervisorShutdownController) -> LinuxAgentRemoteSupervisorShutdownPublish {
        map_remote_shutdown_publish(self.publisher.publish(controller))
    }
}

fn run_remote_process_operation_composition<Executor, Authority, Endpoint, Controller, Publication, ExecutorError, BootstrapError, EndpointError>(
    construct_executor: impl FnOnce() -> Result<Executor, ExecutorError>,
    bootstrap_authority: impl FnOnce(&Executor) -> Result<Authority, BootstrapError>,
    start_endpoint: impl FnOnce(Executor, Authority) -> Result<(Endpoint, Controller), EndpointError>,
    publish_controller: impl FnOnce(Controller) -> Publication,
    drive_lifecycle: impl FnOnce(Endpoint, Publication),
) -> bool {
    let Ok(executor) = construct_executor() else { return false; };
    let Ok(authority) = bootstrap_authority(&executor) else { return false; };
    let Ok((endpoint, controller)) = start_endpoint(executor, authority) else { return false; };
    let publication = publish_controller(controller);
    drive_lifecycle(endpoint, publication);
    true
}

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
    #[must_use]
    #[expect(clippy::too_many_arguments, reason = "C03e-AZ keeps the selected injected remote-operation inputs explicit and typed")]
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
        Self { bind_addr, max_active_workers, capability_authority, session_authentication, expected_requests, admission_timing, on_completion, on_rejection, on_admission_failure }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity, dead_code, reason = "C03e-IM materializes the IL-selected production bind-address input population before separately gated remaining production provenance")]
pub(crate) fn linux_agent_remote_process_operation_inputs_from_production_bind_addr<P, D, T, F, C, R, E>(
    max_active_workers: NonZeroUsize,
    capability_authority: SharedCurrentCapabilityAuthority<P>,
    session_authentication: SessionAuthenticationService,
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>, LinuxAgentRemoteBindAddressSourceError> {
    let bind_addr = load_linux_agent_remote_bind_addr_from_env()?;
    Ok(LinuxAgentRemoteProcessOperationInputs::new(bind_addr, max_active_workers, capability_authority, session_authentication, expected_requests, admission_timing, on_completion, on_rejection, on_admission_failure))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinuxAgentProductionRemoteProcessInputPopulationError {
    WorkerLimitSource(LinuxAgentRemoteMaxActiveWorkersSourceError),
    BindAddressSource(LinuxAgentRemoteBindAddressSourceError),
}
impl std::fmt::Display for LinuxAgentProductionRemoteProcessInputPopulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(match self { Self::WorkerLimitSource(_) => "production worker-limit source failed", Self::BindAddressSource(_) => "production bind-address source failed" }) }
}
impl std::error::Error for LinuxAgentProductionRemoteProcessInputPopulationError {
    fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::WorkerLimitSource(e)=>Some(e),Self::BindAddressSource(e)=>Some(e)}}
}
impl From<LinuxAgentRemoteMaxActiveWorkersSourceError> for LinuxAgentProductionRemoteProcessInputPopulationError { fn from(e: LinuxAgentRemoteMaxActiveWorkersSourceError)->Self{Self::WorkerLimitSource(e)} }
impl From<LinuxAgentRemoteBindAddressSourceError> for LinuxAgentProductionRemoteProcessInputPopulationError { fn from(e: LinuxAgentRemoteBindAddressSourceError)->Self{Self::BindAddressSource(e)} }

#[allow(clippy::type_complexity, dead_code, reason = "C03e-JP materializes the JO-selected production worker-limit input population before separately gated remaining production provenance")]
pub(crate) fn linux_agent_remote_process_operation_inputs_from_production_worker_limit<P,D,T,F,C,R,E>(
    capability_authority: SharedCurrentCapabilityAuthority<P>, session_authentication: SessionAuthenticationService,
    expected_requests:mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D,T>>, admission_timing:F,on_completion:C,on_rejection:R,on_admission_failure:E,
)->Result<LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>,LinuxAgentProductionRemoteProcessInputPopulationError>{
    let max_active_workers=load_linux_agent_remote_max_active_workers_from_env()?;
    Ok(linux_agent_remote_process_operation_inputs_from_production_bind_addr(max_active_workers,capability_authority,session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure)?)
}

#[allow(dead_code, reason = "C03e-IG materializes the IF-selected production process-operation input owner before separately gated executable assembly")]
pub(crate) struct LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>{peer:PeerConnectivityIdentity,remote_process_inputs:LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>}
impl<P,D,T,F,C,R,E> LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>{
    #[must_use]
    #[allow(dead_code, reason = "C03e-IG materializes the IF-selected production process-operation input owner before separately gated executable assembly")]
    pub(crate) const fn new(peer:PeerConnectivityIdentity,remote_process_inputs:LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>)->Self{Self{peer,remote_process_inputs}}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub(crate) enum LinuxAgentProductionPeerInputPopulationError{PeerDeviceSource(LinuxAgentRemotePeerDeviceSourceError),DurableRegistryBootstrap(crate::production_durable_registry_custody_bootstrap::ProductionDurableRegistryCustodyBootstrapError),DurableRegistryLookup(prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError)}
impl std::fmt::Display for LinuxAgentProductionPeerInputPopulationError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(match self{Self::PeerDeviceSource(_)=>"production peer-device source failed",Self::DurableRegistryBootstrap(_)=>"production durable-registry bootstrap failed",Self::DurableRegistryLookup(_)=>"production durable-registry peer lookup failed"})}}
impl std::error::Error for LinuxAgentProductionPeerInputPopulationError{fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::PeerDeviceSource(e)=>Some(e),Self::DurableRegistryBootstrap(e)=>Some(e),Self::DurableRegistryLookup(e)=>Some(e)}}}
impl From<LinuxAgentRemotePeerDeviceSourceError> for LinuxAgentProductionPeerInputPopulationError{fn from(e:LinuxAgentRemotePeerDeviceSourceError)->Self{Self::PeerDeviceSource(e)}}
impl From<crate::production_durable_registry_custody_bootstrap::ProductionDurableRegistryCustodyBootstrapError> for LinuxAgentProductionPeerInputPopulationError{fn from(e:crate::production_durable_registry_custody_bootstrap::ProductionDurableRegistryCustodyBootstrapError)->Self{Self::DurableRegistryBootstrap(e)}}
impl From<prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError> for LinuxAgentProductionPeerInputPopulationError{fn from(e:prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStoreError)->Self{Self::DurableRegistryLookup(e)}}

#[allow(clippy::future_not_send,dead_code,reason="C03e-JK materializes the JJ-selected production peer input population before separately gated remaining production provenance")]
pub(crate) async fn linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer<P,D,T,F,C,R,E>(remote_process_inputs:LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>)->Result<LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>,LinuxAgentProductionPeerInputPopulationError>{
    let device_id=load_linux_agent_remote_peer_device_id_from_env()?;
    let store=crate::production_durable_registry_custody_bootstrap::bootstrap_production_durable_registry_from_systemd_credentials().await?;
    let mut registry_custody=crate::production_durable_registry_runtime_custody::ProductionDurableRegistryRuntimeCustody::from_store(store);
    let peer=registry_custody.peer_connectivity_identity(device_id).await?;
    Ok(LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(peer,remote_process_inputs))
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub(crate) enum LinuxAgentProductionReachabilityRemoteProcessInputPopulationError{RemoteProcessInputs(LinuxAgentProductionRemoteProcessInputPopulationError),PeerInput(LinuxAgentProductionPeerInputPopulationError)}
impl std::fmt::Display for LinuxAgentProductionReachabilityRemoteProcessInputPopulationError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(match self{Self::RemoteProcessInputs(_)=>"production remote-process input population failed",Self::PeerInput(_)=>"production peer input population failed"})}}
impl std::error::Error for LinuxAgentProductionReachabilityRemoteProcessInputPopulationError{fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::RemoteProcessInputs(e)=>Some(e),Self::PeerInput(e)=>Some(e)}}}
impl From<LinuxAgentProductionRemoteProcessInputPopulationError> for LinuxAgentProductionReachabilityRemoteProcessInputPopulationError{fn from(e:LinuxAgentProductionRemoteProcessInputPopulationError)->Self{Self::RemoteProcessInputs(e)}}
impl From<LinuxAgentProductionPeerInputPopulationError> for LinuxAgentProductionReachabilityRemoteProcessInputPopulationError{fn from(e:LinuxAgentProductionPeerInputPopulationError)->Self{Self::PeerInput(e)}}

#[allow(clippy::future_not_send,clippy::type_complexity,dead_code,reason="C03e-JR materializes the JQ-selected production reachability input population composition before separately gated remaining production provenance")]
pub(crate) async fn linux_agent_production_reachability_remote_process_operation_inputs_from_production_worker_limit_and_peer<P,D,T,F,C,R,E>(capability_authority:SharedCurrentCapabilityAuthority<P>,session_authentication:SessionAuthenticationService,expected_requests:mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D,T>>,admission_timing:F,on_completion:C,on_rejection:R,on_admission_failure:E)->Result<LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>,LinuxAgentProductionReachabilityRemoteProcessInputPopulationError>{
    let remote_process_inputs=linux_agent_remote_process_operation_inputs_from_production_worker_limit(capability_authority,session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure)?;
    Ok(linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(remote_process_inputs).await?)
}

pub fn linux_agent_remote_process_operation<P,D,T,F,C,R,E>(inputs:LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>)->impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static where P:PolicyEvaluator+Send+Sync+'static,D:CapabilityDispatcher+Send+'static,T:FnMut()->u64+Send+'static,F:FnMut(&DeviceId)->RemoteSessionRealAdmissionTiming+Send+'static,C:FnMut(RemoteSessionRegisteredWorkerCompletion)+Send+'static,R:FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>)+Send+'static,E:FnMut(RemoteSessionRepeatedAdmissionFailure)+Send+'static{
    move|publisher|{let LinuxAgentRemoteProcessOperationInputs{bind_addr,max_active_workers,capability_authority,mut session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure}=inputs;let _=run_remote_process_operation_composition(RemoteSessionExecutorRuntime::new,RemoteSessionExecutorRuntime::bootstrap_reachability_authority_from_systemd_credentials,move|executor,authority_owner|RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials(executor,authority_owner,bind_addr),move|controller|publisher.publish(controller),move|lifecycle,_publication|{let _=lifecycle.drive_repeated_real_remote_admission_endpoint_lifecycle(max_active_workers,&capability_authority,&mut session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure);});}
}

#[allow(dead_code,reason="C03e-IG materializes the IF-selected production process operation before separately gated executable assembly")]
pub(crate) fn linux_agent_production_reachability_remote_process_operation<P,D,T,F,C,R,E>(inputs:LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>)->impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static where P:PolicyEvaluator+Send+Sync+'static,D:CapabilityDispatcher+Send+'static,T:FnMut()->u64+Send+'static,F:FnMut(&DeviceId)->RemoteSessionRealAdmissionTiming+Send+'static,C:FnMut(RemoteSessionRegisteredWorkerCompletion)+Send+'static,R:FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>)+Send+'static,E:FnMut(RemoteSessionRepeatedAdmissionFailure)+Send+'static{
    move|publisher|{let LinuxAgentProductionReachabilityRemoteProcessOperationInputs{peer,remote_process_inputs}=inputs;let LinuxAgentRemoteProcessOperationInputs{bind_addr,max_active_workers,capability_authority,mut session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure}=remote_process_inputs;let _=run_remote_process_operation_composition(RemoteSessionExecutorRuntime::new,move|executor|executor.bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&peer),move|executor,runtime_custody|runtime_custody.bind_remote_endpoint_with_executor_from_systemd_credentials(executor,bind_addr),move|controller|publisher.publish(controller),move|lifecycle,_publication|{let _=lifecycle.drive_repeated_real_remote_admission_endpoint_lifecycle(max_active_workers,&capability_authority,&mut session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure);});}
}

#[allow(dead_code,reason="C03e-NQ migrates production requester/rendezvous custody to one preconstructed shared authority before separately gated scheduling-authority derivation")]
pub(crate) struct LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>{
    production_inputs:LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>,
    requester_rendezvous_start_policy_source:BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_authority:SharedRequesterRendezvousAuthority,
}
impl<P,D,T,F,C,R,E> LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>{
    #[must_use]
    #[allow(dead_code,reason="C03e-NQ retains one preconstructed shared requester authority without invoking it")]
    pub(crate) const fn new(production_inputs:LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>,requester_rendezvous_start_policy_source:BoundedRequesterRendezvousStartPolicySource,requester_rendezvous_authority:SharedRequesterRendezvousAuthority)->Self{Self{production_inputs,requester_rendezvous_start_policy_source,requester_rendezvous_authority}}
}

#[allow(dead_code,reason="C03e-NQ preserves dormant non-projection production custody with the preconstructed shared authority")]
pub(crate) fn linux_agent_production_reachability_requester_rendezvous_remote_process_operation<P,D,T,F,C,R,E>(inputs:LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>)->impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static where P:PolicyEvaluator+Send+Sync+'static,D:CapabilityDispatcher+Send+'static,T:FnMut()->u64+Send+'static,F:FnMut(&DeviceId)->RemoteSessionRealAdmissionTiming+Send+'static,C:FnMut(RemoteSessionRegisteredWorkerCompletion)+Send+'static,R:FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>)+Send+'static,E:FnMut(RemoteSessionRepeatedAdmissionFailure)+Send+'static{
    let LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs{production_inputs,requester_rendezvous_start_policy_source,requester_rendezvous_authority}=inputs;
    let operation=linux_agent_production_reachability_remote_process_operation(production_inputs);
    move|publisher|{drop(requester_rendezvous_authority);drop(requester_rendezvous_start_policy_source);operation(publisher);}
}

#[allow(dead_code,reason="C03e-LT materializes the LS-selected dormant Linux projection operation before separately gated higher-owner caller migration")]
pub(crate) fn linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection<P,D,T,F,C,R,E>(inputs:LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>,production_durable_capability_authority:Arc<ProductionDurableCapabilityAuthority>)->impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static where P:PolicyEvaluator+Send+Sync+'static,D:CapabilityDispatcher+Send+'static,T:FnMut()->u64+Send+'static,F:FnMut(&DeviceId)->RemoteSessionRealAdmissionTiming+Send+'static,C:FnMut(DeviceId,RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)+Send+'static,R:FnMut(RemoteSessionExpectedDeviceAdmissionRejectionReason,RemoteSessionExpectedDeviceAdmissionRequest<D,T>)+Send+'static,E:FnMut(DeviceId,RemoteSessionRealAdmissionError)+Send+'static{
    let LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs{production_inputs,requester_rendezvous_start_policy_source,requester_rendezvous_authority}=inputs;
    let requester_rendezvous_start_policy_source=Arc::new(requester_rendezvous_start_policy_source);
    move|publisher|{let LinuxAgentProductionReachabilityRemoteProcessOperationInputs{peer,remote_process_inputs}=production_inputs;let LinuxAgentRemoteProcessOperationInputs{bind_addr,max_active_workers,capability_authority,mut session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure}=remote_process_inputs;let _=run_remote_process_operation_composition(RemoteSessionExecutorRuntime::new,move|executor|executor.bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&peer),move|executor,runtime_custody|runtime_custody.bind_remote_endpoint_with_executor_from_systemd_credentials(executor,bind_addr),move|controller|publisher.publish(controller),move|lifecycle,_publication|{let _=lifecycle.drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection(max_active_workers,&capability_authority,production_durable_capability_authority,requester_rendezvous_start_policy_source,&requester_rendezvous_authority,&mut session_authentication,expected_requests,admission_timing,on_completion,on_rejection,on_admission_failure);});}
}

#[allow(dead_code,reason="C03e-EB materializes requester-policy custody before separately gated production assembly")]
pub(crate) struct LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>{remote_process_inputs:LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>,requester_rendezvous_start_policy_source:BoundedRequesterRendezvousStartPolicySource,requester_rendezvous_runtime_owner:CandidatePublicationRequesterRendezvousRuntimeOwner}
impl<P,D,T,F,C,R,E> LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>{#[must_use]#[allow(dead_code,reason="C03e-EB materializes requester-policy custody before separately gated production assembly")]pub(crate) const fn new(remote_process_inputs:LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>,requester_rendezvous_start_policy_source:BoundedRequesterRendezvousStartPolicySource,requester_rendezvous_runtime_owner:CandidatePublicationRequesterRendezvousRuntimeOwner)->Self{Self{remote_process_inputs,requester_rendezvous_start_policy_source,requester_rendezvous_runtime_owner}}}
#[allow(dead_code,reason="C03e-EB materializes requester-policy custody before separately gated production assembly")]
pub(crate) fn linux_agent_requester_rendezvous_remote_process_operation<P,D,T,F,C,R,E>(inputs:LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>)->impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static where P:PolicyEvaluator+Send+Sync+'static,D:CapabilityDispatcher+Send+'static,T:FnMut()->u64+Send+'static,F:FnMut(&DeviceId)->RemoteSessionRealAdmissionTiming+Send+'static,C:FnMut(RemoteSessionRegisteredWorkerCompletion)+Send+'static,R:FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>)+Send+'static,E:FnMut(RemoteSessionRepeatedAdmissionFailure)+Send+'static{let LinuxAgentRequesterRendezvousRemoteProcessOperationInputs{remote_process_inputs,requester_rendezvous_start_policy_source,requester_rendezvous_runtime_owner}=inputs;let operation=linux_agent_remote_process_operation(remote_process_inputs);move|publisher|{drop(requester_rendezvous_runtime_owner);drop(requester_rendezvous_start_policy_source);operation(publisher);}}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum LinuxAgentRemoteProcessControllerFinalization{ShutdownRequested,UnavailableBeforeEndpointStartup}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum LinuxAgentRemoteProcessThreadFinalization{Joined,Panicked}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum LinuxAgentRemoteProcessCompanionFinalization{SpawnFailed,Finalized{controller:LinuxAgentRemoteProcessControllerFinalization,thread:LinuxAgentRemoteProcessThreadFinalization}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct LinuxAgentBootstrapWithRemoteReport{local:LinuxAgentBootstrapReport,remote:LinuxAgentRemoteProcessCompanionFinalization}
impl LinuxAgentBootstrapWithRemoteReport{#[must_use]pub const fn local(self)->LinuxAgentBootstrapReport{self.local}#[must_use]pub const fn remote(self)->LinuxAgentRemoteProcessCompanionFinalization{self.remote}}

fn with_initial_runtime_inputs<R>(operation:impl FnOnce(LocalLinuxProductionRuntimeInputs<'_>)->Result<R,LinuxAgentBootstrapStartFailure>)->Result<R,LinuxAgentBootstrapStartFailure>{let private_dns_config=PrivateDnsConfig::default();let private_dns_snapshot=LocalPrivateDnsSnapshot::try_from_config(&private_dns_config).map_err(|_|LinuxAgentBootstrapStartFailure::new(LinuxAgentBootstrapStartKind::PrivateDnsSnapshot,LinuxAgentBootstrapSignalMaskRestore::NotApplicable))?;let inputs=LocalLinuxProductionRuntimeInputs::new(initial_runtime_config(),BoundedLocalReadPolicy::allow_local_reads(),LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready),&private_dns_snapshot);operation(inputs)}
pub fn run()->Result<LinuxAgentBootstrapReport,LinuxAgentBootstrapStartFailure>{with_initial_runtime_inputs(|inputs|run_signal_aware_linux_production_runtime_from_env(inputs,|_|{}).map(|report|map_terminal_report(&report)).map_err(map_start_failure))}
pub fn run_with_remote_process_companion<F>(operation:F)->Result<LinuxAgentBootstrapWithRemoteReport,LinuxAgentBootstrapStartFailure> where F:FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static{with_initial_runtime_inputs(|inputs|run_with_remote_process_companion_inputs(inputs,operation).map(|(local,remote)|LinuxAgentBootstrapWithRemoteReport{local,remote}))}
#[allow(dead_code,reason="C03e-IK materializes the IJ-selected dormant executable assembly before separately gated caller/input assembly")]
pub(crate) fn run_with_production_reachability_requester_rendezvous_remote_process_companion<P,D,T,F,C,R,E>(inputs:LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>)->Result<LinuxAgentBootstrapWithRemoteReport,LinuxAgentBootstrapStartFailure> where P:PolicyEvaluator+Send+Sync+'static,D:CapabilityDispatcher+Send+'static,T:FnMut()->u64+Send+'static,F:FnMut(&DeviceId)->RemoteSessionRealAdmissionTiming+Send+'static,C:FnMut(RemoteSessionRegisteredWorkerCompletion)+Send+'static,R:FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>)+Send+'static,E:FnMut(RemoteSessionRepeatedAdmissionFailure)+Send+'static{let operation=linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs);run_with_remote_process_companion(operation)}
fn run_with_remote_process_companion_inputs<F>(inputs:LocalLinuxProductionRuntimeInputs<'_>,operation:F)->Result<(LinuxAgentBootstrapReport,LinuxAgentRemoteProcessCompanionFinalization),LinuxAgentBootstrapStartFailure> where F:FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static{let mut remote_finalization=None;let report=run_signal_aware_linux_production_runtime_from_env_with_companion(inputs,|_|{},||RemoteSessionProcessLifecycleOwner::spawn(move|publisher|operation(LinuxAgentRemoteSupervisorShutdownPublisher{publisher})),|companion|remote_finalization=Some(finalize_remote_process_companion(companion))).map_err(map_start_failure)?;let remote_finalization=remote_finalization.expect("signal-aware companion finalizer runs before successful bootstrap return");Ok((map_terminal_report(&report),remote_finalization))}
const fn map_remote_shutdown_publish(publish:RemoteSessionSupervisorShutdownPublish)->LinuxAgentRemoteSupervisorShutdownPublish{match publish{RemoteSessionSupervisorShutdownPublish::Published=>LinuxAgentRemoteSupervisorShutdownPublish::Published,RemoteSessionSupervisorShutdownPublish::ReceiverGoneShutdownRequested=>LinuxAgentRemoteSupervisorShutdownPublish::ReceiverGoneShutdownRequested}}
const fn map_remote_process_controller_finalization(controller:RemoteSessionProcessControllerFinalization)->LinuxAgentRemoteProcessControllerFinalization{match controller{RemoteSessionProcessControllerFinalization::ShutdownRequested=>LinuxAgentRemoteProcessControllerFinalization::ShutdownRequested,RemoteSessionProcessControllerFinalization::UnavailableBeforeEndpointStartup=>LinuxAgentRemoteProcessControllerFinalization::UnavailableBeforeEndpointStartup}}
const fn map_remote_process_thread_finalization(thread:RemoteSessionProcessThreadFinalization)->LinuxAgentRemoteProcessThreadFinalization{match thread{RemoteSessionProcessThreadFinalization::Joined=>LinuxAgentRemoteProcessThreadFinalization::Joined,RemoteSessionProcessThreadFinalization::Panicked=>LinuxAgentRemoteProcessThreadFinalization::Panicked}}
const fn map_remote_process_finalization(finalization:RemoteSessionProcessLifecycleFinalization)->LinuxAgentRemoteProcessCompanionFinalization{LinuxAgentRemoteProcessCompanionFinalization::Finalized{controller:map_remote_process_controller_finalization(finalization.controller()),thread:map_remote_process_thread_finalization(finalization.thread())}}
fn finalize_remote_process_companion(companion:Result<RemoteSessionProcessLifecycleOwner,RemoteSessionProcessLifecycleSpawnError>)->LinuxAgentRemoteProcessCompanionFinalization{companion.map_or(LinuxAgentRemoteProcessCompanionFinalization::SpawnFailed,|owner|map_remote_process_finalization(owner.finalize()))}
fn initial_runtime_config()->LocalLinuxProductionRuntimeConfig{LocalLinuxProductionRuntimeConfig::new(NonZeroUsize::new(2).expect("Phase 101 worker capacity is non-zero"),NonZeroU16::new(8).expect("Phase 101 listener backlog is non-zero"),NonZeroUsize::new(2).expect("Phase 101 scheduling budget is non-zero"),NonZeroUsize::new(1).expect("Phase 101 request budget is non-zero"),LocalLinuxIoBudget::try_new(Duration::from_secs(2)).expect("Phase 101 read I/O budget is non-zero"),LocalLinuxIoBudget::try_new(Duration::from_secs(2)).expect("Phase 101 write I/O budget is non-zero"))}
const fn map_terminal_report(report:&crate::linux_identity::signal_aware_runtime::LocalLinuxSignalAwareRuntimeTerminalReport)->LinuxAgentBootstrapReport{LinuxAgentBootstrapReport{terminal:match report.reason(){LocalLinuxSignalAwareRuntimeTerminalReason::ProgrammaticShutdown=>LinuxAgentBootstrapTerminal::ProgrammaticShutdown,LocalLinuxSignalAwareRuntimeTerminalReason::TerminationSignal(LocalLinuxTerminationSignal::SigTerm)=>LinuxAgentBootstrapTerminal::SigTerm,LocalLinuxSignalAwareRuntimeTerminalReason::TerminationSignal(LocalLinuxTerminationSignal::SigInt)=>LinuxAgentBootstrapTerminal::SigInt,LocalLinuxSignalAwareRuntimeTerminalReason::ReadinessFatal(_)=>LinuxAgentBootstrapTerminal::ReadinessFatal,LocalLinuxSignalAwareRuntimeTerminalReason::RuntimeFatal(_)=>LinuxAgentBootstrapTerminal::RuntimeFatal},counters:map_counters(report.counters()),cleanup:map_cleanup(report.cleanup()),signal_mask_restore:map_signal_mask_restore(report.mask_restore())}}
const fn map_counters(counters:LocalLinuxProductionRuntimeCounters)->LinuxAgentBootstrapCounters{LinuxAgentBootstrapCounters{readiness_steps:counters.readiness_steps(),listener_armed_steps:counters.listener_armed_steps(),runtime_wakes:counters.runtime_wakes(),wait_interruptions:counters.wait_interruptions(),scheduling_attempts:counters.scheduling_attempts(),workers_registered:counters.workers_registered(),worker_completions:counters.worker_completions(),peer_rejections:counters.peer_rejections()}}
const fn map_cleanup(cleanup:LocalLinuxProductionRuntimeCleanup)->LinuxAgentBootstrapCleanup{match cleanup{LocalLinuxProductionRuntimeCleanup::Clean=>LinuxAgentBootstrapCleanup::Clean,LocalLinuxProductionRuntimeCleanup::Failed(_)=>LinuxAgentBootstrapCleanup::Failed}}
const fn map_signal_mask_restore(restore:LocalLinuxTerminationSignalMaskRestore)->LinuxAgentBootstrapSignalMaskRestore{match restore{LocalLinuxTerminationSignalMaskRestore::Restored=>LinuxAgentBootstrapSignalMaskRestore::Restored,LocalLinuxTerminationSignalMaskRestore::Failed=>LinuxAgentBootstrapSignalMaskRestore::Failed}}
const fn map_start_failure(error:LocalLinuxSignalAwareRuntimeStartError)->LinuxAgentBootstrapStartFailure{match error{LocalLinuxSignalAwareRuntimeStartError::SignalSource(error)=>match error{LocalLinuxTerminationSignalSourceCreateError::MaskBlockFailed=>LinuxAgentBootstrapStartFailure::new(LinuxAgentBootstrapStartKind::SignalSource,LinuxAgentBootstrapSignalMaskRestore::NotApplicable),LocalLinuxTerminationSignalSourceCreateError::DescriptorCreateFailed{mask_restore}=>LinuxAgentBootstrapStartFailure::new(LinuxAgentBootstrapStartKind::SignalSource,map_signal_mask_restore(mask_restore))},LocalLinuxSignalAwareRuntimeStartError::Lifecycle{error,mask_restore}=>LinuxAgentBootstrapStartFailure::new(map_lifecycle_start_kind(error),map_signal_mask_restore(mask_restore))}}
const fn map_lifecycle_start_kind(error:LocalLinuxProductionLifecycleAssemblyError)->LinuxAgentBootstrapStartKind{match error{LocalLinuxProductionLifecycleAssemblyError::RuntimeRoot(_)=>LinuxAgentBootstrapStartKind::RuntimeRoot,LocalLinuxProductionLifecycleAssemblyError::RuntimeDirectory(_)=>LinuxAgentBootstrapStartKind::RuntimeDirectory,LocalLinuxProductionLifecycleAssemblyError::InstanceLock(AgentInstanceLockError::AlreadyRunning)=>LinuxAgentBootstrapStartKind::AlreadyRunning,LocalLinuxProductionLifecycleAssemblyError::InstanceLock(_)=>LinuxAgentBootstrapStartKind::InstanceLock,LocalLinuxProductionLifecycleAssemblyError::Bind(_)=>LinuxAgentBootstrapStartKind::Bind,LocalLinuxProductionLifecycleAssemblyError::Listen{..}=>LinuxAgentBootstrapStartKind::Listen,LocalLinuxProductionLifecycleAssemblyError::AcceptReady{..}=>LinuxAgentBootstrapStartKind::AcceptReady,LocalLinuxProductionLifecycleAssemblyError::RuntimeWake{..}=>LinuxAgentBootstrapStartKind::RuntimeWake}}

#[cfg(test)]
mod tests {
    use std::{cell::{Cell,RefCell},ffi::OsString,net::{Ipv4Addr,Ipv6Addr,SocketAddr},num::NonZeroUsize};
    #[cfg(unix)]use std::os::unix::ffi::OsStringExt;
    use prw_connectivity::{PeerConnectivityIdentity,TransportIdentity};
    use prw_core::DeviceId;
    use prw_policy::{BoundedLocalReadPolicy,Capability,Decision,PolicyEvaluator};
    use prw_registry::WorkspaceDeviceRegistry;
    use prw_remote_bridge::{AuthorizedCapabilityRequest,CapabilityDispatcher};
    use prw_session::SessionAuthenticationService;
    use tokio::sync::mpsc;
    use super::*;
    use crate::linux_identity::production_lifecycle::LocalLinuxProductionLifecycleAssemblyError;
    use crate::linux_identity::worker_capacity::LocalLinuxWorkerCapacity;
    use crate::linux_identity::xdg_runtime_root::prw_runtime_directory::agent_instance_lock::AgentInstanceLockError;
    use crate::remote_session_capability_runtime::{RemoteSessionExpectedDeviceAdmissionRejection,RemoteSessionExpectedDeviceAdmissionRequest,RemoteSessionRealAdmissionTiming,RemoteSessionRegisteredWorkerCompletion,RemoteSessionRepeatedAdmissionFailure,RemoteSessionSupervisorShutdownController,SharedCurrentCapabilityAuthority,SharedRequesterRendezvousAuthority,remote_session_process_lifecycle_control::{RemoteSessionProcessLifecycleOwner,RemoteSessionProcessLifecycleSpawnError,RemoteSessionSupervisorShutdownPublish}};

    struct TestDispatcher;
    impl CapabilityDispatcher for TestDispatcher{type Error=();fn dispatch(&mut self,_:&AuthorizedCapabilityRequest)->Result<Vec<u8>,Self::Error>{Ok(Vec::new())}}
    type TestExpectedRequest=RemoteSessionExpectedDeviceAdmissionRequest<TestDispatcher,fn()->u64>;
    type TestExpectedRejection=RemoteSessionExpectedDeviceAdmissionRejection<TestDispatcher,fn()->u64>;
    fn test_verifier_time()->u64{1}
    fn test_admission_timing(_: &DeviceId)->RemoteSessionRealAdmissionTiming{RemoteSessionRealAdmissionTiming::new(1..2,1,1..2)}
    fn test_completion(_:RemoteSessionRegisteredWorkerCompletion){}
    fn test_rejection(_:TestExpectedRejection){}
    fn test_admission_failure(_:RemoteSessionRepeatedAdmissionFailure){}
    fn assert_remote_operation_shape<F>(operation:F) where F:FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher)+Send+'static{drop(operation)}

    #[test] fn remote_bind_source_public_reader_has_exact_selected_shape(){fn assert_signature(_:fn()->Result<SocketAddr,LinuxAgentRemoteBindAddressSourceError>){}assert_eq!(PRW_REMOTE_BIND_ADDR_ENV,"PRW_REMOTE_BIND_ADDR");assert_signature(load_linux_agent_remote_bind_addr_from_env)}
    #[test] fn remote_max_active_workers_source_public_reader_has_exact_selected_shape(){fn assert_signature(_:fn()->Result<NonZeroUsize,LinuxAgentRemoteMaxActiveWorkersSourceError>){}assert_eq!(PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV,"PRW_REMOTE_MAX_ACTIVE_WORKERS");assert_signature(load_linux_agent_remote_max_active_workers_from_env)}
    #[test] fn production_requester_rendezvous_join_factory_construction_is_side_effect_free_and_send_static(){let peer=PeerConnectivityIdentity::new(DeviceId::new("c03e-ii-peer").expect("device"),TransportIdentity::new([0x52;32]).expect("transport"));let(_sender,receiver)=mpsc::channel::<TestExpectedRequest>(1);let remote_process_inputs=LinuxAgentRemoteProcessOperationInputs::new(SocketAddr::from(([127,0,0,1],0)),NonZeroUsize::new(1).expect("nonzero test worker bound"),SharedCurrentCapabilityAuthority::new(WorkspaceDeviceRegistry::new(),BoundedLocalReadPolicy::allow_local_reads()),SessionAuthenticationService::new(),receiver,test_admission_timing as fn(&DeviceId)->RemoteSessionRealAdmissionTiming,test_completion as fn(RemoteSessionRegisteredWorkerCompletion),test_rejection as fn(TestExpectedRejection),test_admission_failure as fn(RemoteSessionRepeatedAdmissionFailure));let production_inputs=LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(peer,remote_process_inputs);let requester_rendezvous_start_policy_source=BoundedRequesterRendezvousStartPolicySource::default();let requester_rendezvous_authority=SharedRequesterRendezvousAuthority::new(CandidatePublicationRequesterRendezvousRuntimeOwner::new(prw_remote_bridge::requester_rendezvous_in_memory_provider::InMemoryRequesterRendezvousAuthorityProvider::new(1).expect("explicit non-zero requester/rendezvous provider capacity")),1).expect("explicit non-zero scheduling-consumption capacity");let inputs=LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(production_inputs,requester_rendezvous_start_policy_source,requester_rendezvous_authority);let operation=linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs);assert_remote_operation_shape(operation)}
    #[test] fn initial_profile_matches_phase_101_lock(){let config=initial_runtime_config();assert_eq!(config.worker_capacity().get(),2);assert_eq!(config.listener_backlog().get(),8);assert_eq!(config.scheduling_attempt_budget().get(),2);assert_eq!(config.worker_config().request_budget().get(),1);let capacity=LocalLinuxWorkerCapacity::new(config.worker_capacity());assert_eq!(capacity.max_workers(),2)}
    #[test] fn phase_101_policy_allows_only_existing_local_reads(){let policy=BoundedLocalReadPolicy::allow_local_reads();assert_eq!(policy.evaluate(Capability::AgentStatusRead),Decision::Allow);assert_eq!(policy.evaluate(Capability::PrivateDnsConfigRead),Decision::Allow);for denied in [Capability::TerminalOpen,Capability::TerminalExec,Capability::FilesRead,Capability::FilesWrite,Capability::FilesDelete,Capability::ForwardingCreate,Capability::DeviceManage,Capability::PolicyManage]{assert_eq!(policy.evaluate(denied),Decision::Deny)}}
    #[test] fn synthetic_remote_process_spawn_failure_remains_secondary_evidence(){assert_eq!(finalize_remote_process_companion(Err(RemoteSessionProcessLifecycleSpawnError)),LinuxAgentRemoteProcessCompanionFinalization::SpawnFailed)}
    #[test] fn injected_remote_process_owner_maps_to_bounded_public_join_evidence(){let owner=RemoteSessionProcessLifecycleOwner::spawn(drop).expect("injected non-networking remote process thread spawns");assert_eq!(finalize_remote_process_companion(Ok(owner)),LinuxAgentRemoteProcessCompanionFinalization::Finalized{controller:LinuxAgentRemoteProcessControllerFinalization::UnavailableBeforeEndpointStartup,thread:LinuxAgentRemoteProcessThreadFinalization::Joined})}
    #[test] fn helper_keeps_selected_types_reachable(){let _=test_verifier_time as fn()->u64;let _=std::mem::size_of::<RefCell<Vec<u8>>>();let _=Ipv4Addr::LOCALHOST;let _=Ipv6Addr::LOCALHOST;let _=OsString::new();let _=LocalLinuxProductionLifecycleAssemblyError::RuntimeRoot;let _=AgentInstanceLockError::AlreadyRunning;let _=RemoteSessionSupervisorShutdownPublish::Published;}
}

/// Fixed non-secret process configuration name for the production expected-device
/// scheduling-consumption terminal-record bound.
#[allow(dead_code,reason="C03e-NN materializes the NM-selected fixed expected-device scheduling-consumption max-records environment source before separately gated ledger representation and population composition")]
pub(crate) const PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_ENV:&str="PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS";
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[allow(dead_code,reason="C03e-NN materializes the NM-selected bounded scheduling-consumption capacity source error before separately gated ledger representation")]
pub(crate) enum LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError{Missing,NonUnicode,InvalidValue}
impl std::fmt::Display for LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(match self{Self::Missing=>"remote expected-device scheduling-consumption max-records configuration missing",Self::NonUnicode=>"remote expected-device scheduling-consumption max-records configuration encoding invalid",Self::InvalidValue=>"remote expected-device scheduling-consumption max-records configuration invalid"})}}
impl std::error::Error for LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError{}
#[allow(dead_code,reason="C03e-NN materializes the NM-selected strict ASCII-decimal scheduling-consumption max-records parser before separately gated ledger representation")]
fn parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(value:Option<OsString>)->Result<usize,LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError>{let value=value.ok_or(LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::Missing)?;let value=value.into_string().map_err(|_|LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::NonUnicode)?;if value.is_empty()||!value.bytes().all(|b|b.is_ascii_digit()){return Err(LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::InvalidValue)}value.parse::<usize>().map_err(|_|LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::InvalidValue)}
#[allow(dead_code,reason="C03e-NN materializes the NM-selected fixed scheduling-consumption max-records environment loader before separately gated ledger representation and population composition")]
pub(crate) fn load_linux_agent_remote_expected_device_scheduling_consumption_max_records_from_env()->Result<usize,LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError>{parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(std::env::var_os(PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_ENV))}

#[cfg(test)]
mod expected_device_scheduling_consumption_capacity_source_tests{
    use std::ffi::OsString;
    #[cfg(unix)]use std::os::unix::ffi::OsStringExt;
    use super::{LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError,PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_ENV,load_linux_agent_remote_expected_device_scheduling_consumption_max_records_from_env,parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value};
    #[test]fn source_has_exact_selected_shape(){fn assert_signature(_:fn()->Result<usize,LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError>){}assert_eq!(PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_ENV,"PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS");assert_signature(load_linux_agent_remote_expected_device_scheduling_consumption_max_records_from_env)}
    #[test]fn source_rejects_missing_empty_and_malformed_values(){assert_eq!(parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(None),Err(LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::Missing));assert_eq!(parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(Some(OsString::new())),Err(LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::InvalidValue));for malformed in [" 1","1 ","+1","-1","1.0","1_0","1e1","1a"]{assert_eq!(parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(Some(OsString::from(malformed))),Err(LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::InvalidValue))}}
    #[cfg(unix)]#[test]fn source_rejects_non_unicode_value(){assert_eq!(parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(Some(OsString::from_vec(vec![0xff]))),Err(LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::NonUnicode))}
    #[test]fn source_preserves_zero_and_exact_magnitude(){for(value,expected)in[("0",0),("0000",0),("17",17),("00017",17)]{assert_eq!(parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(Some(OsString::from(value))),Ok(expected))}assert_eq!(parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(Some(OsString::from(usize::MAX.to_string()))),Ok(usize::MAX))}
    #[test]fn source_rejects_target_usize_overflow(){let overflow=format!("{}0",usize::MAX);assert_eq!(parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(Some(OsString::from(overflow))),Err(LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::InvalidValue))}
}
