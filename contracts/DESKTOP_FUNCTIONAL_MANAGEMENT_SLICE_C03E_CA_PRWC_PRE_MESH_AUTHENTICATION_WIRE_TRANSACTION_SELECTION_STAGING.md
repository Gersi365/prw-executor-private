# Phase 152 C03e-CA — PRWC Pre-Mesh Authentication Wire / Transaction Selection

Status: STAGED SELECTION

Gate target:
`C03E_CA_PRWC_PRE_MESH_AUTHENTICATION_WIRE_TRANSACTION_SELECTED`

## 1. Exact predecessor

Closed C03e-BZ is the authoritative predecessor:
- branch: `phase-152-c03e-bz-candidate-publication-prwc-pre-mesh-authentication-requester-rendezvous-authority-selection-staging`;
- head: `cc226a27b2e404024e4ef6fd8ea089ffff33c2d6`;
- tree: `390904282116c0cac38f7f8c30be1e9a4f5e2c0b`;
- gate: `C03E_BZ_CANDIDATE_PUBLICATION_PRWC_PRE_MESH_AUTHENTICATION_REQUESTER_RENDEZVOUS_AUTHORITY_SELECTED`;
- PR `#195`: body `Status: CLOSED`, draft/open/unmerged.

BZ selected connection-local logical-session ownership above Phase 129 TLS and a separate server-side requester/rendezvous authority for candidate publication. BZ explicitly left the PRWC authentication payload schema, transaction sequence and response/error mapping unselected.

## 2. Exact bounded purpose

CA selects only the pure Phase 129 PRWC pre-mesh logical-session authentication protocol shape needed before any source codec or runtime execution may be materialized.

CA selects:
1. a PRWC-specific inner authentication payload namespace distinct from existing PRWM bytes;
2. exact version/header/operation codes and field ordering;
3. exact outer `ControlMessageKind` pairing;
4. one request-ID-correlated Begin → Challenge → Proof → terminal Response/Error transaction;
5. bounded identifier/signature validation and fail-closed decoding rules;
6. the semantic handoff points to existing Phase 128 `SessionAuthenticationService` and current registry authority.

CA is docs-only. It does not implement a codec, session-ID allocator, request-ID allocator, listener, accepted socket, frame loop, session provider, registry mutation, requester/rendezvous provider, candidate publication execution, networking, deployment or merge.

## 3. Existing authorities remain unchanged

The protocol selected here does not replace existing semantic authority.

Authoritative existing semantics remain:
- `crates/prw-control-plane/src/session_auth.rs`: typed challenge/proof domain, 32-byte nonce, maximum 300-second verifier challenge lifetime, canonical signed message, replay/session/nonce/time checks;
- `crates/prw-session/src/lib.rs`: `SessionAuthenticationService` challenge creation and one-time proof verification producing `AuthenticatedDeviceSession`;
- `crates/prw-registry/src/lib.rs`: current membership/device/session revalidation;
- `crates/prw-control-transport/src/lib.rs`: Phase 129 `PRWC` outer frame, non-zero `u64` request ID, `ControlMessageKind`, maximum 65,536-byte payload;
- C03e-BY: one-connection request-ID custody/lifecycle;
- C03e-BZ: connection-local authenticated-session ownership and separate requester/rendezvous authority.

Existing `crates/prw-remote-bridge/src/session_auth_wire.rs` remains PRWM-specific wire authority only. CA deliberately does not declare its `PRWS` bytes to be PRWC bytes.

## 4. Selected PRWC authentication namespace

The new inner payload namespace is:
- magic: four ASCII bytes `PRWA`;
- major version: `1` encoded as big-endian `u16`;
- minor version: `0` encoded as big-endian `u16`;
- operation: big-endian `u16`;
- reserved flags: big-endian `u16`, required to be zero.

The fixed PRWA header is therefore exactly 12 bytes:

```text
0..4   magic = "PRWA"
4..6   major = 1
6..8   minor = 0
8..10  operation
10..12 reserved = 0
```

`PRWA` is intentionally distinct from:
- outer Phase 129 `PRWC` frame magic;
- existing PRWM logical-session `PRWS` payload magic;
- candidate-publication `PRWP` payload magic.

CA repository audit found no existing `PRWA` code collision at the exact BZ head.

## 5. Selected operations and outer-kind pairing

PRWA v1.0 defines exactly five operations:

| Code | Operation | Required outer kind | Direction / role |
| ---: | --- | --- | --- |
| 1 | `Begin` | `ControlMessageKind::Authentication` | client/originator → server/verifier |
| 2 | `Challenge` | `ControlMessageKind::Authentication` | server/verifier → client/originator |
| 3 | `Proof` | `ControlMessageKind::Authentication` | client/originator → server/verifier |
| 4 | `Authenticated` | `ControlMessageKind::Response` | server/verifier → client/originator, terminal success |
| 5 | `Rejected` | `ControlMessageKind::Error` | server/verifier → client/originator, terminal failure |

Any other outer-kind/operation pairing fails closed.

`Command`, `Event` and `Heartbeat` are never valid outer kinds for PRWA v1.0.

The terminal use of existing `Response` / `Error` preserves C03e-BY's request-completion model rather than inventing a separate completion namespace.

## 6. Common bounded string representation

Every PRWA UTF-8 identifier field is encoded as:
- big-endian `u16` byte length;
- exactly that many UTF-8 bytes.

The wire bound is `1..=1024` bytes, reusing the existing Phase 128 `MAX_SESSION_AUTH_IDENTIFIER_BYTES` limit as a transport bound.

A decoded identifier must also satisfy its existing typed constructor where a typed domain identifier exists. In particular:
- `Begin.device_id` must reconstruct through `DeviceId::new(...)`;
- `Challenge.session_id`, `Proof.session_id` and `Authenticated.session_id` must reconstruct through `SessionId::new(...)`.

The 1024-byte wire ceiling is not a new logical identity authority. Existing constructors and registry/session semantics remain authoritative.

## 7. `Begin` payload — operation 1

`Begin` body is exactly:

```text
device_id_len : u16 BE
 device_id    : device_id_len UTF-8 bytes
```

Rules:
- length must be `1..=1024`;
- UTF-8 must be valid;
- `DeviceId::new(...)` must accept the decoded value;
- no `WorkspaceId`, `UserId`, public key, `TransportIdentity`, candidate data, freshness token, request ID or requester/rendezvous authority appears in this payload.

`Begin.device_id` is an **untrusted selector claim** used only to locate the server-side enrolled-device binding against which proof will later be verified. Possessing or naming a `DeviceId` does not authenticate the connection.

CA intentionally does not put a `SessionId` in `Begin`. The typed `SessionId` used to construct the verifier challenge remains server/verifier-provided state from a separately authorized source. CA does not select its allocator, randomness, persistence or restart semantics.

## 8. `Challenge` payload — operation 2

`Challenge` body is exactly:

```text
session_id_len           : u16 BE
session_id               : session_id_len UTF-8 bytes
nonce                    : 32 bytes
issued_at_unix_seconds   : u64 BE
expires_at_unix_seconds  : u64 BE
```

Rules:
- `session_id` uses the common `1..=1024` UTF-8 bound and typed `SessionId` reconstruction;
- nonce length is exactly `SESSION_AUTH_NONCE_LEN = 32` bytes;
- verifier timestamps are unsigned 64-bit seconds;
- `expires_at_unix_seconds - issued_at_unix_seconds` must be positive and at most `MAX_SESSION_AUTH_CHALLENGE_LIFETIME_SECONDS = 300`;
- reversed, zero or over-bound lifetimes fail closed.

The challenge is verifier-produced state. A client may not substitute another session ID, nonce or timestamp tuple later in the transaction.

## 9. `Proof` payload — operation 3

`Proof` body is exactly:

```text
session_id_len      : u16 BE
session_id          : session_id_len UTF-8 bytes
nonce               : 32 bytes
algorithm            : u16 BE
signature_encoding   : u16 BE
signature_len        : u16 BE
signature            : signature_len bytes
```

Selected v1 signature profile tags:
- `algorithm = 1` → `DeviceIdentityAlgorithm::EcdsaP256Sha256`;
- `signature_encoding = 1` → `DeviceIdentitySignatureEncoding::EcdsaSigValueDer`.

Rules:
- `session_id` uses the common bound and typed constructor;
- nonce is exactly 32 bytes;
- any algorithm or encoding tag other than the locked v1 values fails closed;
- `signature_len` must be `1..=256` bytes;
- signature bytes must reconstruct through the existing `DeviceIdentitySignature` constructor for the locked profile;
- no public key is accepted from this proof; the verifier uses the server-side registered identity binding.

The explicit algorithm/encoding tags are selected for PRWA rather than silently inheriting the exact PRWS body representation.

## 10. `Authenticated` terminal payload — operation 4

`Authenticated` is carried only inside outer `ControlMessageKind::Response`.

Body:

```text
session_id_len : u16 BE
session_id     : session_id_len UTF-8 bytes
```

The session ID must equal the exact verifier challenge/session that completed successfully.

No workspace/user/device/public-key/capability/transport/rendezvous data is returned by this protocol success message. Authentication success grants no capability.

## 11. `Rejected` terminal payload — operation 5

`Rejected` is carried only inside outer `ControlMessageKind::Error`.

Its PRWA body is empty after the 12-byte header.

All externally visible authentication failures map to this single bounded terminal protocol classification. CA intentionally does not expose whether rejection arose from:
- unknown/stale/revoked device;
- inactive membership;
- malformed identifiers;
- session/nonce mismatch;
- expiration/replay;
- unsupported signature profile;
- signature verification failure;
- registry-currentness loss;
- transaction-order/correlation failure.

Internal implementation may preserve more precise private diagnostics, but the wire response selected here does not create an authentication oracle with detailed rejection codes.

## 12. Exact payload size bounds

With the selected v1 fields, maximum inner payload sizes are:
- `Begin`: `12 + 2 + 1024 = 1038` bytes;
- `Challenge`: `12 + 2 + 1024 + 32 + 8 + 8 = 1086` bytes;
- `Proof`: `12 + 2 + 1024 + 32 + 2 + 2 + 2 + 256 = 1332` bytes;
- `Authenticated`: `12 + 2 + 1024 = 1038` bytes;
- `Rejected`: exactly `12` bytes.

All are far below the existing Phase 129 `MAX_CONTROL_PAYLOAD_BYTES = 65_536` ceiling.

A future source codec must still delegate outer payload-size validation to `ControlFrame::new(...)`; these smaller PRWA maxima are additional protocol bounds, not replacements for the transport bound.

## 13. Selected request-ID transaction semantics

The PRWA transaction consumes exactly one C03e-BY-managed non-zero PRWC request ID.

Selected rules:
1. the client/originator allocates one request ID using the BY-selected one-connection monotonic custody;
2. `Begin` opens exactly one outstanding authentication transaction under that ID;
3. `Challenge` must preserve the exact same request ID;
4. `Proof` must preserve the exact same request ID;
5. only terminal outer `Response/Authenticated` or `Error/Rejected` completes that outstanding request ID;
6. Challenge and Proof do not independently allocate or complete request IDs;
7. request ID is never serialized inside PRWA payload bytes;
8. a second concurrent `Begin` on an unauthenticated connection is not selected and must fail closed;
9. after terminal success, the connection does not begin a second authentication transaction; reauthentication/switching identity remains separately gated by BZ.

Request ID remains correlation only. It is not logical identity, freshness, nonce, replay authority or requester/rendezvous authority.

## 14. Selected transaction sequence

The only successful PRWA v1 transaction sequence is:

```text
unauthenticated Phase 129 connection
    -> Authentication / PRWA Begin       [request_id = R]
    -> Authentication / PRWA Challenge   [request_id = R]
    -> Authentication / PRWA Proof       [request_id = R]
    -> Response / PRWA Authenticated      [request_id = R]
    -> connection-local authenticated-session binding becomes usable
```

Terminal failure uses:

```text
... any rejected transaction state ...
    -> Error / PRWA Rejected              [request_id = R]
    -> connection discard; no same-connection retry
```

No success path may omit Challenge or Proof. No proof-first, challenge-first-without-Begin, repeated Proof, repeated Challenge, parallel authentication request, or same-connection retry sequence is selected.

## 15. Semantic mapping to existing authentication authority

A later runtime implementation must preserve this authority order without treating wire decode as authentication success:

1. decode `Begin` strictly and treat its `DeviceId` only as an untrusted lookup selector;
2. resolve the exact current server-side enrolled device binding and reject unavailable/non-participating authority;
3. obtain one separately authorized server-side typed `SessionId` plus verifier-owned issue/expiry times;
4. call existing `SessionAuthenticationService::begin_session(...)` to create the typed challenge;
5. encode/send only the resulting typed challenge fields as PRWA `Challenge`;
6. receive exactly one correlated PRWA `Proof`;
7. require exact transaction request-ID equality;
8. require exact proof `SessionId` equality with the challenge session ID;
9. construct the existing typed `SessionAuthProof` from decoded session ID, nonce and locked-profile signature;
10. call existing `SessionAuthenticationService::submit_proof(...)` exactly once;
11. revalidate the resulting `AuthenticatedDeviceSession` against current `WorkspaceDeviceRegistry` before connection binding is usable;
12. only after those checks may terminal `Authenticated` be emitted and the BZ-selected connection-local binding become semantically usable.

The existing Phase 128 service remains authoritative for nonce, replay, verifier-time, canonical signed message, public identity and signature verification.

## 16. Pending-state cleanup and connection failure semantics

CA reuses the already-reviewed fail-closed transaction principle from the earlier PRWM C03e-G/H line without promoting PRWM bytes:
- once `begin_session(...)` has created pending challenge state, every terminal failure before successful proof commit must abort that pending session exactly once;
- no internal retry, replacement challenge, new request ID or same-connection reauthentication is selected;
- terminal failure discards the unauthenticated connection after the generic `Rejected` response is attempted;
- if a terminal response cannot be written or write outcome is unavailable, the connection is discarded rather than assuming peer agreement.

After successful proof verification, a later runtime must not admit `Command` frames using the new binding until the terminal `Authenticated` response has been successfully written. If terminal success delivery fails, the connection-local binding is discarded with the connection and is not reused on another connection.

These are transaction semantics only. CA does not implement socket close, frame I/O or cleanup source.

## 17. Strict decode / fail-closed requirements

A future PRWA codec must reject at least:
- payload shorter than the 12-byte header;
- wrong magic;
- unsupported major/minor version;
- unknown operation;
- non-zero reserved flags;
- wrong outer-kind/operation pairing;
- zero outer request ID through existing `ControlFrame` authority;
- zero/oversized identifier length;
- invalid UTF-8 or whitespace-only typed identifiers;
- invalid typed `DeviceId` / `SessionId` reconstruction;
- truncated nonce/timestamp/tag/length/signature fields;
- challenge lifetime zero/reversed/over 300 seconds;
- unsupported signature algorithm or encoding tags;
- zero/oversized signature length;
- invalid locked-profile `DeviceIdentitySignature` construction;
- any trailing bytes after the exact selected body.

Successful decode proves only bounded structural/type validity. It does not prove device enrollment, membership currentness, proof validity, authentication success, authorization or candidate-publication authority.

## 18. Identity and authority separation

CA preserves these non-interchangeable meanings:
- `DeviceId` in `Begin` = untrusted selector claim until server-side proof/currentness succeeds;
- `SessionId` = typed authentication-session correlation, verifier-provided in Challenge;
- 32-byte nonce = verifier challenge/replay input;
- device signature = proof input validated against server-side enrolled public identity;
- PRWC request ID = one-connection message transaction correlation only;
- `AuthenticatedDeviceSession` = logical authenticated PRW identity only after existing service verification;
- `TransportIdentity` = separate lower transport identity and not carried by PRWA;
- requester/rendezvous authority = separate server-side state selected in BZ and not carried by PRWA;
- candidate freshness/candidate IDs/endpoints/path kinds = unrelated candidate-publication state.

TLS success, PRWA decode, request-ID novelty and `DeviceId` naming are not authentication success.

## 19. Explicit non-selections

CA does not select or materialize:
- PRWA Rust source codec;
- live Phase 129 listener/server/accepted-stream implementation;
- socket/frame read-write loop;
- `SessionId` allocator, entropy source, persistence or restart semantics;
- request-ID source implementation beyond BY's already-selected lifecycle;
- authenticated-session persistence beyond BZ's connection-local ownership semantics;
- account authentication;
- capability policy/authorization;
- detailed external authentication error codes;
- timeout duration, retry/backoff or reconnect policy;
- requester/rendezvous provider representation/lifecycle;
- candidate publication execution or response protocol;
- candidate-ID/path-kind/endpoint discovery changes;
- reachability-store mutation;
- Agent/Desktop/Android runtime wiring;
- STUN/ICE/TURN/relay/QUIC mesh production activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 20. Safe source successor rule

After CA closure, one bounded **pure in-memory PRWA codec source materialization** checkpoint may be considered.

That source checkpoint may change only:
1. `crates/prw-remote-bridge/Cargo.toml` — promote/add `prw-core` as a production path dependency if required for typed `DeviceId` / `SessionId` codec values; no other dependency change;
2. `crates/prw-remote-bridge/src/root.rs` — export one new pure codec module;
3. `crates/prw-remote-bridge/src/control_session_auth_wire.rs` — PRWA encode/decode plus focused in-memory tests only;
4. one successor contract documenting that exact source materialization.

The codec source successor must not call `SessionAuthenticationService`, registry methods, network I/O, listener/stream APIs, request-ID allocation, requester routing or candidate-publication execution.

Permitted focused tests include:
- exact v1 golden byte layout for all five operations;
- exact outer-kind/operation pairing;
- exact request-ID preservation;
- typed DeviceId/SessionId reconstruction;
- max-bound identifiers and signature;
- 300-second challenge lifetime edge;
- rejection of malformed/truncated/trailing/unsupported-tag payloads;
- proof that all selected maximum payloads fit the Phase 129 control payload ceiling.

If any fifth product/source path is required, stop and re-audit before mutation.

## 21. Exact CA source scope

CA itself is docs-only and may change exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CA_PRWC_PRE_MESH_AUTHENTICATION_WIRE_TRANSACTION_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent/Desktop/Android implementation, transport source, provider/database file, networking configuration or deployment path blocks CA closure.

## 22. Audit basis

Exact BZ source authorities used by CA must remain byte-stable through CA:
- `crates/prw-control-plane/src/session_auth.rs` — `1dbd06d8d9741844e4d8bbb235d27431921a1650`;
- `crates/prw-session/src/lib.rs` — `845d40a3c7879f4ee813e140123d945fa1e72aff`;
- `crates/prw-remote-bridge/src/session_auth_wire.rs` — `492d3e938fcbc75907b345750928717c957204e8`;
- `crates/prw-core/src/lib.rs` — `665afdb5f2627a7d84f09b476302503e66e121e2`;
- `crates/prw-control-transport/src/lib.rs` — `34b0a898572adaa2f77251ca2e9c66ea29973e95`;
- `crates/prw-remote-bridge/Cargo.toml` — `a0e80460c8c101f72dc8b95b77b7ee33aae1f179`;
- `crates/prw-remote-bridge/src/root.rs` — `84c070ab3231062fbda37f5d06163ed8b0c0af83`.

## 23. Validation and closure

CA may close only after:
- exact closed BZ predecessor lineage remains unchanged;
- exact BZ→CA compare contains one docs-only path;
- all audit-basis source files remain byte-stable;
- every automatically triggered workflow reaches terminal non-failing verdict;
- immutable Drive audit is uploaded under project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive predecessor guard, append-only prefix proof and raw post-write verification pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No source/runtime/networking/deployment mutation is authorized by CA closure.