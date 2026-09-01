# Private Remote Workspace — C03e-HV Production Reachability Agent systemd Custody Join Selection Staging

Status: `STAGED_SELECTION_ONLY — DOCS_ONLY — NO_RUNTIME_AUTHORIZATION`

Gate target:

```text
C03E_HV_PRODUCTION_REACHABILITY_AGENT_SYSTEMD_CUSTODY_JOIN_BOUNDARY_SELECTED
```

Canonical closure target:

```text
CLOSED_PRODUCTION_REACHABILITY_AGENT_SYSTEMD_CUSTODY_JOIN_SELECTION
```

## 1. Purpose

C03e-HV selects the exact Agent-owned join between two already-closed production reachability boundaries:

```text
prw-reachability-custody
  -> load_reachability_production_etcd_bootstrap_config_from_systemd_credentials()
  -> ReachabilityProductionEtcdBootstrapConfig

prw-agent C03e-HU
  -> bootstrap_production_reachability(config, peer).await
  -> ProductionReachabilityBootstrapComposition
```

HV selects only how one future crate-internal Agent facade joins those boundaries. It does not invoke that facade from Agent startup, readiness, main, a background task, candidate publication, traversal, listener, dialing, deployment, or production operations.

## 2. Exact predecessor guard

Canonical predecessor: C03e-HU.

Exact predecessor branch:

```text
phase-152-c03e-hu-production-reachability-agent-production-bootstrap-composition-source-materialization
```

Exact predecessor head:

```text
bc60cc30ef5f3a872f7cc814df61e2330b4513ac
```

Exact predecessor tree:

```text
f76cd4fb5eb629d3b695f0f9a43e1d40670f9945
```

Exact HU production bootstrap source blob:

```text
8de308229d072272b96e4217f8ebf6484e666f23
```

Exact Agent crate-root blob:

```text
fc61d077ccd8c12ad9353c61aee8ed4017ec7d43
```

HU Drive audit:

```text
1wl0a90779Nz0FYqrQpMr5vmovuw5Envc
```

HU gate:

```text
C03E_HU_PRODUCTION_REACHABILITY_AGENT_PRODUCTION_BOOTSTRAP_COMPOSITION_SOURCE_MATERIALIZED
```

HU closure:

```text
CLOSED_PRODUCTION_REACHABILITY_AGENT_PRODUCTION_BOOTSTRAP_COMPOSITION_SOURCE_MATERIALIZATION
```

HV must remain a docs-only direct successor of that exact closed head.

## 3. Frozen ownership architecture

The existing ownership law remains unchanged:

```text
prw-reachability-custody
  owns bounded systemd service-credential acquisition and validation

prw-control-plane
  owns endpoint/TLS/provider Client::connect(...) bootstrap and narrowing

prw-remote-bridge
  owns live-authority and durable-store semantics

prw-agent
  owns cross-crate process composition and custody joins
```

HV does not move credential reading into control-plane or HU composition, and does not move provider connection into custody.

## 4. Exact current-source evidence — production custody loader exists

Exact path at closed HU:

```text
crates/prw-reachability-custody/src/lib.rs
```

Exact blob:

```text
0f95b16f940dee160482fe8c9f923a8847e8ea58
```

The crate already exposes:

```text
load_reachability_production_etcd_bootstrap_config_from_systemd_credentials()
 -> Result<ReachabilityProductionEtcdBootstrapConfig, ReachabilityCustodyError>
```

The loader reads the existing eight reachability credentials plus the two dedicated durable-snapshot identity credentials, retains private-key plaintext only in zeroizing buffers, returns one validated opaque production config, and performs no provider network I/O.

HV reuses this exact loader. No credential filename, file-reading rule, permission rule, size bound, endpoint parser, identity parser, or custody error is changed.

## 5. Exact current-source evidence — HU composition exists

Exact path:

```text
crates/prw-agent/src/production_reachability_bootstrap.rs
```

Exact blob:

```text
8de308229d072272b96e4217f8ebf6484e666f23
```

HU already exposes inside a crate-private module:

```text
bootstrap_production_reachability(
    ReachabilityProductionEtcdBootstrapConfig,
    &PeerConnectivityIdentity,
)
 -> Result<ProductionReachabilityBootstrapComposition,
           ProductionReachabilityBootstrapError>
```

HU already freezes the failure-safe ordering:

```text
provider production bootstrap
 -> split live + durable executor
 -> durable owner recovery
 -> live-authority composition
 -> one Agent-owned production composition carrier
```

HV reuses HU exactly and does not duplicate provider/bootstrap/recovery/composition logic.

## 6. Existing public two-role custody facade remains unchanged

Exact path:

```text
crates/prw-agent/src/reachability_authority_custody_bootstrap.rs
```

Exact blob:

```text
2843cbf9cfed7ae26e336ec4a2ead6a97855b2c0
```

That module is already public from the Agent crate root and owns the older two-role live-owner custody join:

```text
bootstrap_reachability_live_owner_authority_from_systemd_credentials()
```

HV explicitly does not widen, replace, rename, or modify that public module or API. Production custody join is selected as a separate crate-internal sibling to avoid exposing a new public production API accidentally.

## 7. Selected new Agent sibling module

HV selects exactly:

```text
crates/prw-agent/src/production_reachability_custody_bootstrap.rs
```

The module is registered crate-internally from:

```text
crates/prw-agent/src/lib.rs
```

It is not a runtime module, global singleton, service locator, task owner, startup hook, readiness hook, or systemd unit adapter.

## 8. Selected input boundary

The selected facade accepts exactly:

```text
&PeerConnectivityIdentity
```

It accepts no endpoint strings, certificate bytes, private-key bytes, trust bundle, credential-directory path, raw etcd Client, raw KvClient, request ID, IP address, or dynamic reachability observation.

Logical peer identity remains typed and independent of IP reachability. Request IDs remain correlation only.

## 9. Selected function

HV selects the future function:

```text
bootstrap_production_reachability_from_systemd_credentials(
    peer: &PeerConnectivityIdentity,
) -> Result<
    ProductionReachabilityBootstrapComposition,
    ProductionReachabilityCustodyBootstrapError,
>
```

The function is reachable only through the crate-internal sibling module selected above.

## 10. Selected ordering law

The exact selected law is:

```text
1. load_reachability_production_etcd_bootstrap_config_from_systemd_credentials()
2. bootstrap_production_reachability(config, peer).await
3. return ProductionReachabilityBootstrapComposition
```

Consequences:

- custody failure occurs before provider network I/O;
- the validated config moves directly into HU;
- HU retains sole Agent ownership of provider-preparation split, durable recovery, and live-authority composition;
- no secret bytes or raw provider clients are returned by the new facade;
- no partial/degraded production composition is returned.

No retry policy is selected.

## 11. Selected error boundary

HV selects one bounded Agent-level error enum:

```text
ProductionReachabilityCustodyBootstrapError {
    Custody(ReachabilityCustodyError),
    Composition(ProductionReachabilityBootstrapError),
}
```

`Display` must remain bounded and non-secret, with semantic messages equivalent to:

```text
production reachability custody bootstrap failed
production reachability composition failed
```

`std::error::Error::source()` may preserve the typed underlying error for diagnostics.

No endpoint, credential path, certificate, private key, trust bytes, provider error details, durable snapshot bytes, or peer key bytes may be formatted into the facade's public display string.

## 12. Failure-safety law

HV freezes:

```text
custody failure
 -> no HU call
 -> no provider connection
 -> no composition result

HU composition failure
 -> no ProductionReachabilityBootstrapComposition returned
 -> no fallback/two-role result
 -> no retry
```

The facade never creates an in-memory fallback authority or fallback durable owner.

## 13. Source-materialization ceiling

The first source successor to HV may touch at most:

```text
crates/prw-agent/src/production_reachability_custody_bootstrap.rs
crates/prw-agent/src/lib.rs
```

No other path is selected.

Specifically not selected:

```text
crates/prw-agent/src/reachability_authority_custody_bootstrap.rs
crates/prw-agent/src/production_reachability_bootstrap.rs
crates/prw-reachability-custody/src/lib.rs
crates/prw-control-plane/**
crates/prw-remote-bridge/**
Cargo.toml
Cargo.lock
systemd unit/package files
workflows
main/startup/runtime files
```

If source materialization requires another path, stop and use a separate corrective/extension checkpoint.

## 14. Required future tests

The future source checkpoint must include bounded source/compile validation for at least:

- exact `&PeerConnectivityIdentity` input shape;
- exact result type `ProductionReachabilityBootstrapComposition`;
- exact two-class error boundary;
- `ReachabilityCustodyError` conversion/wrapping with bounded display and source chaining;
- `ProductionReachabilityBootstrapError` conversion/wrapping with bounded display and source chaining;
- no direct raw credential or provider-client input surface;
- no runtime/startup callsite in the new module.

The test suite must not require real systemd credentials, production credentials, real etcd, or network access merely to test the facade's type/error surface.

Exact-head Rust formatting, Clippy, tests, and workspace build are required. Android validation counts if triggered.

## 15. Runtime activation remains separately gated

HV does not select or authorize:

- `main.rs` integration;
- Agent startup or readiness sequencing;
- calling the new facade from any running service path;
- task spawning or retry loops;
- candidate publication;
- traversal activation;
- listener installation;
- peer dialing;
- background refresh;
- service restart;
- deployment;
- production-state mutation.

Calling the future facade itself will perform the already-existing systemd credential reads followed by HU's already-existing provider and durable-recovery I/O. Merely compiling or naming the facade performs no I/O.

## 16. Security non-authorization

HV does not authorize:

- production credential creation or installation;
- systemd unit/package credential wiring;
- certificate issuance or installation;
- private-key generation or installation;
- etcd auth/RBAC creation or mutation;
- credential rotation;
- trust-bundle mutation;
- endpoint topology mutation;
- credential/client reuse across authority roles.

The dedicated durable principal/role/prefix and three-way identity-separation laws remain unchanged.

## 17. Identity and durable protocol invariants remain frozen

Logical device/peer identity is not fixed-IP based. Dynamic IP is transient reachability only. Request IDs are correlation only.

The durable snapshot protocol remains exact-key, linearizable exact Get, exact dual CAS on mod_revision plus observed bytes, exact Put, with no create-if-absent recovery, scan, Watch, lease/TTL, blind retry, or in-memory fallback.

## 18. Closure requirements

C03e-HV may close only after all of the following hold on one exact head:

1. direct successor of exact closed HU head;
2. exactly one added Markdown contract path and no source/Cargo/workflow/runtime/security change;
3. exact-head required CI completed with no failure;
4. immutable Drive audit uploaded under the canonical audit parent;
5. raw audit readback matches exact local size/hash;
6. exact-title search returns one unique canonical artifact;
7. PR remains draft/open/unmerged;
8. no runtime/security/deployment authorization is implied.

Only then may the gate be recorded as:

```text
C03E_HV_PRODUCTION_REACHABILITY_AGENT_SYSTEMD_CUSTODY_JOIN_BOUNDARY_SELECTED
```

and the checkpoint as:

```text
CLOSED_PRODUCTION_REACHABILITY_AGENT_SYSTEMD_CUSTODY_JOIN_SELECTION
```
