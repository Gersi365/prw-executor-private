# Private Remote Workspace — Phase 152 C03e-J Remote Session Capability Owner Source Materialization Staging

Status: source-materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-i-post-auth-session-lifecycle-ownership-selection-staging`
- head: `67f8b331ba1fa42fa55b84ad4cfcab03a3aded9a`
- tree: `775e6e55c810e2b72b2f319336a6f325d940bcc5`
- gate: `C03E_I_POST_AUTH_REMOTE_SESSION_LIFECYCLE_OWNERSHIP_SELECTED`

## Selection authority

The ownership shape being materialized was already selected by the separately closed C03f checkpoint:

- C03f branch: `phase-152-c03f-agent-remote-session-capability-ownership-selection-staging`
- C03f head: `540dd27a8329f17a7b346faa43afd41cae9c1d8a`
- C03f tree: `293dacbe309be4000e30e53f2333f0c15d8784f3`
- C03f gate: `C03F_AGENT_REMOTE_SESSION_CAPABILITY_OWNERSHIP_SELECTED`

C03e-J does not merge, rebase or rewrite the C03f sibling. It materializes the already-selected owner on the current canonical post-authentication lineage after closed C03e-H and C03e-I.

## Relationship to superseded PR #121

A prior non-canonical attempt materialized the same constructor-only wrapper on an older C03e-C lineage before the authoritative rolling tail was re-read. PR #121 is explicitly marked `SUPERSEDED / NON-CANONICAL. DO NOT MERGE` and has no Drive completion gate.

C03e-J does not cherry-pick, merge or rebase that branch. The prior source was used only as implementation evidence that the already-selected C03f wrapper shape is minimal and compiler-valid in isolation. C03e-J re-materializes the boundary on the authoritative current lineage with current comments, current contract and new exact-head validation.

## Purpose

C03e-J materializes the Agent-owned lifetime wrapper selected by C03f and required by C03e-I before the live peer can later be composed into one outer authenticated remote-session lifecycle owner.

This checkpoint is constructor/ownership only. It does not create a `BoundRemoteSession`, own a live peer, execute capability requests, spawn a session task, publish readiness or wire the Agent binary.

## Materialized owner

The source type is:

`RemoteSessionCapabilityRuntimeOwner`

Its exact contract is:

- consume exactly one existing `prw_remote_bridge::remote_session_binding::BoundRemoteSession` by value;
- retain that value in one private field;
- expose only a by-value constructor in C03e-J;
- expose no public or crate-internal `BoundRemoteSession` accessor in this checkpoint;
- expose no `authorize` or `process_request` wrapper in this checkpoint;
- perform no I/O, authentication, registry lookup, policy evaluation, capability authorization or dispatch during construction.

Exact constructor shape:

`RemoteSessionCapabilityRuntimeOwner::new(BoundRemoteSession) -> RemoteSessionCapabilityRuntimeOwner`

## Why the retained binding remains private

C03f explicitly deferred the Agent-internal capability operation seam. C03e-I likewise requires the capability owner to be transferable into a future outer connected-session owner without making the binding independently replaceable.

Keeping the field private preserves the exact C03e pair:

- immutable authenticated transport-identity snapshot; and
- verifier-owned bounded `RemoteSessionLease` containing the authenticated logical session.

A future operation checkpoint may compose current registry/policy/dispatcher/request-time context through the existing `CapabilityBridge`. C03e-J does not pre-authorize or cache any decision.

## Relationship to C03e-I outer lifecycle owner

C03e-I selected a future `AuthenticatedRemoteSessionRuntimeOwner` that will own:

1. the live C03e-D `AuthenticatedRemotePeerConnection`; and
2. one C03f/C03e-J `RemoteSessionCapabilityRuntimeOwner`.

C03e-J materializes only item 2.

It intentionally does not own:

- `AuthenticatedRemotePeerConnection`;
- `AgentRemoteTransportRuntime`;
- `ReachabilityAuthorityRuntimeOwner`;
- a QUIC stream;
- a request loop;
- a task/worker/executor;
- session collection/registry state;
- retry/reconnect/session-refresh state;
- remote readiness.

The outer connected-session owner remains a separate source-materialization checkpoint after C03e-J closes.

## Authorization remains dynamic

Possession of `RemoteSessionCapabilityRuntimeOwner` is not capability authorization.

The existing `BoundRemoteSession` and `CapabilityBridge` remain authoritative for each future request:

- remote lease issue/expiry validation;
- current authenticated-session registry state;
- current device membership/lifecycle;
- current transport-identity binding;
- bounded PRWC request decoding;
- exact current policy decision;
- dispatcher admission only after successful authorization.

Registry revocation, transport rotation, lease expiry and policy denial therefore remain effective after owner construction.

## Source scope

Relative to exact C03e-I, C03e-J is restricted to exactly these intended paths:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_J_REMOTE_SESSION_CAPABILITY_OWNER_SOURCE_MATERIALIZATION_STAGING.md`;
2. `crates/prw-agent/src/lib.rs` — one module export only;
3. `crates/prw-agent/src/remote_session_capability_runtime.rs` — constructor-only lifetime wrapper and compile-time shape test.

No manifest or lockfile mutation is required because `prw-agent` already has a direct dependency on `prw-remote-bridge`.

## Protected boundaries

C03e-J must keep byte-stable relative to C03e-I:

- every `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/remote_transport_runtime.rs`;
- `crates/prw-agent/src/remote_session_authentication_transaction.rs`;
- C03d session-auth wire source;
- C03e `BoundRemoteSession` source;
- C03e-F session service source;
- registry/policy/dispatcher source;
- existing local Linux runtime/readiness source;
- workflows;
- Android application source;
- packaging/systemd/host-network source.

## Validation requirements

The final exact head must pass the canonical Rust validation surface and, because Rust source changed, canonical Android validation must also complete successfully on that same exact head before Drive closeout.

Skipped workflows remain skipped and are not counted as PASS evidence.

Any formatter/lint finding must be corrected minimally without widening the selected ownership contract.

## Negative guarantees

C03e-J does not:

- merge/rebase C03f or superseded PR #121;
- create a `RemoteSessionLease` or `BoundRemoteSession` at runtime;
- own or close a live peer;
- accept/open/read/write a QUIC stream;
- run logical session authentication;
- authorize or dispatch a capability;
- expose the retained binding;
- select a request loop, task lifecycle, session registry or concurrency model;
- add retry/backoff/reconnect/session refresh;
- wire `main.rs`;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- initialize PRWF, run recovery epochs or activate R1-R4 effects;
- deploy, restart or merge.

## Completion gate

After exact-scope verification, exact-head Rust/Android validation, immutable Drive audit publication and append-only rolling closeout:

`C03E_J_REMOTE_SESSION_CAPABILITY_RUNTIME_OWNER_SOURCE_MATERIALIZED`
