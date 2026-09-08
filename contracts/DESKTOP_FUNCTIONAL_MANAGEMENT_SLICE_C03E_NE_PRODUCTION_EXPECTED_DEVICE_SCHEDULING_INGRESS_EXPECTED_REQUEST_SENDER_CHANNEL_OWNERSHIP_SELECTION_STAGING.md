# Phase 152 C03e-NE — Production expected-device scheduling ingress / expected-request sender-channel ownership selection

Status: STAGING SELECTION

Target gate:

`C03E_NE_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_INGRESS_EXPECTED_REQUEST_SENDER_CHANNEL_OWNERSHIP_SELECTED`

## 1. Purpose

C03e-NE is a documentation-only successor to evidence-closed C03e-ND.

C03e-ND proved that historical authority laws do not establish one complete production owner for `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` construction or its sender/channel lifecycle. ND therefore selected the next audit boundary as the real upstream production source that could legitimately provide expected-device scheduling intent and own the sending side of the existing expected-request queue.

C03e-NE answers that narrower question from the exact ND head:

> Does the current production source prove one authoritative expected-`DeviceId` scheduling ingress together with ownership of the `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>` lifecycle required to feed the already-materialized receiver-side admission supervisor?

The selected answer is **no**.

Selection result:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_INGRESS_OR_EXPECTED_REQUEST_SENDER_CHANNEL_OWNER_PROVEN / EXISTING_PRODUCTION_COMPOSITION_REMAINS_RECEIVER_ONLY / REQUESTER_RENDEZVOUS_TARGET_INTENT_NOT_ADMISSION_SCHEDULING_AUTHORITY / SOURCE_MATERIALIZATION_BLOCKED`

C03e-NE does not invent a scheduler, sender, queue capacity, request source, identity source, retry policy, shutdown law, or executable caller merely to make the existing receiver live.

## 2. Exact predecessor authority

Predecessor branch:

`phase-152-c03e-nd-production-expected-device-admission-request-historical-authority-source-composition-selection`

Exact predecessor head:

`38158c4cc4e3bc5e497c6ba1a0fd3fd01cb18d2b`

Exact predecessor tree:

`0477de26fee239ce280e77a19d77d4f1fbc03342`

C03e-ND selected:

`PARTIAL_HISTORICAL_AUTHORITY_LAWS_CONFIRMED / COMPLETE_PRODUCTION_EXPECTED_REQUEST_COMPOSITION_OWNER_NOT_PROVEN / AUTHENTICATION_PRWM_CORRELATION_AND_EXPECTED_DEVICE_SOURCE_CUSTODY_UNRESOLVED / EXPECTED_REQUEST_SENDER_CHANNEL_LIFECYCLE_UNRESOLVED / SOURCE_MATERIALIZATION_BLOCKED`

C03e-NE preserves every ND identity, correlation, timing, dispatcher, and runtime-activation exclusion.

## 3. Exact receiver-side runtime law

The expected-request type and repeated admission supervisor remain in:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Exact ND blob:

`ef370ca500f118bc067097ddb8f5c37ab597b214`

The request remains:

```rust
pub struct RemoteSessionExpectedDeviceAdmissionRequest<D, T> {
    expected_device_id: DeviceId,
    session_id: SessionId,
    authentication_request_id: u64,
    dispatcher: D,
    verifier_time_unix_seconds: T,
}
```

The production repeated-admission drive accepts only:

```rust
expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>
```

The supervisor tracks the request source independently from the explicit supervisor-shutdown future.

When `poll_recv(...)` returns `None`, the source is marked closed, but shutdown is not fabricated. The exact focused test is named:

`closed_expected_request_source_does_not_fabricate_shutdown`

This law is important for sender ownership. Dropping the last sender changes only request-source openness; it does not request supervisor shutdown. A future producer therefore needs an explicit, reviewed sender-drop/shutdown ordering rather than relying on channel closure as an implicit lifecycle signal.

The same supervisor also proves:

- ready explicit shutdown wins before a prequeued expected request;
- the request source is not polled while the worker collection is at capacity;
- duplicate expected `DeviceId` is rejected before admission timing is sampled;
- admission timing is sampled only for a request that can actually start.

Any production sender must preserve these existing consumer-side laws.

## 4. Endpoint lifecycle remains receiver-only

Exact source:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact ND blob:

`b1afdeba4b0bc59399ff4e4dbf480479f4fb2cfe`

Every audited repeated-admission endpoint drive receives the expected-request receiver by value and forwards it into the executor lifecycle.

The endpoint lifecycle separately owns an explicit non-cloneable supervisor-shutdown controller/signal pair. The controller makes the supervisor future ready; it does not close the expected-request sender because no such sender is owned here.

The endpoint therefore proves:

`RECEIVER_CUSTODY = MATERIALIZED`

but not:

`EXPECTED_REQUEST_SENDER_CUSTODY = MATERIALIZED`

and not:

`EXPECTED_DEVICE_SCHEDULING_INGRESS = MATERIALIZED`

## 5. Production reachability endpoint wrapper remains receiver propagation

Exact source:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

Exact ND blob:

`4030cac84cad1780cc37410d344ba642cb4ac6e4`

`ProductionReachabilityEndpointLifecycleRuntime` retains production reachability custody beside the lower remote endpoint and delegates repeated admission while receiving:

```rust
expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>
```

The production wrapper performs no candidate publication, traversal activation, executable process wiring, or additional admission-source construction.

It therefore cannot be reclassified as a sender owner merely because it is a production-labeled endpoint wrapper.

Result:

`PRODUCTION_REACHABILITY_ENDPOINT_EXPECTED_REQUEST_ROLE = RECEIVER_FORWARDING_ONLY`

## 6. Production reachability runtime custody is not scheduling ingress

Exact source:

`crates/prw-agent/src/production_reachability_runtime_custody.rs`

Exact ND blob:

`ffcddc0253de2b5430be798061ddad8e920a07ac`

This owner retains live reachability authority and durable production-owner custody, then may bind one remote endpoint.

It owns no expected-request sender, channel capacity, expected-device admission queue, scheduling callback, or request publication operation.

Reachability custody therefore remains authority for endpoint/reachability lifecycle only. It is not proof of logical expected-device scheduling intent.

The following substitutions remain invalid:

- current/bound socket address -> expected `DeviceId`;
- connectivity endpoint -> expected `DeviceId`;
- transport identity -> expected `DeviceId` scheduling event;
- durable reachability ownership -> expected-request sender ownership.

## 7. Higher-owner production input composition remains receiver injection

Exact source:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact ND blob:

`f77a07fa54e874620f1472a8efc19b8c9b078b2f`

The higher-owner production population chain retains and forwards already-typed production inputs. Its expected-request surface remains the already-created receiver.

No audited production operation in this owner:

- calls `mpsc::channel(...)` for expected requests;
- owns `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<...>>`;
- publishes an expected request;
- chooses channel capacity;
- defines sender clone policy;
- maps sender closure to process shutdown;
- derives the expected logical `DeviceId` from an authoritative scheduling event.

Moving the receiver through more production wrappers does not prove the missing sending-side provenance.

## 8. Linux bootstrap remains receiver injection

Exact source:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact ND blob:

`709fdbd749fdd63886701fdcc86f32b07110d19e`

The audited production remote-process operation input surfaces take the expected-request receiver as an input.

`mpsc::channel(...)` usage associated with expected requests remains test/synthetic setup rather than a selected production channel owner.

No production `Sender<RemoteSessionExpectedDeviceAdmissionRequest<...>>` surface was proven in this file.

The existing production remote peer / reachability `DeviceId` population is not promoted into expected-device scheduling authority. A configured peer identity value is identity/reachability provenance; it is not an event saying that one logical device should now enter the pre-authentication admission queue.

## 9. Requester/rendezvous target intent is not expected-device admission scheduling authority

Exact intent source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`

Exact ND blob:

`5f616f20699d1c7069f5aa8973200a0359c19cde`

The source explicitly defines `RequesterRendezvousTargetIntent` as:

- requester-nominated logical target intent;
- non-authoritative at construction;
- not a registration fact;
- not authorization;
- subject to separate registry/workspace/policy validation.

`RequesterRendezvousStartIntent` combines an already-authenticated requester session with that nominated target.

This lifecycle is therefore downstream of an already-authenticated requester and models requester-side target nomination. It is not the server-side pre-authentication expected-device admission scheduling source required by the repeated AJ supervisor.

Treating target nomination as an expected-admission send would silently change direction, authentication stage, and authority semantics.

C03e-NE explicitly rejects that reinterpretation.

## 10. Requester/rendezvous runtime owner is provider-record custody only

Exact source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

Exact ND blob:

`082a70af239972a82318f3e17cb3fd8cb45d9e95`

`CandidatePublicationRequesterRendezvousRuntimeOwner` owns one in-memory requester/rendezvous provider and exposes narrow operations for:

- registering one policy-authorized requester/rendezvous start;
- selecting one current requester/rendezvous grant for an authenticated publisher;
- retiring/removing one committed requester/rendezvous record.

Construction explicitly performs no task creation, I/O, readiness publication, or networking.

The runtime owner exposes no expected-request sender or admission-queue operation.

The stored target is expected publisher identity for requester/rendezvous authority. That does not make the provider a producer of `RemoteSessionExpectedDeviceAdmissionRequest`.

## 11. Shared requester/rendezvous authority does not bridge the gap

Exact source:

`crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`

Exact ND blob:

`d550ec8d5aa18ed6885ebed42c52ee742498e9d2`

The shared wrapper serializes requester/rendezvous registration, current-grant selection, durable candidate commit coordination, and exact cleanup.

Its authority is tied to authenticated requester/publisher session state and current requester/rendezvous records.

It exposes no expected-device request queue, sender, send operation, queue capacity, or pre-authentication admission scheduler.

Requester/rendezvous authority remains distinct from repeated expected-device admission authority.

## 12. Sender/channel lifecycle obligations remain entirely unowned

The existing receiver semantics make the missing producer lifecycle observable. A real production sender owner must define all of the following before source materialization can be safe:

1. the exact production event that authorizes creation of one expected-device request;
2. the source of the exact expected logical `DeviceId` for that event;
3. one bounded channel capacity and its provenance;
4. which lifecycle owner creates the sender/receiver pair;
5. whether the sender is non-cloneable by architecture or whether bounded cloning is authorized;
6. which components may retain sender custody;
7. blocking/await/try-send/backpressure behavior;
8. full-channel disposition;
9. receiver-closed disposition;
10. request ownership recovery or loss behavior on failed send;
11. sender-drop ordering during process shutdown;
12. whether queued requests remain eligible during shutdown;
13. explicit coordination with `RemoteSessionSupervisorShutdownController`;
14. no send after the authoritative scheduling event becomes stale/revoked;
15. no reinterpretation of channel closure as supervisor shutdown.

None of these production obligations is proven by the audited source.

## 13. Identity and authority separation

C03e-NE preserves the existing identity chain:

`logical expected DeviceId scheduling intent -> fresh current registry transport resolution -> exact peer acceptance -> logical-session authentication -> post-authenticated owner identity`

The following remain non-substitutable:

- expected `DeviceId` scheduling intent;
- authenticated requester identity;
- requester-nominated rendezvous target;
- authenticated publisher identity;
- `TransportIdentity`;
- connectivity endpoint/IP/port;
- candidate identity/path;
- PRWC request ID;
- authentication PRWM request ID;
- `SessionId`.

A value being typed as `DeviceId` does not prove that it belongs to the expected-admission scheduling lane.

## 14. Remaining constructor authorities stay separately unresolved

Even if a later checkpoint proves a scheduling event and sender owner, C03e-NE does not claim that owner also possesses the remaining expected-request constructor values.

The following remain separately gated unless proven at that exact future boundary:

- production `SessionId` source/custody for the expected-device PRWM admission lane;
- authentication PRWM request-ID source/custody;
- per-request construction/custody of the NB status-only dispatcher;
- expected-request verifier-time provider construction/custody;
- admission timing source sampled at actual attempt start.

No future sender should fabricate any of these values to satisfy the constructor.

## 15. Source-materialization decision

C03e-NE selects **no Rust source materialization**.

In particular, NE does not authorize adding a helper that:

- creates `mpsc::channel(...)`;
- returns a sender/receiver pair;
- clones or stores a sender;
- accepts an arbitrary `DeviceId` and sends an expected request;
- converts requester/rendezvous target intent into an expected request;
- converts configured production peer identity into an admission event;
- generates or borrows unrelated request/session IDs;
- installs a clock closure;
- injects the NB dispatcher into a live caller.

Such a helper would define missing production semantics rather than materialize already-selected semantics.

## 16. Exact missing prerequisite

The next unresolved prerequisite is narrowed to:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_SOURCE_PROVENANCE`

Before sender/channel ownership can be selected or materialized, a later checkpoint must prove the real production event/source that is authoritative for saying:

> this logical `DeviceId` is now eligible to be submitted as one pre-authentication expected-device admission attempt.

That event must be distinct from transport identity, endpoint reachability, authenticated requester identity, requester target nomination, candidate publication state, and synthetic/test inputs.

If no such production event exists in current source, the later checkpoint must record that absence rather than creating one opportunistically inside the Agent runtime.

## 17. Immediate successor ceiling

After evidence-closed C03e-NE, only a separately gated documentation-only checkpoint may assess/select:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_SOURCE_PROVENANCE`

That successor must identify, if it exists:

1. the authoritative upstream lifecycle/component;
2. event ownership and origin;
3. expected logical `DeviceId` provenance;
4. authentication stage at which the event exists;
5. currentness/revocation semantics before queue publication;
6. whether events are one-shot, repeatable, deduplicated, or superseding;
7. failure and shutdown disposition;
8. exact dependency-safe handoff toward a later sender/channel owner.

The successor does not inherit authorization to create a channel or send a request.

## 18. Frozen exclusions

C03e-NE does not select, materialize, or activate:

- a production expected-device scheduler;
- expected-request sender/channel creation;
- channel capacity;
- sender clone policy;
- send/backpressure/retry policy;
- sender-drop/shutdown behavior;
- synthetic expected requests;
- requester/rendezvous target -> admission adaptation;
- production peer DeviceId -> scheduling-event adaptation;
- `SessionId` generation;
- authentication PRWM request-ID allocation;
- verifier-time provider construction;
- admission timing policy;
- dispatcher caller propagation;
- capability provider construction;
- current-registry hydration or positive policy grants;
- candidate traversal/dialing;
- listener/runtime/readiness activation;
- `run()` or `main.rs` mutation;
- Cargo manifest/lockfile/workflow mutation;
- systemd/package/security/credential/certificate/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment/restart/recovery;
- repository configuration/visibility mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update/rebase/squash/history rewrite;
- destructive cleanup.

Runtime activation remains `0%`.

Expected-request producer source materialization remains `0%`.

NB dispatcher caller propagation remains `0%`.

## 19. Closure rule

C03e-NE may close only after:

- exact ND -> NE topology proves one documentation-only path;
- canonical exact-head Rust validation succeeds;
- any path-filtered workflow is recorded as `SKIPPED`, not PASS;
- no Android PASS is claimed unless an Android workflow actually runs on the exact final head;
- one immutable canonical Drive audit is published with byte/hash readback verification;
- final ND/NE/main race guards remain exact;
- the PR remains draft/open/unmerged;
- no C03e-NF successor exists.

`CLOSED` will denote checkpoint closure only, not production readiness or runtime activation.
