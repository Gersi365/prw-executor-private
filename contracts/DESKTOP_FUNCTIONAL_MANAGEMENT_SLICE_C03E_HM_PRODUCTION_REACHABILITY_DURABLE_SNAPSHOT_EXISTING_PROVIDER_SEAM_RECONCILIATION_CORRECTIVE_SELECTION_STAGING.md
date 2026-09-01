# Private Remote Workspace — C03e-HM Production Reachability Durable Snapshot Existing Provider Seam Reconciliation Corrective Selection Staging

Status: `STAGED_CORRECTIVE_SELECTION_ONLY — DOCS_ONLY — NO_SOURCE_MATERIALIZATION — NO_RUNTIME_AUTHORIZATION`

## Purpose

This checkpoint corrects one source-topology premise introduced by C03e-HL before any Rust materialization is allowed to proceed.

C03e-HL correctly identified that a third durable-snapshot etcd connection must never be created as an orphan. However, its exact-source review was scoped too narrowly to `crates/prw-control-plane/src/reachability_acquisition_evidence/` and therefore incorrectly concluded that the repository did not yet contain the durable-snapshot provider executor/store seam.

Exact C03e-HL source proves that seam already exists across the existing dependency boundary:

```text
etcd_client::KvClient
    -> prw-control-plane::ReachabilityDurableSnapshotEtcdExecutor
        -> prw-remote-bridge::ReachabilityDurableSnapshotEtcdStore
            -> ReachabilityDurableStore
                -> ProductionReachabilityOwner<S, T>
```

C03e-HM therefore supersedes only the C03e-HL selections that would create a second durable-snapshot backend/store under `reachability_acquisition_evidence` or make the live-owner preparation facade own duplicated durable semantics.

This checkpoint does not modify Rust source, connect to etcd, load credentials, change TLS/auth/RBAC, activate durable recovery, change Agent/runtime composition, deploy, restart, merge, or mutate production state.

## Exact predecessor guard

Canonical predecessor: C03e-HL.

Exact predecessor head:

```text
16b33ae3b45ce0860bd17fdaf5481410d9215ccb
```

Exact predecessor tree:

```text
eae4b9e1904c570d487e3f501956142d503baba1
```

C03e-HM must remain a direct one-commit docs-only successor of that exact head.

## 1. Exact current-source evidence

At exact C03e-HL head, the durable provider seam is already materialized.

### 1.1 Raw provider executor already exists

Exact path:

```text
crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs
```

Exact blob:

```text
77fc9f345c17c5722c5240f3cead7ea68cb55cac
```

The existing `ReachabilityDurableSnapshotEtcdExecutor`:

- owns one already-created `etcd_client::KvClient`;
- performs exact-key linearizable Get;
- performs the already-selected exact dual-CAS transaction and replacement Put;
- owns provider response-shape validation;
- does not own endpoint selection, TLS/auth/RBAC, credentials, connection creation, retries, runtime tasks, or deployment.

Its constructor is already the required provider narrowing seam:

```text
ReachabilityDurableSnapshotEtcdExecutor::new(kv: KvClient)
```

### 1.2 Bridge semantic durable store already exists

Exact path:

```text
crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs
```

Exact blob:

```text
a381963986c79f8a314088839316d47595ba8686
```

The existing `ReachabilityDurableSnapshotEtcdStore`:

- owns one `ReachabilityDurableSnapshotEtcdExecutor`;
- implements the existing `ReachabilityDurableStore` semantic boundary;
- derives and validates the canonical durable key/value representation;
- enforces requested-peer/key/value binding;
- maps provider results into the existing fail-closed reachability persistence semantics;
- does not own endpoint/TLS/auth/RBAC/credential/bootstrap/runtime/deployment behavior.

Its constructor is already the required semantic adapter seam:

```text
ReachabilityDurableSnapshotEtcdStore::new(provider)
```

### 1.3 Production reachability owner already owns a generic durable store

Exact path:

```text
crates/prw-remote-bridge/src/reachability_owner.rs
```

Exact blob:

```text
8de2e3d21224b339a7d18e926f5127838c903608
```

The existing production owner is:

```text
ProductionReachabilityOwner<S, T>
where
    S: ReachabilityDurableStore
```

and it owns:

```text
store: S
```

`ProductionReachabilityOwner::recover(...)` already accepts the concrete durable store by value and performs authoritative durable recovery through the `ReachabilityDurableStore` boundary.

Therefore the repository already contains the semantic destination for `ReachabilityDurableSnapshotEtcdStore`; the missing work is concrete construction/injection composition, not a new storage abstraction.

### 1.4 Existing control-plane etcd bootstrap remains two-role

Exact path:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

Exact blob:

```text
c454ebe01c9dfdfe6a9117fecffc983f0afe7600
```

The current bootstrap:

- accepts one validated three-member HTTPS endpoint vector;
- accepts one trust bundle;
- accepts distinct live-owner and fence-allocation mTLS identities;
- creates two separate etcd clients;
- narrows each to a `KvClient`;
- drops each broad `Client` handle;
- returns `ReachabilityLiveOwnerAcquisitionPreparation`.

It does not yet construct the dedicated durable-snapshot role connection.

### 1.5 Existing live-owner preparation is not the durable semantic owner

Exact path:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/preparation.rs
```

Exact blob:

```text
2e589a560f102796652475ea7390bec2076791ac
```

`ReachabilityLiveOwnerAcquisitionPreparation` currently owns only:

```text
ReachabilityLiveOwnerEtcdStore
FenceSequenceAllocationEtcdStore
```

This is a control-plane provider-preparation facade. It cannot directly own the bridge-defined `ReachabilityDurableSnapshotEtcdStore` without reversing the existing dependency direction.

The established dependency direction remains:

```text
prw-remote-bridge -> prw-control-plane -> etcd-client
```

C03e-HM therefore forbids solving the composition gap by adding a `prw-control-plane -> prw-remote-bridge` dependency.

## 2. Corrective finding

The following C03e-HL premise is incorrect at the exact C03e-HL head:

```text
"the exact C03e-HK source tree does not yet contain a durable-snapshot runtime backend/store"
```

The exact source evidence above proves that both provider layers already exist.

The actual unresolved prerequisite is narrower:

```text
select the current composition owner that receives the future dedicated durable KvClient,
constructs the existing executor and existing bridge store,
and transfers that concrete ReachabilityDurableStore into ProductionReachabilityOwner
without introducing an orphan provider connection or dependency reversal.
```

## 3. C03e-HL selections superseded by this corrective

C03e-HM supersedes only the following C03e-HL implementation selections:

1. creating `crates/prw-control-plane/src/reachability_acquisition_evidence/durable_snapshot.rs` as a new durable backend/store;
2. duplicating HJ/HF/HG durable semantics inside that new module;
3. extending `ReachabilityLiveOwnerAcquisitionPreparation` to own a second durable-snapshot backend/store;
4. treating creation of that duplicate backend as mandatory Step A before later provider wiring.

Those selections are no longer authorized for a future source checkpoint.

No existing source file is deleted or renamed by C03e-HM. The C03e-HL contract remains immutable historical evidence; C03e-HM records the corrective precedence.

## 4. Selections from C03e-HL that remain valid

The following C03e-HL principles remain valid and are preserved:

- no orphan durable provider connection;
- no generic/global raw etcd client;
- no provider connection before a legitimate consumer destination is identified;
- no credential/client reuse across live-owner, fence-allocation, and durable-snapshot roles;
- no public raw `KvClient`/`Client` API;
- no new process/service/listener/daemon;
- no dependency upgrade merely for this slice;
- fail-closed operation;
- runtime/provider activation remains separately gated.

## 5. Existing durable provider chain is authoritative

Future source work must reuse the existing chain exactly rather than creating a parallel implementation:

```text
Dedicated durable role connection
    -> dedicated durable KvClient
        -> ReachabilityDurableSnapshotEtcdExecutor::new(kv)
            -> ReachabilityDurableSnapshotEtcdStore::new(executor)
                -> ProductionReachabilityOwner<ReachabilityDurableSnapshotEtcdStore, T>
```

The exact higher-level owner/composition constructor that performs this assembly is intentionally not selected by C03e-HM because that source owner still requires a separate exact-tree audit.

No future source checkpoint may infer that owner from historical bootstrap prose alone.

## 6. Dependency and ownership law

The following ownership law is selected:

### Control-plane owns provider execution and provider connection construction only where explicitly selected

`prw-control-plane` may continue to own:

- etcd `Client::connect` where a later source checkpoint explicitly extends the current bootstrap;
- role-scoped client narrowing;
- raw provider executor construction inputs.

It must not import bridge semantic types merely to retain the durable store.

### Bridge owns durable reachability semantics

`prw-remote-bridge` continues to own:

- `ReachabilityDurableSnapshot` semantic representation;
- durable key/value codec binding;
- `ReachabilityDurableStore`;
- `ReachabilityDurableSnapshotEtcdStore`;
- `ProductionReachabilityOwner<S,T>` and durable recovery/commit semantics.

### A higher composition boundary must join them

A later checkpoint must identify a current source boundary that is permitted to depend on both the required control-plane bootstrap output and bridge owner/store types, or must select a minimal dependency-preserving handoff shape that avoids reverse dependency.

That later checkpoint must not create a generic service locator, global singleton, runtime registry, or arbitrary provider factory merely to bridge the two crates.

## 7. HJ/HF/HG durable semantics remain frozen

C03e-HM changes no persistence semantics.

The existing selected durable protocol remains authoritative, including:

- exact canonical peer-derived key;
- `/prw/reachability/durable-snapshot/` durable namespace;
- canonical fixed-width durable value representation selected by the existing codecs;
- exact-key linearizable reads;
- fail-closed requested-peer/key/value validation;
- selected expected-current compare-and-commit behavior;
- no prefix scans;
- no arbitrary range read surface;
- no Watch;
- no lease/TTL ownership;
- no blind mutation retry after indeterminate provider outcome;
- no implicit repair or in-memory authority fallback.

C03e-HM does not alter any codec, provider transaction, or owner recovery rule.

## 8. HK security/topology selections remain frozen future requirements

C03e-HM does not reopen the C03e-HK security/topology selections for the future durable role connection.

Future connection composition remains constrained by the already-selected requirements, including:

- same logical three-member reachability-authority etcd cluster;
- same validated runtime-supplied three-HTTPS-endpoint vector;
- same runtime-supplied trust authority;
- pinned reachability etcd TLS server identity as selected by HK;
- dedicated durable-snapshot principal `prw-reachability-durable-snapshot`;
- dedicated role `prw-reachability-durable-snapshot-rw` restricted to the durable-snapshot namespace;
- dedicated durable certificate/private-key identity;
- role-isolated connection/client context;
- only the dedicated durable `KvClient` crossing the provider executor boundary;
- no credential fallback to live-owner or fence-allocation identities.

This checkpoint does not assert that those future requirements are already materialized in current Rust source.

## 9. Next prerequisite selected by C03e-HM

The next checkpoint must be an exact-source **composition-owner selection**, not source implementation.

Before any Rust modification, that checkpoint must prove:

1. which current crate/module owns construction of `ProductionReachabilityOwner` in the production path;
2. which current crate/module may legally construct `ReachabilityDurableSnapshotEtcdStore` without dependency reversal;
3. how a dedicated durable `KvClient` can reach that composition owner without exposing a broad raw `Client` publicly;
4. whether the current control-plane bootstrap should return a narrow durable provider handle, a dedicated executor, or another minimal dependency-preserving handoff;
5. how the concrete store is transferred immediately into `ProductionReachabilityOwner` so no connection is orphaned;
6. how startup/recovery failure propagates while runtime activation remains disabled until separately authorized;
7. the exact source-path ceiling for the eventual implementation;
8. focused tests that prove dependency direction, role isolation, and non-orphan composition.

No source implementation may begin before those points are selected from the then-current exact head.

## 10. Explicitly forbidden implementation shortcuts

C03e-HM forbids future source work from:

- creating a second durable-snapshot etcd executor;
- creating a second durable-snapshot semantic store;
- creating `reachability_acquisition_evidence/durable_snapshot.rs` merely to satisfy superseded C03e-HL Step A;
- moving bridge durable semantics into control-plane;
- adding a reverse `prw-control-plane -> prw-remote-bridge` dependency;
- making `ReachabilityLiveOwnerAcquisitionPreparation` the semantic durable owner without a separate architecture change;
- passing a generic raw `etcd_client::Client` through bridge public APIs;
- exposing arbitrary key/value provider methods;
- creating a third durable connection before the concrete consumer/store construction path is proven;
- retaining an otherwise unused raw `Client` merely to keep the connection alive;
- using a global/static provider handle;
- reusing live-owner or fence credentials/client context as a temporary durable role;
- falling back to in-memory authority when durable provider composition fails.

## 11. Docs-only materialization ceiling

C03e-HM changes exactly one repository path:

```text
contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HM_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_EXISTING_PROVIDER_SEAM_RECONCILIATION_CORRECTIVE_SELECTION_STAGING.md
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
0 database/schema/auth/RBAC paths
0 certificate/PKI paths
0 repository visibility paths
```

Any additional changed path blocks closure.

## 12. Acceptance gate

C03e-HM may close only when exact-head evidence proves:

1. direct ancestry from C03e-HL head `16b33ae3b45ce0860bd17fdaf5481410d9215ccb`;
2. exactly one ordinary commit ahead of C03e-HL and zero behind;
3. exactly one changed Markdown contract path;
4. no source/manifests/lockfiles/workflows/runtime/deployment/auth/visibility changes;
5. the corrective records exact current existence of both durable provider layers;
6. the corrective preserves `ReachabilityDurableStore` and `ProductionReachabilityOwner<S,T>` as the semantic owner boundary;
7. the corrective forbids duplicate backend/store materialization and dependency reversal;
8. HJ/HF/HG persistence semantics remain unchanged;
9. HK security/topology selections remain unchanged future requirements;
10. the next checkpoint is limited to exact-source composition-owner selection;
11. any exact-head CI/check result is reported accurately, including legitimate path-filter skips;
12. no merge, deployment, runtime activation, RBAC/certificate mutation, or repository visibility mutation occurs.

Gate target:

```text
C03E_HM_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_EXISTING_PROVIDER_SEAM_RECONCILIATION_CORRECTED
```

## 13. Stop conditions

Stop and require a separate checkpoint before any action that would:

- modify Rust source;
- create or connect the durable etcd client;
- modify endpoint/TLS/auth/RBAC/credential configuration;
- alter crate dependency direction;
- construct or activate the production reachability owner in a new runtime path;
- change recovery/startup/shutdown behavior;
- provision etcd users/roles/permissions;
- issue/install/rotate certificates;
- mutate etcd members or cluster state;
- deploy, restart, merge, or change repository visibility.

C03e-HM is solely a corrective selection that reconciles C03e-HL with exact existing HG/HH source topology before any implementation proceeds.