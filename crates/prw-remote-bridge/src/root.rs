//! Production crate root for the PRW remote bridge.
//!
//! Phase 152 C02e preserves the existing Phase 143 bridge implementation as a private submodule
//! and re-exports its public API while adding reviewed dynamic-reachability ownership and the
//! Tranche 5 freshness-token wire contract. This root selection does not activate sockets, tasks,
//! networking, distributed live-owner tenancy or Agent bootstrap behavior.

#[path = "lib.rs"]
mod legacy_bridge;

pub use legacy_bridge::*;

pub mod candidate_publication_freshness;
pub mod candidate_reachability;
pub mod reachability_freshness_wire;
pub mod reachability_owner;
