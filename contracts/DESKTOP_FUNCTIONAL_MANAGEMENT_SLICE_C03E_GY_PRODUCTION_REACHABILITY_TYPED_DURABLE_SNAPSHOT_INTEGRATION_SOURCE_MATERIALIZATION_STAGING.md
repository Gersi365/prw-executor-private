# Phase 152 C03e-GY — Production Reachability Typed Durable Snapshot Integration Source Materialization

Status: `STAGING`

Target gate:
`C03E_GY_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_MATERIALIZED`

## Purpose

C03e-GY source-materializes the C03e-GW typed durable-snapshot integration law under the corrected
C03e-GX compiler-complete four-path ceiling. It changes the bridge-owned durable snapshot from a live
`PeerConnectivityPlan` carrier to provider-neutral `PeerConnectivityPlanDurableState`, projects live
plans immediately before existing durable compare-and-commit calls, and restores loaded plan state
only through `PeerConnectivityPlan::from_durable_state(...)` before local owner installation.

GY is intentionally limited to source semantics and focused tests. It does not select or activate any
persistence byte codec, schema/version, keyspace, database/provider, credentials, process bootstrap,
owner-map population, runtime, networking, deployment, restart, or merge behavior.

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

No default-branch reconstruction, merge result, intermediate partial branch state, or unrelated later
head is a permitted GY base.

## Fresh exact-GX source audit

The exact-GX topology remains consistent with the GX correction:

1. `crates/prw-remote-bridge/src/reachability_owner.rs` still owns
   `ReachabilityDurableSnapshot`, `ReachabilitySnapshotError`, `ReachabilityDurableStore`, candidate
   commit, retirement, recovery, and reload.
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` still owns the focused
   in-memory durable-store production seam tests and direct durable snapshot construction.
3. `crates/prw-agent/src/production_reachability_owner_custody.rs` production code remains generic
   over the bridge store trait; only its existing `#[cfg(test)]` fixture directly constructs a
   `ReachabilityDurableSnapshot` and therefore requires compiler-only adaptation.
4. No manifest, lockfile, workflow, provider, codec, schema, runtime, networking, or additional source
   path is required by the known exact-GX topology.

Any fifth path remains a stop-and-re-audit condition.

## Materialized source law

### 1. Durable snapshot carries typed durable plan state

`ReachabilityDurableSnapshot` now owns:

- `PeerConnectivityPlanDurableState` as its plan member; and
- the existing `CandidatePublicationFreshnessRecord` as its freshness member.

The constructor accepts those typed durable values and rejects an exact-peer mismatch before the
snapshot can enter the durable store seam. A live `PeerConnectivityPlan` is no longer the persistence
carrier.

The `plan()` accessor returns `&PeerConnectivityPlanDurableState`, making the durable representation
explicit to store implementations without selecting any byte encoding or database schema.

### 2. Restoration failure remains a narrow snapshot classification

`ReachabilitySnapshotError` gains one narrow semantic restoration classification:

`PlanRestoration(ConnectivityError)`.

This preserves connectivity-owned validation detail while keeping invalid persisted plan semantics
separate from:

- persistence unavailability/ambiguity;
- missing durable state;
- exact-peer mismatch;
- definite stale expected-current CAS; and
- ordinary candidate-publication validation failure.

No new peer-visible wire error taxonomy is selected.

### 3. Candidate publication projects immediately before durable CAS

The existing commit ordering remains:

1. require current owner;
2. validate authenticated publication admission/currentness;
3. require exact presented freshness;
4. clone and completely validate the staged live plan;
5. issue exactly one verifier-owned replacement freshness token;
6. reject exact-current token repetition;
7. construct staged freshness;
8. project the fully validated staged live plan through `durable_state()`;
9. construct the typed durable snapshot;
10. await the existing expected-current durable CAS;
11. only after definite `Committed`, install the original staged live plan and staged freshness and
    invalidate prior traversal.

Projection itself performs no I/O, provider call, token generation, retry, allocation, or authority
transition.

### 4. Retirement projects the current live plan

Retirement preserves the existing currentness and expected-current checks. The current live plan is
projected through `durable_state()` and combined with the existing retired freshness tombstone before
the existing durable CAS.

No new peer lifecycle is created and no freshness token is minted during retirement.

### 5. Recovery restores before constructing an owner

After one authoritative `load_current(...)`, recovery:

1. rejects a snapshot whose durable plan-state peer or freshness peer differs from the requested exact
   peer lifecycle;
2. destructures the loaded typed snapshot;
3. restores the live plan only through `PeerConnectivityPlan::from_durable_state(...)`;
4. maps restoration failure to `ReachabilityOwnerError::Snapshot(PlanRestoration(...))`;
5. constructs the owner only after complete successful restoration.

No partially restored owner, plan, or freshness state is returned.

Storage absence still never creates new-lifecycle eligibility.

### 6. Reload is atomic with respect to restored plan/freshness installation

`reload_from_store(...)` keeps the current exact peer key from local owner state, then loads one
authoritative snapshot. Missing, ambiguous, cross-peer, or semantically invalid loaded state enters
`RecoveryRequired` and drops any current traversal.

For semantic restoration failure specifically, the loaded freshness is **not** installed independently
of the failed plan. Local plan/freshness replacement occurs only after `from_durable_state(...)`
returns a complete valid live plan.

### 7. Historical candidate-ID anti-reuse survives recovery

Because the durable snapshot now carries `PeerConnectivityPlanDurableState`, its historical
candidate-ID high-watermark crosses persistence. Recovery/reload through
`PeerConnectivityPlan::from_durable_state(...)` restores that high-watermark even when a historical
candidate is no longer active.

Consequently an ID removed before persistence cannot become reusable merely because the process
restarted or reloaded authoritative state.

### 8. Transient reachability observations remain non-durable

`PeerConnectivityPlanDurableState` contains no reachability observations. Recovery/reload therefore
reconstruct every active candidate observation as `Unknown` through the connectivity-owned restoration
boundary. GY does not add observation persistence or a second observation-authority channel.

## Focused validation materialized in GY

The bridge production-seam tests are adapted without deleting existing coverage and add focused proofs
for the new integration:

- successful candidate commit persists typed projected plan candidates/high-watermark and replacement
  freshness;
- historical high-watermark survives durable recovery with no active candidate and blocks removed-ID
  reuse;
- active candidates with missing persisted high-watermark fail recovery through
  `Snapshot(PlanRestoration(InvalidCandidateIdHighWatermark))`;
- invalid durable plan state during reload enters `RecoveryRequired`, drops traversal, and does not
  install loaded freshness independently of the failed plan;
- stale durable expected-state recovery/reload continues to work through the typed carrier;
- retirement persists typed projected plan state with the retired freshness tombstone.

The Agent custody test fixture changes only snapshot construction from the live test plan to
`plan.durable_state()`. Production custody code, owner-map semantics, lookup, and async lexical custody
remain unchanged.

## Exact authorized path set

GY may change exactly these four paths and no others:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`;
3. `crates/prw-agent/src/production_reachability_owner_custody.rs`;
4. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GY_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_MATERIALIZATION_STAGING.md`.

The Agent path is authorized only for its existing test fixture adaptation. A production-code diff in
that file beyond formatter movement is a scope violation.

## Explicit non-selection / non-activation

GY selects or activates no:

- persistence byte codec/framing;
- schema/version/migration law;
- database key encoding or keyspace;
- etcd, SQL, embedded database, or other concrete durable provider;
- provider revision mapping, leases, TTLs, watch loops, retries, or executor/runtime ownership;
- credentials, TLS, RBAC, secrets, or connection bootstrap;
- production owner-map population or Agent startup recovery orchestration;
- new-lifecycle/bootstrap freshness callsite;
- candidate handoff or current-Mesh terminal response activation;
- listener/readiness publication;
- traversal/dialing/networking activation;
- deployment, restart operation, merge, branch deletion, or repository visibility change.

## Explicit rejected interpretations

GY rejects:

- treating a live `PeerConnectivityPlan` as the durable snapshot member;
- reconstructing persisted plan state with `PeerConnectivityPlan::new(...)` and thereby discarding
  historical high-watermark state;
- persisting or restoring transient `Reachable`/`Unreachable` observations;
- installing loaded freshness after typed plan restoration failed;
- treating invalid/missing durable state as new-lifecycle eligibility;
- using typed-snapshot integration as authority to select a codec/provider/schema;
- broadening Agent production custody because its test fixture requires compilation adaptation;
- touching a fifth repository path without a separately documented exact-head correction.

## Validation gate

GY may close only when all of the following hold on one exact final head:

1. exact GX parent/merge base and ahead-only lineage;
2. exactly the four authorized changed paths;
3. no manifest, lockfile, workflow, provider/database, runtime, networking, deployment, or unrelated
   path mutation;
4. root and Android native lockfiles remain byte-stable unless a compiler-proven contradiction stops
   closure;
5. locked dependency graph succeeds;
6. rustfmt succeeds;
7. Clippy with warnings denied succeeds;
8. workspace tests succeed;
9. workspace build succeeds;
10. Android validation, if automatically triggered, succeeds on the same exact final head;
11. no failing or pending automatically triggered exact-head workflow remains;
12. immutable Drive audit is stored under the canonical Private Remote Workspace folder and verified
    by raw byte/hash readback.

Successful closure classification:
`CLOSED_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_MATERIALIZATION`.

Until closure, GY remains `STAGING`, draft, open, and unmerged.

## Safe successor boundary

After canonical GY closure, begin with a fresh exact-GY read-only audit. The next checkpoint must not
assume that codec/schema/provider materialization is now authorized. It must first determine the
narrowest unresolved prerequisite between provider-neutral durable representation, concrete byte
codec/schema/keyspace semantics, and concrete provider implementation. Any provider/runtime activation
remains separately gated.
