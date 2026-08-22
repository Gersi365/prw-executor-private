//! Phase 152 C02f-AU public evidence facade for reconciled live-owner acquisition mapping.
//!
//! This module exposes only the retained provider-neutral evidence types needed by a higher-level
//! semantic mapper. It does not expose etcd stores, transaction submission/retry entry points,
//! endpoint/client construction, runtime activation, or production authority execution.

#[allow(dead_code)]
pub(crate) mod attempt_id_generation;
#[allow(dead_code)]
mod first_owner;
mod preparation;

pub use crate::fence_sequence::{
    FenceSequenceAllocationPlan, FenceSequenceHead, FenceSequenceHeadObservation,
    FenceSequenceTxnCompare, FenceSequenceTxnOperation, SequenceAllocationAttemptId,
};
pub use crate::fence_sequence_allocation_orchestrator::{
    FenceSequenceAllocationResolved, FenceSequenceAllocationResolvedOutcome,
};
pub use crate::fence_sequence_live_owner_bridge::FenceSequenceLiveOwnerAcquisitionPlan;
pub use crate::fence_sequence_live_owner_handoff::FenceSequenceLiveOwnerAcquisitionHandoff;
pub use crate::recovery_epoch::RecoveryEpoch;
pub use first_owner::{
    ReachabilityLiveOwnerFirstOwnerHandoff, ReachabilityLiveOwnerFirstOwnerTxnCompare,
    ReachabilityLiveOwnerFirstOwnerTxnOperation, ReachabilityLiveOwnerFirstOwnerTxnPlan,
};
pub use preparation::{
    ReachabilityLiveOwnerAcquisitionPreparation, ReachabilityLiveOwnerPreparationError,
    ReachabilityLiveOwnerPreparedAcquisition,
};
