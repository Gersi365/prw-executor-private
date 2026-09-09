# Desktop Functional Management Slice C03e-OH

## Production requester/rendezvous expected-device admission scheduling-specific single-completion adaptation source-seam selection

Status: `SELECTION — VALIDATION_PENDING`
Date: 2026-09-09

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_SPECIFIC_SINGLE_COMPLETION_ADAPTATION_SOURCE_SEAM_SELECTION`

Authoritative predecessor: exact closed C03e-OG head `b2584f9c6b5f6c450b8946ef7a9f7d15a27c4cde`, tree `86d7c4202a5314835562df93d86c236e5c7eb246`.

This checkpoint changes documentation only. It re-audits the exact post-OG source and selects the smallest next source seam after the generic single-ready-completion prerequisite. It does not mutate Rust, migrate a caller, construct a producer/channel/request, allocate identifiers, change timing, or activate runtime behavior.

## 1. Decision

`GENERIC_SINGLE_READY_HELPER_REUSED_UNCHANGED / SCHEDULING_SPECIFIC_SINGLE_COMPLETION_ADAPTER_REQUIRED / ONE_EXISTING_RUST_PATH_ONLY / BY_VALUE_OWNER_AND_TERMINAL_CUSTODY_PRESERVED / NO_PEER_DISPOSITION_IN_ADAPTER / NO_DRIVER_MIGRATION / NO_PRODUCER_OR_REQUEST_SEND / SOURCE_MATERIALIZATION_SEPARATELY_GATED`

The immediate future source stage is one additive dormant scheduling-specific adapter in exactly:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

No other Rust path is selected for that immediate stage.

## 2. Fresh authority and concurrency guard

Before assigning C03e-OH:
- C03e-OG was directly re-read at exact head `b2584f9c6b5f6c450b8946ef7a9f7d15a27c4cde` and tree `86d7c4202a5314835562df93d86c236e5c7eb246`;
- PR #521 remained draft/open/unmerged/mergeable and explicitly `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- recent PR chronology still had #521 as newest;
- branch search for `phase-152-c03e-oh` returned zero;
- semantic branch search for scheduling-specific single-completion returned zero;
- PR search for `C03e-OH` returned zero;
- the planned OH contract path returned 404 at exact OG.

Only after these direct checks was token C03e-OH assigned. The token was not inferred merely from alphabetic sequence.

## 3. Exact post-OG source observations

### 3.1 Generic prerequisite is now materially present

Path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`

Exact OG blob:
`4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`

Exact source now contains dormant:

`poll_one_ready_recoverable_worker<K, O, T>(...) -> Poll<RecoverablePersistentWorkerCompletion<K, O, T>>`

Its materialized law is already validated by C03e-OG:
- uses caller-provided `Context`;
- scans until first `Ready` join only;
- stops immediately after first ready join;
- detaches exactly one active entry;
- recovers exact owner through the existing owner-cell helper;
- preserves existing join-result mapping;
- clones only the map key required for detachment;
- moves owner/result by value;
- returns `Poll::Pending` for empty/all-pending maps;
- creates no queue/backlog and no second completion future.

C03e-OH does not reopen or modify that generic helper.

### 3.2 Scheduling integration already owns the exact concrete aliases

Path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact OG blob:
`4d2389bfc5e671f44b23438e258d328d92484895`

The file already defines:
- `RecoverableSchedulingRequesterAwareWorkerEntry` as the generic persistent entry specialized to `AuthenticatedRemoteSessionRuntimeOwner` plus `RequesterRendezvousProductionDurableSchedulingWorkerStop`;
- `RecoverableSchedulingRequesterAwareWorkerCompletion` as the generic completion specialized to `DeviceId`, the exact authenticated owner, and the scheduling-aware stop;
- `ActiveRecoverableSchedulingRequesterAwareWorkers` as the corresponding `HashMap<DeviceId, ...>`.

No new scheduling-specific entry/completion struct is required.

### 3.3 Existing conversion is already proven by the historical batch path

The same integration file already contains `publish_recoverable_scheduling_completion(...)`.

It consumes one exact specialized generic completion, calls `into_parts()`, and constructs the already-existing `RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion::new(device_id, session_owner, result)`.

Therefore the required type conversion is not a new authority decision. The new one-at-a-time adapter only needs to apply that same by-value conversion to the output of the OG generic single-ready helper.

### 3.4 Current scheduling reaper remains batch-oriented

The existing `reap_requester_aware_scheduling_workers(...)` remains callback-based and delegates to `reap_ready_recoverable_workers(...)`, which can detach every ready worker in one call.

Historical batch behavior is still correct for current dormant callers and must remain unchanged.

The future async producer handoff cannot use this batch reaper because OF selected at most one detached completion / one scoped producer future at a time.

### 3.5 Downstream scheduling driver currently calls the batch reaper in two phases

Exact OG `production_durable_repeated_real_admission_collection.rs` still calls `reap_requester_aware_scheduling_workers(...)`:
1. while waiting for shutdown or the next expected request;
2. while waiting for shutdown or one in-flight admission.

C03e-OH does not migrate either call. Cooperative producer/receiver/admission polling remains a later source stage.

### 3.6 Peer disposition remains downstream and must not move into this adapter

Exact OG scheduling completion carrier is defined in:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact blob:
`7603b207b2efdd6c90a886c6e6f0abc232fa3bbc`

`RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion::new(...)` and `into_parts(self)` are already private-accessible in the existing module lineage.

The separate existing disposer:
`dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(...)`
consumes the recovered authenticated owner and applies the NX acknowledgement-only scheduling peer-disposition law before returning only requester authenticated `DeviceId` plus unchanged terminal result custody.

The new single-completion adapter must not call this disposer. Owner disposition belongs later at the already-selected higher-owner boundary immediately before producer delivery.

## 4. Selected immediate future source seam

The next source checkpoint may add one dormant helper with conceptual shape:

`poll_one_requester_aware_scheduling_worker(
    active: &mut ActiveRecoverableSchedulingRequesterAwareWorkers,
    context: &mut Context<'_>,
) -> Poll<RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion>`

The exact function name may remain private implementation detail, but the source semantics are fixed here.

Required implementation law:
1. Import and call exact `poll_one_ready_recoverable_worker(...)` from the OG generic helper module.
2. Pass the exact active scheduling map and caller-provided `Context` unchanged.
3. On `Poll::Pending`, return `Poll::Pending` unchanged.
4. On `Poll::Ready(completion)`, consume exactly that one completion by value.
5. Call `completion.into_parts()` exactly once.
6. Construct exactly one existing `RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion::new(device_id, session_owner, result)`.
7. Return that scheduling-specific completion by value.
8. Do not poll a second worker in the same adapter call beyond whatever pending entries the generic helper law already polls before the first ready result.
9. Do not call the peer disposer.
10. Do not invoke any callback or async producer.

A direct `Poll::map(...)` or equivalent one-branch match is acceptable only if it preserves exactly this law.

## 5. Immediate source ceiling

Exactly one path may change in the next materialization checkpoint:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Permitted changes:
- import `poll_one_ready_recoverable_worker` from the existing generic helper module;
- add one dormant private or `pub(super)` scheduling-specific single-completion adapter as narrowly required by the next consumer;
- add only narrowly necessary `dead_code` acknowledgement;
- add focused same-file tests only if they can be built without widening private authority or inventing synthetic production custody.

Not permitted in this immediate stage:
- modifying `recoverable_persistent_requester_rendezvous_worker.rs`;
- modifying `production_durable_repeated_real_admission_collection.rs`;
- modifying `remote_session_endpoint_lifecycle_runtime.rs`;
- modifying scheduling terminal/grant definitions;
- modifying the peer disposer;
- changing existing batch reaper behavior;
- changing existing drain behavior;
- changing existing scheduling collection callers.

If compilation or correctness requires a second Rust path, the source checkpoint must STOP and return to selection.

## 6. Type and custody preservation

The adapter must preserve exact by-value custody:
- authenticated `DeviceId` remains the active-map key attached to the detached worker;
- exact `AuthenticatedRemoteSessionRuntimeOwner` moves once;
- exact `Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>` moves once;
- no owner clone;
- no scheduling result clone;
- no scheduling grant clone/copy/reconstruction/remint/replay;
- no conversion to an authority-losing tuple beyond the existing one-shot `into_parts()` used immediately to construct the existing scheduling completion envelope.

The adapter creates no new retry token, sender, endpoint, target, timing, dispatcher, request ID, SessionId, or request bytes.

## 7. Identity and scheduling authority remain unchanged

C03e-OH preserves all closed predecessor laws:
- the completion `DeviceId` is requester-side authenticated worker identity, not target expected `DeviceId`;
- target expected `DeviceId` is available only from a consumed construction-eligible scheduling grant later in producer processing;
- requester scheduling `SessionId` cannot substitute for target admission `SessionId`;
- acknowledgement result remains orthogonal to scheduling derivation;
- peer disposition uses acknowledgement-only law and stays downstream;
- request IDs remain correlation only;
- no unrelated PRWC/candidate correlation becomes authentication PRWM request-ID authority.

This adapter does not inspect scheduling eligibility and does not open the grant.

## 8. Why peer disposition is deliberately deferred

The generic worker completion still owns the recovered authenticated session owner. The existing scheduling-specific completion envelope is the correct custody object for carrying that owner to the existing disposer.

Performing peer disposition inside the adapter would couple generic readiness extraction to higher-owner terminal policy and would prevent later driver code from preserving the already-selected ordering:

`detach exactly one -> scheduling-specific envelope -> existing peer disposition -> producer delivery`

Therefore disposition in the adapter is forbidden.

## 9. Why caller migration is deliberately deferred

The production-durable scheduling driver currently has synchronous callback completion publication and batch reaping in both its request-wait and in-flight-admission phases.

Replacing those calls with the single adapter without simultaneously introducing retained producer-future state would either:
- drop the returned completion;
- reintroduce a synchronous callback;
- require a local backlog;
- or prematurely widen the driver into the OF async handoff stage.

All four are outside this source seam.

The adapter must be dormant until a separately gated driver stage consumes it.

## 10. Validation expectations for the future source checkpoint

At minimum:
- exact predecessor merge base must remain C03e-OH closed head;
- only the single selected Rust path may change;
- formatting, Clippy, tests and workspace build must PASS at the exact final head;
- Android PASS may be claimed only if an exact-head Android run exists and succeeds;
- SKIPPED remains not PASS;
- no test may depend on HashMap iteration order;
- no test may widen scheduling grant visibility or introduce cloneability;
- no test may use a production sender/channel/request to validate this dormant adapter.

Where concrete scheduling-owner fixtures make a direct adapter unit test disproportionate or would require new authority constructors, existing OG generic helper tests plus exact type-check/Clippy/workspace tests are acceptable for this narrowly structural adapter. Do not invent production-only constructors solely for a test.

## 11. Later source graph remains separately gated

After the one-file adapter closes, the next fresh audit must re-evaluate the driver stage in:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

That later stage must address the OF-selected cooperative progress law:
- at most one retained producer future;
- explicit supervisor shutdown priority;
- receiver progress while producer send is pending;
- in-flight admission progress while producer work is pending;
- no second detached completion/backlog;
- exact peer disposition before producer delivery;
- shutdown quiescence and pending-send terminal reporting;
- no `try_send`, `blocking_send`, `block_on`, hidden spawn or unbounded queue.

`remote_session_endpoint_lifecycle_runtime.rs` borrowed-producer forwarding and private receipt propagation remains later again unless the fresh driver audit proves a different minimal dependency boundary.

This dependency list is not authorization to mutate those files now.

## 12. Construction dependencies remain unresolved

Actual expected-device request construction/send remains blocked pending separately materialized:
- target admission `SessionId` source under the OB server-local CSPRNG law;
- independent nonzero expected-device PRWM authentication request-ID source;
- NB status-only dispatcher production construction/transfer;
- verifier-time source/interface compatibility;
- production channel construction and sole-sender owner integration;
- final async producer implementation and receipt reporting.

No complete expected-device admission producer is claimed by C03e-OH.

## 13. C03e-OH scope

This checkpoint is documentation-only.

Only this contract path may differ from exact C03e-OG:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_OH_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_SPECIFIC_SINGLE_COMPLETION_ADAPTATION_SOURCE_SEAM_SELECTION_STAGING.md`

Expected topology:
- exact OG merge base;
- one commit;
- one added docs path;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging/deployment changes.

## 14. Explicit non-actions

C03e-OH performs no:
- Rust/source/runtime mutation;
- generic helper change;
- scheduling-specific adapter source materialization;
- current scheduling batch-reaper change;
- drain change;
- caller migration;
- peer disposition movement;
- producer callback/future installation;
- channel or sender creation/clone;
- request construction/send;
- target admission SessionId generation;
- PRWM authentication request-ID allocation;
- NB dispatcher construction;
- verifier-time source/interface mutation;
- requester cleanup;
- candidate/reachability continuation;
- target dial;
- listener/bootstrap/readiness/runtime activation;
- executable main wiring;
- Cargo/lockfile/workflow/Android mutation;
- deployment;
- merge;
- branch deletion;
- force push/history rewrite;
- repository configuration/ruleset/permission change.

## 15. Next separately gated source boundary

If exact-head validation and durable evidence close C03e-OH, the selected immediate future source boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_SPECIFIC_SINGLE_COMPLETION_ADAPTATION_SOURCE_MATERIALIZATION`

Immediate source ceiling:
exactly `repeated_real_admission_requester_aware_persistent_fl_integration.rs` above.

No successor token is assigned by this contract. A fresh exact-head/concurrency audit remains mandatory before source mutation.

C03e-OH becomes CLOSED only after exact-head validation, immutable canonical Drive evidence, post-publication readback, and PR body closure metadata are complete. Keep the PR draft/open/unmerged.