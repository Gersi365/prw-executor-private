# Phase 152 C03e-FQ — Recoverable Spawned Authenticated-Session Custody and Owned FL Worker Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FQ materializes only the C03e-FO-selected bounded spawned ownership seam for the already-materialized requester-aware FL worker, using the C03e-FP shared requester/rendezvous authority.

FQ does not substitute FL into the persistent collection, change real admission, close or reuse an authenticated peer, clean requester records, select candidate/reachability state, dial target traffic, wire a listener, publish readiness, deploy, restart/recover the process, or merge.

## 2. Exact predecessor

FQ is based exactly on closed C03e-FP:

- predecessor branch: `phase-152-c03e-fp-shared-requester-rendezvous-authority-synchronization-borrowed-fl-adaptation-source-materialization-staging`
- predecessor head: `c9578e635cd0768c4707f27415ceebf38daddea7`
- predecessor tree: `38f205848fb86dc4ac3436999ea1baa0ce023296`
- predecessor PR: `#292`, `Status: CLOSED`, draft/open/unmerged
- predecessor gate: `C03E_FP_SHARED_REQUESTER_RENDEZVOUS_AUTHORITY_SYNCHRONIZATION_BORROWED_FL_ADAPTATION_SOURCE_MATERIALIZED`

FP remains frozen.

## 3. Exact source guards

FQ preserves the exact FP implementations of:

- shared requester/rendezvous authority and FO lock ordering;
- FB retained-custody DR continuation;
- FH terminal requester acknowledgement composition;
- FJ serial post-terminal lifecycle;
- FL cancellation-safe worker boundaries;
- FN non-spawned borrowed executor seam;
- existing capability-only spawned/supervised/persistent worker paths;
- real-admission supervisor and endpoint lifecycle.

The only historical executor mutation selected by FQ is one child-module registration line. New FQ logic lives in a dedicated child module.

## 4. Materialized recoverable session-owner representation

FQ materializes one per-invocation supervisor-retained owner cell conceptually and concretely equivalent to:

`Arc<tokio::sync::Mutex<Option<AuthenticatedRemoteSessionRuntimeOwner>>>`

The cell contains exactly one authenticated-session owner before spawn.

No provider, transport peer, session owner, authority record, or authenticated identity is cloned.

Only outer `Arc` custody is cloned between supervisor and worker task.

## 5. Worker must not take owner before FL

The spawned task does not call `take()` on the owner cell before FL execution.

Instead it:

1. acquires the Tokio mutex;
2. borrows the contained `AuthenticatedRemoteSessionRuntimeOwner` mutably;
3. runs exact FL using that mutable borrow;
4. releases the guard when the task returns or unwinds.

This preserves owner recoverability after both normal and abnormal task completion.

## 6. Supervisor-retained custody

The drive seam retains one `Arc` clone outside the spawned task for the whole join interval.

Therefore the task is never the sole owner of the recoverable session-owner cell.

The `JoinHandle` is awaited inside the same bounded executor drive. It is not returned, detached, stored in a persistent collection, or intentionally aborted.

## 7. Normal completion law

On normal task completion:

- exact FL stop is preserved unchanged;
- worker guard has released;
- supervisor reacquires the cell;
- supervisor takes the exact authenticated-session owner by value only after join is terminal;
- completion returns recovered owner plus exact FL stop;
- recovered peer is retained-stopped.

No close, restart, reuse, next ingress, requester cleanup, retry, resend, or replacement task occurs.

## 8. Abnormal join law

If Tokio reports abnormal task completion:

- existing bounded `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion` is preserved;
- no synthetic FL stop is created;
- supervisor still reacquires the retained owner cell after the join is terminal;
- exact authenticated-session owner is recovered by value;
- recovered peer remains retained-stopped with transport/session health unspecified;
- no code-3/code-4 capability close law is reused;
- no peer close, restart, reuse, or replacement task occurs.

## 9. Panic/unwind ownership law

A panic while the spawned task holds the Tokio mutex guard unwinds that guard.

Because the worker never removed the owner from the cell, the exact owner remains present for supervisor recovery after the join reports abnormal completion.

FQ tests this ownership property with a bounded generic retained-cell test that intentionally panics while holding the guard and verifies owner recovery plus the existing bounded join classification.

## 10. Exact FL stop preservation

Normal spawned completion preserves exact:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

No capability-worker stop mapping is introduced.

Therefore:

- FL `Cancelled` remains `Cancelled`;
- `Failed(Ingress(...))` remains exact nested ingress failure;
- `Failed(RequesterResponse(Frame(...)))` remains exact nested framing failure;
- `Failed(RequesterResponse(ResponseIo(...)))` remains exact nested response-I/O failure.

## 11. Shared-current authority custody

FQ clones only the existing cloneable `SharedCurrentCapabilityAuthority` handle into the task.

No registry/policy snapshot is created.

Fresh current-authority reads remain governed by the existing EX/FB path.

## 12. Shared requester/rendezvous authority custody

FQ clones only the existing C03e-FP `SharedRequesterRendezvousAuthority` handle into the task.

All clones point to the same process-local requester/rendezvous runtime owner behind the existing Tokio mutex.

FQ does not change lock ordering, critical-section scope, provider capacity, duplicate semantics, registration behavior, or requester cleanup.

## 13. Requester-aware policy-source custody

FQ accepts one shared immutable requester-aware policy source through `Arc<S>` where `S` satisfies the exact worker `Send + Sync + 'static` bounds.

Only the outer `Arc` moves into the task.

No per-task policy snapshot, mutable policy state, fallback evaluator, or requester-identity substitution is introduced.

## 14. Dispatcher/verifier/cancellation ownership

The spawned task owns by value:

- dispatcher;
- verifier-time provider;
- caller-supplied cancellation future;
- clone handles to shared current authority;
- clone handle to shared requester/rendezvous authority;
- shared immutable requester policy-source handle.

FQ does not require dispatcher or verifier-time recovery after task completion.

## 15. Cancellation law

The spawned seam does not alter FL cancellation semantics.

The caller-supplied cancellation future is moved unchanged into exact FL.

FL remains authoritative for:

- ingress-first cancellation race before requester handoff;
- cancellation deferral through synchronized FB DR and FH terminal response after handoff;
- exact requester-response failure precedence;
- post-FH cancellation observation before next ingress.

FQ adds no timeout, task abort, cancellation rewrite, or supervisor cancellation pair.

## 16. No task-abort law

FQ does not expose the join handle and does not call `abort()`.

Abnormal join handling exists only to preserve ownership if Tokio reports abnormal task completion such as panic.

Future persistent shutdown remains separately gated and must use cooperative FL cancellation unless another checkpoint explicitly selects a different policy.

## 17. Completion custody shape

FQ materializes one bounded completion carrier that owns:

- exact recovered `AuthenticatedRemoteSessionRuntimeOwner`;
- `Result<exact FL worker stop, existing RemoteSessionSpawnedWorkerJoinError>`.

The carrier permits borrowing the recovered owner, reading the exact Copy terminal result, or consuming both parts by value.

It does not authorize a peer disposition.

## 18. Retained-stopped peer law

Every FQ completion, normal or abnormal, returns owner custody without automatically:

- closing the peer;
- dropping the peer as policy;
- restarting FL;
- resuming ingress;
- reusing the session;
- converting failure into cancellation;
- converting cancellation into failure.

FM retained-stopped semantics remain authoritative.

## 19. No drop-as-policy law

FQ's purpose is to prevent task completion from silently discarding authenticated-peer custody.

The authenticated-session owner remains recoverable at the ownership layer after one bounded spawned invocation.

Rust object destruction is not used as an implicit protocol disposition.

## 20. Existing capability executor paths remain unchanged

FQ does not change behavior of:

- `drive_spawned_capability_request_worker`;
- `drive_supervised_capability_request_worker`;
- `drive_persistent_remote_worker_collection`;
- repeated real-admission worker spawn;
- endpoint shutdown lifecycle.

Capability-specific close code 3 and 4 behavior remains isolated to historical capability-worker paths.

## 21. Persistent collection boundary remains closed

FQ does not:

- change `RemoteSessionPersistentWorkerEntry`;
- change persistent completion types;
- change active worker maps;
- substitute FL for capability workers;
- admit multiple requester-aware workers;
- alter duplicate-DeviceId admission;
- alter shutdown/drain loops.

The FQ spawned seam is bounded to one invocation and one joined task.

## 22. Identity law

FQ preserves:

- authenticated PRW application-session lineage as requester logical identity;
- exact authenticated logical `DeviceId` as session/device identity;
- dynamic IP/port as transient endpoint evidence only;
- `TransportIdentity` as lower transport evidence only;
- PRWM `request_id` as correlation only.

Arc identity, mutex address, task ID, join ordering, panic state, cancellation timing, or executor slot never becomes logical identity.

## 23. Requester-record lifecycle remains unchanged

FQ performs no requester/rendezvous record:

- retirement;
- removal;
- TTL expiration;
- rollback;
- reset;
- stop-driven cleanup.

Those remain later gates.

## 24. Candidate/reachability boundary remains closed

FQ authorizes no candidate query/selection, reachability evaluation, endpoint resolution, relay/direct-path selection, target transport establishment, port-forward activation, terminal activation, remote-session establishment, or rendezvous completion claim.

Requester `Accepted` remains accepted-for-continuation only.

## 25. Runtime and deployment boundary remains closed

FQ does not wire the spawned seam into:

- persistent worker admission;
- repeated real admission;
- listener/bootstrap lifecycle;
- `main.rs`;
- readiness publication;
- production worker activation.

FQ adds no dependency/workflow change, package, deployment, restart/recovery, or merge.

## 26. Canonical source shape

FQ source is intentionally split:

1. one dedicated executor child module containing recoverable cell, bounded spawn/join seam, completion carrier, and focused ownership tests;
2. one registration line in the existing executor module;
3. this contract.

No other source path is selected.

## 27. Validation requirement

Closure requires exact-final-head:

- PRW Rust Validation FULL PASS: locked graph, rustfmt, Clippy, tests, build;
- Android Validation FULL PASS if triggered for the exact head;
- expected auxiliary workflow skips only;
- strict FP...FQ compare proving exact FP merge base and only selected paths;
- immutable Drive audit with raw byte-exact readback.

Superseded candidate runs cannot be used as closure evidence.

## 28. Explicit non-goals

No persistent collection substitution.
No real-admission integration.
No listener/process activation.
No peer close.
No peer reuse.
No peer restart.
No requester cleanup.
No candidate/reachability continuation.
No target dialing.
No deployment.
No restart/recovery.
No merge.

## 29. Canonical closure target

`CLOSED_RECOVERABLE_SPAWNED_AUTHENTICATED_SESSION_CUSTODY_OWNED_FL_WORKER_SOURCE_MATERIALIZATION`

## 30. Canonical gate target

`C03E_FQ_RECOVERABLE_SPAWNED_AUTHENTICATED_SESSION_CUSTODY_OWNED_FL_WORKER_SOURCE_MATERIALIZED`

## 31. Next separately gated seam

After FQ closure, the conservative next checkpoint is:

**C03e-FR — recoverable persistent requester-aware worker entry/completion custody selection**.

FR should select only how the existing persistent executor map/admission/completion model must change to retain the FQ session-owner cell, cancellation controller, join handle, authenticated `DeviceId`, and exact FQ completion custody across multiple active FL workers.

FR should remain selection-only unless a concrete source contradiction requires otherwise. Real-admission integration, peer close/reuse, requester cleanup, candidate/reachability, target dialing, deployment, restart/recovery, and merge remain later gates.
