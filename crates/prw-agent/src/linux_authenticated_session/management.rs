//! C03 management-capable deadline composition for an authenticated Linux session.
//!
//! This child module can access the parent session's private authenticated connection and
//! aggregate protocol state without widening the public API. The legacy deadline method in
//! the parent module remains unchanged; this sibling method is crate-private and additive.

use std::os::unix::net::UnixStream;

use prw_policy::PolicyEvaluator;

use super::AuthenticatedLocalLinuxSession;
use crate::linux_identity::deadline_io::{
    LocalLinuxDeadlineReader, LocalLinuxDeadlineStartError, LocalLinuxDeferredDeadlineWriter,
    LocalLinuxIoBudget,
};
use crate::local_commands::boundary_request_response_transaction::LocalBoundaryRequestResponseOutcome;
use crate::local_commands::management_runtime::LocalLinuxManagementRuntimeContext;
use crate::local_commands::management_runtime_server_connection::{
    LocalManagementRuntimeServerConnectionError,
    process_one_runtime_management_at_boundary_on_server_connection,
};
use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
use crate::local_commands::status_snapshot::LocalAgentStatusSnapshot;

impl AuthenticatedLocalLinuxSession<UnixStream> {
    /// Processes exactly one management-capable boundary request with independent I/O budgets.
    ///
    /// The read deadline starts immediately before frame acquisition, exactly like the legacy
    /// deadline path. The response-write deadline remains deferred until the first non-empty
    /// write. Shared management provider state is acquired only after command-3 classification
    /// in the delegated lock-late boundary.
    ///
    /// # Errors
    ///
    /// Returns a read-deadline construction error before I/O, or the coarse crate-internal
    /// management server-connection failure after authoritative state transitions.
    pub(crate) fn process_one_management_with_deadlines<RE: PolicyEvaluator + ?Sized>(
        &mut self,
        read_evaluator: &RE,
        management_context: LocalLinuxManagementRuntimeContext<'_, '_>,
        status_snapshot: LocalAgentStatusSnapshot,
        private_dns_snapshot: &LocalPrivateDnsSnapshot,
        read_budget: LocalLinuxIoBudget,
        write_budget: LocalLinuxIoBudget,
    ) -> Result<LocalBoundaryRequestResponseOutcome, LocalLinuxManagementDeadlineSessionProcessError>
    {
        let Self { connection, state } = self;
        let stream = connection.stream();
        let mut reader = LocalLinuxDeadlineReader::start(stream, read_budget)
            .map_err(LocalLinuxManagementDeadlineSessionProcessError::ReadDeadlineStart)?;
        let mut writer = LocalLinuxDeferredDeadlineWriter::new(stream, write_budget);

        process_one_runtime_management_at_boundary_on_server_connection(
            &mut reader,
            &mut writer,
            state,
            connection,
            read_evaluator,
            management_context,
            status_snapshot,
            private_dns_snapshot,
        )
        .map_err(LocalLinuxManagementDeadlineSessionProcessError::Processing)
    }
}

/// Crate-internal C03 deadline-session failure without changing the legacy public enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLinuxManagementDeadlineSessionProcessError {
    /// The absolute Request-read deadline could not be constructed.
    ReadDeadlineStart(LocalLinuxDeadlineStartError),
    /// The management-capable aggregate Request pipeline failed.
    Processing(LocalManagementRuntimeServerConnectionError),
}

#[cfg(test)]
mod tests {
    use std::fs;
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

    use super::AuthenticatedLocalLinuxSession;
    use crate::LocalIpcRequestId;
    use crate::frame_object::reader::read_frame;
    use crate::frame_object::writer::write_frame;
    use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
    use crate::linux_identity::deadline_io::LocalLinuxIoBudget;
    use crate::local_commands::LocalAgentCommand;
    use crate::local_commands::LocalAgentResponseStatus;
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
                "prw-c03-management-deadline-{}-{sequence}-{label}",
                std::process::id()
            ));
            fs::create_dir(&root_path).expect("management-deadline test root creates");
            let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root_path)
                .expect("management-deadline test root anchors");
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

    fn io_budget() -> LocalLinuxIoBudget {
        LocalLinuxIoBudget::try_new(Duration::from_secs(2)).expect("test I/O budget is non-zero")
    }

    fn status() -> LocalAgentStatusSnapshot {
        LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)
    }

    fn dns() -> LocalPrivateDnsSnapshot {
        LocalPrivateDnsSnapshot::try_from_config(&PrivateDnsConfig::default())
            .expect("default DNS config is bounded")
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

    fn context<'context, 'authority>(
        filesystem: &'authority LocalManagementFilesystemAuthority,
        policy: &'context BoundedLocalManagementPolicy,
        lifecycle: &'context Mutex<LocalLinuxManagementProviderLifecycle<'authority>>,
    ) -> LocalLinuxManagementRuntimeContext<'context, 'authority> {
        LocalLinuxManagementRuntimeContext::new(filesystem, policy, lifecycle)
    }

    fn poison_lifecycle(lifecycle: &Mutex<LocalLinuxManagementProviderLifecycle<'_>>) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = lifecycle.lock().expect("lifecycle lock starts healthy");
            panic!("intentional management-deadline poison");
        }));
        assert!(lifecycle.is_poisoned());
    }

    #[test]
    fn command_three_agent_status_is_correlated_through_deadline_session() {
        let mut harness = Harness::new("management");
        let policy = management_policy(Decision::Allow);
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        let bridge = BridgeCommand::AgentStatus
            .encode()
            .expect("bridge command encodes");
        let frame = build_local_management_request_frame(id(91), &bridge)
            .expect("management request frame builds");
        write_frame(&mut harness.client, &frame).expect("management request writes");

        harness
            .session
            .process_one_management_with_deadlines(
                &BoundedLocalReadPolicy::deny_all(),
                context(&harness.filesystem, &policy, &lifecycle),
                status(),
                &dns(),
                io_budget(),
                io_budget(),
            )
            .expect("management deadline request succeeds");

        let response = read_frame(&mut harness.client).expect("management response reads");
        let terminal =
            validate_terminal_response_frame(&response).expect("management response validates");
        assert_eq!(terminal.request_id(), id(91));
        assert_eq!(terminal.status(), LocalAgentResponseStatus::Ok);
        assert!(harness.session.state().is_usable());
    }

    #[test]
    fn legacy_command_one_does_not_touch_poisoned_lifecycle_under_deadlines() {
        let mut harness = Harness::new("legacy");
        let policy = management_policy(Decision::Deny);
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        poison_lifecycle(&lifecycle);
        let frame = build_local_command_request_frame(id(92), LocalAgentCommand::GetAgentStatus)
            .expect("legacy request frame builds");
        write_frame(&mut harness.client, &frame).expect("legacy request writes");

        harness
            .session
            .process_one_management_with_deadlines(
                &BoundedLocalReadPolicy::allow_local_reads(),
                context(&harness.filesystem, &policy, &lifecycle),
                status(),
                &dns(),
                io_budget(),
                io_budget(),
            )
            .expect("legacy request succeeds through management deadline path");

        let response = read_frame(&mut harness.client).expect("legacy response reads");
        let decoded =
            decode_success_status_frame(&response).expect("legacy status response decodes");
        assert_eq!(decoded.request_id(), id(92));
        assert!(harness.session.state().is_usable());
        assert!(lifecycle.is_poisoned());
    }
}
