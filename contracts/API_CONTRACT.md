# Private Remote Workspace API Contract

Version: `0.4.0`

This document defines typed domain boundaries.

No production network API is activated by Phase 001 through Phase 004.

## Identifier types

The implementation distinguishes:

- WorkspaceId
- UserId
- DeviceId
- TransferId
- SessionId
- EnrollmentId

Identifiers must not be treated as interchangeable raw strings in domain code.

## Device state

Initial lifecycle states:

- PendingEnrollment
- Enrolled
- Revoked

Only the Enrolled state represents normal enrolled-device participation. Phase 004 does not define revocation propagation or recovery semantics.

## Connectivity path

The domain model supports:

- LocalDirect
- InternetDirect
- Relay
- Offline

This is a model only; Phase 004 does not implement path discovery.

## File operations

Initial typed file-operation intents:

- List
- Stat
- Read
- Write
- Copy
- Move
- Rename
- CreateDirectory
- Delete

## Transfer state

Initial states:

- Queued
- Running
- Paused
- Verifying
- Completed
- Failed
- Cancelled

A cross-device move must not be represented as completed until destination verification and finalization have succeeded.

## Private DNS

Private DNS configuration is optional.

The architecture recognizes:

- disabled DNS integration;
- device-name resolution;
- custom resolver configuration;
- split-domain configuration.

Phase 004 does not alter operating-system DNS settings.

## Device identity algorithm

The initial device-identity signature algorithm identifier is:

- EcdsaP256Sha256

`EcdsaP256Sha256` means ECDSA over NIST P-256 using SHA-256 for device-identity signatures.

Device identity remains separate from transport identity. The selected device-identity signature primitive must not be reused implicitly as the future mesh transport-key scheme.

## Device identity public-key encoding

Phase 004 selects the initial serialized public-key encoding:

- `SubjectPublicKeyInfoDer`

`SubjectPublicKeyInfoDer` means DER-encoded X.509 `SubjectPublicKeyInfo` using the RFC 5480 ECC profile:

- algorithm identifier: `id-ecPublicKey`;
- named-curve parameters: `secp256r1`, also known as NIST P-256.

The encoding identifier is explicit. It must not be inferred from byte length, prefix, algorithm identifier alone, or platform origin.

Phase 004 does not yet perform ASN.1 parsing or cryptographic public-key validation. A future cryptographic-provider boundary must reject malformed DER, wrong algorithm identifiers, wrong curve parameters, invalid EC points, and other structurally or cryptographically invalid key material before trust is granted.

## Device identity signature encoding

Phase 004 selects the initial serialized device-identity signature encoding:

- `EcdsaSigValueDer`

`EcdsaSigValueDer` means DER encoding of the RFC 3279 ASN.1 structure:

`ECDSA-Sig-Value ::= SEQUENCE { r INTEGER, s INTEGER }`

The signature encoding identifier is explicit and travels separately from the algorithm identifier.

Phase 004 does not yet sign, verify, parse, normalize, or canonicalize ECDSA signatures. Signatures must not be treated as stable object identifiers.

## Public identity material

A `PublicIdentityMaterial` value contains:

- an explicit `DeviceIdentityAlgorithm`;
- an explicit `DeviceIdentityPublicKeyEncoding`;
- non-empty public bytes.

For the initial profile the expected pair is:

- `EcdsaP256Sha256`;
- `SubjectPublicKeyInfoDer`.

The type records the declared algorithm and encoding but Phase 004 does not itself parse the DER payload.

Private device identity material is not part of the control-plane contract.

## Device identity signature material

A `DeviceIdentitySignature` value contains:

- an explicit `DeviceIdentityAlgorithm`;
- an explicit `DeviceIdentitySignatureEncoding`;
- non-empty signature bytes.

For the initial profile the expected pair is:

- `EcdsaP256Sha256`;
- `EcdsaSigValueDer`.

This is a serialization contract only. No signing or verification backend is implemented in Phase 004.

## Identity binding

A DeviceIdentityBinding associates:

- WorkspaceId;
- UserId;
- DeviceId;
- explicit device-identity algorithm;
- explicit public-key encoding;
- public identity bytes;
- DeviceLifecycle.

The UserId is a logical domain reference and does not imply that account authentication has been selected or implemented.

## Enrollment boundary

An EnrollmentRequest contains:

- EnrollmentId;
- WorkspaceId;
- UserId;
- DeviceId;
- explicit device-identity algorithm;
- explicit public-key encoding;
- public identity bytes.

Enrollment request state is:

- Pending;
- Approved;
- Rejected.

Approved and Rejected are terminal within this typed state model. The concrete approval actor, authentication mechanism, enrollment handshake, trust bootstrap, persistence model, and cryptographic proof-of-possession message remain deferred.

## Revocation boundary

A DeviceRevocation identifies a WorkspaceId and DeviceId to be marked revoked.

Phase 004 does not define:

- propagation timing;
- stale or offline device behavior;
- persistence;
- acknowledgement;
- retry or idempotency semantics.

## Control-plane action boundary

The `prw-control-plane` crate defines transport-agnostic typed actions:

- SubmitEnrollment;
- DecideEnrollment;
- RevokeDevice.

These are domain contracts only. They do not authorize or implement an HTTP server, RPC server, public listener, database, authentication service, cryptographic key store, or deployment.

## Explicit deferrals after Phase 004

Phase 004 deliberately does not choose or implement:

- the concrete Rust cryptographic library or operating-system provider;
- Android Keystore implementation details;
- Ubuntu private-key storage backend;
- private-key creation or persistence;
- strict ASN.1 parser/provider boundary;
- ECDSA signing or verification;
- proof-of-possession message semantics;
- key attestation;
- identity-key rotation or recovery protocol;
- transport-key cryptography;
- control-plane wire protocol.

## Forbidden interpretation

The typed domain definitions in this phase are not authorization to add:

- arbitrary remote shell transport;
- production SSH listeners;
- public network listeners;
- relay servers;
- DNS mutation;
- privileged TUN configuration;
- concrete enrollment networking;
- account authentication;
- database migrations;
- private-key persistence;
- deployments.
