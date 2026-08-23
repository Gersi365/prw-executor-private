# Phase 152 C02f-CG — Agent Reachability Custody Bootstrap Facade Source Materialization Staging

Status: `MATERIALIZING / PRW_AGENT_PROCESS_COMPOSITION_ROOT / SYSTEMD_CUSTODY_TO_PROVIDER_TO_BRIDGE_FACADE / SOURCE_ONLY / NOT_RUNTIME_INVOKED / NO_MAIN_WIRING / NO_READINESS_ACTIVATION / NO_REAL_CREDENTIALS / NO_AUTHORITY_EXECUTION / NO_RECOVERY / NO_PRWF / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

C02f-CG materializes only the remaining source-level facade between the already-closed C02f-CE systemd custody boundary and the already-closed C02f-CF Agent authority bootstrap composition boundary.

This tranche does not wire the facade into `main.rs`, select startup ordering, alter readiness, provide real credentials, or execute provider bootstrap in CI. It creates a callable source boundary only.

## Exact prerequisite

C02f-CG derives only from closed C02f-CF:

- branch: `phase-152-c02f-cf-agent-authority-bootstrap-composition-source-materialization-staging`;
- head: `ea8bf3757cdf273e928631a2599103755bb2e67a`;
- tree: `8d8ed632b38c1fcddb8a929f3b75bc53abf4bc5e`;
- gate: `C02F_CF_AGENT_AUTHORITY_BOOTSTRAP_COMPOSITION_SOURCE_MATERIALIZED`.

Authoritative predecessor boundaries remain unchanged:

- C02f-BY: `prw-agent` is the process-level composition owner;
- C02f-BZ: pure preparation-to-bridge-authority composition;
- C02f-CE: fixed systemd credential custody produces only an opaque validated `ReachabilityLiveOwnerEtcdBootstrapConfig` and performs no provider I/O;
- C02f-CF: Agent consumes that opaque config, invokes control-plane provider bootstrap exactly once when called, then moves the preparation into BZ and returns only the composed bridge authority;
- Phase 125: `prw-agent` already has a production direct-custody precedent and uses a fail-closed custody preflight before entering the runtime facade.

## Materialized facade

The selected source shape is:

```rust
pub async fn bootstrap_reachability_live_owner_authority_from_systemd_credentials(
) -> Result<
    ReachabilityLiveOwnerComposedAsyncAuthority,
    ReachabilityAuthorityCustodyBootstrapError,
>
```

When invoked, the facade must perform only this sequence:

1. call `load_reachability_live_owner_etcd_bootstrap_config_from_systemd_credentials()` exactly once;
2. fail closed as `ReachabilityAuthorityCustodyBootstrapError::Custody` if the CE custody boundary rejects the environment/files/material;
3. pass the opaque config by value to the C02f-CF `bootstrap_reachability_live_owner_authority(...)` facade exactly once;
4. fail closed as `ReachabilityAuthorityCustodyBootstrapError::ProviderBootstrap` if provider bootstrap fails;
5. return only `ReachabilityLiveOwnerComposedAsyncAuthority` on success.

No raw provider client, private key, certificate, endpoint collection, trust bundle, custody file handle or intermediate preparation is returned by the CG public API.

## Failure boundary

CG introduces one bounded Agent-level error enum with exactly two semantic classes:

- `Custody(ReachabilityCustodyError)`;
- `ProviderBootstrap(ReachabilityLiveOwnerEtcdBootstrapError)`.

Both underlying predecessor errors are already bounded/non-secret. CG must not add provider detail, file paths, credential contents, TLS objects, secret bytes or endpoint material to formatting/logging surfaces.

## Dependency boundary

CG adds one direct workspace path dependency:

```toml
prw-reachability-custody = { path = "../prw-reachability-custody" }
```

No crates.io dependency, version, feature, package or transitive graph change is authorized. The only root `Cargo.lock` semantic change allowed is adding `"prw-reachability-custody"` to the existing `prw-agent` package dependency list. The already-existing `prw-reachability-custody` package entry remains byte-stable.

## Runtime boundary

CG must not modify:

- `crates/prw-agent/src/main.rs`;
- device-identity startup preflight;
- Linux runtime-directory or instance-lock sequencing;
- socket/listener creation;
- readiness publication;
- transport startup;
- worker/task creation;
- graceful shutdown;
- systemd units or credential delivery configuration.

Calling the new async facade would read systemd credentials and perform provider network I/O. C02f-CG does not invoke the facade from any runtime/startup surface. A later checkpoint must separately select and authorize runtime ordering.

## Test boundary

CG tests may prove only:

- exact async facade result type;
- exact two-class error wrapping/conversion semantics;
- bounded display/source behavior without secret material.

Tests must not:

- create a Tokio runtime;
- provide credential directories;
- synthesize private keys/certificates/provider endpoints;
- connect to etcd;
- execute acquisition/currentness/release operations;
- launch the Agent binary.

## Explicit exclusions

C02f-CG does not authorize or materialize:

- `main.rs` integration;
- concrete startup/readiness ordering;
- real systemd reachability credential delivery;
- credential provisioning, generation, rotation or reload;
- service-unit/drop-in mutation;
- real provider connectivity;
- live authority execution;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 effect-side stale-fence activation;
- deployment;
- merge.

## Validation requirements

The CG gate may be claimed only after:

1. exact CF ancestry is reverified;
2. CF -> CG compare is limited to this contract, the Agent manifest, root lockfile, one new Agent facade module and the minimal Agent module export;
3. the root lockfile changes only by one `prw-agent` dependency line and the existing `prw-reachability-custody` package entry remains unchanged;
4. `main.rs`, CE custody source, CF facade source and BZ composition source remain byte-stable;
5. any temporary lock materialization helper is removed before final validation;
6. exact-head canonical Rust validation reaches terminal success for locked graph, formatting, Clippy, tests and build;
7. Android/AD/AE verdicts are reported exactly as triggered/skipped/not-triggered;
8. Drive audit is written and raw-read back;
9. rolling status is updated append-only with the previous prefix byte-identical;
10. the CG PR remains draft/open/unmerged.

## Gate

On successful exact-head validation and evidence closeout, the gate is:

`C02F_CG_AGENT_REACHABILITY_CUSTODY_BOOTSTRAP_FACADE_SOURCE_MATERIALIZED`

This gate means only that the source-level systemd-custody-to-provider-to-bridge Agent facade exists and is validated. It does not mean the facade is called by the running Agent, credentials are provisioned, provider bootstrap has occurred in production, the authority is active, or service readiness depends on it.