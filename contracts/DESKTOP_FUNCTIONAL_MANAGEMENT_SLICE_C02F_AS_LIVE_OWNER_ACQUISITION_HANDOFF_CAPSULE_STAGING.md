# Phase 152 C02f-AS — Live-Owner Acquisition Handoff Capsule Staging Contract

## Purpose

C02f-AS closes one provider-neutral evidence-continuity gap between the already-validated C02f-AR
fence-sequence/live-owner acquisition planner and any future provider-execution tranche.

C02f-AR plans from one exact authoritative `LiveOwnerObservation`, but its result intentionally
retains only the committed C02f-AQ allocation resolution plus the resulting C02f-AB
`LiveOwnerTxnPlan`. Existing C02f-AE indeterminate-mutation reconciliation requires the exact
pre-mutation observation as part of its operation-local reconciliation context.

C02f-AS therefore materializes only a typed **handoff capsule** that retains:

1. the exact authoritative pre-mutation `LiveOwnerObservation`; and
2. the exact C02f-AR `FenceSequenceLiveOwnerAcquisitionPlan`.

Before accepting that pair, AS deterministically replays existing C02f-AB acquisition planning and
requires the replay to reproduce the retained AR transaction plan exactly.

AS performs no provider I/O and does not execute or authorize a live-owner transaction.

## Exact base

C02f-AS starts from the exact canonical C02f-AR head:

`7eaf057474d27b6d93e4322553700a9bb620fb61`

C02f-AR remains canonical and byte-stable.

## Evidence-continuity invariant

A future execution/reconciliation boundary needs both:

- the exact deterministic transaction plan; and
- the exact authoritative state against which that plan was created.

AS must not reconstruct, infer or substitute the pre-read observation later from advisory/cache
state. It retains the exact `LiveOwnerObservation` supplied alongside the exact AR result.

The retained observation includes the canonical exact key, exact persisted value bytes, positive
`mod_revision`, and decoded exact-peer authority record already validated by C02f-AB.

## Exact replay-binding gate

AS accepts one `LiveOwnerObservation` and one `FenceSequenceLiveOwnerAcquisitionPlan` only if:

`plan_acquisition(retained_observation, retained_ar_transaction.successor)`

reproduces the complete retained C02f-AR transaction plan exactly.

This reuses existing C02f-AB deterministic validation rather than duplicating transaction rules.
The equality check covers the complete `LiveOwnerTxnPlan`, including:

- exact `mod_revision` compare;
- exact predecessor value compare;
- success exact-key Put bytes;
- failure exact-key linearizable Get;
- exact typed successor record.

Consequences:

- the same logical record at another revision cannot be rebound to an already-created AR plan;
- another peer observation cannot be rebound to the plan;
- a changed predecessor value cannot be rebound to the plan;
- exact-peer, strict-newer fence and attempt-ID freshness invariants remain delegated to C02f-AB;
- no new transaction shape or provider rule is selected by AS.

## Retained allocation and attempt identity

The AS capsule retains the complete AR result unchanged, therefore the exact committed AQ allocation
evidence remains associated with the exact live-owner transaction plan.

AS does not generate or alter either attempt identity domain:

- AJ/AQ `SequenceAllocationAttemptId` remains the sequence reservation mutation identity;
- PRWL `AuthorityAttemptId` remains the live-owner mutation identity.

Neither identifier is derived from the other.

## Provider/runtime boundary

C02f-AS is pure deterministic evidence retention and validation only.

It does not:

- execute the retained live-owner transaction;
- call C02f-AD `ReachabilityLiveOwnerEtcdStore::execute`;
- call C02f-AE `execute_acquisition_with_reconciliation`;
- call the C02f-AC bridge lower-provider port;
- activate `ReachabilityLiveOwnerAsyncAuthority`;
- perform a live-owner etcd read or fresh re-observation;
- issue/reissue a fence-sequence allocation;
- allocate a real live-owner fence;
- construct or connect an etcd client;
- select endpoints, TLS, authentication, RBAC, credentials, lease/TTL/Watch behavior, users, roles,
  permissions or cluster membership;
- contact Spanner or issue a recovery epoch;
- generate sequence-allocation or live-owner authority attempt IDs;
- spawn tasks, timers, retries, detached futures or background work;
- execute traversal/network side effects;
- implement or activate R1-R4 stale-fence effect rejection;
- export the AS capsule through production public `lib.rs`;
- deploy or merge a draft PR.

Any concrete provider execution, reconciliation invocation, async-authority adapter activation or
runtime construction remains a later separately selected boundary.

## Source scope

C02f-AS adds only:

- `crates/prw-control-plane/src/fence_sequence_live_owner_handoff.rs`;
- `crates/prw-control-plane/tests/c02f_as_live_owner_acquisition_handoff.rs`;
- this contract.

C02f-AS does not modify:

- C02f-AR `crates/prw-control-plane/src/fence_sequence_live_owner_bridge.rs`;
- C02f-AQ allocation orchestration source;
- C02f-AP allocation etcd adapter;
- C02f-AE live-owner reconciliation source;
- C02f-AD live-owner etcd wiring;
- C02f-AB live-owner transaction planning source;
- PRWL codec source;
- recovery-epoch source/adapters/orchestrators;
- public `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- root `Cargo.lock`;
- any `prw-remote-bridge` production source.

No dependency or lockfile change is authorized by AS.

## Validation harness requirements

The staging harness must prove at minimum:

1. an exact observation plus its exact AR plan is accepted and both are retained unchanged;
2. the retained AR allocation remains `Committed` and preserves exact AQ evidence;
3. the exact successor remains the locked `(epoch << 64) | sequence` fence selected by AR;
4. the same record bytes at a different `mod_revision` cannot be rebound to the existing AR plan;
5. another peer observation fails closed under existing C02f-AB exact-peer validation;
6. consuming the capsule returns the exact original observation and AR plan;
7. sequence-allocation and live-owner authority-attempt identity domains remain separate;
8. validation performs no provider network I/O.

## Validation gate

C02f-AS is valid only if canonical Rust and Android workflows pass on the exact final AS head and a
fresh AR -> AS compare proves the tranche remains exactly the three intended added files.

AR/AQ/AP/AE/AD/AB source, PRWL codec, recovery source, public `lib.rs`, Cargo manifest, root lockfile
and `prw-remote-bridge` production source must remain byte-stable from the exact validated AR base.

Expected gate after validation:

`C02F_AS_LIVE_OWNER_ACQUISITION_HANDOFF_CAPSULE_VALIDATED`
