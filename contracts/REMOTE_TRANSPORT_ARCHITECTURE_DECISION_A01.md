# Private Remote Workspace — Remote Transport Architecture Decision A01

Status: Phase 139 A01 normative clarification
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Parent decision: `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION.md`

## Purpose

This addendum narrows one implementation detail needed by Phase 140: deriving `TransportIdentity` as SHA-256 over the canonical DER `SubjectPublicKeyInfo` of the presented transport leaf certificate.

It does not alter the Phase 139 protocol decision, the QUIC/TLS profile, the identity-separation model, or any production mutation boundary.

## Normative clarification

Phase 140 may add the following direct support dependency alongside the exact Phase 139 transport dependencies:

```toml
aws-lc-rs = { version = "=1.18.0", default-features = true }
```

Its permitted runtime use in `prw-remote-transport` is narrowly scoped to:

- SHA-256 over canonical transport-certificate SPKI DER for `TransportIdentity` derivation; and
- provider-aligned utility required by the already-selected rustls AWS-LC profile.

This is **not** authorization to introduce a second cryptographic protocol, custom certificate verification, generic signing, private-key generation/import/export, or application-layer encryption.

The transport crate must obtain SPKI bytes through `rustls::server::ParsedCertificate::subject_public_key_info()` and compute the digest through `aws_lc_rs::digest::{SHA256, digest}`. It must not add a second X.509 parser merely for this fingerprint and must not implement SHA-256 itself.

## Exact Phase 140 runtime direct dependency set

```toml
quinn = { version = "=0.11.11", default-features = false, features = ["runtime-tokio", "rustls-aws-lc-rs"] }
rustls = { version = "=0.23.43", default-features = false, features = ["std", "aws_lc_rs"] }
tokio = { version = "=1.53.1", default-features = false, features = ["rt", "macros", "net", "time", "sync", "io-util"] }
aws-lc-rs = { version = "=1.18.0", default-features = true }
```

Internal PRW dependencies may include `prw-connectivity` for the existing typed `TransportIdentity` boundary.

Test-only certificate generation remains outside the runtime dependency set. If Phase 140 uses `rcgen`, it must be a dev-dependency, exact-pinned, AWS-LC-backed, disposable only, and its generated CA/private keys must never be production material.

## Compatibility proof

Disposable run `31949682590`, job `95171127021`, on Rust/Cargo 1.97.1:

- asserted exact direct versions Quinn `0.11.11`, rustls `0.23.43`, Tokio `1.53.1`, aws-lc-rs `1.18.0`;
- compiled a real function that parses `CertificateDer` with rustls `ParsedCertificate`, reads `SubjectPublicKeyInfo`, hashes it with AWS-LC SHA-256, and returns `[u8; 32]`;
- completed `cargo check --locked --all-targets`;
- re-proved repository immutability.

Scratch lock SHA-256: `f364e750683d08d70c7d720e31fac679a8c6af4df2eb88dba40d6c10d2e544f8`.

## Production boundary

No production key, certificate, listener, route, firewall, DNS, Agent service, Android/Desktop artifact, or PowerCode network state is authorized or changed by this clarification.

Final state: `PHASE_139_A01_LOCKED / SPKI_SHA256_PROVIDER_PATH_PROVED / READY_FOR_PHASE_140`.
