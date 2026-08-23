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

C03c intentionally materialized one complete PRWM frame per QUIC send direction. C03d therefore uses one bidirectional QUIC stream per challenge/proof exchange:

- server send direction: exactly one challenge frame, then finish;
- client send direction: exactly one proof frame, then finish.

No extra ACK, lease, retry, reconnect or multi-frame stream semantics are added here.

## Required real validation

C03d must prove on real loopback UDP/QUIC/mTLS sockets that:

1. C03c still revalidates expected peer `TransportIdentity` before the stream is admitted;
2. server creates the challenge with existing `SessionAuthenticationService`;
3. challenge crosses a real QUIC stream in PRWM `SessionAuthentication` framing;
4. client decodes and rehydrates the typed challenge against the expected enrolled device binding;
5. client signs through the existing Phase 128 Ubuntu device-identity signer;
6. proof crosses the reverse QUIC stream direction using the same PRWM request identifier;
7. server decodes and submits the proof to the existing `SessionAuthenticationService`;
8. successful verification returns the exact bound `AuthenticatedDeviceSession`;
9. malformed magic/version/flags/kind/truncation/trailing data, invalid session identifiers, invalid signature bounds/profile, wrong outer PRWM kind and correlation mismatch fail closed before authentication completion.

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
- only the dependency declarations required for the adapter/test boundary;
- one focused real-socket integration test.

No Agent `main.rs`, Android application source, workflow or production runtime activation change is authorized.

## Completion gate

After exact-head CI and Drive evidence closeout:

`C03D_LOGICAL_SESSION_AUTH_WIRE_SOURCE_MATERIALIZED`
