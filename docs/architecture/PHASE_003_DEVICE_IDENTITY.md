# Phase 003 Device Identity Primitive Lock

Status: approved under standing project authorization for build-phase implementation

## Decision

The initial Private Remote Workspace device-identity signature primitive is:

- ECDSA over NIST P-256;
- SHA-256 as the message digest;
- repository algorithm identifier: `EcdsaP256Sha256`.

This decision applies only to long-lived device identity signatures.

## Rationale

The selection is based on platform compatibility and standardization rather than on inventing a private cryptographic design.

NIST FIPS 186-5 defines ECDSA as a supported digital-signature technique, and NIST SP 800-186 specifies recommended elliptic-curve domain parameters including the NIST prime curves.

Android's official Keystore documentation supports EC key generation through `KeyGenParameterSpec` from API level 23 and provides a NIST P-256 signing/verification example. Android's `Signature` API lists Ed25519 only from API level 33.

For the current Ubuntu + Android product targets, P-256 therefore provides a broader path to OS-backed private-key storage on Android while remaining a standard cross-platform signature primitive.

This is an architecture inference from the cited platform/standards constraints. It is not a claim that all Android devices provide identical hardware-backed security.

## Locked boundary

Phase 003 locks:

1. Device identity has an explicit algorithm identifier.
2. The first identifier is `EcdsaP256Sha256`.
3. `PublicIdentityMaterial` carries both the explicit algorithm and non-empty public bytes.
4. Algorithm selection must never be inferred from byte length or implicit encoding.
5. Device identity remains separate from transport identity.

## Deliberately not locked

Phase 003 does not select:

- a concrete Rust crypto crate;
- OpenSSL or another Ubuntu crypto provider;
- Android Keystore implementation details;
- StrongBox as a requirement;
- public-key wire encoding;
- signature wire encoding;
- key attestation;
- private-key backup;
- private-key recovery;
- key rotation;
- transport-key cryptography;
- TLS configuration;
- enrollment-network protocol.

The concrete signing backend must be selected later only after backend-specific security and maintenance review.

## Source references

Primary standards/platform references used for this decision:

- NIST FIPS 186-5, Digital Signature Standard.
- NIST SP 800-186, Recommendations for Discrete Logarithm-based Cryptography: Elliptic Curve Domain Parameters.
- Android `KeyGenParameterSpec` / Android Keystore API reference.
- Android `Signature` API reference.

## Validation requirements

Phase 003 source changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove that public identity material preserves its explicit algorithm identifier and still rejects empty public bytes.
