# Private Remote Workspace — Phase 152 C03e-HH Production Reachability Durable Snapshot etcd Provider Client Composition Ownership Selection

Status: `STAGING / OWNERSHIP_SELECTION_ONLY / NO_SOURCE_MATERIALIZATION / NO_RUNTIME_ACTIVATION`

Gate target: `C03E_HH_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_CLIENT_COMPOSITION_OWNERSHIP_SELECTED`

Canonical predecessor: C03e-HG.

Exact predecessor head:

```text
5657ac7858dac775be2817f98665e40b489c1e4a
```

Exact predecessor tree:

```text
21b307e9b8655f10fd836c710f342e5ea79b0d81
```

This checkpoint selects only the ownership boundary for supplying an already-created, role-scoped `etcd_client::KvClient` to the C03e-HG durable-snapshot provider executor. It deliberately does not select or materialize a connection/bootstrap owner. It does not add Rust source, alter manifests or lockfiles, connect to etcd, select endpoints, configure TLS/auth/RBAC/credentials, change Agent or runtime composition, activate persistence/recovery, start tasks, deploy, restart, merge, or modify production service state.

## 1. Exact C03e-HG source basis

C03e-HG leaves the following exact source topology authoritative:

- control-plane registration: `crates/prw-control-plane/src/lib.rs`, exact blob `c0e84ab71afa12fedd8c402eff0d8bcc247c1b3f`;
- control-plane durable-snapshot raw etcd executor: `crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs`, exact blob `77fc9f345c17c5722c5240f3cead7ea68cb55cac`;
- bridge semantic durable-snapshot etcd store: `crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs`, exact blob `a381963986c79f8a314088839316d47595ba8686`;
- bridge registration: `crates/prw-remote-bridge/src/root.rs`, exact blob `8b829f503380b3d02e8a91a9743017046d8c0b92`;
- C03e-HG materialization contract: `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HG_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_ADAPTER_SOURCE_MATERIALIZATION_STAGING.md`, exact blob `16b81a519ccc82301e30b5374e09505ea672c8b8`.

The exact C03e-HG implementation establishes:

```text
ReachabilityDurableSnapshotEtcdStore
    -> ReachabilityDurableSnapshotEtcdExecutor
        -> already-created etcd_client::KvClient
```

The bridge store constructs around an already-created raw executor. The control-plane executor constructs around an already-created `KvClient`. Neither layer owns endpoint selection or `Client::connect`.

The exact C03e-HG `prw-control-plane/src` directory contains the current provider executors, including:

- `fence_sequence_allocation_etcd.rs`;
- `fence_sequence_initialization_etcd.rs`;
- `reachability_live_owner_etcd.rs`;
- `reachability_durable_snapshot_etcd.rs`.

Those provider executors use the same current boundary: they accept an already-created `KvClient` and keep endpoint/TLS/client bootstrap outside their source checkpoint. The exact C03e-HG control-plane module registry contains no `bootstrap` module, and the historical `crates/prw-control-plane/src/bootstrap.rs` path is absent at the exact C03e-HG head.

C03e-HH therefore must not restore, infer, or silently recreate the historical bootstrap architecture.

## 2. Selected negative ownership law

C03e-HH selects the following negative ownership law:

```text
C03e-HH does not assign ownership of etcd connection creation.
```

At this checkpoint, an already-created, appropriately scoped `KvClient` is an explicit external composition prerequisite for constructing `ReachabilityDurableSnapshotEtcdExecutor`.

No current C03e-HG source path is promoted by this checkpoint into the owner of:

- endpoint discovery or endpoint selection;
- `etcd_client::Client::connect`;
- generic `Client` lifecycle;
- `KvClient` derivation/narrowing from a generic client;
- credentials or authentication material;
- TLS identity/trust configuration;
- RBAC policy or authorization provisioning;
- reconnect/session lifecycle;
- connection health supervision;
- client pooling or sharing policy;
- runtime task ownership;
- shutdown/drain behavior.

Absence of a selected owner is intentional. It is not permission for any provider executor, bridge semantic adapter, Agent constructor, or ad hoc call site to assume that ownership implicitly.

## 3. Existing provider/semantic ownership remains unchanged

C03e-HH does not reopen the two-layer C03e-HF/HG ownership model.

### 3.1 Control-plane remains provider-specific

`prw-control-plane` continues to own only the raw provider operation surface already materialized in C03e-HG:

- default-linearizable exact-key Get;
- exact-key response validation;
- provider revision carriage;
- the selected exact-key dual-CAS Txn;
- exact replacement Put on Txn success;
- one authoritative exact-key Get on compare failure;
- provider response-shape validation;
- fail-closed classification of indeterminate provider RPC outcomes.

`ReachabilityDurableSnapshotEtcdExecutor::new(kv)` continues to consume an already-created `KvClient`.

C03e-HH does not widen the executor so that it accepts endpoints, credentials, TLS material, a generic `Client`, a configuration object, or a connection factory.

### 3.2 Bridge remains semantic

`prw-remote-bridge` continues to own PRW reachability durable semantics through `ReachabilityDurableSnapshotEtcdStore` and the existing codecs/owner boundary.

The bridge store continues to construct around an already-created control-plane executor. It does not acquire an `etcd-client` dependency and does not receive endpoints, credentials, TLS/RBAC configuration, generic provider clients, or connection factories.

Dependency direction remains:

```text
prw-remote-bridge -> prw-control-plane -> etcd-client
```

No reverse `prw-control-plane -> prw-remote-bridge` dependency is selected.

## 4. Role-scoped client requirement

Any later source checkpoint that actually composes a provider client must preserve role scoping.

The durable-snapshot executor must receive only the provider capability it requires:

```text
etcd_client::KvClient
```

A later composition layer must not expose a generic `etcd_client::Client` through bridge semantic APIs merely to satisfy this executor.

This checkpoint does not decide whether a future composition owner obtains the `KvClient` by:

- narrowing a dedicated generic client;
- narrowing a shared generic client;
- receiving a `KvClient` from a higher-level composition boundary;
- another explicitly selected lifecycle mechanism.

That decision requires a separate ownership/materialization checkpoint with exact current-source evidence.

## 5. Historical bootstrap is evidence, not current architecture

An earlier C02f-BX checkpoint historically materialized `crates/prw-control-plane/src/bootstrap.rs` and created/narrowed role-scoped etcd clients for other provider roles.

That historical implementation is not present at the exact C03e-HG head. C03e-HH treats it only as provenance showing that role-scoped narrowing has existed before; it is not a current seam and is not authorization to restore the file, its constructors, its client multiplicity, its endpoint handling, or its lifecycle decisions.

A future checkpoint may reuse an older idea only after proving it against current topology and explicitly selecting the new owner and lifecycle law.

## 6. No implicit composition through Agent/runtime

C03e-HH does not change Agent/bootstrap/runtime behavior.

No existing Agent constructor, owner constructor, bridge root registration, desktop/native entrypoint, background task, or service initialization path is selected as the durable-snapshot etcd connection owner.

In particular, this checkpoint does not authorize:

- constructing `ReachabilityDurableSnapshotEtcdExecutor` inside Agent bootstrap;
- constructing `ReachabilityDurableSnapshotEtcdStore` inside Agent bootstrap;
- replacing any in-memory/test durable store with the etcd-backed store;
- activating durable recovery from etcd;
- adding endpoint/environment/config reads;
- adding startup failure modes associated with etcd connectivity;
- opening network connections during application startup.

Module registration remains non-activating.

## 7. Persistence semantics are frozen from C03e-HF/HG

This ownership selection does not modify the durable persistence protocol.

The following laws remain authoritative:

- exact canonical peer key/value/requested-peer binding;
- default-linearizable exact-key reads;
- semantic freshness validation before provider mutation;
- no create-if-absent behavior;
- exact dual CAS on observed positive `mod_revision` and exact observed raw value bytes;
- one exact Put on CAS success;
- one default-linearizable exact-key Get on CAS failure;
- definite stale classification only when authoritative evidence proves the expected state is no longer current;
- same expected token remaining after failed CAS is ambiguity/invariant failure, including different durable bytes;
- provider Txn RPC ambiguity maps fail closed;
- no blind retry after an indeterminate Txn;
- no scans, watches, leases, TTLs, background reconciliation, multi-peer transaction, or implicit repair.

C03e-HH cannot weaken these laws in the name of client composition.

## 8. Explicitly deferred composition decisions

The following remain unselected and require a later dedicated checkpoint:

1. the exact source module/type that owns etcd connection creation;
2. whether the durable-snapshot client is dedicated or shares a generic connection with another role;
3. endpoint source and normalization;
4. authentication and credential source;
5. TLS trust/client-identity configuration;
6. RBAC provisioning/required permissions;
7. connect options, deadlines, keepalive, retry, and reconnect policy;
8. startup/shutdown/error propagation semantics;
9. runtime/Agent constructor integration;
10. production activation and rollout.

No implementation may infer defaults for these decisions from this contract.

## 9. Required evidence before a future client-composition source checkpoint

Before any source materialization that creates or injects a real etcd client for this durable store, a later checkpoint must establish from the then-current exact source head:

- the concrete composition owner and module path;
- dependency direction and visibility of provider handles;
- whether generic-client sharing is permitted or prohibited;
- exact role-scoped capability handed to the durable executor;
- endpoint/config/TLS/auth/RBAC ownership boundaries;
- lifecycle and failure propagation at startup/shutdown;
- how the composition remains inactive until separately authorized;
- exact path ceiling and dependency/lockfile impact;
- focused tests that prove ownership without activating runtime behavior.

Historical bootstrap code alone is insufficient evidence.

## 10. C03e-HH materialization ceiling

C03e-HH is contract-only.

The complete intended repository delta is exactly one new path:

```text
contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HH_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_CLIENT_COMPOSITION_OWNERSHIP_SELECTION_STAGING.md
```

Allowed delta:

```text
1 Markdown contract
0 Rust source paths
0 Cargo manifests
0 Cargo lockfiles
0 workflow files
0 Agent/runtime paths
0 deployment/configuration paths
0 database/schema/auth/visibility paths
```

Any additional path is out of scope and blocks closure until separately selected.

## 11. Acceptance gate

C03e-HH may close only when exact-head evidence proves all of the following:

1. the branch descends directly from exact C03e-HG head `5657ac7858dac775be2817f98665e40b489c1e4a`;
2. the predecessor merge base is exactly that C03e-HG head;
3. the branch is ahead by exactly one ordinary commit and behind by zero;
4. the net predecessor-to-HH diff contains exactly the one contract path named above;
5. no Rust, manifest, lockfile, workflow, Agent/runtime, deployment/configuration, schema/auth, or visibility path changed;
6. the contract records that exact C03e-HG has no selected connection/bootstrap owner for this executor;
7. the contract preserves injected already-created `KvClient` as the current executor boundary;
8. the contract does not restore historical `bootstrap.rs`;
9. the contract does not select endpoint/TLS/auth/RBAC/reconnect/client-sharing/runtime decisions;
10. all C03e-HF/HG durable persistence semantics remain unchanged;
11. any automatically triggered exact-head validation is recorded accurately, including legitimate path-filter skips;
12. an immutable audit records exact head/tree/path/blob/diff/CI evidence;
13. no merge, deployment, runtime activation, or repository visibility mutation occurs.

Gate target after those facts are verified:

```text
C03E_HH_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_CLIENT_COMPOSITION_OWNERSHIP_SELECTED
```

## 12. Stop conditions

Stop and require a separate checkpoint before any action that would:

- create or connect an etcd client;
- choose or read endpoints;
- choose TLS/auth/RBAC/credentials;
- add/revive a bootstrap source module;
- pass provider connection configuration through bridge semantic APIs;
- modify Agent/runtime construction;
- activate the durable store in a running path;
- alter manifests/lockfiles/workflows;
- change durable-store semantics;
- merge, deploy, restart, or change repository visibility.

C03e-HH selects only the composition ownership boundary. It intentionally leaves the actual etcd connection owner unassigned until a later checkpoint can prove and select that lifecycle against current source topology.