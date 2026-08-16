use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};

use prw_control_transport::{CONTROL_ALPN, ControlTlsClientConfig, ControlTransportError};
use rustls::{
    ServerConfig, ServerConnection,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};

const CA_PEM: &[u8] = include_bytes!("fixtures/phase129-ca.pem");
const SERVER_CERT_PEM: &[u8] = include_bytes!("fixtures/phase129-server.pem");
const SERVER_KEY_PEM: &[u8] = include_bytes!("fixtures/phase129-server-key.pem");

fn disposable_root_der() -> Vec<u8> {
    CertificateDer::from_pem_slice(CA_PEM)
        .expect("parse disposable test CA")
        .as_ref()
        .to_vec()
}

fn server_config(tls13: bool, alpn: &[u8]) -> Arc<ServerConfig> {
    let cert = CertificateDer::from_pem_slice(SERVER_CERT_PEM).expect("parse server cert");
    let key = PrivateKeyDer::from_pem_slice(SERVER_KEY_PEM).expect("parse server key");
    let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
    let versions = if tls13 {
        &[&rustls::version::TLS13][..]
    } else {
        &[&rustls::version::TLS12][..]
    };
    let mut config = ServerConfig::builder_with_provider(provider)
        .with_protocol_versions(versions)
        .expect("build disposable server protocol profile")
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)
        .expect("build disposable server config");
    config.alpn_protocols = vec![alpn.to_vec()];
    Arc::new(config)
}

fn spawn_tls_server(config: Arc<ServerConfig>) -> (std::net::SocketAddr, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind disposable loopback listener");
    let addr = listener.local_addr().expect("loopback address");
    let handle = thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("accept disposable client");
        socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .expect("server read timeout");
        socket
            .set_write_timeout(Some(Duration::from_secs(3)))
            .expect("server write timeout");
        let mut connection = ServerConnection::new(config).expect("server connection");
        while connection.is_handshaking() {
            if connection.complete_io(&mut socket).is_err() {
                return;
            }
        }
    });
    (addr, handle)
}

fn client_config(
    addr: std::net::SocketAddr,
    server_name: &str,
) -> Result<ControlTlsClientConfig, ControlTransportError> {
    ControlTlsClientConfig::new(
        addr,
        server_name,
        &[disposable_root_der()],
        Duration::from_secs(3),
        Duration::from_secs(3),
        Duration::from_secs(3),
    )
}

#[test]
fn tls13_expected_name_and_alpn_establishes_transport() {
    let (addr, server) = spawn_tls_server(server_config(true, CONTROL_ALPN));
    let config = client_config(addr, "control.test").expect("valid client config");
    let _stream = config.connect().expect("TLS 1.3 control transport");
    server.join().expect("server thread");
}

#[test]
fn wrong_server_name_fails_closed() {
    let (addr, server) = spawn_tls_server(server_config(true, CONTROL_ALPN));
    let config = client_config(addr, "wrong.test").expect("valid client config");
    assert!(matches!(
        config.connect(),
        Err(ControlTransportError::TlsHandshake)
    ));
    server.join().expect("server thread");
}

#[test]
fn wrong_alpn_fails_closed_after_tls_handshake() {
    let (addr, server) = spawn_tls_server(server_config(true, b"wrong-control/1"));
    let config = client_config(addr, "control.test").expect("valid client config");
    assert!(matches!(
        config.connect(),
        Err(ControlTransportError::TlsHandshake) | Err(ControlTransportError::WrongAlpn)
    ));
    server.join().expect("server thread");
}

#[test]
fn tls12_only_server_has_no_fallback_path() {
    let (addr, server) = spawn_tls_server(server_config(false, CONTROL_ALPN));
    let config = client_config(addr, "control.test").expect("valid client config");
    assert!(matches!(
        config.connect(),
        Err(ControlTransportError::TlsHandshake)
    ));
    server.join().expect("server thread");
}

#[test]
fn plaintext_peer_has_no_fallback_path() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind plaintext loopback listener");
    let addr = listener.local_addr().expect("loopback address");
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("accept plaintext client");
        socket
            .write_all(b"not tls\n")
            .expect("write disposable plaintext");
    });
    let config = client_config(addr, "control.test").expect("valid client config");
    assert!(matches!(
        config.connect(),
        Err(ControlTransportError::TlsHandshake)
    ));
    server.join().expect("server thread");
}
