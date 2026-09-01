# Private Remote Workspace — C03e-HP Production Reachability Durable Snapshot Control-Plane Bootstrap Selection Staging

Status: `STAGED_SELECTION_ONLY — DOCS_ONLY — NO_SOURCE_MATERIALIZATION — NO_CREDENTIAL_CUSTODY — NO_RUNTIME_AUTHORIZATION`

Gate target:

```text
C03E_HP_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_CONTROL_PLANE_BOOTSTRAP_BOUNDARY_SELECTED
```

Canonical closure target:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_CONTROL_PLANE_BOOTSTRAP_SELECTION
```

## Purpose

C03e-HP performs the exact-source selection required after closed C03e-HO before the control-plane-owned dedicated durable-snapshot provider bootstrap is materialized.

C03e-HO already materialized the Agent-side consumer seam:

```text
ReachabilityDurableSnapshotEtcdExecutor
 -> ReachabilityDurableSnapshotEtcdStore::new(provider)
 -> ProductionReachabilityFreshnessTokenSource::new()
 -> ProductionReachabilityOwnerCustody::recover(store, token_source, peer)
```

The remaining source question for this boundary is how `prw-control-plane` may create the dedicated durable role connection and hand only the already-existing `ReachabilityDurableSnapshotEtcdExecutor` to the Agent composition root without breaking the current two-role systemd custody API, exposing a raw provider client, reusing credentials, or activating runtime behavior.

This checkpoint selects that source shape only. It does not modify Rust source, connect to etcd, read or add credentials, provision certificates, mutate auth/RBAC, change systemd, activate runtime callsites, deploy, restart services, merge, or mutate production state.

## 1. Exact predecessor guard

Canonical predecessor: C03e-HO.

Exact predecessor head:

```text
49bd041c16ed1fba5ce19704e011c263a9dac753
```

Exact predecessor tree:

```text
866ea2e4302c600dccb6045bf8146c3642ea6ba0
```

C03e-HO canonical closure:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_AGENT_COMPOSITION_SOURCE_MATERIALIZATION
```

C03e-HP must remain a docs-only successor of that exact closed source head. No source materialization belongs in HP.

## 2. Frozen HN/HO composition law

The cross-crate ownership law remains:

```text
prw-control-plane: provider connection/TLS/client bootstrap
prw-remote-bridge: durable semantic store
prw-agent: cross-crate production composition and owner custody
```

The dependency-preserving provider handoff remains exactly:

```text
ReachabilityDurableSnapshotEtcdExecutor
```

Not selected:

- raw `etcd_client::Client`
- raw `etcd_client::KvClient`
- bridge semantic store inside control-plane
- Agent-owned `Client::connect(...)`
- duplicate durable executor/store
- reverse dependency `prw-control-plane -> prw-remote-bridge`

## 3. Exact current-source evidence — control-plane bootstrap is two-role

Exact path at C03e-HO:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

Exact blob:

```text
c454ebe01c9dfdfe6a9117fecffc983f0afe7600
```

The current source owns:

- `ReachabilityEtcdClientIdentityMaterial`;
- one exact three-member HTTPS endpoint vector;
- one explicit trust bundle;
- `ReachabilityLiveOwnerEtcdBootstrapConfig` with exactly two identity fields:
  - live-owner identity;
  - fence-allocator identity;
- structural endpoint/trust validation;
- exact cross-role certificate/private-key reuse rejection;
- `bootstrap_reachability_live_owner_preparation(...)`;
- separate `Client::connect(...)` calls for live-owner and fence-allocator;
- narrowing each broad client to its own `KvClient`;
- dropping each broad `Client` before return;
- return of only `ReachabilityLiveOwnerAcquisitionPreparation`.

The existing function and config are already used by current custody and must remain source/compile compatible.

## 4. Exact current-source evidence — systemd custody is still eight-credential/two-role

Exact path:

```text
crates/prw-reachability-custody/src/lib.rs
```

Exact blob:

```text
cc3dcb80344fc62af31db25a9d93469392d29103
```

The current custody source imports and constructs `ReachabilityLiveOwnerEtcdBootstrapConfig` and reads exactly eight fixed reachability authority credentials:

- three authority endpoints;
- one authority CA bundle;
- live-owner certificate;
- live-owner private key;
- fence-allocator certificate;
- fence-allocator private key.

It does not contain durable-snapshot certificate/private-key credential names or reads.

C03e-HP therefore selects an additive control-plane API. A successor must not change the existing two-role constructor in a way that forces custody changes in the same source checkpoint.

## 5. Exact current-source evidence — durable executor already exists

Exact path:

```text
crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs
```

Exact blob:

```text
77fc9f345c17c5722c5240f3cead7ea68cb55cac
```

The existing provider-specific durable executor constructor is:

```text
ReachabilityDurableSnapshotEtcdExecutor::new(kv: KvClient)
```

The executor already owns exact-key linearizable Get and selected dual-CAS provider execution. It does not own TLS/bootstrap or semantic decoding.

A successor must reuse this type directly. No second durable provider implementation is selected.

## 6. Exact current-source evidence — Agent consumer now exists

Exact path:

```text
crates/prw-agent/src/production_reachability_owner_composition.rs
```

Exact blob:

```text
6a338b43995ecc069383e8aee63d7b53a35bc6ff
```

The closed C03e-HO helper consumes a `ReachabilityDurableSnapshotEtcdExecutor` by value and moves it directly through the existing semantic store and owner custody recovery chain.

This makes the producer/consumer contract concrete: the next control-plane materialization must produce exactly that executor, not a broader provider capability.

## 7. Selected additive three-role config

C03e-HP selects a new additive config in the existing control-plane bootstrap module:

```text
ReachabilityProductionEtcdBootstrapConfig
```

Selected owned fields:

```text
endpoints: Vec<String>
trust_bundle_pem: Vec<u8>
live_owner_identity: ReachabilityEtcdClientIdentityMaterial
fence_allocator_identity: ReachabilityEtcdClientIdentityMaterial
durable_snapshot_identity: ReachabilityEtcdClientIdentityMaterial
```

The existing `ReachabilityLiveOwnerEtcdBootstrapConfig` remains unchanged and continues to represent the current two-role custody path.

The new three-role config constructor must reuse the existing endpoint/trust validation law:

- exactly three endpoints;
- HTTPS only;
- stable FQDN authority members;
- no path/query/fragment/user-info;
- no IP literal/wildcard/localhost;
- unique member hostname;
- non-empty explicit trust bundle.

## 8. Selected three-way identity-separation law

All three role identities must remain distinct.

Exact client certificate bytes must not be reused between any pair:

```text
live-owner <-> fence-allocator
live-owner <-> durable-snapshot
fence-allocator <-> durable-snapshot
```

Exact private-key bytes must not be reused between any pair using the same pairwise law.

C03e-HP selects reuse of the existing bounded configuration error classifications:

```text
ReachabilityLiveOwnerEtcdBootstrapConfigError::ReusedClientCertificate
ReachabilityLiveOwnerEtcdBootstrapConfigError::ReusedPrivateKey
```

for any exact pairwise collision. A new role-specific error variant is not required to preserve the fail-closed law.

The new config must not expose certificate/private-key accessors and must not implement `Clone`/`Debug` merely for convenience.

## 9. Selected additive result carrier

C03e-HP selects one new narrow control-plane result carrier:

```text
ReachabilityProductionEtcdBootstrapPreparation
```

Its semantic ownership shape is exactly:

```text
(
    ReachabilityLiveOwnerAcquisitionPreparation,
    ReachabilityDurableSnapshotEtcdExecutor,
)
```

Selected properties:

- owns exactly one existing live-owner/fence preparation;
- owns exactly one dedicated durable executor;
- contains no broad `Client`;
- exposes no raw `KvClient`;
- contains no bridge semantic type;
- contains no endpoint/trust/certificate/private-key field or accessor;
- contains no arbitrary etcd operation surface;
- may be consumed by value through one narrow `into_parts(self)` seam.

The carrier is provider-bootstrap state, not runtime ownership.

## 10. Selected additive provider bootstrap function

C03e-HP selects a new function in the existing control-plane bootstrap module:

```text
bootstrap_reachability_production_preparation(
    config: ReachabilityProductionEtcdBootstrapConfig,
) -> Result<
    ReachabilityProductionEtcdBootstrapPreparation,
    ReachabilityProductionEtcdBootstrapError,
>
```

Calling this function is provider network I/O, but C03e-HP itself does not call it.

The existing two-role function remains unchanged:

```text
bootstrap_reachability_live_owner_preparation(...)
```

## 11. Selected connection construction order and narrowing

The future three-role function must establish three role-scoped connections using the same validated endpoint vector and trust bundle but three distinct identity values.

Selected narrowing sequence:

```text
live-owner Client::connect(...)
 -> live_owner_kv = client.kv_client()
 -> drop broad live-owner Client

fence-allocator Client::connect(...)
 -> fence_allocator_kv = client.kv_client()
 -> drop broad fence Client

durable-snapshot Client::connect(...)
 -> durable_snapshot_kv = client.kv_client()
 -> drop broad durable Client
 -> ReachabilityDurableSnapshotEtcdExecutor::new(durable_snapshot_kv)
```

Then:

```text
ReachabilityLiveOwnerAcquisitionPreparation::from_role_scoped_clients(
    live_owner_kv,
    fence_allocator_kv,
)
```

is paired by value with the durable executor in the new narrow result carrier.

No broad `Client` is retained or returned.

## 12. Selected three-role bootstrap failure classification

C03e-HP selects a new additive error type:

```text
ReachabilityProductionEtcdBootstrapError
```

with exactly three bounded role-specific connection classifications:

```text
LiveOwnerConnect
FenceAllocatorConnect
DurableSnapshotConnect
```

The underlying provider error must not escape through this public semantic boundary.

If any connection fails, no successful aggregate result may be returned.

If live-owner succeeds and fence-allocator fails, the already-created live-owner narrowed handle is dropped with the failed operation scope.

If live-owner and fence-allocator succeed and durable-snapshot fails, both previously-created narrowed handles are dropped with the failed operation scope.

No partial preparation, raw handle, fallback identity, or degraded two-role success may escape from the three-role function.

## 13. Existing two-role API compatibility law

The next source materialization must leave these existing APIs source-compatible:

```text
ReachabilityLiveOwnerEtcdBootstrapConfig::new(...)
bootstrap_reachability_live_owner_preparation(...)
ReachabilityLiveOwnerEtcdBootstrapError
```

No required new argument may be added to the current two-role config constructor.

No durable identity may default to the live-owner or fence-allocator identity.

No current custody caller may be silently redirected into the three-role bootstrap.

## 14. Selected source-materialization ceiling

The first successor source materialization selected by HP is restricted to:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

No other source path is selected for that first materialization.

Specifically not selected in the same source checkpoint:

- `crates/prw-reachability-custody/src/lib.rs`
- systemd unit/package files
- Agent runtime/bootstrap callsites
- `crates/prw-agent/src/production_reachability_owner_composition.rs`
- durable executor implementation
- remote-bridge durable semantic store
- Cargo manifests or lockfile
- workflows

If source compilation proves another path mechanically required, the successor must stop and use a separate corrective/extension checkpoint rather than broadening scope silently.

## 15. Selected validation obligations for source materialization

The source successor must add bounded structural tests in the same existing bootstrap module sufficient to prove at least:

- a three-role config with distinct identities is accepted;
- durable certificate reuse with live-owner is rejected;
- durable certificate reuse with fence-allocator is rejected;
- durable private-key reuse with live-owner is rejected;
- durable private-key reuse with fence-allocator is rejected;
- the existing two-role config tests remain unchanged/passing.

No test is selected that contacts a production endpoint or requires real credentials.

Full workspace formatting, Clippy, tests and build remain required exact-head evidence. Android validation, if triggered by the source diff, must also complete successfully before closure.

## 16. Credential custody remains separately gated

C03e-HP does not select concrete systemd credential names or read paths for the durable identity.

The already-selected dedicated principal/role boundary remains frozen conceptually, but materializing its certificate/private-key custody is a later explicitly gated security/custody checkpoint.

Until that later checkpoint exists, no production caller can construct the new three-role config from systemd custody.

This is intentional sequencing, not authorization for fallback credentials.

## 17. Runtime activation remains separately gated

Not selected:

- Agent startup callsite
- Linux bootstrap callsite
- readiness publication
- shutdown ownership
- listener installation
- candidate publication execution
- traversal provisioning
- peer dialing
- background task/runtime spawn
- production service restart
- deployment

The new bootstrap types/functions, when materialized later, remain dormant source capability until an explicitly authorized caller exists.

## 18. Durable protocol invariants remain unchanged

The future bootstrap materialization must not alter the existing durable protocol:

- exact durable-snapshot key/value binding;
- default-linearizable exact Get;
- exact dual CAS on `mod_revision` plus observed bytes;
- exact replacement Put;
- no create-if-absent recovery;
- no prefix scan;
- no Watch authority path;
- no lease/TTL authority;
- no blind retry;
- no in-memory fallback.

Logical device identity remains independent of fixed IP. Dynamic IP remains transient reachability only. Request IDs remain correlation only.

## 19. Explicit non-authorization

C03e-HP does not authorize or perform:

- Rust source materialization;
- Cargo/dependency changes;
- provider network connections;
- credential reads/writes/provisioning;
- certificate issuance/installation;
- etcd auth/RBAC mutation;
- systemd changes;
- runtime/startup wiring;
- candidate/traversal activation;
- service restart;
- deployment;
- production-state mutation;
- merge;
- branch deletion;
- repository visibility mutation.

## 20. Closure law

C03e-HP may close only when:

1. it remains a docs-only successor of exact closed HO head `49bd041c16ed1fba5ce19704e011c263a9dac753`;
2. the diff contains only this staging contract;
3. required exact-head CI is complete without failure;
4. an immutable Drive audit is written and verified;
5. PR state remains draft/open/unmerged;
6. no source/runtime/credential/security/deployment action occurred.

On closure, the next admissible checkpoint is a source-materialization successor restricted initially to:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

and implementing only the additive three-role control-plane bootstrap boundary selected above.
