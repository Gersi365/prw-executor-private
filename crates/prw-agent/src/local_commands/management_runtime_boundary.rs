//! C03 runtime boundary transaction with late provider-lifecycle acquisition.
//!
//! The frame is acquired and classified before the shared management lifecycle mutex is
//! touched. Legacy commands 1/2 therefore retain the existing read-only decoder, policy,
//! response builder, and guarded writer without depending on management provider state.
//! Additive command 3 delegates the complete frame to the shared C03 runtime context.

#![cfg(target_os = "linux")]

use std::io::{Read, Write};

use prw_policy::PolicyEvaluator;

use super::boundary_request_response_transaction::LocalBoundaryRequestResponseOutcome;
use super::inbound_state::LocalInboundRequestState;
use super::management_request::LOCAL_MANAGEMENT_BRIDGE_COMMAND_CODE;
use super::management_runtime::LocalLinuxManagementRuntimeContext;
use super::policy_response::{
    LocalPolicyResponseBuildError, build_policy_gated_read_only_response,
};
use super::private_dns_snapshot::LocalPrivateDnsSnapshot;
use super::request_frame::{LocalAgentRequestFrameDecodeError, decode_local_command_request_frame};
use super::response_writer::{
    LocalTerminalResponseWriteError, LocalTerminalResponseWriteState,
    write_terminal_response_guarded,
};
use super::status_snapshot::LocalAgentStatusSnapshot;
use super::terminal_response::builder::LocalTerminalResponseBuildError;
use crate::frame_object::boundary_reader::{LocalIpcFrameBoundaryRead, read_frame_at_boundary};
use crate::frame_object::reader::LocalIpcFrameReadError;
use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;

/// One lock-late C03 runtime boundary failure after exact connection-state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LocalManagementRuntimeBoundaryError {
    /// Generic frame acquisition failed and poisoned the inbound direction.
    FrameRead(LocalIpcFrameReadError),
    /// A non-management frame failed the exact legacy commands-1/2 decoder and poisoned inbound.
    ReadOnlyDecode(LocalAgentRequestFrameDecodeError),
    /// Legacy policy-response construction failed before any response write.
    ReadOnlyResponse(LocalPolicyResponseBuildError),
    /// C03 management response construction failed before any response write.
    ManagementResponse(LocalTerminalResponseBuildError),
    /// Response writing failed through the existing guarded writer.
    ResponseWrite(LocalTerminalResponseWriteError),
}

/// Processes one local request while acquiring shared provider state only for command 3.
///
/// Commands 1/2 never inspect or lock the management runtime context. This keeps legacy
/// response bytes independent of management lifecycle health and prevents an idle or
/// read-only local connection from serializing provider state before request classification.
#[expect(
    clippy::too_many_arguments,
    reason = "boundary state, read policy, authenticated peer, and management context remain explicit"
)]
pub(super) fn process_one_runtime_management_capable_at_boundary<R, W, RE, S>(
    reader: &mut R,
    writer: &mut W,
    inbound_state: &mut LocalInboundRequestState,
    response_write_state: &mut LocalTerminalResponseWriteState,
    connection: &AuthenticatedLocalLinuxConnection<S>,
    read_evaluator: &RE,
    management_context: LocalLinuxManagementRuntimeContext<'_, '_>,
    status_snapshot: LocalAgentStatusSnapshot,
    private_dns_snapshot: &LocalPrivateDnsSnapshot,
) -> Result<LocalBoundaryRequestResponseOutcome, LocalManagementRuntimeBoundaryError>
where
    R: Read,
    W: Write,
    RE: PolicyEvaluator + ?Sized,
{
    if inbound_state.is_read_poisoned() {
        return Err(LocalManagementRuntimeBoundaryError::FrameRead(
            LocalIpcFrameReadError::HeaderIo,
        ));
    }
    if response_write_state.is_write_poisoned() {
        return Err(LocalManagementRuntimeBoundaryError::ResponseWrite(
            LocalTerminalResponseWriteError::WritePoisoned,
        ));
    }

    let frame = match read_frame_at_boundary(reader) {
        Ok(LocalIpcFrameBoundaryRead::CleanEof) => {
            return Ok(LocalBoundaryRequestResponseOutcome::CleanEof);
        }
        Ok(LocalIpcFrameBoundaryRead::Frame(frame)) => frame,
        Err(error) => {
            *inbound_state = LocalInboundRequestState::ReadPoisoned;
            return Err(LocalManagementRuntimeBoundaryError::FrameRead(error));
        }
    };

    let response = if payload_command_code(&frame) == Some(LOCAL_MANAGEMENT_BRIDGE_COMMAND_CODE) {
        management_context
            .process_management_frame(&frame, connection, status_snapshot)
            .map_err(LocalManagementRuntimeBoundaryError::ManagementResponse)?
    } else {
        let request = match decode_local_command_request_frame(&frame) {
            Ok(request) => request,
            Err(error) => {
                *inbound_state = LocalInboundRequestState::ReadPoisoned;
                return Err(LocalManagementRuntimeBoundaryError::ReadOnlyDecode(error));
            }
        };
        build_policy_gated_read_only_response(
            request,
            read_evaluator,
            status_snapshot,
            private_dns_snapshot,
        )
        .map_err(LocalManagementRuntimeBoundaryError::ReadOnlyResponse)?
    };

    write_terminal_response_guarded(response_write_state, writer, &response)
        .map_err(LocalManagementRuntimeBoundaryError::ResponseWrite)?;
    Ok(LocalBoundaryRequestResponseOutcome::ResponseWritten)
}

fn payload_command_code(frame: &crate::frame_object::LocalIpcFrame) -> Option<u16> {
    let bytes = frame.payload().as_bytes();
    (bytes.len() >= 2).then(|| u16::from_be_bytes([bytes[0], bytes[1]]))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Cursor;
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    use prw_network::PrivateDnsConfig;
    use prw_policy::{
        BoundedLocalManagementDecisions, BoundedLocalManagementPolicy, BoundedLocalReadPolicy,
        Decision,
    };
    use prw_remote_bridge::BridgeCommand;

    use super::process_one_runtime_management_capable_at_boundary;
    use crate::LocalIpcRequestId;
    use crate::frame_object::reader::read_frame;
    use crate::frame_object::writer::write_frame;
    use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
    use crate::local_commands::LocalAgentCommand;
    use crate::local_commands::LocalAgentResponseStatus;
    use crate::local_commands::boundary_request_response_transaction::{
        LocalBoundaryRequestResponseOutcome, process_and_write_one_read_only_request_at_boundary,
    };
    use crate::local_commands::inbound_state::LocalInboundRequestState;
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
    use crate::local_commands::response_writer::LocalTerminalResponseWriteState;
    use crate::local_commands::status_snapshot::{LocalAgentRuntimeState, LocalAgentStatusSnapshot};
    use crate::local_commands::terminal_response::validate_terminal_response_frame;

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
                "prw-c03-runtime-boundary-{}-{sequence}-{label}",
                std::process::id()
            ));
            fs::create_dir(&root_path).expect("runtime-boundary test root creates");
            let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root_path)
                .expect("runtime-boundary test root anchors");
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

    fn poison_lifecycle(lifecycle: &Mutex<LocalLinuxManagementProviderLifecycle<'_>>) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = lifecycle.lock().expect("lifecycle lock starts healthy");
            panic!("intentional runtime-boundary poison");
        }));
        assert!(lifecycle.is_poisoned());
    }

    fn encoded_legacy_request(request_id: u64, command: LocalAgentCommand) -> Vec<u8> {
        let frame = build_local_command_request_frame(id(request_id), command)
            .expect("legacy request frame builds");
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &frame).expect("legacy request writes to memory");
        bytes
    }

    #[test]
    fn poisoned_management_lifecycle_cannot_change_legacy_command_one_response_bytes() {
        let harness = Harness::new("legacy-poison");
        let request = encoded_legacy_request(41, LocalAgentCommand::GetAgentStatus);
        let read_policy = BoundedLocalReadPolicy::allow_local_reads();
        let dns = dns();

        let mut baseline_output = Vec::new();
        let mut baseline_write_state = LocalTerminalResponseWriteState::new();
        assert_eq!(
            process_and_write_one_read_only_request_at_boundary(
                &mut Cursor::new(request.clone()),
                &mut baseline_output,
                &mut baseline_write_state,
                &read_policy,
                status(),
                &dns,
            ),
            Ok(LocalBoundaryRequestResponseOutcome::ResponseWritten)
        );

        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        poison_lifecycle(&lifecycle);
        let management_policy = management_policy();
        let context = LocalLinuxManagementRuntimeContext::new(
            &harness.filesystem,
            &management_policy,
            &lifecycle,
        );
        let mut runtime_output = Vec::new();
        let mut inbound_state = LocalInboundRequestState::new();
        let mut runtime_write_state = LocalTerminalResponseWriteState::new();

        assert_eq!(
            process_one_runtime_management_capable_at_boundary(
                &mut Cursor::new(request),
                &mut runtime_output,
                &mut inbound_state,
                &mut runtime_write_state,
                &harness.server,
                &read_policy,
                context,
                status(),
                &dns,
            ),
            Ok(LocalBoundaryRequestResponseOutcome::ResponseWritten)
        );
        assert_eq!(runtime_output, baseline_output);
        assert!(inbound_state.can_read());
        assert!(runtime_write_state.can_write());
        assert!(lifecycle.is_poisoned());
    }

    #[test]
    fn clean_eof_does_not_touch_poisoned_management_lifecycle() {
        let harness = Harness::new("eof-poison");
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        poison_lifecycle(&lifecycle);
        let management_policy = management_policy();
        let context = LocalLinuxManagementRuntimeContext::new(
            &harness.filesystem,
            &management_policy,
            &lifecycle,
        );
        let mut output = Vec::new();
        let mut inbound_state = LocalInboundRequestState::new();
        let mut response_write_state = LocalTerminalResponseWriteState::new();
        let dns = dns();

        assert_eq!(
            process_one_runtime_management_capable_at_boundary(
                &mut Cursor::new(Vec::<u8>::new()),
                &mut output,
                &mut inbound_state,
                &mut response_write_state,
                &harness.server,
                &BoundedLocalReadPolicy::allow_local_reads(),
                context,
                status(),
                &dns,
            ),
            Ok(LocalBoundaryRequestResponseOutcome::CleanEof)
        );
        assert!(output.is_empty());
        assert!(inbound_state.can_read());
        assert!(response_write_state.can_write());
        assert!(lifecycle.is_poisoned());
    }

    #[test]
    fn command_three_poisoned_lifecycle_returns_correlated_internal_error() {
        let harness = Harness::new("management-poison");
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        poison_lifecycle(&lifecycle);
        let management_policy = management_policy();
        let context = LocalLinuxManagementRuntimeContext::new(
            &harness.filesystem,
            &management_policy,
            &lifecycle,
        );
        let bridge = BridgeCommand::AgentStatus.encode().expect("bridge command encodes");
        let frame = build_local_management_request_frame(id(77), &bridge)
            .expect("management request frame builds");
        let mut input = Vec::new();
        write_frame(&mut input, &frame).expect("management request writes to memory");
        let mut output = Vec::new();
        let mut inbound_state = LocalInboundRequestState::new();
        let mut response_write_state = LocalTerminalResponseWriteState::new();
        let dns = dns();

        assert_eq!(
            process_one_runtime_management_capable_at_boundary(
                &mut Cursor::new(input),
                &mut output,
                &mut inbound_state,
                &mut response_write_state,
                &harness.server,
                &BoundedLocalReadPolicy::deny_all(),
                context,
                status(),
                &dns,
            ),
            Ok(LocalBoundaryRequestResponseOutcome::ResponseWritten)
        );
        let response = read_frame(&mut Cursor::new(output)).expect("response frame reads");
        let terminal = validate_terminal_response_frame(&response)
            .expect("management poison response validates");
        assert_eq!(terminal.request_id(), id(77));
        assert_eq!(terminal.status(), LocalAgentResponseStatus::InternalError);
        assert!(inbound_state.can_read());
        assert!(response_write_state.can_write());
    }
}
