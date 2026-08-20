//! Production crate root for the PRW remote bridge.
//!
//! Phase 152 C02e preserves the existing Phase 143 bridge implementation as a private submodule
//! and re-exports its public API while adding reviewed dynamic-reachability ownership, Tranche 5
//! freshness-token wire semantics, the Tranche 6 provider-neutral live-owner fencing seam, and the
//! C02f-X-selected asynchronous production live-owner authority port staging.
//! This root selection does not activate sockets, tasks, networking, a concrete distributed
//! tenancy backend or Agent bootstrap behavior.

#[path = "lib.rs"]
mod legacy_bridge;

pub use legacy_bridge::*;

pub mod candidate_publication_freshness;
pub mod candidate_reachability;
pub mod reachability_freshness_wire;
pub mod reachability_live_owner;
pub mod reachability_live_owner_async;
pub mod reachability_owner;
