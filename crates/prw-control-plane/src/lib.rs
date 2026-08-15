//! Transport-agnostic control-plane contracts for Private Remote Workspace.
//!
//! Phase 002 defines typed identity, enrollment, and revocation boundaries only.
//! It does not select an HTTP/RPC protocol, persistence layer, authentication
//! mechanism, cryptographic primitive, or network listener.

use std::fmt;

use prw_core::{DeviceId, DeviceLifecycle, EnrollmentId, UserId, WorkspaceId};

/// Opaque public device-identity material.
///
/// The bytes are intentionally algorithm- and serialization-agnostic in Phase 002.
/// Private identity material is never represented by this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicIdentityMaterial(Vec<u8>);

impl PublicIdentityMaterial {
    /// Creates non-empty opaque public identity material.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityMaterialError::Empty`] when no public bytes are supplied.
    pub fn new(value: impl Into<Vec<u8>>) -> Result<Self, IdentityMaterialError> {
        let value = value.into();
        if value.is_empty() {
            return Err(IdentityMaterialError::Empty);
        }
        Ok(Self(value))
    }

    /// Returns the opaque public bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Public-identity material validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityMaterialError {
    /// Public identity material must not be empty.
    Empty,
}

impl fmt::Display for IdentityMaterialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("public identity material must not be empty"),
        }
    }
}

impl std::error::Error for IdentityMaterialError {}

/// Control-plane view of a device identity bound to a workspace and user.
///
/// `user_id` identifies the logical owner/reference in the domain model; it does
/// not imply that Phase 002 has selected or implemented account authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceIdentityBinding {
    /// Workspace containing the device.
    pub workspace_id: WorkspaceId,
    /// Logical user associated with the device.
    pub user_id: UserId,
    /// Stable device identifier.
    pub device_id: DeviceId,
    /// Opaque public identity material only.
    pub public_identity: PublicIdentityMaterial,
    /// Current device lifecycle.
    pub lifecycle: DeviceLifecycle,
}

/// Enrollment request submitted to the future control plane.
///
/// This is a typed domain message, not a wire-format or transport contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrollmentRequest {
    /// Stable enrollment request identifier.
    pub enrollment_id: EnrollmentId,
    /// Target workspace.
    pub workspace_id: WorkspaceId,
    /// Logical user for whom the device is being enrolled.
    pub user_id: UserId,
    /// Device requesting enrollment.
    pub device_id: DeviceId,
    /// Public device-identity material.
    pub public_identity: PublicIdentityMaterial,
}

/// Architecture-neutral enrollment decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnrollmentDecision {
    /// Enrollment is approved.
    Approve,
    /// Enrollment is rejected.
    Reject,
}

/// Lifecycle of an enrollment request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnrollmentState {
    /// No terminal decision has been recorded.
    Pending,
    /// Enrollment was approved.
    Approved,
    /// Enrollment was rejected.
    Rejected,
}

impl EnrollmentState {
    /// Returns whether no further enrollment decision is valid.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Approved | Self::Rejected)
    }

    /// Applies exactly one terminal decision to a pending enrollment.
    ///
    /// # Errors
    ///
    /// Returns [`EnrollmentTransitionError::AlreadyDecided`] if the enrollment
    /// is already approved or rejected.
    pub const fn decide(
        self,
        decision: EnrollmentDecision,
    ) -> Result<Self, EnrollmentTransitionError> {
        match self {
            Self::Pending => match decision {
                EnrollmentDecision::Approve => Ok(Self::Approved),
                EnrollmentDecision::Reject => Ok(Self::Rejected),
            },
            Self::Approved | Self::Rejected => Err(EnrollmentTransitionError::AlreadyDecided),
        }
    }
}

/// Invalid enrollment-state transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnrollmentTransitionError {
    /// A terminal enrollment decision already exists.
    AlreadyDecided,
}

impl fmt::Display for EnrollmentTransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyDecided => formatter.write_str("enrollment already has a terminal decision"),
        }
    }
}

impl std::error::Error for EnrollmentTransitionError {}

/// Request to mark an enrolled device as revoked.
///
/// Phase 002 deliberately leaves revocation propagation, stale-device behavior,
/// and persistence semantics undefined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceRevocation {
    /// Workspace containing the device.
    pub workspace_id: WorkspaceId,
    /// Device to revoke.
    pub device_id: DeviceId,
}

/// Typed action boundary for a future control-plane service.
///
/// Authorization of these actions, wire transport, persistence, retries, and
/// idempotency semantics are deliberately deferred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlPlaneAction {
    /// Submit a new enrollment request.
    SubmitEnrollment(EnrollmentRequest),
    /// Record an enrollment decision.
    DecideEnrollment {
        /// Enrollment request receiving the decision.
        enrollment_id: EnrollmentId,
        /// Decision to record.
        decision: EnrollmentDecision,
    },
    /// Mark a device as revoked.
    RevokeDevice(DeviceRevocation),
}

#[cfg(test)]
mod tests {
    use super::{
        ControlPlaneAction, EnrollmentDecision, EnrollmentRequest, EnrollmentState,
        EnrollmentTransitionError, IdentityMaterialError, PublicIdentityMaterial,
    };
    use prw_core::{DeviceId, EnrollmentId, UserId, WorkspaceId};

    #[test]
    fn public_identity_material_rejects_empty_bytes() {
        assert_eq!(
            PublicIdentityMaterial::new(Vec::<u8>::new()),
            Err(IdentityMaterialError::Empty)
        );
    }

    #[test]
    fn enrollment_decision_is_terminal() {
        let approved = EnrollmentState::Pending
            .decide(EnrollmentDecision::Approve)
            .expect("pending enrollment may be approved");

        assert!(approved.is_terminal());
        assert_eq!(
            approved.decide(EnrollmentDecision::Reject),
            Err(EnrollmentTransitionError::AlreadyDecided)
        );
    }

    #[test]
    fn typed_action_preserves_identity_boundaries() {
        let request = EnrollmentRequest {
            enrollment_id: EnrollmentId::new("enrollment-1").expect("valid enrollment id"),
            workspace_id: WorkspaceId::new("workspace-1").expect("valid workspace id"),
            user_id: UserId::new("user-1").expect("valid user id"),
            device_id: DeviceId::new("device-1").expect("valid device id"),
            public_identity: PublicIdentityMaterial::new(vec![1, 2, 3])
                .expect("non-empty public identity"),
        };

        assert!(matches!(
            ControlPlaneAction::SubmitEnrollment(request),
            ControlPlaneAction::SubmitEnrollment(_)
        ));
    }
}
