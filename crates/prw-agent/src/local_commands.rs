//! Bounded provider-neutral local Agent command contracts.
//!
//! Phase 008 introduces only read-only command identifiers and correlated
//! response metadata. It does not define payload serialization or runtime
//! dispatch.

use crate::LocalIpcRequestId;

/// Read-only command identifiers admitted by the Phase 008 local baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocalAgentCommand {
    /// Read the Agent's local runtime status snapshot.
    GetAgentStatus,
    /// Read the effective private-DNS configuration snapshot.
    GetPrivateDnsConfig,
}

impl LocalAgentCommand {
    /// Returns the stable Phase 008 command identifier.
    #[must_use]
    pub const fn code(self) -> u16 {
        match self {
            Self::GetAgentStatus => 1,
            Self::GetPrivateDnsConfig => 2,
        }
    }
}

/// Terminal status associated with one local Agent response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocalAgentResponseStatus {
    /// Request completed successfully.
    Ok,
    /// Request metadata or future payload is malformed or invalid.
    InvalidRequest,
    /// Authenticated local peer is not authorized for the requested operation.
    Unauthorized,
    /// Command identifier is not supported by the active Agent protocol.
    UnsupportedCommand,
    /// Request conflicts with the Agent's current state.
    Conflict,
    /// Agent failed internally without exposing implementation-sensitive detail.
    InternalError,
}

impl LocalAgentResponseStatus {
    /// Returns the stable Phase 008 response-status identifier.
    #[must_use]
    pub const fn code(self) -> u16 {
        match self {
            Self::Ok => 0,
            Self::InvalidRequest => 1,
            Self::Unauthorized => 2,
            Self::UnsupportedCommand => 3,
            Self::Conflict => 4,
            Self::InternalError => 5,
        }
    }
}

/// Typed metadata for one local Agent request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalAgentRequestEnvelope {
    request_id: LocalIpcRequestId,
    command: LocalAgentCommand,
}

impl LocalAgentRequestEnvelope {
    /// Creates a typed request envelope.
    #[must_use]
    pub const fn new(request_id: LocalIpcRequestId, command: LocalAgentCommand) -> Self {
        Self {
            request_id,
            command,
        }
    }

    /// Returns the frame correlation identifier.
    #[must_use]
    pub const fn request_id(self) -> LocalIpcRequestId {
        self.request_id
    }

    /// Returns the requested local Agent command.
    #[must_use]
    pub const fn command(self) -> LocalAgentCommand {
        self.command
    }
}

/// Typed metadata for one terminal local Agent response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalAgentResponseEnvelope {
    request_id: LocalIpcRequestId,
    status: LocalAgentResponseStatus,
}

impl LocalAgentResponseEnvelope {
    /// Creates a response envelope correlated to a request identifier.
    #[must_use]
    pub const fn new(request_id: LocalIpcRequestId, status: LocalAgentResponseStatus) -> Self {
        Self { request_id, status }
    }

    /// Returns the correlated request identifier.
    #[must_use]
    pub const fn request_id(self) -> LocalIpcRequestId {
        self.request_id
    }

    /// Returns the terminal response status.
    #[must_use]
    pub const fn status(self) -> LocalAgentResponseStatus {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LocalAgentCommand, LocalAgentRequestEnvelope, LocalAgentResponseEnvelope,
        LocalAgentResponseStatus,
    };
    use crate::LocalIpcRequestId;

    #[test]
    fn read_only_command_codes_are_stable() {
        assert_eq!(LocalAgentCommand::GetAgentStatus.code(), 1);
        assert_eq!(LocalAgentCommand::GetPrivateDnsConfig.code(), 2);
    }

    #[test]
    fn response_status_codes_are_stable() {
        assert_eq!(LocalAgentResponseStatus::Ok.code(), 0);
        assert_eq!(LocalAgentResponseStatus::InvalidRequest.code(), 1);
        assert_eq!(LocalAgentResponseStatus::Unauthorized.code(), 2);
        assert_eq!(LocalAgentResponseStatus::UnsupportedCommand.code(), 3);
        assert_eq!(LocalAgentResponseStatus::Conflict.code(), 4);
        assert_eq!(LocalAgentResponseStatus::InternalError.code(), 5);
    }

    #[test]
    fn response_preserves_request_correlation() {
        let request_id = LocalIpcRequestId::new(42).expect("non-zero request id");
        let request = LocalAgentRequestEnvelope::new(request_id, LocalAgentCommand::GetAgentStatus);
        let status = LocalAgentResponseStatus::Ok;
        let response = LocalAgentResponseEnvelope::new(request.request_id(), status);

        assert_eq!(request.command(), LocalAgentCommand::GetAgentStatus);
        assert_eq!(response.request_id(), request.request_id());
        assert_eq!(response.status(), LocalAgentResponseStatus::Ok);
    }
}
