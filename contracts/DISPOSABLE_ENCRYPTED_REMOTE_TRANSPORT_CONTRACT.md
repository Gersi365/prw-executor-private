# Private Remote Workspace — Disposable Encrypted Remote Transport Contract

Status: Phase 140 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Parent architecture: `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION.md`
Normative clarification: `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION_A01.md`

## Scope

Phase 140 implements the Phase 139 mesh data-plane transport only as source and disposable loopback validation. It introduces a dedicated `prw-remote-transport` crate. It does not integrate the production Agent, publish a production endpoint, perform NAT traversal, deploy relay, alter routing/firewall/DNS, or provision production transport credentials.

## Exact runtime dependencies

The new crate must pin exactly:

- `quinn = 0.11.11`, default features disabled, features `runtime-tokio` and `rustls-aws-lc-rs`;
- `rustls = 0.23.43`, default features disabled, features `std` and `aws_lc_rs`;
- `tokio = 1.53.1`, default features disabled, only runtime/network/time/sync/I/O features required by this crate;
- `aws-lc-rs = 1.18.0` only for the Phase 139 A01 SPKI SHA-256/provider-aligned utility;
- internal `prw-connectivity` for the authoritative `TransportIdentity` type.

Disposable certificate generation may use exactly `rcgen = 0.14.8` as a dev-dependency with AWS-LC-backed crypto. rcgen-generated CA/private-key material is test-only and must never be committed as production credential material.

## Runtime boundary

`prw-remote-transport` owns Quinn/Tokio and QUIC-specific rustls configuration. It must not be imported into `prw-agent` during Phase 140.

Runtime APIs must provide typed operations for:

- deriving `TransportIdentity` from a presented leaf certificate SPKI;
- deriving the deterministic `.mesh.prw.invalid` certificate identity string;
- deterministic initiator selection from two transport identities;
- encoding/decoding the 24-byte PRWM v1.0 control envelope;
- building TLS1.3-only mTLS Quinn client/server configurations from explicit PRW roots and caller-supplied leaf certificate/key material;
- extracting and checking the peer leaf `TransportIdentity` after a fully established Quinn/rustls connection.

No generic TLS verifier, dangerous certificate bypass, OS/public roots, private-key generation/import/export helper, generic signing, shell execution, DNS lookup, TUN/TAP, firewall or route API may be exposed.

## Protocol constants

- QUIC version: v1 only;
- TLS: 1.3 only;
- ALPN: `prw-mesh/1`;
- PRWM major/minor: `1.0`;
- control header: 24 bytes;
- maximum control payload: 65,536 bytes;
- maximum remotely initiated bidirectional streams: 32;
- maximum remotely initiated unidirectional streams: 16;
- idle timeout: finite, initially 30 seconds;
- connect/accept/read/write test-operation timeout: finite, initially 5 seconds for disposable validation.

Initial control kinds remain session-auth, request, response, event, heartbeat and error. Flags are zero and request identifiers are non-zero.

## TLS/QUIC requirements

Client and server configurations must:

- use the explicit AWS-LC rustls provider;
- enable TLS1.3 only;
- negotiate only `prw-mesh/1`;
- trust only caller-provided PRW root certificates;
- require mTLS;
- use normal rustls/WebPKI verification;
- disable client resumption;
- disable early data;
- set server TLS1.3 ticket count to zero;
- set server early-data size to zero;
- use Quinn reliable streams for the Phase 140 application profile;
- cap remote stream counts to 32 bidirectional and 16 unidirectional;
- use finite idle timeout and bounded receive windows appropriate to the 64 KiB control-frame profile.

Runtime source must not call `Connecting::into_0rtt`, rustls `dangerous()`, public/platform verifier constructors, datagram send/receive APIs, DNS resolver APIs or shell/process execution.

## Peer identity

For a fully established Quinn connection using the rustls session, `Connection::peer_identity()` must downcast to the rustls certificate chain. The leaf certificate is the first certificate. Its SPKI SHA-256 is converted through the existing `prw_connectivity::TransportIdentity::new` boundary.

A connection-facing helper must compare the derived peer identity against the expected registry-supplied transport identity and fail closed on missing identity, wrong dynamic type, empty chain, malformed certificate or mismatch.

mTLS success alone remains insufficient for PRW capability authorization.

## Disposable certificate fixtures

Phase 140 tests may generate a private CA and independent peer P-256 leaf keys/certificates at runtime with rcgen. Each leaf must:

- carry the transport-identity-derived DNS SAN;
- include `serverAuth` and `clientAuth` EKUs;
- chain to the disposable private CA;
- use an independent transport key.

The test suite may compute transport identity from a provisional leaf certificate, then issue the final leaf with the derived SAN using the same leaf key so SPKI identity remains stable.

## Required disposable validation

The Phase 140 authoritative validation must prove:

1. exact dependency graph and Rust 1.97.1 compatibility;
2. successful loopback QUIC v1/TLS1.3 mTLS connection using `prw-mesh/1`;
3. client verifies the peer-name derived from expected TransportIdentity;
4. both peers expose and derive the expected peer TransportIdentity after handshake;
5. wrong CA fails;
6. wrong server name fails;
7. missing client certificate fails;
8. expected-identity mismatch fails closed at the PRW binding layer;
9. negotiated ALPN is exactly `prw-mesh/1`;
10. deterministic initiator selection works and equal identities fail;
11. PRWM frame round-trip works and malformed magic/version/kind/flags/request-id/length/truncation fail;
12. stream and idle/buffer bounds are configured;
13. no `into_0rtt`, `dangerous()`, platform roots, QUIC DATAGRAM application API, DNS lookup, shell or production-network path appears in runtime source;
14. full workspace rustfmt, Clippy `-D warnings`, tests and build pass under the locked graph;
15. no PowerCode or other production network/credential/runtime state is mutated.

## Deferred

Phase 141 owns ICE/STUN/TURN candidate discovery/traversal. Phase 142 owns disposable relay service/provider. Phase 143 owns authenticated application capability bridging and specialized terminal/file/forwarding stream schemas.

## Production boundary

Phase 140 is disposable only. No production UDP listener, transport certificate/key, Agent integration/restart, public endpoint, firewall/NAT/router/TUN/TAP/route mutation, DNS change, STUN/ICE/TURN production traffic, relay deployment, Android/Desktop signing or real user-visible remote networking is authorized.

Final target state: `PHASE_140_IMPLEMENTED_AND_DISPOSABLY_VALIDATED / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_141`.
