# C03e-OU — Production requester-rendezvous expected-device admission scheduling-terminal shutdown-suppression eligible-grant terminal-disposal receipt-composition source seam selection

## Status

`SELECTION — SOURCE_MATERIALIZATION_NOT_AUTHORIZED_BY_THIS_DOCUMENT`

Selected boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_SHUTDOWN_SUPPRESSION_ELIGIBLE_GRANT_TERMINAL_DISPOSAL_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION`

This checkpoint is documentation-only. It selects one narrowly bounded future source seam and does not materialize it.

## Authoritative predecessor

Repository: `Gersi365/prw-executor-private`

Repository ID: `1334911207`

Predecessor checkpoint: `C03e-OT`

Exact predecessor head:
`fbb5fd328772a0e4ec14d25c07bac42bea0380d3`

Exact predecessor tree:
`c38ddad8ba9ab0096db5a0cadd3407abbdf8404f`

Exact predecessor endpoint-owner source blob:
`4a7d3660d463bd9529c8da26587e0708115515cd`

Predecessor PR: `#533`

Predecessor lifecycle at fresh audit:
`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

The predecessor remains draft/open/unmerged. No merge, deployment, runtime activation, branch deletion, force update, history rewrite, or repository-configuration mutation is implied or authorized here.

## Fresh concurrency and source-shape findings

Before assigning C03e-OU, live GitHub authority was re-read.

The exact C03e-OT branch remained at the predecessor head and tree above.

PR #533 remained the newest user PR and remained draft/open/unmerged.

The `phase-152-c03e-ou` branch namespace returned zero results before assignment.

Semantic PR searches for shutdown suppression, eligible-grant terminal disposal, and `SuppressedOnShutdown` receipt composition returned no competing successor. Matches were historical predecessor contracts/PRs describing the still-deferred seam, not a materialized or selected C03e-OU successor.

The exact endpoint-owner source was re-read on the C03e-OT head and retained the exact predecessor blob above.

The exact lower cooperative producer implementation was also audited through its materialized GitHub patch. Its shutdown worker drain ordering is already:

1. recover one exact scheduling-aware worker completion;
2. run the existing scheduling peer disposer/projection first;
3. call the caller-owned synchronous `suppress_on_shutdown(DeviceId, Result<...>) -> Receipt` mapper;
4. pass that exact returned receipt to the synchronous receipt observer.

C03e-OU does not change that ordering or lower driver.

## Existing concrete receipt representation

The exact endpoint-owner source already contains the dormant C03e-OR receipt representation.

`RemoteSessionExpectedDeviceAdmissionHandoffDisposition` contains exactly:

- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

`RemoteSessionExpectedDeviceAdmissionHandoffReceiptOutcome` contains exactly two semantic families:

- `Ineligible(Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>)`;
- `EligibleTerminal { acknowledgement_result, disposition }`.

`RemoteSessionExpectedDeviceAdmissionHandoffReceipt` owns requester callback `DeviceId` correlation plus exactly one private outcome.

The receipt is neither `Copy` nor `Clone` and contains no scheduling grant, request, identifier, dispatcher, timing source, sender, channel, endpoint, retry handle, target identity authority, or reachability authority.

## Existing live-completion classification and eligible custody

The exact endpoint-owner source also already contains the dormant C03e-OT classification seam.

`classify_remote_session_expected_device_admission_live_completion(...)` accepts:

- requester callback `DeviceId`; and
- the exact raw `Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

It returns exactly one of:

- `RemoteSessionExpectedDeviceAdmissionLiveCompletionClassification::Ineligible(existing C03e-OR receipt)`; or
- `RemoteSessionExpectedDeviceAdmissionLiveCompletionClassification::Eligible(RemoteSessionExpectedDeviceAdmissionEligibleContinuation)`.

Only `SchedulingTerminal` whose borrowed `scheduling_result()` is `Ok(...)` is eligible.

Abnormal join, `Cancelled`, `Failed(...)`, and `SchedulingTerminal` with scheduling derivation `Err(...)` remain ineligible.

The eligible continuation owns exactly:

- requester callback `DeviceId` correlation;
- one exact one-shot `ExpectedDeviceSchedulingAuthorityGrant` by value;
- one exact requester acknowledgement result by value.

The classifier does not inspect or extract either scheduling-grant field.

## Exact one-shot scheduling-grant law

Fresh exact source confirms `ExpectedDeviceSchedulingAuthorityGrant` is one irreversibly consumed operation-scoped scheduling authority.

It is intentionally neither `Copy` nor `Clone`.

It contains exactly requester scheduling `SessionId` provenance and target expected `DeviceId`.

Its creation occurs only after the requester-session/target key has been inserted into the terminal scheduling-consumption ledger.

Therefore shutdown suppression cannot refund, reissue, reconstruct, remint, replay, or roll back scheduling authority.

For the selected shutdown-only seam, terminal disposal means relinquishing the exact issued grant by value without inspecting or extracting either field. The future mapper must not call `ExpectedDeviceSchedulingAuthorityGrant::into_parts()`.

A lexical `drop(scheduling_grant)` or equivalently unobserved by-value destruction is the selected terminal-disposal semantics. No cleanup or rollback call exists or is selected.

## Selected source boundary

C03e-OU selects the next separately gated source boundary as:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_SHUTDOWN_SUPPRESSION_ELIGIBLE_GRANT_TERMINAL_DISPOSAL_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION`

The immediate future hard source ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

No second Rust path is selected.

The future source stage may add only one private synchronous pure shutdown-suppression receipt mapper, plus only the minimum local dormant lint acknowledgement necessary for that helper.

A representative semantic shape is:

`fn suppress_remote_session_expected_device_admission_handoff_on_shutdown(requester_device_id, completion) -> RemoteSessionExpectedDeviceAdmissionHandoffReceipt`

The exact helper name may vary only if required by local naming consistency; its inputs, output, ordering, custody, and side-effect constraints are fixed by this selection.

## Exact mapper contract

The future mapper must accept exactly the same semantic inputs already required by the generic lower-driver shutdown mapper boundary:

- requester callback `DeviceId`; and
- exact raw `Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

It must return exactly:

`RemoteSessionExpectedDeviceAdmissionHandoffReceipt`.

It must call the existing C03e-OT live-completion classifier exactly once.

### Ineligible family

If classification returns `Ineligible(receipt)`, the mapper must return that exact receipt unchanged.

It must not inspect, project, reconstruct, clone, copy, rewrap, or translate the raw ineligible completion.

It must not manufacture `SuppressedOnShutdown` for an ineligible completion because no eligible one-shot continuation exists to suppress.

### Eligible family

If classification returns `Eligible(continuation)`, the mapper must consume that continuation exactly once.

It may extract only the continuation's three already-selected private custody fields:

- requester callback `DeviceId` correlation;
- the exact one-shot scheduling grant;
- the exact acknowledgement result.

It must terminally dispose of the exact scheduling grant without reading or extracting either grant field.

It must then construct exactly one `RemoteSessionExpectedDeviceAdmissionHandoffReceipt` whose:

- requester correlation is the exact requester callback `DeviceId` retained by the eligible continuation;
- outcome is `EligibleTerminal`;
- acknowledgement result is the exact retained acknowledgement result unchanged;
- disposition is exactly `RemoteSessionExpectedDeviceAdmissionHandoffDisposition::SuppressedOnShutdown`.

The eligible terminal receipt must contain no scheduling grant and no source from which the disposed grant can be reconstructed.

## Shutdown ordering law

C03e-OU selects no lower-driver mutation.

The existing lower cooperative scheduling driver remains the sole owner of shutdown drain ordering:

`scheduling peer disposition -> synchronous suppression mapper -> synchronous receipt observer`.

The future C03e-OU-selected mapper performs only the middle synchronous mapping step.

It must not call the peer disposer itself.

It must not call the receipt observer itself.

It must not request or detect supervisor shutdown itself.

It must not close the receiver, endpoint, transport, sender, or channel.

It must not cancel or drain workers.

Explicit supervisor shutdown remains the sole supervisor-shutdown authority.

## Identity law

Requester callback `DeviceId` remains requester-side authenticated identity/correlation only.

It is never substituted for target expected `DeviceId`.

Target expected `DeviceId` remains sealed inside the one-shot grant until a separately gated request-construction boundary. In the selected shutdown-suppression path no request construction occurs, so target identity is intentionally never extracted.

Requester scheduling `SessionId` remains distinct from any future target admission `SessionId`.

Request IDs remain correlation only unless separately authorized by their own exact lane.

## Acknowledgement orthogonality law

Requester acknowledgement success/failure remains orthogonal to scheduling eligibility and shutdown handoff disposition.

For one eligible shutdown-suppressed continuation, the exact acknowledgement result must be retained unchanged in the resulting `EligibleTerminal` receipt regardless of whether that acknowledgement result is success or error.

An acknowledgement error does not revoke, reconstruct, refund, replace, remint, replay, or invalidate the already-issued scheduling grant before that grant is terminally disposed.

## Producer and backpressure law preserved

The existing C03e-OP higher endpoint-owner producer forwarding remains unchanged and fully generic over `Receipt` with caller-owned `&mut H`.

C03e-OU does not select producer specialization or invocation.

C03e-OU does not select a concrete producer closure.

Eventual production handoff remains exactly:

- one bounded Tokio MPSC channel;
- capacity exactly `1`;
- one higher-owned production sender;
- receiver created once and moved once;
- enqueue only through `sender.send(request).await`;
- full channel means asynchronous backpressure.

Still forbidden:

- sender clone;
- `try_send`;
- `blocking_send`;
- callback `block_on`;
- hidden or detached producer task;
- alternate queue/channel;
- unbounded queue/channel;
- retry queue;
- second producer future.

## Dispositions deliberately not materialized by this selected seam

The selected shutdown-suppression mapper may construct only `SuppressedOnShutdown` for an eligible continuation.

It must not construct:

- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`.

Those dispositions require later separately gated producer/request-construction/channel outcomes.

`Enqueued` continues to mean queue acceptance only. It never proves receiver consumption, admission, authentication, worker success, endpoint success, or reachability success.

## Explicitly deferred after C03e-OU selection

Still separately gated:

- source materialization of the selected shutdown-suppression mapper itself;
- request-construction input boundary selection;
- target admission `SessionId` generation/source;
- independent nonzero expected-device PRWM authentication request-ID generation/source;
- NB status-only dispatcher production construction/transfer;
- verifier-time source/interface compatibility;
- actual `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- exact terminal disposal/composition for request-construction failure;
- actual higher producer/channel owner construction;
- actual sole-sender producer closure;
- `sender.send(request).await`;
- `ChannelClosed` receipt composition;
- `Enqueued` receipt composition;
- specialization/invocation of the generic producer path with the concrete receipt;
- higher process/runtime caller migration;
- requester cleanup/candidate/reachability continuation;
- target dial/listener/bootstrap/readiness/executable activation;
- deployment.

No ordering among independent later request-construction input gates is invented here.

## STOP conditions for future source materialization

The selected future source stage must STOP and return to selection if correctness or compilation requires any of the following:

- a second Rust path;
- visibility widening;
- modification of the lower cooperative driver;
- modification of an existing endpoint lifecycle method;
- modification of the generic producer forwarding method;
- scheduling-grant field inspection or extraction;
- `ExpectedDeviceSchedulingAuthorityGrant::into_parts()`;
- grant clone/copy/reconstruction/remint/replay/rollback/refund;
- target admission `SessionId` generation;
- authentication request-ID generation;
- dispatcher production construction/transfer;
- verifier-time production wiring;
- expected-device request construction;
- channel or sender construction;
- producer specialization or invocation;
- asynchronous work;
- callback `block_on`;
- `try_send` or `blocking_send`;
- task spawn;
- retry/fallback/alternate queue;
- endpoint close or runtime activation;
- broader lifecycle behavior.

## Validation boundary

Validation for this documentation-only selection must bind only to the exact final C03e-OU documentation head.

At minimum, the PRW Rust validation lane must complete successfully on that exact head before the selection can be called validated.

Any Android validation run may be recorded only if it is actually triggered on the exact same head; absence of an Android run must not be converted into an Android PASS claim.

`SKIPPED` is not `PASS`.

No validation result from C03e-OT or another SHA may be inherited as C03e-OU exact-head validation.

## Evidence boundary

After exact-head validation, immutable Drive evidence must be published under the existing canonical evidence parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The evidence artifact must record:

- exact C03e-OT predecessor head/tree/source blob;
- fresh pre-assignment concurrency guard;
- exact C03e-OU final head/tree/contract blob;
- exact OT -> OU topology and changed-path ceiling;
- source-shape findings supporting this selected seam;
- exact-final-head workflow enumeration;
- `SKIPPED IS NOT PASS` where applicable;
- exact evidence filename/parent/MIME/size/hash/readback;
- post-publication GitHub and successor-namespace guards;
- explicit deferred scope and non-actions.

## Non-actions of C03e-OU selection

C03e-OU is documentation-only.

No Rust/source/runtime mutation.

No shutdown-suppression mapper materialization.

No scheduling-grant disposal in live source.

No `EligibleTerminal` live receipt composition.

No grant field inspection/extraction/clone/copy/reconstruction/remint/replay/rollback/refund.

No producer specialization or invocation.

No production channel/sender construction or clone.

No request construction/send.

No target admission `SessionId` generation.

No authentication request-ID generation.

No dispatcher production transfer.

No verifier-time production wiring.

No callback `block_on`.

No `try_send`.

No `blocking_send`.

No hidden/detached task.

No retry or alternate/unbounded queue/channel.

No second producer future.

No lifecycle method mutation.

No caller migration.

No runtime activation.

No Cargo/lockfile/workflow/Android-source mutation.

No deployment.

No merge.

No PR ready conversion or PR closure.

No branch deletion.

No force update.

No rebase/squash/history rewrite.

No repository configuration/ruleset/permission mutation.

No successor token is assigned by this selection.

After C03e-OU closure: `STOP` and require a fresh exact-head/concurrency audit before any selected source materialization.