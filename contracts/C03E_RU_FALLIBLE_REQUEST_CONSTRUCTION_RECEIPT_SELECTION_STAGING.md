# C03e-RU — Fallible request-construction receipt composition source-seam selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CONSTRUCTION_FALLIBLE_TERMINAL_RECEIPT_COMPOSITION_SOURCE_SEAM_SELECTION`

Selected future source boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CONSTRUCTION_FALLIBLE_TERMINAL_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION`

This checkpoint is selection-only. It does not materialize the selected Rust source, invoke a producer, create a channel, send a request, migrate a caller, activate runtime behavior, merge, deploy, or authorize any broader boundary.

## 1. Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-RT:

- PR `#610`;
- branch `phase-152-c03e-rt-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-expected-device-admission-scheduling-terminal-shutdown-suppression-eligible-grant-terminal-disposal-receipt-composition-source-materialization`;
- head `6c1b42cdeaeaba9a686ca97f37e87edc05557c71`;
- tree `4bdfa1d94d651c1d7d914a0ce9dff714b071001e`;
- endpoint-lifecycle source blob `9129a78a81eab86b523136ea2982037c2214921f`;
- final RT aggregate relative to RS: exactly one Rust path, `+47/-0`;
- PR #610 remains draft/open/unmerged/mergeable;
- body status `SOURCE MATERIALIZED — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- canonical immutable RT audit Drive ID `1PGFlPh-5FKoTLrzRCwXT4AiNEpJYDhFZ`;
- canonical RT audit size `16581` bytes;
- canonical RT audit SHA-256 `de520b48685e6653775b53c4ac4ec762cd8e8470a6bd1658f4630ca38fa1489e`.

Stable integrated `main` remains:

- head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

Fresh namespace audit before RU branch creation found zero `phase-152-c03e-ru-*` branches. No prior RU checkpoint was assigned.

## 2. Fresh exact-current source finding

Fresh exact-RT source readback proves all independently selected construction prerequisites already exist in:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

at exact predecessor blob:

`9129a78a81eab86b523136ea2982037c2214921f`.

The current file already contains and must reuse unchanged:

- `RemoteSessionExpectedDeviceAdmissionHandoffDisposition`;
- `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome`;
- `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt`;
- verifier-time-agnostic `RemoteSessionExpectedDeviceAdmissionEligibleContinuation`;
- `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeLiveCompletionClassification`;
- `classify_remote_session_expected_device_admission_fallible_verifier_time_live_completion(...)`;
- RT `map_remote_session_expected_device_admission_fallible_verifier_time_shutdown_suppression(...)`;
- `new_remote_session_expected_device_admission_target_session_id()`;
- `new_remote_session_expected_device_authentication_request_id()`;
- `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource`;
- `RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>`;
- historical `RemoteSessionExpectedDeviceAdmissionRequestConstructionOutcome<D>`;
- historical `construct_remote_session_expected_device_admission_request_with_fallible_verifier_time<D>(...)`;
- historical `produce_remote_session_expected_device_admission_with_fallible_verifier_time(...)`.

Therefore RU must not reselect or duplicate target SessionId generation, authentication request-ID generation, the fallible verifier-time source, constructed-request custody, live classification, shutdown suppression, channel design, or generic producer custody.

## 3. Exact missing compatibility gap

The existing QH request-construction carrier and helper were materialized before the current fallible scheduling receipt family existed.

The current verifier-time-agnostic constructed handoff is reusable:

`RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>`

because successful construction no longer contains the scheduling-worker completion family. It owns only:

- requester correlation `DeviceId`;
- exact requester acknowledgement result;
- exactly one typed `RemoteSessionExpectedDeviceAdmissionRequest<D, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource>`.

However the existing construction outcome is not fallible-receipt-compatible:

`RemoteSessionExpectedDeviceAdmissionRequestConstructionOutcome<D>`

has:

- `Constructed(RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>)`;
- `ConstructionFailed(RemoteSessionExpectedDeviceAdmissionHandoffReceipt)`.

Its `ConstructionFailed` arm is the historical infallible scheduling receipt family.

Likewise the existing QH helper constructs the historical infallible `RemoteSessionExpectedDeviceAdmissionHandoffReceipt` when target SessionId generation or authentication request-ID generation fails.

That is the exact current gap after C03e-RT.

## 4. Historical dependency-order authority

Historical C03e-QG / C03e-QH established the request-construction order:

1. target admission SessionId source;
2. expected-device PRWM authentication request-ID source;
3. bind the existing fallible verifier-time function pointer without sampling it;
4. only after both identifiers succeed, open the one-shot scheduling grant exactly once;
5. take target expected `DeviceId` only from the grant and discard requester scheduling `SessionId` provenance;
6. construct exactly one expected-device request;
7. retain requester correlation + acknowledgement result + exactly one constructed request.

Historical C03e-QI / QJ then selected/materialized concrete production status-only dispatcher provenance.

Historical C03e-QK / QL then selected/materialized capacity-one channel custody.

Historical C03e-QM / QN then selected/materialized async send plus `Enqueued` / `ChannelClosed` receipt composition.

This order is authoritative for dependency separation. RU does not skip into QI/QJ, QK/QL, QM/QN, generic producer specialization, or higher-owner invocation.

## 5. Selected future hard source ceiling

A later separately gated source checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Required predecessor source blob:

`9129a78a81eab86b523136ea2982037c2214921f`

The future materialization must STOP and return to selection if correctness requires:

- a second Rust path;
- another module;
- parent-module re-export;
- visibility widening;
- Cargo or lockfile mutation;
- workflow mutation;
- Android-source mutation;
- historical helper mutation;
- channel/dispatcher-owner/runtime/caller integration.

## 6. Selected future representation law

The future source stage may add only one private fallible construction-outcome family, conceptually:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeRequestConstructionOutcome<D>`

with exactly:

- `Constructed(RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>)`;
- `ConstructionFailed(RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt)`.

Exact local naming may follow existing same-file style, but it must clearly distinguish the new fallible receipt outcome from the historical QH outcome.

The future stage must reuse `RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>` unchanged. It must not add a duplicate fallible constructed-handoff carrier because successful constructed-request custody is verifier-time/scheduling-completion-family agnostic at this boundary.

The new construction outcome is private, dormant, non-authoritative and need not derive `Clone` or `Copy`.

No new handoff disposition enum is selected. The existing `RemoteSessionExpectedDeviceAdmissionHandoffDisposition::ConstructionFailed` must be reused.

## 7. Selected future synchronous constructor law

The future source stage may add exactly one private synchronous constructor sibling that accepts:

- one `RemoteSessionExpectedDeviceAdmissionEligibleContinuation` by value;
- one caller-supplied dispatcher `D` by value;

with:

`D: CapabilityDispatcher + Send + 'static`.

It returns only the new fallible construction-outcome family.

The future helper must remain synchronous. It must perform no channel, send, authentication, admission, runtime, endpoint, network, retry, dispatcher-factory, producer or lifecycle work.

## 8. Exact target SessionId ordering

The future constructor must invoke existing:

`new_remote_session_expected_device_admission_target_session_id()`

exactly once before opening the scheduling grant.

If that source fails:

- do not invoke the authentication request-ID source;
- do not open or inspect the scheduling grant;
- consume the existing eligible continuation exactly once;
- terminally dispose the still-sealed scheduling grant by value using the existing Clippy-clean `scheduling_grant: _` form or exact rustfmt-equivalent;
- preserve requester correlation unchanged;
- preserve acknowledgement result unchanged;
- construct exactly one existing fallible handoff receipt;
- use exact outcome `EligibleTerminal`;
- use exact disposition `ConstructionFailed`;
- return `ConstructionFailed(fallible_receipt)`.

No replacement SessionId attempt is authorized.

## 9. Exact authentication request-ID ordering

Only after target SessionId generation succeeds, the future constructor must invoke existing:

`new_remote_session_expected_device_authentication_request_id()`

exactly once before opening the scheduling grant.

If that source fails:

- do not open or inspect the scheduling grant;
- consume the existing eligible continuation exactly once;
- terminally dispose the still-sealed scheduling grant by value with `scheduling_grant: _` or exact rustfmt-equivalent;
- preserve requester correlation unchanged;
- preserve acknowledgement result unchanged;
- construct exactly one existing fallible `EligibleTerminal` receipt with `ConstructionFailed` disposition;
- return `ConstructionFailed(fallible_receipt)`.

No redraw, retry, zero-to-one substitution, counter fallback, SessionId-derived request ID, or other correlation reuse is authorized.

## 10. Exact verifier-time binding law

After both identifier sources succeed, the future constructor may bind existing:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds`

as exact:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource`.

Construction must not call or sample the provider.

No `SystemTime::now()` call is selected inside request construction.

No cached, defaulted, clamped, saturated, requester-derived, dispatcher-derived, identifier-derived or transport-derived time is selected.

Verifier-time sampling remains owned by the downstream fallible authenticated capability-request path.

## 11. One-shot grant opening law

Only after both independent identifier sources succeed may the future constructor move the scheduling grant out of the continuation and invoke:

`ExpectedDeviceSchedulingAuthorityGrant::into_parts()`

exactly once.

The exact returned target expected `DeviceId` is the only target identity source for the expected-device request.

The requester scheduling `SessionId` returned from that grant is scheduling provenance only and must be consumed/discarded at this construction boundary exactly as in QH.

No grant clone, copy, reconstruction, remint, replay, refund, rollback, replacement or second opening is authorized.

## 12. Exact request construction law

After successful identifiers and exact one-shot grant opening, the future constructor must invoke existing:

`RemoteSessionExpectedDeviceAdmissionRequest::new(...)`

exactly once with:

1. target expected `DeviceId` from the consumed grant;
2. fresh target admission `SessionId` from the existing OX source;
3. fresh nonzero authentication request ID from the existing OZ source;
4. exact caller-supplied dispatcher `D` by value;
5. exact fallible verifier-time function pointer.

No additional request field, wrapper, retry authority or secondary request is selected.

The successful result must be exactly:

`Constructed(RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>)`.

`Constructed` means only local ownership of one typed request. It does not mean queued, observed, authenticated, admitted, authorized, worker-inserted, connected, ready, or successful.

## 13. Receipt-family law

Every construction failure in the future RU-selected source boundary must return the existing fallible receipt family:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt`

with:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome::EligibleTerminal`.

It must not construct or transiently route through:

- `RemoteSessionExpectedDeviceAdmissionHandoffReceipt`;
- `RemoteSessionExpectedDeviceAdmissionHandoffReceiptOutcome`.

No cross-family receipt translation is selected.

The historical QH request-construction outcome/helper remain unchanged for historical consumers.

## 14. Existing fallible classifier and shutdown mapper remain separate

The future constructor receives only an already-proven eligible continuation.

It must not call:

- `classify_remote_session_expected_device_admission_fallible_verifier_time_live_completion(...)`;
- `map_remote_session_expected_device_admission_fallible_verifier_time_shutdown_suppression(...)`.

Live eligibility classification remains C03e-RR authority.

Shutdown-recovered eligible suppression remains C03e-RT authority.

The request constructor must not decide whether a raw worker completion is eligible or shutdown-suppressed.

## 15. Requester / target identity separation

Requester callback `DeviceId` remains requester-side authenticated correlation only.

It must never become target expected identity.

Target expected `DeviceId` comes only from the exact consumed scheduling grant after both identifier sources succeed.

Requester scheduling `SessionId` remains distinct from fresh target admission `SessionId`.

Expected-device PRWM authentication request ID remains one-transaction correlation only and is not identity, scheduling authority, capability authority, endpoint authority or persistence authority.

No value derivation or substitution across these lanes is selected.

## 16. Dispatcher boundary remains caller-supplied only

The future constructor accepts one dispatcher `D` supplied by its caller and moves that exact value into the one constructed request on success.

RU does not select:

- `LinuxAgentProductionRemoteCapabilityDispatcherSource` capture;
- `LocalAgentStatusSnapshot` acquisition;
- dispatcher refresh;
- dispatcher retry custody;
- `Arc`/`Mutex`/`RwLock` dispatcher state;
- live/dynamic/watch status provider.

Concrete production dispatcher provenance remains a later integration gate.

On identifier-source failure, the caller-supplied dispatcher is dropped normally with the failed constructor invocation. No dispatcher retry or refresh is authorized.

## 17. Channel and send boundary remains deferred

The future constructor creates no Tokio channel, sender or receiver.

It performs no:

- `sender.send(request).await`;
- `try_send`;
- `blocking_send`;
- `reserve` / `try_reserve` alternate protocol;
- sender clone;
- spare sender;
- retry/requeue;
- alternate/unbounded channel;
- hidden/detached task;
- callback `block_on`.

The eventual handoff law remains one bounded Tokio MPSC channel with capacity exactly `1`, exactly one higher-owned sender, receiver create-once/move-once, and enqueue only through one awaited `send`.

`ConstructionFailed` is terminal and performs no send.

## 18. `Enqueued` / `ChannelClosed` remain deferred

The future RU-selected constructor may compose only:

`ConstructionFailed`

for identifier-source failure after eligibility has already been proven.

It must not compose:

- `Enqueued`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

`SuppressedOnShutdown` remains C03e-RT authority.

`Enqueued` / `ChannelClosed` remain future async-send authority.

## 19. Producer boundaries remain deferred

The future constructor must not:

- construct a producer closure;
- invoke the generic C03e-RN endpoint producer seam;
- specialize `Receipt`;
- own pending producer-future custody;
- observe a receipt;
- invoke QN historical async producer;
- bind the QJ dispatcher source;
- construct/split QL channel custody;
- move a receiver into QF;
- migrate a higher requester/rendezvous owner or process caller.

Those remain separately selected later gates.

## 20. Historical source preservation law

The future source stage must be additive relative to predecessor blob `9129a78a81eab86b523136ea2982037c2214921f`, except only mechanical same-line formatting if unavoidable and proven by exact diff review.

It must not mutate existing QH:

- function-pointer alias;
- constructed handoff;
- historical construction outcome;
- historical request-construction helper;
- historical async producer.

It must not mutate C03e-RP/RR/RT fallible receipt/classifier/shutdown mapper semantics.

If the final net diff contains unrelated historical hunks, the candidate is rejected and must be corrected forward-only before closure.

## 21. Validation law for future source materialization

Any future source materialization must bind PASS only to its exact final head.

Expected validation for a Rust source change includes:

- locked dependency graph;
- formatting;
- Clippy;
- workspace tests;
- workspace build;
- Android validation if actually triggered.

Path-filtered workflows recorded as `SKIPPED` are not PASS.

No predecessor or superseded-head validation may be inherited.

## 22. Evidence law

Future source materialization requires a separate immutable canonical Drive audit.

It must perform:

1. exact-title zero-collision search immediately before upload;
2. exactly one raw immutable upload to canonical parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
3. exact filename/parent/MIME/size readback;
4. complete raw byte readback;
5. exact SHA-256 and final-LF verification;
6. exact-title post-upload singleton verification;
7. revision-lineage verification with exactly one current revision and no predecessor revision;
8. post-publication PR/branch/main/source/evidence guards;
9. no rewrite of frozen audit bytes after publication.

## 23. Explicit non-actions in C03e-RU

C03e-RU is documentation-only.

It performs no:

- Rust/source/runtime mutation;
- target SessionId generation;
- authentication request-ID generation;
- verifier-time sampling;
- scheduling-grant opening or disposal;
- request construction;
- dispatcher construction;
- channel/sender/receiver creation;
- request send;
- `ConstructionFailed`, `Enqueued`, `ChannelClosed` or shutdown receipt runtime composition;
- producer specialization/invocation;
- receiver wiring;
- higher-owner caller migration;
- requester cleanup/reachability continuation;
- target dial/listener/bootstrap/readiness/executable activation;
- database/auth/control-plane mutation;
- Cargo/lockfile/workflow/Android-source mutation;
- packaging/service/repository configuration mutation;
- merge/deployment/ready-for-review transition;
- PR closure;
- branch deletion/reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

## 24. Stop boundary

C03e-RU closes only selection of the fallible request-construction terminal-receipt compatibility seam.

Keep the RU PR draft/open/unmerged.

Do not materialize the selected Rust source inside RU.

No successor token is assigned by this contract. A fresh exact-head/source/namespace/evidence audit is mandatory before any later source materialization.

After RU evidence closure: `STOP`.
