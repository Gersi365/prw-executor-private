//! Non-activating Linux local peer identity adapter.
//!
//! Phase 057 reads only the Agent effective UID and Linux kernel `SO_PEERCRED`
//! from an already-existing socket file descriptor. It owns no socket lifecycle.

use rustix::fd::AsFd;
use rustix::net::sockopt::socket_peercred;
use rustix::process::geteuid;

/// PRW-local copy of Linux kernel credentials for one connected local peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalLinuxPeerCredentials {
    pid: i32,
    uid: u32,
    gid: u32,
}

impl LocalLinuxPeerCredentials {
    /// Returns the kernel-reported peer process identifier.
    #[must_use]
    pub const fn pid(self) -> i32 {
        self.pid
    }

    /// Returns the kernel-reported peer user identifier.
    #[must_use]
    pub const fn uid(self) -> u32 {
        self.uid
    }

    /// Returns the kernel-reported peer group identifier.
    #[must_use]
    pub const fn gid(self) -> u32 {
        self.gid
    }
}

/// Returns the effective UID under which the Agent is executing.
#[must_use]
pub fn effective_agent_uid() -> u32 {
    geteuid().as_raw()
}

/// Reads Linux kernel `SO_PEERCRED` from an already-existing socket descriptor.
///
/// This function does not read application data from the descriptor and does not
/// make an authorization decision.
///
/// # Errors
///
/// Returns [`LocalLinuxIdentityError::PeerCredentialLookup`] when Linux rejects
/// the `SO_PEERCRED` lookup for the supplied descriptor.
pub fn peer_credentials<Fd: AsFd>(
    fd: Fd,
) -> Result<LocalLinuxPeerCredentials, LocalLinuxIdentityError> {
    let credentials =
        socket_peercred(fd).map_err(|_| LocalLinuxIdentityError::PeerCredentialLookup)?;

    Ok(LocalLinuxPeerCredentials {
        pid: credentials.pid.as_raw_pid(),
        uid: credentials.uid.as_raw(),
        gid: credentials.gid.as_raw(),
    })
}

/// Bounded Linux identity-adapter failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLinuxIdentityError {
    /// Kernel peer-credential lookup failed for the supplied descriptor.
    PeerCredentialLookup,
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixStream;

    use rustix::process::getegid;

    use super::{effective_agent_uid, peer_credentials};

    #[test]
    fn anonymous_unix_pair_reports_current_process_identity() {
        let (left, right) = UnixStream::pair().expect("anonymous Unix pair creates");

        let left_peer = peer_credentials(&left).expect("left peer credentials read");
        let right_peer = peer_credentials(&right).expect("right peer credentials read");
        let process_pid = i32::try_from(std::process::id()).expect("Linux process id fits i32");
        let effective_uid = effective_agent_uid();

        for credentials in [left_peer, right_peer] {
            assert_eq!(credentials.pid(), process_pid);
            assert_eq!(credentials.uid(), effective_uid);
            assert_eq!(credentials.gid(), getegid().as_raw());
        }
    }
}
