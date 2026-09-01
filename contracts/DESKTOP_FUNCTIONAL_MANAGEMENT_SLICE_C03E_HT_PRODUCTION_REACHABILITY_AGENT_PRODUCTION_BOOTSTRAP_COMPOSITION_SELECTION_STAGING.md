# Private Remote Workspace — C03e-HT Production Reachability Agent Production Bootstrap Composition Selection Staging

Status: `STAGED_SELECTION_ONLY — DOCS_ONLY — NO_SYSTEMD_CALLER — NO_RUNTIME_AUTHORIZATION`

Gate target:

```text
C03E_HT_PRODUCTION_REACHABILITY_AGENT_PRODUCTION_BOOTSTRAP_COMPOSITION_BOUNDARY_SELECTED
```

Canonical closure target:

```text
CLOSED_PRODUCTION_REACHABILITY_AGENT_PRODUCTION_BOOTSTRAP_COMPOSITION_SELECTION
```

## 1. Purpose

C03e-HT selects the exact Agent-owned composition boundary required after closed C03e-HS.

Closed checkpoints already materialize all lower-level pieces independently:

```text
prw-reachability-custody
  -> ReachabilityProductionEtcdBootstrapConfig

prw-control-plane
  -> bootstrap_reachability_production_preparation(config)
  -> ReachabilityProductionEtcdBootstrapPreparation
  -> (
       ReachabilityLiveOwnerAcquisitionPreparation,
       ReachabilityDurableSnapshotEtcdExecutor,
     )

prw-agent existing pure live-authority seam
  -> compose_reachability_live_owner_authority(preparation)

prw-agent existing durable-owner seam
  -> recover_production_reachability_owner_custody(executor, peer)
```

HT selects only how `prw-agent` joins those existing pieces from an already-validated opaque production config and a caller-supplied logical peer identity. It does not call the systemd custody loader, create or provision credentials, alter systemd units, activate Agent startup/readiness, spawn tasks, deploy, restart services, merge, or mutate production state.

## 2. Exact predecessor guard

Canonical predecessor: C03e-HS.

Exact predecessor branch:

```text
phase-152-c03e-hs-production-reachability-durable-snapshot-systemd-credential-custody-source-materialization
```

Exact predecessor head:

```text
2269e43f6f731dfcc83aa43b2d2bda532c962c14
```

Exact predecessor tree:

```text
4bf5ea0ebabbc3829aa1d2bf2b47af33262fac56
```

Exact HS custody source blob:

```text
0f95b16f940dee160482fe8c9f923a8847e8ea58
```

HS Drive audit:

```text
1IcjIkX1a59htu0ggwRrPQ3AkM7OOpcbg
```

HS gate:

```text
C03E_HS_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_SYSTEMD_CREDENTIAL_CUSTODY_SOURCE_MATERIALIZED
```

HS closure:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_SYSTEMD_CREDENTIAL_CUSTODY_SOURCE_MATERIALIZATION
```

HT must remain a docs-only direct successor of that exact closed head.

## 3. Frozen ownership architecture

The production ownership law remains:

```text
prw-reachability-custody
  owns bounded systemd credential acquisition and validation

prw-control-plane
  owns endpoint/TLS/provider Client::connect(...) bootstrap and narrowing

prw-remote-bridge
  owns live-authority and durable-store semantic implementations

prw-agent
  owns cross-crate process composition and production owner custody
```

HT does not move responsibilities between crates.

## 4. Exact current-source evidence — existing two-role Agent provider composition

Exact path at closed HS:

```text
crates/prw-agent/src/reachability_authority_bootstrap.rs
```

Exact blob:

```text
fd93c7180801c36925d48e3b10dcb9eeed9690df
```

Current seam:

```text
bootstrap_reachability_live_owner_authority(
    ReachabilityLiveOwnerEtcdBootstrapConfig,
)
 -> ReachabilityLiveOwnerComposedAsyncAuthority
```

It calls the existing two-role control-plane bootstrap and then the pure live-authority composition seam. It does not read credentials and is not wired into runtime startup/readiness.

HT preserves this existing API unchanged. The future production seam is additive rather than replacing or widening the two-role function.

## 5. Exact current-source evidence — pure live-authority composition already exists

Exact path:

```text
crates/prw-agent/src/reachability_authority_composition.rs
```

Exact blob:

```text
91a639bfcc568a1932064f6745af9b04485f444c
```

Current pure seam:

```text
compose_reachability_live_owner_authority(
    ReachabilityLiveOwnerAcquisitionPreparation,
)
 -> ReachabilityLiveOwnerComposedAsyncAuthority
```

Construction performs no provider I/O and accepts no endpoint or credential material.

HT reuses this seam exactly; it does not create a duplicate live-authority constructor.

## 6. Exact current-source evidence — durable owner composition already exists

Exact path:

```text
crates/prw-agent/src/production_reachability_owner_composition.rs
```

Exact blob:

```text
6a338b43995ecc069383e8aee63d7b53a35bc6ff
```

Current seam:

```text
recover_production_reachability_owner_custody(
    ReachabilityDurableSnapshotEtcdExecutor,
    &PeerConnectivityIdentity,
)
 -> Result<ProductionReachabilityEtcdOwnerCustody, ReachabilityOwnerError>
```

It wraps the executor in the existing bridge durable store, creates the existing production freshness-token source, and recovers exactly one production owner into Agent custody.

It does not connect to etcd, read credentials, spawn a task, or activate runtime behavior.

HT reuses this seam exactly.

## 7. Exact current-source evidence — Agent module and dependency shape

Exact Agent crate root:

```text
crates/prw-agent/src/lib.rs
```

Exact blob:

```text
4a40f84ea72e9c94540561e80a94ab39af7d4dcf
```

It already registers the existing production durable-owner composition module and existing live-authority composition/bootstrap modules.

Exact Agent manifest:

```text
crates/prw-agent/Cargo.toml
```

Exact blob:

```text
4c70d6be9b56f39edc10810eefa3428314ed7559
```

The Agent already depends on:

```text
prw-connectivity
prw-control-plane
prw-reachability-custody
prw-remote-bridge
```

Therefore the first production Agent composition source materialization requires no Cargo or lockfile change.

## 8. Selected new Agent sibling module

HT selects one new crate-internal sibling module:

```text
crates/prw-agent/src/production_reachability_bootstrap.rs
```

The module is registered crate-internally from `crates/prw-agent/src/lib.rs`.

It is not a runtime module, service locator, singleton, task owner, or systemd adapter.

The module accepts no endpoint strings, trust bundle bytes, certificate bytes, private-key bytes, raw etcd `Client`, or raw `KvClient`.

## 9. Selected input boundary

The selected composition function accepts exactly:

```text
ReachabilityProductionEtcdBootstrapConfig
&PeerConnectivityIdentity
```

The production config is already opaque and validated. The peer identity is the existing logical connectivity identity required by durable owner recovery.

Fixed IP is not an identity input. Dynamic IP is not an identity input. Request IDs are not identity inputs.

HT does not select a systemd credential directory, credential filenames, environment variables, or secret bytes as Agent composition inputs.

## 10. Selected provider bootstrap call

The Agent seam calls exactly the existing control-plane function:

```text
bootstrap_reachability_production_preparation(config)
```

and consumes the existing output through:

```text
ReachabilityProductionEtcdBootstrapPreparation::into_parts()
```

which yields exactly:

```text
ReachabilityLiveOwnerAcquisitionPreparation
ReachabilityDurableSnapshotEtcdExecutor
```

No raw provider client crosses into Agent.

## 11. Selected failure-safe ordering

HT freezes the following ordering:

```text
1. bootstrap_reachability_production_preparation(config).await
2. preparation.into_parts()
3. recover_production_reachability_owner_custody(durable_executor, peer).await
4. compose_reachability_live_owner_authority(live_preparation)
5. return one Agent-owned production composition carrier
```

The durable owner is recovered before the live-authority object is constructed.

If provider bootstrap fails, no production preparation is returned.

If durable owner recovery fails after provider bootstrap, the inert live acquisition preparation is dropped and no live-authority object is constructed or returned.

No degraded two-role result, fallback owner, fallback store, in-memory authority, or partial success carrier is returned.

The pure live-authority composition occurs only after authoritative durable recovery succeeds.

## 12. Selected result carrier

HT selects one Agent-owned, crate-internal production composition carrier with exactly two semantic members:

```text
ReachabilityLiveOwnerComposedAsyncAuthority
ProductionReachabilityEtcdOwnerCustody
```

The carrier is an ownership boundary only. It performs no background work.

The carrier exposes one consuming `into_parts(self)` seam for a later separately gated runtime/custody caller. HT does not select public mutable accessors, global storage, singleton registration, or service-locator access.

The future source implementation may choose concise field names, but it must preserve exactly these two owned semantic values and no provider/secret values.

## 13. Selected error boundary

HT selects one bounded Agent-level error enum with exactly two semantic classes:

```text
ProviderBootstrap(ReachabilityProductionEtcdBootstrapError)
OwnerRecovery(ReachabilityOwnerError)
```

The error preserves source chaining for diagnostics while its `Display` text must remain bounded and non-secret.

No endpoint, certificate, private key, raw provider error detail, durable stored bytes, peer key bytes, or authentication material may be formatted into the public error string.

No retry policy is selected.

## 14. Selected composition law

The exact selected law is:

```text
ReachabilityProductionEtcdBootstrapConfig
 + &PeerConnectivityIdentity
        |
        v
bootstrap_reachability_production_preparation(config)
        |
        v
ReachabilityProductionEtcdBootstrapPreparation
        |
        v
into_parts()
        |
        +------------------------------+
        |                              |
        v                              v
ReachabilityLiveOwner          ReachabilityDurableSnapshotEtcdExecutor
AcquisitionPreparation                  |
        |                               v
        |                     recover_production_reachability_owner_custody(
        |                         executor,
        |                         peer,
        |                     ).await
        |                               |
        |                               v
        |                     ProductionReachabilityEtcdOwnerCustody
        |                               |
        v                               |
compose_reachability_live_owner_authority(live_preparation)
        |                               |
        v                               v
ReachabilityLiveOwnerComposedAsyncAuthority
        +-------------------------------+
                        |
                        v
              Agent-owned production
              composition carrier
```

No raw client, secret carrier, fallback, or runtime task is part of this law.

## 15. Future source-materialization ceiling

The first source successor to HT is authorized to touch at most:

```text
crates/prw-agent/src/production_reachability_bootstrap.rs
crates/prw-agent/src/lib.rs
```

No Cargo manifest or lockfile change is authorized.

No change to these existing semantic modules is expected or authorized in that first source checkpoint:

```text
crates/prw-agent/src/reachability_authority_bootstrap.rs
crates/prw-agent/src/reachability_authority_composition.rs
crates/prw-agent/src/production_reachability_owner_composition.rs
crates/prw-agent/src/production_reachability_owner_custody.rs
crates/prw-reachability-custody/src/lib.rs
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

If materialization requires an additional source path, stop and select a separate corrective/extension checkpoint rather than silently widening scope.

## 16. Required future tests

The future source materialization must include bounded compile-time/source-level validation for at least:

- exact opaque production-config plus peer-identity input shape;
- exact result carrier semantic members;
- exact two-class error boundary;
- provider bootstrap error conversion/wrapping without secret display detail;
- owner recovery error conversion/wrapping without secret display detail;
- no systemd custody loader call from the new module;
- no runtime/startup callsite.

Network-provider failure ordering remains validated by the existing lower-level control-plane tests; durable recovery semantics remain validated by the existing owner/store tests. HT does not authorize a new disposable-etcd integration surface in the first source materialization.

## 17. Systemd custody remains separately gated

HS materialized:

```text
load_reachability_production_etcd_bootstrap_config_from_systemd_credentials()
```

HT explicitly does **not** select calling that loader from the new production composition module.

A later Agent custody-join checkpoint may select:

```text
systemd custody loader
 -> opaque production config
 -> production Agent bootstrap composition
```

That later checkpoint must remain separate from the pure config-to-composition seam selected here.

## 18. Runtime activation remains separately gated

HT does not select or authorize:

- `main.rs` integration;
- Agent startup/readiness sequencing;
- service startup/shutdown behavior;
- task spawning or retry loops;
- candidate publication;
- traversal activation;
- listener installation;
- peer dialing;
- background refresh;
- service restart;
- deployment;
- production-state mutation.

The selected function may perform the already-existing provider connection and durable recovery I/O only when a future caller explicitly invokes it. Merely compiling or naming the seam performs no I/O.

## 19. Security non-authorization

HT does not authorize:

- systemd unit/package credential wiring;
- production credential file creation;
- certificate issuance or installation;
- private-key generation or installation;
- etcd auth/RBAC creation or mutation;
- credential rotation;
- trust-bundle mutation;
- endpoint topology mutation;
- credential/client reuse across authority roles.

The dedicated durable principal, role, prefix, certificate/private key, and role-isolated provider connection laws remain unchanged.

## 20. Durable protocol and identity law remain frozen

The authoritative durable protocol remains:

```text
exact key/value
linearizable exact Get
exact dual CAS on mod_revision + observed bytes
exact replacement Put
```

Still forbidden:

```text
create-if-absent recovery
prefix scan
Watch authority
lease/TTL
blind retry
in-memory fallback
```

Logical device identity remains independent of fixed IP. Dynamic IP remains transient reachability only. Request IDs remain correlation only.

## 21. Closure criteria

HT may close only if:

1. the branch is a direct docs-only successor of exact HS head `2269e43f6f731dfcc83aa43b2d2bda532c962c14`;
2. exactly one contract Markdown path is changed;
3. no source, Cargo, workflow, systemd, credential, deployment, runtime, repository-configuration, or visibility path changes;
4. exact-head required CI is successful, with intentionally skipped workflows documented;
5. an immutable closure audit is written to the canonical Drive audit parent and verified by readback/hash/unique title;
6. the PR remains draft/open/unmerged.

## 22. Gate and closure

Gate target:

```text
C03E_HT_PRODUCTION_REACHABILITY_AGENT_PRODUCTION_BOOTSTRAP_COMPOSITION_BOUNDARY_SELECTED
```

Canonical closure target:

```text
CLOSED_PRODUCTION_REACHABILITY_AGENT_PRODUCTION_BOOTSTRAP_COMPOSITION_SELECTION
```

Until exact-head validation and immutable audit completion, HT remains staged selection only.