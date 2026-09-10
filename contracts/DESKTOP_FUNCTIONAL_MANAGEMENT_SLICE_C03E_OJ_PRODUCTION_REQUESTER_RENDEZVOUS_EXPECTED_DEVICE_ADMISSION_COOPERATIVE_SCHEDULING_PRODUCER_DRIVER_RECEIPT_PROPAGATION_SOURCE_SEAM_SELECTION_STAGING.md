# Desktop Functional Management Slice C03e-OJ

## Production requester/rendezvous expected-device admission cooperative scheduling producer-driver receipt-propagation source-seam selection

Status: `SELECTION — VALIDATION_PENDING`
Date: 2026-09-10

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_DRIVER_RECEIPT_PROPAGATION_SOURCE_SEAM_SELECTION`

Authoritative predecessor: exact closed C03e-OI head `7e8a0b80d5bf7790703a813b8a52f325b80880a3`, tree `d16039954392f7c798c2f288ecf183faacd12196`.

This checkpoint changes documentation only. It re-audits the exact post-OI source before selecting any producer-driver source mutation. It does not install a production producer, create the selected channel, construct/send an expected-device request, materialize remaining constructor inputs, migrate an endpoint caller, or activate runtime behavior.

## 1. Decision

`OI_SINGLE_COMPLETION_ADAPTER_CONFIRMED / EXISTING_SCHEDULING_DRIVER_STILL_SYNCHRONOUS_BATCH_CALLBACK / BORROWED_LENDING_ASYNC_PRODUCER_REMAINS_SELECTED / DRIVER_RECEIPT_TYPE_MUST_REMAIN_GENERIC / SHUTDOWN_SUPPRESSION_REQUIRES_SYNCHRONOUS_CALLER_MAPPER / COOPERATIVE_DRIVER_CAN_BE_ONE_FILE_DORMANT_SIBLING / ENDPOINT_RECEIPT_TYPE_AND_PRODUCER_FORWARDING_DEFERRED / SOURCE_MATERIALIZATION_SEPARATELY_GATED`

C03e-OJ selects the next source stage as one dormant parallel scheduling-aware producer-driver sibling in the existing production-durable repeated-admission collection file. The new driver may borrow a caller-owned lending async producer, retain at most one lexical producer-call future, preserve receiver/admission/shutdown progress while that future is pending, and report every producer result through one synchronous receipt observer.

The driver itself must remain generic over the producer receipt type. It must not define the concrete expected-device `HandoffReceipt`, inspect grant eligibility for normal production, own the sender, or construct/send a request. Concrete receipt semantics, producer implementation, sole-sender custody and endpoint forwarding remain later stages.

## 2. Fresh exact-OI source observations

### 2.1 OI adapter is materially present

Path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact OI blob:
`e52c248463ca57c51a487df2357f72df0d2212dc`

Exact source now contains dormant `poll_one_requester_aware_scheduling_worker(...)`.

It:
- delegates to exact OG `poll_one_ready_recoverable_worker(...)`;
- returns `Poll::Pending` unchanged;
- on `Ready`, consumes exactly one generic completion by value;
- constructs exactly one existing `RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion` envelope;
- invokes no peer disposition and no callback.

This is sufficient for one-at-a-time scheduling completion extraction. No second extraction primitive is selected.

### 2.2 Existing production-durable scheduling driver is still callback/batch based

Path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Exact OI blob:
`eea51672c46aa83d50e6294a36e912fd0aa51928`

`drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling(...)` still:
- owns the expected-request receiver inside one existing `runtime.block_on`;
- calls batch `reap_requester_aware_scheduling_workers(...)` before polling shutdown/request;
- calls the same batch reaper before polling shutdown/in-flight admission;
- publishes each completion synchronously through `FnMut(RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion)`;
- uses the historical scheduling drain helpers on shutdown.

Therefore OI did not yet create a location that can retain one async producer-call future while the same driver continues consuming its own capacity-one receiver or advancing an in-flight admission.

### 2.3 Existing endpoint scheduling boundary still disposes then calls synchronous completion

The same exact child file exposes `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling(...)`.

Its current law remains:
`collection completion -> dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(...) -> synchronous on_completion(DeviceId, exact scheduling result)`.

Endpoint close and `wait_idle` happen only after the lower collection returns.

C03e-OJ preserves this historical sibling unchanged.

### 2.4 Existing endpoint-owner propagation still forwards a synchronous callback

Path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact OI blob:
`530ed80783023bbd03a1f1d7784a1aeec53d0090`

The OD scheduling-aware endpoint-owner sibling still forwards the lower scheduling callback unchanged. It has no producer borrow, concrete handoff receipt, sender owner, receipt observer or shutdown-suppression mapper.

### 2.5 Existing scheduling peer disposer remains exact authority

Path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact OI blob:
`7603b207b2efdd6c90a886c6e6f0abc232fa3bbc`

`dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(...)`:
- consumes one exact scheduling-specific completion;
- closes recovered owner custody by the existing NX acknowledgement-only disposition law;
- returns only requester-side authenticated `DeviceId` plus the exact unchanged scheduling worker/join result.

Scheduling derivation success/failure does not select peer disposition. C03e-OJ keeps this disposer before any producer or shutdown-suppression delivery.

## 3. Why a direct async callback replacement remains rejected

Replacing the existing synchronous completion callback with `producer(...).await` inline is still incorrect.

With the OE-selected capacity-one channel:
1. one expected request may already occupy the channel;
2. one newly detached scheduling completion may produce another eligible request;
3. its sole-sender producer awaits channel capacity;
4. if the driver awaits that producer in isolation, the same driver cannot poll the receiver;
5. channel capacity cannot be released by the consumer side.

OI removes the all-ready extraction problem but does not remove this self-backpressure cycle. Cooperative polling remains required.

`try_send`, `blocking_send`, callback `block_on`, detached/hidden spawn, sender cloning, a second queue, an unbounded queue, sleep/retry and grant remint/replay remain forbidden repairs.

## 4. Selected normal-running producer interface

The future driver takes a mutable temporary borrow of a caller-owned producer using the already-selected lending async shape, conceptually:

`H: AsyncFnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> R`

and receives `producer: &mut H`.

The concrete source declaration may use the stable equivalent syntax available to the pinned toolchain, but must preserve the following law:
- exact disposed `DeviceId` and scheduling result move by value into one call;
- the call future may borrow `producer` and therefore the caller-owned sole sender;
- exactly one call future may exist;
- no producer future is spawned/detached;
- the future is pinned in an ordinary lexical scope;
- the complete future storage is destroyed before `producer` is invoked again;
- no clone requirement is introduced for terminal result or receipt.

The driver is generic over receipt `R`. It does not know the concrete `HandoffReceipt` representation.

## 5. Selected synchronous receipt observer

The future driver also receives a synchronous observer conceptually:

`O: FnMut(R)`

Each completed producer call transfers its exact `R` once to the observer.

The observer:
- performs no async/blocking work;
- is not endpoint lifecycle status;
- is not permitted to convert a receipt into retry/remint authority;
- receives no sender or request replay handle from the driver.

`Result<(), RemoteSessionPersistentCollectionConfigError>` remains collection configuration/lifecycle status and must not absorb producer receipts.

## 6. Selected shutdown-suppression mapper

Fresh exact-source audit exposes one additional required interface for shutdown drain.

Once explicit supervisor shutdown is observed, no new async producer call may start. However active scheduling workers may still complete while they are cancelled/drained, and their exact terminal custody must still be disposed and classified without losing eligible grants or acknowledgement results.

Therefore the future driver receives a synchronous caller-owned mapper conceptually:

`Q: FnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> R`

This mapper is invoked only after explicit shutdown/quiescence for a completion that did not already start an async producer call.

Selected law:
- existing scheduling peer disposer runs first;
- exact requester `DeviceId` and exact result move once into `Q`;
- `Q` must produce the same generic receipt type `R` as the normal producer;
- the driver immediately moves that receipt once to `O`;
- `Q` performs no async send and creates no pending future;
- later concrete higher-owner implementation must map eligible grants to `SuppressedOnShutdown` while consuming/discarding the grant once and preserving exact acknowledgement custody;
- non-eligible typed results remain representable unchanged in the later concrete receipt;
- no eligible grant returns as retry/remint/replay authority.

The driver itself does not inspect scheduling eligibility or construct the concrete receipt.

## 7. Selected cooperative running-state law

The future dormant driver preserves the existing worker bound, request preflight, single in-flight admission, active-map identity and worker spawn law.

When no producer call is pending, each relevant supervisor poll may extract at most one ready scheduling completion through exact OI `poll_one_requester_aware_scheduling_worker(...)`.

If one completion is ready:
1. detach exactly that one completion;
2. run exact existing scheduling peer disposer;
3. start exactly one borrowed async producer call with the disposed values;
4. retain that one call future until terminal receipt;
5. extract no second scheduling completion while the call future remains pending.

While the producer future is pending, the driver must still make progress on the existing consumer side:
- explicit supervisor shutdown remains highest priority;
- while waiting for a next request, continue polling the expected-request receiver when `active.len() < max_active_workers`;
- if a request is received, preserve existing duplicate-active-device preflight and admission timing before starting the one admission;
- while admission is in flight, continue polling that exact admission future;
- admission success derives the active key only from authenticated session-owner `DeviceId` and inserts the existing scheduling-aware worker;
- admission failure preserves the existing failure callback;
- no second admission is started while one admission future exists.

A pending producer future is not permission to return `Pending` before polling an otherwise eligible receiver or in-flight admission. This is the critical self-backpressure progress law.

## 8. Lexical future lifetime / state-machine law

The future implementation must not place a lending producer-call future into storage whose borrow effectively survives across multiple future producer invocations.

Selected implementation structure:
- the outer scheduling driver has a no-producer state;
- when one completion is detached/disposed, it enters one lexical `drive-one-producer` scope;
- that scope owns exactly one pinned producer-call future and cooperatively advances receiver/admission/shutdown state until the producer future reaches one receipt or explicit shutdown moves the scope into shutdown drain;
- after the receipt is observed and the one future object is destroyed, control returns to the no-producer state and only then may another completion start another producer call.

Equivalent source organization is permitted only if the compiler-visible borrow lifetime and one-future bound are identical.

Do not retain a stale `Pin<&mut _>` or an `Option` whose lifetime prevents provable destruction of the underlying producer future before the next producer invocation.

## 9. Selected explicit-shutdown law

Explicit existing supervisor shutdown remains the only endpoint-supervisor shutdown authority.

When it becomes ready:
1. enter producer quiescence immediately;
2. no new expected request may begin admission;
3. no new async producer call may start;
4. call `expected_requests.close()` so the receiver rejects future sends and wakes a pending capacity wait;
5. deterministically retire/drop any already-buffered but unadmitted request; do not turn it into a new admission after quiescence;
6. request cancellation of all currently active scheduling-aware workers using the existing cancellation authority;
7. if an async producer call was already started before shutdown, retain and poll that exact future to one terminal receipt; do not drop/recreate it as a select loser;
8. if admission was already in flight, retain/drain the exact admission using the existing shutdown law; a post-shutdown authentication success is closed orderly and is not inserted as a worker;
9. drain existing active scheduling workers; each completion is passed through the existing peer disposer, then `Q`, then `O`; no async producer is invoked for these drain completions;
10. finish only after the already-started producer receipt, in-flight admission disposition and active-worker drain reporting are complete.

Receiver close is an effect of already-observed shutdown, not an independent shutdown authority.

Because the sole producer is cooperatively polled on the same current-thread runtime, shutdown is observed before any later producer poll. Closing the receiver therefore prevents a still-pending capacity wait from committing a new request after quiescence; the producer later observes the selected closed-channel terminal behavior. A send that had already returned `Ok(())` before shutdown remains enqueue-only success.

No pending producer future may be abandoned merely because shutdown won.

## 10. Buffered request retirement

The future driver may use a finite nonblocking receiver drain after `Receiver::close()` solely to destroy already-buffered unadmitted requests during explicit shutdown.

This cleanup:
- does not use `try_send`;
- does not create an admission;
- does not report authentication success/failure;
- does not reopen or replace the channel;
- does not create request retry authority;
- does not change the prior producer's enqueue-only receipt semantics.

The OE-selected production channel capacity remains exactly one. The driver does not create or own that channel in this source stage.

## 11. Source ceiling selected by OJ

Immediate next source boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_DRIVER_SOURCE_MATERIALIZATION`

Immediate source ceiling is exactly one existing Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

The future source checkpoint may add only:
- one separately named dormant scheduling-aware producer-driver sibling;
- narrowly scoped private event/poll helpers in the same file if required to express cooperative receiver/admission/producer/shutdown progress;
- focused same-file tests;
- narrowly required dormant lint acknowledgement.

It may reuse unchanged:
- OI `poll_one_requester_aware_scheduling_worker(...)` from the parent module;
- existing scheduling peer disposer;
- existing request preflight/admission transaction/worker spawn/cancellation logic;
- existing scheduling drain helpers where their callback form can safely implement shutdown `dispose -> Q -> O` processing.

If correctness or compilation requires a second Rust path, STOP and return to selection. Do not widen private completion/grant visibility as a workaround.

## 12. Source paths explicitly excluded from the immediate stage

The immediate source stage must not modify:
- `repeated_real_admission_requester_aware_persistent_fl_integration.rs` — OI adapter is complete and reused unchanged;
- `recoverable_persistent_requester_rendezvous_worker.rs` — OG helper is complete and reused unchanged;
- `recoverable_spawned_requester_rendezvous_worker.rs` — existing peer disposer remains authority;
- `remote_session_endpoint_lifecycle_runtime.rs` — producer borrow/receipt forwarding remains later;
- scheduling grant/terminal definition files;
- Linux bootstrap/process lifecycle files;
- Cargo manifests/lockfile/workflows/Android/packaging/deployment files.

The historical scheduling collection and historical scheduling endpoint lifecycle inside the selected child file must remain behaviorally unchanged. The new behavior belongs only in separately named dormant siblings/helpers.

## 13. Later endpoint/receipt stage

After the one-file driver source materialization closes and a fresh exact-head audit succeeds, a later separately gated endpoint source stage may:
- define the concrete private handoff receipt/disposition family at the endpoint/higher-owner boundary;
- add one borrowed-producer scheduling endpoint sibling;
- forward `&mut producer`, synchronous shutdown-suppression mapping, and synchronous receipt observation into the lower producer driver;
- preserve endpoint close plus exact `wait_idle` after the lower driver terminates.

C03e-OJ does not authorize that later file mutation.

The concrete receipt must eventually preserve the OF-selected distinction between non-eligible typed results and eligible acknowledgement custody plus exactly one of `Enqueued`, `ConstructionFailed`, `ChannelClosed`, or `SuppressedOnShutdown`. Eligible grants must never be returned as retry handles.

## 14. Remaining request-construction dependencies

Even after the cooperative driver exists, actual expected-device request construction/send remains blocked pending separately materialized:
- target admission `SessionId` source under the OB server-local CSPRNG law;
- independent nonzero expected-device PRWM authentication request-ID source with live-transaction uniqueness/exhaustion law;
- NB status-only dispatcher construction/transfer;
- verifier-time source/interface compatible with the request-carried `T` bound;
- production creation of the OE-selected capacity-one channel and sole-sender owner;
- concrete producer implementation using those inputs.

No live expected-device request, sender or runtime producer is selected by OJ.

## 15. Future validation requirements

Before the future one-file driver source checkpoint can close, exact-head tests/CI must establish at minimum:
- a pending producer future does not prevent capacity-one receiver progress;
- full-channel producer send can become unblocked because the same driver still polls/consumes the receiver;
- no second scheduling completion is detached while one producer future is pending;
- admission continues to progress while producer work is pending;
- producer receipt is observed exactly once;
- explicit shutdown prevents any new producer/admission start;
- receiver closure wakes/terminates pending producer work without recreating it;
- buffered unadmitted request is retired after shutdown and is not admitted;
- already-started producer future reaches terminal receipt during shutdown;
- drain completions use peer disposer then synchronous suppression mapping, never async producer invocation;
- no non-Clone scheduling result or receipt is duplicated;
- historical scheduling siblings remain unchanged.

Formatting, Clippy, tests and workspace build must pass on the exact final source head. Android PASS may be claimed only if an exact-head Android run exists and succeeds. `SKIPPED != PASS`.

## 16. C03e-OJ documentation scope

Only this staging contract path may differ from exact C03e-OI:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_OJ_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_DRIVER_RECEIPT_PROPAGATION_SOURCE_SEAM_SELECTION_STAGING.md`

No Rust/source/runtime mutation is authorized in OJ itself.

## 17. Explicit non-actions

No Rust/source/runtime mutation.
No producer driver materialization in OJ.
No endpoint producer forwarding.
No concrete `HandoffReceipt` source type.
No production channel creation.
No sender creation/retention/clone.
No request construction/send.
No target admission SessionId generation.
No expected-device PRWM authentication request-ID generation.
No NB dispatcher production wiring.
No verifier-time source/interface mutation.
No caller migration.
No runtime/listener/bootstrap/readiness/executable activation.
No Cargo/lockfile/workflow/Android mutation.
No deployment.
No merge.
No branch deletion.
No PR ready conversion/closure.
No force push/rebase/squash/history rewrite.
No repository configuration/ruleset/permission mutation.

## 18. Closure law

Validate that OI -> OJ is exactly one documentation commit with exact OI merge base and zero source/runtime changes. Bind any PASS claim only to exact final OJ head. Publish one immutable raw audit in the canonical Drive evidence folder, verify exact bytes/SHA-256/title uniqueness, then update only PR body metadata to `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED` while keeping the PR draft/open/unmerged.

No successor checkpoint token is assigned by this contract. A fresh exact-head/concurrency audit is required before the selected one-file source materialization.

After closure: `STOP`.
