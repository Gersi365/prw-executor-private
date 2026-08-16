# Private Remote Workspace — Android Client Foundation Contract

Status: Phase 145 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Input baseline: `56820c39caa8523a7306df4a7e46a2865a9006c7`
Parent architecture: `contracts/ANDROID_CLIENT_ARCHITECTURE_DECISION.md`
Parent authenticated bridge: `contracts/END_TO_END_AUTHENTICATED_CAPABILITY_BRIDGE_CONTRACT.md`

## Purpose

Phase 145 creates the first non-production functional Android application foundation under the Phase 144 architecture lock. It must prove a native Kotlin/Compose shell, non-exportable Android Keystore P-256 custody, a narrow Android/Rust bridge, and a bounded authenticated-connection bootstrap state machine without activating any production PRW endpoint.

Phase 145 remains a development/debug slice. It does not enroll a real production account/device, distribute a signed production APK, activate production STUN/ICE/TURN/relay traffic, mutate host networking, restart/replace the production Agent, or provision production transport credentials.

## Dependency/toolchain gate

Disposable GitHub Actions probe run `31957645210` validated the exact initial Android build/binding stack on Ubuntu 24.04:

- Rust/Cargo 1.97.1;
- Android SDK Platform 36;
- Android Build Tools 36.0.0;
- Android NDK 28.2.13676358;
- `cargo-ndk` 4.1.2;
- `jni` 0.22.4;
- arm64-v8a and x86_64 Android cross-build at API 29;
- Gradle 9.5.0;
- Android Gradle Plugin 9.3.0;
- Kotlin Compose compiler plugin 2.3.21 under AGP built-in Kotlin;
- Compose BOM 2026.06.00;
- `androidx.activity:activity-compose:1.13.0`;
- `androidx.lifecycle:lifecycle-viewmodel-compose:2.10.0`;
- Material 3 from the locked Compose BOM.

The exact scratch JNI Cargo.lock SHA-256 was:

`34bfaab9e331f85e808296b766068f6fa6c7530834003d27190a4fdc8b55c2c6`

The disposable APK SHA-256 was:

`c871ec404c7df4591b8153a96010c8fd7304b0200b6174f47ffe8128c7c0220a`

The probe repository remained byte-clean.

The probe also exposed that the legacy `jni::JNIEnv` alias is deprecated in jni 0.22.4. Authoritative Phase 145 native entrypoints therefore use `jni::EnvUnowned`, not the deprecated alias.

## Repository shape

Phase 145 materializes the Android client under `apps/android`.

The Android native adapter is a standalone Cargo package under:

`apps/android/native`

It is intentionally not added to the root Rust workspace. The root workspace keeps `unsafe_code = "forbid"` for all PRW core crates. Rust 2024 JNI symbol export requires an unsafe attribute such as `#[unsafe(no_mangle)]`; that unavoidable FFI attribute is isolated to the Android adapter package and must not introduce an unsafe block or weaken any root-workspace crate lint.

The adapter may depend on validated PRW crates by path. It must not fork or reimplement their wire formats or security rules.

## Android application baseline

Locked application coordinates:

- package/namespace: `com.privateworkspace.prw`;
- debug application id: `com.privateworkspace.prw`;
- minSdk 29;
- compileSdk 36;
- targetSdk 36;
- versionCode 1;
- versionName `0.1.0-dev`;
- debug only in Phase 145;
- no production signing configuration.

The manifest must default to no backup and must not export any service/provider/receiver. Only the launcher Activity is exported. Phase 145 requests no boot-completed receiver and does not start a foreground connection service automatically.

## Kotlin state model

Phase 145 implements the Phase 144 top-level connection states:

- `Disconnected`;
- `Connecting`;
- `Authenticating`;
- `Connected`;
- `Suspended`;
- `Disconnecting`;
- `Error`.

The application shell exposes an immutable UI state containing at minimum:

- current connection state;
- local identity readiness;
- native bridge readiness;
- authenticated-bootstrap readiness/error;
- no implicit capability grant.

A ViewModel owns presentation state. A connection controller owns state transitions. Compose only observes state and emits typed intents.

No socket is opened by the Activity or composable.

## Android Keystore custody

Phase 145 implements an Android-only `AndroidKeyCustody` boundary.

Two aliases are reserved:

- `prw.device-identity.v1`;
- `prw.transport-identity.v1`.

For each alias:

- provider: `AndroidKeyStore`;
- algorithm: EC;
- curve: `secp256r1`;
- purpose: signing only;
- digest: SHA-256;
- private key must be non-exportable;
- public key SPKI may be returned;
- StrongBox is requested when available on API 28+ and falls back only to ordinary Android Keystore when StrongBox allocation is unavailable;
- failure to use Android Keystore does not fall back to a file/plaintext/exportable key;
- invalidated/unavailable key state fails closed.

The custody class does not expose a generic public arbitrary-message signing API to UI callers. Typed Phase 145 proof methods accept only the canonical PRW enrollment/session proof bytes produced by the native/core boundary and return a DER ECDSA signature. Phase 146 will own real enrollment UX.

## Narrow native bridge

Phase 145 locks exact JNI dependency `jni = 0.22.4` with default features disabled.

The native bridge exports only bounded development functions needed to prove the integration boundary:

1. protocol/version compatibility;
2. validation/round-trip of a bounded Phase 140 PRWM control frame;
3. construction of canonical typed Phase 128 session-authentication proof material from validated identifiers/nonces;
4. validation of a returned DER P-256/SHA-256 signature against supplied public SPKI material;
5. bounded bootstrap-state result categories.

No generic command, arbitrary filesystem path authority, arbitrary destination socket, process execution, raw private-key import/export, or unrestricted signer API is exported.

JNI byte-array/string inputs are size checked before copying/decoding. JNI errors are returned as stable bounded result codes/strings rather than panicking across FFI.

## Authenticated connection bootstrap foundation

Phase 145 does not connect to a production Agent. It proves the bootstrap chain in a disposable/local test boundary:

1. Android Keystore creates/loads the local non-exportable device identity;
2. public SPKI is exposed to the PRW native/core boundary;
3. the core/native boundary constructs canonical typed session-authentication proof bytes using existing PRW identity/session contracts;
4. Android Keystore signs only that canonical typed message;
5. native/core validation verifies the returned signature and produces a bounded authenticated-bootstrap success state;
6. connection controller transitions `Disconnected -> Connecting -> Authenticating -> Connected` only after that validation succeeds;
7. invalid signature/key/session material transitions to `Error` without granting capability;
8. `disconnect` clears the active bootstrap state.

This is an authenticated connection *bootstrap foundation*, not a production network connection. The actual remote endpoint, NAT traversal and relay path are not activated in this phase.

## Build and validation requirements

Phase 145 completion requires:

1. exact Android dependency versions resolve;
2. native adapter builds for arm64-v8a and x86_64 at API 29 using Rust 1.97.1 + cargo-ndk 4.1.2 + NDK 28.2.13676358;
3. no deprecated `jni::JNIEnv` use remains;
4. native adapter contains no unsafe block; only the isolated Rust 2024 symbol-export unsafe attribute is permitted;
5. native adapter tests validate bounded PRWM and typed session-proof behavior on the host where target-independent;
6. Android debug APK assembles and lintDebug passes;
7. APK contains both expected native libraries;
8. Kotlin unit tests prove legal connection-state transitions and fail-closed illegal transitions;
9. Android tests prove Keystore alias/profile code is non-exporting by construction; where emulator/device-backed Keystore execution is available, a disposable generated key must return a public SPKI and `PrivateKey.encoded == null`;
10. no boot receiver, exported service/provider/receiver, production hostname/IP, production credential or release signing config exists;
11. root Rust workspace format/Clippy/tests/build remain green;
12. a permanent Android validation workflow remains after temporary Phase 145 workflows are removed;
13. audit evidence records exact source/APK/native hashes and validation run identifiers.

## Phase 146 handoff

Phase 146 may build enrollment, device list/status and revocation UI on top of the validated Phase 145 custody, state and bridge boundaries. It may not convert the Phase 145 disposable bootstrap into a real production account/device cutover without its own gates.

## Production boundary

Phase 145 MUST NOT:

- distribute/sign a production APK;
- use a real production PRW account/device identity;
- activate a public/LAN production PRW listener;
- activate production STUN/ICE/TURN/relay traffic;
- provision production transport credentials;
- replace/restart the production Agent for the remote data plane;
- alter firewall/NAT/router/TUN/TAP/routes/DNS.

## Completion classification

Target final state:

`PHASE_145_DONE / ANDROID_SHELL_AND_KEY_CUSTODY_VALIDATED / JNI_0_22_4_CARGO_NDK_4_1_2 / NON_EXPORTABLE_KEYSTORE_P256 / AUTHENTICATED_BOOTSTRAP_FOUNDATION_VALIDATED / PERMANENT_ANDROID_CI / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_146`
