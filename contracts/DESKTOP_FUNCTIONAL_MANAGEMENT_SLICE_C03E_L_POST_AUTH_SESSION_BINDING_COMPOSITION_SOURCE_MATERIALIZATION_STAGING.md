# Private Remote Workspace — Phase 152 C03e-L Post-Auth Session Binding Composition Source Materialization Staging

Status: source-materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-k-authenticated-remote-session-runtime-owner-source-materialization-staging`
- head: `41c63ee990ade3b6feb19e018cbfeb11f14d985c`
- tree: `eb9b1a89b680b2aa8296d1dde1780143acf21704`
- gate: `C03E_K_AUTHENTICATED_REMOTE_SESSION_RUNTIME_OWNER_SOURCE_MATERIALIZED`

C03e-K is closed on its immutable branch. C03e-L is created from that exact head without merge, rebase, force-update or mutation of any closed checkpoint.

## Selection authority

C03e-I already selected the post-authentication lifecycle semantics and C03e-J/K materialized the two required lifetime owners.

After C03e-H authentication succeeds:

1. the same live `AuthenticatedRemotePeerConnection` remains open;
2. the authenticated logical `AuthenticatedDeviceSession` is consumed into one application `RemoteSessionLease` through the existing `BoundRemoteSession` constructor;
3. the application lease interval is verifier-owned and separate from the authentication challenge issue/expiry interval;
4. the already-revalidated peer `TransportIdentity` is snapshotted from the same live peer;
5. successful binding is transferred into the C03e-J `RemoteSessionCapabilityRuntimeOwner`;
6. that capability owner and the same live peer are transferred into the C03e-K `AuthenticatedRemoteSessionRuntimeOwner`.

C03e-L materializes exactly this composition transaction and nothing beyond it.

## Materialized function

C03e-L adds:

`compose_authenticated_remote_session(...)`

Exact input shape:

- `AuthenticatedRemotePeerConnection` by value;
- `AuthenticatedDeviceSession` by value;
- one separately verifier-owned `Range<u64>` application lease interval.

Exact output shape:

`Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteBridgeError>`

The existing `RemoteBridgeError` is preserved directly rather than widened into a new error taxonomy.

## Success sequence

The function performs the following deterministic source-level sequence:

1. read `peer.transport_identity()` exactly once;
2. call existing `BoundRemoteSession::new(...)` exactly once with:
   - that transport-identity snapshot;
   - the authenticated logical session by value;
   - application lease `start` as issue time;
   - application lease `end` as expiry time;
3. on success, transfer the binding into `RemoteSessionCapabilityRuntimeOwner::new(...)`;
4. transfer the same peer and capability owner into `AuthenticatedRemoteSessionRuntimeOwner::new(...)`;
5. return the outer owner.

No registry or policy decision is cached during this sequence. Future capability requests remain subject to current registry, transport-binding, lease and policy authority through the existing bridge.

## Separate application lease authority

The `Range<u64>` supplied here is the application-session lease interval selected in C03e-I.

It must not be silently derived from:

- C03e-E authentication challenge issue time;
- C03e-E authentication challenge expiry time;
- C03e-H proof-verification `now` alone;
- QUIC/TLS handshake time;
- connection acceptance time;
- a hidden wall-clock read.

The existing `BoundRemoteSession::new(...)` remains the sole construction seam and delegates lease validation to the existing `RemoteSessionLease::new(...)`, including the locked maximum lifetime.

## Binding failure semantics

If `BoundRemoteSession::new(...)` rejects the application lease:

- no `RemoteSessionCapabilityRuntimeOwner` is created;
- no `AuthenticatedRemoteSessionRuntimeOwner` is created;
- the same live peer is explicitly closed;
- the close uses one fixed, private, non-secret diagnostic;
- the exact existing `RemoteBridgeError` is returned unchanged;
- no retry occurs;
- no replacement session ID is created;
- no replacement lease is selected.

Successful C03e-H authentication already consumed the pending challenge, therefore C03e-L must not call `abort_pending_session(...)` on binding failure.

No authenticated-session deletion API currently exists and C03e-L does not invent one.

## Identity boundary

C03e-L preserves the locked identity model:

- logical identity remains the authenticated PRW device/session identity;
- `TransportIdentity` remains only the lower-transport certificate-derived identity;
- IP address is not logical identity;
- PID/UID/GID are not remote logical identity.

The transport identity used for `BoundRemoteSession` is obtained only from the same already-authenticated peer object, not from request input.

## Current authorization remains dynamic

Creation of `AuthenticatedRemoteSessionRuntimeOwner` is not capability authorization.

For every future capability request the existing bridge remains authoritative for:

- application lease validity at request time;
- current authenticated-session registry state;
- current device lifecycle/membership;
- current transport-identity binding;
- bounded protocol decoding;
- current policy decision;
- dispatcher admission after authorization.

Registry revocation, transport rotation, lease expiry and policy denial therefore remain effective after L composition succeeds.

## Source scope

Relative to exact closed C03e-K, C03e-L is restricted to exactly:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_L_POST_AUTH_SESSION_BINDING_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`;
2. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`.

No module export change is required because C03e-K already exports `AuthenticatedRemoteSessionRuntimeOwner` from the capability-runtime module and the new composition function remains in that existing child module until a later operation-surface checkpoint explicitly selects broader exposure.

## Protected boundaries

C03e-L keeps byte-stable relative to C03e-K:

- every `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_transport_runtime.rs`;
- `crates/prw-agent/src/remote_session_authentication_transaction.rs`;
- C03d session-auth wire source;
- bridge `BoundRemoteSession` source;
- `RemoteSessionLease` source;
- C03e-F session-service source;
- registry/policy/dispatcher source;
- workflows;
- Android application source;
- local Linux runtime/readiness source;
- packaging/systemd/host-network source.

## No request loop or runtime activation

C03e-L does not:

- accept or open a QUIC stream;
- read/write capability frames;
- run a capability request loop;
- expose the retained bound session;
- own `CapabilityBridge`, policy evaluator or dispatcher state;
- spawn a session task/worker;
- select concurrent-session collection/ownership;
- add retry/backoff/reconnect/session refresh;
- wire `main.rs`;
- publish remote readiness;
- bind a new endpoint;
- activate ICE/STUN/TURN/relay;
- load/provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- deploy, restart or merge.

## Validation requirements

Because Rust source changes, the final exact L head must pass both canonical validation surfaces on the same exact SHA:

- PRW Rust validation: locked graph, rustfmt, Clippy, workspace tests, workspace build;
- PRW Android validation: exact toolchains, native adapter, Android application.

Skipped workflows remain skipped and are not counted as PASS evidence.

Any formatter/lint-only defect must be corrected minimally on the same branch without rebase or scope widening.

## Completion gate

After exact-scope verification, exact-head Rust/Android validation, immutable Drive audit publication and append-only rolling closeout:

`C03E_L_POST_AUTH_SESSION_BINDING_COMPOSITION_SOURCE_MATERIALIZED`
