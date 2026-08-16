# Private Remote Workspace — Android Enrollment and Device Management Contract

Status: Phase 146 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Parent architecture: `contracts/ANDROID_CLIENT_ARCHITECTURE_DECISION.md`
Parent Android foundation: `contracts/ANDROID_CLIENT_FOUNDATION_CONTRACT.md`
Parent registry authority: `contracts/DEVICE_REGISTRY_WORKSPACE_MEMBERSHIP_CONTRACT.md`
Parent productization roadmap: `contracts/PRODUCTIZATION_ROADMAP_AND_MUTATION_GATES_CONTRACT.md`

## Purpose

Phase 146 adds the first bounded Android enrollment, device list/status, and revocation presentation surfaces on top of the validated Phase 145 Kotlin/Compose, Android Keystore, state-machine, and narrow Rust/JNI foundation.

Phase 146 remains a non-production functional slice. It does not enroll a real production account/device, activate a production control-plane endpoint, persist a production registry, distribute a signed production APK, or activate production remote networking.

## Existing authority that MUST be reused

Phase 146 does not create a second enrollment, registry, identity, or capability model.

The existing Rust workspace remains authoritative for:

- `EnrollmentRequest`, `EnrollmentProofNonce`, `EnrollmentProofOfPossession`, and canonical enrollment proof message encoding;
- P-256/SHA-256 device-identity signature verification;
- `DeviceLifecycle::{PendingEnrollment, Enrolled, Revoked}`;
- immutable `WorkspaceId` / `UserId` / `DeviceId` identity binding;
- terminal device revocation semantics;
- separation of workspace role metadata from capability authorization.

Android is a typed client/presentation adapter only.

## Enrollment proof boundary

Phase 146 extends the existing narrow Android native adapter with enrollment-specific functions only.

The Kotlin/Rust boundary may expose:

1. construction of the exact canonical enrollment proof-of-possession message from a bounded local JNI envelope;
2. verification of a DER P-256/SHA-256 device-identity signature over that canonical message.

The JNI envelope is an Android-local adapter format only. It MUST NOT become a new PRW network/wire protocol.

The envelope carries exactly the fields required to reconstruct the existing Rust `EnrollmentRequest` plus the existing 32-byte enrollment challenge nonce:

- enrollment identifier;
- workspace identifier;
- user identifier;
- device identifier;
- public device-identity SPKI;
- 32-byte challenge nonce.

The native adapter MUST call the existing `encode_enrollment_proof_message` and existing device-identity verifier. It MUST NOT duplicate their canonical encoding or cryptographic rules.

Existing bounds remain authoritative:

- identifier: maximum 1024 UTF-8 bytes each;
- public identity SPKI: maximum 256 bytes;
- nonce: exactly 32 bytes;
- canonical proof message: maximum 4442 bytes;
- DER signature input: bounded to the existing Android adapter signature limit.

Malformed, oversized, unsupported, or cryptographically invalid inputs fail closed with bounded adapter outcomes.

## Android Keystore signing boundary

`AndroidKeyCustody` may add a typed enrollment-proof signing method parallel to the existing typed session-proof method.

Requirements:

- it signs only canonical enrollment proof bytes supplied by the native/core boundary;
- the same non-exportable `prw.device-identity.v1` P-256 key is used;
- `PrivateKey.encoded` must remain null;
- no generic arbitrary-message signing API is introduced;
- no plaintext/file/exportable private-key fallback is introduced;
- StrongBox request and ordinary AndroidKeyStore fallback behavior remain unchanged.

Transport identity remains separate and is not rotated or reused as device identity.

## Enrollment presentation state

Phase 146 may represent these client-side presentation states:

- `NotReady` — local device identity/native bridge is not ready;
- `Ready` — local device identity is ready for a typed enrollment challenge;
- `ProofValidated` — a disposable/local typed enrollment proof has been constructed, signed, and verified;
- `Error` — the local enrollment proof path failed closed.

`ProofValidated` is NOT equivalent to an authoritative server-side enrollment decision and MUST NOT change a device lifecycle to `Enrolled` by itself.

No local UI state may claim a production account/device enrollment exists unless an authoritative future control-plane result has actually been received and validated.

## Device list/status projection

Phase 146 introduces a bounded immutable Android presentation snapshot for devices.

Each device snapshot contains only:

- `DeviceId`;
- authoritative `DeviceLifecycle` mapped exactly to `PendingEnrollment`, `Enrolled`, or `Revoked`.

The initial non-production implementation may consume injected/disposable authoritative snapshots for tests and UI demonstration. It does not invent durable registry persistence or a production fetch endpoint.

The Android layer MUST NOT add security-semantic statuses such as `Trusted`, `Admin`, `Authorized`, or `Online` unless a later authoritative contract defines them.

A presentation label never grants a capability.

## Revocation intent boundary

The Android UI may emit a typed revocation intent only for an authoritative snapshot whose lifecycle is exactly `Enrolled`.

Rules:

- a local revocation request does not mutate the displayed authoritative lifecycle to `Revoked`;
- the request enters a bounded pending-intent state only;
- only a later authoritative device snapshot may change displayed lifecycle to `Revoked`;
- `PendingEnrollment` and `Revoked` devices reject a new revocation intent;
- repeated pending revocation for the same device is rejected or treated as the same pending intent without creating a second authority event;
- a snapshot that becomes `Revoked` clears any matching pending intent;
- device identifiers are never rebound or rewritten.

Phase 146 does not send this intent to a production control plane.

## UI slice

The Phase 146 Compose surface may display:

- Phase 145 connection/bootstrap state;
- local enrollment-proof readiness/result;
- a bounded device list using authoritative/injected snapshots;
- exact lifecycle labels;
- a typed revoke control only when the selected device is currently `Enrolled`;
- pending-revocation indication that is visually distinct from authoritative `Revoked` state.

Compose emits typed intents only. ViewModel/controller logic owns state transitions.

## Dependency boundary

Phase 146 adds no new Android, JNI, cryptographic, networking, persistence, analytics, or account-authentication dependency unless a separate dependency probe and justification is first recorded.

The existing Phase 145 dependency/toolchain lock remains in force.

## Required validation

Phase 146 completion requires at least:

1. native adapter format/Clippy/tests remain green;
2. canonical enrollment message construction delegates to existing Rust enrollment proof encoding;
3. a valid disposable typed enrollment proof verifies using the existing device-identity verifier;
4. altered signature, malformed request, invalid identifier, invalid SPKI, wrong nonce length, and oversize input fail closed;
5. Android Keystore enrollment signing remains typed and non-exporting by construction;
6. Kotlin unit tests prove device lifecycle projection uses only the three authoritative lifecycle values;
7. revocation intent is permitted only from `Enrolled`;
8. requesting revocation does not locally forge `Revoked` lifecycle;
9. authoritative `Revoked` snapshot clears matching pending intent;
10. no role-to-capability or UI-status-to-capability mapping is introduced;
11. Android unit tests, `lintDebug`, and debug APK assembly pass;
12. permanent Android CI remains green;
13. root Rust workspace format/Clippy/tests/build remain green;
14. audit evidence records exact source/run identifiers and confirms no production side effect.

## Explicitly deferred

Phase 146 does not implement or activate:

- real account login/authentication;
- production enrollment approval or durable registry persistence;
- production device-list fetch endpoint;
- production revocation propagation;
- production transport/network connection;
- terminal, files/transfers, forwarding, or private DNS UX;
- production Android signing/distribution;
- production Agent replacement/restart;
- firewall/NAT/router/TUN/TAP/route/DNS mutation.

## Phase 147 handoff

Phase 147 may add bounded remote terminal UX and terminal session lifecycle on top of the validated Android foundation and device-selection presentation state. It may not infer terminal capability from a device lifecycle or UI role; the existing authenticated capability bridge remains authoritative.

## Completion classification

Target functional state:

`PHASE_146_FUNCTIONALLY_VALIDATED / TYPED_ANDROID_ENROLLMENT_PROOF / NON_EXPORTABLE_KEYSTORE_DEVICE_SIGNATURE / AUTHORITATIVE_DEVICE_LIFECYCLE_PROJECTION / REVOCATION_INTENT_DOES_NOT_FORGE_AUTHORITY / NO_NEW_DEPENDENCIES / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_147`
