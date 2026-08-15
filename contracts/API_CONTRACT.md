# Private Remote Workspace API Contract

Version: `0.2.0`

This document defines typed domain boundaries.

No production network API is activated by Phase 001 or Phase 002.

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

Only the Enrolled state represents normal enrolled-device participation. Phase 002 does not define revocation propagation or recovery semantics.

## Connectivity path

The domain model supports:

- LocalDirect
- InternetDirect
- Relay
- Offline

This is a model only; Phase 002 does not implement path discovery.

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

Phase 002 does not alter operating-system DNS settings.

## Phase 002 identity boundary

Phase 002 represents device public-identity material as opaque, non-empty public bytes.

This representation deliberately does not choose:

- a signature or key-agreement primitive;
- an identity-key library;
- a wire serialization;
- an on-device private-key storage strategy;
- identity-key rotation or recovery semantics.

Private device identity material is not part of the control-plane contract.

A DeviceIdentityBinding associates:

- WorkspaceId;
- UserId;
- DeviceId;
- opaque public identity material;
- DeviceLifecycle.

The UserId is a logical domain reference and does not imply that account authentication has been selected or implemented.

## Phase 002 enrollment boundary

An EnrollmentRequest contains:

- EnrollmentId;
- WorkspaceId;
- UserId;
- DeviceId;
- opaque public identity material.

Enrollment request state is:

- Pending;
- Approved;
- Rejected.

Approved and Rejected are terminal within this typed Phase 002 state model. The concrete approval actor, authentication mechanism, enrollment handshake, trust bootstrap, and persistence model remain deferred.

## Phase 002 revocation boundary

A DeviceRevocation identifies a WorkspaceId and DeviceId to be marked revoked.

Phase 002 does not define:

- propagation timing;
- stale or offline device behavior;
- persistence;
- acknowledgement;
- retry or idempotency semantics.

## Phase 002 control-plane action boundary

The `prw-control-plane` crate defines transport-agnostic typed actions:

- SubmitEnrollment;
- DecideEnrollment;
- RevokeDevice.

These are domain contracts only. They do not authorize or implement an HTTP server, RPC server, public listener, database, authentication service, or deployment.

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
- deployments.
