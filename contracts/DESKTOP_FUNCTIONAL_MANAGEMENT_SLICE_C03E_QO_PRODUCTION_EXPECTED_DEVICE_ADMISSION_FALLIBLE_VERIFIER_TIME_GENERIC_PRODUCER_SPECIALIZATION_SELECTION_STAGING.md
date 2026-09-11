# Desktop Functional Management Slice — C03e-QO

## Production Expected-Device Admission Fallible Verifier-Time Generic Producer Specialization Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_GENERIC_PRODUCER_SPECIALIZATION_SELECTION`

Selected future boundary:
`PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_GENERIC_PRODUCER_SPECIALIZATION_SOURCE_MATERIALIZATION`

## 1. Checkpoint purpose

C03e-QO selects only the next dormant same-file generic producer-specialization boundary after the evidence-closed C03e-QN fallible-verifier-time async producer send / receipt composition source materialization.

The selected future source checkpoint may compose the already-existing private QN live producer body with the already-existing C03e-OP generic endpoint-owner producer forwarding seam and the already-existing shutdown-suppression receipt mapper. It may specialize only the generic producer and receipt types needed for that dormant endpoint-module composition.

C03e-QO is documentation-only. It does not materialize the specialization adapter, capture the private QJ dispatcher source, construct or split QL channel custody, invoke a higher owner, move a receiver into QF, add receipt-observation policy, activate runtime behavior, merge, or deploy.

## 2. Exact predecessor authority

Authoritative predecessor:
`C03e-QN — Production Expected-Device Admission Fallible Verifier-Time Async Producer Send / Receipt Composition Source Materialization`.

QN branch:
`phase-152-c03e-qn-production-expected-device-admission-fallible-verifier-time-async-producer-send-receipt-composition-source-materialization`

Exact QN final head:
`7fab8593f9287ccc4262bff931bb609c4a38a3dd`

Exact QN final tree:
`f13839c40c1e234cf62f68e1ce14b18d57a677e6`

Exact QN endpoint source blob:
`d0c096116a74f35eec23bd189afadf8e5e3220c3`

QN PR:
`#579`

QN PR status:
`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

QN PR remains draft/open/unmerged/mergeable.

QN immutable audit:
`C03E_QN_PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ASYNC_PRODUCER_SEND_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION_AUDIT_2026-09-11.md`

Canonical QN Drive ID:
`1nDA1J7cOwzXFebBuxzX1fao9gS4uPgix`

Canonical parent:
`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

QN audit bytes:
`19532`

QN audit SHA-256:
`0a7b1c44ce330681073a9a1b85e11b1ee4f8e25a986622b2c9863d945ca66585`

## 3. Fresh post-QN closure proof

Before QO selection, QN was re-audited after evidence publication and PR-body closure.

Fresh live checks proved:
- QN branch still pointed to exact final head `7fab8593f9287ccc4262bff931bb609c4a38a3dd`;
- QN tree remained `f13839c40c1e234cf62f68e1ce14b18d57a677e6`;
- PR #579 remained draft/open/unmerged/mergeable and evidence-closed;
- exact-final-head QN Rust and Android validation remained SUCCESS;
- C02f-AD and C02f-AE remained SKIPPED, not PASS;
- canonical QN Drive exact-title search returned exactly one artifact, Drive ID `1nDA1J7cOwzXFebBuxzX1fao9gS4uPgix`;
- integrated `main` remained unchanged;
- no QO branch or title-scoped QO PR existed immediately before branch creation.

Therefore QO begins from the exact evidence-closed QN state rather than from an inferred or stale predecessor.

## 4. Stable integrated baseline

Integrated `main` remains outside this checkpoint.

Exact `main` head:
`7c993fa93977a0bb84e0d030874eee7fd0cae77f`

Exact `main` tree:
`63b8e59ca53797fdea6b95432e16f35eaf473604`

QO must not modify `main`.

## 5. Exact-current QN live producer proof

Exact QN endpoint source:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

contains one private dormant async helper:
`produce_remote_session_expected_device_admission_with_fallible_verifier_time<D, F>(...)`.

Its logical inputs are exactly:
- requester callback `DeviceId` correlation;
- exact scheduling-aware requester completion result;
- temporary mutable borrow of an infallible dispatcher factory `F: FnMut() -> D`;
- immutable borrow of one typed `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<D, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource>>`.

Its output is exactly one private:
`RemoteSessionExpectedDeviceAdmissionHandoffReceipt`.

For one invocation QN performs exactly:
1. live-completion classification once;
2. exact ineligible receipt return with no dispatcher construction;
3. dispatcher-factory invocation once only after eligibility;
4. QH request construction once;
5. exact `ConstructionFailed` receipt return with no send;
6. one `sender.send(request).await` only for `Constructed`;
7. send success -> `Enqueued`;
8. receiver closure -> terminal drop of returned unsent request and `ChannelClosed`;
9. exactly one terminal receipt return.

QN creates no channel, clones no sender, performs no retry/requeue, spawns no producer task, and does not call `try_send`, `blocking_send`, callback-local `block_on`, reserve, or try-reserve.

## 6. `Constructed` remains distinct from `Enqueued`

QH `Constructed` proves only local request construction.

QN creates `Enqueued` only after `sender.send(request).await` returns `Ok(())`.

A full channel leaves the producer future pending under ordinary Tokio asynchronous backpressure.

A full channel is not `Enqueued`, not `ChannelClosed`, and not shutdown suppression.

QO must preserve this distinction exactly.

## 7. Exact-current private receipt proof

The exact endpoint source retains private:
- `RemoteSessionExpectedDeviceAdmissionHandoffDisposition`;
- `RemoteSessionExpectedDeviceAdmissionHandoffReceiptOutcome`;
- `RemoteSessionExpectedDeviceAdmissionHandoffReceipt`.

The bounded dispositions remain exactly:
- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

QN returns that private receipt directly.

QO does not widen receipt visibility, add a public receipt type, add serialization, add logging, add persistence, or select a higher-level receipt projection.

## 8. Exact-current shutdown-suppression proof

The same endpoint source already contains private synchronous:
`map_remote_session_expected_device_admission_shutdown_suppression(...)`.

It reuses the existing live-completion classifier.

For an ineligible completion it returns the exact existing ineligible receipt unchanged.

For a shutdown-recovered eligible completion it terminally disposes the sealed scheduling grant without opening or inspecting grant fields, preserves the exact acknowledgement result, and returns only `SuppressedOnShutdown`.

It constructs no dispatcher, invokes no QH request construction, and performs no send.

QO selects this exact mapper for the generic shutdown-suppression callback. It does not replace, wrap, reinterpret, or broaden that mapper.

## 9. Exact-current C03e-OP generic endpoint producer seam

The exact endpoint source already contains:
`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer(...)`.

The source comment records that C03e-OP materialized the ON-selected dormant higher endpoint-owner producer forwarding before separately gated concrete receipt and producer/channel composition.

That existing seam receives:
- one by-value expected-request receiver;
- caller-owned `producer: &mut H`;
- synchronous `suppress_on_shutdown: Q`;
- synchronous `observe_receipt: O`;
- all existing endpoint lifecycle, authority, authentication, admission-timing, rejection, and admission-failure inputs.

Its producer bound is generic over the exact scheduling-aware completion input and an abstract `Receipt`:
`H: std::ops::AsyncFnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> Receipt`.

The shutdown mapper must return that same `Receipt` type.

The observer consumes that same `Receipt` type.

The seam already delegates to the executor driver and owns no channel construction, concrete request construction, concrete receipt construction, identifier generation, dispatcher construction, timing source, task, listener, readiness publication, or executable activation.

QO must not rewrite that existing generic seam.

## 10. Exact-current generic driver producer custody

The underlying generic producer driver retains at most one pending producer future:
`pending_producer: Option<Pin<Box<H::CallRefFuture<'_>>>>`.

When there is no pending producer, its biased select gives supervisor shutdown precedence over receiving a new completion.

When a live completion wins, the driver starts exactly one producer future for that completion.

The driver does not spawn that producer future into a detached task.

The future remains owned inside the serial collection driver.

QO must preserve this ownership shape and must not introduce a second pending producer future, detached producer task, parallel producer queue, or duplicate completion delivery.

## 11. Shutdown-first precedence before producer start

If supervisor shutdown wins before a new completion has been accepted for live producer work, the driver enters shutdown recovery without starting another live producer invocation.

Completions recovered during shutdown are mapped through the supplied synchronous shutdown-suppression callback and then observed.

Under QO, that callback is selected to be exactly:
`map_remote_session_expected_device_admission_shutdown_suppression`.

Therefore a recovered eligible completion that never entered QN live production remains forbidden from dispatcher construction, QH construction, and send.

## 12. Already-started producer drain law

If one live producer future has already started and supervisor shutdown later wins, the existing driver does not retroactively reinterpret that in-flight producer as `SuppressedOnShutdown`.

The exact already-started producer future remains owned by the driver and is drained to exactly one receipt during shutdown cleanup.

Consequently an already-started QN invocation may complete its existing QH/send path according to QN semantics after shutdown has become selected by the driver.

QO must not:
- cancel that producer future;
- invoke shutdown suppression on the same completion;
- start a replacement producer;
- retry the send;
- duplicate the completion;
- remap its terminal receipt.

This is existing driver behavior, not new runtime policy selected by QO.

## 13. Proven missing specialization

After QN, the following concrete pieces exist independently in the same endpoint module:
- QN live producer body returning the private handoff receipt;
- shutdown-suppression mapper returning the same private handoff receipt;
- C03e-OP generic endpoint producer seam parameterized over `H`, `Q`, `O`, and `Receipt`.

What remains absent is one dormant same-file specialization adapter that binds:
- `Receipt` to `RemoteSessionExpectedDeviceAdmissionHandoffReceipt`;
- live `H` to one local async producer closure delegating to QN;
- shutdown `Q` to the exact existing shutdown-suppression mapper;
- `O` to a caller-supplied same-call receipt observer.

That missing type-level and callback composition is the exact QO-selected future boundary.

## 14. Selected future source ceiling

Immediate future source-materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

No second Rust path is authorized.

If correct implementation requires modifying `linux_bootstrap.rs`, `production_durable_capability_higher_owner_custody.rs`, another endpoint/executor source, `main.rs`, Cargo/lockfile, workflow, Android, packaging, deployment, or repository configuration: STOP and return to selection.

## 15. Selected future adapter shape

The future QP source checkpoint may add exactly one private dormant same-file adapter equivalent in authority to:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_expected_device_fallible_verifier_time_producer(...)`.

The exact identifier is not authoritative; the authority surface and behavior selected here are authoritative.

The adapter may consume the endpoint owner once and accept the same existing lifecycle inputs needed by the C03e-OP generic producer seam, with expected-request request time specialized to:
`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource`.

It may additionally accept only:
- one temporary mutable borrow of a caller-owned infallible dispatcher factory `G: FnMut() -> D`;
- one immutable borrow of the exact typed expected-request sender;
- one caller-supplied receipt observer consuming the existing private concrete receipt.

It may receive the exact expected-request receiver by value from its caller.

It may not create, clone, split, replace, persist, or return channel endpoints.

## 16. Selected local live producer closure

Inside the future private adapter, exactly one local producer closure may be formed.

For each live invocation that the generic driver provides to that closure, it must delegate exactly once to:
`produce_remote_session_expected_device_admission_with_fallible_verifier_time(...)`.

The closure may capture only the temporary dispatcher-factory borrow and typed sender borrow required by QN.

The closure must not capture:
- QJ dispatcher-source concrete type;
- QL channel custody concrete type;
- receiver ownership;
- endpoint transport;
- supervisor shutdown controller/signal;
- durable authority internals;
- scheduling grants outside the completion value;
- retry state;
- queue capacity state;
- status-refresh state;
- task handles.

## 17. Dispatcher-factory law remains abstract

QO does not capture the private QJ dispatcher source.

The future adapter remains generic over one caller-supplied infallible factory equivalent to:
`FnMut() -> D`.

QN remains sole owner of the point at which that factory is invoked: only after one completion is classified eligible.

The future specialization adapter must not pre-create a dispatcher, call the factory for ineligible completions, create a second dispatcher, refresh status, retry factory invocation, or cache dispatcher state.

Concrete capture of:
`LinuxAgentProductionRemoteCapabilityDispatcherSource`

remains separately gated.

## 18. QJ privacy law remains unchanged

`LinuxAgentProductionRemoteCapabilityDispatcherSource` remains private to `linux_bootstrap.rs`.

QO does not import, re-export, alias, widen, move, or duplicate that type.

QO does not call `LocalAgentStatusSnapshot::current(...)`.

QO does not select a new snapshot source or dynamic refreshed-state model.

A later separately gated production composition may capture the exact existing QJ source and supply a temporary factory equivalent to its existing `new_dispatcher(&self)`.

## 19. QL channel custody law remains unchanged

The existing higher-owner source retains private non-cloneable:
`LinuxAgentProductionExpectedDeviceAdmissionChannel<D, T>`.

Its `new()` remains the only selected capacity-one channel constructor and performs exactly one:
`mpsc::channel(1)`.

Its consuming `into_parts(self)` remains the only selected ownership split.

QO does not call either method.

The future QP specialization adapter likewise must not construct or split QL custody. It accepts caller-supplied sender/receiver endpoints only.

Concrete QL construction and exact sender/receiver transfer remain separately gated.

## 20. Sole-sender law remains unchanged

The future QP adapter receives only an immutable sender borrow for QN use.

Neither QO nor QP may clone the sender.

Forbidden:
- `Sender::clone()`;
- spare sender;
- fallback sender;
- retry sender;
- shutdown sender;
- metrics sender;
- second channel;
- alternate queue;
- unbounded queue.

The sole production sender must remain move-once authority owned by a later separately gated higher composition.

## 21. Backpressure law remains unchanged

QN remains sole owner of the live enqueue operation:
`sender.send(request).await`.

The future QP adapter adds no queue operation around it.

Full capacity remains ordinary asynchronous Tokio backpressure.

Forbidden:
- `try_send`;
- `blocking_send`;
- callback-local `block_on`;
- reserve/try-reserve alternate protocol;
- timeout queue escape;
- busy-loop retry;
- requeue;
- hidden/detached send task.

## 22. QH construction order remains unchanged

The future specialization adapter does not construct requests directly.

QN continues to delegate exactly once to QH for eligible live completion.

QH remains sole owner of:
1. fresh target admission SessionId;
2. fresh nonzero authentication request ID;
3. exact fallible verifier-time function-pointer binding without sampling;
4. scheduling-grant consumption only after identifier success;
5. target DeviceId extraction only from the grant;
6. requester scheduling SessionId discard as scheduling provenance;
7. exactly one request construction attempt.

No retry, remint, replay, refund, rollback, alternate identifier, status refresh, or second construction attempt is selected.

## 23. Concrete receipt specialization law

The future adapter may specialize the generic endpoint seam's `Receipt` type only to:
`RemoteSessionExpectedDeviceAdmissionHandoffReceipt`.

It may not introduce a wrapper receipt, option, result envelope, boxed receipt, serialized receipt, or projected public receipt.

The live producer and shutdown mapper already return exactly that private type.

The adapter must forward each returned receipt exactly once to the supplied observer callback.

## 24. Receipt observer law

QO selects no higher-level receipt interpretation policy.

The future adapter may accept one caller-supplied synchronous observer equivalent to:
`FnMut(RemoteSessionExpectedDeviceAdmissionHandoffReceipt)`

only because the adapter remains private inside the same endpoint module.

The adapter must pass that observer unchanged to the existing generic seam.

It must not itself:
- inspect receipt fields;
- branch on disposition;
- log receipt contents;
- persist receipts;
- emit metrics from receipt disposition;
- retry based on receipt disposition;
- convert receipt to readiness or worker-success evidence;
- expose receipt outside the module;
- widen receipt type visibility.

Higher receipt projection/observation policy remains separately gated.

## 25. Receiver law remains unchanged

The future private adapter may receive exactly one existing typed expected-request receiver by value and forward it once into the existing generic endpoint producer seam.

It must not:
- create a receiver;
- replace a receiver;
- recreate the channel;
- clone or duplicate receiver custody;
- consume requests itself;
- move the receiver into QF directly;
- expose receiver accessors.

The exact production receiver transfer from QL toward the existing QF production-source companion remains separately gated at a higher composition boundary.

## 26. No new task or concurrency boundary

The future specialization adapter creates no task.

It must not call `tokio::spawn`, `spawn_local`, executor spawn helpers, detached futures, join sets, background loops, or thread primitives.

The local producer closure remains caller-stack / driver-owned future state under the existing generic collection driver.

The existing driver remains sole owner of pending-producer custody and shutdown draining.

## 27. No duplicate completion path

For one scheduling-aware completion, the existing generic driver selects either:
- one live producer invocation; or
- shutdown-recovered suppression before producer start.

If a live producer has already started, its future drains to its one receipt and the same completion must not also enter shutdown suppression.

The future QP adapter must not clone, copy, cache, replay, or route the completion through both paths.

## 28. Ineligible completion law remains unchanged

For an ineligible live completion, QN returns the exact existing ineligible receipt before dispatcher construction, QH construction, or send.

For an ineligible shutdown-recovered completion, the existing suppression mapper returns the exact existing ineligible receipt.

The future specialization adapter must not distinguish or reinterpret those receipts.

## 29. Shutdown-suppressed eligible law remains unchanged

A shutdown-recovered eligible completion that has not started live producer work must enter only the existing suppression mapper.

That mapper terminally disposes the sealed grant and returns `SuppressedOnShutdown` without dispatcher construction, QH request construction, or send.

The future adapter must not invoke QN for that same completion.

## 30. Already-started QN send is not retroactively suppressed

If the generic driver has already started QN for one live completion and shutdown then wins, the existing pending producer future remains the terminal path for that completion.

Its existing send may remain pending under capacity-one backpressure until it resolves according to existing driver/channel lifecycle behavior.

The future adapter must not create a timeout, cancellation path, alternate queue, second send, or `SuppressedOnShutdown` rewrite for that already-started invocation.

## 31. Identity and authority separation

Requester callback DeviceId remains requester-side correlation only.

Target DeviceId remains obtainable only from the QH-consumed scheduling grant.

Requester scheduling SessionId remains distinct from target admission SessionId.

Target admission SessionId and authentication request ID remain QH-generated only.

Verifier time remains the exact fallible function pointer bound by QH and is not sampled during producer composition or request construction.

Dispatcher factory possession is dispatcher-construction capability only after eligibility.

Sender possession is enqueue authority only.

Receipt possession is terminal handoff observation data only.

None of these independently proves authentication, admission, authorization, reachability, worker insertion, endpoint success, readiness, or deployment state.

## 32. No runtime activation

Neither QO nor the selected future QP source materialization adds any executable caller.

Forbidden in QP:
- `main.rs` wiring;
- endpoint bind invocation;
- listener creation;
- readiness publication;
- process-signal consumption;
- process/runtime caller migration;
- requester/rendezvous higher-owner invocation;
- background task activation;
- network activation;
- deployment.

The specialization remains dormant until a later separately gated caller invokes it.

## 33. Exact QP one-file source scope

The selected future QP source checkpoint may modify only:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Allowed change:
- one private dormant same-file specialization adapter;
- narrowly focused same-file tests only if practical and if they do not require a second source path or runtime activation.

No other source file is selected.

## 34. QP implementation guardrails

Before any future QP source write, re-read the exact QO head and endpoint source blob.

After materialization, prove direct QO -> QP diff contains only the selected endpoint source path.

Prove the new code:
- delegates live production to QN rather than reimplementing it;
- delegates shutdown suppression to the exact existing mapper;
- delegates endpoint producer lifecycle to the existing generic C03e-OP seam;
- creates at most one local producer closure;
- creates no task;
- creates no channel;
- clones no sender;
- performs no additional send;
- performs no QJ concrete source capture;
- performs no QL split;
- adds no receipt interpretation policy;
- adds no runtime caller.

If rustfmt/Clippy requires correction, use forward-only normal commits; no reset/rebase/squash/force update/history rewrite.

## 35. Rust type/lifetime feasibility gate

QO selects authority and composition semantics, not an exemption from Rust's borrow/type rules.

Future QP must prove that one local async producer closure can borrow the caller-supplied dispatcher factory and sender for the duration required by the existing `AsyncFnMut` generic producer seam without widening ownership or introducing a task.

If the exact selected composition cannot compile without:
- moving to a second source file;
- widening private receipt visibility;
- cloning sender authority;
- boxing or making the dispatcher source global;
- adding synchronization/cache state;
- changing the generic driver signature;
- adding a detached task;
then QP must STOP and return to a new selection checkpoint rather than expanding scope.

## 36. Explicitly deferred after future QP

Still separately gated after the selected QP materialization:
1. concrete QJ dispatcher-source capture into a production-owned factory;
2. QL capacity-one channel custody construction and consuming sender/receiver ownership split;
3. exact higher-owner composition of those production sources with the QP specialization;
4. exact receiver transfer into the existing QF fallible-verifier-time production-source companion path;
5. higher receipt projection/observation policy;
6. requester/rendezvous higher-owner integration;
7. process/runtime caller migration;
8. executable caller;
9. listener/readiness/process/runtime/network activation;
10. merge/deployment.

QO does not invent a final ordering among independent later integration gates beyond the authority and custody laws already selected.

## 37. Repository / platform exclusions

QO and the selected QP boundary do not authorize changes to:
- authentication protocols;
- trust roots or certificates;
- private keys or credentials;
- RBAC / policy semantics;
- database schemas or data;
- control-plane APIs;
- repository settings;
- branch protection;
- workflow permissions;
- package publication;
- deployment configuration.

## 38. Validation authority

C03e-QO is valid only for its exact final head after the docs-only contract commit.

No predecessor CI result may be inherited as QO PASS authority.

If pull-request workflows register for QO, only runs bound to the exact QO final head may support validation claims.

A skipped workflow must be recorded as SKIPPED, not PASS.

If no workflow registers for a class such as Android on the exact docs-only head, the audit must state that no exact-head run was observed rather than inheriting an earlier PASS.

## 39. Immutable evidence law

After exact-head validation, publish exactly one immutable QO audit under canonical Drive parent:
`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`.

The audit must record:
- exact QO branch/head/tree;
- exact contract blob;
- direct QN -> QO topology;
- exact PR state;
- exact-head CI enumeration;
- unchanged `main`;
- selected future QP source ceiling and guardrails;
- pre-upload zero-match check;
- frozen local byte count and SHA-256;
- Drive ID/parent/title/size;
- raw Drive readback byte/hash verification;
- post-upload exact-title singleton check.

The frozen audit should retain internal status:
`SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`.

After immutable evidence is published and verified, the PR body may record:
`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

## 40. PR state law

The QO PR must remain:
- draft;
- open;
- unmerged.

QO does not authorize ready-for-review conversion, merge, PR closure, branch deletion, deployment, restart, or runtime activation.

## 41. STOP boundary

C03e-QO ends after selection validation, immutable evidence publication, PR evidence-closure update, and final closure re-audit.

Do not create C03e-QP inside C03e-QO closure.

STOP after C03e-QO.