# C03e-RW — Fallible Async Producer Terminal Receipt Composition Source-Seam Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_ASYNC_PRODUCER_SEND_FALLIBLE_TERMINAL_RECEIPT_COMPOSITION_SOURCE_SEAM_SELECTION`

Selected future boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_ASYNC_PRODUCER_SEND_FALLIBLE_TERMINAL_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION`

This checkpoint is contract/documentation only. It selects one future dormant private async producer sibling and performs no Rust/source/runtime mutation.

## Exact predecessor

Evidence-closed C03e-RV is the only predecessor authority:

- branch: `phase-152-c03e-rv-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-expected-device-admission-request-construction-fallible-receipt-composition-source-materialization`
- head: `97e4dd05646e2952e6bc6e134d43e0732f0c9221`
- tree: `5877d372941b07700dcdfc9b75d3e3886a7573f7`
- endpoint source blob: `2c9f27105a1f67ab017ffbde4d369cee03f84d8c`
- PR: `#612`, draft/open/unmerged/mergeable, evidence-closed
- immutable RV audit Drive ID: `1EVvacUNzalC6r56ZOJBeifv3AEVa2PKE`
- RV audit bytes: `11312`
- RV audit SHA-256: `8d5d6284b3606e607b88386c15f7fa472a48bcc1aa1d074985464e2ba9313108`

The future source stage MUST start from this exact predecessor source blob. If the source blob differs, STOP and return to selection.

## Fresh source facts that define the seam

The exact RV source already contains and preserves unchanged:

1. `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome`, whose `Ineligible(...)` owns the exact fallible scheduling-worker completion and whose `EligibleTerminal` owns the exact acknowledgement result plus the existing bounded handoff disposition.
2. `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt`, which preserves requester callback `DeviceId` as correlation only.
3. `RemoteSessionExpectedDeviceAdmissionEligibleContinuation`, which owns requester correlation, one exact sealed `ExpectedDeviceSchedulingAuthorityGrant`, and the exact acknowledgement result.
4. `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeLiveCompletionClassification`, with `Ineligible(existing fallible receipt)` and `Eligible(existing continuation)`.
5. `classify_remote_session_expected_device_admission_fallible_verifier_time_live_completion(...)`, the C03e-RR pure classifier authority.
6. `map_remote_session_expected_device_admission_fallible_verifier_time_shutdown_suppression(...)`, the C03e-RT synchronous shutdown-only mapper authority.
7. `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeRequestConstructionOutcome<D>`, whose successful branch reuses the existing verifier-time-agnostic constructed handoff and whose failure branch owns the existing fallible receipt.
8. `construct_remote_session_expected_device_admission_request_with_fallible_verifier_time_and_fallible_receipt<D>(...)`, the C03e-RV synchronous construction authority.
9. the older C03e-QN dormant async producer `produce_remote_session_expected_device_admission_with_fallible_verifier_time(...)`, which remains the historical send/backpressure analogue but still accepts the historical non-fallible scheduling completion and returns the historical non-fallible receipt family.

The remaining compatibility gap is therefore not classification, shutdown suppression, identifier sourcing, verifier-time binding, request construction, or the Tokio send protocol individually. The gap is one producer-level sibling that composes the already-materialized fallible completion/classifier/construction/receipt family with the already-validated historical QN async send law.

## Historical QN send law retained as precedent

The existing C03e-QN helper is precedent only. Its send semantics remain authoritative for the future sibling:

1. classify the exact live completion once;
2. ineligible custody returns immediately, with no dispatcher construction;
3. eligible custody invokes one caller-supplied infallible dispatcher factory exactly once;
4. request construction is invoked exactly once;
5. construction failure returns the exact terminal receipt unchanged and performs no send;
6. constructed custody performs exactly one borrowed `sender.send(request).await`;
7. successful queue acceptance maps only to `Enqueued`;
8. receiver closure maps only to `ChannelClosed` and terminally drops the returned unsent request;
9. the helper returns exactly one terminal handoff receipt;
10. ordinary bounded Tokio asynchronous backpressure is preserved.

`Constructed` is not `Enqueued`. `Enqueued` proves queue acceptance only; it does not prove receiver observation, authentication, admission, authorization, worker insertion, endpoint success, or readiness.

## Selected future source seam

A future separately gated source checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Required predecessor blob:

`2c9f27105a1f67ab017ffbde4d369cee03f84d8c`

It may add only one private dormant async producer sibling, conceptually:

`produce_remote_session_expected_device_admission_with_fallible_verifier_time_and_fallible_receipt(...)`

Exact naming/rustfmt layout is not semantic, but the helper MUST have the following custody and behavior.

### Input/output family

The helper must accept:

- requester callback `DeviceId`;
- the exact fallible scheduling-worker completion:
  `Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`;
- one temporary mutable borrow of a caller-supplied infallible dispatcher factory of the same authority shape used by QN;
- one temporary immutable borrow of the exact typed bounded Tokio sender used for expected-device admission requests.

The helper must return:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt`.

It must not convert through, construct, or return the historical non-fallible receipt family.

### Exact live classification

The helper MUST call:

`classify_remote_session_expected_device_admission_fallible_verifier_time_live_completion(...)`

exactly once.

If the result is `Ineligible(receipt)`, it MUST return that exact existing receipt unchanged.

Before that return it MUST NOT:

- call the dispatcher factory;
- construct a request;
- generate any identifier;
- sample verifier time;
- inspect or reconstruct fallible completion provenance;
- send or reserve channel capacity.

### Eligible dispatcher construction

Only after receiving one `Eligible(continuation)` may the helper invoke the caller-supplied infallible dispatcher factory.

The factory MUST be invoked exactly once for that eligible attempt.

The helper MUST NOT own, clone, memoize, retry, replace, re-export, or manufacture dispatcher-source authority.

Concrete production dispatcher-source capture remains separately gated.

### Exact fallible request construction

The helper MUST invoke:

`construct_remote_session_expected_device_admission_request_with_fallible_verifier_time_and_fallible_receipt(...)`

exactly once with the exact continuation and exactly one dispatcher returned by the caller-supplied factory.

The helper MUST NOT independently:

- source target admission `SessionId`;
- source expected-device authentication request ID;
- bind or sample verifier time;
- open, inspect, remint, clone, replay, refund, rollback, or reconstruct the scheduling grant;
- derive target expected `DeviceId` from requester correlation;
- reconstruct a request.

Those laws remain solely inside the RV construction authority.

### ConstructionFailed law

If RV construction returns `ConstructionFailed(receipt)`, the producer MUST return that exact existing fallible receipt unchanged.

It MUST perform no send and no channel-capacity operation.

It MUST NOT translate the receipt into the historical non-fallible family.

### Constructed/send law

If RV construction returns `Constructed(existing constructed handoff)`, the helper may destructure exactly that handoff to obtain:

- requester callback correlation;
- exact acknowledgement result;
- exactly one expected-device admission request.

It MUST then perform exactly one ordinary awaited send through the borrowed sender:

`sender.send(request).await`

or the exact rustfmt-equivalent expression.

Forbidden send alternatives include:

- sender clone;
- `try_send`;
- `blocking_send`;
- `reserve`/`try_reserve` alternate protocols;
- callback `block_on`;
- timeout escape;
- hidden/detached task;
- second producer future;
- retry/requeue;
- alternate or unbounded channel;
- spare/fallback sender.

The helper must retain ordinary Tokio asynchronous backpressure until the one send future terminates.

### Send success law

Only `Ok(())` from the exact awaited send maps to a newly constructed fallible receipt with:

- exact requester callback `DeviceId`;
- exact acknowledgement result;
- outcome `EligibleTerminal`;
- disposition `RemoteSessionExpectedDeviceAdmissionHandoffDisposition::Enqueued`.

No stronger success meaning is selected.

### Receiver-closed law

If the send returns the request because the receiver is closed:

- the returned unsent request MUST be terminally dropped locally;
- no retry, requeue, alternate receiver, rollback, grant reconstruction, or shutdown reinterpretation is permitted;
- exactly one fallible `EligibleTerminal` receipt is constructed with exact requester callback correlation, exact acknowledgement result, and disposition `ChannelClosed`.

Receiver closure is not supervisor shutdown. The existing RT shutdown-suppression mapper remains distinct and MUST NOT be invoked from this live producer sibling.

## Authority and identity invariants

The future helper must preserve all existing identity separation:

- requester callback `DeviceId` remains requester-side correlation only;
- target expected `DeviceId` remains sourced only by the RV-consumed scheduling grant;
- requester scheduling `SessionId` provenance remains distinct from the target admission `SessionId` and remains discarded by RV construction;
- the target admission `SessionId` source and authentication request-ID source remain exactly the RV sources;
- acknowledgement result remains orthogonal to eligibility and is preserved exactly into terminal receipts;
- fallible verifier-time provenance is preserved for ineligible custody and is never flattened into the historical receipt family;
- the verifier-time function pointer remains bound by RV and is not sampled by this producer.

## Hard source ceiling

Future materialization is permitted to change exactly one path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

It may add only the private dormant async sibling described above, plus minimal local comments/allow annotations required for compilation/lints.

It MUST NOT edit the existing RR classifier, RT shutdown mapper, RV construction outcome/helper, QN historical producer, request type, receipt representations, disposition enum, continuation representation, scheduling grant type, imports unless compilation strictly requires an import already implied by the selected helper, or any second Rust path.

If correct materialization requires a second Rust path, visibility widening, type-family redesign, channel ownership change, or producer invocation, STOP and return to selection.

## Explicitly deferred

Still separately gated after the future source materialization:

- concrete production dispatcher-source capture into a producer-owned factory;
- status snapshot acquisition/refresh policy;
- production channel construction;
- sender/receiver ownership split;
- sole-sender custody policy;
- concrete generic producer closure ownership/capture;
- specialization or invocation of any higher generic producer seam;
- receiver transfer into a higher owner;
- higher receipt observation policy;
- requester/rendezvous higher-owner integration;
- process/runtime caller migration;
- endpoint/listener/bootstrap/readiness/network activation;
- database/auth/control-plane mutation;
- Cargo/lockfile mutation;
- workflow mutation;
- Android source mutation;
- packaging/service/repository configuration;
- deployment;
- merge;
- ready-for-review conversion;
- PR closure;
- branch deletion;
- reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

No ordering among independent later integration gates is invented beyond existing authority/custody laws.

## Validation contract for this selection checkpoint

C03e-RW itself is docs-only.

Validation authority must bind only to the exact final RW head. `SKIPPED` is not PASS. No PASS is inherited from RV or any historical checkpoint.

The exact RV -> RW compare must show:

- merge base exact RV head `97e4dd05646e2952e6bc6e134d43e0732f0c9221`;
- ahead only, behind zero;
- exactly one changed documentation path;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/packaging/deployment/repository-configuration changes.

## Evidence protocol

After exact-head validation succeeds:

1. freeze a local markdown audit at internal status `SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`;
2. perform exact-title zero-collision search in canonical Drive parent immediately before upload;
3. upload frozen bytes exactly once;
4. verify title, parent, MIME, byte count, raw bytes/SHA-256/final LF, singleton exact-title result, and exactly one initial revision;
5. never mutate/replace/re-upload the immutable audit;
6. record `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED` only in the PR body;
7. perform final RW/PR/main/Drive readbacks and successor-namespace zero-collision guard;
8. STOP.

Canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

## Non-actions in C03e-RW

This selection checkpoint performs no Rust/source/runtime mutation, classifier invocation, shutdown mapping, grant consumption/disposal, identifier generation, verifier-time binding/sampling, dispatcher construction, request construction, channel construction, sender clone, send, receipt transition, producer invocation, caller migration, runtime/network activation, dependency/workflow/Android-source mutation, repository configuration mutation, merge, deployment, ready conversion, PR close, branch deletion, reset/rebase/squash/force/history rewrite, or destructive evidence cleanup.

After evidence closure: **STOP**. Do not materialize the selected source inside C03e-RW.
