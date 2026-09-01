# Private Remote Workspace — Phase 152 C03e-HJ Production Reachability Durable Snapshot etcd Security Principal / Keyspace Authorization Prerequisite Selection

Status: `STAGING / PREREQUISITE_SELECTION_ONLY / NO_SECURITY_MUTATION / NO_RUNTIME_ACTIVATION`

Gate target:

```text
C03E_HJ_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_SECURITY_PRINCIPAL_KEYSPACE_AUTHORIZATION_PREREQUISITE_BOUNDARY_SELECTED
```

Canonical predecessor: C03e-HI.

Exact predecessor head:

```text
0c4112e7afd9f587e21f64cce7b5d218b867c455
```

Exact predecessor tree:

```text
2950e7f89a8008160c237e2b7e36fe1c5f24ed50
```

This checkpoint selects only the provider-security authorization envelope that a future production durable-snapshot etcd client must satisfy before client/bootstrap source materialization may proceed. It does not provision a principal, certificate, key, RBAC role, grant, cluster, endpoint, connection, config field, Agent composition, runtime path, or production activation.

C03e-HJ is contract-only. It adds no Rust source and performs no auth/RBAC mutation.

## 1. Frozen predecessor laws

C03e-HI remains authoritative that the current two-role reachability authority bootstrap is evidence for role isolation only and is not authorization to add durable-snapshot as a third role by analogy.

The durable provider stack remains:

```text
ReachabilityDurableSnapshotEtcdStore
    -> already-created ReachabilityDurableSnapshotEtcdExecutor
        -> already-created role-scoped etcd_client::KvClient
```

No durable connection/bootstrap owner, cluster placement, TLS identity, trust policy, principal, RBAC grant, client-sharing policy, or Agent/runtime wiring is selected by C03e-HI.

C03e-HJ does not reopen those negative ownership and lifecycle laws.

## 2. Exact durable keyspace evidence

C03e-HD selected the exact dedicated durable-snapshot authority prefix:

```text
/prw/reachability/durable-snapshot/
```

That domain is explicitly distinct from the existing live-owner domain:

```text
/prw/reachability/live-owner/
```

HD selected exact-record access and did not authorize prefix/range scans as an authoritative read path. The prefix reserves the durable authority domain but does not itself authorize enumeration, Watch, range recovery, or another authority class.

C03e-HJ preserves that exact domain separation.

## 3. Exact provider-operation evidence

C03e-HF/HG selected and materialized the bounded durable etcd operation set:

- one default-linearizable exact-key Get for load/current observation;
- one authoritative exact-key Get before a compare-and-commit;
- one dual compare on the exact key using positive observed `mod_revision` and exact observed raw bytes;
- one exact-key Put on the transaction success branch;
- one default-linearizable exact-key Get on the compare-failure branch;
- no create-if-absent transaction;
- no scan/range enumeration;
- no Watch;
- no lease/TTL;
- no retry/reconciliation worker;
- no multi-peer transaction.

The raw provider executor accepts an already-created `KvClient` and performs no endpoint/client/security bootstrap.

C03e-HJ therefore has enough evidence to bound the required provider capability without selecting credential bytes or a concrete security provisioning mechanism.

## 4. Exact current security precedent

At exact C03e-HI head, current control-plane provider bootstrap remains:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

It is a bounded two-role bootstrap for live-owner authority and fence-sequence allocation. It uses distinct role-scoped mTLS identities and separate connections for those existing roles. It rejects exact certificate/private-key reuse across those two roles and performs no auth/RBAC mutation.

That source does not define a durable-snapshot identity, principal, role, grant, or client.

The existing live-owner or fence-allocation identities therefore provide precedent only. They are not durable-snapshot credentials by inheritance.

## 5. Selected durable authorization-envelope law

C03e-HJ selects the following provider-neutral security law:

```text
A future durable-snapshot etcd capability must be explicitly authorized
for the durable-snapshot authority domain and only for the provider
operations required by the frozen C03e-HF/HG exact-record protocol.
Existing live-owner or fence-allocation authority must not be treated as
implicit durable-snapshot authority.
```

This is a capability boundary, not a credential assignment.

A future source checkpoint may not accept a durable `KvClient` merely because it was produced for an existing reachability authority role.

## 6. Keyspace authorization ceiling

Any future durable-snapshot principal/RBAC selection must be constrained so that durable capability is no broader than the exact authority domain rooted at:

```text
/prw/reachability/durable-snapshot/
```

It must not gain durable-derived authorization to:

- `/prw/reachability/live-owner/`;
- fence-allocation authority keys;
- another PRW authority domain;
- cluster-wide arbitrary keys;
- administrative/auth/RBAC mutation surfaces.

C03e-HJ does not select the concrete etcd range expression, role name, user name, certificate subject, provisioning command, or credential mapping that would enforce this ceiling. Those are later security-materialization details.

## 7. Operation authorization ceiling

The future durable capability needs only the provider rights necessary to execute the frozen HG exact-record protocol: exact-key reads and the selected compare/Put transaction on durable-snapshot records.

C03e-HJ does not authorize capability for:

- Delete;
- prefix/range enumeration as an application protocol;
- Watch;
- lease management;
- auth/RBAC administration;
- member/cluster administration;
- maintenance operations;
- schema/provisioning mutation;
- unrelated keyspaces.

If a concrete provider permission model cannot express the selected application-operation boundary directly, a later security checkpoint must document the smallest enforceable permission set and any unavoidable excess before source materialization. HJ does not silently widen permissions.

## 8. Existing-principal reuse prohibition by default

C03e-HJ selects a fail-closed composition rule:

```text
No existing live-owner or fence-allocation principal/KvClient may be
reused for durable-snapshot unless a later explicit security selection
proves that the resulting capability satisfies this HJ authorization
envelope without silently widening either authority domain.
```

This rule does not itself mandate a third certificate, a third user, a third connection, or a separate cluster.

A later checkpoint may choose a dedicated principal or may justify another provider-specific construction, but it must do so explicitly from exact then-current policy/source evidence.

## 9. Credential and TLS boundary remains unselected

C03e-HJ does not select:

- certificate/private-key bytes;
- certificate subject/SAN conventions;
- whether a third dedicated mTLS identity is required;
- whether a trust bundle is shared or separate;
- credential source/custody/rotation/reload;
- username/password/token authentication;
- mapping from TLS identity to etcd auth principal;
- secret storage or deployment.

No secret or private-key material may enter a successor merely because HJ selected an authorization envelope.

## 10. Cluster and connection boundary remains unselected

C03e-HJ does not select:

- existing authority cluster versus a separate cluster;
- endpoint set or endpoint normalization;
- shared versus dedicated broad `etcd_client::Client`;
- number of network connections;
- connection pooling;
- reconnect/session/keepalive policy;
- timeout/retry/health supervision;
- provider connection lifecycle owner.

Current two-role same-cluster topology remains precedent only.

## 11. Bootstrap and Agent boundary remains unselected

C03e-HJ does not select or materialize:

- extension of `ReachabilityLiveOwnerEtcdBootstrapConfig`;
- extension of `reachability_acquisition_evidence::bootstrap`;
- a new durable provider bootstrap module;
- construction of `ReachabilityDurableSnapshotEtcdExecutor` in Agent;
- construction/injection of `ReachabilityDurableSnapshotEtcdStore` in Agent;
- startup/readiness dependency on etcd;
- durable recovery activation;
- task/background ownership;
- deployment or cutover.

Any such step requires a separately selected source/runtime checkpoint.

## 12. Frozen persistence semantics

C03e-HJ changes no durable storage semantics. In particular it preserves:

- exact canonical peer/key/value/requested-peer binding;
- default-linearizable exact-key reads;
- semantic freshness validation before mutation;
- no create-if-absent path;
- exact dual CAS on positive observed `mod_revision` plus exact observed raw bytes;
- exact Put success branch;
- exact Get compare-failure branch;
- fail-closed same-token/different-bytes ambiguity;
- fail-closed indeterminate Txn RPC behavior;
- no blind retry;
- no scan/Watch/lease/TTL/reconciliation/multi-peer transaction.

Security/client composition may not weaken those laws.

## 13. Required evidence before security/client source materialization

A later checkpoint must explicitly establish from exact then-current evidence at least:

1. concrete cluster placement;
2. exact endpoint/config ownership;
3. concrete durable authentication principal/identity strategy;
4. trust-bundle ownership;
5. concrete least-privilege RBAC/keyspace enforcement satisfying this HJ envelope;
6. provider-specific mapping from authentication identity to authorization principal;
7. dedicated/shared connection decision;
8. exact role-scoped capability handed to `ReachabilityDurableSnapshotEtcdExecutor`;
9. connection/session lifecycle owner;
10. exact source path/dependency/lockfile ceiling;
11. focused validation proving no cross-role capability reuse;
12. continued non-activation until a separately authorized runtime checkpoint.

If those facts are not established, the successor must remain contract-only.

## 14. C03e-HJ materialization ceiling

The complete intended repository delta is exactly one new path:

```text
contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HJ_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_SECURITY_PRINCIPAL_KEYSPACE_AUTHORIZATION_PREREQUISITE_SELECTION_STAGING.md
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
0 auth/RBAC/security material paths
0 visibility changes
```

Any additional changed path is a scope contradiction and blocks closure.

## 15. Acceptance gate

C03e-HJ may close only when exact-head evidence proves:

1. direct descent from exact C03e-HI head `0c4112e7afd9f587e21f64cce7b5d218b867c455`;
2. merge base exactly equals that HI head;
3. branch is ahead exactly one ordinary commit and behind zero;
4. predecessor-to-HJ diff contains exactly the one Markdown contract path above;
5. no Rust/manifest/lockfile/workflow/runtime/deployment/security/visibility path changed;
6. the contract preserves HI's prohibition on analogical third-role bootstrap extension;
7. the dedicated durable key domain remains `/prw/reachability/durable-snapshot/`;
8. provider capability remains bounded to the frozen HG exact-record protocol;
9. existing live-owner/fence authority is not treated as implicit durable authority;
10. no concrete principal/TLS/RBAC grant/cluster/connection/bootstrap/runtime choice is claimed as materialized;
11. exact-head CI is terminal with no failing or pending automatically triggered workflow;
12. immutable Drive audit records exact head/tree/path/blob/diff/CI evidence;
13. no merge, deployment, runtime activation, auth/RBAC mutation, credential handling, or visibility mutation occurs.

## 16. Target closure

Canonical closure target:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_SECURITY_PRINCIPAL_KEYSPACE_AUTHORIZATION_PREREQUISITE_SELECTION
```

Canonical gate target:

```text
C03E_HJ_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_SECURITY_PRINCIPAL_KEYSPACE_AUTHORIZATION_PREREQUISITE_BOUNDARY_SELECTED
```

Until the acceptance gate passes, HJ remains STAGING.

## 17. Safe successor

After HJ closure, begin with a fresh exact-HJ-head read-only audit.

A successor may select a concrete durable principal/TLS/RBAC/client topology only if exact current security policy and source evidence support every choice. Otherwise it must continue to defer concrete provider-client source materialization.

HJ does not authorize merge, deployment, runtime activation, endpoint selection, credential loading, RBAC mutation, or repository visibility change.
