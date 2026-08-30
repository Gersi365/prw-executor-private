//! Isolated Agent-owned requester/rendezvous one-shot target-request composition.
//!
//! C03e-ER materializes only the C03e-EP-selected one-shot composition after C03e-EQ provided the
//! bridge-owned requester-specific receive adapter. C03e-EV additionally materializes the separately
//! selected single-owner one-transaction post-authenticated ingress seam while leaving the ER method
//! itself unchanged and uninvoked. C03e-EX adds the isolated C03e-EW-selected repeated ingress loop
//! and executor-neutral cancellation-aware worker seam without integrating either into active runtime
//! ownership. C03e-EZ threads the C03e-EY-selected exact requester response-stream custody only
//! through the ET -> EV -> EX handoff while keeping the continuation uninvoked. C03e-GE adds only an
//! explicit fail-closed compatibility arm when current-Mesh candidate publication reaches this
//! still-dormant Agent transaction before any candidate handoff/execution semantics have been selected.
//! The existing capability loop and worker do not invoke these seams. None of these seams activates
//! requester/candidate authority/provider execution, candidate or requester response semantics, retry,
//! peer-close policy, dialing, readiness publication, or runtime activation.

use std::{
    future::{Future, poll_fn},
    task::Poll,
};

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::{
    CapabilityBridge, CapabilityDispatcher,
    authorized_request_dispatch::dispatch_authorized_request,
    post_auth_control_stream_ingress::{
        PostAuthControlStreamIngress, receive_post_auth_control_stream_ingress,
    },
    requester_rendezvous_target_request_io::receive_requester_rendezvous_target_request,
};

use super::super::{
    AuthenticatedRemoteSessionPostAuthIngressOutcome,
    AuthenticatedRemoteSessionPostAuthIngressTransactionError,
    RequesterRendezvousCorrelatedStartIntent, RequesterRendezvousOneShotTransactionError,
    RequesterRendezvousResponseStreamCustodyHandoff, SharedCurrentCapabilityAuthority,
    adapt_decoded_requester_rendezvous_target_device_id,
    adapt_post_auth_requester_rendezvous_target_intent,
};
use super::AuthenticatedRemoteSessionRuntimeOwner;

const REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_CODE: u32 = 6;
const REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_REASON: &[u8] =
    b"remote requester-aware session terminated";

impl AuthenticatedRemoteSessionRuntimeOwner {
    /// Processes exactly one post-authenticated control stream through the C03e-ET family ingress.
    ///
    /// This C03e-EV seam is the single Agent-owned acceptance point for one isolated transaction. It
    /// accepts exactly one stream from the retained authenticated peer and transfers that stream by
    /// value into `receive_post_auth_control_stream_ingress(...)`, which performs exactly one bounded
    /// PRWM frame read and typed family classification.
    ///
    /// Capability-family processing reuses the exact already-read frame, the existing bound-session
    /// registry/policy authorization path, the existing authorized dispatcher, and the exact same
    /// stream retained by the bridge custody envelope for response I/O. No request re-read or stream
    /// replacement occurs.
    ///
    /// Requester/rendezvous-family processing keeps the strict decoded request together with the exact
    /// same ET stream, reads the nominated logical target only from that request, composes it through
    /// the existing C03e-EO then C03e-EJ helpers, and returns one C03e-EZ response-stream custody
    /// handoff. The request's outer `request_id` remains correlation only inside the retained strict
    /// request. Processing stops before C03e-DV, registry/requester-policy/provider execution,
    /// candidate selection, requester response construction/write, or dialing.
    ///
    /// Candidate-publication ingress is recognized as a distinct family but C03e-GE selects no Agent
    /// candidate handoff or execution. This dormant seam therefore fails closed with the explicit
    /// `CandidatePublicationHandoffNotSelected` classification. It does not reinterpret the request as
    /// capability/requester traffic, call FY/GA/GC, write a candidate response, or accept another
    /// stream.
    ///
    /// The method performs one transaction only. It does not replace or invoke the existing
    /// capability loop/worker, does not invoke the isolated C03e-ER accept seam, and does not create a
    /// repeated combined loop, task, queue, retry, reconnect, fairness policy, backpressure policy,
    /// peer-close policy, readiness state, listener activation, deployment, or merge behavior.
    ///
    /// # Errors
    ///
    /// Preserves distinguishable failure classes for one authenticated stream accept, strict typed
    /// ingress, existing capability authorization/dispatch, same-stream capability response I/O, and
    /// the explicit unselected candidate-publication higher-owner handoff barrier. No failure is
    /// translated into fabricated success or a requester/candidate response frame.
    #[allow(
        dead_code,
        clippy::needless_pass_by_ref_mut,
        reason = "C03e-EV intentionally preserves the C03e-EU-selected exclusive mutable-owner transaction custody before separately gated combined-loop integration"
    )]
    pub(crate) async fn process_one_post_auth_control_stream_ingress<
        P: PolicyEvaluator + Send + Sync,
        D: CapabilityDispatcher + Send,
    >(
        &mut self,
        authority: &SharedCurrentCapabilityAuthority<P>,
        now_unix_seconds: u64,
        dispatcher: &mut D,
    ) -> Result<
        AuthenticatedRemoteSessionPostAuthIngressOutcome,
        AuthenticatedRemoteSessionPostAuthIngressTransactionError,
    > {
        let stream = self.peer.accept_control_stream().await?;
        let ingress = receive_post_auth_control_stream_ingress(stream).await?;

        match ingress {
            PostAuthControlStreamIngress::Capability(transaction) => {
                let bound_session = &self.capability_owner.bound_session;
                let authorized = authority
                    .with_current_authority(|registry, policy| {
                        let bridge = CapabilityBridge::new(registry, policy);
                        bound_session.authorize(
                            &bridge,
                            now_unix_seconds,
                            transaction.request_frame(),
                        )
                    })
                    .await?;
                let response = dispatch_authorized_request(&authorized, dispatcher)?;
                transaction.send_response_frame(&response).await?;
                Ok(AuthenticatedRemoteSessionPostAuthIngressOutcome::CapabilityProcessed)
            }
            PostAuthControlStreamIngress::RequesterRendezvous(transaction) => {
                let target_intent = adapt_decoded_requester_rendezvous_target_device_id(
                    transaction.request().target_device_id().clone(),
                );
                let start_intent =
                    adapt_post_auth_requester_rendezvous_target_intent(self, target_intent);
                Ok(
                    AuthenticatedRemoteSessionPostAuthIngressOutcome::RequesterRendezvous(
                        Box::new(RequesterRendezvousResponseStreamCustodyHandoff::new(
                            transaction,
                            start_intent,
                        )),
                    ),
                )
            }
            PostAuthControlStreamIngress::CandidatePublication(_transaction) => Err(
                AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected,
            ),
        }
    }

    /// Runs the isolated C03e-EW-selected repeated post-authenticated ingress loop.
    ///
    /// Exactly one C03e-EV transaction is in flight per iteration. Verifier time is sampled once
    /// immediately before each EV invocation. Capability success is the only outcome that reaches the
    /// next iteration. One requester/rendezvous result is a typed C03e-EZ handoff barrier retaining
    /// the strict request, exact response stream and session-derived start intent without accepting
    /// another stream. The first EV transaction failure — including the explicit GE unselected
    /// candidate-publication handoff barrier — terminates the loop unchanged.
    ///
    /// This method never calls `accept_control_stream()` directly and never invokes the historical
    /// capability-only `process_one_capability_request(...)` path. It therefore introduces no second
    /// authenticated acceptor, family-specific queue, speculative pre-accept, concurrent transaction,
    /// retry, reconnect, provider execution, requester response, peer close, dialing, readiness or
    /// runtime activation.
    ///
    /// # Errors
    ///
    /// Returns the first [`AuthenticatedRemoteSessionPostAuthIngressTransactionError`] emitted by the
    /// exact C03e-EV transaction seam. No retry, fallback, suppression or replacement is performed.
    #[allow(
        dead_code,
        reason = "C03e-EX materializes the isolated EW-selected repeated ingress loop before separately gated runtime integration"
    )]
    pub(crate) async fn run_repeated_post_auth_control_stream_ingress<
        P: PolicyEvaluator + Send + Sync,
        D: CapabilityDispatcher + Send,
        T: FnMut() -> u64 + Send,
    >(
        &mut self,
        authority: &SharedCurrentCapabilityAuthority<P>,
        mut verifier_time_unix_seconds: T,
        dispatcher: &mut D,
    ) -> Result<
        RequesterRendezvousResponseStreamCustodyHandoff,
        AuthenticatedRemoteSessionPostAuthIngressTransactionError,
    > {
        loop {
            let now_unix_seconds = verifier_time_unix_seconds();
            match self
                .process_one_post_auth_control_stream_ingress(
                    authority,
                    now_unix_seconds,
                    dispatcher,
                )
                .await?
            {
                AuthenticatedRemoteSessionPostAuthIngressOutcome::CapabilityProcessed => {}
                AuthenticatedRemoteSessionPostAuthIngressOutcome::RequesterRendezvous(handoff) => {
                    return Ok(*handoff);
                }
            }
        }
    }

    /// Runs one executor-neutral cancellation-aware C03e-EX worker body without spawning a task.
    ///
    /// The worker owns exactly one repeated C03e-EX loop future and one caller-supplied cancellation
    /// future. The loop is polled first on each wake so an already-ready requester handoff or EV
    /// failure retains its exact classification. Cancellation wins only while the repeated loop is
    /// pending. The in-flight loop future is dropped when the lexical race block exits before the
    /// cancellation result leaves this method, releasing the exclusive mutable owner borrow first.
    ///
    /// Return classes are intentionally minimal and distinguishable:
    ///
    /// - `Ok(Some(handoff))` is the requester/rendezvous response-stream custody handoff barrier;
    /// - `Ok(None)` is caller-owned cancellation;
    /// - `Err(error)` is the first unchanged C03e-EV transaction failure.
    ///
    /// Cancellation performs no whole-peer close in this checkpoint. The existing capability-only
    /// code-4 diagnostic is not widened to mixed-family traffic, and no replacement close code is
    /// invented. No task, channel, queue, retry, reconnect, provider action, requester response,
    /// dialing, readiness state, listener activation or deployment is created.
    ///
    /// # Errors
    ///
    /// Returns the exact first repeated-loop C03e-EV transaction error without reclassification.
    #[allow(
        dead_code,
        reason = "C03e-EX materializes the isolated EW-selected executor-neutral worker before separately gated runtime integration"
    )]
    pub(crate) async fn run_repeated_post_auth_control_stream_ingress_worker<
        P: PolicyEvaluator + Send + Sync,
        D: CapabilityDispatcher + Send,
        T: FnMut() -> u64 + Send,
        C: Future<Output = ()> + Send,
    >(
        &mut self,
        authority: &SharedCurrentCapabilityAuthority<P>,
        verifier_time_unix_seconds: T,
        dispatcher: &mut D,
        cancellation: C,
    ) -> Result<
        Option<RequesterRendezvousResponseStreamCustodyHandoff>,
        AuthenticatedRemoteSessionPostAuthIngressTransactionError,
    > {
        {
            let mut ingress_loop = Box::pin(self.run_repeated_post_auth_control_stream_ingress(
                authority,
                verifier_time_unix_seconds,
                dispatcher,
            ));
            let mut cancellation = Box::pin(cancellation);

            poll_fn(|context| {
                match ingress_loop.as_mut().poll(context) {
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
        }
    }

    /// Receives and composes exactly one requester/rendezvous target request on one new stream.
    ///
    /// The retained authenticated peer supplies the only stream acceptance authority. The bridge
    /// receives and strictly decodes exactly one bounded PRWM/PRWZ target request. Outer
    /// `request_id` is copied only into the separate correlation tuple element; it is never used as
    /// requester, target, session, transport, registry, policy, or provider identity.
    ///
    /// The decoded logical target is transferred by value through the existing C03e-EO adaptation
    /// and then through the existing C03e-EJ authenticated-session adaptation. Successful return
    /// proves only correlation preservation plus construction of the non-authoritative
    /// `RequesterRendezvousStartIntent`.
    ///
    /// This method does not invoke C03e-DV, current registry/requester policy/provider execution,
    /// candidate selection, response construction or stream write. It does not retry, loop, close
    /// the peer, or integrate with the existing capability worker. The method remains uninvoked by
    /// runtime source until deterministic control-stream demultiplexing is separately selected.
    ///
    /// # Errors
    ///
    /// Preserves authenticated-peer stream-accept failure as
    /// [`RequesterRendezvousOneShotTransactionError::Accept`] and the existing C03e-EQ
    /// receive/decode failure as [`RequesterRendezvousOneShotTransactionError::Wire`]. No failure
    /// is translated into a response frame or rendezvous authority result.
    #[allow(
        dead_code,
        reason = "C03e-ER materializes the isolated one-shot transaction before separately gated deterministic stream demultiplexing/runtime invocation"
    )]
    pub(crate) async fn receive_requester_rendezvous_start_intent_once(
        &self,
    ) -> Result<RequesterRendezvousCorrelatedStartIntent, RequesterRendezvousOneShotTransactionError>
    {
        let mut stream = self.peer.accept_control_stream().await?;
        let request = receive_requester_rendezvous_target_request(&mut stream).await?;
        let request_id = request.request_id();
        let target_intent =
            adapt_decoded_requester_rendezvous_target_device_id(request.into_target_device_id());
        let start_intent = adapt_post_auth_requester_rendezvous_target_intent(self, target_intent);
        Ok((request_id, start_intent))
    }

    /// Consumes one recovered authenticated owner after requester-aware FL or join failure.
    ///
    /// This C03e-FW seam performs terminal peer disposition only. It closes the exact retained peer
    /// once with the fixed non-secret requester-aware code-6 diagnostic. It performs no requester
    /// record cleanup, session deletion, retry/reconnect, peer reuse, worker restart, candidate or
    /// reachability work, target dialing, runtime activation, deployment, or merge.
    pub(in super::super) fn close_for_requester_aware_terminal_failure(self) {
        self.peer.close(
            REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_CODE,
            REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_REASON,
        );
    }
}

#[cfg(test)]
mod fw_requester_aware_terminal_close_tests {
    use super::{
        AuthenticatedRemoteSessionRuntimeOwner,
        REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_CODE,
        REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_REASON,
    };

    fn assert_consuming_close_signature(close: fn(AuthenticatedRemoteSessionRuntimeOwner)) {
        let _ = close;
    }

    #[test]
    fn requester_aware_terminal_failure_close_is_consuming() {
        assert_consuming_close_signature(
            AuthenticatedRemoteSessionRuntimeOwner::close_for_requester_aware_terminal_failure,
        );
    }

    #[test]
    fn requester_aware_terminal_failure_close_uses_fixed_code_six_diagnostic() {
        assert_eq!(REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_CODE, 6);
        assert_eq!(
            REMOTE_REQUESTER_AWARE_SESSION_TERMINATION_CLOSE_REASON,
            b"remote requester-aware session terminated"
        );
    }
}
