# C03e-QQ — Production Durable Post-Auth Fallible Verifier-Time Repeated Ingress Selection

Status: `SELECTION — VALIDATION PENDING`

Selected future boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REPEATED_INGRESS_SOURCE_MATERIALIZATION`

## 1. Purpose

C03e-QQ is the return-to-selection checkpoint required by the evidence-closed C03e-QP pre-mutation type-feasibility STOP.

QP proved that the QO-selected concrete generic-producer specialization cannot lawfully connect the exact QH/QN fallible verifier-time request family to the existing C03e-OP / C03e-OM producer stack because that stack still constrains verifier time as `FnMut() -> u64`.

QQ does not retry QP, does not weaken the fallible verifier-time contract, and does not change the existing producer stack. It selects the narrowest first prerequisite in the already-existing production-durable requester/rendezvous ingress lineage: a fallible verifier-time sibling of the current C03e-KO production-durable repeated post-auth ingress loop.

The future source checkpoint selected here is intentionally below requester DR/scheduling, below persistent worker custody, below the cooperative producer driver, and below endpoint specialization.

## 2. Exact authoritative repository base

Because QP stopped before mutation, no QP branch, commit or PR exists. QQ therefore branches directly from the exact evidence-closed QO head:

`0b3cb44a122cbcd92f26989d379a86d2bf5025d5`

QO tree:

`c26f48c0660fdb753adb8a6b8c9e452d15ec1ef7`

QO PR #580 remains draft/open/unmerged/mergeable and records:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

The QO immutable audit remains canonical under Drive ID:

`18yzZNkjYKBI9J9NZ4uYOVaGbNlcrrHie`

## 3. Exact C03e-QP feasibility-stop predecessor evidence

QP produced no repository mutation.

Canonical QP feasibility-stop audit:

`C03E_QP_PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_GENERIC_PRODUCER_SPECIALIZATION_SOURCE_MATERIALIZATION_FEASIBILITY_STOP_AUDIT_2026-09-11.md`

Drive ID:

`1-ioq3suWj0-hBV_8rH6MmJlyxuuW-CRF`

Bytes:

`11744`

SHA-256:

`2f04ce3ec789bf708f184741d91bfb665c036e857e0d933efea6bb09f1b0742f`

QP classified the blocked boundary as:

`SOURCE MATERIALIZATION — BLOCKED BEFORE MUTATION — TYPE FEASIBILITY STOP`

Fresh QQ preflight exact-title search re-confirmed that canonical singleton and fresh branch/PR searches proved the QQ namespace was empty before branch creation.

## 4. QP mismatch that QQ must resolve without semantic loss

The expected-device production request family uses the exact QH/QN alias:

```rust
type RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource =
    fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>;
```

The current C03e-OP endpoint producer seam and C03e-OM/OK lower cooperative producer stack instead require:

```rust
T: FnMut() -> u64 + Send + 'static
```

A source returning `Result<u64, PrwaVerifierSourceError>` cannot satisfy `FnMut() -> u64` without destroying or suppressing the selected failure semantics.

QQ therefore rejects all unwrap/default/panic/fallback/time-cache adaptations and rejects a second translated request/channel family.

## 5. Proven deeper mismatch in the scheduling-aware worker lineage

Fresh exact-QO source inspection proves the mismatch is not limited to the endpoint wrapper.

The C03e-OK cooperative scheduling admission helper requires:

```rust
T: FnMut() -> u64 + Send + 'static
```

and successful admission delegates to:

`spawn_recoverable_requester_aware_worker_with_production_durable_scheduling(...)`.

That persistent scheduling-aware worker constructor also requires:

```rust
T: FnMut() -> u64 + Send + 'static
```

and delegates to:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`.

The scheduling-aware requester worker likewise constrains verifier time as `FnMut() -> u64` and, before requester DR/scheduling work, delegates to the existing production-durable cancellation-aware ingress worker.

Therefore adding a fallible bound only at C03e-OM or C03e-OP would be an invalid superficial repair.

## 6. Existing fallible worker lineage proves the required failure model

The repository already contains a separate fallible verifier-time capability-session lineage:

- `run_fallible_verifier_time_capability_request_loop(...)`;
- `run_fallible_verifier_time_capability_request_worker(...)`;
- fallible executor/spawn/supervisor/persistent propagation;
- fallible repeated real-admission collection;
- fallible endpoint lifecycle.

Those seams require:

```rust
T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>
```

and preserve `PrwaVerifierSourceError` as a typed fail-closed terminal rather than coercing it to an arbitrary time value.

QQ preserves that established failure philosophy while selecting the first missing production-durable mixed-family ingress prerequisite.

## 7. Exact current production-durable mixed-family ingress seam

Exact current source:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

Exact QO blob:

`06c64d6ff69ef6862ea3d9c3dd3b4c0c1e34051d`

The existing C03e-KO method is:

`run_repeated_post_auth_control_stream_ingress_with_production_durable_capability(...)`.

It owns a serial loop over the existing one-transaction C03e-KM wrapper.

Its current behavior is:

1. sample infallible verifier time exactly once immediately before one C03e-KM invocation;
2. call the existing one-transaction production-durable accept/read/process wrapper exactly once;
3. on `CapabilityProcessed`, iterate serially;
4. on `RequesterRendezvous(handoff)`, return the exact handoff by value;
5. on exact existing post-auth ingress error, return that error unchanged;
6. perform no cancellation, requester DR, acknowledgement, scheduling derivation, task spawn, queueing, retry or runtime activation.

This is the first existing seam whose time-source signature is incompatible with the QH/QN fallible request family while all lower one-transaction processing already consumes an explicit `u64` value.

## 8. Why C03e-KM does not need a fallible sibling

The existing one-transaction C03e-KM wrapper accepts a concrete:

```rust
now_unix_seconds: u64
```

It does not own verifier-time acquisition.

Therefore fallibility belongs immediately above KM at the repeated-loop sampling boundary, not inside KM and not below the production-durable transaction processor.

A future fallible KO sibling can sample the fallible provider first and invoke the exact existing KM only after successful acquisition.

No typed family-ingress semantics, stream ownership, durable authorization, dispatcher behavior or response I/O needs to change.

## 9. Selected future source ceiling

The immediate future C03e-QR source-materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

No second Rust path is authorized.

If correct QR implementation requires modifying:

- `authenticated_remote_session_runtime.rs` parent source;
- `requester_rendezvous_retained_custody_dr_continuation.rs`;
- executor/runtime collection sources;
- endpoint lifecycle source;
- production higher-owner source;
- `linux_bootstrap.rs`;
- Cargo/lockfile/workflow/Android/packaging/deployment/repository settings;

QR must STOP and return to selection.

## 10. Selected future bounded error family

QR may add exactly one new bounded error family in the selected child source, equivalent in authority to:

`AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError`.

The exact identifier is not authoritative; the semantic surface is.

It must contain exactly two terminal categories:

1. `VerifierTime(PrwaVerifierSourceError)` — exact verifier-time source failure before the next one-transaction ingress invocation;
2. `Ingress(AuthenticatedRemoteSessionPostAuthIngressTransactionError)` — exact existing C03e-KM post-auth ingress failure after successful verifier-time acquisition.

No third category, generic string envelope, boxed dynamic error, retry classification, default-time category or synthetic cancellation category is selected.

The error may be visible only as narrowly as required for later siblings inside `crate::remote_session_capability_runtime`; no public API exposure is selected.

## 11. Exact verifier-time sampling law

The future fallible KO sibling receives:

```rust
T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send
```

For each serial iteration it must invoke the source exactly once, immediately before the corresponding existing C03e-KM invocation.

No eager prefetch, cache, batch sample, duplicate sample, post-accept sample or background sampling is allowed.

A successful sample is passed as the exact `u64` argument to C03e-KM.

## 12. Fail-closed verifier-time error law

If verifier-time acquisition returns `Err(error)`:

- return `VerifierTime(error)` immediately;
- do not invoke C03e-KM;
- therefore do not accept a control stream;
- do not read or classify a family frame;
- do not authorize or dispatch capability traffic;
- do not create a requester handoff;
- do not perform requester DR/scheduling work;
- do not retry or sample again.

The exact `PrwaVerifierSourceError` must remain recoverable through the bounded typed error.

## 13. Successful sample / existing KM law

If verifier-time acquisition returns `Ok(now_unix_seconds)`, invoke exactly one existing:

`process_one_post_auth_control_stream_ingress_with_production_durable_capability(...)`

with:

- the exact existing durable capability authority borrow;
- that exact sampled `u64`;
- the exact caller-supplied dispatcher borrow.

QR must not copy/reimplement KM family logic.

## 14. Capability-success serial continuation

If KM returns:

`AuthenticatedRemoteSessionPostAuthIngressOutcome::CapabilityProcessed`

then the future fallible loop begins exactly one next serial iteration.

The next iteration obtains a fresh fallible verifier-time sample exactly once before its own KM invocation.

No overlap, pipelining, queueing, task spawn or concurrent stream acceptance is selected.

## 15. Requester handoff law

If KM returns:

`AuthenticatedRemoteSessionPostAuthIngressOutcome::RequesterRendezvous(handoff)`

then the future loop returns the exact `RequesterRendezvousResponseStreamCustodyHandoff` by value and stops.

It performs no DR continuation, acknowledgement framing/I/O, scheduling derivation, candidate continuation or next ingress iteration.

The exact response-stream custody and authenticated requester start-intent lineage remain unchanged.

## 16. Existing KM error law

If KM returns one exact:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError`

then the future loop returns `Ingress(exact_error)`.

No flattening, stringification, suppression, retry, fallback, fabricated requester response or second stream attempt is selected.

Nested production-durable authority/dispatch/response provenance must remain intact.

## 17. Candidate-publication behavior remains unchanged

Current C03e-KM candidate-publication handling remains the existing explicit fail-closed higher-owner barrier.

QR does not add candidate provider execution, candidate response behavior, candidate scheduling, target dialing or candidate authority mutation.

Any existing candidate-publication ingress error is retained under the `Ingress(...)` branch unchanged.

## 18. Cancellation remains separately gated

QQ does not select a fallible counterpart to:

`run_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability(...)`.

That cancellation-aware KQ sibling remains a later checkpoint after the fallible repeated loop exists and is validated.

QR therefore adds no cancellation future, polling race, cancellation terminal or peer-close behavior.

## 19. Requester/rendezvous worker remains separately gated

QQ does not select a fallible counterpart to:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`.

No requester DR, acknowledgement, scheduling-authority derivation, cancellation-after-ack logic or scheduling-terminal custody changes are authorized by QQ/QR.

## 20. Persistent scheduling worker remains separately gated

QQ does not modify:

`spawn_recoverable_requester_aware_worker_with_production_durable_scheduling(...)`.

No task-spawn behavior, retained authenticated-session owner custody, cancellation controller, dispatcher ownership or scheduling worker result type is changed.

## 21. Cooperative producer stack remains separately gated

QQ/QR do not modify:

- `finish_cooperative_scheduling_admission(...)`;
- `drive_pending_cooperative_scheduling_producer(...)`;
- `drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling_producer(...)`;
- C03e-OM endpoint executor wrapper;
- C03e-OP endpoint owner wrapper.

All of those remain infallible-verifier-time historical seams until separately selected fallible counterparts propagate through the prerequisite chain.

## 22. QN producer and QH request construction remain unchanged

QQ does not alter the already-materialized expected-device live producer or request constructor.

QH continues to bind the exact fallible PRWA verifier-time function pointer without sampling it.

QN continues to enqueue exactly that request family with exactly one `sender.send(request).await`.

`Constructed` remains distinct from `Enqueued`.

## 23. QJ and QL custody remain unchanged

QQ/QR do not:

- capture or widen `LinuxAgentProductionRemoteCapabilityDispatcherSource`;
- mint a second production status snapshot;
- construct `LinuxAgentProductionExpectedDeviceAdmissionChannel`;
- call its `into_parts(self)`;
- clone the sender;
- create a second/unbounded/alternate channel;
- move a receiver into QF.

## 24. No new retry/default semantics

Forbidden in QR:

- `unwrap`, `unwrap_or`, `unwrap_or_default`, `expect` on verifier-time result as a policy decision;
- fixed or zero timestamp fallback;
- last-known-good cache;
- retry loop;
- second sample after failure;
- mapping verifier-time failure to ingress success/failure categories;
- converting fallible provider into `FnMut() -> u64`;
- panic as normal failure handling.

## 25. No new concurrency boundary

QR creates no task, thread, queue, channel, background loop or detached future.

The selected helper is one ordinary async serial loop, matching existing KO ownership shape.

## 26. Visibility law

The new error and loop may expose only the minimum crate-internal visibility required for later sibling composition inside `remote_session_capability_runtime`.

QQ selects no public API, no external-crate surface and no widening of existing QH/QN private receipt or authority types.

## 27. Source-layout law

QR should extend the existing C03e-KO child module because that file already owns:

- durable typed ingress processing;
- one-transaction KM wrapper;
- repeated KO loop;
- KQ cancellation-aware worker.

Moving this first fallible prerequisite into endpoint/executor/higher-owner files solely to avoid a new typed error would violate existing modular architecture.

## 28. Same-file tests

QR may add narrowly focused tests in the same selected source file if practical.

Tests may prove type/error surface and pure sampling/order behavior, but must not require a second source path, network activation, production credentials, runtime listener activation or repository workflow mutation.

Tests are optional if existing workspace validation sufficiently exercises type/lint/build integrity and a safe isolated test would require scope expansion.

## 29. Rust feasibility gate for QR

Before source mutation, QR must re-read the exact QQ final head and exact selected source blob.

If implementing the selected loop requires a second source file, modification of KM, changing existing infallible KO/KQ signatures, widening production authority, or introducing a channel/task, QR must STOP before mutation.

Rustfmt/Clippy corrections may use forward-only commits only within the selected source path.

No reset/rebase/squash/force update/history rewrite is permitted.

## 30. Future propagation after QR remains explicitly staged

Even after successful QR materialization, at minimum these remain separately gated:

1. fallible verifier-time production-durable cancellation-aware post-auth ingress worker;
2. fallible verifier-time production-durable requester/rendezvous serial lifecycle;
3. fallible verifier-time scheduling-aware requester sibling preserving exact scheduling terminal custody;
4. fallible persistent recoverable scheduling-worker propagation;
5. fallible cooperative scheduling producer driver counterpart;
6. fallible executor endpoint producer wrapper counterpart;
7. fallible endpoint-owner producer wrapper counterpart;
8. retry of concrete QN receipt/producer specialization against that validated fallible producer stack;
9. QJ dispatcher-source capture;
10. QL channel construction/split;
11. higher-owner source composition;
12. receiver transfer into QF;
13. higher receipt observation policy;
14. process/executable caller;
15. listener/readiness/runtime/network activation;
16. merge/deployment.

No future step may be silently collapsed into QR.

## 31. Repository / platform exclusions

QQ and future QR do not authorize changes to:

- authentication protocol;
- trust roots/certificates/private keys/credentials;
- RBAC or policy semantics;
- database/schema/data;
- control-plane API;
- repository settings or branch protection;
- workflow permissions;
- Android application behavior;
- package publication;
- deployment configuration.

## 32. Validation authority

QQ is a docs-only selection checkpoint.

Only CI runs bound to the exact final QQ head may support QQ validation claims.

A skipped workflow must be recorded as `SKIPPED`, not PASS.

If Android does not register on the docs-only head, no Android PASS may be inherited from QO or earlier checkpoints.

## 33. Immutable evidence law

After exact-head validation, publish exactly one immutable QQ audit under canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The frozen audit must record:

- exact QO base and QP feasibility-stop evidence;
- exact QQ branch/head/tree;
- exact contract blob;
- direct QO -> QQ topology;
- selected one-file QR source ceiling;
- exact source blobs proving the prerequisite chain;
- exact-head CI enumeration;
- unchanged `main`;
- pre-upload zero-match check;
- frozen byte count + SHA-256;
- Drive ID/parent/title/size;
- raw readback verification;
- post-upload exact-title singleton check.

The frozen audit should retain internal status:

`SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`.

After evidence publication and verification, the QQ PR body may record:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

## 34. PR state law

The QQ PR must remain:

- draft;
- open;
- unmerged.

No ready-for-review conversion, merge, close, branch deletion, deployment or restart is authorized.

## 35. STOP boundary

C03e-QQ ends after docs-only selection validation, immutable evidence publication, PR evidence-closure update and final closure re-audit.

Do not create C03e-QR inside QQ closure.

STOP after C03e-QQ.
