# Private Remote Workspace Remote Device Session Authentication Contract

Version: `0.1.0`

Status: Phase 128 implementation lock

## Purpose

Phase 128 establishes a bounded, replay-resistant authenticated device-session identity after device enrollment. It proves that the holder of the enrolled device private key is participating in one fresh server-issued session challenge and binds that proof to the immutable enrolled workspace/user/device/public-identity snapshot.

Authentication and authorization remain separate. Completing this handshake creates an authenticated session identity only; it does not grant file, terminal, forwarding, networking, DNS, administrative, or workspace-management capabilities.

## Preconditions

A session challenge may be created only from an immutable `DeviceIdentityBinding` whose lifecycle is exactly `DeviceLifecycle::Enrolled`.

`PendingEnrollment` and `Revoked` bindings fail closed before challenge creation.

The production control plane must re-check the current durable registry/revocation state when durable registry semantics exist. An old in-memory enrolled snapshot must never override a later revocation.

## Cryptographic profile

Phase 128 reuses the locked device-identity profile:

- algorithm: ECDSA P-256 with SHA-256;
- public identity: canonical X.509 SubjectPublicKeyInfo DER;
- signature encoding: ASN.1 DER;
- private key remains inside the existing typed signer/custody boundary;
- no generic arbitrary-message signing API may be added.

The exact session-authentication domain separator is:

```text
PRW\0DeviceSessionAuthentication\0
```

The exact canonical message version is unsigned big-endian `u16` value `1`.

## Challenge

The verifier generates exactly 32 cryptographically secure random bytes for each challenge.

A challenge is bound to exactly one strongly typed `SessionId` and contains verifier-owned:

- `SessionId`;
- 32-byte nonce;
- issue time in Unix seconds;
- expiry time in Unix seconds.

Challenge lifetime must be greater than zero and at most 300 seconds.

Verifier time before issue time is rejected. Verifier time at or after expiry is rejected.

## Canonical signed message

The signer and verifier independently reconstruct the canonical message from trusted typed state. Client-supplied duplicate binding fields are not trusted.

The encoded fields, in exact order, are:

1. domain separator bytes;
2. big-endian `u16` message version;
3. length-prefixed UTF-8 `SessionId`;
4. length-prefixed UTF-8 `WorkspaceId`;
5. length-prefixed UTF-8 `UserId`;
6. length-prefixed UTF-8 `DeviceId`;
7. big-endian `u16` device-identity algorithm code;
8. big-endian `u16` public-key encoding code;
9. length-prefixed public-identity bytes;
10. exact 32-byte server nonce.

Each variable-length field uses an unsigned big-endian `u32` byte-length prefix immediately followed by the bytes.

Challenge timestamps are deliberately not part of the signed bytes. They are server-owned freshness/replay policy validated before signature verification.

## Bounds

Phase 128 locks these initial limits:

- each identifier: 1 through 1024 UTF-8 bytes;
- public identity: 1 through 256 bytes;
- nonce: exactly 32 bytes;
- challenge lifetime: 1 through 300 seconds;
- canonical message length: checked arithmetic only and bounded by the exact derived maximum for the locked fields.

Any unsupported algorithm, unsupported public-key encoding, invalid length, checked-arithmetic overflow, or oversized message fails closed before cryptographic verification.

## Proof submission

The device submits only:

- `SessionId`;
- challenge nonce;
- typed device-identity signature.

The server-owned challenge state validates before crypto:

- challenge is not already consumed;
- verifier time is within the challenge window;
- submitted `SessionId` matches the pending state;
- submitted nonce exactly matches the active server challenge.

The production device-identity verifier then verifies the DER signature over the canonical message using the bound enrolled public identity.

Only after successful signature verification may the challenge be consumed.

A successful challenge is single-use. Replay fails closed.

## Authenticated session identity

Successful verification may produce an immutable authenticated-session identity containing exactly:

- `SessionId`;
- `WorkspaceId`;
- `UserId`;
- `DeviceId`;
- bound public identity.

The authenticated session identity does not contain a capability set and must not be interpreted as authorization.

## Session-service state

The initial Phase 128 orchestration may use bounded in-memory pending/completed state for source/disposable validation. Durable session registry, distributed replay state, multi-process coordination, reconnect tickets, refresh tokens, and cross-host failover are deferred.

A future durable implementation must preserve equivalent atomic compare-and-consume semantics.

## Required tests

Tests must prove at least:

- only an `Enrolled` binding may begin authentication;
- fresh challenge nonce is exactly 32 bytes;
- valid typed proof authenticates exactly the bound session/workspace/user/device/public identity;
- proof replay after successful completion is rejected;
- proof for another session is rejected without consuming the correct session challenge;
- changed workspace/user/device/public identity causes signature verification failure;
- wrong nonce is rejected before crypto consumption;
- expired and not-yet-valid challenges fail closed;
- malformed/wrong-profile public identity or signature remains rejected through the existing production verifier boundary;
- authenticated session creation grants no capability implicitly.

## Explicitly deferred

Phase 128 does not implement or activate:

- TCP/QUIC/WebSocket/HTTP transport;
- Internet listeners;
- TLS certificate issuance;
- account password/OAuth/passkey login;
- durable device/workspace/session database;
- file transfer;
- terminal/SSH;
- port forwarding;
- NAT traversal;
- mesh networking;
- relay service;
- DNS;
- Android/Desktop UI.

## Production causal boundary

Real PowerCode production session authentication may execute only after:

1. the Phase 126 identity-aware Agent is installed and production device private identity is activated and attested;
2. Phase 127 real production enrollment has completed and the current device registry state is `Enrolled`;
3. the Phase 128 implementation and disposable validation pass permanent CI.

Source/disposable Phase 128 implementation may proceed before those production prerequisites, but it must not claim a production authenticated session.
