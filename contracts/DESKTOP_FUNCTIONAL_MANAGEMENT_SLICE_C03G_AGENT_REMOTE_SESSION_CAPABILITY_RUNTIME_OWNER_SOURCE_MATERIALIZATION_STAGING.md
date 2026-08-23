# Private Remote Workspace — Phase 152 C03g Agent Remote Session Capability Runtime Owner Source Materialization Staging

Status: source-materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`

## Integration predecessor

- branch: `phase-152-c03e-c-authority-gated-agent-remote-endpoint-owner-source-materialization-staging`
- head: `857097a381d1d146a5036f19e77d1d138e778b23`
- tree: `38136161e4987c82dbddd53e20a66ab1defb9148`
- gate: `C03E_C_AUTHORITY_GATED_AGENT_REMOTE_ENDPOINT_OWNER_SOURCE_MATERIALIZED`

## Selection authority

C03f is a closed documentation-only sibling selection:

- branch: `phase-152-c03f-agent-remote-session-capability-ownership-selection-staging`
- head: `540dd27a8329f17a7b346faa43afd41cae9c1d8a`
- tree: `293dacbe309be4000e30e53f2333f0c15d8784f3`
- gate: `C03F_AGENT_REMOTE_SESSION_CAPABILITY_OWNERSHIP_SELECTED`

C03g does not merge or rebase the C03f sibling. It materializes that selected boundary on the current C03e-C integration line, which is an exact descendant of the C03e remote-session binding source and additionally contains the closed C03e-B/C transport/TLS and authority-gated endpoint-owner materialization.

## Purpose

C03g materializes the first Agent-owned source type that can retain one existing `prw_remote_bridge::remote_session_binding::BoundRemoteSession` for the application-layer lifetime of one admitted remote-session capability context.

This checkpoint is ownership-only. It does not select or materialize the later operation seam, task lifecycle, session registry, concurrency model, shutdown ordering, remote readiness, or runtime wiring.

## Materialized owner

The new Agent source type is:

`RemoteSessionCapabilityRuntimeOwner`

Its exact source contract is:

- consume one existing `BoundRemoteSession` by value;
- retain that binding in one private field;
- perform no network I/O, authentication, registry validation, policy evaluation, capability authorization, or dispatch during construction;
- expose no public or crate-internal binding accessor in C03g;
- expose no `authorize` or `process_request` convenience wrapper in C03g;
- own no socket, QUIC endpoint/connection/stream, dispatcher, policy evaluator, registry, worker, task, retry state, reconnect state, or readiness state.

The constructor is therefore intentionally only:

`RemoteSessionCapabilityRuntimeOwner::new(BoundRemoteSession) -> RemoteSessionCapabilityRuntimeOwner`

## Why the binding remains private

C03f explicitly deferred the Agent-internal operation seam to a later checkpoint.

C03g therefore does not make `BoundRemoteSession` independently replaceable or retrievable by runtime callers. The private field preserves one selected transport-identity snapshot plus one logical session lease as the exact pair created by C03e.

A future operation-seam checkpoint may decide how current Agent-owned registry, policy, dispatcher and request-time context are composed with this owner. That later seam must continue to use the existing `CapabilityBridge` as the per-request authorization authority.

## Authorization remains outside the owner

Holding `RemoteSessionCapabilityRuntimeOwner` is not authorization to execute a capability.

The existing C03e `BoundRemoteSession` semantics remain authoritative:

- current registry state is revalidated through the existing bridge on every request;
- the bound transport identity is supplied internally by `BoundRemoteSession`;
- lease expiry remains effective;
- policy remains current per request;
- dispatch occurs only after bridge authorization succeeds.

C03g neither caches nor manufactures a successful authorization result.

## Reachability and endpoint ordering

C03g does not alter the closed C02f reachability-authority ordering or the C03e-C authority-gated endpoint-owner semantics.

The current source relationships remain:

`ReachabilityAuthorityRuntimeOwner`
→ `AgentRemoteTransportRuntime`
→ real authority-gated QUIC server endpoint

and, independently at the application-session layer:

`authenticated transport identity`
→ `AuthenticatedDeviceSession`
→ `BoundRemoteSession`
→ `RemoteSessionCapabilityRuntimeOwner`.

C03g does not yet compose those two chains into a running accept/auth/session task.

Consequently:

- local Agent `Ready` semantics remain unchanged;
- C03g does not publish remote readiness;
- C03g does not bypass reachability authority;
- C03g does not accept a transport peer;
- C03g does not run session challenge/proof authentication;
- C03g does not create `BoundRemoteSession` at runtime.

## Local IPC separation

`AuthenticatedLocalLinuxConnection` and `AuthenticatedLocalLinuxSession` remain local Unix-domain, kernel-credential, same-effective-UID boundaries.

C03g does not reuse, embed, inherit from, or adapt those local IPC types for remote-session capability ownership.

## Source scope

Relative to exact C03e-C head `857097a381d1d146a5036f19e77d1d138e778b23`, C03g is restricted to exactly these intended paths:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03G_AGENT_REMOTE_SESSION_CAPABILITY_RUNTIME_OWNER_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-agent/src/lib.rs`
3. `crates/prw-agent/src/remote_session_capability_runtime.rs`

The `lib.rs` change is only the module export required to compile the new source.

## Protected byte-stable boundaries

C03g must not modify:

- any `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- existing local Linux runtime/readiness source;
- `crates/prw-agent/src/remote_transport_runtime.rs`;
- reachability-authority source;
- remote transport implementation source;
- C03d session-auth wire source;
- C03e `BoundRemoteSession` source;
- workflow files;
- Android application source;
- packaging, deployment, systemd, DNS, firewall, routing, TUN/TAP, or credential provisioning source.

## Validation expectation

The final exact-head candidate should be validated by the repository's canonical push/PR validation surfaces without manually dispatching unrelated disposable workflows.

Expected claims are made only from observed exact-head results. Skipped workflows remain skipped and are not counted as PASS evidence.

## Negative guarantees

C03g does not:

- merge or rebase C03f into C03e-C;
- wire `main.rs`;
- bind or accept a socket;
- open or operate a QUIC connection/stream;
- run logical session authentication;
- create or refresh a remote session lease;
- authorize or dispatch a PRWC capability request;
- expose a request-selected transport identity;
- expose a request-selected registry/policy/dispatcher;
- spawn tasks/workers;
- select concurrency limits;
- select cancellation/drain ordering;
- add retry/backoff/reconnect/session refresh;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- provision or rotate credentials;
- initialize PRWF, execute recovery epochs, or activate R1–R4 effects;
- deploy, restart, or merge.

## Completion gate

After exact-scope verification, exact-head validation, immutable Drive audit publication, and append-only rolling status closeout:

`C03G_AGENT_REMOTE_SESSION_CAPABILITY_RUNTIME_OWNER_SOURCE_MATERIALIZED`
