# Phase 152 C03e-BH — Explicit Connectivity Candidate Assembly Selection

Status: STAGED SELECTION

Gate target:
`C03E_BH_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SELECTED`

## 1. Exact predecessor

Closed C03e-BG:
- branch: `phase-152-c03e-bg-remote-admission-production-input-provenance-selection-staging`;
- head: `cc354fe4d49c1facbcd3629f3ae7776b8412c47d`;
- tree: `8bea2d08a7d20d857d10ee295e9dc3c66da2d604`;
- gate: `C03E_BG_REMOTE_ADMISSION_PRODUCTION_INPUT_PROVENANCE_SELECTED`.

BG remains authoritative for remote-admission input provenance and explicitly leaves candidate ID/path-kind/priority/candidate construction/publication separately gated.

## 2. Purpose

C03e-BD already materialized a provider-neutral projection from one observed `SocketAddr` into the existing validated `ConnectivityEndpoint` type.

That projection intentionally stops before candidate construction because a valid endpoint does not establish:
- a `CandidateId`;
- a `ConnectivityPathKind`;
- candidate priority/currentness;
- publication freshness;
- reachability;
- routability;
- authorization.

BH selects only one narrow provenance-preserving candidate-assembly boundary for a future source tranche:

`already-typed CandidateId + explicitly supplied ConnectivityPathKind + validated ConnectivityEndpoint -> ConnectivityCandidate`

BH does not select where the first two inputs come from.

This checkpoint does not infer path kind from an address and does not allocate or fabricate candidate identifiers.

## 3. Existing Phase 135 candidate authority remains unchanged

`prw-connectivity` remains authoritative for the candidate domain model.

A `ConnectivityCandidate` consists of exactly:
- one non-zero typed `CandidateId`;
- one explicit `ConnectivityPathKind`;
- one validated `ConnectivityEndpoint`.

The initial path-kind registry remains exactly:
- `LocalDirect`;
- `InternetDirect`;
- `Relay`.

The Phase 135 plan remains authoritative for:
- maximum candidate count;
- duplicate candidate-ID rejection;
- duplicate exact `(path kind, endpoint)` rejection;
- candidate-ID high-water/non-reuse rules across refresh;
- reachability observation storage;
- deterministic path selection.

BH does not duplicate or weaken any of those rules.

## 4. Endpoint scope classification is not inferred

The Phase 135 contract explicitly places endpoint scope classification in discovery/provider logic.

Therefore BH rejects all automatic mappings such as:
- RFC1918/private address -> `LocalDirect`;
- loopback -> `LocalDirect`;
- globally routable address -> `InternetDirect`;
- server-reflexive/STUN result -> automatically `InternetDirect`;
- non-private address -> automatically `InternetDirect`;
- any address/port shape -> `Relay`.

Address syntax and address scope are insufficient by themselves to prove product path class.

The future assembly helper must accept an already explicit typed `ConnectivityPathKind` and preserve it unchanged.

## 5. CandidateId provenance is not selected

`CandidateId` remains plan-scoped identity for candidate correlation only.

Its existing properties remain authoritative:
- raw value must be non-zero;
- within one plan lifetime a removed identifier cannot be reused;
- newly introduced identifiers must advance above the prior plan high-water mark;
- a retained identifier is valid only for the exact same path kind and endpoint.

BH does not select:
- an allocator;
- a counter owner;
- randomness;
- persistence;
- restart behavior;
- cross-process uniqueness;
- distributed allocation;
- derivation from IP/port;
- derivation from transport identity;
- derivation from DeviceId;
- derivation from request/session IDs.

The future assembly helper must accept an already constructed typed `CandidateId` and preserve it unchanged.

## 6. Selected assembly semantic

BH selects exactly one semantic operation:

```text
assemble_explicit_connectivity_candidate(
    candidate_id: CandidateId,
    path_kind: ConnectivityPathKind,
    endpoint: ConnectivityEndpoint,
) -> ConnectivityCandidate
```

The operation is an adapter around the existing `ConnectivityCandidate::new(...)` constructor only.

Required behavior:
1. accept only already-typed components;
2. preserve the exact `CandidateId` unchanged;
3. preserve the exact `ConnectivityPathKind` unchanged;
4. preserve the exact validated `ConnectivityEndpoint` unchanged;
5. construct exactly one `ConnectivityCandidate` through the existing domain constructor;
6. perform no I/O and no authority lookup;
7. perform no candidate publication or plan mutation.

The exact Rust function name may match the selected semantic name unless source constraints require a mechanically equivalent naming correction during materialization.

## 7. Why assembly returns no new validation result

All selected inputs are already validated domain types:
- `CandidateId` construction rejects zero;
- `ConnectivityEndpoint` construction rejects invalid address/port classes;
- `ConnectivityPathKind` is a closed enum;
- `ConnectivityCandidate::new` currently performs exact typed composition.

BH therefore does not invent a duplicate validation/error layer around typed assembly.

Any future raw/untrusted input must cross the existing typed constructors before this assembly boundary.

## 8. Relationship to C03e-BD endpoint projection

C03e-BD remains authoritative for:

`observed SocketAddr -> ConnectivityEndpoint`

BH does not rewrite BD or combine path-kind classification into the projection.

A future caller may perform:

```text
observed SocketAddr
    -> C03e-BD projection
    -> validated ConnectivityEndpoint
    + separately sourced CandidateId
    + separately sourced ConnectivityPathKind
    -> BH-selected candidate assembly
```

Failure of BD endpoint validation remains fail-closed before candidate assembly.

Successful BD projection still does not prove which candidate path kind applies.

## 9. Relationship to Phase 141 ICE/STUN

Phase 141 consumes existing PRW candidates for ICE correlation.

It does not own the Phase 135 candidate identifier or product path-kind classifier.

A selected ICE pair may produce a typed reachability observation for an existing candidate, but it does not retroactively authorize candidate-ID or path-kind fabrication.

BH therefore remains upstream of ICE observation correlation and does not activate traversal.

## 10. Host/server-reflexive ICE classes remain distinct from product path kind

Phase 141 models ICE candidate classes such as `Host` and `ServerReflexive`.

BH does not equate those protocol classes with Phase 135 product path kinds.

In particular:
- `Host` is not automatically `LocalDirect`;
- `ServerReflexive` is not automatically `InternetDirect`;
- a successful STUN mapping is not automatically publication authority;
- an ICE class is not a `CandidateId` source.

Any future mapping from discovery/traversal observations into product path kinds requires a separately reviewed provenance/classification checkpoint.

## 11. Relationship to relay

Phase 136/142 relay paths consume an already selected `Relay` candidate.

Relay provider/session success does not create a candidate path kind and does not allocate a Phase 135 candidate ID.

BH does not permit a caller to infer `Relay` merely because a relay provider or route token exists.

The `Relay` path-kind input to candidate assembly must already be explicitly selected by separately authorized provider/discovery logic.

## 12. Publication remains separately gated

Creating a `ConnectivityCandidate` does not publish it.

BH does not invoke:
- `publish_current_candidates`;
- authenticated publication admission;
- `PeerConnectivityPlan::refresh_candidates`;
- publication freshness/currentness checks;
- durable provider CAS;
- reachability live-owner authority;
- any network signaling operation.

Candidate construction is not candidate publication.

## 13. Identity separation

BH preserves all current identity boundaries:
- `DeviceId` is logical PRW device identity;
- authenticated PRW session identity is application identity;
- `TransportIdentity` is lower transport certificate identity;
- `CandidateId` is plan-scoped candidate correlation only;
- `ConnectivityEndpoint` is transient endpoint state;
- `ConnectivityPathKind` is product path classification;
- `SessionId` is authentication correlation only;
- request IDs are message correlation only.

No candidate field becomes authentication, authorization, membership or currentness evidence.

## 14. No readiness or routability claim

A constructed candidate is typed configuration only.

Candidate construction does not prove:
- the endpoint is reachable;
- the endpoint is externally routable;
- the endpoint belongs to the intended interface;
- firewall/NAT permits traffic;
- STUN/ICE succeeded;
- relay allocation exists;
- the candidate is current;
- the candidate was authenticated/published;
- the remote process is ready.

Those remain separate authority/evidence domains.

## 15. Selected future source ownership

The future materialization belongs in:

`crates/prw-remote-bridge/src/candidate_reachability.rs`

Rationale:
- C03e-BD endpoint projection already lives there;
- the module already owns candidate-publication semantics above `prw-connectivity`;
- `prw-remote-bridge` already depends on `prw-connectivity`;
- this avoids widening `prw-agent` with a direct `prw-connectivity` dependency merely for typed candidate assembly.

The lower `prw-connectivity` constructor remains unchanged and authoritative.

## 16. Intended source materialization shape

The first source tranche after BH may add only the selected pure adapter and focused non-networking tests.

Expected semantic shape:

```rust
pub const fn assemble_explicit_connectivity_candidate(
    candidate_id: CandidateId,
    path_kind: ConnectivityPathKind,
    endpoint: ConnectivityEndpoint,
) -> ConnectivityCandidate {
    ConnectivityCandidate::new(candidate_id, path_kind, endpoint)
}
```

Equivalent formatting/import placement is allowed.

The source tranche must not:
- accept raw `u64` as the assembly API instead of `CandidateId`;
- accept a raw IP/port instead of `ConnectivityEndpoint`;
- infer or rewrite `ConnectivityPathKind`;
- allocate candidate IDs;
- mutate a plan;
- publish candidates;
- perform network I/O.

## 17. Focused validation requirements for the future source tranche

Focused tests must prove at minimum:
1. exact function signature uses typed `CandidateId`, `ConnectivityPathKind`, and `ConnectivityEndpoint`;
2. exact candidate ID is preserved;
3. exact path kind is preserved for `LocalDirect`;
4. exact path kind is preserved for `InternetDirect`;
5. exact path kind is preserved for `Relay`;
6. exact endpoint is preserved;
7. no plan mutation is required;
8. no network operation occurs.

Workspace validation remains authoritative after source materialization.

## 18. Explicit non-selections

BH does not select or materialize:
- candidate-ID allocator/custody;
- path-kind classifier/provenance;
- endpoint discovery/interface enumeration;
- address-scope policy;
- candidate priority beyond existing Phase 135 path order;
- candidate publication signaling;
- publication freshness/currentness source;
- registry/database/provider mutation;
- reachability observations;
- STUN/ICE/TURN/relay activation;
- production bind-interface policy;
- externally-routable inference;
- expected-device provenance;
- SessionId/request-id production;
- remote capability dispatcher/provider backends;
- Agent `main.rs` activation;
- readiness/process-exit policy;
- retries/reconnect/rebind/rebootstrap/replacement;
- systemd/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- merge.

## 19. Exact intended BG -> BH scope

BH is docs-only.

The exact branch must differ from closed BG only by:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BH_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SELECTION_STAGING.md`

Any Rust source, Cargo manifest, lockfile, workflow, Agent, provider, registry, networking, packaging or deployment change blocks BH closure.

## 20. Closure conditions

BH can close only after:
- exact BG predecessor lineage remains unchanged;
- exact BG -> BH compare is one docs-only path;
- canonical Rust validation on the exact final BH head reaches terminal success;
- any automatically triggered workflow reaches a terminal non-failing verdict before closure;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No production source materialization is authorized merely by BH closure.

Gate target remains:
`C03E_BH_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SELECTED`
