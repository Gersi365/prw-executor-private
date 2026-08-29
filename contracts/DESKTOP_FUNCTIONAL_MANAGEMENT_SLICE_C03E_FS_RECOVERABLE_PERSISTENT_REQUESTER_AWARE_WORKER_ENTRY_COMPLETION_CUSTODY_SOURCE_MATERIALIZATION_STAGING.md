# Phase 152 C03e-FS — Recoverable Persistent Requester-Aware Worker Entry / Completion Custody Source Materialization

Status: VALIDATING

## 1. Scope

C03e-FS materializes only the C03e-FR-selected pre-production persistent requester-aware worker custody primitives and one injected collection seam.

FS does not substitute requester-aware FL into repeated real admission, does not modify the existing capability persistent collection behavior, does not close/reuse/restart authenticated peers, does not clean requester records, does not continue candidate/reachability, does not dial targets, does not activate listener/bootstrap/readiness, does not deploy, restart/recover the process, or merge.

## 2. Exact predecessor

Canonical predecessor is exact C03e-FR:

- branch: `phase-152-c03e-fr-recoverable-persistent-requester-aware-worker-entry-completion-custody-selection-staging`
- head: `ec0a3cf45f5303822fa6d4fdaf87b9ed082647f0`
- tree: `634519b726596fcd972bd4e7cf8500ddb2db1610`
- FR contract blob: `07226cc921e69f675ab6a853174965fd9b9fc9df`

FR is frozen.

## 3. Exact predecessor source guards

FS materialization is bounded by these exact FR-head source guards:

1. executor / current capability persistent collection:
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - blob `5d2dec029050fcc6215439bf3b377da7064b980e`

2. FQ recoverable spawned requester-aware FL custody:
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`
   - blob `b42e870e7bf3a6b38a72084e3341ba738a863a1d`

3. exact requester-aware FL worker / stop family:
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `bc0b9c49471d515b721c9cf47cd27ec3111f32ca`

4. existing cooperative cancellation primitive:
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_worker_cancellation.rs`
   - blob `1ee692595a7e900facee53478b05da2a7c96ff59`

5. parent remote-session ownership/export surface:
   - `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - blob `450f27574270f84a88f77276afb1618b84476035`

Any contradiction with these guards requires a new gate rather than silent FS widening.

## 4. Materialized source shape

FS adds one child module below the existing FQ recoverable spawned requester-aware worker module.

The FQ parent module receives only one child-module registration line.

All new entry/completion/reaping/shutdown logic lives in the new child module.

The large executor source remains byte-stable.

The existing requester-aware FL source remains byte-stable.

The existing cancellation primitive remains byte-stable.

No Cargo dependency or workflow is added to the canonical FS tree.

## 5. Materialized generic recoverable entry primitive

FS materializes an internal generic entry primitive conceptually equivalent to:

`RecoverablePersistentWorkerEntry<O, T>`

The entry owns exactly:

- one `Arc<TokioMutex<Option<O>>>` recoverable owner cell;
- one exact existing `RemoteSessionWorkerCancellationController`;
- one `JoinHandle<T>`.

The entry owns no retry token, transport identity, policy snapshot, requester cleanup authority, peer-close authority or replacement-task authority.

## 6. Exact requester-aware entry specialization

FS materializes the exact requester-aware specialization:

`RecoverableRequesterAwareWorkerEntry`

with:

- owner = `AuthenticatedRemoteSessionRuntimeOwner`;
- task output = `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`.

This is the FR-selected active-entry custody shape.

It does not itself spawn or activate production work.

## 7. Materialized generic completion primitive

FS materializes an ownership-bearing terminal completion primitive conceptually equivalent to:

`RecoverablePersistentWorkerCompletion<K, O, T>`

The completion owns:

- exact removed active-map key;
- exact recovered owner by value;
- `Result<T, RemoteSessionSpawnedWorkerJoinError>`.

The completion is not selected as `Copy` or `Clone`.

It can be transferred by value so a later higher owner receives complete custody rather than a join fact detached from peer ownership.

## 8. Exact requester-aware completion specialization

FS materializes exact requester-aware completion custody:

- key = logical authenticated `DeviceId`;
- owner = `AuthenticatedRemoteSessionRuntimeOwner`;
- normal task output = exact requester-aware FL worker stop;
- abnormal task result = existing `RemoteSessionSpawnedWorkerJoinError`.

The completion preserves exact FL stop taxonomy without translation.

## 9. Terminal detach / recovery ordering

FS materializes the FR-selected terminal order:

**join terminal -> detach active entry -> recover exact owner -> map/preserve terminal result -> publish completion**

Ready join results are first collected from active entries.

Each terminal key is then removed from the active map before owner recovery.

Only after removal does the supervisor synchronously recover exact owner custody from the retained owner cell.

Only after exact owner recovery does the completion callback receive the ownership-bearing completion.

This prevents completion publication while owner custody remains stranded in an internal cell.

## 10. Terminal owner recovery invariant

Because a terminal Tokio join means the task stack has completed/unwound, the worker-held owner-cell mutex guard must already be released.

FS therefore recovers owner custody from a terminal entry using non-waiting mutex acquisition.

Failure to acquire that mutex after terminal join is treated as an internal ownership invariant violation rather than a new runtime/retry state.

The owner cell must still contain exact owner value because FQ/FR prohibit worker `take()` before FL execution.

## 11. Normal task result preservation

A normal join result containing exact FL stop remains:

`Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop)`.

FS does not translate:

- `Cancelled`;
- `Failed(Ingress(...))`;
- `Failed(RequesterResponse(Frame(...)))`;
- `Failed(RequesterResponse(ResponseIo(...)))`.

A normal Tokio task returning FL `Failed(...)` is still a normal task join.

## 12. Abnormal task classification

Raw Tokio abnormal join continues through exact existing:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

FS exposes no raw `JoinError`, panic payload, Tokio task ID or runtime identity.

No synthetic FL stop is created for abnormal task completion.

Exact owner custody is recovered before abnormal completion publication.

## 13. Materialized injected collection seam

FS adds one pre-production generic persistent collection helper plus an exact requester-aware wrapper.

The collection helper owns only:

- bounded active-map scheduling;
- duplicate-active rejection;
- ready terminal reaping;
- owner recovery;
- completion transfer;
- cooperative shutdown cancellation;
- drain of retained join handles.

The caller injects:

- candidate stream;
- key extraction;
- entry construction/spawn;
- completion sink;
- duplicate rejection sink;
- supervisor shutdown future.

Therefore FS does not wire real admission or production worker spawning.

## 14. Active map identity

The exact requester-aware wrapper uses `DeviceId` as the active-map key.

FS does not introduce IP, port, endpoint tuple, `TransportIdentity`, request ID, task ID, pointer identity or mutex address as worker identity.

Any exact entry construction used by a later gate must derive the key from authenticated session ownership according to FR.

## 15. Duplicate active barrier

The injected collection checks the active map before invoking the supplied spawn/entry-construction closure.

When the key is already active:

- no second entry is spawned;
- candidate is returned to the rejection callback;
- existing `RemoteSessionWorkerAdmissionRejectionReason::DuplicateActiveDevice` is preserved.

FS creates no second duplicate error family.

## 16. Ready-completion-first scheduling

On each supervisor poll, FS first reaps ready terminal workers and recovers their owners.

Only afterward does the same poll consider:

1. supervisor shutdown;
2. new admission.

Therefore a worker already terminal on that wake is not spuriously cancellation-signalled because shutdown is also ready.

Its active slot is removed before admission/shutdown scheduling continues.

## 17. Active capacity

The exact requester-aware wrapper reuses existing persistent capacity validation against `MAX_REGISTERED_DEVICES`.

FS does not add a requester-specific second capacity ceiling.

Detached terminal completions no longer occupy the active map.

No persistent stopped-peer tombstone or quarantine is added.

## 18. No DeviceId tombstone

FS materializes no scheduler tombstone after a terminal entry is detached.

This preserves FR's distinction:

- active-map vacancy = scheduling fact;
- recovered retained-stopped peer custody = higher-owner lifecycle fact.

Vacancy does not authorize reuse of the recovered peer.

A later same-DeviceId session remains subject to separately authoritative admission/authentication.

## 19. Cooperative cancellation

Every active entry owns exactly one existing `RemoteSessionWorkerCancellationController`.

The corresponding signal belongs to the worker selected by the injected spawn closure.

FS does not clone cancellation authority.

Repeated cancellation request remains idempotent.

No controller request closes transport, aborts a task or joins a task.

## 20. Shutdown order

When supervisor shutdown wins after ready-completion reaping:

1. admission stops;
2. cancellation is requested for every still-active entry;
3. no entry or join handle is discarded;
4. the same active map is drained to empty;
5. each terminal entry is detached;
6. each exact owner is recovered;
7. each completion is published;
8. collection returns only after active map is empty.

No task-abort fallback is materialized.

No timeout-drop of the worker future is materialized.

## 21. FL cancellation law remains authoritative

FS adds no new cancellation race around exact FL.

A future exact requester-aware spawn continues to pass the existing cancellation signal into FL unchanged.

Therefore FL remains authoritative for:

- ingress-first pre-handoff cancellation ordering;
- cancellation deferral during FB/FH requester critical custody;
- requester-response failure precedence during that interval;
- post-FH cancellation observation before next ingress.

Persistent shutdown may wait for those exact protected boundaries.

## 22. Admission-source closure

The injected collection preserves the existing supervisor law that closing the admission source alone does not cancel active workers and does not terminate the supervisor.

The explicit supervisor-shutdown future remains authoritative for orderly collection termination.

## 23. Tests materialized

FS adds focused pre-production ownership tests covering:

1. normal terminal entry detachment + exact owner recovery + exact result preservation;
2. abnormal task completion + exact owner recovery + bounded join error preservation;
3. duplicate active key rejection before second spawn;
4. ready completion recovery before same-wake shutdown observation;
5. shutdown cancellation of all active entries followed by drain and owner recovery;
6. closed admission source not terminating supervisor before explicit shutdown;
7. preservation of existing registered-device capacity ceiling.

Tests use injected scalar owners/results and do not activate real remote admission or transport.

## 24. Existing FQ bounded spawned seam remains intact

FS does not replace or alter FQ's bounded `drive_recoverable_spawned_requester_rendezvous_worker` semantics.

FQ remains the exact proof that normal/abnormal task completion preserves authenticated-session owner custody.

FS reuses that custody model for persistent entry representation and reaping only.

## 25. Existing capability collection remains intact

FS does not modify:

- `RemoteSessionPersistentWorkerEntry<T>`;
- `RemoteSessionRegisteredWorkerCompletion`;
- `run_persistent_worker_collection(...)`;
- `drive_persistent_remote_worker_collection(...)`;
- capability worker stop taxonomy;
- capability code-3/code-4 close behavior.

Requester-aware FS primitives are separate pre-production surfaces.

## 26. Repeated real admission remains intact

FS does not modify:

- `spawn_registered_worker(...)`;
- `drive_repeated_real_remote_admission_collection(...)`;
- pre-auth expected-device scheduling;
- authenticated admission transaction;
- current capability-only persistent worker insertion;
- endpoint shutdown lifecycle.

No requester-aware FL worker is inserted by real admission in FS.

## 27. Shared requester/rendezvous authority remains intact

FP shared requester authority semantics remain unchanged:

- one process-local runtime owner;
- one cloneable Arc wrapper;
- one Tokio async mutex;
- no provider clone/snapshot;
- lock order requester authority -> current registry/policy read;
- requester mutex released before FH response I/O.

FS active entry adds no requester-authority lock.

## 28. Identity law

FS preserves:

- authenticated PRW session lineage as requester logical identity;
- logical `DeviceId` as worker map identity;
- dynamic IP/port as transient endpoint evidence only;
- `TransportIdentity` as lower transport evidence only;
- PRWM `request_id` as correlation only.

Mutex identity, Arc identity, task identity, join order, cancellation timing or active-slot position are not logical identity.

## 29. Peer lifecycle remains closed

FS performs no automatic peer close on:

- FL `Cancelled`;
- exact FL typed failure;
- abnormal join;
- supervisor shutdown;
- collection completion.

Capability code 3/code 4 are not widened.

Recovered owner remains retained-stopped.

FS does not authorize restart, reuse or next ingress on recovered peer.

## 30. Requester record lifecycle remains closed

FS performs no requester retirement, record removal, TTL cleanup, rollback, reset, cancellation cleanup, abnormal-join cleanup or shutdown cleanup.

## 31. Candidate/reachability boundary remains closed

FS performs no candidate query, candidate selection, reachability evaluation, endpoint resolution, relay/direct-path decision, target transport establishment, forwarding, terminal activation or rendezvous-completion claim.

Requester `Accepted` remains accepted-for-continuation only.

## 32. Runtime/deployment boundary remains closed

FS does not:

- wire requester-aware persistent collection into repeated real admission;
- replace production-adjacent capability workers;
- alter listener/bootstrap/process lifecycle;
- publish readiness;
- alter Android source;
- add dependencies/workflows;
- package;
- deploy;
- restart/recover;
- merge.

## 33. Source-materialization acceptance criteria

FS can close only if exact final tree proves:

- exact FR merge base;
- only FS contract + FQ parent registration + new FS child module are changed;
- no helper workflow path in canonical diff;
- Rust validation FULL PASS on exact final head;
- Android validation FULL PASS if the repository triggers it for the source-changing exact final head;
- immutable Drive audit raw readback is byte-exact;
- PR remains open/draft/unmerged with semantic `Status: CLOSED`.

## 34. Canonical closure target

`CLOSED_RECOVERABLE_PERSISTENT_REQUESTER_AWARE_WORKER_ENTRY_COMPLETION_CUSTODY_SOURCE_MATERIALIZATION`

## 35. Canonical gate target

`C03E_FS_RECOVERABLE_PERSISTENT_REQUESTER_AWARE_WORKER_ENTRY_COMPLETION_CUSTODY_SOURCE_MATERIALIZED`

## 36. Next separately gated checkpoint after successful FS closure

The next checkpoint should select only repeated real-admission substitution/integration semantics for requester-aware persistent FL custody.

It must decide how successful authenticated real admission composes exact requester-aware entry construction, shared requester authority, requester-aware policy source, completion custody, duplicate active behavior, and shutdown ownership without yet changing peer disposition, requester cleanup, candidate/reachability continuation, deployment or merge.
