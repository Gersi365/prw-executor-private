//! Agent-owned lifetime custody for candidate-publication requester/rendezvous authority.
//!
//! C03e-CZ materializes the C03e-CY-selected by-value lifetime owner. C03e-DN additionally
//! materializes only the C03e-DM-selected crate-internal registration mutation composition after
//! registry validation and dedicated requester/rendezvous-start policy authorization. It does not
//! expose raw provider access, retirement/authorization operations, select synchronization, start
//! tasks/listeners, perform I/O, publish readiness, or activate production networking.

use prw_remote_bridge::requester_rendezvous_in_memory_provider::{
    InMemoryRequesterRendezvousAuthorityProvider, RequesterRendezvousLifecycleError,
};

use crate::candidate_publication_requester_rendezvous_start_intent::policy_admission::PolicyAuthorizedRequesterRendezvousStart;

/// Owns exactly one already-configured process-local requester/rendezvous authority provider.
///
/// Construction transfers lifetime custody only. The provider remains private. The only
/// requester/rendezvous-start mutation exposed by this owner is crate-internal and requires the
/// post-registry, post-policy provenance selected by C03e-DL/DM. This type still exposes no raw
/// provider access, retirement, publisher authorization, task/thread ownership, synchronization,
/// persistence, readiness, or networking behavior.
#[derive(Debug)]
pub struct CandidatePublicationRequesterRendezvousRuntimeOwner {
    provider: InMemoryRequesterRendezvousAuthorityProvider,
}

impl CandidatePublicationRequesterRendezvousRuntimeOwner {
    /// Takes by-value lifetime ownership of one already-configured provider.
    ///
    /// Provider capacity and lifecycle state are established by the caller before construction.
    /// This constructor performs no registration, authorization, I/O, task creation, or readiness
    /// publication.
    #[must_use]
    pub const fn new(provider: InMemoryRequesterRendezvousAuthorityProvider) -> Self {
        Self { provider }
    }

    /// Registers one policy-authorized requester/rendezvous start in the private provider.
    ///
    /// The sole authority input is the consumed C03e-DK policy-authorized provenance carrier. The
    /// exact authenticated requester session and DI-validated target `DeviceId` are cloned only
    /// inside this post-policy mutation boundary to satisfy the concrete provider's owned storage
    /// signature. No raw provider reference or reusable registration token is exposed.
    ///
    /// # Errors
    ///
    /// Propagates the bounded requester/rendezvous provider lifecycle error. Duplicate identity and
    /// capacity exhaustion therefore remain fail-before-mutation outcomes of `register_current`.
    pub(crate) fn register_policy_authorized_requester_rendezvous_start(
        &mut self,
        authorized: PolicyAuthorizedRequesterRendezvousStart,
    ) -> Result<(), RequesterRendezvousLifecycleError> {
        let validated = authorized.registry_validated();
        let requester_session = validated.requester_session().clone();
        let target_device_id = validated.target_device_id().clone();

        self.provider
            .register_current(requester_session, target_device_id)
    }
}

#[cfg(test)]
mod tests {
    use prw_remote_bridge::requester_rendezvous_in_memory_provider::{
        InMemoryRequesterRendezvousAuthorityProvider, RequesterRendezvousLifecycleError,
    };

    use super::CandidatePublicationRequesterRendezvousRuntimeOwner;
    use crate::candidate_publication_requester_rendezvous_start_intent::policy_admission::PolicyAuthorizedRequesterRendezvousStart;

    #[test]
    fn constructor_has_exact_selected_by_value_shape() {
        let constructor: fn(
            InMemoryRequesterRendezvousAuthorityProvider,
        ) -> CandidatePublicationRequesterRendezvousRuntimeOwner =
            CandidatePublicationRequesterRendezvousRuntimeOwner::new;
        let provider = InMemoryRequesterRendezvousAuthorityProvider::new(1)
            .expect("explicit non-zero provider capacity");

        let _owner = constructor(provider);
    }

    #[test]
    fn owner_construction_consumes_an_existing_provider_without_provider_clone() {
        let provider = InMemoryRequesterRendezvousAuthorityProvider::new(2)
            .expect("explicit non-zero provider capacity");

        let _owner = CandidatePublicationRequesterRendezvousRuntimeOwner::new(provider);
    }

    #[test]
    fn registration_method_has_exact_selected_consuming_shape() {
        let registration: fn(
            &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
            PolicyAuthorizedRequesterRendezvousStart,
        ) -> Result<(), RequesterRendezvousLifecycleError> =
            CandidatePublicationRequesterRendezvousRuntimeOwner::register_policy_authorized_requester_rendezvous_start;

        let _ = registration;
    }
}
