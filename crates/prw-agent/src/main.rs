//! Headless Private Remote Workspace Agent bootstrap.
//!
//! Phase 001 deliberately starts no listeners and performs no system mutation.

use prw_network::PrivateDnsConfig;

fn main() {
    let dns = PrivateDnsConfig::default();

    println!(
        "Private Remote Workspace Agent bootstrap: private_dns_enabled={}",
        dns.enabled
    );
}
