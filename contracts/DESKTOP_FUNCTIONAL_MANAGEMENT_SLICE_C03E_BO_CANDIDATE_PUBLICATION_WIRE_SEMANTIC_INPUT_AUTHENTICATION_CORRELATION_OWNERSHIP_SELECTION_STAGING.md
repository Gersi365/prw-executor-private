# Phase 152 C03e-BO — Candidate Publication Wire Semantic Input / Authentication / Correlation Ownership Selection

Status: STAGED SELECTION

Gate target:
`C03E_BO_CANDIDATE_PUBLICATION_WIRE_SEMANTIC_INPUT_AUTHENTICATION_CORRELATION_OWNERSHIP_SELECTED`

## 1. Exact predecessor

Closed C03e-BN:
- branch: `phase-152-c03e-bn-candidate-publication-control-plane-carrier-rendezvous-boundary-selection-staging`;
- head: `6600f0bdbfad03b8b89517935a4df50e1e66ee7d`;
- tree: `5390e4bf7673da147b4fa11ebed9b323890b3b42`;
- gate: `C03E_BN_CANDIDATE_PUBLICATION_CONTROL_PLANE_CARRIER_RENDEZVOUS_BOUNDARY_SELECTED`.

BN selected only the carrier/layering direction: candidate-set signaling required before or during reachability establishment belongs on the existing Phase 129 control plane selected by Phase 139, not on the mesh data plane whose establishment depends on those candidates.

BN deliberately left exact candidate-publication payload bytes, operation registry, broker/routing, logical authentication wire composition, request-id allocation/custody, `SessionId` allocation/custody and production networking unselected.

## 2. Exact repository audit basis

BO is grounded in the exact BN snapshot, including these existing source boundaries:

- `crates/prw-remote-bridge/src/candidate_reachability.rs` blob `51b294cfb3772925651a05bdcb034cd051204efb`;
- `crates/prw-remote-bridge/src/reachability_owner.rs` blob `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
- `crates/prw-remote-bridge/src/candidate_publication_freshness.rs` blob `fd7c2f095999b6a6479be79c562637fe5f46634c`;
- `crates/prw-control-transport/src/lib.rs` blob `34b0a898572adaa2f77251ca2e9c66ea29973e95`;
- exact BN branch/head/tree above.

The audit establishes the following source-level facts.

### 2.1 Publisher semantic construction already separates authenticated context from submitted fields

Existing `publish_current_candidates(...)` has the semantic shape:

```rust
publish_current_candidates(
    registry,
    publisher_session,
    presented_transport_identity,
    candidates,
)
```

The publisher's logical `DeviceId` is not accepted as an arbitrary submitted target. It is derived from the registry-current authenticated publisher session. The presented `TransportIdentity` is then independently revalidated for that exact logical device before `PeerConnectivityIdentity` is formed.

### 2.2 Publication commit already separates requester context and freshness

Existing `ProductionReachabilityOwner::commit_candidate_publication(...)` consumes, as separate authoritative inputs:

```text
registry
requester_session
AuthenticatedCandidatePublication
presented_freshness
```

Therefore requester identity/routing context, authenticated publisher provenance, candidate content and verifier-owned publication freshness are already separate semantic authorities.

### 2.3 Existing freshness state is opaque verifier authority

`CandidatePublicationFreshnessToken` is an opaque non-zero 32-byte verifier-issued token. Existing source explicitly forbids deriving replacement freshness from publisher input, clocks, request IDs, candidate IDs or endpoints.

### 2.4 Existing Phase 129 framing owns envelope correlation only

Phase 129 `ControlFrame` contains:

```text
ControlMessageKind
non-zero u64 request_id
bounded payload bytes
```

The crate explicitly states that message semantics remain above the transport layer and that TLS success is transport authentication only.

### 2.5 Existing candidate domain remains bounded and typed

The candidate domain already requires:
- at most `MAX_CONNECTIVITY_CANDIDATES = 16` candidates;
- non-zero `CandidateId(u64)`;
- explicit `ConnectivityPathKind`;
- explicit validated IP address plus non-zero `u16` port;
- no hostname/resolver input;
- no duplicate candidate IDs;
- no duplicate exact `(path kind, endpoint)` tuples;
- plan-lifetime anti-reuse/rebinding constraints at refresh.

BO does not replace any of these domain validators.

## 3. Selected publisher-submission semantic inputs

BO selects only the semantic contents that a future bounded candidate-publication **publisher submission payload** is permitted to represent before current authenticated admission:

```text
presented_transport_identity: TransportIdentity
presented_freshness: CandidatePublicationFreshnessToken
candidates: bounded vector of ConnectivityCandidate
```

These are submitted values, not authority by themselves.

The future decode/adaptation path must treat them as untrusted typed inputs until existing registry/currentness/freshness/plan validation succeeds.

BO does not yet select their byte encoding, numeric wire tags, payload magic, payload version, operation code or frame kind.

## 4. Logical publisher authentication ownership

BO selects this ownership rule:

> Logical publisher identity is owned by the current authenticated PRW control-plane session context above Phase 129 TLS. It is not reconstructed from client-supplied candidate-publication payload fields.

Consequently a future publisher-submission payload must not carry an authoritative client-selected:
- `DeviceId`;
- `WorkspaceId`;
- `UserId`;
- `SessionId` as proof of identity;
- serialized `AuthenticatedDeviceSession`;
- public identity material as a substitute for completed session authentication.

A decoded publisher submission may become an `AuthenticatedCandidatePublication` only by supplying the current server-side authenticated publisher session to existing `publish_current_candidates(...)` semantics and revalidating the presented `TransportIdentity` against that exact logical device.

BO does not select the missing control-plane logical session-authentication wire composition, server session table/provider, listener, or session establishment protocol.

## 5. Publisher `DeviceId` is derived, not routed from payload

The publisher's logical identity remains derived from current authenticated context:

```text
current authenticated publisher session
    -> registry-current DeviceId/workspace/user binding
    + presented TransportIdentity
    -> independent transport-identity currentness validation
    -> PeerConnectivityIdentity
```

The payload may not ask the verifier to publish candidates "for" another arbitrary `DeviceId`.

This preserves the existing anti-retargeting property of `publish_current_candidates(...)`.

## 6. Requester / recipient routing remains outside publisher payload

BO does not select a requester/recipient target field in the publisher submission.

In particular, the future publisher submission must not acquire authority by carrying a client-selected:
- requester `DeviceId`;
- recipient `DeviceId`;
- workspace routing key;
- requester `SessionId`;
- `SocketAddr` / `ConnectivityEndpoint` as routing identity;
- `TransportIdentity` of another device as logical rendezvous target;
- live-owner fence as recipient identity;
- request ID as routing identity.

Requester/recipient selection must come from a separately current authenticated control-plane rendezvous/routing context and must still pass the existing requester/publisher same-workspace and exact-target admission checks before commit.

The exact rendezvous request model, expected-device scheduling provenance, routing provider/table and broker dispatch remain unselected in BO.

## 7. Publication freshness ownership

BO selects that the current candidate-publication freshness token is a publisher-presented semantic input but remains verifier-owned authority.

The publication payload may carry the exact opaque token bytes only so the verifier can compare them against its current authoritative state.

Carrying a token does not make it current.

The verifier remains responsible for:
- current lifecycle lookup/recovery semantics;
- exact token comparison;
- replacement-token generation;
- durable compare-and-commit;
- fail-closed handling of stale, missing, retired or ambiguous state.

The token must not be derived from or replaced by:
- Phase 129 request ID;
- `SessionId`;
- `CandidateId`;
- endpoint address/port;
- path kind;
- timestamp;
- TCP connection identity;
- TLS handshake state.

BO does not claim that the production control-plane freshness-token delivery/bootstrap path is already materialized. Existing `PRWF`/PRWM freshness delivery must not be silently reused as the Phase 129 pre-mesh delivery mechanism.

## 8. Candidate vector ownership and bounds

BO selects that the publisher submission carries a complete proposed candidate vector, not one implicit mutable candidate operation.

The decoded vector must remain subject to the existing complete-set validators:
- candidate count `0..=16`;
- each `CandidateId` non-zero;
- each endpoint explicit and valid;
- no duplicate candidate IDs;
- no duplicate exact `(path kind, endpoint)` tuples;
- existing plan high-water/rebinding rules at refresh.

An empty candidate vector remains only a typed complete candidate-set proposal; it does not imply liveness, readiness, revocation, or network shutdown.

BO does not select candidate-ID production/custody, path-kind classification, endpoint discovery, priority, ICE foundation, relay allocation or public-routability inference.

## 9. Correlation ownership

BO selects a strict layering rule for correlation:

> Candidate-publication application semantics own no independent request/correlation identifier. If carried by Phase 129, request correlation belongs to the outer existing `PRWC` envelope.

Therefore a future inner candidate-publication payload must not duplicate, reinterpret or derive an internal request identifier from the Phase 129 `request_id`.

The outer `PRWC request_id` remains:
- non-zero by existing transport validation;
- message correlation only;
- non-authenticating;
- non-authorizing;
- non-freshness evidence;
- non-routing identity;
- non-candidate identity.

BO does **not** select production request-id allocation/custody, uniqueness scope, randomness/monotonicity, persistence or restart semantics.

Accordingly, a source-materialization successor may materialize an **inner bounded publisher-submission representation/codec without an outer frame allocator**. It must not introduce a production request-id generator or claim that an arbitrary caller-provided ID is authoritative production custody.

## 10. `SessionId` ownership remains authentication-only

`SessionId` remains correlation within the enrolled-device authentication lifecycle.

BO does not place `SessionId` into the candidate-publication payload as:
- publisher identity proof;
- requester routing authority;
- publication freshness;
- candidate correlation;
- Phase 129 request-id source.

Production `SessionId` allocation/custody remains separately unselected.

## 11. Exact semantic adaptation order selected by BO

BO selects this semantic dependency order for a future publisher submission:

```text
bounded publisher-submission bytes
    -> strict bounded/versioned inner decode
    -> typed presented TransportIdentity
    -> typed presented verifier freshness token
    -> typed bounded candidate vector
    -> current server-side authenticated publisher session context
    -> publish_current_candidates(...)
       * revalidate publisher session currentness
       * derive publisher DeviceId from authenticated context
       * revalidate presented TransportIdentity for that DeviceId
       * validate complete candidate set
    -> separately authoritative requester/rendezvous context
    -> current requester/publisher/workspace/exact-target admission
    -> verifier freshness comparison
    -> staged plan refresh validation
    -> durable compare-and-commit
```

No earlier decode or transport event may skip a later authority check.

## 12. Payload fields explicitly not selected

BO explicitly does not select these as publisher-submission fields:
- publisher `DeviceId`;
- publisher `WorkspaceId`;
- publisher `UserId`;
- serialized `AuthenticatedDeviceSession`;
- authoritative `SessionId`;
- requester/recipient `DeviceId`;
- requester/recipient `SessionId`;
- arbitrary workspace routing target;
- PRWC request ID duplicated inside payload;
- PRWM request ID;
- live-owner fence;
- reachability observation;
- selected path;
- readiness bit;
- STUN/ICE/TURN state;
- relay provider token/handle;
- DNS name/hostname;
- source or destination socket as logical identity.

## 13. Wire bytes remain unselected

BO does not select or materialize:
- candidate-publication payload magic;
- major/minor payload version;
- operation code registry;
- `ControlMessageKind` mapping;
- integer endianness;
- candidate count field width;
- `CandidateId` byte width on wire;
- `ConnectivityPathKind` numeric wire tags;
- IPv4/IPv6 address-family tags;
- endpoint address/port byte layout;
- freshness-token placement/order;
- transport-identity placement/order;
- error-response schema;
- acknowledgements;
- idempotency keys;
- retry semantics.

Those exact codec details require a separate selection checkpoint before source materialization.

## 14. Phase 129 carrier remains envelope only

BO preserves BN's carrier boundary.

Phase 129 `PRWC/prw-control/1` may carry the future publication payload, but its existing properties remain only:
- bounded frame envelope;
- message kind;
- non-zero request correlation field;
- TCP + TLS 1.3 transport;
- ALPN `prw-control/1`;
- configured server trust/transport authentication.

TLS connection success still does not establish logical publisher identity or publication authorization.

## 15. Mesh/data-plane non-authority remains unchanged

BO does not move candidate publication to `PRWM/prw-mesh/1`.

`PRWF` freshness messages, mesh capability messages and established QUIC control streams remain distinct protocols/boundaries and must not be reused as the initial candidate publication carrier by structural similarity.

## 16. Existing semantic validators remain authoritative

Future decoding must adapt into existing typed/domain validation rather than clone its rules into a weaker wire-only authority.

Specifically:
- `TransportIdentity::new(...)` remains authoritative for non-zero transport identity representation;
- `CandidatePublicationFreshnessToken::new(...)` remains authoritative for non-zero opaque freshness representation;
- `CandidateId::new(...)` remains authoritative for non-zero candidate identifiers;
- `ConnectivityEndpoint::new(...)` remains authoritative for explicit endpoint validation;
- `PeerConnectivityPlan` remains authoritative for candidate-set bounds/duplicates/high-water semantics;
- `publish_current_candidates(...)` remains authoritative for current publisher/session/transport provenance;
- `validate_authenticated_publication_admission(...)` remains authoritative for requester/publisher/workspace/exact-target currentness;
- `ProductionReachabilityOwner::commit_candidate_publication(...)` remains authoritative for verifier freshness and durable commit ordering.

A future wire codec must not make successful decode equivalent to semantic acceptance.

## 17. Exact non-authorities

BO explicitly preserves these non-authorities:
- TCP connection identity;
- TLS certificate/server-authentication success;
- `PRWC request_id`;
- `PRWM request_id`;
- `SessionId`;
- `TransportIdentity` alone;
- `CandidateId`;
- candidate endpoint;
- path kind;
- freshness token possession alone;
- live-owner fence;
- candidate ordering;
- candidate count;
- broker connection lifetime.

None of these by itself proves authentication, authorization, currentness, reachability, public routability, liveness or readiness.

## 18. Explicit non-selections

BO does not select or materialize:
- exact publication wire codec/schema/version/magic;
- outer `ControlMessageKind` mapping;
- Phase 129 request-id allocator/custody;
- `SessionId` allocator/custody;
- control-plane logical authentication wire protocol;
- authenticated session store/provider;
- requester/recipient routing schema;
- expected-device rendezvous/scheduling provenance;
- broker/server/dispatcher implementation;
- listener/acceptor;
- retries/idempotency/deduplication/ack timing;
- candidate-ID allocator;
- path-kind classifier;
- endpoint discovery provider;
- control-plane freshness bootstrap/delivery materialization;
- STUN/ICE/TURN/relay production activation;
- registry/provider/database mutation;
- Agent bootstrap/readiness activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 19. Source-materialization successor rule

A source-materialization successor is **not yet authorized to wrap or send Phase 129 frames** merely by BO closure.

The next safe checkpoint should first select the exact bounded inner publisher-submission codec/schema, including versioning, exact field encodings, path-kind tags, IP-family encoding, candidate-count representation and malformed-input behavior.

Only after that separate exact codec selection may a source successor materialize a pure inner encode/decode adapter that:
- performs no I/O;
- allocates no request ID;
- authenticates no session;
- routes to no requester;
- commits no reachability state;
- opens no socket;
- activates no production networking.

Outer PRWC wrapping/sending and production request-id custody remain separately gated.

## 20. Identity and security invariants

BO preserves:
- `DeviceId` / current authenticated PRW session identity as logical identity;
- `TransportIdentity` as independently rotatable lower transport certificate identity only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `ConnectivityPathKind` as explicit product path classification only;
- `SessionId` as authentication correlation only;
- `PRWC` request ID as outer message correlation only;
- freshness token as verifier-owned publication replay/currentness state only.

Submitted candidate metadata is not authentication, authorization, currentness, reachability, liveness, public-routability or readiness evidence.

## 21. Exact intended BN -> BO scope

The final BO branch must differ from closed BN only by this contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BO_CANDIDATE_PUBLICATION_WIRE_SEMANTIC_INPUT_AUTHENTICATION_CORRELATION_OWNERSHIP_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent implementation, registry/provider, packaging/systemd, networking or deployment change blocks BO closure.

## 22. Validation requirements

BO can close only after:
- exact BN predecessor head/tree remain unchanged;
- exact BN -> BO compare is one docs-only path;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- immutable BO audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

Gate target remains:
`C03E_BO_CANDIDATE_PUBLICATION_WIRE_SEMANTIC_INPUT_AUTHENTICATION_CORRELATION_OWNERSHIP_SELECTED`
