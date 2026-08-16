# PRW Android Application

Phase 147 builds on the validated Phase 145/146 non-production Android foundation.

Current development responsibilities:

- Kotlin + Jetpack Compose UI;
- Android Keystore non-exportable P-256 device/transport identity custody;
- narrow Rust/JNI adapter reusing authoritative PRW codecs and identity verification;
- local typed authenticated bootstrap and enrollment-proof validation;
- bounded authoritative/injected device lifecycle projection and revocation intent;
- bounded terminal presentation lifecycle;
- native terminal open/input/resize/read/close payload projection through the existing Phase 143 `BridgeCommand` codec;
- explicit disposable terminal open/output/close callbacks that do not forge remote authority.

Phase 147 adds no generic run-command API and no production socket, remote shell, PTY activation, account cutover, endpoint publication, or Android release signing/distribution.
