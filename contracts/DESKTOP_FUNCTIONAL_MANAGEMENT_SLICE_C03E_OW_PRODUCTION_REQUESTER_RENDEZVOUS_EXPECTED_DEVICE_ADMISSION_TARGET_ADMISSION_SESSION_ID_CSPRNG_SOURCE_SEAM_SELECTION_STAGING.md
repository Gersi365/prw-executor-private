# C03e-OW — Target Admission SessionId CSPRNG Source Seam Selection

## Status

`SELECTION — VALIDATION_PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_TARGET_ADMISSION_SESSION_ID_CSPRNG_SOURCE_SEAM_SELECTION`

Selected future source boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_TARGET_ADMISSION_SESSION_ID_CSPRNG_SOURCE_MATERIALIZATION`

## Authoritative predecessor

C03e-OV is the exact predecessor.

- predecessor head: `26f221d767e1be10cfbc69889af7c3c1534b9123`
- predecessor tree: `3ffdf29753c698b98915b4a13e471c4e2f930db7`
- predecessor endpoint-owner source blob: `104fc309a6767a2a5bc08814c8aadb52a941758b`
- predecessor PR: `#535`
- predecessor lifecycle: `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

C03e-OV materialized only the private synchronous shutdown-suppression mapper. Request construction and every construction input remained separately gated.

## Fresh successor/concurrency guard

Before assigning C03e-OW:

- exact C03e-OV branch was re-read at the exact predecessor head/tree;
- PR #535 remained draft/open/unmerged/mergeable and CLOSED;
- recent chronology remained headed by PR #535;
- `C03e-OW` searches disclosed no actual successor PR or implementation branch; references found in predecessor text stated only that OW was not assigned;
- semantic searches for a target-admission SessionId CSPRNG source disclosed historical authority/precedent, not a current competing successor;
- the exact proposed C03e-OW contract path was absent on the exact C03e-OV head;
- exact C03e-OV source and exact-current dependencies were re-read before this selection.

Only after those guards was token C03e-OW assigned.

## Fresh exact-current source findings

### Expected-device request constructor already exists

The exact C03e-OV `RemoteSessionExpectedDeviceAdmissionRequest` constructor is already materialized and requires these separately owned inputs:

1. target admission `SessionId`;
2. target expected `DeviceId`;
3. `PrwmAuthenticationPayload`;
4. `RemoteSessionExpectedDeviceAdmissionStatusOnlyCapabilityDispatcher`;
5. verifier-owned `now_unix_ms`.

Therefore the target admission `SessionId` can be selected as an independent construction prerequisite without constructing the request itself.

### Existing PRWA randomness code is algorithmic precedent only

Exact C03e-OV `crates/prw-session/src/prwa_verifier_source.rs` proves an existing production-compatible algorithm:

- `aws_lc_rs::rand::SystemRandom`;
- exactly 32 random bytes;
- exactly 64 lowercase hexadecimal ASCII characters;
- typed construction through `SessionId::new(...)`;
- one randomness acquisition;
- fail closed on randomness or typed-construction failure.

Its `PrwaVerifierSessionContext` is not selected for reuse by this expected-device admission lane. The context also owns PRWA-specific verifier times and challenge lifetime, which are outside this boundary.

### No Cargo widening required

Exact C03e-OV `crates/prw-agent/Cargo.toml` already contains direct dependency:

`aws-lc-rs = { version = "=1.18.0", default-features = false, features = ["alloc", "non-fips"] }`

Therefore future Agent-local target-admission SessionId materialization does not require Cargo or lockfile changes solely to acquire cryptographic randomness.

## Historical authority retained

C03e-OB remains authoritative for identifier custody:

- target admission SessionId is fresh server-local CSPRNG material;
- exact entropy shape is 32 random bytes / 256 bits;
- representation is 64 lowercase hexadecimal ASCII characters converted through existing typed `SessionId` construction;
- PRWA context-object reuse is forbidden;
- SessionId is not derived from requester identity, target expected DeviceId, requester scheduling SessionId, PRWC request ID, expected-device authentication request ID, time, endpoint, transport identity, credential material, registry state, candidate state, or user/product input;
- one generation attempt only;
- no hidden collision retry or replacement SessionId;
- `SessionAuthenticationService::begin_session(...)` remains the authoritative duplicate-session gate when the later admission path actually begins the session;
- generation or later duplicate failure is terminal for the already-consumed construction continuation under separately gated ConstructionFailed composition; it does not authorize grant remint/replay/rollback.

## Selected immediate future source seam

The next source checkpoint may materialize only one private Agent-local target-admission SessionId generator in:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

This exact one-file ceiling is selected because:

- the current eligible-continuation, concrete receipt, classifier, shutdown mapper and future higher producer composition boundary are already co-located there;
- `prw-agent` already directly owns `aws-lc-rs`;
- `SessionId` is available from `prw_core` and can be imported locally;
- no public API, cross-crate helper, Cargo mutation or second Rust path is needed merely to materialize this primitive.

If correct compilation or semantics require a second Rust source path, Cargo/lockfile mutation, public API widening, reuse of `PrwaVerifierSessionContext`, request construction, request-ID source, dispatcher construction, verifier-time wiring, channel/sender construction or lifecycle activation, the future source checkpoint must STOP and return to selection.

## Exact future generator law

The future private generator must:

1. allocate exactly `[0_u8; 32]` as local entropy storage;
2. use OS-backed `aws_lc_rs::rand::SystemRandom` through the existing `SecureRandom` interface;
3. perform exactly one `fill(...)` randomness acquisition;
4. fail closed if that acquisition fails;
5. encode every byte into exactly two lowercase hexadecimal characters using a fixed lowercase hex alphabet;
6. produce exactly 64 ASCII characters;
7. invoke existing `SessionId::new(...)` exactly once;
8. fail closed if typed SessionId construction fails;
9. return only the typed fresh target admission `SessionId`.

The generator must not loop, retry, regenerate after collision, persist the value, consult registry state, call `begin_session`, or perform network/request/producer work.

## Future error boundary

A narrowly private bounded error representation may distinguish only what this primitive itself can fail on:

- cryptographic randomness acquisition failure;
- typed SessionId construction failure.

It must not absorb later duplicate-session rejection, request construction failure, authentication failure, channel closure, dispatcher failure, verifier-time failure, admission rejection or endpoint lifecycle error.

No public error API is selected.

## Identity and correlation separation

The new target admission SessionId is a fresh server-local admission identifier.

It remains strictly distinct from:

- requester callback `DeviceId`, which is requester-side authenticated correlation only;
- target expected `DeviceId`, which is obtained only later from the consumed construction-eligible scheduling grant;
- requester scheduling `SessionId`, which records requester-side scheduling provenance inside that grant;
- expected-device PRWM authentication request ID, which is a separately generated nonzero single-transaction correlation value;
- any PRWC correlation ID;
- any endpoint/candidate/reachability identifier.

No value conversion or derivation among those lanes is selected.

## One-shot scheduling grant law preserved

C03e-OW does not inspect or consume the eligible scheduling grant.

The exact grant remains:

- non-`Copy`;
- non-`Clone`;
- operation-scoped;
- the only future source of target expected `DeviceId` and requester scheduling provenance;
- not reconstructible, remintable, replayable, refundable or rollback-capable.

Generating a target admission SessionId does not itself consume, inspect or authorize opening the grant.

## Construction-failure ordering remains deferred

The future SessionId generator alone does not decide when it is called relative to grant `into_parts()`, request-ID generation, dispatcher construction or verifier-time acquisition.

That ordering remains a separately gated construction-composition decision.

In particular, C03e-OW does not authorize:

- opening the grant merely to generate SessionId;
- request construction;
- ConstructionFailed receipt composition;
- retrying SessionId generation after any downstream failure.

## Expected-device PRWM request-ID remains separate

C03e-OW does not materialize or select the source implementation for the expected-device PRWM authentication request ID.

Existing C03e-OB law remains:

- it is independent from target admission SessionId;
- it is nonzero;
- it correlates exactly one expected-device Challenge -> Proof transaction;
- inbound proof request ID must match the outbound challenge request ID;
- PRWC/requester/terminal-ack correlation values or owners must not be reused.

That source remains a later separate gate.

## Dispatcher remains separate

C03e-OW does not construct or transfer the NB-selected status-only dispatcher.

The eventual request must own the production concrete status-only dispatcher by value, but its exact construction/transfer point remains separately gated.

## Verifier time remains separate

C03e-OW does not select or materialize expected-device request verifier-time production.

The request-carried time input remains explicit and must not be silently sourced by calling `SystemTime::now()` inside `verify()`.

Historical PRWA time source is not automatically shape-compatible with the expected-device request lane and is not imported by this selection.

## Producer/channel law preserved

No channel or producer mutation is selected here.

Eventual handoff remains exactly:

- one bounded Tokio MPSC channel;
- capacity exactly 1;
- receiver created once and moved once;
- exactly one higher-owned production sender;
- no sender clone;
- enqueue only through `sender.send(request).await`;
- full channel means async backpressure;
- no `try_send`;
- no `blocking_send`;
- no callback `block_on`;
- no hidden/detached producer task;
- no alternate/unbounded/retry queue;
- no second producer future.

Channel close remains a terminal handoff outcome, not a supervisor-shutdown authority.

## Receipt law preserved

C03e-OW does not construct any new handoff receipt.

Existing bounded dispositions remain:

- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

Only `SuppressedOnShutdown` composition is materially present today.

`Enqueued` continues to mean queue acceptance only.

SessionId generation failure will belong to later separately gated `ConstructionFailed` composition; C03e-OW does not wire that outcome.

## Shutdown law preserved

Explicit supervisor shutdown remains the sole endpoint-supervisor shutdown authority.

C03e-OW changes no shutdown detection, suppression mapping, channel close, endpoint close, worker drain or lifecycle behavior.

## Immediate future hard source ceiling

Exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Allowed future delta only:

- private `SessionId` import as required;
- private `SecureRandom` / `SystemRandom` imports as required;
- private constants for 32 random bytes / 64 lowercase hex bytes / lowercase hex alphabet if needed;
- private bounded generator error type if needed;
- one private pure/server-local target admission SessionId generation helper;
- narrowly focused same-file tests if practical;
- minimum dormant lint acknowledgement if required and reasoned.

Forbidden in that immediate materialization:

- reuse or modification of `PrwaVerifierSessionContext`;
- `prw-session` source modification;
- Cargo/lockfile mutation;
- eligible scheduling-grant inspection or `into_parts()`;
- request-ID generation;
- dispatcher construction/transfer;
- verifier-time acquisition/wiring;
- `RemoteSessionExpectedDeviceAdmissionRequest::new(...)` invocation;
- ConstructionFailed/ChannelClosed/Enqueued receipt composition;
- production channel/sender creation;
- producer closure specialization/invocation;
- `sender.send(request).await`;
- lifecycle method mutation;
- caller migration;
- runtime/listener/bootstrap/readiness/executable activation;
- deployment.

## Remaining separately gated dependencies after this selection

Still deferred:

- target admission SessionId CSPRNG source materialization itself;
- independent nonzero expected-device PRWM authentication request-ID source selection/materialization;
- NB status-only dispatcher production construction/transfer;
- verifier-time source/interface compatibility;
- exact construction-order/custody composition around the one-shot grant;
- actual `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- exact ConstructionFailed terminal grant disposal/receipt composition;
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

No ordering among independent later input gates is invented beyond the authority constraints above.

## Explicit non-actions

C03e-OW is documentation-only.

No Rust/source/runtime mutation.
No target admission SessionId generation at runtime.
No expected-device PRWM authentication request-ID generation.
No grant inspection, extraction, disposal, clone, copy, reconstruction, remint, replay, rollback or refund.
No request construction/send.
No dispatcher production wiring.
No verifier-time wiring.
No channel/sender construction or clone.
No producer specialization/invocation.
No `try_send`.
No `blocking_send`.
No callback `block_on`.
No hidden/detached spawn.
No retry/alternate/unbounded queue.
No second producer future.
No lifecycle method mutation.
No lower cooperative-driver mutation.
No visibility widening.
No caller migration.
No runtime activation.
No Cargo/lockfile/workflow/Android-source mutation.
No deployment.
No merge.
No PR ready conversion or PR closure.
No branch deletion.
No force push.
No reset/rebase/squash/history rewrite.
No repository configuration/ruleset/permission mutation.

Keep the future C03e-OW PR draft/open/unmerged.

After exact-final-head validation and immutable evidence closure: STOP.
No successor token is assigned by this selection.