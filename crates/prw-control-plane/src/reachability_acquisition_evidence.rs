//! Phase 152 C02f-AU public evidence facade for reconciled live-owner acquisition mapping.
//!
//! This module exposes the retained provider-neutral evidence types needed by higher-level semantic
//! mapping. C02f-BS additionally exposes one lifetime-bounded acquisition-execution capability over
//! the exact live-owner store already owned by the C02f-BM preparation facade. The capability does
//! not expose the store, `KvClient`, endpoint/configuration state, generic transaction execution,
//! currentness, release, runtime activation, or provider construction.

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
    ReachabilityLiveOwnerAcquisitionExecution, ReachabilityLiveOwnerAcquisitionPreparation,
    ReachabilityLiveOwnerPreparationError, ReachabilityLiveOwnerPreparedAcquisition,
};
