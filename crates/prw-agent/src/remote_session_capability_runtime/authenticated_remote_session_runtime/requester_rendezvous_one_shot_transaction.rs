//! Isolated Agent-owned requester/rendezvous one-shot target-request composition.
//!
//! C03e-ER materializes only the C03e-EP-selected one-shot composition after C03e-EQ provided the
//! bridge-owned requester-specific receive adapter. The existing capability loop and worker do not
//! invoke this seam. It performs no requester/rendezvous authority execution, provider mutation,
//! response write, retry, loop integration, peer close, dialing, readiness publication, or runtime
//! activation.

use prw_remote_bridge::requester_rendezvous_target_request_io::receive_requester_rendezvous_target_request;

use super::super::{
    RequesterRendezvousCorrelatedStartIntent, RequesterRendezvousOneShotTransactionError,
    adapt_decoded_requester_rendezvous_target_device_id,
    adapt_post_auth_requester_rendezvous_target_intent,
};
use super::AuthenticatedRemoteSessionRuntimeOwner;

impl AuthenticatedRemoteSessionRuntimeOwner {
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
        &mut self,
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
