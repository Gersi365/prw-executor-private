# Phase 152 C03e-EM — Requester/Rendezvous PRWM Target-Request Codec Source Materialization

Status: `STAGING_SOURCE_MATERIALIZATION`

Target gate: `C03E_EM_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_CODEC_SOURCE_MATERIALIZED`

## 1. Purpose

C03e-EM source-materializes only the pure bridge-owned PRWZ v1.0 requester/rendezvous target-request codec selected by C03e-EK and corrected at the crate boundary by C03e-EL.

This checkpoint creates no requester/rendezvous authority and performs no stream I/O. It only encodes and decodes one already-bounded PRWM `Request` frame carrying one caller-nominated logical target `DeviceId` plus the existing outer request correlation value.

Successful decode proves only structural validity and typed target construction. It does not prove requester authentication, requester or target currentness, workspace/user scope, requester-target inequality, policy approval, provider registration, reachability, or network readiness.

## 2. Exact predecessor

C03e-EM is rooted exactly at durably closed C03e-EL:

- predecessor branch: `phase-152-c03e-el-requester-rendezvous-prwm-target-request-codec-crate-boundary-corrective-selection-staging`
- predecessor head: `7fbe8ba28f4c78b4c288fdbfbc417a9a228295f3`
- predecessor tree: `09e54d250681236503e296f435c04310eeeeec2f`
- predecessor PR: `#262`, draft/open/unmerged, `Status: CLOSED`
- predecessor classification: `CLOSED_CORRECTIVE_SELECTION`
- predecessor target gate: `C03E_EL_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_CODEC_CRATE_BOUNDARY_CORRECTIVE_SELECTED`
- predecessor rolling size: `1110886` bytes
- predecessor rolling SHA-256: `e382c99cc296f6fd10596df71dfeb5a7fa17a9b8de69248915dba31eb71445cd`

No earlier checkpoint is reopened.

## 3. Exact materialization scope

Expected changed paths are exactly:

1. this C03e-EM contract;
2. `crates/prw-remote-bridge/src/requester_rendezvous_target_request_wire.rs`;
3. `crates/prw-remote-bridge/src/root.rs` only for one module registration line.

No Agent source, manifest, lockfile, generic bridge implementation, transport runtime, worker/admission, bootstrap or main source may change.

## 4. Bridge-owned wire request type

The new pure bridge module owns one typed decoded request semantically equivalent to:

```text
RequesterRendezvousTargetWireRequest {
    request_id: u64,
    target_device_id: DeviceId,
}
```

The type must preserve:

- the exact existing non-zero outer PRWM `request_id` as correlation only;
- the exact typed logical target `DeviceId` decoded from the PRWZ body.

The type must not contain requester identity, authenticated session, `TransportIdentity`, `SessionId`, registry principal, policy decision, provider state, endpoint/candidate data, publisher identity, or any default/fallback target.

Narrow accessors and ownership transfer of the target are allowed. Possession of the value is not authorization.

## 5. Fixed PRWZ v1.0 request representation

The source must implement exactly the closed EK wire profile:

```text
outer: existing PRWM ControlFrame
  kind: Request
  request_id: non-zero correlation only
  payload:
    magic:       4 bytes = "PRWZ"
    major:       u16 big-endian = 1
    minor:       u16 big-endian = 0
    operation:   u16 big-endian = 1
    flags:       u16 big-endian = 0
    target_len:  u16 big-endian
    target_utf8: target_len bytes
```

The fixed inner header before the target body is exactly 12 bytes.

The target UTF-8 byte length is restricted to `1..=1024` at this wire boundary.

## 6. Pure encoder

The new module may expose a function semantically equivalent to:

```rust
pub fn encode_requester_rendezvous_target_request_frame(
    request_id: u64,
    target_device_id: &DeviceId,
) -> Result<ControlFrame, RequesterRendezvousTargetWireError>
```

It must:

1. use outer PRWM `ControlMessageKind::Request`;
2. emit exact PRWZ magic/version/operation/zero flags;
3. encode the target as u16 length + exact UTF-8 bytes;
4. reject target byte length outside `1..=1024`;
5. preserve supplied correlation through existing `ControlFrame::new(...)` validation;
6. perform no requester authentication, registry lookup, policy evaluation, provider mutation or I/O.

A target `DeviceId` longer than the selected wire bound must fail even though the core domain type itself has no global maximum length.

## 7. Pure decoder

The new module must expose a function semantically equivalent to:

```rust
pub fn decode_requester_rendezvous_target_request_frame(
    frame: &ControlFrame,
) -> Result<RequesterRendezvousTargetWireRequest, RequesterRendezvousTargetWireError>
```

It must fail closed unless all of the following hold:

1. outer kind is PRWM `Request`;
2. inner magic is exactly `PRWZ`;
3. major/minor are exactly `1.0`;
4. operation is exactly `1`;
5. reserved flags are exactly zero;
6. target length is in `1..=1024`;
7. exactly that many target bytes are present;
8. target bytes are valid UTF-8;
9. `DeviceId::new(...)` succeeds;
10. no trailing bytes remain.

The decoded request must copy the exact outer request ID unchanged and must not derive any identity from it.

## 8. Error boundary

The pure module may define one bounded error enum with only wire/frame-construction concerns, including:

- wrong outer PRWM kind;
- invalid PRWZ payload;
- existing PRWM frame-construction error.

No error variant may represent requester authorization denial, registry currentness failure, policy rejection or provider mutation because those operations do not occur here.

## 9. Requester identity invariant

Requester identity is absent from this codec and remains only the exact authenticated application session retained later by Agent runtime custody.

The following must not become requester identity:

- decoded target `DeviceId`;
- outer request ID;
- `TransportIdentity`;
- `SessionId`;
- control stream;
- endpoint/IP;
- candidate data;
- publisher identity;
- repeated-admission `expected_device_id`;
- role;
- registry/policy/provider state;
- default or environment state.

## 10. Target identity invariant

The target is exactly the `DeviceId` represented by the one PRWZ target field.

The codec must not infer, normalize, substitute or default the target from requester/session/transport/request/publication/endpoint/candidate/registry/policy/provider state.

Successful decode is target nomination only.

## 11. Crate boundary invariant

The codec lives in `prw-remote-bridge` and uses only already-existing bridge dependencies, including `prw-core` and `prw-remote-transport`.

It must not:

- add a bridge -> Agent dependency;
- widen the Agent target-intent module visibility;
- name Agent-owned `RequesterRendezvousTargetIntent`;
- modify any manifest or lockfile.

The future Agent adaptation from decoded `DeviceId` to the existing crate-private target-intent remains separately gated.

## 12. Generic capability separation

C03e-EM must not modify:

- generic capability inner magic `PRWC`;
- `BridgeCommand`;
- `BridgeCommand::operation_code()`;
- `BridgeCommand::required_capability()`;
- `CapabilityBridge::authorize(...)`;
- generic capability request I/O loop.

PRWZ request decoding is a distinct inner request protocol and must not fall through into generic command decoding.

## 13. Root registration

`crates/prw-remote-bridge/src/root.rs` may change only by adding the new reviewed public wire module registration:

```rust
pub mod requester_rendezvous_target_request_wire;
```

No other root export or visibility change is selected.

## 14. Tests required

The source module must include strict side-effect-free tests covering at least:

- valid encode/decode round trip preserving exact request ID and target;
- wrong outer kind rejection;
- wrong magic rejection;
- unsupported version rejection;
- unknown operation rejection;
- non-zero flags rejection;
- zero target length rejection;
- target length above 1024 rejection;
- invalid UTF-8 rejection;
- whitespace-only target rejection through `DeviceId::new(...)`;
- trailing-byte rejection;
- encode rejection for a `DeviceId` above the 1024-byte wire bound;
- zero outer request ID rejected through existing PRWM frame validation.

Tests must perform no socket I/O, registry/policy/provider action, task spawn or network activation.

## 15. Exact predecessor source anchors

At closed C03e-EL:

- bridge root: `293c94d56533ccac9d0b5e0301366623116f1788`
- bridge Cargo: `5fd48263be415aac28dee1c71a4031a4a02ad36c`
- remote transport: `47b007f3f4151b8971a57997db22b223f8d70ce6`
- session-auth PRWM codec: `492d3e938fcbc75907b345750928717c957204e8`
- generic bridge: `7b1c5c62339983da6ae2556f73510d7582ec0c5b`
- core domain types: `665afdb5f2627a7d84f09b476302503e66e121e2`
- Agent lib visibility: `58b37553c2f089e0f5f4a911be2f40893e18173c`
- Agent target-intent source: `5f616f20699d1c7069f5aa8973200a0359c19cde`
- EJ caller seam root: `f669ca79baecaef8bc90df1cf3bb53799db67623`
- Agent Cargo: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root Cargo.lock: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native Cargo.lock: `cce9ca06190a196661ab38d54a747893e26af95f`

All anchors except the bridge root must remain byte-stable. The bridge root may differ only by the one selected module registration line.

## 16. Explicit exclusions

C03e-EM does not select or materialize:

- Agent-side wire-request adaptation;
- `RequesterRendezvousTargetIntent` construction from wire;
- requester-specific stream acceptance or `MeshControlStream` I/O;
- success/error response protocol;
- request-ID lifecycle/replay/idempotency semantics;
- EJ execution from I/O;
- C03e-DV invocation;
- requester-aware registry/policy/provider execution;
- provider construction/capacity/persistence;
- generic `BridgeCommand` changes;
- worker/admission/public process-input widening;
- bootstrap/main/network activation;
- deployment;
- restart/recovery;
- merge.

## 17. Closure criteria

C03e-EM may close only on one exact final head proving:

1. exact C03e-EL merge base;
2. ahead only and zero behind;
3. changed paths limited to this contract, the new PRWZ module and one-line bridge-root registration;
4. no Agent/manifest/lock changes;
5. strict codec tests source-materialized;
6. all unaffected anchors byte-stable;
7. canonical Rust validation FULL PASS;
8. Android classified exactly as triggered or not triggered, never inferring PASS from absence/SKIPPED;
9. no exact-final-head workflow pending/failing;
10. immutable Drive audit raw-readback byte-exact;
11. rolling predecessor exact post-EL before append;
12. exact EL prefix preserved after append;
13. EM closure/classification/target-gate markers each exactly once;
14. PR remains draft/open/unmerged.

## 18. Successor boundary

After durable C03e-EM closure, any successor must begin with a fresh exact-head topology audit.

A likely next step is an Agent-only adaptation selection/materialization from the bridge-owned decoded target `DeviceId` into the existing crate-private `RequesterRendezvousTargetIntent`. That remains separate from stream I/O and still does not authorize C03e-DV invocation.

## 19. Target gate

C03e-EM targets exactly:

`C03E_EM_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_CODEC_SOURCE_MATERIALIZED`

Passing this gate means only that a pure bounded bridge-owned PRWZ request codec exists and validates. It does not mean any requester/rendezvous remote transaction executes.