# C03e-QK — Production Expected-Device Admission Capacity-One Channel / Sole-Sender Custody Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_CAPACITY_ONE_CHANNEL_SOLE_SENDER_CUSTODY_SELECTION`

Selected future boundary:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_CAPACITY_ONE_CHANNEL_SOLE_SENDER_CUSTODY_SOURCE_MATERIALIZATION`

## 1. Exact predecessor authority

This checkpoint is based only on evidence-closed C03e-QJ.

Exact predecessor branch:

`phase-152-c03e-qj-production-expected-device-admission-status-only-dispatcher-factory-custody-source-materialization`

Exact predecessor head:

`d9befda4887a8a1ba8921de5236e577f1d7e275e`

Exact predecessor tree:

`4c1702c7f0b9f8fb0dc3b232457a5b0854259708`

Exact QJ `linux_bootstrap.rs` blob:

`f39c3f99b2967a0b75f1ade871f1d73a4e5d18b5`

Exact QH endpoint-lifecycle source blob retained at QJ:

`808d3cae4d31214923ee5380f8ac0f76dd171485`

Exact production higher-owner source blob retained at QJ:

`0121e25738de66932324ea7921c9905249c2155d`

C03e-QJ PR #575 remains draft/open/unmerged and records:

`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

Canonical C03e-QJ immutable evidence is Drive object:

`1vdzujHcVg_NPW1Y6Sdq03nKMzx9EE3F_`

under canonical parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`.

Its frozen/raw-readback stream is `16786` bytes with SHA-256:

`afc0f83f1c90aeb6a2842db5e731d2da09e00c39c7d66a3349486391d2f392e7`.

## 2. Fresh post-QJ audit result

Fresh live audit before assigning QK confirmed:

- integrated `main` remained `7c993fa93977a0bb84e0d030874eee7fd0cae77f`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`;
- QJ remained exact head/tree/source blobs above;
- PR #575 remained draft/open/unmerged/mergeable;
- exact-final-head QJ CI remained Rust #1784 `SUCCESS`, Android #1715 `SUCCESS`, C02f-AD #1034 `SKIPPED`, C02f-AE #1025 `SKIPPED`;
- the canonical QJ Drive audit remained the exact-title singleton under the canonical parent;
- no `phase-152-c03e-qk-*` branch existed before QK assignment;
- recent PR chronology remained headed by QJ #575;
- QJ remained the latest C03e Q-series checkpoint.

The fresh audit permits one separately gated successor selection only. QK itself does not materialize the selected channel custody.

## 3. Exact unresolved boundary after QJ

QJ resolved only concrete production dispatcher snapshot provenance/factory custody.

The next unresolved dependency needed before any concrete async producer can send a QH-constructed request is narrower than producer invocation:

> Where is the one expected-request channel created, which owner retains the sole sender, and how is the single receiver transferred toward the already-existing production fallible-verifier-time consumer path without introducing sender clones, alternate queues, sends, tasks, or runtime activation?

QK selects only that channel/custody question.

QK does not select or materialize:

- concrete producer closure/function composition;
- QJ dispatcher construction at producer time;
- QH request construction invocation;
- `sender.send(request).await`;
- `Enqueued` / `ChannelClosed` receipt mapping;
- generic producer specialization/invocation;
- receiver-to-QF invocation wiring;
- requester/rendezvous higher-owner integration;
- executable/runtime activation.

## 4. Exact receiver consumers already exist

Fresh exact-QJ source proves that expected-request receiver ownership is already represented throughout the existing consumer stack.

### 4.1 Endpoint-owner producer seam

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

already contains the dormant generic producer forwarding method:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling_producer(...)`.

That seam accepts by value:

`expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`.

The method explicitly states that it constructs no channel, sender, expected-device request, concrete receipt, identifier, dispatcher, timing source, task, listener, readiness state or executable activation.

Therefore this existing endpoint layer is a receiver consumer, not the missing channel-construction owner.

### 4.2 Linux process-operation ownership

`crates/prw-agent/src/linux_bootstrap.rs`

already defines `LinuxAgentRemoteProcessOperationInputs<P,D,T,...>` with exactly one by-value field:

`expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`.

Its constructor takes that receiver by value, and the existing production bind/worker-limit population chain forwards the exact receiver unchanged.

The process-operation owner therefore already supports create-once / move-once receiver transfer and does not need a second receiver representation.

### 4.3 Production higher-owner population seam

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

already imports `tokio::sync::mpsc` and contains the production source-population chain that accepts the exact expected-request receiver by value.

In particular, the C03e-QF helper:

`run_with_production_durable_reachability_remote_process_companion_with_fallible_verifier_time_completion_projection_from_production_sources(...)`

accepts:

`expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

and moves it into the existing fail-closed production population chain.

This exact higher-owner source is therefore the narrowest proven place to materialize the missing channel custody without mutating endpoint internals or Linux bootstrap input types.

## 5. Existing consumer semantics remain authoritative

The repeated real-admission fallible-verifier-time supervisor already consumes the receiver by value and polls it under existing shutdown/capacity rules.

QK does not alter:

- duplicate expected-device preflight;
- admission timing sampling;
- one in-flight admission custody;
- shutdown precedence;
- authenticated owner-derived `DeviceId` worker authority;
- worker cancellation/join behavior;
- endpoint close/idle drain ownership.

The new future channel custody only supplies the already-typed receiver expected by those seams.

## 6. Selected channel primitive

The future source materialization must use exactly one bounded Tokio MPSC channel:

`tokio::sync::mpsc`.

The exact capacity is:

`1`.

Equivalent construction:

`mpsc::channel(1)`.

No configuration source, environment variable, constant derived from worker capacity, dynamic tuning or fallback capacity is selected.

Capacity `1` is part of the selected authority boundary and must not silently become `0`, `2`, `MAX_REGISTERED_DEVICES`, an unbounded queue or any runtime-derived value.

## 7. Selected future custody representation

A later separately gated source checkpoint may add one private non-cloneable custody concept in:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

with authority equivalent to:

`LinuxAgentProductionExpectedDeviceAdmissionChannel<D, T>`

or an equivalently narrow private name.

It may own exactly:

- one `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- one `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`.

It must own no:

- dispatcher or dispatcher source;
- eligible continuation;
- scheduling grant;
- requester or target `DeviceId`;
- requester or target `SessionId`;
- authentication request ID;
- verifier-time provider or sampled verifier time;
- acknowledgement result;
- handoff receipt/disposition;
- endpoint/executor owner;
- requester/rendezvous authority;
- durable capability authority;
- retry token;
- task handle;
- listener/readiness/process authority.

The custody represents only one bounded queue pair.

## 8. Selected channel construction law

The future private constructor may create exactly one channel pair through:

`mpsc::channel(1)`.

Construction is infallible and therefore introduces no error variant, retry, fallback or alternate queue.

The constructor must not:

- call `mpsc::unbounded_channel()`;
- create more than one bounded channel;
- clone the returned sender;
- create a spare receiver;
- retain a second sender elsewhere;
- spawn a producer task;
- send any request;
- inspect channel capacity dynamically;
- fabricate a receipt.

## 9. Sole-sender law

Exactly one sender returned by the one `mpsc::channel(1)` call is retained by the future custody.

The selected source must not implement or derive `Clone` or `Copy` for the custody in a way that could duplicate sender authority.

The selected source must not call:

`Sender::clone()`

or otherwise duplicate sender ownership.

The eventual producer checkpoint may receive that exact sender by value. QK does not authorize retaining a producer-side clone, fallback sender, shutdown sender, retry sender or metrics sender.

The one sender is enqueue authority only. It is not identity, authentication, admission, authorization, scheduling-grant or acknowledgement authority.

## 10. Receiver create-once / move-once law

Exactly one receiver returned by the one channel construction is retained beside the sole sender until explicit ownership split.

The receiver cannot be cloned and must not be replaced or recreated.

A future narrow consuming method equivalent to:

`into_parts(self) -> (Sender<...>, Receiver<...>)`

may move out exactly the one sender and one receiver once.

After `into_parts`, the custody is consumed and cannot recover either endpoint.

No accessor returning a sender clone is selected.

No borrowed receiver accessor is required for the production path.

## 11. Future ownership transfer law

QK selects only the ownership direction needed by later checkpoints:

1. one future higher-owner channel custody creates exactly one `(sender, receiver)` pair;
2. the exact sender later moves by value into exactly one separately gated concrete expected-device producer composition;
3. the exact receiver later moves by value into the existing C03e-QF fallible-verifier-time production-source companion input path;
4. neither endpoint is recreated after transfer.

QK does not materialize either transfer into an invocation site.

The future channel source materialization may expose ownership-only decomposition, but must not invoke QF, QH or the generic producer.

## 12. Full-channel behavior is asynchronous backpressure

Capacity `1` intentionally permits one queued request while the consumer is not ready.

When that one slot is full, the eventual producer must await normal Tokio MPSC backpressure.

No selected future path may convert full-channel state into:

- immediate drop;
- `try_send` error mapping;
- busy loop;
- timeout;
- retry queue;
- second channel;
- detached buffering task;
- blocking thread.

QK introduces no timeout or queue-pressure receipt class.

## 13. Future send law remains separately gated

QK does not send.

A later separately gated concrete producer composition may enqueue only through exact async semantics equivalent to:

`sender.send(request).await`.

No other send primitive is selected.

Specifically forbidden:

- `try_send`;
- `blocking_send`;
- callback-local `block_on`;
- `spawn` merely to hide async send;
- sender clone plus background send;
- alternate/unbounded queue;
- retry/requeue after send failure.

## 14. Future send terminal mapping

The existing handoff disposition already contains:

- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

QK does not materialize any mapping, but preserves the future exact meaning:

- successful `sender.send(request).await` may map to `Enqueued`;
- send failure because the receiver is closed may map to `ChannelClosed`;
- the unsent request returned by Tokio on closed-channel failure is terminally dropped by that later producer path;
- no retry, requeue, replacement channel or dispatcher reconstruction follows a closed-channel send failure.

No distinction such as `QueueFull` is selected because bounded full state is backpressure, not terminal failure.

## 15. `Constructed` remains distinct from `Enqueued`

QH `Constructed(...)` means exactly one typed expected-device request has been constructed and is locally owned by the constructed handoff.

It does not mean:

- a sender exists;
- send began;
- send completed;
- receiver observed the request;
- authentication or admission succeeded;
- worker insertion succeeded;
- capability authorization succeeded;
- acknowledgement succeeded.

Only a later successful async send may classify the handoff as `Enqueued`.

QK creates neither state transition.

## 16. QH construction law remains unchanged

QK does not modify or invoke the QH request-construction helper.

The existing order remains:

1. fresh target admission `SessionId` source;
2. only on success, fresh nonzero expected-device authentication request ID source;
3. bind exact fallible verifier-time function pointer without sampling;
4. only after both identifier successes consume the one-shot scheduling grant;
5. target expected `DeviceId` only from the consumed grant;
6. construct exactly one request with the exact caller-supplied dispatcher.

Channel existence must not reorder or weaken any of these steps.

## 17. QJ dispatcher law remains unchanged

QK does not construct or modify `LinuxAgentProductionRemoteCapabilityDispatcherSource`.

The later concrete producer remains responsible, under a separate checkpoint, for obtaining exactly one fresh dispatcher from the QJ source only after the completion is eligible and not shutdown-suppressed, then moving that dispatcher once into QH construction.

The channel custody does not own the QJ dispatcher source and cannot mint a dispatcher.

## 18. Shutdown-suppression law remains unchanged

The existing live-completion classifier and shutdown-suppression receipt mapper remain authoritative.

A completion mapped to `SuppressedOnShutdown` must not enter future QH request construction or future channel send.

QK does not change the order of eligibility classification, shutdown suppression, dispatcher creation, request construction or send.

## 19. Selected future source ceiling

The immediate future source-materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

A future QL source materialization may add only the dormant capacity-one channel custody and focused same-file ownership/type-shape tests if practical.

It must not modify:

- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`;
- `main.rs`;
- Cargo manifests or `Cargo.lock`;
- workflows;
- Android sources;
- packaging/deployment files.

If correct channel-custody materialization requires a second Rust path, QL must STOP and return to a fresh selection checkpoint.

## 20. Future QL source-materialization exclusions

The selected one-file future source checkpoint must not:

- construct a concrete dispatcher;
- call the QJ dispatcher factory;
- classify a requester completion;
- inspect or consume a scheduling grant;
- call QH request construction;
- construct an expected-device request;
- send a request;
- invoke `sender.send(...)`;
- call `try_send` or `blocking_send`;
- produce `Enqueued` or `ChannelClosed` receipts;
- specialize/invoke the generic cooperative producer;
- invoke QF with the receiver;
- add a process/executable invocation site;
- create a background producer task;
- activate endpoint/listener/readiness/runtime/network behavior;
- widen public API;
- change credentials/certificates/private keys/trust/RBAC;
- mutate DB/schema/control plane/authentication policy;
- modify repository settings/rulesets/permissions;
- merge, deploy, restart or recover production.

## 21. Later separately gated dependencies after channel-custody materialization

Even after future QL closes, the following remain separate:

1. concrete async producer composition receiving one eligible live continuation;
2. exact QJ dispatcher acquisition in that producer;
3. QH request construction invocation;
4. exact `sender.send(request).await` enqueue;
5. `Enqueued` / `ChannelClosed` receipt composition;
6. specialization/invocation of the existing generic cooperative producer path with concrete receipt;
7. actual receiver transfer into QF production-source companion invocation;
8. requester/rendezvous higher-owner integration;
9. process/runtime caller migration;
10. executable caller;
11. listener/readiness/process/runtime/network activation;
12. deployment/merge.

QK does not collapse these gates.

## 22. Documentation-only scope of QK

QK itself changes exactly one contract path and no Rust/source/runtime path.

QK does not create a channel, sender, receiver, dispatcher, expected request, producer closure, receipt mapping or executable caller.

Therefore QK changes no production behavior.

## 23. Closure target

After exact-final-head CI and immutable evidence publication, QK may be classified:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

only for the exact final QK head.

Keep the QK PR draft/open/unmerged.

STOP after C03e-QK closure.

Do not create C03e-QL inside QK closure.