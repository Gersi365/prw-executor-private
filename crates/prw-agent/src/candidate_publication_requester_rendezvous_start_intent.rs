//! Agent-internal non-authoritative requester/rendezvous start-intent carrier.
//!
//! C03e-DD materializes only the C03e-DC-selected semantic envelope. The requester session must
//! come from already-authenticated server-held operation state, while `target_device_id` remains
//! requester-nominated intent until separately gated registry/workspace/policy validation succeeds.
//! This module performs no validation, authorization, provider mutation, wire handling, I/O,
//! synchronization, task/listener activation, networking, readiness publication, or deployment.

use prw_core::DeviceId;
use prw_session::AuthenticatedDeviceSession;

/// One unvalidated requester-side intent to begin rendezvous toward one logical target device.
///
/// This value is deliberately neither `Copy` nor `Clone`. Possession is not authorization or a
/// current-registration fact; later composition must separately establish requester currentness,
/// target eligibility, workspace relationship, and the exact policy decision before provider
/// mutation can be considered.
pub(crate) struct RequesterRendezvousStartIntent {
    requester_session: AuthenticatedDeviceSession,
    target_device_id: DeviceId,
}

impl RequesterRendezvousStartIntent {
    /// Packages already-authenticated server-held requester identity with nominated target intent.
    ///
    /// Construction performs ownership composition only. It does not validate registry state,
    /// evaluate policy, mutate requester/rendezvous authority, inspect transport identity, or
    /// perform I/O.
    #[must_use]
    pub(crate) const fn new(
        requester_session: AuthenticatedDeviceSession,
        target_device_id: DeviceId,
    ) -> Self {
        Self {
            requester_session,
            target_device_id,
        }
    }

    /// Returns the already-authenticated requester session carried by this unvalidated intent.
    #[must_use]
    pub(crate) const fn requester_session(&self) -> &AuthenticatedDeviceSession {
        &self.requester_session
    }

    /// Returns the requester-nominated logical target, which is not yet registration authority.
    #[must_use]
    pub(crate) const fn target_device_id(&self) -> &DeviceId {
        &self.target_device_id
    }
}

#[cfg(test)]
mod tests {
    use prw_core::DeviceId;
    use prw_session::AuthenticatedDeviceSession;

    use super::RequesterRendezvousStartIntent;

    type IntentCtor = fn(AuthenticatedDeviceSession, DeviceId) -> RequesterRendezvousStartIntent;
    type SessionAccessor = fn(&RequesterRendezvousStartIntent) -> &AuthenticatedDeviceSession;
    type TargetAccessor = fn(&RequesterRendezvousStartIntent) -> &DeviceId;

    #[test]
    fn carrier_surface_preserves_selected_owned_input_and_read_only_output_shapes() {
        let constructor: IntentCtor = RequesterRendezvousStartIntent::new;
        let requester_session: SessionAccessor = RequesterRendezvousStartIntent::requester_session;
        let target_device_id: TargetAccessor = RequesterRendezvousStartIntent::target_device_id;

        let _ = (constructor, requester_session, target_device_id);
    }
}
