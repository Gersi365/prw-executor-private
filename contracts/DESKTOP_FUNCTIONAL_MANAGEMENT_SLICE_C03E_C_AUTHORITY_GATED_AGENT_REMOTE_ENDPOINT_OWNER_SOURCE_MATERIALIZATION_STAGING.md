# Private Remote Workspace — Phase 152 C03e-C Authority-Gated Agent Remote Endpoint Owner Source Materialization Staging

Status: source/materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03e-b-mesh-transport-tls-handoff-source-materialization-staging`
Exact predecessor head: `b25283ec86fa6faf9f17450d6cded521f03924b6`
Exact predecessor tree: `25d3330af2d6ad450e305c300093fe28b1958ee4`
Predecessor gate: `C03E_B_MESH_TRANSPORT_TLS_HANDOFF_SOURCE_MATERIALIZED`

## Purpose

C03e-C materializes the first Agent-owned real remote-network runtime object. It binds a real QUIC server endpoint only when the caller already possesses the opaque `ReachabilityAuthorityRuntimeOwner` produced by the closed C02f authority-admission line.

This checkpoint closes the C03a requirement that remote transport construction be authority-gated. It deliberately does not yet accept a peer, run logical session authentication, create a `RemoteSessionLease`, dispatch a capability, or publish remote readiness. Those session-loop semantics remain separately gated after this owner exists.

## Selected composition

The source shape is:

`ReachabilityAuthorityRuntimeOwner`
+
`fixed systemd mesh credentials`
+
`explicit SocketAddr`
→ `AgentRemoteTransportRuntime`
→ bridge-owned `RemoteServerTransportRuntime`
→ C03e-B typed DER helper
→ C03c real `UdpSocket` / Quinn server endpoint.

There is no public constructor that accepts a raw provider client, arbitrary bridge authority, boolean admission flag, or untrusted capability token.

## Bridge-owned server endpoint wrapper

`prw-remote-bridge` gains a narrow `RemoteServerTransportRuntime` wrapper around the existing C03c `MeshQuicEndpoint`.

Its constructor:

- consumes owned root/leaf/private-key DER;
- delegates TLS construction to the C03e-B `build_server_config_from_der(...)` helper;
- binds the real server endpoint only after TLS configuration succeeds;
- exposes only the kernel-selected local address, explicit close, and wait-idle operations in this checkpoint.

It does not accept peers or expose a capability path yet. C03e-D may extend this already-owned endpoint with the separately gated authenticated session loop.

## Agent authority ownership

`AgentRemoteTransportRuntime` owns both:

1. the exact `ReachabilityAuthorityRuntimeOwner` supplied by the caller; and
2. the bound `RemoteServerTransportRuntime`.

The authority owner is therefore retained for the full lifetime of the remote endpoint owner. Construction cannot succeed without consuming an admitted owner.

The current local Agent `Ready` state is not consulted and is not changed.

## Credential transfer

The constructor loads only the fixed C03b systemd credential set and consumes the C03e-B handoff:

`MeshTransportCredentialMaterial::into_transport_tls_der(self)`.

The PKCS#8 key remains in `Zeroizing<Vec<u8>>` until the immediate private composition step. The implementation moves the existing `Vec<u8>` out with `std::mem::take` and passes it directly into the transport-owned DER helper. It does not clone the key and exposes no new public key accessor.

## Bind failure and authority recovery

A failed credential/TLS/socket bind must not silently destroy the already-admitted authority owner.

`AgentRemoteTransportBindFailure` therefore owns:

- the original `ReachabilityAuthorityRuntimeOwner`; and
- one stable `AgentRemoteTransportBindError` classification.

The caller may recover the authority owner by consuming the failure object. No automatic retry is introduced. A failure never creates an `AgentRemoteTransportRuntime` and never publishes remote readiness.

## Error semantics

The stable bind error classes are:

- `Credential(MeshTransportCustodyError)` for fixed credential acquisition failure;
- `Transport(RemoteServerTransportRuntimeError)` for TLS configuration or real endpoint bind failure.

The underlying existing errors are preserved as sources where applicable.

## Protected boundaries

Relative to C03e-B, C03e-C must not change:

- any Cargo manifest;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- existing local Linux runtime/readiness modules;
- C02f authority implementation semantics;
- C03d session-auth wire format;
- C03e remote-session binding semantics;
- workflows;
- Android application source.

## Expected source mutation

Exactly five paths are expected:

1. this contract;
2. `crates/prw-remote-bridge/src/remote_server_transport_runtime.rs`;
3. `crates/prw-remote-bridge/src/root.rs` (one module export);
4. `crates/prw-agent/src/remote_transport_runtime.rs`;
5. `crates/prw-agent/src/lib.rs` (one module export).

No dependency or lockfile mutation is required because `prw-agent` already depends on `prw-reachability-custody` and `prw-remote-bridge`, while `prw-remote-bridge` already depends on `prw-remote-transport`.

## Focused validation

Source validation must prove at minimum:

- the Agent constructor signature requires `ReachabilityAuthorityRuntimeOwner`;
- malformed TLS material fails before a server runtime exists;
- a bind failure retains recoverable authority ownership;
- the bridge wrapper exposes no session/capability authorization shortcut;
- root and Android native locked dependency graphs remain unchanged;
- full Rust workspace validation and Android validation pass on the exact final head.

C03c remains the canonical proof that the underlying endpoint uses real operating-system UDP sockets and Quinn QUIC; C03e-C composes that proven primitive behind the Agent authority gate rather than duplicating transport mechanics.

## Negative guarantees

C03e-C does not:

- wire `main.rs`;
- spawn a listener task or background worker;
- accept a QUIC peer;
- execute challenge/proof authentication;
- create `BoundRemoteSession` or `RemoteSessionLease`;
- process a PRWC capability request;
- publish `RemoteTransportReady` or alter local `Ready`;
- run ICE/STUN/TURN or relay;
- retry/reconnect automatically;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- deploy, restart, merge, initialize PRWF, execute recovery epochs, or activate R1–R4 effects.

## Parent C03e status

C03e-C closes only authority-gated real endpoint ownership. The planned C03e parent remains open until a later sub-checkpoint materializes real QUIC peer acceptance, logical session proof, current identity binding, `BoundRemoteSession`, and typed capability request/response over the accepted connection.

## Completion gate

After exact-head canonical Rust/Android validation and Drive closeout:

`C03E_C_AUTHORITY_GATED_AGENT_REMOTE_ENDPOINT_OWNER_SOURCE_MATERIALIZED`
