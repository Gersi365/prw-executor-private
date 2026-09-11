# Desktop Functional Management Slice — C03e-QG Production Expected-Device Admission Fallible Verifier-Time Request Construction Composition Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_REQUEST_CONSTRUCTION_COMPOSITION_SELECTION`

Gate:

`C03E_QG_PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_REQUEST_CONSTRUCTION_COMPOSITION_SELECTED`

## 1. Authority and purpose

This checkpoint is documentation-only. It selects the smallest next independently materializable source seam after evidence-closed C03e-QF. It does not materialize Rust, construct a live expected-device request, create a channel, send a request, invoke the generic producer path, activate a runtime, or mutate an executable caller.

Authoritative predecessor:

- checkpoint: `C03e-QF`;
- branch: `phase-152-c03e-qf-production-durable-reachability-fallible-verifier-time-production-source-population-companion-composition-source-materialization`;
- exact head: `5475e1280a730abc09af1cc2d3834b325532641d`;
- exact tree: `d142ff12d86c5cdfb2c5a1a3f36e90f22911a338`;
- exact higher-owner source blob: `0121e25738de66932324ea7921c9905249c2155d`;
- PR #571 remains draft/open/unmerged;
- immutable QF evidence remains the closure authority.

Fresh integrated `main` at selection start remains:

- head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

Fresh successor search found no pre-existing `phase-152-c03e-qg-*` branch before this checkpoint was created.

## 2. Exact QF terminal seam

C03e-QF materially proves one dormant higher-owner helper:

`run_with_production_durable_reachability_remote_process_companion_with_fallible_verifier_time_completion_projection_from_production_sources(...)`

Its expected-request input remains:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

with:

- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`.

QF therefore consumes already-constructed request values. It does not receive a dispatcher and verifier-time provider as independent top-level arguments that can be installed after the request has entered the receiver. Concrete `D` and `T` provenance must be established at request construction before the request is enqueued.

This distinction is authoritative for QG: a wrapper that merely specializes the QF receiver type without constructing compatible requests would not establish a production producer seam.

## 3. Exact request carrier contract

At exact QF head, `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` already owns exactly:

1. target expected `DeviceId`;
2. target admission `SessionId`;
3. expected-device PRWM authentication request ID `u64`;
4. dispatcher `D`;
5. verifier-time provider `T`.

Its existing `new(...)` constructor is ownership-only and infallible after those five values exist.

QG does not alter this carrier, constructor, visibility, genericity, authentication flow, or real-admission semantics.

## 4. Construction prerequisites already materially present

The exact endpoint-owner source already contains the following dormant source seams.

### 4.1 Eligible continuation custody

`RemoteSessionExpectedDeviceAdmissionEligibleContinuation`

retains exactly:

- requester callback `DeviceId` correlation;
- one exact non-`Copy`/non-`Clone` `ExpectedDeviceSchedulingAuthorityGrant`;
- exact requester terminal acknowledgement result.

Only the existing live-completion classifier can produce this carrier from `SchedulingTerminal + Ok(grant)`.

### 4.2 Shutdown suppression

`map_remote_session_expected_device_admission_shutdown_suppression(...)`

already terminally disposes an eligible grant without opening it and emits the existing `SuppressedOnShutdown` receipt disposition. QG does not modify or reuse that mapper for normal construction.

### 4.3 Target admission SessionId source

`new_remote_session_expected_device_admission_target_session_id()`

is materially present and fail-closed. It performs exactly one independent 32-byte OS-backed CSPRNG fill, exact lowercase-hex encoding, and typed `SessionId` construction. It does not retry.

### 4.4 Expected-device authentication request-ID source

`new_remote_session_expected_device_authentication_request_id()`

is materially present and fail-closed. It performs exactly one independent 8-byte OS-backed CSPRNG fill, one big-endian `u64` conversion, rejects zero, and does not retry.

### 4.5 Scheduling grant extraction

`ExpectedDeviceSchedulingAuthorityGrant::into_parts()` is materially present and consumes the exact one-shot grant by value into:

`(requester_scheduling_session_id, target_device_id)`.

The requester scheduling `SessionId` is provenance only and is not the future target admission `SessionId`.

## 5. Fallible verifier-time compatibility now proven end-to-end

The exact current PRWA verifier source exposes:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds() -> Result<u64, PrwaVerifierSourceError>`.

The C03e-P/Q lineage propagated that exact fallible call-time provider shape through:

- capability request processing;
- supervised worker execution;
- repeated admission;
- endpoint lifecycle;
- production reachability;
- Linux operation construction;
- higher-owner operation construction;
- companion assembly;
- QF production-source population composition.

At exact QF, the function's signature is directly compatible with the required request-carried `T` bound. No panic/default/saturation/frozen-time adapter, error erasure, pre-sampling, caching, retry or timing-interface widening is required.

QG therefore selects the existing PRWA wall-clock source itself as the concrete expected-device request verifier-time provider.

The future construction seam must install it as a function pointer/provider value. It must **not call or sample it during request construction**. Verifier time remains call-time authority inside the existing lower authentication/capability lifecycle.

## 6. Dispatcher state remains separately gated

The exact repository materially contains the crate-private status-only dispatcher:

`LinuxAgentProductionRemoteCapabilityDispatcher`.

Its constructor requires one `LocalAgentStatusSnapshot`, and its `CapabilityDispatcher` implementation supports only `AgentStatus`; all provider-backed command families fail with the existing zero-data unsupported-provider classification.

However, no exact-current production seam proves which live `LocalAgentStatusSnapshot` must be captured for one expected-device request producer, nor where that dispatcher should be constructed and transferred into the producer.

QG therefore does **not** invent a production status snapshot or silently instantiate the NB dispatcher.

The future request-construction helper remains generic over caller-supplied `D: CapabilityDispatcher + Send + 'static`.

Concrete NB dispatcher construction/transfer is a later separately gated boundary.

## 7. Selected future source boundary

The next separately gated source checkpoint is selected as:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_REQUEST_CONSTRUCTION_COMPOSITION_SOURCE_MATERIALIZATION`

Immediate hard source ceiling:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exactly one Rust path may change.

If correct materialization requires a second Rust path, Cargo/lockfile mutation, visibility widening, `linux_bootstrap.rs` mutation, a new public API, channel/sender creation, producer invocation, or caller migration, the source checkpoint must STOP and return to selection.

## 8. Selected future concrete verifier-time type

The future source may define one private type alias solely to make the concrete request type explicit:

`type RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource = fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>;`

The exact provider value must be:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds`

coerced to that function-pointer type.

No wrapper that changes behavior is selected.

## 9. Selected future constructed-handoff custody

Successful request construction must preserve the values needed for later handoff-result composition without putting authority back into the one-shot grant.

The future source may add one private non-`Copy`/non-`Clone` carrier equivalent to:

`RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>`

owning exactly:

- requester callback `DeviceId` correlation;
- exact requester acknowledgement result;
- exactly one `RemoteSessionExpectedDeviceAdmissionRequest<D, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource>`.

It must not own:

- an `ExpectedDeviceSchedulingAuthorityGrant`;
- requester scheduling `SessionId` as reusable authority;
- a sender/channel;
- a retry token;
- endpoint/runtime authority;
- a second dispatcher or verifier-time provider.

The carrier exists only so a later send checkpoint can map terminal send disposition into `Enqueued` or `ChannelClosed` while preserving requester correlation and acknowledgement disposition.

## 10. Selected future construction outcome

The future source may add one private two-family outcome equivalent to:

`RemoteSessionExpectedDeviceAdmissionRequestConstructionOutcome<D>`

with exactly:

- `Constructed(RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>)`;
- `ConstructionFailed(RemoteSessionExpectedDeviceAdmissionHandoffReceipt)`.

The existing receipt disposition `ConstructionFailed` is the only selected terminal projection for target-SessionId or authentication-request-ID source failure.

No raw randomness failure, generated bytes, identifier candidate, grant contents, dispatcher state, verifier-time value or retry authority may appear in the receipt.

## 11. Selected future construction order

The future helper must preserve this exact order.

### Step 1 — consume eligible continuation as one operation

Receive exactly one `RemoteSessionExpectedDeviceAdmissionEligibleContinuation` plus exactly one caller-supplied dispatcher `D`.

Do not clone/copy/reconstruct the continuation or scheduling grant.

### Step 2 — generate target admission SessionId before opening grant

Invoke `new_remote_session_expected_device_admission_target_session_id()` exactly once.

On failure:

- do not invoke the authentication request-ID source;
- do not call scheduling-grant `into_parts()`;
- terminally dispose the still-sealed one-shot grant by value;
- preserve requester correlation and exact acknowledgement result;
- return exactly one existing `EligibleTerminal` receipt with disposition `ConstructionFailed`;
- perform no retry, replacement SessionId generation, scheduling rollback, grant remint or replay.

### Step 3 — generate authentication request ID before opening grant

Only after target SessionId success, invoke `new_remote_session_expected_device_authentication_request_id()` exactly once.

On failure:

- do not call scheduling-grant `into_parts()`;
- terminally dispose the still-sealed one-shot grant by value;
- preserve requester correlation and exact acknowledgement result;
- return exactly one existing `EligibleTerminal` receipt with disposition `ConstructionFailed`;
- perform no redraw, zero replacement, SessionId retry, scheduling rollback, grant remint or replay.

### Step 4 — bind concrete verifier-time provider without sampling

Bind the exact function pointer to `current_prwa_verifier_unix_seconds`.

Do not call it, sample wall-clock time, cache a timestamp, preflight it, replace failure with a default, or convert it into an infallible closure.

### Step 5 — open the one-shot scheduling grant exactly once

Only after both identifier sources have succeeded, call `ExpectedDeviceSchedulingAuthorityGrant::into_parts()` exactly once.

The returned target `DeviceId` becomes the request's exact expected-device identity.

The returned requester scheduling `SessionId` is consumed/discarded as scheduling provenance. It must not become the target admission SessionId, authentication request ID, request correlation, dispatcher input, verifier-time input, channel key, retry key, or receipt authority.

No target identity may be sourced from requester callback `DeviceId`.

### Step 6 — construct exactly one request

Invoke `RemoteSessionExpectedDeviceAdmissionRequest::new(...)` exactly once with:

1. target expected `DeviceId` from the consumed grant;
2. fresh target admission `SessionId` from the existing C03e-OX source;
3. fresh nonzero authentication request ID from the existing C03e-OZ source;
4. exact caller-supplied dispatcher `D` by value;
5. exact concrete fallible verifier-time function pointer.

The existing constructor is infallible once these inputs exist; no post-grant construction fallback is selected.

### Step 7 — retain only post-construction handoff custody

Return exactly one constructed-handoff carrier owning requester correlation, acknowledgement result and the constructed request.

The one-shot scheduling grant no longer exists after successful construction.

## 12. Selected future helper shape

The future source may add one private synchronous helper equivalent to:

`construct_remote_session_expected_device_admission_request_with_fallible_verifier_time<D>(continuation, dispatcher) -> RemoteSessionExpectedDeviceAdmissionRequestConstructionOutcome<D>`

with:

`D: CapabilityDispatcher + Send + 'static`.

The helper must be synchronous because it performs only local ownership composition and CSPRNG acquisition. It must not create a future, await, spawn, block, access Tokio runtime state, or touch a channel.

## 13. Failure and terminal-disposition law

Construction failure means only failure to obtain one of the two independently selected fresh identifier inputs before grant opening.

For either construction failure:

- the scheduling grant is terminally consumed/disposed by ownership loss without field extraction;
- acknowledgement result is preserved exactly;
- requester correlation is preserved exactly;
- dispatcher `D` may be dropped normally with the failed operation; it is not returned as retry custody;
- receipt disposition is exactly `ConstructionFailed`;
- no retry or alternate request is authorized.

`ConstructionFailed` does not mean authentication failure, admission rejection, capability transaction failure, channel closure, receiver consumption failure, endpoint failure or reachability failure.

## 14. Success is not enqueue

`Constructed(...)` means only that one typed expected-device request now exists under local ownership.

It does not mean:

- the request was enqueued;
- the receiver observed it;
- the target authenticated;
- a worker was admitted;
- capability authorization succeeded;
- requester acknowledgement succeeded;
- endpoint or reachability succeeded.

The existing `Enqueued` disposition remains reserved for a later successful `sender.send(request).await` result.

The existing `ChannelClosed` disposition remains reserved for a later failed async send due to receiver closure.

## 15. Preserved producer/channel law

QG does not create or select a new queue design.

The previously selected production handoff law remains:

- exactly one bounded Tokio MPSC channel;
- capacity exactly `1`;
- exactly one higher-owned production sender;
- receiver created once and moved once;
- enqueue only through `sender.send(request).await`;
- a full channel means asynchronous backpressure;
- no sender clone;
- no `try_send`;
- no `blocking_send`;
- no callback `block_on`;
- no hidden/detached producer task;
- no alternate/unbounded/retry queue;
- no second producer future.

QG does not materialize channel construction, sender custody, producer closure, send behavior, or concrete receipt specialization.

## 16. Preserved identity and authority separation

The future construction seam must preserve:

- requester callback `DeviceId` is requester-side authenticated correlation only;
- target expected `DeviceId` comes only from the consumed scheduling grant;
- requester scheduling `SessionId` is not target admission `SessionId`;
- target admission `SessionId` comes only from the independent C03e-OX source;
- expected-device PRWM authentication request ID comes only from the independent C03e-OZ source;
- PRWC/requester/terminal-ack correlation is not reused;
- verifier time remains call-time server authority, not identity or request ID;
- dispatcher is capability execution custody, not identity/scheduling authority.

## 17. Preserved shutdown law

Explicit supervisor shutdown remains the sole endpoint-supervisor shutdown authority.

The existing `SuppressedOnShutdown` mapper remains the only selected shutdown-recovered eligible-grant terminal path and must not be changed by the request-construction source checkpoint.

Channel closure remains a later terminal handoff disposition and is not a supervisor-shutdown signal.

## 18. Immediate future source exclusions

The future one-file source materialization must not:

- construct `LinuxAgentProductionRemoteCapabilityDispatcher`;
- select a `LocalAgentStatusSnapshot` production provenance;
- mutate `linux_bootstrap.rs`;
- create an expected-request channel;
- create or clone a sender;
- send a request;
- invoke/specialize the generic cooperative producer path;
- call QF directly;
- mutate the QF higher-owner source;
- sample verifier time during construction;
- widen fallible verifier time to infallible;
- call `SessionAuthenticationService::begin_session(...)`;
- retry SessionId generation or request-ID generation;
- remint/replay/rollback a scheduling grant;
- use requester callback `DeviceId` as target identity;
- reuse requester scheduling `SessionId` as target admission SessionId;
- change endpoint close/wait-idle ordering;
- modify executor/lower cooperative driver behavior;
- activate listener/readiness/process-signal/runtime/network behavior;
- mutate `run()` or `main.rs`;
- mutate Cargo/lockfile/workflows/Android source;
- change service/systemd/package/credential/certificate/private-key/trust/RBAC/DB/schema/control-plane/auth/repository configuration;
- merge, deploy, restart or recover production state.

## 19. Later separately gated dependencies

After successful source materialization of this selected seam, still separately gated are:

1. concrete NB status-only dispatcher production snapshot provenance and construction/transfer;
2. actual bounded capacity-one expected-request channel construction and sole-sender higher-owner custody;
3. producer closure that classifies one completion, constructs the request with the concrete dispatcher, and performs `sender.send(request).await`;
4. exact `Enqueued` and `ChannelClosed` receipt composition;
5. specialization/invocation of the existing generic cooperative producer path with the concrete receipt;
6. higher process/runtime caller migration connecting the production producer receiver to QF;
7. executable caller composition;
8. listener/readiness/process-signal/runtime activation;
9. deployment.

No ordering beyond the dependencies proved by this selection is inferred for unrelated later product work.

## 20. QG repository ceiling

QG itself may change only this contract path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_QG_PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_REQUEST_CONSTRUCTION_COMPOSITION_SELECTION_STAGING.md`

Zero Rust/source/runtime/workflow/manifest/lockfile/Android/packaging/executable changes are permitted in QG.

## 21. Validation and closure requirements

QG may be classified closed only if all of the following hold on the exact final QG head:

- QF -> QG is ahead-only with exact QF merge base;
- exactly one documentation path changed;
- PR remains draft/open/unmerged on exact QF base and exact QG head;
- exact-head Rust validation is terminal `SUCCESS` before any PASS claim;
- path-filtered workflows are recorded as `SKIPPED`, not PASS;
- no Android PASS is claimed unless an exact-head Android workflow actually runs and succeeds;
- integrated `main` remains unchanged;
- successor namespace is re-audited before closure;
- one immutable QG audit is frozen, uploaded to the canonical Drive parent, raw-read back, byte/hash verified, and exact-title uniqueness verified.

## 22. STOP

After QG closure: **STOP**.

Do not create the future source-materialization successor inside QG closure. A fresh exact-head/concurrency audit is mandatory before assigning the successor token or mutating the selected Rust path.
