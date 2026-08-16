# PRW Android Application

Phase 145 establishes the first non-production native Android client foundation.

Architecture:

- Kotlin + Jetpack Compose UI;
- Android Keystore non-exportable P-256 identity custody;
- ViewModel + StateFlow presentation model;
- a narrow standalone Rust/JNI adapter under `native/`;
- no production endpoint, production account, release signing, boot receiver, or hidden daemon.

Build prerequisites are locked by `contracts/ANDROID_CLIENT_ARCHITECTURE_DECISION.md` and `contracts/ANDROID_CLIENT_FOUNDATION_CONTRACT.md`.
