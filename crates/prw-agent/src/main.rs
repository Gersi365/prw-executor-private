//! Headless Private Remote Workspace Agent binary bootstrap.
//!
//! Phase 102 keeps the executable thin: the library's Linux bootstrap
//! facade owns the fixed Phase 101 profile and delegates to the validated
//! signal-aware runtime. The binary owns only bounded stderr reporting and
//! the simple success/failure process exit mapping.

use std::process::ExitCode;

#[cfg(target_os = "linux")]
fn main() -> ExitCode {
    match prw_agent::linux_bootstrap::run() {
        Ok(report) => {
            let counters = report.counters();
            let success = report.is_success();
            eprintln!(
                "prw-agent event=terminal terminal={} exit={} readiness_steps={} listener_armed_steps={} runtime_wakes={} wait_interruptions={} scheduling_attempts={} workers_registered={} worker_completions={} peer_rejections={} cleanup={} signal_mask_restore={}",
                report.terminal().token(),
                if success { "success" } else { "failure" },
                counters.readiness_steps(),
                counters.listener_armed_steps(),
                counters.runtime_wakes(),
                counters.wait_interruptions(),
                counters.scheduling_attempts(),
                counters.workers_registered(),
                counters.worker_completions(),
                counters.peer_rejections(),
                report.cleanup().token(),
                report.signal_mask_restore().token(),
            );
            if success {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(failure) => {
            eprintln!(
                "prw-agent event=startup_failure kind={} exit=failure signal_mask_restore={}",
                failure.kind().token(),
                failure.signal_mask_restore().token(),
            );
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() -> ExitCode {
    eprintln!(
        "prw-agent event=startup_failure kind=unsupported_platform exit=failure signal_mask_restore=not_applicable"
    );
    ExitCode::FAILURE
}
