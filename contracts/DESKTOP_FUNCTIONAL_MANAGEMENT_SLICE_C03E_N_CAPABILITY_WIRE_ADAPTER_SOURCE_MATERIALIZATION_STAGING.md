# Private Remote Workspace — Phase 152 C03e-N Capability Wire Adapter Source Materialization Staging

Status: source-materialization staging
Date: 2026-08-24
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-m-capability-request-operation-surface-selection-staging`
- head: `b03a039c701467b2f34f7250140b22ba34b57c64`
- tree: `a4a5b13c65d686b9f285fee43c1255f7abb4c5bc`
- gate: `C03E_M_CAPABILITY_REQUEST_OPERATION_SURFACE_SELECTED`

C03e-M is closed on its immutable branch. C03e-N is created from that exact head without merge, rebase, force-update or mutation of any closed checkpoint.

## Selection authority

C03e-M selected a bridge-owned post-authentication capability wire adapter before any Agent single-request transaction or request loop is materialized.

The selected purpose is narrow: keep lower `MeshControlStream`, `ControlFrame` and `MeshQuicRuntimeError` mechanics behind `prw-remote-bridge` so `prw-agent` does not gain a direct `prw-remote-transport` dependency merely to perform one bounded request/response exchange.

## Materialized module

C03e-N adds:

`prw_remote_bridge::capability_request_wire`

The production crate root is `crates/prw-remote-bridge/src/root.rs`; the existing Cargo manifest selects that root explicitly. C03e-N therefore adds exactly one module export there and does not modify the legacy Phase 143 `src/lib.rs` implementation.

## Materialized error boundary

`CapabilityRequestWireError` is bridge-owned, non-exhaustive, copyable and comparable, with one current variant:

- `Runtime(MeshQuicRuntimeError)`

The existing lower runtime error is preserved as the standard error source. No Quinn object, raw stream half, socket error or parallel application error taxonomy is exposed.

## Materialized receive operation

`receive_capability_request_frame(&mut MeshControlStream)`:

- receives exactly one complete bounded PRWM frame through the existing `MeshControlStream::receive_frame()` primitive;
- returns the existing `ControlFrame` unchanged;
- preserves existing bounded read, timeout and PRWM transport validation failures through the bridge-owned wire error;
- performs no retry and accepts no second stream.

The adapter intentionally does not duplicate application request-kind validation. Existing `CapabilityBridge::authorize/process_request` remains authoritative for requiring `ControlMessageKind::Request`, decoding PRWC, validating lease/current registry/current transport binding and evaluating current policy.

## Materialized send operation

`send_capability_response_frame(&mut MeshControlStream, &ControlFrame)`:

- sends exactly one already-constructed bounded PRWM frame through existing `MeshControlStream::send_frame()`;
- therefore uses the existing bounded write and send-direction finish behavior;
- preserves lower runtime failures through the bridge-owned wire error;
- does not construct another response envelope;
- does not rewrite request correlation;
- does not invent a remote error-response protocol.

The future Agent transaction must supply the success response produced by existing `BoundRemoteSession::process_request(...)` / `CapabilityBridge::process_request(...)`.

## Authority boundary

C03e-N is transport adaptation only. It does not:

- authenticate transport or logical session identity;
- own or expose `BoundRemoteSession`;
- select or modify `TransportIdentity`;
- validate application-session lease time;
- query or cache current registry state;
- evaluate or cache policy;
- decode PRWC commands directly;
- invoke a capability dispatcher;
- select verifier time;
- decide whole-peer lifetime;
- close a peer;
- retry/reconnect;
- publish readiness.

Logical identity remains the authenticated PRW device/session identity. Lower `TransportIdentity` remains transport-only authority already bound by prior checkpoints.

## Dependency boundary

No manifest mutation is required or authorized.

`prw-remote-bridge` already depends on `prw-remote-transport` and owns the adapter. `prw-agent/Cargo.toml` remains byte-stable and continues not to depend directly on `prw-remote-transport`.

## Exact source scope

Relative to exact closed C03e-M, C03e-N is restricted to exactly three paths:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_N_CAPABILITY_WIRE_ADAPTER_SOURCE_MATERIALIZATION_STAGING.md`;
2. `crates/prw-remote-bridge/src/root.rs` — one module export only;
3. `crates/prw-remote-bridge/src/capability_request_wire.rs` — new bridge-owned adapter.

## Protected boundaries

C03e-N keeps byte-stable relative to C03e-M:

- every `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- legacy bridge `crates/prw-remote-bridge/src/lib.rs`;
- `remote_server_transport_runtime.rs`;
- `remote_session_binding.rs`;
- `session_auth_wire.rs`;
- all `prw-agent` source including capability owner, authenticated outer owner, auth transaction, transport runtime, `lib.rs` and `main.rs`;
- session service;
- registry/policy/dispatcher semantics;
- workflows;
- Android application source;
- readiness/local runtime;
- packaging/systemd/host-network source.

## Still separately gated

C03e-N does not materialize:

- `AuthenticatedRemoteSessionRuntimeOwner` accepting a capability stream;
- a call to `BoundRemoteSession::process_request(...)` from Agent code;
- a complete one-stream/one-request Agent transaction;
- any multi-request loop;
- concurrent streams or authenticated sessions;
- tasks/workers/cancellation/join;
- peer-close lifecycle policy after request failure;
- retry/backoff/reconnect/session refresh;
- Agent `main.rs` wiring;
- remote readiness.

The immediate next checkpoint after N closes is the separately gated Agent-owned one-stream/one-request transaction selected by C03e-M.

## Validation requirements

Because Rust source changes, the final exact N head must pass both canonical validation surfaces on the same exact SHA:

- PRW Rust validation: locked graph, rustfmt, Clippy, workspace tests, workspace build;
- PRW Android validation: exact toolchains, native adapter, Android application.

Skipped workflows remain skipped and are not counted as PASS evidence. Any formatter/lint-only defect must be corrected minimally on the same branch without rebase or scope widening.

## Completion gate

After exact-scope verification, exact-head Rust/Android validation, immutable Drive audit publication and append-only rolling closeout:

`C03E_N_CAPABILITY_WIRE_ADAPTER_SOURCE_MATERIALIZED`
