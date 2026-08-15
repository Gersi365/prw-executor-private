//! Authorization capability model for Private Remote Workspace.

/// Explicit remote capabilities.
///
/// This enum is intentionally narrow. It does not imply that the capability
/// is implemented merely because it is represented in the domain model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Open a terminal session.
    TerminalOpen,
    /// Execute a command through an authorized terminal/session mechanism.
    TerminalExec,
    /// Read filesystem content.
    FilesRead,
    /// Create or modify filesystem content.
    FilesWrite,
    /// Delete filesystem content.
    FilesDelete,
    /// Create an authorized port forward.
    ForwardingCreate,
    /// Manage a device.
    DeviceManage,
    /// Manage authorization policy.
    PolicyManage,
}

/// Minimal authorization decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Operation is explicitly allowed.
    Allow,
    /// Operation is not allowed.
    Deny,
}

/// A policy evaluator can decide whether a capability is granted.
///
/// Phase 001 provides only the interface boundary.
pub trait PolicyEvaluator {
    /// Evaluates a requested capability.
    fn evaluate(&self, capability: Capability) -> Decision;
}

#[cfg(test)]
mod tests {
    use super::{Capability, Decision, PolicyEvaluator};

    struct DenyAll;

    impl PolicyEvaluator for DenyAll {
        fn evaluate(&self, _capability: Capability) -> Decision {
            Decision::Deny
        }
    }

    #[test]
    fn evaluator_can_deny_capability() {
        let evaluator = DenyAll;
        assert_eq!(evaluator.evaluate(Capability::FilesDelete), Decision::Deny);
    }
}
