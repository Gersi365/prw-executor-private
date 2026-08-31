# Phase 152 C03e-GW — Production Reachability Typed Durable Snapshot Plan-State Integration Semantics Selection

Status: `STAGING`

Target gate:
`C03E_GW_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_PLAN_STATE_INTEGRATION_SEMANTICS_SELECTED`

## Purpose

C03e-GW selects only the semantic integration law that connects the C03e-GV-materialized
`PeerConnectivityPlanDurableState` to the existing provider-neutral production reachability durable
snapshot/load/CAS seam.

GW does not source-materialize that integration and does not select a persistence byte codec,
schema/version, migration policy, database keyspace, concrete durable-store provider, credentials,
production owner population/recovery orchestration, candidate handoff, runtime/networking,
deployment, or merge.

## Exact predecessor

GW is rooted directly at canonically closed C03e-GV:

- predecessor PR: `#324`;
- exact GV branch:
  `phase-152-c03e-gv-production-reachability-durable-plan-state-projection-restoration-source-materialization-staging`;
- exact GV head: `07a0fc9e854194f189e4990fb78ceb19759b89f7`;
- exact GV tree: `d95ac4a95282eaaa2bdf1fcb6bb19b572b5647e2`;
- GV gate:
  `C03E_GV_PRODUCTION_REACHABILITY_DURABLE_PLAN_STATE_PROJECTION_RESTORATION_SOURCE_MATERIALIZED`;
- GV remains intentionally draft/open/unmerged.

No intermediate branch, merge result, `main`, reconstructed local state, or later unrelated head is a
permitted GW base.

## Fresh exact-GV-head audit

The exact GV head confirms all of the following.

### Existing bridge durable snapshot still carries the live plan

`crates/prw-remote-bridge/src/reachability_owner.rs` still defines
`ReachabilityDurableSnapshot` as:

- one live `PeerConnectivityPlan`;
- one `CandidatePublicationFreshnessRecord`.

Its module-level and trait documentation still state that persistence serialization format,
database product, replication/transaction implementation, and concrete provider remain outside the
bridge semantic owner.

### Existing durable owner operations still install/load the live plan directly

The production owner currently:

- clones a staged live plan into `ReachabilityDurableSnapshot` before candidate durable CAS;
- clones the current live plan into the retirement snapshot before retirement durable CAS;
- receives a live plan directly from `load_current(...)` during `recover(...)`;
- receives and installs a live plan directly during `reload_from_store(...)`.

Therefore the GV typed durable projection/restoration boundary is not yet part of the authoritative
reachability persistence seam.

### GV now provides the exact typed plan-state carrier required by that seam

`crates/prw-connectivity/src/lib.rs` now owns provider-neutral
`PeerConnectivityPlanDurableState`, containing exactly:

- exact `PeerConnectivityIdentity`;
- complete current typed candidate vector in exact plan order;
- exact optional historical `CandidateId` high-watermark.

`PeerConnectivityPlan::durable_state()` projects live plan state while dropping transient
reachability observations.

`PeerConnectivityPlan::from_durable_state(...)` validates durable candidate/high-water semantics and
restores all observations as `Unknown`.

### Freshness is already represented as typed durable semantic state

`CandidatePublicationFreshnessRecord` remains the logical persistence-neutral freshness schema
boundary for the same exact peer lifecycle. No second freshness representation is required for this
checkpoint.

### Historical checkpoints preserve the downstream persistence split

C03e-GQ selected only the awaitable durable execution-model prerequisite and explicitly left
provider selection plus schema/serialization separately gated.

C03e-GR materialized the provider-neutral awaitable load/CAS execution path and explicitly closed
without a concrete persistence provider/schema/credentials.

C03e-GU selected the typed plan durable projection/restoration semantics while explicitly excluding
byte codec/schema/version, keyspace and provider.

C03e-GV materialized only that typed connectivity carrier/projection/restoration and preserved the
same exclusions.

No later exact-GV source establishes a conflicting codec/provider/keyspace selection.

### Dependency topology does not require a new package for the expected source successor

`prw-remote-bridge` already has a normal path dependency on `prw-connectivity`.

Therefore the semantic integration selected here is expected to materialize through existing types
without adding a serialization/database dependency. Any future compiler evidence to the contrary is
a stop-and-re-audit condition, not authority to silently broaden scope.

## Selected canonical law

### 1. Durable snapshot plan member is typed durable plan state

The future production reachability durable snapshot must carry:

- `PeerConnectivityPlanDurableState` for the plan portion; and
- `CandidatePublicationFreshnessRecord` for freshness/lifecycle state.

The durable snapshot must not carry a live `PeerConnectivityPlan` as persistence authority.

The typed plan-state member contains no reachability observations and therefore cannot restore stale
`Reachable`/`Unreachable` evidence after restart/recovery.

### 2. Snapshot peer coherence remains mandatory

One durable snapshot must describe one exact peer lifecycle.

Before a snapshot may enter the durable CAS seam, its typed plan-state peer and freshness-record peer
must be equal.

A peer mismatch remains a structural snapshot failure. GW does not weaken, remove, or reinterpret
the existing exact-peer boundary.

### 3. Typed plan semantic validation occurs at plan restoration

`PeerConnectivityPlanDurableState::from_parts(...)` is intentionally a decoded/provider-neutral
carrier construction boundary, not the final authority for plan invariants.

Loaded typed plan state becomes a usable live plan only through
`PeerConnectivityPlan::from_durable_state(...)`.

Invalid durable candidate/high-water state must fail closed as a semantic snapshot/restoration
failure. It must not be treated as a successfully recovered plan and must not be silently repaired,
rebaselined, truncated, renumbered, or reconstructed through `PeerConnectivityPlan::new(...)`.

### 4. Candidate publication commit ordering gains exactly one projection step

The existing production ordering remains authoritative:

1. current-owner requirement;
2. authenticated identity/workspace/transport admission;
3. exact current freshness equality;
4. complete candidate validation on a staged live plan;
5. exactly one fresh verifier token issuance;
6. staged replacement freshness construction;
7. typed durable plan-state projection from the fully validated staged live plan;
8. peer-consistent durable snapshot construction;
9. durable expected-current compare-and-commit;
10. local live plan/freshness install only after definite `Committed`;
11. old traversal invalidation at that same successful local-install boundary.

Projection performs no provider call, no I/O, no token generation, no candidate allocation and no
authority transition by itself.

If projection/snapshot construction fails, no durable CAS occurs and current local state remains
unchanged.

### 5. Candidate durable commit stores no transient reachability observation

Because the staged live plan is projected through `durable_state()`, the durable replacement carries
only current configured candidates plus historical candidate-ID high-water state.

Any local/transient reachability observation is outside the durable snapshot and cannot become
post-recovery authority.

### 6. Retirement uses the same typed durable projection

Retirement must project the current live plan through `durable_state()` before constructing the
retired durable snapshot.

The snapshot then combines:

- typed plan durable state; and
- the existing `CandidatePublicationFreshnessRecord::retired(...)` tombstone.

Retirement projection does not allocate a candidate ID, mint freshness, preserve observations, or
create a replacement peer lifecycle.

The existing expected-current durable CAS law remains unchanged.

### 7. Recovery restores before owner construction

`ProductionReachabilityOwner::recover(...)` remains load-only with zero freshness generation.

After authoritative `load_current(peer)` returns a snapshot, recovery must:

1. require the snapshot plan-state peer and freshness peer to match the exact requested peer;
2. restore the live plan only through `PeerConnectivityPlan::from_durable_state(...)`;
3. construct the owner only after the complete restoration succeeds;
4. install no traversal session during recovery;
5. derive owner mode from the loaded durable freshness lifecycle exactly as before.

Missing state remains `DurableStateMissing`; persistence ambiguity remains persistence failure.

An invalid typed durable plan must fail closed and return no partially constructed owner.

Storage absence or invalid durable state must never be interpreted as implicit new-lifecycle
eligibility.

### 8. Reload restores before local installation

`reload_from_store(...)` must restore the loaded typed durable plan completely before replacing the
local cached plan/freshness.

If load is missing, ambiguous, cross-peer, or semantically invalid:

- any current traversal is dropped;
- owner mode is/enters `RecoveryRequired`;
- no partially restored plan is installed;
- no loaded freshness is installed independently of a failed plan restoration;
- prior local plan/freshness may remain only as non-authoritative last-known cache under
  `RecoveryRequired` semantics.

Only a complete valid snapshot restoration may atomically replace the local plan/freshness pair and
return the owner to the mode derived from durable lifecycle state.

### 9. Restoration errors are semantic, not a new storage success

A durable typed plan that decodes into structurally representable state but fails
`PeerConnectivityPlan::from_durable_state(...)` is not a successful recovery.

The future bridge source must expose one narrow stable snapshot/restoration error classification for
that failure, preserving the underlying connectivity semantic classification where practical.

That error is distinct from:

- `ReachabilityPersistenceError::UnavailableOrAmbiguous`;
- definite stale expected-current CAS;
- missing durable state.

GW does not authorize a broader error taxonomy or new peer-visible protocol error.

### 10. Durable store remains provider-neutral

`ReachabilityDurableStore` remains the sole bridge-owned provider-neutral durable load/CAS seam for
candidate reachability authority.

GW does not create a second store model, sidecar persistence path, synchronous bridge, helper thread,
private runtime, or alternate owner authority.

A future concrete provider must consume/produce the typed durable snapshot through this existing
seam.

### 11. Snapshot access remains typed

A future codec/provider may inspect the durable snapshot through typed accessors/parts needed to
encode/decode it, but GW selects no byte representation.

There is no authorization to expose or persist private live-plan internals merely to make a codec
convenient.

### 12. No codec/schema/version is selected

GW deliberately selects no:

- byte format;
- field tags;
- framing;
- schema version;
- migration format;
- checksum;
- compression;
- protobuf/serde/bincode/postcard/JSON/CBOR representation;
- database row/value encoding.

Typed Rust semantic state is not a persistence byte schema.

### 13. No keyspace/provider is selected

GW deliberately selects no:

- etcd key path;
- SQL table/column/index;
- embedded database;
- transaction library;
- provider revision mapping;
- lease/TTL policy;
- credential/TLS/RBAC configuration;
- connection bootstrap.

The existing exact-current CAS semantics remain abstract/provider-neutral.

### 14. No lifecycle/bootstrap activation is selected

GW does not choose or invoke a new-lifecycle/bootstrap reachability creation callsite.

It does not populate the production owner map, recover owners at Agent startup, wire candidate
handoff, emit current-Mesh responses, or make any dormant production composition path live.

### 15. No runtime/network activation is selected

GW performs and authorizes no:

- socket/listener construction;
- readiness gating;
- traversal/dialing activation;
- worker/task spawning;
- cancellation integration;
- DNS/network adapter use;
- deployment/restart/recovery operation.

## Explicit rejected interpretations

GW rejects all of the following interpretations:

- persisting `PeerConnectivityPlan` directly as the long-term durable semantic representation;
- reconstructing loaded durable plans through `PeerConnectivityPlan::new(...)`;
- dropping historical candidate-ID high-water during load/recovery;
- persisting transient reachability observations;
- silently repairing invalid high-water state;
- accepting partial plan restoration while installing loaded freshness;
- treating invalid/missing durable state as new-lifecycle authority;
- adding a second durable-store trait for the same candidate-reachability authority;
- selecting a serialization/database provider implicitly as part of typed integration;
- activating Agent owner recovery/population merely because typed restoration now exists.

## Expected narrow source successor

A source-materialization successor must begin with a fresh audit from the exact closed GW head.

If repository/compiler topology still supports the selected law, the expected narrow materialization
is limited to:

1. `crates/prw-remote-bridge/src/reachability_owner.rs` — replace live-plan durable snapshot carriage
   with typed durable plan-state carriage; add projection/restoration at existing commit,
   retirement, recover and reload seams; add one narrow restoration error classification as needed;
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` — adapt in-memory durable
   store fixtures and add focused recovery/reload/anti-reuse/restoration-failure coverage;
3. one source-materialization contract.

No Cargo manifest or lockfile change is expected from the exact GW audit because
`prw-remote-bridge` already depends normally on `prw-connectivity`.

Any additional repository path, dependency, provider/schema decision, runtime activation, or
compiler-driven semantic contradiction is a stop-and-re-audit condition rather than automatic scope
expansion.

## Validation gate

GW may close only when all of the following hold on one exact final head:

1. exact GV merge base and ahead-only lineage;
2. exactly one changed path — this semantics-selection contract;
3. no Rust/Kotlin source, manifest, lockfile or workflow mutation;
4. locked dependency graph succeeds;
5. rustfmt succeeds;
6. Clippy with warnings denied succeeds;
7. workspace tests succeed;
8. workspace build succeeds;
9. Android validation, if automatically triggered, succeeds on the same exact head;
10. no failing or pending automatically triggered exact-head workflow remains;
11. immutable Drive audit is stored under the canonical Private Remote Workspace folder and
    verified by raw byte/hash readback.

Successful closure classification:
`CLOSED_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_PLAN_STATE_INTEGRATION_SEMANTICS_SELECTION`.

Until closure, GW remains `STAGING`, draft, open, and unmerged.
