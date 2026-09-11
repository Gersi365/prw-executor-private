# Desktop Functional Management Slice — C03e-QM

## Production Expected-Device Admission Fallible Verifier-Time Async Producer Send / Receipt Composition Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ASYNC_PRODUCER_SEND_RECEIPT_COMPOSITION_SELECTION`

Selected future boundary:
`PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ASYNC_PRODUCER_SEND_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION`

## 1. Checkpoint purpose

C03e-QM selects only the next dormant concrete async producer-body boundary after the evidence-closed C03e-QL capacity-one channel-custody source materialization.

The selected future source body will remain inside the existing expected-device endpoint-lifecycle module so it can reuse the already-private live-completion classifier, eligible-continuation custody, QH request-construction composition, constructed-handoff custody, concrete handoff receipt and bounded handoff dispositions without widening any of those interfaces.

C03e-QM is documentation-only. It does not materialize the producer body, invoke a producer, split QL custody, bind the QJ dispatcher source, wire a receiver into QF, add a caller, activate runtime behavior, merge, or deploy.

## 2. Exact predecessor authority

Authoritative predecessor:
`C03e-QL — Production Expected-Device Admission Capacity-One Channel / Sole-Sender Custody Source Materialization`

QL branch:
`phase-152-c03e-ql-production-expected-device-admission-capacity-one-channel-sole-sender-custody-source-materialization`

Exact QL head:
`5001d0e6b772db56b45a1ebef318f89d2633ed6f`

Exact QL tree:
`c2a32460cf0a1e3a03f52b3539fca7b705cdc0ee`

Exact QL higher-owner source blob:
`093cff1e4643f995f0cdc5e337ecfc3bbc2ec582`

QL PR:
`#577`

QL PR is evidence-closed and must remain draft/open/unmerged.

QL immutable audit:
`C03E_QL_PRODUCTION_EXPECTED_DEVICE_ADMISSION_CAPACITY_ONE_CHANNEL_SOLE_SENDER_CUSTODY_SOURCE_MATERIALIZATION_AUDIT_2026-09-11.md`

Canonical QL Drive ID:
`16XrCzowG115E5KQ5LG9FBh6YYS7fJt_S`

Canonical parent:
`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

QL audit bytes:
`11736`

QL audit SHA-256:
`6813da1396af6938c9854751d07e263bc3fb901845baa1162ce6c6f2cceef928`

## 3. Stable integrated baseline

Integrated `main` remains outside this checkpoint and was re-read before selection.

Exact `main` head:
`7c993fa93977a0bb84e0d030874eee7fd0cae77f`

Exact `main` tree:
`63b8e59ca53797fdea6b95432e16f35eaf473604`

QM must not modify `main`.

## 4. Exact-current QL channel custody proof

Exact QL source:
`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

contains one private non-cloneable:
`LinuxAgentProductionExpectedDeviceAdmissionChannel<D, T>`.

It owns exactly:
- one `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- one `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`.

Its `new()` performs exactly one:
`mpsc::channel(1)`.

Its `into_parts(self)` consumes the custody and returns the exact sender and receiver by value.

There is no sender clone, spare sender, second channel, unbounded channel, dynamic capacity, request construction, send, receipt composition, producer invocation, receiver invocation, background task, or runtime activation in QL.

Therefore QM must consume no new channel authority and must not alter QL custody.

## 5. Exact-current producer forwarding proof

Exact QL endpoint source:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

already contains the C03e-OP higher endpoint-owner producer forwarding seam:
`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer(...)`.

That seam already receives:
- caller-owned `producer: &mut H`;
- one by-value expected-request receiver;
- synchronous shutdown suppression returning the same generic receipt type;
- synchronous receipt observation consuming that same receipt type.

The producer bound remains:
`H: std::ops::AsyncFnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> Receipt`.

The generic producer seam is already materialized and must not be rewritten by the immediate future producer-body source checkpoint.

Concrete specialization/invocation of that generic seam remains separately gated after the producer body exists.

## 6. Exact-current private receipt / classification proof

The same exact endpoint source already contains private:
- `RemoteSessionExpectedDeviceAdmissionHandoffDisposition`;
- `RemoteSessionExpectedDeviceAdmissionHandoffReceiptOutcome`;
- `RemoteSessionExpectedDeviceAdmissionHandoffReceipt`;
- `RemoteSessionExpectedDeviceAdmissionEligibleContinuation`;
- `RemoteSessionExpectedDeviceAdmissionLiveCompletionClassification`;
- `classify_remote_session_expected_device_admission_live_completion(...)`;
- `map_remote_session_expected_device_admission_shutdown_suppression(...)`.

Bounded dispositions are exactly:
- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

Only `SchedulingTerminal` with an issued scheduling grant is live-production eligible.

Every ineligible live completion already maps to the exact existing ineligible receipt.

Shutdown-recovered eligible completion already terminally disposes its sealed scheduling grant and maps to `SuppressedOnShutdown`; it must never enter the selected live producer body.

## 7. Exact-current QH request-construction proof

The exact endpoint source already contains:
`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource`

as the exact function-pointer type:
`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

It already contains:
`RemoteSessionExpectedDeviceAdmissionConstructedHandoff<D>`

retaining exactly:
- requester callback `DeviceId` correlation;
- exact acknowledgement result;
- exactly one typed expected-device admission request.

It already contains:
`RemoteSessionExpectedDeviceAdmissionRequestConstructionOutcome<D>`

with exactly:
- `Constructed(...)`;
- `ConstructionFailed(existing receipt)`.

It already contains:
`construct_remote_session_expected_device_admission_request_with_fallible_verifier_time<D>(continuation, dispatcher)`.

QH remains authoritative for request construction and its ordering:
1. target admission `SessionId` source;
2. authentication request-ID source;
3. bind exact fallible verifier-time provider without sampling;
4. only after identifier successes consume the scheduling grant;
5. target DeviceId only from the grant;
6. construct one request with the caller-supplied dispatcher;
7. return constructed handoff.

QM does not alter any QH identifier, grant, verifier-time, or request-construction semantics.

## 8. Exact-current QJ dispatcher-source proof

Exact QL `linux_bootstrap.rs` retains private:
`LinuxAgentProductionRemoteCapabilityDispatcherSource`.

It owns exactly one copied production `LocalAgentStatusSnapshot` captured from the existing production runtime inputs.

Its `new_dispatcher(&self)` returns one fresh existing:
`LinuxAgentProductionRemoteCapabilityDispatcher`.

The source is private to `linux_bootstrap.rs` and C03e-QM deliberately does not widen it or bind it into the producer body.

The selected producer body therefore accepts an abstract caller-supplied infallible dispatcher factory instead of naming or importing the private QJ source.

Concrete QJ-source capture and transfer into that factory remain separately gated.

## 9. Proven missing producer/send composition

Fresh exact-QL endpoint-source inspection proves:
- no typed `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<...>>` exists in the endpoint module;
- `ChannelClosed` has no current composition site beyond its existing enum variant;
- the QH construction helper explicitly remains dormant before separately gated producer/send composition.

Therefore the next narrow missing behavior is one dormant async live-producer body that composes existing classification, caller-supplied dispatcher creation, QH request construction, capacity-one sender backpressure, and terminal handoff receipt creation.

## 10. Selected future source ceiling

Immediate future source-materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

No second Rust path is authorized.

If correct implementation requires changing `linux_bootstrap.rs`, `production_durable_capability_higher_owner_custody.rs`, another endpoint/executor file, Cargo/lockfile, workflow, Android, packaging, deployment, or repository configuration: STOP and return to selection.

## 11. Selected future producer-body shape

The future source checkpoint may add exactly one private dormant async helper equivalent in authority to:

`produce_remote_session_expected_device_admission_with_fallible_verifier_time(...)`.

Its logical inputs are exactly:
1. requester callback `DeviceId`;
2. exact scheduling-aware requester completion result;
3. one temporary mutable borrow of an infallible dispatcher factory that returns `D`;
4. one immutable borrow of the exact production expected-request sender for request type `RemoteSessionExpectedDeviceAdmissionRequest<D, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeSource>`.

Its logical output is exactly one existing private:
`RemoteSessionExpectedDeviceAdmissionHandoffReceipt`.

The helper remains private to the endpoint module.

It must not return or expose scheduling grants, constructed handoffs, requests, sender authority, dispatcher source authority, or verifier-time authority.

## 12. Dispatcher-factory law

The future producer body must accept a caller-supplied infallible factory equivalent to:
`FnMut() -> D`
where:
`D: CapabilityDispatcher + Send + 'static`.

The factory is invoked only after the existing live-completion classifier has proven one eligible continuation.

Ineligible live completion must return its existing receipt without invoking the dispatcher factory.

For one eligible continuation the factory is invoked exactly once.

No second dispatcher, fallback dispatcher, retry dispatcher, status refresh, alternate snapshot, dynamic dispatcher cache, `Arc`, `Mutex`, `RwLock`, atomic, watch channel, or remote-selected status state is selected.

If QH request construction later fails because an identifier source fails, the freshly created dispatcher drops normally with the failed construction attempt. There is no dispatcher retry.

## 13. Selected live-producer order

For one normal live producer invocation, exact selected order is:

1. call `classify_remote_session_expected_device_admission_live_completion(requester_device_id, completion)` exactly once;
2. if classification is `Ineligible(receipt)`, return that exact receipt unchanged and STOP that invocation;
3. if classification is `Eligible(continuation)`, invoke the caller-supplied dispatcher factory exactly once;
4. call `construct_remote_session_expected_device_admission_request_with_fallible_verifier_time(continuation, dispatcher)` exactly once;
5. if outcome is `ConstructionFailed(receipt)`, return that exact existing receipt unchanged and perform no send;
6. if outcome is `Constructed(handoff)`, destructure exactly its requester correlation, acknowledgement result and one request;
7. call exactly one `sender.send(request).await` on the borrowed sole production sender;
8. on send success, construct exactly one existing `EligibleTerminal` receipt with the exact requester correlation, exact acknowledgement result and disposition `Enqueued`;
9. on receiver-closed send failure, terminally drop the returned unsent request and construct exactly one existing `EligibleTerminal` receipt with the exact requester correlation, exact acknowledgement result and disposition `ChannelClosed`;
10. return that receipt.

No other terminal branch is selected for the live producer body.

## 14. Backpressure and sender law

The future helper receives only `&mpsc::Sender<...>`.

It does not own, clone, replace, reconstruct, persist, return, or re-export sender authority.

Exactly one send attempt is selected:
`sender.send(request).await`.

Full capacity is ordinary Tokio asynchronous backpressure and is not a terminal receipt class.

Forbidden:
- `Sender::clone()`;
- `try_send`;
- `blocking_send`;
- `reserve` / `try_reserve` as an alternate enqueue protocol;
- callback-local `block_on`;
- timeout-based queue escape;
- hidden/detached producer task;
- retry/requeue;
- second channel;
- alternate/unbounded queue;
- fallback sender;
- second producer future.

## 15. `Constructed` versus `Enqueued`

QH `Constructed` remains a local request-construction outcome only.

The future producer body must not create `Enqueued` before `sender.send(request).await` returns success.

A full channel does not produce `Enqueued` and does not produce `ChannelClosed`; the future remains pending under asynchronous backpressure.

Only successful send completion produces `Enqueued`.

Only receiver-closed send failure produces `ChannelClosed`.

Neither disposition proves receiver observation, authentication, admission, authorization, worker insertion, endpoint success, acknowledgement success, or runtime readiness.

## 16. Closed-channel terminal law

When `sender.send(request).await` returns `Err`, Tokio returns the unsent request.

The selected producer body must terminally drop that returned request.

It must not:
- retry the send;
- reconstruct a dispatcher;
- reconstruct the request;
- reopen identifiers;
- remint/replay/refund/rollback the consumed scheduling grant;
- create a replacement channel;
- enqueue into another queue;
- return the request to a caller;
- reinterpret channel closure as supervisor shutdown.

The exact receipt disposition is only `ChannelClosed`.

## 17. Construction-failure law

If QH returns `ConstructionFailed(existing receipt)`, the future producer body must return that exact receipt unchanged.

It must not send, inspect the disposed grant, regenerate identifiers, refresh status, create another dispatcher, retry construction, or remap `ConstructionFailed` to another disposition.

## 18. Ineligible completion law

If the existing classifier returns `Ineligible(existing receipt)`, the future producer body returns the exact receipt unchanged.

It must not:
- invoke the dispatcher factory;
- invoke QH;
- inspect scheduling authority further;
- borrow/use the sender for enqueue;
- create a new receipt;
- project or reconstruct the raw ineligible completion.

## 19. Shutdown-suppression separation

The existing synchronous shutdown-suppression mapper remains authoritative for shutdown-recovered completions.

The selected live producer body does not call or replace that mapper.

A shutdown-suppressed eligible completion must terminally dispose its sealed grant through the existing mapper and must never construct a dispatcher, invoke QH, or send a request.

`SuppressedOnShutdown` is not created by the selected live producer body.

## 20. Identity and authority separation

Requester callback `DeviceId` remains requester-side correlation only.

Target expected DeviceId remains obtainable only from the QH-consumed scheduling grant.

Requester scheduling SessionId remains distinct from target admission SessionId.

Target admission SessionId remains generated only by the existing QH source.

Authentication request ID remains generated only by the existing QH source.

Verifier time remains the exact QH-bound fallible function pointer and is not sampled during producer construction or request construction.

Channel sender possession is enqueue authority only; it is not identity, authentication, scheduling, admission, authorization, reachability, readiness, or worker-success authority.

## 21. QJ status-snapshot law remains unchanged

QM does not bind the producer to the QJ source yet.

A later separately gated caller/factory composition may capture the exact existing `LinuxAgentProductionRemoteCapabilityDispatcherSource` and provide an infallible factory equivalent to calling its existing `new_dispatcher(&self)` once per eligible producer invocation.

No independent `LocalAgentStatusSnapshot::current(...)` call is selected.

No dynamic status refresh is selected.

## 22. QL custody law remains unchanged

QM does not call:
- `LinuxAgentProductionExpectedDeviceAdmissionChannel::new()`;
- `into_parts(self)`.

QM does not move either channel endpoint.

A later separately gated higher-owner composition must create/split the exact one QL custody and retain the sole sender in exactly one producer owner while moving the exact receiver toward the existing QF consumer path.

## 23. Generic producer specialization remains deferred

The existing C03e-OP endpoint producer seam remains generic over `Receipt`.

QM does not select or materialize the closure that satisfies:
`AsyncFnMut(DeviceId, Result<...>) -> RemoteSessionExpectedDeviceAdmissionHandoffReceipt`.

QM does not invoke the generic endpoint producer path.

That closure/factory capture and specialization are separately gated after the private producer body is materialized.

## 24. Receiver-to-QF wiring remains deferred

QM does not invoke:
`run_with_production_durable_reachability_remote_process_companion_with_fallible_verifier_time_completion_projection_from_production_sources(...)`.

QM does not move the QL receiver into QF.

No requester/rendezvous higher-owner join, process caller, executable caller, listener, readiness, endpoint activation or network behavior is selected.

## 25. Future source-stage exclusions

The immediate future producer-body materialization must not:
- modify any second Rust path;
- widen visibility of private receipt/classifier/QH types/functions;
- bind or import the private QJ dispatcher source;
- construct or split QL channel custody;
- clone sender authority;
- create a receiver;
- invoke the generic producer endpoint seam;
- create a producer closure owner;
- wire the receiver into QF;
- add requester/rendezvous higher-owner integration;
- add a process/runtime/executable caller;
- modify QH identifier or grant order;
- sample verifier time during construction;
- modify status snapshot provenance;
- spawn tasks;
- activate listener/readiness/runtime/network behavior;
- modify Cargo/lockfile/workflows/Android/packaging/deployment;
- modify credentials/certificates/private keys/trust/RBAC/database/schema/control plane/authentication policy;
- modify repository configuration;
- merge, deploy, close PRs, mark ready, delete branches, or rewrite history.

## 26. Later separately gated work after future producer-body materialization

At minimum the following remain separate:
1. concrete binding of the QJ dispatcher source into one producer-owned factory;
2. QL channel custody construction and exact ownership split in a higher owner;
3. concrete generic `AsyncFnMut` producer closure ownership/capture;
4. specialization/invocation of the C03e-OP generic producer endpoint path with the concrete receipt;
5. exact receiver transfer into the QF production-source companion input path;
6. receipt observation policy at the higher owner;
7. requester/rendezvous higher-owner integration;
8. process/runtime caller migration;
9. executable caller;
10. listener/readiness/runtime/network activation;
11. merge/deployment.

## 27. C03e-QM source-selection decision

C03e-QM selects exactly:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ASYNC_PRODUCER_SEND_RECEIPT_COMPOSITION_SOURCE_MATERIALIZATION`

with hard one-file ceiling:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

The future source checkpoint may add only the dormant private async producer body described above.

If exact compilation or semantics require any second source path, visibility widening, channel-owner mutation, concrete QJ source binding, generic producer specialization/invocation, receiver wiring, or runtime caller change: STOP and return to selection.

## 28. Validation requirements for this selection checkpoint

C03e-QM itself changes documentation only.

Closure requires:
- exact QL ancestry;
- exactly one documentation path;
- zero Rust/source/runtime changes;
- exact-final-head CI authority only;
- PR kept draft/open/unmerged;
- immutable canonical Drive audit with byte/hash readback verification;
- no successor branch/PR created inside QM closure.

## 29. STOP

After C03e-QM selection is evidence-closed: STOP.

Do not create the future source-materialization checkpoint inside C03e-QM closure.
