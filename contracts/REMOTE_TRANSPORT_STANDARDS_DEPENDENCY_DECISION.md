# Private Remote Workspace — Remote Transport Standards and Dependency Decision

Status: Phase 139 architecture decision lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Architecture approval: explicit user approval received for Phase 139

## Purpose

Phase 139 locks the standards, dependency ownership, protocol versioning, framing limits, peer identity binding, and transport credential lifecycle for the PRW remote data plane.

This is a decision/source-readiness phase only. It does not activate a socket, publish a production endpoint, provision a production transport key/certificate, modify PowerCode networking, change firewall/routes/DNS, or deploy a relay.

## Existing boundary preserved

The Phase 129 host control-plane transport remains a separate outbound TCP/TLS 1.3 channel using `prw-control/1`.

The Phase 140+ remote **data plane** is a separate transport and MUST NOT reinterpret a successful transport handshake as application authorization.

The following identities remain distinct:

1. user/account identity;
2. durable PRW `DeviceId` and its Phase 126 device-identity proof key;
3. network/transport identity and transport credentials;
4. authenticated application session identity from Phase 128.

Transport-key rotation MUST NOT replace the durable `DeviceId`.

## Data-plane standards lock

The initial PRW data-plane transport is:

- QUIC version 1 as specified by RFC 9000;
- TLS 1.3 integration for QUIC as specified by RFC 9001 and RFC 8446;
- UDP as required by QUIC;
- mutual X.509 certificate authentication using a private PRW transport trust hierarchy;
- application ALPN exactly `prw-data/1`;
- reliable QUIC bidirectional streams for the initial application protocol.

The initial profile explicitly disables or defers:

- QUIC version 2;
- TLS versions below 1.3;
- TLS/QUIC 0-RTT application data;
- QUIC DATAGRAM extension;
- HTTP/3;
- WebTransport;
- plaintext fallback;
- public OS/WebPKI trust-store fallback;
- custom certificate-verification bypasses.

A later versioned architecture change may add QUIC v2 or QUIC DATAGRAM only after compatibility, downgrade, replay, and capability-boundary validation.

## Exact Rust dependency lock

Phase 140 will introduce one transport-owner crate, provisionally named `prw-data-transport`, with exact direct dependencies:

```toml
quinn = { version = "=0.11.11", default-features = false, features = ["runtime-tokio", "rustls-aws-lc-rs"] }
rustls = { version = "=0.23.43", default-features = false, features = ["std", "aws_lc_rs"] }
tokio = { version = "=1.53.1", default-features = false, features = ["rt", "macros", "net", "time", "sync", "io-util"] }
```

The Quinn platform verifier is not enabled. Trust roots are explicit PRW inputs.

The AWS-LC rustls provider is retained to align with the existing PRW audited crypto provider choice. The initial data transport does not introduce a second TLS crypto provider.

Phase 139 disposable dependency probe run `31949211214` compiled this exact direct dependency set under Rust/Cargo `1.97.1` without mutating the repository graph. Scratch `Cargo.lock` SHA-256 was:

`b1a4be15f6036967971334f8ac7536c0a987e10aff0f38bdc49ca8286743c332`

That scratch lockfile is evidence only and MUST NOT replace the repository `Cargo.lock`. Phase 140 must regenerate and validate the real workspace lockfile transactionally.

## Dependency ownership

Transport implementation dependencies are intentionally contained:

- `prw-data-transport` owns Quinn, Tokio, rustls QUIC configuration, endpoint/socket lifecycle, stream I/O, transport timeouts, cancellation, and transport error mapping;
- `prw-connectivity` remains a pure candidate/observation/selection model and MUST NOT acquire Quinn/Tokio/socket ownership;
- `prw-relay` remains an opaque fallback-policy boundary and MUST NOT become a TLS plaintext termination point;
- file, terminal, forwarding, registry, policy, and session crates MUST NOT depend directly on Quinn or Tokio merely to use remote transport;
- the Agent/capability bridge will consume a narrow typed transport interface in Phase 143 rather than exposing raw Quinn objects to capability code.

## Peer certificate and trust profile

Initial transport certificates are private-PRW X.509 certificates using ECDSA P-256 signing/key material compatible with the locked AWS-LC provider.

Both QUIC peers MUST present a certificate. Both sides MUST validate the peer chain against explicitly provisioned PRW transport roots/intermediates. The OS public root store is not an implicit trust source.

Each issued transport certificate also carries a PRW-assigned synthetic DNS SAN of the form:

`t-<32-lowercase-hex>.prw.invalid`

The 128-bit hex component is an issuance-time transport endpoint name, not a `DeviceId` and not a DNS-resolution dependency. The `.invalid` suffix is intentionally non-routable. The expected name is supplied from authenticated coordination metadata and is used for certificate-name verification/SNI without requiring DNS lookup.

## Transport identity binding

Normal TLS chain/name verification is necessary but not sufficient.

After a TLS handshake, the exact end-entity certificate DER is hashed with SHA-256 using the existing audited provider. The resulting 32-byte `TransportCredentialFingerprint` is compared against current authenticated registry/control-plane metadata that binds:

- transport endpoint name;
- transport credential fingerprint;
- PRW transport identity;
- durable `DeviceId`;
- workspace membership/lifecycle state;
- credential validity/revocation state.

A certificate that validates cryptographically but is not currently bound to the expected enrolled device/workspace fails closed.

TLS transport authentication MUST NOT by itself grant files, terminal, forwarding, enrollment, policy, or management capabilities. Phase 128 application-session authentication and Phase 130 current-state registry revalidation remain required before protected application operations.

## Transport key and certificate lifecycle

Transport private keys are separate from the Phase 126 durable device-identity private key.

Rules:

- generate transport private keys on-device whenever technically possible;
- never upload a transport private key to the control plane as normal operation;
- store transport keys through platform-specific secure custody selected for that client/Agent platform;
- certificate issuance occurs only after current enrolled-device authorization;
- initial production certificate validity MUST NOT exceed 30 days;
- renewal creates a newly auditable credential record;
- controlled rotation may temporarily allow at most two active transport credential fingerprints for one device;
- once the new credential is confirmed usable, the old credential is retired/revoked from current registry metadata;
- device revocation invalidates all associated transport credentials for future authorized sessions;
- transport credential rotation never changes `DeviceId`;
- no certificate or resumed transport state replaces Phase 128 application-session authentication.

Production transport key/certificate provisioning remains forbidden until the Phase 154 production activation gate.

## Application protocol and framing lock

The remote data-plane application protocol has an independent version from QUIC itself.

Initial application protocol version:

- major: `1`;
- minor: `0`;
- ALPN: `prw-data/1`.

Each reliable application message on a QUIC stream uses a fixed 24-byte network-byte-order envelope:

1. magic — 4 bytes: `PRWD`;
2. application protocol major — `u16`, value `1`;
3. application protocol minor — `u16`, value `0`;
4. message kind — `u16`;
5. flags — `u16`, value `0` in the initial profile;
6. request identifier — non-zero big-endian `u64`;
7. payload length — big-endian `u32`.

Initial maximum envelope payload: `2097152` bytes (2 MiB).

The 2 MiB transport bound is a ceiling, not a capability allowance. Tighter existing capability bounds remain authoritative, including terminal I/O and the Phase 132 1 MiB file-transfer chunk limit.

The reader MUST validate the complete header and payload length before allocating the payload. Unknown protocol major, unsupported flags, zero request identifier, unknown/disallowed message kind, oversized payload, and truncation fail closed.

Phase 140 may use disposable probe message kinds to validate the transport. Production capability message-kind allocation is deferred to the Phase 143 authenticated capability bridge.

No arbitrary shell command semantics are introduced by the transport envelope.

## Stream profile

The initial transport uses reliable bidirectional QUIC streams only.

- application framing may carry multiple bounded messages per stream where the capability protocol requires a long-lived session;
- unidirectional application streams are disabled in the initial profile;
- QUIC DATAGRAM is disabled;
- the implementation MUST apply explicit bounded concurrent-stream and buffer limits in Phase 140;
- application backpressure MUST be propagated rather than converted into unbounded buffering.

## NAT traversal standards lock for Phase 141

Phase 141 MUST build on standard NAT-traversal mechanisms rather than a proprietary hole-punching protocol:

- ICE semantics: RFC 8445;
- STUN: RFC 8489;
- TURN relay traversal: RFC 8656.

STUN is a traversal tool, not authentication or authorization. TURN is fallback infrastructure and does not supersede the Phase 136 direct-before-relay policy.

The Phase 135 selection order remains authoritative:

`LocalDirect -> InternetDirect -> Relay -> Offline`

Phase 141 may select a vetted implementation dependency after an exact Rust/toolchain compatibility probe, provided it preserves these standards and boundaries. That implementation choice does not authorize production STUN/TURN traffic.

## Relay security consequence

A relay or TURN-style fallback MUST NOT receive PRW application plaintext merely because packets transit it.

The end-to-end QUIC/TLS application security association is between authenticated PRW peers. Phase 142 relay/provider work must preserve opaque forwarding relative to application payloads and remain subordinate to the Phase 136 fallback-only policy.

## DNS independence

Basic remote connectivity continues to use explicit IP/socket candidates from the connectivity/NAT-traversal plane.

The synthetic certificate server name is not resolved through system DNS. Optional private DNS from Phase 137 remains a usability/naming feature and MUST NOT become a prerequisite for establishing the base data plane.

## Phase 140 required validation consequences

Phase 140 must prove at minimum, in disposable environments:

- exact locked dependency graph and Rust 1.97.1 compatibility;
- QUIC v1 + TLS 1.3 successful loopback connection;
- ALPN exactly `prw-data/1`;
- mutual certificate authentication;
- explicit private trust roots only;
- expected synthetic server-name success and wrong-name failure;
- untrusted CA failure;
- missing/invalid client certificate failure;
- transport credential fingerprint binding success and wrong-binding failure;
- 0-RTT unavailable to application code;
- bounded 24-byte `PRWD` framing and 2 MiB ceiling;
- cancellation/timeout behavior;
- no repository-external production networking or PowerCode mutation.

## Production boundary

Phase 139 authorizes no production transport activation.

Until Phase 154 is explicitly approved, forbidden production actions include transport key/certificate provisioning, new public/LAN data-plane listener activation, persistent STUN/ICE/TURN activity, relay deployment, TUN/route/firewall/DNS mutation, and Agent replacement/restart for this data plane.

## Final decision

`PHASE_139_REMOTE_TRANSPORT_ARCHITECTURE_LOCKED / QUIC_V1_TLS13_MTLS / QUINN_0_11_11_RUSTLS_0_23_43_TOKIO_1_53_1 / EXPLICIT_PRIVATE_TRUST / DEVICE_AND_TRANSPORT_IDENTITY_SEPARATE / ICE_STUN_TURN_STANDARDIZED_FOR_PHASE_141 / NO_PRODUCTION_NETWORK_MUTATION / READY_FOR_PHASE_140_DISPOSABLE_IMPLEMENTATION`
