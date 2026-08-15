# Phase 002 Scope Lock

Status: approved for build-phase implementation

## Purpose

Phase 002 locks the minimum identity, enrollment, and control-plane domain boundaries required to continue implementation without prematurely selecting production networking, cryptography, authentication, persistence, or deployment architecture.

## Locked decisions

### 1. Strong enrollment identity

`EnrollmentId` is a first-class strongly typed identifier alongside WorkspaceId, UserId, DeviceId, TransferId, and SessionId.

### 2. Device public identity boundary

The control-plane domain may carry opaque, non-empty public device-identity bytes.

This is intentionally not a cryptographic algorithm decision. Phase 002 does not define:

- the signing or key-agreement primitive;
- the key library;
- wire serialization;
- private-key storage;
- key rotation;
- key recovery.

Private identity-key material is never represented by the Phase 002 control-plane types.

### 3. Identity binding

A device identity binding keeps WorkspaceId, UserId, DeviceId, public identity material, and DeviceLifecycle distinct.

UserId is a logical domain reference. Account authentication remains a separate future decision.

### 4. Enrollment lifecycle

A typed enrollment request has a stable EnrollmentId and begins in a Pending state.

The only Phase 002 terminal decisions are:

- Approved;
- Rejected.

A terminal decision cannot be replaced by a second decision in the typed lifecycle model.

The concrete approval actor, approval UI, trust-bootstrap protocol, and authentication mechanism remain deferred.

### 5. Revocation boundary

Revocation remains represented by DeviceLifecycle::Revoked and a typed DeviceRevocation action containing WorkspaceId and DeviceId.

Propagation, stale/offline-device behavior, acknowledgement, retry, idempotency, and persistence semantics remain deferred.

### 6. Control-plane crate boundary

A pure Rust library crate named `prw-control-plane` owns transport-agnostic control-plane domain contracts.

Phase 002 actions are limited to:

- SubmitEnrollment;
- DecideEnrollment;
- RevokeDevice.

The crate starts no listener and performs no I/O or persistence.

## Explicitly deferred

Phase 002 does not select or implement:

- account authentication;
- production HTTP/RPC/API protocol;
- database or persistent storage;
- device-identity cryptographic primitive;
- transport cryptography;
- WireGuard/TUN integration;
- NAT traversal;
- STUN/ICE/TURN;
- relay runtime;
- DNS mutation;
- systemd activation;
- privileged helper IPC;
- SSH or terminal transport;
- production file-transfer protocol;
- Android runtime implementation;
- desktop runtime implementation;
- deployment.

## Validation boundary

Phase 002 implementation must continue to pass the repository Rust validation baseline:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- empty public identity material is rejected;
- an enrollment can receive only one terminal decision;
- strong identity types remain distinct;
- only DeviceLifecycle::Enrolled represents normal participation.
