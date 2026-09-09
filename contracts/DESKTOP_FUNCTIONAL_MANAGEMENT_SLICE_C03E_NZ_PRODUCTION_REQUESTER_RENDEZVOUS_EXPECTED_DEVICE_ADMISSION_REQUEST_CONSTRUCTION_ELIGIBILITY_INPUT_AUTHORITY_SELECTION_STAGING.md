# Phase 152 C03e-NZ — Production requester/rendezvous expected-device admission-request construction eligibility and input-authority selection

Status: `STAGING — SELECTION ONLY — SOURCE MATERIALIZATION BLOCKED`

Gate reserved for closure:
`C03E_NZ_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CONSTRUCTION_ELIGIBILITY_INPUT_AUTHORITY_SELECTED`

Closure token reserved for validated evidence-backed closure:
`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CONSTRUCTION_ELIGIBILITY_INPUT_AUTHORITY_SELECTION`

## 1. Exact predecessor authority

This documentation-only checkpoint is rooted only at exact closed C03e-NY.

- exact C03e-NY head: `25a95b28dec3b87a229a58062a7bfbea29f57775`
- exact C03e-NY tree: `8a9d54334351d9cdf039a70361f6ccff9dda095b`
- exact C03e-NY PR: `#513`, draft/open/unmerged, checkpoint-local CLOSED
- no C03e-NZ branch or PR existed at the fresh pre-creation concurrency check

No blocked or superseded branch is adopted as authority.

## 2. Purpose

C03e-NY closed the higher-owner custody and peer-disposition path for the one-shot expected-device scheduling result. It intentionally stopped before expected-device admission-request construction, request-channel send, target admission `SessionId` production, authentication PRWM request-ID production, dispatcher/verifier-time production custody, or runtime activation.

C03e-NZ selects only the semantic eligibility and field-authority law for a later expected-device admission-request construction attempt. It does not materialize Rust source.

The question answered here is narrower than sender/channel ownership or runtime wiring:

> Given one C03e-NY scheduling-aware terminal result, when is one existing `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` construction semantically eligible, which existing identity is authoritative for each already-proven field, and which constructor inputs remain unresolved production dependencies?

## 3. Fresh exact-NY source observations

### 3.1 Scheduling authority carrier

Exact path:
`crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`

Exact NY blob:
`e6d030a33f291ec78e2ddaf83bc9888711e3c4bb`

`ExpectedDeviceSchedulingAuthorityGrant` is an intentionally non-`Copy`, non-`Clone` one-shot carrier owning exactly:

- requester `SessionId`;
- target logical `DeviceId`.

It explicitly owns no endpoint, transport identity, request ID, admission session ID, timing value, dispatcher, sender, candidate payload, or reachability authority.

The exact grant can be consumed by value through:
`into_parts(self) -> (SessionId, DeviceId)`.

The grant is constructed only after fresh requester/target authority checks and terminal scheduling-consumption insertion. Therefore no downstream stage may clone, reconstruct, remint, replace, or infer an equivalent grant.

### 3.2 Scheduling terminal result and acknowledgement channel

Exact path:
`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact NY blob:
`d1a68ea88d6721a622e0fd8ec54ef23ab136726f`

`RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome` owns two explicitly orthogonal channels:

1. `Result<ExpectedDeviceSchedulingAuthorityGrant, ExpectedDeviceSchedulingAuthorityDerivationError>`;
2. `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`.

The carrier intentionally owns no request channel, dispatcher, timing value, target admission session ID, authentication PRWM request ID, endpoint, or reachability authority.

`RequesterRendezvousProductionDurableSchedulingWorkerStop` preserves three terminal classes:

- `Cancelled` — no scheduling result exists;
- `Failed(...)` — lifecycle failed before a scheduling result exists;
- `SchedulingTerminal(...)` — requester/rendezvous DR succeeded and scheduling plus acknowledgement custody exists.

The scheduling-aware requester worker derives scheduling authority before acknowledgement framing/I/O and returns both channels upward before another cancellation poll or requester ingress cycle.

### 3.3 Higher-owner peer disposition does not rewrite scheduling authority

Exact path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact NY blob:
`7603b207b2efdd6c90a886c6e6f0abc232fa3bbc`

C03e-NY selected:

- cancellation -> existing orderly shutdown;
- pre-scheduling lifecycle failure -> existing requester-aware terminal failure;
- abnormal join -> existing requester-aware terminal failure;
- scheduling terminal + acknowledgement success -> existing orderly shutdown;
- scheduling terminal + acknowledgement failure -> existing requester-aware terminal failure.

Scheduling derivation success/failure does not control requester peer disposition.

This means peer disposition and scheduling-result custody remain distinct. Disposing the recovered requester peer does not erase, recreate, mutate, or reinterpret the already-produced scheduling result.

### 3.4 Scheduling-aware endpoint callback boundary

Exact path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Exact NY blob:
`eea51672c46aa83d50e6294a36e912fd0aa51928`

The dormant scheduling-aware endpoint callback receives only:

- authenticated active-map `DeviceId` for the completed requester-side worker; and
- exact `Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The recovered authenticated-session owner is consumed by the selected peer-disposition law before this callback is invoked.

The callback `DeviceId` is the authenticated requester-side worker identity. It is not the target expected-device identity and must not be substituted for `ExpectedDeviceSchedulingAuthorityGrant::target_device_id()`.

### 3.5 Existing expected-device admission-request primitive

Exact path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Exact NY blob:
`ef370ca500f118bc067097ddb8f5c37ab597b214`

The existing public request carrier is:
`RemoteSessionExpectedDeviceAdmissionRequest<D, T>`.

Its existing constructor requires exactly:

1. expected target `DeviceId`;
2. target admission `SessionId`;
3. authentication request ID (`u64`);
4. dispatcher `D`;
5. verifier-time provider `T`.

The constructor is already the selected receiver-compatible nominal shape. C03e-NZ selects no second request struct and no wrapper that bypasses this shape.

The repeated real-admission supervisor consumes this request, rejects duplicate expected target `DeviceId` before timing/admission, samples admission timing only for an attempt that can start, then transfers the request parts into the existing expected-device admission transaction.

### 3.6 Dispatcher and verifier-time custody are not returned by NY completion

Exact scheduling-aware persistent worker integration path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact NY blob:
`4d2389bfc5e671f44b23438e258d328d92484895`

For an admitted requester worker, `RemoteSessionWorkerAdmission<D, T>` is decomposed into:

- authenticated session owner;
- dispatcher;
- verifier-time provider.

The dispatcher and verifier-time provider move into the requester worker task. The recoverable completion path preserves authenticated owner plus worker result, but it does not return dispatcher or verifier-time custody.

Therefore the C03e-NY endpoint callback cannot construct a new `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` merely by consuming the scheduling grant. Two constructor dependencies (`D` and `T`) are absent from that callback boundary by design.

### 3.7 No separate production-durable scheduling-request authority exists in registry custody

Exact path:
`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact NY blob:
`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

This file owns durable-registry custody and durable capability authorization. It does not define an expected-device admission-request constructor, target admission session generator, authentication request-ID source, expected-request sender, or scheduling grant alternative.

No child `production_durable_registry_runtime_custody/expected_device_scheduling_authority.rs` or `expected_device_admission_request.rs` path exists at exact NY. Any earlier inferred child-module path is non-authoritative and rejected by direct exact-head readback.

## 4. Closed selection

C03e-NZ selects:

`EXISTING_REMOTE_SESSION_EXPECTED_DEVICE_ADMISSION_REQUEST_IS_EVENTUAL_CONSTRUCTION_SHAPE / SCHEDULING_TERMINAL_DERIVATION_SUCCESS_IS_THE_ONLY_CONSTRUCTION_ELIGIBILITY_AUTHORITY / ACKNOWLEDGEMENT_DISPOSITION_REMAINS_ORTHOGONAL_AND_DOES_NOT_REVOKE_OR_REMINT_THE_ONE_SHOT_GRANT / TARGET_DEVICE_ID_MUST_COME_ONLY_FROM_THE_CONSUMED_SCHEDULING_GRANT / REQUESTER_CALLBACK_DEVICE_ID_MUST_NOT_BE_SUBSTITUTED_FOR_TARGET_DEVICE_ID / REQUESTER_SESSION_ID_MUST_NOT_BE_SUBSTITUTED_FOR_TARGET_ADMISSION_SESSION_ID / TARGET_ADMISSION_SESSION_ID_AUTHENTICATION_REQUEST_ID_DISPATCHER_AND_VERIFIER_TIME_CUSTODY_REMAIN_UNRESOLVED / CURRENT_NY_COMPLETION_CALLBACK_IS_INSUFFICIENT_FOR_REQUEST_CONSTRUCTION / EXPECTED_REQUEST_SENDER_CHANNEL_REMAINS_UNRESOLVED / NO_SOURCE_MATERIALIZATION_SELECTED`

## 5. Construction eligibility partition

A future request-construction stage may become eligible only from this exact terminal class:

`Ok(RequesterRendezvousProductionDurableSchedulingWorkerStop::SchedulingTerminal(outcome))`

with:

`outcome.scheduling_result() == Ok(grant)`.

No other terminal class is construction-eligible.

### 5.1 Ineligible classes

The following produce no expected-device admission request:

- requester worker `Cancelled`;
- requester worker `Failed(...)`;
- abnormal requester worker join;
- `SchedulingTerminal` whose scheduling derivation result is `Err(...)`.

No retry, fallback, replacement grant, synthetic target, alternate policy path, or remint is selected for these classes.

### 5.2 Acknowledgement result is not scheduling-authority revocation

For `SchedulingTerminal`, acknowledgement disposition is a separate terminal channel.

C03e-NZ selects that an acknowledgement failure does not mutate the already-terminal scheduling-consumption ledger, does not convert `Ok(grant)` into a derivation error, and does not authorize a replacement grant. The exact `Ok(grant)` remains the sole scheduling-authority object for downstream eligibility.

This preserves the already-selected orthogonality and prevents an acknowledgement transport/framing failure from silently becoming a new scheduling-authority revocation mechanism.

This selection does not add any requester-visible response, retry, resend, or acknowledgement recovery behavior.

## 6. Exact identity mapping into the existing constructor

### 6.1 Expected target `DeviceId`

Future constructor field:
`expected_device_id`.

Selected authority:
`ExpectedDeviceSchedulingAuthorityGrant::target_device_id()` / the consumed grant's target `DeviceId`.

Forbidden substitutes:

- scheduling endpoint callback `DeviceId` (requester-side authenticated worker identity);
- requester `SessionId`;
- configured peer identity;
- transport identity;
- endpoint/IP/port;
- candidate/reachability state;
- request/correlation ID;
- post-auth identity from a different session.

### 6.2 Requester `SessionId`

The grant's requester `SessionId` remains provenance for the one-shot scheduling decision that was already authorized and terminally consumed.

It is not the future target admission `SessionId`.

C03e-NZ explicitly forbids passing the requester `SessionId` into `RemoteSessionExpectedDeviceAdmissionRequest::new(..., session_id, ...)` merely because both are nominally `SessionId`.

Doing so would collapse two distinct session roles and create an unaudited authentication identity reuse law.

### 6.3 Target admission `SessionId`

Required by the existing request constructor.

Current exact-NY scheduling grant: absent.
Current exact-NY scheduling endpoint callback: absent.
Current selected production source/custody: not proven.

Status: `UNRESOLVED — SEPARATELY GATED`.

### 6.4 Authentication request ID

Required by the existing request constructor as `u64`.

Current exact-NY scheduling grant: absent.
Current exact-NY scheduling endpoint callback: absent.
No prior PRWC correlation/request-ID lane may be silently reused as this expected-device authentication request-ID producer.

Status: `UNRESOLVED — SEPARATELY GATED`.

### 6.5 Dispatcher `D`

Required by the existing request constructor.

The requester-side dispatcher currently moves into the scheduling-aware worker and is not returned in NY completion custody. C03e-NZ does not select recycling that consumed requester dispatcher into a target admission request.

A future production owner/factory must prove exact dispatcher provenance and lifetime before construction.

Status: `UNRESOLVED — SEPARATELY GATED`.

### 6.6 Verifier-time provider `T`

Required by the existing request constructor.

The requester-side verifier-time provider currently moves into the scheduling-aware worker and is not returned in NY completion custody. C03e-NZ does not select recycling that consumed requester-side provider or inventing a replacement clock closure.

A future production owner/factory must prove exact verifier-time authority and lifetime before construction.

Status: `UNRESOLVED — SEPARATELY GATED`.

## 7. Sender/channel remains a separate prerequisite

Even a fully constructed `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` is not self-sending.

The scheduling-aware repeated admission endpoint currently accepts:
`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`.

No exact-NY production source audited here owns the matching expected-request sender lifecycle.

C03e-NZ therefore selects no:

- channel construction;
- sender clone policy;
- channel capacity;
- backpressure policy;
- send ordering;
- closed-channel classification;
- sender-drop shutdown law;
- runtime task responsible for sending.

## 8. No direct source successor is authorized

C03e-NZ intentionally does not select a Rust source-materialization successor because exact request construction still lacks four production inputs and sender/channel ownership.

Changing the existing NY completion envelope merely to compile a constructor would force unselected custody decisions for dispatcher/verifier-time and would not solve target admission `SessionId`, authentication request ID, or sender lifecycle.

Creating synthetic values, reusing requester session identity, reusing unrelated request correlation, cloning/reconstructing the one-shot grant, or adding a test-only sender in production code is forbidden.

## 9. Next blocking documentation boundary

After C03e-NZ is independently validated and evidence-closed, the next separately gated documentation boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CONSTRUCTION_DEPENDENCY_PROVENANCE_CUSTODY_SELECTION`

That future gate must audit and select, at minimum:

1. production target admission `SessionId` source, uniqueness/lifetime, and ownership;
2. authentication PRWM request-ID source, uniqueness/correlation scope, and ownership;
3. production dispatcher creation/custody for the target admission request;
4. verifier-time provider creation/custody for the target admission request;
5. whether those four dependencies are produced by one coherent factory/owner or by separately gated sub-stages;
6. exact relation between that producer and the C03e-NY scheduling terminal callback;
7. exact expected-request sender/channel owner, capacity, backpressure and shutdown law, or proof that sender ownership must remain a later separate gate;
8. exact smallest future Rust source ceiling;
9. proof no requester `SessionId` is reused as target admission `SessionId`;
10. proof no unrelated PRWC/candidate/request correlation ID is reused as the authentication request ID;
11. proof target `DeviceId` comes only from the consumed scheduling grant;
12. proof the one-shot grant is consumed exactly once and never cloned/reminted;
13. proof no listener/bootstrap/readiness/runtime activation is introduced merely to materialize construction dependencies.

The future dependency-selection gate remains documentation-only unless it separately proves a bounded materialization seam.

## 10. Explicit non-actions and frozen exclusions

C03e-NZ performs no Rust/Kotlin source mutation.

It does not:

- construct `RemoteSessionExpectedDeviceAdmissionRequest`;
- generate or select a target admission `SessionId`;
- allocate/reuse an authentication PRWM request ID;
- create or select a dispatcher;
- create or select a verifier-time source;
- create an expected-request channel;
- retain or send through an expected-request sender;
- add a background task;
- modify scheduling-consumption state;
- retry/remint/replay a scheduling grant;
- modify requester acknowledgement framing/I/O;
- alter peer-disposition close codes;
- clean requester records;
- continue candidate/reachability work;
- select a dial target;
- activate a listener/bootstrap/readiness path;
- mutate Cargo manifests or lockfiles;
- mutate workflows;
- mutate Android source/packaging;
- deploy/restart/recover services;
- merge;
- delete branches;
- change repository configuration, rulesets or permissions.

## 11. Validation requirement

Closure claims must bind only to the exact final C03e-NZ docs-only head.

At minimum:

- exact NY -> NZ compare must be ahead-only with exact NY merge base;
- exactly one changed path: this contract;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging changes;
- PRW Rust Validation must be observed on the exact final head and reported accurately;
- any Android workflow is reported only if actually observed on the exact final head;
- path-filtered `SKIPPED` workflows remain `SKIPPED`, never PASS;
- immutable raw evidence must be published and byte/hash read back before checkpoint closure metadata is written.

## 12. STOP condition

After evidence-backed C03e-NZ selection closure: `STOP`.

No construction-dependency source mutation may begin from this checkpoint. A fresh exact-head/concurrency audit is mandatory before the next documentation gate.
