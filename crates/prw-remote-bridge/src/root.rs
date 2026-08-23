//! Production crate root for the PRW remote bridge.
//!
//! Phase 152 C02e preserves the existing Phase 143 bridge implementation as a private submodule
//! and re-exports its public API while adding reviewed dynamic-reachability ownership, Tranche 5
//! freshness-token wire semantics, the Tranche 6 provider-neutral live-owner fencing seam, the
//! C02f-Y-selected asynchronous production live-owner authority port, the C02f-AC definitive
//! provider-outcome wrapper, and the C02f-BS common acquisition sub-composition. This root selection
//! does not activate sockets, tasks, networking, a concrete distributed tenancy backend or Agent
//! bootstrap behavior.

#[path = "lib.rs"]
mod legacy_bridge;

pub use legacy_bridge::*;

pub mod candidate_publication_freshness;
pub mod candidate_reachability;
pub mod reachability_freshness_wire;
pub mod reachability_live_owner;
pub mod reachability_live_owner_acquisition_composition;
pub mod reachability_live_owner_async;
pub mod reachability_live_owner_currentness_execution;
pub mod reachability_live_owner_first_owner_acquisition;
// C02f-Y intentionally preserves the explicit `impl Future + Send` async authority contract.
#[allow(clippy::manual_async_fn)]
pub mod reachability_live_owner_provider_bridge;
pub mod reachability_live_owner_reconciled_acquisition;
pub mod reachability_live_owner_reconciled_acquisition_execution;
pub mod reachability_live_owner_reconciled_release;
pub mod reachability_live_owner_reconciled_release_execution;
pub mod reachability_owner;
pub mod session_auth_wire;
