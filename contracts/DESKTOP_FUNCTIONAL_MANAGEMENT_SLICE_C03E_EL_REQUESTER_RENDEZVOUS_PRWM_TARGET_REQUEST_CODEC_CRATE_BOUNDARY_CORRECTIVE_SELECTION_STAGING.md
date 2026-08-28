# Phase 152 C03e-EL — Requester/Rendezvous PRWM Target-Request Codec Crate-Boundary Corrective Selection

Status: `STAGING_SELECTION`

Target gate: `C03E_EL_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_CODEC_CRATE_BOUNDARY_CORRECTIVE_SELECTED`

## 1. Purpose

C03e-EL selects the narrow source-level crate boundary required to materialize the C03e-EK-selected PRWZ v1.0 target-request codec without creating a dependency cycle, widening Agent authority types, or mixing pure wire decoding with Agent requester identity custody.

C03e-EK remains authoritative for all wire bytes and semantics. C03e-EL changes none of the selected PRWZ header, version, operation, target bound, outer PRWM kind, or correlation rules.

The correction is limited to the typed handoff produced by the pure bridge codec.

## 2. Exact predecessor

C03e-EL is rooted exactly at durably closed C03e-EK:

- predecessor branch: `phase-152-c03e-ek-candidate-publication-requester-rendezvous-prwm-target-request-wire-selection-staging`
- predecessor head: `d5bf975e5fc1f7f4e7af9976c10cea84af347719`
- predecessor tree: `2e7de59a56db24d5d7587831b4e8f512096ffd60`
- predecessor PR: `#261`, draft/open/unmerged, `Status: CLOSED`
- predecessor closure classification: `CLOSED_PRWM_TARGET_REQUEST_WIRE_SELECTION`
- predecessor target gate: `C03E_EK_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_WIRE_SELECTED`
- post-EK rolling evidence: `1107632` bytes
- post-EK rolling SHA-256: `e344759b4d86c8360767ee6dd103faa789a830e5187525b79b0b915f18ef108c`

No earlier checkpoint is reopened.

## 3. Concrete preflight contradiction discovered after EK

### 3.1 Agent target-intent type is effective crate-private

`crates/prw-agent/src/lib.rs`

- exact C03e-EK blob: `58b37553c2f089e0f5f4a911be2f40893e18173c`
- the containing module is declared:

```rust
pub(crate) mod candidate_publication_requester_rendezvous_start_intent;
```

`RequesterRendezvousTargetIntent` therefore cannot be named by `prw-remote-bridge` as a public or private dependency type.

This visibility is intentional and must not be widened merely for a codec.

### 3.2 Existing dependency direction is Agent -> bridge

`crates/prw-agent/Cargo.toml`

- exact C03e-EK blob: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- already contains `prw-remote-bridge = { path = "../prw-remote-bridge" }`.

`crates/prw-remote-bridge/Cargo.toml`

- exact C03e-EK blob: `5fd48263be415aac28dee1c71a4031a4a02ad36c`
- depends on `prw-core`, `prw-control-plane`, `prw-remote-transport`, and other lower/domain crates;
- does not and must not depend on `prw-agent`.

Adding bridge -> Agent would create a direct crate dependency cycle and invert the existing layering.

### 3.3 Bridge is the natural owner of PRWM codec mechanics

`prw-remote-bridge` already owns:

- `session_auth_wire.rs` for PRWS over PRWM;
- `capability_request_wire.rs` for bounded PRWM stream frame I/O;
- generic bridge payload encode/decode;
- `remote_server_transport_runtime.rs` around the lower transport.

Its crate root `crates/prw-remote-bridge/src/root.rs`, exact blob `293c94d56533ccac9d0b5e0301366623116f1788`, exposes reviewed bridge wire modules without granting Agent authorization authority.

Therefore moving pure PRWZ codec mechanics into Agent solely to reach the Agent target-intent type is not selected.

## 4. Corrective boundary selected by C03e-EL

C03e-EL selects the pure PRWZ codec to live in `prw-remote-bridge` and to return a bridge-owned typed wire request whose semantic shape is:

```text
RequesterRendezvousTargetWireRequest {
    request_id: u64,
    target_device_id: DeviceId,
}
```

The exact Rust type name may vary only for local readability/lint compliance. The semantic fields may not.

The bridge-owned wire request MUST contain:

- exactly one non-zero outer PRWM request correlation value copied unchanged from `ControlFrame::request_id()`;
- exactly one typed logical target `DeviceId` decoded from PRWZ.

It MUST NOT contain:

- requester identity;
- authenticated session;
- `TransportIdentity`;
- `SessionId`;
- registry principal;
- policy decision;
- provider state;
- endpoint/candidate data;
- default/fallback target.

## 5. Source-materialization placement selected

A future source-materialization checkpoint may add:

```text
crates/prw-remote-bridge/src/requester_rendezvous_target_request_wire.rs
```

plus the minimum crate-root registration required by the existing bridge module pattern.

The codec module may be publicly reachable from `prw-remote-bridge` because it represents bounded wire data, not requester authorization authority. Public fields are not required; preferred shape is private fields with narrow accessors/consuming extraction.

This public bridge wire surface MUST NOT expose the Agent crate-private target-intent type or any requester policy/provider authority.

## 6. Corrected pure decode contract

A future pure decoder should have semantics equivalent to:

```rust
pub fn decode_requester_rendezvous_target_request_frame(
    frame: &ControlFrame,
) -> Result<RequesterRendezvousTargetWireRequest, RequesterRendezvousTargetWireError>
```

It must implement the exact C03e-EK wire selection:

1. require outer `ControlMessageKind::Request`;
2. require inner magic `PRWZ`;
3. require major `1`, minor `0`;
4. require operation `1`;
5. require reserved flags `0`;
6. require target length in `1..=1024`;
7. require exact UTF-8 target bytes and no trailing bytes;
8. construct target using `DeviceId::new(...)`;
9. preserve exact outer non-zero `request_id` from the existing validated frame;
10. return only the bridge-owned typed wire request.

No registry/policy/provider/Agent call occurs during decode.

## 7. Pure encode contract

For symmetry, fixture/client use, and exact round-trip validation, the future pure module may also expose an encoder semantically equivalent to:

```rust
pub fn encode_requester_rendezvous_target_request_frame(
    request_id: u64,
    target_device_id: &DeviceId,
) -> Result<ControlFrame, RequesterRendezvousTargetWireError>
```

It must:

- use outer PRWM `ControlMessageKind::Request`;
- emit exactly the EK-selected PRWZ v1.0 bytes;
- enforce the same 1024-byte wire bound;
- preserve the supplied non-zero correlation through existing `ControlFrame::new(...)` validation;
- perform no requester authentication or authorization.

The target is an input value only. The encoder does not assert that the target exists or is authorized.

## 8. Agent-side adaptation remains a later separate checkpoint

Because `RequesterRendezvousTargetIntent` is Agent-owned and effective crate-private, a later Agent checkpoint may consume the bridge wire request and construct the existing target intent locally, semantically:

```text
wire_request.into_target_device_id()
    -> RequesterRendezvousTargetIntent::new(target_device_id)
    -> existing C03e-EJ caller seam
```

That future Agent adaptation is not part of the pure codec materialization selected here.

This preserves the closed identity split:

- bridge wire layer owns target bytes/correlation only;
- Agent authenticated-session layer owns requester identity;
- existing target-intent type remains Agent-internal;
- C03e-EJ remains the canonical post-auth typed caller seam.

## 9. Relationship to C03e-EK

C03e-EL does not change C03e-EK wire semantics.

The following remain fixed exactly as selected by EK:

- outer PRWM `Request`;
- inner `PRWZ` magic;
- version `1.0`;
- 12-byte inner header;
- operation `1`;
- zero flags;
- u16 length-prefixed UTF-8 target;
- target ceiling 1024 bytes;
- outer request ID correlation-only semantics;
- requester identity absent from wire;
- generic `BridgeCommand` exclusion;
- response semantics separately gated.

C03e-EL corrects only which crate may own the first decoded typed handoff.

## 10. Why Agent visibility widening is rejected

C03e-EL rejects making `candidate_publication_requester_rendezvous_start_intent` public or exporting `RequesterRendezvousTargetIntent` outside `prw-agent` merely to satisfy the bridge codec.

That would expose an Agent authorization-adjacent intent type across a public crate boundary without architectural need.

The bridge already has a sufficient lower-level domain type: `prw_core::DeviceId`.

## 11. Why a bridge -> Agent dependency is rejected

A bridge dependency on Agent would:

- create a direct dependency cycle because Agent already depends on bridge;
- invert the intended lower-level bridge / higher-level Agent relationship;
- couple pure wire parsing to Agent runtime authority state;
- force unrelated codec compilation through Agent lifecycle dependencies.

No manifest mutation is selected to create such a relationship.

## 12. Requester identity invariant

Requester identity remains only the exact authenticated application session retained by `AuthenticatedRemoteSessionRuntimeOwner`.

Neither the bridge-owned wire request nor its target `DeviceId` may become requester identity.

Outer request ID, transport identity, session correlation, stream, endpoint, candidate, publisher identity, repeated-admission expected device identity, role, policy/provider state and defaults remain prohibited substitutes.

## 13. Target identity invariant

The bridge-owned `target_device_id` is exactly the typed logical `DeviceId` decoded from the PRWZ target field.

It is nomination only and MUST NOT be inferred, rewritten, defaulted, normalized from another identity source, or treated as authorized provider state.

## 14. Error boundary

The future bridge module may define one bounded wire error enum covering only structural/typed/frame construction failures such as:

- wrong outer kind;
- invalid PRWZ payload;
- invalid target bound/UTF-8/domain value;
- existing PRWM frame construction failure.

It must not contain requester authorization denials or provider mutation results because those have not occurred.

## 15. Response and I/O remain separately gated

C03e-EL selects no:

- stream acceptance;
- `MeshControlStream` receive/send transaction;
- requester-specific response frame;
- policy/provider error mapping;
- retry, timeout, close, replay or idempotency semantics;
- Agent EJ invocation from I/O.

The immediate source-materialization successor is pure encode/decode only.

## 16. C03e-DV remains deliberately uncalled

No C03e-EL selected type/function borrows or accepts:

- `SharedCurrentCapabilityAuthority<P>`;
- requester-aware policy source;
- requester/rendezvous runtime owner.

No registry validation, requester-aware policy evaluation or provider registration is selected.

## 17. Expected immediate source-materialization scope

After durable EL closure, the immediate successor may modify only what is necessary for the pure bridge codec, expected to be:

1. one new `prw-remote-bridge` PRWZ wire source module;
2. `crates/prw-remote-bridge/src/root.rs` for module registration/export;
3. the source-materialization contract;
4. no Agent source change;
5. no manifest or lockfile change.

The materialization should include strict round-trip and malformed-frame tests entirely within the bridge crate.

## 18. Exact source/dependency anchors

At closed C03e-EK:

- Agent lib visibility: `58b37553c2f089e0f5f4a911be2f40893e18173c`
- Agent Cargo: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- bridge Cargo: `5fd48263be415aac28dee1c71a4031a4a02ad36c`
- bridge root: `293c94d56533ccac9d0b5e0301366623116f1788`
- remote transport: `47b007f3f4151b8971a57997db22b223f8d70ce6`
- generic bridge: `7b1c5c62339983da6ae2556f73510d7582ec0c5b`
- session-auth PRWM codec: `492d3e938fcbc75907b345750928717c957204e8`
- Agent target-intent source: `5f616f20699d1c7069f5aa8973200a0359c19cde`
- EJ caller seam root: `f669ca79baecaef8bc90df1cf3bb53799db67623`
- root Cargo.lock: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native Cargo.lock: `cce9ca06190a196661ab38d54a747893e26af95f`.

## 19. Explicit exclusions

C03e-EL does not select or materialize:

- PRWZ codec source in this checkpoint;
- bridge -> Agent dependency;
- Agent target-intent visibility widening;
- Agent-side wire-request adaptation;
- stream I/O transaction;
- response/error protocol;
- EJ execution from I/O;
- C03e-DV invocation;
- requester-aware policy/provider execution;
- generic `BridgeCommand` change;
- worker/admission/public process-input widening;
- bootstrap/main/network activation;
- deployment;
- restart/recovery;
- merge.

## 20. Closure criteria

C03e-EL may close only if:

1. exact C03e-EK merge base is preserved;
2. ahead only, zero behind;
3. only this docs contract changed;
4. all cited source/manifests/locks remain byte-stable;
5. canonical exact-head Rust validation is terminal FULL PASS where triggered;
6. Android is classified exactly as triggered/not-triggered and SKIPPED is never reported as PASS;
7. no exact-final-head workflow is pending/failing;
8. immutable Drive audit raw-readback is byte-exact;
9. rolling predecessor is exact post-EK before append;
10. exact EK prefix is preserved after append;
11. EL closure/classification/target-gate markers each occur exactly once;
12. PR remains draft/open/unmerged.

## 21. Successor boundary

After durable C03e-EL closure, the next checkpoint may source-materialize only the pure bridge-owned PRWZ target-request codec and its bridge root registration/tests.

It does not gain permission to modify Agent source, invoke EJ/DV, add stream I/O, add responses, activate networking, deploy or merge.

## 22. Target gate

C03e-EL targets exactly:

`C03E_EL_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_CODEC_CRATE_BOUNDARY_CORRECTIVE_SELECTED`

Passing this gate means only that the compile-safe crate boundary for the EK-selected wire codec is fixed. It does not mean codec source exists or that requester/rendezvous execution is active.
