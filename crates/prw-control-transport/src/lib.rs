//! Outbound-only TLS control-plane transport for Private Remote Workspace.
//!
//! This crate establishes the Phase 129 server-authenticated TLS transport and a
//! bounded binary frame envelope. TLS success is transport authentication only: it
//! does not enroll a device, authenticate a PRW session, or grant capabilities.

use std::{
    fmt,
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
    sync::Arc,
    time::Duration,
};

use rustls::{
    ClientConfig, ClientConnection, ProtocolVersion, RootCertStore, StreamOwned,
    pki_types::{CertificateDer, ServerName},
};

/// Exact Phase 129 ALPN identifier.
pub const CONTROL_ALPN: &[u8] = b"prw-control/1";
/// Exact frame magic.
pub const CONTROL_FRAME_MAGIC: [u8; 4] = *b"PRWC";
/// Initial control transport major version.
pub const CONTROL_PROTOCOL_MAJOR: u16 = 1;
/// Initial control transport minor version.
pub const CONTROL_PROTOCOL_MINOR: u16 = 0;
/// Exact fixed frame header length.
pub const CONTROL_FRAME_HEADER_LEN: usize = 24;
/// Maximum Phase 129 frame payload length.
pub const MAX_CONTROL_PAYLOAD_BYTES: usize = 65_536;
/// Maximum explicit trust-anchor count.
pub const MAX_CONTROL_TRUST_ANCHORS: usize = 16;
/// Maximum DER bytes accepted for one trust anchor.
pub const MAX_CONTROL_TRUST_ANCHOR_BYTES: usize = 65_536;
/// Maximum individual socket timeout accepted by the transport constructor.
pub const MAX_CONTROL_TIMEOUT: Duration = Duration::from_secs(60);

/// Transport-envelope message kind. Semantics remain above the transport layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ControlMessageKind {
    /// Authentication protocol payload.
    Authentication = 1,
    /// Command payload whose semantics and authorization are defined elsewhere.
    Command = 2,
    /// Response payload.
    Response = 3,
    /// Event payload.
    Event = 4,
    /// Heartbeat payload.
    Heartbeat = 5,
    /// Bounded protocol error payload.
    Error = 6,
}

impl ControlMessageKind {
    const fn from_code(code: u16) -> Option<Self> {
        match code {
            1 => Some(Self::Authentication),
            2 => Some(Self::Command),
            3 => Some(Self::Response),
            4 => Some(Self::Event),
            5 => Some(Self::Heartbeat),
            6 => Some(Self::Error),
            _ => None,
        }
    }
}

/// One bounded Phase 129 control transport frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFrame {
    kind: ControlMessageKind,
    request_id: u64,
    payload: Vec<u8>,
}

impl ControlFrame {
    /// Creates a validated frame.
    ///
    /// # Errors
    ///
    /// Rejects request identifier zero and payloads above the locked bound.
    pub fn new(
        kind: ControlMessageKind,
        request_id: u64,
        payload: impl Into<Vec<u8>>,
    ) -> Result<Self, ControlFrameError> {
        if request_id == 0 {
            return Err(ControlFrameError::ZeroRequestId);
        }
        let payload = payload.into();
        if payload.len() > MAX_CONTROL_PAYLOAD_BYTES {
            return Err(ControlFrameError::PayloadTooLarge);
        }
        Ok(Self {
            kind,
            request_id,
            payload,
        })
    }

    /// Returns the envelope message kind.
    #[must_use]
    pub const fn kind(&self) -> ControlMessageKind {
        self.kind
    }

    /// Returns the non-zero request identifier.
    #[must_use]
    pub const fn request_id(&self) -> u64 {
        self.request_id
    }

    /// Returns the exact payload bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Stable frame codec failure classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlFrameError {
    /// Fixed header could not be read completely.
    TruncatedHeader,
    /// Header magic is not the locked PRW control magic.
    InvalidMagic,
    /// Major/minor version is unsupported.
    UnsupportedVersion,
    /// Message kind code is unknown.
    UnknownKind,
    /// Reserved flags are non-zero.
    NonZeroFlags,
    /// Request identifier is zero.
    ZeroRequestId,
    /// Payload length exceeds the locked bound.
    PayloadTooLarge,
    /// Payload could not be read completely.
    TruncatedPayload,
    /// An underlying read failed for a reason other than clean truncation.
    ReadIo,
    /// An underlying write failed.
    WriteIo,
}

impl fmt::Display for ControlFrameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TruncatedHeader => "truncated control frame header",
            Self::InvalidMagic => "invalid control frame magic",
            Self::UnsupportedVersion => "unsupported control protocol version",
            Self::UnknownKind => "unknown control frame kind",
            Self::NonZeroFlags => "control frame flags must be zero",
            Self::ZeroRequestId => "control frame request id must be non-zero",
            Self::PayloadTooLarge => "control frame payload too large",
            Self::TruncatedPayload => "truncated control frame payload",
            Self::ReadIo => "control frame read failed",
            Self::WriteIo => "control frame write failed",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ControlFrameError {}

/// Writes one exact Phase 129 frame to a byte stream.
///
/// # Errors
///
/// Returns [`ControlFrameError`] if validation or writing fails.
pub fn write_control_frame<W: Write>(
    writer: &mut W,
    frame: &ControlFrame,
) -> Result<(), ControlFrameError> {
    if frame.request_id == 0 {
        return Err(ControlFrameError::ZeroRequestId);
    }
    if frame.payload.len() > MAX_CONTROL_PAYLOAD_BYTES {
        return Err(ControlFrameError::PayloadTooLarge);
    }
    let payload_len =
        u32::try_from(frame.payload.len()).map_err(|_| ControlFrameError::PayloadTooLarge)?;
    let mut header = [0_u8; CONTROL_FRAME_HEADER_LEN];
    header[0..4].copy_from_slice(&CONTROL_FRAME_MAGIC);
    header[4..6].copy_from_slice(&CONTROL_PROTOCOL_MAJOR.to_be_bytes());
    header[6..8].copy_from_slice(&CONTROL_PROTOCOL_MINOR.to_be_bytes());
    header[8..10].copy_from_slice(&(frame.kind as u16).to_be_bytes());
    header[10..12].copy_from_slice(&0_u16.to_be_bytes());
    header[12..20].copy_from_slice(&frame.request_id.to_be_bytes());
    header[20..24].copy_from_slice(&payload_len.to_be_bytes());
    writer
        .write_all(&header)
        .map_err(|_| ControlFrameError::WriteIo)?;
    writer
        .write_all(&frame.payload)
        .map_err(|_| ControlFrameError::WriteIo)
}

/// Reads and validates one exact Phase 129 frame from a byte stream.
///
/// Header validation occurs before payload allocation.
///
/// # Errors
///
/// Returns [`ControlFrameError`] for malformed, truncated, oversized, or failed input.
pub fn read_control_frame<R: Read>(reader: &mut R) -> Result<ControlFrame, ControlFrameError> {
    let mut header = [0_u8; CONTROL_FRAME_HEADER_LEN];
    read_exact_classified(reader, &mut header, true)?;
    if header[0..4] != CONTROL_FRAME_MAGIC {
        return Err(ControlFrameError::InvalidMagic);
    }
    let major = u16::from_be_bytes([header[4], header[5]]);
    let minor = u16::from_be_bytes([header[6], header[7]]);
    if major != CONTROL_PROTOCOL_MAJOR || minor != CONTROL_PROTOCOL_MINOR {
        return Err(ControlFrameError::UnsupportedVersion);
    }
    let kind_code = u16::from_be_bytes([header[8], header[9]]);
    let kind = ControlMessageKind::from_code(kind_code).ok_or(ControlFrameError::UnknownKind)?;
    let flags = u16::from_be_bytes([header[10], header[11]]);
    if flags != 0 {
        return Err(ControlFrameError::NonZeroFlags);
    }
    let request_id = u64::from_be_bytes(
        header[12..20]
            .try_into()
            .expect("fixed eight-byte request identifier slice"),
    );
    if request_id == 0 {
        return Err(ControlFrameError::ZeroRequestId);
    }
    let payload_len = u32::from_be_bytes(
        header[20..24]
            .try_into()
            .expect("fixed four-byte payload length slice"),
    ) as usize;
    if payload_len > MAX_CONTROL_PAYLOAD_BYTES {
        return Err(ControlFrameError::PayloadTooLarge);
    }
    let mut payload = vec![0_u8; payload_len];
    if payload_len != 0 {
        read_exact_classified(reader, &mut payload, false)?;
    }
    Ok(ControlFrame {
        kind,
        request_id,
        payload,
    })
}

fn read_exact_classified<R: Read>(
    reader: &mut R,
    target: &mut [u8],
    header: bool,
) -> Result<(), ControlFrameError> {
    let mut filled = 0;
    while filled < target.len() {
        match reader.read(&mut target[filled..]) {
            Ok(0) => {
                return Err(if header {
                    ControlFrameError::TruncatedHeader
                } else {
                    ControlFrameError::TruncatedPayload
                });
            }
            Ok(count) => filled += count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return Err(ControlFrameError::ReadIo),
        }
    }
    Ok(())
}

/// Bounded one-attempt outbound TLS control transport configuration.
pub struct ControlTlsClientConfig {
    remote_addr: SocketAddr,
    server_name: ServerName<'static>,
    tls_config: Arc<ClientConfig>,
    connect_timeout: Duration,
    read_timeout: Duration,
    write_timeout: Duration,
}

impl fmt::Debug for ControlTlsClientConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ControlTlsClientConfig")
            .field("remote_addr", &self.remote_addr)
            .field("server_name", &self.server_name)
            .field("connect_timeout", &self.connect_timeout)
            .field("read_timeout", &self.read_timeout)
            .field("write_timeout", &self.write_timeout)
            .finish_non_exhaustive()
    }
}

/// Stable outbound TLS transport failure classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlTransportError {
    /// No trust anchor, too many anchors, or an anchor exceeded the byte bound.
    TrustAnchorBounds,
    /// A supplied trust anchor was not a valid certificate for the rustls store.
    InvalidTrustAnchor,
    /// TLS server identity could not be represented as a valid `ServerName`.
    InvalidServerName,
    /// One or more configured timeouts were zero or exceeded the bound.
    InvalidTimeout,
    /// TLS 1.3 configuration could not be constructed with the locked provider.
    TlsConfiguration,
    /// TCP connection failed within the bounded attempt.
    TcpConnect,
    /// TCP socket options could not be applied.
    SocketConfiguration,
    /// rustls client connection construction failed.
    TlsConnectionConstruction,
    /// TLS handshake failed, including normal certificate verification failure.
    TlsHandshake,
    /// Negotiated protocol was not TLS 1.3.
    WrongTlsVersion,
    /// Negotiated ALPN was absent or not exactly the PRW control ALPN.
    WrongAlpn,
}

impl fmt::Display for ControlTransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::TrustAnchorBounds => "control transport trust-anchor bounds rejected",
            Self::InvalidTrustAnchor => "invalid control transport trust anchor",
            Self::InvalidServerName => "invalid control transport server name",
            Self::InvalidTimeout => "invalid control transport timeout",
            Self::TlsConfiguration => "control transport TLS configuration failed",
            Self::TcpConnect => "control transport TCP connect failed",
            Self::SocketConfiguration => "control transport socket configuration failed",
            Self::TlsConnectionConstruction => "control transport TLS connection construction failed",
            Self::TlsHandshake => "control transport TLS handshake failed",
            Self::WrongTlsVersion => "control transport negotiated wrong TLS version",
            Self::WrongAlpn => "control transport negotiated wrong ALPN",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ControlTransportError {}

impl ControlTlsClientConfig {
    /// Builds a TLS 1.3-only outbound client config from explicit trust anchors.
    ///
    /// `remote_addr` is used directly for TCP and is deliberately separate from the
    /// expected TLS server identity, so DNS resolution is not required here.
    ///
    /// # Errors
    ///
    /// Rejects invalid trust roots, server names, time bounds, or TLS configuration.
    pub fn new(
        remote_addr: SocketAddr,
        server_name: impl Into<String>,
        trust_anchors_der: &[Vec<u8>],
        connect_timeout: Duration,
        read_timeout: Duration,
        write_timeout: Duration,
    ) -> Result<Self, ControlTransportError> {
        validate_timeout(connect_timeout)?;
        validate_timeout(read_timeout)?;
        validate_timeout(write_timeout)?;
        if trust_anchors_der.is_empty() || trust_anchors_der.len() > MAX_CONTROL_TRUST_ANCHORS {
            return Err(ControlTransportError::TrustAnchorBounds);
        }
        let mut roots = RootCertStore::empty();
        for anchor in trust_anchors_der {
            if anchor.is_empty() || anchor.len() > MAX_CONTROL_TRUST_ANCHOR_BYTES {
                return Err(ControlTransportError::TrustAnchorBounds);
            }
            roots
                .add(CertificateDer::from(anchor.clone()))
                .map_err(|_| ControlTransportError::InvalidTrustAnchor)?;
        }

        let server_name = ServerName::try_from(server_name.into())
            .map_err(|_| ControlTransportError::InvalidServerName)?;
        let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
        let builder = ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|_| ControlTransportError::TlsConfiguration)?;
        let mut tls_config = builder.with_root_certificates(roots).with_no_client_auth();
        tls_config.alpn_protocols = vec![CONTROL_ALPN.to_vec()];
        tls_config.enable_early_data = false;

        Ok(Self {
            remote_addr,
            server_name,
            tls_config: Arc::new(tls_config),
            connect_timeout,
            read_timeout,
            write_timeout,
        })
    }

    /// Performs one bounded outbound TCP/TLS connection attempt.
    ///
    /// A returned stream has completed TLS certificate verification, negotiated
    /// TLS 1.3, and negotiated exactly the Phase 129 ALPN identifier.
    ///
    /// # Errors
    ///
    /// Returns [`ControlTransportError`] for TCP, socket, TLS, version, or ALPN failure.
    pub fn connect(&self) -> Result<ControlTlsStream, ControlTransportError> {
        let mut socket = TcpStream::connect_timeout(&self.remote_addr, self.connect_timeout)
            .map_err(|_| ControlTransportError::TcpConnect)?;
        socket
            .set_read_timeout(Some(self.read_timeout))
            .map_err(|_| ControlTransportError::SocketConfiguration)?;
        socket
            .set_write_timeout(Some(self.write_timeout))
            .map_err(|_| ControlTransportError::SocketConfiguration)?;
        socket
            .set_nodelay(true)
            .map_err(|_| ControlTransportError::SocketConfiguration)?;

        let mut connection = ClientConnection::new(self.tls_config.clone(), self.server_name.clone())
            .map_err(|_| ControlTransportError::TlsConnectionConstruction)?;
        while connection.is_handshaking() {
            connection
                .complete_io(&mut socket)
                .map_err(|_| ControlTransportError::TlsHandshake)?;
        }
        if connection.protocol_version() != Some(ProtocolVersion::TLSv1_3) {
            return Err(ControlTransportError::WrongTlsVersion);
        }
        if connection.alpn_protocol() != Some(CONTROL_ALPN) {
            return Err(ControlTransportError::WrongAlpn);
        }

        Ok(ControlTlsStream {
            inner: StreamOwned::new(connection, socket),
        })
    }
}

fn validate_timeout(timeout: Duration) -> Result<(), ControlTransportError> {
    if timeout.is_zero() || timeout > MAX_CONTROL_TIMEOUT {
        return Err(ControlTransportError::InvalidTimeout);
    }
    Ok(())
}

/// Established server-authenticated TLS 1.3 PRW control transport.
pub struct ControlTlsStream {
    inner: StreamOwned<ClientConnection, TcpStream>,
}

impl fmt::Debug for ControlTlsStream {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ControlTlsStream")
            .finish_non_exhaustive()
    }
}

impl ControlTlsStream {
    /// Writes one bounded control frame over the established TLS stream.
    ///
    /// # Errors
    ///
    /// Returns [`ControlFrameError`] when frame validation or encrypted I/O fails.
    pub fn write_frame(&mut self, frame: &ControlFrame) -> Result<(), ControlFrameError> {
        write_control_frame(&mut self.inner, frame)?;
        self.inner.flush().map_err(|_| ControlFrameError::WriteIo)
    }

    /// Reads one bounded control frame from the established TLS stream.
    ///
    /// # Errors
    ///
    /// Returns [`ControlFrameError`] for malformed/truncated input or encrypted I/O failure.
    pub fn read_frame(&mut self) -> Result<ControlFrame, ControlFrameError> {
        read_control_frame(&mut self.inner)
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, net::SocketAddr, time::Duration};

    use super::{
        CONTROL_FRAME_HEADER_LEN, CONTROL_FRAME_MAGIC, CONTROL_PROTOCOL_MAJOR,
        CONTROL_PROTOCOL_MINOR, ControlFrame, ControlFrameError, ControlMessageKind,
        ControlTlsClientConfig, ControlTransportError, MAX_CONTROL_PAYLOAD_BYTES,
        read_control_frame, write_control_frame,
    };

    #[test]
    fn frame_round_trip_preserves_exact_fields() {
        let frame = ControlFrame::new(ControlMessageKind::Authentication, 42, b"proof".to_vec())
            .expect("valid frame");
        let mut bytes = Vec::new();
        write_control_frame(&mut bytes, &frame).expect("encode frame");
        let decoded = read_control_frame(&mut Cursor::new(bytes)).expect("decode frame");
        assert_eq!(decoded, frame);
    }

    #[test]
    fn locked_header_bytes_are_exact() {
        let frame = ControlFrame::new(ControlMessageKind::Heartbeat, 0x0102_0304_0506_0708, vec![])
            .expect("valid frame");
        let mut bytes = Vec::new();
        write_control_frame(&mut bytes, &frame).expect("encode frame");
        assert_eq!(bytes.len(), CONTROL_FRAME_HEADER_LEN);
        assert_eq!(&bytes[0..4], &CONTROL_FRAME_MAGIC);
        assert_eq!(&bytes[4..6], &CONTROL_PROTOCOL_MAJOR.to_be_bytes());
        assert_eq!(&bytes[6..8], &CONTROL_PROTOCOL_MINOR.to_be_bytes());
        assert_eq!(&bytes[8..10], &(ControlMessageKind::Heartbeat as u16).to_be_bytes());
        assert_eq!(&bytes[10..12], &[0, 0]);
        assert_eq!(&bytes[12..20], &0x0102_0304_0506_0708_u64.to_be_bytes());
        assert_eq!(&bytes[20..24], &[0, 0, 0, 0]);
    }

    #[test]
    fn constructor_rejects_zero_request_id_and_oversize_payload() {
        assert_eq!(
            ControlFrame::new(ControlMessageKind::Command, 0, vec![]),
            Err(ControlFrameError::ZeroRequestId)
        );
        assert_eq!(
            ControlFrame::new(
                ControlMessageKind::Command,
                1,
                vec![0; MAX_CONTROL_PAYLOAD_BYTES + 1]
            ),
            Err(ControlFrameError::PayloadTooLarge)
        );
    }

    #[test]
    fn decoder_rejects_each_locked_header_violation() {
        let base = ControlFrame::new(ControlMessageKind::Response, 7, vec![]).expect("frame");
        let mut bytes = Vec::new();
        write_control_frame(&mut bytes, &base).expect("encode");

        let mut invalid = bytes.clone();
        invalid[0] = b'X';
        assert_eq!(
            read_control_frame(&mut Cursor::new(invalid)),
            Err(ControlFrameError::InvalidMagic)
        );

        let mut invalid = bytes.clone();
        invalid[5] = 2;
        assert_eq!(
            read_control_frame(&mut Cursor::new(invalid)),
            Err(ControlFrameError::UnsupportedVersion)
        );

        let mut invalid = bytes.clone();
        invalid[8..10].copy_from_slice(&99_u16.to_be_bytes());
        assert_eq!(
            read_control_frame(&mut Cursor::new(invalid)),
            Err(ControlFrameError::UnknownKind)
        );

        let mut invalid = bytes.clone();
        invalid[10..12].copy_from_slice(&1_u16.to_be_bytes());
        assert_eq!(
            read_control_frame(&mut Cursor::new(invalid)),
            Err(ControlFrameError::NonZeroFlags)
        );

        let mut invalid = bytes.clone();
        invalid[12..20].copy_from_slice(&0_u64.to_be_bytes());
        assert_eq!(
            read_control_frame(&mut Cursor::new(invalid)),
            Err(ControlFrameError::ZeroRequestId)
        );

        let mut invalid = bytes;
        invalid[20..24].copy_from_slice(&((MAX_CONTROL_PAYLOAD_BYTES as u32) + 1).to_be_bytes());
        assert_eq!(
            read_control_frame(&mut Cursor::new(invalid)),
            Err(ControlFrameError::PayloadTooLarge)
        );
    }

    #[test]
    fn decoder_rejects_header_and_payload_truncation() {
        assert_eq!(
            read_control_frame(&mut Cursor::new(vec![0_u8; CONTROL_FRAME_HEADER_LEN - 1])),
            Err(ControlFrameError::TruncatedHeader)
        );

        let frame = ControlFrame::new(ControlMessageKind::Event, 9, vec![1, 2, 3]).expect("frame");
        let mut bytes = Vec::new();
        write_control_frame(&mut bytes, &frame).expect("encode");
        bytes.pop();
        assert_eq!(
            read_control_frame(&mut Cursor::new(bytes)),
            Err(ControlFrameError::TruncatedPayload)
        );
    }

    #[test]
    fn tls_config_rejects_empty_roots_invalid_server_name_and_timeout() {
        let addr: SocketAddr = "127.0.0.1:443".parse().expect("socket addr");
        assert!(matches!(
            ControlTlsClientConfig::new(
                addr,
                "control.test",
                &[],
                Duration::from_secs(1),
                Duration::from_secs(1),
                Duration::from_secs(1)
            ),
            Err(ControlTransportError::TrustAnchorBounds)
        ));
        assert!(matches!(
            ControlTlsClientConfig::new(
                addr,
                "not a valid server name",
                &[vec![1, 2, 3]],
                Duration::from_secs(1),
                Duration::from_secs(1),
                Duration::from_secs(1)
            ),
            Err(ControlTransportError::InvalidTrustAnchor)
                | Err(ControlTransportError::InvalidServerName)
        ));
        assert!(matches!(
            ControlTlsClientConfig::new(
                addr,
                "control.test",
                &[vec![1, 2, 3]],
                Duration::ZERO,
                Duration::from_secs(1),
                Duration::from_secs(1)
            ),
            Err(ControlTransportError::InvalidTimeout)
        ));
    }
}
