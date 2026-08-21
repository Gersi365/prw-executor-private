# Phase 152 C02f-AQ — Fence-Sequence Allocation Reconciliation Orchestrator Staging Contract

## Purpose

C02f-AQ materializes the bounded reconciliation/reissue orchestration for one already-retained C02f-AJ within-epoch fence-sequence allocation plan using the already-validated C02f-AP real etcd adapter boundary.

AQ does not plan a new allocation, generate an allocation attempt identifier, initialize PRWF state, construct/connect provider clients, choose endpoints or credentials, activate live-owner authority, or deploy runtime infrastructure.

The tranche exists to close one specific boundary left open by AP: an AP submission result may be definitive or indeterminate, but no deliberate retransmission may occur until the exact retained allocation plan has been freshly re-observed and AJ has proven that exact plan not committed.

## Exact base

AQ is based on the exact validated C02f-AP head:

`ae34587a190f8a31b0c473d450e02ca3c44ffff1`

Base branch:

`phase-152-c02f-ap-fence-sequence-allocation-etcd-adapter-staging`

AP remains an unchanged historical checkpoint.

## Retained-plan ownership

AQ accepts exactly one already-retained `FenceSequenceAllocationPlan`.

AQ does not:

- perform the initial PRWF head read;
- call `plan_allocation`;
- generate or replace `SequenceAllocationAttemptId`;
- modify the retained predecessor observation;
- change the retained sequence number;
- change the retained reservation key;
- construct a new compare/success/failure plan after submission begins.

Every permitted submission is an equality-preserving clone of the exact retained AJ plan supplied by the caller.

## Authority seam

AQ introduces a static-dispatch `FenceSequenceAllocationAuthority` seam with two async operations:

1. submit the exact retained allocation plan;
2. freshly re-observe the exact retained plan.

`FenceSequenceAllocationEtcdStore` implements the seam by delegating to AP:

- AP definitive `Applied` -> AQ `Applied`;
- AP definitive `CompareFailed(classification)` -> AQ `CompareFailed(classification)`;
- AP `MutationIndeterminate` provider/RPC error -> AQ `MutationIndeterminate`;
- every other AP/provider/decoding/shape error remains a fatal authority error.

AQ does not erase fatal AP errors into retryable outcomes.

## Terminal outcomes

AQ has only two successful terminal outcomes:

- `Committed` — the exact retained allocation attempt is authoritatively committed;
- `Superseded` — the retained reservation/sequence slot is authoritatively owned by another attempt.

AQ never converts ambiguous or contradictory state into either successful outcome.

## First submission reconciliation

The first exact retained-plan submission is handled as follows.

### Definitive Applied

A definitive AP `Applied` response is not by itself returned as terminal authority.

AQ performs one fresh AP re-observation of the exact retained plan.

- fresh `Committed` -> terminal `Committed`;
- fresh `Superseded` -> contradictory state, fail closed;
- fresh `ProvenNotCommitted` -> contradictory state, fail closed.

A definitive successful two-Put response followed by a fresh observation that does not preserve the retained attempt as committed is treated as a contradiction, never as reissue authority.

### Definitive CompareFailed(Committed)

This is terminal `Committed` with no reissue.

The AP failure branch already returned the exact canonical head + reservation reads from the definitive failed transaction and AJ classified the retained attempt as committed.

### Definitive CompareFailed(Superseded)

This is terminal `Superseded` with no reissue.

### Definitive CompareFailed(ProvenNotCommitted)

This classification alone does **not** authorize retransmission in AQ.

AQ must perform one explicit fresh AP re-observation of the exact retained plan before any deliberate reissue can occur.

The fresh result is handled as follows:

- `Committed` -> terminal `Committed`;
- `Superseded` -> terminal `Superseded`;
- `ProvenNotCommitted` -> eligible to consume the retained AJ one-reissue budget.

This fresh-proof rule is intentionally stricter than the earlier initialization AO behavior and implements the explicit AP contract boundary.

### MutationIndeterminate

AQ performs one explicit fresh AP re-observation before any possible retransmission.

- `Committed` -> terminal `Committed`;
- `Superseded` -> terminal `Superseded`;
- `ProvenNotCommitted` -> eligible to consume the retained AJ one-reissue budget.

Provider ambiguity alone never authorizes retransmission.

## Existing AJ reissue budget

AQ reuses the existing provider-neutral `FenceSequenceReissueBudget` selected in AJ.

Only a fresh `FenceSequenceReobservation::ProvenNotCommitted` observation may be passed to `consume`.

AQ does not create an independent retry counter or a second reissue budget.

After the one AJ budget allowance is consumed, AQ performs exactly one second submission using an equality-preserving clone of the exact retained plan.

## Second submission terminal boundary

There is no third submission path.

The second exact retained-plan submission is handled as follows:

- `Applied` -> one fresh re-observation; only fresh `Committed` succeeds;
- `CompareFailed(Committed)` -> terminal `Committed`;
- `CompareFailed(Superseded)` -> terminal `Superseded`;
- `CompareFailed(ProvenNotCommitted)` -> `ReissueLimitReached`;
- `MutationIndeterminate` -> one fresh re-observation:
  - `Committed` -> terminal `Committed`;
  - `Superseded` -> terminal `Superseded`;
  - `ProvenNotCommitted` -> `ReissueLimitReached`.

AQ never consumes a second budget and never performs a third submit.

## Fail-closed authority behavior

Fatal authority errors terminate orchestration immediately.

Examples include:

- AP read failure outside the explicitly mapped mutation-indeterminate submit outcome;
- malformed PRWF/PRWR bytes;
- missing initialized head during allocation re-observation;
- impossible exact-key cardinality/key mismatch;
- malformed retained plan shape;
- impossible Txn response shape;
- AJ contradictory re-observation state.

AQ does not convert those conditions into `ProvenNotCommitted`, retry authority, or successful terminal state.

## No detached/background work

AQ owns no spawned task, detached future, timer, retry loop, or background worker.

Dropping a pending orchestration future cannot leave an independent reissue path running.

Any future restart policy remains explicit and caller-owned outside AQ.

## Source scope

C02f-AQ adds only:

- `crates/prw-control-plane/src/fence_sequence_allocation_orchestrator.rs`;
- `crates/prw-control-plane/tests/c02f_aq_fence_sequence_allocation_orchestrator.rs`;
- this contract.

C02f-AQ does not modify:

- AJ `crates/prw-control-plane/src/fence_sequence.rs`;
- AM `crates/prw-control-plane/src/fence_sequence_initialization.rs`;
- AN `crates/prw-control-plane/src/fence_sequence_initialization_etcd.rs`;
- AO `crates/prw-control-plane/src/fence_sequence_initialization_orchestrator.rs`;
- AP `crates/prw-control-plane/src/fence_sequence_allocation_etcd.rs`;
- `crates/prw-control-plane/src/recovery_epoch.rs`;
- `crates/prw-control-plane/src/recovery_epoch_orchestrator.rs`;
- `crates/prw-control-plane/src/recovery_epoch_spanner.rs`;
- public `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- root `Cargo.lock`;
- any `prw-remote-bridge` source.

No dependency or lockfile change is authorized by AQ.

## Explicit non-goals

C02f-AQ does not:

- export AQ through the production public `lib.rs` surface;
- perform initial allocation planning;
- generate `SequenceAllocationAttemptId` values;
- choose the current recovery epoch;
- initialize PRWF state;
- allocate a real production fence sequence during validation;
- create/connect an etcd client or select endpoints;
- configure TLS, authentication, RBAC, credentials, leases, TTLs, Watches, users, roles, permissions, or cluster membership;
- construct/contact Spanner or issue a recovery epoch;
- bridge a completed allocation into live-owner acquisition;
- activate `ReachabilityLiveOwnerAsyncAuthority`;
- execute snapshot restore or disaster recovery;
- implement R1-R4 stale-fence effect rejection;
- select deployment/runtime process lifecycle;
- deploy;
- merge any draft PR.

## Validation gate

The tranche is valid only if canonical Rust and Android workflows pass on the exact final AQ head and a fresh AP -> AQ compare proves the diff remains exactly the three intended added files.

AJ allocation source, AM/AN/AO initialization source, AP adapter source, recovery source, public `lib.rs`, Cargo manifest, root lockfile, and remote-bridge source must remain byte-stable from the validated AP base.

Expected gate after validation:

`C02F_AQ_FENCE_SEQUENCE_ALLOCATION_RECONCILIATION_ORCHESTRATOR_VALIDATED`
