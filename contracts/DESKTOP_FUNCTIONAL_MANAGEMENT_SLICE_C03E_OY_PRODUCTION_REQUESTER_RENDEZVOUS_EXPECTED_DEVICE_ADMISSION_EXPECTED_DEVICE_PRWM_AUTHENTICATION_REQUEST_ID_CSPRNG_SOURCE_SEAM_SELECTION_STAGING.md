# C03e-OY — expected-device PRWM authentication request-ID CSPRNG source seam selection

## Status

`SELECTION — VALIDATION_PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_EXPECTED_DEVICE_PRWM_AUTHENTICATION_REQUEST_ID_CSPRNG_SOURCE_SEAM_SELECTION`

Selected future source boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_EXPECTED_DEVICE_PRWM_AUTHENTICATION_REQUEST_ID_CSPRNG_SOURCE_MATERIALIZATION`

## Authoritative predecessor

C03e-OY begins only after exact C03e-OX durable closure.

Exact predecessor branch:
`phase-152-c03e-ox-production-requester-rendezvous-expected-device-admission-target-admission-session-id-csprng-source-materialization`

Exact predecessor head:
`e00904f27bd80304e36e8900245e0337491179fe`

Exact predecessor tree:
`8393e2562b7fceb148f7ab527a104c582d33bdd1`

Exact predecessor endpoint-owner source blob:
`34393595b535daa23ef0d319ac1650f1dd2ba9b8`

Predecessor PR:
`#537`

Predecessor lifecycle:
`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Predecessor canonical Drive evidence ID:
`1ompBB95hg5Jn9pQ8YWHimmDMYfvMtyOY`

Predecessor evidence bytes:
`18123`

Predecessor evidence SHA-256:
`f08a0acc68f9ba7dc65489083d45032b4a13218ab9d6a35c9f74357685f9fe7b`

## Fresh concurrency guard before token assignment

Before assigning C03e-OY, live GitHub was re-read and showed:

- exact C03e-OX PR #537 remained `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- PR #537 remained draft/open/unmerged/mergeable on exact C03e-OW/C03e-OX refs;
- exact C03e-OX branch remained at `e00904f27bd80304e36e8900245e0337491179fe`, tree `8393e2562b7fceb148f7ab527a104c582d33bdd1`;
- exact endpoint-owner source remained blob `34393595b535daa23ef0d319ac1650f1dd2ba9b8`;
- recent PR chronology remained headed by #537;
- branch search for `phase-152-c03e-oy` returned zero results;
- title-scoped PR search for `C03e-OY:` returned zero results;
- the exact proposed C03e-OY contract path returned `404` on exact C03e-OX;
- a broad PR search self-matched #537 only because its closed body states that C03e-OY is not assigned; that self-reference is not a competing successor.

Only after these guards was C03e-OY assigned.

## Fresh exact-current source proof

### Existing request carrier remains raw-u64 correlation custody

Exact C03e-OX source in:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

contains the existing pre-authentication carrier:

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>`

with independent fields:

- `expected_device_id: DeviceId`;
- `session_id: SessionId`;
- `authentication_request_id: u64`;
- `dispatcher: D`;
- `verifier_time_unix_seconds: T`.

Its existing `new(...)` accepts `authentication_request_id: u64` directly, and `into_parts()` returns the same exact `u64` by value.

C03e-OY does not change that request type, constructor, storage representation or visibility.

### Existing real-admission transaction preserves caller-supplied request ID

Exact C03e-OX source in:
`crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`

already accepts:

`authentication_request_id: u64`

as an explicit caller-owned input to:

`admit_expected_remote_device_session(...)`.

After expected-peer resolution, acceptance and challenge preparation, the transaction forwards that exact value unchanged into:

`complete_registry_bound_session_authentication(...)`.

No request-ID generation occurs inside real-admission today.

### Existing authentication transaction defines the correlation semantics

Exact C03e-OX source in:
`crates/prw-agent/src/remote_session_authentication_transaction.rs`

already accepts:

`request_id: u64`

as an explicit input to:

`complete_registry_bound_session_authentication(...)`.

The existing transaction:

1. sends exactly one Challenge message using that exact request ID;
2. receives exactly one Proof candidate;
3. compares the inbound Proof request ID against the exact outbound Challenge request ID;
4. fails with `RequestIdMismatch` if they differ;
5. performs no request-ID generation, retry or replacement inside the wire transaction.

Therefore request-ID generation remains correctly upstream of the existing authentication transaction.

### Existing OX randomness dependency already exists in the target source file

Exact C03e-OX endpoint-owner source:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

already imports:

`aws_lc_rs::rand::{SecureRandom, SystemRandom}`

for the independently materialized C03e-OX target-admission SessionId source.

The same crate already had a direct `aws-lc-rs` dependency before OX, so a future request-ID source does not require Cargo/lockfile widening solely for randomness.

C03e-OY does not reuse C03e-OX random bytes, SessionId representation or any PRWA context object. It selects only a separate fresh CSPRNG draw through the already-available API.

## Historical authority retained from C03e-OB

C03e-OB remains authoritative for the expected-device PRWM request-ID lane:

- the request ID is correlation only;
- it is independently generated for the expected-device authentication transaction;
- it must be nonzero;
- exactly one outbound Challenge and its inbound Proof share the same request ID;
- the inbound Proof request ID must equal the outbound Challenge request ID;
- PRWC request-ID values and owners must not be reused;
- historical PRWC lifecycle machinery is precedent only and is not a current source owner for this lane.

C03e-OY does not widen this request ID into identity, authorization, replay authority, scheduling authority, endpoint authority or persistence key.

## Selected next source boundary

C03e-OY selects the next separately gated source boundary as:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_EXPECTED_DEVICE_PRWM_AUTHENTICATION_REQUEST_ID_CSPRNG_SOURCE_MATERIALIZATION`

The immediate future hard source ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

If correct materialization requires any second Rust path, Cargo/lockfile change, wire-type change, request-carrier change, public visibility widening, shared counter/static owner, PRWC lifecycle reuse, request construction, scheduling-grant opening, dispatcher construction, verifier-time wiring, channel/sender work, producer specialization or lifecycle activation: `STOP` and return to selection.

## Selected future request-ID source shape

The future materialization may add only one private Agent-local dormant request-ID source plus narrowly necessary private constants/error representation and focused same-file tests if practical.

A future helper equivalent to:

`new_remote_session_expected_device_authentication_request_id()`

must return one `Result<u64, ...>` and materialize only the following law.

### Exact CSPRNG generation law

The future source must:

1. allocate exactly one local `[0_u8; 8]` entropy buffer;
2. construct/use the OS-backed `aws_lc_rs::rand::SystemRandom` source through `SecureRandom`;
3. perform exactly one `fill(...)` into that exact 8-byte buffer;
4. map randomness acquisition failure directly to a bounded private source error and fail closed;
5. convert those exact 8 bytes to exactly one `u64` using one explicit stable byte-order conversion selected as `u64::from_be_bytes(...)`;
6. reject the value `0` and fail closed;
7. return the nonzero `u64` unchanged;
8. perform no retry, redraw, increment, decrement, wrap, fallback or replacement after either randomness failure or zero output.

The future source error family should remain private and bounded to source-local reasons equivalent to:

- `Randomness`;
- `Zero`.

The source error must not absorb wire, authentication, session, dispatcher, verifier-time, request-construction, channel, endpoint, worker, scheduling or receipt errors.

## Why CSPRNG rather than a shared counter

The exact current source graph has no materialized expected-device request-ID lifecycle owner or counter.

A private local CSPRNG source is the smallest independently materializable source because it:

- requires no global/static mutable state;
- requires no `AtomicU64`, mutex, registry record or process-lifetime counter owner;
- does not invent restart persistence or uniqueness authority;
- does not reuse PRWC request-ID state or ownership;
- does not couple request IDs to requester identity, target identity, SessionId, scheduling provenance, wall clock or endpoint state;
- uses an exact-current direct dependency and API already present in the selected one-file source ceiling.

C03e-OY does not claim global uniqueness. The selected semantics require fresh independent nonzero correlation for one bounded Challenge -> Proof transaction only.

## Independence law

The future expected-device PRWM request ID must be generated from a distinct local CSPRNG draw.

It must not be derived from, copied from, aliased to, hashed from, encoded from or otherwise reuse:

- the target-admission SessionId generated by C03e-OX;
- any byte from the C03e-OX 32-byte SessionId entropy buffer;
- requester callback `DeviceId`;
- target expected `DeviceId`;
- requester scheduling `SessionId`;
- any PRWC correlation value;
- requester-rendezvous outer request IDs;
- terminal-ack correlation;
- endpoint/transport/candidate/registry/reachability identifiers;
- verifier time or wall-clock time;
- dispatcher state;
- scheduling-grant content.

Using the same `SystemRandom` API type is not value/state reuse: the future source must request a separate fresh 8-byte draw for this lane.

## Nonzero / zero-output law

`0` is not an authorized expected-device PRWM authentication request ID.

If the one selected 8-byte CSPRNG draw decodes to zero:

- return the bounded `Zero` source failure;
- do not redraw;
- do not convert zero to one;
- do not increment/wrap;
- do not borrow another correlation value;
- do not reuse a SessionId fragment;
- do not continue request construction with zero.

The future producer/composition layer must decide separately how source failure maps to the already-selected `ConstructionFailed` terminal receipt while preserving one-shot scheduling authority. OY does not select that composition ordering.

## Request-ID transaction scope

The generated nonzero `u64` authorizes no operation by itself.

Its only selected meaning is correlation for exactly one expected-device logical-session authentication Challenge -> Proof exchange.

Once later request construction installs that value:

- outbound Challenge uses that exact value;
- inbound Proof must carry that exact value;
- mismatch remains the existing authentication transaction's `RequestIdMismatch` failure;
- the request ID is not reused for a second Challenge;
- it is not recycled after failure;
- it is not a capability token;
- it is not proof of identity, authorization, scheduling eligibility, successful authentication or successful admission.

## Target SessionId separation preserved

C03e-OX target-admission SessionId generation remains a separate completed source seam.

The target SessionId:

- is 32 random bytes encoded as 64 lowercase hexadecimal characters and typed through `SessionId::new(...)`;
- remains the logical-session identifier;
- is not the PRWM request ID;
- is not a source of PRWM request-ID bits;
- has its own duplicate-session authority later in `SessionAuthenticationService::begin_session(...)`.

The PRWM request ID remains a raw nonzero `u64` correlation lane with independent generation and no shared retry law.

## One-shot scheduling grant law preserved

C03e-OY does not inspect, open, consume, dispose, clone, copy, reconstruct, remint, replay, refund or roll back `ExpectedDeviceSchedulingAuthorityGrant`.

The future request-ID helper must likewise accept no scheduling grant and inspect no grant field.

Target expected `DeviceId` and requester scheduling provenance remain sealed inside the one-shot grant until a later separately gated construction/custody composition boundary.

No request-ID generation success or failure creates scheduling authority or authorizes grant remint/retry.

## Construction ordering remains deferred

C03e-OY deliberately does not select ordering among:

- target-admission SessionId generation;
- expected-device PRWM authentication request-ID generation;
- scheduling-grant `into_parts()`;
- NB status-only dispatcher production construction/transfer;
- verifier-time acquisition;
- `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- `ConstructionFailed` receipt composition;
- channel send.

The future request-ID helper is an independently materializable prerequisite only.

A later composition selection must define exactly where failure can occur relative to one-shot grant opening and how an already-consumed grant is terminally disposed on construction failure.

## Dispatcher boundary remains separate

C03e-OY constructs or transfers no NB status-only dispatcher.

Existing `CapabilityDispatcher` generic custody remains untouched.

Production construction/transfer of the concrete NB status-only dispatcher remains separately gated.

## Verifier-time boundary remains separate

C03e-OY creates no verifier-time source or adapter.

It does not:

- call `SystemTime::now()` for request ID generation;
- derive request ID from time;
- resolve the existing fallible PRWA clock vs request-carried `T: FnMut() -> u64` compatibility question;
- panic/default/saturate/freeze verifier time;
- widen the verifier-time interface.

## Request construction remains separate

C03e-OY does not construct `RemoteSessionExpectedDeviceAdmissionRequest`.

It does not fill:

- expected target `DeviceId`;
- target SessionId;
- dispatcher;
- verifier-time provider;
- authentication request ID into a live request.

It does not build any authentication payload or invoke real admission.

## Receipt composition remains separate

C03e-OY does not compose:

- `ConstructionFailed`;
- `ChannelClosed`;
- `Enqueued`.

Existing `SuppressedOnShutdown` composition remains untouched.

A future request-ID source failure is merely one possible construction-phase failure cause. Exact terminal grant disposal and conversion to `ConstructionFailed` remain separately gated.

## Producer / channel / backpressure law preserved

C03e-OE and subsequent producer selections remain authoritative:

- eventual expected-request handoff uses exactly one bounded Tokio MPSC channel;
- capacity is exactly `1`;
- exactly one higher production owner retains the sole sender;
- receiver is created once and moved once;
- eventual enqueue is only `sender.send(request).await`;
- a full channel means asynchronous backpressure;
- channel closure is terminal for that one-shot produced continuation.

C03e-OY creates no channel/sender and performs no send.

Still forbidden:

- sender clone;
- `try_send`;
- `blocking_send`;
- callback `block_on`;
- hidden/detached producer spawn;
- alternate/unbounded/retry queue;
- second producer future;
- scheduling rollback/remint/replay after channel failure.

## Shutdown law preserved

Explicit supervisor shutdown remains the sole endpoint-supervisor shutdown authority.

C03e-OY adds no shutdown detection, mapper, peer disposal, receiver close, sender drop, worker cancellation or endpoint close behavior.

The existing C03e-OV shutdown-suppression receipt mapper remains unchanged.

## No lifecycle activation

C03e-OY is documentation-only.

The selected future helper is intended to remain private and dormant when materialized.

No existing endpoint lifecycle method, executor, lower cooperative driver, process runtime, listener, readiness/bootstrap or executable path is selected for migration or activation here.

## Immediate future hard source ceiling

Only after C03e-OY is exact-final-head validated, evidence-recorded and closed may a fresh successor audit consider source materialization.

The only permissible immediate future source path is:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Allowed future delta only:

- one private constant equivalent to request-ID random byte width `8`, if useful;
- one private bounded source error with only `Randomness` and `Zero`;
- one private dormant expected-device PRWM authentication request-ID generator;
- exactly one 8-byte `SystemRandom` fill;
- exactly one `u64::from_be_bytes(...)` conversion;
- explicit zero rejection;
- narrowly required dormant lint acknowledgement;
- focused same-file tests if practical without introducing injectable production randomness or broader interfaces.

Forbidden in the immediate source-materialization stage:

- second source path;
- Cargo/lockfile changes;
- PRWC lifecycle/value/owner reuse;
- shared/static/atomic request-ID counter;
- retries/redraws/fallbacks;
- request-carrier changes;
- authentication-transaction changes;
- real-admission transaction changes;
- scheduling-grant access/opening;
- target SessionId generator changes;
- dispatcher construction/transfer;
- verifier-time wiring;
- expected-device request construction;
- producer specialization/invocation;
- channel/sender construction/send;
- receipt composition;
- lifecycle/caller migration;
- runtime activation/deployment.

## Validation requirement for this selection

All PASS claims for C03e-OY must bind only to the exact final C03e-OY documentation head.

At minimum:

- PRW Rust Validation must reach terminal `SUCCESS` on the exact final head;
- every pull-request-triggered workflow on that exact head must be enumerated;
- `SKIPPED` must never be reported as PASS;
- Android PASS may be claimed only if an Android workflow exists on the same exact head and reaches terminal `SUCCESS`;
- no predecessor CI result may be inherited as OY PASS.

## Durable evidence requirement

After exact-final-head validation:

1. fresh exact-title search in canonical Drive evidence parent must return zero artifacts;
2. freeze an immutable plain-text audit with status `SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`;
3. publish exactly one canonical raw `text/plain` artifact;
4. verify exact filename, canonical parent, MIME and provider size;
5. perform raw Drive readback and verify exact byte count and SHA-256 against frozen bytes;
6. exact-title canonical-parent postsearch must return exactly one canonical artifact with the same Drive ID;
7. remove any temporary staging surface used only to generate export bytes;
8. re-read C03e-OY branch/head/tree/contract, PR, recent chronology and `main`;
9. verify successor namespace remains zero-result and unassigned;
10. only then mark PR body `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

Canonical Drive evidence parent remains:
`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

## Remaining separately gated dependencies after C03e-OY selection

Still deferred:

- source materialization of this selected expected-device PRWM authentication request-ID source;
- NB status-only dispatcher production construction/transfer;
- verifier-time source/interface compatibility;
- exact construction-order and custody composition around the one-shot scheduling grant;
- actual `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- `ConstructionFailed` terminal grant disposal and receipt composition;
- actual higher producer/channel owner construction;
- sole-sender producer closure;
- `sender.send(request).await`;
- `ChannelClosed` receipt composition;
- `Enqueued` receipt composition;
- specialization/invocation of the generic producer path with concrete receipt;
- higher process/runtime caller migration;
- requester cleanup/candidate/reachability continuation;
- target dial/listener/bootstrap/readiness/executable activation;
- deployment.

No ordering among independent later construction-input gates is invented beyond the source-local law selected here.

## Explicit non-actions

C03e-OY is documentation-only.

No Rust/source/runtime mutation.
No request-ID source materialization.
No random request-ID generation at runtime.
No PRWC request-ID lifecycle/value/owner reuse.
No shared/static/atomic counter.
No target SessionId source mutation.
No grant inspection/opening/extraction/disposal/clone/copy/reconstruction/remint/replay/refund/rollback.
No dispatcher production construction/transfer.
No verifier-time source/interface mutation.
No expected-device request construction.
No authentication payload construction.
No channel or sender creation/clone.
No `sender.send(request).await`.
No producer specialization/invocation.
No `ConstructionFailed`, `ChannelClosed` or `Enqueued` composition.
No callback `block_on`, `try_send`, `blocking_send`, hidden/detached spawn, alternate/unbounded/retry queue or second producer future.
No endpoint lifecycle/executor/lower-driver/caller mutation.
No listener/readiness/bootstrap/executable activation.
No Cargo/lockfile/workflow/Android source mutation.
No deployment.
No merge.
No PR ready-for-review conversion or PR closure.
No branch deletion.
No force update/reset/rebase/squash/history rewrite.
No repository configuration/ruleset/permission mutation.

No successor token is assigned by C03e-OY.
A fresh exact-head/concurrency audit is mandatory before the selected source materialization.

Keep the C03e-OY PR draft/open/unmerged.

After durable C03e-OY closure: `STOP`.
