# Phase 152 C03e-GZ — Production Reachability Typed Durable Snapshot Integration Source Scope-Correction II Semantics Selection

Status: `STAGING`

Target gate:
`C03E_GZ_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_II_SEMANTICS_SELECTED`

## Purpose

C03e-GZ is a documentation-only second source-scope correction rooted directly at canonically closed
C03e-GX. It records exact compiler evidence from the blocked C03e-GY source candidate without making
that failed/staging source candidate part of the authoritative successor lineage.

GX authorized a maximum four-path materialization after a fresh audit discovered one Agent test
fixture that directly constructed `ReachabilityDurableSnapshot`. GY then materialized those four paths
and exact-head validation proved an additional pre-existing bridge integration-test fixture that also
directly constructs `ReachabilityDurableSnapshot` from a live `PeerConnectivityPlan`.

Because that fifth path was outside the GX ceiling, GY correctly stopped rather than broadening in
place. GZ selects only the corrected compiler-complete source path ceiling. It does not itself
materialize source semantics and does not select any persistence codec, schema/version, keyspace,
durable provider, credentials, startup/runtime, networking, deployment, restart, or merge behavior.

## Exact authoritative predecessor

GZ is rooted directly at canonically closed C03e-GX:

- predecessor PR: `#326`;
- exact GX branch:
  `phase-152-c03e-gx-production-reachability-typed-durable-snapshot-integration-source-scope-correction-semantics-selection-staging`;
- exact GX head: `c1b5350e59d86829e11a0c8f3a366dcb180d84f0`;
- exact GX tree: `580ab4c8a71a0de1c206154b4e10c5302e0d469c`;
- GX canonical gate:
  `C03E_GX_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_SEMANTICS_SELECTED`;
- GX remains intentionally draft/open/unmerged.

The blocked GY source branch is audit evidence only and is not a permitted GZ parent or merge base.
No default branch, merge result, reconstructed local tree, or unrelated later head is a permitted base.

## Blocked GY compiler evidence

### Initial source candidate

GY PR `#329` was opened from exact GX with the GX-authorized four paths:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`;
3. `crates/prw-agent/src/production_reachability_owner_custody.rs` — test-fixture-only adaptation;
4. one GY source-materialization contract.

Initial GY head `cb6ca534f38b30647292aa1665107f91a8338f30` passed the locked dependency
graph but failed rustfmt. The formatter correction changed only the already-authorized bridge owner and
focused test paths, producing corrected head `a40a533ad92bfb1e0a672f7b419b46abb6b2fa04`
and tree `2d8a941617ba6308062dc5f8f889d0b43cb0c49e`.

### Exact corrected-head failure

PRW Rust Validation #1406 on exact corrected GY head `a40a533ad92bfb1e0a672f7b419b46abb6b2fa04`
proved:

- locked dependency graph: PASS;
- rustfmt: PASS;
- Clippy compilation: FAIL before Clippy lint completion, tests, or workspace build.

The compiler reported two exact type mismatches in:

`crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`

Both callsites pass a live `PeerConnectivityPlan` to the now-typed
`ReachabilityDurableSnapshot::new(...)` constructor:

- the shared `store_for(...)` fixture uses `fixture.plan.clone()`;
- the resynchronization replacement fixture uses `fixture.plan.clone()`.

The required adaptation is mechanically bounded to projecting the existing fixture plan through
`fixture.plan.durable_state()` before snapshot construction.

No product behavior, wire framing, resynchronization ordering, freshness generation, or store
semantics change is required by those callsites.

### Production freshness-wire source remains compatible

Exact blocked-GY production source
`crates/prw-remote-bridge/src/reachability_freshness_wire.rs` does not construct or restore durable
snapshots. Its authenticated resynchronization path loads the provider-neutral snapshot, checks
`snapshot.plan().peer()` and `snapshot.freshness().peer()`, then reads only the freshness lifecycle.

`PeerConnectivityPlanDurableState` exposes the same exact `peer()` identity accessor, so this
production source remains shape-compatible and does not require modification.

### Historical reference harnesses remain independent

Fresh exact-file audit confirms the candidate-freshness authority and bootstrap reference harnesses do
not directly use `ReachabilityDurableSnapshot`; they remain test-local semantic references over live
plans and test-local freshness lifecycle types. They require no change from typed persistence state.

## Corrected canonical source path ceiling

A source-materialization successor must begin with a fresh read-only audit from the exact closed GZ
head. If topology remains consistent, its maximum authorized changed-path set is exactly five paths:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`;
3. `crates/prw-agent/src/production_reachability_owner_custody.rs`;
4. `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`;
5. one source-materialization contract.

The Agent path remains authorized **only** for the existing `#[cfg(test)]` fixture projection. The
freshness-wire test path is authorized **only** for compiler-required typed snapshot fixture
projection and focused assertions if proven necessary; no production wire/source semantic change is
authorized by that path.

Any sixth repository path is a stop-and-re-audit condition. A manifest, lockfile, provider, workflow,
runtime, or unrelated source change must never be folded into the successor merely to make CI green.

## Preserved GW/GX semantic law

GZ corrects scope only. It does not alter or supersede the durable integration semantics selected by
GW and preserved by GX.

The source successor must still materialize:

- `ReachabilityDurableSnapshot` carrying `PeerConnectivityPlanDurableState` plus the existing exact
  peer freshness record;
- exact cross-member peer coherence before persistence;
- candidate-publication projection through `durable_state()` only after complete staged candidate
  validation and verifier token issuance and immediately before the existing durable CAS;
- local live plan/freshness installation only after definite durable `Committed`;
- retirement projection through `durable_state()` before the existing tombstone CAS;
- recovery and reload restoration only through `PeerConnectivityPlan::from_durable_state(...)`;
- one narrow snapshot/restoration classification preserving `ConnectivityError` where practical;
- no partially installed loaded freshness when plan restoration fails;
- recovery-required mode plus traversal invalidation on reload failure;
- complete historical candidate-ID high-watermark preservation across recovery/reload;
- transient reachability observations remaining non-durable and restored as `Unknown`.

## Required successor validation

Subject to a fresh exact-GZ audit, the next source successor should preserve all prior focused GY test
intent and additionally prove that existing freshness resynchronization fixtures compile and retain
unchanged semantics when their durable snapshots receive `fixture.plan.durable_state()`.

At minimum, exact-final-head validation must prove:

- successful typed durable recovery;
- historical high-watermark survives recovery and blocks removed-ID reuse;
- invalid durable high-watermark fails through the narrow restoration classification;
- invalid reload enters `RecoveryRequired`, drops traversal, and avoids partial freshness install;
- candidate commit persists typed projected plan state and advances freshness only after durable CAS;
- retirement persists typed projected plan state with retired freshness;
- authenticated freshness resynchronization remains a non-mutating durable load and exact current-token
  redelivery with zero token generation and zero durable compare-and-commit;
- resynchronization still reads authoritative durable state on each request and observes a replaced
  current token when the test store is replaced.

## Explicit non-selection / non-activation

GZ selects no:

- persistence byte codec or framing;
- schema/version or migration policy;
- database key encoding/keyspace;
- etcd, SQL, embedded database, or other concrete provider;
- provider revisions, transactions beyond the existing semantic CAS, leases, TTLs, watch loops, retry
  policy, or executor/runtime ownership;
- credentials, TLS, RBAC, secrets, or connection bootstrap;
- production owner-map population or Agent startup recovery orchestration;
- new-lifecycle/bootstrap freshness construction callsite;
- candidate handoff/current-Mesh response activation;
- worker/cancellation integration;
- listener/readiness/traversal/dialing/networking activation;
- deployment, restart operation, merge, branch deletion, or repository-visibility mutation.

## Explicit rejected interpretations

GZ rejects:

- adding the fifth path directly to blocked GY and calling the original GX ceiling satisfied;
- treating the blocked GY source head as canonically closed or as GZ's authoritative predecessor;
- modifying production freshness-wire logic merely because its integration test constructs durable
  fixtures;
- broadening Agent production custody because its test module needs typed projection;
- adding a sixth path without a fresh exact-head audit and separate correction;
- weakening restoration/high-watermark semantics to preserve an old fixture constructor call;
- selecting codec/provider/schema behavior while repairing compiler topology;
- treating missing or invalid durable state as new-lifecycle eligibility.

## Expected successor

After canonical GZ closure, the expected fresh source-materialization successor is C03e-HA. It must
begin from the exact closed GZ head and re-audit the five-path ceiling before any mutation.

If that audit remains consistent, HA may materialize the same GW/GX durable snapshot semantics across
the corrected five-path set. Any sixth path or dependency/provider/runtime contradiction must stop HA
rather than being silently absorbed.

## Validation gate

GZ may close only when all of the following hold on one exact final head:

1. exact GX parent/merge base and ahead-only lineage;
2. exactly one changed path — this GZ scope-correction contract;
3. no Rust/Kotlin source, Cargo manifest/lockfile, workflow, provider/database, runtime, networking,
   deployment, or unrelated repository mutation;
4. locked dependency graph succeeds;
5. rustfmt succeeds;
6. Clippy with warnings denied succeeds;
7. workspace tests succeed;
8. workspace build succeeds;
9. Android validation, if automatically triggered, succeeds on the same exact head;
10. no failing or pending automatically triggered exact-head workflow remains;
11. immutable Drive audit is stored under the canonical Private Remote Workspace folder and verified
    by raw byte/hash readback.

Successful closure classification:
`CLOSED_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_II_SEMANTICS_SELECTION`.

Until closure, GZ remains `STAGING`, draft, open, and unmerged.
