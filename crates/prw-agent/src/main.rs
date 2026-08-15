//! Headless Private Remote Workspace Agent bootstrap.
//!
//! The current build-phase bootstrap starts no listeners and performs no system
//! mutation. Phase 006 defines the future local IPC contract in the library
//! without activating it.

use prw_network::PrivateDnsConfig;

fn main() {
    let dns = PrivateDnsConfig::default();

    println!(
        "Private Remote Workspace Agent bootstrap: private_dns_enabled={}",
        dns.enabled
    );
}
