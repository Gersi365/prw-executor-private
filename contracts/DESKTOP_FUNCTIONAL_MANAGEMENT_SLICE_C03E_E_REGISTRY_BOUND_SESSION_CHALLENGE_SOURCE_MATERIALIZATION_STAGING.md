# Private Remote Workspace — Phase 152 C03e-E Registry-Bound Session Challenge Source Materialization Staging

Status: source/materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03e-d-authenticated-peer-handoff-source-materialization-staging`
Exact predecessor head: `06fb5c249b414937b212b0e05d0f9bd4eab9f288`
Exact predecessor tree: `89e15d587810fde36a4ee05829beac53aa26d4ca`
Predecessor gate: `C03E_D_AUTHENTICATED_PEER_HANDOFF_SOURCE_MATERIALIZED`

## Purpose

C03e-E materializes the first logical-session preparation step after the C03e-D lower-transport-authenticated peer handoff. It permits the Agent to begin one existing Phase 128 session-authentication challenge only after the accepted peer's already-validated `TransportIdentity` matches the current transport binding for an exact logical `DeviceId` in the existing `WorkspaceDeviceRegistry`.

This checkpoint deliberately stops before C03d wire I/O. It does not send the challenge, receive a proof, verify a proof, create `AuthenticatedDeviceSession`, create `RemoteSessionLease`/`BoundRemoteSession`, materialize the C03f owner, spawn a task, or publish remote readiness.

## Selected preparation seam

`AgentRemoteTransportRuntime` gains one synchronous preparation method:

`begin_registry_bound_session_challenge(...)`.

The method requires:

- the already-existing `AgentRemoteTransportRuntime`, retaining C02f reachability-authority admission;
- one opaque C03e-D `AuthenticatedRemotePeerConnection`;
- the authoritative current `WorkspaceDeviceRegistry`;
- the existing mutable `SessionAuthenticationService`;
- an exact logical `DeviceId`;
- one typed `SessionId`;
- one verifier-owned half-open challenge validity range `issued_at_unix_seconds..expires_at_unix_seconds`.

The selected sequence is:

1. read only the accepted peer's already-revalidated transport identity;
2. call the existing registry `validate_transport_identity(device_id, peer_transport_identity)` currentness gate;
3. retrieve that exact currently registered device record by `DeviceId`;
4. clone only its registry-owned `DeviceIdentityBinding` snapshot;
5. call the existing `SessionAuthenticationService::begin_session(...)` with that registry-owned binding, typed session identifier, and the range start/end as the verifier-owned issue/expiry times;
6. return the existing typed `SessionAuthChallenge`.

No caller-supplied `DeviceIdentityBinding` is accepted by this Agent method.

## Authority invariants

C03e-E preserves these boundaries:

- `DeviceId` remains the logical device selector;
- `TransportIdentity` remains a lower-transport certificate identity, not a logical identity substitute;
- the accepted peer's transport identity must equal the current registry transport binding for the selected `DeviceId` before a challenge is created;
- the device identity binding used by Phase 128 comes only from the current registry record;
- `SessionAuthenticationService` remains the sole owner of nonce generation, challenge lifetime validation, pending-session state, duplicate-session rejection and later proof verification;
- challenge preparation is authentication setup only, not authentication success or authorization.

C03e-E intentionally does not add a reverse `TransportIdentity -> DeviceId` registry lookup. The logical `DeviceId` must already be selected by a separately reviewed higher-level session-initiation path; C03e-E only proves that the selected logical device and accepted lower transport currently agree in the registry.

## Pending-session ownership

A successful `SessionAuthenticationService::begin_session(...)` creates the existing Phase 128 pending challenge state. C03e-E performs no network I/O after that mutation, so it introduces no new partial-I/O cleanup problem.

The later C03d wire-execution checkpoint must separately select and materialize failure cleanup/cancellation semantics before it sends a prepared challenge over an unreliable stream. C03e-E does not add retry, cancellation, timeout, or pending-state purge behavior.

## Expected source mutation

Relative to exact C03e-D, exactly two paths are expected:

1. this contract;
2. `crates/prw-agent/src/remote_transport_runtime.rs`.

No new module, dependency, manifest, lockfile, registry implementation, session implementation, bridge implementation, workflow, or Android application change is required.

## Focused validation

Source validation must prove at minimum:

- the new method requires an `AuthenticatedRemotePeerConnection`, authoritative registry, exact `DeviceId`, typed `SessionId`, verifier-owned validity range, and existing `SessionAuthenticationService`;
- the public method accepts no `DeviceIdentityBinding` argument;
- current registry transport validation occurs before `begin_session`;
- the binding passed to `begin_session` is cloned only from `registry.device(device_id).binding()`;
- the validity range is forwarded only as the existing Phase 128 verifier-owned issue/expiry boundary and lifetime validation remains owned by `SessionAuthenticationService`;
- registry and session errors retain their existing typed classifications inside one narrow Agent error envelope;
- C03e-D accepted-peer source, C03d wire, C03e binding, manifests, lockfiles, `main.rs`, workflows and Android application source remain byte-stable;
- canonical Rust and Android validation pass on the exact final source head.

## Relationship to downstream checkpoints

After C03e-E, a separately gated wire-execution checkpoint may:

- accept one bounded control stream from the same C03e-D peer;
- encode/send the prepared C03d challenge;
- receive and decode exactly one proof;
- enforce request/session envelope correlation;
- submit the typed proof to the same existing `SessionAuthenticationService`;
- define deterministic cleanup for pending state on wire/proof failure.

Only after successful proof verification may a later checkpoint construct C03e `BoundRemoteSession` and then materialize/compose the separately selected C03f Agent session-capability owner.

## Protected boundaries

C03e-E must not change:

- `crates/prw-registry/src/lib.rs`;
- `crates/prw-session/src/lib.rs`;
- any `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- existing local Linux runtime/readiness modules;
- C02f reachability authority semantics;
- C03e-D accepted-peer implementation;
- C03d session-auth wire implementation;
- C03e `BoundRemoteSession` implementation;
- C03f contract;
- workflows;
- Android application source.

## Negative guarantees

C03e-E does not:

- accept another network peer;
- open or accept a control stream;
- send or receive PRWM/PRWS frames;
- verify a device proof;
- create `AuthenticatedDeviceSession`, `RemoteSessionLease`, or `BoundRemoteSession`;
- materialize `RemoteSessionCapabilityRuntimeOwner`;
- authorize or dispatch a capability;
- wire `main.rs`;
- spawn tasks/workers/executors;
- publish remote readiness or alter local `Ready`;
- add retry/cancel/purge/session-refresh behavior;
- run ICE/STUN/TURN or relay;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- execute PRWF, recovery epochs or R1–R4 effects;
- deploy, restart, rebase or merge.

## Completion gate

After exact-head canonical Rust/Android validation and Drive closeout:

`C03E_E_REGISTRY_BOUND_SESSION_CHALLENGE_SOURCE_MATERIALIZED`
