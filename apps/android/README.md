# PRW Android Application

Phase 146 builds on the validated non-production native Android foundation.

Current development responsibilities:

- Kotlin + Jetpack Compose UI;
- Android Keystore non-exportable P-256 device/transport identity custody;
- narrow Rust/JNI adapter reusing authoritative PRW codecs and identity verification;
- local typed authenticated bootstrap proof;
- local typed enrollment proof validation;
- bounded authoritative/injected device lifecycle projection;
- revocation intent that never forges authoritative `Revoked` state.

Phase 146 does not contact a production enrollment/device registry endpoint, perform production revocation propagation, activate production networking, or add production signing/distribution.
