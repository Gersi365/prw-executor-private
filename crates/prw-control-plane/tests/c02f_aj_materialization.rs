//! Phase 152 C02f-AJ staging compile harness for provider-neutral recovery source.
//!
//! The modules are included directly so the AJ source can be compiled and unit-tested before
//! public library exposure or provider I/O wiring is selected. This test performs no network I/O,
//! credential lookup, runtime activation, recovery execution, epoch issuance or sequence allocation.

#[path = "../src/recovery_epoch.rs"]
mod recovery_epoch;
#[path = "../src/fence_sequence.rs"]
mod fence_sequence;

#[test]
fn c02f_aj_provider_neutral_modules_are_linked_into_validation() {
    assert_eq!(recovery_epoch::RECOVERY_EPOCH_BYTES, 8);
    assert_eq!(fence_sequence::FENCE_SEQUENCE_HEAD_RECORD_BYTES, 22);
    assert_eq!(fence_sequence::FENCE_SEQUENCE_RESERVATION_RECORD_BYTES, 54);
}
