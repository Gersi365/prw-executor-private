# C03e-OS — Production requester/rendezvous expected-device admission scheduling-terminal live-completion eligibility classification + eligible-continuation custody source-seam selection

Status: `SELECTION — VALIDATION_PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_LIVE_COMPLETION_ELIGIBILITY_CLASSIFICATION_ELIGIBLE_CONTINUATION_CUSTODY_SOURCE_SEAM_SELECTION`

## 1. Exact predecessor authority

This checkpoint begins only from the exact closed C03e-OR source-materialization head:

- branch: `phase-152-c03e-or-production-requester-rendezvous-expected-device-admission-scheduling-terminal-async-producer-handoff-receipt-representation-source-materialization`;
- head: `4eea1aad7a0bb052617ea5bacff67432c4cfd367`;
- tree: `71ae5438a6ded5b9a5834bca118ea4527cff63cd`;
- authorized `remote_session_endpoint_lifecycle_runtime.rs` blob: `3fc8c92afa54d1d929df7c4b50d2fc782cb6d9fa`;
- PR #531 remains draft/open/unmerged;
- C03e-OR is `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

Live GitHub state remains authoritative over historical handoff text.

## 2. Fresh concurrency guard

Before assigning C03e-OS, the continuation audit established:

- no `phase-152-c03e-os...` branch existed;
- no newer user PR existed after #531;
- the proposed C03e-OS contract path did not exist on exact C03e-OR;
- semantic branch searches for classification / eligible continuation returned no competing successor;
- exact C03e-OR source remained unchanged;
- the source evidence needed for one-file classification was directly re-read.

Only after those zero-result guards was C03e-OS assigned.

## 3. Fresh exact-source findings

### 3.1 Existing C03e-OR receipt representation

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs` already contains the boundary-private dormant representation selected by C03e-OQ and materialized by C03e-OR:

- `RemoteSessionExpectedDeviceAdmissionHandoffDisposition` with exactly:
  - `Enqueued`;
  - `ConstructionFailed`;
  - `ChannelClosed`;
  - `SuppressedOnShutdown`;
- `RemoteSessionExpectedDeviceAdmissionHandoffReceiptOutcome` with exactly:
  - `Ineligible(Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>)`;
  - `EligibleTerminal { acknowledgement_result, disposition }`;
- `RemoteSessionExpectedDeviceAdmissionHandoffReceipt` with exactly requester `DeviceId` correlation plus one outcome.

The receipt/outcome are private and neither require nor derive `Clone`/`Copy`.

### 3.2 Exact scheduling-terminal source shape

`requester_rendezvous_retained_custody_dr_continuation.rs` exposes to its parent/siblings the exact production scheduling terminal result:

`RequesterRendezvousProductionDurableSchedulingWorkerStop`

with exactly:

- `Cancelled`;
- `Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError)`;
- `SchedulingTerminal(RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome)`.

`RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome` owns two orthogonal channels:

1. `Result<ExpectedDeviceSchedulingAuthorityGrant, ExpectedDeviceSchedulingAuthorityDerivationError>`;
2. `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`.

Its existing sibling-visible accessors allow:

- a borrowed inspection of the scheduling-result discriminant through `scheduling_result()`; and
- exact by-value transfer through `into_parts()`.

No visibility widening or source change in that sibling module is required for the selected classification seam.

### 3.3 Exact scheduling grant law

`ExpectedDeviceSchedulingAuthorityGrant` is already sibling-visible from `shared_requester_rendezvous_authority.rs`.

It is intentionally neither `Copy` nor `Clone` and owns exactly:

- requester scheduling `SessionId` provenance; and
- target expected `DeviceId`.

Possession proves terminal scheduling consumption already occurred for that operation key. The grant contains no target admission `SessionId`, authentication request ID, dispatcher, verifier-time value/provider, sender, channel, endpoint or reachability authority.

The selected classifier does not open this grant through `requester_session_id()`, `target_device_id()` or `into_parts()`; it only moves the exact `Ok(grant)` by value into an eligible continuation.

## 4. Selected next source boundary

C03e-OS selects the next separately gated source boundary as:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_LIVE_COMPLETION_ELIGIBILITY_CLASSIFICATION_ELIGIBLE_CONTINUATION_CUSTODY_SOURCE_MATERIALIZATION`

The immediate future hard source ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

No second Rust path is selected.

## 5. Selected one-file representation and classifier

The future source materialization may add only the minimum private dormant types/helper required to classify one live producer callback completion without constructing a request or invoking a producer.

### 5.1 Eligible continuation carrier

A private non-`Copy`, non-`Clone` carrier may be introduced conceptually as:

`RemoteSessionExpectedDeviceAdmissionEligibleHandoffContinuation`

It owns exactly:

- requester callback `DeviceId` correlation;
- one exact `ExpectedDeviceSchedulingAuthorityGrant` by value;
- one exact `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>` by value.

It owns no handoff disposition yet because no handoff attempt has occurred.

It owns no request, sender, receiver, channel, admission `SessionId`, authentication request ID, dispatcher, verifier-time provider, endpoint, transport, reachability authority, retry token or other production input.

### 5.2 Classification result carrier

A private classification carrier may be introduced conceptually as:

`RemoteSessionExpectedDeviceAdmissionHandoffCompletionClassification`

with exactly two semantic families:

- `Ineligible(RemoteSessionExpectedDeviceAdmissionHandoffReceipt)`;
- `Eligible(RemoteSessionExpectedDeviceAdmissionEligibleHandoffContinuation)`.

This classification itself authorizes no producer call and no request construction.

### 5.3 Pure live completion classifier

A private synchronous helper may be introduced conceptually as:

`classify_remote_session_expected_device_admission_handoff_completion(...)`

Inputs are exactly:

- requester callback `DeviceId` by value; and
- exact raw completion by value:
  `Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The classifier returns exactly one classification carrier.

It performs no async work, I/O, task spawning, channel operation, authority lookup, registry lookup, policy evaluation, random generation, timing observation, endpoint mutation or lifecycle drive.

## 6. Exact classification matrix

### 6.1 Abnormal worker join

Input:

`Err(RemoteSessionSpawnedWorkerJoinError::...)`

Result:

`Ineligible` receipt retaining the exact original raw `Err(...)` by value.

No scheduling state exists to inspect.

### 6.2 Cancellation

Input:

`Ok(RequesterRendezvousProductionDurableSchedulingWorkerStop::Cancelled)`

Result:

`Ineligible` receipt retaining the exact original raw `Ok(Cancelled)` by value.

### 6.3 Pre-scheduling lifecycle failure

Input:

`Ok(RequesterRendezvousProductionDurableSchedulingWorkerStop::Failed(error))`

Result:

`Ineligible` receipt retaining the exact original raw typed failure by value.

### 6.4 Scheduling terminal with derivation failure

Input:

`Ok(RequesterRendezvousProductionDurableSchedulingWorkerStop::SchedulingTerminal(outcome))`

where `outcome.scheduling_result()` is `Err(...)`.

Result:

`Ineligible` receipt retaining the exact original full `SchedulingTerminal(outcome)` by value.

The classifier must not destructure and reconstruct this ineligible terminal merely to observe the error. It may inspect the scheduling-result discriminant by borrow and then move the untouched original completion into the C03e-OR `Ineligible` receipt.

This preserves both exact scheduling-derivation failure custody and exact orthogonal acknowledgement-result custody without requiring a constructor that is private to the requester module.

### 6.5 Scheduling terminal with exact grant

Input:

`Ok(RequesterRendezvousProductionDurableSchedulingWorkerStop::SchedulingTerminal(outcome))`

where `outcome.scheduling_result()` is `Ok(grant)`.

Result:

`Eligible` continuation.

Only on this path may the classifier consume `outcome.into_parts()` exactly once and move:

- exact `ExpectedDeviceSchedulingAuthorityGrant`; and
- exact acknowledgement result

into the eligible continuation beside requester callback `DeviceId` correlation.

No grant field is inspected or extracted in this checkpoint.

## 7. Eligibility law

Construction eligibility remains exactly the C03e-NZ law:

Only:

`SchedulingTerminal + scheduling_result == Ok(grant)`

is eligible for later expected-device request construction.

Explicitly ineligible:

- abnormal join;
- `Cancelled`;
- `Failed(...)`;
- `SchedulingTerminal` with scheduling derivation `Err(...)`.

Requester acknowledgement success/failure is orthogonal to scheduling eligibility. An acknowledgement error does not revoke, reconstruct, remint or invalidate an already-issued scheduling grant.

## 8. Receipt law at this stage

For every ineligible completion, the classifier may construct exactly one C03e-OR receipt with:

- the same requester callback `DeviceId`; and
- `RemoteSessionExpectedDeviceAdmissionHandoffReceiptOutcome::Ineligible(exact_original_completion)`.

No ineligible completion receives a handoff disposition because no eligible handoff was attempted.

For an eligible completion, this stage must not prematurely construct `EligibleTerminal` receipt state. The one-shot grant remains live in the eligible continuation until a later separately gated handoff/disposal boundary determines exactly one terminal disposition.

## 9. One-shot grant custody law

The classifier may move one exact eligible grant into the eligible continuation, but it must not:

- clone or copy the grant;
- call `ExpectedDeviceSchedulingAuthorityGrant::into_parts()`;
- read target `DeviceId` from the grant;
- read requester scheduling `SessionId` from the grant;
- construct target admission `SessionId`;
- construct authentication request ID;
- construct or move a dispatcher;
- bind verifier-time custody;
- construct an expected-device request;
- enqueue a request;
- return the grant to scheduling authority;
- remint, replay or replace the grant;
- roll back terminal scheduling consumption.

Dropping/disposal of an eligible continuation without a handoff disposition is not selected here; shutdown suppression remains a separate boundary.

## 10. Identity law

Requester callback `DeviceId` remains requester-side authenticated identity/correlation only.

It must never be substituted for the target expected `DeviceId`.

The target expected `DeviceId` remains available only inside the exact one-shot scheduling grant until a later request-construction stage consumes the grant.

Requester scheduling `SessionId` inside the grant remains distinct from future target admission `SessionId`.

No request ID acquires authority semantics; request IDs remain correlation only.

## 11. Producer / channel law remains unchanged

The exact C03e-OP higher endpoint-owner producer forwarding remains generic over `Receipt` and takes only caller-owned `&mut H`.

C03e-OS does not select specialization or invocation of that generic method.

The existing production handoff law remains:

- exactly one bounded Tokio MPSC channel;
- capacity exactly `1`;
- exactly one higher-owned production sender;
- receiver created once and moved once;
- eventual enqueue only through `sender.send(request).await`;
- a full channel means asynchronous backpressure;
- no sender clone;
- no `try_send`;
- no `blocking_send`;
- no callback `block_on`;
- no hidden/detached producer task;
- no alternate/unbounded/retry queue;
- no second producer future.

None of those behaviors is materialized by the selected classifier.

## 12. Shutdown law remains unchanged

Explicit supervisor shutdown remains the sole supervisor-shutdown authority.

The future C03e-OS-selected source stage does not add the concrete shutdown suppression mapper.

For shutdown-recovered completions, the existing lower cooperative driver still performs existing scheduling peer disposition before invoking the caller-supplied synchronous suppression mapper.

A later separately gated suppression boundary must decide how one eligible continuation is terminally suppressed and mapped to `SuppressedOnShutdown` while consuming/discarding the grant exactly once and without producer invocation.

C03e-OS does not authorize that behavior.

## 13. Source-materialization allowance

The immediate source successor may add only:

- the private eligible-continuation carrier;
- the private two-family classification carrier;
- one private synchronous pure classifier;
- the minimum sibling-private imports required to name the exact grant/outcome types;
- narrowly scoped `dead_code`, `large_enum_variant` or similar local lint acknowledgement only if compiler/Clippy requires it and no semantic refactor is needed;
- focused same-file tests of the classification matrix if practical without widening source scope.

No existing endpoint lifecycle method needs modification.

No lower executor/cooperative-driver source change is selected.

## 14. Mandatory STOP conditions

The future source materialization must STOP and return to selection if correctness or compilation requires any of:

- a second Rust path;
- visibility widening in `requester_rendezvous_retained_custody_dr_continuation.rs`;
- visibility widening in `shared_requester_rendezvous_authority.rs`;
- a lossy projected ineligible error family;
- reconstruction of an ineligible `SchedulingTerminal` after destructuring;
- grant clone/copy/remint/replay/rollback;
- grant field extraction;
- shutdown suppression behavior;
- producer specialization/invocation;
- production channel/sender construction;
- request construction/send;
- target admission `SessionId` generation;
- authentication request-ID generation;
- dispatcher construction/transfer;
- verifier-time production wiring;
- endpoint/lifecycle mutation;
- runtime activation;
- broader architectural redesign.

## 15. Still separately gated after this selection

- source materialization of this classifier/eligible-continuation seam;
- concrete shutdown suppression mapping;
- terminal disposal of an eligible grant on suppressed/construction-failed/channel-closed paths;
- actual higher producer/channel owner construction;
- target admission `SessionId` source under the selected server-local CSPRNG law;
- independent nonzero expected-device PRWM authentication request-ID source;
- NB status-only dispatcher production construction/transfer;
- verifier-time source/interface compatibility;
- actual `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- actual `sender.send(request).await` producer body;
- specialization/invocation of the generic producer path with the concrete receipt;
- higher process/runtime caller migration;
- requester cleanup/candidate/reachability continuation;
- target dial/listener/bootstrap/readiness/executable activation;
- deployment.

No ordering among independent request-construction input gates is invented by C03e-OS.

## 16. Explicit non-actions in C03e-OS

C03e-OS is selection/documentation only.

No Rust/source/runtime mutation.
No live classifier source materialization.
No grant inspection/consumption/disposal.
No receipt constructor source addition.
No shutdown suppression mapper.
No producer specialization or invocation.
No channel/sender construction or clone.
No request construction/send.
No target admission `SessionId` generation.
No authentication request-ID generation.
No dispatcher production wiring.
No verifier-time mutation.
No callback `block_on`.
No `try_send`.
No `blocking_send`.
No hidden/detached spawn.
No retry queue.
No alternate/unbounded queue/channel.
No second producer future.
No caller migration.
No requester cleanup/candidate/reachability continuation.
No target dial/listener/bootstrap/readiness/runtime/executable activation.
No Cargo/lockfile/workflow/Android source mutation.
No deployment.
No merge.
No PR ready-for-review conversion or closure.
No branch deletion.
No force update.
No rebase/squash/history rewrite.
No repository configuration/ruleset/permission mutation.

## 17. Validation and closure rule

All validation claims for C03e-OS must bind only to the exact final C03e-OS head.

Expected documentation-only validation behavior:

- PRW Rust Validation must be evaluated if triggered;
- any path-filtered workflow reported `SKIPPED` remains `SKIPPED`, not PASS;
- Android PASS is claimed only if an Android workflow actually runs successfully on the exact final head;
- no validation result from C03e-OR or another SHA may be inherited.

After exact-final-head validation, immutable Drive evidence must be published and read back under the canonical PRW evidence parent before marking the selection evidence-recorded/closed.

Keep the future C03e-OS PR draft/open/unmerged.

No successor token is assigned by this document.
A fresh exact-head/concurrency audit is mandatory before any selected source materialization.

After evidence closure: `STOP`.
