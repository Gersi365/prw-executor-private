//! Narrow public Linux Agent binary-bootstrap facade.
//!
//! Phase 102 keeps the internal Linux lifecycle/readiness/signal/worker graph
//! crate-private. This module exposes only the fixed initial bootstrap profile,
//! bounded startup/terminal classifications, and one call into the already-
//! validated Phase 098 signal-aware runtime.

use std::num::{NonZeroU16, NonZeroUsize};
use std::time::Duration;

use prw_network::PrivateDnsConfig;
use prw_policy::BoundedLocalReadPolicy;

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
    RemoteSessionSupervisorShutdownController,
    remote_session_process_lifecycle_control::{
        RemoteSessionProcessControllerFinalization, RemoteSessionProcessLifecycleFinalization,
        RemoteSessionProcessLifecycleOwner, RemoteSessionProcessLifecycleSpawnError,
        RemoteSessionProcessThreadFinalization, RemoteSessionSupervisorShutdownPublish,
        RemoteSessionSupervisorShutdownPublisher,
    },
};

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
    /// Returns the bounded token used by the initial stderr summary contract.
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

/// Secondary bounded finalization evidence for the injected remote process companion.
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
        run_with_remote_process_companion_inputs(inputs, operation).map(|(local, remote)| {
            LinuxAgentBootstrapWithRemoteReport { local, remote }
        })
    })
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
    match companion {
        Ok(owner) => map_remote_process_finalization(owner.finalize()),
        Err(_) => LinuxAgentRemoteProcessCompanionFinalization::SpawnFailed,
    }
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
    use prw_policy::{BoundedLocalReadPolicy, Capability, Decision, PolicyEvaluator};

    use super::{
        LinuxAgentBootstrapCleanup, LinuxAgentBootstrapCounters, LinuxAgentBootstrapReport,
        LinuxAgentBootstrapSignalMaskRestore, LinuxAgentBootstrapStartFailure,
        LinuxAgentBootstrapStartKind, LinuxAgentBootstrapTerminal,
        LinuxAgentBootstrapWithRemoteReport, LinuxAgentRemoteProcessCompanionFinalization,
        LinuxAgentRemoteProcessControllerFinalization, LinuxAgentRemoteProcessThreadFinalization,
        LinuxAgentRemoteSupervisorShutdownPublish, LinuxAgentRemoteSupervisorShutdownPublisher,
        finalize_remote_process_companion, initial_runtime_config, map_lifecycle_start_kind,
        map_remote_shutdown_publish, run, run_with_remote_process_companion,
    };
    use crate::linux_identity::production_lifecycle::LocalLinuxProductionLifecycleAssemblyError;
    use crate::linux_identity::worker_capacity::LocalLinuxWorkerCapacity;
    use crate::linux_identity::xdg_runtime_root::prw_runtime_directory::agent_instance_lock::AgentInstanceLockError;
    use crate::remote_session_capability_runtime::{
        RemoteSessionSupervisorShutdownController,
        remote_session_process_lifecycle_control::{
            RemoteSessionProcessLifecycleOwner, RemoteSessionProcessLifecycleSpawnError,
            RemoteSessionSupervisorShutdownPublish,
        },
    };

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
        fn operation(_: LinuxAgentRemoteSupervisorShutdownPublisher) {}
        fn assert_signature(
            entry: fn(
                fn(LinuxAgentRemoteSupervisorShutdownPublisher),
            ) -> Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>,
        ) {
            let _ = entry;
        }

        assert_signature(run_with_remote_process_companion::<
            fn(LinuxAgentRemoteSupervisorShutdownPublisher),
        >);
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
