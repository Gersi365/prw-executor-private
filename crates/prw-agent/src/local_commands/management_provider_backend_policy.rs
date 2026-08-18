//! Pure provider-backend policy seams for Phase 152 C02d.
//!
//! This module deliberately performs no PTY, process, socket, thread, or runtime
//! operation. It separates provider-owned terminal template selection and
//! forwarding egress authorization from request decoding before any concrete
//! Linux adapter is introduced.

use prw_forwarding::TcpForwardSpec;
use prw_terminal::TerminalProfile;

/// Provider-owned terminal launch-template identifier.
///
/// The identifier is derived only from the already-typed terminal profile. It
/// intentionally carries no executable path, argument vector, environment,
/// working directory, or request-controlled string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinuxTerminalLaunchTemplateId {
    /// Provider-owned template corresponding to the POSIX-shell profile.
    PosixInteractiveShell,
    /// Provider-owned template corresponding to the Bash-shell profile.
    BashInteractiveShell,
}

impl LinuxTerminalLaunchTemplateId {
    /// Maps one admitted named terminal profile to its provider-owned template ID.
    #[must_use]
    pub(crate) const fn for_profile(profile: TerminalProfile) -> Self {
        match profile {
            TerminalProfile::PosixShell => Self::PosixInteractiveShell,
            TerminalProfile::BashShell => Self::BashInteractiveShell,
        }
    }
}

/// Agent-owned forwarding-target policy decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ForwardingEgressDecision {
    /// Exact validated forwarding target is permitted by provider policy.
    Allow,
    /// Exact validated forwarding target is denied by provider policy.
    Deny,
}

/// Pure policy boundary evaluated before a concrete forwarding backend connects.
///
/// The input is already a validated [`TcpForwardSpec`]. Implementations cannot
/// widen the loopback bind domain, introduce DNS, or receive raw request bytes.
/// Production assembly must provide an explicitly reviewed implementation before
/// any real socket adapter is wired.
pub(crate) trait ForwardingEgressPolicy {
    /// Evaluates the exact validated forwarding specification.
    fn evaluate(&self, spec: TcpForwardSpec) -> ForwardingEgressDecision;
}

/// Fail-closed forwarding egress policy used before production policy selection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DenyAllForwardingEgressPolicy;

impl ForwardingEgressPolicy for DenyAllForwardingEgressPolicy {
    fn evaluate(&self, _spec: TcpForwardSpec) -> ForwardingEgressDecision {
        ForwardingEgressDecision::Deny
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use prw_forwarding::{ForwardTarget, LoopbackBind, LoopbackFamily, TcpForwardSpec};
    use prw_terminal::TerminalProfile;

    use super::{
        DenyAllForwardingEgressPolicy, ForwardingEgressDecision, ForwardingEgressPolicy,
        LinuxTerminalLaunchTemplateId,
    };

    fn spec(bind_port: u16, target_port: u16) -> TcpForwardSpec {
        TcpForwardSpec::new(
            LoopbackBind::new(LoopbackFamily::Ipv4, bind_port).expect("valid loopback bind"),
            ForwardTarget::new(IpAddr::V4(Ipv4Addr::LOCALHOST), target_port)
                .expect("valid explicit target"),
        )
    }

    #[test]
    fn terminal_profile_maps_only_to_provider_owned_template_id() {
        assert_eq!(
            LinuxTerminalLaunchTemplateId::for_profile(TerminalProfile::PosixShell),
            LinuxTerminalLaunchTemplateId::PosixInteractiveShell
        );
        assert_eq!(
            LinuxTerminalLaunchTemplateId::for_profile(TerminalProfile::BashShell),
            LinuxTerminalLaunchTemplateId::BashInteractiveShell
        );
    }

    #[test]
    fn default_forwarding_egress_policy_is_deny_all() {
        let policy = DenyAllForwardingEgressPolicy;
        assert_eq!(
            policy.evaluate(spec(2200, 22)),
            ForwardingEgressDecision::Deny
        );
        assert_eq!(
            policy.evaluate(spec(8443, 443)),
            ForwardingEgressDecision::Deny
        );
    }

    #[derive(Debug, Clone, Copy)]
    struct ExactSpecPolicy {
        allowed: TcpForwardSpec,
    }

    impl ForwardingEgressPolicy for ExactSpecPolicy {
        fn evaluate(&self, spec: TcpForwardSpec) -> ForwardingEgressDecision {
            if spec == self.allowed {
                ForwardingEgressDecision::Allow
            } else {
                ForwardingEgressDecision::Deny
            }
        }
    }

    #[test]
    fn policy_boundary_can_allow_only_one_exact_validated_spec() {
        let allowed = spec(2200, 22);
        let policy = ExactSpecPolicy { allowed };

        assert_eq!(policy.evaluate(allowed), ForwardingEgressDecision::Allow);
        assert_eq!(
            policy.evaluate(spec(2201, 22)),
            ForwardingEgressDecision::Deny
        );
        assert_eq!(
            policy.evaluate(spec(2200, 23)),
            ForwardingEgressDecision::Deny
        );
    }
}
