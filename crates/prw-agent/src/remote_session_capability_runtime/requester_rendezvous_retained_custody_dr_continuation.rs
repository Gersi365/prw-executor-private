//! Agent-owned retained-custody requester/rendezvous DR continuation.
//!
//! C03e-FB materializes only the C03e-FA-selected continuation from one exact C03e-EZ
//! response-stream custody handoff through the existing shared-current authority read and existing
//! C03e-DR DI -> DP -> DK -> DN composition. The exact bridge requester transaction survives both
//! DR success and DR failure for separately gated response mapping. This module performs no response
//! construction/write, second read, loop resume, candidate selection, dialing, runtime activation,
//! deployment or merge.

use prw_policy::PolicyEvaluator;
use prw_remote_bridge::post_auth_control_stream_ingress::PostAuthRequesterRendezvousTransaction;

use super::{
    RequesterRendezvousResponseStreamCustodyHandoff, SharedCurrentCapabilityAuthority,
};
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
    #[must_use]
    pub(super) const fn dr_result(
        &self,
    ) -> &Result<(), RequesterRendezvousStartCompositionError> {
        &self.dr_result
    }

    /// Transfers the exact requester transaction and exact terminal DR result by value.
    ///
    /// This is custody transfer only and performs no stream I/O or response construction.
    #[must_use]
    pub(super) fn into_parts(
        self,
    ) -> (
        PostAuthRequesterRendezvousTransaction,
        Result<(), RequesterRendezvousStartCompositionError>,
    ) {
        (self.requester_transaction, self.dr_result)
    }
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
