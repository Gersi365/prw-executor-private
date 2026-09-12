# C03e-RQ — Production-Durable Post-Auth Fallible-Verifier-Time Requester/Rendezvous Expected-Device Admission Scheduling-Terminal Live-Completion Eligibility Classification / Eligible-Continuation Custody Source-Seam Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_LIVE_COMPLETION_ELIGIBILITY_CLASSIFICATION_ELIGIBLE_CONTINUATION_CUSTODY_SOURCE_SEAM_SELECTION`

This checkpoint is documentation-only. It selects one future source materialization boundary and performs no Rust/source/runtime mutation.

## 1. Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-RP:

- branch `phase-152-c03e-rp-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-expected-device-admission-scheduling-terminal-async-producer-handoff-receipt-representation-source-materialization`;
- exact head `2ef6c06466235927aa5ea36c1e456f55939ae6c5`;
- exact tree `30ca8cf064e428e8722f56389a8fa3b90c719392`;
- exact source blob `ab04a4735f7891a11fd5797bd6905c03be5ed354`;
- PR #606 remains draft/open/unmerged/mergeable and evidence-closed;
- immutable C03e-RP Drive evidence ID `155aUdvOE22qiKHmDqNVYgwaUa41bqRgR`;
- frozen evidence bytes `15307`;
- frozen evidence SHA-256 `5f006ee58718302e01c664973545bd7c1bdd5ff339311bec44aca2bfce0e9287`.

C03e-RP materialized only the private fallible receipt representation and explicitly deferred live-completion classification and eligible-continuation custody.

## 2. Historical ordering authority

Historical C03e-OQ -> C03e-OR -> C03e-OS -> C03e-OT establishes the required ordering:

1. select concrete handoff receipt representation;
2. materialize receipt representation;
3. separately select live-completion eligibility classification and eligible-continuation custody;
4. separately materialize that classifier/custody source seam.

Historical C03e-OS selected only `SchedulingTerminal + scheduling_result == Ok(grant)` as eligible and required borrowed discriminant inspection before any by-value terminal decomposition.

Historical C03e-OT then materialized that rule without producer/channel/request construction or runtime activation.

C03e-RQ applies the same ordering to the fallible-verifier-time scheduling lane after C03e-RP.

## 3. Fresh exact source facts

Fresh exact C03e-RP source establishes:

### 3.1 Fallible scheduling-aware worker stop

`RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop` has exactly three terminal families:

- `Cancelled`;
- `Failed(RequesterRendezvousFallibleVerifierTimePostTerminalResponseSerialLifecycleError)`;
- `SchedulingTerminal(RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome)`.

The fallible stop reuses the same non-Clone scheduling terminal carrier as the historical infallible scheduling lane. It does not flatten fallible ingress/requester-response failure provenance.

### 3.2 Scheduling terminal carrier

`RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome` owns two orthogonal channels:

- `scheduling_result: Result<ExpectedDeviceSchedulingAuthorityGrant, ExpectedDeviceSchedulingAuthorityDerivationError>`;
- `acknowledgement_result: Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`.

It exposes sibling-visible borrowed `scheduling_result()` inspection and by-value `into_parts()`.

### 3.3 One-shot scheduling grant

`ExpectedDeviceSchedulingAuthorityGrant` is sibling-visible, non-`Copy`, non-`Clone`, and owns exactly:

- requester scheduling `SessionId` provenance;
- target expected `DeviceId`.

It owns no target admission `SessionId`, authentication request ID, dispatcher, verifier-time provider, sender/channel, endpoint, transport or reachability authority.

### 3.4 Existing eligible continuation is verifier-time-agnostic

The existing private `RemoteSessionExpectedDeviceAdmissionEligibleContinuation` already owns exactly:

- requester callback `DeviceId` correlation;
- exact one-shot `ExpectedDeviceSchedulingAuthorityGrant`;
- exact requester acknowledgement result.

No field in that continuation encodes infallible verifier-time semantics. Once a live completion has been proven `SchedulingTerminal + Ok(grant)`, the fallible failure channel is no longer present in the eligible branch.

Therefore C03e-RQ selects **reuse of the existing eligible continuation**. A second fallible-specific eligible-continuation struct would be redundant and would widen the source surface without adding provenance.

### 3.5 Existing fallible receipt is the ineligible custody authority

C03e-RP materialized private `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt` with private `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome`.

Its `Ineligible(...)` arm owns exact:

`Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

by value.

That receipt is the selected ineligible custody carrier for the future fallible classifier.

## 4. Selected next source boundary

C03e-RQ selects the next separately gated source boundary as:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_LIVE_COMPLETION_ELIGIBILITY_CLASSIFICATION_ELIGIBLE_CONTINUATION_CUSTODY_SOURCE_MATERIALIZATION`

The immediate future hard source ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Required predecessor blob for that future source stage:

`ab04a4735f7891a11fd5797bd6905c03be5ed354`

If materialization requires a second Rust path, a visibility widening in another module, or a change to the lower scheduling-terminal/grant types, the source stage must STOP and return to selection.

## 5. Exact selected source shape

The future source-materialization stage may add only the minimum fallible classification surface:

1. one private two-family fallible classification carrier, conceptually:
   - `Ineligible(existing C03e-RP fallible receipt)`;
   - `Eligible(existing RemoteSessionExpectedDeviceAdmissionEligibleContinuation)`;
2. one private synchronous pure classifier from requester callback `DeviceId` plus exact fallible completion into that classification;
3. only minimum local lint acknowledgement and focused same-file tests if strictly useful to prove the selected pure classification law.

The future source stage must **reuse**:

- `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt` for ineligible custody;
- `RemoteSessionExpectedDeviceAdmissionEligibleContinuation` for eligible custody;
- `ExpectedDeviceSchedulingAuthorityGrant` without changing its visibility or representation;
- existing acknowledgement-result type unchanged.

The future source stage must not add a second eligible-continuation struct.

## 6. Exact classification law

The only eligible live completion is:

`Ok(RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop::SchedulingTerminal(terminal_outcome))`

where borrowed:

`terminal_outcome.scheduling_result().is_ok()`

is true.

Explicitly ineligible are:

- `Err(RemoteSessionSpawnedWorkerJoinError::...)`;
- `Ok(...::Cancelled)`;
- `Ok(...::Failed(...))`, preserving the exact fallible lifecycle error family;
- `Ok(...::SchedulingTerminal(...))` whose borrowed scheduling result is `Err(ExpectedDeviceSchedulingAuthorityDerivationError)`.

Requester acknowledgement success/failure is orthogonal to scheduling eligibility and must not affect the eligibility predicate.

## 7. Borrow-first preservation law

The future classifier must determine eligibility by **borrowing the original full completion first**.

For every ineligible case, especially `SchedulingTerminal + scheduling_result == Err(...)`, it must then move the **untouched original full completion** into:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome::Ineligible(...)`

inside the existing C03e-RP fallible receipt.

It must not destructure and reconstruct an ineligible scheduling terminal.

It must not flatten a fallible `Failed(...)` result into the historical infallible failure family.

It must not project a scheduling derivation error away from its orthogonal acknowledgement result.

## 8. Eligible branch consumption law

Only after the borrowed predicate has proven exact `SchedulingTerminal + Ok(grant)` may the future classifier consume the scheduling terminal carrier.

On that branch only:

1. consume terminal `into_parts()` exactly once;
2. require the already-proven scheduling result to be `Ok(grant)`;
3. move the exact grant into the existing `RemoteSessionExpectedDeviceAdmissionEligibleContinuation`;
4. move the exact acknowledgement result into that same continuation;
5. preserve requester callback `DeviceId` unchanged as requester correlation.

The classifier must not call `ExpectedDeviceSchedulingAuthorityGrant::into_parts()`.

The classifier must not inspect either grant field.

No `EligibleTerminal` handoff receipt is constructed by this stage because no handoff disposition has yet been determined.

## 9. Identity and one-shot authority law

Requester callback `DeviceId` remains requester-side authenticated identity/correlation only.

It is never target expected identity.

Target expected `DeviceId` remains sealed inside the exact one-shot scheduling grant until a later request-construction boundary.

Requester scheduling `SessionId` remains sealed in that grant and remains distinct from any future target admission `SessionId`.

The future stage must not:

- clone or copy the grant;
- reconstruct or remint a grant;
- replay or roll back scheduling consumption;
- return the grant to shared requester authority;
- derive target identity from requester callback `DeviceId`;
- generate a target admission `SessionId`;
- generate a PRWM authentication request ID.

## 10. Fallible provenance law

The future classifier must preserve exact fallible requester-lifecycle provenance.

`RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop::Failed(...)` remains an ineligible full completion stored by value in the C03e-RP fallible receipt.

The stage must not convert it to `RequesterRendezvousProductionDurableSchedulingWorkerStop::Failed(...)` or any bounded projection.

A verifier-time/production-durable ingress failure and an existing requester-response failure therefore remain distinguishable according to their existing fallible error type.

## 11. Producer boundary remains generic and unspecialized

C03e-RN remains the higher endpoint-owner forwarding boundary and remains generic over `Receipt` with caller-owned borrowed `&mut H` producer custody.

C03e-RQ selects no specialization or invocation of that generic producer path.

The future classification source stage must not:

- construct a producer;
- invoke C03e-RN with the fallible receipt;
- store, clone or move producer ownership;
- create a second producer future;
- spawn a hidden/detached producer task;
- mutate lower cooperative scheduling-driver ordering.

## 12. Shutdown behavior remains separately gated

Explicit supervisor shutdown remains the sole supervisor-shutdown authority.

C03e-RQ selects no concrete fallible shutdown-suppression mapper.

The future classifier source stage must not terminally dispose an eligible scheduling grant as `SuppressedOnShutdown`.

Shutdown suppression and eligible-grant terminal disposal remain separate later gates after classifier/continuation materialization.

## 13. Production channel/backpressure law remains deferred

Eventual production handoff remains constrained to:

- one bounded Tokio MPSC channel;
- capacity exactly `1`;
- exactly one higher-owned production sender;
- receiver created once and moved once;
- enqueue only through `sender.send(request).await`;
- full channel means asynchronous backpressure.

C03e-RQ selects no channel/sender construction and no producer send body.

Forbidden here and in the immediate classifier source stage:

- sender clone;
- `try_send`;
- `blocking_send`;
- callback `block_on`;
- alternate/unbounded/retry queue;
- hidden producer task;
- second producer future.

## 14. Request construction remains deferred

C03e-RQ selects no:

- target admission `SessionId` source;
- expected-device PRWM authentication request-ID source;
- production dispatcher construction or transfer;
- verifier-time source binding;
- `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- request send;
- request retry or reconstruction.

Those remain separately gated after eligible continuation exists.

## 15. Exact source non-actions for the immediate future stage

The selected future source materialization must perform no:

- second Rust path mutation;
- existing type visibility widening;
- historical source-line deletion unless an unavoidable compiler-only local correction is separately evidenced and remains within the exact one-file ceiling;
- lower scheduling stop mutation;
- scheduling-terminal carrier mutation;
- scheduling-grant mutation;
- live request construction;
- channel/sender construction;
- producer specialization/invocation;
- higher process/runtime caller migration;
- endpoint/listener/dial/bootstrap/readiness activation;
- database/auth/control-plane mutation;
- Cargo/lockfile/workflow/Android-source mutation;
- packaging/deployment;
- merge/ready-for-review;
- branch deletion/reset/rebase/squash/force/history rewrite.

## 16. C03e-RQ documentation-only law

C03e-RQ itself changes only this contract document.

It performs zero Rust/source/runtime mutation and assigns no source implementation in this checkpoint.

The selected source materialization must be a later fresh checkpoint after C03e-RQ is fully evidence-closed.

## 17. Validation law

All C03e-RQ PASS claims must bind only to the exact final C03e-RQ head after this documentation-only commit.

`SKIPPED` is never PASS.

If no Android workflow is registered for the docs-only exact head, no Android PASS may be claimed or inherited.

## 18. Immutable evidence law

After exact-head validation succeeds, C03e-RQ must publish exactly one immutable audit artifact to canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The audit bytes must freeze internal state as:

`SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`

Publication requires:

1. exact-title zero-collision search immediately before upload;
2. exactly one raw upload;
3. metadata verification;
4. raw readback byte/hash/final-LF identity;
5. exact-title singleton verification;
6. exactly one current revision with `previousRevisionId = null`;
7. PR-body closure binding after publication;
8. no rewrite of immutable published bytes.

## 19. Stop boundary

After C03e-RQ selection is validated, evidence-published and PR-body-closed:

`STOP`

Do not create or materialize the selected classifier source checkpoint inside C03e-RQ closure.

No successor token is assigned by this contract.