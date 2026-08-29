# Phase 152 C03e-FR — Recoverable Persistent Requester-Aware Worker Entry / Completion Custody Selection

Status: CLOSED

## 1. Scope

C03e-FR is a semantics-selection checkpoint only.

It selects the narrow persistent-collection ownership model required to carry the already-materialized C03e-FQ recoverable spawned requester-aware FL custody into a future persistent executor integration without changing the existing authenticated identity, FL cancellation law, retained-stopped peer law, requester/rendezvous authority synchronization, or real-admission semantics.

FR does **not** materialize a persistent requester-aware worker, substitute FL into the existing persistent collection, alter real admission, close or reuse a peer, clean requester records, select candidate/reachability state, dial target traffic, activate a listener, publish readiness, deploy, restart/recover the process, or merge.

## 2. Exact predecessor

Canonical predecessor is exact C03e-FQ:

- branch: `phase-152-c03e-fq-recoverable-spawned-authenticated-session-custody-owned-fl-worker-source-materialization-staging`
- head: `ca0a48e2eccdea568c37a58f1bb462a394c0b7e3`
- tree: `870167298d85092439ab915dd27520edfae63c8e`
- FQ contract blob: `e3a052fdf927457678ff098c58ecd6dc1b9e86ec`
- executor registration blob: `5d2dec029050fcc6215439bf3b377da7064b980e`
- recoverable spawned FL module blob: `b42e870e7bf3a6b38a72084e3341ba738a863a1d`

FQ is frozen.

## 3. Exact source audit guards

FR was selected against the exact FQ tree and these byte-stable sources:

1. persistent executor / repeated real-admission source
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - blob `5d2dec029050fcc6215439bf3b377da7064b980e`

2. FQ recoverable spawned requester-aware FL custody
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`
   - blob `b42e870e7bf3a6b38a72084e3341ba738a863a1d`

3. exact requester-aware FL worker and stop family
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `bc0b9c49471d515b721c9cf47cd27ec3111f32ca`

4. single-worker cooperative cancellation primitive
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_worker_cancellation.rs`
   - blob `1ee692595a7e900facee53478b05da2a7c96ff59`

5. Agent remote-session parent ownership/export surface
   - `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - blob `450f27574270f84a88f77276afb1618b84476035`

A contradiction in any of these exact guards requires a new gate rather than silent FR reinterpretation.

## 4. Existing persistent collection law that FR preserves

The current persistent executor model already establishes these relevant properties:

- active workers are keyed by logical `DeviceId`;
- the key is derived from the authenticated runtime owner rather than supplied as transport metadata;
- duplicate active `DeviceId` is rejected before worker spawn;
- worker entry owns one cancellation controller and one Tokio join handle;
- ready completions are reaped before shutdown/admission decisions on each supervisor poll;
- orderly supervisor shutdown stops new admission, requests cancellation for all still-active workers, and drains the same retained handles until the active map is empty;
- no task abort is used for orderly shutdown;
- closed admission input alone does not stop active workers or the supervisor;
- capacity remains bounded by `MAX_REGISTERED_DEVICES`;
- completed active keys cease occupying the active-worker map and therefore do not remain scheduler tombstones.

FR preserves those scheduler properties unless recoverable requester-aware ownership requires a narrower entry/completion representation.

## 5. Why the existing capability-worker entry is insufficient for FL

The existing persistent capability-worker entry conceptually contains only:

`{ cancellation_controller, JoinHandle<AuthenticatedRemoteSessionWorkerStop> }`

The existing capability worker moves the authenticated-session owner by value into the spawned task. Its terminal stop law already owns capability-specific peer close behavior, so normal completion does not need to return the session owner to the supervisor.

Exact FL is different:

- FM/FO/FQ require every normal or abnormal FL completion to leave the authenticated-session owner recoverable;
- the recovered peer remains `retained-stopped`;
- FL does not widen capability code-3/code-4 close semantics;
- abnormal Tokio completion must not lose session-owner custody.

Therefore FR rejects direct reuse of the old entry shape for requester-aware FL.

A future requester-aware persistent entry must retain recoverable owner custody **outside the spawned task**.

## 6. Selected active-map identity representation

The future requester-aware active collection is selected conceptually as:

`HashMap<DeviceId, RecoverableRequesterAwareWorkerEntry>`

The map key is the exact authenticated logical `DeviceId` derived from the authenticated session owner before the owner enters recoverable task custody.

The `DeviceId` key remains the only collection scheduling identity.

FR does not select:

- IP address;
- port;
- endpoint tuple;
- `TransportIdentity`;
- PRWM `request_id`;
- Tokio task ID;
- `Arc` pointer identity;
- mutex address;
- join ordering;
- cancellation timing;
- session-owner cell address

as logical identity or as an alternative worker key.

The entry does not need a second authoritative copy of `DeviceId`; the map key is sufficient. A completion may own the key by value when the entry leaves the active map.

## 7. Selected recoverable requester-aware entry shape

The future requester-aware persistent entry must own exactly the persistent supervision custody needed for one active FL worker:

1. **recoverable authenticated-session owner cell**
   - conceptually the same FQ shape:
     `Arc<TokioMutex<Option<AuthenticatedRemoteSessionRuntimeOwner>>>`;
   - supervisor/map entry retains one `Arc` clone;
   - spawned task receives one `Arc` clone;
   - authenticated-session owner itself is not cloned;
   - worker does not `take()` the owner before FL.

2. **one non-cloneable cooperative cancellation controller**
   - exact existing `RemoteSessionWorkerCancellationController`;
   - paired signal is moved into exact FL worker;
   - controller remains only in supervisor entry custody.

3. **one exact Tokio join handle**
   - task output is exact
     `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`;
   - raw Tokio join error, task ID and panic payload remain hidden;
   - abnormal completion maps only through existing
     `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

No dispatcher, verifier-time provider, policy snapshot, requester runtime clone, transport identity, retry token, peer-close authority, or requester cleanup authority is retained in the persistent entry merely for recovery.

## 8. Selected spawn custody sequence

A future persistent materialization must preserve this ownership order:

1. receive one already-authenticated session owner through the existing authenticated admission lineage;
2. derive/copy the exact logical `DeviceId` from that owner while the owner is still directly available;
3. ensure the active map has no entry for that exact authenticated `DeviceId`;
4. create exactly one cooperative cancellation controller/signal pair;
5. place the exact authenticated-session owner into one recoverable owner cell;
6. retain one owner-cell handle in supervisor entry custody;
7. move a second owner-cell handle plus the cancellation signal and exact FL dependencies into exactly one spawned task;
8. worker locks the cell and borrows the contained owner mutably for exact FL;
9. insert the supervisor entry under the authenticated `DeviceId` key.

No worker is allowed to remove the owner from the cell before FL.

No second task may concurrently borrow the same owner cell.

No peer close, reuse, restart, retry, candidate/reachability continuation, target dial, or requester cleanup is coupled to insertion.

## 9. Exact FL stop preservation inside the join handle

Normal Tokio task completion returns the exact existing FL stop unchanged:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

Therefore the persistent layer must preserve:

- `Cancelled`;
- `Failed(Ingress(...))`;
- `Failed(RequesterResponse(Frame(...)))`;
- `Failed(RequesterResponse(ResponseIo(...)))`.

FR rejects translation into:

- `AuthenticatedRemoteSessionWorkerStop`;
- capability code-3/code-4 terminal classes;
- generic collection failure;
- peer-close result;
- admission failure;
- requester cleanup result.

A normal task join containing an FL `Failed(...)` is still a normal Tokio join and must not be reclassified as abnormal task completion.

## 10. Selected completion custody

A future requester-aware persistent completion record must own:

1. exact logical `DeviceId` removed from the active map;
2. exact recovered `AuthenticatedRemoteSessionRuntimeOwner`;
3. exact terminal result:

`Result<RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

The completion record is ownership-bearing and therefore is not selected as `Copy` or `Clone` merely for convenience.

The exact session owner is recovered before the completion is emitted to the higher owner.

No completion callback may receive only a join result while session-owner custody remains stranded in an internal owner cell.

## 11. Normal completion recovery law

When the join handle reaches normal completion:

1. exact FL task is already terminal;
2. the task's owner-cell mutex guard has been released after FL return;
3. supervisor detaches/removes the completed entry from the active-worker map;
4. supervisor recovers the exact authenticated-session owner from the retained owner cell;
5. supervisor composes one completion with the removed `DeviceId`, recovered owner, and exact `Ok(FL stop)`;
6. completion is transferred to the higher-owner completion sink.

The recovered owner remains **retained-stopped**.

Normal completion does not itself:

- close the peer;
- intentionally drop the owner as protocol disposition;
- restart FL;
- reuse the peer;
- begin next ingress;
- run requester cleanup;
- continue candidate/reachability work;
- dial a target.

## 12. Abnormal join recovery law

When Tokio reports abnormal task completion:

1. task unwinding/termination releases any owner-cell guard;
2. because worker never removed the owner from the cell, exact authenticated-session owner remains recoverable;
3. supervisor detaches/removes the entry from the active map;
4. supervisor recovers the exact owner from the retained cell;
5. join failure maps only to existing
   `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`;
6. completion is emitted as:
   `DeviceId + recovered owner + Err(AbnormalTaskCompletion)`.

No synthetic FL stop is fabricated.

No panic payload, task ID, raw `JoinError`, retry token, or transport-close code is exposed.

The recovered peer remains retained-stopped with health unspecified.

Abnormal join does **not** authorize automatic peer close, peer reuse, restart, retry, requester cleanup or worker replacement.

## 13. Active-map removal precedes completion publication

FR selects this ordering for a terminal entry:

**join terminal -> detach active entry -> recover exact owner -> publish one completion**.

The worker is no longer considered active only after the exact entry is detached from the active map.

Completion publication occurs only after recoverable session-owner custody has been successfully reassembled.

This preserves the existing collection concept that completed workers cease occupying active capacity.

It also ensures a future admission poll does not treat a terminal/recovered worker as still active merely because completion delivery is pending.

## 14. No retained-stopped DeviceId tombstone is selected

FR does **not** invent a new persistent `DeviceId` tombstone, quarantine set, stopped-peer reservation, or hidden completion map after the active entry is detached.

This is deliberate:

- current collection duplicate semantics are explicitly **duplicate active device** semantics;
- current completed-worker behavior frees active collection capacity;
- FO/FQ select retained-stopped peer custody but do not select a scheduler tombstone;
- adding an indefinite retained-completion reservation would silently widen admission semantics and could consume bounded capacity without an active worker.

Therefore a removed completion does not remain in the active map solely to block future admissions.

This does **not** mean the recovered peer may be reused.

A later session for the same logical `DeviceId` must still arrive through the separately authoritative admission/authentication path. The old recovered peer remains separate higher-owner retained-stopped custody until a later explicit peer-disposition gate decides what to do with it.

The active-map vacancy is a scheduler fact, not protocol peer-disposition authority.

## 15. Duplicate active DeviceId law remains unchanged

While a requester-aware worker entry remains active, a second candidate for the same authenticated logical `DeviceId` is rejected before spawning another worker.

FR does not weaken the existing duplicate-active-device barrier.

It also does not reinterpret a pre-authentication expected `DeviceId` as proof of post-authentication identity. Exact authenticated `DeviceId` derived from successful authenticated session ownership remains authoritative for active map insertion.

## 16. Ready completion ordering

FR preserves the existing supervisor ordering preference:

ready worker completions are reaped before evaluating orderly shutdown or admitting another candidate on the same supervisor wake.

For requester-aware workers, "reaped" now means more than polling the join handle. It includes:

- detaching the terminal active entry;
- recovering exact session owner;
- composing exact completion custody;
- publishing that completion.

A ready terminal worker must not be unnecessarily cancellation-signalled merely because shutdown becomes ready on the same wake after the worker has already completed.

## 17. Cooperative cancellation custody

FR reuses the existing single-worker cancellation primitive unchanged.

For every active requester-aware entry:

- supervisor owns one `RemoteSessionWorkerCancellationController`;
- task owns the paired one-shot signal future;
- controller is not cloned;
- repeated cancellation requests remain idempotent;
- dropping the controller alone does not constitute cancellation;
- controller request performs no peer close, task abort or join.

The cancellation signal is passed unchanged into exact FL.

Exact FL remains sole authority for cancellation-safe boundaries.

## 18. FL cancellation boundary remains authoritative during persistent shutdown

Exact FL already establishes:

- cancellation may win while EX ingress remains pending;
- once requester handoff exists, cancellation is not polled during exact FB/FH requester critical custody;
- exact requester frame/response failure wins if it occurs while cancellation has become ready during that protected interval;
- after FH success, cancellation is observed before another EX cycle.

Persistent shutdown must not bypass this law.

Therefore FR rejects:

- Tokio task abort;
- opaque timeout-drop of the FL future;
- dropping the owner-cell task to force shutdown;
- direct stream close as a cancellation substitute;
- cancellation injection into FB/FH;
- a second cancellation race around the whole FL task.

A shutdown drain may therefore wait for one in-progress requester FB/FH critical section to reach its existing terminal boundary before FL reports `Cancelled` or exact failure.

## 19. Selected orderly shutdown / drain law

When persistent supervisor shutdown wins:

1. ready completions are reaped first under the selected ordering;
2. admission stops;
3. supervisor iterates every still-active requester-aware entry and requests cooperative cancellation through its retained controller;
4. no active entry is removed merely because cancellation was requested;
5. the same join handles remain retained and driven;
6. each handle is drained to terminal completion;
7. each terminal entry is detached from the active map;
8. exact session owner is recovered from the retained cell;
9. exact completion is published;
10. supervisor returns only after the requester-aware active map is empty.

There is no selected task-abort fallback.

There is no selected shutdown timeout that destroys requester custody.

There is no selected automatic peer-close pass before or after drain.

## 20. Admission-source closure remains non-terminal

Closing a future requester-aware admission/request source does not itself cancel active workers and does not itself end the supervisor.

The separately owned supervisor-shutdown future remains the orderly shutdown authority, preserving the current persistent collection law.

## 21. Capacity law remains unchanged

Requester-aware active collection capacity remains bounded by the existing `MAX_REGISTERED_DEVICES` ceiling.

FR does not create a second independent requester capacity ceiling.

Recovered completions transferred out of the active map are not counted as active worker capacity merely because the higher owner still possesses a retained-stopped peer.

Any later policy that wants to bound unresolved retained-stopped completions is a separate lifecycle/disposition gate and is not selected here.

## 22. Shared current authority remains clone-handle based

Future spawned requester-aware workers may receive the already-existing cloneable shared-current authority handle exactly as selected/materialized before FQ.

The underlying registry/policy authority remains one current source of truth.

No per-task registry/policy snapshot is selected.

Protected work continues to use fresh current authority reads through existing exact seams.

## 23. Shared requester/rendezvous authority remains one synchronized process-local owner

FR preserves the FO/FP synchronization law:

- one cloneable shared handle around one process-local requester/rendezvous runtime owner;
- Tokio async mutex as the already-selected synchronization primitive;
- no provider clone/snapshot/shard;
- exact lock order remains requester/rendezvous mutex -> shared-current registry/policy read;
- requester guard is released before response I/O.

Persistent collection ownership does not add another requester authority lock or reverse existing lock order.

## 24. Immutable policy source sharing remains unchanged

A future persistent requester-aware worker may receive only the already-selected immutable requester-aware policy source sharing form.

FR does not select per-worker mutable policy copies or snapshots.

Policy-source sharing is not logical identity and does not become peer/session custody.

## 25. Dispatcher and verifier-time ownership

FR preserves FQ's bounded worker dependency law:

- dispatcher moves by value into one spawned task;
- verifier-time provider moves by value into one spawned task;
- neither is required to be recovered with the session owner after task completion;
- exact FL continues fresh verifier-time sampling through its existing internal transaction boundaries.

No verifier-time sample is cached in the persistent entry as authorization state.

## 26. Completion callback / collection law

FR preserves immediate bounded completion transfer rather than selecting an unbounded internal completion history.

A future materialization may use the existing callback-style completion sink or another separately gated bounded transfer surface, but one invariant is fixed:

**the supervisor must transfer complete custody — DeviceId + recovered owner + exact FL/join terminal result — rather than report a partial terminal fact and retain hidden peer ownership.**

FR does not select:

- an unbounded completion vector;
- permanent retained-completion map;
- implicit peer disposal when callback returns;
- automatic restart from completion callback;
- automatic candidate/reachability continuation from completion callback.

## 27. Re-admission after terminal active removal

Once a requester-aware entry has reached terminal join, been detached, and had its owner recovered, that worker no longer occupies the active map.

A later admission for the same logical `DeviceId` is governed by the existing admission/authentication and duplicate-**active** semantics.

FR does not authorize constructing that later admission from the recovered retained-stopped peer.

It must be independently admitted/authenticated through the appropriate existing path.

FR also does not select a rule that the old retained-stopped owner must be closed before an independently authenticated later session can occupy the active worker slot. Such a stronger single-peer-per-DeviceId disposition law is outside this collection-custody checkpoint.

## 28. No capability close-law reuse

Requester-aware FL completion does not inherit capability-only:

- code 3 `remote capability session terminated`;
- code 4 `remote capability session shutdown`.

FR does not invent a requester-aware whole-peer close code.

Orderly persistent shutdown requests FL cancellation only. It does not close the peer as a substitute for cooperative cancellation.

After owner recovery, peer disposition remains a higher-owner retained-stopped question.

## 29. No automatic peer reuse

Possession of a recovered authenticated-session owner in a completion record does not authorize:

- calling FL again on the same peer;
- restarting a worker;
- accepting another stream;
- replaying requester DR;
- sending another acknowledgement;
- target dialing;
- port forwarding;
- terminal activation.

A later explicit lifecycle gate must authorize any reuse.

## 30. No requester-record cleanup

FR selects no requester/rendezvous authority mutation on worker completion.

Specifically no:

- requester retirement;
- requester removal;
- registration rollback;
- TTL expiration;
- cancellation-triggered cleanup;
- abnormal-join cleanup;
- shutdown cleanup

is selected.

Requester record lifetime remains a separate gate.

## 31. No candidate/reachability continuation

A requester-visible accepted acknowledgement remains accepted-for-continuation only.

Neither FL completion nor persistent completion custody implies:

- target registration success beyond already-completed DR semantics;
- current candidate availability;
- reachability success;
- endpoint selection;
- relay/direct-path decision;
- target dial success;
- rendezvous completion;
- remote-session establishment.

FR performs no candidate/reachability continuation.

## 32. No production substitution yet

FR does not modify or replace current:

- `RemoteSessionPersistentWorkerEntry<T>`;
- `RemoteSessionRegisteredWorkerCompletion`;
- `run_persistent_worker_collection(...)`;
- `spawn_registered_worker(...)`;
- `drive_persistent_remote_worker_collection(...)`;
- `drive_repeated_real_remote_admission_collection(...)`;
- endpoint lifecycle teardown;
- listener/bootstrap/process lifecycle.

Existing capability-only production-adjacent paths remain byte-stable in FR.

The selected requester-aware entry/completion model is conceptual until a separately authorized source-materialization checkpoint.

## 33. Materialization constraints for the next source gate

A future source-materialization checkpoint may materialize only the selected requester-aware persistent entry/completion custody primitives and focused tests.

It must preserve:

- authenticated `DeviceId` map key;
- FQ-equivalent recoverable owner cell;
- one cancellation controller;
- one join handle returning exact FL stop;
- exact owner recovery after normal and abnormal join;
- exact completion ownership;
- ready-completion-first ordering;
- cancellation-all-then-drain shutdown law;
- no task abort;
- no peer close/reuse;
- no requester cleanup;
- no real-admission substitution unless separately authorized.

## 34. Rejected alternatives

FR explicitly rejects:

1. moving `AuthenticatedRemoteSessionRuntimeOwner` solely into the persistent task;
2. using only `{controller, JoinHandle<FL stop>}` without recoverable owner custody;
3. cloning the authenticated-session owner;
4. using transport endpoint/IP/port as map identity;
5. mapping FL stop to capability-worker stop;
6. closing the peer on every FL stop;
7. aborting FL tasks during orderly shutdown;
8. dropping an in-progress requester FB/FH future to satisfy shutdown;
9. inventing a permanent completed-DeviceId tombstone;
10. keeping a terminal entry marked active after exact owner recovery solely to block future admission;
11. silently reusing the recovered peer for a replacement worker;
12. adding requester-record cleanup;
13. substituting requester-aware FL directly into real admission in FR;
14. candidate/reachability continuation;
15. target dialing/listener activation/readiness/deployment/restart/merge.

## 35. Selected canonical law

The C03e-FR canonical selection is:

> A future persistent requester-aware FL collection remains keyed by authenticated logical `DeviceId`, but each active entry must retain FQ-equivalent recoverable authenticated-session owner custody beside one cooperative cancellation controller and one join handle returning exact FL stop. Terminal join detaches the active entry, recovers the exact session owner, and publishes one ownership-bearing completion containing `DeviceId + recovered owner + Result<exact FL stop, bounded join error>`. Orderly shutdown reaps already-ready completions first, requests cancellation for every still-active entry, then drains the same handles and recovers every owner without task abort or peer close. Completion frees active scheduler capacity but does not authorize reuse or disposition of the retained-stopped peer and does not create a DeviceId tombstone.

## 36. Canonical closure

`CLOSED_RECOVERABLE_PERSISTENT_REQUESTER_AWARE_WORKER_ENTRY_COMPLETION_CUSTODY_SELECTION`

## 37. Canonical gate

`C03E_FR_RECOVERABLE_PERSISTENT_REQUESTER_AWARE_WORKER_ENTRY_COMPLETION_CUSTODY_SELECTED`

## 38. Next separately gated checkpoint

**C03e-FS — recoverable persistent requester-aware worker entry/completion custody source materialization**

FS should remain pre-production and may materialize only the FR-selected requester-aware persistent entry/completion primitives plus focused ownership/reaping/shutdown tests over injected workers. It must not yet substitute requester-aware FL into repeated real admission, close/reuse peers, clean requester records, continue candidate/reachability, dial targets, activate listener/bootstrap, publish readiness, deploy, restart/recover, or merge.
