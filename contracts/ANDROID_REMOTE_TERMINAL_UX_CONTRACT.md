# Private Remote Workspace — Android Remote Terminal UX Contract

Status: Phase 147 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Parent roadmap: `contracts/PRODUCTIZATION_ROADMAP_AND_MUTATION_GATES_CONTRACT.md`
Parent terminal domain: `contracts/TERMINAL_SESSION_FOUNDATION_CONTRACT.md`
Parent Android foundation: `contracts/ANDROID_CLIENT_FOUNDATION_CONTRACT.md`
Parent Android device-management slice: `contracts/ANDROID_ENROLLMENT_DEVICE_MANAGEMENT_CONTRACT.md`
Parent remote bridge: Phase 143 `prw-remote-bridge`

## Purpose

Phase 147 adds the first bounded Android remote-terminal presentation and typed terminal-command projection on top of the validated Phase 145/146 Android foundation.

This remains a non-production functional slice. It does not activate a production socket, contact a production Agent, spawn a local or remote shell from Android, distribute a production APK, or cross the Phase 154 production remote-network mutation gate.

## Existing authority that MUST be reused

Phase 147 does not create a second terminal protocol, terminal capability model, session domain, or arbitrary command API.

The existing Rust workspace remains authoritative for:

- `TerminalSessionId` non-zero identity;
- `TerminalProfile::{PosixShell, BashShell}`;
- `TerminalGeometry` bounds of 1 through 1000 columns/rows;
- terminal I/O bound of 64 KiB at the terminal-domain layer;
- maximum 32 active terminal sessions at the broker layer;
- lifecycle `Opening`, `Open`, `Closing`, `Closed`, `Failed`;
- Phase 143 `BridgeCommand::{TerminalOpen, TerminalInput, TerminalResize, TerminalRead, TerminalClose}`;
- Phase 143 PRWC operation codes and payload encoding/decoding;
- server-side capability distinction: `TerminalOpen` for open and `TerminalExec` for input/resize/read/close;
- current-registry/session/transport/capability revalidation before real dispatch.

Android remains a typed client/presentation adapter and is not an authorization authority.

## Dependency decision

Phase 147 may add internal path dependencies from `apps/android/native` to:

- `prw-remote-bridge`;
- `prw-terminal`.

No new third-party terminal/network/crypto library is selected by this decision.

Evidence:

- initial probe run `31968272078` was inconclusive because its harness invoked an Android target build without the NDK target compiler;
- corrective NDK probe run `31968345916` used the already-validated NDK `28.2.13676358`, Rust `1.97.1`, and `cargo-ndk 4.1.2`, then successfully built the native adapter for `arm64-v8a` with `prw-remote-bridge` injected only in the runner;
- no product source was materialized by either probe.

Phase 147 completion still requires both `arm64-v8a` and `x86_64` builds on the final candidate.

## Native Android terminal-command boundary

The Android native adapter may expose narrow JNI functions that construct the five terminal PRWC payloads.

They MUST instantiate existing Rust terminal domain values and call existing `BridgeCommand::encode()`.

They MUST NOT duplicate PRWC header/operation encoding in Kotlin or in a second Rust codec.

Required operations:

1. terminal open — non-zero session identifier, named profile, validated geometry;
2. terminal input — non-empty bounded bytes;
3. terminal resize — existing session identifier plus validated geometry;
4. terminal read — non-zero bounded requested byte count;
5. terminal close — existing non-zero session identifier.

Because the Phase 143 bridge inline payload bound is narrower than the 64 KiB terminal-domain bound, Android terminal input/read projection MUST respect the existing bridge bound rather than widening it.

Malformed profile code, zero/negative session identifier, invalid geometry, empty/oversized input, zero/oversized read request, malformed PRWC payload, or other invalid typed values fail closed.

## No arbitrary run-command API

Phase 147 MUST NOT introduce any API shaped as:

- `runCommand(String)`;
- caller-supplied executable path/argument vector;
- shell fragment launch API;
- environment injection API;
- arbitrary process-spawn request.

Text typed after an authorized terminal session is open is terminal input, not a new generic command-execution endpoint.

## Android presentation lifecycle

The Android presentation layer may represent one active disposable terminal view with these states:

- `Closed`;
- `Opening`;
- `Open`;
- `Closing`;
- `Failed`.

A local open request only emits a typed PRWC open payload and moves presentation to `Opening`.

It MUST NOT claim `Open` until an explicit authoritative/disposable acceptance callback is applied.

Likewise, a local close request moves presentation to `Closing`; only an authoritative/disposable close result moves it to `Closed`.

Input, resize and output-read intents are accepted only while presentation is `Open`.

A failed/closed session does not silently reopen.

## Output and transcript boundary

Phase 147 may accept explicitly injected/disposable remote-output bytes for presentation tests/demonstration.

Rules:

- Android does not fabricate remote shell output after sending input;
- one injected output chunk must stay within the existing terminal I/O bound;
- the UI keeps a separate bounded transcript-memory limit so repeated output cannot grow memory without bound;
- Phase 147 does not claim ANSI/VT terminal-emulation completeness; output is a bounded text/transcript presentation surface;
- a later rendering improvement must not alter terminal authority or wire semantics.

## Device/capability separation

A device lifecycle label or workspace role is not terminal capability authority.

The Android UI MUST NOT infer `TerminalOpen` or `TerminalExec` permission from:

- `PendingEnrollment`, `Enrolled`, or `Revoked` presentation labels;
- role metadata;
- local UI state.

Real terminal dispatch remains subject to the existing authenticated transport/session/current-registry/capability bridge.

Phase 147 itself does not send terminal payloads to a production endpoint.

## UI slice

The Compose surface may provide:

- named profile choice (`PosixShell` or `BashShell`);
- bounded terminal geometry controls/defaults;
- open-request control;
- explicit disposable open-acceptance control for non-production validation;
- bounded text input and send control while `Open`;
- bounded output-read request;
- bounded transcript display;
- resize request;
- close request and explicit disposable close completion;
- lifecycle/detail labels that distinguish local intent from authoritative state.

The UI MUST make the non-production/disposable nature visible.

## Dependency and build boundary

No Android UI dependency is added for Phase 147.

The existing Compose/Material3 stack remains unchanged.

The native Cargo graph may change only through the two approved internal path dependencies and their already-root-locked transitive workspace graph. Any unexpected new third-party version selection must be surfaced before materialization.

## Required validation

Phase 147 completion requires at least:

1. exact dependency probe evidence retained;
2. native `Cargo.lock` regenerated deterministically after the approved internal path dependencies;
3. native fmt/Clippy/tests green;
4. native tests prove all five terminal operations encode through `BridgeCommand` and decode back to the exact typed variant;
5. invalid session/profile/geometry/input/read bounds fail closed;
6. no generic arbitrary-command/process-spawn API exists;
7. Android controller tests prove open request does not forge `Open` state;
8. controller tests prove input/resize/read are rejected outside `Open`;
9. controller tests prove close request does not forge `Closed` state;
10. controller tests prove remote output is not fabricated from input;
11. transcript/output memory bounds are tested;
12. both `arm64-v8a` and `x86_64` native release builds pass;
13. Android unit tests, `lintDebug`, and debug APK assembly pass;
14. APK contains both native libraries;
15. root Rust locked graph/fmt/Clippy/tests/build remain green;
16. permanent Android CI revalidates the materialized source;
17. authoritative audit evidence records exact source/run/hash identifiers and no production side effect.

## Explicitly deferred

Phase 147 does not implement or activate:

- production remote terminal transport send/receive;
- production Agent terminal dispatch;
- production PTY/process spawning changes;
- real account/device cutover;
- reconnect/reattach persistence;
- complete ANSI/VT terminal emulation;
- clipboard/file integration;
- file browser/transfers (Phase 148);
- port forwarding/network/private-DNS UI (Phase 149);
- production APK signing/distribution;
- production listener, relay, TUN/TAP, route, firewall/NAT/router/DNS mutation;
- production Agent replacement/restart.

## Phase 148 handoff

Phase 148 may add the Android remote file browser and bounded upload/download/resume/progress/retry slice using the existing file-service/file-transfer/remote-bridge authority.

It must not use terminal input as a filesystem API or infer file capability from terminal/device UI state.

## Completion classification

Target functional state:

`PHASE_147_FUNCTIONALLY_VALIDATED / ANDROID_TYPED_TERMINAL_UX / EXISTING_PRW_REMOTE_BRIDGE_CODEC_REUSED / NO_ARBITRARY_RUN_COMMAND_API / OPEN_CLOSE_AUTHORITY_NOT_FORGED / BOUNDED_TERMINAL_IO_AND_TRANSCRIPT / ARM64_X86_64_NATIVE_PASS / ANDROID_UNIT_LINT_APK_PASS / ROOT_RUST_PASS / PERMANENT_ANDROID_CI_PASS / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_148`
