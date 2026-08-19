//! Shared C03 local-management runtime context.
//!
//! The production owner keeps filesystem authority, explicit management policy, and
//! provider lifecycle outside request decoding. This context only borrows those
//! already-assembled objects. A mutex serializes provider lifecycle mutation so terminal,
//! transfer, and forwarding records survive individual local Unix connections while
//! remaining shared by scoped same-UID workers.

#![cfg(target_os = "linux")]

use std::sync::Mutex;

use prw_policy::BoundedLocalManagementPolicy;

use super::LocalAgentResponseStatus;
use super::management_authority::LocalManagementFilesystemAuthority;
use super::management_execution::process_authenticated_linux_management_with_local_authorities;
use super::management_linux_backends::{LinuxLocalForwardingBackend, LinuxLocalTerminalBackend};
use super::management_provider_backend_policy::ExactForwardingEgressPolicy;
use super::management_provider_lifecycle::LocalManagementProviderLifecycle;
use super::terminal_response::builder::{
    LocalTerminalResponseBuildError, build_terminal_response_frame,
};
use crate::frame_object::LocalIpcFrame;
use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
use crate::local_commands::status_snapshot::LocalAgentStatusSnapshot;

/// Concrete C03 Linux provider lifecycle used by the local production runtime.
pub(super) type LocalLinuxManagementProviderLifecycle<'authority> =
    LocalManagementProviderLifecycle<
        'authority,
        LinuxLocalTerminalBackend,
        LinuxLocalForwardingBackend<ExactForwardingEgressPolicy>,
    >;

/// Borrow-only runtime context shared by scoped authenticated local workers.
///
/// The provider lifecycle may retain the longer-lived filesystem authority while each
/// worker only borrows the policy and serialized lifecycle for its own scoped lifetime.
#[derive(Clone, Copy)]
pub(super) struct LocalLinuxManagementRuntimeContext<'context, 'authority> {
    filesystem: &'authority LocalManagementFilesystemAuthority,
    policy: &'context BoundedLocalManagementPolicy,
    lifecycle: &'context Mutex<LocalLinuxManagementProviderLifecycle<'authority>>,
}

impl<'context, 'authority> LocalLinuxManagementRuntimeContext<'context, 'authority> {
    /// Couples already-existing Agent-owned C03 authority and provider state.
    #[must_use]
    pub(super) const fn new(
        filesystem: &'authority LocalManagementFilesystemAuthority,
        policy: &'context BoundedLocalManagementPolicy,
        lifecycle: &'context Mutex<LocalLinuxManagementProviderLifecycle<'authority>>,
    ) -> Self {
        Self {
            filesystem,
            policy,
            lifecycle,
        }
    }

    /// Executes one already-framed command-3 request through serialized shared provider state.
    ///
    /// Mutex poisoning is treated as ambiguous provider state and therefore fails closed with
    /// one correlated `InternalError`. It never retries provider mutation or emits `Ok`.
    ///
    /// # Errors
    ///
    /// Returns only the existing terminal-response frame construction failure.
    pub(super) fn process_management_frame<S>(
        self,
        frame: &LocalIpcFrame,
        connection: &AuthenticatedLocalLinuxConnection<S>,
        status_snapshot: LocalAgentStatusSnapshot,
    ) -> Result<LocalIpcFrame, LocalTerminalResponseBuildError> {
        let request_id = frame.header().request_id();
        let Ok(mut lifecycle) = self.lifecycle.lock() else {
            return build_terminal_response_frame(
                request_id,
                LocalAgentResponseStatus::InternalError,
                &[],
            );
        };

        process_authenticated_linux_management_with_local_authorities(
            frame,
            connection,
            self.policy,
            self.filesystem,
            &mut lifecycle,
            status_snapshot,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    use prw_policy::{
        BoundedLocalManagementDecisions, BoundedLocalManagementPolicy, Decision,
    };
    use prw_remote_bridge::BridgeCommand;
    use prw_terminal::{TerminalGeometry, TerminalProfile, TerminalSessionId};

    use super::{LocalLinuxManagementProviderLifecycle, LocalLinuxManagementRuntimeContext};
    use crate::LocalIpcRequestId;
    use crate::linux_identity::authenticated_connection::AuthenticatedLocalLinuxConnection;
    use crate::local_commands::LocalAgentResponseStatus;
    use crate::local_commands::management_authority::LocalManagementFilesystemAuthority;
    use crate::local_commands::management_linux_backends::{
        LinuxLocalForwardingBackend, LinuxLocalTerminalBackend,
    };
    use crate::local_commands::management_provider_backend_policy::ExactForwardingEgressPolicy;
    use crate::local_commands::management_provider_lifecycle::LocalManagementProviderLifecycle;
    use crate::local_commands::management_request::build_local_management_request_frame;
    use crate::local_commands::status_snapshot::{LocalAgentRuntimeState, LocalAgentStatusSnapshot};
    use crate::local_commands::terminal_response::validate_terminal_response_frame;

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

    struct Harness {
        root_path: PathBuf,
        filesystem: LocalManagementFilesystemAuthority,
    }

    impl Harness {
        fn new(label: &str) -> Self {
            let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let root_path = std::env::temp_dir().join(format!(
                "prw-c03-runtime-context-{}-{sequence}-{label}",
                std::process::id()
            ));
            fs::create_dir(&root_path).expect("runtime-context test root creates");
            let filesystem = LocalManagementFilesystemAuthority::open_trusted_root(&root_path)
                .expect("runtime-context test root anchors");
            Self {
                root_path,
                filesystem,
            }
        }
    }

    impl Drop for Harness {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root_path);
        }
    }

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("request id is non-zero")
    }

    fn terminal_id(value: u64) -> TerminalSessionId {
        TerminalSessionId::new(value).expect("terminal session id is non-zero")
    }

    fn status() -> LocalAgentStatusSnapshot {
        LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)
    }

    fn policy() -> BoundedLocalManagementPolicy {
        BoundedLocalManagementPolicy::new(BoundedLocalManagementDecisions {
            agent_status: Decision::Allow,
            private_dns: Decision::Deny,
            terminal_open: Decision::Allow,
            terminal_exec: Decision::Allow,
            files_read: Decision::Deny,
            files_write: Decision::Deny,
            forwarding_create: Decision::Deny,
        })
    }

    fn lifecycle(
        filesystem: &LocalManagementFilesystemAuthority,
    ) -> LocalLinuxManagementProviderLifecycle<'_> {
        let egress = ExactForwardingEgressPolicy::try_from_targets(&[])
            .expect("empty exact-target allowlist is bounded");
        LocalManagementProviderLifecycle::new(
            filesystem,
            LinuxLocalTerminalBackend,
            LinuxLocalForwardingBackend::new(egress),
        )
    }

    fn connection() -> (AuthenticatedLocalLinuxConnection<UnixStream>, UnixStream) {
        let (server, client) = UnixStream::pair().expect("same-user local pair creates");
        let server = AuthenticatedLocalLinuxConnection::try_new(server)
            .expect("same-UID local pair authenticates");
        (server, client)
    }

    fn management_frame(request_id: u64, command: &BridgeCommand) -> crate::frame_object::LocalIpcFrame {
        let bytes = command.encode().expect("canonical bridge command encodes");
        build_local_management_request_frame(id(request_id), &bytes)
            .expect("command-3 management frame builds")
    }

    fn response_status(frame: &crate::frame_object::LocalIpcFrame) -> LocalAgentResponseStatus {
        validate_terminal_response_frame(frame)
            .expect("terminal response validates")
            .status()
    }

    fn assert_sync<T: Sync>(_: &T) {}

    #[test]
    fn shared_runtime_context_is_sync_for_scoped_workers() {
        let harness = Harness::new("sync");
        let management_policy = policy();
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        let context = LocalLinuxManagementRuntimeContext::new(
            &harness.filesystem,
            &management_policy,
            &lifecycle,
        );
        assert_sync(&context);
    }

    #[test]
    fn terminal_record_survives_same_uid_connection_replacement() {
        let harness = Harness::new("reconnect");
        let management_policy = policy();
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        let session_id = terminal_id(152_031);

        {
            let context = LocalLinuxManagementRuntimeContext::new(
                &harness.filesystem,
                &management_policy,
                &lifecycle,
            );
            let (first_connection, first_client) = connection();
            let open = management_frame(
                1,
                &BridgeCommand::TerminalOpen {
                    session_id,
                    profile: TerminalProfile::PosixShell,
                    geometry: TerminalGeometry::new(80, 24).expect("bounded geometry"),
                },
            );
            let open_response = context
                .process_management_frame(&open, &first_connection, status())
                .expect("terminal-open response builds");
            assert_eq!(response_status(&open_response), LocalAgentResponseStatus::Ok);
            drop(first_connection);
            drop(first_client);

            let (second_connection, second_client) = connection();
            let close = management_frame(2, &BridgeCommand::TerminalClose(session_id));
            let close_response = context
                .process_management_frame(&close, &second_connection, status())
                .expect("terminal-close response builds");
            assert_eq!(response_status(&close_response), LocalAgentResponseStatus::Ok);
            drop(second_connection);
            drop(second_client);
        }

        let lifecycle = lifecycle
            .into_inner()
            .expect("runtime lifecycle lock remains healthy");
        assert!(
            lifecycle.try_finish().is_ok(),
            "terminal close leaves provider lifecycle quiescent"
        );
    }

    #[test]
    fn poisoned_shared_lifecycle_fails_closed_with_correlated_internal_error() {
        let harness = Harness::new("poison");
        let management_policy = policy();
        let lifecycle = Mutex::new(lifecycle(&harness.filesystem));
        let context = LocalLinuxManagementRuntimeContext::new(
            &harness.filesystem,
            &management_policy,
            &lifecycle,
        );

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = lifecycle.lock().expect("lifecycle lock starts healthy");
            panic!("intentional runtime-context poison");
        }));

        let (connection, client) = connection();
        let request = management_frame(77, &BridgeCommand::AgentStatus);
        let response = context
            .process_management_frame(&request, &connection, status())
            .expect("poison failure response builds");
        let terminal = validate_terminal_response_frame(&response)
            .expect("poison response is a valid terminal response");
        assert_eq!(terminal.request_id(), id(77));
        assert_eq!(terminal.status(), LocalAgentResponseStatus::InternalError);
        drop(connection);
        drop(client);
    }
}
