//! Finite C03 management-capable authenticated Linux session worker body.
//!
//! This sibling preserves the existing worker capacity permit, request budget, read deadline,
//! write deadline, stop taxonomy, and registry-compatible result type. The legacy worker body
//! remains unchanged. Management failures are intentionally collapsed to a coarse worker error
//! so provider, filesystem, policy, and deadline internals do not escape the worker boundary.

use std::os::unix::net::UnixStream;

use prw_policy::PolicyEvaluator;

use super::authenticated_session::AuthenticatedLocalLinuxSession;
use super::session_worker::{
    LocalLinuxSessionWorkerConfig, LocalLinuxSessionWorkerError, LocalLinuxSessionWorkerStop,
};
use super::worker_capacity::LocalLinuxWorkerPermit;
use crate::local_commands::boundary_request_response_transaction::LocalBoundaryRequestResponseOutcome;
use crate::local_commands::management_runtime::LocalLinuxManagementRuntimeContext;
use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
use crate::local_commands::status_snapshot::LocalAgentStatusSnapshot;

/// Runs one authenticated session through the finite C03 management-capable deadline path.
///
/// The session and worker permit are consumed exactly like the legacy worker. The permit stays
/// live for the complete function scope and is released by RAII on every return or unwind.
/// Every request receives the existing per-request absolute read deadline and deferred response-
/// write deadline. Shared provider state remains lock-late inside command-3 classification.
///
/// # Errors
///
/// Returns [`LocalLinuxSessionWorkerError::ManagementProcessing`] on the first management-capable
/// request-pipeline failure. The error intentionally exposes only the number of prior completed
/// responses and does not surface provider, filesystem, policy, or deadline implementation detail.
pub fn run_authenticated_management_session_worker<E: PolicyEvaluator + ?Sized>(
    mut session: AuthenticatedLocalLinuxSession<UnixStream>,
    _permit: LocalLinuxWorkerPermit,
    read_evaluator: &E,
    management_context: LocalLinuxManagementRuntimeContext<'_, '_>,
    status_snapshot: LocalAgentStatusSnapshot,
    private_dns_snapshot: &LocalPrivateDnsSnapshot,
    config: LocalLinuxSessionWorkerConfig,
) -> Result<LocalLinuxSessionWorkerStop, LocalLinuxSessionWorkerError> {
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
                return Err(LocalLinuxSessionWorkerError::ManagementProcessing {
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

    use super::run_authenticated_management_session_worker;
    use crate::LocalIpcRequestId;
    use crate::frame_object::reader::read_frame;
    use crate::frame_object::writer::write_frame;
    use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
    use crate::linux_identity::authenticated_session::AuthenticatedLocalLinuxSession;
    use crate::linux_identity::deadline_io::LocalLinuxIoBudget;
    use crate::linux_identity::session_worker::{
        LocalLinuxSessionWorkerConfig, LocalLinuxSessionWorkerError, LocalLinuxSessionWorkerStop,
    };
    use crate::linux_identity::worker_capacity::LocalLinuxWorkerCapacity;
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
    use crate::local_commands::request_frame::stream::write_local_command_request;
    use crate::local_commands::status_snapshot::response_frame::decode_success_status_frame;
    use crate::local_commands::status_snapshot::{LocalAgentRuntimeState, LocalAgentStatusSnapshot};
    use crate::local_commands::terminal_response::validate_terminal_response_frame;

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("test request id is non-zero")
    }

    fn test_root(label: &str) -> PathBuf {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "prw-c03-management-worker-{}-{sequence}-{label}",
            std::process::id()
        ));
        fs::create_dir(&root).expect("management worker test root creates");
        root
    }

    fn status() -> LocalAgentStatusSnapshot {
        LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)
    }

    fn dns() -> LocalPrivateDnsSnapshot {
        LocalPrivateDnsSnapshot::try_from_config(&PrivateDnsConfig::default())
            .expect("default private DNS config is bounded")
    }

    fn worker_config(requests: usize, read_ms: u64, write_ms: u64) -> LocalLinuxSessionWorkerConfig {
        LocalLinuxSessionWorkerConfig::new(
            NonZeroUsize::new(requests).expect("worker request budget is non-zero"),
            LocalLinuxIoBudget::try_new(Duration::from_millis(read_ms))
                .expect("read I/O budget is non-zero"),
            LocalLinuxIoBudget::try_new(Duration::from_millis(write_ms))
                .expect("write I/O budget is non-zero"),
        )
    }

    fn worker_capacity() -> LocalLinuxWorkerCapacity {
        LocalLinuxWorkerCapacity::new(NonZeroUsize::new(1).expect("capacity is non-zero"))
    }

    fn session(stream: UnixStream) -> AuthenticatedLocalLinuxSession<UnixStream> {
        let connection = AuthenticatedLocalLinuxConnection::try_new(stream)
            .expect("same-UID local pair authenticates");
        AuthenticatedLocalLinuxSession::new(connection)
    }

    fn management_policy(agent_status: Decision) -> BoundedLocalManagementPolicy {
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

    fn poison_lifecycle(lifecycle: &Mutex<LocalLinuxManagementProviderLifecycle<'_>>) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = lifecycle.lock().expect("lifecycle lock starts healthy");
            panic!("intentional management-worker poison");
        }));
        assert!(lifecycle.is_poisoned());
    }

    #[test]
    fn command_three_uses_same_budget_and_releases_worker_permit() {
        let root = test_root("management");
        let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root)
            .expect("management worker test root anchors");
        let policy = management_policy(Decision::Allow);
        let lifecycle = Mutex::new(lifecycle(&filesystem));
        let context = LocalLinuxManagementRuntimeContext::new(&filesystem, &policy, &lifecycle);
        let (server, mut client) = UnixStream::pair().expect("anonymous Unix pair creates");
        let capacity = worker_capacity();
        let permit = capacity.try_acquire().expect("worker permit acquires");
        let bridge = BridgeCommand::AgentStatus.encode().expect("bridge command encodes");
        let request = build_local_management_request_frame(id(501), &bridge)
            .expect("management request frame builds");
        write_frame(&mut client, &request).expect("management request writes");

        assert_eq!(
            run_authenticated_management_session_worker(
                session(server),
                permit,
                &BoundedLocalReadPolicy::deny_all(),
                context,
                status(),
                &dns(),
                worker_config(1, 500, 500),
            ),
            Ok(LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
                responses_written: 1,
            })
        );
        assert_eq!(capacity.active_workers(), 0);

        let response = read_frame(&mut client).expect("management response reads");
        let terminal = validate_terminal_response_frame(&response)
            .expect("management terminal response validates");
        assert_eq!(terminal.request_id(), id(501));
        assert_eq!(terminal.status(), LocalAgentResponseStatus::Ok);
        fs::remove_dir_all(root).expect("management worker test root removes");
    }

    #[test]
    fn legacy_command_one_never_touches_poisoned_management_lifecycle() {
        let root = test_root("legacy");
        let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root)
            .expect("management worker test root anchors");
        let policy = management_policy(Decision::Deny);
        let lifecycle = Mutex::new(lifecycle(&filesystem));
        poison_lifecycle(&lifecycle);
        let context = LocalLinuxManagementRuntimeContext::new(&filesystem, &policy, &lifecycle);
        let (server, mut client) = UnixStream::pair().expect("anonymous Unix pair creates");
        let capacity = worker_capacity();
        let permit = capacity.try_acquire().expect("worker permit acquires");
        write_local_command_request(&mut client, id(502), LocalAgentCommand::GetAgentStatus)
            .expect("legacy request writes");

        assert_eq!(
            run_authenticated_management_session_worker(
                session(server),
                permit,
                &BoundedLocalReadPolicy::allow_local_reads(),
                context,
                status(),
                &dns(),
                worker_config(1, 500, 500),
            ),
            Ok(LocalLinuxSessionWorkerStop::RequestBudgetExhausted {
                responses_written: 1,
            })
        );
        assert_eq!(capacity.active_workers(), 0);
        assert!(lifecycle.is_poisoned());

        let response = read_frame(&mut client).expect("legacy response reads");
        let decoded = decode_success_status_frame(&response).expect("legacy status response decodes");
        assert_eq!(decoded.request_id(), id(502));
        fs::remove_dir_all(root).expect("management worker test root removes");
    }

    #[test]
    fn read_deadline_failure_is_coarse_and_releases_worker_permit() {
        let root = test_root("deadline");
        let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root)
            .expect("management worker test root anchors");
        let policy = management_policy(Decision::Allow);
        let lifecycle = Mutex::new(lifecycle(&filesystem));
        let context = LocalLinuxManagementRuntimeContext::new(&filesystem, &policy, &lifecycle);
        let (server, client) = UnixStream::pair().expect("anonymous Unix pair creates");
        let capacity = worker_capacity();
        let permit = capacity.try_acquire().expect("worker permit acquires");

        assert_eq!(
            run_authenticated_management_session_worker(
                session(server),
                permit,
                &BoundedLocalReadPolicy::allow_local_reads(),
                context,
                status(),
                &dns(),
                worker_config(1, 10, 500),
            ),
            Err(LocalLinuxSessionWorkerError::ManagementProcessing {
                responses_written: 0,
            })
        );
        assert_eq!(capacity.active_workers(), 0);
        assert!(!lifecycle.is_poisoned());
        drop(client);
        fs::remove_dir_all(root).expect("management worker test root removes");
    }
}
