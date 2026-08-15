//! Provider-neutral local IPC contract for the Ubuntu PRW Agent.
//!
//! Phase 006 records the local endpoint and authorization boundary only. It
//! performs no socket I/O and creates no filesystem objects.

use std::path::{Path, PathBuf};

/// Application directory below `$XDG_RUNTIME_DIR`.
pub const AGENT_RUNTIME_SUBDIRECTORY: &str = "private-remote-workspace";
/// Agent socket filename.
pub const AGENT_SOCKET_FILENAME: &str = "agent.sock";
/// Required Unix mode for the PRW-owned runtime subdirectory.
pub const AGENT_RUNTIME_DIRECTORY_MODE: u32 = 0o700;
/// Required Unix mode for the filesystem-backed Agent socket.
pub const AGENT_SOCKET_MODE: u32 = 0o600;

/// Local transport used between Ubuntu clients and the unprivileged PRW Agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalIpcTransport {
    /// Filesystem-backed Unix-domain `SOCK_STREAM` socket.
    UnixDomainStream,
}

/// Kernel-backed source used to authenticate a connected local peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPeerCredentialSource {
    /// Linux `SO_PEERCRED` on a connected Unix-domain stream socket.
    LinuxSoPeerCred,
}

/// Baseline local authorization rule after peer credentials are obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPeerAuthorization {
    /// Accept only a peer whose kernel-reported UID matches the Agent UID.
    SameUserId,
}

/// Provider-neutral Ubuntu local IPC security contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalIpcContract {
    /// Local transport family and socket semantics.
    pub transport: LocalIpcTransport,
    /// Kernel-backed peer-credential source.
    pub peer_credentials: LocalPeerCredentialSource,
    /// Baseline authorization rule applied to peer credentials.
    pub authorization: LocalPeerAuthorization,
    /// Required Unix mode for the PRW runtime subdirectory.
    pub runtime_directory_mode: u32,
    /// Required Unix mode for the Agent socket filesystem entry.
    pub socket_mode: u32,
}

impl LocalIpcContract {
    /// Returns the locked Phase 006 Ubuntu local IPC baseline.
    #[must_use]
    pub const fn baseline() -> Self {
        Self {
            transport: LocalIpcTransport::UnixDomainStream,
            peer_credentials: LocalPeerCredentialSource::LinuxSoPeerCred,
            authorization: LocalPeerAuthorization::SameUserId,
            runtime_directory_mode: AGENT_RUNTIME_DIRECTORY_MODE,
            socket_mode: AGENT_SOCKET_MODE,
        }
    }

    /// Returns the Agent socket path beneath a supplied XDG runtime directory.
    #[must_use]
    pub fn socket_path(xdg_runtime_dir: &Path) -> PathBuf {
        xdg_runtime_dir
            .join(AGENT_RUNTIME_SUBDIRECTORY)
            .join(AGENT_SOCKET_FILENAME)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{
        AGENT_RUNTIME_DIRECTORY_MODE, AGENT_SOCKET_MODE, LocalIpcContract,
        LocalIpcTransport, LocalPeerAuthorization, LocalPeerCredentialSource,
    };

    #[test]
    fn baseline_contract_is_same_user_unix_socket() {
        let contract = LocalIpcContract::baseline();

        assert_eq!(contract.transport, LocalIpcTransport::UnixDomainStream);
        assert_eq!(
            contract.peer_credentials,
            LocalPeerCredentialSource::LinuxSoPeerCred
        );
        assert_eq!(
            contract.authorization,
            LocalPeerAuthorization::SameUserId
        );
        assert_eq!(contract.runtime_directory_mode, AGENT_RUNTIME_DIRECTORY_MODE);
        assert_eq!(contract.socket_mode, AGENT_SOCKET_MODE);
        assert_eq!(contract.runtime_directory_mode, 0o700);
        assert_eq!(contract.socket_mode, 0o600);
    }

    #[test]
    fn socket_path_is_beneath_xdg_runtime_dir() {
        assert_eq!(
            LocalIpcContract::socket_path(Path::new("/run/user/1000")),
            PathBuf::from("/run/user/1000/private-remote-workspace/agent.sock")
        );
    }
}
