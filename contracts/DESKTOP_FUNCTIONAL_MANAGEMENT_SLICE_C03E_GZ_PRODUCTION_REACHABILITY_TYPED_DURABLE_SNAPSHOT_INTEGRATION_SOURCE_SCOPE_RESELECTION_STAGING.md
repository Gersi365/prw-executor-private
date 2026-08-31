# Phase 152 C03e-GZ — Production Reachability Typed Durable Snapshot Integration Source Scope Re-selection

Status: `STAGING`

Target gate:
`C03E_GZ_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_RESELECTED`

## Purpose

C03e-GZ is a documentation-only source-scope re-selection rooted directly at canonically closed
C03e-GX. It preserves the C03e-GW typed durable-snapshot integration semantics and corrects only the
source-materialization path ceiling after blocked C03e-GY exact-head compiler evidence proved that the
GX-selected four-path successor was still incomplete.

Blocked GY remains immutable blocker evidence, not predecessor authority and not an ancestor of GZ.
GZ performs no Rust/Kotlin source materialization and selects no persistence codec, schema/version,
keyspace, durable provider, credentials, owner-map population, bootstrap, runtime, networking,
deployment, restart, or merge behavior.

## Exact closed predecessor

GZ is rooted directly at canonically closed C03e-GX:

- predecessor PR: `#326`;
- exact GX branch:
  `phase-152-c03e-gx-production-reachability-typed-durable-snapshot-integration-source-scope-correction-semantics-selection-staging`;
- exact GX head: `c1b5350e59d86829e11a0c8f3a366dcb180d84f0`;
- exact GX tree: `580ab4c8a71a0de1c206154b4e10c5302e0d469c`;
- canonical GX gate:
  `C03E_GX_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_SEMANTICS_SELECTED`;
- GX remains intentionally draft/open/unmerged after canonical closure.

No blocked GY commit, PR merge ref, default branch, local reconstruction, or unrelated later head is a
permitted GZ base.

## Blocked GY evidence — not lineage authority

C03e-GY PR `#327` attempted the exact GX-authorized four-path source materialization.

Initial GY candidate:

- head: `d76d083438387e186cadca661f9398f52db83c3c`;
- tree: `14d5654b68ed1f0dcfb415e24b6ac2396f45809a`;
- exact GX merge base;
- four authorized paths only.

Initial exact-head Rust validation:

- PRW Rust Validation `#1402`;
- run `33392749544`;
- job `99490006331`;
- locked dependency graph: PASS;
- rustfmt: FAIL only;
- Clippy/tests/build: skipped after formatter failure.

The formatter diagnostics were confined to already-authorized bridge owner and bridge focused-test
paths. A mechanical formatting-only correction produced the second GY candidate without semantic or
path expansion.

Corrected blocked GY candidate:

- head: `811b171cf76c75de9f30a44bfc966c4d4205f7ab`;
- tree: `a5280e0b959cce0fe1a5128dc1e4c54251765b94`;
- exact GX merge base;
- ahead 2 / behind 0;
- final net changed paths still exactly four;
- no manifest/lockfile/workflow/provider/runtime/networking/deployment path changed.

Final blocked GY blobs:

1. `crates/prw-remote-bridge/src/reachability_owner.rs` —
   `8de2e3d21224b339a7d18e926f5127838c903608`;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` —
   `31dba5d8bd1209aa4b7f4e3a9970dcc3de912c3f`;
3. `crates/prw-agent/src/production_reachability_owner_custody.rs` —
   `f006a74e21492cae36e04aee06bc2b23a0206b7d`;
4. GY source-materialization contract —
   `d31b609b7777e201d89aceccfa86831284d4ead9`.

Corrected-head Rust validation:

- PRW Rust Validation `#1403`;
- run `33393165924`;
- job `99491362547`;
- locked dependency graph: PASS;
- rustfmt: PASS;
- Clippy: FAIL;
- workspace tests/build: skipped after Clippy failure.

The failure is a compile-time API propagation contradiction, not a semantic test assertion failure.

## Newly proven fifth compiler-required path

The exact #1403 compiler diagnostic identifies:

`crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`

Exact unchanged blob on blocked GY head:

`6bcc8695fffec073676d030cb50b69c4334ff50b`

That existing test path owns an in-memory implementation of `ReachabilityDurableStore` for freshness
wire/resynchronization tests. It stores `ReachabilityDurableSnapshot` values and contains two existing
snapshot-construction callsites that compile against the pre-GY live-plan constructor.

### Callsite 1 — common store fixture

`store_for(...)` constructs the lifecycle-specific freshness record and then calls:

`ReachabilityDurableSnapshot::new(fixture.plan.clone(), record)`

After the bridge owner snapshot constructor is materialized to accept
`PeerConnectivityPlanDurableState`, this test-only fixture must pass:

`fixture.plan.durable_state()`

instead of the live plan clone.

### Callsite 2 — resynchronization durable replacement fixture

`resync_reads_durable_state_each_time_and_returns_the_new_current_token()` replaces the in-memory
snapshot through:

`ReachabilityDurableSnapshot::new(fixture.plan.clone(), established_freshness)`

The same compiler-required test-only adaptation is:

`fixture.plan.durable_state()`

No production `reachability_freshness_wire` source change is selected by this evidence. The existing
production resynchronization law remains read-only with respect to durable current state and generates
no freshness token and performs no compare-and-commit.

## Why GY is blocked rather than widened

GX explicitly selected a maximum four-path source successor and stated that any fifth path is a
stop-and-re-audit condition. The exact #1403 diagnostic proves that condition occurred.

Therefore:

- GY gate was not achieved;
- GY must not absorb the fifth path;
- GY PR `#327` remains draft/open/unmerged and blocked;
- GY source heads remain evidence only;
- no GY CI result is reusable as successor closure authority;
- the source scope must be re-selected from exact CLOSED GX before any corrected source attempt.

This follows established PRW precedent where a blocked source attempt remains separate evidence and a
docs-only scope re-selection branches from the last canonically closed authority rather than from the
blocked source candidate.

## Corrected canonical source-scope law

A corrected source-materialization successor must begin with a fresh exact-GZ-head audit.

If that audit remains consistent with the known compiler topology, the authorized **maximum five-path
set** is exactly:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`;
3. `crates/prw-agent/src/production_reachability_owner_custody.rs`;
4. `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`;
5. one corrected source-materialization contract.

Path 3 is authorized only for the existing Agent `#[cfg(test)]` fixture adaptation from live plan to
`plan.durable_state()`. Production owner custody, map lookup, recovery delegation, async custody, and
Agent composition semantics in that file remain unchanged.

Path 4 is authorized only for the two existing freshness-wire/resynchronization test snapshot
constructors proven by #1403 and exact source inspection. It does not authorize production freshness
wire semantics, wire format, resynchronization ordering, bootstrap delivery behavior, token generation,
or durable mutation changes.

Any **sixth repository path** is a stop-and-re-audit condition. A later compiler error in another path
must not be silently folded into the corrected source checkpoint.

Repository code-search indexing returned no usable symbol hits for this repository, so GZ deliberately
does not claim an unsupported exhaustive symbol inventory. The five-path ceiling is the maximum scope
currently proven necessary by exact source and compiler evidence, with an explicit sixth-path stop.

## Preserved GW/GX semantic law

GZ changes scope only. It does not alter the semantics selected by GW and preserved by GX.

A corrected source successor must still materialize all of the following exactly.

### Typed durable snapshot carriage

`ReachabilityDurableSnapshot` plan state becomes `PeerConnectivityPlanDurableState`, paired with the
existing `CandidatePublicationFreshnessRecord` for the same exact peer lifecycle.

A live `PeerConnectivityPlan` is not the persistence-authority member of the snapshot.

### Snapshot peer coherence

Typed durable plan-state peer and freshness-record peer must match before the snapshot enters the
durable CAS seam. Peer mismatch remains fail-closed structural snapshot failure.

### Candidate commit projection

After complete staged candidate validation and verifier token issuance, the staged live plan is
projected through `durable_state()` immediately before durable snapshot construction. The existing
expected-current durable CAS remains unchanged. Only definite `Committed` installs the original
validated live staged plan and staged freshness locally.

### Retirement projection

Retirement projects the exact current live plan through `durable_state()` and combines it with the
existing retired freshness tombstone before the existing expected-current durable CAS.

### Recovery restoration

After `load_current(...)`, recovery checks exact peer coherence and restores live plan state only
through `PeerConnectivityPlan::from_durable_state(...)`. The production owner is constructed only
after complete successful restoration.

### Reload restoration and no partial install

`reload_from_store(...)` restores the complete typed plan before replacing local plan/freshness. On
missing, ambiguous, cross-peer, or semantically invalid durable state, traversal is dropped, the owner
is/enters `RecoveryRequired`, and loaded plan/freshness are not partially installed.

### Narrow restoration classification

A narrow snapshot/restoration classification may preserve the underlying `ConnectivityError` under
the existing owner Snapshot error channel. No broader peer-visible protocol error taxonomy is
selected.

### Historical anti-reuse

Recovery/reload through `from_durable_state(...)` preserves the complete historical candidate-ID
high-watermark so removed historical IDs cannot become reusable after restart/recovery.

### Transient observations remain non-durable

`PeerConnectivityPlanDurableState` carries no transient reachability observations. Restored candidate
observations are `Unknown`, and restored path selection cannot inherit prior transient `Reachable`
state.

## Preserved freshness resynchronization law

The newly authorized test-fixture path does not alter production freshness-resynchronization semantics.
A corrected source successor must preserve that:

- authenticated currentness is validated before durable lookup;
- authoritative durable state is loaded for the exact peer lifecycle;
- resynchronization re-delivers only the exact existing authoritative current freshness token;
- resynchronization performs no token generation;
- resynchronization performs no durable compare-and-commit;
- bootstrap delivery remains separately governed by an authoritative bootstrap record;
- wire framing and failure-code semantics remain unchanged.

Only test snapshot construction adapts to the new typed durable snapshot constructor.

## Dependency and lockfile boundary

No new package is required. The relevant crates already depend on the connectivity and bridge APIs
used by the existing paths.

No Cargo manifest or lockfile mutation is selected.

Expected unchanged lockfiles remain:

- root `Cargo.lock` blob `e5e1433660491fceb0fed54b48b20db78ef422cc`;
- Android native `Cargo.lock` blob `cce9ca06190a196661ab38d54a747893e26af95f`.

Any manifest/lockfile change is outside this five-path selection and stops closure.

## Explicit non-selection / non-activation

GZ selects no:

- persistence byte codec/framing;
- schema/version or migration policy;
- database key encoding/keyspace;
- etcd, SQL, embedded database, or other concrete durable provider;
- provider revision mapping, leases, TTLs, retries, or runtime bridge;
- credentials, TLS, RBAC, secrets, or connection bootstrap;
- production owner-map population or Agent startup recovery orchestration;
- new-lifecycle/bootstrap freshness issuance callsite;
- current-Mesh candidate handoff or response activation;
- worker/cancellation integration;
- listener/readiness/traversal/dialing/networking activation;
- deployment, process restart/recovery operation, merge, branch deletion, or repository visibility
  mutation.

## Explicit rejected interpretations

GZ rejects:

- adding the fifth path directly to blocked GY;
- treating blocked GY as predecessor authority;
- changing production freshness-wire/resynchronization semantics because its test fixture needs a
  typed-snapshot constructor adaptation;
- changing Agent production custody semantics because its test fixture needs the same adaptation;
- adding any sixth path without a new exact-head audit and separate scope correction;
- selecting codec/provider/schema/keyspace behavior while repairing typed snapshot integration;
- weakening durable anti-reuse or restoration failure semantics to avoid fixture changes;
- reconstructing persisted plan state through `PeerConnectivityPlan::new(...)`;
- treating missing or invalid durable state as new-lifecycle eligibility.

## Expected corrected successor

After canonical GZ closure, a corrected source-materialization successor must be separately named only
after a fresh exact-GZ audit confirms the five-path topology. GZ does not pre-authorize a checkpoint
name beyond this semantic/path boundary.

The corrected successor may reuse the blocked GY source design as audit evidence, but it must be
newly rooted from the exact closed GZ head rather than moving or rebasing the blocked GY branch.

## Validation gate

GZ may close only when all of the following hold on one exact final head:

1. exact GX parent/merge base and ahead-only lineage;
2. exactly one changed path — this scope re-selection contract;
3. blocked GY is not an ancestor of GZ;
4. no Rust/Kotlin source, Cargo manifest/lockfile, workflow, provider, runtime, networking, or
   deployment mutation;
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
`CLOSED_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_RESELECTION`.

Until closure, GZ remains `STAGING`, draft, open, and unmerged.
