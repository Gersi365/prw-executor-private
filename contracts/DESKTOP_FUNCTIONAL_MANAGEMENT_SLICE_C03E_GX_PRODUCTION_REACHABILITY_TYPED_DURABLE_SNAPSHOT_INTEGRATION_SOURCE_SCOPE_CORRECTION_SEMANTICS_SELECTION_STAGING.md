# Phase 152 C03e-GX — Production Reachability Typed Durable Snapshot Integration Source Scope-Correction Semantics Selection

Status: `STAGING`

Target gate:
`C03E_GX_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_SEMANTICS_SELECTED`

## Purpose

C03e-GX is a documentation-only scope-correction checkpoint rooted directly at canonically closed
C03e-GW. It records a fresh exact-GW compiler-topology contradiction discovered before any GW-selected
source materialization was attempted.

GW correctly selected the typed durable-snapshot plan-state integration semantics, but its expected
narrow source successor named only the bridge owner source, the bridge focused production-seam tests,
and one source-materialization contract. A fresh exact-GW audit proves that one additional existing
Agent source path contains test fixtures that directly construct `ReachabilityDurableSnapshot` from a
live `PeerConnectivityPlan`. Changing the bridge snapshot plan member to
`PeerConnectivityPlanDurableState` therefore requires a compiler-only adaptation in that Agent test
module as part of the same source materialization.

GX selects only this corrected source-scope law. It does not source-materialize the GW semantics and
does not select or activate any persistence codec, schema/version, keyspace, durable provider,
credentials, owner-map population, runtime, networking, deployment, restart, or merge behavior.

## Exact predecessor

GX is rooted directly at canonically closed C03e-GW:

- predecessor PR: `#325`;
- exact GW branch:
  `phase-152-c03e-gw-production-reachability-typed-durable-snapshot-plan-state-integration-semantics-selection-staging`;
- exact GW head: `caf3db4f2fc9dd9431fd2a73706f20693f8a400d`;
- exact GW tree: `2387216eece7759365eacb99149f2e1d4d8ce70d`;
- GW gate:
  `C03E_GW_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_PLAN_STATE_INTEGRATION_SEMANTICS_SELECTED`;
- GW remains intentionally draft/open/unmerged.

No intermediate branch, merge result, default branch, reconstructed local tree, or later unrelated head
is a permitted GX base.

## Fresh exact-GW source audit

### 1. Bridge owner remains the semantic source seam

Exact GW `crates/prw-remote-bridge/src/reachability_owner.rs` remains blob
`fb7543361ea3a144ae9275284b41bf0ef63df2ad`.

It still:

- imports and stores live `PeerConnectivityPlan` inside `ReachabilityDurableSnapshot`;
- constructs snapshots from live plans in candidate commit and retirement;
- loads and installs the live plan directly in `recover(...)` and `reload_from_store(...)`;
- owns `ReachabilitySnapshotError` and the existing provider-neutral `ReachabilityDurableStore` seam.

This remains the sole production source path requiring semantic snapshot-shape integration.

### 2. Bridge focused production-seam tests require direct adaptation

Exact GW `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` remains blob
`8c14372258ba7cbbc8074d75d17e1d17fd9752f8`.

Its in-memory store retains `ReachabilityDurableSnapshot`, filters loaded snapshots through
`snapshot.plan().peer()`, seeds snapshots through `ReachabilityDurableSnapshot::new(...)`, and replaces
durable state in recovery/reload tests. These fixtures are the correct bridge-level place to prove
GW restoration and anti-reuse semantics after source materialization.

### 3. Fresh audit exposes one additional compiler-required Agent test path

Exact GW `crates/prw-agent/src/production_reachability_owner_custody.rs` contains production code that
remains generic over `ReachabilityDurableStore` and delegates recovery to
`ProductionReachabilityOwner::recover(...)`; production custody itself does not own durable snapshot
shape.

However its `#[cfg(test)]` module imports `ReachabilityDurableSnapshot` and constructs an established
snapshot by:

- creating a live empty `PeerConnectivityPlan`;
- creating typed established freshness;
- calling `ReachabilityDurableSnapshot::new(plan, freshness)`.

Once the bridge constructor accepts `PeerConnectivityPlanDurableState`, that existing test fixture no
longer compiles without a narrow projection adaptation. This is a direct compiler dependency, not a
new product feature or architecture decision.

### 4. Candidate-publication execution does not depend on snapshot shape

Exact GW `crates/prw-remote-bridge/src/candidate_publication_execution.rs` consumes only the generic
`ProductionReachabilityOwner`, `ReachabilityDurableStore`, and owner error/result surfaces. It does not
construct, destructure, persist, or inspect `ReachabilityDurableSnapshot`.

No change to that path is selected or expected.

### 5. Shared current capability authority does not depend on snapshot shape

Exact GW
`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs` owns
only the current registry/policy `RwLock` custody and bounded sync/async read operations. It does not
name or construct `ReachabilityDurableSnapshot`.

No change to that path is selected or expected.

### 6. Shared requester/rendezvous authority does not directly depend on snapshot shape

Exact GW
`crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`
remains generic over `ProductionReachabilityOwner` / `ReachabilityDurableStore` during dormant
candidate semantic execution. A direct exact-file search finds no `ReachabilityDurableSnapshot`
reference in that path.

No change to that path is selected or expected from the known exact-GW topology.

### 7. No new package is required

`prw-remote-bridge` already has a normal path dependency on `prw-connectivity`, and `prw-agent`
already depends on both the bridge and connectivity crates needed by its existing custody test
fixtures. GX selects no manifest or lockfile mutation.

## Corrected canonical source-scope law

A source-materialization successor must begin with a fresh audit from the exact closed GX head.

If that audit remains consistent, the authorized maximum changed-path set is exactly four paths:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`;
3. `crates/prw-agent/src/production_reachability_owner_custody.rs`;
4. one source-materialization contract.

The third path is authorized **only** for compiler-required adaptation and focused validation inside
its existing test module. GX does not authorize changing production custody ownership, map lookup,
recovery orchestration, async custody, runtime behavior, or Agent composition semantics in that file.

Any fifth repository path is a stop-and-re-audit condition. A compiler error in another path must not
be silently folded into the same source checkpoint.

## GW semantic law remains unchanged

GX corrects scope only. It does not modify or supersede the semantics selected by GW.

The source successor must still materialize all of the following exactly:

### Typed durable snapshot carriage

`ReachabilityDurableSnapshot` plan state becomes `PeerConnectivityPlanDurableState`, paired with the
existing `CandidatePublicationFreshnessRecord` for the same exact peer lifecycle.

A live `PeerConnectivityPlan` is not the persistence-authority member of the snapshot.

### Snapshot peer coherence

The typed durable plan-state peer and freshness-record peer must match before the snapshot enters the
durable CAS seam. Peer mismatch remains a fail-closed structural snapshot error.

### Candidate commit projection

After complete staged candidate validation and verifier token issuance, the staged live plan is
projected through `durable_state()` immediately before constructing the durable snapshot. The existing
durable expected-current CAS remains unchanged, and the original validated live staged plan becomes
local current state only after definite `Committed`.

Projection itself performs no I/O, provider call, token generation, candidate allocation, or authority
transition.

### Retirement projection

Retirement projects the current live plan through `durable_state()` and combines that typed state with
the existing retired freshness tombstone before the existing expected-current durable CAS.

### Recovery restoration

After `load_current(...)`, recovery checks exact peer coherence and restores the live plan only through
`PeerConnectivityPlan::from_durable_state(...)`. The owner is constructed only after complete valid
restoration. Invalid persisted plan semantics return no partially constructed owner.

### Reload restoration

`reload_from_store(...)` restores the complete typed plan before replacing local plan/freshness. On
missing, ambiguous, cross-peer, or semantically invalid durable state, traversal is dropped, the owner
is/enters `RecoveryRequired`, and no loaded plan/freshness pair is partially installed.

### Restoration failure classification

One narrow snapshot/restoration classification may preserve the underlying `ConnectivityError` where
practical. It remains distinct from persistence ambiguity, missing durable state, and definite stale
CAS. GX authorizes no broader peer-visible protocol error taxonomy.

### Historical anti-reuse remains durable

Recovery/reload through `from_durable_state(...)` must preserve the complete historical candidate-ID
high-watermark so a removed historical ID cannot become reusable after restart/recovery.

### Transient observations remain non-durable

Reachability observations are absent from `PeerConnectivityPlanDurableState`; all restored candidate
observations are `Unknown`, and restored selection cannot inherit prior transient `Reachable` state.

## Required focused validation in the source successor

Subject to fresh exact-GX audit, focused bridge tests should prove at least:

- successful recovery from typed durable state;
- historical high-watermark survives durable recovery and blocks removed-ID reuse;
- invalid/missing persisted high-water for active candidates fails recovery through the narrow
  snapshot/restoration classification;
- invalid durable state during reload enters `RecoveryRequired`, drops traversal, and does not install
  loaded freshness independently of the failed plan;
- successful commit persists typed projected plan state and advances freshness only after durable CAS;
- retirement persists typed projected plan state with the retired freshness tombstone.

The Agent custody test adaptation should prove only that its existing recovery/custody fixtures remain
valid when snapshot construction receives `plan.durable_state()` instead of the live plan. It must not
add production activation or new ownership semantics.

## Explicit non-selection / non-activation

GX selects no:

- persistence byte codec or framing;
- schema/version or migration policy;
- database key encoding/keyspace;
- etcd, SQL, embedded database, or other concrete provider;
- provider revision mapping, leases, TTLs, retry policy, or hidden runtime bridge;
- credentials, TLS, RBAC, secrets, or connection bootstrap;
- production owner-map population or Agent startup recovery orchestration;
- new-lifecycle/bootstrap freshness callsite;
- candidate handoff/current-Mesh response activation;
- worker/cancellation integration;
- listener/readiness/traversal/dialing/networking activation;
- deployment, restart/recovery operation, merge, branch deletion, or repository-visibility mutation.

## Explicit rejected interpretations

GX rejects all of the following:

- treating the discovered Agent test dependency as authority to broaden production Agent semantics;
- silently adding the Agent path to the GW-expected three-path source checkpoint without a scope
  correction;
- changing production custody code merely because the same file contains the compiler-required test;
- adding any fifth path without a new exact-head audit and separately documented correction;
- selecting a codec/provider/schema while repairing typed snapshot integration;
- weakening durable anti-reuse or restoration failure semantics to avoid test changes;
- using `PeerConnectivityPlan::new(...)` to reconstruct persisted plan state;
- treating missing or invalid durable state as new-lifecycle eligibility.

## Expected successor

After canonical GX closure, the expected source-materialization successor is C03e-GY, beginning from
the exact closed GX head with a fresh read-only audit.

If the exact topology remains consistent, GY may touch only the corrected four-path set above. A
fifth path, manifest/lockfile change, provider/schema decision, or runtime activation is a stop and
must be handled separately rather than folded into GY.

## Validation gate

GX may close only when all of the following hold on one exact final head:

1. exact GW parent/merge base and ahead-only lineage;
2. exactly one changed path — this scope-correction contract;
3. no Rust/Kotlin source, Cargo manifest/lockfile, workflow, provider, runtime, or networking mutation;
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
`CLOSED_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_SEMANTICS_SELECTION`.

Until closure, GX remains `STAGING`, draft, open, and unmerged.
