use aws_lc_rs::{
    digest::{SHA256, digest},
    rand::SystemRandom,
    signature::{ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair},
};
use prw_device_identity_signer::UbuntuEnrollmentSigner;

fn lowercase_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

#[test]
fn public_spki_fingerprint_is_exact_sha256_lowercase_hex() {
    let private = EcdsaKeyPair::generate_pkcs8(
        &ECDSA_P256_SHA256_ASN1_SIGNING,
        &SystemRandom::new(),
    )
    .expect("generate disposable P-256 identity");
    let signer = UbuntuEnrollmentSigner::from_pkcs8_v1_der(private.as_ref())
        .expect("load canonical signer");

    let expected = lowercase_hex(digest(&SHA256, signer.public_identity().as_bytes()).as_ref());
    let actual = signer.public_spki_sha256_hex();

    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 64);
    assert!(
        actual
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
}

#[test]
fn reloading_same_private_identity_preserves_fingerprint() {
    let private = EcdsaKeyPair::generate_pkcs8(
        &ECDSA_P256_SHA256_ASN1_SIGNING,
        &SystemRandom::new(),
    )
    .expect("generate disposable P-256 identity");
    let first = UbuntuEnrollmentSigner::from_pkcs8_v1_der(private.as_ref()).expect("first load");
    let second = UbuntuEnrollmentSigner::from_pkcs8_v1_der(private.as_ref()).expect("second load");

    assert_eq!(
        first.public_spki_sha256_hex(),
        second.public_spki_sha256_hex()
    );
}
