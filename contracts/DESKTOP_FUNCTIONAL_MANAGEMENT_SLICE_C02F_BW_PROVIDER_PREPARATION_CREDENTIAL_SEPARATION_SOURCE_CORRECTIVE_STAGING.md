# Phase 152 C02f-BW — Provider Preparation Credential Separation Source Corrective Staging

Status: `SOURCE_CORRECTIVE_STAGING / BV_DERIVED / ROLE_SEPARATED_KVCLIENT_INPUTS / CONTROL_PLANE_OWNED_RAW_CLIENT_PAIRING / PREPARATION_FACADE_PRESERVED / NO_TLS_FEATURE_MATERIALIZATION / NO_CARGO_OR_LOCKFILE_MUTATION / NO_ENDPOINT_VALUES / NO_SECRET_MATERIAL / NO_CONNECT / NO_AUTH_RBAC_MUTATION / NO_RUNTIME_ACTIVATION / NO_RECOVERY_EXECUTION / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22
Repository: `Gersi365/prw-executor-private`

## Authorization

User authorization for this tranche is interpreted narrowly as approval for the next provider-preparation credential-separation source corrective only.

TLS feature enablement, Cargo/lockfile mutation, endpoint materialization/contact, certificate/private-key material, etcd auth/RBAC mutation, runtime activation, deployment and merge remain excluded.

## Exact prerequisite

C02f-BW derives only from closed C02f-BV:

- base branch: `phase-152-c02f-bv-provider-client-bootstrap-composition-selection-staging`;
- base SHA: `29ee45bc6a822484a434df3003ecbd56eca3b60b`;
- base tree: `2b8327ebcd51671daf1c8fef604d97da0c604796`;
- inherited gate: `C02F_BV_PROVIDER_CLIENT_BOOTSTRAP_COMPOSITION_SELECTED`.

BV identified that the current C02f-BM preparation constructor accepts one authenticated `KvClient` and derives both live-owner and fence-allocation stores from that one client context. That shape cannot preserve the closed AG live-owner principal separation and AI fence-allocator principal separation under the selected mTLS/CN authentication model.

## Corrective objective

C02f-BW corrects only the already-materialized preparation construction seam.

The corrective must ensure that:

1. the preparation facade owns one live-owner store backed only by a live-owner role-scoped `KvClient`;
2. the preparation facade owns one fence-sequence allocation store backed only by a fence-allocator role-scoped `KvClient`;
3. the old public one-`KvClient` construction path no longer exists;
4. raw role-scoped `KvClient` pairing remains owned inside `prw-control-plane`, not exposed to `prw-remote-bridge` or arbitrary external callers;
5. preparation, acquisition execution, lifecycle execution, currentness and release semantics remain unchanged;
6. no provider connection, TLS configuration, endpoint selection or credential loading is materialized here.

## Selected source shape

`ReachabilityLiveOwnerAcquisitionPreparation` remains the outward preparation facade already consumed by BU.

Its provider-specific raw-client constructor becomes crate-private and receives two explicit role-scoped handles:

```text
from_role_scoped_clients(
    live_owner_kv,
    fence_allocator_kv,
) -> ReachabilityLiveOwnerAcquisitionPreparation
```

The constructor performs no network I/O and does not select endpoints, TLS, credentials or runtime ownership.

The live-owner client is consumed only by `ReachabilityLiveOwnerEtcdStore`.

The fence-allocator client is consumed only by `FenceSequenceAllocationEtcdStore`.

No cloning of one supplied client into both authority roles is permitted.

## Same-cluster invariant

C02f-BW does not materialize the future bootstrap/configuration object selected by BV.

Therefore the two raw clients are accepted only through a crate-private control-plane seam. The later separately authorized provider-bootstrap tranche remains responsible for deriving both role-scoped clients from one validated immutable logical authority-cluster configuration before calling this constructor.

C02f-BW does not claim runtime cluster-ID verification or endpoint-contact proof.

## Compatibility boundary

The following are preserved unchanged:

- `ReachabilityLiveOwnerAcquisitionPreparation::prepare` semantics;
- `ReachabilityLiveOwnerAcquisitionExecution` semantics;
- `ReachabilityLiveOwnerLifecycleExecution` semantics;
- BU bridge constructor `ReachabilityLiveOwnerComposedAsyncAuthority::new(preparation)`;
- live-owner and fence-allocation provider algorithms and reconciliation behavior;
- provider-neutral evidence and error mappings.

Any call site that previously depended on public direct construction from one raw `KvClient` is intentionally blocked by this corrective and must not be restored by widening credentials or reintroducing one-client cloning.

## Validation target

The final BW head must demonstrate:

- exact BV merge base;
- only the authorized contract/source corrective diff;
- Cargo manifests and `Cargo.lock` byte-stable;
- no workflow, Android, Agent, runtime or deployment mutation;
- canonical Rust validation through locked dependency graph, fmt, Clippy, workspace tests and workspace build.

Provider workflows may be path-filtered according to their existing definitions; no result may be claimed unless a corresponding run is registered for the exact final head.

## Explicit exclusions

C02f-BW does **not** authorize or materialize:

- `etcd-client` TLS feature enablement;
- any Cargo dependency or lockfile change;
- endpoints, ports, DNS names or topology values;
- CA/certificate/private-key material or secret loading;
- `Client::connect` or other endpoint contact;
- etcd users, roles, permissions or auth enablement;
- recovery execution or sequence-head initialization;
- runtime/executor/task ownership;
- Agent integration;
- R1-R4 activation;
- deployment;
- retargeting or merge.

## Gate target

`C02F_BW_PROVIDER_PREPARATION_CREDENTIAL_SEPARATION_SOURCE_CORRECTED`

The gate may close only after exact-head source validation and evidence persistence.