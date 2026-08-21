# Phase 152 C02f-AO — Fence-Sequence Initialization Reconciliation Orchestrator Staging Contract

## Purpose

C02f-AO materializes the bounded reconciliation/reissue state machine for one exact retained
C02f-AM fence-sequence initialization transaction executed through the already-validated C02f-AN
real `etcd-client` adapter.

C02f-AO does not own the initial PRWF head read or initial AM planning. The caller must already hold
one exact `FenceSequenceInitializationTxnPlan`. AO owns only the bounded lifecycle from the first AN
submission through a terminal reconciled result.

## Exact base

- base tranche: C02f-AN
- exact validated base/head: `800d996ef1d9c940f4b12f88f1b5af35e75b5e5b`
- base PR #59 remains draft and unmerged

## Retained-plan invariant

AO never calls `plan_initialization` and never derives a new mutation plan after submission begins.

Every submission receives an equality-preserving clone of the exact retained AM transaction plan.
No target epoch, predecessor observation, compare, Put value, or failure-branch Get is changed
between the first submission and the optional one exact reissue.

## Authority seam

AO introduces a static-dispatch internal authority seam with two operations:

1. submit the exact retained initialization transaction plan;
2. freshly re-observe the exact retained plan.

The existing AN `FenceSequenceInitializationEtcdStore` implements this seam by delegating to:

- AN `execute` for submission;
- AN `reobserve` for fresh default-linearizable re-observation.

AN `MutationIndeterminate` is represented to the AO state machine only as an indeterminate submission
outcome so that reconciliation must occur before any retransmission. Other AN adapter/provider/domain
failures remain fatal authority errors and do not authorize reissue.

## First-submission reconciliation

The first exact submission is classified as follows:

- definitive `Applied` -> perform one fresh re-observation before returning authority;
- definitive compare-failure `Current` -> terminal `Current`;
- definitive compare-failure `Superseded` -> terminal `Superseded`;
- definitive compare-failure `ProvenNotCommitted` -> permit one exact reissue;
- `MutationIndeterminate` -> perform one fresh re-observation before any possible reissue.

After an indeterminate first submission:

- fresh `Current` -> terminal `Current`;
- fresh `Superseded` -> terminal `Superseded`;
- fresh `ProvenNotCommitted` -> permit one exact reissue;
- authority/re-observation failure -> fail closed, no reissue.

## Definitive apply confirmation

A definitive AN `Applied` response does not by itself cause AO to return `Current`.

AO performs one fresh re-observation against the exact retained plan:

- `Current` -> terminal `Current`;
- `Superseded` -> terminal `Superseded`;
- `ProvenNotCommitted` after a definitive successful apply -> contradictory state and hard failure.

This prevents AO from promoting a stale assumption to authority after the target epoch has already
been superseded and fails closed if durable state contradicts the definitive apply response.

## One exact reissue maximum

Only `ProvenNotCommitted` permits retransmission.

AO contains exactly one second-submit path. The second submission is terminal:

- `Applied` -> fresh confirmation as above;
- compare-failure `Current` -> terminal `Current`;
- compare-failure `Superseded` -> terminal `Superseded`;
- compare-failure `ProvenNotCommitted` -> `ReissueLimitReached`;
- `MutationIndeterminate` -> one fresh re-observation;
  - `Current` -> terminal `Current`;
  - `Superseded` -> terminal `Superseded`;
  - `ProvenNotCommitted` -> `ReissueLimitReached`.

There is no third-submit path.

## Cancellation boundary

AO spawns no task and owns no detached/background retry mechanism.

If the orchestration future is dropped while a fresh re-observation is pending, no reissue can occur
after cancellation. The caller owns any future restart policy and must construct a new explicit
operation rather than relying on hidden work retained by AO.

## Source scope

C02f-AO adds only:

- `crates/prw-control-plane/src/fence_sequence_initialization_orchestrator.rs`;
- `crates/prw-control-plane/tests/c02f_ao_fence_sequence_initialization_orchestrator.rs`;
- this contract.

C02f-AO does not modify:

- C02f-AM `crates/prw-control-plane/src/fence_sequence_initialization.rs`;
- C02f-AN `crates/prw-control-plane/src/fence_sequence_initialization_etcd.rs`;
- AJ `crates/prw-control-plane/src/fence_sequence.rs`;
- `crates/prw-control-plane/src/recovery_epoch.rs`;
- `crates/prw-control-plane/src/recovery_epoch_orchestrator.rs`;
- `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- root `Cargo.lock`.

No dependency or lockfile change is authorized by AO.

## Explicit non-goals

C02f-AO does not:

- perform the initial PRWF head read;
- call AM `plan_initialization` at runtime;
- export AM, AN, or AO through the production public `lib.rs` surface;
- create or connect an etcd client;
- select etcd endpoints;
- configure TLS, authentication, RBAC, credentials, leases, TTLs, Watches, users, roles,
  permissions, or cluster membership;
- initialize a real production PRWF head in validation;
- allocate a real fence sequence;
- construct/contact Spanner or issue a real recovery epoch;
- execute snapshot restore or disaster recovery;
- implement R1-R4 lower-epoch effect rejection;
- activate production authority;
- select deployment topology;
- deploy;
- merge any draft PR.

## Validation gate

The tranche is valid only if canonical Rust and Android workflows pass on the exact final AO head
and a fresh AN -> AO compare proves that the diff remains exactly the three intended added files.
AM source, AN adapter source, AJ fence-sequence source, recovery source/orchestrator, public `lib.rs`,
Cargo manifest, and lockfile must remain byte-stable from the validated AN base.

Expected gate after validation:

`C02F_AO_FENCE_SEQUENCE_INITIALIZATION_RECONCILIATION_ORCHESTRATOR_VALIDATED`
