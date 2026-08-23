# Private Remote Workspace — Phase 152 C03e-K Authenticated Remote Session Runtime Owner Source Materialization Staging

Status: source-materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-j-remote-session-capability-owner-source-materialization-staging`
- head: `c1d0849b8f12f2b192d07b79509fc4166fc769b1`
- tree: `16d5ba1392d3c6c99b1ebfec73c78adfc7d9de22`
- gate: `C03E_J_REMOTE_SESSION_CAPABILITY_RUNTIME_OWNER_SOURCE_MATERIALIZED`

C03e-J is closed on its own immutable branch. C03e-K is a downstream branch from the exact closed J head and does not rewrite, merge, rebase or force-update J.

## Selection authority

C03e-I already selected the post-authentication lifetime relationship:

1. retain the same live `AuthenticatedRemotePeerConnection` after successful logical-session authentication;
2. create one `BoundRemoteSession` from the authenticated session and verifier-owned application lease in a later composition step;
3. transfer that binding into the C03f/C03e-J `RemoteSessionCapabilityRuntimeOwner`;
4. retain the live authenticated peer and capability owner together under one future Agent-owned `AuthenticatedRemoteSessionRuntimeOwner` for the connected authenticated application-session lifetime.

C03e-K materializes only item 4's pure ownership type and constructor. It does not materialize item 2 or item 3 composition.

## Purpose

C03e-K introduces the source-level outer lifetime boundary required before the post-authentication binding/composition transaction can be materialized separately.

The new type is:

`AuthenticatedRemoteSessionRuntimeOwner`

Its constructor shape is:

`AuthenticatedRemoteSessionRuntimeOwner::new(AuthenticatedRemotePeerConnection, RemoteSessionCapabilityRuntimeOwner) -> AuthenticatedRemoteSessionRuntimeOwner`

Construction is ownership composition only.

## Exact ownership

The owner consumes and privately retains exactly:

- one existing `prw_remote_bridge::remote_server_transport_runtime::AuthenticatedRemotePeerConnection` by value; and
- one existing C03e-J `RemoteSessionCapabilityRuntimeOwner` by value.

It does not consume or retain:

- `AgentRemoteTransportRuntime`;
- `ReachabilityAuthorityRuntimeOwner`;
- `SessionAuthenticationService`;
- `AuthenticatedDeviceSession` independently of the capability owner;
- `WorkspaceDeviceRegistry`;
- `CapabilityBridge`;
- dispatcher state;
- a QUIC control stream;
- a task/worker handle;
- retry/reconnect/session-refresh state;
- remote readiness state.

## Why the live peer is retained

C03e-I selected one connected authenticated remote application-session lifetime. The peer must therefore remain owned together with the bound capability context after post-authentication composition succeeds.

The peer remains the existing lower-transport-authenticated connection. C03e-K does not reinterpret its `TransportIdentity` as a logical device identity and does not grant capability authority merely because the peer is retained.

Logical identity remains the authenticated PRW session/device identity carried through the existing session and binding layers.

## Why the capability owner is retained

The C03e-J capability owner retains the exact `BoundRemoteSession` pair selected earlier:

- transport-identity snapshot; and
- bounded authenticated logical-session lease.

Current registry/policy/transport-binding checks remain dynamic per future request through the existing bridge authority. C03e-K adds no cached authorization decision.

## No post-auth binding composition yet

C03e-K intentionally does not call `BoundRemoteSession::new(...)`.

Therefore this checkpoint does not yet select or materialize:

- the exact source function that receives the verifier-owned application lease interval;
- the exact point at which `peer.transport_identity()` is snapshotted for binding construction;
- the close-on-binding-failure implementation;
- a narrow composition error envelope, if one is required;
- transfer of a newly-created binding through the C03e-J constructor into this outer owner.

Those are reserved for the next separately gated post-auth composition source checkpoint.

## No I/O or runtime activation

`AuthenticatedRemoteSessionRuntimeOwner::new(...)` performs no:

- network read/write;
- stream accept/open;
- authentication;
- registry lookup;
- policy evaluation;
- capability authorization/dispatch;
- peer close;
- task spawn;
- request loop;
- readiness publication;
- timer/retry/reconnect behavior;
- endpoint bind/listen;
- credential load.

The type only moves two already-existing owners into private fields.

## Source scope

Relative to exact closed C03e-J, C03e-K is restricted to:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_K_AUTHENTICATED_REMOTE_SESSION_RUNTIME_OWNER_SOURCE_MATERIALIZATION_STAGING.md`;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs` — downstream submodule declaration/re-export only;
3. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` — outer owner source and constructor-shape test.

No manifest or lockfile change is required because `prw-agent` already directly depends on `prw-remote-bridge`.

## Protected boundaries

C03e-K must keep byte-stable relative to C03e-J:

- every `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/remote_transport_runtime.rs`;
- `crates/prw-agent/src/remote_session_authentication_transaction.rs`;
- C03d wire source;
- C03e `BoundRemoteSession` source;
- C03e-F session service source;
- registry/policy/dispatcher source;
- workflows;
- Android application source;
- local Linux runtime/readiness source;
- packaging/systemd/host-network source.

The C03e-J capability-owner source changes only to declare its child module and re-export the new outer owner. The existing `RemoteSessionCapabilityRuntimeOwner` fields, constructor and test contract remain otherwise unchanged.

## Validation requirements

Because Rust source changes, the final exact K head must pass:

- canonical PRW Rust validation: locked graph, rustfmt, Clippy, workspace tests, workspace build; and
- canonical PRW Android validation: exact toolchains, native adapter, Android application.

Both PASS claims must refer to the same exact final head. Skipped workflows remain skipped and are not counted as PASS evidence.

Any formatter/lint-only defect must be corrected minimally on the same branch without rebasing or widening scope.

## Negative guarantees

C03e-K does not:

- merge/rebase/force-update a closed checkpoint;
- call `BoundRemoteSession::new(...)`;
- create or choose an application lease interval;
- call pending-session abort;
- invent authenticated-session deletion;
- close a peer;
- accept/open/read/write a stream;
- authorize or dispatch a capability;
- expose the retained peer or capability owner;
- select a request-loop API;
- select session collection/concurrency ownership;
- add retry/backoff/reconnect/session refresh;
- wire `main.rs`;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- initialize PRWF or recovery effects;
- deploy, restart or merge.

## Completion gate

After exact-scope verification, exact-head Rust/Android validation, immutable Drive audit publication and append-only rolling closeout:

`C03E_K_AUTHENTICATED_REMOTE_SESSION_RUNTIME_OWNER_SOURCE_MATERIALIZED`
