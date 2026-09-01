# Private Remote Workspace — C03e-HN Production Reachability Durable Snapshot Composition Owner Selection Staging

Status: `STAGED_SELECTION_ONLY — DOCS_ONLY — NO_SOURCE_MATERIALIZATION — NO_RUNTIME_AUTHORIZATION`

Gate target:

```text
C03E_HN_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_COMPOSITION_OWNER_BOUNDARY_SELECTED
```

## Purpose

C03e-HN performs the exact-source composition-owner selection required by C03e-HM before any additional Rust materialization is allowed.

C03e-HM corrected the repository topology and proved that the durable provider executor and semantic store already exist. The remaining question is not how to invent another durable backend. The remaining question is where the existing layers may be joined without reversing dependencies, exposing a broad provider client, creating an orphan provider connection, or activating production runtime behavior.

C03e-HN selects that boundary from the exact current source tree only.

This checkpoint does **not** modify Rust source, connect to etcd, load or add credentials, change TLS/auth/RBAC, recover a production reachability owner, activate candidate publication, alter Agent startup/readiness, deploy, restart, merge, or mutate production state.

## Exact predecessor guard

Canonical predecessor: C03e-HM.

Exact predecessor head:

```text
03cce394d1e0ed06ca03175cc56f147a58a59261
```

Exact predecessor tree:

```text
1f5a2dc5f35628401519acab49cacc039585b2ad
```

C03e-HN must remain a direct one-commit docs-only successor of that exact head.

## 1. Frozen C03e-HM correction

C03e-HM remains authoritative that the existing durable provider chain is:

```text
Dedicated durable role connection
    -> dedicated durable KvClient
        -> ReachabilityDurableSnapshotEtcdExecutor::new(kv)
            -> ReachabilityDurableSnapshotEtcdStore::new(executor)
                -> ProductionReachabilityOwner<S, T>
```

No parallel durable executor, semantic store, codec, persistence abstraction, or provider transaction protocol may be created by a successor merely to make composition easier.

The following dependency direction remains frozen:

```text
prw-remote-bridge -> prw-control-plane -> etcd-client
```

No successor may add `prw-control-plane -> prw-remote-bridge`.

## 2. Exact current-source evidence — process composition root

At the exact C03e-HM head, `prw-agent` already depends on both crates that must be joined:

```text
crates/prw-agent/Cargo.toml
```

Exact blob:

```text
4c70d6be9b56f39edc10810eefa3428314ed7559
```

The manifest already contains both:

```text
prw-control-plane = { path = "../prw-control-plane" }
prw-remote-bridge = { path = "../prw-remote-bridge" }
```

No new crate dependency is required for Agent-level composition.

Exact Agent source also contains established cross-crate composition precedents:

```text
crates/prw-agent/src/reachability_authority_composition.rs
crates/prw-agent/src/reachability_authority_bootstrap.rs
crates/prw-agent/src/reachability_authority_custody_bootstrap.rs
```

`reachability_authority_composition.rs` explicitly records the already-selected law that `prw-agent` is the process-level composition root while `prw-control-plane` retains provider/bootstrap ownership and `prw-remote-bridge` retains bridge semantic ownership.

Therefore C03e-HN selects:

```text
prw-agent is the production durable-snapshot cross-crate composition owner.
```

This is a composition ownership selection, not runtime activation.

## 3. Exact current-source evidence — ProductionReachabilityOwner construction

Exact path:

```text
crates/prw-agent/src/production_reachability_owner_custody.rs
```

Exact blob at C03e-HM:

```text
f006a74e21492cae36e04aee06bc2b23a0206b7d
```

The current Agent-owned custody type is:

```text
ProductionReachabilityOwnerCustody<S, T>
```

and its existing authoritative construction seam is:

```text
ProductionReachabilityOwnerCustody::recover(
    store: S,
    token_source: T,
    peer: &PeerConnectivityIdentity,
)
```

That method immediately delegates to:

```text
ProductionReachabilityOwner::recover(store, token_source, peer)
```

and retains the recovered owner by value.

C03e-HN therefore selects `production_reachability_owner_custody.rs` and specifically `ProductionReachabilityOwnerCustody::recover(...)` as the already-existing final Agent-owned construction destination. A successor must reuse this seam rather than creating a second production owner wrapper or bypassing Agent custody.

## 4. Exact current-source evidence — durable store ownership law

Exact bridge owner path:

```text
crates/prw-remote-bridge/src/reachability_owner.rs
```

Exact blob:

```text
8de2e3d21224b339a7d18e926f5127838c903608
```

The current owner is generic over:

```text
S: ReachabilityDurableStore
T: CandidatePublicationFreshnessTokenSource
```

and owns:

```text
store: S
```

by value.

`ProductionReachabilityOwner::recover(...)` performs the first authoritative `load_current(peer)` before returning an owner. Later commit/reload operations continue through that same retained store.

This proves the non-orphan transfer law:

```text
once ReachabilityDurableSnapshotEtcdStore is constructed,
it must be moved directly into ProductionReachabilityOwnerCustody::recover(...).
```

No global registry, singleton, side channel, service locator, detached provider owner, or spare connection-retention object is required or selected.

## 5. Exact current-source evidence — existing durable executor and semantic store

Exact control-plane provider path:

```text
crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs
```

Exact blob:

```text
77fc9f345c17c5722c5240f3cead7ea68cb55cac
```

Existing narrowing constructor:

```text
ReachabilityDurableSnapshotEtcdExecutor::new(kv: KvClient)
```

The executor owns raw provider execution only.

Exact bridge semantic-store path:

```text
crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs
```

Exact blob:

```text
a381963986c79f8a314088839316d47595ba8686
```

Existing semantic constructor:

```text
ReachabilityDurableSnapshotEtcdStore::new(provider)
```

The bridge store already implements `ReachabilityDurableStore` and owns durable key/value binding plus fail-closed persistence classification.

No modification to either durable implementation is selected by C03e-HN.

## 6. Exact current-source evidence — production freshness source

Exact Agent path:

```text
crates/prw-agent/src/production_reachability_freshness_token_source.rs
```

Exact blob:

```text
907e6569b04190a70a09676078c1939154e742b4
```

The existing concrete verifier source is:

```text
ProductionReachabilityFreshnessTokenSource::new()
```

and already implements `CandidatePublicationFreshnessTokenSource` through the selected OS-backed cryptographic randomness provider.

C03e-HN therefore selects this existing type as `T` for future concrete production-owner composition. No new token generator, clock-derived token, UUID source, or fallback entropy path is selected.

## 7. Exact current-source evidence — current control-plane bootstrap remains two-role

Exact path:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

Exact blob:

```text
c454ebe01c9dfdfe6a9117fecffc983f0afe7600
```

At C03e-HM the bootstrap:

- validates one exact three-member HTTPS endpoint vector;
- validates one explicit trust bundle;
- accepts distinct live-owner and fence-allocator identities;
- establishes separate role connections;
- narrows each broad `Client` to its `KvClient`;
- drops the broad `Client` handles;
- returns `ReachabilityLiveOwnerAcquisitionPreparation`.

It does not yet accept the HK-selected dedicated durable-snapshot identity and does not construct a durable role connection.

C03e-HN preserves `prw-control-plane` as the owner of `Client::connect(...)` and role-scoped provider connection construction. `prw-agent` must not become an etcd connection/TLS/credential bootstrap owner merely because it is the cross-crate composition root.

## 8. Selected minimal provider handoff

C03e-HN resolves the C03e-HM handoff question as follows:

```text
The control-plane bootstrap must hand the composition root a
ReachabilityDurableSnapshotEtcdExecutor, not a broad etcd Client and not a raw KvClient.
```

The future dedicated durable role connection remains created inside the existing control-plane provider-bootstrap ownership domain. Immediately after the dedicated role connection yields its `KvClient`, control-plane must narrow it through:

```text
ReachabilityDurableSnapshotEtcdExecutor::new(durable_snapshot_kv)
```

Only that dedicated executor may cross the control-plane-to-Agent composition boundary for this slice.

C03e-HN does not authorize a new public method returning `etcd_client::Client` or `etcd_client::KvClient`. It does not rely on or expand any existing `into_inner` escape hatch as a composition mechanism.

## 9. Selected bootstrap result shape

The existing `ReachabilityLiveOwnerAcquisitionPreparation` remains a live-owner/fence provider preparation and must not be changed into the semantic durable store owner.

For the future three-role production bootstrap, control-plane may expose one narrow aggregate result whose semantic shape is exactly:

```text
(
    ReachabilityLiveOwnerAcquisitionPreparation,
    ReachabilityDurableSnapshotEtcdExecutor,
)
```

The concrete Rust carrier name is intentionally left to the source-materialization checkpoint; its shape and authority ceiling are selected here.

Required properties of that carrier:

- owns exactly the existing live-owner preparation plus one dedicated durable executor;
- contains no raw broad `Client`;
- contains no raw `KvClient` field exposed to Agent;
- contains no bridge semantic type;
- creates no reverse dependency;
- carries no endpoint, trust-bundle, certificate, or private-key accessor;
- has no generic arbitrary-etcd execution surface;
- is consumed or split by value at the Agent composition boundary.

The existing two-role preparation type remains unchanged in semantic ownership.

## 10. Additive compatibility law for current credential custody

Exact current custody path:

```text
crates/prw-reachability-custody/src/lib.rs
```

Exact blob:

```text
cc3dcb80344fc62af31db25a9d93469392d29103
```

At C03e-HM this crate reads exactly eight fixed reachability authority credentials and constructs the existing two-role `ReachabilityLiveOwnerEtcdBootstrapConfig`.

C03e-HN does not authorize adding durable certificate/private-key credential names, reading new secrets, modifying systemd custody, or changing service packaging.

Therefore a first source-materialization successor must preserve compile-time compatibility with the current two-role custody path. It must not require an immediate breaking constructor change that silently forces `prw-reachability-custody` or production systemd credentials into the same source checkpoint.

A later explicitly gated custody/security checkpoint must materialize the HK-selected dedicated durable identity into the systemd credential path before any production three-role bootstrap callsite can be activated.

This sequencing does not authorize a credential fallback. A production durable connection remains impossible until its dedicated credentials are explicitly materialized.

## 11. Selected Agent-side concrete assembly

After control-plane has produced the narrow dedicated durable executor, the Agent-owned composition path is selected as exactly:

```text
ReachabilityDurableSnapshotEtcdExecutor
    -> ReachabilityDurableSnapshotEtcdStore::new(executor)
        -> ProductionReachabilityFreshnessTokenSource::new()
            -> ProductionReachabilityOwnerCustody::recover(store, token_source, peer)
```

The store and token source move by value into the existing custody recovery operation.

The future composition helper must not:

- call `Client::connect`;
- receive endpoint strings;
- receive certificate/private-key material;
- receive a raw `KvClient`;
- clone a provider role context;
- create a fallback in-memory store;
- construct a second owner outside `ProductionReachabilityOwnerCustody`;
- spawn a task or runtime;
- publish readiness;
- execute candidate publication;
- provision traversal;
- dial a peer;
- install a listener;
- mutate startup/shutdown behavior.

## 12. Selected future Agent source owner

The first future Rust composition materialization, when separately authorized, should be a narrow sibling module under the existing Agent production-owner boundary:

```text
crates/prw-agent/src/production_reachability_owner_composition.rs
```

That module may own only the executor-to-store-to-token-source-to-custody handoff selected in Section 11.

It must remain crate-internal unless a later exact-source requirement proves a public API is needed.

The existing:

```text
crates/prw-agent/src/production_reachability_owner_custody.rs
crates/prw-agent/src/production_reachability_freshness_token_source.rs
```

remain authoritative and should be reused without redesign.

Adding the crate-internal module declaration in:

```text
crates/prw-agent/src/lib.rs
```

would be part of that separately authorized source checkpoint only.

## 13. Failure propagation law

The future composition must remain fail-closed in two distinct stages.

### Provider bootstrap failure

If any required role-scoped connection for the future three-role bootstrap cannot be established, no successful aggregate provider result may be returned.

A dedicated durable connection failure must receive its own bounded bootstrap classification rather than being normalized to live-owner or fence success.

No raw partial durable handle may escape on failure.

### Durable owner recovery failure

After a durable executor has been handed to Agent and wrapped in `ReachabilityDurableSnapshotEtcdStore`, `ProductionReachabilityOwnerCustody::recover(...)` remains the authoritative recovery boundary.

Missing, ambiguous, mismatched, invalid, recovery-required, or retired durable state must retain the existing `ReachabilityOwnerError` semantics.

No source successor may convert recovery failure into:

- an empty/new owner;
- a memory-only baseline;
- a default candidate plan;
- a retry-created authority record;
- runtime readiness;
- partial candidate capability.

Provider bootstrap and durable recovery errors remain distinct causes even if a later Agent facade maps them into one bounded startup/composition error enum.

## 14. Runtime activation remains separately gated

C03e-HN selects ownership and handoff only.

It does not select a callsite in:

```text
crates/prw-agent/src/main.rs
crates/prw-agent/src/linux_bootstrap.rs
```

It does not wire durable recovery into the running Agent, listener readiness, candidate publication rendezvous, remote-session lifecycle, traversal, or process startup.

A later runtime-activation checkpoint must prove ordering, lifetime, failure propagation, shutdown behavior, and readiness semantics from the then-current exact source before any composition helper is called by a production process path.

## 15. Future source-materialization ceiling

C03e-HN itself authorizes **zero** Rust paths.

For a later separately authorized source-materialization checkpoint, the maximum initially selected ceiling is:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
crates/prw-agent/src/production_reachability_owner_composition.rs
crates/prw-agent/src/lib.rs
```

The control-plane path may materialize only the additive dedicated durable role handoff needed to return the existing executor without breaking the current two-role custody callsite.

The Agent paths may materialize only the selected concrete assembly helper and its crate-internal declaration.

The following existing durable semantic paths are expected to remain unchanged:

```text
crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs
crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs
crates/prw-remote-bridge/src/reachability_owner.rs
crates/prw-agent/src/production_reachability_owner_custody.rs
crates/prw-agent/src/production_reachability_freshness_token_source.rs
```

No Cargo manifest or lockfile change is selected.

If exact implementation proves an additional path is mechanically required, source work must stop and select that path in a separate corrective/extension checkpoint rather than silently broadening the ceiling.

Credential custody, service packaging, deployment configuration, RBAC provisioning, certificate materialization, and production runtime callsites are outside this source ceiling.

## 16. Focused validation selected for the later source checkpoint

A future source-materialization checkpoint must include focused evidence for at least:

1. the durable role is represented by a dedicated provider connection/input rather than live-owner/fence client reuse;
2. the control-plane-to-Agent handoff type carries `ReachabilityDurableSnapshotEtcdExecutor`, not raw `Client`/`KvClient`;
3. the bridge store is constructed exactly once from that executor;
4. the store moves directly into `ProductionReachabilityOwnerCustody::recover(...)`;
5. the existing `ProductionReachabilityFreshnessTokenSource` is used as the concrete token source;
6. no fallback owner/store is constructed after bootstrap or recovery failure;
7. current two-role custody source still compiles unchanged during the first additive source checkpoint;
8. crate dependency direction remains unchanged;
9. durable semantic executor/store/owner files remain byte-for-byte unchanged unless a separately selected defect requires correction;
10. the new Agent composition helper owns no runtime/task/listener/readiness behavior;
11. formatting succeeds;
12. Clippy with warnings denied succeeds;
13. workspace tests succeed;
14. workspace build succeeds;
15. any environment/tooling failure is reported separately from a source defect.

Disposable-provider integration evidence may be added only where the existing test infrastructure can exercise the new role boundary without provisioning or mutating production security state.

## 17. HK security/topology selections remain frozen

C03e-HN does not reopen C03e-HK.

Future durable provider composition remains constrained by:

- the same logical three-member reachability-authority etcd cluster;
- the same validated runtime-supplied three-HTTPS-endpoint vector;
- the same runtime-supplied trust authority;
- the selected pinned reachability etcd TLS server identity;
- dedicated principal `prw-reachability-durable-snapshot`;
- dedicated role `prw-reachability-durable-snapshot-rw` restricted to `/prw/reachability/durable-snapshot/`;
- dedicated durable certificate/private-key identity;
- role-isolated connection/client context;
- no credential fallback to live-owner or fence-allocation identities.

HN does not provision or validate those production credentials because no credential material is touched here.

## 18. HF/HG/HM durable semantics remain frozen

C03e-HN changes no persistence semantics, including:

- canonical exact peer-derived durable key;
- canonical durable value representation;
- default-linearizable exact-key reads;
- requested-peer/key/value binding;
- exact expected-current compare-and-commit behavior;
- fail-closed same-token ambiguity;
- no create-if-absent recovery shortcut;
- no prefix scan;
- no arbitrary range application API;
- no Watch;
- no lease/TTL ownership;
- no blind retry after an indeterminate mutation result;
- no implicit repair;
- no in-memory authority fallback.

## 19. Explicitly forbidden shortcuts

C03e-HN forbids a future successor from:

- moving durable semantic ownership into `prw-control-plane`;
- adding `prw-control-plane -> prw-remote-bridge`;
- creating another durable executor or store;
- returning raw `Client` or raw `KvClient` from a new production bootstrap facade;
- cloning a live-owner/fence client for durable use;
- constructing the bridge store inside control-plane;
- making `ReachabilityLiveOwnerAcquisitionPreparation` own bridge durable semantics;
- bypassing `ProductionReachabilityOwnerCustody::recover`;
- creating a global provider handle or singleton;
- retaining an unused provider connection merely to keep it alive;
- using current two-role systemd credentials as a durable credential fallback;
- changing credential files or service units as part of the first composition source step;
- activating the composition from `main.rs` or Linux runtime without a later gate;
- broadening into refactoring, dependency upgrades, deployment, or unrelated cleanup.

## 20. C03e-HN docs-only materialization ceiling

This checkpoint changes exactly one repository path:

```text
contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HN_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_COMPOSITION_OWNER_SELECTION_STAGING.md
```

Allowed delta:

```text
1 Markdown contract
0 Rust source paths
0 Cargo manifests
0 Cargo lockfiles
0 workflow files
0 Agent runtime paths
0 credential files
0 service/deployment paths
0 auth/RBAC paths
0 certificate/PKI paths
0 database/schema paths
0 repository visibility paths
```

Any additional changed path blocks closure.

## 21. Acceptance gate

C03e-HN may close only when exact-head evidence proves:

1. direct ancestry from C03e-HM head `03cce394d1e0ed06ca03175cc56f147a58a59261`;
2. exactly one ordinary commit ahead of C03e-HM and zero behind;
3. exactly one changed Markdown contract path;
4. no Rust/manifests/lockfiles/workflows/runtime/deployment/security/visibility changes;
5. `prw-agent` is selected as the cross-crate composition owner from exact current dependencies and composition precedent;
6. `ProductionReachabilityOwnerCustody::recover(...)` is selected as the existing production-owner construction destination;
7. the dedicated durable provider handoff is the existing `ReachabilityDurableSnapshotEtcdExecutor`, not raw `Client`/`KvClient`;
8. `ReachabilityDurableSnapshotEtcdStore::new(executor)` remains bridge-owned;
9. `ProductionReachabilityFreshnessTokenSource` remains the selected production token source;
10. the store is selected to move directly into custody recovery with no orphan/global holder;
11. current two-role credential custody remains unchanged and production durable credentials remain separately gated;
12. runtime activation remains separately gated;
13. the future source-path ceiling is explicit and fail-closed;
14. exact-head CI/check status is reported accurately, including legitimate path-filter skips;
15. no merge, deployment, credential/RBAC/certificate mutation, runtime activation, repository-visibility mutation, or destructive cleanup occurs.

Canonical closure target:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_COMPOSITION_OWNER_SELECTION
```

## 22. Stop conditions

Stop and require a separate checkpoint before any action that would:

- modify Rust source;
- create/connect the durable etcd client;
- alter `ReachabilityLiveOwnerEtcdBootstrapConfig` source;
- change systemd credential custody;
- add durable certificate/private-key credential names;
- provision etcd users/roles/permissions;
- issue/install/rotate certificates;
- modify endpoints, trust roots, DNS, firewall, or cluster membership;
- construct/recover the production owner at runtime;
- change Agent startup/readiness/shutdown behavior;
- activate candidate publication or traversal;
- deploy/restart a service;
- merge a staging branch;
- change repository visibility;
- rewrite or delete historical evidence.

---

C03e-HN selects only the exact composition owner and minimal dependency-preserving handoff required by C03e-HM. The selected next source direction is additive and narrow: control-plane may eventually produce the existing dedicated durable executor, Agent may wrap it in the existing bridge store and move it into the existing production-owner custody recovery seam, and all credential/runtime activation remains separately gated.
