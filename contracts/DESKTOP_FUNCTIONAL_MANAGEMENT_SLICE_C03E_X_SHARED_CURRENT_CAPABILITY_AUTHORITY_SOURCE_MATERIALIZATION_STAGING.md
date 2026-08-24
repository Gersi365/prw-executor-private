# Phase 152 C03e-X — Shared Current Capability Authority Source Materialization Staging

Status: STAGED

Target gate:

`C03E_X_SHARED_CURRENT_CAPABILITY_AUTHORITY_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-W:

- branch: `phase-152-c03e-w-shared-current-capability-authority-selection-staging`
- head: `84ad85d5a72b7c9e5be2b11f01ecf762cd22bc01`
- tree: `925e4215993c8deef28913415007c08cabc4f303`
- gate: `C03E_W_SHARED_CURRENT_CAPABILITY_AUTHORITY_SELECTED`

C03e-X preserves exact W lineage.

## Purpose

Materialize only the C03e-W-selected shared-current authority and canonical already-authorized dispatch split required before later worker integration:

1. materialize one Agent-owned shared current registry/policy owner;
2. materialize one bounded internal synchronous operation under a fresh shared-authority read guard;
3. materialize one bridge-owned helper for dispatcher execution and bounded response-frame construction after authorization;
4. make the existing `CapabilityBridge::process_request(...)` use that same helper so post-authorization behavior has one canonical implementation;
5. stop before changing C03e-O/Q/S/V worker signatures, task spawn, worker collection, concurrent session admission, Agent binary wiring, readiness or runtime activation.

## Agent-owned shared authority source

New source:

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`

The materialized public owner is:

`SharedCurrentCapabilityAuthority<P>`

It owns one private:

`Arc<tokio::sync::RwLock<CurrentCapabilityAuthorityState<P>>>`

The private state owns together:

- one current `WorkspaceDeviceRegistry` by value; and
- one current policy evaluator `P` by value.

No split registry/policy locks are introduced.

## Clone semantics

`SharedCurrentCapabilityAuthority<P>` implements `Clone` manually by cloning only the outer `Arc`.

The implementation must not require `P: Clone` and must not require `WorkspaceDeviceRegistry: Clone`.

Tests prove that two owner clones point to the same shared allocation and that a non-Clone policy implementation can still be owned and the authority owner cloned.

This prevents accidental per-worker registry or policy snapshots.

## Internal read operation

C03e-X materializes one Agent-module-internal asynchronous method equivalent in responsibility to:

`with_current_authority(...)`

The method:

1. asynchronously acquires one Tokio read guard;
2. passes `&WorkspaceDeviceRegistry` and `&P` to exactly one synchronous caller operation;
3. requires a return type independent of those borrowed arguments;
4. returns only after the synchronous operation completes;
5. releases the guard before the outer caller may perform subsequent dispatcher execution, network I/O or lifecycle work.

The method is `pub(super)`, not public domain API, and raw `RwLockReadGuard` / `RwLockWriteGuard` values are not exposed.

It is intentionally staged before the separately gated worker integration checkpoint, so a narrow `dead_code` allowance with explicit reason is permitted only on this internal seam.

## No mutation API yet

C03e-X does not select or materialize a public generic raw-guard mutation API.

Future registry/policy mutation integration must use the same combined state write lock, but exact management mutation surfaces remain separately gated.

No currentness snapshot or capability-decision cache is added.

## Bridge-owned authorized dispatch helper

New source:

`crates/prw-remote-bridge/src/authorized_request_dispatch.rs`

The materialized helper is:

`dispatch_authorized_request(...)`

It accepts:

- one existing owned `AuthorizedCapabilityRequest` by reference; and
- one mutable existing `CapabilityDispatcher`.

It performs exactly the existing post-authorization behavior:

1. invoke dispatcher exactly once;
2. map dispatcher failure to existing `RemoteBridgeError::DispatchFailed`;
3. enforce existing `MAX_CONTROL_PAYLOAD_BYTES`;
4. return existing `RemoteBridgeError::DispatchResponseTooLarge` when exceeded;
5. construct one `ControlMessageKind::Response` frame using the original request ID;
6. map any response-frame constructor failure to existing `RemoteBridgeError::ResponseFrameRejected`.

It performs no authorization, retry, negative-response invention, replacement session or peer close.

## Canonical legacy bridge delegation

The existing `CapabilityBridge::process_request(...)` remains public and behavior-compatible, but its post-authorization implementation is refactored to delegate to the new helper:

1. call existing `authorize(...)` exactly once;
2. call `authorized_request_dispatch::dispatch_authorized_request(...)` exactly once;
3. return that result unchanged.

The old local response construction block is removed from `process_request(...)`.

The now-unused private `map_transport_error(...)` helper and its `RemoteTransportError` import are removed because the new bridge module maps the same constructor failure directly to `RemoteBridgeError::ResponseFrameRejected`.

This refactor does not change public error classification or response framing.

## Production bridge root exposure

Production crate root `crates/prw-remote-bridge/src/root.rs` exposes:

`pub mod authorized_request_dispatch;`

The legacy bridge remains the existing private module and its public API remains re-exported.

## Agent module exposure

Existing `crates/prw-agent/src/remote_session_capability_runtime.rs` declares the new child module and re-exports only:

`SharedCurrentCapabilityAuthority`

The private state type and lock guards are not re-exported.

## Dependency boundary

C03e-X adds no dependency or feature.

Existing C03e-U Agent Tokio dependency already includes `sync`. Existing Agent dependencies already include `prw-policy` and `prw-registry`.

Therefore these must remain unchanged:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

No direct `prw-remote-transport` dependency may be added to Agent.

## Existing worker path remains unchanged

C03e-X deliberately does not change:

- `AuthenticatedRemoteSessionRuntimeOwner::process_one_capability_request(...)`;
- C03e-Q `run_capability_request_loop(...)`;
- C03e-S `run_capability_request_worker(...)`;
- C03e-V `RemoteSessionExecutorRuntime::drive_capability_request_worker(...)`.

Those still use their current borrowed `CapabilityBridge` boundary until a separately gated integration checkpoint replaces that borrow with the shared-current owner.

This staging split prevents C03e-X from silently selecting spawned task semantics or concurrent admission.

## Current authorization invariants retained

The source materialized here does not change the current authorization rules.

Later integration must continue to use existing `BoundRemoteSession::authorize(...)` / `CapabilityBridge::authorize(...)`, which retain:

- verifier-owned lease time validation;
- current `WorkspaceDeviceRegistry::validate_authenticated_session(...)`;
- current transport identity validation;
- request decoding;
- current policy evaluation.

The owned `AuthorizedCapabilityRequest` remains one-request evidence only and is never reusable for a later request.

## Lock hold boundary

The shared-current owner source is designed so a later integration can enforce the C03e-W hard rule that no authority guard is held across:

- stream accept;
- frame receive;
- dispatcher execution;
- response send;
- filesystem/terminal/forwarding side effects;
- cancellation wait;
- task spawn;
- join/drain;
- readiness publication.

C03e-X itself does not yet integrate the worker request path with this owner.

## Identity invariants

No lock/runtime/task value becomes PRW identity.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- PID/UID/GID/thread/runtime/lock identifiers are not logical identity.

## Temporary corrective workflow

Because GitHub's contents API exposes whole-file replacement rather than a bounded patch primitive for the large legacy bridge file, X stages one branch-local self-removing workflow only to perform the exact canonical delegation patch.

Required corrective behavior:

- exact branch/path prechecks;
- exact text-boundary replacement only in `crates/prw-remote-bridge/src/lib.rs`;
- canonical rustfmt over the workspace;
- locked dependency graph check;
- changed-path allow-list;
- remove the temporary workflow in the same corrective commit;
- push only a fast-forward commit to the same X branch.

The temporary workflow must be absent from the final tree and final W -> X net diff.

Any CI runs on a head containing the temporary workflow are diagnostic/superseded only and cannot serve as completion evidence.

## Intended final net diff boundary

Final W -> X net diff is intended to contain exactly these six paths:

1. this X contract;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`;
4. `crates/prw-remote-bridge/src/authorized_request_dispatch.rs`;
5. `crates/prw-remote-bridge/src/lib.rs`;
6. `crates/prw-remote-bridge/src/root.rs`.

Expected unchanged paths include:

- `Cargo.lock`;
- Agent and bridge Cargo manifests;
- authenticated remote-session worker source;
- executor drive source;
- Agent `lib.rs` and `main.rs`;
- Android application source;
- permanent workflows;
- readiness, packaging/systemd and host-network source.

## Validation requirements

Closure requires on the final exact head after the temporary workflow is absent:

- exact W merge base;
- exact final net-scope review;
- locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS;
- workspace tests PASS;
- workspace build PASS;
- canonical Android native/application validation because Rust source changes are present;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback verification;
- append-only rolling Drive update preserving the complete post-W prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-X means only that the selected shared-current authority owner exists in Agent source and the bridge has one canonical already-authorized dispatch/response helper.

It does not mean the existing remote worker path consumes that owner, shared mutations are wired, session workers are spawned, multiple sessions are admitted concurrently, the Agent binary is wired, remote transport is activated, or readiness may be published.

The next checkpoint must explicitly integrate the existing single-worker transaction path with this shared-current authority before any spawned-task ownership is selected.

Target gate:

`C03E_X_SHARED_CURRENT_CAPABILITY_AUTHORITY_SOURCE_MATERIALIZED`
