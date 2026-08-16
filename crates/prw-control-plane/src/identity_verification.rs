//! Device-identity verification boundary for the locked PRW identity profile.
//!
//! This module validates only public identity material and signatures. It does
//! not generate, persist, export, or upload production private keys.

use std::fmt;

use aws_lc_rs::{
    encoding::{AsDer, PublicKeyX509Der},
    signature::{ECDSA_P256_SHA256_ASN1, ParsedPublicKey},
};

use crate::{
    DeviceIdentityAlgorithm, DeviceIdentityPublicKeyEncoding, DeviceIdentitySignature,
    DeviceIdentitySignatureEncoding, PublicIdentityMaterial,
};

/// Failure while validating or verifying the locked PRW device-identity profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceIdentityVerificationError {
    /// The declared algorithm or encoding profile is not supported by this verifier.
    UnsupportedProfile,
    /// Public-key bytes were malformed or incompatible with the locked P-256 profile.
    InvalidPublicKey,
    /// The supplied public key was not the exact canonical DER SPKI representation.
    NonCanonicalPublicKeyEncoding,
    /// Signature verification failed or the signature encoding was malformed.
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

/// Verifies a device-identity signature using the locked PRW P-256 profile.
///
/// The accepted profile is exactly:
///
/// - ECDSA over NIST P-256 with SHA-256;
/// - DER X.509 `SubjectPublicKeyInfo` public keys;
/// - DER RFC 3279 `ECDSA-Sig-Value` signatures.
///
/// The provider parser is followed by canonical X.509 DER re-serialization and
/// byte-for-byte comparison. This deliberately rejects alternate encodings such
/// as raw SEC1 points even when the underlying provider can parse them.
///
/// # Errors
///
/// Returns [`DeviceIdentityVerificationError`] when the declared profile is not
/// the locked profile, when the public key is invalid or non-canonical, or when
/// signature verification fails.
pub fn verify_device_identity_signature(
    public_identity: &PublicIdentityMaterial,
    message: &[u8],
    signature: &DeviceIdentitySignature,
) -> Result<(), DeviceIdentityVerificationError> {
    if public_identity.algorithm() != DeviceIdentityAlgorithm::EcdsaP256Sha256
        || public_identity.encoding()
            != DeviceIdentityPublicKeyEncoding::SubjectPublicKeyInfoDer
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

#[cfg(test)]
mod tests {
    use aws_lc_rs::{
        encoding::AsDer,
        rand::SystemRandom,
        signature::{
            ECDSA_P256_SHA256_ASN1_SIGNING, ECDSA_P256_SHA256_FIXED_SIGNING,
            ECDSA_P384_SHA384_ASN1_SIGNING, EcdsaKeyPair, KeyPair,
        },
    };

    use super::{DeviceIdentityVerificationError, verify_device_identity_signature};
    use crate::{
        DeviceIdentityAlgorithm, DeviceIdentityPublicKeyEncoding, DeviceIdentitySignature,
        DeviceIdentitySignatureEncoding, PublicIdentityMaterial,
    };

    const MESSAGE: &[u8] = b"private-remote-workspace phase-111 identity verification";
    const EC_PUBLIC_KEY_OID_DER: &[u8] = &[0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
    const P256_CURVE_OID_DER: &[u8] = &[0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];
    const UNCOMPRESSED_POINT_PREFIX: &[u8] = &[0x03, 0x42, 0x00, 0x04];

    fn identity_from_der(der: Vec<u8>) -> PublicIdentityMaterial {
        PublicIdentityMaterial::new(
            DeviceIdentityAlgorithm::EcdsaP256Sha256,
            DeviceIdentityPublicKeyEncoding::SubjectPublicKeyInfoDer,
            der,
        )
        .expect("test public identity is non-empty")
    }

    fn signature_from_bytes(bytes: Vec<u8>) -> DeviceIdentitySignature {
        DeviceIdentitySignature::new(
            DeviceIdentityAlgorithm::EcdsaP256Sha256,
            DeviceIdentitySignatureEncoding::EcdsaSigValueDer,
            bytes,
        )
        .expect("test signature is non-empty")
    }

    fn p256_fixture() -> (EcdsaKeyPair, PublicIdentityMaterial) {
        let key_pair = EcdsaKeyPair::generate(&ECDSA_P256_SHA256_ASN1_SIGNING)
            .expect("generate disposable P-256 test key");
        let public_der = key_pair
            .public_key()
            .as_der()
            .expect("serialize disposable public key as SPKI DER");
        let identity = identity_from_der(public_der.as_ref().to_vec());
        (key_pair, identity)
    }

    fn replace_once(bytes: &mut [u8], needle: &[u8], replacement_last: u8) {
        let index = bytes
            .windows(needle.len())
            .position(|window| window == needle)
            .expect("expected DER marker exists");
        bytes[index + needle.len() - 1] = replacement_last;
    }

    #[test]
    fn verifies_valid_p256_spki_and_der_signature() {
        let (key_pair, identity) = p256_fixture();
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&identity, MESSAGE, &signature),
            Ok(())
        );
    }

    #[test]
    fn rejects_raw_sec1_point_even_if_provider_can_parse_it() {
        let (key_pair, _) = p256_fixture();
        let raw_point = key_pair.public_key().as_ref().to_vec();
        let identity = identity_from_der(raw_point);
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&identity, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::NonCanonicalPublicKeyEncoding)
        );
    }

    #[test]
    fn rejects_malformed_spki_der() {
        let (key_pair, identity) = p256_fixture();
        let mut malformed = identity.as_bytes().to_vec();
        malformed[0] = 0x31;
        let malformed = identity_from_der(malformed);
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&malformed, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidPublicKey)
        );
    }

    #[test]
    fn rejects_spki_with_trailing_bytes() {
        let (key_pair, identity) = p256_fixture();
        let mut trailing = identity.as_bytes().to_vec();
        trailing.push(0x00);
        let trailing = identity_from_der(trailing);
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert!(matches!(
            verify_device_identity_signature(&trailing, MESSAGE, &signature),
            Err(
                DeviceIdentityVerificationError::InvalidPublicKey
                    | DeviceIdentityVerificationError::NonCanonicalPublicKeyEncoding
            )
        ));
    }

    #[test]
    fn rejects_wrong_public_key_algorithm_oid() {
        let (key_pair, identity) = p256_fixture();
        let mut wrong_oid = identity.as_bytes().to_vec();
        replace_once(&mut wrong_oid, EC_PUBLIC_KEY_OID_DER, 0x02);
        let wrong_oid = identity_from_der(wrong_oid);
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&wrong_oid, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidPublicKey)
        );
    }

    #[test]
    fn rejects_wrong_named_curve_oid() {
        let (key_pair, identity) = p256_fixture();
        let mut wrong_curve = identity.as_bytes().to_vec();
        replace_once(&mut wrong_curve, P256_CURVE_OID_DER, 0x08);
        let wrong_curve = identity_from_der(wrong_curve);
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&wrong_curve, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidPublicKey)
        );
    }

    #[test]
    fn rejects_off_curve_public_point() {
        let (key_pair, identity) = p256_fixture();
        let mut invalid_point = identity.as_bytes().to_vec();
        let marker = invalid_point
            .windows(UNCOMPRESSED_POINT_PREFIX.len())
            .position(|window| window == UNCOMPRESSED_POINT_PREFIX)
            .expect("SPKI contains uncompressed P-256 point");
        let coordinates_start = marker + UNCOMPRESSED_POINT_PREFIX.len();
        invalid_point[coordinates_start..coordinates_start + 64].fill(0);
        let invalid_point = identity_from_der(invalid_point);
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&invalid_point, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidPublicKey)
        );
    }

    #[test]
    fn rejects_p384_key_under_p256_profile() {
        let key_pair = EcdsaKeyPair::generate(&ECDSA_P384_SHA384_ASN1_SIGNING)
            .expect("generate disposable P-384 test key");
        let public_der = key_pair
            .public_key()
            .as_der()
            .expect("serialize P-384 test key as SPKI DER");
        let identity = identity_from_der(public_der.as_ref().to_vec());
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable P-384 test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&identity, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidPublicKey)
        );
    }

    #[test]
    fn rejects_modified_message() {
        let (key_pair, identity) = p256_fixture();
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&identity, b"modified message", &signature),
            Err(DeviceIdentityVerificationError::InvalidSignature)
        );
    }

    #[test]
    fn rejects_modified_signature() {
        let (key_pair, identity) = p256_fixture();
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let mut bytes = signature.as_ref().to_vec();
        let last = bytes.len() - 1;
        bytes[last] ^= 0x01;
        let signature = signature_from_bytes(bytes);

        assert_eq!(
            verify_device_identity_signature(&identity, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidSignature)
        );
    }

    #[test]
    fn rejects_malformed_signature_der() {
        let (key_pair, identity) = p256_fixture();
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign with disposable test key");
        let mut bytes = signature.as_ref().to_vec();
        bytes.truncate(bytes.len() - 1);
        let signature = signature_from_bytes(bytes);

        assert_eq!(
            verify_device_identity_signature(&identity, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidSignature)
        );
    }

    #[test]
    fn rejects_fixed_width_signature_claimed_as_der() {
        let key_pair = EcdsaKeyPair::generate(&ECDSA_P256_SHA256_FIXED_SIGNING)
            .expect("generate disposable fixed-format P-256 test key");
        let public_der = key_pair
            .public_key()
            .as_der()
            .expect("serialize disposable public key as SPKI DER");
        let identity = identity_from_der(public_der.as_ref().to_vec());
        let rng = SystemRandom::new();
        let signature = key_pair
            .sign(&rng, MESSAGE)
            .expect("sign fixed-format test signature");
        let signature = signature_from_bytes(signature.as_ref().to_vec());

        assert_eq!(
            verify_device_identity_signature(&identity, MESSAGE, &signature),
            Err(DeviceIdentityVerificationError::InvalidSignature)
        );
    }
}
