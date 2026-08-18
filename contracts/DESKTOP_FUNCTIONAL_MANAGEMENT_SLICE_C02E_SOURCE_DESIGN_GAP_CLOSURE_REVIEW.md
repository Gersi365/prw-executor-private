# Phase 152 C02e — Source/Design Gap-Closure Review

Status: `PASS_STATIC_GAP_REVIEW / SOURCE_DESIGN_SEMANTICS_CLOSED_TO_CURRENT_AUTHORITY / NO_FURTHER_SAFE_RUNTIME_NEUTRAL_SEMANTIC_GAP_IDENTIFIED / IMPLEMENTATION_VALIDATION_AND_EXTERNAL_AUTHORITY_GATES_REMAIN / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Review head: `5e27789449cb34213b2c7d3e4c02dcf480b8102b`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

Relation to C02d at review:

- ahead: 42 commits;
- behind: 0;
- changed paths: 44.

## Purpose

C02e has progressed through a sequence of source/design locks, corrective reviews and test-only reference models. This review determines whether another **runtime-neutral semantic gap** can still be closed safely under the current authority, or whether the remaining work now depends on gates that C02e has deliberately not opened.

This is not a completion claim for implementation validation, wire protocol, durable freshness, actual Phase 141 composition, production orchestration or deployment.

## Source/design semantics now closed

### 1. Logical identity is not endpoint identity

`DeviceId`, authenticated session identity, current `TransportIdentity`, and transient candidate IP/port remain distinct.

Endpoint movement with unchanged logical/transport identity is candidate refresh, not device replacement.

### 2. Candidate refresh is transactional and plan-scoped

`PeerConnectivityPlan` remains authoritative for candidate vectors, observations and deterministic selection.

Successful refresh validates the complete vector before authoritative replacement and resets observations.

Failed refresh preserves the previous plan state.

### 3. Candidate IDs are lifetime-fresh within one plan

An exact retained candidate may keep its ID.

A removed candidate ID cannot later return in that plan, and an existing ID cannot be rebound to another endpoint/path.

The private high-water rule prevents delayed Phase 141 correlation from aliasing a later candidate.

### 4. Candidate publication provenance/admission is authenticated and current

The source-only private candidate semantic adapter derives publisher identity from a registry-current authenticated session and current `TransportIdentity`.

Consumption checks current requester, current publisher, same workspace, exact publication/plan peer identity and exact current target transport before candidate mutation.

The non-mutating `validate_authenticated_publication_admission(...)` seam now exposes that same ordering to an upper freshness authority without duplicating a second admission model.

### 5. Transport rotation is a replacement-peer lifecycle

`TransportIdentity` rotation makes the old plan/publication/traversal/freshness lifecycle stale.

The same logical `DeviceId` may continue, but a replacement plan for the new current transport identity is required. No in-place peer identity rebinding is allowed.

### 6. Publication freshness is verifier-owned and upper-reachability-scoped

Candidate-publication freshness is not Phase 129 `request_id`, endpoint state, candidate ID, ICE success or Phase 128 session-auth nonce state.

It belongs to the upper reachability composition authority or a transactionally coupled verifier subauthority.

Its lifecycle is target/publisher peer state and is requester-independent.

Ordinary authenticated session renewal/reconnect for the same `DeviceId + TransportIdentity` does not reset freshness.

### 7. Publication admission/freshness/candidate ordering is fixed

The locked order is:

`current requester/publisher/workspace/target/transport admission`

`-> exact expected verifier freshness`

`-> complete candidate validation`

`-> accepted commit`

Rejected identity/admission does not disclose/consume freshness.

Stale freshness does not mutate candidate state.

Candidate failure does not advance freshness.

### 8. New-peer bootstrap and existing-peer state loss are distinct

Only an authoritatively legitimate new peer lifecycle may enter verifier-owned bootstrap.

Missing/unavailable freshness for an existing peer lifecycle is recovery-required and fails closed; it is not an implicit first-publication state.

Automatic same-identity re-baselining remains forbidden without a separately reviewed recovery protocol.

### 9. First-publication bootstrap is non-consuming on candidate failure

A correct current bootstrap presentation followed by invalid target-plan candidate semantics leaves bootstrap current and retryable.

At most one competing first publication from one bootstrap state may commit.

### 10. Traversal observations have their own lifecycle currentness

Publication freshness and traversal observation currentness are separate states.

Every successful full candidate refresh makes the previous Phase 141 traversal session and its queued/unapplied observations stale, including for an exactly retained candidate.

Candidate ID existence alone cannot prove traversal-session currentness.

### 11. Refresh and observation admission must linearize

An old observation racing refresh is either:

- applied completely before refresh and then cleared by successful refresh; or
- rejected after refresh because its traversal lifecycle is stale.

The forbidden outcome is old traversal evidence repopulating a refreshed plan.

### 12. Successful refresh recovers forward

If candidate publication commits but replacement traversal construction later fails:

- refreshed plan remains current;
- publication freshness remains advanced;
- old traversal remains stale;
- no reachability is invented;
- recovery proceeds forward by later replacement traversal establishment, never by resurrecting stale state.

### 13. Upper ownership pattern is established without naming production placement

Repository precedent supports lower state machines retaining local authority and a later upper composition owner serializing cross-object lifecycle transitions.

A source/disposable exclusive `&mut self` reference owner is a valid semantics model, not a production synchronization mandate.

No current crate owns both actual `PeerConnectivityPlan` and actual `IceConnectivitySession`, so concrete production placement remains unselected.

## Test-only source references now staged

C02e contains integration-test source references for:

- exclusive plan/traversal lifecycle ownership using a test-local traversal marker;
- established candidate-publication freshness compare/stage/commit behavior;
- requester-independent freshness and same-peer session-renewal continuity;
- verifier state unavailable/fail-closed behavior;
- new vs established vs recovery-required freshness lifecycle distinction;
- non-consuming first-publication candidate failure;
- distinct replacement-peer bootstrap after transport rotation.

All freshness/traversal markers used by these references are explicitly test-local and non-normative.

No test has been executed while the build gate is closed.

## Remaining work requiring authority beyond the current source/design gate

Static review found no additional runtime-neutral semantic gap that can be safely closed without choosing or validating one of the following externally gated dimensions.

### Gate A — concrete candidate-publication freshness representation / protocol

Still unselected:

- token/counter/nonce/timestamp representation;
- initial/bootstrap material representation;
- canonical proof/message binding;
- wire payload/message kind;
- persistence schema;
- durable atomic transaction mechanism;
- failover/recovery/re-baselining protocol.

Choosing these values now would invent production protocol/security state rather than reuse an authoritative reviewed representation.

### Gate B — actual Phase 141 integration-test dependency and Cargo validation

The future narrow test edge is reviewed as a `prw-remote-bridge` dev-dependency on `prw-nat-traversal`.

Adding it requires Cargo-tool materialization of the dependency graph/lockfile plus focused and full-workspace validation. The current Cargo/build gate is closed, so no manifest/lock mutation is authorized here.

### Gate C — concrete production upper owner / synchronization / runtime

Still unselected:

- final crate/module placement;
- production synchronization primitive;
- async/thread/task ownership;
- queue/cancellation mechanics;
- socket/network adapter;
- actual traversal replacement/restart runtime;
- Agent/bootstrap integration.

Selecting these now would be an architecture/runtime mutation.

### Gate D — implementation validation

Current source/tests remain staged until separately authorized execution of formatting, compiler/Clippy/tests/build validation.

Static review cannot substitute for executable evidence.

## Re-derivation guard

Future work should not reopen the already locked C02e semantics above merely because a concrete implementation mechanism is later selected.

A future freshness representation, persistence layer, actual Phase 141 integration or runtime owner must implement these invariants rather than redefine them unless a new explicit architecture/security review authorizes a change.

## Next safe work under current authority

With runtime-neutral semantics closed, the remaining safe work is **validation-tranche preparation and evidence synchronization**:

1. define the exact focused/static/executable validation scope that would be run when the build/Cargo gate is opened;
2. distinguish current-source validation from the later optional actual-Phase141 dev-edge validation so the latter cannot silently broaden dependency state;
3. synchronize mutable branch evidence to the real C02e head and full manifest;
4. continue monitoring the branch for concurrent corrections before any future gate transition.

No executable validation or dependency mutation is authorized by this checkpoint itself.

## Result

`C02E_SOURCE_DESIGN_GAP_REVIEW_PASS / CURRENT_RUNTIME_NEUTRAL_SEMANTICS_CLOSED / REMAINING_WORK_IS_REPRESENTATION_PROTOCOL_CARGO_VALIDATION_PRODUCTION_OWNER_OR_EXECUTION_GATED / C02D_UNTOUCHED`
