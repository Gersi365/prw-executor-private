//! Agent-owned retained-custody requester/rendezvous DR continuation and terminal response composition.
//!
//! C03e-FB materializes the C03e-FA-selected continuation from one exact C03e-EZ response-stream
//! custody handoff through the existing shared-current authority read and existing C03e-DR
//! DI -> DP -> DK -> DN composition. The exact bridge requester transaction survives both DR success
//! and DR failure. C03e-FH adds only the C03e-FG-selected Agent-owned terminal composition from that
//! exact retained DR result through the existing C03e-FD pure acknowledgement framing boundary into
//! the existing C03e-FF consuming same-stream send surface. This module still performs no second
//! read, loop resume, peer-close policy, candidate/reachability selection, dialing, runtime
//! activation, deployment or merge.

use std::fmt;

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::{
    post_auth_control_stream_ingress::{
        PostAuthRequesterRendezvousTransaction,
        RequesterRendezvousDrAcknowledgementResponseIoError,
    },
    requester_rendezvous_dr_acknowledgement_wire::{
        RequesterRendezvousDrAcknowledgementWireError,
        encode_requester_rendezvous_dr_result_for_transaction,
    },
};

use super::{RequesterRendezvousResponseStreamCustodyHandoff, SharedCurrentCapabilityAuthority};
use crate::{
    candidate_publication_requester_rendezvous_runtime::CandidatePublicationRequesterRendezvousRuntimeOwner,
    candidate_publication_requester_rendezvous_start_intent::{
        composition::{
            RequesterRendezvousStartCompositionError,
            validate_authorize_and_register_requester_rendezvous_start,
        },
        policy_source::RequesterRendezvousStartPolicySource,
    },
};

/// Terminal C03e-FB custody after exactly one existing DR authority composition.
///
/// The bridge requester transaction is retained by value regardless of whether DR returned `Ok(())`
/// or one exact [`RequesterRendezvousStartCompositionError`]. Possession of this value does not
/// imply endpoint selection, reachability, rendezvous completion, response delivery or transport
/// establishment.
pub(super) struct RequesterRendezvousRetainedCustodyDrContinuation {
    requester_transaction: PostAuthRequesterRendezvousTransaction,
    dr_result: Result<(), RequesterRendezvousStartCompositionError>,
}

impl RequesterRendezvousRetainedCustodyDrContinuation {
    /// Borrows the exact bridge requester transaction retained across DR.
    #[must_use]
    pub(super) const fn requester_transaction(&self) -> &PostAuthRequesterRendezvousTransaction {
        &self.requester_transaction
    }

    /// Borrows the exact terminal DR result without translating or flattening its failure class.
    pub(super) const fn dr_result(&self) -> &Result<(), RequesterRendezvousStartCompositionError> {
        &self.dr_result
    }

    /// Transfers the exact requester transaction and exact terminal DR result by value.
    ///
    /// This is custody transfer only and performs no stream I/O or response construction.
    pub(super) fn into_parts(
        self,
    ) -> (
        PostAuthRequesterRendezvousTransaction,
        Result<(), RequesterRendezvousStartCompositionError>,
    ) {
        (self.requester_transaction, self.dr_result)
    }
}

/// Failure while completing one exact retained requester/rendezvous DR acknowledgement response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError {
    /// Existing C03e-FD pure acknowledgement framing failed before any response write attempt.
    Frame(RequesterRendezvousDrAcknowledgementWireError),
    /// Existing C03e-FF exact same-stream response write or send-direction finish failed.
    ResponseIo(RequesterRendezvousDrAcknowledgementResponseIoError),
}

impl fmt::Display for RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Frame(_) => "requester rendezvous DR acknowledgement framing failed",
            Self::ResponseIo(_) => "requester rendezvous DR acknowledgement response I/O failed",
        })
    }
}

impl std::error::Error for RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Frame(error) => Some(error),
            Self::ResponseIo(error) => Some(error),
        }
    }
}

impl From<RequesterRendezvousDrAcknowledgementWireError>
    for RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError
{
    fn from(error: RequesterRendezvousDrAcknowledgementWireError) -> Self {
        Self::Frame(error)
    }
}

impl From<RequesterRendezvousDrAcknowledgementResponseIoError>
    for RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError
{
    fn from(error: RequesterRendezvousDrAcknowledgementResponseIoError) -> Self {
        Self::ResponseIo(error)
    }
}

/// Consumes one exact retained DR continuation and completes exactly one terminal acknowledgement
/// response through the existing FD framing and FF same-stream send boundaries.
///
/// The continuation is borrowed only long enough for FD to project the exact already-completed DR
/// result and echo the exact original PRWM request correlation. A semantic DR `Err(_)` therefore
/// remains one valid generic rejected acknowledgement rather than becoming a composition failure.
/// After successful framing, the continuation is consumed by value and exact requester transaction
/// custody is transferred exactly once into FF. No result path returns retry-capable continuation,
/// transaction or raw-stream custody.
///
/// # Errors
///
/// Returns [`RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError::Frame`] if the
/// existing FD framing boundary fails. No response write is attempted on that path and the consumed
/// continuation is not returned for retry.
///
/// Returns [`RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError::ResponseIo`] if
/// the existing FF same-stream write/finish fails. FF consumes exact requester transaction custody;
/// no retry, resend, replacement stream or duplicate acknowledgement is attempted.
pub(super) async fn complete_requester_rendezvous_terminal_dr_acknowledgement_response(
    continuation: RequesterRendezvousRetainedCustodyDrContinuation,
) -> Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError> {
    let acknowledgement_frame = encode_requester_rendezvous_dr_result_for_transaction(
        continuation.requester_transaction(),
        continuation.dr_result(),
    )?;

    let (requester_transaction, _) = continuation.into_parts();
    requester_transaction
        .send_dr_acknowledgement_frame(&acknowledgement_frame)
        .await?;

    Ok(())
}

/// Consumes one exact EZ handoff and runs exactly one existing DR composition under current authority.
///
/// The exact EZ `RequesterRendezvousStartIntent` is consumed directly; this seam does not extract a
/// target-only `DeviceId` or call the C03e-DV convenience method that would reconstruct a second
/// start intent. The principal-agnostic capability policy yielded by current authority is ignored;
/// the supplied requester-aware policy source remains the sole DP requester policy source.
///
/// The bridge requester transaction remains outside the authority closure and therefore survives
/// unchanged on both DR success and failure. The current-authority read guard spans only the
/// synchronous DR call and is released before this function returns. No response I/O occurs here.
pub(super) async fn continue_requester_rendezvous_retained_custody_through_dr<
    P: PolicyEvaluator + Send + Sync,
    S: RequesterRendezvousStartPolicySource + Sync + ?Sized,
>(
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    handoff: RequesterRendezvousResponseStreamCustodyHandoff,
) -> RequesterRendezvousRetainedCustodyDrContinuation {
    let RequesterRendezvousResponseStreamCustodyHandoff {
        requester_transaction,
        start_intent,
    } = handoff;

    let dr_result = authority
        .with_current_authority(|registry, _current_capability_policy| {
            validate_authorize_and_register_requester_rendezvous_start(
                registry,
                policy_source,
                runtime_owner,
                start_intent,
            )
        })
        .await;

    RequesterRendezvousRetainedCustodyDrContinuation {
        requester_transaction,
        dr_result,
    }
}

#[cfg(test)]
mod tests {
    use prw_remote_bridge::{
        post_auth_control_stream_ingress::RequesterRendezvousDrAcknowledgementResponseIoError,
        requester_rendezvous_dr_acknowledgement_wire::RequesterRendezvousDrAcknowledgementWireError,
    };

    use super::{
        RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
        complete_requester_rendezvous_terminal_dr_acknowledgement_response,
    };

    fn assert_frame_error_conversion(
        conversion: fn(
            RequesterRendezvousDrAcknowledgementWireError,
        ) -> RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
    ) {
        let _ = conversion;
    }

    fn assert_response_io_error_conversion(
        conversion: fn(
            RequesterRendezvousDrAcknowledgementResponseIoError,
        ) -> RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
    ) {
        let _ = conversion;
    }

    #[test]
    fn terminal_dr_acknowledgement_response_composition_surface_is_materialized() {
        let _ = complete_requester_rendezvous_terminal_dr_acknowledgement_response;
    }

    #[test]
    fn terminal_response_error_family_preserves_exact_two_lower_categories() {
        assert_frame_error_conversion(
            RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError::from,
        );
        assert_response_io_error_conversion(
            RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError::from,
        );
    }
}
