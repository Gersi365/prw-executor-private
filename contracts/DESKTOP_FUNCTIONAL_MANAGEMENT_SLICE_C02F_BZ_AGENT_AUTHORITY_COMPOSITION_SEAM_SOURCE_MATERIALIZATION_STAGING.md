# Phase 152 C02f-BZ — Agent Authority Composition Seam Source Materialization Staging

Status: `MATERIALIZING / PRW_AGENT_COMPOSITION_ROOT / PREPARATION_TO_COMPOSED_ASYNC_AUTHORITY_ONLY / CONTROL_PLANE_PROVIDER_BOOTSTRAP_PRESERVED / REMOTE_BRIDGE_AUTHORITY_SEMANTICS_PRESERVED / NO_SECRET_CUSTODY / NO_PROVIDER_BOOTSTRAP_INVOCATION / NO_MAIN_WIRING / NO_RUNTIME_ACTIVATION / NO_RECOVERY / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

The user approved the post-C02f-BY recommendation to materialize the smallest Agent-owned authority composition seam.

C02f-BZ is source materialization only. It does not activate provider bootstrap or the authority in the running Agent. The source seam accepts one already-created `ReachabilityLiveOwnerAcquisitionPreparation` and returns one bridge-owned `ReachabilityLiveOwnerComposedAsyncAuthority`.

## Exact prerequisite

C02f-BZ derives only from C02f-BY:

- branch: `phase-152-c02f-by-agent-runtime-authority-bootstrap-ownership-selection-staging`;
- head: `8758360c418939c3d8b15f173dd27b8ce0e8b2d7`;
- tree: `8e73f37a8a01310ed24ea3cb8fd22978c9bc90c2`;
- gate: `C02F_BY_AGENT_RUNTIME_AUTHORITY_BOOTSTRAP_OWNERSHIP_SELECTED`.

C02f-BY selected the following stable ownership split:

```text
prw-agent
    = process-level composition owner

prw-control-plane
    = provider/TLS/two-role client bootstrap owner

prw-remote-bridge
    = acquisition/currentness/release authority semantics owner
```

BZ must preserve this split exactly.

## Materialized seam

The selected Agent-owned source shape is:

```rust
pub const fn compose_reachability_live_owner_authority(
    preparation: ReachabilityLiveOwnerAcquisitionPreparation,
) -> ReachabilityLiveOwnerComposedAsyncAuthority
```

The body delegates directly to the already-materialized bridge constructor:

```rust
ReachabilityLiveOwnerComposedAsyncAuthority::new(preparation)
```

No alternative authority implementation, provider wrapper, raw client forwarding, executor or runtime object is introduced.

## Dependency direction

BZ may add a direct `prw-agent -> prw-control-plane` dependency because the Agent composition root must name the preparation type it receives from the control-plane provider bootstrap.

The intended DAG remains acyclic:

```text
prw-agent --------------------> prw-control-plane
    |
    +--------------------------> prw-remote-bridge
                                     |
                                     +------------> prw-control-plane
```

Forbidden dependency inversions remain:

- `prw-control-plane -> prw-agent`;
- `prw-control-plane -> prw-remote-bridge`;
- provider/TLS/etc. construction inside `prw-remote-bridge`;
- authority lifecycle semantics inside `prw-agent`.

## Source boundary

The BZ module belongs to `prw-agent` and must be callable by a later process-bootstrap tranche without modifying `main.rs` now.

The seam owns only type-level composition. It does not:

- create or validate endpoint strings;
- load certificates/private keys;
- construct `ReachabilityEtcdClientIdentityMaterial`;
- construct `ReachabilityLiveOwnerEtcdBootstrapConfig`;
- call `bootstrap_reachability_live_owner_preparation(...)`;
- connect to etcd;
- expose `etcd_client::Client` or `KvClient`;
- mutate auth/RBAC or etcd membership;
- create tasks/executors/background workers;
- execute acquisition/currentness/release;
- alter Agent readiness;
- enter the production runtime loop.

## Test boundary

BZ tests must remain runtime/provider independent.

They may prove:

1. the exact function signature consumes `ReachabilityLiveOwnerAcquisitionPreparation` and returns `ReachabilityLiveOwnerComposedAsyncAuthority`;
2. the returned concrete type implements the existing `ReachabilityLiveOwnerAsyncAuthority` port.

They must not need a live etcd cluster, synthetic credentials, fake endpoints, a Tokio runtime or provider calls.

## Explicit exclusions

C02f-BZ does not authorize or materialize:

- secret custody selection;
- production endpoint selection;
- systemd credential naming/loading for etcd material;
- provider bootstrap invocation;
- `main.rs` changes;
- Agent startup sequencing changes;
- service readiness activation;
- authority acquisition/currentness/release execution;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 effect-side stale-fence enforcement activation;
- deployment;
- merge of any pull request.

## Validation requirements

The BZ gate may be claimed only after:

1. exact BY ancestry is reverified;
2. BY -> BZ compare is audited for the intended narrow source/manifest/contract delta;
3. Cargo.lock remains unchanged unless Cargo itself proves a required deterministic graph correction;
4. exact-head canonical Rust validation reaches terminal success for locked graph, formatting, Clippy, tests and build;
5. Android workflow verdict is reported exactly as triggered/skipped/not-triggered and is never inferred;
6. AD/AE disposable workflow verdicts are reported accurately;
7. Drive audit is written and read back;
8. rolling status is updated append-only with the previous prefix byte-identical;
9. the BZ PR remains draft/open/unmerged.

## Gate

On successful exact-head validation and evidence closeout, the gate is:

`C02F_BZ_AGENT_AUTHORITY_COMPOSITION_SEAM_SOURCE_MATERIALIZED`

This gate means only that the pure Agent-owned preparation-to-authority composition seam exists and validates. It does not mean the provider bootstrap or production authority is active.
