# Phase 152 C02f-AE — Indeterminate Mutation Reconciliation Design Staging

Status: `DESIGN_STAGED / CONTROL_PLANE_OWNS_RECONCILIATION / EXACT_PLAN_RETENTION / LINEARIZABLE_REOBSERVATION_REQUIRED / ONE_BOUNDED_DELIBERATE_REISSUE / SECOND_INDETERMINATE_REOBSERVED / NO_THIRD_SUBMIT / NO_BLIND_RETRY / NO_DETACHED_TASK / RESULT_SUPPRESSION_FAULT_INJECTION_SELECTED / PUBLIC_ERROR_ENUM_UNCHANGED / NO_TLS_AUTH_RBAC / NO_PRODUCTION_ENDPOINT / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Design staging branch: `phase-152-c02f-ae-indeterminate-reconciliation-design-staging`
Design base / completed C02f-AD evidence head: `e457491624ea8810d9d523e3d16b123312b9ca9e`
Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

C02f-AE closes the orchestration and validation-design gap for an etcd mutation whose client-side RPC completes without a definitive `TxnResponse`.

C02f-Z already selects the safety semantics: non-definitive RPC failure does not prove non-commit; exact intended successor/fence/`AuthorityAttemptId` must be retained; a fresh linearizable exact-key Get is mandatory before any retransmission; the same logical mutation reuses the same intended fence/attempt ID; superseding authority wins; blind retry is forbidden.

C02f-AB already materializes pure deterministic reconciliation as `Committed`, `ProvenNotCommitted`, or `Superseded`, failing closed for missing/cross-peer/ABA/impossible state. C02f-AD already maps a real `KvClient::txn` RPC error to `MutationIndeterminate` while borrowing rather than consuming the exact `LiveOwnerTxnPlan`.

C02f-AE therefore does not redesign key/value encoding, dual-CAS Txn shape or the deterministic classifier. It selects the missing operation-local orchestration, bounded reissue policy and fault-injection validation boundary.

## Non-goals

C02f-AE does not select or activate production endpoints/topology, TLS/auth/RBAC, Watch/lease/TTL authority, recovery epoch/high-water, production first-absence bootstrap, production fence/RNG activation, Agent/runtime ownership, durable crash journals, R1-R4 effects, Phase 153/154 production activation, deployment or merge.

## Selected ownership

The reconciliation orchestrator is owned by `prw-control-plane` because it consumes provider-owned `MutationIndeterminate`, performs provider linearizable Gets and consumes C02f-AB provider-owned classifiers. `prw-control-plane` must not depend on `prw-remote-bridge`; bridge code must not implement etcd-specific reconciliation.

Dependency direction remains:

```text
prw-core <- prw-connectivity <- prw-control-plane
                               ^
                               |
                     prw-remote-bridge
```

The public semantic error enum remains only `UnavailableOrAmbiguous` and `FenceExhausted`.

## Selected operation-local capsule

The source tranche should retain one private/crate-private non-`Clone` operation-local mutation capsule conceptually shaped as:

```text
LiveOwnerPendingMutation
    Acquisition { before: LiveOwnerObservation, plan: LiveOwnerTxnPlan }
    Release     { before: LiveOwnerObservation, plan: LiveOwnerTxnPlan }
```

Required invariants:

1. `before.key` is the exact key used by plan compares and branches.
2. the `mod_revision` compare equals `before.mod_revision`.
3. the exact-value compare equals `before.value`.
4. success is the canonical encoded `plan.successor`.
5. acquisition successor is same exact peer, `Current`, strictly newer fence and fresh attempt ID.
6. release successor is exactly `before.record().released_successor()`.
7. capsule owns predecessor observation and exact plan until the operation resolves.
8. no caller-controlled request values reconstruct the mutation after an indeterminate result.
9. no public clone/reissue handle is exposed.

## Selected orchestration state machine

### A — initial submit

Submit the exact retained plan through C02f-AD.

- definitive `Committed` => return committed;
- definitive `CompareFailed(observation)` => return the existing definitive negative result;
- structural/provider validation error => fail closed;
- `MutationIndeterminate` => transition to `MustReobserve`.

No mutation is retransmitted directly from `MutationIndeterminate`.

### B — mandatory re-observation

The only permitted safety-relevant provider operation is one new default-linearizable Get of the exact authority key. No serializable read, Watch, cache, prefix scan or mutation is permitted before that Get is decoded and classified.

Unavailable, absent-established, malformed, cross-peer or impossible state terminates fail-closed.

### C — deterministic reconciliation

Acquisition calls the existing acquisition reconciler with exact `before`, exact intended successor from the retained plan and the fresh authoritative observation. Release calls the existing release reconciler with exact `before` and the new observation.

`Committed` means the original logical mutation is proven committed. Acquisition may succeed only while re-observed state is still the exact intended `Current` record. Release may succeed only from the matching `Released` peer+fence+attempt record. No additional Txn is submitted.

`Superseded` means the original operation cannot regain authority. C02f-AE forbids fabricating a synthetic etcd `TxnResponse` or falsely labelling this as a raw Txn compare failure. No grant is constructed and no old plan is reissued. The first source materialization may map this provider-internal terminal state fail-closed through the existing bridge error surface; a later narrow internal-API decision may optimize acquisition supersession to provider-neutral `Contended`.

`ProvenNotCommitted` requires exact predecessor bytes **and exact predecessor `mod_revision`**. Only this result permits one deliberate reissue.

## Selected bounded reissue policy

C02f-AE permits **at most one deliberate reissue per top-level mutation invocation**.

The reissued transaction must be the exact retained plan: same key, same two compares, same predecessor revision/value bytes, same successor bytes, same fence and same `AuthorityAttemptId`. No new fence or attempt ID is generated and no successor is rebuilt from request parameters.

Reissue outcomes:

- definitive committed => success;
- definitive compare failure => existing definitive negative result;
- structural error => fail closed;
- second `MutationIndeterminate` => mandatory second linearizable re-observation.

After a second indeterminate:

- `Committed` => committed;
- `Superseded` => non-authoritative/fail-closed;
- unavailable/missing/corrupt/impossible => fail closed;
- `ProvenNotCommitted` again => fail closed and **do not submit a third Txn**.

Thus one top-level operation can submit at most two Txns. There is no unbounded loop, hidden backoff/timer policy or mutation storm.

## Cancellation rule

Dropping/cancelling the top-level Future never produces semantic success. C02f-AE spawns no detached reconciliation task and no background retry worker. If a server-side mutation committed before cancellation, persisted authority remains authoritative and later operations discover it through normal authoritative reads. Durable crash-resume journaling is not selected here.

## Selected fault-injection boundary

The first executable validation uses **deterministic provider-result suppression** around a real disposable etcd server rather than timing-sensitive packet loss.

The property under test is the orchestration response when the server may have processed a mutation but the caller has no definitive result. A test-only wrapper may delegate a real Txn, deliberately discard its definitive result and surface an internal `Indeterminate` signal. It may also inject pre-submit indeterminate state without sending a Txn.

This proves reconciliation ordering deterministically. It does not claim validation of every gRPC/HTTP2 failure mode. A later transport-realism proxy/reset gate may be added without changing the selected semantics.

The internal I/O seam should conceptually expose:

```text
linearizable_observation(peer)
execute(plan) -> Definitive(outcome) | Indeterminate
```

The real adapter maps only C02f-AD `MutationIndeterminate` to the internal indeterminate signal; codec/structural errors remain fail-closed.

## Required executable scenarios

### AE-1 acquisition response suppressed after real commit

Real acquisition commits; definitive result is suppressed; one re-observation sees exact intended Current peer+fence+attempt; classify `Committed`; return success with exactly one actual Txn and no retry.

### AE-2 acquisition indeterminate before submit

Inject indeterminate without submitting; re-observe exact predecessor+revision; classify `ProvenNotCommitted`; deliberately submit the exact retained plan once; commit with unchanged fence/attempt/successor bytes.

### AE-3 acquisition committed then superseded

Commit intended acquisition, suppress result, advance fixture to a strictly newer valid Current before reconciliation; re-observe `Superseded`; no grant and no old-plan retry.

### AE-4 acquisition committed then released

Commit intended Current, suppress result, authoritatively release same fence+attempt before reconciliation; classify `Superseded`; no grant and no retry.

### AE-5 release response suppressed after real commit

Real release commits canonical Released state, result is suppressed, re-observation sees same peer+fence+attempt Released; classify `Committed`; no second release Txn.

### AE-6 release indeterminate before submit

Inject pre-submit indeterminate, re-observe exact predecessor+revision, classify `ProvenNotCommitted`, submit exact release plan once; preserve same fence/attempt.

### AE-7 ABA-like same bytes at new revision

Predecessor-equivalent bytes at a different revision must produce `ImpossibleReobservedState`, fail closed and submit no retry.

### AE-8 re-observation unavailable/malformed

After indeterminate, unavailable or invalid authoritative observation must fail closed with zero reissue and no semantic authority.

### AE-9 reissue itself indeterminate then committed

First attempt proves not committed; exact plan is reissued once; reissue commits but its response is suppressed; second re-observation sees intended successor; resolve committed and submit no third Txn.

### AE-10 reissue itself indeterminate then proven not committed

First attempt proves not committed; one exact reissue is permitted; second pre-submit indeterminate is injected; second re-observation still sees exact predecessor; terminate fail-closed with no third Txn.

### AE-11 cancellation

Poll an operation into an unresolved phase and drop it. Assert no semantic grant/release result and no detached retry/reconciliation activity.

## Validation instrumentation

Test-only instrumentation may record exact-key Gets, actual Txn submissions, injected indeterminate signals, reconciliation classifications and deliberate reissue transitions. The harness must fail if a second Txn submission occurs without an intervening authoritative re-observation classified `ProvenNotCommitted`.

Expected trace shape when a reissue is allowed:

```text
TxnAttempt
Indeterminate
LinearizableReobserve
ProvenNotCommitted
TxnReissue
```

Instrumentation is not production authority state.

## Error mapping

C02f-AE does not widen the public error enum. Unresolved indeterminate state, unavailable/missing/corrupt/impossible re-observation and provider-internal superseded state not honestly representable by the existing definitive-mutation bridge port initially map to `UnavailableOrAmbiguous`. `FenceExhausted` remains reserved for the separate allocator boundary. No internal failure maps to `Current`, `Granted` or `Released`.

## Implementation order after this design

1. materialize control-plane operation-local reconciliation orchestration;
2. add deterministic no-network tests for mandatory re-observation, exact-plan reissue, one-reissue bound, second-indeterminate behavior and cancellation/no-detached-task;
3. run canonical Rust validation;
4. extend the disposable etcd harness with deterministic result-suppression scenarios;
5. run canonical Rust plus dedicated C02f-AE disposable validation;
6. freeze PASS evidence;
7. only then consider a separate transport-realism fault gate if useful.

TLS/auth/RBAC, production endpoints and recovery/high-water remain later independent gates.

## Acceptance criteria

The eventual implementation must prove:

1. no path retransmits directly from `MutationIndeterminate`;
2. every permitted reissue is preceded by a fresh linearizable exact-key Get;
3. only `ProvenNotCommitted` permits reissue;
4. reissue reuses the exact plan/fence/attempt ID;
5. at most one reissue occurs per top-level invocation;
6. a second indeterminate is re-observed before termination;
7. a second `ProvenNotCommitted` does not permit a third submit;
8. reconciliation success cannot be emitted from superseded state;
9. `Superseded` never produces a grant;
10. same bytes at a new revision do not prove non-commit;
11. cancellation never creates semantic success or a detached task;
12. public errors remain fail-closed;
13. no production endpoint/runtime/TLS/recovery/R1-R4 activation is introduced.

## Gate decision

`C02F_AE_INDETERMINATE_MUTATION_RECONCILIATION_DESIGN_STAGED`

This document does not itself authorize source materialization, push, PR creation, merge or production activation.
