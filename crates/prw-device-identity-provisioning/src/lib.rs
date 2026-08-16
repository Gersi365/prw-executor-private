//! Creation-only Ubuntu device-identity provisioning for Private Remote Workspace.
//!
//! This crate generates a new P-256 identity locally and persists only a systemd
//! encrypted credential. It deliberately exposes no existing-key import/export,
//! no arbitrary credential name/path selection, no enrollment, and no networking.

#![cfg(target_os = "linux")]

use std::{
    env,
    fmt,
    fs::{self, File, OpenOptions, Permissions},
    io::Write,
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use aws_lc_rs::{
    rand::SystemRandom,
    signature::{ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair},
};
use prw_device_identity_custody::SYSTEMD_DEVICE_IDENTITY_CREDENTIAL_NAME;
use prw_device_identity_signer::UbuntuEnrollmentSigner;
use rustix::{
    fs::{CWD, RenameFlags, renameat_with},
    process::getuid,
};
use zeroize::Zeroizing;

/// Relative location of the persistent encrypted device-identity credential.
pub const DEVICE_IDENTITY_CIPHERTEXT_RELATIVE_PATH: &str =
    "private-remote-workspace/credentials/device-identity-private-key-v1.cred";

/// Relative location of the per-user service drop-in bound by Phase 124.
pub const DEVICE_IDENTITY_DROPIN_RELATIVE_PATH: &str =
    "systemd/user/prw-agent.service.d/20-device-identity-credential.conf";

const SYSTEMD_CREDS_PATH: &str = "/usr/bin/systemd-creds";
const MAX_ENCRYPTED_CREDENTIAL_BYTES: usize = 65_536;
const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
const CIPHERTEXT_MODE: u32 = 0o600;

/// Successful creation-only identity provisioning result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisionedDeviceIdentity {
    public_spki_sha256: [u8; 32],
    encrypted_credential_path: PathBuf,
}

impl ProvisionedDeviceIdentity {
    /// Returns SHA-256 over canonical public `SubjectPublicKeyInfo` DER.
    #[must_use]
    pub const fn public_spki_sha256(&self) -> [u8; 32] {
        self.public_spki_sha256
    }

    /// Returns the committed encrypted credential path.
    #[must_use]
    pub fn encrypted_credential_path(&self) -> &Path {
        &self.encrypted_credential_path
    }
}

/// Bounded non-secret failure classification for first identity provisioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceIdentityProvisioningError {
    /// XDG state/HOME did not resolve to an absolute usable state root.
    InvalidStateRoot,
    /// XDG config/HOME did not resolve to an absolute usable config root.
    InvalidConfigRoot,
    /// A required existing directory failed ownership/type/mode validation.
    InsecureDirectory,
    /// A production identity ciphertext or credential drop-in already exists.
    AlreadyProvisioned,
    /// A required private PRW directory could not be created safely.
    DirectoryCreationFailed,
    /// Local P-256 private identity generation failed.
    KeyGenerationFailed,
    /// The generated key failed canonical signer construction.
    GeneratedIdentityInvalid,
    /// The locked systemd credential utility could not be started or failed.
    CredentialEncryptionFailed,
    /// The encrypted credential output was empty or exceeded its bound.
    EncryptedCredentialOutOfBounds,
    /// Temporary ciphertext persistence or validation failed.
    CiphertextWriteFailed,
    /// No-replace atomic ciphertext commit failed.
    CiphertextCommitFailed,
    /// Final parent-directory durability synchronization failed.
    DirectorySyncFailed,
}

impl fmt::Display for DeviceIdentityProvisioningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidStateRoot => "invalid device identity state root",
            Self::InvalidConfigRoot => "invalid device identity config root",
            Self::InsecureDirectory => "insecure device identity directory",
            Self::AlreadyProvisioned => "device identity is already provisioned",
            Self::DirectoryCreationFailed => "device identity directory creation failed",
            Self::KeyGenerationFailed => "device identity key generation failed",
            Self::GeneratedIdentityInvalid => "generated device identity failed validation",
            Self::CredentialEncryptionFailed => "device identity credential encryption failed",
            Self::EncryptedCredentialOutOfBounds => {
                "encrypted device identity credential out of bounds"
            }
            Self::CiphertextWriteFailed => "encrypted device identity persistence failed",
            Self::CiphertextCommitFailed => "encrypted device identity commit failed",
            Self::DirectorySyncFailed => "device identity directory sync failed",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for DeviceIdentityProvisioningError {}

/// Formats a public SHA-256 fingerprint as exactly 64 lowercase hexadecimal digits.
#[must_use]
pub fn sha256_hex(fingerprint: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(64);
    for byte in fingerprint {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

/// Generates and atomically commits the first Ubuntu production device identity.
///
/// The private PKCS#8 value exists only in provider/zeroizing process memory and the
/// stdin pipe to `/usr/bin/systemd-creds`. Only encrypted ciphertext is persisted.
///
/// # Errors
///
/// Returns [`DeviceIdentityProvisioningError`] for invalid XDG roots, pre-existing
/// identity state, generation/encryption failure, insecure filesystem state, or
/// durability/atomic-commit failure.
pub fn provision_first_ubuntu_device_identity(
) -> Result<ProvisionedDeviceIdentity, DeviceIdentityProvisioningError> {
    let uid = getuid().as_raw();
    let state_root = resolve_xdg_root("XDG_STATE_HOME", ".local/state")
        .ok_or(DeviceIdentityProvisioningError::InvalidStateRoot)?;
    let config_root = resolve_xdg_root("XDG_CONFIG_HOME", ".config")
        .ok_or(DeviceIdentityProvisioningError::InvalidConfigRoot)?;

    validate_existing_root(&state_root, uid)?;
    if config_root.exists() {
        validate_existing_root(&config_root, uid)?;
    }

    let final_ciphertext = state_root.join(DEVICE_IDENTITY_CIPHERTEXT_RELATIVE_PATH);
    let dropin = config_root.join(DEVICE_IDENTITY_DROPIN_RELATIVE_PATH);
    require_absent(&final_ciphertext)?;
    require_absent(&dropin)?;

    let application_dir = state_root.join("private-remote-workspace");
    ensure_private_directory(&application_dir, uid)?;
    let credential_dir = application_dir.join("credentials");
    ensure_private_directory(&credential_dir, uid)?;
    require_absent(&final_ciphertext)?;

    let generated = EcdsaKeyPair::generate_pkcs8(
        &ECDSA_P256_SHA256_ASN1_SIGNING,
        &SystemRandom::new(),
    )
    .map_err(|_| DeviceIdentityProvisioningError::KeyGenerationFailed)?;
    let private_pkcs8 = Zeroizing::new(generated.as_ref().to_vec());
    let signer = UbuntuEnrollmentSigner::from_pkcs8_v1_der(private_pkcs8.as_slice())
        .map_err(|_| DeviceIdentityProvisioningError::GeneratedIdentityInvalid)?;
    let public_spki_sha256 = signer.public_identity_sha256();

    let ciphertext = encrypt_with_systemd_creds(private_pkcs8.as_slice())?;
    drop(private_pkcs8);

    let temp_ciphertext = credential_dir.join(format!(
        ".device-identity-private-key-v1.cred.phase126.{}.tmp",
        std::process::id()
    ));
    require_absent(&temp_ciphertext)
        .map_err(|_| DeviceIdentityProvisioningError::CiphertextWriteFailed)?;

    if let Err(error) = write_ciphertext_temp(&temp_ciphertext, &ciphertext, uid) {
        let _ = fs::remove_file(&temp_ciphertext);
        return Err(error);
    }

    let commit_result = renameat_with(
        CWD,
        &temp_ciphertext,
        CWD,
        &final_ciphertext,
        RenameFlags::NOREPLACE,
    );
    if commit_result.is_err() {
        let _ = fs::remove_file(&temp_ciphertext);
        return Err(DeviceIdentityProvisioningError::CiphertextCommitFailed);
    }

    if sync_directory(&credential_dir).is_err() {
        let _ = fs::remove_file(&final_ciphertext);
        let _ = sync_directory(&credential_dir);
        return Err(DeviceIdentityProvisioningError::DirectorySyncFailed);
    }

    Ok(ProvisionedDeviceIdentity {
        public_spki_sha256,
        encrypted_credential_path: final_ciphertext,
    })
}

fn resolve_xdg_root(variable: &str, home_suffix: &str) -> Option<PathBuf> {
    if let Some(value) = env::var_os(variable) {
        if value.is_empty() {
            return None;
        }
        let path = PathBuf::from(value);
        return path.is_absolute().then_some(path);
    }

    let home = env::var_os("HOME")?;
    if home.is_empty() {
        return None;
    }
    let home = PathBuf::from(home);
    if !home.is_absolute() {
        return None;
    }
    Some(home.join(home_suffix))
}

fn validate_existing_root(
    path: &Path,
    uid: u32,
) -> Result<(), DeviceIdentityProvisioningError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| DeviceIdentityProvisioningError::InsecureDirectory)?;
    if metadata.file_type().is_symlink()
        || !metadata.is_dir()
        || metadata.uid() != uid
        || metadata.mode() & 0o022 != 0
    {
        return Err(DeviceIdentityProvisioningError::InsecureDirectory);
    }
    Ok(())
}

fn ensure_private_directory(
    path: &Path,
    uid: u32,
) -> Result<(), DeviceIdentityProvisioningError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink()
                || !metadata.is_dir()
                || metadata.uid() != uid
                || metadata.mode() & 0o777 != PRIVATE_DIRECTORY_MODE
            {
                return Err(DeviceIdentityProvisioningError::InsecureDirectory);
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path)
                .map_err(|_| DeviceIdentityProvisioningError::DirectoryCreationFailed)?;
            fs::set_permissions(path, Permissions::from_mode(PRIVATE_DIRECTORY_MODE))
                .map_err(|_| DeviceIdentityProvisioningError::DirectoryCreationFailed)?;
            let metadata = fs::symlink_metadata(path)
                .map_err(|_| DeviceIdentityProvisioningError::DirectoryCreationFailed)?;
            if metadata.file_type().is_symlink()
                || !metadata.is_dir()
                || metadata.uid() != uid
                || metadata.mode() & 0o777 != PRIVATE_DIRECTORY_MODE
            {
                return Err(DeviceIdentityProvisioningError::InsecureDirectory);
            }
        }
        Err(_) => return Err(DeviceIdentityProvisioningError::InsecureDirectory),
    }
    Ok(())
}

fn require_absent(path: &Path) -> Result<(), DeviceIdentityProvisioningError> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Ok(_) => Err(DeviceIdentityProvisioningError::AlreadyProvisioned),
        Err(_) => Err(DeviceIdentityProvisioningError::AlreadyProvisioned),
    }
}

fn encrypt_with_systemd_creds(
    private_pkcs8: &[u8],
) -> Result<Vec<u8>, DeviceIdentityProvisioningError> {
    let mut child = Command::new(SYSTEMD_CREDS_PATH)
        .arg("--user")
        .arg("encrypt")
        .arg(format!("--name={SYSTEMD_DEVICE_IDENTITY_CREDENTIAL_NAME}"))
        .arg("-")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| DeviceIdentityProvisioningError::CredentialEncryptionFailed)?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or(DeviceIdentityProvisioningError::CredentialEncryptionFailed)?;
    stdin
        .write_all(private_pkcs8)
        .map_err(|_| DeviceIdentityProvisioningError::CredentialEncryptionFailed)?;
    drop(stdin);

    let output = child
        .wait_with_output()
        .map_err(|_| DeviceIdentityProvisioningError::CredentialEncryptionFailed)?;
    if !output.status.success() {
        return Err(DeviceIdentityProvisioningError::CredentialEncryptionFailed);
    }
    if output.stdout.is_empty() || output.stdout.len() > MAX_ENCRYPTED_CREDENTIAL_BYTES {
        return Err(DeviceIdentityProvisioningError::EncryptedCredentialOutOfBounds);
    }
    Ok(output.stdout)
}

fn write_ciphertext_temp(
    path: &Path,
    ciphertext: &[u8],
    uid: u32,
) -> Result<(), DeviceIdentityProvisioningError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(CIPHERTEXT_MODE)
        .open(path)
        .map_err(|_| DeviceIdentityProvisioningError::CiphertextWriteFailed)?;
    file.write_all(ciphertext)
        .map_err(|_| DeviceIdentityProvisioningError::CiphertextWriteFailed)?;
    file.sync_all()
        .map_err(|_| DeviceIdentityProvisioningError::CiphertextWriteFailed)?;
    fs::set_permissions(path, Permissions::from_mode(CIPHERTEXT_MODE))
        .map_err(|_| DeviceIdentityProvisioningError::CiphertextWriteFailed)?;

    let metadata = fs::symlink_metadata(path)
        .map_err(|_| DeviceIdentityProvisioningError::CiphertextWriteFailed)?;
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.uid() != uid
        || metadata.mode() & 0o777 != CIPHERTEXT_MODE
    {
        return Err(DeviceIdentityProvisioningError::CiphertextWriteFailed);
    }
    Ok(())
}

fn sync_directory(path: &Path) -> std::io::Result<()> {
    File::open(path)?.sync_all()
}

#[cfg(test)]
mod tests {
    use super::sha256_hex;

    #[test]
    fn public_fingerprint_hex_is_fixed_lowercase_width() {
        let mut fingerprint = [0_u8; 32];
        fingerprint[0] = 0xab;
        fingerprint[31] = 0xef;
        let encoded = sha256_hex(fingerprint);
        assert_eq!(encoded.len(), 64);
        assert_eq!(&encoded[..2], "ab");
        assert_eq!(&encoded[62..], "ef");
        assert!(encoded.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(encoded, encoded.to_ascii_lowercase());
    }
}
