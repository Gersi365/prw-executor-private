# C03e-PK — Production Requester/Rendezvous Expected-Device Admission Fallible Verifier-Time Persistent Worker Collection Result/Ownership Source-Seam Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_PERSISTENT_WORKER_COLLECTION_RESULT_OWNERSHIP_SOURCE_SEAM_SELECTION`

Selected later boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_PERSISTENT_WORKER_COLLECTION_RESULT_OWNERSHIP_SOURCE_MATERIALIZATION`

## 1. Exact predecessor

The authoritative predecessor is the closed C03e-PJ supervised executor worker drive source materialization.

- PJ branch: `phase-152-c03e-pj-production-requester-rendezvous-expected-device-admission-fallible-verifier-time-supervised-executor-worker-drive-source-materialization`
- PJ head: `a20e54f8952879863b60fa1d9c6a270aa39e2715`
- PJ tree: `a9dcda71ab4e55c6c2746ea4dd007b42ed87ea0b`
- executor source blob: `38ec3a537db03fec0540ae433716463491cdbbca`
- PJ PR: `#549`
- PJ PR remains draft/open/unmerged.

The canonical immutable PJ audit is the byte-verified Drive object with ID `1GzKqQj_EgcK4iQUJy03Kz_mCbU4p0FAH` under canonical parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`.

A later duplicate upload was detected and quarantined by metadata-only rename; the canonical evidence bytes were not rewritten. This does not alter PJ source authority.

## 2. Selection purpose

C03e-PJ closes the single-worker supervised compatibility layer for the fallible verifier-time worker. The next unresolved executor compatibility surface is the historical persistent worker collection boundary.

The exact PJ source shows that the persistent collection core is already generic over worker terminal type. Therefore C03e-PK must not reimplement or retype that generic machinery. It selects only the fixed historical result/ownership seam required to expose the fallible worker terminal through one persistent collection drive sibling.

This is a documentation-only checkpoint. No Rust source is modified by C03e-PK.

## 3. Fresh exact-source observations

At exact PJ source blob `38ec3a537db03fec0540ae433716463491cdbbca`:

- `RemoteSessionPersistentWorkerEntry<T>` is generic over worker terminal `T`;
- `map_worker_join_result<T>` is generic over `T`;
- persistent reaping/draining helpers are generic over `T`;
- `run_persistent_worker_collection<T, ...>` is generic over the worker terminal and completion callback;
- `RemoteSessionWorkerAdmission<D, T>` already carries dispatcher and verifier-time provider generically;
- `RemoteSessionWorkerAdmissionRejection<D, T>` already preserves rejected admission ownership generically.

The fixed historical result surface is:

`RemoteSessionRegisteredWorkerCompletion`

which stores:

`Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

The fixed historical drive surface is:

`RemoteSessionExecutorRuntime::drive_persistent_remote_worker_collection`

whose verifier-time provider is still:

`T: FnMut() -> u64 + Send + 'static`

and whose completion callback still consumes:

`RemoteSessionRegisteredWorkerCompletion`

Those two fixed surfaces, not the generic collection machinery, are the selected compatibility gap.

## 4. Historical decomposition evidence

Historical C03e-AG/C03e-AH established the persistent collection invariants:

- one long-lived private current-thread runtime drive owns the collection lifetime;
- active workers are keyed by authenticated logical `DeviceId`;
- one active worker per `DeviceId`;
- duplicate rejection occurs before ownership-consuming spawn;
- caller-bounded capacity is enforced against `MAX_REGISTERED_DEVICES`;
- admission is not polled while full;
- ready completions are reaped before shutdown/admission;
- completion accounting preserves `DeviceId` plus bounded worker/join result;
- orderly shutdown requests cancellation for all retained workers before draining them;
- admission-source closure does not fabricate supervisor shutdown.

The current chain is intentionally narrower than historical AH: admission/request-carrier migration is not selected here. Only persistent completion/result ownership compatibility is selected.

## 5. Selected future source ceiling

A later, separately gated C03e-PL source materialization may modify exactly one existing Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No other Rust/source path, Cargo manifest, lockfile, workflow, Android source, contract, transport, endpoint, requester/rendezvous, database, authentication, authorization or production-startup path is selected by PK.

Focused same-file tests/doc comments necessary to prove the selected seam are permitted only inside that same source ceiling.

## 6. Selected fallible completion sibling

A later C03e-PL materialization may add one sibling completion wrapper conceptually named:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

Maximum selected visibility: `pub(super)` unless fresh C03e-PL source/lint evidence proves a narrower visibility cannot support the same-file compatibility seam.

The sibling must preserve exactly:

- authenticated logical `DeviceId`;
- `Result<AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

It must not collapse or reclassify:

- `AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Cancelled`;
- `AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Failed(exact_PB_error)`;
- `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

The wrapper may expose bounded accessors analogous to the historical completion type, but it must not require the fallible worker stop or embedded PB error to become `Copy`, `Clone`, stringified, flattened or converted merely to mimic the historical accessor shape.

If value-returning accessors are not type-correct under the exact fallible stop traits, the future materialization must use reference/ownership-preserving access rather than widening trait bounds.

## 7. Selected persistent drive sibling

The selected future executor sibling is conceptually:

`drive_persistent_fallible_verifier_time_remote_worker_collection`

Maximum selected visibility: `pub(super)`.

Conceptual signature:

```rust
pub(super) fn drive_persistent_fallible_verifier_time_remote_worker_collection<
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static,
    S: Future<Output = ()> + Send,
    C: FnMut(RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion),
    R: FnMut(RemoteSessionWorkerAdmissionRejection<D, T>),
>(
    &mut self,
    max_active_workers: NonZeroUsize,
    authority: &SharedCurrentCapabilityAuthority<P>,
    admissions: mpsc::Receiver<RemoteSessionWorkerAdmission<D, T>>,
    supervisor_shutdown: S,
    on_completion: C,
    on_rejection: R,
) -> Result<(), RemoteSessionPersistentCollectionConfigError>
```

The exact future signature remains subject to rustfmt/compiler verification, but the semantic bounds and ownership law above are selected.

## 8. Generic-core reuse law

C03e-PL must reuse the existing generic persistent collection core unchanged:

`run_persistent_worker_collection`

It must also reuse unchanged:

- `RemoteSessionPersistentWorkerEntry<T>`;
- persistent reap/drain helpers;
- `map_worker_join_result<T>`;
- capacity validation;
- `RemoteSessionWorkerAdmission<D, T>`;
- `RemoteSessionWorkerAdmissionRejection<D, T>`;
- `remote_session_worker_cancellation_pair()`.

No new fallible-specific persistent collection algorithm/helper is selected.

No existing generic helper is to be specialized, duplicated, retyped or made public merely for this seam.

## 9. Spawn and worker-delegation law

For each admitted authenticated worker candidate, the future persistent drive sibling must mirror the historical collection spawn closure while substituting only the fallible verifier-time worker body.

Per admitted worker:

1. clone the shared-current authority exactly once for that spawned task;
2. consume the existing `RemoteSessionWorkerAdmission<D, T>` into authenticated session owner, dispatcher and fallible verifier-time provider;
3. create exactly one existing cancellation controller/signal pair;
4. create exactly one `tokio::spawn(async move { ... })` worker;
5. move owner, dispatcher, provider and cancellation signal into that task;
6. invoke `run_fallible_verifier_time_capability_request_worker` directly and exactly once;
7. pass `cancellation_signal.into_cancelled()` unchanged;
8. return the existing cancellation controller and same join handle to the generic collection core.

The task must not call:

- PF synchronous borrowed fallible bridge;
- PH synchronous spawned fallible bridge;
- PJ synchronous supervised fallible bridge.

Direct PD async-worker delegation preserves collection-owned cancellation and join custody and prevents nested private-runtime entry.

## 10. Completion and error law

The generic collection core remains responsible for join reaping and bounded abnormal-join mapping.

The future persistent compatibility seam must preserve:

`Ok(AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Cancelled)`

as a normal completed worker result.

It must preserve:

`Ok(AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Failed(exact_PB_error))`

as a normal completed worker result.

Only abnormal Tokio task completion remains:

`Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)`

No new persistent error enum, executor error wrapper, string projection, retry result, synthetic cancellation result or raw Tokio `JoinError` is selected.

## 11. DeviceId and ownership law

The authenticated session owner's logical `DeviceId` remains the active-worker key and completion identity.

The future sibling must not introduce:

- transport identity as the active key;
- session ID as the active key;
- request ID as the active key;
- verifier timestamp as the active key;
- caller-supplied replacement DeviceId.

Duplicate-active admission continues to reject before spawn and returns the untouched generic `RemoteSessionWorkerAdmission<D, T>` through the existing rejection wrapper.

No rejected authenticated owner may be silently dropped merely to bridge fallible typing.

## 12. Capacity and shutdown law

The existing persistent collection configuration and runtime behavior remain authoritative:

- caller-supplied `NonZeroUsize` capacity;
- reject capacity above `MAX_REGISTERED_DEVICES` before runtime work;
- retained-entry count is capacity accounting;
- do not poll admission while full;
- reap ready completions before shutdown/admission work;
- on orderly supervisor shutdown, stop admission;
- request cancellation for every retained worker before draining;
- drain/reap the same retained handles until the map is empty;
- admission-source closure alone does not fabricate shutdown.

Fallible verifier-time worker failures do not terminate the whole collection unless the existing collection algorithm already terminates for the corresponding generic completion event. They are completion values delivered through the selected fallible completion wrapper.

## 13. Verifier-time law

The future persistent drive sibling receives verifier time only through:

`FnMut() -> Result<u64, PrwaVerifierSourceError>`

The executor collection layer must not perform verifier-time sampling itself.

No:

- eager sample;
- retry;
- fallback;
- default;
- timestamp cache;
- stale-value reuse;
- clamp;
- saturation;
- conversion to infallible provider;
- conversion of verifier error to cancellation;
- string projection of verifier error

is selected.

PB/PD remain authoritative for verifier-time/request-loop failure and peer-close classification.

## 14. Historical API preservation

The following historical surfaces must remain untouched by a later C03e-PL materialization:

- `RemoteSessionRegisteredWorkerCompletion`;
- `drive_persistent_remote_worker_collection`;
- historical infallible `RemoteSessionWorkerAdmission<D, T>` uses;
- historical infallible spawned/supervised worker paths;
- historical persistent tests except narrowly additive sibling tests;
- repeated real-admission supervisor typing.

C03e-PK does not select converting the historical completion wrapper into a generic public type because that would create unnecessary public/API churn and ripple into later historical paths.

## 15. Explicitly deferred downstream graph

C03e-PK does not select or authorize:

1. repeated real-admission active-worker type migration;
2. `spawn_registered_worker` fallible migration;
3. `record_finished_worker` migration;
4. `PersistentSupervisorExit` completion-vector migration;
5. `drive_repeated_real_remote_admission_collection` migration;
6. expected-device request-carrier migration;
7. admission timing changes;
8. expected-device request producer changes;
9. concrete verifier-time provider installation;
10. production caller invocation;
11. endpoint lifecycle/startup/readiness wiring;
12. requester/rendezvous mutation;
13. listener/network activation;
14. merge or deployment.

Those are later separately gated compatibility/production boundaries.

## 16. C03e-PK repository scope

This selection checkpoint itself must remain documentation-only.

Expected PK repository scope:

- one new contract file only;
- zero Rust/source changes;
- zero Cargo/lock changes;
- zero workflow changes;
- zero Android changes;
- zero runtime behavior changes.

The PK contract is the only authority added by this checkpoint.

## 17. Validation law

After the PK contract commit and draft PR exist, exact-head CI must be enumerated.

For the exact PK head:

- Rust formatting/Clippy/tests/build must succeed if `PRW Rust Validation` is registered;
- every registered workflow must be reported with its real terminal conclusion;
- path-filtered `SKIPPED` workflows are not PASS;
- if Android is not registered for the docs-only exact head, record `NOT TRIGGERED`; do not inherit a PJ Android PASS.

No validation result from a different SHA may be used as PK evidence.

## 18. Immutable evidence law

After exact-head validation succeeds, freeze one immutable Markdown audit for C03e-PK.

Before upload:

- perform a fresh non-trashed exact-title Drive search;
- if an exact-title object already exists, do not blindly upload another copy; inspect/verify it first;
- upload exactly once only if no canonical object exists.

After upload:

- verify exact filename, MIME, byte size and canonical parent;
- raw-read back the Drive bytes;
- recompute and compare SHA-256;
- post-search exact title and require one canonical object;
- record the Drive receipt in the PR body without rewriting the frozen audit.

## 19. Closure and stop condition

C03e-PK closes only when:

- exact PJ predecessor remains unchanged;
- PK remains exactly one docs-only contract commit;
- exact-head validation succeeds according to registered workflows;
- immutable Drive evidence is byte-verified and unique at the canonical title;
- PK PR remains draft/open/unmerged;
- `main` remains unchanged;
- the later C03e-PL namespace remains empty.

After C03e-PK closes, stop at PK.

C03e-PL source materialization requires a fresh exact-head/source/concurrency audit in a separate continuation. It is not created or materialized by this checkpoint.
