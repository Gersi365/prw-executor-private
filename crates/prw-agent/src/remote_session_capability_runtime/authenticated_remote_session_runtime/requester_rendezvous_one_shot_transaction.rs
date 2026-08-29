//! Isolated Agent-owned requester/rendezvous one-shot target-request composition.
//!
//! C03e-ER materializes only the C03e-EP-selected one-shot composition after C03e-EQ provided the
//! bridge-owned requester-specific receive adapter. C03e-EV additionally materializes the separately
//! selected single-owner one-transaction post-authenticated ingress seam while leaving the ER method
//! itself unchanged and uninvoked. The existing capability loop and worker do not invoke either seam.
//! Neither seam activates requester/rendezvous authority/provider execution, a combined loop, retry,
//! peer-close policy, dialing, readiness publication, or runtime activation.

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
    SharedCurrentCapabilityAuthority, adapt_decoded_requester_rendezvous_target_device_id,
    adapt_post_auth_requester_rendezvous_target_intent,
};
use super::AuthenticatedRemoteSessionRuntimeOwner;

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
    /// Requester/rendezvous-family processing preserves outer `request_id` only as correlation,
    /// consumes the strict decoded logical target `DeviceId`, composes it through the existing C03e-EO
    /// then C03e-EJ helpers, and returns the existing correlated non-authoritative start-intent shape.
    /// It stops before C03e-DV, registry/requester-policy/provider execution, candidate selection,
    /// requester response construction/write, or dialing.
    ///
    /// The method performs one transaction only. It does not replace or invoke the existing
    /// capability loop/worker, does not invoke the isolated C03e-ER accept seam, and does not create a
    /// repeated combined loop, task, queue, retry, reconnect, fairness policy, backpressure policy,
    /// peer-close policy, readiness state, listener activation, deployment, or merge behavior.
    ///
    /// # Errors
    ///
    /// Preserves distinguishable failure classes for the one authenticated stream accept, C03e-ET
    /// ingress/strict PRWZ handling, existing capability authorization/dispatch, and existing
    /// same-stream capability response I/O. No failure is translated into fabricated success or a
    /// requester/rendezvous response frame.
    #[allow(
        dead_code,
        reason = "C03e-EV materializes the selected one-transaction ingress seam before separately gated combined-loop integration"
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
            PostAuthControlStreamIngress::RequesterRendezvous(request) => {
                let request_id = request.request_id();
                let target_intent = adapt_decoded_requester_rendezvous_target_device_id(
                    request.into_target_device_id(),
                );
                let start_intent =
                    adapt_post_auth_requester_rendezvous_target_intent(self, target_intent);
                Ok(
                    AuthenticatedRemoteSessionPostAuthIngressOutcome::RequesterRendezvous((
                        request_id,
                        start_intent,
                    )),
                )
            }
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
}
