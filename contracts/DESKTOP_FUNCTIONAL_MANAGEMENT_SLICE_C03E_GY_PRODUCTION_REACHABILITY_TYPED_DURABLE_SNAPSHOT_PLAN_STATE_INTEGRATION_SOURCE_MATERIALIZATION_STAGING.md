# Phase 152 C03e-GY — Production Reachability Typed Durable Snapshot Plan-State Integration Source Materialization

Status: `STAGING`

Target gate:
`C03E_GY_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_PLAN_STATE_INTEGRATION_SOURCE_MATERIALIZED`

## Purpose

C03e-GY materializes only the C03e-GW-selected typed durable-snapshot plan-state integration semantics
under the C03e-GX-corrected source path ceiling. It changes the provider-neutral reachability-owner
snapshot from a live `PeerConnectivityPlan` member to `PeerConnectivityPlanDurableState`, projects live
plans immediately before the existing durable compare-and-commit seam, and restores loaded durable
plan state through `PeerConnectivityPlan::from_durable_state(...)` before it can become local current
owner state.

GY also performs the exact compiler-required Agent custody test-fixture adaptation selected by GX and
adds focused bridge seam tests for restoration, historical candidate-ID anti-reuse, no-partial-install
reload failure, and typed plan-state persistence at candidate and retirement commit points.

GY selects no persistence codec, schema/version, keyspace, concrete database/provider, credentials,
owner-map population, bootstrap, runtime, networking, deployment, restart, or merge behavior.

## Exact predecessor

GY is rooted directly at canonically closed C03e-GX:

- predecessor PR: `#326`;
- exact GX branch:
  `phase-152-c03e-gx-production-reachability-typed-durable-snapshot-integration-source-scope-correction-semantics-selection-staging`;
- exact GX head: `c1b5350e59d86829e11a0c8f3a366dcb180d84f0`;
- exact GX tree: `580ab4c8a71a0de1c206154b4e10c5302e0d469c`;
- GX gate:
  `C03E_GX_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_SEMANTICS_SELECTED`;
- GX remains intentionally draft/open/unmerged.

No intermediate branch, merge result, default branch, local reconstruction, or unrelated head is a
permitted GY base.

## Authorized changed-path set

C03e-GX authorizes a maximum four-path source successor. GY uses exactly those four paths:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`;
3. `crates/prw-agent/src/production_reachability_owner_custody.rs`;
4. this source-materialization contract.

The Agent path changes only its existing `#[cfg(test)]` durable-snapshot fixture. Production custody,
map lookup, recovery delegation, bounded mutable access, async custody, and runtime composition in
that file are unchanged.

Any fifth path is outside GY and requires a separately bounded successor.

## Materialized bridge-owner semantics

### Typed durable snapshot carriage

`ReachabilityDurableSnapshot` now carries:

- `PeerConnectivityPlanDurableState` for provider-neutral durable plan semantics; and
- the existing `CandidatePublicationFreshnessRecord` for the same exact peer lifecycle.

The constructor accepts typed durable plan state, not a live `PeerConnectivityPlan`, and rejects an
exact peer mismatch before the snapshot can enter the persistence transaction seam.

Transient reachability observations therefore cannot be embedded in the durable snapshot through the
owner path.

### Narrow restoration failure classification

`ReachabilitySnapshotError` retains `PeerMismatch` and adds only:

`PlanRestoration(ConnectivityError)`

This preserves the connectivity-owned semantic restoration classification without inventing a new
peer-visible protocol error taxonomy. `ReachabilityOwnerError::Snapshot(...)` remains the existing
owner-level carrier.

### Candidate publication projection ordering

The existing candidate commit ordering remains:

1. require current owner;
2. validate authenticated publication admission against the live current plan;
3. require exact presented freshness;
4. clone and fully validate the staged live candidate plan;
5. issue exactly one replacement verifier freshness token;
6. reject exact-current replacement freshness;
7. construct staged established freshness;
8. project the fully validated staged live plan through `durable_state()`;
9. construct the peer-consistent typed durable snapshot;
10. await the existing expected-current durable compare-and-commit;
11. only on definite `Committed`, install the original validated live staged plan and staged
    freshness locally and invalidate any old traversal.

Projection performs no I/O, provider call, retry, token generation, authority transition, or candidate
allocation.

### Retirement projection

Retirement now projects the exact current live plan through `durable_state()` and pairs that typed
state with the existing retired freshness tombstone before the existing expected-current durable CAS.

Retirement still creates no replacement peer lifecycle and does not generate a new freshness token.

### Recovery restoration

`ProductionReachabilityOwner::recover(...)` still performs exactly one authoritative durable load.
After load it:

1. requires exact snapshot plan-state peer and freshness peer equality with the requested peer;
2. destructures the loaded snapshot;
3. restores the live plan only through `PeerConnectivityPlan::from_durable_state(...)`;
4. maps restoration failure to `ReachabilityOwnerError::Snapshot(PlanRestoration(...))`;
5. computes mode from the loaded freshness lifecycle;
6. constructs the owner only after complete successful plan restoration.

Missing or ambiguous storage remains fail-closed. Invalid persisted plan semantics return no partially
constructed owner and never become new-lifecycle eligibility.

### Reload restoration and no partial install

`reload_from_store(...)` still loads against the exact current peer. Missing, ambiguous, or cross-peer
state enters `RecoveryRequired` and drops traversal as before.

For a peer-consistent snapshot, GY restores the complete typed plan through
`PeerConnectivityPlan::from_durable_state(...)` before assigning either loaded plan or loaded
freshness. If restoration fails:

- traversal is dropped;
- mode is/enters `RecoveryRequired`;
- the previous local plan remains unchanged;
- the previous local freshness remains unchanged;
- loaded freshness is not independently installed.

Only a completely restored plan/freshness pair replaces local state.

## Historical anti-reuse and transient observation law

Because persistence now carries `PeerConnectivityPlanDurableState`, recovery/reload preserve the full
historical candidate-ID high-watermark. A candidate ID removed before persistence cannot become
reusable merely because the process restarted or the owner recovered.

The durable carrier omits transient reachability observations. Restoration initializes observations to
`Unknown` through the existing connectivity-owned restoration law; GY does not add an alternate
observation persistence path.

## Focused bridge validation materialized

The existing production seam tests are adapted to construct snapshots from `plan.durable_state()` and
now additionally prove:

- successful candidate commit persists a typed durable plan state equal to the committed live plan
  projection while freshness advances only on durable commit;
- recovery from a typed durable state preserves a historical candidate-ID high-watermark and rejects
  reuse of a removed historical ID;
- typed durable state with active candidates but missing high-watermark fails recovery through
  `ReachabilitySnapshotError::PlanRestoration(ConnectivityError::InvalidCandidateIdHighWatermark)`;
- invalid typed durable state during `reload_from_store(...)` enters `RecoveryRequired`, drops the
  current traversal, preserves the previous local plan, and does not independently install loaded
  freshness;
- retirement persists a typed durable plan projection beside the retired freshness tombstone.

Existing stale-CAS, ambiguous persistence, candidate validation, post-commit traversal, and retirement
coverage remains in place.

## Agent custody compiler-only adaptation

The existing Agent custody test helper previously constructed:

`ReachabilityDurableSnapshot::new(plan, freshness)`

It now constructs:

`ReachabilityDurableSnapshot::new(plan.durable_state(), freshness)`

No production statement before `#[cfg(test)]` in that file changes. This adaptation adds no Agent
provider, owner-map population, startup recovery, synchronization, runtime, listener, readiness,
networking, or deployment behavior.

## Dependency and lockfile boundary

GY uses only existing public APIs from the already-present `prw-connectivity` dependency. It selects
no Cargo manifest or lockfile mutation and no package/version change.

Expected unchanged lockfiles:

- root `Cargo.lock` blob `e5e1433660491fceb0fed54b48b20db78ef422cc`;
- Android native `Cargo.lock` blob `cce9ca06190a196661ab38d54a747893e26af95f`.

## Explicit non-selection / non-activation

GY selects no:

- persistence byte codec/framing;
- schema/version or migration policy;
- database key encoding or keyspace;
- etcd, SQL, embedded database, or other concrete durable provider;
- provider revision mapping, leases, TTLs, retry policy, or runtime bridge;
- credentials, TLS, RBAC, secrets, or connection bootstrap;
- production owner-map population or Agent startup recovery orchestration;
- new-lifecycle/bootstrap freshness callsite;
- current-Mesh candidate handoff or response activation;
- worker/cancellation integration;
- listener/readiness/traversal/dialing/networking activation;
- deployment, process restart/recovery operation, merge, branch deletion, or repository-visibility
  mutation.

## Explicit rejected interpretations

GY rejects:

- persisting the live `PeerConnectivityPlan` as snapshot authority;
- reconstructing persisted plans through `PeerConnectivityPlan::new(...)` instead of
  `from_durable_state(...)`;
- dropping historical high-watermark state during recovery;
- persisting transient reachability observations;
- installing loaded freshness when typed plan restoration failed;
- converting invalid/missing durable state into new-lifecycle eligibility;
- changing Agent production custody semantics merely because its existing test fixture required a
  compiler adaptation;
- adding a fifth path, manifest/lockfile mutation, codec/provider/schema decision, runtime activation,
  or deployment behavior under GY.

## Validation gate

GY may close only when all of the following hold on one exact final head:

1. exact GX parent/merge base and ahead-only lineage;
2. changed paths are exactly the four authorized paths above and no fifth path exists;
3. Agent production custody code remains semantically unchanged outside its existing test module;
4. no Cargo manifest/lockfile, workflow, provider, schema, runtime, networking, or deployment path
   changes;
5. locked dependency graph succeeds;
6. rustfmt succeeds;
7. Clippy with warnings denied succeeds;
8. workspace tests succeed;
9. workspace build succeeds;
10. Android validation, if automatically triggered, succeeds on the same exact head;
11. no failing or pending automatically triggered exact-head workflow remains;
12. immutable Drive audit is stored under the canonical Private Remote Workspace folder and verified
    by raw byte/hash readback.

Successful closure classification:
`CLOSED_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_PLAN_STATE_INTEGRATION_SOURCE_MATERIALIZATION`.

Until closure, GY remains `STAGING`, draft, open, and unmerged.

## Safe successor boundary

After canonical GY closure, the next step must start from a fresh exact-GY-head audit. GY does not
pre-authorize a persistence codec/provider/schema, owner-map startup population, new-lifecycle
bootstrap, current-Mesh response activation, listener/runtime activation, networking, deployment, or
merge. Any such materialization requires its own explicit semantics and source boundary.
