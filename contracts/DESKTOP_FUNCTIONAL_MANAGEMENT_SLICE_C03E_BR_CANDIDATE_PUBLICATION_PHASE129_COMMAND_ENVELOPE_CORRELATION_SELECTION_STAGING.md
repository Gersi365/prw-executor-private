# Phase 152 C03e-BR — Candidate Publication Phase 129 Command Envelope / Correlation Selection

Status: STAGED SELECTION

Gate target:
`C03E_BR_CANDIDATE_PUBLICATION_PHASE129_COMMAND_ENVELOPE_CORRELATION_SELECTED`

## 1. Exact predecessor

Closed C03e-BQ:
- branch: `phase-152-c03e-bq-candidate-publication-inner-bounded-codec-source-materialization-staging`;
- head: `43d45ed174ae246723cf431818ac47a00a52c897`;
- tree: `7b54e08e0804ecb2dace09c501d36b9c69e8e09b`;
- gate: `C03E_BQ_CANDIDATE_PUBLICATION_INNER_BOUNDED_CODEC_SOURCE_MATERIALIZED`;
- PR `#186`: `Status: CLOSED`, draft/open/unmerged.

BQ materialized only the BP-selected pure inner `PRWP` v1.0 publisher candidate-set codec plus focused non-networking tests. It deliberately materialized no PRWC wrapping, request correlation, I/O, authentication, routing, publication admission, networking activation, deployment, or merge.

Earlier locked direction remains authoritative:
- C03e-BN selected the existing Phase 129 control plane as the pre-mesh candidate-publication carrier boundary;
- C03e-BO selected that application semantics own no independent request identifier and that Phase 129 `PRWC request_id` remains outer message correlation only;
- C03e-BP selected the exact inner `PRWP` v1.0 schema;
- C03e-BQ materialized that inner codec only.

BR must preserve all four boundaries rather than redesign them.

## 2. Exact repository audit basis

BR is grounded in the exact closed BQ snapshot.

### 2.1 Existing Phase 129 transport envelope

`crates/prw-control-transport/src/lib.rs` at BQ:
- blob: `34b0a898572adaa2f77251ca2e9c66ea29973e95`;
- ALPN: `prw-control/1`;
- frame magic: `PRWC`;
- protocol version: `1.0`;
- fixed frame header: 24 bytes;
- maximum frame payload: 65,536 bytes.

Existing `ControlMessageKind` values are:

```text
Authentication = 1
Command        = 2
Response       = 3
Event          = 4
Heartbeat      = 5
Error          = 6
```

The transport source defines `Command` as a command payload whose semantics and authorization are defined elsewhere, and explicitly states that message semantics remain above the transport layer.

Existing `ControlFrame::new(...)` accepts:

```rust
ControlFrame::new(
    kind: ControlMessageKind,
    request_id: u64,
    payload: impl Into<Vec<u8>>,
)
```

and rejects:
- `request_id == 0`;
- payloads above `MAX_CONTROL_PAYLOAD_BYTES`.

The frame codec also rejects unknown kind codes, non-zero reserved flags, malformed headers, unsupported versions, oversized payloads, and truncation.

Phase 129 remains outbound-only transport machinery in the current crate. TLS success remains server transport authentication only and does not itself authenticate a PRW logical session or authorize candidate publication.

### 2.2 Existing inner candidate-publication codec

`crates/prw-remote-bridge/src/candidate_publication_wire.rs` at BQ:
- blob: `299042938b38b65b78f737926f74b8567e5046fb`;
- inner magic: `PRWP`;
- inner version: `1.0`;
- inner operation: `1 = PublisherCandidateSetSubmission`;
- exact inner payload bound: `80..=592` bytes.

The inner codec explicitly performs no PRWC wrapping, request correlation, authentication, routing, publication admission, freshness rotation, socket I/O, candidate discovery/classification, reachability mutation, networking activation, or deployment.

### 2.3 Current crate-dependency boundary

`crates/prw-remote-bridge/Cargo.toml` at BQ:
- blob: `e48178c903654c8102c099351d1d8407329821c7`.

`prw-remote-bridge` currently does **not** depend on `prw-control-transport`.

Therefore BR must not silently materialize a bridge-to-control-transport dependency or place generic transport/application coupling into the wrong crate. Exact source ownership and dependency direction remain a separate decision after BR.

## 3. Selected outer envelope kind

BR selects the existing Phase 129 envelope kind:

`ControlMessageKind::Command = 2`

for a future candidate-publication publisher-submission PRWC frame.

BR deliberately selects **reuse of the existing generic Command kind**, not a new `ControlMessageKind` code.

Rationale:
- BN already selected Phase 129 as the carrier plane;
- the existing transport contract says command semantics and authorization are defined elsewhere;
- candidate publication is an application command/request semantic above the transport layer;
- the inner `PRWP` magic/version/operation already provides the exact bounded application-level discriminator;
- adding a dedicated transport kind would widen the Phase 129 transport protocol registry without a demonstrated transport-layer need;
- reusing `Command` preserves the existing transport/app layering rather than making the generic transport crate candidate-publication-aware.

Selection of `Command` is an envelope classification only. It does not make a frame authenticated, authorized, current, routable, admitted, reachable, live, or ready.

## 4. No new Phase 129 kind selected

BR explicitly does **not** select:
- a new `CandidatePublication` `ControlMessageKind` variant;
- a new u16 transport message-kind code;
- a change to `ControlMessageKind::from_code(...)`;
- a change to Phase 129 frame magic/version/header layout;
- a change to ALPN;
- a change to the 65,536-byte payload bound.

The Phase 129 transport source is expected to remain byte-stable for BR itself.

## 5. Exact selected envelope composition

BR selects only this abstract composition for a future publisher submission:

```text
CandidatePublicationWireSubmission
    -> existing PRWP v1.0 encode
       * payload length 80..=592
    + externally supplied non-zero outer request correlation value
    -> existing Phase 129 ControlFrame
       * kind = ControlMessageKind::Command
       * request_id = supplied outer correlation value
       * payload = exact PRWP bytes
```

The candidate-publication application payload does not acquire an internal request ID.

The `PRWP` bytes remain byte-for-byte the BQ inner payload. No outer field is copied into or derived into the inner payload.

## 6. Exact selected inbound interpretation boundary

For a future **pure in-memory** candidate-publication frame adapter, BR selects this interpretation order only:

```text
already-decoded Phase 129 ControlFrame
    -> require kind == ControlMessageKind::Command
    -> retain outer request_id only as outer correlation metadata
    -> pass exact payload bytes to CandidatePublicationWireSubmission::decode(...)
    -> obtain bounded typed submitted values
    -> later current authenticated publisher/session semantic validation
    -> later separately authoritative requester/rendezvous admission
    -> later verifier freshness/current-plan validation
    -> later durable compare-and-commit
```

A candidate-publication adapter must fail closed if the supplied outer frame kind is not `Command`.

A successful outer-frame decode plus successful inner `PRWP` decode remains only envelope + structural/type validity. It is not semantic admission.

BR does not select an inbound listener, stream reader, server dispatch loop, broker, or routing implementation.

## 7. Correlation ownership

BR preserves BO exactly:

> The Phase 129 `PRWC request_id` is outer message correlation only.

It is not:
- logical publisher identity;
- requester/recipient identity;
- authentication proof;
- authorization proof;
- publication freshness;
- candidate identity;
- candidate-set generation;
- routing authority;
- workspace authority;
- liveness/readiness evidence;
- replay protection by itself.

The candidate-publication inner payload contains no duplicate request/correlation identifier.

## 8. Request-id custody remains unselected

BR does not select how a production non-zero Phase 129 request ID is generated or owned.

Still unselected:
- allocator implementation;
- monotonic versus random generation;
- uniqueness scope;
- per-connection versus process-wide versus durable scope;
- crash/restart persistence;
- database-backed allocation;
- collision recovery;
- request lifetime/reuse rules;
- deriving IDs from timestamps;
- deriving IDs from `SessionId`;
- deriving IDs from `CandidateId`;
- deriving IDs from `DeviceId` or `TransportIdentity`.

A future pure adapter may consume a non-zero request correlation value only after an upper caller/correlation boundary supplies it. Such acceptance does not claim that arbitrary caller input is authoritative production allocator custody.

Production sending remains blocked until request-id custody is deliberately selected/materialized at the appropriate owner.

## 9. `SessionId` remains separate

`SessionId` remains authentication-lifecycle correlation only.

BR does not:
- put `SessionId` into `PRWP`;
- map `SessionId` into PRWC request ID;
- use `SessionId` as publisher identity proof;
- use `SessionId` as rendezvous routing authority;
- use `SessionId` as freshness or candidate identity.

Production `SessionId` allocation/custody remains separately outside BR.

## 10. Envelope size compatibility

BQ inner payload bounds are strictly inside the existing Phase 129 payload bound:

```text
PRWP minimum payload = 80 bytes
PRWP maximum payload = 592 bytes
PRWC maximum payload = 65,536 bytes
```

Therefore every structurally valid BQ inner payload fits within the existing Phase 129 envelope payload bound without changing transport limits.

If a future frame is serialized by already-existing Phase 129 mechanics, the mathematical total frame size would be:
- minimum: `24 + 80 = 104` bytes;
- maximum: `24 + 592 = 616` bytes.

BR records these bounds only as compatibility arithmetic. It does not authorize stream serialization, socket I/O, connection establishment, or production sending.

## 11. Inner discriminator remains authoritative for candidate-publication syntax

Because BR reuses generic `Command`, exact candidate-publication syntax remains identified by the existing inner selection:
- `PRWP` magic;
- version `1.0`;
- operation `1 = PublisherCandidateSetSubmission`;
- strict BQ malformed-input behavior.

The generic outer `Command` kind must not make arbitrary command payloads candidate publications.

Only payloads that pass the exact `PRWP` decoder are structurally valid candidate-publication submissions.

## 12. Response/error protocol remains unselected

BR does not select:
- `ControlMessageKind::Response` as a candidate-publication acknowledgement;
- `ControlMessageKind::Error` as a candidate-publication error response;
- response payload schema;
- response correlation rules beyond the existing outer request-id concept;
- acknowledgement timing;
- retry behavior;
- idempotency key;
- deduplication window;
- timeout semantics;
- retry-safe durable commit behavior.

A future source adapter must not fabricate response/ack/error semantics merely because Phase 129 has generic `Response` and `Error` kinds.

## 13. Authentication remains above transport

BR does not turn any Phase 129 transport property into logical PRW authentication.

Specifically not logical authentication/authorization authority:
- TCP connection existence;
- TLS handshake success;
- server certificate validation;
- ALPN negotiation;
- `ControlMessageKind::Command`;
- non-zero PRWC request ID;
- successful PRWC frame decode;
- successful PRWP decode.

Logical publisher identity remains the current authenticated PRW control-plane session context selected by BO and existing semantic validators.

The future decoded submission must still be adapted through current server-side authenticated session context and existing publication validators before admission.

## 14. Requester/rendezvous routing remains unselected

BR does not add requester or recipient routing information to the inner or outer candidate-publication representation.

It does not select:
- requester `DeviceId` field;
- recipient `DeviceId` field;
- workspace routing field;
- requester `SessionId` field;
- routing provider/table;
- expected-device scheduling provenance;
- broker dispatch key;
- connection identity as logical target;
- request ID as routing identity.

Current authenticated logical identity/workspace semantics remain the authority boundary. Exact requester/rendezvous acquisition and dispatch remain separately gated.

## 15. Freshness remains independent

The verifier-owned `CandidatePublicationFreshnessToken` remains inside the exact BQ `PRWP` payload solely as a presented opaque semantic input for authoritative comparison.

Neither the outer `Command` kind nor PRWC request ID can replace or derive freshness.

A newly correlated command may still present stale, missing, retired, or invalid publication freshness and must fail at the later verifier authority boundary.

## 16. Source ownership/dependency direction remains unselected

BR deliberately stops before source materialization because the exact BQ repository shows:
- the generic Phase 129 `ControlFrame` type is owned by `prw-control-transport`;
- the candidate-publication `PRWP` codec is owned by `prw-remote-bridge`;
- `prw-remote-bridge` currently has no dependency on `prw-control-transport`.

BR does not silently choose among possible ownership arrangements.

In particular BR does not yet authorize:
- adding `prw-control-transport` to `prw-remote-bridge/Cargo.toml`;
- making `prw-control-transport` depend on `prw-remote-bridge`;
- moving `candidate_publication_wire.rs` between crates;
- creating a new integration crate;
- relocating existing Phase 129 frame types;
- exposing a transport-specific API from the inner codec module.

The next safe checkpoint must first select the pure adapter ownership/dependency direction while preserving acyclic layering and the existing generic transport boundary.

## 17. No source materialization in BR

BR is docs-only selection.

It does not change:
- `crates/prw-control-transport/src/lib.rs`;
- `crates/prw-remote-bridge/src/candidate_publication_wire.rs`;
- `crates/prw-remote-bridge/src/root.rs`;
- any Cargo manifest or lockfile;
- any Android source/configuration;
- any workflow;
- any Agent source;
- any registry/provider/database source;
- any transport/listener/dispatcher source;
- any deployment or host/network configuration.

## 18. Explicit non-selections

BR does not select or materialize:
- new Phase 129 kind/code;
- transport frame format change;
- source adapter ownership;
- Cargo dependency direction;
- request-id allocator/custody;
- SessionId allocator/custody;
- logical control-plane authentication wire protocol;
- authenticated session store/provider;
- requester/recipient routing schema/provider;
- expected-device rendezvous scheduling;
- inbound server/listener/acceptor;
- broker/dispatcher;
- outbound production send path;
- response/ack/error schema;
- retry/idempotency/deduplication;
- candidate-ID allocator/custody;
- path-kind classifier/provenance implementation;
- endpoint discovery provider;
- freshness bootstrap/resynchronization delivery;
- publication admission or reachability mutation;
- registry/provider/database mutation;
- STUN/ICE/TURN/relay/QUIC production activation;
- Agent activation/readiness;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 19. Next checkpoint rule

After BR closure, the next safe checkpoint should be a bounded **adapter ownership/dependency selection** only.

That checkpoint must audit the repository dependency graph and select where a pure in-memory `PRWP <-> PRWC Command` composition adapter belongs without:
- introducing a dependency cycle;
- making generic transport depend on candidate-publication application semantics;
- moving existing semantic authority into transport;
- materializing I/O;
- selecting request-id production custody;
- selecting routing/authentication/server dispatch;
- activating networking.

Only after that ownership/dependency selection may a later source-materialization checkpoint consider a pure in-memory adapter. Any required Cargo dependency change must be explicitly bounded by that later contract rather than hidden inside BR.

## 20. Identity/security invariants

BR preserves:
- `DeviceId` / current authenticated PRW session identity as logical identity;
- `TransportIdentity` as independently rotatable lower-transport certificate identity only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `ConnectivityPathKind` as explicit product path classification only;
- `SessionId` as authentication correlation only;
- `PRWC request_id` as outer message correlation only;
- candidate-publication freshness as verifier-owned replay/currentness authority only.

Transport success, envelope kind, request correlation, decode success, candidate metadata, and freshness-token possession alone are not authentication, authorization, publication currentness, reachability, public-routability, liveness, or readiness evidence.

## 21. Exact intended BQ -> BR scope

The final BR branch must differ from closed BQ only by this contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BR_CANDIDATE_PUBLICATION_PHASE129_COMMAND_ENVELOPE_CORRELATION_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent implementation, registry/provider, transport implementation, networking, packaging/systemd, or deployment change blocks BR closure.

## 22. Validation and closure requirements

BR may close only after:
- exact BQ predecessor lineage remains unchanged;
- exact BQ -> BR compare is one docs-only path;
- BQ source blobs used as audit basis remain exact and byte-stable;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit is uploaded inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence `C02E_BRANCH_STATUS.md` passes a fresh predecessor guard, append-only prefix proof, and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No source adapter, Cargo dependency, request-id allocator, routing, authentication protocol, networking activation, deployment, or merge is authorized by BR closure.

Gate target remains:
`C03E_BR_CANDIDATE_PUBLICATION_PHASE129_COMMAND_ENVELOPE_CORRELATION_SELECTED`
