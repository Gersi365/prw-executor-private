# Private Remote Workspace — Phase 152 C03e Remote Session Transport Binding Source Materialization Staging

Status: source/materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03d-logical-session-auth-wire-source-materialization-staging`
Exact predecessor head: `6af22ceb830b505b8be25951d9335959b70b3dcd`
Exact predecessor tree: `89f0298e58d3250da75a9a5ecae0630c4a9ce739`
Predecessor gate: `C03D_LOGICAL_SESSION_AUTH_WIRE_SOURCE_MATERIALIZED`

## Purpose

C03e closes the first post-authentication composition gap between the C03c transport-authenticated peer identity, the C03d/Phase-128 logical authenticated device session, and the existing Phase-143 remote capability admission path.

The existing `CapabilityBridge` already fail-closes on lease time, current registry state, transport-identity binding, request codec and capability policy. C03e does not duplicate those checks. It materializes one opaque binding object so later Agent remote-runtime code does not repeatedly pass an independently selected `TransportIdentity` and `RemoteSessionLease` for every request.

## Selected ownership boundary

The new reusable source type is `BoundRemoteSession` in `prw-remote-bridge`.

It owns exactly:

- one immutable transport-identity snapshot obtained by its caller after lower-layer transport authentication;
- one existing `RemoteSessionLease`, constructed from one existing `AuthenticatedDeviceSession` and verifier-owned lease times.

Its fields are private. It exposes immutable inspection only and delegates authorization/dispatch to the existing `CapabilityBridge` using its stored pair.

The composition is therefore:

`C03c authenticated TransportIdentity`
→ `C03d / Phase-128 AuthenticatedDeviceSession`
→ `C03e BoundRemoteSession { transport identity, RemoteSessionLease }`
→ `existing CapabilityBridge`
→ `current registry + current transport binding + exact policy capability`
→ `existing dispatcher boundary`

## Security invariant

C03e does **not** declare the transport identity and logical session mutually valid merely because they were placed into one object. The current registry remains authoritative on every capability request.

Consequently:

- a mismatched transport identity still fails as `TransportIdentityRejected`;
- a stale/revoked/mutated authenticated device session still fails through current-registry validation;
- an expired/not-yet-valid lease still fails through existing lease validation;
- denied capability policy still prevents dispatch;
- malformed/wrong-kind request frames still fail through the existing bridge codec/admission path.

The value of the binding is ownership/caller discipline: after construction, request authorization no longer accepts a second independently supplied transport identity that could accidentally diverge from the transport peer selected for that remote session context.

## Constructor boundary

`BoundRemoteSession::new(...)` accepts:

1. one transport identity that the caller obtained from an already-authenticated lower transport context;
2. one `AuthenticatedDeviceSession` returned by the existing session-authentication authority;
3. verifier-owned lease issue and expiry timestamps.

The constructor delegates lease validation exactly once to `RemoteSessionLease::new`. It performs no network I/O, registry mutation, policy evaluation or capability dispatch.

A caller that supplies a stale or incorrect transport identity cannot turn that mistake into authorization: the existing `CapabilityBridge` validates the stored transport identity against current registry state on every request.

## Delegation API

The binding exposes two bounded operations:

- `authorize(...)` delegates to `CapabilityBridge::authorize(...)` with the stored transport identity and lease;
- `process_request(...)` delegates to `CapabilityBridge::process_request(...)` with the same stored pair.

No authorization rule is copied into C03e.

## Required focused validation

C03e must prove that:

1. a Phase-128 authenticated device session can be wrapped in the existing bounded `RemoteSessionLease` through the new binding;
2. the binding preserves the exact authenticated session identity and selected transport identity;
3. an allowed bounded request reaches existing `CapabilityBridge` and produces the same authorized principal/capability semantics;
4. the binding does not expose a second transport-identity argument on its request methods;
5. rotating current registry transport identity causes the previously bound transport identity to fail closed as `TransportIdentityRejected`;
6. lease expiry still fails through the existing `RemoteBridgeError::SessionExpired` path;
7. invalid lease duration still fails through `RemoteSessionLease::new` semantics;
8. no dispatcher invocation occurs when bridge authorization fails;
9. no new dependency, lockfile, network runtime, Agent lifecycle or capability implementation is introduced.

## Protected byte-stable boundaries

Relative to C03d, C03e must not change:

- `crates/prw-remote-bridge/Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- any Agent local runtime/lifecycle source;
- workflow files;
- Android application source.

## Negative guarantees

C03e does not:

- bind or own a UDP socket;
- own a Quinn endpoint or QUIC connection;
- open/accept QUIC streams;
- execute the C03d challenge/proof exchange;
- create an Agent remote listener;
- spawn runtime tasks or workers;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- add retry/backoff/reconnect/session refresh;
- provision credentials;
- mutate systemd/firewall/NAT/router/TUN/TAP/routes/DNS;
- execute recovery epochs, PRWF or R1–R4 effects;
- deploy, restart or merge.

## Expected source mutation

Exactly four paths are expected:

1. this contract;
2. `crates/prw-remote-bridge/src/remote_session_binding.rs`;
3. the one-line module export in `crates/prw-remote-bridge/src/root.rs`;
4. one focused integration test for binding/bridge behavior.

## Completion gate

After exact-head Rust/Android CI and Drive evidence closeout:

`C03E_REMOTE_SESSION_TRANSPORT_BINDING_SOURCE_MATERIALIZED`
