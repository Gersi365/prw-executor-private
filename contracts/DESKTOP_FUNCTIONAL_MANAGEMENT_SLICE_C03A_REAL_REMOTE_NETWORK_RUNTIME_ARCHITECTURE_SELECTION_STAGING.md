# Phase 152 C03a — Real Remote Network Runtime Architecture Selection Staging

Status: `ARCHITECTURE_SELECTED / REAL_SOCKET_DATA_PLANE / QUIC_V1_TLS13_MTLS / AUTHORITY_GATED_AGENT_REMOTE_CAPABILITY / AUTHENTICATED_DEVICE_SESSION / CURRENT_TRANSPORT_BINDING / DIRECT_THEN_RELAY / REAL_ICE_STUN_DRIVER_REQUIRED / REAL_RELAY_NETWORK_DRIVER_REQUIRED / ANDROID_CLIENT_RUNTIME_REQUIRED / NO_PRODUCTION_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Purpose

C03a selects the bounded architecture required to turn the already-validated PRW networking, session, reachability and capability components into a **real remote network connection runtime**.

The target is not a mock and not an in-memory-only bridge. The completed C03 line must prove real operating-system network sockets, QUIC v1/TLS 1.3 mutual authentication, authenticated PRW device-session admission, current TransportIdentity validation, typed capability request/response, direct path selection, and relay fallback through network I/O.

C03 source/runtime completion is distinct from deployment proof. A physical Internet-to-Internet production proof requires executing the built client and Agent on real hosts/networks and is not manufactured by repository CI.

## Exact prerequisite

C03a derives only from closed C02f-CL:

- branch: `phase-152-c02f-cl-reachability-control-plane-authority-source-completion-staging`;
- head: `b06c2e46f6fdd40f52df287d98bc8e05af37115d`;
- tree: `5d8d30fb41444355091ba5ae65039fa99d2f44f7`;
- gate: `C02F_CL_REACHABILITY_CONTROL_PLANE_AUTHORITY_SOURCE_COMPLETE`.

The authority implementation is not reopened absent new contradictory evidence.

## Existing validated building blocks

The current repository already contains these usable components:

1. `prw-remote-transport`
   - real `UdpSocket` + Quinn endpoint capability;
   - QUIC v1 only;
   - TLS 1.3 only;
   - mandatory mutual TLS;
   - ALPN `prw-mesh/1`;
   - explicit private trust roots;
   - SPKI-SHA256 `TransportIdentity` derivation;
   - exact expected-peer TransportIdentity validation;
   - bounded PRWM request/response framing;
   - real socket loopback validation.
2. `prw-session`
   - fresh challenge creation;
   - typed device proof verification;
   - immutable `AuthenticatedDeviceSession`.
3. `prw-registry`
   - current enrolled device state;
   - current logical-device/TransportIdentity binding.
4. `prw-remote-bridge`
   - bounded `RemoteSessionLease`;
   - current registry and transport revalidation;
   - per-capability policy;
   - typed capability commands and correlated responses.
5. `prw-connectivity`
   - `LocalDirect -> InternetDirect -> Relay -> Offline` selection;
   - dynamic candidate refresh while preserving peer identity.
6. `prw-nat-traversal`
   - standards-based Sans-I/O STUN/ICE state machine.
7. `prw-relay` / `prw-relay-service`
   - fallback-only opaque relay protocol and bounded provider semantics.
8. C02f authority chain
   - real control-plane provider bootstrap;
   - custody;
   - admission;
   - Agent-owned authority lifetime owner.

The missing work is composition and real network driving, not replacement of these foundations.

## Definition of C03 source/runtime completion

The remote-network source/runtime scope is complete only when all of the following are materialized and canonically validated:

1. Ubuntu mesh transport credential custody suitable for the existing QUIC/mTLS profile.
2. Reusable real-socket QUIC endpoint runtime APIs rather than test-only endpoint construction.
3. A bounded PRWM session-authentication wire protocol that carries begin/challenge/proof/admitted transitions without weakening the existing typed proof semantics.
4. Agent remote runtime ownership that can be created only after successful reachability-authority admission.
5. Agent remote listener/session handling over real QUIC streams.
6. Current certificate-derived TransportIdentity revalidation against the authenticated logical device before capability dispatch.
7. `RemoteSessionLease` creation only after successful typed session proof.
8. Typed PRWC capability requests flowing over the established real transport and correlated responses returning over that transport.
9. A real UDP driver for STUN/ICE Sans-I/O datagrams and timeout progression.
10. Direct candidate connection using the selected current direct endpoint.
11. A real network relay driver/service for fallback-only opaque frames when no direct path is selected/reachable.
12. Direct-before-relay orchestration preserving the existing deterministic path policy.
13. A client runtime path that consumes the same protocol stack; Android remains the first user-facing client and must preserve non-exportable platform key custody.
14. End-to-end CI using real operating-system sockets proving the authenticated request/response path and key negative cases.
15. Remote readiness that is separate from existing local Agent `Ready` and is never published merely because local IPC is healthy.

## Agent ownership and authority gate

The Agent is the process-level owner of the remote-capability runtime.

The remote runtime may be constructed only from an admitted `ReachabilityAuthorityRuntimeOwner` or another capability token derived from it without bypassing admission. A public constructor accepting an arbitrary bridge authority or raw provider client is forbidden.

Authority bootstrap failure must leave the existing local Agent surface available under C02f-CH while the remote capability remains unavailable.

Authority admission does not itself create a socket. The remote endpoint transition is a separate fallible step.

## Ubuntu mesh transport credentials

The Ubuntu Agent requires a transport identity distinct from logical `DeviceId` and from the enrollment/device-identity signing key.

The selected custody boundary uses fixed systemd-delivered credentials for:

- private trust root certificate DER;
- Agent mesh leaf certificate DER;
- Agent mesh private key PKCS#8 DER.

The loader must reuse the Phase 122-style custody invariants already used elsewhere: fixed credential names, no caller-selected path, no symlink following, bounded reads, expected owner/mode checks where applicable, and zeroizing private-key storage before ownership is transferred into rustls.

No plaintext fallback path, environment-variable secret value, home-directory key file, or generated replacement identity is permitted.

Certificate-derived `TransportIdentity` is authoritative for the transport key and must match the registry/control-plane binding selected for that Agent identity before remote readiness may be published.

## Real QUIC endpoint runtime

`prw-remote-transport` remains the owner of transport mechanics.

The reusable runtime must expose bounded APIs for:

- binding a server endpoint to an explicit `SocketAddr` using a real UDP socket;
- binding a client endpoint to an explicit local socket address;
- connecting to one explicit selected peer endpoint and expected TransportIdentity;
- accepting one connection and obtaining its authenticated TransportIdentity;
- opening/accepting bounded bidirectional control streams;
- sending/receiving exactly one bounded PRWM `ControlFrame` with finite timeout;
- clean endpoint/connection close.

No DNS discovery is added to the mesh data plane. Candidate endpoints remain explicit IP + port values.

## Session-authentication wire protocol

TLS transport authentication and PRW logical session authentication remain distinct.

A versioned bounded session-authentication payload carried only inside `ControlMessageKind::SessionAuthentication` must support:

1. client begin containing bounded logical session/device identifiers sufficient to resolve the current enrolled binding;
2. server challenge containing the exact fresh typed challenge fields produced by `SessionAuthenticationService`;
3. client proof containing the exact typed `SessionAuthProof` fields/signature;
4. server admitted result containing no capability grant and no secret material.

The codec must be deterministic, bounded, reject trailing data/unknown versions/states, and construct existing typed domain values rather than alternate unchecked representations.

No capability request is accepted before admission completes.

## Current identity binding

For every admitted connection:

- QUIC/rustls provides the authenticated certificate chain;
- PRW derives the peer `TransportIdentity` from leaf SPKI;
- session proof establishes the logical `DeviceId` and authenticated PRW session;
- registry revalidation proves that this `DeviceId` currently owns this exact `TransportIdentity`;
- only then can `CapabilityBridge` authorize typed operations.

A valid mTLS certificate that is stale for the logical device fails closed.

## Direct connectivity runtime

The selected connectivity plan remains authoritative.

The runtime may attempt only a currently selected `LocalDirect` or `InternetDirect` candidate. A candidate is an explicit IP/port endpoint and does not grant identity or capability.

STUN/ICE becomes real only through a narrow UDP driver around the existing Sans-I/O state machine. The driver owns socket send/receive/timers; the protocol crate retains candidate/check semantics.

No firewall, router, route, TUN/TAP or arbitrary host-network mutation is introduced by this line.

## Relay fallback runtime

Relay remains fallback-only.

The existing in-memory relay-service validation is insufficient for C03 completion. A network driver/service must transport the existing opaque relay routing envelope over a real bounded socket protocol while preserving:

- opaque end-to-end payload bytes;
- two-participant route isolation;
- route-token validation;
- queue/frame bounds;
- no application payload parsing by the relay;
- no capability grant by the relay.

The relay must not terminate PRW application mTLS/session authorization semantics in a way that grants it plaintext authority.

## Android client runtime

Android remains the first client runtime.

The existing architecture lock remains authoritative:

- Kotlin owns Android lifecycle/foreground-service orchestration;
- Rust owns PRW protocol/domain logic;
- Android device and transport private keys remain Android Keystore-backed and non-exportable;
- no Android-only alternate wire protocol;
- TLS profile remains QUIC v1/TLS1.3/mTLS/`prw-mesh/1`/explicit roots/no early data;
- process death requires reauthentication.

Because the current disposable Rust helper consumes `PrivateKeyDer`, C03 must not claim Android production-grade transport-key completion until the audited platform-backed TLS signing path is materialized.

## Readiness model

Readiness is split:

- **LocalReady**: existing local IPC surface, unchanged by C03.
- **AuthorityAdmitted**: reachability authority successfully admitted/owned.
- **RemoteTransportReady**: required mesh credentials are valid, authority is admitted, network endpoint is bound and transport identity is current.
- **RemoteSessionAdmitted**: one peer has completed mTLS + logical session proof + current registry binding.

No later state can be inferred from an earlier one.

## Failure semantics

All C03 stages fail closed:

- credential error -> no remote endpoint;
- authority error -> no remote endpoint;
- bind/connect error -> no remote readiness;
- TLS/ALPN error -> no session;
- TransportIdentity mismatch -> close/reject;
- session proof error -> no lease;
- stale/revoked/suspended registry state -> no dispatch;
- policy denial -> no operation;
- malformed/oversized wire input -> close/reject as bounded by the relevant protocol;
- direct-path failure -> relay is considered only through the selected fallback policy, never as an implicit capability bypass.

## CI proof required

The final C03 completion evidence must include real-socket tests proving at minimum:

- UDP QUIC listener/client handshake;
- mandatory mTLS;
- exact peer TransportIdentity;
- session begin/challenge/proof/admission over the network;
- current DeviceId/TransportIdentity binding;
- one authorized typed request and correlated response over QUIC;
- invalid certificate/root/name/transport binding/session proof denied;
- revoked or expired session denied;
- direct path preference;
- real STUN/ICE datagram driving in disposable network fixtures;
- real relay socket fallback fixture and opaque byte preservation;
- no local `Ready` semantic regression.

Where GitHub-hosted CI cannot prove public Internet/NAT topology, that limitation must be reported explicitly rather than relabeled as a pass.

## Production/deployment boundary

C03 authorizes source/runtime materialization and disposable/CI real-socket execution only.

It does not itself authorize:

- installing/replacing/restarting the production Agent;
- provisioning real production transport certificates/keys;
- opening a production LAN/public listener;
- firewall/router/NAT changes;
- persistent production STUN/TURN/relay service activation;
- Play Store/external Android distribution;
- production account/device cutover;
- merge of draft PRs.

A later explicit deployment transaction is required for physical Internet-to-Internet proof on real networks.

## Planned checkpoint sequence

The selected implementation order is:

1. C03b — Ubuntu mesh transport credential custody.
2. C03c — reusable real QUIC endpoint/control-stream runtime.
3. C03d — bounded logical session-auth wire codec.
4. C03e — Agent authority-gated remote runtime + real QUIC session/capability bridge.
5. C03f — real STUN/ICE UDP driver and direct connectivity orchestrator.
6. C03g — real network relay fallback driver/service.
7. C03h — direct-to-relay integrated connectivity runtime.
8. C03i — Android platform-backed transport signing/runtime integration.
9. C03j — end-to-end real-socket completion evidence and source/runtime completion gate.

A checkpoint may be split further if implementation evidence exposes a material lifecycle/security decision; it must not be silently widened.

## Gate

After canonical validation and evidence closeout:

`C03A_REAL_REMOTE_NETWORK_RUNTIME_ARCHITECTURE_SELECTED`
