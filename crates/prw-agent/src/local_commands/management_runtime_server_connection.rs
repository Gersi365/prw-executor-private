//! C03 runtime management composition over the existing aggregate server connection state.
//!
//! This layer does not own a second inbound or response-write state. It checks the same
//! `LocalServerConnectionState` used by commands 1/2, borrows its component states, and
//! delegates one request to the lock-late runtime boundary.

#![cfg(target_os = "linux")]

use std::io::{Read, Write};

use prw_policy::PolicyEvaluator;

use super::boundary_request_response_transaction::LocalBoundaryRequestResponseOutcome;
use super::management_runtime::LocalLinuxManagementRuntimeContext;
use super::management_runtime_boundary::{
    LocalManagementRuntimeBoundaryError, process_one_runtime_management_capable_at_boundary,
};
use super::private_dns_snapshot::LocalPrivateDnsSnapshot;
use super::server_connection_state::{
    LocalServerConnectionState, LocalServerConnectionUnusableReason,
};
use super::status_snapshot::LocalAgentStatusSnapshot;
use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;

/// One C03 runtime server-connection failure over the existing aggregate state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LocalManagementRuntimeServerConnectionError {
    /// The aggregate state was already unusable before any I/O.
    ConnectionUnusable(LocalServerConnectionUnusableReason),
    /// The lock-late boundary failed after applying its authoritative state transition.
    Transaction(LocalManagementRuntimeBoundaryError),
}

/// Processes one lock-late management-capable request through the existing aggregate state.
#[expect(
    clippy::too_many_arguments,
    reason = "authenticated peer, read policy, runtime authority, and protocol snapshots remain explicit"
)]
pub(super) fn process_one_runtime_management_at_boundary_on_server_connection<R, W, RE, S>(
    reader: &mut R,
    writer: &mut W,
    state: &mut LocalServerConnectionState,
    connection: &AuthenticatedLocalLinuxConnection<S>,
    read_evaluator: &RE,
    management_context: LocalLinuxManagementRuntimeContext<'_, '_>,
    status_snapshot: LocalAgentStatusSnapshot,
    private_dns_snapshot: &LocalPrivateDnsSnapshot,
) -> Result<LocalBoundaryRequestResponseOutcome, LocalManagementRuntimeServerConnectionError>
where
    R: Read,
    W: Write,
    RE: PolicyEvaluator + ?Sized,
{
    if let Some(reason) = state.unusable_reason() {
        return Err(LocalManagementRuntimeServerConnectionError::ConnectionUnusable(reason));
    }

    let (inbound_state, response_write_state) = state.runtime_component_states_mut();
    process_one_runtime_management_capable_at_boundary(
        reader,
        writer,
        inbound_state,
        response_write_state,
        connection,
        read_evaluator,
        management_context,
        status_snapshot,
        private_dns_snapshot,
    )
    .map_err(LocalManagementRuntimeServerConnectionError::Transaction)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{Cursor, Error, Result as IoResult, Write};
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    use prw_network::PrivateDnsConfig;
    use prw_policy::{
        BoundedLocalManagementDecisions, BoundedLocalManagementPolicy, BoundedLocalReadPolicy,
        Decision,
    };

    use super::{
        LocalManagementRuntimeServerConnectionError,
        process_one_runtime_management_at_boundary_on_server_connection,
    };
    use crate::LocalIpcRequestId;
    use crate::frame_object::writer::write_frame;
    use crate::frame_object::{LocalIpcFrame, LocalIpcPayload};
    use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
    use crate::local_commands::LocalAgentCommand;
    use crate::local_commands::management_authority::LocalManagementFilesystemAuthority;
    use crate::local_commands::management_linux_backends::{
        LinuxLocalForwardingBackend, LinuxLocalTerminalBackend,
    };
    use crate::local_commands::management_provider_backend_policy::ExactForwardingEgressPolicy;
    use crate::local_commands::management_provider_lifecycle::LocalManagementProviderLifecycle;
    use crate::local_commands::management_runtime::{
        LocalLinuxManagementProviderLifecycle, LocalLinuxManagementRuntimeContext,
    };
    use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
    use crate::local_commands::request_frame::build_local_command_request_frame;
    use crate::local_commands::response_writer::LocalTerminalResponseWriteError;
    use crate::local_commands::server_connection_state::{
        LocalServerConnectionState, LocalServerConnectionUnusableReason,
    };
    use crate::local_commands::status_snapshot::{LocalAgentRuntimeState, LocalAgentStatusSnapshot};
    use crate::{LocalIpcFrameHeader, LocalIpcMessageKind, LocalIpcProtocolVersion};

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

    struct Harness {
        root_path: PathBuf,
        filesystem: LocalManagementFilesystemAuthority,
        server: AuthenticatedLocalLinuxConnection<UnixStream>,
        client: UnixStream,
    }

    impl Harness {
        fn new(label: &str) -> Self {
            let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let root_path = std::env::temp_dir().join(format!(
                "prw-c03-runtime-server-state-{}-{sequence}-{label}",
                std::process::id()
            ));
            fs::create_dir(&root_path).expect("runtime-server-state test root creates");
            let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root_path)
                .expect("runtime-server-state test root anchors");
            let (server, client) = UnixStream::pair().expect("same-user local pair creates");
            let server = AuthenticatedLocalLinuxConnection::try_new(server)
                .expect("same-UID local pair authenticates");
            Self {
                root_path,
                filesystem,
                server,
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

    fn management_policy() -> BoundedLocalManagementPolicy {
        BoundedLocalManagementPolicy::new(BoundedLocalManagementDecisions {
            agent_status: Decision::Allow,
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
            panic!("intentional runtime-server-state poison");
        }));
        assert!(lifecycle.is_poisoned());
    }

    fn encoded_legacy_request(request_id: u64) -> Vec<u8> {
        let frame =
            build_local_command_request_frame(id(request_id), LocalAgentCommand::GetAgentStatus)
                .expect("legacy request frame builds");
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &frame).expect("legacy request writes to memory");
        bytes
    }

    fn malformed_non_management_frame(request_id: u64) -> Vec<u8> {
        let payload = LocalIpcPayload::new(vec![0, 99]).expect("bounded malformed payload");
        let header = LocalIpcFrameHeader::new(
            LocalIpcProtocolVersion::current(),
            LocalIpcMessageKind::Request,
            id(request_id),
            payload.len(),
        )
        .expect("matching malformed frame header");
        let frame = LocalIpcFrame::new(header, payload).expect("matching malformed frame");
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &frame).expect("malformed frame writes to memory");
        bytes
    }

    #[test]
    fn poisoned_management_lifecycle_legacy_request_keeps_same_aggregate_usable() {
        let harness = Harness::new("legacy");
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        poison_lifecycle(&lifecycle);
        let policy = management_policy();
        let mut state = LocalServerConnectionState::new();
        let mut output = Vec::new();
        let dns = dns();

        process_one_runtime_management_at_boundary_on_server_connection(
            &mut Cursor::new(encoded_legacy_request(51)),
            &mut output,
            &mut state,
            &harness.server,
            &BoundedLocalReadPolicy::allow_local_reads(),
            context(&harness.filesystem, &policy, &lifecycle),
            status(),
            &dns,
        )
        .expect("legacy request succeeds without management lock");

        assert!(state.is_usable());
        assert!(!output.is_empty());
        assert!(lifecycle.is_poisoned());
    }

    #[test]
    fn malformed_legacy_frame_poisons_inbound_on_same_aggregate_state() {
        let harness = Harness::new("malformed");
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        let policy = management_policy();
        let mut state = LocalServerConnectionState::new();
        let mut output = Vec::new();
        let dns = dns();

        assert!(
            process_one_runtime_management_at_boundary_on_server_connection(
                &mut Cursor::new(malformed_non_management_frame(52)),
                &mut output,
                &mut state,
                &harness.server,
                &BoundedLocalReadPolicy::allow_local_reads(),
                context(&harness.filesystem, &policy, &lifecycle),
                status(),
                &dns,
            )
            .is_err()
        );
        assert_eq!(
            state.unusable_reason(),
            Some(LocalServerConnectionUnusableReason::InboundRead)
        );
        assert!(output.is_empty());
    }

    #[test]
    fn response_write_failure_poisons_write_on_same_aggregate_state() {
        let harness = Harness::new("write-failure");
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        let policy = management_policy();
        let mut state = LocalServerConnectionState::new();
        let dns = dns();
        let mut writer = FailImmediately;

        let error = process_one_runtime_management_at_boundary_on_server_connection(
            &mut Cursor::new(encoded_legacy_request(53)),
            &mut writer,
            &mut state,
            &harness.server,
            &BoundedLocalReadPolicy::allow_local_reads(),
            context(&harness.filesystem, &policy, &lifecycle),
            status(),
            &dns,
        )
        .expect_err("write failure is reported");

        assert!(matches!(
            error,
            LocalManagementRuntimeServerConnectionError::Transaction(
                crate::local_commands::management_runtime_boundary::LocalManagementRuntimeBoundaryError::ResponseWrite(
                    LocalTerminalResponseWriteError::Io
                )
            )
        ));
        assert_eq!(
            state.unusable_reason(),
            Some(LocalServerConnectionUnusableReason::ResponseWrite)
        );
    }

    struct FailImmediately;

    impl Write for FailImmediately {
        fn write(&mut self, _buf: &[u8]) -> IoResult<usize> {
            Err(Error::other("intentional write failure"))
        }

        fn flush(&mut self) -> IoResult<()> {
            Ok(())
        }
    }
}
