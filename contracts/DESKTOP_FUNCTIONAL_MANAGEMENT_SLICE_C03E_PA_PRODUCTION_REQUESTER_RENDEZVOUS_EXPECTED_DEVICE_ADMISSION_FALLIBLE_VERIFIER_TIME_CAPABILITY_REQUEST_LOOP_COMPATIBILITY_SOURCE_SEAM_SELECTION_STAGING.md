# C03e-PA — Production requester-rendezvous expected-device admission fallible verifier-time capability-request loop compatibility source seam selection

## Status

`SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_CAPABILITY_REQUEST_LOOP_COMPATIBILITY_SOURCE_SEAM_SELECTION`

Selected future source boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_CAPABILITY_REQUEST_LOOP_COMPATIBILITY_SOURCE_MATERIALIZATION`

This checkpoint is documentation-only. It selects one additive source prerequisite and does not materialize, invoke, migrate, activate, merge, deploy, or publish runtime behavior.

## 1. Exact predecessor authority

Authoritative predecessor is exact closed C03e-OZ:

- head: `df2dee189cf5dc412b5c42ced28db2f9f3e1ff82`;
- tree: `bc08d9596c097a42fde2b212eaf6f5a60dd8158d`;
- PR #539 remains draft/open/unmerged;
- C03e-OZ endpoint-owner source blob: `e9f82f32ba875c2461513e445f81962720dc3298`.

Fresh source proof for this selection binds to the same exact C03e-OZ head and additionally reads:

- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`, blob `ef370ca500f118bc067097ddb8f5c37ab597b214`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`, blob `f95c4f1bb2d424ea7d15647ecb1d6153aebc480c`;
- `crates/prw-session/src/prwa_verifier_source.rs`, blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`.

Fresh C03e-PA namespace and semantic-equivalent guards were zero-result before branch creation. No earlier PA checkpoint or equivalent fallible-verifier-time capability-request seam was found.

## 2. Exact incompatibility proved by current source

The existing repeated real-admission request carrier is:

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>`

and owns exactly the expected target `DeviceId`, target admission `SessionId`, expected-device authentication request ID, dispatcher, and verifier-time provider `T`.

Its existing constructor accepts those values independently and `into_parts()` returns them unchanged by value.

After expected-device AJ success, the exact current supervisor moves `dispatcher` and `verifier_time_unix_seconds` into `RemoteSessionWorkerAdmission`, then `spawn_registered_worker(...)` moves them into the authenticated worker.

The exact current worker stack constrains verifier time as:

`T: FnMut() -> u64 + Send`

or the corresponding `'static` form where task ownership requires it.

The exact current `AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_loop(...)` samples the provider once per transaction with:

`let now_unix_seconds = verifier_time_unix_seconds();`

and therefore has no typed failure channel for verifier-time acquisition.

By contrast, the existing server-local PRWA verifier clock source is:

`current_prwa_verifier_unix_seconds() -> Result<u64, PrwaVerifierSourceError>`.

Its wall-clock conversion uses `SystemTime::now()` followed by checked duration since `UNIX_EPOCH`, and a pre-epoch/non-representable time returns `PrwaVerifierSourceError::VerifierTime` fail-closed.

Therefore the current infallible `FnMut() -> u64` worker interface cannot faithfully carry the existing production verifier-time failure semantics.

An adapter that erases the `Result` through `unwrap`, `expect`, panic, default-to-zero, saturation, frozen time, cached time, previous-time reuse, fabricated success, or any other fallback is explicitly rejected.

## 3. Why the immediate boundary is additive and one-file

Changing the request carrier, repeated supervisor, worker admission, executor wrappers, worker stop family, and authenticated request loop in one checkpoint would cross multiple Rust paths and multiple ownership/lifecycle layers.

That would combine compatibility selection, failure taxonomy, task propagation, request-carried custody, and runtime composition into one mutation.

C03e-PA instead selects the smallest independently materializable prerequisite: one dormant sibling fallible verifier-time capability-request loop in the existing authenticated-session source file.

Historical infallible methods remain untouched during the selected first materialization.

The later migration of the fallible result through worker, executor, repeated-admission request carrier, producer construction, and runtime caller remains separately gated.

## 4. Hard future source ceiling

The immediate C03e-PA-selected source materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

No second Rust path is authorized by this selection.

If correct compilation or semantics require mutation of `remote_session_executor_runtime.rs`, `remote_session_endpoint_lifecycle_runtime.rs`, `prwa_verifier_source.rs`, Cargo metadata, lockfiles, workflows, Android source, packaging, process lifecycle, or executable wiring, the source checkpoint must STOP and return to selection.

## 5. Selected fallible verifier-time callback law

The dormant sibling seam must accept a verifier-time provider whose semantic shape is equivalent to:

`T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send`

The exact existing `PrwaVerifierSourceError` must remain the preserved source-error type for this first compatibility boundary; the source stage must not convert it to a boolean, string, integer sentinel, panic, generic I/O failure, or fabricated transaction failure.

The provider is sampled exactly once immediately before each existing capability request transaction attempt.

A successful sample yields the exact `u64` unchanged to the existing one-request transaction.

A failed sample is terminal for that capability-session loop invocation and performs no transaction attempt for that iteration.

No verifier-time sample retry, redraw, second clock read, cached fallback, default, saturation, clamping, monotonic substitution, requester-provided time, transport-derived time, dispatcher-derived time, or time-from-ID derivation is selected.

## 6. Selected bounded loop failure representation

The source materialization may add one private or narrowly Agent-internal bounded failure representation equivalent to:

- `VerifierTime(PrwaVerifierSourceError)`; or
- `Transaction(AuthenticatedRemoteSessionCapabilityTransactionError)`.

The representation must preserve each underlying typed error unchanged by value.

No raw Tokio error, panic payload, process error, wall-clock object, request payload, credential, dispatcher, authority, endpoint, scheduling grant, sender, or retry token belongs in this error carrier.

The existing `AuthenticatedRemoteSessionCapabilityTransactionError` family remains unchanged in the first source materialization.

The existing `AuthenticatedRemoteSessionWorkerStop` family also remains unchanged in the first source materialization; worker-level propagation is a later separately gated boundary.

## 7. Exact close / fail-closed law

The sibling fallible loop must preserve the same retained-peer termination discipline as the historical capability request loop.

On verifier-time acquisition failure:

1. no control stream is accepted for a new transaction;
2. no capability request frame is read;
3. no authorization is evaluated;
4. no dispatcher is invoked;
5. no response is emitted;
6. the retained peer is closed exactly once using the existing fixed capability-session termination diagnostic owned by this module;
7. the exact verifier-time source error is returned under the bounded fallible-loop failure representation.

On an existing capability transaction failure:

1. the retained peer is closed exactly once using the same existing capability-session termination diagnostic;
2. the exact existing transaction error is returned under the bounded fallible-loop failure representation;
3. no retry, replacement stream, replacement session, fallback dispatcher, or fabricated response occurs.

The source stage must not introduce a different close code merely because verifier-time sampling failed.

## 8. Historical compatibility law

The existing infallible `run_capability_request_loop<T: FnMut() -> u64>` remains source- and behavior-preserved during the first materialization.

The existing `run_capability_request_worker(...)`, executor wrappers, repeated supervisor, request carrier, endpoint lifecycle, and producer-forwarding methods remain untouched.

The selected sibling must be dormant: no current production caller is migrated to it in the first source checkpoint.

No public API widening is selected solely to expose this seam outside the minimum existing Agent-internal ownership boundary.

If compiler visibility requires broader exposure than the authenticated-session/executor relationship can justify, STOP and return to selection.

## 9. Existing PRWA verifier source remains authoritative and unchanged

`crates/prw-session/src/prwa_verifier_source.rs` remains the existing server-local wall-clock source authority for this lane.

C03e-PA does not modify `current_prwa_verifier_unix_seconds()` or `PrwaVerifierSourceError`.

C03e-PA does not reuse `PrwaVerifierSessionContext` as the expected-device request carrier. That context also owns a separate verifier session `SessionId` and challenge lifetime, which are outside this compatibility boundary.

Only the existing fallible verifier-time observation/error semantics are relevant to this selection.

The future runtime may supply the existing clock function through the selected fallible callback seam, but that invocation/wiring is not materialized by C03e-PA selection.

## 10. Construction and custody separation preserved

C03e-OX target-admission `SessionId` generation remains a separate completed source seam.

C03e-OZ expected-device PRWM authentication request-ID generation remains a separate completed source seam.

The one-shot `ExpectedDeviceSchedulingAuthorityGrant` remains sealed until a later construction/custody checkpoint.

C03e-PA does not call `ExpectedDeviceSchedulingAuthorityGrant::into_parts()` and does not inspect, open, extract, dispose, clone, copy, reconstruct, remint, replay, refund, or roll back that grant.

Requester callback `DeviceId` remains requester-side authenticated correlation and is not the target expected `DeviceId`.

Target expected `DeviceId` remains obtainable only from the consumed construction-eligible scheduling grant.

Requester scheduling `SessionId` remains distinct from target admission `SessionId`.

Request IDs remain correlation only.

Verifier time is neither identity nor authorization authority; it is an explicit input to existing time-bound verification.

## 11. Dispatcher boundary remains separate

The existing NB status-only dispatcher type/source remains separate.

C03e-PA constructs, transfers, stores, invokes, or widens no production dispatcher.

The first fallible-loop materialization only preserves the existing dispatcher borrow and exact transaction invocation shape after a successful verifier-time sample.

NB production dispatcher construction/transfer remains a later independently gated construction input.

## 12. Request construction remains separate

C03e-PA constructs no `RemoteSessionExpectedDeviceAdmissionRequest`.

It does not install a target `DeviceId`, target admission `SessionId`, authentication request ID, dispatcher, or verifier-time provider into a live expected-device request.

It does not decide the final relative construction order among:

- target `SessionId` generation;
- authentication request-ID generation;
- scheduling-grant opening;
- NB dispatcher construction/transfer;
- verifier-time callback construction;
- request construction;
- construction-failure disposition;
- channel enqueue.

That composition remains separately gated.

## 13. Receipt / construction-failure boundary remains separate

C03e-PA does not construct `RemoteSessionExpectedDeviceAdmissionHandoffReceipt`.

It does not emit `ConstructionFailed`, `ChannelClosed`, `Enqueued`, or `SuppressedOnShutdown`.

The existing C03e-OV shutdown-suppression mapper remains unchanged.

A future failure while preparing a live verifier-time callback or request may eventually participate in `ConstructionFailed` composition, but that ownership/disposal law is not selected here.

## 14. Producer / channel / backpressure law preserved

The eventual expected-device request handoff remains exactly one bounded Tokio MPSC channel of capacity `1`.

Exactly one higher production owner retains the sole sender.

The receiver is created once and moved once.

The eventual enqueue remains only:

`sender.send(request).await`

A full channel means asynchronous backpressure.

C03e-PA creates no channel, sender, receiver, producer closure, producer future, or request.

No sender clone, `try_send`, `blocking_send`, callback `block_on`, hidden or detached task, alternate queue, unbounded queue, retry queue, second channel, second producer future, sleep/retry loop, or backpressure bypass is selected.

Channel closure remains a terminal handoff result; it is not supervisor shutdown authority.

## 15. Shutdown law preserved

Explicit supervisor shutdown remains the sole endpoint-supervisor shutdown authority.

C03e-PA introduces no shutdown detector, alternate cancellation source, sender-drop shutdown semantics, receiver-close shutdown semantics, transport-close shortcut, or endpoint-close reordering.

Existing lower cooperative producer/admission/worker quiescence remains unchanged.

Existing endpoint close and `wait_idle` ordering remains unchanged.

## 16. Staged propagation graph after the selected first source checkpoint

Only after the one-file fallible-loop seam is materially present and exact-head validated may later selection consider propagation through the following layers, one boundary at a time:

1. fallible worker wrapper / bounded worker-stop propagation;
2. executor spawn/supervisor compatibility with the fallible provider;
3. repeated real-admission worker admission and request-carried provider compatibility;
4. endpoint lifecycle forwarding compatibility if source proves necessary;
5. concrete expected-device request construction with the already-selected target SessionId, request ID, dispatcher, and verifier-time provider;
6. construction-failure one-shot grant disposal and receipt composition;
7. higher production channel/sender owner construction;
8. sole-sender producer closure;
9. exact `sender.send(request).await` result mapping to `Enqueued` or `ChannelClosed`;
10. concrete-receipt producer specialization/invocation;
11. higher process/runtime caller migration;
12. requester cleanup/candidate/reachability continuation;
13. target dial/listener/bootstrap/readiness/executable activation;
14. deployment.

This list is dependency mapping only. It does not preassign checkpoint tokens or authorize any listed mutation.

## 17. Selected first source-stage tests

If practical within the single authorized Rust file, focused tests should prove the sibling fallible loop semantics without activating a production caller.

At minimum source/CI review must prove:

- one successful verifier-time sample is passed unchanged to one existing transaction attempt;
- verifier-time failure is preserved and prevents the transaction attempt;
- no time retry/fallback occurs;
- existing transaction failure remains preserved;
- peer-close behavior remains bounded and deterministic;
- historical infallible loop remains unchanged.

If those tests require a second production Rust path merely to construct fixtures, the source stage may rely on existing workspace validation rather than widen the checkpoint.

## 18. Validation authority for this selection

C03e-PA selection is not validated until exact-final-head CI completes on the final documentation head.

No CI result from C03e-OZ, another branch, a synthetic older candidate, or a superseded PA head may be inherited as PA PASS evidence.

`SKIPPED` is not `PASS`.

Any correction to this documentation checkpoint requires a forward-only commit followed by fresh exact-final-head validation.

## 19. Evidence publication law

After exact-final-head validation, C03e-PA requires one immutable raw evidence artifact in the canonical Drive audit parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The audit must record exact predecessor/head/tree/contract blob, exact compare topology, exact-head CI results, the source proof and selected seam, explicit non-actions, and post-publication readback verification.

Canonical evidence publication requires exact-title presearch zero, one raw upload, exact metadata/readback, byte count and SHA-256 verification, and exact-title postsearch yielding one canonical artifact.

The immutable audit may freeze its internal status before publication; final PR metadata records post-publication closure truth.

## 20. Explicit non-actions

C03e-PA is documentation-only.

No Rust/source/runtime mutation.
No verifier-time loop materialization yet.
No worker-stop/error migration.
No executor mutation.
No repeated-supervisor mutation.
No request-carrier mutation.
No endpoint-lifecycle mutation.
No PRWA source mutation.
No target SessionId generation change.
No authentication request-ID generation change.
No scheduling-grant opening or disposal.
No NB dispatcher construction/transfer.
No expected-device request construction.
No authentication payload construction.
No channel/sender/receiver construction or clone.
No `sender.send(request).await`.
No `try_send` or `blocking_send`.
No callback `block_on`.
No hidden/detached producer task.
No alternate/unbounded/retry queue.
No receipt composition.
No caller migration.
No requester cleanup/reachability continuation.
No target dial/listener/bootstrap/readiness/executable activation.
No Cargo/lockfile/workflow/Android-source/packaging mutation.
No deployment.
No merge.
No PR ready-for-review conversion or close.
No branch deletion.
No reset/rebase/squash/force update/history rewrite.
No repository configuration/ruleset/permission mutation.

## 21. Closure condition

C03e-PA may close only after:

- exact docs-only topology is proved from C03e-OZ;
- exact-final-head validation is terminal and reviewed;
- durable Drive evidence is published and raw-readback verified;
- PR metadata is updated to `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- PR remains draft/open/unmerged;
- no successor token is assigned.

C03e-PA does not assign C03e-PB. A fresh exact-head and concurrency audit is mandatory before any later source materialization checkpoint.
