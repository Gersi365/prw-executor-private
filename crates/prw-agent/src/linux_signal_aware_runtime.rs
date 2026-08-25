//! Callable signal-aware production-local Linux runtime.
//!
//! Phase 098 composes the thread-affine SIGTERM/SIGINT source, one-step
//! signal-aware readiness, Phase 092 runtime-specific scheduling, Phase 097
//! bounded lifetime evidence/error policy, and Phase 096 lifecycle cleanup.
//! C03e-AT adds one crate-internal owned companion seam whose finalizer runs
//! after local listener cleanup and before exact signal-mask restoration.
//! This module remains below `main.rs` and systemd activation.

use std::thread;

use super::accept_ready::AcceptReadyAgentSocket;
use super::bounded_scheduler_cycle::{LocalLinuxSchedulerControl, LocalLinuxSchedulingCycleStop};
use super::production_lifecycle::{
    LocalLinuxProductionLifecycleAssemblyError, with_local_linux_production_lifecycle_from_env,
};
use super::production_runtime_loop::LocalLinuxProductionRuntimeInputs;
use super::production_runtime_types::{
    LocalLinuxProductionRuntimeCleanup, LocalLinuxProductionRuntimeCounters,
    LocalLinuxProductionRuntimeFatalError, LocalLinuxRuntimeErrorDisposition,
    classify_production_runtime_error,
};
use super::runtime_orchestration::{
    LocalLinuxRuntimeOrchestrationError, LocalLinuxRuntimeSchedulerContext,
    LocalLinuxRuntimeShutdownHandle, run_bounded_runtime_scheduling_cycle,
};
use super::runtime_wake::LocalLinuxRuntimeWake;
use super::signal_aware_readiness::{
    LocalLinuxSignalAwareReadinessError, LocalLinuxSignalAwareReadinessOutcome,
    wait_once_for_signal_aware_linux_runtime_readiness,
};
use super::termination_signal::{
    LocalLinuxTerminationSignal, LocalLinuxTerminationSignalMaskRestore,
    LocalLinuxTerminationSignalSource, LocalLinuxTerminationSignalSourceCreateError,
};
use super::worker_capacity::LocalLinuxWorkerCapacity;
use super::worker_completion::LocalLinuxScopedWorkerCompletion;
use super::worker_registry::{
    LocalLinuxRegisteredWorkerCancellation, LocalLinuxScopedWorkerRegistry,
};

/// Terminal reason for one signal-aware production-local runtime lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLinuxSignalAwareRuntimeTerminalReason {
    /// Programmatic monotonic shutdown was observed through the runtime wake/control path.
    ProgrammaticShutdown,
    /// SIGTERM or SIGINT won readiness precedence and initiated shutdown.
    TerminationSignal(LocalLinuxTerminationSignal),
    /// Signal-aware readiness failed closed.
    ReadinessFatal(LocalLinuxSignalAwareReadinessError),
    /// Runtime-specific scheduling failed under the Phase 095 fail-stop policy.
    RuntimeFatal(LocalLinuxProductionRuntimeFatalError),
}

/// Final report after loop teardown, listener cleanup, and signal-mask restoration.
#[derive(Debug, PartialEq, Eq)]
pub struct LocalLinuxSignalAwareRuntimeTerminalReport {
    reason: LocalLinuxSignalAwareRuntimeTerminalReason,
    counters: LocalLinuxProductionRuntimeCounters,
    cancellations: Vec<LocalLinuxRegisteredWorkerCancellation>,
    final_completions: Vec<LocalLinuxScopedWorkerCompletion>,
    cleanup: LocalLinuxProductionRuntimeCleanup,
    mask_restore: LocalLinuxTerminationSignalMaskRestore,
}

impl LocalLinuxSignalAwareRuntimeTerminalReport {
    /// Returns the original runtime terminal reason.
    #[must_use]
    pub const fn reason(&self) -> LocalLinuxSignalAwareRuntimeTerminalReason {
        self.reason
    }

    /// Returns memory-bounded process-lifetime counters.
    #[must_use]
    pub const fn counters(&self) -> LocalLinuxProductionRuntimeCounters {
        self.counters
    }

    /// Returns final worker cancellation outcomes, bounded by configured capacity.
    #[must_use]
    pub fn cancellations(&self) -> &[LocalLinuxRegisteredWorkerCancellation] {
        &self.cancellations
    }

    /// Returns final joined worker completions, bounded by configured capacity.
    #[must_use]
    pub fn final_completions(&self) -> &[LocalLinuxScopedWorkerCompletion] {
        &self.final_completions
    }

    /// Returns listener/socket cleanup evidence.
    #[must_use]
    pub const fn cleanup(&self) -> LocalLinuxProductionRuntimeCleanup {
        self.cleanup
    }

    /// Returns exact calling-thread signal-mask restoration evidence.
    #[must_use]
    pub const fn mask_restore(&self) -> LocalLinuxTerminationSignalMaskRestore {
        self.mask_restore
    }
}

/// Startup failure before the signal-aware runtime loop owns a terminal reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLinuxSignalAwareRuntimeStartError {
    /// Safe termination signal source could not be created.
    SignalSource(LocalLinuxTerminationSignalSourceCreateError),
    /// Phase 096 lifecycle assembly failed after the signal mask was installed.
    Lifecycle {
        /// Original lifecycle assembly failure.
        error: LocalLinuxProductionLifecycleAssemblyError,
        /// Signal-mask restoration attempted after the lifecycle failure.
        mask_restore: LocalLinuxTerminationSignalMaskRestore,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalLinuxSignalAwareRuntimeLoopExit {
    reason: LocalLinuxSignalAwareRuntimeTerminalReason,
    counters: LocalLinuxProductionRuntimeCounters,
    cancellations: Vec<LocalLinuxRegisteredWorkerCancellation>,
    final_completions: Vec<LocalLinuxScopedWorkerCompletion>,
}

/// Runs the signal-aware long-running loop over already-live lifecycle resources.
#[must_use]
pub fn run_signal_aware_linux_production_runtime_loop(
    listener: &AcceptReadyAgentSocket<'_>,
    signal_source: &LocalLinuxTerminationSignalSource,
    wake: &LocalLinuxRuntimeWake,
    capacity: &LocalLinuxWorkerCapacity,
    control: &LocalLinuxSchedulerControl,
    inputs: LocalLinuxProductionRuntimeInputs<'_>,
) -> LocalLinuxSignalAwareRuntimeLoopExit {
    let context = LocalLinuxRuntimeSchedulerContext::new(
        capacity,
        inputs.policy(),
        inputs.status_snapshot(),
        inputs.private_dns_snapshot(),
        inputs.config().worker_config(),
        wake.notifier(),
    );

    thread::scope(|scope| {
        let mut registry = LocalLinuxScopedWorkerRegistry::new();
        let mut counters = LocalLinuxProductionRuntimeCounters::default();

        let reason = loop {
            let readiness = wait_once_for_signal_aware_linux_runtime_readiness(
                listener,
                signal_source,
                wake,
                capacity,
                &mut registry,
                control,
            );

            let readiness = match readiness {
                Ok(report) => {
                    counters.record_signal_aware_readiness(&report);
                    report
                }
                Err(error) => {
                    break LocalLinuxSignalAwareRuntimeTerminalReason::ReadinessFatal(error);
                }
            };

            match readiness.outcome() {
                LocalLinuxSignalAwareReadinessOutcome::ShutdownObserved => {
                    break LocalLinuxSignalAwareRuntimeTerminalReason::ProgrammaticShutdown;
                }
                LocalLinuxSignalAwareReadinessOutcome::TerminationSignal(signal) => {
                    break LocalLinuxSignalAwareRuntimeTerminalReason::TerminationSignal(signal);
                }
                LocalLinuxSignalAwareReadinessOutcome::RuntimeWake
                | LocalLinuxSignalAwareReadinessOutcome::WaitInterrupted => continue,
                LocalLinuxSignalAwareReadinessOutcome::ListenerReady => {}
            }

            match run_bounded_runtime_scheduling_cycle(
                scope,
                listener,
                &mut registry,
                control,
                &context,
                inputs.config().scheduling_attempt_budget(),
            ) {
                Ok(report) => {
                    counters.record_runtime_scheduling_report(&report);
                    if report.stop() == LocalLinuxSchedulingCycleStop::ShutdownRequested {
                        break LocalLinuxSignalAwareRuntimeTerminalReason::ProgrammaticShutdown;
                    }
                }
                Err(error) => {
                    counters.record_runtime_scheduling_error(&error);
                    let orchestration_error =
                        LocalLinuxRuntimeOrchestrationError::Scheduling(error);
                    match classify_production_runtime_error(&orchestration_error) {
                        LocalLinuxRuntimeErrorDisposition::ContinueAfterPeerRejection => {
                            counters.record_peer_rejection();
                        }
                        LocalLinuxRuntimeErrorDisposition::FailStop => {
                            let fatal =
                                LocalLinuxProductionRuntimeFatalError::from_orchestration_error(
                                    &orchestration_error,
                                )
                                .expect(
                                    "Phase 098 fail-stop scheduling error preserves fatal cause",
                                );
                            break LocalLinuxSignalAwareRuntimeTerminalReason::RuntimeFatal(fatal);
                        }
                    }
                }
            }
        };

        let cancellations = registry.cancel_all();
        let final_completions = registry.join_all();
        counters.record_final_completions(final_completions.len());

        LocalLinuxSignalAwareRuntimeLoopExit {
            reason,
            counters,
            cancellations,
            final_completions,
        }
    })
}

/// Runs the complete signal-aware local lifecycle with one owned process companion.
///
/// The companion is started only after the existing signal source and local lifecycle have been
/// assembled. Its owned value is carried through the unchanged local runtime loop and explicit
/// listener/socket cleanup. The consuming finalizer runs after that cleanup and before exact prior
/// signal-mask restoration.
///
/// This is a crate-internal composition seam, not a generic public lifecycle plugin surface.
///
/// # Errors
///
/// Returns the existing bounded signal-source or lifecycle-assembly failure unchanged. Companion
/// startup/finalization policy is represented by the caller-owned companion value and does not add a
/// new local startup failure class.
pub(crate) fn run_signal_aware_linux_production_runtime_from_env_with_companion<
    F,
    S,
    O,
    G,
>(
    inputs: LocalLinuxProductionRuntimeInputs<'_>,
    on_started: F,
    start_companion: S,
    finalize_companion: G,
) -> Result<LocalLinuxSignalAwareRuntimeTerminalReport, LocalLinuxSignalAwareRuntimeStartError>
where
    F: FnOnce(LocalLinuxRuntimeShutdownHandle),
    S: FnOnce() -> O,
    G: FnOnce(O),
{
    let signal_source = LocalLinuxTerminationSignalSource::create()
        .map_err(LocalLinuxSignalAwareRuntimeStartError::SignalSource)?;

    let execution = match with_local_linux_production_lifecycle_from_env(
        inputs.config(),
        |listener, wake, capacity, control| {
            on_started(LocalLinuxRuntimeShutdownHandle::new(
                control.clone(),
                wake.notifier(),
            ));
            let companion = start_companion();
            let exit = run_signal_aware_linux_production_runtime_loop(
                listener,
                &signal_source,
                wake,
                capacity,
                control,
                inputs,
            );
            (exit, companion)
        },
    ) {
        Ok(execution) => execution,
        Err(error) => {
            let mask_restore = signal_source.restore();
            return Err(LocalLinuxSignalAwareRuntimeStartError::Lifecycle {
                error,
                mask_restore,
            });
        }
    };

    let cleanup = execution.cleanup();
    let (exit, companion) = execution.into_value();
    finalize_companion(companion);
    let mask_restore = signal_source.restore();

    Ok(LocalLinuxSignalAwareRuntimeTerminalReport {
        reason: exit.reason,
        counters: exit.counters,
        cancellations: exit.cancellations,
        final_completions: exit.final_completions,
        cleanup,
        mask_restore,
    })
}

/// Runs the complete signal-aware production-local runtime from process environment.
///
/// The SIGTERM/SIGINT mask and `SignalFd` are established before Phase 096 lifecycle
/// assembly, ensuring every later worker thread inherits the blocked termination
/// mask. On normal terminal return, Phase 096 removes the listener/socket first;
/// only then is the exact prior calling-thread signal mask restored.
///
/// `on_started` remains a deterministic pre-bootstrap seam and receives the
/// programmatic shutdown handle after lifecycle assembly but before the first wait.
///
/// # Errors
///
/// Returns a bounded startup error only for signal-source or lifecycle assembly
/// failure. Runtime/readiness failures become typed terminal report reasons.
pub fn run_signal_aware_linux_production_runtime_from_env<F>(
    inputs: LocalLinuxProductionRuntimeInputs<'_>,
    on_started: F,
) -> Result<LocalLinuxSignalAwareRuntimeTerminalReport, LocalLinuxSignalAwareRuntimeStartError>
where
    F: FnOnce(LocalLinuxRuntimeShutdownHandle),
{
    run_signal_aware_linux_production_runtime_from_env_with_companion(
        inputs,
        on_started,
        || (),
        |()| {},
    )
}

#[cfg(test)]
fn run_signal_aware_linux_production_runtime_in_root_path_with_companion<F, S, O, G>(
    root_path: &std::path::Path,
    inputs: LocalLinuxProductionRuntimeInputs<'_>,
    on_started: F,
    start_companion: S,
    finalize_companion: G,
) -> Result<LocalLinuxSignalAwareRuntimeTerminalReport, LocalLinuxSignalAwareRuntimeStartError>
where
    F: FnOnce(LocalLinuxRuntimeShutdownHandle),
    S: FnOnce() -> O,
    G: FnOnce(O),
{
    let signal_source = LocalLinuxTerminationSignalSource::create()
        .map_err(LocalLinuxSignalAwareRuntimeStartError::SignalSource)?;

    let execution =
        match super::production_lifecycle::with_local_linux_production_lifecycle_in_root_path(
            root_path,
            inputs.config(),
            |listener, wake, capacity, control| {
                on_started(LocalLinuxRuntimeShutdownHandle::new(
                    control.clone(),
                    wake.notifier(),
                ));
                let companion = start_companion();
                let exit = run_signal_aware_linux_production_runtime_loop(
                    listener,
                    &signal_source,
                    wake,
                    capacity,
                    control,
                    inputs,
                );
                (exit, companion)
            },
        ) {
            Ok(execution) => execution,
            Err(error) => {
                let mask_restore = signal_source.restore();
                return Err(LocalLinuxSignalAwareRuntimeStartError::Lifecycle {
                    error,
                    mask_restore,
                });
            }
        };

    let cleanup = execution.cleanup();
    let (exit, companion) = execution.into_value();
    finalize_companion(companion);
    let mask_restore = signal_source.restore();

    Ok(LocalLinuxSignalAwareRuntimeTerminalReport {
        reason: exit.reason,
        counters: exit.counters,
        cancellations: exit.cancellations,
        final_completions: exit.final_completions,
        cleanup,
        mask_restore,
    })
}

#[cfg(test)]
fn run_signal_aware_linux_production_runtime_in_root_path<F>(
    root_path: &std::path::Path,
    inputs: LocalLinuxProductionRuntimeInputs<'_>,
    on_started: F,
) -> Result<LocalLinuxSignalAwareRuntimeTerminalReport, LocalLinuxSignalAwareRuntimeStartError>
where
    F: FnOnce(LocalLinuxRuntimeShutdownHandle),
{
    run_signal_aware_linux_production_runtime_in_root_path_with_companion(
        root_path,
        inputs,
        on_started,
        || (),
        |()| {},
    )
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::fs::{self, Permissions};
    use std::num::{NonZeroU16, NonZeroUsize};
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::net::UnixStream;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;

    use nix::sys::signal::{SigSet, Signal, raise};
    use prw_network::PrivateDnsConfig;
    use prw_policy::BoundedLocalReadPolicy;

    use super::{
        LocalLinuxSignalAwareRuntimeTerminalReason,
        run_signal_aware_linux_production_runtime_in_root_path,
        run_signal_aware_linux_production_runtime_in_root_path_with_companion,
    };
    use crate::linux_identity::deadline_io::LocalLinuxIoBudget;
    use crate::linux_identity::production_runtime_loop::LocalLinuxProductionRuntimeInputs;
    use crate::linux_identity::production_runtime_types::{
        LocalLinuxProductionRuntimeCleanup, LocalLinuxProductionRuntimeConfig,
    };
    use crate::linux_identity::termination_signal::{
        LocalLinuxTerminationSignal, LocalLinuxTerminationSignalMaskRestore,
    };
    use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
    use crate::local_commands::status_snapshot::{
        LocalAgentRuntimeState, LocalAgentStatusSnapshot,
    };
    use crate::{AGENT_RUNTIME_SUBDIRECTORY, AGENT_SOCKET_FILENAME};

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);
    const SUBPROCESS_ENV: &str = "PRW_PHASE_098_SIGNAL_RUNTIME_SUBPROCESS";

    fn create_root(label: &str) -> PathBuf {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "prw-phase-098-runtime-{}-{sequence}-{label}",
            std::process::id()
        ));
        fs::create_dir(&root).expect("temporary Phase 098 runtime root creates");
        fs::set_permissions(&root, Permissions::from_mode(0o700))
            .expect("temporary Phase 098 runtime root mode sets");
        root
    }

    fn config() -> LocalLinuxProductionRuntimeConfig {
        LocalLinuxProductionRuntimeConfig::new(
            NonZeroUsize::new(2).expect("capacity nonzero"),
            NonZeroU16::new(8).expect("backlog nonzero"),
            NonZeroUsize::new(2).expect("attempt budget nonzero"),
            NonZeroUsize::new(1).expect("request budget nonzero"),
            LocalLinuxIoBudget::try_new(Duration::from_secs(2)).expect("read budget nonzero"),
            LocalLinuxIoBudget::try_new(Duration::from_secs(2)).expect("write budget nonzero"),
        )
    }

    fn dns_snapshot() -> LocalPrivateDnsSnapshot {
        LocalPrivateDnsSnapshot::try_from_config(&PrivateDnsConfig::default())
            .expect("default DNS config is bounded")
    }

    fn inputs(dns: &LocalPrivateDnsSnapshot) -> LocalLinuxProductionRuntimeInputs<'_> {
        LocalLinuxProductionRuntimeInputs::new(
            config(),
            BoundedLocalReadPolicy::allow_local_reads(),
            LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready),
            dns,
        )
    }

    fn socket_path(root: &Path) -> PathBuf {
        root.join(AGENT_RUNTIME_SUBDIRECTORY)
            .join(AGENT_SOCKET_FILENAME)
    }

    #[test]
    fn companion_finalizes_after_listener_cleanup_before_signal_mask_restore() {
        let original_mask = SigSet::thread_get_mask().expect("original companion test mask reads");
        let root = create_root("companion-ordering");
        let path = socket_path(&root);
        let dns = dns_snapshot();
        let start_saw_live_socket = Cell::new(false);
        let finalizer_ran = Cell::new(false);
        let finalizer_saw_blocked_termination_mask = Cell::new(false);

        let report = run_signal_aware_linux_production_runtime_in_root_path_with_companion(
            &root,
            inputs(&dns),
            |shutdown| {
                shutdown
                    .request_shutdown_and_wake()
                    .expect("local programmatic shutdown wake succeeds");
            },
            || {
                start_saw_live_socket.set(path.exists());
            },
            |()| {
                assert!(
                    !path.exists(),
                    "local listener/socket cleanup must finish before companion finalization"
                );
                let finalizer_mask =
                    SigSet::thread_get_mask().expect("companion finalizer mask reads");
                finalizer_saw_blocked_termination_mask.set(
                    finalizer_mask.contains(Signal::SIGTERM)
                        && finalizer_mask.contains(Signal::SIGINT),
                );
                finalizer_ran.set(true);
            },
        )
        .expect("companion-aware signal runtime starts");

        assert!(start_saw_live_socket.get());
        assert!(finalizer_ran.get());
        assert!(finalizer_saw_blocked_termination_mask.get());
        assert_eq!(
            report.reason(),
            LocalLinuxSignalAwareRuntimeTerminalReason::ProgrammaticShutdown
        );
        assert_eq!(report.cleanup(), LocalLinuxProductionRuntimeCleanup::Clean);
        assert_eq!(
            report.mask_restore(),
            LocalLinuxTerminationSignalMaskRestore::Restored
        );
        assert_eq!(
            SigSet::thread_get_mask().expect("restored companion test mask reads"),
            original_mask
        );
        fs::remove_dir_all(root).expect("temporary companion ordering root removes");
    }

    #[test]
    fn signal_beats_simultaneous_listener_and_restores_mask_after_cleanup() {
        if std::env::var_os(SUBPROCESS_ENV).is_some() {
            run_signal_precedence_subprocess();
            return;
        }

        let executable = std::env::current_exe().expect("current test executable resolves");
        let output = Command::new(executable)
            .arg("--exact")
            .arg(
                "linux_identity::signal_aware_runtime::tests::signal_beats_simultaneous_listener_and_restores_mask_after_cleanup",
            )
            .arg("--nocapture")
            .env(SUBPROCESS_ENV, "1")
            .output()
            .expect("isolated Phase 098 signal-runtime subprocess starts");

        assert!(
            output.status.success(),
            "isolated Phase 098 signal-runtime subprocess failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn run_signal_precedence_subprocess() {
        let original_mask = SigSet::thread_get_mask().expect("original mask reads");
        let root = create_root("signal-precedence");
        let path = socket_path(&root);
        let dns = dns_snapshot();
        let mut queued_client = None;

        let report = run_signal_aware_linux_production_runtime_in_root_path(
            &root,
            inputs(&dns),
            |_shutdown| {
                queued_client = Some(UnixStream::connect(&path).expect("queued client connects"));
                raise(Signal::SIGTERM)
                    .expect("SIGTERM posts to current masked runtime test thread");
            },
        )
        .expect("signal-aware runtime starts");

        assert_eq!(
            report.reason(),
            LocalLinuxSignalAwareRuntimeTerminalReason::TerminationSignal(
                LocalLinuxTerminationSignal::SigTerm
            )
        );
        assert_eq!(report.counters().scheduling_attempts(), 0);
        assert_eq!(report.counters().workers_registered(), 0);
        assert!(report.cancellations().is_empty());
        assert!(report.final_completions().is_empty());
        assert_eq!(report.cleanup(), LocalLinuxProductionRuntimeCleanup::Clean);
        assert_eq!(
            report.mask_restore(),
            LocalLinuxTerminationSignalMaskRestore::Restored
        );
        assert!(!path.exists());
        assert_eq!(
            SigSet::thread_get_mask().expect("restored runtime test mask reads"),
            original_mask
        );

        drop(queued_client);
        fs::remove_dir_all(root).expect("temporary signal precedence root removes");
    }
}
