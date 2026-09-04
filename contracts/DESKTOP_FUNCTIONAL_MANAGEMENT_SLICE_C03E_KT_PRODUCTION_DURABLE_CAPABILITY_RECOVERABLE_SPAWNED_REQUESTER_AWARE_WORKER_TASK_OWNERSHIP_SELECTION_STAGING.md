# C03e-KT Production Durable Capability Recoverable Spawned Requester-Aware Worker Task-Ownership Selection

Status: `SELECTION_STAGING`

Gate:

`C03E_KT_PRODUCTION_DURABLE_CAPABILITY_RECOVERABLE_SPAWNED_REQUESTER_AWARE_WORKER_TASK_OWNERSHIP_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_RECOVERABLE_SPAWNED_REQUESTER_AWARE_WORKER_TASK_OWNERSHIP_SELECTION`

## 1. Purpose

C03e-KT selects one future additive dormant FQ spawned-worker seam that moves an explicitly shared production durable capability-authority handle into the existing bounded recoverable requester-aware task while preserving requester/rendezvous authority as a distinct lane.

KT is selection-only. It does not materialize the selected FQ seam, does not modify FU repeated real-admission integration, does not add `Clone` to `ProductionDurableCapabilityAuthority`, does not create an authority aggregate, does not populate production executable authority inputs, does not activate runtime/network behavior, and does not merge/deploy/restart/recover anything.

## 2. Exact predecessor authority

Predecessor checkpoint:

`C03e-KS — Production durable capability requester-aware serial lifecycle dual-authority source materialization`

Exact KS branch:

`phase-152-c03e-ks-production-durable-capability-requester-aware-serial-lifecycle-dual-authority-source-materialization`

Exact KS head / required merge base:

`c51a12247dfed0c153ec70ed4b1482f8ab98845b`

Exact KS tree:

`fdbc04716085434cbbab065fa9aa7833bf006715`

Exact KS FI source path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact KS FI blob:

`a8cb82f4eda44a207ba889bacd60c3f24c1901e7`

KS PR #430 remains draft/open/unmerged and evidence-closed.

## 3. Fresh FQ spawned-task finding

Exact FQ path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact KS FQ blob:

`bc4520b2c13308b446230b43a2650d02e5b42cc2`

The existing FQ method:

`RemoteSessionExecutorRuntime::drive_recoverable_spawned_requester_rendezvous_worker(...)`

creates one `tokio::spawn(async move { ... })` task and therefore requires owned `'static` task inputs.

Before spawning it already converts borrowed cloneable shared handles into task-owned values:

- `SharedCurrentCapabilityAuthority<P>` is cloned into the task;
- `SharedRequesterRendezvousAuthority` is cloned into the task;
- requester policy source is already an `Arc<S>`;
- dispatcher, verifier-time provider and caller cancellation move by value;
- the authenticated-session owner remains recoverable through the existing supervisor-retained `Arc<Mutex<Option<_>>>` custody cell.

FQ then invokes the existing legacy FI cancellation-aware worker from inside the spawned task.

## 4. Fresh durable-authority custody finding

Exact durable-authority source path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact KS blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

`ProductionDurableCapabilityAuthority` is public in the Agent crate and its exact KS tests establish that it is `Send + Sync`.

Its private durable-registry custody already contains an internal:

`Arc<Mutex<ProductionDurableRegistryRuntimeCustody>>`

but the outer authority type itself is not `Clone`.

KT does not infer that authority identity should be cloneable merely because its private internal custody uses `Arc`.

## 5. No authority-type Clone is required

FQ already imports and uses `std::sync::Arc` for task ownership.

A future FQ durable seam can accept one caller-owned:

`Arc<ProductionDurableCapabilityAuthority>`

by value and move that exact shared handle into the `'static` spawned task.

Inside the task, ordinary dereference borrowing can supply:

`&ProductionDurableCapabilityAuthority`

to the existing KS FI durable worker.

Therefore KT selects external shared task custody around the authority and explicitly does **not** select:

- `impl Clone for ProductionDurableCapabilityAuthority`;
- `#[derive(Clone)]` on the authority;
- exposure of the authority's private inner `Arc`;
- a generic registry-custody getter;
- a second authority wrapper/newtype;
- a global authority singleton;
- a dynamic authority map;
- an authority aggregate/context type.

## 6. Selected future FQ seam

A later source checkpoint may add exactly one dormant Agent-internal method on:

`RemoteSessionExecutorRuntime`

Selected name:

`drive_recoverable_spawned_requester_rendezvous_worker_with_production_durable_capability(...)`

Selected generic families remain aligned with existing FQ:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> u64 + Send + 'static`;
- `S: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static`;
- `C: Future<Output = ()> + Send + 'static`.

Selected argument responsibilities:

- `&mut self` retains existing executor/runtime ownership;
- one `AuthenticatedRemoteSessionRuntimeOwner` by value retains exact peer custody semantics;
- one `Arc<ProductionDurableCapabilityAuthority>` by value supplies durable capability authority to the spawned task;
- one borrowed `&SharedCurrentCapabilityAuthority<P>` remains requester-DR current authority and is cloned using its existing selected clone semantics;
- one `Arc<S>` remains requester-aware policy-source custody;
- one borrowed `&SharedRequesterRendezvousAuthority` remains requester/rendezvous authority and is cloned using its existing selected semantics;
- verifier-time provider, dispatcher and cancellation continue moving by value.

Selected return remains exactly:

`RecoverableSpawnedRequesterRendezvousWorkerCompletion`

No new completion type is selected.

## 7. Exact spawned-task composition

The future FQ durable seam must preserve the existing FQ recoverable task-ownership law:

1. place the exact authenticated-session owner into one existing-shape `Arc<Mutex<Option<_>>>` recoverable cell;
2. retain one cell handle outside the task;
3. move one cell handle into the task;
4. move the exact supplied `Arc<ProductionDurableCapabilityAuthority>` into the task;
5. clone existing shared-current requester-DR authority before spawn using its already-selected clone semantics;
6. clone existing shared requester/rendezvous authority before spawn using its already-selected clone semantics;
7. move policy source, verifier-time provider, dispatcher and cancellation into the task exactly once;
8. inside the task, lock the owner cell and borrow the exact authenticated-session owner mutably;
9. invoke exactly the existing KS FI durable dual-authority worker;
10. pass durable capability authority only as a borrow from the task-owned `Arc`;
11. pass shared-current authority only as requester-DR authority;
12. await one exact KS worker stop;
13. release the owner-cell guard;
14. join the spawned task through the existing bounded join mapping;
15. recover the exact authenticated-session owner by value from the supervisor-retained cell;
16. return the existing completion shape with exact KS stop or existing abnormal-task classification.

## 8. Exact KS invocation

The selected FQ seam must call only:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

for requester-aware worker execution.

It must supply:

- `capability_authority.as_ref()` or an equivalent ordinary borrow of the exact task-owned `Arc<ProductionDurableCapabilityAuthority>`;
- the existing cloned shared-current authority only in the requester-DR parameter;
- the existing policy source;
- the existing shared requester/rendezvous authority;
- the exact verifier-time provider;
- the exact mutable dispatcher;
- the exact caller cancellation future.

The new FQ seam must not call the legacy FI worker and then separately invoke durable capability processing.

## 9. Two authority lanes remain non-substitutable

### Lane A — durable capability ingress

`Arc<ProductionDurableCapabilityAuthority>` is task custody only.

Its borrowed inner authority may flow only into the KS durable capability-ingress lane.

It must not be used for:

- requester DR;
- requester policy evaluation;
- requester registration;
- candidate-publication authority;
- transport identity derivation;
- task identity;
- worker admission identity.

### Lane B — requester DR current authority

`SharedCurrentCapabilityAuthority<P>` remains the requester/admission authority used by existing requester DR semantics.

It must not be used as a fallback capability authority inside the new durable path.

KT preserves the established distinction:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

## 10. Authority sharing semantics

The selected outer `Arc<ProductionDurableCapabilityAuthority>` means only that multiple separately gated higher owners may eventually retain shared ownership of the same authority object.

It does not imply:

- cloning durable-registry semantic state;
- cloning authenticated session identity;
- creating an independent authorization domain;
- positive capability grants;
- registry mutation rights;
- policy widening;
- task-local authority reconstruction.

The inner deny-all production policy remains unchanged.

## 11. Session-owner recovery invariant

The future FQ durable seam must reuse the existing recoverable owner-cell pattern.

The spawned task may borrow the authenticated-session owner mutably while executing KS, but it must never `take()` the owner from the cell.

After normal KS stop, panic or abnormal join classification, the supervisor must recover the exact owner by value through the existing `join_and_recover_owned_value(...)` pattern.

No detached owner, replacement peer, second session owner or automatic worker restart is selected.

## 12. Cancellation invariant

FQ introduces no new cancellation race.

The caller cancellation future moves by value into the spawned task and then into KS exactly once.

KS remains responsible for its already-selected lifecycle law:

- KQ owns the pre-handoff ingress/cancellation race;
- cancellation is not polled during requester DR/terminal response;
- cancellation is checked after a successful terminal response before another durable ingress cycle.

FQ only preserves task ownership and terminal join/recovery around that exact behavior.

## 13. Verifier-time invariant

FQ does not sample verifier time.

The verifier-time provider moves by value into the task and is transferred into KS.

KS/KQ/KO remain the existing sampling chain, with KO sampling immediately before each KM durable transaction.

No task-start timestamp, spawn timestamp or join timestamp may become capability verifier time.

## 14. Error and stop preservation

The future FQ durable seam introduces no new error or stop enum.

Normal task completion returns the exact existing:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

from KS.

Abnormal task completion remains represented only through the existing:

`RemoteSessionSpawnedWorkerJoinError`

and the existing completion result shape.

No KS ingress/requester-response failure is flattened, retried, replaced or converted into cancellation.

## 15. Existing peer-disposition behavior remains out of the seam

KT does not alter the existing post-completion peer-disposition classifier/consumer.

The future FQ durable seam returns recoverable completion custody only.

It does not:

- close the peer during normal task execution;
- invoke orderly-shutdown disposition itself;
- invoke requester-aware terminal-failure disposition itself;
- restart/reuse the peer;
- clean requester records;
- select reachability/candidates;
- dial targets.

## 16. FQ legacy preservation

The later source materialization must leave the existing:

`drive_recoverable_spawned_requester_rendezvous_worker(...)`

unchanged and available on the legacy shared-current ingress path.

The new durable FQ method is additive and dormant.

Existing completion types, join helper, owner-cell alias, disposition classifier and tests remain unchanged unless rustfmt-only formatting of newly inserted code requires local formatting around the insertion boundary.

No existing caller is redirected in the same checkpoint.

## 17. KS FI preservation

Exact KS FI path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact KS blob:

`a8cb82f4eda44a207ba889bacd60c3f24c1901e7`

KT selects no FI mutation.

The durable FI worker remains the exact requester-aware lifecycle authority called by the future FQ seam.

## 18. FU preservation and later dependency

Exact FU path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact KS FU blob:

`2a07f03bb3c1739e4963a16c0ba7c30ae753d24e`

KT selects no FU mutation.

FU currently owns repeated real-admission integration and may spawn multiple requester-aware device workers. A future separately selected FU successor may need to receive one shared `Arc<ProductionDurableCapabilityAuthority>` and clone only that outer Arc per admitted worker before invoking the future FQ durable seam.

That FU propagation is explicitly **not** selected here.

## 19. Durable authority custody preservation

Exact path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact KS blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

The later FQ materialization must leave this file byte-identical.

No `Clone`, getter, wrapper, constructor change, policy change or registry-custody visibility expansion is selected.

## 20. Production executable population remains later

KT does not select where the first production `Arc<ProductionDurableCapabilityAuthority>` is constructed or retained at executable/runtime aggregate level.

The existing authority constructor still consumes one `ProductionDurableRegistryRuntimeCustody` by value and returns one authority object.

A future separately gated higher-owner checkpoint may decide where that object is wrapped in one outer `Arc` and how the shared handle enters FU/runtime custody.

No executable population or startup wiring is selected in KT.

## 21. Candidate-publication behavior

Candidate publication remains fail-closed through the existing KS/KQ mixed-family chain.

FQ task ownership adds no candidate authority, provider, response, retry, reachability continuation or target dialing.

## 22. Exact source-successor ceiling

The later KT source materialization may change at most one source path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

No second source path is selected.

The following must remain unchanged unless a new gate is opened:

- KS FI source path;
- FU repeated real-admission requester-aware integration;
- durable authority custody source;
- KQ durable ingress worker source;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests/lockfile;
- workflows;
- Android source.

If correct compilation requires a second path, an authority-type `Clone`, an authority aggregate, FU mutation, production executable population or runtime activation, STOP and open a separate gate.

## 23. Source-successor proof obligations

A later source checkpoint must prove at minimum:

1. exactly one FQ source path changed;
2. final net diff is additive with no legacy FQ behavior change;
3. KS FI blob remains byte-identical;
4. FU blob remains byte-identical;
5. durable authority custody blob remains byte-identical;
6. no `Clone` is added to `ProductionDurableCapabilityAuthority`;
7. new FQ method accepts one owned `Arc<ProductionDurableCapabilityAuthority>`;
8. that Arc is moved into exactly one bounded spawned task;
9. the task supplies only a borrow of the durable authority to KS;
10. shared-current authority remains requester-DR authority;
11. no capability fallback to shared-current authority exists;
12. existing recoverable authenticated-session owner-cell law remains unchanged;
13. caller cancellation is transferred only into KS;
14. FQ samples no verifier time;
15. no new task beyond the one existing-shape bounded FQ worker task is created;
16. no channel/queue/global authority state is created;
17. no peer close/restart/reuse is added to the new seam;
18. new FQ durable method remains dormant/uninvoked by FU.

## 24. Selection validation scope

KT itself is documentation-only.

Exact-final-head CI for the final KT head is the only validation authority. Path-filtered skipped workflows are not PASS, and no Android PASS may be claimed if the docs-only branch does not trigger Android validation.

## 25. Explicit exclusions

KT does not perform or authorize:

- FQ source materialization;
- FU durable-authority propagation;
- production executable/runtime aggregate population;
- `Clone` on `ProductionDurableCapabilityAuthority`;
- inner durable-registry Arc exposure;
- generic custody getters;
- authority aggregate/context creation;
- requester DR authority replacement;
- KS/KQ/KO/KM/KK/KG mutation;
- positive production capability grants;
- candidate provider execution;
- peer-close policy changes;
- extra tasks/channels/queues;
- listener/bind/readiness/runtime/network activation;
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

## 26. STOP boundary

STOP after KT selection closure.

Do not materialize the selected FQ durable task-ownership seam in KT.

Do not propagate durable authority into FU, construct executable production authority handles, activate runtime/network behavior, merge, deploy, restart/recover or mutate repository configuration without a separately selected successor gate.
