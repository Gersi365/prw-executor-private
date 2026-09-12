# C03e-RO — Production-Durable Post-Auth Fallible-Verifier-Time Expected-Device Handoff Receipt Representation Source-Seam Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_RECEIPT_REPRESENTATION_SOURCE_SEAM_SELECTION`

## 1. Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-RN:

- head `4ddbcecfb6dd43cce809f7ef28d4775fa14c6379`;
- tree `856862a520202e1d4aaa232b602715fe1a7c8bba`;
- exact higher endpoint-owner source blob `0c89e5f7100df6324d037931af0a3f09e21751e0`;
- PR #604 remains draft/open/unmerged/mergeable and evidence-closed;
- immutable RN Drive evidence ID `1RtepB4GPYSaO5L2TFkxYsqkOZ7j6rfNo`.

C03e-RN materialized only the generic higher endpoint-owner producer forwarding seam and explicitly deferred concrete receipt representation, live classification, shutdown suppression composition, channel ownership, request construction/send, producer specialization, higher caller migration and runtime activation.

## 2. Historical ordering authority

Historical production scheduling chain establishes the required ordering:

1. C03e-OP materialized generic higher endpoint-owner producer forwarding;
2. C03e-OQ selected concrete handoff receipt representation;
3. C03e-OR materialized only that representation;
4. only then did C03e-OS select live-completion eligibility classification and eligible-continuation custody.

C03e-RO preserves that order for the fallible-verifier-time lane. It does not skip directly from generic producer forwarding to classification or producer specialization.

## 3. Fresh exact-source findings

The current higher endpoint-owner source already contains the historical verifier-time-agnostic bounded disposition:

`RemoteSessionExpectedDeviceAdmissionHandoffDisposition`

with exactly four zero-data variants:

- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

That disposition owns no scheduling grant, request, identifier, dispatcher, verifier-time provider, sender, channel, endpoint, retry handle or authority-bearing payload. Its semantics are handoff-terminal semantics, not verifier-time semantics, so the future fallible receipt representation must reuse it rather than duplicate it.

The historical receipt outcome is not reusable because its `Ineligible(...)` arm owns the infallible scheduling stop:

`Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The fallible lane instead has the distinct exact terminal type:

`RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop`.

That stop preserves fallible requester-lifecycle failure provenance while sharing the same non-Clone scheduling terminal carrier:

`RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome`.

Its scheduling result and requester acknowledgement result remain orthogonal.

## 4. Selected next source boundary

C03e-RO selects exactly one future source boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_RECEIPT_REPRESENTATION_SOURCE_MATERIALIZATION`

The source stage is separate from this selection checkpoint.

## 5. Hard source ceiling

The future source-materialization checkpoint may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Required predecessor blob for that path:

`0c89e5f7100df6324d037931af0a3f09e21751e0`

If correct materialization requires any second Rust path, visibility widening outside the existing private sibling boundary, live classification, scheduling-grant consumption/disposal, producer/channel construction, request construction/send or broader lifecycle change, `STOP` and return to selection.

## 6. Selected future representation

The future source stage may add only two new boundary-private fallible types, plus narrowly necessary local lint annotations/comments:

### 6.1 Fallible receipt outcome

A private outcome family equivalent in semantic shape to historical C03e-OR but preserving the exact fallible terminal type.

Expected semantic shape:

- `Ineligible(Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>)`;
- `EligibleTerminal { acknowledgement_result: Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>, disposition: RemoteSessionExpectedDeviceAdmissionHandoffDisposition }`.

The exact type name may follow existing local naming style but must unambiguously distinguish the fallible-verifier-time receipt family from the historical infallible receipt family.

### 6.2 Fallible receipt carrier

One private non-`Copy`/non-`Clone` carrier owning exactly:

- requester callback `DeviceId` correlation; and
- one fallible receipt outcome.

No scheduling grant may be retained in the receipt.

## 7. Reuse law for bounded disposition

The future source stage must reuse the existing private:

`RemoteSessionExpectedDeviceAdmissionHandoffDisposition`

It must not add a parallel fallible disposition enum merely to rename the same four terminal handoff states.

Semantics remain exact:

- `Enqueued` means queue acceptance only;
- `ConstructionFailed` means the already-consumed continuation reached terminal construction failure;
- `ChannelClosed` means the one send attempt could not enqueue because the receiver was closed;
- `SuppressedOnShutdown` means shutdown recovery terminally suppressed a would-be live handoff.

None authorizes retry, remint, replay, rollback, alternate channel, sender clone or scheduling-grant reconstruction.

## 8. Exact by-value ineligible custody

The future `Ineligible(...)` arm must retain the exact original full fallible scheduling-stop/join result by value:

`Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

It must not project that value into a lossy enum merely to exclude eligibility statically.

This representation stage does not itself prove non-eligibility. A later separately gated classifier must perform that proof before constructing the `Ineligible` receipt from a live completion.

No live completion may be inspected or classified in the receipt-representation materialization checkpoint.

## 9. Eligible-terminal receipt law

The future eligible-terminal receipt state may contain only:

- requester acknowledgement result:
  `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`;
- the existing bounded handoff disposition.

It must not contain:

- `ExpectedDeviceSchedulingAuthorityGrant`;
- target expected `DeviceId`;
- requester scheduling `SessionId`;
- target admission `SessionId`;
- authentication request ID;
- dispatcher;
- verifier-time source/provider;
- request;
- sender/channel;
- endpoint/transport;
- retry authority.

## 10. Identity and authority preservation

Requester callback `DeviceId` remains requester-side authenticated identity/correlation only. It is never target expected identity.

Target expected `DeviceId` remains obtainable only from one consumed construction-eligible scheduling grant in a later separately gated stage.

Requester scheduling `SessionId` remains distinct from future target admission `SessionId`.

The scheduling grant remains one-shot, non-`Copy`, non-`Clone`. No clone, copy, reconstruction, remint, replay, refund or rollback is authorized.

Requester acknowledgement success/failure remains orthogonal to scheduling eligibility.

Fallible requester-lifecycle failure provenance remains exact and must not be flattened into the historical infallible failure family.

## 11. Producer boundary remains generic

C03e-RN remains unchanged and generic over `Receipt`.

The future receipt-representation source checkpoint must not:

- specialize the C03e-RN method with the new concrete receipt;
- construct a producer closure;
- invoke the generic producer path;
- store or clone a producer;
- create a second producer future;
- alter lower cooperative scheduling behavior.

## 12. Backpressure and channel laws remain deferred

No channel or sender is selected for materialization here.

Existing eventual production law remains:

- exactly one bounded Tokio MPSC channel;
- capacity exactly `1`;
- receiver create once / move once;
- one higher-owned production sender;
- no sender clone;
- enqueue through `sender.send(request).await` only;
- full channel means ordinary async backpressure;
- no `try_send`;
- no `blocking_send`;
- no callback `block_on`;
- no hidden/detached producer task;
- no alternate/unbounded/retry queue.

## 13. Shutdown and endpoint laws remain lower-owned

Explicit supervisor shutdown remains the sole supervisor-shutdown authority.

C03e-RN and its lower C03e-RL/C03e-RJ chain remain sole owners of cooperative producer/admission/worker quiescence and endpoint close/wait-idle ordering.

The future receipt representation adds no shutdown mapper and performs no endpoint action.

## 14. Explicitly deferred after this selection

Still separately gated:

- source materialization of the selected fallible receipt representation;
- fallible live-completion eligibility classification;
- fallible eligible-continuation carrier/custody;
- scheduling-result discriminant inspection;
- scheduling terminal `into_parts()` consumption;
- scheduling-grant terminal disposal on suppressed/construction-failed/channel-closed paths;
- concrete fallible shutdown-suppression mapper;
- production channel/sender ownership;
- target admission `SessionId` generation;
- independent nonzero expected-device PRWM authentication request-ID generation;
- concrete status-only dispatcher construction/transfer;
- concrete higher verifier-time source binding;
- `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- `sender.send(request).await` producer body;
- generic producer specialization with concrete receipt;
- higher process/runtime caller migration;
- requester cleanup/candidate/reachability continuation;
- target dial/listener/bootstrap/readiness/executable activation;
- auth/database/control-plane mutation;
- Cargo/lockfile/workflow/Android-source mutation;
- packaging/service/repository configuration;
- deployment;
- merge/ready-for-review;
- branch deletion/reset/rebase/squash/force/history rewrite.

## 15. Selection checkpoint rules

C03e-RO itself is documentation-only.

It must contain:

- exactly one commit relative to C03e-RN;
- exactly one added documentation path;
- zero Rust/source/runtime changes;
- zero Cargo/lockfile/workflow/Android/packaging/deployment changes.

Exact-head validation and immutable evidence publication are required before this selection may close.

`SKIPPED` is not PASS.

After evidence closure: `STOP`.
