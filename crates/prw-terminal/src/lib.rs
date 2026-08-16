//! Typed terminal-session foundation for Private Remote Workspace.
//!
//! Phase 133 defines bounded lifecycle and backend contracts only. It does not expose
//! an arbitrary run-command API and does not spawn a PTY or shell by itself.

use std::fmt;

use prw_core::{DeviceId, SessionId, UserId, WorkspaceId};
use prw_registry::RegistryValidatedPrincipal;

/// Maximum accepted terminal input or output chunk.
pub const MAX_TERMINAL_IO_BYTES: usize = 65_536;
/// Maximum terminal geometry dimension.
pub const MAX_TERMINAL_DIMENSION: u16 = 1_000;

/// Stable terminal-session identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalSessionId(u64);

impl TerminalSessionId {
    /// Creates a non-zero terminal-session identifier.
    pub const fn new(value: u64) -> Result<Self, TerminalError> {
        if value == 0 { Err(TerminalError::InvalidIdentifier) } else { Ok(Self(value)) }
    }

    /// Returns the raw identifier.
    #[must_use]
    pub const fn get(self) -> u64 { self.0 }
}

/// Allowed named terminal profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalProfile { PosixShell, BashShell }

/// Bounded terminal geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalGeometry { columns: u16, rows: u16 }

impl TerminalGeometry {
    /// Creates valid non-zero geometry bounded to 1000x1000.
    pub const fn new(columns: u16, rows: u16) -> Result<Self, TerminalError> {
        if columns == 0 || rows == 0 || columns > MAX_TERMINAL_DIMENSION || rows > MAX_TERMINAL_DIMENSION {
            return Err(TerminalError::InvalidGeometry);
        }
        Ok(Self { columns, rows })
    }
    #[must_use] pub const fn columns(self) -> u16 { self.columns }
    #[must_use] pub const fn rows(self) -> u16 { self.rows }
}

/// Registry-current identity snapshot attached to a terminal session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalPrincipal {
    workspace_id: WorkspaceId,
    user_id: UserId,
    device_id: DeviceId,
    authenticated_session_id: SessionId,
}

impl TerminalPrincipal {
    /// Captures current-registry identity plus the authenticated session id.
    #[must_use]
    pub fn from_registry(principal: &RegistryValidatedPrincipal, session_id: SessionId) -> Self {
        Self {
            workspace_id: principal.workspace_id().clone(),
            user_id: principal.user_id().clone(),
            device_id: principal.device_id().clone(),
            authenticated_session_id: session_id,
        }
    }
    #[must_use] pub const fn workspace_id(&self) -> &WorkspaceId { &self.workspace_id }
    #[must_use] pub const fn user_id(&self) -> &UserId { &self.user_id }
    #[must_use] pub const fn device_id(&self) -> &DeviceId { &self.device_id }
    #[must_use] pub const fn authenticated_session_id(&self) -> &SessionId { &self.authenticated_session_id }
}

/// Terminal lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalState { Opening, Open, Closing, Closed, Failed }

/// Typed backend contract. No executable path, argument vector, raw command string,
/// environment injection, filesystem path, network target, or privilege instruction is accepted.
pub trait TerminalBackend {
    type Handle;
    fn open(&mut self, profile: TerminalProfile, geometry: TerminalGeometry) -> Result<Self::Handle, TerminalError>;
    fn write_input(&mut self, handle: &mut Self::Handle, bytes: &[u8]) -> Result<(), TerminalError>;
    fn resize(&mut self, handle: &mut Self::Handle, geometry: TerminalGeometry) -> Result<(), TerminalError>;
    fn read_output(&mut self, handle: &mut Self::Handle, maximum_bytes: usize) -> Result<Vec<u8>, TerminalError>;
    fn close(&mut self, handle: Self::Handle) -> Result<(), TerminalError>;
}

/// Provider-neutral terminal session record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSession {
    id: TerminalSessionId,
    principal: TerminalPrincipal,
    profile: TerminalProfile,
    geometry: TerminalGeometry,
    state: TerminalState,
}

impl TerminalSession {
    #[must_use]
    pub const fn new(id: TerminalSessionId, principal: TerminalPrincipal, profile: TerminalProfile, geometry: TerminalGeometry) -> Self {
        Self { id, principal, profile, geometry, state: TerminalState::Opening }
    }
    #[must_use] pub const fn id(&self) -> TerminalSessionId { self.id }
    #[must_use] pub const fn state(&self) -> TerminalState { self.state }
    #[must_use] pub const fn principal(&self) -> &TerminalPrincipal { &self.principal }
    #[must_use] pub const fn profile(&self) -> TerminalProfile { self.profile }
    #[must_use] pub const fn geometry(&self) -> TerminalGeometry { self.geometry }

    /// Marks successful backend opening.
    pub fn mark_open(&mut self) -> Result<(), TerminalError> {
        if self.state != TerminalState::Opening { return Err(TerminalError::InvalidState); }
        self.state = TerminalState::Open;
        Ok(())
    }

    /// Validates bounded input while open.
    pub fn validate_input(&self, bytes: &[u8]) -> Result<(), TerminalError> {
        if self.state != TerminalState::Open { return Err(TerminalError::InvalidState); }
        if bytes.len() > MAX_TERMINAL_IO_BYTES { return Err(TerminalError::IoTooLarge); }
        Ok(())
    }

    /// Applies already-validated geometry while open.
    pub fn resize(&mut self, geometry: TerminalGeometry) -> Result<(), TerminalError> {
        if self.state != TerminalState::Open { return Err(TerminalError::InvalidState); }
        self.geometry = geometry;
        Ok(())
    }

    /// Begins terminal close.
    pub fn begin_close(&mut self) -> Result<(), TerminalError> {
        if self.state != TerminalState::Open { return Err(TerminalError::InvalidState); }
        self.state = TerminalState::Closing;
        Ok(())
    }

    /// Marks successful close; closed sessions cannot silently reopen.
    pub fn mark_closed(&mut self) -> Result<(), TerminalError> {
        if self.state != TerminalState::Closing { return Err(TerminalError::InvalidState); }
        self.state = TerminalState::Closed;
        Ok(())
    }

    /// Marks terminal failure from any non-terminal state.
    pub fn mark_failed(&mut self) -> Result<(), TerminalError> {
        if matches!(self.state, TerminalState::Closed | TerminalState::Failed) { return Err(TerminalError::InvalidState); }
        self.state = TerminalState::Failed;
        Ok(())
    }
}

/// Stable terminal failure classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TerminalError { InvalidIdentifier, InvalidGeometry, InvalidState, IoTooLarge, Backend }

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidIdentifier => "terminal session id must be non-zero",
            Self::InvalidGeometry => "terminal geometry is out of bounds",
            Self::InvalidState => "terminal operation is invalid for current state",
            Self::IoTooLarge => "terminal io chunk exceeds bound",
            Self::Backend => "terminal backend operation failed",
        })
    }
}
impl std::error::Error for TerminalError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn principal() -> TerminalPrincipal {
        TerminalPrincipal {
            workspace_id: WorkspaceId::new("workspace-1").expect("workspace"),
            user_id: UserId::new("user-1").expect("user"),
            device_id: DeviceId::new("device-1").expect("device"),
            authenticated_session_id: SessionId::new("session-1").expect("session"),
        }
    }

    #[test]
    fn identifiers_and_geometry_are_bounded() {
        assert_eq!(TerminalSessionId::new(0), Err(TerminalError::InvalidIdentifier));
        assert_eq!(TerminalGeometry::new(0, 24), Err(TerminalError::InvalidGeometry));
        assert!(TerminalGeometry::new(80, 24).is_ok());
    }

    #[test]
    fn open_io_resize_close_is_one_way() {
        let mut session = TerminalSession::new(
            TerminalSessionId::new(1).expect("id"), principal(), TerminalProfile::PosixShell,
            TerminalGeometry::new(80, 24).expect("geometry"),
        );
        session.mark_open().expect("open");
        session.validate_input(b"echo bounded").expect("input");
        session.resize(TerminalGeometry::new(120, 40).expect("geometry")).expect("resize");
        session.begin_close().expect("closing");
        session.mark_closed().expect("closed");
        assert_eq!(session.mark_open(), Err(TerminalError::InvalidState));
    }

    #[test]
    fn io_is_rejected_when_not_open_or_oversized() {
        let mut session = TerminalSession::new(
            TerminalSessionId::new(2).expect("id"), principal(), TerminalProfile::BashShell,
            TerminalGeometry::new(80, 24).expect("geometry"),
        );
        assert_eq!(session.validate_input(b"x"), Err(TerminalError::InvalidState));
        session.mark_open().expect("open");
        assert_eq!(session.validate_input(&vec![0; MAX_TERMINAL_IO_BYTES + 1]), Err(TerminalError::IoTooLarge));
    }
}
