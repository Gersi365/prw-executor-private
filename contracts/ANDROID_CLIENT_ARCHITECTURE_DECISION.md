# Private Remote Workspace — Android Client Architecture Decision

Status: Phase 144 architecture lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Input baseline: `5ab3521db0e30401ab5a1e5179265b71435d4153`
Parent roadmap: `contracts/PRODUCTIZATION_ROADMAP_AND_MUTATION_GATES_CONTRACT.md`
Parent bridge: `contracts/END_TO_END_AUTHENTICATED_CAPABILITY_BRIDGE_CONTRACT.md`

## Approval

The user explicitly approved the complete Phase 144 recommendation on 2026-08-16 and authorized continued work through the remaining roadmap at the same deliberate audit-first pace.

This approval locks the architecture below. It does not itself activate production networking, production credentials, Android distribution signing, real-account cutover, production Agent replacement/restart, firewall/NAT/router/TUN/TAP/route/DNS mutation, or any other Phase 154 production transaction.

## Product role

Android is the first user-facing mobile PRW client and a PRW mesh endpoint. The application remains independent of Tailscale, Termius or another separately installed VPN/SSH/file-management product.

The Phase 145–149 Android client must provide, in bounded slices:

- application shell and connection state;
- Android device identity custody;
- authenticated PRW connection bootstrap;
- enrollment/device list/status/revocation;
- terminal UX;
- file browser and resumable transfer UX;
- port-forward management;
- private-network status;
- optional private-DNS settings.

## Native Android architecture

The Android application is native Kotlin using Jetpack Compose.

Locked responsibilities:

### Kotlin / Android layer

Kotlin owns:

- Activity/Application lifecycle;
- Compose UI;
- ViewModel/state-flow presentation model;
- Android foreground/background execution policy;
- notifications and user-visible long-running-connection state;
- Android Keystore calls and platform key handles;
- Android permissions and platform policy;
- Android socket/service activation orchestration;
- conversion between platform events and the narrow PRW native bridge.

Kotlin must not reimplement the existing PRWM/PRWC codecs, capability mapping, registry revalidation rules, transfer invariants, terminal domain invariants or forwarding policy when those are already validated in Rust.

### Rust PRW layer

The existing Rust workspace remains authoritative for:

- PRW domain identifiers and lifecycle types;
- policy/capability mapping;
- enrollment/session messages and validation;
- registry/current transport-identity validation;
- file-service and file-transfer domain rules;
- terminal and forwarding domain rules;
- connectivity/NAT-traversal/relay domain logic;
- QUIC/TLS configuration, identity derivation and PRWM framing;
- Phase 143 authenticated capability bridge.

A new Android-specific Rust adapter may compose those crates for the mobile client. It must not weaken their bounds or duplicate an alternate protocol stack.

## Kotlin/Rust bridge boundary

The Android/Rust integration is intentionally narrow.

Rules:

- no generic `eval`, shell, executable, filesystem-path authority or arbitrary socket command crosses the bridge;
- commands crossing the boundary are typed PRW client intents and bounded byte buffers;
- errors crossing the boundary use stable bounded error categories and never expose private key material;
- callbacks/events are bounded and versioned;
- Rust core crates retain `unsafe_code = "forbid"`;
- any unavoidable JNI/native binding `unsafe` is isolated in one Android adapter crate/module and is not permitted to spread into PRW core crates;
- generated binding glue is treated as an implementation detail behind audited safe wrappers;
- no private key bytes cross from Android Keystore into Kotlin UI state or generic Rust buffers.

Phase 145 owns the exact binding implementation/tool selection and must validate it before adding runtime capability.

## Android key custody

Android device identity and transport identity private keys use Android Keystore-backed non-exportable EC P-256 keys.

Locked policy:

- algorithm: EC P-256 / ECDSA SHA-256, matching the existing PRW identity profile;
- private keys are generated inside Android Keystore and are not exported as PKCS#8;
- public SPKI material may be exported for PRW enrollment/registry use;
- StrongBox is requested when supported and appropriate; inability to allocate StrongBox falls back to normal Android Keystore rather than to plaintext/exportable private-key storage;
- the application may request only typed PRW signatures over canonical PRW messages;
- no generic arbitrary-message signing API is exposed to the UI/application layer;
- key invalidation is fail-closed and requires re-enrollment/recovery rather than silently generating a replacement identity under the same device record;
- transport-key rotation remains separate from device identity rotation.

The existing Ubuntu systemd custody adapter is Linux-specific and is not reused on Android.

## TLS/transport signer boundary

The current Phase 140 `prw-remote-transport` disposable API can construct TLS configuration from an exportable `PrivateKeyDer`. Android production architecture MUST NOT export a Keystore private key merely to satisfy that helper.

The Android transport adapter must therefore introduce a platform-backed signing path compatible with the existing rustls/QUIC identity profile, or an equivalent audited platform integration that keeps private-key operations in Android Keystore.

Requirements:

- TLS 1.3 only;
- QUIC v1;
- ALPN `prw-mesh/1`;
- existing SPKI-SHA256 `TransportIdentity` derivation;
- explicit trust roots;
- no early data;
- fail-closed peer identity match;
- private signing operation remains inside platform custody;
- no alternate Android-only wire protocol.

## Android execution model

The UI does not own the transport session.

Locked model:

1. Compose screen emits a typed user intent;
2. ViewModel owns presentation state and delegates to an application connection controller;
3. the connection controller owns a single explicit PRW connection state machine;
4. a user-visible foreground service owns a persistent active remote connection when Android policy requires it;
5. the service/native adapter owns network lifecycle and cancellation;
6. UI observes immutable state/events and may request connect/disconnect/pause/resume actions;
7. process death is fail-closed; reconnect requires reconstructing validated state and reauthentication rather than assuming an old session remains valid.

No hidden always-on daemon or boot-time remote activation is introduced by this decision.

## Foreground/background policy

Long-running user-enabled remote connectivity must follow Android foreground-service restrictions and remain visibly attributable to the user.

The initial profile:

- no background connection is started merely because the phone booted;
- no background activity launch is used to create a remote session;
- persistent connected-device networking uses a declared/user-visible foreground-service model where required;
- long file transfers use Android-compliant user-initiated/data-transfer scheduling and may not depend on an immortal background service;
- stopping the visible connection service terminates or suspends the owned remote connection cleanly;
- notification actions are bounded to safe typed actions such as disconnect/pause where implemented.

## Android toolchain lock

The approved Phase 144 baseline is:

- Android UI language: Kotlin;
- UI toolkit: Jetpack Compose;
- minimum SDK: API 29 (Android 10);
- compile SDK: API 36;
- target SDK: API 36;
- Android Gradle Plugin: 9.3.0;
- Gradle: 9.5.0;
- JDK: 17;
- Android SDK Build Tools baseline: 36.0.0;
- Android NDK baseline for native integration: 28.2.13676358;
- Compose BOM: `2026.06.00` stable;
- AGP built-in Kotlin is used; `org.jetbrains.kotlin.android` / `kotlin-android` is not applied.

Rationale:

- AGP 9.3.0 officially supports API levels through 37 and documents Gradle 9.5.0, Build Tools 36.0.0, NDK 28.2.13676358 and JDK 17 as its compatibility baseline;
- Google Play requires new mobile apps/updates to target API 36 or higher starting 2026-08-31;
- Compose BOM `2026.06.00` is the current documented stable BOM at the decision date;
- API 29 is a deliberate product floor, not a Keystore technical minimum, chosen to avoid carrying legacy Android 6–9 lifecycle/background compatibility into a new security-sensitive remote administration product.

## Dependency policy

Phase 144 does not authorize a broad Android dependency set.

Rules for Phase 145 onward:

- prefer AndroidX/Jetpack first-party components;
- every new native/FFI/network/security dependency is pinned and reviewed before adoption;
- no second QUIC/TLS/ICE/relay protocol stack is added merely for Android convenience;
- no analytics/advertising SDK is introduced by default;
- no cloud messaging dependency is required for initial direct application use unless separately justified;
- dependency additions must be visible in phase audit evidence.

## State model

The initial Android UI state is a pure presentation projection of authoritative client/session state.

Top-level connection states:

- `Disconnected`;
- `Connecting`;
- `Authenticating`;
- `Connected`;
- `Suspended`;
- `Disconnecting`;
- `Error`.

State must separately represent:

- local enrollment/device-identity state;
- selected remote device;
- connectivity path (`LocalDirect`, `InternetDirect`, `Relay`, `Offline`);
- authenticated application-session state;
- active terminal/transfer/forward counts;
- optional private-DNS status.

A UI state label never grants a capability.

## Phase 145 handoff

Phase 145 is authorized to create the non-production Android application shell and the narrow Android/native foundation required to prove:

- reproducible Android debug build;
- API 29–36 manifest/config baseline;
- Compose shell/state model;
- Android Keystore non-exportable P-256 custody and typed proof adapter;
- narrow safe Kotlin/Rust bridge boundary;
- authenticated connection bootstrap against disposable/local test endpoints;
- no production endpoint activation or distribution signing.

If a binding/toolchain dependency is needed, Phase 145 must first probe and pin it before materializing it into the app.

## Production boundary

This architecture decision does not authorize:

- Play Store or external Android distribution;
- production signing keys;
- real production account/device cutover;
- production STUN/ICE/TURN/relay activation;
- production Agent replacement/restart;
- firewall/NAT/router/TUN/TAP/route/DNS mutation;
- public/LAN production listener activation.

Those remain behind later gates and the exact Phase 154 transaction.

## Completion classification

`PHASE_144_DONE / NATIVE_KOTLIN_COMPOSE_ANDROID_ARCHITECTURE_LOCKED / ANDROID_KEYSTORE_NON_EXPORTABLE_P256 / RUST_PRW_CORE_REUSED_THROUGH_NARROW_ADAPTER / API29_MIN_API36_TARGET / AGP_9_3_GRADLE_9_5_JDK17 / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_145`
