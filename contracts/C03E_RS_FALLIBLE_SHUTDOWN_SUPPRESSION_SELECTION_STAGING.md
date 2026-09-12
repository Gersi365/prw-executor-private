# C03e-RS — Fallible Shutdown Suppression / Eligible-Grant Terminal Disposal Receipt Composition Source-Seam Selection

Status:

`SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_SHUTDOWN_SUPPRESSION_ELIGIBLE_GRANT_TERMINAL_DISPOSAL_RECEIPT_COMPOSITION_SOURCE_SEAM_SELECTION`

Selected future source boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_SHUTDOWN_SUPPRESSION_ELIGIBLE_GRANT_TERMINAL_DISPOSAL_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION`

This checkpoint is documentation-only. It selects one future dormant private synchronous shutdown-suppression receipt mapper after evidence-closed C03e-RR. It does not materialize Rust source, specialize a producer, construct a request/channel/sender, activate runtime behavior, merge, deploy, or widen visibility.

## 1. Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-RR:

- branch `phase-152-c03e-rr-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-expected-device-admission-scheduling-terminal-live-completion-eligibility-classification-eligible-continuation-custody-source-materialization`;
- head `3feabb3bb1c92015d90e70f3630d35152e4d97c1`;
- tree `351f0dc68e00b2a15e87ef8516069f99bfbb2ee5`;
- source blob `1ee2410f72d4621f1cd8082ad7dcbea1565bef4a`;
- PR #608 remains draft/open/unmerged/mergeable with evidence-closed status;
- immutable C03e-RR Drive evidence ID `1zMvrHO1Bz1X5pfp1FCo79KW8Izj447Co`;
- frozen/readback bytes `16721`;
- SHA-256 `1b0906e942300d38fdb9d9f6f0e8434bf2aca490996cdc78d72aa9932028bd2d`.

C03e-RR materialized only:

1. private `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeLiveCompletionClassification`;
2. private synchronous `classify_remote_session_expected_device_admission_fallible_verifier_time_live_completion(...)`.

It explicitly deferred fallible shutdown-suppression receipt mapping, eligible-grant terminal disposal, `EligibleTerminal` receipt construction, request/channel/producer work, producer specialization/invocation, higher caller migration and runtime activation.

## 2. Historical ordering authority

Historical C03e-OT -> C03e-OU -> C03e-OV remains the direct semantic ordering authority:

1. C03e-OT materialized live-completion classification and eligible-continuation custody;
2. C03e-OU selected a shutdown-suppression mapper;
3. C03e-OV materialized only that mapper.

Historical C03e-OU selected one private synchronous pure mapper after classification and before request-construction / producer / channel work.

Historical C03e-OV materialized the selected mapper with final by-value grant disposal expressed through `scheduling_grant: _` so no grant field was bound or observed.

C03e-RS selects the exact fallible-verifier-time counterpart of that shutdown-suppression seam.

## 3. Fresh exact C03e-RR source facts

Exact C03e-RR source already contains:

- `RemoteSessionExpectedDeviceAdmissionHandoffDisposition::SuppressedOnShutdown`;
- `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome` with:
  - `Ineligible(Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>)`;
  - `EligibleTerminal { acknowledgement_result, disposition }`;
- `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt` carrying requester correlation and one exact fallible receipt outcome;
- existing verifier-time-agnostic `RemoteSessionExpectedDeviceAdmissionEligibleContinuation` carrying requester callback `DeviceId`, exact one-shot `ExpectedDeviceSchedulingAuthorityGrant`, and exact acknowledgement result;
- C03e-RR fallible live-completion classifier.

The existing historical infallible shutdown-suppression mapper remains in the same source file and is not selected for modification.

No new receipt representation, handoff disposition, eligible continuation, scheduling grant, or error family is required for the future fallible mapper.

## 4. Selected future hard source ceiling

The immediate future source-materialization checkpoint is ceilinged to exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Required predecessor blob:

`1ee2410f72d4621f1cd8082ad7dcbea1565bef4a`

If correct source materialization requires:

- a second Rust path;
- parent-module re-export;
- visibility widening;
- lower cooperative driver mutation;
- executor method mutation;
- receipt representation mutation;
- classifier mutation;
- scheduling-terminal/grant mutation;
- producer specialization or invocation;
- request/channel/caller/runtime mutation;

then the future source checkpoint must STOP and return to selection.

## 5. Selected future mapper

The future source checkpoint may add exactly one private synchronous mapper conceptually named:

`map_remote_session_expected_device_admission_fallible_verifier_time_shutdown_suppression(...)`

It must accept exactly:

1. requester callback `DeviceId`;
2. exact fallible completion:
   `Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

It must return exactly:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt`

without an additional error envelope.

No async signature, future, task, channel, sender or runtime handle is selected.

## 6. Exact classifier-use law

The future mapper must call:

`classify_remote_session_expected_device_admission_fallible_verifier_time_live_completion(...)`

exactly once.

The mapper must not reproduce or fork classification logic.

It must not inspect the raw fallible completion before or after invoking the classifier except through the classifier's returned bounded two-family result.

C03e-RR remains sole authority for live eligibility classification.

## 7. Exact ineligible mapping law

For:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeLiveCompletionClassification::Ineligible(receipt)`

the future mapper must return that exact existing fallible receipt unchanged.

It must not:

- reconstruct the receipt;
- project the contained completion;
- rewrite requester callback identity;
- flatten fallible failure provenance;
- translate join failure;
- translate scheduling derivation failure;
- convert to the historical infallible receipt family;
- attach a shutdown disposition to an ineligible completion.

## 8. Exact eligible shutdown-suppression law

For:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeLiveCompletionClassification::Eligible(continuation)`

the future mapper must consume the continuation exactly once.

It must terminally dispose the exact one-shot scheduling grant by value without reading, binding or extracting either grant field.

Selected expression shape is the historical C03e-OV form or exact rustfmt-equivalent:

`let RemoteSessionExpectedDeviceAdmissionEligibleContinuation { requester_device_id, scheduling_grant: _, acknowledgement_result } = continuation;`

This disposal is irreversible and terminal for the already-issued grant on this shutdown-suppression path.

The mapper must preserve the exact acknowledgement result unchanged.

It must construct exactly one fallible receipt:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt`

whose outcome is exactly:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome::EligibleTerminal { acknowledgement_result, disposition: RemoteSessionExpectedDeviceAdmissionHandoffDisposition::SuppressedOnShutdown }`

No other disposition is selected here.

## 9. Grant opacity / one-shot authority law

The future mapper must not call:

`ExpectedDeviceSchedulingAuthorityGrant::into_parts()`.

It must not inspect or extract:

- requester scheduling `SessionId`;
- target expected `DeviceId`.

It must not clone, copy, reconstruct, remint, refund, replay, roll back, return or reauthorize the grant.

A lexical `drop(scheduling_grant)` or equivalent unobserved by-value destruction is semantically acceptable, but the historical Clippy-clean `scheduling_grant: _` pattern is the selected implementation precedent.

## 10. Identity law

Requester callback `DeviceId` remains requester-side authenticated identity/correlation only.

It is never target expected identity.

Target expected `DeviceId` remains sealed inside the scheduling grant until the grant is terminally disposed on this shutdown path.

Requester scheduling `SessionId` remains distinct from any future target admission `SessionId`.

No target admission `SessionId` or authentication request ID exists in this selected mapper.

## 11. Acknowledgement orthogonality law

Requester acknowledgement success/failure remains orthogonal to scheduling eligibility and shutdown suppression.

The future mapper must preserve the exact acknowledgement result unchanged into the `EligibleTerminal` receipt.

It must not:

- convert acknowledgement failure into ineligibility;
- suppress or erase acknowledgement failure;
- change shutdown disposition based on acknowledgement result;
- introduce a new acknowledgement error wrapper.

## 12. Fallible provenance law

The exact fallible worker-stop failure family remains untouched by the mapper because it can only arrive through an existing `Ineligible(receipt)` classification.

No conversion is selected from:

`RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop`

to:

`RequesterRendezvousProductionDurableSchedulingWorkerStop`.

Verifier-time / production-durable ingress and requester-response failure provenance therefore remains exact.

## 13. Shutdown authority law

Explicit supervisor shutdown remains the sole supervisor-shutdown authority.

The future mapper does not detect shutdown.

It only maps an already shutdown-recovered completion when invoked by the existing lower cooperative producer path.

It must not:

- request shutdown;
- observe process signals;
- cancel workers;
- drain workers;
- dispose scheduling peers;
- close receiver/channel/endpoint/transport;
- call `wait_idle()`;
- create a second shutdown source.

Existing lower cooperative scheduling / producer shutdown ordering remains authoritative and unchanged.

## 14. Producer law

C03e-RN remains the higher endpoint-owner fallible generic producer forwarding seam.

Its producer remains caller-owned and mutably borrowed as `&mut H`.

Receipt custody remains generic there.

C03e-RS does not select:

- concrete producer specialization;
- concrete receipt specialization at C03e-RN;
- producer invocation;
- producer ownership movement/storage/clone;
- second producer future;
- detached/background producer task.

The future mapper is merely compatible with the existing synchronous suppression-mapper generic slot; compatibility is not invocation authority.

## 15. Channel / backpressure law remains deferred

No channel or sender is selected by C03e-RS.

The eventual handoff law remains separately gated:

- one bounded Tokio MPSC channel;
- capacity exactly `1`;
- exactly one higher-owned production sender;
- receiver create-once/move-once;
- enqueue only via `sender.send(request).await`;
- full channel means ordinary asynchronous backpressure.

Still forbidden here:

- sender clone;
- `try_send`;
- `blocking_send`;
- callback `block_on`;
- alternate/unbounded/retry queue;
- hidden producer task;
- second producer future.

## 16. Request-construction lanes remain deferred

C03e-RS selects no:

- target admission `SessionId` source;
- independent nonzero expected-device PRWM authentication request-ID source;
- dispatcher production construction or transfer;
- concrete fallible verifier-time source binding;
- `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- request send;
- request retry/reconstruction.

Those lanes begin only after a later separately gated request-construction selection.

## 17. Other receipt dispositions remain deferred

C03e-RS selects only shutdown suppression.

The following remain separately gated:

- `ConstructionFailed`;
- `ChannelClosed`;
- `Enqueued`.

They require later construction/channel/send outcomes and must not be inferred or constructed by the shutdown-suppression mapper.

## 18. No lifecycle or visibility mutation

The future source checkpoint must not modify:

- existing endpoint lifecycle methods;
- C03e-RN higher endpoint-owner producer forwarding;
- C03e-RL executor adapter;
- lower fallible cooperative producer driver;
- historical infallible classifier/mapper;
- receipt type visibility;
- classifier visibility;
- scheduling grant visibility;
- parent module exports.

No public or crate-visible API expansion is selected.

## 19. Expected future patch shape

The expected future source patch is additive and local:

- one doc-commented private synchronous function;
- optional narrow local lint acknowledgement only if exact CI requires it;
- no imports if exact current source remains sufficient;
- no tests unless strictly necessary to satisfy the selected semantics without widening source scope.

Historical C03e-OV indicates an approximately 46-line additive mapper and one possible forward-only Clippy correction if a no-effect underscore binding is used.

The future materialization must prefer the already Clippy-clean `scheduling_grant: _` disposal shape to avoid repeating that historical correction.

## 20. Validation law for C03e-RS

C03e-RS itself is documentation-only.

Validation claims may bind only to the exact final C03e-RS head.

`SKIPPED` is never PASS.

No Android PASS may be claimed unless an Android workflow actually runs and succeeds on the exact final RS head.

No CI result may be inherited from C03e-RR or another SHA.

## 21. Immutable evidence law

After exact-final-head C03e-RS validation:

1. freeze one immutable audit artifact;
2. exact-title zero-collision preflight in canonical Drive parent;
3. exactly one raw upload;
4. verify exact metadata, bytes, SHA-256 and final LF;
5. require exact-title singleton;
6. require one current revision with no predecessor;
7. bind final publication truth only in PR body;
8. never rewrite frozen audit bytes after publication.

Canonical Drive parent remains:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

## 22. Stable main / repository law

`main` is expected to remain exactly:

- head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

C03e-RS authorizes no `main` mutation.

It also authorizes no repository configuration, ruleset, permission, branch-protection, visibility or deployment mutation.

## 23. Explicitly deferred after C03e-RS

Still separately gated:

- source materialization of the selected fallible shutdown-suppression mapper;
- request-construction input boundary selection;
- target admission `SessionId` generation/source;
- independent nonzero expected-device PRWM authentication request-ID source;
- dispatcher production provenance;
- concrete fallible verifier-time production binding;
- actual expected-device request construction;
- construction-failure terminal composition;
- higher production channel/sender ownership;
- sole-sender producer closure;
- `sender.send(request).await`;
- `ChannelClosed` receipt composition;
- `Enqueued` receipt composition;
- generic producer specialization/invocation with concrete fallible receipt;
- higher process/runtime caller migration;
- requester cleanup/candidate/reachability continuation;
- target dial/listener/bootstrap/readiness/executable activation;
- database/auth/control-plane mutation;
- Cargo/lockfile/workflow/Android-source mutation;
- packaging/service/repository configuration;
- deployment;
- merge/ready-for-review;
- PR closure;
- branch deletion/reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

## 24. Stop boundary

C03e-RS is selection-only.

After exact-head validation, immutable evidence publication and PR-body closure binding:

`STOP`

Do not create or materialize the future source successor in the same closure.
