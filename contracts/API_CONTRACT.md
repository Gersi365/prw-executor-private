# Private Remote Workspace API Contract

Version: `0.3.0`

This document defines typed domain boundaries.

No production network API is activated by Phase 001, Phase 002, or Phase 003.

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

Only the Enrolled state represents normal enrolled-device participation. Phase 003 does not define revocation propagation or recovery semantics.

## Connectivity path

The domain model supports:

- LocalDirect
- InternetDirect
- Relay
- Offline

This is a model only; Phase 003 does not implement path discovery.

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

Phase 003 does not alter operating-system DNS settings.

## Device identity algorithm

Phase 003 selects the first device-identity signature algorithm identifier:

- EcdsaP256Sha256

`EcdsaP256Sha256` means ECDSA over NIST P-256 using SHA-256 for device-identity signatures.

This is a primitive-level contract decision only. Phase 003 deliberately does not choose:

- the concrete cryptographic library or operating-system provider;
- private-key storage backend;
- public-key wire encoding;
- signature wire encoding;
- key attestation format;
- identity-key rotation or recovery protocol.

Device identity remains separate from transport identity. The selected device-identity signature primitive must not be reused implicitly as the future mesh transport-key scheme.

## Public identity material

A `PublicIdentityMaterial` value contains:

- an explicit `DeviceIdentityAlgorithm`;
- non-empty opaque public bytes.

The algorithm must be explicit and must not be inferred from byte length, prefix, or another implicit property.

Private device identity material is not part of the control-plane contract.

## Identity binding

A DeviceIdentityBinding associates:

- WorkspaceId;
- UserId;
- DeviceId;
- explicit device-identity algorithm plus opaque public bytes;
- DeviceLifecycle.

The UserId is a logical domain reference and does not imply that account authentication has been selected or implemented.

## Enrollment boundary

An EnrollmentRequest contains:

- EnrollmentId;
- WorkspaceId;
- UserId;
- DeviceId;
- explicit device-identity algorithm plus opaque public identity bytes.

Enrollment request state is:

- Pending;
- Approved;
- Rejected.

Approved and Rejected are terminal within this typed state model. The concrete approval actor, authentication mechanism, enrollment handshake, trust bootstrap, and persistence model remain deferred.

## Revocation boundary

A DeviceRevocation identifies a WorkspaceId and DeviceId to be marked revoked.

Phase 003 does not define:

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
