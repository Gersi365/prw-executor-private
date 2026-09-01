# Private Remote Workspace — Phase 152 C03e-HI Production Reachability Durable Snapshot etcd Provider Client Role-Isolation Prerequisite Selection

Status: `STAGING / PREREQUISITE_SELECTION_ONLY / NO_SOURCE_MATERIALIZATION / NO_RUNTIME_ACTIVATION`

Gate target:

```text
C03E_HI_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_CLIENT_ROLE_ISOLATION_PREREQUISITE_BOUNDARY_SELECTED
```

Canonical predecessor: C03e-HH.

Exact predecessor head:

```text
195bc3254622f64884300bc36d211be36ef4e5e8
```

Exact predecessor tree:

```text
adcd5a7696ebf572d7ad1b6cbc18113008d76942
```

This checkpoint selects only the prerequisite boundary that must be satisfied before any production durable-snapshot etcd provider client can be composed. It deliberately does **not** select a durable-snapshot cluster, TLS identity, RBAC policy, connection-sharing policy, bootstrap config extension, Agent/runtime wiring, or production activation.

C03e-HI is contract-only. It does not add Rust source, alter Cargo manifests or lockfiles, change workflows, connect to etcd, read endpoint or secret material, mutate auth/RBAC, modify Agent/runtime composition, deploy, restart, merge, or modify repository visibility.

## 1. Frozen predecessor ownership law

C03e-HH remains authoritative for the current durable provider stack:

```text
ReachabilityDurableSnapshotEtcdStore
    -> already-created ReachabilityDurableSnapshotEtcdExecutor
        -> already-created role-scoped etcd_client::KvClient
```

The bridge remains semantic. The control-plane durable executor remains provider-specific. Neither durable layer owns endpoint selection or `etcd_client::Client::connect`.

C03e-HH also remains authoritative that no durable-snapshot connection/bootstrap owner is selected. An already-created, appropriately scoped `KvClient` is still an external composition prerequisite.

C03e-HI does not reopen or weaken that negative ownership law.

## 2. Exact current bootstrap evidence

At exact C03e-HH head `195bc3254622f64884300bc36d211be36ef4e5e8`, the current provider bootstrap exists at:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

Exact blob:

```text
c454ebe01c9dfdfe6a9117fecffc983f0afe7600
```

That source is explicitly a bounded **two-role** authority bootstrap. Its documented and materialized roles are:

1. live-owner authority;
2. fence-sequence allocation.

The exact source establishes all of the following for those two roles only:

- one exact three-member HTTPS authority endpoint set is shared by both roles;
- one explicit private trust bundle is shared by both roles;
- each role receives distinct non-printable mTLS identity material;
- exact client-certificate reuse across those two roles is rejected;
- exact private-key reuse across those two roles is rejected;
- one independent `etcd_client::Client::connect(...)` call is performed for live-owner;
- one independent `etcd_client::Client::connect(...)` call is performed for fence allocation;
- each broad `Client` is narrowed to its own `KvClient`;
- broad `Client` handles are dropped before the preparation is returned;
- the bootstrap does not mutate auth/RBAC and does not activate runtime tasks or production service state.

The current bootstrap therefore proves a role-isolation precedent for the already-selected live-owner and fence-allocation roles.

It does **not** prove that durable-snapshot is a third role in that bootstrap.

## 3. Exact current Agent composition evidence

At the same exact C03e-HH head, current Agent composition remains split from provider bootstrap.

Current Agent provider/bootstrap composition path:

```text
crates/prw-agent/src/reachability_authority_bootstrap.rs
```

Exact blob:

```text
fd93c7180801c36925d48e3b10dcb9eeed9690df
```

It consumes `ReachabilityLiveOwnerEtcdBootstrapConfig`, calls the control-plane live-owner/fence preparation bootstrap, and composes the resulting live-owner authority. Its own source states that it does not load credentials, alter Agent startup/readiness, create tasks, execute authority lifecycle operations, or wire the function into the running Agent.

Current pure semantic handoff path:

```text
crates/prw-agent/src/reachability_authority_composition.rs
```

Exact blob:

```text
91a639bfcc568a1932064f6745af9b04485f444c
```

That seam consumes an already-created live-owner preparation and performs no provider I/O or runtime activation.

Neither exact Agent seam selects or composes a durable-snapshot etcd client.

## 4. Selected prerequisite law

C03e-HI selects the following law:

```text
The existing two-role authority bootstrap is evidence for isolation,
not authorization to add durable-snapshot as a third role by analogy.
```

Before any durable-snapshot provider-client source materialization, a later dedicated selection checkpoint must explicitly choose the durable role's provider/security/lifecycle boundaries from exact then-current evidence.

Until that later selection exists, source materialization that creates, narrows, injects, or wires a durable production etcd client is blocked.

## 5. Durable-specific choices intentionally remain unselected

C03e-HI does **not** decide any of the following:

### 5.1 Cluster placement

Not selected:

- whether durable-snapshot uses the existing three-member reachability authority cluster;
- whether it uses a separate etcd cluster;
- whether endpoint discovery/normalization is shared with existing roles;
- whether a separate endpoint configuration object is required.

The fact that live-owner and fence allocation share one cluster is not sufficient authorization to place durable-snapshot there.

### 5.2 TLS identity and trust

Not selected:

- whether durable-snapshot receives a third dedicated client certificate/private key;
- whether any existing role identity may be reused;
- whether the existing private trust bundle is reused;
- certificate rotation, expiry, reload, or custody mechanics for a durable role.

The fact that the existing two roles reject certificate/private-key reuse is a role-isolation precedent, not a durable identity assignment.

### 5.3 RBAC and key ownership

Not selected:

- the durable-snapshot principal;
- exact read/write permissions;
- key-prefix ownership;
- whether permissions overlap with live-owner or fence-allocation roles;
- provisioning or mutation of etcd auth/RBAC state.

C03e-HI performs no auth/RBAC mutation.

### 5.4 Connection and client isolation

Not selected:

- whether durable-snapshot obtains a dedicated broad `etcd_client::Client`;
- whether it shares a broad client with another role;
- whether a higher-level owner supplies an already-narrowed `KvClient`;
- connection pooling, reconnect/session ownership, keepalive, retry, timeout, or health supervision.

The existing two independent `Client::connect(...)` calls prove current isolation for live-owner and fence allocation only.

### 5.5 Bootstrap source ownership

Not selected:

- whether `reachability_acquisition_evidence::bootstrap` is extended;
- whether a separate control-plane bootstrap module is introduced;
- whether a higher-level process composition seam owns durable client creation;
- the exact durable bootstrap config type or fields.

No current source path is promoted by C03e-HI into durable connection ownership.

### 5.6 Agent/runtime integration

Not selected:

- construction of `ReachabilityDurableSnapshotEtcdExecutor` in Agent bootstrap;
- construction of `ReachabilityDurableSnapshotEtcdStore` in Agent bootstrap;
- replacement of any in-memory/test durable store;
- startup/readiness dependencies on etcd;
- durable recovery activation;
- background tasks;
- production rollout or cutover.

## 6. Explicit prohibition on analogical extension

A later implementation must not reason as follows:

```text
live-owner + fence allocator already use the authority cluster,
therefore durable-snapshot may be added to the same bootstrap with a third client.
```

That conclusion is not selected by current contracts or source.

Likewise, current separation of the two existing mTLS identities must not be converted into an implicit durable rule such as:

```text
durable-snapshot must receive a third identity and third connection.
```

That may be a reasonable candidate, but it requires a separate explicit selection backed by security, RBAC, key-ownership, and lifecycle evidence.

## 7. Frozen persistence semantics

C03e-HI changes no C03e-HF/HG persistence semantics.

The following remain frozen:

- exact canonical peer/key/value/requested-peer binding;
- default-linearizable exact-key reads;
- semantic freshness validation before provider mutation;
- no create-if-absent behavior;
- exact dual CAS on positive observed `mod_revision` and exact observed raw bytes;
- one exact replacement Put on CAS success;
- one authoritative exact-key Get on compare failure;
- same expected token remaining after failed CAS maps to ambiguity/invariant failure, including different durable bytes;
- provider Txn RPC ambiguity fails closed;
- no blind retry after an indeterminate Txn;
- no scans, watches, leases, TTLs, background reconciliation, multi-peer transaction, or implicit repair.

Provider-client composition may not weaken these laws.

## 8. Required evidence for the next durable client selection checkpoint

Before source materialization may resume, a later checkpoint must establish from exact current source and selected policy evidence:

1. same-cluster versus separate-cluster placement;
2. endpoint/config ownership and normalization rules;
3. dedicated versus reusable TLS client identity;
4. trust-bundle ownership;
5. exact RBAC principal and permissions for the durable keyspace;
6. exact key/prefix ownership boundary;
7. dedicated versus shared broad client/connection;
8. exact role-scoped capability handed to `ReachabilityDurableSnapshotEtcdExecutor`;
9. provider connection lifecycle owner;
10. startup/shutdown/reconnect/error propagation policy;
11. exact control-plane versus Agent composition boundary;
12. how source remains non-activating until a separately authorized runtime checkpoint;
13. exact path/dependency/lockfile ceiling and focused tests.

If those facts are not selected explicitly, the next checkpoint must remain contract-only.

## 9. C03e-HI materialization ceiling

The complete intended repository delta is exactly one new path:

```text
contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HI_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_CLIENT_ROLE_ISOLATION_PREREQUISITE_SELECTION_STAGING.md
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

Any additional path is out of scope and blocks closure.

## 10. Acceptance gate

C03e-HI may close only when exact-head evidence proves all of the following:

1. the branch descends directly from exact C03e-HH head `195bc3254622f64884300bc36d211be36ef4e5e8`;
2. predecessor merge base is exactly that C03e-HH head;
3. branch is ahead by exactly one ordinary commit and behind by zero;
4. predecessor-to-HI diff contains exactly the one Markdown contract path named above;
5. no Rust, manifest, lockfile, workflow, Agent/runtime, deployment/configuration, schema/auth, or visibility path changed;
6. the contract preserves the C03e-HH unassigned durable connection-owner law;
7. the contract records the current two-role control-plane bootstrap as precedent only;
8. no durable cluster placement is selected;
9. no durable TLS identity/trust policy is selected;
10. no durable RBAC/keyspace policy is selected;
11. no durable connection-sharing policy is selected;
12. no bootstrap source path or Agent/runtime seam is selected for durable wiring;
13. C03e-HF/HG persistence semantics remain frozen;
14. exact-head validation is recorded accurately, including legitimate path-filter skips;
15. immutable audit records exact head/tree/path/blob/diff/CI evidence;
16. no merge, deployment, runtime activation, auth/RBAC mutation, or repository visibility mutation occurs.

Gate target after those facts are verified:

```text
C03E_HI_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_CLIENT_ROLE_ISOLATION_PREREQUISITE_BOUNDARY_SELECTED
```

## 11. Stop conditions

Stop and require a separate checkpoint before any action that would:

- add durable-snapshot to the existing two-role bootstrap;
- create/connect/narrow a durable etcd client;
- choose or read durable endpoints;
- choose or load durable TLS/auth credentials;
- create or alter durable RBAC policy;
- decide shared versus dedicated provider connection;
- modify `ReachabilityLiveOwnerEtcdBootstrapConfig` for durable use;
- add a durable bootstrap source module;
- wire durable provider state through Agent/runtime construction;
- activate durable recovery or persistence in a running path;
- alter manifests/lockfiles/workflows;
- change durable-store semantics;
- merge, deploy, restart, or change repository visibility.

C03e-HI selects only the prerequisite boundary: current role-isolation evidence is insufficient to authorize durable provider-client source materialization.