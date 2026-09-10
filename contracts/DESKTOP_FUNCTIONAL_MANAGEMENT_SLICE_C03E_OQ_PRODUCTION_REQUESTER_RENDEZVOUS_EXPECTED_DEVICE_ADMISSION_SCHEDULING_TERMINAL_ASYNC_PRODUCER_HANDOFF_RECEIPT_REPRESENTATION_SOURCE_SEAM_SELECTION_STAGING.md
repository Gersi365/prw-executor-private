# Desktop Functional Management Slice C03e-OQ

## Production requester/rendezvous expected-device admission scheduling-terminal async producer handoff receipt representation source seam selection

Status: `SELECTION — VALIDATION_PENDING`
Date: 2026-09-10

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_RECEIPT_REPRESENTATION_SOURCE_SEAM_SELECTION`

Authoritative predecessor: exact closed C03e-OP head `1aeb6e57498026a6648612c04f82853a3da8283b`, tree `c215562631121e63b708624040ee20afcd317e70`.

This checkpoint is documentation-only. It selects the smallest independently materializable concrete receipt representation required by the already-materialized generic producer path. It does not materialize Rust source, classify a live scheduling completion, construct a production channel/sender, construct an expected-device admission request, install a producer, migrate a caller, activate runtime behavior, merge, or deploy.

## 1. Decision

`BOUNDARY_PRIVATE_HANDOFF_RECEIPT_REPRESENTATION_SELECTED / REQUESTER_CORRELATION_RETAINED / INELIGIBLE_EXACT_COMPLETION_CUSTODY_RETAINED_BY_VALUE / ELIGIBLE_ACKNOWLEDGEMENT_PLUS_BOUNDED_DISPOSITION_RETAINED / ELIGIBLE_GRANT_FORBIDDEN_FROM_RECEIPT / NO_CLASSIFICATION_OR_PRODUCER_WIRING / ONE_FILE_ADDITIVE_SOURCE_CEILING`

The immediate next source boundary is:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_RECEIPT_REPRESENTATION_SOURCE_MATERIALIZATION`.

The future source materialization may touch exactly one existing Rust path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`.

No second Rust path is selected.

## 2. Exact current source authority

All source observations are from exact C03e-OP head `1aeb6e57498026a6648612c04f82853a3da8283b`.

Exact endpoint-owner file:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact C03e-OP blob:
`aea6e4cc88d02a439f6ca493209893dfb4c29345`.

The exact file already imports and can name the production scheduling worker stop and generic join error used by the producer boundary. It is inside the same private `remote_session_capability_runtime` ownership graph as the scheduling terminal/grant and requester acknowledgement types. No visibility widening is needed merely to declare the receipt representation.

The exact C03e-OP method is materially present:
`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer(...)`.

Its producer remains generic:
`H: AsyncFnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> Receipt`.

It receives only caller-owned `&mut H`, keeps `Receipt` generic, forwards the synchronous shutdown-suppression mapper and synchronous receipt observer, calls the exact C03e-OM executor producer endpoint-lifecycle adapter once, and leaves endpoint close/wait-idle below that boundary.

Therefore the missing concrete receipt type is now independently materializable without changing the generic producer driver or endpoint-owner forwarding method.

## 3. Prior selected semantic authority

C03e-OF selected one private handoff receipt envelope with requester correlation and two semantic families:

1. Ineligible: retain the exact original non-eligible scheduling-stop/join result by value, including typed derivation and acknowledgement channels where present.
2. Eligible terminal: retain the exact requester acknowledgement result by value plus exactly one bounded disposition:
   - `Enqueued`
   - `ConstructionFailed`
   - `ChannelClosed`
   - `SuppressedOnShutdown`

The eligible scheduling grant must never be retained in the receipt. It is one-shot authority and must be consumed or deterministically disposed before eligible terminal reporting.

`Enqueued` means only `Sender::send(request).await` returned `Ok(())`. It does not prove receiver consumption, admission, authentication, worker success, endpoint success, or reachability success.

`ConstructionFailed`, `ChannelClosed`, and `SuppressedOnShutdown` remain terminal for that one-shot scheduling continuation. None authorizes retry, rollback, remint, replay, alternate channel, or request reconstruction.

The endpoint lifecycle `Result<(), RemoteSessionPersistentCollectionConfigError>` remains lifecycle/configuration status and must not absorb producer outcome.

## 4. Selected Rust representation shape

The future one-file source stage should add only a small private representation family in `remote_session_endpoint_lifecycle_runtime.rs`.

Conceptually, select a bounded disposition enum equivalent to:

`RemoteSessionExpectedDeviceAdmissionHandoffDisposition`

with exactly four variants:
- `Enqueued`
- `ConstructionFailed`
- `ChannelClosed`
- `SuppressedOnShutdown`

The disposition is zero-data. It stores no request bytes, target identity, path, endpoint, credential, grant, sender, channel, retry handle, producer token, timing source, dispatcher, SessionId, request ID, or error payload.

Select one concrete receipt carrier equivalent to:

`RemoteSessionExpectedDeviceAdmissionHandoffReceipt`

that owns:
- exact requester `DeviceId` correlation; and
- one private outcome family.

Select one module-private outcome family equivalent to:

`RemoteSessionExpectedDeviceAdmissionHandoffReceiptOutcome`

with exactly two semantic families:

### Ineligible

Retain the exact original by-value completion:

`Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

The representation does not project a cancellation, lifecycle failure, scheduling derivation failure, acknowledgement failure, or join failure into a new lossy public classification. The exact existing typed result remains owned by the receipt.

This representation is valid only for completion values that are non-eligible for expected-device request construction. An eligible `SchedulingTerminal` containing `Ok(ExpectedDeviceSchedulingAuthorityGrant)` must never enter the Ineligible receipt family.

Because preserving the exact original result and statically excluding the eligible variant are competing representation goals, OQ does not invent a second projected ineligible enum merely for type-level exclusion. Instead, the raw ineligible constructor remains module-private. A later separately gated classification/producer composition must prove non-eligibility before invoking it. No parent/crate/public constructor is selected.

### Eligible terminal

Retain exactly:
- requester `DeviceId` correlation;
- exact `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>` acknowledgement disposition by value; and
- exactly one `RemoteSessionExpectedDeviceAdmissionHandoffDisposition`.

The eligible receipt contains no `ExpectedDeviceSchedulingAuthorityGrant` and no source from which that grant can be reconstructed.

No eligible target `DeviceId`, requester scheduling `SessionId`, admission `SessionId`, authentication request ID, dispatcher, verifier time, request body, sender, or retry capability belongs in the receipt.

## 5. Visibility and construction law

The representation remains inside the existing private `remote_session_capability_runtime` graph.

The concrete receipt and bounded disposition may use only the narrow visibility needed by a later higher-owner producer composition. Do not re-export either through the crate public API and do not widen the scheduling terminal/grant or requester acknowledgement types.

The private outcome representation and raw Ineligible constructor should remain module-private so later code cannot bypass the selected eligibility proof merely by constructing an arbitrary receipt from an eligible grant-bearing completion.

A narrowly visible Eligible-terminal constructor is allowed only if it accepts no grant and therefore cannot preserve or return scheduling authority in the receipt.

No `Clone` or `Copy` is selected for the receipt. Exact by-value custody is the law. The bounded zero-data disposition may be `Copy`/`Clone` if ordinary lint/shape conventions make that useful; that does not grant any authority.

No serde/wire representation, persistence, database record, log schema, IPC/public API, or bridge protocol is selected.

## 6. Explicitly deferred classification

OQ does not select or materialize the function that determines whether one producer input is eligible.

A later gate must decide the exact by-value classification from:
`(requester DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>)`

into either:
- an immediate Ineligible receipt preserving the exact original result; or
- an eligible construction continuation owning the exact one-shot scheduling grant plus exact acknowledgement result.

That later classification must preserve the existing law that the requester callback `DeviceId` is requester identity/correlation only. Target expected `DeviceId` comes only from the consumed eligible scheduling grant.

OQ does not add an eligible continuation carrier, inspect `scheduling_result()`, call `into_parts()`, consume a grant, dispose a grant, or map shutdown suppression.

## 7. Explicitly deferred producer/channel composition

C03e-OE remains authoritative for the eventual request channel/sender law:
- exactly one bounded Tokio MPSC channel;
- capacity exactly `1`;
- receiver create-once/move-once custody;
- exactly one production sender owner;
- no sender clone;
- enqueue only `sender.send(request).await`;
- asynchronous backpressure on full capacity;
- no `try_send`;
- no `blocking_send`;
- no callback `block_on`;
- no detached/hidden producer task;
- no unbounded/alternate/retry queue.

OQ materializes none of those future actions.

The already-materialized C03e-OK/OM/OP cooperative producer path remains dormant and generic. OQ does not specialize or invoke it with the concrete receipt.

## 8. Shutdown law preserved

Explicit supervisor shutdown remains the sole endpoint-supervisor shutdown authority.

OQ does not introduce a shutdown mapper. It only selects the concrete representation required by a later mapper.

The later separately gated suppression mapper must preserve C03e-OF semantics:
- no new producer call after quiescence;
- existing scheduling peer disposition first;
- non-eligible drain completion becomes the same exact Ineligible receipt;
- eligible grant is consumed/disposed once and reported as `SuppressedOnShutdown` with exact acknowledgement custody;
- no request construction/send for shutdown-suppressed eligible work;
- pending already-started producer future is preserved to terminal receipt;
- receiver closure remains an effect of already-observed shutdown, not a second shutdown authority.

No shutdown behavior changes in OQ.

## 9. Identity and one-shot authority laws preserved

- requester completion `DeviceId` remains authenticated requester identity/correlation;
- requester completion `DeviceId` never substitutes for target expected `DeviceId`;
- target expected `DeviceId` comes only from one consumed construction-eligible scheduling grant;
- requester scheduling `SessionId` remains distinct from future target admission `SessionId`;
- scheduling grant remains non-`Copy` and non-`Clone`;
- no grant clone/copy/reconstruction/remint/replay;
- no scheduling-consumption rollback;
- request IDs remain correlation only unless separately authorized by their exact producer lane;
- acknowledgement disposition remains orthogonal to scheduling eligibility;
- existing scheduling peer disposition remains before normal producer delivery or shutdown suppression.

## 10. Immediate future source ceiling

After OQ is fully validated/evidence-closed, a fresh audit may assign a successor token for:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_RECEIPT_REPRESENTATION_SOURCE_MATERIALIZATION`.

That future source stage may change exactly one Rust path:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`.

Allowed delta only:
- import the exact requester acknowledgement error type if required;
- add the private bounded four-variant disposition;
- add the concrete private receipt carrier and private two-family outcome representation;
- add minimal private/narrow constructors/accessors needed to express and test the representation without classifying live producer input;
- add focused same-file representation tests if practical;
- add only narrowly required reasoned dormant lint acknowledgement.

It must not:
- change the existing C03e-OP producer-forwarding method signature/body;
- change the C03e-OM executor adapter;
- change the C03e-OK cooperative driver;
- add a completion classifier;
- consume/inspect scheduling grants in runtime code;
- add a shutdown suppression mapper;
- create a channel or sender;
- construct a producer closure;
- construct/send an expected-device request;
- add SessionId/request-ID/dispatcher/timing inputs;
- migrate a caller;
- activate a listener/runtime/executable path.

If source correctness or compilation requires a second Rust path, visibility widening of requester/grant types, producer/channel construction, classification logic, or broader lifecycle change: `STOP` and return to selection.

## 11. Validation requirements for the future source stage

Any later receipt source materialization must bind PASS only to its exact final head.

At minimum verify:
- exact predecessor merge base;
- final changed-path ceiling is exactly the selected endpoint file;
- no historical source-line deletion unless explicitly required and audited;
- formatting;
- Clippy with warnings denied;
- workspace tests;
- workspace build;
- any Android workflow that actually runs on the final head is reported exactly; a skipped or absent Android run is not a PASS;
- all diagnostic/superseded runs remain clearly separated from final PASS authority.

Representation-focused tests may prove:
- Ineligible receipt can retain a known non-eligible completion by value without `Clone`;
- requester correlation is retained exactly;
- eligible terminal receipt owns acknowledgement result plus each bounded disposition and no grant field;
- receipt itself is not accidentally made `Copy`/`Clone`.

No test requires fabricating an eligible scheduling grant in this representation-only stage. If testing would require changing a sibling source path solely to expose private constructors, do not do so; keep the one-file ceiling and rely on compile/shape tests appropriate to the representation.

## 12. Remaining separately gated dependencies

Still separately gated after OQ:
- concrete receipt source materialization itself;
- eligibility classification / eligible continuation custody;
- shutdown suppression mapping to the concrete receipt;
- actual higher producer/channel owner construction under the capacity-one sole-sender law;
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

No ordering among the remaining independent construction-input gates is invented here.

## 13. Documentation-only non-actions

C03e-OQ changes documentation only.

No Rust/source/runtime mutation.
No receipt Rust type materialization.
No completion classification.
No scheduling grant inspection/consumption/disposal.
No concrete producer installation.
No production channel construction.
No sender construction or clone.
No request construction/send.
No target admission SessionId generation.
No expected-device PRWM authentication request-ID generation.
No NB dispatcher production wiring.
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
No force push/rebase/squash/history rewrite.
No repository configuration/ruleset/permission mutation.

## 14. Closure protocol

Before closing this documentation checkpoint:
- verify exact OP predecessor head/tree unchanged;
- verify exact OQ branch head/tree;
- verify OP -> OQ has exact OP merge base;
- verify exactly one changed documentation path and zero source/runtime paths;
- run exact-final-head CI and classify skipped workflows as skipped, never PASS;
- publish one immutable raw audit in canonical Drive parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- verify exact title/parent/MIME/size/raw-byte SHA-256 readback;
- re-read branch, PR, recent PR chronology and main after publication;
- keep the PR draft/open/unmerged.

No successor token is assigned by this document. A fresh exact-head/concurrency audit is mandatory after OQ closure before any source materialization.

After verified closure: `STOP`.
