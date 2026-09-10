# Desktop Functional Management Slice C03e-OL

## Production requester/rendezvous expected-device admission cooperative scheduling producer endpoint-lifecycle adapter source-seam selection

Status: `SELECTION — VALIDATION_PENDING`
Date: 2026-09-10

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_ENDPOINT_LIFECYCLE_ADAPTER_SOURCE_SEAM_SELECTION`

Authoritative predecessor: exact implementation-validated C03e-OK head `e5d063996462c3417705e0c529c1ac06535e9cb2`, tree `31906a48961042d9dd7ab363dfbaff0c18feadb2`.

This checkpoint changes documentation only. It audits the exact post-OK source graph and selects the smallest next independently materializable source seam needed before the higher endpoint owner can borrow and forward the eventual concrete production handoff producer. It does not change Rust, define a concrete receipt family, create a channel/sender, construct or send an expected-device admission request, migrate a caller, or activate runtime behavior.

## 1. Decision

`EXECUTOR_ENDPOINT_LIFECYCLE_ADAPTER_REQUIRED / SAME_OK_SOURCE_FILE / GENERIC_RECEIPT_PRESERVED / BORROWED_PRODUCER_FORWARDING_PRESERVED / EXISTING_ENDPOINT_CLOSE_AND_IDLE_DRAIN_REUSED / HIGHER_ENDPOINT_OWNER_AND_CONCRETE_RECEIPT_DEFERRED`

C03e-OL selects the next separately gated source boundary as:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_ENDPOINT_LIFECYCLE_ADAPTER_SOURCE_MATERIALIZATION`

The immediate future source ceiling is exactly one existing Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

The future source stage may add only one dormant parallel executor endpoint-lifecycle adapter around the exact C03e-OK cooperative producer collection driver, plus narrowly required same-file lint acknowledgement and focused same-file compile/shape tests if necessary.

If correctness or compilation requires a second Rust path, visibility widening outside this selected adapter, a concrete receipt type, or any higher-owner mutation, STOP and return to selection.

## 2. Fresh concurrency and exact-head audit

Immediately before assigning C03e-OL:
- exact C03e-OK remained at head `e5d063996462c3417705e0c529c1ac06535e9cb2`, tree `31906a48961042d9dd7ab363dfbaff0c18feadb2`;
- exact final C03e-OK authorized source blob remained `3598d941517457d97ba884817fc306cd20fd343c`;
- no `phase-152-c03e-ol...` branch existed;
- no C03e-OL pull request existed;
- the proposed C03e-OL contract path returned 404 on exact C03e-OK;
- the live `phase-152-c03e-o...` namespace ended at C03e-OK;
- recent PR chronology ended at draft/open/unmerged PR #525.

The C03e-OL branch is therefore created directly from exact C03e-OK without retargeting, rebasing, merge, force movement, or successor overwrite.

## 3. Exact post-OK source observations

### 3.1 C03e-OK lower cooperative producer driver is materially present

Exact C03e-OK materialized in the selected collection file:

`drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling_producer(...)`

Its relevant selected interface remains generic:
- caller-owned lending producer: `producer: &mut H`;
- `H: AsyncFnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> Receipt`;
- synchronous shutdown suppression mapper `Q` returns the same generic `Receipt`;
- synchronous observer `O: FnMut(Receipt)` receives each terminal receipt;
- no concrete `HandoffReceipt` is named by the lower driver;
- the producer call future remains lexical and is never spawned or detached.

The driver also preserves the OJ/OF cooperative progress law, including explicit shutdown priority, receiver/admission progress while one producer future is pending, no second producer future, and existing scheduling peer disposition before normal producer delivery.

### 3.2 Historical executor endpoint lifecycle owns close and idle drain

The same exact source file already contains the existing scheduling-aware executor endpoint method:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling(...)`

That method calls the historical scheduling collection, then performs:
- `transport_runtime.close(0, b"remote endpoint shutdown")`;
- `self.runtime.block_on(transport_runtime.wait_idle())`;
- returns the original collection result.

Endpoint close and idle drain therefore already have an exact established lifecycle location. The future cooperative producer endpoint adapter must reuse that law rather than moving close/wait-idle into the higher owner or duplicating shutdown authority.

### 3.3 The higher endpoint owner cannot directly call the C03e-OK lower driver

The C03e-OK cooperative collection driver is declared with visibility restricted to the `remote_session_executor_runtime` subtree.

Exact `remote_session_capability_runtime.rs` declares:
- `remote_session_endpoint_lifecycle_runtime`;
- `remote_session_executor_runtime`;

as sibling modules.

Therefore the higher endpoint owner cannot directly call the C03e-OK collection driver without either:
- widening that lower method beyond its current executor-private boundary; or
- adding an executor endpoint adapter with the same visibility pattern already used by the existing scheduling endpoint method.

C03e-OL rejects ad-hoc visibility widening and selects the second option.

## 4. Selected future executor endpoint adapter

The future one-file source stage may add one dormant parallel method on `RemoteSessionExecutorRuntime`, conceptually:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer(...)`

The exact name may follow local naming constraints, but the semantic boundary is fixed.

It must:
1. accept the same non-producer lifecycle authorities and inputs as the existing scheduling-aware endpoint method;
2. additionally accept the exact borrowed producer `&mut H` selected by OF/OK;
3. additionally accept the synchronous shutdown suppression mapper `Q` and synchronous receipt observer `O` selected by OJ/OK;
4. keep the receipt type fully generic;
5. invoke the exact C03e-OK cooperative producer collection driver exactly once;
6. after that invocation returns, perform the same endpoint close and `wait_idle` sequence as the historical scheduling endpoint lifecycle;
7. return the original `Result<(), RemoteSessionPersistentCollectionConfigError>` unchanged.

The adapter may use the same visibility scope as the existing executor endpoint lifecycle surface that is callable by `remote_session_endpoint_lifecycle_runtime`; it must not widen the lower cooperative collection driver itself.

## 5. Selected ownership and receipt law

This stage remains below the concrete production composition.

The executor endpoint adapter is generic over `Receipt`. It must not define, construct, export, project, or pattern-match a concrete `HandoffReceipt`.

The future higher owner remains responsible for the separately gated concrete receipt interface and production producer composition.

The adapter must preserve:
- exact requester-side completion `DeviceId` as correlation identity;
- exact disposed scheduling terminal/join result by value;
- target expected `DeviceId` only from a later consumed eligible scheduling grant;
- requester scheduling `SessionId` distinct from future target admission `SessionId`;
- one-shot non-Copy/non-Clone scheduling grant custody;
- no grant clone/copy/reconstruction/remint/replay;
- no scheduling-consumption rollback;
- request IDs as correlation only unless their exact producer lane separately authorizes construction.

## 6. Producer borrowing law

The adapter receives only a temporary mutable borrow of the caller-owned lending producer.

It must not:
- own or clone the future sole sender;
- store the producer in endpoint/executor/worker state;
- wrap the producer in `Arc<Mutex<_>>` or another shared owner;
- create a detached task;
- create a second producer future;
- use callback `block_on`, `try_send`, `blocking_send`, or an alternate queue;
- extend the producer borrow beyond the executor endpoint invocation.

The exact C03e-OK driver remains the owner of one lexical producer-call future and its cooperative polling state machine. The new endpoint adapter is only a lifecycle wrapper and must not reimplement that state machine.

## 7. Shutdown law

Explicit supervisor shutdown remains the sole supervisor-shutdown authority.

The future adapter must not add another shutdown controller or infer shutdown from sender drop, channel close, receipt disposition, request-source closure, endpoint close, or transport idle state.

All producer/admission/worker quiescence and drain behavior remains inside the exact C03e-OK cooperative driver. Only after that driver returns does the adapter execute the existing endpoint close plus idle drain sequence.

This ordering is selected:

`cooperative driver terminal disposition -> endpoint transport close -> wait_idle -> return original collection result`

Do not close the endpoint early merely to unblock producer backpressure. Receiver closure required for pending producer shutdown remains owned by the C03e-OK cooperative driver after explicit supervisor shutdown is observed.

## 8. Rejected alternatives

### 8.1 Direct higher-owner call to the lower C03e-OK driver
Rejected because the exact module hierarchy makes the lower driver executor-private. Widening it solely for the higher endpoint owner would weaken the already-selected layering and bypass the existing endpoint lifecycle close/wait-idle surface.

### 8.2 Mutating both executor and higher endpoint owner in one successor
Rejected because the next independently materializable prerequisite is one file. A two-path change would collapse the staged OF dependency graph and make exact failure attribution weaker.

### 8.3 Defining concrete HandoffReceipt in the executor file
Rejected. OF selected the receipt semantics as a higher boundary-private production interface; OK deliberately kept the lower driver generic. The next adapter must preserve that genericity.

### 8.4 Moving endpoint close/wait-idle to the higher endpoint owner
Rejected. Existing executor endpoint lifecycle already owns this sequence. Duplicating or moving it would create lifecycle drift.

### 8.5 Installing channel/sender/request production now
Rejected. Production owner composition and all request-construction inputs remain separately gated.

## 9. Immediate future source ceiling

Only this path may change in the next source materialization:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Allowed source delta:
- one additive dormant executor endpoint-lifecycle producer adapter;
- the minimum documentation comments for that method;
- narrowly required local lint acknowledgement;
- focused same-file tests or compile-shape assertions only if required to validate the adapter's exact generic/borrow/return shape.

Historical methods, C03e-OK cooperative driver logic, current scheduling collection, current endpoint lifecycle, helper behavior, test behavior, and visibility of unrelated items remain unchanged.

No second Rust path is authorized.

## 10. Required future source validation

The future source materialization must demonstrate at minimum:
- exact C03e-OL predecessor/head topology;
- exactly one changed Rust path;
- no unrelated line deletion or historical method rewrite;
- borrowed `&mut H` forwards once into the C03e-OK driver;
- generic `Receipt` remains generic across adapter/driver boundary;
- suppression mapper and receipt observer are forwarded without replacement or hidden buffering;
- existing endpoint close then `wait_idle` executes after the lower driver returns;
- lower result is returned unchanged;
- no second shutdown authority;
- no concrete HandoffReceipt, production sender/channel, request construction, runtime caller, manifest/workflow/Android/deployment change;
- exact-final-head locked metadata, formatting, Clippy, tests and workspace build PASS.

SKIPPED is never PASS. A PASS on a superseded head does not validate a later head.

## 11. Later separately gated higher-owner boundary

Only after the executor endpoint adapter is materially present and exact-head validated may a fresh selection consider the remaining OF path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

That later boundary may evaluate:
- endpoint-owner forwarding of the borrowed producer;
- the boundary-private concrete handoff receipt interface selected by OF;
- exact placement of receipt correlation/disposition types;
- preservation of the sole-sender higher-owner law.

C03e-OL does not authorize that later source mutation.

## 12. Remaining production dependencies

Still separately gated after C03e-OL:
- boundary-private concrete `HandoffReceipt` source materialization;
- higher endpoint-owner borrowed-producer forwarding;
- production expected-request channel construction and sole-sender ownership;
- target admission `SessionId` source under the selected server-local CSPRNG law;
- independent nonzero expected-device PRWM authentication request-ID source;
- NB status-only dispatcher production construction/transfer;
- verifier-time source/interface compatibility;
- actual expected-device request construction and async enqueue;
- higher process/runtime caller migration;
- requester cleanup/candidate/reachability continuation;
- target dial, listener/bootstrap/readiness/executable activation;
- deployment.

## 13. Explicit non-actions

No Rust/source/runtime mutation in C03e-OL itself. No C03e-OK driver mutation. No visibility widening. No endpoint-owner source change. No concrete receipt type. No production sender/channel construction or clone. No expected-device request construction/send. No target admission SessionId generation. No expected-device PRWM authentication request-ID generation. No NB dispatcher production wiring. No verifier-time mutation. No grant clone/copy/reconstruction/remint/replay. No scheduling-consumption rollback. No callback `block_on`, hidden spawn, `try_send`, `blocking_send`, retry queue, alternate queue, or second producer future. No caller migration. No listener/bootstrap/readiness/runtime/executable activation. No Cargo/lockfile/workflow/Android source mutation. No deployment, merge, PR ready conversion/closure, branch deletion, force push/history rewrite, repository configuration/ruleset/permission change.

## 14. Documentation closure requirements

C03e-OL itself may change only this contract path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_OL_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_ENDPOINT_LIFECYCLE_ADAPTER_SOURCE_SEAM_SELECTION_STAGING.md`

Before closure:
- compare exact C03e-OK -> C03e-OL;
- require ahead 1 / behind 0 and exact C03e-OK merge base;
- require exactly one documentation path and zero Rust/Cargo/lockfile/workflow/Android/runtime changes;
- bind CI claims only to exact final C03e-OL head;
- record SKIPPED as SKIPPED, never PASS;
- publish one immutable raw audit to the canonical Drive evidence parent and verify readback if the established evidence workflow remains available;
- re-read branch and PR after evidence publication;
- keep the PR draft/open/unmerged.

No successor token is assigned until that closure is complete.

C03e-OL selection stops after documentation validation and durable evidence publication. The selected one-file executor endpoint-lifecycle adapter requires a fresh exact-head/concurrency audit before source materialization.
