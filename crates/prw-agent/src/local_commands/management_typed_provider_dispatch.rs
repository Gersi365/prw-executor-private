//! C02c typed provider dispatch seam for admitted local management commands.
//!
//! This module performs only typed provider operations over already-decoded
//! [`prw_remote_bridge::BridgeCommand`] values. It deliberately does not encode local
//! response bytes, enter the production server loop, select host filesystem roots,
//! fabricate remote principals, or construct provider backends from request data.

use std::ptr;

use prw_file_service::{FileServiceError, RemoteDirectoryEntry, RemoteMetadata};
use prw_file_transfer::{FileTransferError, download_chunk};
use prw_forwarding::{ForwardingError, ForwardingPrincipal, PortForwardBackend};
use prw_remote_bridge::BridgeCommand;
use prw_terminal::{TerminalBackend, TerminalError, TerminalPrincipal};

use super::management_authority::LocalManagementFamilyAuthority;
use super::management_dispatch::LocalManagementAuthorityContext;
use super::management_provider_lifecycle::LocalManagementProviderLifecycle;
use super::management_request::LocalManagementAdmission;
use super::status_snapshot::LocalAgentStatusSnapshot;

/// Typed provider result retained before any local response-byte encoding decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalManagementTypedProviderResult {
    /// Existing bounded Agent status snapshot.
    AgentStatus(LocalAgentStatusSnapshot),
    /// Existing descriptor-anchored bounded directory listing.
    DirectoryEntries(Vec<RemoteDirectoryEntry>),
    /// Existing descriptor-anchored bounded metadata snapshot.
    Metadata(RemoteMetadata),
    /// Provider operation completed with no response body selected at this gate.
    Empty,
    /// Transfer operation returned the exact persisted/resume offset.
    Offset(u64),
    /// Provider returned bounded bytes, for download or terminal output.
    Bytes(Vec<u8>),
}

/// Fail-closed typed provider-dispatch failure before response encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalManagementTypedProviderDispatchError {
    /// Supplied family authority does not match the admitted canonical command.
    AuthorityFamilyMismatch,
    /// File/transfer authority is not the exact filesystem authority owned by lifecycle.
    FilesystemAuthorityMismatch,
    /// Terminal/forwarding operation attempted to cross an existing principal binding.
    PrincipalMismatch,
    /// Descriptor-anchored file operation failed.
    File(FileServiceError),
    /// Descriptor-anchored transfer operation failed.
    Transfer(FileTransferError),
    /// Typed terminal broker operation failed.
    Terminal(TerminalError),
    /// Typed forwarding broker operation failed.
    Forwarding(ForwardingError),
}

/// Dispatches one admitted canonical command through already-owned typed providers.
///
/// The function first proves family correlation by constructing the existing C02c
/// request-bound authority context. File/transfer calls additionally require pointer
/// identity with the lifecycle's exact Agent-owned descriptor authority. Terminal and
/// forwarding calls derive principals only from registry-revalidated PRW-session
/// authority and prevent operations against broker records owned by another principal.
///
/// No local response byte representation is chosen here. Successful output remains in
/// [`LocalManagementTypedProviderResult`] for a later reviewed response-encoding gate.
///
/// # Errors
///
/// Fails before provider mutation on authority-family, filesystem-authority, or
/// principal-binding mismatch. Provider errors are retained as typed classifications.
pub(crate) fn dispatch_admitted_management_command<T, F>(
    admission: &LocalManagementAdmission,
    authority: LocalManagementFamilyAuthority<'_>,
    lifecycle: &mut LocalManagementProviderLifecycle<'_, T, F>,
    agent_status: LocalAgentStatusSnapshot,
) -> Result<LocalManagementTypedProviderResult, LocalManagementTypedProviderDispatchError>
where
    T: TerminalBackend,
    F: PortForwardBackend,
{
    LocalManagementAuthorityContext::from_agent_owned_authority(admission, authority)
        .ok_or(LocalManagementTypedProviderDispatchError::AuthorityFamilyMismatch)?;

    match admission.command() {
        BridgeCommand::AgentStatus => Ok(LocalManagementTypedProviderResult::AgentStatus(
            agent_status,
        )),
        BridgeCommand::FileList(path) => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .filesystem()
                .root()
                .list_directory(path)
                .map(LocalManagementTypedProviderResult::DirectoryEntries)
                .map_err(LocalManagementTypedProviderDispatchError::File)
        }
        BridgeCommand::FileStat(path) => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .filesystem()
                .root()
                .metadata(path)
                .map(LocalManagementTypedProviderResult::Metadata)
                .map_err(LocalManagementTypedProviderDispatchError::File)
        }
        BridgeCommand::FileCreate { path, contents } => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .filesystem()
                .root()
                .create_file(path, contents)
                .map(|()| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::File)
        }
        BridgeCommand::DirectoryCreate(path) => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .filesystem()
                .root()
                .create_directory(path)
                .map(|()| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::File)
        }
        BridgeCommand::UploadBegin(plan) => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .transfers_mut()
                .begin(plan.clone())
                .map(LocalManagementTypedProviderResult::Offset)
                .map_err(LocalManagementTypedProviderDispatchError::Transfer)
        }
        BridgeCommand::UploadResume(plan) => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .transfers_mut()
                .resume(plan.clone())
                .map(LocalManagementTypedProviderResult::Offset)
                .map_err(LocalManagementTypedProviderDispatchError::Transfer)
        }
        BridgeCommand::UploadChunk {
            transfer_id,
            offset,
            chunk,
        } => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .transfers_mut()
                .upload_chunk(*transfer_id, *offset, chunk)
                .map(LocalManagementTypedProviderResult::Offset)
                .map_err(LocalManagementTypedProviderDispatchError::Transfer)
        }
        BridgeCommand::UploadFinalize(transfer_id) => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .transfers_mut()
                .finalize(*transfer_id)
                .map(|()| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Transfer)
        }
        BridgeCommand::UploadAbort(transfer_id) => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            lifecycle
                .transfers_mut()
                .abort(*transfer_id)
                .map(|()| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Transfer)
        }
        BridgeCommand::DownloadChunk {
            path,
            offset,
            requested_len,
        } => {
            require_exact_filesystem_authority(authority, lifecycle)?;
            download_chunk(
                lifecycle.filesystem().root(),
                path,
                *offset,
                *requested_len,
            )
            .map(LocalManagementTypedProviderResult::Bytes)
            .map_err(LocalManagementTypedProviderDispatchError::Transfer)
        }
        BridgeCommand::TerminalOpen {
            session_id,
            profile,
            geometry,
        } => {
            let principal = terminal_principal(authority)?;
            if let Some(existing) = lifecycle.terminal().session(*session_id) {
                require_same_terminal_principal(existing.principal(), &principal)?;
            }
            lifecycle
                .terminal_mut()
                .open_session(*session_id, principal, *profile, *geometry)
                .map(|_| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Terminal)
        }
        BridgeCommand::TerminalInput { session_id, bytes } => {
            let principal = terminal_principal(authority)?;
            require_terminal_session_principal(lifecycle, *session_id, &principal)?;
            lifecycle
                .terminal_mut()
                .write_input(*session_id, bytes)
                .map(|()| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Terminal)
        }
        BridgeCommand::TerminalResize {
            session_id,
            geometry,
        } => {
            let principal = terminal_principal(authority)?;
            require_terminal_session_principal(lifecycle, *session_id, &principal)?;
            lifecycle
                .terminal_mut()
                .resize_session(*session_id, *geometry)
                .map(|()| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Terminal)
        }
        BridgeCommand::TerminalRead {
            session_id,
            maximum_bytes,
        } => {
            let principal = terminal_principal(authority)?;
            require_terminal_session_principal(lifecycle, *session_id, &principal)?;
            lifecycle
                .terminal_mut()
                .read_output(*session_id, *maximum_bytes)
                .map(LocalManagementTypedProviderResult::Bytes)
                .map_err(LocalManagementTypedProviderDispatchError::Terminal)
        }
        BridgeCommand::TerminalClose(session_id) => {
            let principal = terminal_principal(authority)?;
            require_terminal_session_principal(lifecycle, *session_id, &principal)?;
            lifecycle
                .terminal_mut()
                .close_session(*session_id)
                .map(|_| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Terminal)
        }
        BridgeCommand::ForwardOpen { forward_id, spec } => {
            let principal = forwarding_principal(authority)?;
            if let Some(existing) = lifecycle.forwarding().session(*forward_id) {
                require_same_forwarding_principal(existing.principal(), &principal)?;
            }
            lifecycle
                .forwarding_mut()
                .open_session(*forward_id, principal, *spec)
                .map(|_| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Forwarding)
        }
        BridgeCommand::ForwardClose(forward_id) => {
            let principal = forwarding_principal(authority)?;
            require_forwarding_session_principal(lifecycle, *forward_id, &principal)?;
            lifecycle
                .forwarding_mut()
                .close_session(*forward_id)
                .map(|_| LocalManagementTypedProviderResult::Empty)
                .map_err(LocalManagementTypedProviderDispatchError::Forwarding)
        }
    }
}

fn require_exact_filesystem_authority<T, F>(
    authority: LocalManagementFamilyAuthority<'_>,
    lifecycle: &LocalManagementProviderLifecycle<'_, T, F>,
) -> Result<(), LocalManagementTypedProviderDispatchError>
where
    T: TerminalBackend,
    F: PortForwardBackend,
{
    let supplied = authority
        .filesystem()
        .ok_or(LocalManagementTypedProviderDispatchError::AuthorityFamilyMismatch)?;
    if ptr::eq(supplied, lifecycle.filesystem()) {
        Ok(())
    } else {
        Err(LocalManagementTypedProviderDispatchError::FilesystemAuthorityMismatch)
    }
}

fn terminal_principal(
    authority: LocalManagementFamilyAuthority<'_>,
) -> Result<TerminalPrincipal, LocalManagementTypedProviderDispatchError> {
    authority
        .remote_session()
        .map(|remote| remote.terminal_principal())
        .ok_or(LocalManagementTypedProviderDispatchError::AuthorityFamilyMismatch)
}

fn forwarding_principal(
    authority: LocalManagementFamilyAuthority<'_>,
) -> Result<ForwardingPrincipal, LocalManagementTypedProviderDispatchError> {
    authority
        .remote_session()
        .map(|remote| remote.forwarding_principal())
        .ok_or(LocalManagementTypedProviderDispatchError::AuthorityFamilyMismatch)
}

fn require_same_terminal_principal(
    existing: &TerminalPrincipal,
    current: &TerminalPrincipal,
) -> Result<(), LocalManagementTypedProviderDispatchError> {
    if existing == current {
        Ok(())
    } else {
        Err(LocalManagementTypedProviderDispatchError::PrincipalMismatch)
    }
}

fn require_terminal_session_principal<T, F>(
    lifecycle: &LocalManagementProviderLifecycle<'_, T, F>,
    session_id: prw_terminal::TerminalSessionId,
    current: &TerminalPrincipal,
) -> Result<(), LocalManagementTypedProviderDispatchError>
where
    T: TerminalBackend,
    F: PortForwardBackend,
{
    let existing = lifecycle
        .terminal()
        .session(session_id)
        .ok_or(LocalManagementTypedProviderDispatchError::Terminal(
            TerminalError::UnknownSession,
        ))?;
    require_same_terminal_principal(existing.principal(), current)
}

fn require_same_forwarding_principal(
    existing: &ForwardingPrincipal,
    current: &ForwardingPrincipal,
) -> Result<(), LocalManagementTypedProviderDispatchError> {
    if existing == current {
        Ok(())
    } else {
        Err(LocalManagementTypedProviderDispatchError::PrincipalMismatch)
    }
}

fn require_forwarding_session_principal<T, F>(
    lifecycle: &LocalManagementProviderLifecycle<'_, T, F>,
    forward_id: prw_forwarding::PortForwardId,
    current: &ForwardingPrincipal,
) -> Result<(), LocalManagementTypedProviderDispatchError>
where
    T: TerminalBackend,
    F: PortForwardBackend,
{
    let existing = lifecycle
        .forwarding()
        .session(forward_id)
        .ok_or(LocalManagementTypedProviderDispatchError::Forwarding(
            ForwardingError::UnknownSession,
        ))?;
    require_same_forwarding_principal(existing.principal(), current)
}
