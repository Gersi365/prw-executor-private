# Private Remote Workspace — Phase 152 C03f Agent Remote Session Capability Ownership Selection Staging

Status: architecture/ownership selection staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03e-remote-session-transport-binding-source-materialization-staging`
Exact predecessor head: `1f98ba092946b84399336d8d35b5b1219fbc3075`
Exact predecessor tree: `4ed702a5c9a9562cc727d8c6e2fd52e5edeab79b`
Predecessor gate: `C03E_REMOTE_SESSION_TRANSPORT_BINDING_SOURCE_MATERIALIZED`

## Purpose

C03f selects the first legitimate Agent-side process ownership boundary for the C03e `BoundRemoteSession` without activating remote runtime behavior.

C03e intentionally materialized a reusable provider-neutral binding in `prw-remote-bridge`: one authenticated transport-identity snapshot plus one bounded logical `RemoteSessionLease`, with current registry and policy remaining authoritative inside the existing `CapabilityBridge` on every request.

The repository does not yet contain a concrete Agent remote listener/session manager that legitimately owns that application-layer bound-session lifetime. Existing `AuthenticatedLocalLinuxConnection` and `AuthenticatedLocalLinuxSession` are local Unix same-UID IPC types and must not be repurposed as remote-session ownership.

C03f therefore selects an Agent-owned remote-session capability lifetime owner as the next boundary before source materialization or runtime wiring.

## Selected owner

The selected future source type is an Agent-owned `RemoteSessionCapabilityRuntimeOwner`.

Its narrow ownership contract is:

- consume exactly one existing `prw_remote_bridge::remote_session_binding::BoundRemoteSession` by value;
- retain that binding for the application-layer lifetime of one admitted remote-session capability context;
- expose the retained binding only through a narrow Agent-internal operation seam selected in a later checkpoint;
- perform no transport, authentication, registry, policy or capability decision during pure ownership construction.

The selected ownership chain is therefore:

`lower transport owns authenticated QUIC connection / TransportIdentity`
→ `session-auth authority establishes AuthenticatedDeviceSession`
→ `C03e BoundRemoteSession binds transport snapshot + RemoteSessionLease`
→ `C03f-selected Agent RemoteSessionCapabilityRuntimeOwner retains one BoundRemoteSession`
→ `existing CapabilityBridge remains per-request authority`
→ `existing dispatcher boundary receives only authorized typed capability requests`

## Why Agent owns this lifetime

`prw-remote-bridge` owns reusable application admission semantics, not process-level Agent runtime lifetime.

The Agent is the correct future process-level owner because it will eventually coordinate:

- the lifetime of an admitted remote application session;
- access to current Agent-owned registry/policy/dispatcher context;
- later remote transport/session task lifecycle;
- later shutdown and cancellation sequencing.

C03f selects only that ownership responsibility. It does not materialize or activate those later runtime components.

## Explicit non-reuse of local IPC session types

The existing Agent types:

- `AuthenticatedLocalLinuxConnection`;
- `AuthenticatedLocalLinuxSession`;

are strictly local Unix-domain, kernel-credential, same-effective-UID boundaries. They own local IPC semantics and existing local command processing state.

C03f explicitly rejects using either type as a parent, wrapper or base for remote-session capability ownership. Remote transport identity, logical device-session authentication and current registry validation are distinct security domains and must remain distinct from local same-UID IPC.

## Ownership versus authorization

Holding a `BoundRemoteSession` is not authorization to execute a remote capability.

The selected owner must not cache or manufacture an authorization result. Every future request still flows through the existing `CapabilityBridge`, which remains authoritative for:

1. remote-session lease time;
2. current membership/device/session registry state;
3. current transport-identity binding;
4. bounded request decoding;
5. exact capability policy;
6. dispatcher admission only after all preceding gates succeed.

Consequently, registry revocation, membership changes, transport rotation, lease expiry and policy denial remain effective after the owner has been constructed.

## Reachability-authority ordering remains authoritative

C02f-CH and its subsequent materialization chain remain authoritative for future remote/reachability admission.

C03f does not declare that construction of `RemoteSessionCapabilityRuntimeOwner` is sufficient for a running Agent to expose remote capability service. Future runtime composition must still require the previously selected successful reachability-authority admission before authority-dependent remote admission.

Therefore:

- local Agent Ready remains unchanged;
- local Ready does not imply remote Ready;
- a C03f owner does not publish remote Ready;
- a C03f owner does not bypass reachability authority;
- reachability-authority failure must still make the future remote capability unavailable rather than falling back.

Exact sequencing between the reachability-authority runtime owner, real remote transport lifecycle and this remote-session capability owner remains separately gated.

## What the selected owner does not own

`RemoteSessionCapabilityRuntimeOwner` must not own or construct:

- a UDP socket;
- a Quinn endpoint;
- a QUIC connection;
- a QUIC stream;
- a `SessionAuthenticationService`;
- a `WorkspaceDeviceRegistry`;
- a `PolicyEvaluator`;
- a `CapabilityBridge` with borrowed process dependencies;
- a capability dispatcher/backend;
- a reachability-authority provider/client;
- a task/executor/worker;
- retry/backoff/reconnect state;
- remote readiness state.

Those lifetimes remain with their existing or future dedicated owners.

## Future request seam constraint

A later source-materialization checkpoint may expose an Agent-internal operation seam from the owner, but that seam must preserve these constraints:

- the caller cannot replace the `BoundRemoteSession` transport identity on a request;
- current registry/policy context must be supplied or composed through the existing `CapabilityBridge` rather than cached as a successful decision;
- request time remains verifier/caller owned;
- dispatch occurs only after existing bridge authorization;
- existing `RemoteBridgeError` classifications remain authoritative unless a separately reviewed Agent-level error envelope is required.

C03f does not choose a concrete executor/task signature.

## Lifecycle ordering not selected here

C03f deliberately does not select:

- which task accepts a real remote connection;
- when session-auth challenge/proof exchange runs relative to stream creation;
- how `BoundRemoteSession` is transferred into the Agent owner;
- how many concurrent owners are allowed;
- where owners are registered;
- how owners are cancelled/drained;
- shutdown/release ordering;
- retry/reconnect/session refresh;
- remote readiness publication;
- reachability-authority refresh/release sequencing.

Those require separate gated checkpoints because they affect runtime and lifecycle semantics rather than ownership selection alone.

## Source-materialization implication

The next legitimate source checkpoint after C03f is expected to materialize only the selected Agent-owned lifetime wrapper around one `BoundRemoteSession`, analogous to the earlier source-only Agent reachability-authority lifetime owner pattern.

That source checkpoint should remain constructor/ownership focused and should not wire `main.rs`, listener/runtime loops or readiness unless a separate runtime-composition gate explicitly authorizes it.

## Protected boundaries

C03f is documentation-only. Relative to C03e it must change exactly one path: this contract.

It must not change:

- any Rust source file;
- any `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- Agent local runtime/lifecycle source;
- remote transport source;
- reachability authority source;
- workflow files;
- Android application source.

## Negative guarantees

C03f does not:

- activate a remote/public listener;
- bind a socket;
- accept/connect QUIC;
- perform session authentication;
- construct a `BoundRemoteSession` at runtime;
- authorize or dispatch a capability;
- spawn tasks/workers;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- add retry/backoff/reconnect;
- provision credentials;
- execute recovery epochs, PRWF or R1–R4 effects;
- deploy, restart or merge.

## Completion gate

After exact-head CI and Drive evidence closeout:

`C03F_AGENT_REMOTE_SESSION_CAPABILITY_OWNERSHIP_SELECTED`
