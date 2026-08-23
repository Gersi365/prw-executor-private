use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use aws_lc_rs::{
    rand::SystemRandom,
    signature::{ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair},
};
use prw_connectivity::TransportIdentity;
use prw_control_plane::DeviceIdentityBinding;
use prw_core::{DeviceId, DeviceLifecycle, SessionId, UserId, WorkspaceId};
use prw_device_identity_signer::UbuntuEnrollmentSigner;
use prw_remote_bridge::session_auth_wire::{
    SESSION_AUTH_WIRE_MAGIC, SessionAuthenticationWireChallenge, SessionAuthenticationWireError,
    SessionAuthenticationWireMessage, decode_session_authentication_frame,
    receive_session_authentication_message, send_session_authentication_message,
};
use prw_remote_transport::{
    ControlFrame, ControlMessageKind, build_client_config, build_server_config,
    runtime::MeshQuicEndpoint, transport_identity_from_certificate, transport_server_name,
};
use prw_session::SessionAuthenticationService;
use rcgen::{
    BasicConstraints, CertificateParams, CertifiedIssuer, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose, PKCS_ECDSA_P256_SHA256,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

struct PeerMaterial {
    certificate: CertificateDer<'static>,
    private_key: Vec<u8>,
    identity: TransportIdentity,
}

fn private_key(bytes: &[u8]) -> PrivateKeyDer<'static> {
    PrivatePkcs8KeyDer::from(bytes.to_vec()).into()
}

fn make_leaf(ca: &CertifiedIssuer<'_, KeyPair>) -> PeerMaterial {
    let key = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).expect("generate test leaf key");
    let mut provisional_params =
        CertificateParams::new(vec!["placeholder.mesh.prw.invalid".to_owned()])
            .expect("valid placeholder name");
    provisional_params.extended_key_usages = vec![
        ExtendedKeyUsagePurpose::ServerAuth,
        ExtendedKeyUsagePurpose::ClientAuth,
    ];
    provisional_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    let provisional = provisional_params
        .signed_by(&key, ca)
        .expect("sign provisional test leaf");
    let identity =
        transport_identity_from_certificate(provisional.der()).expect("derive transport identity");

    let mut final_params =
        CertificateParams::new(vec![transport_server_name(identity)]).expect("valid transport SAN");
    final_params.extended_key_usages = vec![
        ExtendedKeyUsagePurpose::ServerAuth,
        ExtendedKeyUsagePurpose::ClientAuth,
    ];
    final_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    let certificate = final_params
        .signed_by(&key, ca)
        .expect("sign final test leaf")
        .der()
        .clone();
    assert_eq!(
        transport_identity_from_certificate(&certificate),
        Ok(identity)
    );

    PeerMaterial {
        certificate,
        private_key: key.serialize_der(),
        identity,
    }
}

fn transport_fixtures() -> (CertificateDer<'static>, PeerMaterial, PeerMaterial) {
    let ca_key = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256).expect("generate disposable CA");
    let mut ca_params = CertificateParams::new(Vec::<String>::new()).expect("CA params");
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::CrlSign,
    ];
    let ca = CertifiedIssuer::self_signed(ca_params, ca_key).expect("self-sign disposable CA");
    (ca.der().clone(), make_leaf(&ca), make_leaf(&ca))
}

fn device_signer() -> UbuntuEnrollmentSigner {
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &SystemRandom::new())
        .expect("generate disposable device key");
    UbuntuEnrollmentSigner::from_pkcs8_v1_der(pkcs8.as_ref()).expect("load device signer")
}

fn device_binding(signer: &UbuntuEnrollmentSigner) -> DeviceIdentityBinding {
    DeviceIdentityBinding {
        workspace_id: WorkspaceId::new("workspace-c03d").expect("workspace id"),
        user_id: UserId::new("user-c03d").expect("user id"),
        device_id: DeviceId::new("device-c03d").expect("device id"),
        public_identity: signer.public_identity().clone(),
        lifecycle: DeviceLifecycle::Enrolled,
    }
}

const fn loopback_any() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)
}

#[tokio::test(flavor = "current_thread")]
async fn logical_session_challenge_and_proof_cross_real_quic_and_authenticate() {
    let (root, server_material, client_material) = transport_fixtures();
    let server = MeshQuicEndpoint::bind_server(
        loopback_any(),
        build_server_config(
            vec![root.clone()],
            vec![server_material.certificate.clone()],
            private_key(&server_material.private_key),
        )
        .expect("server TLS config"),
    )
    .expect("bind server endpoint");
    let client = MeshQuicEndpoint::bind_client(
        loopback_any(),
        build_client_config(
            vec![root],
            vec![client_material.certificate.clone()],
            private_key(&client_material.private_key),
        )
        .expect("client TLS config"),
    )
    .expect("bind client endpoint");
    let server_addr = server.local_addr().expect("server local address");

    let signer = device_signer();
    let binding = device_binding(&signer);
    let session_id = SessionId::new("session-c03d").expect("session id");
    let mut service = SessionAuthenticationService::new();
    let challenge = service
        .begin_session(binding.clone(), session_id.clone(), 1_000, 1_200)
        .expect("begin typed session authentication");
    let wire_challenge = SessionAuthenticationWireMessage::Challenge(
        SessionAuthenticationWireChallenge::from_typed(&challenge),
    );

    let server_side = async {
        let connection = server
            .accept_authenticated(client_material.identity)
            .await
            .expect("transport-authenticated server connection");
        let mut stream = connection
            .accept_control_stream()
            .await
            .expect("accept session-auth stream");
        send_session_authentication_message(&mut stream, 91, &wire_challenge)
            .await
            .expect("send typed challenge wire message");
        let (request_id, message) = receive_session_authentication_message(&mut stream)
            .await
            .expect("receive typed proof wire message");
        assert_eq!(request_id, 91);
        let proof = match message {
            SessionAuthenticationWireMessage::Proof(proof) => proof,
            SessionAuthenticationWireMessage::Challenge(_) => panic!("expected proof"),
        };
        let authenticated = service
            .submit_proof(&session_id, &proof, 1_001)
            .expect("existing session service authenticates proof");
        assert_eq!(authenticated.session_id(), &session_id);
        assert_eq!(authenticated.workspace_id(), &binding.workspace_id);
        assert_eq!(authenticated.user_id(), &binding.user_id);
        assert_eq!(authenticated.device_id(), &binding.device_id);
        connection
    };

    let client_side = async {
        let connection = client
            .connect_authenticated(server_addr, server_material.identity)
            .await
            .expect("transport-authenticated client connection");
        let mut stream = connection
            .open_control_stream()
            .await
            .expect("open session-auth stream");
        let (request_id, message) = receive_session_authentication_message(&mut stream)
            .await
            .expect("receive challenge wire message");
        assert_eq!(request_id, 91);
        let wire = match message {
            SessionAuthenticationWireMessage::Challenge(wire) => wire,
            SessionAuthenticationWireMessage::Proof(_) => panic!("expected challenge"),
        };
        let typed_challenge = wire
            .to_typed_challenge(&binding)
            .expect("rehydrate typed challenge against enrolled binding");
        assert_eq!(typed_challenge, challenge);
        let proof = signer
            .sign_session_auth_proof(&binding, &typed_challenge)
            .expect("sign existing Phase 128 proof");
        send_session_authentication_message(
            &mut stream,
            request_id,
            &SessionAuthenticationWireMessage::Proof(proof),
        )
        .await
        .expect("send proof wire message");
        connection
    };

    let (server_connection, client_connection) = tokio::join!(server_side, client_side);
    server_connection.close(0, b"done");
    client_connection.close(0, b"done");
    server.close(0, b"done");
    client.close(0, b"done");
    server.wait_idle().await;
    client.wait_idle().await;
}

#[test]
fn session_wire_fails_closed_on_wrong_outer_kind_and_malformed_header() {
    let wrong_outer = ControlFrame::new(ControlMessageKind::Request, 7, Vec::new())
        .expect("bounded wrong outer frame");
    assert_eq!(
        decode_session_authentication_frame(&wrong_outer),
        Err(SessionAuthenticationWireError::InvalidOuterKind)
    );

    let mut malformed = Vec::new();
    malformed.extend_from_slice(&SESSION_AUTH_WIRE_MAGIC);
    malformed.extend_from_slice(&1_u16.to_be_bytes());
    malformed.extend_from_slice(&0_u16.to_be_bytes());
    malformed.extend_from_slice(&1_u16.to_be_bytes());
    malformed.extend_from_slice(&1_u16.to_be_bytes());
    let frame = ControlFrame::new(ControlMessageKind::SessionAuthentication, 8, malformed)
        .expect("bounded malformed session frame");
    assert_eq!(
        decode_session_authentication_frame(&frame),
        Err(SessionAuthenticationWireError::InvalidPayload)
    );
}
