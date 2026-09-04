//! Agent-owned retained-custody requester/rendezvous DR continuation, terminal response composition,
//! isolated post-terminal serial lifecycle, and cancellation-aware higher-owner worker.
//!
//! C03e-FB materializes the C03e-FA-selected continuation from one exact C03e-EZ response-stream
//! custody handoff through the existing shared-current authority read and existing C03e-DR
//! DI -> DP -> DK -> DN composition. The exact bridge requester transaction survives both DR success
//! and DR failure. C03e-FH adds only the C03e-FG-selected Agent-owned terminal composition from that
//! exact retained DR result through the existing C03e-FD pure acknowledgement framing boundary into
//! the existing C03e-FF consuming same-stream send surface. C03e-FJ adds only the C03e-FI-selected
//! isolated serial lifecycle that resumes the existing EV/EX mixed-family ingress after FH success
//! and fail-stops on existing ingress or requester-response failure. C03e-FL adds only the
//! C03e-FK-selected cancellation-aware higher-owner worker around those exact serial transaction
//! boundaries. This module still performs no automatic peer close, candidate/reachability selection,
//! dialing, runtime activation, deployment or merge.

use std::{
    fmt,
    future::{Future, poll_fn},
    task::Poll,
};

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::{
    CapabilityDispatcher,
    post_auth_control_stream_ingress::{
        PostAuthRequesterRendezvousTransaction, RequesterRendezvousDrAcknowledgementResponseIoError,
    },
    requester_rendezvous_dr_acknowledgement_wire::{
        RequesterRendezvousDrAcknowledgementWireError,
        encode_requester_rendezvous_dr_result_for_transaction,
    },
};

use super::{
    AuthenticatedRemoteSessionPostAuthIngressTransactionError,
    AuthenticatedRemoteSessionRuntimeOwner, RequesterRendezvousResponseStreamCustodyHandoff,
    SharedCurrentCapabilityAuthority, SharedRequesterRendezvousAuthority,
};
use crate::candidate_publication_requester_rendezvous_start_intent::{
    composition::RequesterRendezvousStartCompositionError,
    policy_source::RequesterRendezvousStartPolicySource,
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

/// Failure while running the isolated C03e-FI-selected serial post-terminal requester lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum RequesterRendezvousPostTerminalResponseSerialLifecycleError {
    /// Existing EV/EX mixed-family ingress failed before a requester response transaction completed.
    Ingress(AuthenticatedRemoteSessionPostAuthIngressTransactionError),
    /// Existing FH requester terminal response composition failed after one requester handoff.
    RequesterResponse(RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError),
}

impl fmt::Display for RequesterRendezvousPostTerminalResponseSerialLifecycleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Ingress(_) => "post-authenticated mixed-family ingress failed",
            Self::RequesterResponse(_) => "requester rendezvous terminal response failed",
        })
    }
}

impl std::error::Error for RequesterRendezvousPostTerminalResponseSerialLifecycleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ingress(error) => Some(error),
            Self::RequesterResponse(error) => Some(error),
        }
    }
}

impl From<AuthenticatedRemoteSessionPostAuthIngressTransactionError>
    for RequesterRendezvousPostTerminalResponseSerialLifecycleError
{
    fn from(error: AuthenticatedRemoteSessionPostAuthIngressTransactionError) -> Self {
        Self::Ingress(error)
    }
}

impl From<RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>
    for RequesterRendezvousPostTerminalResponseSerialLifecycleError
{
    fn from(error: RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError) -> Self {
        Self::RequesterResponse(error)
    }
}

/// Terminal higher-owner result of the C03e-FK-selected cancellation-aware serial lifecycle worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop {
    /// Caller-owned cancellation won at one selected cancellation-safe lifecycle boundary.
    Cancelled,
    /// Existing FJ lifecycle semantics reached one exact typed ingress or requester-response failure.
    Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError),
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
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    handoff: RequesterRendezvousResponseStreamCustodyHandoff,
) -> RequesterRendezvousRetainedCustodyDrContinuation {
    let RequesterRendezvousResponseStreamCustodyHandoff {
        requester_transaction,
        start_intent,
    } = handoff;

    let dr_result = requester_rendezvous_authority
        .validate_authorize_and_register_requester_rendezvous_start(
            authority,
            policy_source,
            start_intent,
        )
        .await;

    RequesterRendezvousRetainedCustodyDrContinuation {
        requester_transaction,
        dr_result,
    }
}

/// Runs the isolated C03e-FI-selected requester-aware serial post-authenticated lifecycle.
///
/// The existing EV/EX mixed-family ingress remains the only stream-accept/read loop. It runs until
/// either the first ingress failure or one requester/rendezvous handoff. A requester handoff is
/// consumed exactly once by existing FB DR continuation and then exactly once by existing FH terminal
/// acknowledgement composition. Only FH success reaches the next EV/EX cycle. A successfully sent
/// generic rejected acknowledgement is therefore transaction-complete and also resumes serial
/// ingress. No EV/EX cycle overlaps requester DR/response custody.
///
/// This seam creates no second acceptor, task, channel, queue, retry, resend, replacement stream,
/// duplicate acknowledgement, automatic peer close, cancellation race, candidate/reachability
/// continuation, target dial, runtime/listener activation, deployment or merge behavior.
///
/// # Errors
///
/// Returns [`RequesterRendezvousPostTerminalResponseSerialLifecycleError::Ingress`] for the exact
/// first existing EV/EX ingress failure. Returns
/// [`RequesterRendezvousPostTerminalResponseSerialLifecycleError::RequesterResponse`] for the exact
/// existing FH `Frame` or `ResponseIo` failure. Either failure stops this serial lifecycle before
/// another EV/EX ingress cycle begins.
pub(super) async fn run_requester_rendezvous_post_terminal_response_serial_lifecycle<
    P: PolicyEvaluator + Send + Sync,
    D: CapabilityDispatcher + Send,
    T: FnMut() -> u64 + Send,
    S: RequesterRendezvousStartPolicySource + Sync + ?Sized,
>(
    session_owner: &mut AuthenticatedRemoteSessionRuntimeOwner,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    mut verifier_time_unix_seconds: T,
    dispatcher: &mut D,
) -> Result<(), RequesterRendezvousPostTerminalResponseSerialLifecycleError> {
    loop {
        let handoff = session_owner
            .run_repeated_post_auth_control_stream_ingress(
                authority,
                &mut verifier_time_unix_seconds,
                dispatcher,
            )
            .await?;

        let continuation = continue_requester_rendezvous_retained_custody_through_dr(
            authority,
            policy_source,
            requester_rendezvous_authority,
            handoff,
        )
        .await;

        complete_requester_rendezvous_terminal_dr_acknowledgement_response(continuation).await?;
    }
}

/// Runs one cancellation-aware higher-owner worker over the existing requester-aware serial lifecycle.
///
/// Before requester handoff, the exact EX repeated-ingress future is polled first and caller
/// cancellation is polled second. Therefore an already-ready requester handoff or ingress failure
/// preserves the existing EX classification; cancellation wins only while ingress remains pending.
///
/// Once one requester handoff exists, cancellation is deliberately not polled while exact FB DR
/// continuation and exact FH terminal acknowledgement response composition run to one typed terminal
/// result. This prevents caller cancellation from becoming a new response-abandonment path after
/// requester authorization/registration may already have committed. Existing bounded transport
/// timeouts remain authoritative inside FH.
///
/// After FH success, cancellation is polled once before another EX cycle can begin. If cancellation
/// became ready during FB/FH, this worker returns `Cancelled` before the next verifier-time sample,
/// stream accept or frame receive. Otherwise the next serial EX cycle begins normally.
///
/// No path closes the authenticated peer, retries ingress/DR/response work, allocates a replacement
/// stream, duplicates an acknowledgement, widens capability close codes, starts a task/channel/queue,
/// selects candidate/reachability state, dials target traffic, activates a runtime/listener, deploys,
/// or merges anything.
pub(super) async fn run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker<
    P: PolicyEvaluator + Send + Sync,
    D: CapabilityDispatcher + Send,
    T: FnMut() -> u64 + Send,
    S: RequesterRendezvousStartPolicySource + Sync + ?Sized,
    C: Future<Output = ()> + Send,
>(
    session_owner: &mut AuthenticatedRemoteSessionRuntimeOwner,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    mut verifier_time_unix_seconds: T,
    dispatcher: &mut D,
    cancellation: C,
) -> RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop {
    let mut cancellation = Box::pin(cancellation);

    loop {
        let ingress_result: Result<
            Option<RequesterRendezvousResponseStreamCustodyHandoff>,
            AuthenticatedRemoteSessionPostAuthIngressTransactionError,
        > = {
            let mut ingress =
                Box::pin(session_owner.run_repeated_post_auth_control_stream_ingress(
                    authority,
                    &mut verifier_time_unix_seconds,
                    dispatcher,
                ));

            poll_fn(|context| {
                match ingress.as_mut().poll(context) {
                    Poll::Ready(Ok(handoff)) => return Poll::Ready(Ok(Some(handoff))),
                    Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                    Poll::Pending => {}
                }

                match cancellation.as_mut().poll(context) {
                    Poll::Ready(()) => Poll::Ready(Ok(None)),
                    Poll::Pending => Poll::Pending,
                }
            })
            .await
        };

        let handoff = match ingress_result {
            Ok(Some(handoff)) => handoff,
            Ok(None) => {
                return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled;
            }
            Err(error) => {
                return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(
                    error.into(),
                );
            }
        };

        let continuation = continue_requester_rendezvous_retained_custody_through_dr(
            authority,
            policy_source,
            requester_rendezvous_authority,
            handoff,
        )
        .await;

        if let Err(error) =
            complete_requester_rendezvous_terminal_dr_acknowledgement_response(continuation).await
        {
            return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(
                error.into(),
            );
        }

        let cancellation_ready = poll_fn(|context| {
            Poll::Ready(matches!(
                cancellation.as_mut().poll(context),
                Poll::Ready(())
            ))
        })
        .await;

        if cancellation_ready {
            return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled;
        }
    }
}

/// Runs the KR-selected requester-aware serial lifecycle with distinct durable-capability and
/// requester-DR authority lanes.
///
/// Before requester handoff, the existing KQ durable cancellation worker owns the exact
/// ingress/cancellation race. The durable capability authority is passed only to that worker, while
/// the existing shared-current requester authority remains reserved for DR continuation after one
/// requester handoff. One pinned caller cancellation future is retained across every serial cycle;
/// each KQ invocation receives only a temporary polling adapter over that retained future.
///
/// Cancellation is deliberately not polled during requester DR or terminal acknowledgement response
/// composition. After one successful terminal response it is checked exactly once before another KQ
/// cycle can sample verifier time or accept another stream. No task, channel, queue, peer close,
/// requester retry, candidate continuation, runtime activation, deployment or merge behavior is
/// introduced here.
#[allow(
    dead_code,
    reason = "C03e-KS materializes the KR-selected dormant dual-authority FI worker before separately gated FQ/FU ownership propagation"
)]
#[expect(
    clippy::too_many_arguments,
    reason = "KR keeps durable capability ingress authority and requester DR authority as explicit distinct inputs"
)]
pub(super) async fn run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability<
    P: PolicyEvaluator + Send + Sync,
    D: CapabilityDispatcher + Send,
    T: FnMut() -> u64 + Send,
    S: RequesterRendezvousStartPolicySource + Sync + ?Sized,
    C: Future<Output = ()> + Send,
>(
    session_owner: &mut AuthenticatedRemoteSessionRuntimeOwner,
    capability_authority: &crate::production_durable_registry_runtime_custody::ProductionDurableCapabilityAuthority,
    requester_dr_authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    mut verifier_time_unix_seconds: T,
    dispatcher: &mut D,
    cancellation: C,
) -> RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop {
    let mut cancellation = Box::pin(cancellation);

    loop {
        let cancellation_adapter = poll_fn(|context| cancellation.as_mut().poll(context));
        let ingress_result = session_owner
            .run_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability(
                capability_authority,
                &mut verifier_time_unix_seconds,
                dispatcher,
                cancellation_adapter,
            )
            .await;

        let handoff = match ingress_result {
            Ok(Some(handoff)) => handoff,
            Ok(None) => {
                return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled;
            }
            Err(error) => {
                return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(
                    error.into(),
                );
            }
        };

        let continuation = continue_requester_rendezvous_retained_custody_through_dr(
            requester_dr_authority,
            policy_source,
            requester_rendezvous_authority,
            handoff,
        )
        .await;

        if let Err(error) =
            complete_requester_rendezvous_terminal_dr_acknowledgement_response(continuation).await
        {
            return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(
                error.into(),
            );
        }

        let cancellation_ready = poll_fn(|context| {
            Poll::Ready(matches!(
                cancellation.as_mut().poll(context),
                Poll::Ready(())
            ))
        })
        .await;

        if cancellation_ready {
            return RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled;
        }
    }
}

#[cfg(test)]
mod tests {
    use prw_remote_bridge::{
        post_auth_control_stream_ingress::RequesterRendezvousDrAcknowledgementResponseIoError,
        requester_rendezvous_dr_acknowledgement_wire::RequesterRendezvousDrAcknowledgementWireError,
    };

    use super::{
        RequesterRendezvousPostTerminalResponseSerialLifecycleError,
        RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
        RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
        complete_requester_rendezvous_terminal_dr_acknowledgement_response,
    };
    use crate::remote_session_capability_runtime::AuthenticatedRemoteSessionPostAuthIngressTransactionError;

    fn assert_frame_error_conversion(
        conversion: fn(
            RequesterRendezvousDrAcknowledgementWireError,
        )
            -> RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
    ) {
        let _ = conversion;
    }

    fn assert_response_io_error_conversion(
        conversion: fn(
            RequesterRendezvousDrAcknowledgementResponseIoError,
        )
            -> RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
    ) {
        let _ = conversion;
    }

    fn assert_ingress_lifecycle_error_conversion(
        conversion: fn(
            AuthenticatedRemoteSessionPostAuthIngressTransactionError,
        ) -> RequesterRendezvousPostTerminalResponseSerialLifecycleError,
    ) {
        let _ = conversion;
    }

    fn assert_requester_response_lifecycle_error_conversion(
        conversion: fn(
            RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
        ) -> RequesterRendezvousPostTerminalResponseSerialLifecycleError,
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

    #[test]
    fn serial_lifecycle_error_family_preserves_ingress_and_requester_response_categories() {
        assert_ingress_lifecycle_error_conversion(
            RequesterRendezvousPostTerminalResponseSerialLifecycleError::from,
        );
        assert_requester_response_lifecycle_error_conversion(
            RequesterRendezvousPostTerminalResponseSerialLifecycleError::from,
        );
    }

    #[test]
    fn cancellation_aware_worker_stop_exposes_local_cancelled_class() {
        assert_eq!(
            RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled,
            RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled
        );
    }
}
