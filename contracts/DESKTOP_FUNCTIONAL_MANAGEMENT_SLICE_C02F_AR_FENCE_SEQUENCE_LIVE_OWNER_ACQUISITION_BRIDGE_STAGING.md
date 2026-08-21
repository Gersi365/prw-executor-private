# Phase 152 C02f-AR — Fence-Sequence to Live-Owner Acquisition Bridge Staging Contract

## Purpose

C02f-AR materializes the smallest provider-neutral bridge between the already-validated C02f-AQ
within-epoch fence-sequence allocation result and the already-validated C02f-AB deterministic
live-owner acquisition transaction planner.

The bridge does not execute a live-owner transaction. It only converts one **committed** AQ
allocation into the canonical selected PRW fence representation, constructs one typed Current
live-owner successor for an exact peer, and delegates deterministic transaction planning to C02f-AB.

C02f-AR performs no provider I/O, runtime/client construction, randomness generation, endpoint or
credential selection, live-owner authority activation, R1-R4 side-effect fencing, deployment or PR
merge.

## Exact base

C02f-AR starts from the exact validated C02f-AQ head:

`15985dd574b6025dc66eea1f78eb4e95d0c1cf70`

C02f-AQ remains canonical and byte-stable.

## Retained ordering lock

C02f-AH already locked the application-owned 128-bit fence ordering:

`fence = (epoch << 64) | sequence`

Therefore C02f-AR does not select a new fence representation.

The composition is exact:

- bits 127..64: non-zero durable recovery epoch;
- bits 63..0: non-zero within-epoch sequence;
- numeric u128 ordering is the selected recovery-aware fence ordering;
- zero epoch and zero sequence remain unissued/reserved by their existing typed domains;
- no decrease, reuse or wrap is authorized.

The existing PRWL codec continues to persist the resulting non-zero u128 fence as exactly 16
unsigned big-endian bytes. No PRWL schema change is introduced.

## Committed-only bridge gate

The bridge accepts one `FenceSequenceAllocationResolved` produced by C02f-AQ.

Only:

`FenceSequenceAllocationResolvedOutcome::Committed`

may authorize canonical live-owner fence composition.

`Superseded` is terminal and must return a bridge error. A superseded AQ plan never authorizes use of
its sequence because the reservation slot is not authoritative for the retained allocation attempt.

C02f-AR does not reinterpret an AQ submission result, re-run AQ reconciliation, read PRWF/PRWR, or
issue/reissue a sequence allocation.

## Exact allocation evidence retention

The bridge result retains the exact `FenceSequenceAllocationResolved` together with the resulting
C02f-AB `LiveOwnerTxnPlan`.

This preserves a deterministic typed relationship between:

1. the exact AQ allocation resolution that authorized the fence;
2. the canonical epoch/sequence-derived u128 fence;
3. the exact live-owner successor and deterministic transaction plan that consumes that fence.

The bridge does not manufacture a second allocation plan or change the retained AQ plan.

## Live-owner successor construction

For one exact caller-supplied `PeerConnectivityIdentity`, C02f-AR constructs:

`ReachabilityLiveOwnerAuthorityRecord::current(peer, canonical_fence, authority_attempt_id)`

The live-owner `AuthorityAttemptId` remains a separate mutation-attempt identity from AJ/AQ
`SequenceAllocationAttemptId`.

C02f-AR does not generate either identifier and does not derive one from the other.

The caller must provide an already-generated non-zero live-owner authority-attempt identifier.
Existing C02f-AB planning remains authoritative for rejecting reuse of the preceding live-owner
attempt identifier.

## Delegation to existing C02f-AB invariants

After committed fence composition, C02f-AR delegates to existing `plan_acquisition`.

C02f-AR therefore does not duplicate or weaken C02f-AB validation. C02f-AB continues to enforce:

- exact peer namespace preservation;
- successor lifecycle `Current`;
- successor fence strictly newer than the authoritative observed fence;
- fresh live-owner authority-attempt identity relative to the observed state;
- canonical PRWL successor encoding;
- exact dual-CAS transaction planning over the already-observed live-owner state.

A committed sequence allocation from an older recovery epoch therefore cannot bypass an already
newer live-owner fence: C02f-AB returns `FenceNotStrictlyNewer` and no provider transaction is
executed by this tranche.

## Provider/runtime boundary

C02f-AR is pure deterministic composition/planning only.

It does not:

- perform the initial or subsequent live-owner etcd read;
- execute the C02f-AB live-owner transaction;
- call the C02f-AC provider bridge;
- activate `ReachabilityLiveOwnerAsyncAuthority`;
- construct or connect an etcd client;
- select endpoints, TLS, authentication, RBAC, credentials, lease/TTL/Watch behavior, users, roles,
  permissions or cluster membership;
- contact Spanner or issue a recovery epoch;
- read, initialize, allocate or mutate real production PRWF/PRWR state;
- allocate a real production live-owner fence during validation;
- generate sequence-allocation or live-owner authority attempt IDs;
- spawn tasks, timers, background retries or detached futures;
- execute traversal/network side effects;
- implement or activate R1-R4 stale-fence effect rejection;
- deploy or merge a draft PR.

## Source scope

C02f-AR adds only:

- `crates/prw-control-plane/src/fence_sequence_live_owner_bridge.rs`;
- `crates/prw-control-plane/tests/c02f_ar_fence_sequence_live_owner_bridge.rs`;
- this contract.

C02f-AR does not modify:

- AJ `crates/prw-control-plane/src/fence_sequence.rs`;
- AM `crates/prw-control-plane/src/fence_sequence_initialization.rs`;
- AN `crates/prw-control-plane/src/fence_sequence_initialization_etcd.rs`;
- AO `crates/prw-control-plane/src/fence_sequence_initialization_orchestrator.rs`;
- AP `crates/prw-control-plane/src/fence_sequence_allocation_etcd.rs`;
- AQ `crates/prw-control-plane/src/fence_sequence_allocation_orchestrator.rs`;
- C02f-AB `crates/prw-control-plane/src/reachability_live_owner_txn.rs`;
- `crates/prw-control-plane/src/reachability_live_owner_codec.rs`;
- recovery-epoch source/adapters/orchestrators;
- public `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- root `Cargo.lock`;
- any `prw-remote-bridge` production source.

No dependency or lockfile change is authorized by AR.

## Validation harness requirements

The staging harness must prove at minimum:

1. committed AQ `(epoch=9, sequence=42)` composes exactly `(9 << 64) | 42`;
2. the bridge retains exact AQ allocation evidence beside the resulting C02f-AB transaction plan;
3. the live-owner successor is exact-peer, `Current`, and carries the exact composed fence;
4. sequence-allocation and live-owner authority-attempt identities remain distinct domains;
5. a superseded AQ allocation cannot produce a live-owner fence or acquisition plan;
6. an allocation from an older recovery epoch cannot bypass C02f-AB strict fence monotonicity;
7. exact peer mismatch remains rejected by C02f-AB;
8. reuse of the prior live-owner authority-attempt identifier remains rejected by C02f-AB.

The harness includes production source modules directly and must perform no provider network I/O.

## Validation gate

C02f-AR is valid only if canonical Rust and Android workflows pass on the exact final AR head and a
fresh AQ -> AR compare proves that the tranche remains exactly the three intended added files.

AJ/AM/AN/AO/AP/AQ source, live-owner codec/transaction source, recovery source, public `lib.rs`,
Cargo manifest, root lockfile and remote-bridge production source must remain byte-stable from the
validated AQ base.

Expected gate after validation:

`C02F_AR_FENCE_SEQUENCE_LIVE_OWNER_ACQUISITION_BRIDGE_VALIDATED`
