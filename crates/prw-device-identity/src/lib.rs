//! Production device-identity public signature verification for Private Remote Workspace.
//!
//! This crate owns the provider-specific verification boundary for the locked PRW
//! device-identity profile. It deliberately exposes no private-key generation,
//! private-key import/export, signing, persistence, or transport-cryptography API.

use std::fmt;

use aws_lc_rs::{
    encoding::{AsDer, PublicKeyX509Der},
    signature::{ECDSA_P256_SHA256_ASN1, ParsedPublicKey},
};
use prw_control_plane::{
    DeviceIdentityAlgorithm, DeviceIdentityPublicKeyEncoding, DeviceIdentitySignature,
    DeviceIdentitySignatureEncoding, PublicIdentityMaterial,
};

/// Failure while verifying the locked PRW device-identity profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceIdentityVerificationError {
    /// The declared algorithm or encoding does not match the locked PRW profile.
    UnsupportedProfile,
    /// Public-key bytes are malformed or incompatible with the locked P-256 profile.
    InvalidPublicKey,
    /// The key is valid but is not the exact canonical DER SPKI representation.
    NonCanonicalPublicKeyEncoding,
    /// Signature verification failed or the signature encoding is malformed.
    InvalidSignature,
}

impl fmt::Display for DeviceIdentityVerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedProfile => formatter.write_str("unsupported device identity profile"),
            Self::InvalidPublicKey => formatter.write_str("invalid device identity public key"),
            Self::NonCanonicalPublicKeyEncoding => {
                formatter.write_str("device identity public key is not canonical DER SPKI")
            }
            Self::InvalidSignature => formatter.write_str("invalid device identity signature"),
        }
    }
}

impl std::error::Error for DeviceIdentityVerificationError {}

/// Verifies a signature under the locked PRW device-identity profile.
///
/// The accepted profile is exactly:
///
/// - ECDSA over NIST P-256 with SHA-256;
/// - DER X.509 `SubjectPublicKeyInfo` public keys;
/// - DER RFC 3279 `ECDSA-Sig-Value` signatures.
///
/// Provider input flexibility is intentionally narrower at the PRW boundary. The
/// parsed key is re-serialized as canonical X.509 DER and must match the supplied
/// bytes exactly before the signature is verified. This rejects alternate public-key
/// encodings such as raw SEC1 points even if the provider can parse them.
///
/// # Errors
///
/// Returns [`DeviceIdentityVerificationError`] when the declared profile is not the
/// locked profile, when the public key is invalid or non-canonical, or when signature
/// verification fails.
pub fn verify_device_identity_signature(
    public_identity: &PublicIdentityMaterial,
    message: &[u8],
    signature: &DeviceIdentitySignature,
) -> Result<(), DeviceIdentityVerificationError> {
    if public_identity.algorithm() != DeviceIdentityAlgorithm::EcdsaP256Sha256
        || public_identity.encoding() != DeviceIdentityPublicKeyEncoding::SubjectPublicKeyInfoDer
        || signature.algorithm() != DeviceIdentityAlgorithm::EcdsaP256Sha256
        || signature.encoding() != DeviceIdentitySignatureEncoding::EcdsaSigValueDer
    {
        return Err(DeviceIdentityVerificationError::UnsupportedProfile);
    }

    let parsed = ParsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, public_identity.as_bytes())
        .map_err(|_| DeviceIdentityVerificationError::InvalidPublicKey)?;
    let canonical: PublicKeyX509Der<'static> = parsed
        .as_der()
        .map_err(|_| DeviceIdentityVerificationError::InvalidPublicKey)?;

    if canonical.as_ref() != public_identity.as_bytes() {
        return Err(DeviceIdentityVerificationError::NonCanonicalPublicKeyEncoding);
    }

    parsed
        .verify_sig(message, signature.as_bytes())
        .map_err(|_| DeviceIdentityVerificationError::InvalidSignature)
}
