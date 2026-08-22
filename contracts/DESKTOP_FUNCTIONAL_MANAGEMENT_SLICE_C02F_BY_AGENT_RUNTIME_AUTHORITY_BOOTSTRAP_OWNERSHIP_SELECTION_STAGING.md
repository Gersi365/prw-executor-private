# Phase 152 C02f-BY — Agent / Runtime Authority Bootstrap Ownership Selection Staging

Status: `SELECTED / DOCUMENTATION_ONLY / PRW_AGENT_PROCESS_COMPOSITION_ROOT / CONTROL_PLANE_PROVIDER_BOOTSTRAP_PRESERVED / REMOTE_BRIDGE_AUTHORITY_SEMANTICS_PRESERVED / THIN_BINARY_BOUNDARY / NO_AGENT_SOURCE_WIRING / NO_SECRET_CUSTODY_SELECTION / NO_RUNTIME_ACTIVATION / NO_RECOVERY_EXECUTION / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

The user approved the recommended post-C02f-BX direction: select `prw-agent` as the external process-composition owner that will eventually join the already-materialized control-plane provider bootstrap to the already-materialized remote-bridge composed async authority.

This checkpoint is documentation only. It selects ownership and layering. It does not modify Rust source, Cargo manifests, lockfiles, workflows, endpoint values, secret material, provider credentials, runtime startup behavior, R1-R4 effect enforcement, deployment state or pull-request merge state.

## Exact prerequisite

C02f-BY derives only from closed C02f-BX:

- branch: `phase-152-c02f-bx-provider-client-bootstrap-source-materialization-staging`;
- head: `e71b1d8c765d91cfe708523b7edd11d8306a9290`;
- tree: `60e70441d50206c5792fdb6aee20f7e82001cc22`;
- gate: `C02F_BX_PROVIDER_CLIENT_BOOTSTRAP_SOURCE_MATERIALIZED`.

C02f-BX already materialized the control-plane-owned provider bootstrap that creates two role-scoped authenticated etcd client contexts from one immutable logical authority-cluster configuration and returns only `ReachabilityLiveOwnerAcquisitionPreparation`.

C02f-BU already materialized the bridge-owned `ReachabilityLiveOwnerComposedAsyncAuthority` over one already-created preparation facade.

BY composes those ownership decisions; it does not redesign either implementation.

## Selected process-composition owner

C02f-BY selects **`prw-agent` as the process-level composition root** for the future production authority bootstrap handoff.

The ownership model is:

```text
prw-agent process composition
        |
        | validated provider-neutral bootstrap material
        v
prw-control-plane provider bootstrap
        |
        | ReachabilityLiveOwnerAcquisitionPreparation
        v
prw-remote-bridge ReachabilityLiveOwnerComposedAsyncAuthority
        |
        v
prw-agent runtime/lifecycle integration
```

This is a composition-root role only. It does not move provider implementation into `prw-agent` and does not move runtime/process ownership into either lower-level crate.

## Preserved control-plane responsibility

`prw-control-plane` remains the sole selected owner of provider-specific authority construction.

The future Agent composition layer may call the already-materialized control-plane bootstrap, but it must not reimplement or absorb:

- `etcd_client::Client::connect` semantics;
- TLS option construction;
- explicit private trust-bundle application;
- role-scoped mTLS client identity application;
- raw `Client` -> `KvClient` narrowing;
- live-owner/fence-allocator client-role separation;
- creation of `ReachabilityLiveOwnerAcquisitionPreparation` from the two role-scoped clients.

No raw etcd client/store capability may be retained by Agent merely because Agent owns the outer composition.

## Preserved remote-bridge responsibility

`prw-remote-bridge` remains the sole selected owner of the composed live-owner async authority semantics.

The future Agent composition layer may construct the already-materialized bridge authority by supplying exactly one prepared facade by value, conceptually:

```rust
ReachabilityLiveOwnerComposedAsyncAuthority::new(preparation)
```

Agent must not duplicate or bypass the bridge-owned acquisition/currentness/release state machines.

The bridge must continue not to receive:

- endpoint strings;
- TLS configuration;
- certificates/private keys;
- raw `etcd_client::Client` or `KvClient`;
- provider usernames/roles;
- runtime executor handles;
- secret-store handles.

## Why Agent is the selected composition root

The repository layering already supports the required call direction:

```text
prw-agent -> prw-remote-bridge -> prw-control-plane
```

The Agent crate also owns the standalone Linux binary/bootstrap/runtime boundary, including startup sequencing, runtime lifecycle and terminal reporting.

Therefore the process-level decision of when provider bootstrap occurs and when the resulting authority is admitted into the running Agent belongs above both provider construction and bridge semantic composition.

Placing that responsibility in `prw-control-plane` would require lower-layer knowledge of `prw-remote-bridge` and would invert the existing dependency direction or create a dependency cycle.

Placing provider bootstrap inside `prw-remote-bridge` would mix provider/TLS/security construction with authority semantics and would violate BT/BV/BX layering.

## Thin executable boundary

C02f-BY does **not** select direct provider/bootstrap implementation inside `crates/prw-agent/src/main.rs`.

The standalone binary must remain a thin process boundary.

A later source tranche should prefer one narrow Agent-side composition/bootstrap facade, conceptually:

```text
main.rs
  -> prw_agent authority/process bootstrap facade
       -> prw-control-plane provider bootstrap
       -> prw-remote-bridge authority constructor
       -> existing Agent runtime/lifecycle boundary
```

Exact module/function/type names remain a source-materialization detail.

## Dependency selection for later source materialization

C02f-BY permits a future **direct normal dependency from `prw-agent` to `prw-control-plane`** if required to call the control-plane bootstrap explicitly from the composition root.

The permitted dependency DAG is:

```text
prw-agent ----------------------> prw-control-plane
   |
   +----> prw-remote-bridge ----> prw-control-plane
```

This is an acyclic dependency graph and makes the composition-root dependency explicit.

BY does not materialize that dependency yet. A later source tranche must prove the minimal manifest/source scope before changing Cargo metadata or lockfiles.

BY rejects introducing reverse dependencies such as:

- `prw-control-plane -> prw-remote-bridge`;
- `prw-control-plane -> prw-agent`;
- `prw-remote-bridge -> prw-agent`.

## Selected future composition operation

A later separately authorized source tranche may materialize one bounded process-composition operation with only this responsibility:

1. receive already-validated authority bootstrap configuration/material from a separate custody/configuration boundary;
2. call `prw-control-plane` provider bootstrap exactly once for that logical startup attempt;
3. receive one `ReachabilityLiveOwnerAcquisitionPreparation` or fail closed;
4. pass that preparation by value into the existing `ReachabilityLiveOwnerComposedAsyncAuthority` constructor;
5. return/retain only the composed authority at the Agent runtime-composition layer;
6. expose no raw provider client/store or secret material after successful composition.

This future operation must not itself execute live-owner acquisition, currentness, release, recovery or R1-R4 side effects merely as part of construction.

## Fail-closed composition semantics

The Agent process composition must fail closed if provider bootstrap fails or if the preparation cannot be converted into the selected bridge authority shape.

It must not:

- start with a partially constructed authority;
- silently disable live-owner authority and continue as if ready;
- fall back to an in-memory/local authority;
- fall back to plaintext etcd;
- collapse the two role-scoped identities;
- switch to root/admin credentials;
- manufacture readiness from provider-construction failure.

Exact process exit/readiness policy remains a later runtime selection, but failure may never mean authority readiness.

## Secret/configuration custody remains a separate gate

C02f-BY deliberately does not decide where concrete production authority material is stored or loaded.

Still deferred:

- the three concrete etcd member FQDNs/ports;
- private CA/trust-bundle bytes;
- live-owner runtime client certificate/private key;
- fence-allocator runtime client certificate/private key;
- systemd credential names or file descriptors;
- filesystem paths;
- cloud secret manager/KMS/HSM integration;
- certificate issuance, rotation and reload;
- secret zeroization/custody lifecycle beyond existing type boundaries.

No secret value may be committed to Git or Drive as part of BY or its later ordinary audit evidence.

## Runtime/readiness sequencing remains a separate gate

Selecting Agent as composition root does not select when authority bootstrap occurs relative to:

- runtime-directory creation;
- instance-lock acquisition;
- local listener binding;
- listener readiness;
- remote transport readiness;
- worker startup;
- recovery-epoch proof;
- PRWF initialization/currentness proof;
- service readiness publication;
- graceful shutdown.

Those orderings require a later explicit runtime/readiness checkpoint.

In particular, successful etcd client construction alone is not production readiness.

## Recovery boundary

BY does not authorize the Agent composition root to issue a recovery epoch or initialize missing fence-sequence state as a provider-bootstrap side effect.

Normal authority bootstrap remains separate from:

- Spanner recovery authority;
- recovery-epoch issuance;
- PRWF epoch initialization;
- cluster recovery workflows.

A missing/unproven recovery prerequisite must remain fail-closed according to the separately selected recovery lifecycle.

## R1-R4 boundary

BY does not activate stale-fence enforcement at effect sinks.

Constructing `ReachabilityLiveOwnerComposedAsyncAuthority` does not by itself authorize remote terminal, file, forwarding or other externally visible effects.

R1-R4 effect-boundary enforcement remains separately gated and must not be inferred from successful provider/bootstrap composition.

## Minimum scope for the next source tranche

The next separately authorized source tranche should prefer only:

1. one narrow Agent-side authority composition module/facade;
2. the minimum `prw-agent` facade/export update required to expose that composition internally to the binary/runtime layer;
3. a direct `prw-control-plane` dependency in `prw-agent` only if the compiler proves it necessary for the explicit composition call;
4. focused unit/compile tests proving ownership and fail-closed handoff without real endpoints or secret material;
5. `Cargo.lock` only if canonical locked dependency validation proves an actual graph change.

It should not yet include:

- concrete secret loading;
- real endpoint values;
- production invocation from `main.rs`;
- runtime/readiness activation;
- recovery execution;
- R1-R4 effect integration;
- deployment.

## Source-materialization stop conditions

A later source tranche must stop for re-selection rather than widen BY if implementation would require any of the following:

- exposing raw etcd clients/stores through bridge or Agent public APIs;
- making `prw-control-plane` depend on `prw-remote-bridge` or `prw-agent`;
- moving acquisition/currentness/release semantics out of the bridge;
- putting provider TLS/client construction into bridge;
- coupling concrete secret-store technology to the bridge;
- creating detached/background authority-bootstrap tasks;
- activating production authority during ordinary library construction;
- weakening the two-role least-privilege separation selected by AG/AI/BV/BW/BX.

## Explicit exclusions

C02f-BY does not:

- modify Rust source;
- modify Cargo manifests or lockfiles;
- load/create/store secret material;
- select concrete etcd endpoints;
- connect to etcd;
- mutate etcd auth/RBAC or membership;
- execute acquisition/currentness/release;
- issue a recovery epoch;
- initialize PRWF state;
- wire the authority into the running Agent;
- modify `main.rs`;
- create a runtime/task/background worker;
- activate service readiness;
- activate R1-R4;
- deploy;
- merge any pull request.

## Validation gate

The documentation-only gate is:

`C02F_BY_AGENT_RUNTIME_AUTHORITY_BOOTSTRAP_OWNERSHIP_SELECTED`

It may be claimed only after:

1. exact BX ancestry is reverified;
2. BX -> BY compare proves exactly one documentation file addition and no source/manifest/lock/workflow mutation;
3. canonical repository validation on the exact final BY head reaches its actual terminal verdicts;
4. Drive evidence is written/read back and the rolling status is updated append-only;
5. the BY pull request remains draft/open/unmerged.
