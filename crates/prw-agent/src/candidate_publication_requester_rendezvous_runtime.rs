//! Agent-owned lifetime custody for candidate-publication requester/rendezvous authority.
//!
//! C03e-CZ materializes only the C03e-CY-selected by-value lifetime owner. It does not expose
//! requester/rendezvous operations, select synchronization, start tasks/listeners, perform I/O,
//! publish readiness, or activate production networking.

use prw_remote_bridge::requester_rendezvous_in_memory_provider::InMemoryRequesterRendezvousAuthorityProvider;

/// Owns exactly one already-configured process-local requester/rendezvous authority provider.
///
/// Construction transfers lifetime custody only. The provider remains private and this type does
/// not expose registration, retirement, authorization, execution, raw-provider access, task/thread
/// ownership, synchronization, persistence, readiness, or networking behavior.
#[derive(Debug)]
pub struct CandidatePublicationRequesterRendezvousRuntimeOwner {
    _provider: InMemoryRequesterRendezvousAuthorityProvider,
}

impl CandidatePublicationRequesterRendezvousRuntimeOwner {
    /// Takes by-value lifetime ownership of one already-configured provider.
    ///
    /// Provider capacity and lifecycle state are established by the caller before construction.
    /// This constructor performs no registration, authorization, I/O, task creation, or readiness
    /// publication.
    #[must_use]
    pub const fn new(provider: InMemoryRequesterRendezvousAuthorityProvider) -> Self {
        Self {
            _provider: provider,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CandidatePublicationRequesterRendezvousRuntimeOwner;
    use prw_remote_bridge::requester_rendezvous_in_memory_provider::InMemoryRequesterRendezvousAuthorityProvider;

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
}
