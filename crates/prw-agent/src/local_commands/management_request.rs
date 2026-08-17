//! Bounded framing for Phase 152 local typed management requests.
//!
//! This module owns only the additive local command-3 envelope. The embedded
//! bytes remain opaque to `prw-agent` during Slice B and are validated through
//! the canonical `prw-remote-bridge::BridgeCommand` codec at the existing
//! integration boundary. No policy evaluation, provider dispatch, socket I/O,
//! or host mutation occurs here.

use std::fmt;

use crate::frame_object::{
    LocalIpcFrame, LocalIpcFrameError, LocalIpcPayload, LocalIpcPayloadError,
};
use crate::{
    LocalIpcFrameHeader, LocalIpcFrameHeaderError, LocalIpcMessageKind, LocalIpcProtocolVersion,
    LocalIpcRequestId,
};

/// Additive Phase 152 local command identifier for one canonical PRWC management payload.
pub const LOCAL_MANAGEMENT_BRIDGE_COMMAND_CODE: u16 = 3;
/// Fixed command and embedded-length prefix: two-byte command plus four-byte length.
pub const LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH: usize = 6;
/// Maximum embedded canonical PRWC payload retained from the Phase 140/143 control bound.
pub const LOCAL_MANAGEMENT_BRIDGE_MAX_PAYLOAD_LENGTH: usize = 65_536;
/// Maximum complete local command-3 payload under the retained PRWC bound.
pub const LOCAL_MANAGEMENT_REQUEST_MAX_PAYLOAD_LENGTH: usize =
    LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH + LOCAL_MANAGEMENT_BRIDGE_MAX_PAYLOAD_LENGTH;

/// One decoded local management request with outer local correlation preserved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalManagementRequestEnvelope {
    request_id: LocalIpcRequestId,
    bridge_payload: Vec<u8>,
}

impl LocalManagementRequestEnvelope {
    /// Returns the existing local IPC request-correlation identifier.
    #[must_use]
    pub const fn request_id(&self) -> LocalIpcRequestId {
        self.request_id
    }

    /// Returns the exact embedded canonical PRWC payload bytes.
    #[must_use]
    pub const fn bridge_payload(&self) -> &[u8] {
        self.bridge_payload.as_slice()
    }
}

/// Builds one local command-3 Request frame around an already encoded canonical PRWC payload.
///
/// # Errors
///
/// Rejects an empty embedded payload, an embedded payload above the retained
/// 65,536-byte PRWC bound, or a defensive failure from the validated local
/// payload/header/frame constructors.
pub fn build_local_management_request_frame(
    request_id: LocalIpcRequestId,
    bridge_payload: &[u8],
) -> Result<LocalIpcFrame, LocalManagementRequestBuildError> {
    if bridge_payload.is_empty() {
        return Err(LocalManagementRequestBuildError::EmptyBridgePayload);
    }
    if bridge_payload.len() > LOCAL_MANAGEMENT_BRIDGE_MAX_PAYLOAD_LENGTH {
        return Err(LocalManagementRequestBuildError::BridgePayloadTooLarge);
    }

    let bridge_payload_length = u32::try_from(bridge_payload.len())
        .map_err(|_| LocalManagementRequestBuildError::BridgePayloadTooLarge)?;
    let mut bytes =
        Vec::with_capacity(LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH + bridge_payload.len());
    bytes.extend_from_slice(&LOCAL_MANAGEMENT_BRIDGE_COMMAND_CODE.to_be_bytes());
    bytes.extend_from_slice(&bridge_payload_length.to_be_bytes());
    bytes.extend_from_slice(bridge_payload);

    let payload = LocalIpcPayload::new(bytes).map_err(LocalManagementRequestBuildError::Payload)?;
    let header = LocalIpcFrameHeader::new(
        LocalIpcProtocolVersion::current(),
        LocalIpcMessageKind::Request,
        request_id,
        payload.len(),
    )
    .map_err(LocalManagementRequestBuildError::Header)?;

    LocalIpcFrame::new(header, payload).map_err(LocalManagementRequestBuildError::Frame)
}

/// Decodes only the additive local command-3 framing envelope.
///
/// # Errors
///
/// Fails closed for a non-Request frame, short prefix, wrong command code,
/// zero-length or oversized embedded body, or any mismatch between the declared
/// embedded length and the exact remaining bytes. The embedded PRWC body is not
/// interpreted by this Agent-owned framing module in Slice B.
pub fn decode_local_management_request_frame(
    frame: &LocalIpcFrame,
) -> Result<LocalManagementRequestEnvelope, LocalManagementRequestDecodeError> {
    if frame.header().kind() != LocalIpcMessageKind::Request {
        return Err(LocalManagementRequestDecodeError::NonRequestKind);
    }

    let payload = frame.payload().as_bytes();
    if payload.len() < LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH {
        return Err(LocalManagementRequestDecodeError::PrefixTooShort);
    }

    let command_code = u16::from_be_bytes([payload[0], payload[1]]);
    if command_code != LOCAL_MANAGEMENT_BRIDGE_COMMAND_CODE {
        return Err(LocalManagementRequestDecodeError::WrongCommand);
    }

    let declared_u32 = u32::from_be_bytes([payload[2], payload[3], payload[4], payload[5]]);
    if declared_u32 == 0 {
        return Err(LocalManagementRequestDecodeError::EmptyBridgePayload);
    }
    if declared_u32 > 65_536 {
        return Err(LocalManagementRequestDecodeError::BridgePayloadTooLarge);
    }
    let declared = usize::try_from(declared_u32)
        .map_err(|_| LocalManagementRequestDecodeError::BridgePayloadTooLarge)?;
    let actual = payload.len() - LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH;
    if actual != declared {
        return Err(LocalManagementRequestDecodeError::LengthMismatch { declared, actual });
    }

    Ok(LocalManagementRequestEnvelope {
        request_id: frame.header().request_id(),
        bridge_payload: payload[LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH..].to_vec(),
    })
}

/// Defensive failure while constructing a local management request frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalManagementRequestBuildError {
    /// Canonical PRWC payload must not be empty.
    EmptyBridgePayload,
    /// Canonical PRWC payload exceeds the retained Phase 140/143 bound.
    BridgePayloadTooLarge,
    /// Validated local payload construction failed.
    Payload(LocalIpcPayloadError),
    /// Validated local header construction failed.
    Header(LocalIpcFrameHeaderError),
    /// Local header/payload coupling failed.
    Frame(LocalIpcFrameError),
}

impl fmt::Display for LocalManagementRequestBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyBridgePayload => "local management bridge payload must not be empty",
            Self::BridgePayloadTooLarge => "local management bridge payload exceeds PRWC bound",
            Self::Payload(_) => "local management request payload construction failed",
            Self::Header(_) => "local management request header construction failed",
            Self::Frame(_) => "local management request frame construction failed",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for LocalManagementRequestBuildError {}

/// Fail-closed failure while decoding the local command-3 framing envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalManagementRequestDecodeError {
    /// Only outer local `Request` frames are valid management requests.
    NonRequestKind,
    /// Payload is shorter than the fixed six-byte management prefix.
    PrefixTooShort,
    /// Payload does not carry local management command code 3.
    WrongCommand,
    /// Embedded canonical PRWC body has zero bytes.
    EmptyBridgePayload,
    /// Embedded canonical PRWC body exceeds the retained bound.
    BridgePayloadTooLarge,
    /// Declared embedded length differs from the exact remaining bytes.
    LengthMismatch {
        /// Length declared by the management prefix.
        declared: usize,
        /// Exact number of bytes following the prefix.
        actual: usize,
    },
}

impl fmt::Display for LocalManagementRequestDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonRequestKind => formatter.write_str("local management frame is not a Request"),
            Self::PrefixTooShort => {
                formatter.write_str("local management payload prefix is truncated")
            }
            Self::WrongCommand => {
                formatter.write_str("local management payload has wrong command code")
            }
            Self::EmptyBridgePayload => {
                formatter.write_str("local management bridge payload is empty")
            }
            Self::BridgePayloadTooLarge => {
                formatter.write_str("local management bridge payload exceeds PRWC bound")
            }
            Self::LengthMismatch { declared, actual } => write!(
                formatter,
                "local management bridge payload length mismatch: declared {declared}, actual {actual}"
            ),
        }
    }
}

impl std::error::Error for LocalManagementRequestDecodeError {}

#[cfg(test)]
mod tests {
    use super::{
        LOCAL_MANAGEMENT_BRIDGE_COMMAND_CODE, LOCAL_MANAGEMENT_BRIDGE_MAX_PAYLOAD_LENGTH,
        LOCAL_MANAGEMENT_REQUEST_MAX_PAYLOAD_LENGTH, LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH,
        LocalManagementRequestBuildError, LocalManagementRequestDecodeError,
        build_local_management_request_frame, decode_local_management_request_frame,
    };
    use crate::frame_object::{LocalIpcFrame, LocalIpcPayload};
    use crate::{
        LocalIpcFrameHeader, LocalIpcMessageKind, LocalIpcProtocolVersion, LocalIpcRequestId,
    };

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("non-zero request id")
    }

    fn frame(kind: LocalIpcMessageKind, request_id: u64, payload: Vec<u8>) -> LocalIpcFrame {
        let payload = LocalIpcPayload::new(payload).expect("bounded local payload");
        let header = LocalIpcFrameHeader::new(
            LocalIpcProtocolVersion::current(),
            kind,
            id(request_id),
            payload.len(),
        )
        .expect("valid local header");
        LocalIpcFrame::new(header, payload).expect("matching frame")
    }

    #[test]
    fn management_constants_lock_additive_command_and_bounds() {
        assert_eq!(LOCAL_MANAGEMENT_BRIDGE_COMMAND_CODE, 3);
        assert_eq!(LOCAL_MANAGEMENT_REQUEST_PREFIX_LENGTH, 6);
        assert_eq!(LOCAL_MANAGEMENT_BRIDGE_MAX_PAYLOAD_LENGTH, 65_536);
        assert_eq!(LOCAL_MANAGEMENT_REQUEST_MAX_PAYLOAD_LENGTH, 65_542);
    }

    #[test]
    fn management_payload_round_trips_with_outer_request_id() {
        let bridge_payload = b"PRWC-disposable-typed-payload";
        let request = build_local_management_request_frame(id(152), bridge_payload)
            .expect("bounded management request builds");

        assert_eq!(request.header().request_id(), id(152));
        assert_eq!(request.header().kind(), LocalIpcMessageKind::Request);
        assert_eq!(&request.payload().as_bytes()[..2], &[0, 3]);

        let decoded =
            decode_local_management_request_frame(&request).expect("management envelope decodes");
        assert_eq!(decoded.request_id(), id(152));
        assert_eq!(decoded.bridge_payload(), bridge_payload);
    }

    #[test]
    fn empty_and_oversized_bridge_payloads_fail_before_frame_build() {
        assert_eq!(
            build_local_management_request_frame(id(153), &[]),
            Err(LocalManagementRequestBuildError::EmptyBridgePayload)
        );
        let oversized = vec![0; LOCAL_MANAGEMENT_BRIDGE_MAX_PAYLOAD_LENGTH + 1];
        assert_eq!(
            build_local_management_request_frame(id(154), &oversized),
            Err(LocalManagementRequestBuildError::BridgePayloadTooLarge)
        );
    }

    #[test]
    fn exact_maximum_bridge_payload_is_accepted() {
        let bridge_payload = vec![7; LOCAL_MANAGEMENT_BRIDGE_MAX_PAYLOAD_LENGTH];
        let request = build_local_management_request_frame(id(155), &bridge_payload)
            .expect("maximum PRWC payload remains within local control bound");
        let payload_length = usize::try_from(request.header().payload_length())
            .expect("local payload length fits usize");
        assert_eq!(payload_length, LOCAL_MANAGEMENT_REQUEST_MAX_PAYLOAD_LENGTH);
        let decoded =
            decode_local_management_request_frame(&request).expect("maximum envelope decodes");
        assert_eq!(decoded.bridge_payload(), bridge_payload.as_slice());
    }

    #[test]
    fn wrong_command_and_non_request_kind_fail_closed() {
        let wrong_command = frame(LocalIpcMessageKind::Request, 156, vec![0, 4, 0, 0, 0, 1, 9]);
        assert_eq!(
            decode_local_management_request_frame(&wrong_command),
            Err(LocalManagementRequestDecodeError::WrongCommand)
        );

        let response = frame(
            LocalIpcMessageKind::Response,
            157,
            vec![0, 3, 0, 0, 0, 1, 9],
        );
        assert_eq!(
            decode_local_management_request_frame(&response),
            Err(LocalManagementRequestDecodeError::NonRequestKind)
        );
    }

    #[test]
    fn truncated_trailing_and_zero_length_bodies_fail_closed() {
        let short = frame(LocalIpcMessageKind::Request, 158, vec![0, 3, 0, 0, 0]);
        assert_eq!(
            decode_local_management_request_frame(&short),
            Err(LocalManagementRequestDecodeError::PrefixTooShort)
        );

        let truncated = frame(LocalIpcMessageKind::Request, 159, vec![0, 3, 0, 0, 0, 2, 9]);
        assert_eq!(
            decode_local_management_request_frame(&truncated),
            Err(LocalManagementRequestDecodeError::LengthMismatch {
                declared: 2,
                actual: 1,
            })
        );

        let trailing = frame(
            LocalIpcMessageKind::Request,
            160,
            vec![0, 3, 0, 0, 0, 1, 9, 10],
        );
        assert_eq!(
            decode_local_management_request_frame(&trailing),
            Err(LocalManagementRequestDecodeError::LengthMismatch {
                declared: 1,
                actual: 2,
            })
        );

        let empty = frame(LocalIpcMessageKind::Request, 161, vec![0, 3, 0, 0, 0, 0]);
        assert_eq!(
            decode_local_management_request_frame(&empty),
            Err(LocalManagementRequestDecodeError::EmptyBridgePayload)
        );
    }

    #[test]
    fn declared_payload_above_retained_prwc_bound_fails_closed() {
        let declared = 65_537_u32.to_be_bytes();
        let mut payload = vec![0, 3];
        payload.extend_from_slice(&declared);
        let request = frame(LocalIpcMessageKind::Request, 162, payload);

        assert_eq!(
            decode_local_management_request_frame(&request),
            Err(LocalManagementRequestDecodeError::BridgePayloadTooLarge)
        );
    }
}
