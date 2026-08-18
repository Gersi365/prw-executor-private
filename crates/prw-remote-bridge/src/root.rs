//! Production crate root for the PRW remote bridge.
//!
//! Phase 152 C02e Tranche 4 preserves the existing Phase 143 bridge implementation byte-for-byte
//! as a private submodule and re-exports its public API while adding the reviewed dynamic-
//! reachability modules. This root selection does not activate sockets, tasks, networking or Agent
//! bootstrap behavior.

#[path = "lib.rs"]
mod legacy_bridge;

pub use legacy_bridge::*;

pub mod candidate_publication_freshness;
pub mod candidate_reachability;
pub mod reachability_owner;
