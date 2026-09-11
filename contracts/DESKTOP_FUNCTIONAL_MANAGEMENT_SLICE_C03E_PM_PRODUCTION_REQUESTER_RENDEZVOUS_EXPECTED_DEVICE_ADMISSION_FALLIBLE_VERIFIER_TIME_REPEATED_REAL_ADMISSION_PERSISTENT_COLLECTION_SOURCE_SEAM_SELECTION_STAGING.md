# Desktop Functional Management Slice C03e-PM — Production Requester/Rendezvous Expected-Device Admission Fallible Verifier-Time Repeated Real-Admission Persistent Collection Source-Seam Selection

Status: `SELECTION — SOURCE MATERIALIZATION DEFERRED`

Selected later boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_REPEATED_REAL_ADMISSION_PERSISTENT_COLLECTION_SOURCE_MATERIALIZATION`

## 1. Exact predecessor

C03e-PM is a documentation-only selection checkpoint whose authoritative predecessor is closed C03e-PL.

PL branch:

`phase-152-c03e-pl-production-requester-rendezvous-expected-device-admission-fallible-verifier-time-persistent-worker-collection-result-ownership-source-materialization`

Exact PL head:

`a4c7aec56256ae741ea2193c318e531e4d73e5e1`

Exact PL tree:

`2f508048bc4759521ec760b475e6eb05e3740801`

Exact PL executor source blob:

`dbf308cf27b981e7509c7f37e2cb1a51ec04e109`

Closed PL PR:

`#551 — C03e-PL: materialize fallible verifier-time persistent worker collection result ownership`

PL remains draft/open/unmerged. Its administrative closure does not merge or activate the source.

Canonical immutable PL audit:

- Drive ID: `1L1CXzdYlL2jmoNyEqoc7iBX_Nl4IOGMz`
- filename: `C03E_PL_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_PERSISTENT_WORKER_COLLECTION_RESULT_OWNERSHIP_SOURCE_MATERIALIZATION_AUDIT_2026-09-11.md`
- canonical parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`
- MIME: `text/markdown`
- bytes: `16020`
- SHA-256: `165bad557f6d12452df52788f8d3a566149b6d57360ffc2ff1c68a653a4b2f55`

## 2. Fresh exact-source observation

The exact PL executor source proves that the generic persistent worker collection and the generic expected-device request carrier are already capable of carrying arbitrary worker-terminal and verifier-provider types.

Already generic and therefore not selected for replacement:

- `RemoteSessionPersistentWorkerEntry<T>`;
- `reap_ready_persistent_workers<K, T, C>`;
- `run_persistent_worker_collection<K, Candidate, T, ...>`;
- `RemoteSessionWorkerAdmission<D, T>`;
- `RemoteSessionWorkerAdmissionRejection<D, T>`;
- `RemoteSessionExpectedDeviceAdmissionRequest<D, T>`;
- `RemoteSessionExpectedDeviceAdmissionRejection<D, T>`;
- `prepare_expected_request<D, T, V, F, R>`;
- shutdown/request and shutdown/in-flight-admission polling helpers.

The exact PL source contains no `PersistentSupervisorExit` type. No such stale continuity concept is used as PM authority.

## 3. Immediate fixed compatibility gap after PL

The repeated real-admission supervisor remains fixed to the historical infallible worker terminal/provider at these local surfaces:

- `ActiveRemoteWorkers = HashMap<DeviceId, RemoteSessionPersistentWorkerEntry<AuthenticatedRemoteSessionWorkerStop>>`;
- `reap_registered_workers` reports `RemoteSessionRegisteredWorkerCompletion`;
- `drain_registered_workers` reports `RemoteSessionRegisteredWorkerCompletion`;
- `drain_inflight_admission` reports `RemoteSessionRegisteredWorkerCompletion` while retaining an in-flight AJ future;
- `spawn_registered_worker` requires `T: FnMut() -> u64 + Send + 'static` and invokes `run_capability_request_worker`;
- `drive_repeated_real_remote_admission_collection` requires `T: FnMut() -> u64 + Send + 'static` and `C: FnMut(RemoteSessionRegisteredWorkerCompletion)`.

C03e-PL already supplies the exact fallible completion sibling:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

and the fallible persistent worker terminal:

`AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop`.

Therefore the immediate next compatibility gap is the repeated real-admission collection itself, not the endpoint lifecycle, request carrier, verifier provider installation, or production caller.

## 4. Historical decomposition confirmation

Historical C03e-AK/C03e-AL selected/materialized repeated real admission plus persistent collection as one supervisor layer.

Historical C03e-AM/C03e-AN then selected/materialized endpoint shutdown lifecycle separately.

PM preserves that decomposition for the fallible verifier-time migration:

- PM selects only repeated real-admission persistent-collection compatibility;
- a later PN may materialize that compatibility;
- endpoint lifecycle compatibility remains a later separately gated selection/materialization;
- production producer/provider/caller activation remains further deferred.

Historical PR-number continuity is not itself authority; exact PL source and fresh GitHub title verification are authority.

## 5. PM selection decision

C03e-PM selects one later dormant fallible verifier-time repeated-real-admission persistent collection sibling.

Conceptual public surface:

`RemoteSessionExecutorRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_collection`

Maximum selected visibility should match the existing repeated real-admission collection surface required by its same-module/later composition. A later PN must not widen visibility beyond what exact source composition requires.

The historical `drive_repeated_real_remote_admission_collection` remains untouched.

## 6. Selected later source ceiling

A later C03e-PN source materialization may modify exactly one existing Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No other Rust path is selected.

No contract, Cargo manifest, lockfile, workflow, Android application source, transport implementation, requester/rendezvous module, database, authentication, authorization, endpoint startup, `main.rs`, readiness, packaging, host or deployment path is selected.

Narrow same-file tests/documentation strictly necessary to prove the selected seam are allowed only in the same Rust path.

## 7. Selected fallible active-worker custody

The future PN implementation must preserve a separate fallible active-worker map type equivalent to:

`HashMap<DeviceId, RemoteSessionPersistentWorkerEntry<AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop>>`

The authenticated logical `DeviceId` remains the sole active-worker key.

No transport identity, expected pre-authentication `DeviceId`, session ID, request ID, timestamp or verifier sample becomes active-worker authority.

The existing generic persistent entry remains unchanged.

## 8. Selected completion/reap law

The future fallible repeated supervisor must report active-worker completion through:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

preserving:

- authenticated logical `DeviceId`;
- exact `Result<AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

Normal worker terminals remain normal completion values:

- `Cancelled`;
- `Failed(exact_PB_error)`.

Only abnormal Tokio task completion remains the existing bounded `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

No string projection, raw `JoinError`, panic payload, task ID, synthetic cancellation, retry outcome or collection-level fatal conversion is selected.

Future fallible reap/drain helpers should be narrow siblings over the already-generic `reap_ready_persistent_workers`; they must not duplicate or reimplement the generic join-polling algorithm.

## 9. Selected per-admission worker spawn law

A later PN must add a fallible verifier-time sibling of the local historical `spawn_registered_worker` behavior.

Conceptual behavior:

1. receive one existing `RemoteSessionWorkerAdmission<D, T>`;
2. clone shared-current authority exactly once;
3. consume session owner, dispatcher and verifier-time provider by value;
4. create exactly one existing cancellation controller/signal pair;
5. create exactly one `tokio::spawn(async move { ... })`;
6. directly invoke `run_fallible_verifier_time_capability_request_worker` exactly once;
7. pass `cancellation_signal.into_cancelled()` unchanged;
8. return one `RemoteSessionPersistentWorkerEntry<AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop>` retaining the same controller/handle custody.

Selected provider bound:

`T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`

The spawned task must not call any synchronous executor compatibility bridge:

- not PF borrowed fallible drive;
- not PH spawned fallible drive;
- not PJ supervised fallible drive;
- not PL persistent synchronous drive.

Direct async PD-worker delegation avoids nested private-runtime entry and preserves repeated-supervisor cancellation/join custody.

## 10. Selected repeated collection signature law

Conceptual future sibling:

```rust
pub fn drive_repeated_real_fallible_verifier_time_remote_admission_collection<
    P,
    D,
    T,
    S,
    F,
    C,
    R,
    E,
>(
    &mut self,
    max_active_workers: NonZeroUsize,
    transport_runtime: &AgentRemoteTransportRuntime,
    authority: &SharedCurrentCapabilityAuthority<P>,
    session_authentication: &mut SessionAuthenticationService,
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    supervisor_shutdown: S,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<(), RemoteSessionPersistentCollectionConfigError>
```

Selected semantic bounds:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`;
- `S: Future<Output = ()> + Send`;
- `F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming`;
- `C: FnMut(RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion)`;
- `R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>)`;
- `E: FnMut(RemoteSessionRepeatedAdmissionFailure)`.

No extra `Clone`, `Copy`, `Default`, `Unpin`, unrelated `Sync`, or other widening is selected.

## 11. Expected-device request law

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>` is already generic and remains unchanged.

The request continues to own:

- expected pre-authentication logical `DeviceId` used only for scheduling/preflight;
- session ID;
- authentication request ID;
- dispatcher;
- verifier-time provider `T`.

PM does not select a new request-carrier type.

Duplicate expected-device rejection must continue before timing sampling, AJ/network work, or ownership-consuming worker spawn. The untouched rejected request must retain the exact dispatcher and fallible verifier-time provider.

Expected pre-authentication `DeviceId` remains scheduling intent only; authenticated owner-derived `DeviceId` remains post-auth worker authority.

## 12. Admission timing and AJ law

The future fallible repeated supervisor must preserve the existing AJ transaction behavior unchanged.

`RemoteSessionRealAdmissionTiming` remains the sole per-attempt timing bundle for:

- challenge validity;
- authentication-now;
- application lease.

Those timing inputs are distinct from the worker's fallible verifier-time provider.

The verifier-time provider `T` must not be sampled during preflight or AJ.

At most one AJ future remains in flight at a time.

No retry, reconnect, replacement, parallel pre-authentication attempt, detached producer or fallback AJ is selected.

## 13. Ordinary AJ success law

On ordinary AJ success before supervisor shutdown:

1. derive the authenticated logical `DeviceId` from the returned authenticated session owner;
2. retain the existing debug assertion that the authenticated identity matches the expected scheduling identity;
3. construct the existing generic `RemoteSessionWorkerAdmission::new(session_owner, dispatcher, verifier_time_provider)`;
4. require a vacant authenticated `DeviceId` slot under the existing single-in-flight preflight invariant;
5. spawn exactly one fallible verifier-time worker through the PN-selected local spawn sibling;
6. retain that worker in the fallible active map.

No verifier-time sample occurs before the worker itself requests one through PD/PB.

## 14. Ordinary AJ failure law

An ordinary AJ failure remains reported through the existing:

`RemoteSessionRepeatedAdmissionFailure`

with:

- expected scheduling `DeviceId`;
- exact existing `RemoteSessionRealAdmissionError`.

AJ failure does not terminate the repeated supervisor, fabricate a worker completion, create a retry, or convert the error into verifier-time failure.

The unused dispatcher/verifier provider for the failed attempt is not moved into a worker.

## 15. Shutdown and in-flight AJ law

The existing repeated-supervisor ordering remains authoritative.

Ready active-worker completions are reaped before shutdown/request work.

Supervisor shutdown continues to win before a new request or against a pending in-flight AJ according to the existing poll helpers.

When shutdown occurs with no AJ in flight:

- request cancellation for every retained fallible worker;
- drain the same retained worker handles to terminal completion;
- report each completion through the fallible PL completion wrapper;
- return only after the active map is empty.

When shutdown occurs while AJ is in flight:

- request cancellation for every retained active fallible worker;
- retain and drain the same in-flight AJ future instead of dropping/aborting it;
- continue reaping fallible worker completions while AJ drains;
- if AJ drains to failure, report the exact existing AJ failure;
- if AJ drains to success after shutdown, call the existing authenticated-owner orderly-shutdown close seam and do not spawn/insert a worker;
- then drain all remaining active fallible workers.

Supervisor shutdown does not fabricate a fallible worker `Cancelled` terminal; real worker cancellation terminal remains produced by the worker path itself.

## 16. Capacity, duplicate and source-closure law

Existing capacity semantics remain unchanged:

- validate `NonZeroUsize` against `MAX_REGISTERED_DEVICES` before runtime work;
- active map length remains the sole active-worker capacity accounting source;
- expected-request source is not polled while full;
- duplicate expected scheduling `DeviceId` is rejected before timing/network work;
- post-authenticated active insertion still uses authenticated owner-derived `DeviceId`;
- request-source closure alone does not fabricate supervisor shutdown.

No second capacity counter or alternate identity index is selected.

## 17. Verifier-time failure law

The repeated supervisor does not inspect, sample or reinterpret verifier-time errors.

A fallible worker terminal:

`Failed(exact_PB_error)`

is reported as that worker's normal persistent completion and does not terminate/restart the repeated supervisor.

No retry, fallback, default timestamp, cache, clamp, saturation, stale timestamp reuse, error stringification or failure-to-cancellation conversion is selected.

PB/PD remain authoritative for verifier-time/request-processing classification and peer-close behavior.

## 18. Historical helper preservation

The future PN implementation should prefer additive fallible siblings for the small repeated-supervisor helpers that are fixed to historical worker/completion types.

Historical helpers and historical repeated collection behavior remain untouched.

The already-generic helpers should be reused unchanged rather than copied:

- request/shutdown poll helper;
- in-flight AJ/shutdown poll helper;
- `prepare_expected_request`;
- generic persistent worker entry;
- generic ready-worker reaper;
- cancellation pair constructor;
- capacity validator.

No broad genericization refactor of historical repeated supervisor code is selected merely to reduce duplication.

## 19. Endpoint lifecycle explicitly deferred

The existing historical:

`drive_repeated_real_remote_admission_endpoint_lifecycle`

remains untouched and continues to compose the historical infallible repeated collection.

PM does not select a fallible endpoint-lifecycle sibling.

A later checkpoint may assess endpoint lifecycle compatibility only after PN is separately materialized and validated.

Endpoint close code/reason, `wait_idle()` ordering, readiness, startup and listener activation are outside PM.

## 20. Production/provider graph explicitly deferred

PM does not select or authorize:

- a concrete production verifier-time provider;
- requester/rendezvous authority mutation;
- request-producer migration;
- production expected-device source wiring;
- endpoint bind/listener acceptance;
- endpoint lifecycle fallible composition;
- signal handling;
- readiness publication;
- `main.rs` integration;
- systemd/host mutation;
- deployment;
- merge.

The future PN sibling remains dormant until separately selected downstream composition reaches it.

## 21. Error and peer-close preservation

The repeated supervisor introduces no new error enum for normal fallible worker failure.

Existing collection configuration error remains:

`RemoteSessionPersistentCollectionConfigError`.

Existing AJ failure remains:

`RemoteSessionRealAdmissionError` through `RemoteSessionRepeatedAdmissionFailure`.

Existing abnormal worker join mapping remains:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

Worker `Failed(exact_PB_error)` remains inside the fallible worker completion result.

No repeated-supervisor layer performs a direct peer close for verifier-time failure. PD/PB remain close authority.

## 22. Selected later validation law

A later PN materialization must be validated only on its exact final head.

Minimum acceptance:

- final PL -> PN compare has exactly the selected Rust path and no unrelated historical hunks;
- Rust formatting passes;
- Clippy passes;
- workspace tests pass;
- workspace build passes;
- Android validation is reported according to actual exact-head workflow state;
- path-filtered workflows are reported as `SKIPPED`, not PASS, when skipped;
- source blob/tree/head are re-read after validation;
- PR remains draft/open/unmerged;
- immutable Drive evidence is frozen, uploaded once, raw-readback verified and uniquely titled;
- successor namespace remains empty at closure.

Any formatting/lint corrective commit must remain source-local and preserve full forward history; no force push, squash, rebase or history rewrite.

## 23. PM explicit non-actions

C03e-PM does not:

- modify Rust/source;
- materialize C03e-PN;
- modify PL source;
- genericize historical repeated helpers;
- change the existing expected request/rejection carrier;
- change AJ timing;
- change authentication;
- install a verifier-time provider;
- add retry/fallback/cache/default/clamp/saturation;
- add a fallible endpoint lifecycle;
- activate a listener or network endpoint;
- change startup/readiness;
- modify `main.rs`;
- change dependencies, database or security architecture;
- merge any PR;
- deploy;
- mark a draft PR ready;
- delete branches;
- force-push, squash, rebase or rewrite history.

## 24. Stop rule

C03e-PM closes only a selection decision.

After PM evidence is published and its draft PR is administratively marked:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

stop at PM.

C03e-PN source materialization is a separate checkpoint requiring a fresh exact-head, source, concurrency and evidence audit before any Rust mutation.
