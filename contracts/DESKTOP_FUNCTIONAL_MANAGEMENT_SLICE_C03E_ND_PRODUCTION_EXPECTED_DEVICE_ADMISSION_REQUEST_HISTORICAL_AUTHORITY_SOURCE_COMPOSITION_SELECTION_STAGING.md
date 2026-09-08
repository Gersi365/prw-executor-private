# Phase 152 C03e-ND — Production expected-device admission request historical authority / source-composition selection

Status: STAGING SELECTION

Target gate:

`C03E_ND_PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_HISTORICAL_AUTHORITY_SOURCE_COMPOSITION_SELECTED`

## 1. Purpose

C03e-ND is a documentation-only successor to evidence-closed C03e-NC.

C03e-NC proved that the exact production remote-admission lineage consumes `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` through already-supplied receiver/custody inputs but does not prove a production producer that constructs the complete request and owns the sending side/channel.

C03e-ND answers the narrower historical-authority question before any source mutation:

> Do already-closed checkpoints provide production authority and source custody for the fields and lifecycle needed to compose a real `RemoteSessionExpectedDeviceAdmissionRequest`, such that a later source checkpoint could safely materialize the producer without inventing identity, correlation, timing, channel, or runtime semantics?

The answer selected here is **no**.

Historical checkpoints provide important semantic laws and dependency ordering, but the exact C03e-NC lineage still does not prove one production source-composition owner for the complete expected request or its sender/channel lifecycle.

Selection result:

`PARTIAL_HISTORICAL_AUTHORITY_LAWS_CONFIRMED / COMPLETE_PRODUCTION_EXPECTED_REQUEST_COMPOSITION_OWNER_NOT_PROVEN / AUTHENTICATION_PRWM_CORRELATION_AND_EXPECTED_DEVICE_SOURCE_CUSTODY_UNRESOLVED / EXPECTED_REQUEST_SENDER_CHANNEL_LIFECYCLE_UNRESOLVED / SOURCE_MATERIALIZATION_BLOCKED`

## 2. Exact predecessor authority

Predecessor branch:

`phase-152-c03e-nc-production-expected-device-admission-request-producer-provenance-selection`

Exact predecessor head:

`57dbed0aba3e784e55184b27ba39d2163084228a`

Exact predecessor tree:

`a2160ebd2d9eee3afeb79daab7c46712731007a1`

Predecessor selection result:

`NO_PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_PRODUCER_PROVEN / STATUS_ONLY_DISPATCHER_CALLER_PROPAGATION_BLOCKED_ON_EXPECTED_REQUEST_PRODUCTION_SEAM / RUNTIME_ACTIVATION_DEFERRED`

C03e-ND must not weaken that closed result by treating historical terminology as proof of a currently materialized producer.

## 3. Exact current request shape

At the exact C03e-NC head, the production admission request remains:

```rust
pub struct RemoteSessionExpectedDeviceAdmissionRequest<D, T> {
    expected_device_id: DeviceId,
    session_id: SessionId,
    authentication_request_id: u64,
    dispatcher: D,
    verifier_time_unix_seconds: T,
}
```

Its constructor requires all five values by value:

```rust
RemoteSessionExpectedDeviceAdmissionRequest::new(
    expected_device_id,
    session_id,
    authentication_request_id,
    dispatcher,
    verifier_time_unix_seconds,
)
```

The exact source path is:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Exact C03e-NC blob:

`ef370ca500f118bc067097ddb8f5c37ab597b214`

The expected `DeviceId` is explicitly pre-authentication scheduling intent. It is not authenticated post-handshake identity and it is not a substitute for current transport identity.

## 4. Exact consumer/source-custody shape

The repeated endpoint lifecycle accepts only the receiving side:

```rust
expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>
```

and separately accepts:

```rust
admission_timing: F
```

with:

```rust
F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming
```

Exact endpoint source path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact C03e-NC blob:

`b1afdeba4b0bc59399ff4e4dbf480479f4fb2cfe`

This source owns/forwards a receiver. It does not construct the sender, choose its capacity, publish requests into it, or establish sender-drop/shutdown semantics.

`RemoteSessionRealAdmissionTiming` is also separate from the expected request. It contains challenge-validity, authentication-now, and application-lease timing sampled when an admission attempt begins. Therefore an expected-request producer cannot be declared complete merely by constructing the request struct; production timing custody must remain coherent with the admission attempt.

## 5. Exact AJ transaction boundary

The exact real-admission transaction remains:

`crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`

Exact C03e-NC blob:

`812b56e9b948a41f2f746eb406ba24567efbd528`

`admit_expected_remote_device_session(...)` receives:

- expected logical `DeviceId`;
- `SessionId`;
- challenge-validity range;
- authentication PRWM request ID;
- authentication verifier time;
- application-lease range.

The AJ transaction itself resolves expected lower-transport identity from fresh current registry state before peer acceptance and performs another current-authority read before challenge preparation. Therefore production `TransportIdentity` is **not** a field that an expected-request producer should invent or freeze into the expected request.

This preserves the existing law:

`expected DeviceId scheduling intent -> fresh current registry transport resolution -> exact peer acceptance -> fresh challenge authority -> logical-session authentication -> post-auth binding`

## 6. C03e-BG historical dependency ordering

Closed C03e-BG selected the non-fabricated production remote-admission input dependency ordering:

1. authoritative current registry + policy source;
2. concrete typed capability-provider/dispatcher custody;
3. production `SessionId` custody/production;
4. authentication PRWM request-id custody/production;
5. verifier-owned request/admission timing sources;
6. authoritative pre-handshake expected-`DeviceId` scheduling provenance;
7. bounded worker-capacity configuration and observation callbacks;
8. only then expected-request + remote-process input composition.

This ordering remains authoritative and is preserved.

However C03e-BG explicitly did **not** select/materialize:

- a concrete expected-device producer;
- a production `SessionId` generator;
- an authentication request-id allocator;
- a timing/clock policy.

C03e-ND therefore treats BG as dependency/authority ordering, not as producer source provenance.

## 7. Dispatcher authority after C03e-NB

The dispatcher dependency has advanced since C03e-BG.

Closed C03e-NB materialized a dormant crate-private production status-only dispatcher:

`LinuxAgentProductionRemoteCapabilityDispatcher`

It owns one `LocalAgentStatusSnapshot`, implements `CapabilityDispatcher + Send + 'static`, returns the existing five-byte typed status result for `AgentStatus`, and fails closed for all file/transfer/terminal/forwarding families.

C03e-NB therefore proves a valid by-value dispatcher type exists for a later expected request.

It does **not** prove:

- which production owner constructs an instance for each expected request;
- which request-generation lifecycle samples the status snapshot;
- which sender owns the dispatcher until admission;
- any live caller migration.

C03e-ND preserves NB as a solved type/materialization prerequisite while keeping its production request-caller propagation gated.

## 8. SessionId historical authority assessment

Closed C03e-CE selected a verifier-side `SessionId` authority for the Phase-129 PRWA authentication lane:

- fresh 32-byte / 256-bit cryptographically secure random value per admissible PRWA Begin;
- lowercase hexadecimal ASCII, exactly 64 bytes;
- construction through existing `SessionId::new(...)`;
- OS-backed cryptographic randomness;
- no derivation from request IDs, DeviceId, time, TransportIdentity, endpoints, or requester/rendezvous state;
- fail closed on collision;
- no persistent allocator.

That law is security-relevant precedent, but C03e-ND does **not** silently transfer it into the C03e-H/AJ expected-device PRWM admission lane.

The expected request requires a `SessionId` before the AJ transaction. No exact C03e-NC production source audited here is proven to invoke the C03e-CE PRWA generation authority or an equivalent separately selected PRWM SessionId producer and then move that value into `RemoteSessionExpectedDeviceAdmissionRequest::new(...)`.

Therefore:

`PRODUCTION_EXPECTED_REQUEST_SESSION_ID_SOURCE_CUSTODY = UNPROVEN`

A later checkpoint may reuse a historical semantic law only after explicitly proving protocol/lifecycle applicability and selecting the exact production source seam. It must not infer equivalence from the shared Rust `SessionId` type alone.

## 9. Authentication request-ID historical authority assessment

The expected request field is:

`authentication_request_id: u64`

This value flows into the existing C03e-H/AJ logical-session authentication transaction as PRWM correlation.

Closed C03e-BY/CJ selected and materialized a bridge-owned request-ID lifecycle for the Phase-129 PRWC connection lane. That lifecycle is connection-local, non-zero, monotonic, bounded in-flight, and correlation-only.

C03e-ND explicitly rejects treating the PRWC lifecycle as the producer for this expected-request authentication PRWM request ID.

Reasons:

1. the carrier/lifecycle authority is different;
2. PRWC candidate-publication/pre-mesh correlation is not the expected-device PRWM authentication transaction;
3. the shared primitive type `u64` does not establish semantic identity;
4. historical contracts repeatedly preserve request-ID lanes as correlation-only and separate from logical/session identity;
5. C03e-BG explicitly left the authentication PRWM request-id allocator unselected.

No later historical checkpoint audited for ND proves a distinct production allocator/custody source for this exact expected-request field.

Therefore:

`PRODUCTION_EXPECTED_REQUEST_AUTHENTICATION_PRWM_REQUEST_ID_SOURCE_CUSTODY = UNPROVEN`

No constant, zero, random ad hoc value, PRWC ID reuse, capability-frame request ID, requester-rendezvous request ID, or process-global counter may be substituted.

## 10. Verifier-time historical authority assessment

The expected request owns a generic verifier-time provider:

`T: FnMut() -> u64 + Send + 'static`

Closed C03e-CE selected server/verifier wall-clock semantics for the Phase-129 PRWA verifier:

- `SystemTime::now()`;
- checked duration since `UNIX_EPOCH`;
- whole `u64` seconds;
- fail closed on pre-epoch/overflow;
- fresh observation at proof verification.

Again, this is related verifier-side precedent but not automatic source provenance for the expected-device PRWM request lane.

No exact C03e-NC production source audited here proves construction of the `T` provider and its movement into `RemoteSessionExpectedDeviceAdmissionRequest::new(...)`.

Separately, the repeated-admission supervisor obtains `RemoteSessionRealAdmissionTiming` through another callback at actual attempt start. C03e-ND must not collapse these two timing surfaces or freeze attempt timing prematurely in the queue producer.

Therefore:

`PRODUCTION_EXPECTED_REQUEST_VERIFIER_TIME_PROVIDER_SOURCE_CUSTODY = UNPROVEN`

and:

`PRODUCTION_REAL_ADMISSION_TIMING_SOURCE_CUSTODY = SEPARATELY_UNPROVEN_FOR_COMPLETE_PRODUCTION_COMPOSITION`

## 11. Expected DeviceId scheduling provenance assessment

Historical AJ/AK/AL and BG semantics are consistent:

- the expected `DeviceId` is pre-authentication scheduling intent;
- duplicate scheduling may be rejected before network/authentication work;
- current registry state remains authoritative for lower-transport resolution;
- authenticated owner-derived `DeviceId` remains authoritative after authentication;
- expected-device intent is not transport identity and is not post-authentication proof.

However no exact production source audited at C03e-NC proves the upstream event/source that authoritatively emits the expected logical `DeviceId` into the expected-request queue.

Candidate-publication peer selection, requester identity, requester-rendezvous target nomination, bound endpoint address, peer socket address, current remote peer identity, environment literals, test fixtures, and a fixed DeviceId are all rejected as implicit substitutes.

Therefore:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_SOURCE_CUSTODY = UNPROVEN`

## 12. Sender/channel lifecycle assessment

The consumer side is materialized as a Tokio bounded `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`.

No exact production source audited for ND proves:

- creation of the production `mpsc::channel(...)` for expected requests;
- channel capacity provenance;
- ownership of the `mpsc::Sender`;
- which event is allowed to send one expected request;
- send backpressure/failure handling;
- sender cloning policy;
- sender-drop semantics;
- shutdown ordering between producer, receiver, endpoint lifecycle, and process supervisor;
- prevention of sends after shutdown or authority revocation;
- whether queued requests are drained, rejected, or abandoned during shutdown.

The consumer contract already states that closure of the request source is not equivalent to supervisor shutdown. Therefore an arbitrary sender-drop implementation could change lifecycle semantics.

Result:

`PRODUCTION_EXPECTED_REQUEST_SENDER_CHANNEL_LIFECYCLE = UNPROVEN`

## 13. Authority matrix

| Required input / custody | Historical semantic law | Exact production source/custody proven for expected-request producer? | ND result |
| --- | --- | --- | --- |
| current registry / transport resolution | AJ/BG current-authority law | consumer/admission side exists | preserved; not producer field |
| expected `DeviceId` | AJ/AK/BG scheduling-intent law | no authoritative production upstream emitter proven | unresolved |
| `SessionId` | CE provides PRWA verifier precedent | no applicability + producer seam proven for PRWM expected-request lane | unresolved |
| authentication PRWM request ID | BG requires separate custody | BY/CJ are PRWC lane only; no PRWM allocator source proven | unresolved |
| dispatcher | NB status-only dispatcher materialized | type exists; per-request production constructor/caller not proven | partially solved, propagation blocked |
| verifier-time provider `T` | CE provides PRWA wall-clock precedent | no expected-request provider construction source proven | unresolved |
| AJ admission timing bundle | AK/AL fresh-at-attempt law | no complete production timing producer proven | unresolved |
| expected-request sender/channel | receiver semantics materialized | no production sender/channel owner proven | unresolved |

## 14. Source-composition conclusion

No exact production source at C03e-NC owns all required values together.

No historical checkpoint can be safely interpreted as having already selected a complete composition owner.

The smallest safe conclusion is therefore **not** to add a constructor wrapper around `RemoteSessionExpectedDeviceAdmissionRequest::new(...)`.

A wrapper that merely accepts unresolved values as parameters would not close producer provenance. A wrapper that invents those values would violate BG/NC authority ordering. A wrapper that creates a channel without upstream event/shutdown authority would invent lifecycle semantics.

C03e-ND consequently selects a documentation-only blocking boundary:

`EXPECTED_REQUEST_COMPOSITION_REQUIRES_EXPLICIT_UPSTREAM_SCHEDULING_AND_CORRELATION_SOURCE_SELECTION_BEFORE_SOURCE_MATERIALIZATION`

## 15. Immediate successor ceiling

After evidence-closed C03e-ND, the next separately gated checkpoint may select **one missing upstream production authority boundary only**.

The preferred next prerequisite is:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_INGRESS_AND_EXPECTED_REQUEST_SENDER_CHANNEL_OWNERSHIP`

That selection must answer, without source mutation:

1. which real production event/lifecycle yields an authoritative expected `DeviceId` scheduling intent;
2. which owner creates and retains the bounded expected-request sender;
3. channel capacity provenance;
4. sender clone/no-clone policy;
5. send backpressure and closed-channel behavior;
6. producer shutdown and sender-drop ordering relative to the existing endpoint supervisor;
7. how revocation/currentness is rechecked so queued scheduling intent cannot become transport/auth authority;
8. whether that owner also has legitimate custody of `SessionId`, authentication PRWM request ID, dispatcher, and verifier-time provider, or whether those remain further prerequisites.

If that source does not own all remaining constructor values, a later checkpoint must select those remaining authorities separately. No source materialization is inherited merely from ND closure.

## 16. Explicitly not authorized by C03e-ND

C03e-ND does not authorize:

- Rust/Kotlin/source mutation;
- `RemoteSessionExpectedDeviceAdmissionRequest::new(...)` caller materialization;
- expected-request `mpsc::channel(...)` creation;
- sender construction, cloning, retention, or send calls;
- synthetic/fixed/default `DeviceId`;
- environment-derived expected DeviceId;
- reuse of current peer identity as scheduling intent;
- reuse of requester/rendezvous target identity;
- `SessionId` generation or allocator materialization;
- PRWC request-ID reuse as authentication PRWM request ID;
- a new authentication PRWM request-id allocator;
- clock/time-provider source materialization;
- admission timing policy materialization;
- NB dispatcher caller propagation;
- provider construction;
- capability authorization changes;
- registry hydration or policy mutation;
- authentication algorithm/protocol changes;
- endpoint/listener/process activation;
- `run()` or `main.rs` mutation;
- readiness publication;
- deployment/restart/recovery;
- systemd/package/credential/certificate/trust/RBAC changes;
- Cargo manifest/lockfile/workflow changes;
- repository configuration/visibility mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update/rebase/squash/history rewrite.

## 17. Validation expectations

This checkpoint is documentation-only.

Closure validation must bind to the exact final ND head and prove:

- predecessor/merge base exact C03e-NC head;
- exactly one changed contract path;
- zero Rust/source/runtime/Cargo/lockfile/workflow changes;
- PRW Rust Validation terminal success on exact final head;
- path-filtered workflow conclusions recorded exactly;
- Android conclusion only if an exact-head Android workflow actually runs;
- immutable Drive audit published once to the canonical evidence parent with exact raw readback byte/hash verification;
- final branch/main/PR/successor race checks.

## 18. Gate / closure classification

Target gate:

`C03E_ND_PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_HISTORICAL_AUTHORITY_SOURCE_COMPOSITION_SELECTED`

Expected closure result:

`PARTIAL_HISTORICAL_AUTHORITY_LAWS_CONFIRMED / COMPLETE_PRODUCTION_EXPECTED_REQUEST_COMPOSITION_OWNER_NOT_PROVEN / SOURCE_MATERIALIZATION_BLOCKED`

C03e-ND closes only the historical-authority/source-composition assessment. Runtime activation remains 0%.

After closure: **STOP and fresh exact-head audit before the next authority-selection checkpoint.**
