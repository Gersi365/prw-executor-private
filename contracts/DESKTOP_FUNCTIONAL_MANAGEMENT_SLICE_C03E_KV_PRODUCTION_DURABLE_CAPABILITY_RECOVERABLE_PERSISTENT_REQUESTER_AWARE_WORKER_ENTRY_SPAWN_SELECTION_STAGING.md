# Desktop Functional Management Slice C03e-KV — Production Durable Capability Recoverable Persistent Requester-Aware Worker Entry Spawn Selection Staging

Status: `SELECTION_STAGING`

Gate:

`C03E_KV_PRODUCTION_DURABLE_CAPABILITY_RECOVERABLE_PERSISTENT_REQUESTER_AWARE_WORKER_ENTRY_SPAWN_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_RECOVERABLE_PERSISTENT_REQUESTER_AWARE_WORKER_ENTRY_SPAWN_SELECTION`

## 1. Purpose

C03e-KV is a docs-only exact-source selection checkpoint after the evidence-closed C03e-KU FQ task-ownership source materialization.

KV performs a fresh exact-head audit of the existing FU repeated real-admission persistent integration and FS recoverable persistent custody before selecting any durable-authority propagation.

The audit finds that FU does not call either bounded FQ driver. FU instead creates one persistent worker entry directly, retains its `JoinHandle` in the active map through FS custody, and reaps or drains that entry later.

Therefore the next smallest correct source seam is not yet the full repeated-admission collection and is not a call to the KU bounded FQ driver.

KV selects exactly one future additive dormant FU helper that constructs one production-durable recoverable persistent requester-aware worker entry while preserving the existing FS custody contract.

No Rust source is changed by KV.

## 2. Exact predecessor authority

Predecessor checkpoint:

`C03e-KU — Production durable capability recoverable spawned requester-aware worker task-ownership source materialization`

Exact KU branch:

`phase-152-c03e-ku-production-durable-capability-recoverable-spawned-requester-aware-worker-task-ownership-source-materialization`

Exact KU head / required merge base:

`b7526919dc77d698cb9cb4487c8e9c2cf694b52f`

Exact KU tree:

`607bbe50fa80563f9338efaafc5bfb482da9bbd6`

Exact KU FQ blob:

`cb1f11d97a42a1ab754230696d2dbcc0860c128e`

KU PR #432 remains draft, open, unmerged and evidence-closed.

## 3. Exact FU source authority

FU path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact KU FU blob:

`2a07f03bb3c1739e4963a16c0ba7c30ae753d24e`

FU currently owns:

- repeated expected-device request scheduling;
- duplicate-active-device rejection;
- one in-flight AJ admission transaction;
- authenticated `DeviceId` active-map keying;
- construction of one `RemoteSessionWorkerAdmission<D, T>`;
- direct spawning of one persistent requester-aware task through `spawn_recoverable_requester_aware_worker(...)`;
- insertion of one `RecoverableRequesterAwareWorkerEntry` into the active map;
- ready completion reaping;
- cooperative cancellation and drain on orderly supervisor shutdown.

FU currently passes `SharedCurrentCapabilityAuthority<P>` both to admission and to the legacy requester-aware worker path.

## 4. Exact FS persistent-custody authority

FS path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`

Exact KU FS blob:

`264d18d57aafe9c6f67683843ded656e40d2d8cb`

FS already provides the required generic custody primitives:

- `RecoverablePersistentWorkerEntry<O, T>`;
- `RecoverableRequesterAwareWorkerEntry` specialization;
- supervisor-retained owner-cell custody;
- cancellation controller custody;
- retained `JoinHandle` custody;
- ready completion owner recovery;
- cooperative cancellation request;
- exact drain semantics.

No FS mutation is selected.

## 5. Exact durable-authority authority

Path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact KU blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

`ProductionDurableCapabilityAuthority` remains:

- public inside the Agent crate;
- `Send + Sync`;
- non-Clone;
- privately backed by the durable-registry custody;
- coupled to the concrete production deny-all capability policy baseline.

KV selects no mutation of this type.

## 6. Exact KS FI durable worker authority

FI path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact inherited KU FI blob:

`a8cb82f4eda44a207ba889bacd60c3f24c1901e7`

Existing durable FI worker:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

This remains the exact requester-aware serial lifecycle that the future KV source seam must execute inside the persistent worker task.

No FI mutation is selected.

## 7. Fresh KU/FU composition finding

KU materialized one bounded FQ method:

`drive_recoverable_spawned_requester_rendezvous_worker_with_production_durable_capability(...)`

That method:

1. creates a recoverable owner cell;
2. spawns one task;
3. runs the KS FI durable worker;
4. joins the task inside `self.runtime.block_on(...)`;
5. recovers the authenticated-session owner;
6. returns `RecoverableSpawnedRequesterRendezvousWorkerCompletion` only after the worker is terminal.

FU requires a different lifecycle shape.

FU must return immediately from worker construction with one active persistent entry containing:

- supervisor owner-cell custody;
- cooperative cancellation controller;
- still-live `JoinHandle`.

That entry remains in the active map while repeated admission continues.

Therefore calling the KU bounded FQ driver from FU's persistent spawn boundary would collapse the persistent lifetime into a blocking join and would not produce the required FS entry.

KV explicitly rejects that composition.

## 8. Selected future source seam

A later source-materialization checkpoint may add exactly one private dormant helper inside the FU file:

`spawn_recoverable_requester_aware_worker_with_production_durable_capability(...)`

Selected signature shape:

```rust
fn spawn_recoverable_requester_aware_worker_with_production_durable_capability<P, D, T, S>(
    admission: RemoteSessionWorkerAdmission<D, T>,
    capability_authority: Arc<ProductionDurableCapabilityAuthority>,
    requester_dr_authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &Arc<S>,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
) -> RecoverableRequesterAwareWorkerEntry
where
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> u64 + Send + 'static,
    S: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
```

The exact function name may be rustfmt-formatted normally but must retain the full semantic distinction from the legacy spawn helper.

## 9. Durable authority ownership law

The future helper accepts one:

`Arc<ProductionDurableCapabilityAuthority>`

by value.

The helper does not clone the authority type itself.

For one persistent entry, the exact caller-supplied outer Arc moves directly into the spawned task.

Inside the task the KS FI worker receives only:

`capability_authority.as_ref()`

or an equivalent ordinary borrow.

The outer Arc means higher-owner shared custody only. It does not create a second authorization domain or clone durable-registry semantic state.

## 10. Requester DR authority law

The future helper receives:

`&SharedCurrentCapabilityAuthority<P>`

as `requester_dr_authority`.

It may clone only that already-cloneable handle for task ownership, exactly as the existing legacy FU helper does today.

Inside the task that authority reaches only the requester-DR parameter of the KS FI durable worker.

It must not become fallback capability authority.

KV preserves:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

## 11. Requester/rendezvous authority law

The existing:

`SharedRequesterRendezvousAuthority`

may be cloned exactly once for worker-task custody using its already-selected semantics.

It remains requester/rendezvous authority only.

It is not a capability authority, transport identity source, candidate-publication authority or logical-session identity source.

## 12. Policy-source law

The existing requester-aware policy source remains:

`Arc<S>`

The helper borrows `&Arc<S>` from its caller and clones that Arc for the worker task exactly as the legacy FU helper already does.

No new policy cache, global policy source or policy conversion is selected.

## 13. Worker-admission consumption law

The helper consumes one exact:

`RemoteSessionWorkerAdmission<D, T>`

by value.

It calls the existing `into_parts()` exactly once to recover:

- the exact authenticated-session owner;
- dispatcher;
- verifier-time provider.

It does not authenticate another session, sample admission timing, alter logical identity or perform another transport accept.

## 14. Owner-cell law

The helper creates the same existing-shape owner cell as the legacy FU helper:

`Arc<Mutex<Option<AuthenticatedRemoteSessionRuntimeOwner>>>`

The supervisor side remains stored in `RecoverablePersistentWorkerEntry`.

The task receives one cloned owner-cell handle.

The task locks and mutably borrows the exact retained owner.

The task must never `take()` the owner.

Normal return or panic must release the guard so FS can recover exact owner custody after terminal join.

## 15. Cancellation law

The helper creates exactly one existing:

`remote_session_worker_cancellation_pair()`

The cancellation controller remains in the FS persistent entry.

The cancellation signal moves into the task and is converted exactly once through:

`cancellation_signal.into_cancelled()`

The resulting future moves into the KS FI durable worker.

No additional cancellation race, abort handle or detached cancellation task is selected.

## 16. Spawn law

The helper creates exactly one:

`tokio::spawn(async move { ... })`

The task owns:

- worker-side owner-cell handle;
- outer durable-authority Arc;
- cloned requester-DR authority handle;
- cloned requester/rendezvous authority handle;
- cloned requester policy-source Arc;
- dispatcher;
- verifier-time provider;
- cancellation future.

No second task, queue, channel, actor, global map or background supervisor is selected.

## 17. Exact KS FI invocation law

The task must invoke exactly:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

with:

1. mutable borrowed authenticated-session owner;
2. borrowed durable capability authority;
3. borrowed requester-DR shared-current authority;
4. requester policy source;
5. requester/rendezvous authority;
6. exact verifier-time provider;
7. mutable dispatcher;
8. exact cancellation future.

No durable authorization/dispatch/response logic is duplicated inside FU.

## 18. Returned persistent-entry law

The helper returns exactly:

`RecoverableRequesterAwareWorkerEntry`

constructed through the existing:

`RecoverablePersistentWorkerEntry::new(owner_cell, cancellation_controller, worker_handle)`

No new entry type, completion type or error family is selected.

FS remains the authority for ready reaping, owner recovery, cancellation request and drain.

## 19. Legacy FU helper preservation

Existing helper:

`spawn_recoverable_requester_aware_worker(...)`

must remain unchanged and available.

The new durable helper is additive.

No existing caller is redirected in the same source-materialization checkpoint.

The existing repeated real-admission collection therefore remains legacy/shared-current after the future source-materialization checkpoint.

## 20. Repeated-admission collection remains separately gated

Existing method:

`drive_recoverable_repeated_real_remote_admission_collection(...)`

must remain unchanged and must not call the new durable helper in the immediate source successor.

A later separately selected checkpoint may choose a durable repeated-admission integration surface that:

- receives one higher-owner outer `Arc<ProductionDurableCapabilityAuthority>`;
- retains that Arc across the repeated supervisor lifetime;
- clones only the outer Arc once per newly admitted durable worker;
- preserves the shared-current authority for AJ admission and requester DR;
- invokes the new durable persistent-entry helper on each successful admission.

No such caller migration is selected by KV.

## 21. KU bounded FQ preservation

KU FQ path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact KU blob:

`cb1f11d97a42a1ab754230696d2dbcc0860c128e`

KV selects no FQ mutation.

The KU bounded FQ durable method remains dormant and separately useful for a lifecycle in which the caller intentionally waits for terminal worker completion before recovering owner custody.

FU persistent-entry construction must not call it.

## 22. FS preservation

Exact FS blob:

`264d18d57aafe9c6f67683843ded656e40d2d8cb`

No FS mutation is selected.

No new persistent entry/completion type is needed.

## 23. FI preservation

Exact FI blob:

`a8cb82f4eda44a207ba889bacd60c3f24c1901e7`

No FI mutation is selected.

## 24. Durable-authority custody preservation

Exact durable-authority custody blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

No `Clone`, getter, extraction seam, visibility expansion, policy change or ownership wrapper is selected in that file.

## 25. Admission semantics preservation

The future helper performs no AJ admission.

Existing FU admission continues to use:

`admit_expected_remote_device_session(...)`

with the existing `SharedCurrentCapabilityAuthority<P>` lane.

The helper receives only a post-admission `RemoteSessionWorkerAdmission` and therefore cannot alter:

- expected-device preflight;
- session authentication;
- authenticated `DeviceId` derivation;
- admission timing;
- duplicate-device admission rules;
- in-flight shutdown-vs-admission ordering.

## 26. Identity invariants

Logical identity remains the authenticated PRW application-session device identity.

Static IP is never identity.

Transport identity is evidence only.

Outer PRWM `request_id` remains correlation only.

Durable capability authority custody does not become logical identity or requester identity.

## 27. Candidate-publication behavior

Candidate-publication ingress remains separately gated and fail-closed through the existing lower durable typed-ingress chain.

The future FU helper adds no candidate provider, candidate response, reachability continuation, retry or dialing.

## 28. Production policy boundary

The concrete production capability policy remains deny-all.

KV does not select any positive capability grant.

Task creation or successful ingress does not imply authorization.

## 29. Exact future source ceiling

The next source-materialization checkpoint may change at most:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

No second source path is selected.

The following must remain byte-identical:

- KU FQ source;
- KS FI source;
- FS persistent-custody source;
- durable-authority custody source;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- `linux_bootstrap.rs`;
- `main.rs`;
- Cargo manifests and lockfile;
- workflows;
- Android source.

If compilation requires another source path, STOP and open a new gate rather than widening scope.

## 30. Future source proof obligations

The next source checkpoint must prove at minimum:

1. exactly one FU source path changed;
2. final net diff is additive-only unless rustfmt requires local formatting;
3. existing legacy FU helper remains behaviorally unchanged;
4. existing repeated real-admission method remains unchanged and does not invoke the new helper;
5. KU FQ remains byte-identical;
6. FS remains byte-identical;
7. FI remains byte-identical;
8. durable-authority custody remains byte-identical;
9. no `Clone` is added to `ProductionDurableCapabilityAuthority`;
10. new helper accepts one owned outer durable-authority Arc;
11. outer Arc moves into exactly one persistent worker task;
12. task passes only a borrow of durable authority to KS FI;
13. requester-DR authority remains shared-current;
14. no shared-current fallback exists on durable capability path;
15. exact owner-cell recovery shape is retained;
16. existing cancellation pair is retained;
17. exact `RecoverableRequesterAwareWorkerEntry` is returned;
18. no repeated-admission caller migration occurs;
19. no extra task/channel/queue/global authority state is created;
20. no peer close/reuse/restart/reconnect or runtime activation is added.

## 31. Explicit exclusions

KV does not perform or authorize:

- FU Rust source materialization in KV;
- repeated-admission collection durable caller migration;
- executable/runtime durable-authority population;
- construction location for the first production outer durable-authority Arc;
- `Clone` on `ProductionDurableCapabilityAuthority`;
- private inner Arc exposure;
- authority aggregate/context creation;
- requester DR authority replacement;
- AJ admission authority replacement;
- KU FQ mutation;
- FS mutation;
- FI mutation;
- lower KQ/KO/KM/KK/KG mutation;
- positive production capability grants;
- candidate provider execution;
- reachability/dialing;
- peer reuse/restart/reconnect;
- extra tasks/channels/queues;
- listener/readiness/runtime/network activation;
- `run()` or `main.rs` mutation;
- manifest/lockfile/workflow/Android-source mutation;
- deployment/restart/recovery;
- database/schema/control-plane mutation;
- repository configuration/visibility mutation;
- merge;
- PR close;
- ready-for-review conversion;
- branch deletion;
- history rewrite.

## 32. STOP boundary

STOP after KV selection closure.

Do not materialize the selected FU durable persistent-entry helper in KV.

Do not migrate the repeated real-admission collection to durable capability authority.

Do not populate executable/runtime durable authority inputs or activate production networking.

Any source materialization requires a separately validated successor checkpoint based on the exact final KV head.
