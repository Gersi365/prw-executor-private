//! Phase 152 C02e source-level registry/reachability admission tests.
//!
//! These tests intentionally perform no socket, NAT traversal, relay, DNS, filesystem,
//! process, PTY, runtime, deployment, or service-manager operation. They stage the
//! fail-closed ordering required before transient endpoint refresh may be admitted by a
//! later runtime integration.

use std::net::{IpAddr, Ipv4Addr};

use aws_lc_rs::{
    rand::SystemRandom,
    signature::{ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair},
};
use prw_connectivity::{
    CandidateId, ConnectivityCandidate, ConnectivityEndpoint, ConnectivityError,
    ConnectivityPathKind, PeerConnectivityIdentity, PeerConnectivityPlan,
    ReachabilityObservation, SelectedConnectivityPath, TransportIdentity,
};
use prw_control_plane::DeviceIdentityBinding;
use prw_core::{DeviceId, DeviceLifecycle, UserId, WorkspaceId};
use prw_device_identity_signer::UbuntuEnrollmentSigner;
use prw_registry::{RegistryError, WorkspaceDeviceRegistry, WorkspaceRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReachabilityAdmissionError {
    Registry(RegistryError),
    Connectivity(ConnectivityError),
}

fn refresh_after_current_registry_revalidation(
    registry: &WorkspaceDeviceRegistry,
    plan: &mut PeerConnectivityPlan,
    candidates: Vec<ConnectivityCandidate>,
) -> Result<(), ReachabilityAdmissionError> {
    registry
        .validate_transport_identity(
            plan.peer().device_id(),
            plan.peer().transport_identity(),
        )
        .map_err(ReachabilityAdmissionError::Registry)?;
    plan.refresh_candidates(candidates)
        .map_err(ReachabilityAdmissionError::Connectivity)
}

fn signer() -> UbuntuEnrollmentSigner {
    let pkcs8 =
        EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &SystemRandom::new())
            .expect("generate disposable C02e key");
    UbuntuEnrollmentSigner::from_pkcs8_v1_der(pkcs8.as_ref())
        .expect("load disposable C02e signer")
}

fn transport(seed: u8) -> TransportIdentity {
    TransportIdentity::new([seed; 32]).expect("non-zero transport identity")
}

fn candidate(value: u64, kind: ConnectivityPathKind, port: u16) -> ConnectivityCandidate {
    ConnectivityCandidate::new(
        CandidateId::new(value).expect("non-zero candidate id"),
        kind,
        ConnectivityEndpoint::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)
            .expect("valid explicit endpoint"),
    )
}

fn registry_and_plan(
    current_transport: TransportIdentity,
) -> (WorkspaceDeviceRegistry, DeviceId, PeerConnectivityPlan) {
    let signer = signer();
    let workspace_id = WorkspaceId::new("workspace-c02e").expect("workspace id");
    let user_id = UserId::new("user-c02e").expect("user id");
    let device_id = DeviceId::new("device-c02e").expect("device id");

    let binding = DeviceIdentityBinding {
        workspace_id: workspace_id.clone(),
        user_id: user_id.clone(),
        device_id: device_id.clone(),
        public_identity: signer.public_identity().clone(),
        lifecycle: DeviceLifecycle::Enrolled,
    };

    let mut registry = WorkspaceDeviceRegistry::new();
    registry
        .add_membership(workspace_id, user_id, WorkspaceRole::Member)
        .expect("active membership");
    registry.register_device(binding).expect("registered device");
    registry
        .bind_transport_identity(&device_id, current_transport)
        .expect("current transport identity");

    let identity = PeerConnectivityIdentity::new(device_id.clone(), current_transport);
    let plan = PeerConnectivityPlan::new(
        identity,
        vec![candidate(1, ConnectivityPathKind::LocalDirect, 2001)],
    )
    .expect("initial connectivity plan");

    (registry, device_id, plan)
}

#[test]
fn current_registry_identity_allows_transient_candidate_refresh() {
    let current_transport = transport(1);
    let (registry, _device_id, mut plan) = registry_and_plan(current_transport);
    plan.set_observation(
        CandidateId::new(1).expect("candidate id"),
        ReachabilityObservation::Reachable,
    )
    .expect("old path observation");
    let expected_identity = plan.peer().clone();

    refresh_after_current_registry_revalidation(
        &registry,
        &mut plan,
        vec![
            candidate(2, ConnectivityPathKind::InternetDirect, 3002),
            candidate(3, ConnectivityPathKind::Relay, 3003),
        ],
    )
    .expect("registry-current peer may accept refreshed transient endpoints");

    assert_eq!(plan.peer(), &expected_identity);
    assert_eq!(plan.peer().transport_identity(), current_transport);
    assert_eq!(plan.candidate_count(), 2);
    assert_eq!(plan.selected_path(), SelectedConnectivityPath::Offline);
    assert_eq!(
        plan.set_observation(
            CandidateId::new(1).expect("removed candidate id"),
            ReachabilityObservation::Reachable,
        ),
        Err(ConnectivityError::UnknownCandidate)
    );
}

#[test]
fn transport_rotation_rejects_stale_plan_before_endpoint_mutation() {
    let original_transport = transport(1);
    let replacement_transport = transport(2);
    let (mut registry, device_id, mut plan) = registry_and_plan(original_transport);
    plan.set_observation(
        CandidateId::new(1).expect("candidate id"),
        ReachabilityObservation::Reachable,
    )
    .expect("old path observation");
    let before = plan.clone();

    registry
        .rotate_transport_identity(&device_id, original_transport, replacement_transport)
        .expect("registry transport rotation");

    assert_eq!(
        refresh_after_current_registry_revalidation(
            &registry,
            &mut plan,
            vec![candidate(2, ConnectivityPathKind::InternetDirect, 3002)],
        ),
        Err(ReachabilityAdmissionError::Registry(
            RegistryError::TransportIdentityMismatch
        ))
    );
    assert_eq!(plan, before);
}

#[test]
fn device_revocation_rejects_refresh_before_endpoint_mutation() {
    let current_transport = transport(1);
    let (mut registry, device_id, mut plan) = registry_and_plan(current_transport);
    let before = plan.clone();

    registry.revoke_device(&device_id).expect("revoke device");

    assert_eq!(
        refresh_after_current_registry_revalidation(
            &registry,
            &mut plan,
            vec![candidate(2, ConnectivityPathKind::Relay, 3002)],
        ),
        Err(ReachabilityAdmissionError::Registry(
            RegistryError::DeviceRevoked
        ))
    );
    assert_eq!(plan, before);
}
