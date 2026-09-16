//! Linux-only managed user-systemd configuration writer selected by C03e-TV.
//!
//! This module owns only the two PRW-managed non-secret drop-ins. It never reloads or starts a
//! service, changes enablement/linger state, or mutates the vendor unit or identity drop-in.

use std::{
    fmt,
    fs::{self, File, Permissions},
    io::{Read, Write},
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};

use rustix::{
    fs::{AtFlags, FlockOperation, Mode, OFlags, flock, mkdirat, open, openat, renameat, unlinkat},
    io::Errno,
    process::{geteuid, getuid},
};

use crate::{
    AgentExecutionMode, ConfiguredRemoteBundle, PRW_AGENT_EXECUTION_MODE,
    PRW_REMOTE_APPLICATION_LEASE_SECONDS, PRW_REMOTE_BIND_ADDR,
    PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS, PRW_REMOTE_MAX_ACTIVE_WORKERS,
    PRW_REMOTE_PEER_DEVICE_ID, PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS,
    SystemdSerializationError, recognize_configured_remote_drop_in,
    recognize_execution_mode_drop_in, render_configured_remote_drop_in,
    render_execution_mode_drop_in,
};

/// Exact managed mode drop-in filename.
pub const EXECUTION_MODE_DROPIN_NAME: &str = "30-agent-execution-mode.conf";
/// Exact managed configured-remote bundle drop-in filename.
pub const CONFIGURED_REMOTE_INPUTS_DROPIN_NAME: &str = "40-configured-remote-inputs.conf";
/// Separate identity-custody drop-in filename, never owned by this writer.
pub const DEVICE_IDENTITY_DROPIN_NAME: &str = "20-device-identity-credential.conf";

const MANAGED_DIRECTORY_NAME: &str = "prw-agent.service.d";
const USER_UNIT_DIRECTORY_RELATIVE: &str = "systemd/user";
const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
const MANAGED_LEAF_MODE: u32 = 0o600;
const SYSTEMD_ANALYZE_PATH: &str = "/usr/bin/systemd-analyze";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Explicit intended-user filesystem context supplied by a separately gated caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntendedUserSystemdContext {
    uid: u32,
    home_dir: PathBuf,
    xdg_config_root: PathBuf,
}

/// Intended-user context is not absolute/testable as required by C03e-TV.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntendedUserSystemdContextError;

impl fmt::Display for IntendedUserSystemdContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("intended-user systemd context invalid")
    }
}

impl std::error::Error for IntendedUserSystemdContextError {}

impl IntendedUserSystemdContext {
    /// Constructs one explicit intended-user context without consulting ambient HOME/XDG values.
    ///
    /// # Errors
    ///
    /// Rejects non-absolute HOME or XDG configuration roots.
    pub fn new(
        uid: u32,
        home_dir: PathBuf,
        xdg_config_root: PathBuf,
    ) -> Result<Self, IntendedUserSystemdContextError> {
        if !home_dir.is_absolute() || !xdg_config_root.is_absolute() {
            return Err(IntendedUserSystemdContextError);
        }
        Ok(Self {
            uid,
            home_dir,
            xdg_config_root,
        })
    }

    /// Returns the intended numeric user ID.
    #[must_use]
    pub const fn uid(&self) -> u32 {
        self.uid
    }

    /// Returns the explicit intended-user HOME path.
    #[must_use]
    pub fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    /// Returns the explicit effective XDG configuration root.
    #[must_use]
    pub fn xdg_config_root(&self) -> &Path {
        &self.xdg_config_root
    }

    fn user_unit_directory(&self) -> PathBuf {
        self.xdg_config_root.join(USER_UNIT_DIRECTORY_RELATIVE)
    }

    fn managed_directory(&self) -> PathBuf {
        self.user_unit_directory().join(MANAGED_DIRECTORY_NAME)
    }
}

/// Fully validated desired PRW-managed file state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagedAgentConfiguration {
    /// Canonical local-only mode with no managed remote-input bundle.
    LocalOnly,
    /// Canonical configured-remote mode with one complete six-value bundle.
    ConfiguredRemote(Box<ConfiguredRemoteBundle>),
}

impl ManagedAgentConfiguration {
    const fn mode(&self) -> AgentExecutionMode {
        match self {
            Self::LocalOnly => AgentExecutionMode::LocalOnly,
            Self::ConfiguredRemote(_) => AgentExecutionMode::ConfiguredRemote,
        }
    }
}

/// Successful verified managed-file transaction result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedSystemdWriteResult {
    mode: AgentExecutionMode,
}

impl ManagedSystemdWriteResult {
    /// Returns the exact verified desired execution mode.
    #[must_use]
    pub const fn mode(self) -> AgentExecutionMode {
        self.mode
    }
}

/// Bounded failure for the TV-selected managed-file transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ManagedSystemdConfigurationError {
    /// Current real/effective user is not the explicitly intended user.
    IntendedUserMismatch,
    /// Required XDG/systemd parent directory custody is unsafe or unavailable.
    ParentDirectoryCustody,
    /// Managed drop-in directory could not be safely created/opened/validated.
    ManagedDirectoryCustody,
    /// Advisory writer lock could not be acquired.
    WriterLock,
    /// Existing managed leaf metadata is unsafe.
    ManagedLeafCustody,
    /// Existing managed leaf content is not recognized canonical PRW content.
    ForeignManagedLeaf,
    /// Installed systemd tooling could not provide trustworthy user-unit search paths.
    UnitSearchPathDiscovery,
    /// External effective service custody can affect one of the seven selected variables.
    ExternalConfigurationConflict,
    /// Canonical desired-state serialization rejected semantic text.
    Serialization(SystemdSerializationError),
    /// Same-directory temp staging or durability validation failed.
    Staging,
    /// One selected namespace commit/removal failed.
    Commit,
    /// Final reopened desired-state verification failed.
    PostCommitVerification,
    /// Exact in-process managed-file rollback failed.
    Rollback,
}

impl fmt::Display for ManagedSystemdConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::IntendedUserMismatch => "managed systemd intended-user mismatch",
            Self::ParentDirectoryCustody => "managed systemd parent-directory custody failed",
            Self::ManagedDirectoryCustody => "managed systemd directory custody failed",
            Self::WriterLock => "managed systemd writer lock failed",
            Self::ManagedLeafCustody => "managed systemd leaf custody failed",
            Self::ForeignManagedLeaf => "managed systemd leaf content unrecognized",
            Self::UnitSearchPathDiscovery => "systemd user-unit search-path discovery failed",
            Self::ExternalConfigurationConflict => "competing systemd environment custody detected",
            Self::Serialization(_) => "managed systemd serialization failed",
            Self::Staging => "managed systemd staging failed",
            Self::Commit => "managed systemd commit failed",
            Self::PostCommitVerification => "managed systemd post-commit verification failed",
            Self::Rollback => "managed systemd rollback failed",
        })
    }
}

impl std::error::Error for ManagedSystemdConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SystemdSerializationError> for ManagedSystemdConfigurationError {
    fn from(error: SystemdSerializationError) -> Self {
        Self::Serialization(error)
    }
}

/// Commits and verifies only the TV-selected managed `30-`/`40-` file desired state.
///
/// This function performs a read-only `systemd-analyze --user unit-paths` query for effective
/// custody preflight. It never invokes daemon reload, start/restart/stop, enablement or linger APIs.
///
/// # Errors
///
/// Fails closed on intended-user mismatch, unsafe filesystem custody, external competing
/// configuration, serialization/staging/commit/verification failure, or rollback failure.
pub fn apply_managed_systemd_configuration(
    context: &IntendedUserSystemdContext,
    desired: &ManagedAgentConfiguration,
) -> Result<ManagedSystemdWriteResult, ManagedSystemdConfigurationError> {
    validate_current_user(context)?;
    let unit_paths = discover_user_unit_search_paths(context)?;
    let mut event_sink = |_| {};
    apply_with_unit_paths(
        context,
        desired,
        &unit_paths,
        &mut event_sink,
        FaultInjection::None,
    )
}

fn validate_current_user(
    context: &IntendedUserSystemdContext,
) -> Result<(), ManagedSystemdConfigurationError> {
    if getuid().as_raw() != context.uid || geteuid().as_raw() != context.uid {
        return Err(ManagedSystemdConfigurationError::IntendedUserMismatch);
    }
    Ok(())
}

fn discover_user_unit_search_paths(
    context: &IntendedUserSystemdContext,
) -> Result<Vec<PathBuf>, ManagedSystemdConfigurationError> {
    let output = Command::new(SYSTEMD_ANALYZE_PATH)
        .arg("--user")
        .arg("unit-paths")
        .env("HOME", &context.home_dir)
        .env("XDG_CONFIG_HOME", &context.xdg_config_root)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| ManagedSystemdConfigurationError::UnitSearchPathDiscovery)?;
    if !output.status.success() {
        return Err(ManagedSystemdConfigurationError::UnitSearchPathDiscovery);
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|_| ManagedSystemdConfigurationError::UnitSearchPathDiscovery)?;
    let mut paths = Vec::new();
    for line in text.lines() {
        let path = PathBuf::from(line);
        if line.is_empty() || !path.is_absolute() {
            return Err(ManagedSystemdConfigurationError::UnitSearchPathDiscovery);
        }
        if !paths.contains(&path) {
            paths.push(path);
        }
    }
    if paths.is_empty() {
        return Err(ManagedSystemdConfigurationError::UnitSearchPathDiscovery);
    }
    Ok(paths)
}

struct ManagedDirectory {
    file: File,
    path: PathBuf,
}

fn validate_owned_directory(path: &Path, uid: u32, exact_mode: Option<u32>) -> bool {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return false;
    };
    !metadata.file_type().is_symlink()
        && metadata.is_dir()
        && metadata.uid() == uid
        && exact_mode.is_none_or(|mode| metadata.mode() & 0o777 == mode)
}

fn open_managed_directory(
    context: &IntendedUserSystemdContext,
) -> Result<ManagedDirectory, ManagedSystemdConfigurationError> {
    let config_root = context.xdg_config_root();
    let systemd_dir = config_root.join("systemd");
    let user_dir = context.user_unit_directory();
    if !validate_owned_directory(config_root, context.uid, None)
        || !validate_owned_directory(&systemd_dir, context.uid, None)
        || !validate_owned_directory(&user_dir, context.uid, None)
    {
        return Err(ManagedSystemdConfigurationError::ParentDirectoryCustody);
    }

    let user_dir_fd = open(
        &user_dir,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
        Mode::empty(),
    )
    .map_err(|_| ManagedSystemdConfigurationError::ParentDirectoryCustody)?;

    let managed_path = context.managed_directory();
    let created = match fs::symlink_metadata(&managed_path) {
        Ok(_) => false,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match mkdirat(
                &user_dir_fd,
                MANAGED_DIRECTORY_NAME,
                Mode::from_raw_mode(PRIVATE_DIRECTORY_MODE),
            ) {
                Ok(()) => true,
                Err(Errno::EXIST) => false,
                Err(_) => return Err(ManagedSystemdConfigurationError::ManagedDirectoryCustody),
            }
        }
        Err(_) => return Err(ManagedSystemdConfigurationError::ManagedDirectoryCustody),
    };

    let managed_fd = openat(
        &user_dir_fd,
        MANAGED_DIRECTORY_NAME,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
        Mode::empty(),
    )
    .map_err(|_| ManagedSystemdConfigurationError::ManagedDirectoryCustody)?;
    let file = File::from(managed_fd);
    if created {
        file.set_permissions(Permissions::from_mode(PRIVATE_DIRECTORY_MODE))
            .map_err(|_| ManagedSystemdConfigurationError::ManagedDirectoryCustody)?;
    }
    let metadata = file
        .metadata()
        .map_err(|_| ManagedSystemdConfigurationError::ManagedDirectoryCustody)?;
    if !metadata.is_dir()
        || metadata.uid() != context.uid
        || metadata.mode() & 0o777 != PRIVATE_DIRECTORY_MODE
    {
        return Err(ManagedSystemdConfigurationError::ManagedDirectoryCustody);
    }
    flock(&file, FlockOperation::LockExclusive)
        .map_err(|_| ManagedSystemdConfigurationError::WriterLock)?;
    Ok(ManagedDirectory {
        file,
        path: managed_path,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LeafSnapshot {
    bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
enum LeafKind {
    ExecutionMode,
    ConfiguredRemote,
}

const fn validate_leaf_metadata_values(
    is_file: bool,
    uid: u32,
    expected_uid: u32,
    mode: u32,
) -> bool {
    is_file && uid == expected_uid && mode & 0o777 == MANAGED_LEAF_MODE
}

fn snapshot_leaf(
    directory: &ManagedDirectory,
    name: &str,
    kind: LeafKind,
    uid: u32,
) -> Result<LeafSnapshot, ManagedSystemdConfigurationError> {
    let fd = match openat(
        &directory.file,
        name,
        OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
        Mode::empty(),
    ) {
        Ok(fd) => fd,
        Err(Errno::NOENT) => return Ok(LeafSnapshot { bytes: None }),
        Err(_) => return Err(ManagedSystemdConfigurationError::ManagedLeafCustody),
    };
    let mut file = File::from(fd);
    let metadata = file
        .metadata()
        .map_err(|_| ManagedSystemdConfigurationError::ManagedLeafCustody)?;
    if !validate_leaf_metadata_values(metadata.is_file(), metadata.uid(), uid, metadata.mode()) {
        return Err(ManagedSystemdConfigurationError::ManagedLeafCustody);
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|_| ManagedSystemdConfigurationError::ManagedLeafCustody)?;
    let recognized = match kind {
        LeafKind::ExecutionMode => recognize_execution_mode_drop_in(&bytes).map(|_| ()),
        LeafKind::ConfiguredRemote => recognize_configured_remote_drop_in(&bytes).map(|_| ()),
    };
    if recognized.is_err() {
        return Err(ManagedSystemdConfigurationError::ForeignManagedLeaf);
    }
    Ok(LeafSnapshot { bytes: Some(bytes) })
}

fn preflight_external_configuration(
    directory: &ManagedDirectory,
    unit_paths: &[PathBuf],
) -> Result<(), ManagedSystemdConfigurationError> {
    for unit_path in unit_paths {
        let unit = unit_path.join("prw-agent.service");
        match fs::symlink_metadata(&unit) {
            Ok(_) => {
                if external_fragment_conflicts(&unit)? {
                    return Err(ManagedSystemdConfigurationError::ExternalConfigurationConflict);
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => {
                return Err(ManagedSystemdConfigurationError::ExternalConfigurationConflict);
            }
        }

        let dropin_dir = unit_path.join(MANAGED_DIRECTORY_NAME);
        match fs::symlink_metadata(&dropin_dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => {
                return Err(ManagedSystemdConfigurationError::ExternalConfigurationConflict);
            }
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(ManagedSystemdConfigurationError::ExternalConfigurationConflict);
            }
            Ok(_) => {}
        }
        let entries = fs::read_dir(&dropin_dir)
            .map_err(|_| ManagedSystemdConfigurationError::ExternalConfigurationConflict)?;
        for entry in entries {
            let entry = entry
                .map_err(|_| ManagedSystemdConfigurationError::ExternalConfigurationConflict)?;
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if dropin_dir == directory.path
                && matches!(
                    name.as_ref(),
                    EXECUTION_MODE_DROPIN_NAME | CONFIGURED_REMOTE_INPUTS_DROPIN_NAME
                )
            {
                continue;
            }
            if external_fragment_conflicts(&path)? {
                return Err(ManagedSystemdConfigurationError::ExternalConfigurationConflict);
            }
        }
    }
    Ok(())
}

fn external_fragment_conflicts(path: &Path) -> Result<bool, ManagedSystemdConfigurationError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| ManagedSystemdConfigurationError::ExternalConfigurationConflict)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Ok(true);
    }
    let text = fs::read_to_string(path)
        .map_err(|_| ManagedSystemdConfigurationError::ExternalConfigurationConflict)?;
    Ok(fragment_text_conflicts(&text))
}

fn fragment_text_conflicts(text: &str) -> bool {
    const SELECTED: [&str; 7] = [
        PRW_AGENT_EXECUTION_MODE,
        PRW_REMOTE_BIND_ADDR,
        PRW_REMOTE_PEER_DEVICE_ID,
        PRW_REMOTE_MAX_ACTIVE_WORKERS,
        PRW_REMOTE_APPLICATION_LEASE_SECONDS,
        PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS,
        PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS,
    ];
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if line.ends_with('\\') {
            return true;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if key == "EnvironmentFile" {
            return true;
        }
        if matches!(key, "Environment" | "UnsetEnvironment" | "PassEnvironment")
            && SELECTED.iter().any(|name| value.contains(name))
        {
            return true;
        }
    }
    false
}

struct StagedFile {
    name: String,
}

fn next_temp_name(label: &str) -> String {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!(
        ".prw-agent-configuration.{label}.{}.{}.tmp",
        std::process::id(),
        sequence
    )
}

fn stage_bytes(
    directory: &ManagedDirectory,
    label: &str,
    bytes: &[u8],
    uid: u32,
) -> Result<StagedFile, ManagedSystemdConfigurationError> {
    let name = next_temp_name(label);
    let fd = openat(
        &directory.file,
        name.as_str(),
        OFlags::CREATE | OFlags::EXCL | OFlags::WRONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
        Mode::from_raw_mode(MANAGED_LEAF_MODE),
    )
    .map_err(|_| ManagedSystemdConfigurationError::Staging)?;
    let mut file = File::from(fd);
    let stage_result = (|| {
        file.set_permissions(Permissions::from_mode(MANAGED_LEAF_MODE))?;
        file.write_all(bytes)?;
        file.sync_all()?;
        let metadata = file.metadata()?;
        if !validate_leaf_metadata_values(metadata.is_file(), metadata.uid(), uid, metadata.mode())
        {
            return Err(std::io::Error::other("staged metadata invalid"));
        }
        Ok::<(), std::io::Error>(())
    })();
    drop(file);
    if stage_result.is_err() {
        let _ = unlinkat(&directory.file, name.as_str(), AtFlags::empty());
        return Err(ManagedSystemdConfigurationError::Staging);
    }
    let reopened = openat(
        &directory.file,
        name.as_str(),
        OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
        Mode::empty(),
    )
    .map_err(|_| ManagedSystemdConfigurationError::Staging)?;
    let mut reopened = File::from(reopened);
    let mut readback = Vec::new();
    reopened
        .read_to_end(&mut readback)
        .map_err(|_| ManagedSystemdConfigurationError::Staging)?;
    if readback != bytes {
        let _ = unlinkat(&directory.file, name.as_str(), AtFlags::empty());
        return Err(ManagedSystemdConfigurationError::Staging);
    }
    Ok(StagedFile { name })
}

fn cleanup_stage(directory: &ManagedDirectory, staged: Option<&StagedFile>) {
    if let Some(staged) = staged {
        let _ = unlinkat(&directory.file, staged.name.as_str(), AtFlags::empty());
    }
}

fn commit_stage(
    directory: &ManagedDirectory,
    staged: &StagedFile,
    destination: &str,
) -> Result<(), ManagedSystemdConfigurationError> {
    renameat(
        &directory.file,
        staged.name.as_str(),
        &directory.file,
        destination,
    )
    .map_err(|_| ManagedSystemdConfigurationError::Commit)?;
    directory
        .file
        .sync_all()
        .map_err(|_| ManagedSystemdConfigurationError::Commit)
}

fn remove_leaf(
    directory: &ManagedDirectory,
    name: &str,
) -> Result<(), ManagedSystemdConfigurationError> {
    unlinkat(&directory.file, name, AtFlags::empty())
        .map_err(|_| ManagedSystemdConfigurationError::Commit)?;
    directory
        .file
        .sync_all()
        .map_err(|_| ManagedSystemdConfigurationError::Commit)
}

fn verify_desired_state(
    directory: &ManagedDirectory,
    desired: &ManagedAgentConfiguration,
    uid: u32,
) -> Result<(), ManagedSystemdConfigurationError> {
    let mode = snapshot_leaf(
        directory,
        EXECUTION_MODE_DROPIN_NAME,
        LeafKind::ExecutionMode,
        uid,
    )
    .map_err(|_| ManagedSystemdConfigurationError::PostCommitVerification)?;
    let expected_mode = render_execution_mode_drop_in(desired.mode()).as_bytes();
    if mode.bytes.as_deref() != Some(expected_mode) {
        return Err(ManagedSystemdConfigurationError::PostCommitVerification);
    }
    let remote = snapshot_leaf(
        directory,
        CONFIGURED_REMOTE_INPUTS_DROPIN_NAME,
        LeafKind::ConfiguredRemote,
        uid,
    )
    .map_err(|_| ManagedSystemdConfigurationError::PostCommitVerification)?;
    match desired {
        ManagedAgentConfiguration::LocalOnly => {
            if remote.bytes.is_some() {
                return Err(ManagedSystemdConfigurationError::PostCommitVerification);
            }
        }
        ManagedAgentConfiguration::ConfiguredRemote(bundle) => {
            let expected = render_configured_remote_drop_in(bundle)
                .map_err(ManagedSystemdConfigurationError::Serialization)?;
            if remote.bytes.as_deref() != Some(expected.as_bytes()) {
                return Err(ManagedSystemdConfigurationError::PostCommitVerification);
            }
        }
    }
    Ok(())
}

fn restore_leaf(
    directory: &ManagedDirectory,
    name: &str,
    kind: LeafKind,
    snapshot: &LeafSnapshot,
    uid: u32,
) -> Result<(), ManagedSystemdConfigurationError> {
    let current = snapshot_leaf(directory, name, kind, uid);
    match (&snapshot.bytes, current) {
        (None, Ok(LeafSnapshot { bytes: None })) => Ok(()),
        (None, Ok(_)) => {
            unlinkat(&directory.file, name, AtFlags::empty())
                .map_err(|_| ManagedSystemdConfigurationError::Rollback)?;
            directory
                .file
                .sync_all()
                .map_err(|_| ManagedSystemdConfigurationError::Rollback)
        }
        (Some(bytes), Ok(_)) => {
            let staged = stage_bytes(directory, "rollback", bytes, uid)
                .map_err(|_| ManagedSystemdConfigurationError::Rollback)?;
            renameat(&directory.file, staged.name.as_str(), &directory.file, name)
                .map_err(|_| ManagedSystemdConfigurationError::Rollback)?;
            directory
                .file
                .sync_all()
                .map_err(|_| ManagedSystemdConfigurationError::Rollback)
        }
        (_, Err(_)) => Err(ManagedSystemdConfigurationError::Rollback),
    }
}

fn rollback(
    directory: &ManagedDirectory,
    desired: &ManagedAgentConfiguration,
    before_mode: &LeafSnapshot,
    before_remote: &LeafSnapshot,
    uid: u32,
    fault: FaultInjection,
) -> Result<(), ManagedSystemdConfigurationError> {
    if matches!(fault, FaultInjection::RollbackFailure) {
        return Err(ManagedSystemdConfigurationError::Rollback);
    }
    match desired {
        ManagedAgentConfiguration::ConfiguredRemote(_) => {
            restore_leaf(
                directory,
                EXECUTION_MODE_DROPIN_NAME,
                LeafKind::ExecutionMode,
                before_mode,
                uid,
            )?;
            restore_leaf(
                directory,
                CONFIGURED_REMOTE_INPUTS_DROPIN_NAME,
                LeafKind::ConfiguredRemote,
                before_remote,
                uid,
            )?;
        }
        ManagedAgentConfiguration::LocalOnly => {
            restore_leaf(
                directory,
                CONFIGURED_REMOTE_INPUTS_DROPIN_NAME,
                LeafKind::ConfiguredRemote,
                before_remote,
                uid,
            )?;
            restore_leaf(
                directory,
                EXECUTION_MODE_DROPIN_NAME,
                LeafKind::ExecutionMode,
                before_mode,
                uid,
            )?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WriterEvent {
    CommitRemoteInputs,
    CommitExecutionMode,
    RemoveRemoteInputs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "C03e-TW deterministic fault injection is exercised only by unit tests"
    )
)]
enum FaultInjection {
    None,
    AfterFirstCommit,
    PostCommitVerification,
    RollbackFailure,
}

fn fail_with_rollback(
    directory: &ManagedDirectory,
    desired: &ManagedAgentConfiguration,
    before_mode: &LeafSnapshot,
    before_remote: &LeafSnapshot,
    uid: u32,
    cause: ManagedSystemdConfigurationError,
    fault: FaultInjection,
) -> ManagedSystemdConfigurationError {
    match rollback(directory, desired, before_mode, before_remote, uid, fault) {
        Ok(()) => cause,
        Err(_) => ManagedSystemdConfigurationError::Rollback,
    }
}

fn commit_desired_state(
    directory: &ManagedDirectory,
    desired: &ManagedAgentConfiguration,
    before_remote: &LeafSnapshot,
    mode_stage: &mut Option<StagedFile>,
    remote_stage: &mut Option<StagedFile>,
    event_sink: &mut impl FnMut(WriterEvent),
    fault: FaultInjection,
) -> Result<(), ManagedSystemdConfigurationError> {
    match desired {
        ManagedAgentConfiguration::ConfiguredRemote(_) => {
            let remote = remote_stage.take().expect("configured remote stage exists");
            commit_stage(directory, &remote, CONFIGURED_REMOTE_INPUTS_DROPIN_NAME)?;
            event_sink(WriterEvent::CommitRemoteInputs);
            if matches!(
                fault,
                FaultInjection::AfterFirstCommit | FaultInjection::RollbackFailure
            ) {
                return Err(ManagedSystemdConfigurationError::Commit);
            }
            let mode = mode_stage.take().expect("mode stage exists");
            commit_stage(directory, &mode, EXECUTION_MODE_DROPIN_NAME)?;
            event_sink(WriterEvent::CommitExecutionMode);
        }
        ManagedAgentConfiguration::LocalOnly => {
            let mode = mode_stage.take().expect("mode stage exists");
            commit_stage(directory, &mode, EXECUTION_MODE_DROPIN_NAME)?;
            event_sink(WriterEvent::CommitExecutionMode);
            if matches!(
                fault,
                FaultInjection::AfterFirstCommit | FaultInjection::RollbackFailure
            ) {
                return Err(ManagedSystemdConfigurationError::Commit);
            }
            if before_remote.bytes.is_some() {
                remove_leaf(directory, CONFIGURED_REMOTE_INPUTS_DROPIN_NAME)?;
                event_sink(WriterEvent::RemoveRemoteInputs);
            }
        }
    }
    Ok(())
}

fn apply_with_unit_paths(
    context: &IntendedUserSystemdContext,
    desired: &ManagedAgentConfiguration,
    unit_paths: &[PathBuf],
    event_sink: &mut impl FnMut(WriterEvent),
    fault: FaultInjection,
) -> Result<ManagedSystemdWriteResult, ManagedSystemdConfigurationError> {
    validate_current_user(context)?;
    let directory = open_managed_directory(context)?;
    let before_mode = snapshot_leaf(
        &directory,
        EXECUTION_MODE_DROPIN_NAME,
        LeafKind::ExecutionMode,
        context.uid,
    )?;
    let before_remote = snapshot_leaf(
        &directory,
        CONFIGURED_REMOTE_INPUTS_DROPIN_NAME,
        LeafKind::ConfiguredRemote,
        context.uid,
    )?;
    preflight_external_configuration(&directory, unit_paths)?;

    let mode_bytes = render_execution_mode_drop_in(desired.mode()).as_bytes();
    let remote_rendered = match desired {
        ManagedAgentConfiguration::LocalOnly => None,
        ManagedAgentConfiguration::ConfiguredRemote(bundle) => {
            Some(render_configured_remote_drop_in(bundle)?)
        }
    };

    let mut mode_stage = Some(stage_bytes(&directory, "mode", mode_bytes, context.uid)?);
    let mut remote_stage = match remote_rendered.as_deref() {
        Some(bytes) => match stage_bytes(&directory, "remote", bytes.as_bytes(), context.uid) {
            Ok(staged) => Some(staged),
            Err(error) => {
                cleanup_stage(&directory, mode_stage.as_ref());
                return Err(error);
            }
        },
        None => None,
    };

    let current_mode = snapshot_leaf(
        &directory,
        EXECUTION_MODE_DROPIN_NAME,
        LeafKind::ExecutionMode,
        context.uid,
    );
    let current_remote = snapshot_leaf(
        &directory,
        CONFIGURED_REMOTE_INPUTS_DROPIN_NAME,
        LeafKind::ConfiguredRemote,
        context.uid,
    );
    if current_mode.as_ref().ok() != Some(&before_mode)
        || current_remote.as_ref().ok() != Some(&before_remote)
    {
        cleanup_stage(&directory, mode_stage.as_ref());
        cleanup_stage(&directory, remote_stage.as_ref());
        return Err(ManagedSystemdConfigurationError::ManagedLeafCustody);
    }

    let commit_result = commit_desired_state(
        &directory,
        desired,
        &before_remote,
        &mut mode_stage,
        &mut remote_stage,
        event_sink,
        fault,
    );
    cleanup_stage(&directory, mode_stage.as_ref());
    cleanup_stage(&directory, remote_stage.as_ref());
    if let Err(error) = commit_result {
        return Err(fail_with_rollback(
            &directory,
            desired,
            &before_mode,
            &before_remote,
            context.uid,
            error,
            fault,
        ));
    }

    if matches!(fault, FaultInjection::PostCommitVerification) {
        return Err(fail_with_rollback(
            &directory,
            desired,
            &before_mode,
            &before_remote,
            context.uid,
            ManagedSystemdConfigurationError::PostCommitVerification,
            fault,
        ));
    }
    if let Err(error) = verify_desired_state(&directory, desired, context.uid) {
        return Err(fail_with_rollback(
            &directory,
            desired,
            &before_mode,
            &before_remote,
            context.uid,
            error,
            fault,
        ));
    }

    Ok(ManagedSystemdWriteResult {
        mode: desired.mode(),
    })
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        fs,
        os::unix::fs::{PermissionsExt, symlink},
        path::PathBuf,
        rc::Rc,
    };

    use rustix::process::getuid;

    use super::*;

    struct Sandbox {
        root: PathBuf,
        context: IntendedUserSystemdContext,
        user_unit: PathBuf,
        managed: PathBuf,
    }

    impl Sandbox {
        fn new() -> Self {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "prw-agent-configuration-test-{}-{sequence}",
                std::process::id()
            ));
            let config = root.join("config");
            let user_unit = config.join("systemd/user");
            fs::create_dir_all(&user_unit).expect("test user unit directory");
            for path in [&root, &config, &config.join("systemd"), &user_unit] {
                fs::set_permissions(path, fs::Permissions::from_mode(0o700))
                    .expect("test directory mode");
            }
            let uid = getuid().as_raw();
            let context = IntendedUserSystemdContext::new(uid, root.clone(), config)
                .expect("absolute intended-user context");
            let managed = user_unit.join(MANAGED_DIRECTORY_NAME);
            Self {
                root,
                context,
                user_unit,
                managed,
            }
        }

        fn unit_paths(&self) -> Vec<PathBuf> {
            vec![self.user_unit.clone()]
        }

        fn ensure_managed(&self) {
            fs::create_dir_all(&self.managed).expect("managed test directory");
            fs::set_permissions(&self.managed, fs::Permissions::from_mode(0o700))
                .expect("managed directory mode");
        }

        fn write_managed(&self, name: &str, bytes: &[u8]) {
            self.ensure_managed();
            let path = self.managed.join(name);
            fs::write(&path, bytes).expect("managed test file");
            fs::set_permissions(path, fs::Permissions::from_mode(0o600))
                .expect("managed leaf mode");
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn bundle(peer: &str) -> ConfiguredRemoteBundle {
        ConfiguredRemoteBundle::try_new("127.0.0.1:0", peer, "0002", "003600", "0007", "0009")
            .expect("valid test bundle")
    }

    fn apply_test(
        sandbox: &Sandbox,
        desired: &ManagedAgentConfiguration,
        fault: FaultInjection,
    ) -> (
        Result<ManagedSystemdWriteResult, ManagedSystemdConfigurationError>,
        Vec<WriterEvent>,
    ) {
        let events = Rc::new(RefCell::new(Vec::new()));
        let sink_events = Rc::clone(&events);
        let mut sink = move |event| sink_events.borrow_mut().push(event);
        let result = apply_with_unit_paths(
            &sandbox.context,
            desired,
            &sandbox.unit_paths(),
            &mut sink,
            fault,
        );
        let captured = events.borrow().clone();
        (result, captured)
    }

    #[test]
    fn configured_remote_commits_complete_40_before_configured_30() {
        let sandbox = Sandbox::new();
        let desired = ManagedAgentConfiguration::ConfiguredRemote(Box::new(bundle("peer-1")));
        let (result, events) = apply_test(&sandbox, &desired, FaultInjection::None);
        assert_eq!(
            result.expect("configured transaction").mode(),
            AgentExecutionMode::ConfiguredRemote
        );
        assert_eq!(
            events,
            [
                WriterEvent::CommitRemoteInputs,
                WriterEvent::CommitExecutionMode
            ]
        );
        assert_eq!(
            fs::read(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("mode bytes"),
            render_execution_mode_drop_in(AgentExecutionMode::ConfiguredRemote).as_bytes()
        );
        let expected_remote = render_configured_remote_drop_in(&bundle("peer-1")).expect("render");
        assert_eq!(
            fs::read(sandbox.managed.join(CONFIGURED_REMOTE_INPUTS_DROPIN_NAME))
                .expect("remote bytes"),
            expected_remote.as_bytes()
        );
    }

    #[test]
    fn local_only_commits_30_before_recognized_40_removal() {
        let sandbox = Sandbox::new();
        sandbox.write_managed(
            EXECUTION_MODE_DROPIN_NAME,
            render_execution_mode_drop_in(AgentExecutionMode::ConfiguredRemote).as_bytes(),
        );
        let remote = render_configured_remote_drop_in(&bundle("peer-2")).expect("render");
        sandbox.write_managed(CONFIGURED_REMOTE_INPUTS_DROPIN_NAME, remote.as_bytes());
        let (result, events) = apply_test(
            &sandbox,
            &ManagedAgentConfiguration::LocalOnly,
            FaultInjection::None,
        );
        assert_eq!(
            result.expect("local transaction").mode(),
            AgentExecutionMode::LocalOnly
        );
        assert_eq!(
            events,
            [
                WriterEvent::CommitExecutionMode,
                WriterEvent::RemoveRemoteInputs
            ]
        );
        assert!(
            !sandbox
                .managed
                .join(CONFIGURED_REMOTE_INPUTS_DROPIN_NAME)
                .exists()
        );
    }

    #[test]
    fn external_conflict_fails_before_managed_leaf_mutation() {
        let sandbox = Sandbox::new();
        sandbox.write_managed(
            EXECUTION_MODE_DROPIN_NAME,
            render_execution_mode_drop_in(AgentExecutionMode::LocalOnly).as_bytes(),
        );
        fs::write(
            sandbox.managed.join("50-external.conf"),
            "[Service]\nEnvironment=PRW_REMOTE_BIND_ADDR=127.0.0.1:9\n",
        )
        .expect("external conflict");
        let before = fs::read(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("before");
        let (result, events) = apply_test(
            &sandbox,
            &ManagedAgentConfiguration::ConfiguredRemote(Box::new(bundle("peer-3"))),
            FaultInjection::None,
        );
        assert_eq!(
            result,
            Err(ManagedSystemdConfigurationError::ExternalConfigurationConflict)
        );
        assert!(events.is_empty());
        assert_eq!(
            fs::read(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("after"),
            before
        );
        assert!(
            !sandbox
                .managed
                .join(CONFIGURED_REMOTE_INPUTS_DROPIN_NAME)
                .exists()
        );
    }

    #[test]
    fn foreign_symlink_nonregular_and_wrong_mode_managed_leaves_fail_closed() {
        let sandbox = Sandbox::new();
        sandbox.ensure_managed();
        let target = sandbox.root.join("outside");
        fs::write(&target, "outside").expect("outside file");
        symlink(&target, sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("symlink");
        let (symlink_result, _) = apply_test(
            &sandbox,
            &ManagedAgentConfiguration::LocalOnly,
            FaultInjection::None,
        );
        assert_eq!(
            symlink_result,
            Err(ManagedSystemdConfigurationError::ManagedLeafCustody)
        );
        fs::remove_file(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("remove symlink");
        fs::create_dir(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("directory leaf");
        let (directory_result, _) = apply_test(
            &sandbox,
            &ManagedAgentConfiguration::LocalOnly,
            FaultInjection::None,
        );
        assert_eq!(
            directory_result,
            Err(ManagedSystemdConfigurationError::ManagedLeafCustody)
        );
        fs::remove_dir(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("remove directory");
        sandbox.write_managed(
            EXECUTION_MODE_DROPIN_NAME,
            render_execution_mode_drop_in(AgentExecutionMode::LocalOnly).as_bytes(),
        );
        fs::set_permissions(
            sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME),
            fs::Permissions::from_mode(0o644),
        )
        .expect("wrong mode");
        let (mode_result, _) = apply_test(
            &sandbox,
            &ManagedAgentConfiguration::LocalOnly,
            FaultInjection::None,
        );
        assert_eq!(
            mode_result,
            Err(ManagedSystemdConfigurationError::ManagedLeafCustody)
        );
        assert!(!validate_leaf_metadata_values(
            true,
            sandbox.context.uid + 1,
            sandbox.context.uid,
            0o600
        ));
    }

    #[test]
    fn foreign_canonical_shape_mismatch_is_not_taken_over() {
        let sandbox = Sandbox::new();
        sandbox.write_managed(
            EXECUTION_MODE_DROPIN_NAME,
            b"[Service]\nEnvironment=PRW_AGENT_EXECUTION_MODE=LOCAL_ONLY\n",
        );
        let (result, _) = apply_test(
            &sandbox,
            &ManagedAgentConfiguration::LocalOnly,
            FaultInjection::None,
        );
        assert_eq!(
            result,
            Err(ManagedSystemdConfigurationError::ForeignManagedLeaf)
        );
    }

    #[test]
    fn injected_second_step_failure_restores_exact_prestate() {
        let sandbox = Sandbox::new();
        sandbox.write_managed(
            EXECUTION_MODE_DROPIN_NAME,
            render_execution_mode_drop_in(AgentExecutionMode::LocalOnly).as_bytes(),
        );
        let before_mode =
            fs::read(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("before");
        let desired = ManagedAgentConfiguration::ConfiguredRemote(Box::new(bundle("peer-4")));
        let (result, events) = apply_test(&sandbox, &desired, FaultInjection::AfterFirstCommit);
        assert_eq!(result, Err(ManagedSystemdConfigurationError::Commit));
        assert_eq!(events, [WriterEvent::CommitRemoteInputs]);
        assert_eq!(
            fs::read(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("mode restored"),
            before_mode
        );
        assert!(
            !sandbox
                .managed
                .join(CONFIGURED_REMOTE_INPUTS_DROPIN_NAME)
                .exists()
        );
    }

    #[test]
    fn postcommit_verification_failure_rolls_back_both_managed_leaves() {
        let sandbox = Sandbox::new();
        sandbox.write_managed(
            EXECUTION_MODE_DROPIN_NAME,
            render_execution_mode_drop_in(AgentExecutionMode::LocalOnly).as_bytes(),
        );
        let before = fs::read(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("before");
        let desired = ManagedAgentConfiguration::ConfiguredRemote(Box::new(bundle("peer-5")));
        let (result, _) = apply_test(&sandbox, &desired, FaultInjection::PostCommitVerification);
        assert_eq!(
            result,
            Err(ManagedSystemdConfigurationError::PostCommitVerification)
        );
        assert_eq!(
            fs::read(sandbox.managed.join(EXECUTION_MODE_DROPIN_NAME)).expect("restored mode"),
            before
        );
        assert!(
            !sandbox
                .managed
                .join(CONFIGURED_REMOTE_INPUTS_DROPIN_NAME)
                .exists()
        );
    }

    #[test]
    fn rollback_failure_has_distinct_bounded_classification() {
        let sandbox = Sandbox::new();
        sandbox.write_managed(
            EXECUTION_MODE_DROPIN_NAME,
            render_execution_mode_drop_in(AgentExecutionMode::LocalOnly).as_bytes(),
        );
        let desired = ManagedAgentConfiguration::ConfiguredRemote(Box::new(bundle("peer-6")));
        let (result, events) = apply_test(&sandbox, &desired, FaultInjection::RollbackFailure);
        assert_eq!(result, Err(ManagedSystemdConfigurationError::Rollback));
        assert_eq!(events, [WriterEvent::CommitRemoteInputs]);
    }

    #[test]
    fn serialization_failure_occurs_before_any_temp_or_managed_leaf_stage() {
        let sandbox = Sandbox::new();
        let desired =
            ManagedAgentConfiguration::ConfiguredRemote(Box::new(bundle("peer\ncontrol")));
        let (result, events) = apply_test(&sandbox, &desired, FaultInjection::None);
        assert_eq!(
            result,
            Err(ManagedSystemdConfigurationError::Serialization(
                SystemdSerializationError::NonPrintableValue
            ))
        );
        assert!(events.is_empty());
        let entries = fs::read_dir(&sandbox.managed)
            .expect("managed directory exists after custody validation")
            .collect::<Result<Vec<_>, _>>()
            .expect("managed directory readable");
        assert!(entries.is_empty());
    }

    #[test]
    fn non_directory_external_unit_path_fails_closed() {
        let sandbox = Sandbox::new();
        let invalid_unit_path = sandbox.root.join("not-a-directory");
        fs::write(&invalid_unit_path, "not a unit search directory").expect("test file");
        let mut events = Vec::new();
        let result = apply_with_unit_paths(
            &sandbox.context,
            &ManagedAgentConfiguration::LocalOnly,
            &[invalid_unit_path],
            &mut |event| events.push(event),
            FaultInjection::None,
        );
        assert_eq!(
            result,
            Err(ManagedSystemdConfigurationError::ExternalConfigurationConflict)
        );
        assert!(events.is_empty());
    }

    #[test]
    fn fragment_preflight_rejects_selected_custody_and_ambiguous_environment_file() {
        assert!(fragment_text_conflicts(
            "[Service]\nEnvironment=PRW_AGENT_EXECUTION_MODE=local_only\n"
        ));
        assert!(fragment_text_conflicts(
            "[Service]\nUnsetEnvironment=PRW_REMOTE_BIND_ADDR\n"
        ));
        assert!(fragment_text_conflicts(
            "[Service]\nPassEnvironment=PRW_REMOTE_PEER_DEVICE_ID\n"
        ));
        assert!(fragment_text_conflicts(
            "[Service]\nEnvironmentFile=-/etc/prw/environment\n"
        ));
        assert!(fragment_text_conflicts(
            "[Service]\nEnvironmentFile = -/etc/prw/environment\n"
        ));
        assert!(fragment_text_conflicts(
            "[Service]\nEnvironment=UNRELATED=value \\\n PRW_AGENT_EXECUTION_MODE=configured_remote\n"
        ));
        assert!(!fragment_text_conflicts(
            "[Service]\nEnvironment=UNRELATED=value\nLoadCredentialEncrypted=identity:/path\n"
        ));
    }

    #[test]
    fn context_rejects_relative_roots() {
        assert_eq!(
            IntendedUserSystemdContext::new(1, PathBuf::from("relative"), PathBuf::from("/abs")),
            Err(IntendedUserSystemdContextError)
        );
    }

    #[test]
    fn systemd_recognition_error_remains_separate_from_writer_error_surface() {
        let error = recognize_execution_mode_drop_in(b"not-canonical").expect_err("not canonical");
        assert_eq!(error, crate::SystemdRecognitionError::NonCanonical);
    }
}
