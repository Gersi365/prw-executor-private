//! Agent-internal requester-aware policy-source boundary for requester/rendezvous start.
//!
//! C03e-DP materializes only the C03e-DO-selected source interface required to resolve the
//! existing policy evaluator from the exact authenticated requester retained by C03e-DI. It does
//! not materialize a concrete policy store, cache, fallback policy, caller composition, provider
//! mutation, wire handling, I/O, runtime activation, networking, or deployment.

use std::fmt;

use prw_policy::PolicyEvaluator;
use prw_session::AuthenticatedDeviceSession;

/// Stable fail-closed failure while resolving the policy evaluator for one exact requester.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum RequesterRendezvousStartPolicySourceError {
    /// No authoritative requester policy is currently available.
    Unavailable,
    /// The authoritative requester policy cannot be resolved deterministically.
    Indeterminate,
}

impl fmt::Display for RequesterRendezvousStartPolicySourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Unavailable => "requester rendezvous start policy source unavailable",
            Self::Indeterminate => "requester rendezvous start policy source indeterminate",
        })
    }
}

impl std::error::Error for RequesterRendezvousStartPolicySourceError {}

/// Resolves the existing policy evaluator authoritative for one exact authenticated requester.
///
/// The requester must be the exact server-held [`AuthenticatedDeviceSession`] that survived the
/// separately materialized C03e-DI current-registry validation. Implementations must fail closed
/// rather than substitute a process-global/default evaluator or another requester's evaluator.
///
/// The returned evaluator borrow is tied to `self`, not to the requester borrow. This permits a
/// later caller to end its temporary requester-session borrow and then move the unchanged C03e-DI
/// provenance carrier by value into the existing C03e-DK admission function.
///
/// This trait selects policy source only. It does not evaluate a capability, create
/// `PolicyAuthorizedRequesterRendezvousStart`, mutate requester/rendezvous provider state, or grant
/// reusable registration authority.
pub(crate) trait RequesterRendezvousStartPolicySource {
    /// Existing policy evaluator type resolved by this source.
    type Evaluator: PolicyEvaluator + ?Sized;

    /// Resolves the evaluator authoritative for the exact authenticated requester.
    ///
    /// # Errors
    ///
    /// Returns a bounded fail-closed source error when authoritative policy is unavailable or
    /// cannot be resolved deterministically. Failure must not select a fallback evaluator.
    fn evaluator_for_requester<'a>(
        &'a self,
        requester: &AuthenticatedDeviceSession,
    ) -> Result<&'a Self::Evaluator, RequesterRendezvousStartPolicySourceError>;
}

#[cfg(test)]
mod tests {
    use prw_policy::{Capability, Decision, PolicyEvaluator};
    use prw_session::AuthenticatedDeviceSession;

    use super::{RequesterRendezvousStartPolicySource, RequesterRendezvousStartPolicySourceError};

    struct SignatureOnlyEvaluator;

    impl PolicyEvaluator for SignatureOnlyEvaluator {
        fn evaluate(&self, _capability: Capability) -> Decision {
            Decision::Deny
        }
    }

    struct SignatureOnlySource {
        evaluator: SignatureOnlyEvaluator,
    }

    impl RequesterRendezvousStartPolicySource for SignatureOnlySource {
        type Evaluator = SignatureOnlyEvaluator;

        fn evaluator_for_requester<'a>(
            &'a self,
            _requester: &AuthenticatedDeviceSession,
        ) -> Result<&'a Self::Evaluator, RequesterRendezvousStartPolicySourceError> {
            Ok(&self.evaluator)
        }
    }

    #[test]
    fn source_method_keeps_evaluator_borrow_owned_by_source_not_requester_borrow() {
        let method: for<'source, 'requester> fn(
            &'source SignatureOnlySource,
            &'requester AuthenticatedDeviceSession,
        ) -> Result<
            &'source SignatureOnlyEvaluator,
            RequesterRendezvousStartPolicySourceError,
        > = <SignatureOnlySource as RequesterRendezvousStartPolicySource>::evaluator_for_requester;

        let _ = method;
    }

    #[test]
    fn source_failures_are_stable_and_distinct() {
        assert_eq!(
            RequesterRendezvousStartPolicySourceError::Unavailable.to_string(),
            "requester rendezvous start policy source unavailable"
        );
        assert_eq!(
            RequesterRendezvousStartPolicySourceError::Indeterminate.to_string(),
            "requester rendezvous start policy source indeterminate"
        );
        assert_ne!(
            RequesterRendezvousStartPolicySourceError::Unavailable,
            RequesterRendezvousStartPolicySourceError::Indeterminate
        );
    }
}
