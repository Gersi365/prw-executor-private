# Private Remote Workspace — Phase 152 C03d Logical Session Authentication Wire Source Materialization Staging

Status: source/materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03c-reusable-real-quic-socket-runtime-source-materialization-staging`
Exact predecessor head: `c9ffcc76d6af5eccdbb3c963dac906f7df892f4f`
Exact predecessor tree: `35f34310cf2f5176d074dcc9554fbcf92011b48c`
Predecessor gate: `C03C_REUSABLE_REAL_QUIC_SOCKET_RUNTIME_SOURCE_MATERIALIZED`

## Purpose

C03d materializes the reusable wire adapter that carries the already-locked Phase 128 enrolled-device session challenge/proof semantics over the real C03c QUIC stream boundary.

This checkpoint does not define a second authentication protocol. The server continues to create and verify challenges through `prw-session::SessionAuthenticationService`; the wire adapter only serializes bounded challenge/proof values into the already-reserved PRWM `ControlMessageKind::SessionAuthentication` envelope.

## Locked layering

The source path is:

`C03c transport-authenticated QUIC connection`
→ `PRWM SessionAuthentication frame`
→ `C03d bounded challenge/proof wire codec`
→ `Phase 128 SessionAuthenticationService`
→ `AuthenticatedDeviceSession`

Transport authentication remains distinct from logical PRW session authentication. A valid mTLS peer certificate or expected `TransportIdentity` does not itself create an `AuthenticatedDeviceSession`.

C03d inherits C03c's already-closed real-kernel UDP/QUIC/mTLS and expected-peer-`TransportIdentity` evidence. C03d does not duplicate the C03c certificate/socket fixture merely to retest that lower transport layer; its focused validation proves the newly introduced PRWS codec and Phase 128 typed-authentication composition. A later composition checkpoint may exercise the entire session exchange over a running real-socket orchestration path without changing these wire semantics.

## PRWS v1 payload

Every C03d payload carried inside PRWM `SessionAuthentication` uses:

- magic: 4 bytes `PRWS`;
- protocol major: `u16 = 1`;
- protocol minor: `u16 = 0`;
- message kind: `u16`;
- flags: `u16 = 0`.

Message kinds:

1. challenge;
2. proof.

The PRWM non-zero request identifier correlates one challenge/proof exchange. The proof must echo the challenge request identifier at the orchestration boundary.

### Challenge body

- length-prefixed UTF-8 `SessionId`, bounded by the existing Phase 128 identifier ceiling;
- exact 32-byte `SessionAuthNonce`;
- verifier-owned `issued_at_unix_seconds` (`u64`);
- verifier-owned `expires_at_unix_seconds` (`u64`).

The client-side adapter rehydrates a typed `SessionAuthChallenge` only through existing `SessionAuthChallengeState::new` using the client's expected enrolled `DeviceIdentityBinding`. That preserves the existing lifetime and enrolled-binding checks without adding a control-plane wire constructor.

### Proof body

- length-prefixed UTF-8 `SessionId`;
- exact 32-byte `SessionAuthNonce`;
- bounded length-prefixed signature bytes.

PRWS v1 implies the already-locked Phase 128 device-signature profile only:

- `DeviceIdentityAlgorithm::EcdsaP256Sha256`;
- `DeviceIdentitySignatureEncoding::EcdsaSigValueDer`.

No algorithm negotiation or bearer credential is introduced.

## QUIC stream rule

C03c intentionally materialized one complete PRWM frame per QUIC send direction. C03d therefore materializes async stream adapters for one bidirectional QUIC stream per challenge/proof exchange:

- server send direction: exactly one challenge frame, then finish;
- client send direction: exactly one proof frame, then finish.

No extra ACK, lease, retry, reconnect or multi-frame stream semantics are added here.

## Required focused validation

C03d must prove that:

1. C03c remains the authoritative real UDP/QUIC/mTLS and expected-peer-`TransportIdentity` lower transport boundary;
2. server creates the challenge with existing `SessionAuthenticationService`;
3. the typed challenge encodes into PRWM `SessionAuthentication` with the selected PRWS v1 payload and a non-zero correlation identifier;
4. client decodes and rehydrates the typed challenge against the expected enrolled device binding;
5. client signs through the existing Phase 128 Ubuntu device-identity signer;
6. the proof encodes/decodes under the same PRWM correlation identifier and locked P-256 DER signature profile;
7. server submits the decoded proof to the existing `SessionAuthenticationService`;
8. successful verification returns the exact bound `AuthenticatedDeviceSession`;
9. malformed magic/version/flags/kind/truncation/trailing data, invalid session identifiers, invalid signature bounds/profile and wrong outer PRWM kind fail closed before authentication completion;
10. the async send/receive adapters compile directly against the already-validated C03c `MeshControlStream` boundary without introducing another transport implementation;
11. no new direct certificate-generation, TLS-runtime or executor dependency is added to `prw-remote-bridge`, so the locked dependency graph remains byte-stable except for the existing `prw-core` dependency moving from test-only to production use.

## Negative guarantees

C03d does not:

- create a `RemoteSessionLease`;
- run current-registry membership revalidation;
- authorize or dispatch a capability;
- publish a remote listener or Agent readiness;
- wire the running Agent lifecycle;
- activate ICE/STUN/TURN/relay;
- add retry/backoff/reconnect/session refresh;
- provision production credentials;
- mutate systemd/firewall/NAT/router/TUN/TAP/routes/DNS;
- execute recovery epochs, PRWF or R1-R4 effects;
- deploy, restart or merge.

## Expected source mutation

C03d should remain bounded to:

- this contract;
- one reusable `prw-remote-bridge` session-auth wire module and module export;
- the minimal `prw-core` dependency-scope move required by the production wire adapter;
- one focused typed challenge/proof composition validation using dependencies already present in the locked graph.

`Cargo.lock` must remain byte-stable. No Agent `main.rs`, Android application source, workflow or production runtime activation change is authorized.

## Completion gate

After exact-head CI and Drive evidence closeout:

`C03D_LOGICAL_SESSION_AUTH_WIRE_SOURCE_MATERIALIZED`
