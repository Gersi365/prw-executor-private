# Phase 152 C03e-OB — Production requester/rendezvous expected-device admission identifier, timing, and sender production custody selection

Status: `STAGING — SELECTION ONLY — SOURCE MATERIALIZATION BLOCKED`

Gate reserved for closure:
`C03E_OB_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_IDENTIFIER_TIMING_AND_SENDER_PRODUCTION_CUSTODY_SELECTED`

Closure reserved for exact validated/evidence-recorded head only:
`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_IDENTIFIER_TIMING_AND_SENDER_PRODUCTION_CUSTODY_SELECTION`

## 1. Exact predecessor authority

This checkpoint is documentation-only and is rooted at exact closed C03e-OA:

`c612eb9f10836f4f106a3bdab8262ac750c346be`

Exact C03e-OA tree:

`9bfbc71fc7d42c9b087e0d75b73c4c2861060443`

C03e-OA proved that target `DeviceId` provenance is resolved only by the consumed construction-eligible `ExpectedDeviceSchedulingAuthorityGrant`, while target admission `SessionId`, authentication PRWM request ID, request-carried verifier-time provider, expected-request sender/channel ownership, and NB dispatcher caller composition remained unresolved.

C03e-OB does not construct or send an expected-device admission request.

## 2. Question selected here

The only question closed by this checkpoint is:

> What exact authority/lifecycle laws must govern the still-missing target admission `SessionId`, authentication PRWM request ID, verifier-owned timing, and expected-request sender/channel custody before a coherent expected-device admission producer or any source-materialization seam may be selected?

This is not a source implementation checkpoint.

## 3. Exact current receiver-compatible request

The exact OA source retains:

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>`

with exactly five owned constructor inputs:

1. expected target `DeviceId`;
2. target admission `SessionId`;
3. authentication request ID as `u64`;
4. dispatcher `D`;
5. verifier-time provider `T`.

`authentication_request_id == 0` is rejected by the request constructor. The constructor validates but does not allocate the request ID.

The request transfers dispatcher and `T` custody by value into the receiver-side executor path.

## 4. Exact expected-device authentication transaction facts

Exact OA `admit_expected_remote_device_session(...)` consumes:

- expected logical `DeviceId`;
- target `SessionId`;
- verifier-owned challenge validity range;
- authentication PRWM request ID;
- verifier-owned proof-submission `authentication_now_unix_seconds`;
- application lease range.

Exact OA `complete_registry_bound_session_authentication(...)`:

1. accepts exactly one peer control stream;
2. sends one challenge using the supplied PRWM `request_id`;
3. receives exactly one proof candidate;
4. requires the returned proof frame request ID to equal that exact request ID;
5. requires the proof session identifier to equal the prepared challenge `SessionId`;
6. submits the proof exactly once using verifier-owned `now_unix_seconds`;
7. aborts pending session state and closes the peer on terminal failure.

Therefore request ID and SessionId are distinct correlation domains and neither may substitute for the other.

## 5. Timing domains remain distinct

Exact OA proves two separate timing custody surfaces:

### 5.1 Real-admission timing bundle

`RemoteSessionRealAdmissionTiming` owns:

- challenge validity `Range<u64>`;
- authentication proof-verification `now_unix_seconds`;
- application lease `Range<u64>`.

This bundle is supplied by the receiver-side real-admission caller. C03e-OB does not reinterpret it as an expected-request constructor field.

### 5.2 Request-carried verifier-time provider `T`

The expected request separately owns `T`, with current bounds:

`T: FnMut() -> u64 + Send + 'static`

That provider is moved into the authenticated capability worker after real admission succeeds.

C03e-OB therefore forbids conflating:

- challenge issue/expiry time;
- proof-submission time;
- application lease time;
- request-carried capability verifier time.

Any later producer must preserve those distinct authorities even if a future server-local clock source is shared underneath.

## 6. Target admission SessionId production law

The target admission `SessionId` MUST be fresh, server-local, and independent of every requester/scheduling/correlation identity.

C03e-OB selects the existing PRWA verifier source only as a proven **algorithmic precedent**, not as a reusable expected-admission context object.

Selected future SessionId generation law:

1. obtain exactly 32 bytes from an OS-backed cryptographically secure random source;
2. encode them as exactly 64 lowercase hexadecimal ASCII bytes;
3. construct the existing typed `SessionId` from that opaque value;
4. perform no normalization, truncation, timestamp prefixing, DeviceId prefixing, request-ID embedding, or requester-session derivation;
5. one generation attempt belongs to exactly one construction-eligible consumed scheduling grant;
6. generation failure fails closed and produces no request;
7. there is no producer collision-retry loop and no scheduling-grant remint/retry/replay.

The concrete PRWA `PrwaVerifierSessionContext` MUST NOT be reused as the expected-admission producer because it bundles PRWA-specific challenge-window custody and exposes the SessionId only through its own PRWA context.

The algorithmic precedent may be re-expressed only in a separately gated expected-admission source primitive.

## 7. SessionId uniqueness and lifetime authority

Exact OA `SessionAuthenticationService::begin_session(...)` remains the authoritative duplicate-session gate. It rejects a `SessionId` already pending or authenticated as `SessionAlreadyExists`.

Therefore:

- producer freshness is probabilistic cryptographic generation;
- authoritative current-process uniqueness is enforced by `SessionAuthenticationService`;
- a duplicate rejection is terminal for that consumed scheduling grant;
- C03e-OB selects no retry with a replacement SessionId;
- C03e-OB selects no grant remint, rollback, or replay after duplicate rejection;
- the generated target SessionId belongs only to the one expected admission transaction and resulting authenticated session if successful.

The requester `SessionId` carried by scheduling authority MUST NOT be reused as the target admission SessionId.

## 8. Authentication PRWM request-ID law

The authentication PRWM request ID is correlation-only for one expected-device logical authentication transaction.

Selected law:

- it MUST be non-zero;
- it MUST be independently produced for the expected-device PRWM lane;
- the same exact ID MUST label the outbound challenge and MUST equal the inbound proof frame ID;
- a mismatch is terminal authentication failure;
- it MUST NOT become DeviceId, SessionId, scheduling authority, freshness authority, policy authority, transport identity, or capability authority;
- it MUST NOT be sourced from requester-rendezvous PRWC correlation, candidate-publication PRWC correlation, configured values, timestamps, hashes, SessionId bytes, or test literals.

No current exact OA production allocator for this expected-device PRWM ID is proved.

## 9. PRWC request-ID history is precedent only

Historical PRWC request-ID checkpoints selected connection-local non-zero correlation behavior, but direct exact-current OA source is authoritative.

Exact OA `crates/prw-control-transport/src/lib.rs` currently validates non-zero PRWC `ControlFrame.request_id` values but does not expose a current production expected-device PRWM allocator or custody owner.

Therefore C03e-OB selects:

`PRWC_REQUEST_ID_LAW_MAY_INFORM_CORRELATION_DISCIPLINE / PRWC_VALUE_OR_OWNER_REUSE_FOR_EXPECTED_DEVICE_PRWM_AUTHENTICATION_FORBIDDEN`

A future expected-device PRWM producer must receive its own separately materialized request-ID source.

## 10. Authentication request-ID uniqueness scope

C03e-OB requires a future expected-device PRWM request-ID source to prevent two simultaneously live authentication transactions owned by the same producer instance from sharing one request ID.

The future source must also define deterministic exhaustion behavior before request send.

C03e-OB does not select a concrete counter/random allocator implementation because no exact current source seam or owner has yet been proved for that state.

Global cross-process uniqueness and durable persistence are not required or selected.

No reuse after producer restart, persistence, recovery replay, or distributed allocator is selected.

## 11. Verifier-owned time law

All timing used to create or verify expected-device authentication authority MUST be server/verifier owned.

Requester input, scheduling timestamps, acknowledgement timestamps, endpoint observations, candidate freshness values, request IDs, SessionIds, and transport timestamps are forbidden substitutes.

The current PRWA helper `current_prwa_verifier_unix_seconds()` is a proven server-local wall-clock precedent, but it returns a fallible `Result<u64, PrwaVerifierSourceError>` and is PRWA-specific.

The expected-request `T` surface is currently infallible `FnMut() -> u64`.

C03e-OB therefore explicitly DOES NOT select an adapter that:

- unwraps or panics on clock failure;
- substitutes zero;
- saturates silently;
- freezes one issue-time sample for all later capability verification;
- converts a scheduling/requester timestamp into verifier time.

Selected state:

`SERVER_LOCAL_VERIFIER_TIME_AUTHORITY_REQUIRED / CURRENT_PRWA_RESULT_RETURNING_SOURCE_NOT_DIRECTLY_SHAPE_COMPATIBLE_WITH_EXPECTED_REQUEST_T / FAIL_CLOSED_EXPECTED_LANE_TIME_SOURCE_DESIGN_REMAINS_SEPARATELY_GATED`

## 12. Admission timing and request-carried T must not alias accidentally

A later source design may deliberately derive multiple verifier-owned timing surfaces from one server-local clock authority only after it proves:

- challenge validity semantics remain unchanged;
- proof-submission time remains a fresh verifier observation where required;
- application lease timing remains independently correct;
- capability verifier time remains current enough for its own authorization checks;
- a clock acquisition/representation failure cannot fabricate valid time;
- no caller/requester-controlled timestamp enters any of these surfaces.

C03e-OB itself selects no such adapter implementation.

## 13. Expected-request sender/channel exact-current finding

Exact OA `linux_bootstrap.rs` accepts only:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

through `LinuxAgentRemoteProcessOperationInputs` and downstream executor composition.

Fresh exact-current audit found `mpsc::channel(...)` creation for this shape only in tests. No production expected-request `mpsc::Sender<...>` owner, channel-construction site, or producer lifetime is proved in the current bootstrap.

Therefore:

`CURRENT_PRODUCTION_COMPOSITION_REMAINS_RECEIVER_INJECTED / EXPECTED_REQUEST_SENDER_OWNER_NOT_PROVEN`

## 14. Sender/channel custody law

Any future expected-request channel MUST be the matching sending side for the already-selected receiver-side executor lane; it MUST NOT introduce an unbounded bypass queue or a second independent admission pipeline.

A future sender owner must define before source materialization:

1. one authoritative channel-construction point;
2. finite positive capacity;
3. exact relationship, if any, between channel capacity and active-worker capacity;
4. whether sender cloning is prohibited or narrowly bounded;
5. exact backpressure behavior;
6. closed-channel behavior;
7. shutdown ordering between producer sender drop and receiver/executor termination;
8. how an unsent request is disposed after the scheduling grant has already been consumed;
9. proof that send failure does not remint/replay scheduling authority.

C03e-OB does not invent the capacity value or owner location because exact-current source does not prove them.

## 15. Sender failure after scheduling consumption

The scheduling-consumption tombstone remains terminal producer-side authority consumption.

Therefore a later failure to:

- generate SessionId;
- produce authentication request ID;
- acquire verifier-time custody;
- construct the dispatcher;
- construct the channel request;
- enqueue/send because of full/closed channel;

MUST fail closed for that consumed grant and MUST NOT delete scheduling consumption, remint a grant, retry automatically, or synthesize a replacement admission attempt.

## 16. NB dispatcher construction remains a separate composition dependency

Exact OA retains the C03e-NB crate-private status-only:

`LinuxAgentProductionRemoteCapabilityDispatcher`

in `crates/prw-agent/src/linux_bootstrap.rs`.

Its current production expected-request caller construction remains unmaterialized. Exact-current uses of `LinuxAgentProductionRemoteCapabilityDispatcher::new(...)` observed in this audit are test-side, not a production request producer.

C03e-OB therefore does not claim a complete producer merely because the dispatcher type exists.

Unsupported file/transfer/terminal/forwarding families remain fail-closed.

## 17. Coherent producer requirements after this selection

A later coherent expected-request producer must own, for one consumed scheduling grant:

1. the exact target DeviceId from that grant;
2. one freshly generated target admission SessionId under section 6;
3. one independently owned non-zero PRWM authentication request ID under sections 8–10;
4. one allowed NB status-only dispatcher instance unless a later gate changes that dispatcher selection;
5. one verifier-owned request-carried `T` whose failure/time semantics are explicitly solved;
6. one selected sender for the existing receiver-side executor lane.

These values must refer to one and only one expected admission attempt.

## 18. Classification

C03e-OB selection result:

`TARGET_ADMISSION_SESSION_ID_SERVER_LOCAL_CSPRNG_LAW_SELECTED / PRWA_CONTEXT_OBJECT_REUSE_FORBIDDEN / SESSION_AUTHENTICATION_SERVICE_REMAINS_DUPLICATE_AUTHORITY / EXPECTED_DEVICE_PRWM_REQUEST_ID_NONZERO_SAME_TRANSACTION_CORRELATION_LAW_SELECTED / PRWC_VALUE_AND_OWNER_REUSE_FORBIDDEN / EXPECTED_DEVICE_PRWM_REQUEST_ID_SOURCE_OWNER_UNMATERIALIZED / SERVER_LOCAL_VERIFIER_TIME_AUTHORITY_REQUIRED / CURRENT_PRWA_FALLIBLE_CLOCK_NOT_DIRECTLY_COMPATIBLE_WITH_REQUEST_T / EXPECTED_LANE_TIME_SOURCE_DESIGN_UNRESOLVED / CURRENT_BOOTSTRAP_RECEIVER_ONLY / EXPECTED_REQUEST_SENDER_CHANNEL_OWNER_CAPACITY_AND_SHUTDOWN_UNRESOLVED / NB_DISPATCHER_CALLER_COMPOSITION_UNMATERIALIZED / COMPLETE_EXPECTED_REQUEST_PRODUCER_NOT_PROVEN / SOURCE_MATERIALIZATION_BLOCKED`

## 19. Why source materialization remains blocked

C03e-OB does not authorize source materialization because exact-current source still lacks a selected concrete owner/seam for at least:

- expected-device PRWM request-ID state;
- fail-closed request-carried verifier-time `T` construction;
- expected-request channel construction/sender ownership/capacity/shutdown;
- NB dispatcher construction at that producer boundary.

The SessionId law is selected, but no expected-admission-specific source seam is selected here.

## 20. Smallest next blocking prerequisite

After independent C03e-OB closure, the next separately gated documentation boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_PRODUCER_OWNER_SOURCE_SEAM_DECOMPOSITION_SELECTION`

That future gate must determine, from fresh exact-head source:

1. the exact owner that may construct the expected-request channel and retain its sender;
2. whether SessionId, PRWM request-ID and verifier-time primitives are materialized together or split into smaller source checkpoints;
3. exact source path(s) for each primitive and owner composition;
4. channel capacity source and shutdown/drop ordering;
5. exact NB dispatcher construction point;
6. whether the current `T: FnMut() -> u64` shape can be satisfied fail-closed without interface widening;
7. the smallest deterministic future source ceiling;
8. STOP if satisfying the law requires runtime activation or an unselected second architecture boundary.

That future gate remains documentation-only unless a later separately closed source-materialization checkpoint is selected.

No successor token is inferred solely from naming.

## 21. Immediate source ceiling

For C03e-OB itself:

`ZERO_RUST_SOURCE_PATHS`

Only this one documentation contract may differ from exact C03e-OA.

Any Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging change blocks closure.

## 22. Validation plan

Closure requires exact-head readback proving:

- branch rooted directly at exact closed OA head;
- exact OA merge base;
- only this documentation contract differs;
- no source/runtime/dependency/workflow mutation;
- PR remains draft/open/unmerged;
- exact-head Rust validation reaches terminal success;
- every skipped workflow is recorded as `SKIPPED`, never PASS;
- no Android PASS is claimed unless an exact-head Android run exists and succeeds.

## 23. Durable evidence plan

After exact-head validation:

1. freeze one immutable raw text audit;
2. exact-title search in canonical Drive evidence parent must return zero;
3. upload raw `text/plain` without conversion to the canonical parent;
4. verify title, MIME, parent and exact byte size;
5. raw-download/readback the artifact;
6. recompute SHA-256 and require byte/hash equality;
7. exact-title post-search must return exactly one artifact with the same Drive ID;
8. re-read branch and PR before closure metadata mutation;
9. update PR body only to CLOSED metadata;
10. keep PR draft/open/unmerged;
11. STOP.

Canonical evidence parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

## 24. Explicit non-actions

C03e-OB authorizes none of the following:

- Rust source mutation;
- expected-device request construction/send;
- production channel creation;
- sender retention or clone;
- target admission SessionId generation at runtime;
- authentication PRWM request-ID allocator materialization;
- PRWC request-ID owner reuse;
- verifier-time adapter/source materialization;
- timing-interface widening;
- NB dispatcher production caller wiring;
- broader capability-provider construction;
- requester acknowledgement retry/resend;
- scheduling grant clone/copy/reconstruction/remint/replay;
- scheduling-consumption rollback;
- requester cleanup;
- candidate/reachability continuation;
- target dial;
- listener/bootstrap/readiness/runtime activation;
- Cargo/lockfile/workflow/Android mutation;
- deployment;
- merge;
- branch deletion;
- force push/history rewrite;
- repository configuration/ruleset/permission change.

## 25. Closure rule

C03e-OB may close only if its exact final head validates and durable evidence proves the repository delta is documentation-only and the classification above remains unchanged.

C03e-OB closure does not authorize the next source mutation.

After closure:

`STOP`
