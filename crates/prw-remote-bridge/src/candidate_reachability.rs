//! Source-only authenticated candidate-publication semantics for PRW dynamic reachability.
//!
//! Phase 152 C02e factors the already-locked provenance/admission ordering into one
//! bounded semantic adapter. This module intentionally defines no wire encoding,
//! freshness/replay token, socket operation, discovery authority, or runtime wiring.
//! It is not exported by `prw-remote-bridge` while the candidate wire/replay adapter
//! remains unselected.

use std::fmt;

use prw_connectivity::{
    ConnectivityCandidate, ConnectivityError, PeerConnectivityIdentity, PeerConnectivityPlan,
    TransportIdentity,
};
use prw_registry::{RegistryError, WorkspaceDeviceRegistry};
use prw_session::AuthenticatedDeviceSession;

/// Stable source-level failure classification for candidate publication/admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CandidateReachabilityError {
    /// A current registry/session/transport identity check failed.
    Registry(RegistryError),
    /// Requester and publisher are not in the same current workspace.
    WorkspaceMismatch,
    /// Publication identity does not exactly match the target connectivity plan.
    PublicationTargetMismatch,
    /// Candidate validation or transactional refresh failed.
    Connectivity(ConnectivityError),
}

impl fmt::Display for CandidateReachabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Registry(error) => {
                write!(formatter, "candidate registry admission failed: {error}")
            }
            Self::WorkspaceMismatch => {
                formatter.write_str("candidate publisher and requester workspace mismatch")
            }
            Self::PublicationTargetMismatch => {
                formatter.write_str("candidate publication target identity mismatch")
            }
            Self::Connectivity(error) => {
                write!(
                    formatter,
                    "candidate connectivity validation failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for CandidateReachabilityError {}

/// Candidate set whose peer identity is derived from a registry-current authenticated publisher.
///
/// This object is a semantic snapshot only. It intentionally carries no generic control-frame
/// request identifier and makes no freshness/replay claim. A future reviewed wire adapter must
/// add an independent bounded freshness mechanism before production consumption is possible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedCandidatePublication {
    publisher_session: AuthenticatedDeviceSession,
    peer: PeerConnectivityIdentity,
    candidates: Vec<ConnectivityCandidate>,
}

impl AuthenticatedCandidatePublication {
    /// Returns the authenticated publisher session snapshot used to derive provenance.
    #[must_use]
    pub const fn publisher_session(&self) -> &AuthenticatedDeviceSession {
        &self.publisher_session
    }

    /// Returns the exact logical/transport identity derived for the publisher.
    #[must_use]
    pub const fn peer(&self) -> &PeerConnectivityIdentity {
        &self.peer
    }

    /// Returns the bounded validated candidate vector.
    #[must_use]
    pub fn candidates(&self) -> &[ConnectivityCandidate] {
        &self.candidates
    }
}

/// Creates one bounded candidate publication from a current authenticated publisher.
///
/// The caller cannot supply an arbitrary target `DeviceId`: the logical target is derived from
/// the publisher session after current-registry validation. The presented transport identity is
/// separately revalidated for that exact publisher device. The complete candidate set is checked
/// before a publication object can exist.
///
/// # Errors
///
/// Fails closed on stale publisher session/registry state, stale transport identity, or invalid
/// candidate-set semantics.
pub fn publish_current_candidates(
    registry: &WorkspaceDeviceRegistry,
    publisher_session: &AuthenticatedDeviceSession,
    presented_transport_identity: TransportIdentity,
    candidates: Vec<ConnectivityCandidate>,
) -> Result<AuthenticatedCandidatePublication, CandidateReachabilityError> {
    let publisher = registry
        .validate_authenticated_session(publisher_session)
        .map_err(CandidateReachabilityError::Registry)?;
    registry
        .validate_transport_identity(publisher.device_id(), presented_transport_identity)
        .map_err(CandidateReachabilityError::Registry)?;

    let peer =
        PeerConnectivityIdentity::new(publisher.device_id().clone(), presented_transport_identity);
    PeerConnectivityPlan::new(peer.clone(), candidates.clone())
        .map_err(CandidateReachabilityError::Connectivity)?;

    Ok(AuthenticatedCandidatePublication {
        publisher_session: publisher_session.clone(),
        peer,
        candidates,
    })
}

/// Revalidates requester, publisher, workspace and exact target currentness without mutating the
/// target connectivity plan.
///
/// This source-only precheck exists so a later upper composition authority can place its
/// independent publication-freshness comparison after current identity/workspace admission but
/// before candidate-plan mutation.
///
/// Admission order is intentionally fixed:
///
/// 1. requester authenticated session is registry-current;
/// 2. publisher authenticated session is registry-current;
/// 3. requester and publisher share the same current workspace;
/// 4. publication peer exactly matches the target plan and publisher device;
/// 5. target transport identity remains registry-current.
///
/// # Errors
///
/// Fails closed on stale/cross-workspace/retargeted identity. The supplied plan is never mutated.
pub fn validate_authenticated_publication_admission(
    registry: &WorkspaceDeviceRegistry,
    requester_session: &AuthenticatedDeviceSession,
    publication: &AuthenticatedCandidatePublication,
    plan: &PeerConnectivityPlan,
) -> Result<(), CandidateReachabilityError> {
    let requester = registry
        .validate_authenticated_session(requester_session)
        .map_err(CandidateReachabilityError::Registry)?;
    let publisher = registry
        .validate_authenticated_session(publication.publisher_session())
        .map_err(CandidateReachabilityError::Registry)?;

    if requester.workspace_id() != publisher.workspace_id() {
        return Err(CandidateReachabilityError::WorkspaceMismatch);
    }
    if publisher.device_id() != publication.peer().device_id() || plan.peer() != publication.peer()
    {
        return Err(CandidateReachabilityError::PublicationTargetMismatch);
    }

    registry
        .validate_transport_identity(
            publication.peer().device_id(),
            publication.peer().transport_identity(),
        )
        .map_err(CandidateReachabilityError::Registry)
}

/// Applies one publication only after requester, publisher, workspace and target currentness.
///
/// Admission order is intentionally fixed:
///
/// 1. requester authenticated session is registry-current;
/// 2. publisher authenticated session is registry-current;
/// 3. requester and publisher share the same current workspace;
/// 4. publication peer exactly matches the target plan and publisher device;
/// 5. target transport identity remains registry-current;
/// 6. only then may the connectivity plan transactionally validate and replace candidates.
///
/// Any failure before the final step leaves the target plan untouched. Candidate-ID non-rebinding
/// and full-vector transactional validation are delegated to `PeerConnectivityPlan`.
///
/// # Errors
///
/// Fails closed on stale/cross-workspace/retargeted identity or invalid refresh semantics.
pub fn refresh_from_authenticated_publication(
    registry: &WorkspaceDeviceRegistry,
    requester_session: &AuthenticatedDeviceSession,
    publication: &AuthenticatedCandidatePublication,
    plan: &mut PeerConnectivityPlan,
) -> Result<(), CandidateReachabilityError> {
    validate_authenticated_publication_admission(registry, requester_session, publication, plan)?;

    plan.refresh_candidates(publication.candidates.clone())
        .map_err(CandidateReachabilityError::Connectivity)
}
