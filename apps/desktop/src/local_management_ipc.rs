#![allow(
    dead_code,
    reason = "Phase 152 Slice B local management IPC integration is exercised by deterministic tests before live Agent dispatch"
)]

use prw_agent::{
    LocalIpcRequestId,
    frame_object::LocalIpcFrame,
    local_commands::management_request::{
        LocalManagementRequestBuildError, build_local_management_request_frame,
    },
};
use prw_remote_bridge::{BridgeCommand, RemoteBridgeError};

/// Pure client-side failure while composing one typed local management request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalManagementClientError {
    /// Canonical PRWC encoding or decoding rejected the typed operation.
    Bridge(RemoteBridgeError),
    /// Agent-owned local command-3 framing rejected the encoded body.
    Local(LocalManagementRequestBuildError),
}

/// Encodes one existing typed bridge command and wraps it in the Agent-owned
/// local command-3 request envelope.
///
/// This function performs no socket I/O and does not treat construction as
/// authorization, dispatch, acknowledgement, or completion.
///
/// # Errors
///
/// Preserves canonical PRWC encoding failures and bounded local framing
/// failures without introducing an alternate command representation.
pub fn build_bridge_management_request(
    request_id: LocalIpcRequestId,
    command: &BridgeCommand,
) -> Result<LocalIpcFrame, LocalManagementClientError> {
    let bridge_payload = command
        .encode()
        .map_err(LocalManagementClientError::Bridge)?;
    build_local_management_request_frame(request_id, &bridge_payload)
        .map_err(LocalManagementClientError::Local)
}

/// Revalidates one canonical PRWC payload as an existing [`BridgeCommand`]
/// before delegating to the typed local command-3 request builder.
///
/// This is a presentation/integration adapter only: canonical bridge bytes are
/// not an alternate command schema, and successful construction still performs
/// no socket I/O, authorization, dispatch, acknowledgement, or completion.
///
/// # Errors
///
/// Rejects malformed canonical PRWC payloads and preserves local command-3
/// framing failures through the existing client error surface.
pub fn build_encoded_bridge_management_request(
    request_id: LocalIpcRequestId,
    bridge_payload: &[u8],
) -> Result<LocalIpcFrame, LocalManagementClientError> {
    let command =
        BridgeCommand::decode(bridge_payload).map_err(LocalManagementClientError::Bridge)?;
    build_bridge_management_request(request_id, &command)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use prw_agent::{
        LocalIpcRequestId,
        local_commands::{
            codec::{LocalAgentRequestDecodeError, decode_request_command},
            management_request::{
                LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH, decode_local_management_request_frame,
            },
        },
    };
    use prw_file_service::RemotePath;
    use prw_forwarding::{
        ForwardTarget, LoopbackBind, LoopbackFamily, PortForwardId, TcpForwardSpec,
    };
    use prw_remote_bridge::BridgeCommand;
    use prw_terminal::{TerminalGeometry, TerminalProfile, TerminalSessionId};

    use super::{build_bridge_management_request, build_encoded_bridge_management_request};

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("non-zero request id")
    }

    fn representative_commands() -> [(u64, BridgeCommand); 3] {
        let terminal = BridgeCommand::TerminalOpen {
            session_id: TerminalSessionId::new(152).expect("valid terminal session id"),
            profile: TerminalProfile::BashShell,
            geometry: TerminalGeometry::new(120, 40).expect("valid terminal geometry"),
        };
        let files =
            BridgeCommand::FileList(RemotePath::parse("docs").expect("valid relative path"));
        let forward = BridgeCommand::ForwardOpen {
            forward_id: PortForwardId::new(152).expect("valid forward id"),
            spec: TcpForwardSpec::new(
                LoopbackBind::new(LoopbackFamily::Ipv4, 41_152).expect("valid loopback bind"),
                ForwardTarget::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 22)
                    .expect("valid explicit target"),
            ),
        };

        [(152, terminal), (153, files), (154, forward)]
    }

    #[test]
    fn legacy_two_byte_namespace_remains_byte_compatible_and_rejects_code_three() {
        assert_eq!(
            decode_request_command(&[0, 1]).expect("legacy status command"),
            prw_agent::local_commands::LocalAgentCommand::GetAgentStatus
        );
        assert_eq!(
            decode_request_command(&[0, 2]).expect("legacy private DNS command"),
            prw_agent::local_commands::LocalAgentCommand::GetPrivateDnsConfig
        );
        assert_eq!(
            decode_request_command(&[0, 3]),
            Err(LocalAgentRequestDecodeError::UnknownCommand)
        );
    }

    #[test]
    fn typed_terminal_file_and_forwarding_intents_round_trip_through_local_command_three() {
        for (request_id, command) in representative_commands() {
            let expected_bridge_payload = command.encode().expect("canonical PRWC encoding");
            let expected_capability = command.required_capability();
            let frame = build_bridge_management_request(id(request_id), &command)
                .expect("typed local management request builds");

            assert_eq!(frame.header().request_id(), id(request_id));
            assert_eq!(&frame.payload().as_bytes()[..2], &[0, 3]);
            assert_eq!(
                &frame.payload().as_bytes()[LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH..],
                expected_bridge_payload.as_slice()
            );

            let local = decode_local_management_request_frame(&frame)
                .expect("Agent-owned management envelope decodes");
            assert_eq!(local.request_id(), id(request_id));
            let decoded = BridgeCommand::decode(local.bridge_payload())
                .expect("canonical PRWC bridge payload decodes");
            assert_eq!(decoded, command);
            assert_eq!(decoded.required_capability(), expected_capability);
        }
    }

    #[test]
    fn encoded_bridge_adapter_reuses_command_three_without_alternate_schema() {
        let command = BridgeCommand::FileList(
            RemotePath::parse("workspace").expect("valid relative path"),
        );
        let bridge_payload = command.encode().expect("canonical PRWC encoding");
        let frame = build_encoded_bridge_management_request(id(156), &bridge_payload)
            .expect("encoded canonical bridge payload builds local request");

        assert_eq!(frame.header().request_id(), id(156));
        assert_eq!(&frame.payload().as_bytes()[..2], &[0, 3]);
        assert_eq!(
            &frame.payload().as_bytes()[LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH..],
            bridge_payload.as_slice()
        );
        let local = decode_local_management_request_frame(&frame)
            .expect("Agent-owned management envelope decodes");
        assert_eq!(local.bridge_payload(), bridge_payload.as_slice());
    }

    #[test]
    fn local_schema_contains_only_command_length_and_canonical_bridge_bytes() {
        let command = BridgeCommand::FileList(RemotePath::parse("workspace").expect("valid path"));
        let bridge_payload = command.encode().expect("canonical PRWC encoding");
        let frame = build_bridge_management_request(id(155), &command)
            .expect("typed local management request builds");
        let local_payload = frame.payload().as_bytes();

        assert_eq!(&local_payload[..2], &[0, 3]);
        let declared = u32::from_be_bytes([
            local_payload[2],
            local_payload[3],
            local_payload[4],
            local_payload[5],
        ]);
        assert_eq!(
            usize::try_from(declared).expect("declared local body length fits usize"),
            bridge_payload.len()
        );
        assert_eq!(
            &local_payload[LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH..],
            bridge_payload.as_slice()
        );
        assert_eq!(
            local_payload.len(),
            LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH + bridge_payload.len()
        );
    }
}
