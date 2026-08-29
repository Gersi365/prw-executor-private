# Phase 152 C03e-FO — Requester/Rendezvous Authority Synchronization and Owned/Persistent FL Custody Selection (Staging)

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-FO selects only the synchronization and custody law required before the already-materialized C03e-FL requester-aware cancellation-aware serial lifecycle worker can safely enter an owned/spawned or persistent multi-worker executor path.

FO does not materialize a synchronization wrapper, change FL/FN source, spawn an FL task, change the persistent collection, close or reuse a peer, activate candidate/reachability continuation, dial target traffic, wire a listener, deploy, restart/recover, or merge.

## 2. Exact predecessor

FO is based exactly on closed C03e-FN:

- predecessor branch: `phase-152-c03e-fn-borrowed-executor-drive-fl-source-materialization-staging`
- predecessor head: `e34ebc0187b3139890e47b2081e80e4f4bfc47f3`
- predecessor tree: `331873b4470ddbf4eaf984c2149fd2c953a816db`
- predecessor PR: `#290`, `Status: CLOSED`, draft/open/unmerged
- predecessor gate: `C03E_FN_BORROWED_EXECUTOR_DRIVE_FL_SOURCE_MATERIALIZED`

C03e-FN remains frozen.

## 3. Exact audited source guards

FO relies on these exact FN-head source blobs and does not mutate them:

1. FL requester-aware serial lifecycle and cancellation-aware worker
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `2a4bcbf48965b8ef5fa3202b3bb3ef46b3f96f31`

2. FN executor runtime including persistent capability collection and borrowed FL drive seam
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - blob `a8359a88cbe924ad5d75eb9121e6d5b1bc0a8ee8`

3. process-local requester/rendezvous runtime owner
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
   - blob `68ba74e82cf703664b7ee090a10fc1c6cce1609d`

4. bounded requester/rendezvous in-memory provider
   - `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`
   - blob `d01cfbc37433f6099e216397b9bf243aa55c53bc`

5. requester/rendezvous DR DI -> DP -> DK -> DN composition
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
   - blob `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`

6. requester/rendezvous policy-authorized owned provenance carrier
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`
   - blob `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`

7. immutable bounded requester-aware policy source
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`
   - blob `f7377011a3ab2034c14d9018a5c0f268f6660ffa`

8. shared-current registry/policy authority
   - `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
   - blob `50356b47d3c5304b67edd424e9286beb028ace16`

9. parent remote-session runtime module
   - `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - blob `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`

## 4. Audited synchronization problem

The exact requester/rendezvous provider is one process-local bounded `Vec`-backed authority state.

It explicitly retains no synchronization primitive and all provider mutations/authorization surfaces require mutable access.

`CandidatePublicationRequesterRendezvousRuntimeOwner` owns exactly one provider by value and exposes registration through `&mut self` only. It is not clone-backed shared authority and exposes no raw provider sharing surface.

Exact FL currently accepts `&mut CandidatePublicationRequesterRendezvousRuntimeOwner` for the entire worker invocation. That shape is safe for one borrowed FN invocation, but it prevents concurrent persistent FL workers from sharing the same process-local authority without a separately selected synchronization/custody mechanism.

Tokio current-thread execution does not remove this ownership problem. Multiple spawned futures can interleave on the same thread.

## 5. Audited persistent-worker custody problem

The existing capability-only persistent collection moves one `AuthenticatedRemoteSessionRuntimeOwner` by value into one spawned task.

Its completion surface returns only logical `DeviceId` plus `Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

That is valid for the historical capability-only worker because its selected worker semantics perform code-3/code-4 peer close before normal terminal return.

FL deliberately performs no whole-peer close on `Cancelled` or `Failed(...)`.

Therefore direct substitution of FL into the existing persistent task body would allow normal task completion to discard authenticated-peer custody without an explicitly selected peer disposition. Abnormal join would also provide no recoverable session owner.

FO must solve both problems before persistent FL activation is permitted.

## 6. Selected synchronization principle

FO selects:

**one cloneable Agent-owned shared requester/rendezvous authority handle around exactly one existing process-local runtime owner, using one Tokio async mutex; provider state is never cloned or snapshotted.**

Conceptually:

```text
SharedRequesterRendezvousAuthority
    -> Arc<TokioMutex<CandidatePublicationRequesterRendezvousRuntimeOwner>>
```

The exact Rust type/name remains a source-materialization detail.

The wrapper, not raw `Arc`, raw mutex, mutex guard, or provider reference, should be the operation-facing authority surface.

## 7. Why an async mutex is selected

FO selects Tokio async mutex semantics rather than a blocking standard-library mutex because requester-aware workers execute inside asynchronous current-thread runtime work.

A contended blocking mutex could block the only runtime thread while another future needs that thread to release the same lock.

The selected async mutex permits contention to suspend only the requesting future.

FO does not treat mutex fairness, wake order, task order, executor slot, or lock acquisition order among independent requesters as logical authority or identity.

## 8. Single-state law

There remains exactly one process-local requester/rendezvous authority state.

Cloning the future shared authority handle clones only outer synchronization ownership.

It must not clone:

- provider records;
- requester sessions;
- lifecycle state;
- configured provider capacity;
- current/retired record state;
- authorization result;
- registration result.

No worker-local requester-authority snapshot is selected.

## 9. Exact synchronized DR critical section

The shared requester-authority lock is acquired only after one exact FL requester handoff has reached the existing FB DR continuation boundary.

It is not held while the worker waits for ordinary capability traffic, waits for a new control stream, sends the terminal requester acknowledgement, or waits between independent requester transactions.

Once acquired, the lock spans the exact existing coherent DR composition:

1. acquire requester/rendezvous async mutex;
2. while that guard remains held, enter exact existing `SharedCurrentCapabilityAuthority::with_current_authority(...)` read operation;
3. execute exact existing DI -> DP -> DK -> DN `validate_authorize_and_register_requester_rendezvous_start(...)` synchronously inside that current-authority read;
4. finish the exact DR result;
5. release the current registry/policy read guard;
6. release requester/rendezvous mutex before FD/FH response framing/write.

No requester-authority lock remains held during terminal response I/O.

## 10. Coherent authorization/registration law

FO deliberately does not split DI/DP/DK authorization from DN registration across an unlocked interval.

The existing composition currently keeps current registry/policy read coherence through the registration mutation.

A future concurrent adaptation must preserve that law rather than returning an owned policy-authorized carrier, releasing current authority, and registering later without revalidation.

Therefore the requester mutex is acquired before the existing current-authority read and remains held through the synchronous DI -> DP -> DK -> DN composition.

## 11. Lock-order law

FO selects one nested order for this exact requester DR operation:

**requester/rendezvous async mutex first -> current registry/policy authority read second.**

No future FO-descendant source may acquire the requester/rendezvous mutex while already holding a current-authority read or write guard.

No reverse nested lock order is authorized.

The shared requester authority wrapper should encapsulate this order so ordinary worker callers cannot manipulate raw guards.

## 12. No lock during response I/O

The requester/rendezvous authority guard must be released before FD/FH acknowledgement construction/write reaches asynchronous response I/O.

This prevents one slow requester response stream from serializing unrelated requester registrations.

It also preserves the existing separation between:

- semantic DR result;
- FD projection/framing;
- FF response I/O;
- FH terminal composition.

## 13. Cancellation interaction

Existing FL cancellation deferral after requester handoff remains authoritative.

Waiting to acquire the selected requester/rendezvous mutex is part of the exact post-handoff FB DR critical section.

Therefore caller cancellation is not polled as a new competing outcome while one requester handoff is waiting for synchronized DR authority access.

After exact DR and FH complete successfully, existing FL post-FH cancellation observation remains unchanged.

FO selects no lock-wait cancellation shortcut, abandoned handoff, registration retry, duplicate acknowledgement, or replacement transaction.

## 14. Requester-authority contention law

Multiple persistent FL workers may concurrently own independent authenticated sessions and execute independent ingress/response work.

Only requester/rendezvous DR authority composition is serialized through the one shared requester authority lock.

Mutex contention is not:

- requester identity;
- priority authority;
- admission authority;
- retry authority;
- peer failure;
- session failure;
- rendezvous completion.

No waiter may bypass the exact DR gate using a cloned provider or stale snapshot.

## 15. Provider capacity and duplicate law

The existing bounded provider capacity remains one global process-local capacity.

Existing exact duplicate detection and capacity exhaustion remain serialized against the same underlying record set.

FO does not partition capacity per worker, per DeviceId, per transport, per task, or per executor slot.

Provider record order remains non-authoritative.

## 16. Policy-source custody selection

The existing bounded requester-aware policy source is immutable after one-shot construction and exposes read-only lookup.

For future spawned/persistent FL integration, FO selects one shared immutable process-local policy-source allocation, conceptually `Arc<S>`, where the concrete source satisfies the required `Send + Sync + 'static` worker bounds.

Worker tasks may clone only the outer shared policy-source handle.

They must not clone policy bindings into per-task snapshots or substitute a process-global fallback evaluator.

The exact authenticated requester session remains the lookup principal.

## 17. Shared-current capability authority remains unchanged

The existing `SharedCurrentCapabilityAuthority` remains the sole current registry/principal-agnostic capability authority.

FO does not merge requester/rendezvous provider state into that RwLock.

The two authorities remain distinct because they have different lifecycle, mutation, and ownership semantics.

The selected nested DR operation coordinates them only at the exact DI -> DP -> DK -> DN composition boundary.

## 18. Selected authenticated-session custody principle for spawned FL

FO selects:

**supervisor-retained recoverable authenticated-session custody; the spawned FL task must not be the only by-value owner of the authenticated session.**

A future spawned FL integration should place the exact `AuthenticatedRemoteSessionRuntimeOwner` inside one per-worker supervisor-retained async ownership cell and let the task borrow it mutably for the worker lifetime.

Conceptually:

```text
Arc<TokioMutex<Option<AuthenticatedRemoteSessionRuntimeOwner>>>
```

The exact Rust wrapper/type name is deferred.

The active supervisor entry retains one clone of this cell while the worker task retains another.

## 19. Session-owner cell law

The per-worker session-owner cell contains exactly one authenticated-session owner.

The worker must not `take()` the owner out of the cell before running FL.

Instead it acquires the cell guard and borrows the contained owner as `&mut AuthenticatedRemoteSessionRuntimeOwner` for exact FL execution.

Consequently:

- normal FL stop releases the guard while the owner remains in the cell;
- task panic unwinds the guard while the owner remains in the cell;
- task cancellation/abort, if later introduced, must not first remove the owner from the cell;
- supervisor can recover the exact owner after the join handle is terminal.

FO itself authorizes no task abort.

## 20. Why recoverable cell custody is selected

This shape preserves FM's explicit no-drop-as-peer-policy rule.

If the session owner were moved directly into the spawned task, abnormal task completion could destroy authenticated-peer custody before the supervisor can apply any explicit later peer disposition.

Keeping the owner in a supervisor-retained cell makes normal and abnormal task completion recoverable at the ownership layer.

This does not mean the underlying transport is guaranteed healthy after an abnormal task result. It means only that explicit Rust owner custody remains available for a later typed disposition decision.

## 21. Worker task ownership law

A future persistent FL task may own by value:

- its dispatcher;
- its verifier-time provider;
- its cancellation signal;
- clone handles to shared current authority;
- clone handles to shared requester/rendezvous authority;
- clone handles to shared immutable requester policy source.

It must not be the sole by-value owner of the authenticated session.

Dispatcher/verifier-time recovery is not selected as required completion custody.

## 22. Exact FL stop law

Normal spawned FL completion preserves the exact existing:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

unchanged.

No capability-only stop mapping is selected.

`Cancelled` remains `Cancelled`.

`Failed(Ingress(...))` remains exact nested ingress failure.

`Failed(RequesterResponse(Frame(...)))` remains exact nested frame failure.

`Failed(RequesterResponse(ResponseIo(...)))` remains exact nested response-I/O failure.

## 23. Selected completion custody shape

After one persistent FL task reaches a terminal join result, the supervisor must recover the exact authenticated-session owner from the retained owner cell before removing the active entry.

The conceptual completion record contains:

- exact authenticated logical `DeviceId` key;
- exact recovered `AuthenticatedRemoteSessionRuntimeOwner` by value;
- `Result<exact FL worker stop, existing bounded spawned-worker join error>`.

Exact Rust naming is deferred.

No normal completion callback may receive only DeviceId + stop while silently discarding session-owner custody.

## 24. Normal FL completion disposition

For every exact normal FL stop:

- recover exact authenticated-session owner;
- preserve exact FL stop;
- mark that worker invocation terminal;
- remove the active worker entry only after recovery;
- return completion custody to the higher owner;
- do not automatically close peer;
- do not automatically restart worker;
- do not automatically resume ingress;
- do not reuse the session.

This is the existing FM retained-stopped peer law carried into owned task custody.

## 25. Abnormal join disposition

If Tokio reports abnormal task completion:

- preserve existing bounded `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion` or an exact future equivalent;
- recover the exact authenticated-session owner from the supervisor-retained owner cell;
- do not claim that FL returned a normal stop;
- do not synthesize `Cancelled`;
- do not reuse capability code 3 or code 4;
- do not intentionally drop the recovered owner as peer policy;
- do not restart the worker;
- do not begin another ingress cycle.

Transport/session health after abnormal join remains unknown and separately gated.

## 26. No task-abort policy

FO does not select task abort as shutdown or timeout behavior.

Future persistent FL shutdown should retain the existing cooperative model:

1. request cancellation through the existing controller/signal pair;
2. continue driving the exact same join handle;
3. let FL observe cancellation only at its existing safe boundaries;
4. await terminal task completion;
5. recover authenticated-session owner custody;
6. report exact FL stop or abnormal join result.

No forced abort is authorized merely to accelerate shutdown.

## 27. Persistent worker key law

The existing authenticated logical `DeviceId` remains the persistent active-worker key.

FO does not change duplicate-active-device admission semantics.

A dynamic IP/port, `TransportIdentity`, task ID, mutex address, cancellation controller, stream ID, request ID, or requester-authority lock order never becomes the active-worker logical identity.

## 28. Admission rejection custody

Existing duplicate active-device rejection remains fail-before-spawn and must continue returning the untouched admission candidate for explicit higher-owner custody.

FO does not authorize opening a second FL task for the same active logical `DeviceId` merely because the first task is waiting on requester-authority synchronization.

## 29. Persistent shutdown law

On higher-owner persistent shutdown:

- stop admitting new workers;
- request cancellation for all active FL workers;
- do not reuse capability-only close code 4 inside FL;
- drain the same task handles;
- recover every recoverable authenticated-session owner before active-entry removal;
- surface exact normal FL stop or abnormal join classification for each entry.

No automatic peer disposition is selected after recovery.

## 30. Peer close/reuse remains separately gated

FO solves custody, not final peer disposition.

After a normal or abnormal persistent FL completion, the recovered authenticated-session owner remains retained-stopped.

A later explicit policy must decide, per terminal class, whether to:

- close the peer with a newly selected requester-aware reason/code;
- retain for diagnostics;
- drain/retire;
- permit controlled reuse;
- destroy the owner after explicit disposition.

Until that gate closes, none of those actions is authorized.

## 31. Requester-authority record cleanup remains separate

Worker completion, cancellation, failure, abnormal join, persistent shutdown, or recovered session custody does not automatically retire/remove requester/rendezvous records.

FO does not select:

- requester record rollback;
- session-stop-driven retirement;
- TTL cleanup;
- retired-record removal;
- provider reset.

Existing provider lifecycle remains unchanged.

## 32. Candidate/publisher future synchronization law

If a later candidate-publication/publisher path needs access to the same requester/rendezvous provider, it must reuse the same shared requester-authority allocation.

It must not construct a second provider copy or independent synchronization wrapper around duplicated state.

FO does not materialize or activate that publisher-side operation.

Any future operation requiring both requester authority and current registry/policy authority must obey one separately documented lock-order law; reverse nested acquisition is prohibited.

## 33. No authority flattening

FO does not merge these concepts:

- authenticated session identity;
- current registry/policy authority;
- requester-aware policy source;
- requester/rendezvous registration authority;
- transport identity;
- request correlation.

Each remains a distinct authority/custody layer.

## 34. Identity law

FO preserves:

- authenticated PRW application-session lineage as requester identity;
- logical `DeviceId` as device identity and persistent worker key where already selected;
- dynamic IP/port as transient endpoint information only;
- `TransportIdentity` as lower transport evidence only;
- PRWM `request_id` as correlation only.

Mutex ownership, lock wait order, Arc allocation identity, task identity, join result, cancellation timing, or executor position does not become logical identity.

## 35. Candidate/reachability boundary remains closed

FO authorizes no:

- candidate query/selection;
- reachability evaluation;
- endpoint resolution;
- relay/direct-path selection;
- target QUIC/TCP establishment;
- port-forward activation;
- terminal activation;
- remote-session establishment;
- rendezvous completion claim.

A successfully sent requester `Accepted` acknowledgement remains accepted-for-continuation only.

## 36. Production/runtime boundary remains closed

FO does not:

- add the shared requester-authority wrapper in Rust;
- modify FL/FN signatures;
- create a session-owner custody cell;
- spawn an FL task;
- replace capability-only persistent collection behavior;
- wire real admissions to FL;
- change endpoint lifecycle;
- change listener/admission behavior;
- publish readiness;
- alter process lifecycle or `main.rs`;
- change Android source;
- change dependencies/workflows;
- package;
- deploy;
- restart/recover;
- merge.

## 37. Source-materialization sequencing

FO selects a staged source sequence rather than one broad persistent-runtime mutation.

The first source checkpoint after FO should materialize only the shared synchronized requester/rendezvous authority and adapt the existing FB/FL/FN borrowed path to use that shared authority while remaining non-spawned and uninvoked by production persistent collection.

Only after that synchronized borrowed path validates should a later checkpoint materialize the supervisor-retained recoverable authenticated-session owner cell and owned/spawned FL task custody.

Persistent collection substitution/integration remains later still.

## 38. First next source seam

The next checkpoint should conservatively be:

**C03e-FP — shared requester/rendezvous authority synchronization and borrowed FL adaptation source materialization.**

FP should:

- materialize one Agent-owned cloneable async-mutex requester-authority wrapper around exact existing runtime owner;
- expose no raw provider/mutex guard;
- preserve one provider state;
- preserve exact DI -> DP -> DK -> DN coherent composition under the selected requester-lock -> current-authority-read order;
- release requester lock before FH response I/O;
- adapt FB/FL/FN borrowed invocation to the shared authority handle;
- keep FN non-spawned;
- keep persistent collection unchanged;
- preserve exact FL stop/cancellation law;
- add no peer disposition, candidate/reachability work, listener activation, deployment, restart/recovery, or merge.

## 39. Explicit non-goals

No Rust source mutation.
No Android mutation.
No dependency/workflow mutation.
No provider clone.
No per-worker requester-authority snapshot.
No raw mutex/provider exposure.
No direct persistent FL integration.
No spawned FL task.
No task abort.
No implicit owner drop policy.
No code-3/code-4 widening.
No new mixed-family close code.
No peer close.
No peer restart/reuse.
No requester record cleanup.
No candidate/reachability/endpoint/relay selection.
No target dialing.
No port-forward/terminal/session activation.
No listener/process activation.
No packaging.
No deployment.
No restart/recovery.
No merge.

## 40. Canonical closure target

`CLOSED_REQUESTER_RENDEZVOUS_AUTHORITY_SYNCHRONIZATION_OWNED_PERSISTENT_FL_CUSTODY_SELECTION`

## 41. Canonical gate target

`C03E_FO_REQUESTER_RENDEZVOUS_AUTHORITY_SYNCHRONIZATION_OWNED_PERSISTENT_FL_CUSTODY_SELECTED`

## 42. Next separately gated checkpoint

**C03e-FP — shared requester/rendezvous authority synchronization and borrowed FL adaptation source materialization**.

Recoverable spawned-session custody, persistent multi-worker FL integration, peer close/reuse policy, requester-record cleanup, candidate/reachability continuation, target dialing, deployment, restart/recovery, and merge remain later gates.
