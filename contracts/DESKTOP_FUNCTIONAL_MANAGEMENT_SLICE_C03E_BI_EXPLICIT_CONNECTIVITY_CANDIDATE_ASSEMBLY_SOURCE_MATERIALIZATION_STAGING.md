# Phase 152 C03e-BI — Explicit Connectivity Candidate Assembly Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Gate target:
`C03E_BI_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-BH:
- branch: `phase-152-c03e-bh-explicit-connectivity-candidate-assembly-selection-staging`;
- head: `cbf3496139b74063c7a663f9d7856b8f54208464`;
- tree: `b1fed74bfe83005934c63b5318cdeca41201fea5`;
- gate: `C03E_BH_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SELECTED`.

BH selected a pure provenance-preserving typed assembly boundary:

`CandidateId + ConnectivityPathKind + ConnectivityEndpoint -> ConnectivityCandidate`

BH explicitly did not select candidate-ID allocation/custody, path-kind classification/provenance, endpoint discovery, publication, reachability currentness, traversal activation, Agent activation or deployment.

## 2. Purpose

BI materializes exactly the pure source adapter selected by BH and focused non-networking tests.

It does not widen the selected semantics and does not resolve any of the provenance boundaries deliberately left open by BH.

## 3. Exact materialized helper

The materialized public helper is:

```rust
#[must_use]
pub const fn assemble_explicit_connectivity_candidate(
    candidate_id: CandidateId,
    path_kind: ConnectivityPathKind,
    endpoint: ConnectivityEndpoint,
) -> ConnectivityCandidate {
    ConnectivityCandidate::new(candidate_id, path_kind, endpoint)
}
```

The helper is owned by:

`crates/prw-remote-bridge/src/candidate_reachability.rs`

It is adjacent to the existing C03e-BD endpoint projection and above the unchanged `prw-connectivity` domain constructor.

## 4. Exact preservation semantics

The helper:
1. accepts an already-validated typed `CandidateId`;
2. accepts an already-explicit typed `ConnectivityPathKind`;
3. accepts an already-validated typed `ConnectivityEndpoint`;
4. preserves the exact candidate ID unchanged;
5. preserves the exact path kind unchanged;
6. preserves the exact endpoint unchanged;
7. delegates directly to existing `ConnectivityCandidate::new(...)`;
8. returns the resulting typed `ConnectivityCandidate`.

No raw identifier or raw endpoint input is accepted by this helper.

## 5. No candidate-ID allocation or custody

BI does not allocate, sequence, randomize, persist or derive `CandidateId` values.

The existing Phase 135 rules remain authoritative:
- zero is invalid at typed construction;
- IDs are plan-scoped correlation only;
- retired IDs are not reusable in a plan lifetime;
- newly introduced IDs must respect the plan high-water rule;
- retained IDs remain bound to the exact same path kind and endpoint.

BI does not create a new allocator or move those rules into the bridge.

## 6. No path-kind inference

BI does not infer or rewrite `ConnectivityPathKind` from:
- IP address family;
- private/public address ranges;
- loopback status;
- interface ownership;
- STUN server-reflexive state;
- ICE Host/ServerReflexive class;
- relay-provider availability;
- port number;
- `SocketAddr` shape;
- `DeviceId` or `TransportIdentity`.

The supplied typed path kind is preserved exactly.

`Host` is not automatically `LocalDirect` and `ServerReflexive` is not automatically `InternetDirect`.

## 7. Relationship to C03e-BD

C03e-BD remains authoritative for:

`observed SocketAddr -> validated ConnectivityEndpoint`

BI does not change that projection and does not add classification to it.

A caller may compose BD and BI only when separate authoritative sources have already supplied a typed candidate ID and explicit path kind.

Successful endpoint projection alone remains insufficient to select a path kind.

## 8. Existing lower domain authority remains unchanged

`crates/prw-connectivity/src/lib.rs` remains the authoritative lower domain model for:
- `CandidateId`;
- `ConnectivityEndpoint`;
- `ConnectivityPathKind`;
- `ConnectivityCandidate`;
- `PeerConnectivityPlan`;
- duplicate/capacity checks;
- candidate-ID high-water/non-reuse;
- observations;
- deterministic path selection.

BI must leave this lower source byte-stable relative to BH.

## 9. Publication remains unchanged and separate

BI does not change the existing authenticated candidate publication operations.

The new helper does not call or bypass:
- `publish_current_candidates`;
- `validate_authenticated_publication_admission`;
- `refresh_from_authenticated_publication`;
- `PeerConnectivityPlan::new`;
- `PeerConnectivityPlan::refresh_candidates`;
- publication freshness verification;
- durable provider CAS;
- live-owner authority.

Constructing one candidate is not publication, currentness or reachability evidence.

## 10. Focused tests materialized

BI adds focused pure tests in the same module.

### Exact typed signature

`explicit_candidate_assembly_has_exact_selected_shape`

Proves the helper has exact callable shape:

```text
fn(CandidateId, ConnectivityPathKind, ConnectivityEndpoint) -> ConnectivityCandidate
```

### Candidate ID and endpoint preservation

`explicit_candidate_assembly_preserves_candidate_id_and_endpoint`

Uses a typed non-zero candidate ID and validated documentation endpoint, then proves the returned candidate preserves both exactly.

### Path-kind preservation

`explicit_candidate_assembly_preserves_each_explicit_path_kind`

Runs the same typed assembly through:
- `LocalDirect`;
- `InternetDirect`;
- `Relay`;

and proves the exact supplied path kind, candidate ID and endpoint are preserved for each case.

These tests are non-networking and do not mutate a peer plan or publication owner.

## 11. Existing C03e-BD tests remain intact

The existing endpoint-projection tests remain present and continue to cover:
- exact signature;
- IPv4 preservation;
- IPv6 preservation;
- zero-port rejection;
- unspecified-address rejection;
- multicast rejection;
- IPv4 limited-broadcast rejection.

BI does not weaken the endpoint validation boundary.

## 12. Source scope

The intended BH -> BI final scope is exactly two paths:

1. `crates/prw-remote-bridge/src/candidate_reachability.rs`;
2. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BI_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SOURCE_MATERIALIZATION_STAGING.md`.

No Cargo manifest, lockfile, workspace root, Agent, workflow, provider, registry, systemd, packaging or networking path is authorized to change.

## 13. Dependency boundary

BI adds no external or internal dependency.

`prw-remote-bridge` already depends on `prw-connectivity`, so no Agent dependency widening is required.

The lower `prw-connectivity` constructor remains unchanged.

## 14. Identity and authority invariants

BI preserves:
- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower-transport certificate identity only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityPathKind` as product path classification only;
- `SessionId` as authentication correlation only;
- request IDs as message correlation only.

Candidate assembly does not become authentication, authorization, membership, publication freshness/currentness, reachability, routability or readiness evidence.

## 15. Explicit non-materializations

BI does not materialize or select:
- candidate-ID allocator/custody;
- candidate path-kind classifier/provenance;
- endpoint discovery/interface enumeration;
- address-scope/public-routability policy;
- candidate ranking beyond existing Phase 135 order;
- candidate publication signaling;
- publication freshness/currentness source;
- registry/database/provider mutation;
- STUN/ICE/TURN/relay activation;
- production bind-interface policy;
- expected-device rendezvous/provenance;
- SessionId generator;
- authentication request-id allocator;
- remote capability dispatcher/provider backends;
- Agent `main.rs` wiring;
- readiness or process-exit policy;
- retry/reconnect/rebind/rebootstrap/replacement;
- systemd/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 16. Validation and closure conditions

BI can close only after:
- exact BH predecessor lineage remains unchanged;
- exact BH -> BI final compare is exactly the two intended paths;
- lower `prw-connectivity` source remains byte-stable;
- canonical Rust validation on the exact final BI head is terminal success;
- Android validation, if automatically triggered, reaches terminal success including native adapter and application validation;
- any other automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No production activation is authorized by BI closure.

Gate target remains:
`C03E_BI_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SOURCE_MATERIALIZED`
