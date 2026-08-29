//! Bridge-owned post-authenticated single-read control-stream family ingress.
//!
//! C03e-ET materializes only the C03e-ES-selected one-read family-custody boundary on one already
//! accepted authenticated control stream. The bridge reads exactly one bounded PRWM frame, routes an
//! exact `PRWZ` payload prefix through the existing strict requester/rendezvous decoder, and preserves
//! every other frame plus the same stream as capability transaction custody. This module does not
//! accept another stream, authenticate a session, authorize a capability, execute requester policy or
//! provider logic, select candidates, define requester/rendezvous response semantics, retry I/O,
//! close a peer, dial traffic, activate a loop/listener, or deploy anything.

use std::fmt;

use prw_remote_transport::{
    ControlFrame,
    runtime::{MeshControlStream, MeshQuicRuntimeError},
};

use crate::{
    capability_request_wire::{CapabilityRequestWireError, send_capability_response_frame},
    requester_rendezvous_target_request_wire::{
        REQUESTER_RENDEZVOUS_TARGET_REQUEST_WIRE_MAGIC, RequesterRendezvousTargetWireError,
        RequesterRendezvousTargetWireRequest, decode_requester_rendezvous_target_request_frame,
    },
};

/// One already-read capability transaction that retains same-stream response custody.
pub struct PostAuthCapabilityTransaction {
    request_frame: ControlFrame,
    stream: MeshControlStream,
}

impl PostAuthCapabilityTransaction {
    /// Borrows the exact already-received bounded PRWM frame for the existing capability bridge.
    #[must_use]
    pub const fn request_frame(&self) -> &ControlFrame {
        &self.request_frame
    }

    /// Consumes this custody envelope and sends exactly one already-constructed capability response
    /// through the existing bridge-owned response adapter on the same control stream.
    ///
    /// # Errors
    ///
    /// Preserves the existing bounded capability response I/O failure classification.
    pub async fn send_response_frame(
        self,
        response_frame: &ControlFrame,
    ) -> Result<(), CapabilityRequestWireError> {
        let Self { mut stream, .. } = self;
        send_capability_response_frame(&mut stream, response_frame).await
    }
}

/// Typed result of one bridge-owned post-authenticated control-stream read and family selection.
pub enum PostAuthControlStreamIngress {
    /// Exact `PRWZ` prefix was observed and the existing strict requester/rendezvous decoder passed.
    RequesterRendezvous(RequesterRendezvousTargetWireRequest),
    /// Any non-`PRWZ` frame remains on the legacy capability path with same-stream custody retained.
    Capability(PostAuthCapabilityTransaction),
}

/// Failure while materializing one post-authenticated single-read family ingress result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PostAuthControlStreamIngressError {
    /// Existing bounded PRWM stream/frame receive failed.
    Runtime(MeshQuicRuntimeError),
    /// Exact `PRWZ` prefix selected requester/rendezvous, but strict PRWZ semantics failed.
    RequesterRendezvousWire(RequesterRendezvousTargetWireError),
}

impl fmt::Display for PostAuthControlStreamIngressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Runtime(_) => "post-authenticated control-stream receive failed",
            Self::RequesterRendezvousWire(_) => {
                "post-authenticated requester/rendezvous wire decode failed"
            }
        })
    }
}

impl std::error::Error for PostAuthControlStreamIngressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Runtime(error) => Some(error),
            Self::RequesterRendezvousWire(error) => Some(error),
        }
    }
}

impl From<MeshQuicRuntimeError> for PostAuthControlStreamIngressError {
    fn from(error: MeshQuicRuntimeError) -> Self {
        Self::Runtime(error)
    }
}

impl From<RequesterRendezvousTargetWireError> for PostAuthControlStreamIngressError {
    fn from(error: RequesterRendezvousTargetWireError) -> Self {
        Self::RequesterRendezvousWire(error)
    }
}

/// Reads exactly one bounded PRWM frame from one already-accepted authenticated control stream and
/// returns one typed family-custody result.
///
/// Routing is deliberately asymmetric and legacy-preserving: an exact first-four-byte `PRWZ`
/// payload prefix selects the existing strict requester/rendezvous codec; every other bounded PRWM
/// frame, including short, unknown, malformed or `PRWC` payloads, remains on the existing capability
/// path. Prefix recognition is only family selection and grants no authentication, authorization,
/// target eligibility, provider, reachability, candidate or rendezvous-success authority.
///
/// The stream is consumed by value so lower transport custody cannot remain simultaneously with the
/// caller. A capability outcome retains the exact already-read frame and that exact stream for the
/// existing same-stream response. A requester/rendezvous outcome returns only the strict decoded
/// request; no requester response semantics are introduced here.
///
/// # Errors
///
/// Preserves one bounded stream receive failure as [`PostAuthControlStreamIngressError::Runtime`].
/// If and only if the exact `PRWZ` prefix selects requester/rendezvous, strict decoder failure is
/// preserved as [`PostAuthControlStreamIngressError::RequesterRendezvousWire`]. No fallback decode,
/// retry, second read, response write or peer close is performed.
pub async fn receive_post_auth_control_stream_ingress(
    mut stream: MeshControlStream,
) -> Result<PostAuthControlStreamIngress, PostAuthControlStreamIngressError> {
    let frame = stream.receive_frame().await?;
    if is_requester_rendezvous_family(&frame) {
        let request = decode_requester_rendezvous_target_request_frame(&frame)?;
        return Ok(PostAuthControlStreamIngress::RequesterRendezvous(request));
    }

    Ok(PostAuthControlStreamIngress::Capability(
        PostAuthCapabilityTransaction {
            request_frame: frame,
            stream,
        },
    ))
}

fn is_requester_rendezvous_family(frame: &ControlFrame) -> bool {
    frame
        .payload()
        .starts_with(REQUESTER_RENDEZVOUS_TARGET_REQUEST_WIRE_MAGIC.as_slice())
}

#[cfg(test)]
mod tests {
    use prw_remote_transport::{ControlFrame, ControlMessageKind};

    use super::{is_requester_rendezvous_family, receive_post_auth_control_stream_ingress};
    use crate::requester_rendezvous_target_request_wire::{
        RequesterRendezvousTargetWireError, decode_requester_rendezvous_target_request_frame,
    };

    fn frame(kind: ControlMessageKind, payload: &[u8]) -> ControlFrame {
        ControlFrame::new(kind, 17, payload.to_vec()).expect("bounded test frame must be valid")
    }

    #[test]
    fn ingress_surface_exposes_only_one_stream_consuming_receive_operation() {
        let _ = receive_post_auth_control_stream_ingress;
    }

    #[test]
    fn classifier_diverts_only_exact_prwz_prefix() {
        assert!(is_requester_rendezvous_family(&frame(
            ControlMessageKind::Request,
            b"PRWZ"
        )));
        assert!(!is_requester_rendezvous_family(&frame(
            ControlMessageKind::Request,
            b"PRWC"
        )));
        assert!(!is_requester_rendezvous_family(&frame(
            ControlMessageKind::Request,
            b"PRW"
        )));
        assert!(!is_requester_rendezvous_family(&frame(
            ControlMessageKind::Request,
            b"ABCD"
        )));
    }

    #[test]
    fn family_recognition_does_not_replace_strict_prwz_validation() {
        let candidate = frame(ControlMessageKind::Event, b"PRWZ");
        assert!(is_requester_rendezvous_family(&candidate));
        assert_eq!(
            decode_requester_rendezvous_target_request_frame(&candidate),
            Err(RequesterRendezvousTargetWireError::InvalidOuterKind)
        );
    }
}
