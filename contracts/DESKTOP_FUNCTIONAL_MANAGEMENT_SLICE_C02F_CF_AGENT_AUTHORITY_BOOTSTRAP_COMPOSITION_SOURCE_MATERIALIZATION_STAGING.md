# Phase 152 C02f-CF — Agent Authority Bootstrap Composition Source Materialization Staging

Status: `MATERIALIZING / PRW_AGENT_PROCESS_COMPOSITION_ROOT / OPAQUE_CONFIG_TO_PROVIDER_PREPARATION_TO_BRIDGE_AUTHORITY / PROVIDER_BOOTSTRAP_CALL_PATH_MATERIALIZED / NOT_RUNTIME_INVOKED / NO_CUSTODY_CALL / NO_MAIN_WIRING / NO_READINESS_ACTIVATION / NO_AUTHORITY_EXECUTION / NO_RECOVERY / NO_PRWF / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

C02f-CF materializes only the smallest source-level Agent composition operation already selected by C02f-BY and enabled by the later C02f-BZ and C02f-CE boundaries.

C02f-CF does not activate this operation in the running Agent. It adds no `main.rs` call, startup sequencing, readiness transition, runtime task, systemd mutation, credential delivery, recovery path or deployment action.

## Exact prerequisite

C02f-CF derives only from closed C02f-CE:

- branch: `phase-152-c02f-ce-reachability-custody-source-materialization-staging`;
- head: `04f92c92eb528b275b2e2802007f714ee78d7218`;
- tree: `905b429dd559048779924ac6c38954101594a6cd`;
- gate: `C02F_CE_REACHABILITY_CUSTODY_SOURCE_MATERIALIZED`.

The previously selected and materialized boundaries remain authoritative:

- C02f-BY: `prw-agent` is the process-level composition root;
- C02f-BZ: the pure Agent seam converts one already-created `ReachabilityLiveOwnerAcquisitionPreparation` into one bridge-owned `ReachabilityLiveOwnerComposedAsyncAuthority`;
- C02f-BX/CD: `prw-control-plane` owns provider/TLS/two-role authenticated-client bootstrap and exposes the bounded bootstrap operation;
- C02f-CE: the separate systemd custody crate can construct one opaque validated `ReachabilityLiveOwnerEtcdBootstrapConfig` without provider I/O.

CF preserves those ownership boundaries exactly.

## Materialized operation

The selected Agent-side source shape is:

```rust
pub async fn bootstrap_reachability_live_owner_authority(
    config: ReachabilityLiveOwnerEtcdBootstrapConfig,
) -> Result<
    ReachabilityLiveOwnerComposedAsyncAuthority,
    ReachabilityLiveOwnerEtcdBootstrapError,
>
```

The implementation must perform only this bounded sequence:

1. consume exactly one already-validated `ReachabilityLiveOwnerEtcdBootstrapConfig`;
2. call `bootstrap_reachability_live_owner_preparation(config).await` exactly once;
3. fail closed with the existing non-secret provider bootstrap error if that call fails;
4. move the returned `ReachabilityLiveOwnerAcquisitionPreparation` into the existing C02f-BZ `compose_reachability_live_owner_authority(...)` seam;
5. return only `ReachabilityLiveOwnerComposedAsyncAuthority`.

The function body contains the selected provider bootstrap call path. Therefore calling the function performs provider network I/O. C02f-CF itself does not wire or invoke the function from any runtime/startup surface.

## Dependency and capability boundary

No new Cargo dependency is authorized or required. `prw-agent` already has the exact normal dependencies on `prw-control-plane` and `prw-remote-bridge` established by earlier closed tranches.

CF must not expose or retain:

- `etcd_client::Client`;
- `etcd_client::KvClient`;
- TLS configuration objects;
- endpoint vectors or trust bundles outside the opaque input config;
- certificates/private keys;
- secret-store or credential-directory handles;
- raw acquisition/currentness/release provider handles.

The existing control-plane narrowing and bridge semantic ownership remain unchanged.

## Custody boundary

C02f-CF deliberately does not call `prw-reachability-custody`.

The opaque config is supplied by the caller. This preserves C02f-BY's selected ordering in which process composition receives already-validated authority bootstrap configuration from a separately gated custody/configuration boundary.

A later checkpoint must separately decide whether and where the CE systemd-custody loader is invoked relative to Agent startup and readiness. CF must not make that sequencing decision implicitly.

## Runtime boundary

C02f-CF must not modify:

- `crates/prw-agent/src/main.rs`;
- Linux bootstrap/startup ordering;
- runtime-directory or instance-lock sequencing;
- local listener binding/readiness;
- remote transport readiness;
- worker creation or task spawning;
- service readiness publication;
- graceful shutdown behavior.

Successful provider composition is not service readiness.

## Test boundary

CF tests may prove only the type-level async composition signature without invoking it.

They must not:

- start a Tokio runtime;
- connect to etcd;
- load systemd credentials;
- synthesize private keys or provider endpoints;
- execute authority acquisition/currentness/release operations.

Canonical workspace CI remains the source/build validation boundary.

## Explicit exclusions

C02f-CF does not authorize or materialize:

- CE custody invocation;
- concrete endpoint or credential values;
- `LoadCredential=` / `LoadCredentialEncrypted=` edits;
- credential provisioning, generation, rotation or reload;
- runtime invocation of the new bootstrap function;
- `main.rs` integration;
- Agent startup/readiness sequencing;
- authority acquisition/currentness/release execution;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 effect-side stale-fence activation;
- deployment;
- merge.

## Validation requirements

The CF gate may be claimed only after:

1. exact CE ancestry is reverified;
2. CE -> CF compare is limited to this contract, one new Agent module and the minimal Agent module export;
3. Cargo manifests and lockfiles remain byte-stable;
4. BZ source remains byte-stable;
5. exact-head canonical Rust validation reaches terminal success for locked graph, formatting, Clippy, tests and build;
6. Android/AD/AE verdicts are reported exactly as triggered/skipped/not-triggered;
7. Drive audit is written and read back;
8. rolling status is updated append-only with the previous prefix byte-identical;
9. the CF PR remains draft/open/unmerged.

## Gate

On successful exact-head validation and evidence closeout, the gate is:

`C02F_CF_AGENT_AUTHORITY_BOOTSTRAP_COMPOSITION_SOURCE_MATERIALIZED`

This gate means only that the previously selected Agent config-to-provider-to-bridge composition call path exists as validated source. It does not mean custody is wired, provider bootstrap has run in production, the authority is active, or the Agent is ready.