# Phase 152 C03e-EK — Candidate-Publication Requester/Rendezvous PRWM Target-Request Wire Selection

Status: `STAGING_SELECTION`

Target gate: `C03E_EK_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_WIRE_SELECTED`

## 1. Purpose

C03e-EK selects, but does not source-materialize, the first bounded requester-specific remote-session wire representation that may carry one explicit caller-nominated rendezvous target after logical-session authentication.

The selected lane is a dedicated inner subprotocol inside the existing PRWM `ControlMessageKind::Request` envelope. It is not a new generic `BridgeCommand`, not PRWC control-plane traffic, and not requester/rendezvous authorization or provider execution.

Successful future decode may prove only that one bounded, structurally valid logical target `DeviceId` was nominated by the authenticated remote-session caller. It does not prove that the requester or target currently exists, that workspace/user scope matches, that requester and target differ, that requester-aware policy allows the operation, or that provider registration succeeded.

## 2. Exact predecessor

C03e-EK is rooted exactly at durably closed C03e-EJ:

- predecessor branch: `phase-152-c03e-ej-candidate-publication-requester-rendezvous-post-auth-target-intent-caller-ingress-source-materialization-staging`
- predecessor head: `da29135178bb46120bc645769fe7cf4ec7f1925f`
- predecessor tree: `1ba9752824ce6f8f4346883f105e4b940b4877ad`
- predecessor PR: `#260`, draft/open/unmerged, `Status: CLOSED`
- predecessor closure classification: `SOURCE_MATERIALIZATION_FULL_PASS`
- predecessor target gate: `C03E_EJ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_AUTH_TARGET_INTENT_CALLER_INGRESS_SOURCE_MATERIALIZED`
- post-EJ rolling evidence: `1105173` bytes
- post-EJ rolling SHA-256: `db17ff711efba2d31d3bb9d47ba1631aa2df9fca9647cc39d69f8bbc499349e9`

No earlier checkpoint is reopened.

## 3. Fresh exact-head topology

### 3.1 Authenticated remote-session transport is PRWM over QUIC

`crates/prw-remote-transport/src/lib.rs`

- exact C03e-EJ blob: `47b007f3f4151b8971a57997db22b223f8d70ce6`
- ALPN: `prw-mesh/1`
- outer control magic: `PRWM`
- protocol version: `1.0`
- fixed outer header: 24 bytes
- maximum outer payload: 65,536 bytes
- `ControlMessageKind::Request` is the existing bounded request envelope
- outer `request_id` must be non-zero and is correlation only.

This is the transport lane retained by `AuthenticatedRemoteSessionRuntimeOwner`. C03e-EK therefore selects PRWM, not Phase-129 PRWC TLS control-plane framing.

### 3.2 A post-authenticated peer can accept another bounded control stream

`crates/prw-remote-bridge/src/remote_server_transport_runtime.rs`

- exact C03e-EJ blob: `14b774d11c1c123f001580be252eb036329d6d2e`
- `AuthenticatedRemotePeerConnection::accept_control_stream()` yields one existing bounded `MeshControlStream` after lower transport establishment.

The authenticated-session runtime already retains that peer and currently accepts control streams for generic capability transactions. A future requester-specific transaction can therefore be selected independently without exposing the raw Quinn connection.

C03e-EK itself does not accept a stream or change stream scheduling.

### 3.3 Existing PRWM subprotocol precedent

`crates/prw-remote-bridge/src/session_auth_wire.rs`

- exact C03e-EJ blob: `492d3e938fcbc75907b345750928717c957204e8`
- inner magic `PRWS`
- version `1.0`
- 12-byte inner header: magic + major + minor + message kind + reserved flags
- bounded typed fields inside one reserved PRWM message kind
- exact request correlation preserved separately by outer PRWM `request_id`.

This establishes the repository precedent for a distinct, versioned inner protocol carried by PRWM without reinterpreting transport correlation as identity.

### 3.4 Existing generic capability request lane must remain separate

`crates/prw-remote-bridge/src/lib.rs`

- exact C03e-EJ blob: `7b1c5c62339983da6ae2556f73510d7582ec0c5b`
- generic capability payload magic is `PRWC`
- generic inner version is `1.0`
- generic inner header is 12 bytes
- generic operation codes 1..18 decode only into existing `BridgeCommand` variants
- `CapabilityBridge::authorize(...)` requires outer PRWM `Request`, decodes `BridgeCommand`, derives `required_capability()`, and evaluates the principal-agnostic `PolicyEvaluator`.

Requester/rendezvous authorization was deliberately separated from that principal-agnostic evaluator by the closed C03e-DW/DX/DV chain. C03e-EK therefore MUST NOT add a requester/rendezvous operation code to `BridgeCommand` or reuse generic `required_capability()` authorization.

### 3.5 Generic PRWM I/O adapter is transport-only precedent

`crates/prw-remote-bridge/src/capability_request_wire.rs`

- exact C03e-EJ blob: `4a24af6316e2c17c0980c12e787791848174be9b`
- receives/sends exactly one bounded PRWM frame on an existing `MeshControlStream`
- does not itself establish application semantics or authorization.

A future requester-specific wire adapter may use the same bounded stream primitive, but must decode the dedicated requester/rendezvous inner protocol before handing a typed target to Agent code.

### 3.6 PRWC/PRWA/PRWP are separate control-plane precedents only

Exact C03e-EJ anchors:

- `crates/prw-control-transport/src/lib.rs`: `88f70e187e865119ff6401d05019cdac7b5392ad` — outer `PRWC`
- `crates/prw-remote-bridge/src/control_session_auth_wire.rs`: `77c6f401ef73c0b2a97645ae8bc83524c769a905` — inner `PRWA`
- `crates/prw-remote-bridge/src/candidate_publication_wire.rs`: `299042938b38b65b78f737926f74b8567e5046fb` — inner `PRWP`
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs`: `20ff7d2bc5f32596a3c0696aa387e6735f8f2031` — wraps PRWP in PRWC `Command`.

Those prove bounded versioned codec patterns, but they are not the authenticated requester remote-session carrier.

## 4. Selected requester/rendezvous target request subprotocol

C03e-EK selects a dedicated PRWM `Request` inner protocol with the following fixed initial profile:

```text
outer: existing PRWM ControlFrame
  kind: Request
  request_id: non-zero correlation only
  payload:
    magic:       4 bytes = "PRWZ"
    major:       u16 big-endian = 1
    minor:       u16 big-endian = 0
    operation:   u16 big-endian = 1  // requester rendezvous start target nomination
    flags:       u16 big-endian = 0
    target_len:  u16 big-endian
    target_utf8: target_len bytes
```

Initial constants selected semantically:

- requester/rendezvous inner magic: `PRWZ`
- major version: `1`
- minor version: `0`
- fixed inner header before target body: `12` bytes
- operation `1`: requester rendezvous start target nomination
- reserved flags: exactly zero
- target identifier length ceiling: `1024` UTF-8 bytes.

The exact Rust constant/type names may differ only for local clarity or lint compliance.

## 5. Namespace rationale

The new inner magic must distinguish requester/rendezvous target requests from existing application protocols before any generic `BridgeCommand` decode occurs.

`PRWR` is explicitly unavailable because it is already the relay routing magic in `crates/prw-relay-service/src/lib.rs` at exact C03e-EJ blob `b8d6602ac12dc392fe498c8a5f18e580e4555b8e`.

Repository search at the exact C03e-EJ lineage found no existing `PRWZ` protocol magic. C03e-EK selects `PRWZ` as the rendezvous-specific namespace to prevent collision with `PRWS`, `PRWA`, `PRWP`, `PRWC`, `PRWM`, and existing relay `PRWR`.

The outer PRWM magic remains unchanged.

## 6. Target encoding and bounds

The target body contains exactly one UTF-8 logical `DeviceId`.

A future decoder must:

1. require `target_len` in `1..=1024`;
2. require exactly `target_len` remaining bytes for the target and reject trailing bytes;
3. require valid UTF-8;
4. construct the typed value using `DeviceId::new(...)` and fail closed if domain construction fails;
5. create exactly one `RequesterRendezvousTargetIntent` from that typed target.

The 1024-byte ceiling reuses the existing enrolled logical-session identifier ceiling `MAX_SESSION_AUTH_IDENTIFIER_BYTES = 1024` from exact C03e-EJ `crates/prw-control-plane/src/session_auth.rs` blob `1dbd06d8d9741844e4d8bbb235d27431921a1650`.

C03e-EK does not broaden `DeviceId` domain semantics globally. The bound is wire-specific admission.

## 7. Outer request correlation invariant

Outer PRWM `request_id` is only request/response correlation.

It MUST NOT be used as:

- requester identity;
- target identity;
- `SessionId`;
- provider key;
- policy key;
- candidate-publication identity;
- replay authorization proof.

A future decoder/transaction may preserve the exact non-zero request ID beside the typed target for a separately selected response transaction, but the target intent itself must contain only the target `DeviceId`.

## 8. Requester identity invariant

Requester identity is not encoded in PRWZ.

The logical requester remains only the exact authenticated application session retained by `AuthenticatedRemoteSessionRuntimeOwner`.

A future Agent transaction may combine:

```text
retained AuthenticatedRemoteSessionRuntimeOwner
+ decoded RequesterRendezvousTargetIntent
-> existing C03e-EJ caller seam
```

Requester identity MUST NOT be sourced or replaced from:

- PRWZ target bytes;
- outer request ID;
- `TransportIdentity`;
- `SessionId`;
- control-stream identity;
- endpoint/IP address;
- candidate data;
- candidate-publication publisher identity;
- repeated-admission `expected_device_id` as a second assertion;
- role, registry enumeration, policy output, provider state, cache or default configuration.

## 9. Target identity invariant

The logical target is exactly the `DeviceId` decoded from the one PRWZ target field.

It MUST NOT be inferred, replaced, normalized, defaulted or cross-filled from requester/session/transport/request/endpoints/candidates/publication/registry/policy/provider/global/environment state.

Successful decode is target nomination only.

## 10. Selected decode result

A future pure decoder selected by this checkpoint should return a typed shape semantically equivalent to:

```text
RequesterRendezvousTargetRequest {
    request_id: u64,
    target_intent: RequesterRendezvousTargetIntent,
}
```

The exact wrapper spelling is not fixed. The semantic split is fixed:

- `request_id` is correlation only;
- `target_intent` contains only the target logical `DeviceId`;
- requester identity is absent and comes later from authenticated runtime custody.

If a wrapper is unnecessary for source visibility, an equivalent tuple may be used, provided these semantics remain explicit and no requester identity is introduced.

## 11. Failure behavior selected for decode

The future codec must fail closed on at least:

- outer kind not PRWM `Request`;
- inner payload shorter than the 12-byte PRWZ header;
- wrong magic;
- unsupported major/minor version;
- unknown operation;
- non-zero reserved flags;
- zero or >1024 target length;
- truncated target bytes;
- invalid UTF-8;
- `DeviceId::new(...)` rejection;
- trailing bytes.

The exact Rust error enum spelling remains a source-materialization detail. Errors must be bounded and must not expose an authorization decision that has not occurred.

## 12. Response semantics remain separately gated

C03e-EK selects only the target-request ingress representation and decode boundary.

It does NOT select:

- success response payload;
- rejection/error response payload;
- response magic or operation tags;
- mapping of requester policy/provider errors to remote responses;
- peer-close behavior;
- retry semantics;
- timeout semantics;
- duplicate request-ID lifecycle;
- idempotency/replay semantics.

Those require later explicit selection after the request transaction and authority composition boundaries are audited.

## 13. Stream transaction remains separately gated

C03e-EK does not source-materialize stream acceptance or I/O.

A later checkpoint may select/materialize one requester-specific transaction that:

1. operates only after `AuthenticatedRemoteSessionRuntimeOwner` exists;
2. accepts one control stream from that retained peer;
3. receives exactly one bounded PRWM frame;
4. decodes only PRWZ request semantics;
5. obtains exactly one typed `RequesterRendezvousTargetIntent`;
6. passes that target to the existing C03e-EJ caller seam.

Even that future transaction does not gain permission from C03e-EK to invoke C03e-DV.

## 14. C03e-DV remains deliberately uncalled

C03e-EK does not call or change:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

The selected wire decoder does not borrow:

- `SharedCurrentCapabilityAuthority<P>`;
- `BoundedRequesterRendezvousStartPolicySource`;
- `CandidatePublicationRequesterRendezvousRuntimeOwner`.

It performs no registry validation, requester-aware policy evaluation or provider registration.

A later explicit caller-composition checkpoint is required before any connection from decoded PRWZ target intent to C03e-DV execution.

## 15. Generic capability lane remains byte-stable

C03e-EK does not modify or extend:

- `BridgeCommand`;
- `BridgeCommand::operation_code()`;
- `BridgeCommand::required_capability()`;
- generic bridge inner magic `PRWC`;
- `CapabilityBridge::authorize(...)`;
- authorized capability dispatcher;
- generic capability request loop.

A PRWZ payload must be identified and decoded by the requester-specific lane before generic bridge decode. It must never obtain requester/rendezvous authority by failing into or being reinterpreted as a generic command.

## 16. Expected future source-materialization scope

Only after C03e-EK closes durably, a successor may source-materialize the selected pure codec.

Expected narrow source scope is:

- one new requester/rendezvous PRWZ wire module in `prw-remote-bridge`, or an equivalently private bounded module;
- one module registration/export only as required by existing crate boundaries;
- typed request encode/decode helpers over existing PRWM `ControlFrame`;
- strict bounds/version/kind/flags/trailing-byte validation;
- side-effect-free tests.

No manifest/lock change is expected because `prw-remote-bridge` already depends on the required PRW core/control-plane/remote-transport types. A later implementation must verify this before mutation.

The source-materialization checkpoint must not simultaneously add Agent stream acceptance, EJ invocation, C03e-DV invocation or response semantics.

## 17. Dependency/source anchors to preserve

At exact closed C03e-EJ:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`
- remote transport: `47b007f3f4151b8971a57997db22b223f8d70ce6`
- remote-server transport runtime: `14b774d11c1c123f001580be252eb036329d6d2e`
- session-auth PRWM codec: `492d3e938fcbc75907b345750928717c957204e8`
- generic capability bridge: `7b1c5c62339983da6ae2556f73510d7582ec0c5b`
- generic capability PRWM adapter: `4a24af6316e2c17c0980c12e787791848174be9b`
- EJ Agent caller seam root: `f669ca79baecaef8bc90df1cf3bb53799db67623`
- authenticated-session runtime: `4bf8c5589a5523e4bd5ea97e960b1f7f921d5e7d`
- target-intent source: `5f616f20699d1c7069f5aa8973200a0359c19cde`.

## 18. Explicit exclusions

C03e-EK does not select or materialize:

- source implementation of PRWZ in this checkpoint;
- Agent stream acceptance or requester-specific I/O transaction;
- response/error wire semantics;
- C03e-EJ invocation from I/O;
- C03e-DV invocation;
- requester-aware registry/policy/provider execution;
- requester-policy population/default/refresh/persistence;
- provider construction/capacity/persistence;
- generic `BridgeCommand` changes;
- worker/admission/channel/signature widening;
- public process-input/factory widening;
- target inference/defaulting;
- PRWC requester routing;
- candidate-publication authority inversion;
- STUN/ICE/TURN behavior;
- bootstrap/main assembly;
- listener/readiness/network activation;
- deployment;
- restart/recovery;
- merge.

## 19. Closure criteria

C03e-EK may close only on one exact final head proving:

1. exact C03e-EJ merge base;
2. ahead only, zero behind;
3. changed path set contains only this contract;
4. zero Rust/Kotlin/Gradle/manifest/lock mutation;
5. all topology/source anchors above remain byte-stable;
6. dependency anchors remain byte-stable;
7. canonical exact-head Rust validation has no pending/failing result and FULL PASS where triggered;
8. Android is classified exactly as triggered/not-triggered and SKIPPED is never reported as PASS;
9. immutable Drive audit raw-readback byte-exact;
10. rolling predecessor is exact post-EJ before append;
11. exact EJ prefix is preserved after append;
12. EK closure/classification/target-gate markers each occur exactly once;
13. PR remains draft/open/unmerged.

## 20. Successor boundary

If C03e-EK closes durably, the next checkpoint may source-materialize only the selected pure PRWZ requester/rendezvous target-request codec.

It does not gain permission to simultaneously implement Agent stream I/O, response semantics, EJ transaction execution, C03e-DV execution, networking activation, deployment or merge.

## 21. Target gate

C03e-EK targets exactly:

`C03E_EK_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_WIRE_SELECTED`

Passing this gate means only that the bounded requester-specific PRWM target-request representation is selected. It does not mean the codec exists, a remote client can execute it, requester authorization occurs, provider state mutates, or production networking is active.
