//! C03 management-capable finite authenticated Linux session worker.
//!
//! This child module leaves the existing public finite worker unchanged. It reuses the same
//! worker permit, request budget, and per-request I/O deadlines while delegating each request
//! to the crate-private management-capable authenticated-session seam.

use std::os::unix::net::UnixStream;

use prw_policy::PolicyEvaluator;

use super::{LocalLinuxSessionWorkerConfig, LocalLinuxSessionWorkerStop};
use crate::linux_identity::authenticated_session::AuthenticatedLocalLinuxSession;
use crate::linux_identity::worker_capacity::LocalLinuxWorkerPermit;
use crate::local_commands::boundary_request_response_transaction::LocalBoundaryRequestResponseOutcome;
use crate::local_commands::management_runtime::LocalLinuxManagementRuntimeContext;
use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
use crate::local_commands::status_snapshot::LocalAgentStatusSnapshot;

/// Coarse crate-internal failure for one management-capable finite worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalLinuxManagementSessionWorkerError {
    /// One management-capable Request failed after the stated number of prior responses.
    Processing {
        /// Number of terminal responses completed before the failing Request.
        responses_written: usize,
    },
}

/// Runs one authenticated session through the C03 management-capable deadline path.
///
/// The existing public worker remains unchanged. The permit stays live for the entire worker
/// scope and is released by RAII on every return or unwind. Every Request receives the same
/// fresh read and deferred response-write budgets as the legacy finite worker. Shared provider
/// state remains lock-late because the delegated session seam classifies command 3 before
/// acquiring the management lifecycle mutex.
pub(crate) fn run_authenticated_session_worker_with_management<RE: PolicyEvaluator + ?Sized>(
    mut session: AuthenticatedLocalLinuxSession<UnixStream>,
    _permit: LocalLinuxWorkerPermit,
    read_evaluator: &RE,
    management_context: LocalLinuxManagementRuntimeContext<'_, '_>,
    status_snapshot: LocalAgentStatusSnapshot,
    private_dns_snapshot: &LocalPrivateDnsSnapshot,
    config: LocalLinuxSessionWorkerConfig,
) -> Result<LocalLinuxSessionWorkerStop, LocalLinuxManagementSessionWorkerError> {
    for responses_written in 0..config.request_budget().get() {
        match session.process_one_management_with_deadlines(
            read_evaluator,
            management_context,
            status_snapshot,
            private_dns_snapshot,
            config.read_budget(),
            config.write_budget(),
        ) {
            Ok(LocalBoundaryRequestResponseOutcome::ResponseWritten) => {}
            Ok(LocalBoundaryRequestResponseOutcome::CleanEof) => {
                return Ok(LocalLinuxSessionWorkerStop::CleanEof { responses_written });
            }
            Err(_) => {
                return Err(LocalLinuxManagementSessionWorkerError::Processing {
                    responses_written,
                });
            }
        }
    }

    Ok(LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
        responses_written: config.request_budget().get(),
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;
    use std::num::NonZeroUsize;
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;

    use prw_network::PrivateDnsConfig;
    use prw_policy::{
        BoundedLocalManagementDecisions, BoundedLocalManagementPolicy, BoundedLocalReadPolicy,
        Decision,
    };
    use prw_remote_bridge::BridgeCommand;

    use super::run_authenticated_session_worker_with_management;
    use crate::LocalIpcRequestId;
    use crate::frame_object::reader::read_frame;
    use crate::frame_object::writer::write_frame;
    use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
    use crate::linux_identity::authenticated_session::AuthenticatedLocalLinuxSession;
    use crate::linux_identity::deadline_io::LocalLinuxIoBudget;
    use crate::linux_identity::worker_capacity::LocalLinuxWorkerCapacity;
    use crate::linux_identity::worker::LocalLinuxSessionWorkerConfig;
    use crate::local_commands::{LocalAgentCommand, LocalAgentResponseStatus};
    use crate::local_commands::management_authority::LocalManagementFilesystemAuthority;
    use crate::local_commands::management_linux_backends::{
        LinuxLocalForwardingBackend, LinuxLocalTerminalBackend,
    };
    use crate::local_commands::management_provider_backend_policy::ExactForwardingEgressPolicy;
    use crate::local_commands::management_provider_lifecycle::LocalManagementProviderLifecycle;
    use crate::local_commands::management_request::build_local_management_request_frame;
    use crate::local_commands::management_runtime::{
        LocalLinuxManagementProviderLifecycle, LocalLinuxManagementRuntimeContext,
    };
    use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
    use crate::local_commands::request_frame::build_local_command_request_frame;
    use crate::local_commands::status_snapshot::response_frame::decode_success_status_frame;
    use crate::local_commands::status_snapshot::{
        LocalAgentRuntimeState, LocalAgentStatusSnapshot,
    };
    use crate::local_commands::terminal_response::validate_terminal_response_frame;

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

    struct Harness {
        root_path: PathBuf,
        filesystem: LocalManagementFilesystemAuthority,
        session: AuthenticatedLocalLinuxSession<UnixStream>,
        client: UnixStream,
    }

    impl Harness {
        fn new(label: &str) -> Self {
            let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let root_path = std::env::temp_dir().join(format!(
                "prw-c03-management-worker-{}-{sequence}-{label}",
                std::process::id()
            ));
            fs::create_dir(&root_path).expect("management-worker test root creates");
            let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root_path)
                .expect("management-worker test root anchors");
            let (server, client) = UnixStream::pair().expect("same-user local pair creates");
            let server = AuthenticatedLocalLinuxConnection::try_new(server)
                .expect("same-UID local pair authenticates");
            Self {
                root_path,
                filesystem,
                session: AuthenticatedLocalLinuxSession::new(server),
                client,
            }
        }
    }

    impl Drop for Harness {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root_path);
        }
    }

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("request id is non-zero")
    }

    fn status() -> LocalAgentStatusSnapshot {
        LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)
    }

    fn dns() -> LocalPrivateDnsSnapshot {
        LocalPrivateDnsSnapshot::try_from_config(&PrivateDnsConfig::default())
            .expect("default DNS config is bounded")
    }

    fn policy(agent_status: Decision) -> BoundedLocalManagementPolicy {
        BoundedLocalManagementPolicy::new(BoundedLocalManagementDecisions {
            agent_status,
            private_dns: Decision::Deny,
            terminal_open: Decision::Deny,
            terminal_exec: Decision::Deny,
            files_read: Decision::Deny,
            files_write: Decision::Deny,
            forwarding_create: Decision::Deny,
        })
    }

    fn lifecycle(
        filesystem: &LocalManagementFilesystemAuthority,
    ) -> LocalLinuxManagementProviderLifecycle<'_> {
        let egress = ExactForwardingEgressPolicy::try_from_targets(&[])
            .expect("empty exact-target allowlist is bounded");
        LocalManagementProviderLifecycle::new(
            filesystem,
            LinuxLocalTerminalBackend,
            LinuxLocalForwardingBackend::new(egress),
        )
    }

    fn context<'context, 'authority>(
        filesystem: &'authority LocalManagementFilesystemAuthority,
        policy: &'context BoundedLocalManagementPolicy,
        lifecycle: &'context Mutex<LocalLinuxManagementProviderLifecycle<'authority>>,
    ) -> LocalLinuxManagementRuntimeContext<'context, 'authority> {
        LocalLinuxManagementRuntimeContext::new(filesystem, policy, lifecycle)
    }

    fn worker_config() -> LocalLinuxSessionWorkerConfig {
        LocalLinuxSessionWorkerConfig::new(
            NonZeroUsize::new(1).expect("one request is non-zero"),
            LocalLinuxIoBudget::try_new(Duration::from_secs(2))
                .expect("read budget is non-zero"),
            LocalLinuxIoBudget::try_new(Duration::from_secs(2))
                .expect("write budget is non-zero"),
        )
    }

    fn poison_lifecycle(lifecycle: &Mutex<LocalLinuxManagementProviderLifecycle<'_>>) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = lifecycle.lock().expect("lifecycle lock starts healthy");
            panic!("intentional management-worker poison");
        }));
        assert!(lifecycle.is_poisoned());
    }

    #[test]
    fn command_three_agent_status_runs_through_finite_management_worker() {
        let mut harness = Harness::new("management");
        let management_policy = policy(Decision::Allow);
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        let capacity = LocalLinuxWorkerCapacity::new(
            NonZeroUsize::new(1).expect("worker capacity is non-zero"),
        );
        let permit = capacity.try_acquire().expect("worker permit acquires");
        let bridge = BridgeCommand::AgentStatus
            .encode()
            .expect("bridge command encodes");
        let frame = build_local_management_request_frame(id(101), &bridge)
            .expect("management request frame builds");
        write_frame(&mut harness.client, &frame).expect("management request writes");

        let stop = run_authenticated_session_worker_with_management(
            harness.session,
            permit,
            &BoundedLocalReadPolicy::deny_all(),
            context(&harness.filesystem, &management_policy, &lifecycle),
            status(),
            &dns(),
            worker_config(),
        )
        .expect("management worker succeeds");
        assert_eq!(
            stop,
            super::LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
                responses_written: 1
            }
        );
        assert_eq!(capacity.active_workers(), 0);

        let response = read_frame(&mut harness.client).expect("management response reads");
        let terminal =
            validate_terminal_response_frame(&response).expect("management response validates");
        assert_eq!(terminal.request_id(), id(101));
        assert_eq!(terminal.status(), LocalAgentResponseStatus::Ok);
    }

    #[test]
    fn legacy_request_ignores_poisoned_management_lifecycle_in_worker() {
        let mut harness = Harness::new("legacy");
        let management_policy = policy(Decision::Deny);
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        poison_lifecycle(&lifecycle);
        let capacity = LocalLinuxWorkerCapacity::new(
            NonZeroUsize::new(1).expect("worker capacity is non-zero"),
        );
        let permit = capacity.try_acquire().expect("worker permit acquires");
        let frame = build_local_command_request_frame(id(102), LocalAgentCommand::GetAgentStatus)
            .expect("legacy request frame builds");
        write_frame(&mut harness.client, &frame).expect("legacy request writes");

        run_authenticated_session_worker_with_management(
            harness.session,
            permit,
            &BoundedLocalReadPolicy::allow_local_reads(),
            context(&harness.filesystem, &management_policy, &lifecycle),
            status(),
            &dns(),
            worker_config(),
        )
        .expect("legacy request succeeds through management worker");
        assert_eq!(capacity.active_workers(), 0);
        assert!(lifecycle.is_poisoned());

        let response = read_frame(&mut harness.client).expect("legacy response reads");
        let decoded =
            decode_success_status_frame(&response).expect("legacy status response decodes");
        assert_eq!(decoded.request_id(), id(102));

        let mut trailing = [0_u8; 1];
        assert_eq!(
            harness
                .client
                .read(&mut trailing)
                .expect("worker stream reaches EOF"),
            0
        );
    }
}
