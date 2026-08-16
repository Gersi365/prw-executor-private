//! Creation-only Ubuntu production device-identity provisioning boundary.
//!
//! This crate creates one fresh P-256 identity, derives only its public fingerprint,
//! and commits only a systemd-encrypted credential to the locked XDG state path.
//! It deliberately exposes no existing-key export, caller-supplied private-key input,
//! arbitrary credential name/path selection, shell execution, enrollment, or service
//! activation API.

use std::{fmt, path::PathBuf};

/// Absolute systemd credential tool selected by the Phase 126 contract.
pub const SYSTEMD_CREDS_BINARY: &str = "/usr/bin/systemd-creds";
/// Locked encrypted credential filename under the PRW credential state directory.
pub const ENCRYPTED_DEVICE_IDENTITY_FILENAME: &str = "device-identity-private-key-v1.cred";

/// Successful first-provisioning result containing public/non-secret metadata only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisionedDeviceIdentity {
    public_spki_sha256: String,
    encrypted_credential_path: PathBuf,
}

impl ProvisionedDeviceIdentity {
    /// Returns the lowercase SHA-256 fingerprint of canonical public SPKI DER.
    #[must_use]
    pub fn public_spki_sha256(&self) -> &str {
        &self.public_spki_sha256
    }

    /// Returns the committed encrypted credential path.
    #[must_use]
    pub fn encrypted_credential_path(&self) -> &std::path::Path {
        &self.encrypted_credential_path
    }
}

/// Bounded first-provisioning failure classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvisioningError {
    /// This provisioning boundary is available only on Linux.
    UnsupportedPlatform,
    /// XDG/HOME did not resolve to one absolute state root.
    StateRootUnavailable,
    /// A required state directory was missing, malformed, foreign-owned, writable by others, or a symlink.
    DestinationNotSecure,
    /// A production identity already occupies the locked final destination.
    IdentityAlreadyExists,
    /// A fresh same-directory encrypted-candidate name could not be reserved logically.
    TemporaryTargetUnavailable,
    /// The cryptographic provider failed to generate a fresh P-256 PKCS#8 identity.
    KeyGenerationFailed,
    /// The generated identity did not satisfy the locked canonical signer profile.
    GeneratedIdentityInvalid,
    /// `/usr/bin/systemd-creds` could not be started.
    EncryptLaunchFailed,
    /// Plaintext transfer to the bounded `systemd-creds` stdin pipe failed.
    EncryptInputFailed,
    /// `systemd-creds` rejected or failed the encryption operation.
    EncryptFailed,
    /// The encrypted candidate failed regular-file, ownership, mode, size, or inode validation.
    EncryptedCandidateInvalid,
    /// The encrypted candidate or its parent directory could not be durably synchronized.
    SyncFailed,
    /// Creation-only atomic commit to the final name failed.
    CommitFailed,
}

impl ProvisioningError {
    /// Returns a bounded machine-readable classification token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::UnsupportedPlatform => "unsupported_platform",
            Self::StateRootUnavailable => "state_root_unavailable",
            Self::DestinationNotSecure => "destination_not_secure",
            Self::IdentityAlreadyExists => "identity_already_exists",
            Self::TemporaryTargetUnavailable => "temporary_target_unavailable",
            Self::KeyGenerationFailed => "key_generation_failed",
            Self::GeneratedIdentityInvalid => "generated_identity_invalid",
            Self::EncryptLaunchFailed => "encrypt_launch_failed",
            Self::EncryptInputFailed => "encrypt_input_failed",
            Self::EncryptFailed => "encrypt_failed",
            Self::EncryptedCandidateInvalid => "encrypted_candidate_invalid",
            Self::SyncFailed => "sync_failed",
            Self::CommitFailed => "commit_failed",
        }
    }
}

impl fmt::Display for ProvisioningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.token())
    }
}

impl std::error::Error for ProvisioningError {}

/// Creates and commits exactly one fresh Ubuntu production device identity.
///
/// The destination is derived only from `XDG_STATE_HOME` or the standard
/// `$HOME/.local/state` fallback. The logical systemd credential name and final
/// filename are compile-time fixed. Existing final identity state is never replaced.
///
/// # Errors
///
/// Returns [`ProvisioningError`] when platform, destination safety, key generation,
/// systemd encryption, encrypted-candidate verification, durable sync, or creation-only
/// commit requirements fail.
pub fn provision_first_ubuntu_device_identity()
-> Result<ProvisionedDeviceIdentity, ProvisioningError> {
    #[cfg(target_os = "linux")]
    {
        linux::provision_first()
    }

    #[cfg(not(target_os = "linux"))]
    {
        Err(ProvisioningError::UnsupportedPlatform)
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::{
        env,
        fs::{self, DirBuilder, File},
        io::{self, Write},
        os::unix::fs::{DirBuilderExt, MetadataExt},
        path::{Path, PathBuf},
        process::{Command, Stdio},
    };

    use aws_lc_rs::{
        rand::{SecureRandom, SystemRandom},
        signature::{ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair},
    };
    use prw_device_identity_custody::SYSTEMD_DEVICE_IDENTITY_CREDENTIAL_NAME;
    use prw_device_identity_signer::UbuntuEnrollmentSigner;
    use rustix::{
        fs::{Mode, OFlags, open},
        process::geteuid,
    };
    use zeroize::Zeroizing;

    use super::{
        ENCRYPTED_DEVICE_IDENTITY_FILENAME, ProvisionedDeviceIdentity, ProvisioningError,
        SYSTEMD_CREDS_BINARY,
    };

    const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
    const ENCRYPTED_FILE_MODE: u32 = 0o600;
    const INSECURE_WRITE_BITS: u32 = 0o022;
    const MAX_ENCRYPTED_CREDENTIAL_BYTES: u64 = 64 * 1024;

    struct PreparedTarget {
        directory: PathBuf,
        temporary: PathBuf,
        final_path: PathBuf,
    }

    pub fn provision_first() -> Result<ProvisionedDeviceIdentity, ProvisioningError> {
        let state_root = resolve_state_root()?;
        let target = prepare_target_for_state_root(&state_root)?;
        let (private_key, public_spki_sha256) = generate_identity()?;

        if let Err(error) = encrypt_candidate(&target.temporary, private_key) {
            let _ = fs::remove_file(&target.temporary);
            return Err(error);
        }

        let opened_candidate = validate_and_sync_candidate(&target.temporary)?;
        if let Err(error) = commit_creation_only(&target, &opened_candidate) {
            let _ = fs::remove_file(&target.temporary);
            return Err(error);
        }

        Ok(ProvisionedDeviceIdentity {
            public_spki_sha256,
            encrypted_credential_path: target.final_path,
        })
    }

    fn resolve_state_root() -> Result<PathBuf, ProvisioningError> {
        let root = match env::var_os("XDG_STATE_HOME") {
            Some(value) if !value.is_empty() => PathBuf::from(value),
            Some(_) => return Err(ProvisioningError::StateRootUnavailable),
            None => {
                let home = env::var_os("HOME").ok_or(ProvisioningError::StateRootUnavailable)?;
                if home.is_empty() {
                    return Err(ProvisioningError::StateRootUnavailable);
                }
                PathBuf::from(home).join(".local/state")
            }
        };
        if !root.is_absolute() {
            return Err(ProvisioningError::StateRootUnavailable);
        }
        Ok(root)
    }

    fn prepare_target_for_state_root(root: &Path) -> Result<PreparedTarget, ProvisioningError> {
        validate_state_root(root)?;
        let product_directory = root.join("private-remote-workspace");
        ensure_private_directory(&product_directory)?;
        let credential_directory = product_directory.join("credentials");
        ensure_private_directory(&credential_directory)?;

        let final_path = credential_directory.join(ENCRYPTED_DEVICE_IDENTITY_FILENAME);
        match fs::symlink_metadata(&final_path) {
            Ok(_) => return Err(ProvisioningError::IdentityAlreadyExists),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(_) => return Err(ProvisioningError::DestinationNotSecure),
        }

        let temporary = fresh_temporary_path(&credential_directory)?;
        Ok(PreparedTarget {
            directory: credential_directory,
            temporary,
            final_path,
        })
    }

    fn validate_state_root(root: &Path) -> Result<(), ProvisioningError> {
        let metadata = fs::symlink_metadata(root).map_err(|_| ProvisioningError::DestinationNotSecure)?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(ProvisioningError::DestinationNotSecure);
        }
        if metadata.uid() != geteuid().as_raw() || metadata.mode() & INSECURE_WRITE_BITS != 0 {
            return Err(ProvisioningError::DestinationNotSecure);
        }
        Ok(())
    }

    fn ensure_private_directory(path: &Path) -> Result<(), ProvisioningError> {
        match fs::symlink_metadata(path) {
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let mut builder = DirBuilder::new();
                builder.mode(PRIVATE_DIRECTORY_MODE);
                match builder.create(path) {
                    Ok(()) => {}
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(_) => return Err(ProvisioningError::DestinationNotSecure),
                }
            }
            Err(_) => return Err(ProvisioningError::DestinationNotSecure),
        }

        let metadata = fs::symlink_metadata(path).map_err(|_| ProvisioningError::DestinationNotSecure)?;
        if metadata.file_type().is_symlink()
            || !metadata.file_type().is_dir()
            || metadata.uid() != geteuid().as_raw()
            || metadata.mode() & 0o777 != PRIVATE_DIRECTORY_MODE
        {
            return Err(ProvisioningError::DestinationNotSecure);
        }
        Ok(())
    }

    fn fresh_temporary_path(directory: &Path) -> Result<PathBuf, ProvisioningError> {
        let mut random = [0_u8; 16];
        SystemRandom::new()
            .fill(&mut random)
            .map_err(|_| ProvisioningError::TemporaryTargetUnavailable)?;
        let suffix = lowercase_hex(&random);
        let candidate = directory.join(format!(
            ".{ENCRYPTED_DEVICE_IDENTITY_FILENAME}.tmp-{suffix}"
        ));
        match fs::symlink_metadata(&candidate) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(candidate),
            _ => Err(ProvisioningError::TemporaryTargetUnavailable),
        }
    }

    fn generate_identity() -> Result<(Zeroizing<Vec<u8>>, String), ProvisioningError> {
        let private_key = {
            let generated = EcdsaKeyPair::generate_pkcs8(
                &ECDSA_P256_SHA256_ASN1_SIGNING,
                &SystemRandom::new(),
            )
            .map_err(|_| ProvisioningError::KeyGenerationFailed)?;
            Zeroizing::new(generated.as_ref().to_vec())
        };

        let signer = UbuntuEnrollmentSigner::from_pkcs8_v1_der(private_key.as_ref())
            .map_err(|_| ProvisioningError::GeneratedIdentityInvalid)?;
        let fingerprint = signer.public_spki_sha256_hex();
        drop(signer);
        Ok((private_key, fingerprint))
    }

    fn encrypt_candidate(
        temporary: &Path,
        mut private_key: Zeroizing<Vec<u8>>,
    ) -> Result<(), ProvisioningError> {
        let mut child = Command::new(SYSTEMD_CREDS_BINARY)
            .arg("--user")
            .arg("encrypt")
            .arg(format!("--name={SYSTEMD_DEVICE_IDENTITY_CREDENTIAL_NAME}"))
            .arg("-")
            .arg(temporary)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| ProvisioningError::EncryptLaunchFailed)?;

        let write_result = match child.stdin.take() {
            Some(mut stdin) => stdin.write_all(private_key.as_ref()).and_then(|()| stdin.flush()),
            None => Err(io::Error::other("child stdin unavailable")),
        };
        private_key.clear();
        drop(private_key);

        if write_result.is_err() {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ProvisioningError::EncryptInputFailed);
        }

        let status = child.wait().map_err(|_| ProvisioningError::EncryptFailed)?;
        if !status.success() {
            return Err(ProvisioningError::EncryptFailed);
        }
        Ok(())
    }

    fn validate_and_sync_candidate(path: &Path) -> Result<File, ProvisioningError> {
        let before = fs::symlink_metadata(path).map_err(|_| ProvisioningError::EncryptedCandidateInvalid)?;
        validate_encrypted_metadata(&before)?;

        let owned_fd = open(
            path,
            OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|_| ProvisioningError::EncryptedCandidateInvalid)?;
        let file = File::from(owned_fd);
        let opened = file.metadata().map_err(|_| ProvisioningError::EncryptedCandidateInvalid)?;
        validate_encrypted_metadata(&opened)?;
        if before.dev() != opened.dev() || before.ino() != opened.ino() {
            return Err(ProvisioningError::EncryptedCandidateInvalid);
        }
        file.sync_all().map_err(|_| ProvisioningError::SyncFailed)?;
        Ok(file)
    }

    fn validate_encrypted_metadata(metadata: &fs::Metadata) -> Result<(), ProvisioningError> {
        if !metadata.file_type().is_file()
            || metadata.uid() != geteuid().as_raw()
            || metadata.mode() & 0o777 != ENCRYPTED_FILE_MODE
            || metadata.len() == 0
            || metadata.len() > MAX_ENCRYPTED_CREDENTIAL_BYTES
        {
            return Err(ProvisioningError::EncryptedCandidateInvalid);
        }
        Ok(())
    }

    fn commit_creation_only(
        target: &PreparedTarget,
        opened_candidate: &File,
    ) -> Result<(), ProvisioningError> {
        let opened_metadata = opened_candidate
            .metadata()
            .map_err(|_| ProvisioningError::EncryptedCandidateInvalid)?;

        match fs::hard_link(&target.temporary, &target.final_path) {
            Ok(()) => {}
            Err(_) if target.final_path.exists() => {
                return Err(ProvisioningError::IdentityAlreadyExists);
            }
            Err(_) => return Err(ProvisioningError::CommitFailed),
        }

        let final_metadata = match fs::symlink_metadata(&target.final_path) {
            Ok(metadata) => metadata,
            Err(_) => {
                let _ = fs::remove_file(&target.final_path);
                return Err(ProvisioningError::CommitFailed);
            }
        };
        if validate_encrypted_metadata(&final_metadata).is_err()
            || final_metadata.dev() != opened_metadata.dev()
            || final_metadata.ino() != opened_metadata.ino()
        {
            let _ = fs::remove_file(&target.final_path);
            return Err(ProvisioningError::CommitFailed);
        }

        if fs::remove_file(&target.temporary).is_err() {
            let _ = fs::remove_file(&target.final_path);
            return Err(ProvisioningError::CommitFailed);
        }
        sync_directory(&target.directory)?;
        Ok(())
    }

    fn sync_directory(directory: &Path) -> Result<(), ProvisioningError> {
        File::open(directory)
            .and_then(|file| file.sync_all())
            .map_err(|_| ProvisioningError::SyncFailed)
    }

    fn lowercase_hex(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut encoded = String::with_capacity(bytes.len() * 2);
        for &byte in bytes {
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        encoded
    }

    #[cfg(test)]
    mod tests {
        use std::{
            fs,
            os::unix::fs::{PermissionsExt, symlink},
            path::{Path, PathBuf},
            process,
            sync::atomic::{AtomicU64, Ordering},
        };

        use super::{
            ENCRYPTED_DEVICE_IDENTITY_FILENAME, PRIVATE_DIRECTORY_MODE, ProvisioningError,
            prepare_target_for_state_root,
        };

        static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(1);

        struct TestRoot {
            path: PathBuf,
        }

        impl TestRoot {
            fn new(label: &str) -> Self {
                let id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);
                let path = std::env::temp_dir().join(format!(
                    "prw-phase126-a01-provisioning-{}-{id}-{label}",
                    process::id()
                ));
                fs::create_dir(&path).expect("create isolated provisioning test root");
                fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
                    .expect("secure provisioning test root mode");
                Self { path }
            }

            fn path(&self) -> &Path {
                &self.path
            }
        }

        impl Drop for TestRoot {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.path);
            }
        }

        #[test]
        fn preparation_creates_only_secure_private_directories() {
            let root = TestRoot::new("secure");
            let target = prepare_target_for_state_root(root.path()).expect("prepare secure target");
            let product = root.path().join("private-remote-workspace");
            let credentials = product.join("credentials");

            assert_eq!(
                fs::metadata(product).expect("product metadata").permissions().mode() & 0o777,
                PRIVATE_DIRECTORY_MODE
            );
            assert_eq!(
                fs::metadata(credentials)
                    .expect("credential metadata")
                    .permissions()
                    .mode()
                    & 0o777,
                PRIVATE_DIRECTORY_MODE
            );
            assert_eq!(
                target.final_path.file_name().and_then(|name| name.to_str()),
                Some(ENCRYPTED_DEVICE_IDENTITY_FILENAME)
            );
            assert!(!target.final_path.exists());
            assert!(!target.temporary.exists());
        }

        #[test]
        fn existing_final_identity_is_never_replaced() {
            let root = TestRoot::new("existing");
            let first = prepare_target_for_state_root(root.path()).expect("prepare first target");
            fs::write(&first.final_path, b"existing encrypted identity")
                .expect("write sentinel final identity");
            fs::set_permissions(&first.final_path, fs::Permissions::from_mode(0o600))
                .expect("secure sentinel mode");

            assert_eq!(
                prepare_target_for_state_root(root.path()).unwrap_err(),
                ProvisioningError::IdentityAlreadyExists
            );
            assert_eq!(
                fs::read(&first.final_path).expect("sentinel remains readable"),
                b"existing encrypted identity"
            );
        }

        #[test]
        fn symlinked_product_directory_fails_closed() {
            let root = TestRoot::new("symlink");
            let outside = TestRoot::new("outside");
            symlink(outside.path(), root.path().join("private-remote-workspace"))
                .expect("create malicious product symlink");

            assert_eq!(
                prepare_target_for_state_root(root.path()).unwrap_err(),
                ProvisioningError::DestinationNotSecure
            );
        }

        #[test]
        fn insecure_credential_directory_fails_closed() {
            let root = TestRoot::new("mode");
            let product = root.path().join("private-remote-workspace");
            let credentials = product.join("credentials");
            fs::create_dir(&product).expect("create product directory");
            fs::set_permissions(&product, fs::Permissions::from_mode(0o700))
                .expect("secure product mode");
            fs::create_dir(&credentials).expect("create credentials directory");
            fs::set_permissions(&credentials, fs::Permissions::from_mode(0o770))
                .expect("intentionally insecure credentials mode");

            assert_eq!(
                prepare_target_for_state_root(root.path()).unwrap_err(),
                ProvisioningError::DestinationNotSecure
            );
        }
    }
}
