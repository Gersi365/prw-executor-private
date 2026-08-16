# Private Remote Workspace — Remote Transport Architecture Decision

Status: Phase 139 architecture decision lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Decision baseline: `7b430b1f54caf26778d59d97cac754ff1e2edd43`

## Purpose

Phase 139 selects the standard transport/security building blocks for the PRW remote data plane and locks the boundaries that Phase 140 may implement in disposable environments. This phase does not activate a production endpoint, publish a listener, create a TUN/TAP device, alter firewall/routes/DNS, replace or restart the production Agent, or provision production transport credentials.

The existing Phase 129 control-plane transport remains valid and separate. Phase 139 does not replace it.

## Decision summary

PRW will use two distinct remote transport planes:

1. **Control plane** — retain the Phase 129 outbound TCP + TLS 1.3 transport with ALPN `prw-control/1` for signaling, enrollment/session orchestration, registry state, candidate exchange and control-plane coordination.
2. **Data plane** — use QUIC version 1 over UDP with TLS 1.3, mutual certificate authentication, ALPN `prw-mesh/1`, explicit PRW trust roots and reliable QUIC streams.

The data plane is not WebRTC DataChannel, a generic WebSocket tunnel, a custom encrypted UDP protocol, or a plaintext overlay. PRW does not define new cryptographic primitives.

## Standards lock

The initial remote data plane uses:

- QUIC version 1 — RFC 9000;
- QUIC TLS integration — RFC 9001;
- TLS 1.3 through rustls;
- ICE — RFC 8445 — for Phase 141 candidate connectivity orchestration;
- STUN — RFC 8489 — for Phase 141 server-reflexive candidate discovery;
- TURN — RFC 8656 — for Phase 141/142 UDP relay fallback where direct connectivity is unavailable.

QUIC version 2 is not enabled in the initial profile. A future protocol-version migration requires a separate compatibility decision and explicit interoperability validation.

## Exact initial Rust runtime dependency lock

Phase 140 may introduce a dedicated remote-transport crate using exactly:

```toml
quinn = { version = "=0.11.11", default-features = false, features = ["runtime-tokio", "rustls-aws-lc-rs"] }
rustls = { version = "=0.23.43", default-features = false, features = ["std", "aws_lc_rs"] }
tokio = { version = "=1.53.1", default-features = false, features = ["rt", "macros", "net", "time", "sync", "io-util"] }
```

Rules:

- no Quinn default feature set;
- no `platform-verifier` feature;
- no `rustls-ring` provider feature;
- AWS-LC remains the selected rustls provider, consistent with existing PRW cryptographic provider direction;
- direct dependency versions are exact, while all transitives remain fixed by the authoritative Cargo lockfile;
- Phase 140 must not silently substitute versions if the exact set stops resolving or compiling.

A Phase 139 disposable scratch probe on Rust/Cargo 1.97.1 compiled this exact direct dependency set successfully. The scratch `Cargo.lock` SHA-256 was `b1a4be15f6036967971334f8ac7536c0a987e10aff0f38bdc49ca8286743c332`.

## Dependency ownership

Phase 140 should introduce a dedicated transport component, conceptually `prw-remote-transport`, that owns Quinn/Tokio and the QUIC-specific rustls configuration.

The existing `prw-control-transport` crate remains the Phase 129 control-plane transport and must not be converted into the mesh data-plane implementation.

The Agent must not gain ad hoc Quinn/Tokio usage in unrelated modules. Agent integration with the new transport remains a later authenticated capability-bridge concern.

Exact ICE/STUN/TURN implementation dependencies are deliberately **not** pinned by Phase 139. The standards are locked here; Phase 141 must perform a separate current-library compatibility/security probe before selecting exact ICE/STUN/TURN crate versions. PRW must not implement ICE, STUN or TURN cryptographic/authentication machinery from scratch.

## TLS and QUIC security profile

The Phase 140 disposable implementation must enforce:

- QUIC v1 only;
- TLS 1.3 only;
- ALPN exactly `prw-mesh/1`;
- no plaintext fallback;
- no TLS certificate-validation bypass;
- no rustls `dangerous()` verifier;
- no public/OS trust-store fallback;
- explicit PRW transport trust roots;
- mutual TLS on every mesh QUIC connection;
- no TLS 0-RTT / no Quinn `into_0rtt` path;
- TLS session resumption disabled in the initial profile;
- no application use of QUIC DATAGRAM frames in the initial profile;
- reliable QUIC streams only for the initial application profile;
- no active QUIC connection-migration policy in Phase 140; Phase 141 candidate changes may establish a new authenticated connection instead.

Successful QUIC/TLS establishment proves a transport peer identity. It does **not** grant workspace membership, application session identity, file access, terminal access, forwarding rights or any other PRW capability.

## Transport identity and key separation

PRW preserves three independent identities:

1. `DeviceId` — logical enrolled-device identity;
2. Phase 126 device identity key — long-lived device proof/enrollment identity;
3. `TransportIdentity` — identity of the mesh transport certificate/key.

The Phase 126 device-identity private key MUST NOT be reused as the QUIC/TLS transport private key.

Each participating device receives a separate transport keypair and short-lived X.509 leaf certificate. Initial key algorithm: ECDSA P-256 using audited provider functionality already accepted by PRW. The private transport key is generated locally and remains non-exported under platform custody appropriate to that client/host.

`TransportIdentity` is defined as:

`SHA-256(canonical DER SubjectPublicKeyInfo of the transport leaf public key)`

and is exactly 32 bytes.

The device registry binds the current `DeviceId` to the expected `TransportIdentity` and certificate lifecycle metadata. Revocation/current-state checks remain authoritative; certificate validity alone is insufficient authorization.

## Certificate trust and peer-name profile

Transport certificates chain only to explicitly provisioned PRW private transport trust roots. Public Web PKI roots and the operating-system trust store are not implicit trust inputs.

A transport leaf certificate must include both `clientAuth` and `serverAuth` extended-key usages because either peer may act as the QUIC connection initiator after path selection.

To preserve normal certificate hostname verification without making endpoint discovery depend on DNS, the TLS DNS SAN/SNI name is deterministically derived from the full lowercase hexadecimal `TransportIdentity`:

`t-<first-32-hex>.<last-32-hex>.mesh.prw.invalid`

The `.invalid` namespace is used only as a certificate identity string. It MUST NOT be resolved to discover the peer endpoint. The actual QUIC destination comes from an explicit selected IP/UDP candidate.

Client-side server authentication uses normal rustls/WebPKI certificate-chain and server-name verification against the expected transport identity-derived name. Server-side client authentication requires a valid PRW client certificate chain; after the handshake, PRW derives the presented client `TransportIdentity` from SPKI and revalidates it against current registry state before allowing application-session establishment.

No custom verifier may accept a certificate merely because its embedded `DeviceId` or other application metadata looks plausible.

## Certificate lifecycle

Initial transport leaf certificate policy:

- maximum validity: 30 days;
- renewal should begin when 10 days or less remain;
- expiry is fail-closed for new connections;
- registry/device revocation overrides unexpired certificate validity;
- transport CA private signing keys are never stored on the PRW Agent or clients;
- certificate issuance/renewal must be authorized by an enrolled current device using existing PRW device-identity/session proof boundaries;
- transport-key rotation produces a new `TransportIdentity` and requires an atomic registry update with old-identity revocation/retirement semantics.

Phase 139 locks the lifecycle semantics, not a production certificate-authority implementation library. Disposable Phase 140 certificate fixtures/generation must not become production CA material.

## Deterministic peer connection role

After Phase 141 selects an explicit usable candidate pair, both peers may own UDP-capable QUIC endpoints, but only one should initiate the application QUIC connection for the same peer pair.

The deterministic initiator is the peer with the lexicographically smaller 32-byte `TransportIdentity`. This rule prevents duplicate simultaneous application connections while remaining independent of `DeviceId`, DNS names and endpoint addresses.

If the expected peer transport identity is missing or equal to the local identity, connection establishment fails closed.

## Initial mesh application framing

QUIC provides transport framing and encryption, but PRW still requires a bounded, versioned application envelope for the reserved mesh control stream.

The deterministic initiator opens the first application bidirectional control stream. Its initial fixed header is 24 bytes, network byte order:

1. magic — 4 bytes: `PRWM`;
2. protocol major — `u16`, value `1`;
3. protocol minor — `u16`, value `0`;
4. message kind — `u16`;
5. flags — `u16`, value `0` initially;
6. request identifier — `u64`, non-zero;
7. payload length — `u32`.

Initial message-kind registry:

- `1` — session authentication;
- `2` — request;
- `3` — response;
- `4` — event;
- `5` — heartbeat;
- `6` — error.

Initial maximum control payload: 65,536 bytes.

The decoder validates the complete header before allocation and rejects invalid magic/version/kind/flags/request-id, oversize length and truncation. Unknown major versions fail closed. A future minor version may only add backward-compatible semantics explicitly defined by contract.

Phase 139 does not lock bulk file, terminal byte-stream or forwarding stream schemas. Those belong to the Phase 143 authenticated capability bridge.

Initial per-connection application limits for Phase 140 validation:

- at most 32 remotely initiated bidirectional streams;
- at most 16 remotely initiated unidirectional streams;
- explicit bounded handshake/idle/read/write/cancellation behavior;
- no unbounded buffering or task creation.

Exact timeout durations may be selected by the Phase 140 implementation contract but must be finite and validated.

## NAT traversal and relay architecture

Phase 141 will gather and validate ICE-style candidates and feed observations into the already-built Phase 135 selector. Candidate signaling occurs through the authenticated control plane; private DNS is not required.

Preference remains:

`LocalDirect -> InternetDirect -> Relay -> Offline`

TURN relay is a UDP path fallback. It carries the same end-to-end QUIC packets used on a direct path; the TURN/PRW relay path does not terminate the peer-to-peer mesh TLS session and therefore is not a plaintext application-data termination point.

Phase 142 may implement a PRW relay service/provider around the Phase 136 opacity contract, but it must preserve this end-to-end encryption property.

## Relationship to application authentication and authorization

Mesh mTLS establishes transport identity only.

Before a remote peer can invoke PRW capabilities:

1. the presented transport identity must map to a current non-revoked registry device;
2. Phase 128-style authenticated application-session proof must complete;
3. Phase 130 current workspace/device membership must revalidate;
4. the specific capability must pass its own policy boundary.

Transport success never implies capability authorization.

## Phase 140 validation requirements

Phase 140 must remain disposable and prove at minimum:

- exact dependency pins and locked graph;
- loopback QUIC v1 connection using TLS 1.3 and ALPN `prw-mesh/1`;
- mutual certificate authentication with explicit private test roots;
- correct peer-name verification using the transport identity-derived SAN;
- transport-identity extraction/binding;
- wrong CA, wrong name, missing client certificate and revoked/mismatched expected identity fail closed;
- 0-RTT/resumption paths are unavailable in the initial profile;
- bounded control-frame round-trip and malformed-frame rejection;
- deterministic initiator selection;
- stream-count/buffer/time bounds;
- no production PowerCode networking or credential mutation.

## Production boundary

Phase 139 authorizes architecture selection only. It does not cross Phase 154 or any equivalent production mutation gate.

Until separately approved at the exact production activation phase, Phase 139/140 work MUST NOT:

- expose a production UDP listener;
- enable a public/LAN data-plane endpoint;
- alter firewall/NAT/router/TUN/TAP/routes;
- perform production ICE/STUN/TURN activation;
- deploy a production relay service;
- provision production transport CA/private credentials;
- change production DNS/resolver state;
- replace/restart the production Agent for the new data plane;
- distribute/sign a real Android/Desktop production client.

## Final decision

`PHASE_139_ARCHITECTURE_LOCKED / QUIC_V1_TLS13_MTLS / QUINN_0_11_11_RUSTLS_0_23_43_TOKIO_1_53_1 / ICE_STUN_TURN_STANDARDS_LOCKED / PRODUCTION_MUTATION_NOT_AUTHORIZED / READY_FOR_PHASE_140_DISPOSABLE_IMPLEMENTATION`
