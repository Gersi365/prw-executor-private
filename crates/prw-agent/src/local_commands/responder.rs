//! Pure in-memory responder for the currently admitted read-only local commands.
//!
//! Phase 036 selects an existing successful response-frame builder from typed
//! request metadata and caller-supplied snapshots. It performs no host reads,
//! authorization, or transport I/O.

use crate::frame_object::LocalIpcFrame;

use super::private_dns_response::{
    LocalPrivateDnsFrameBuildError, build_success_private_dns_frame,
};
use super::private_dns_snapshot::LocalPrivateDnsSnapshot;
use super::status_snapshot::LocalAgentStatusSnapshot;
use super::status_snapshot::response_frame::build_success_status_frame;
use super::terminal_response::builder::LocalTerminalResponseBuildError;
use super::{LocalAgentCommand, LocalAgentRequestEnvelope};

/// Builds one successful terminal response for a validated read-only request.
///
/// The caller supplies both current read-only snapshots. This function only
/// selects the command-specific existing builder and preserves the request ID.
/// It does not perform authorization or read live host state.
///
/// # Errors
///
/// Returns [`LocalReadOnlyResponseBuildError::Status`] if the existing status
/// response builder fails defensively, or
/// [`LocalReadOnlyResponseBuildError::PrivateDns`] if bounded private-DNS
/// encoding/frame construction fails defensively.
pub fn build_read_only_success_response(
    request: LocalAgentRequestEnvelope,
    status_snapshot: LocalAgentStatusSnapshot,
    private_dns_snapshot: &LocalPrivateDnsSnapshot,
) -> Result<LocalIpcFrame, LocalReadOnlyResponseBuildError> {
    match request.command() {
        LocalAgentCommand::GetAgentStatus => {
            build_success_status_frame(request.request_id(), status_snapshot)
                .map_err(LocalReadOnlyResponseBuildError::Status)
        }
        LocalAgentCommand::GetPrivateDnsConfig => {
            build_success_private_dns_frame(request.request_id(), private_dns_snapshot)
                .map_err(LocalReadOnlyResponseBuildError::PrivateDns)
        }
    }
}

/// Defensive Phase 036 read-only response construction failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalReadOnlyResponseBuildError {
    /// Existing `GetAgentStatus` successful-frame construction failed.
    Status(LocalTerminalResponseBuildError),
    /// Existing `GetPrivateDnsConfig` successful-frame construction failed.
    PrivateDns(LocalPrivateDnsFrameBuildError),
}

#[cfg(test)]
mod tests {
    use super::build_read_only_success_response;
    use crate::LocalIpcRequestId;
    use crate::local_commands::private_dns_response::decode_success_private_dns_frame;
    use crate::local_commands::private_dns_snapshot::LocalPrivateDnsSnapshot;
    use crate::local_commands::status_snapshot::response_frame::decode_success_status_frame;
    use crate::local_commands::status_snapshot::{
        LocalAgentRuntimeState, LocalAgentStatusSnapshot,
    };
    use crate::local_commands::{LocalAgentCommand, LocalAgentRequestEnvelope};
    use prw_network::PrivateDnsConfig;

    fn id(value: u64) -> LocalIpcRequestId {
        LocalIpcRequestId::new(value).expect("non-zero request id")
    }

    fn dns_snapshot() -> LocalPrivateDnsSnapshot {
        LocalPrivateDnsSnapshot::try_from_config(&PrivateDnsConfig {
            enabled: true,
            device_naming: true,
            resolvers: vec!["10.0.0.53".into()],
            split_domains: vec!["corp.example".into()],
        })
        .expect("bounded DNS snapshot")
    }

    #[test]
    fn status_request_uses_existing_status_builder_and_preserves_correlation() {
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready);
        let dns = dns_snapshot();
        let request = LocalAgentRequestEnvelope::new(id(180), LocalAgentCommand::GetAgentStatus);

        let frame = build_read_only_success_response(request, status, &dns)
            .expect("status response builds");
        let decoded = decode_success_status_frame(&frame).expect("status response decodes");

        assert_eq!(decoded.request_id(), id(180));
        assert_eq!(decoded.snapshot(), status);
    }

    #[test]
    fn private_dns_request_uses_existing_dns_builder_and_preserves_correlation() {
        let status = LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Degraded);
        let dns = dns_snapshot();
        let request =
            LocalAgentRequestEnvelope::new(id(181), LocalAgentCommand::GetPrivateDnsConfig);

        let frame = build_read_only_success_response(request, status, &dns)
            .expect("private DNS response builds");
        let decoded =
            decode_success_private_dns_frame(&frame).expect("private DNS response decodes");

        assert_eq!(decoded.request_id(), id(181));
        assert_eq!(decoded.snapshot(), &dns);
    }
}
