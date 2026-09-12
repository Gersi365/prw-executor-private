# C03e-RE Production-Durable Post-Auth Fallible-Verifier-Time Requester/Rendezvous Scheduling-Aware Recoverable Repeated-Real-Admission Collection Custody Adapters Selection — Staging

## Status

`SELECTION — STAGING`

## Boundary

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COLLECTION_CUSTODY_ADAPTERS_SELECTION`

Selected future boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COLLECTION_CUSTODY_ADAPTERS_SOURCE_MATERIALIZATION`

## Exact predecessor

This selection starts only from evidence-closed C03e-RD.

RD exact identity:

- head `e5a6077dc0aa111776bff2a6ae625085120822f5`;
- tree `b419392b9ba1a511f4d445d410aeadf9a8e500eb`;
- final RD source blob `0c45ffa31c9e3263264544ae149ba29a6991fdc1`;
- PR #594 remains draft/open/unmerged/mergeable and evidence-closed;
- canonical RD audit Drive ID `1dk8vENOqAz8WokIA-xgcj5IT4alYezWt`, 19491 bytes, SHA-256 `bd0aac4818ac761fdb76b5fd4d177c848ebdde231bba5548c4e69c6a7df31f21`.

RD materialized the exact fallible scheduling completion peer-disposition classifier and consuming disposer, then explicitly stopped before collection migration, active-map migration, publish/poll/reap/drain migration, repeated-real-admission callback migration, producer propagation or endpoint/higher-owner propagation.

## Fresh source finding

The immediate missing dependency is narrower than the production-durable repeated-real-admission collection driver itself.

Exact RD source inspection shows that the repeated-real-admission integration module already contains:

- the exact QZ fallible scheduling recoverable worker-entry alias;
- the exact QZ fallible scheduling worker-entry constructor;
- the historical infallible scheduling active-map alias;
- the historical infallible scheduling generic completion alias;
- the historical scheduling publish adapter;
- the historical scheduling reap adapter;
- the historical scheduling cancellation fan-out adapter;
- the historical scheduling full-drain adapter;
- the historical scheduling in-flight-admission drain adapter.

However, the parallel fallible-verifier-time scheduling path does not yet have the collection-owned aliases and publish/reap/cancel/drain adapters needed by a later repeated-real-admission collection driver.

The separate production-durable collection file therefore cannot be migrated cleanly to exact fallible scheduling custody without first materializing this integration-local custody-adapter layer or widening the future collection checkpoint to a second Rust path.

RE selects the smaller one-path prerequisite instead.

## Exact source evidence

Future RF target path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact RD predecessor blob:

`db1398a64e51898df072bb49511dd6f0c99dbeb7`

The target already defines:

`RecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkerEntry`

as:

`RecoverablePersistentWorkerEntry<AuthenticatedRemoteSessionRuntimeOwner, RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop>`

and already defines the dormant QZ constructor:

`spawn_recoverable_fallible_verifier_time_requester_aware_worker_with_production_durable_scheduling(...)`

with exact verifier-time bound:

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`

No change to that QZ constructor is selected.

Historical scheduling custody in the same file proves the local adapter decomposition. It uses:

- `RecoverableSchedulingRequesterAwareWorkerCompletion`;
- `ActiveRecoverableSchedulingRequesterAwareWorkers`;
- `publish_recoverable_scheduling_completion(...)`;
- `reap_requester_aware_scheduling_workers(...)`;
- `request_all_requester_aware_scheduling_worker_cancellations(...)`;
- `drain_requester_aware_scheduling_workers(...)`;
- `drain_inflight_scheduling_admission(...)`.

Those surfaces adapt only exact generic recoverable custody to the scheduling-specific repeated-real-admission completion envelope and do not perform admission, peer disposition, scheduling derivation, endpoint teardown or higher-owner propagation.

## Selected future RF hard source ceiling

Future C03e-RF source materialization may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Required predecessor blob:

`db1398a64e51898df072bb49511dd6f0c99dbeb7`

No second Rust path is selected.

If exact compilation requires mutation of the parent recoverable-worker module, production-durable collection file, QX lifecycle, generic persistent custody, Cargo/lockfile, workflow, Android source, endpoint source, producer source or higher-owner source, RF must STOP and return to selection rather than widen this ceiling silently.

## Selected future RF imports

RF may extend the existing local `super::{...}` import group only enough to name the already-materialized exact RD/RB envelope:

`RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion`

No re-export or visibility widening is selected.

The exact QX fallible scheduling stop is already imported in this file and must remain the terminal result type:

`RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop`

No historical scheduling stop is substituted.

## Selected future RF generic completion alias

RF may add exactly one sibling generic completion alias conceptually named:

`RecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkerCompletion`

with exact underlying custody:

`RecoverablePersistentWorkerCompletion<
    DeviceId,
    AuthenticatedRemoteSessionRuntimeOwner,
    RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop,
>`

The alias must preserve exact by-value owner and terminal-result custody.

No clone/copy requirement, error conversion, result projection, stringification, remint, replay, default or side channel is selected.

## Selected future RF active-map alias

RF may add exactly one sibling active-map alias conceptually named:

`ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers`

with exact underlying map:

`HashMap<DeviceId, RecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkerEntry>`

Authenticated `DeviceId` remains the map key.

No second map, shadow map, alternate key, requester SessionId key, request-ID key, scheduling SessionId key, queue or registry is selected.

## Selected future RF publish adapter

RF may add one private sibling conceptually named:

`publish_recoverable_fallible_verifier_time_scheduling_completion(...)`

It must:

1. consume one exact `RecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkerCompletion` by value;
2. call existing generic completion `into_parts()` exactly once;
3. obtain exact authenticated `DeviceId`, recovered `AuthenticatedRemoteSessionRuntimeOwner`, and exact fallible scheduling worker/join result by value;
4. construct exactly one existing `RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion::new(...)`;
5. invoke the caller-supplied completion callback exactly once with that exact envelope.

It must not close the recovered peer, classify peer disposition, alter the worker result, clone the result, convert it to the historical scheduling stop or retain secondary custody.

RD's consuming disposer remains the separately gated owner-disposition boundary above this publication layer.

## Selected future RF reap adapter

RF may add one private sibling conceptually named:

`reap_requester_aware_fallible_verifier_time_scheduling_workers(...)`

It must:

1. borrow the exact active fallible scheduling map;
2. adapt completion publication only through the new exact publish adapter;
3. delegate ready-worker recovery to existing `reap_ready_recoverable_workers(...)`;
4. preserve existing generic ready-completion ordering and owner recovery semantics.

No direct task polling loop, second recovery algorithm, close/disposition action or callback projection is selected.

## Selected future RF cancellation fan-out adapter

RF may add one private sibling conceptually named:

`request_all_requester_aware_fallible_verifier_time_scheduling_worker_cancellations(...)`

It must delegate only to existing:

`request_all_recoverable_worker_cancellations(...)`

using the exact active fallible scheduling map.

No new cancellation primitive, timeout, retry, abort, peer close or task replacement is selected.

## Selected future RF full-drain adapter

RF may add one private async sibling conceptually named:

`drain_requester_aware_fallible_verifier_time_scheduling_workers(...)`

It must:

1. borrow the exact active fallible scheduling map mutably;
2. adapt each recovered generic completion only through the exact new publish adapter;
3. delegate actual drain/recovery to existing `drain_recoverable_workers(...)`;
4. preserve by-value authenticated owner and exact fallible scheduling result custody into the existing repeated-real-admission completion envelope.

No owner disposition, peer reuse, retry, respawn, restart, producer receipt mapping or endpoint teardown is selected.

## Selected future RF in-flight admission drain adapter

RF may add one private async sibling conceptually named:

`drain_inflight_fallible_verifier_time_scheduling_admission(...)`

It must mirror the historical scheduling adapter only at the exact fallible scheduling map/completion types:

1. retain the caller's in-flight admission future by pinned mutable borrow;
2. while admission remains pending, reap exact fallible scheduling worker completions through the new fallible reap adapter;
3. return the exact admission future output unchanged;
4. add no new admission transaction, timeout, retry, cancellation or close behavior.

## Explicitly not selected in RF

RE does not select a fallible sibling of:

`poll_one_requester_aware_scheduling_worker(...)`

That single-completion polling adapter exists for the later cooperative scheduling producer-driver layer and is not needed by the immediate non-cooperative repeated-real-admission collection prerequisite.

If a later cooperative producer migration requires a fallible `poll_one` sibling, it requires its own selection gate.

RE also does not select or authorize RF to add or modify:

- `drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling(...)`;
- any fallible repeated-real-admission collection driver;
- any endpoint lifecycle wrapper;
- any cooperative producer event enum or producer-driving helper;
- any completion disposer or peer-disposition classifier;
- any worker-entry constructor;
- any expected-request type or construction/send path;
- any admission timing source;
- any session/request identifier source;
- any verifier-time sampling/default/cache/fallback/retry logic.

## Exact byte-stable future guards

Future RF must keep byte-stable outside its one selected path, including at least:

1. RD parent recoverable-worker source:
   `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`
   blob `0c45ffa31c9e3263264544ae149ba29a6991fdc1`;

2. production-durable repeated-admission collection:
   `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`
   blob `08dc8160d72c41bc9a211c2c2e15f0e65d067b74`;

3. QX lifecycle:
   `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   blob `39f91dc8510df49620cf3336e98656532b46c158`;

4. generic recoverable persistent custody:
   `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`
   blob `4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`.

Within the selected integration file, the existing QZ fallible worker-entry alias and constructor must retain their current semantics and signatures. RF adds collection-custody adapters beside them; it does not migrate their callers.

## Ownership and provenance law

RF must preserve these invariants:

1. authenticated `DeviceId` is the active-map identity;
2. the existing generic recoverable entry remains sole owner of owner-cell, cancellation controller and join handle custody;
3. generic completion recovery yields the exact recovered authenticated owner once;
4. the exact `Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>` crosses the adapter layer by value unchanged;
5. nested verifier-time, ingress, requester-response and scheduling-terminal provenance remains inside that exact result;
6. no adapter performs RD peer disposition;
7. no adapter consumes scheduling-terminal custody;
8. no adapter samples verifier time;
9. no adapter constructs, authenticates or admits a remote session;
10. no adapter creates a task, channel, queue, retry loop or second owner cell.

## Why the collection driver remains later

The historical production-durable scheduling collection in the separate collection file owns a larger supervisor law:

- active-map construction;
- ready completion reaping;
- shutdown/request arbitration;
- duplicate-device preflight;
- admission timing;
- AJ invocation;
- worker insertion;
- shutdown cancellation;
- in-flight AJ drain;
- final active-worker drain.

Migrating that driver to fallible scheduling custody is a distinct semantic step. RE deliberately prepares only the missing exact local custody adapters so a later selection can evaluate that larger driver law against a one-path collection-file source ceiling.

This prevents RE/RF from bundling collection-driver migration with prerequisite type/custody plumbing across two Rust files.

## Later gates preserved

After future RF, still separately gated are:

- fallible verifier-time scheduling production-durable repeated-real-admission collection selection/materialization;
- active collection caller migration;
- fallible completion callback/disposer wiring into an endpoint lifecycle;
- fallible `poll_one` cooperative completion adapter;
- cooperative scheduling producer-driver migration;
- producer receipt/suppression migration;
- endpoint lifecycle propagation;
- higher endpoint-owner propagation;
- concrete expected-device producer reintegration;
- dispatcher-source capture;
- request-channel construction and ownership split;
- receiver wiring;
- higher receipt observation;
- request construction/send;
- target admission SessionId allocation;
- request-ID allocation;
- concrete verifier-time provider installation/sampling;
- listener/readiness/network/runtime activation;
- retry/reconnect/restart;
- auth/trust/RBAC mutation;
- DB/schema/control-plane mutation;
- Cargo/lockfile/workflow mutation;
- Android source mutation;
- packaging/service/systemd mutation;
- repository configuration mutation;
- merge;
- deployment;
- ready-for-review transition;
- PR close;
- branch deletion;
- reset/rebase/squash/force/history rewrite;
- destructive historical evidence cleanup.

Historical QS duplicate evidence remains untouched.

## Selection closure rule

C03e-RE is documentation-only.

The RE branch must differ from exact RD only by this one contract path. Exact-final-head Rust validation must succeed if registered. Path-filtered workflows must be recorded as `SKIPPED`, not PASS. No Android PASS may be inherited if Android does not register on the docs-only head.

Only after exact-final-head validation and immutable Drive publication/readback may the PR body become:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Keep the RE PR draft/open/unmerged.

**STOP after C03e-RE closure. Do not materialize C03e-RF inside this checkpoint.**
