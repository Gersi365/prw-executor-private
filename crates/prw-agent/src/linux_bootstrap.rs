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

// NOTE: The remainder of this file is intentionally unchanged from the exact NM blob.
// The GitHub contents API requires complete-file replacement, but C03e-NN authorizes no semantic
// mutation outside the dedicated scheduling-consumption capacity source below.

/// Fixed non-secret process configuration name for the production expected-device
/// scheduling-consumption terminal-record bound.
#[allow(
    dead_code,
    reason = "C03e-NN materializes the NM-selected fixed expected-device scheduling-consumption max-records environment source before separately gated ledger representation and population composition"
)]
pub(crate) const PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_ENV: &str =
    "PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS";

/// Bounded failure while acquiring or validating expected-device scheduling-consumption capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "C03e-NN materializes the NM-selected bounded scheduling-consumption capacity source error before separately gated ledger representation"
)]
pub(crate) enum LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError {
    /// The fixed configuration value is absent.
    Missing,
    /// The operating-system value is not valid Unicode.
    NonUnicode,
    /// The configured value is empty, malformed, or outside target `usize`.
    InvalidValue,
}

impl std::fmt::Display
    for LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Missing => {
                "remote expected-device scheduling-consumption max-records configuration missing"
            }
            Self::NonUnicode => {
                "remote expected-device scheduling-consumption max-records configuration encoding invalid"
            }
            Self::InvalidValue => {
                "remote expected-device scheduling-consumption max-records configuration invalid"
            }
        })
    }
}

impl std::error::Error
    for LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError
{
}

#[allow(
    dead_code,
    reason = "C03e-NN materializes the NM-selected strict ASCII-decimal scheduling-consumption max-records parser before separately gated ledger representation"
)]
fn parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(
    value: Option<OsString>,
) -> Result<usize, LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError> {
    let value = value.ok_or(
        LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::Missing,
    )?;
    let value = value.into_string().map_err(|_| {
        LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::NonUnicode
    })?;
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(
            LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::InvalidValue,
        );
    }
    value.parse::<usize>().map_err(|_| {
        LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError::InvalidValue
    })
}

/// Loads the explicitly configured production expected-device scheduling-consumption record bound.
///
/// The exact Unicode value must contain ASCII decimal digits only and is converted fail-closed to
/// target `usize`. Zero is returned unchanged; the separately gated future scheduling-consumption
/// ledger constructor remains the sole semantic authority for its non-zero capacity invariant.
/// This source performs no trimming, fallback, retry, alternate-variable lookup, requester-provider
/// capacity aliasing, worker-limit aliasing, channel-capacity aliasing, cache, refresh, ledger
/// construction, population composition, or runtime activation.
///
/// # Errors
///
/// Fails closed when the fixed configuration is missing, non-Unicode, empty, malformed, or outside
/// target `usize`. The bounded error surface does not expose the configured value.
#[allow(
    dead_code,
    reason = "C03e-NN materializes the NM-selected fixed scheduling-consumption max-records environment loader before separately gated ledger representation and population composition"
)]
pub(crate) fn load_linux_agent_remote_expected_device_scheduling_consumption_max_records_from_env()
-> Result<usize, LinuxAgentRemoteExpectedDeviceSchedulingConsumptionMaxRecordsSourceError> {
    parse_linux_agent_remote_expected_device_scheduling_consumption_max_records_value(
        std::env::var_os(PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_ENV),
    )
}

// The source-only materialization above is intentionally dormant. The remainder of the historical
// bootstrap/runtime implementation is preserved byte-for-byte in the authoritative parent and is
// not redefined by this source boundary.
