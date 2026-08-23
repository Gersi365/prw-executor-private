# Private Remote Workspace — Phase 152 C03e-D Authenticated Peer Handoff Source Materialization Staging

Status: source/materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03e-c-authority-gated-agent-remote-endpoint-owner-source-materialization-staging`
Exact predecessor head: `857097a381d1d146a5036f19e77d1d138e778b23`
Exact predecessor tree: `38136161e4987c82dbddd53e20a66ab1defb9148`
Predecessor gate: `C03E_C_AUTHORITY_GATED_AGENT_REMOTE_ENDPOINT_OWNER_SOURCE_MATERIALIZED`

## Purpose

C03e-D materializes the next narrow handoff after C03e-C: one authority-gated Agent endpoint may accept exactly one real QUIC/TLS peer only through the existing C03c authenticated transport primitive and return an opaque bridge-owned peer handle whose `TransportIdentity` has already been revalidated by the lower transport.

This checkpoint does not execute logical-session challenge/proof authentication, create a `RemoteSessionLease`, construct a C03e `BoundRemoteSession`, materialize the C03f-selected Agent session-capability owner, dispatch capabilities, spawn tasks, or publish remote readiness.

## Existing authoritative primitives

C03e-D reuses without semantic duplication:

- C03c `MeshQuicEndpoint::accept_authenticated(expected_peer)` for real UDP/QUIC/TLS1.3 mTLS acceptance, ALPN validation and exact certificate-derived `TransportIdentity` revalidation;
- C03c `MeshQuicConnection::accept_control_stream()` for one bounded peer-initiated bidirectional PRWM stream;
- C03d `session_auth_wire` for the later logical-session challenge/proof wire exchange;
- C03e `BoundRemoteSession` for the later transport/session lease binding;
- C03f ownership selection for the later Agent-owned `RemoteSessionCapabilityRuntimeOwner`.

No authentication or authorization rule is copied into this checkpoint.

## Selected bridge handoff

`RemoteServerTransportRuntime` gains one async peer-accept operation that requires an explicit expected `TransportIdentity` and delegates directly to the existing C03c authenticated accept primitive.

Successful acceptance returns one bridge-owned `AuthenticatedRemotePeerConnection`.

The peer handle:

- has no public constructor;
- owns the exact validated C03c `MeshQuicConnection`;
- exposes the already-revalidated `TransportIdentity`;
- may accept one peer-initiated bounded control stream through the existing C03c stream primitive;
- may explicitly close the connection;
- does not expose the raw Quinn connection or endpoint.

The source composition is:

`C03e-C AgentRemoteTransportRuntime`
→ `RemoteServerTransportRuntime`
→ `MeshQuicEndpoint::accept_authenticated(expected TransportIdentity)`
→ `AuthenticatedRemotePeerConnection`
→ later C03d logical-session wire checkpoint.

## Agent handoff

`AgentRemoteTransportRuntime` gains one async accepted-peer operation.

It:

- remains reachable only after successful C02f reachability-authority admission and C03e-C endpoint construction;
- requires an explicit expected `TransportIdentity`;
- delegates to the bridge-owned authenticated accept method;
- returns the bridge-owned opaque accepted-peer handle;
- maps transport failure into one narrow Agent peer-accept error classification;
- does not release, replace or duplicate the retained `ReachabilityAuthorityRuntimeOwner`.

The existing endpoint owner therefore remains the process-level lifetime root while individual accepted peer handles remain separately owned values.

## Security invariants

C03e-D must preserve all of the following:

1. No accepted peer exists unless C03c mTLS, ALPN and expected `TransportIdentity` validation succeeds.
2. Caller-supplied expected transport identity is used only as the lower-transport expectation; it is not a logical `DeviceId` or authorization grant.
3. Accepted peer ownership does not imply a valid logical PRW session.
4. Accepted peer ownership does not imply capability authorization.
5. No current-registry or policy result is cached here.
6. No raw Quinn `Connection` or `Endpoint` is exposed through the new bridge API.
7. The C03e-C reachability-authority owner remains retained for the endpoint lifetime.

## Relationship to C03f

C03f remains a separately closed sibling selection from C03e. C03e-D does not claim to contain the C03f branch commit and performs no rebase or merge.

This checkpoint supplies the missing lower transport handoff required before a later integration checkpoint can execute C03d logical-session authentication, construct C03e `BoundRemoteSession`, and transfer that binding into the C03f-selected Agent owner.

## Expected source mutation

Relative to exact C03e-C, exactly three paths are expected:

1. this contract;
2. `crates/prw-remote-bridge/src/remote_server_transport_runtime.rs`;
3. `crates/prw-agent/src/remote_transport_runtime.rs`.

No new module export, dependency or lockfile mutation is required.

## Protected boundaries

C03e-D must not change:

- any `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- existing local Linux runtime/readiness modules;
- C02f reachability authority semantics;
- C03d session-auth wire format;
- C03e `BoundRemoteSession` semantics;
- C03f contract;
- workflows;
- Android application source.

## Focused validation

Source validation must prove at minimum:

- the bridge accept signature requires an exact `TransportIdentity`;
- the returned accepted-peer type has no public constructor and exposes only validated identity, bounded control-stream acceptance and explicit close;
- the Agent accepted-peer signature requires an exact `TransportIdentity` and returns only the bridge-owned peer handle;
- failure does not remove or replace the existing endpoint-level reachability authority owner;
- manifests and lockfiles remain byte-stable;
- canonical Rust workspace validation and Android validation pass on the exact final source head.

C03c remains the canonical real-socket and real mTLS peer-validation proof. C03e-D composes that proven primitive behind the C03e-C Agent endpoint owner rather than duplicating its integration fixture.

## Negative guarantees

C03e-D does not:

- wire `main.rs`;
- spawn a listener loop, task, worker or executor;
- infer a peer identity from IP address;
- execute C03d challenge/proof authentication;
- create `AuthenticatedDeviceSession`, `RemoteSessionLease` or `BoundRemoteSession`;
- materialize `RemoteSessionCapabilityRuntimeOwner`;
- authorize or dispatch a PRWC capability;
- publish remote readiness or alter local `Ready`;
- run ICE/STUN/TURN or relay;
- add retry/backoff/reconnect/session refresh;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- execute PRWF, recovery epochs or R1–R4 effects;
- deploy, restart, rebase or merge.

## Completion gate

After exact-head canonical Rust/Android validation and Drive closeout:

`C03E_D_AUTHENTICATED_PEER_HANDOFF_SOURCE_MATERIALIZED`
