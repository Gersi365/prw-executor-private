# Phase 004 Device Identity Encoding Lock

Status: approved under standing project authorization for build-phase implementation

## Decision

Phase 004 locks the initial serialized forms associated with the Phase 003 device-identity primitive.

### Public key

Repository encoding identifier:

- `SubjectPublicKeyInfoDer`

Meaning:

- DER-encoded X.509 `SubjectPublicKeyInfo`;
- ECC key algorithm identifier `id-ecPublicKey`;
- named-curve parameter `secp256r1`, also known as NIST P-256.

The profile follows RFC 5480.

### Signature

Repository encoding identifier:

- `EcdsaSigValueDer`

Meaning:

- DER-encoded RFC 3279 `ECDSA-Sig-Value`;
- ASN.1 structure `SEQUENCE { r INTEGER, s INTEGER }`.

## Rationale

Phase 003 intentionally left serialization undefined. A concrete enrollment or proof-of-possession protocol cannot safely interoperate across Ubuntu and Android if public keys and signatures are only opaque byte strings whose encodings are inferred by convention.

RFC 5480 defines the ECC `SubjectPublicKeyInfo` profile and requires `id-ecPublicKey` support. For P-256 it identifies the `secp256r1` named curve.

Android's `java.security.Key` API describes X.509 `SubjectPublicKeyInfo` as the ASN.1 format used for public keys, and `X509EncodedKeySpec` accepts and returns X.509-encoded public-key bytes.

RFC 3279 defines the interoperable ECDSA signature transfer form as an ASN.1 sequence containing `r` and `s`.

The selected formats are therefore standardized, self-describing enough for provider import/export boundaries, and compatible with the current Android platform direction without inventing a PRW-specific cryptographic encoding.

## Locked boundary

Phase 004 locks:

1. `PublicIdentityMaterial` carries an explicit device-identity algorithm.
2. It also carries an explicit public-key encoding.
3. The initial public-key encoding is `SubjectPublicKeyInfoDer`.
4. `DeviceIdentitySignature` carries an explicit device-identity algorithm.
5. It also carries an explicit signature encoding.
6. The initial signature encoding is `EcdsaSigValueDer`.
7. Neither algorithm nor encoding may be inferred from byte length or byte prefix.
8. Signature bytes are not stable identifiers.

## Validation boundary

Phase 004 types require non-empty byte strings and preserve explicit algorithm/encoding metadata.

They deliberately do not parse or cryptographically validate DER. A future cryptographic-provider boundary must perform strict parsing and reject:

- malformed DER;
- wrong `SubjectPublicKeyInfo` algorithm identifiers;
- wrong curve parameters;
- invalid EC points;
- malformed `ECDSA-Sig-Value` structures;
- cryptographically invalid signatures.

This avoids writing an ad-hoc ASN.1 or ECC validator inside the domain-model crate.

## Deliberately not locked

Phase 004 does not select or implement:

- a Rust cryptography crate;
- OpenSSL or another Ubuntu crypto provider;
- Android Keystore policy details;
- StrongBox requirements;
- private-key creation or persistence;
- ECDSA sign/verify execution;
- strict DER parser implementation;
- ECDSA low-S canonicalization policy;
- proof-of-possession message semantics;
- key attestation;
- backup or recovery;
- key rotation;
- transport-key cryptography;
- TLS configuration;
- enrollment-network protocol.

## Primary references

- RFC 5480, Elliptic Curve Cryptography Subject Public Key Information.
- RFC 3279, Algorithms and Identifiers for the Internet X.509 Public Key Infrastructure.
- Android `java.security.Key` API reference.
- Android `X509EncodedKeySpec` API reference.

## Validation requirements

Phase 004 source changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- public identity rejects empty bytes;
- public identity preserves algorithm and public-key encoding;
- signature material rejects empty bytes;
- signature material preserves algorithm and signature encoding.
