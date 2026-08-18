# Phase 152 C02e — Tranche 4 Production Reachability Owner + Persistence/Synchronization Seam

Status: `PRODUCTION_OWNER_SELECTED / PRW_REMOTE_BRIDGE_UPPER_COMPOSITION / PHASE141_NORMAL_DEPENDENCY / DURABLE_EXPECTED_CURRENT_CAS_SEAM / FAIL_CLOSED_RECOVERY / RETIRED_TOMBSTONE_COMMIT / RUNTIME_TENANCY_NOT_ACTIVATED / WIRE_UNSELECTED / STORAGE_BACKEND_UNSELECTED / NO_NETWORK_IO / NO_AGENT_ACTIVATION`

Tranche 3 closeout head: `6168d500b25627190aa272ff34fdc186465ebc04`
Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

Tranche 4 selects and materializes the upper source owner that composes the already-validated C02e state machines. It also selects the durable expected-current compare-and-commit boundary needed to keep candidate-publication freshness and accepted candidate state linearizable across recovery or competing writers.

It does **not** activate a network adapter, socket, async task, long-running loop, Agent bootstrap path, database product, persistence serialization, replication system, candidate/freshness wire message, or deployment.

## Concrete owner placement

The concrete production-compiled owner is:

`prw_remote_bridge::reachability_owner::ProductionReachabilityOwner`

`prw-remote-bridge` is selected because it already sits above authenticated session, registry and connectivity semantics. Tranche 4 promotes the existing validated `prw-remote-bridge -> prw-nat-traversal` edge from dev-only to a normal local dependency so this upper owner can hold the actual Phase 141 Sans-I/O `IceConnectivitySession` without moving ICE state into a lower domain.

The dependency direction remains upper-to-lower. `prw-connectivity` does not gain NAT traversal ownership and `prw-nat-traversal` does not gain registry/session/publication authority.

## State owned for one exact peer lifecycle

One owner instance holds:

1. exactly one current `PeerConnectivityPlan`;
2. exactly one `CandidatePublicationFreshnessRecord` for the same `DeviceId + TransportIdentity`;
3. at most one current `IceConnectivitySession`;
4. one `ReachabilityDurableStore` implementation handle;
5. one verifier-owned `CandidatePublicationFreshnessTokenSource`.

No second candidate-plan model or second ICE state machine is introduced.

## Durable snapshot boundary

`ReachabilityDurableSnapshot` couples the accepted plan snapshot and freshness record and rejects a peer mismatch.

The persistence seam deliberately remains typed and provider-neutral. This tranche does not select bytes-on-disk, schema versioning, database keys, SQL/NoSQL technology, replication, cloud provider or encryption-at-rest mechanism.

Reachability observations remain transient evidence. A successful candidate publication stages a refreshed plan whose observations are `Unknown`; that accepted snapshot is what the owner supplies to durable compare-and-commit. Later traversal observations mutate only the current in-memory plan and are not independently persisted by this tranche. Recovery therefore resumes from the last accepted publication snapshot rather than treating a historical `Reachable` observation as durable truth.

## Expected-current CAS contract

`ReachabilityDurableStore::compare_and_commit(expected_current, replacement)` is the selected persistence arbitration seam.

A conforming implementation must be linearizable for one exact peer lifecycle:

- `Committed` means the complete replacement snapshot is durably current;
- `StaleExpected` is a definite non-commit because durable current state no longer contains the presented expected freshness token;
- an error means the caller cannot safely infer whether a commit happened and must recover authoritatively.

Storage absence is never new-lifecycle authority.

## Candidate-publication commit ordering

`ProductionReachabilityOwner::commit_candidate_publication(...)` is ordered as follows:

1. owner must be `Current`;
2. requester/publisher sessions, workspace, exact publication target and target `TransportIdentity` are revalidated against current registry state;
3. presented publication freshness must equal the owner's exact current verifier token;
4. the complete candidate refresh is validated on a cloned/staged `PeerConnectivityPlan`;
5. a distinct replacement token is issued by the verifier-owned token source;
6. a peer-consistent replacement durable snapshot is staged;
7. durable expected-current CAS runs;
8. only `Committed` installs the staged plan/freshness locally and invalidates the previous traversal session.

Identity, freshness, candidate-validation or token-generation failure before step 7 leaves local accepted plan/freshness/traversal unchanged.

## Stale and ambiguous persistence results

A definite `StaleExpected` proves that another authoritative transition is ahead of the local owner. The local owner therefore:

- drops its traversal session;
- enters `RecoveryRequired`;
- refuses later publication/traversal operations until authoritative durable reload.

An unavailable or ambiguous persistence result is treated even more conservatively: commit status may be unknown, so the owner performs the same fail-closed transition. It never retries by assuming the old token is still current.

`reload_from_store()` is the only Tranche 4 recovery path back to `Current` or `Retired`. Missing, ambiguous or peer-mismatched durable state stays fail closed.

## Traversal lifecycle

A committed candidate publication always invalidates the previous traversal session, including when every candidate is retained exactly.

Continued traversal is a separate post-commit step through `ReachabilityTraversalFactory::build_for_current_plan(...)`.

Factory failure never rolls back an accepted publication. The new plan and freshness remain current, the old traversal remains stale, and the owner remains without a traversal session until a later valid replacement construction succeeds.

`poll_and_apply_current_reachability(...)` keeps a Phase 141 `CandidateReachabilityUpdate` inside the serialized owner call and revalidates the plan's exact transport identity immediately before polling/application. The update is not exposed for delayed application outside the owner boundary.

## Transport rotation and durable retirement

Transport rotation remains a stronger lifecycle boundary.

After the registry no longer accepts the plan's exact old `TransportIdentity`, `retire_noncurrent_lifecycle(...)` may CAS the exact old peer lifecycle to a durable `Retired` tombstone. On successful retirement the traversal session is dropped and the old peer can no longer accept publications or observations.

This method does not create the replacement peer lifecycle. The replacement `DeviceId + new TransportIdentity` requires separately authorized bootstrap state. Reuse of historical transport bytes does not erase the tombstone automatically.

## Synchronization scope and activation gate

`&mut self` provides exact serialization inside one owner instance. The durable CAS seam arbitrates competing **accepted-state writes** across owner instances/processes.

Tranche 4 does **not** claim that CAS alone supplies a distributed active-owner lease for transient traversal observations. Before any long-running/network runtime can be activated, a separately reviewed runtime-tenancy/fencing rule must guarantee that one exact peer lifecycle does not have competing live traversal owners whose transient observations can drive routing concurrently.

This limitation is intentional and keeps runtime activation closed. The source owner and persistence transaction boundary are production-compiled; the long-running tenancy/actor/task/network mechanism is not.

## Dependency change

Tranche 4 changes only the local dependency kind already validated in Tranche 2:

- before: `prw-nat-traversal` was a `prw-remote-bridge` dev-dependency;
- after: `prw-nat-traversal` is a normal `prw-remote-bridge` dependency.

No external package, version or package identity is intentionally added by this decision. Cargo/lock materialization must be validated rather than assumed.

## Explicitly closed

Tranche 4 does not select or activate:

- a concrete persistence database/backend;
- persistence serialization/schema/replication;
- distributed owner lease/fencing/runtime tenancy;
- candidate-publication/freshness wire codec;
- ICE coordination wire schema;
- UDP/TCP/STUN/TURN/ICE network adapter;
- async runtime, scheduler, cancellation loop or background task;
- Agent/bootstrap traversal activation;
- relay allocation/provider integration beyond existing boundaries;
- deployment, signing, privileged mutation, PR merge or production rollout.

## Validation requirements

Before closeout, an exact-head validator must prove:

- `prw-nat-traversal` is a normal local dependency of `prw-remote-bridge` and is not duplicated as dev-only;
- Cargo.lock remains semantically/exactly stable unless Cargo proves a required local-graph materialization;
- focused production-owner tests pass;
- focused Clippy passes with warnings denied;
- full workspace Clippy/tests/build pass locked;
- tracked Cargo cache rewrites are normalized and final source/dependency drift is empty;
- temporary validation harness is removed after authoritative PASS evidence;
- no network/runtime/Agent activation source is introduced by this tranche.
