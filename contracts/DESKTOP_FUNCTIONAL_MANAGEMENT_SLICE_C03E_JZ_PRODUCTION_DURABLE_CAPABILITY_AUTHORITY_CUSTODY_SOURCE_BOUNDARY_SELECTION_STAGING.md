# Desktop Functional Management Slice — C03e-JZ Production Durable Capability-Authority Custody Source Boundary Selection

Status: `SELECTION_STAGING`
Date: `2026-09-04`

## 1. Checkpoint classification

C03e-JZ is a documentation-only source-boundary selection checkpoint.

Target gate:

`C03E_JZ_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_CUSTODY_SOURCE_BOUNDARY_SELECTED`

Target closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_CUSTODY_SOURCE_BOUNDARY_SELECTION`

JZ performs the fresh exact-head audit required after closed C03e-JY and selects only the smallest dormant Agent-side custody materialization for the already-reviewed production durable capability-authority owner shape.

JZ does not materialize Rust source. It does not invoke `DurableCapabilityBridge`, replace `LinuxAgentRemoteProcessOperationInputs`, create an expected-request producer, invoke a dispatcher, alter `run()` or `main.rs`, activate listener/runtime/network behavior, deploy, or merge.

## 2. Exact predecessor authority

Closed predecessor:

`C03e-JY — Production Remote Capability Deny-All Policy Source Materialization`

Predecessor branch:

`phase-152-c03e-jy-production-remote-capability-deny-all-policy-source-materialization`

Exact predecessor head / required merge base:

`3f5e993f1a87a8bffe268b88da4149ab3c057a35`

Exact predecessor tree:

`b2a485ed8dd2cccdb60c94306444443293821600`

Exact predecessor policy source blob:

`3056b53e81c4429314d9f890dcf2bf3e80d433b8`

JY gate:

`C03E_JY_PRODUCTION_REMOTE_CAPABILITY_DENY_ALL_POLICY_SOURCE_MATERIALIZED`

JY closure:

`CLOSED_PRODUCTION_REMOTE_CAPABILITY_DENY_ALL_POLICY_SOURCE_MATERIALIZATION`

JY is closed with exact-final-head Rust and Android validation plus immutable Drive evidence and leaves Agent durable capability-authority custody/composition unresolved.

## 3. Fresh namespace and predecessor guards

Before JZ mutation:

- JY branch re-read remained exact head `3f5e993f1a87a8bffe268b88da4149ab3c057a35`;
- JY tree remained `b2a485ed8dd2cccdb60c94306444443293821600`;
- PR #410 remained draft/open/unmerged on exact JY head;
- branch search for `phase-152-c03e-jz` returned zero results;
- PR search for `C03e-JZ` returned zero results;
- the intended JZ contract path did not exist at the exact JY head.

No existing successor branch, PR or contract artifact is reused or overwritten.

## 4. Exact current Agent durable-registry custody

Exact source:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact blob:

`fd78512a24b824483a101962c3c63d91ad4b2cc1`

The existing owner is:

`ProductionDurableRegistryRuntimeCustody`

and retains exactly one `DurableRegistryEtcdStore` privately.

Its current public/crate-visible surface exposes only:

- `from_store(...)` — one side-effect-free ownership adaptation;
- `peer_connectivity_identity(...)` — one operation-specific current peer-identity lookup.

It intentionally exposes no:

- `store()`;
- `store_mut()`;
- `into_store()`;
- raw executor getter;
- raw etcd client;
- endpoint/credential extraction surface.

JZ preserves this private-store discipline.

## 5. Exact current production policy prerequisite

Exact source:

`crates/prw-policy/src/lib.rs`

Exact blob:

`3056b53e81c4429314d9f890dcf2bf3e80d433b8`

Closed JY materialized:

`ProductionRemoteCapabilityDenyAllPolicy`

as a fieldless `PolicyEvaluator` that returns exactly `Decision::Deny` for every represented capability.

It has no positive-grant constructor, mutable grant state or external source.

JZ does not alter policy semantics.

## 6. Exact current durable bridge prerequisite

Exact source:

`crates/prw-remote-bridge/src/lib.rs`

Exact blob:

`ad6833cc4e71a372810b260f157126a3df6645e5`

Closed JW materialized:

`DurableCapabilityBridge<'a, P: PolicyEvaluator + Sync>`

which borrows:

- `&'a mut DurableRegistryEtcdStore`;
- `&'a P`.

Its async `authorize(...)` preserves the reviewed request-kind -> lease -> same-pair durable session/transport -> PRWC decode -> capability -> policy -> private authorized-request order.

JZ does not invoke or modify this bridge.

## 7. Existing JT custody shape remains authoritative

Closed C03e-JT already selected the conceptual Agent owner:

```text
ProductionDurableCapabilityAuthority {
    registry_custody: Arc<tokio::sync::Mutex<ProductionDurableRegistryRuntimeCustody>>,
    policy: ProductionRemoteCapabilityDenyAllPolicy,
}
```

The Tokio mutex exists only because the privately retained durable semantic store requires mutable operation custody.

The selected shared owner must not clone durable registry state into snapshots.

No mutex guard may later be held across dispatcher execution, response I/O, worker-cancellation waits or unrelated runtime lifecycle work.

JZ narrows only where and how this already-selected dormant owner may first be materialized.

## 8. Exact current Agent dependency topology

Exact manifest:

`crates/prw-agent/Cargo.toml`

Exact blob:

`4c70d6be9b56f39edc10810eefa3428314ed7559`

The Agent already directly depends on:

- `prw-policy`;
- `prw-registry`;
- `prw-remote-bridge`;
- Tokio with the `sync` feature.

Therefore the selected dormant owner requires no manifest or lockfile change.

The Agent does **not** directly depend on `prw-remote-transport` at this checkpoint. JZ deliberately does not select a bridge-invocation method that would expose `ControlFrame` at the Agent boundary.

## 9. Exact current module topology

Exact Agent crate root:

`crates/prw-agent/src/lib.rs`

Exact blob:

`8b50cb5c5c2e711648cba8424ed2015be5606360`

The existing module:

`production_durable_registry_runtime_custody`

is already mounted crate-internally.

Therefore the smallest dormant owner materialization can remain in the already-mounted file:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

and requires no `lib.rs` mutation or new module declaration.

## 10. Existing remote-process aggregate remains incompatible and out of scope

Exact source:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact blob:

`f2a87c45bd8d96bf1555b65210531c94c722eb2f`

`LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` still stores:

`SharedCurrentCapabilityAuthority<P>`

and existing production input-population helpers still accept that in-memory authority type.

JZ does not replace, adapt, reinterpret or remove that field.

Any durable-authority aggregate replacement requires a later separately gated interface checkpoint after the dormant owner exists.

## 11. Selected immediate source owner

After JZ closure, the immediate source-materialization successor may change exactly one repository path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

No new file or module declaration is selected.

## 12. Selected exact dormant authority type

The immediate successor may add one crate-internal type named exactly:

`ProductionDurableCapabilityAuthority`

with custody equivalent to:

```text
pub(crate) struct ProductionDurableCapabilityAuthority {
    registry_custody: Arc<tokio::sync::Mutex<ProductionDurableRegistryRuntimeCustody>>,
    policy: ProductionRemoteCapabilityDenyAllPolicy,
}
```

The fields remain private.

The type must own no:

- raw `DurableRegistryEtcdStore` separate from existing custody;
- raw etcd/provider client;
- endpoint or credential object;
- `ControlFrame`;
- dispatcher;
- session receiver;
- listener/socket;
- executable/runtime handle.

## 13. Selected constructor law

The immediate successor may add exactly one side-effect-free constructor equivalent to:

```text
ProductionDurableCapabilityAuthority::from_registry_custody(
    registry_custody: ProductionDurableRegistryRuntimeCustody,
) -> ProductionDurableCapabilityAuthority
```

The constructor must:

1. consume the exact supplied `ProductionDurableRegistryRuntimeCustody` by value;
2. wrap that exact value in `tokio::sync::Mutex`;
3. wrap the mutex in one `Arc`;
4. construct the fieldless `ProductionRemoteCapabilityDenyAllPolicy` internally;
5. return the dormant owner without awaiting or performing I/O.

No caller-selected policy evaluator is accepted by this initial production constructor.

## 14. Clone/share rule

The immediate source successor may make `ProductionDurableCapabilityAuthority` cloneable only by cloning the outer `Arc` and copying the fieldless deny-all policy.

If `Clone` is materialized:

- both values must share the same underlying `ProductionDurableRegistryRuntimeCustody` mutex;
- no registry/store snapshot may be cloned;
- no provider connection is duplicated by constructor logic;
- cloning performs no I/O.

Clone support is permitted but not required for the first dormant materialization.

## 15. No authorization method in the immediate successor

The immediate source successor must **not** add an authorization/invocation method yet.

It must not accept:

- `TransportIdentity`;
- `RemoteSessionLease`;
- `ControlFrame`;
- request ID;
- decoded `BridgeCommand`;
- requested `Capability`;
- dispatcher;
- callback;
- worker handle.

It must not construct or call `DurableCapabilityBridge`.

This keeps the first owner materialization purely custodial and side-effect-free.

## 16. Why bridge invocation is separately gated

`DurableCapabilityBridge::authorize(...)` currently accepts `&ControlFrame`, while the Agent manifest does not directly depend on `prw-remote-transport`.

JZ does not silently widen the manifest or invent a re-export/wrapper merely to force bridge invocation into the same checkpoint.

A later fresh checkpoint must select the exact operation-specific Agent invocation seam, including whether:

- `prw-agent` gains a direct `prw-remote-transport` dependency;
- an already-reviewed bridge-owned wrapper/re-export is preferable;
- the operation-specific method lives on `ProductionDurableRegistryRuntimeCustody` or the higher authority owner;
- exact lock scope and error surface.

Mentioning these alternatives is not authorization to implement them.

## 17. Existing durable store privacy remains mandatory

The immediate source successor must not add:

- `store()`;
- `store_mut()`;
- `into_store()`;
- raw provider getters;
- generic closure access to the inner store;
- arbitrary transaction callbacks.

Later bridge invocation must remain operation-specific.

## 18. Policy remains fail-closed and concrete

The selected owner stores exactly:

`ProductionRemoteCapabilityDenyAllPolicy`

not a generic `P` and not either local policy type.

This initial owner therefore cannot carry a policy that returns `Decision::Allow`.

Any future allow-bearing production policy owner requires a new explicit provenance/interface checkpoint.

## 19. Immediate-successor test ceiling

The one-file source successor may add only focused same-file tests for:

1. constructor type/signature shape;
2. `Send + Sync` shape if useful;
3. optional clone outer-Arc sharing if `Clone` is materialized;
4. compile-time confirmation that ownership adaptation is synchronous/non-async.

Tests must perform no:

- provider/network I/O;
- environment mutation;
- filesystem mutation;
- credential read;
- lock contention loop;
- runtime/listener activation.

## 20. Exact immediate path ceiling

The immediate materialization successor may change only:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

It may add only:

1. `Arc` / Tokio `Mutex` imports;
2. `ProductionRemoteCapabilityDenyAllPolicy` import;
3. `ProductionDurableCapabilityAuthority`;
4. selected side-effect-free constructor;
5. optional `Clone` consistent with the outer-Arc rule;
6. focused same-file tests;
7. strictly local rustfmt/lint corrections required by this source shape.

## 21. Explicit immediate-successor exclusions

The immediate source successor must not mutate:

- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/Cargo.toml`;
- `Cargo.lock`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-remote-bridge/*`;
- `crates/prw-policy/*`;
- `crates/prw-registry/*`;
- any workflow;
- Android/application code;
- packaging/service/systemd files;
- `run()` or `main.rs`.

It must not materialize authorization calls, aggregate replacement, expected-request production, dispatcher custody, executable assembly or runtime/network activation.

## 22. Identity and authorization invariants

JZ preserves:

`PRW logical device/session identity -> registry/discovery -> current reachable endpoint/candidates -> authenticated transport`

Specifically:

- `DeviceId` remains logical device identity;
- `TransportIdentity` remains current transport evidence;
- transport-key rotation does not redefine logical device identity;
- IP/port remains transient reachability data;
- `SessionId` remains session correlation/lifetime context;
- PRWM request ID remains transaction correlation only;
- custody construction is not authentication;
- mutex/Arc ownership is not authorization;
- deny-all policy possession is not positive authorization;
- no `AuthorizedCapabilityRequest` is constructed by the selected immediate successor.

No PID/UID/GID or host account identity becomes PRW logical identity.

## 23. Later lock-scope law

When a later separately gated operation-specific authorization method is selected, the registry mutex may cover only the bounded durable authority transaction necessary to obtain an authorization result.

The lock must be released before:

- dispatcher execution;
- response serialization/I/O;
- terminal/file/forwarding operation execution;
- worker completion waits;
- cancellation waits;
- unrelated runtime lifecycle work.

JZ records this law but does not materialize the method.

## 24. Explicit JZ exclusions

C03e-JZ does not perform or authorize:

- Rust/source materialization in JZ;
- generic inner-store exposure;
- durable bridge invocation;
- `ControlFrame` dependency widening;
- allow-bearing production policy;
- registry mutation or snapshot/mirror authority;
- session-authentication production population;
- expected-request producer population;
- dispatcher/provider production assembly;
- timing/callback production sourcing;
- requester/rendezvous custody population or invocation;
- aggregate input replacement;
- operation-factory invocation;
- remote-process companion spawn;
- `run()` or `main.rs` mutation;
- listener/bind/readiness/runtime/network activation;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment/restart/recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 25. Closure and successor rule

After JZ closure: **STOP**.

The immediate successor may only materialize the dormant `ProductionDurableCapabilityAuthority` custody owner in:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

After that source materialization closes, another fresh exact-head audit is mandatory before selecting any operation-specific durable authorization invocation seam or aggregate-interface replacement.

## 26. Validation and immutable evidence rule

JZ closure requires:

1. exact-final-head CI bound only to the final JZ commit;
2. skipped workflows represented only as skips;
3. immutable Markdown audit upload to the canonical Drive parent;
4. raw Drive readback with exact byte-count and SHA-256 equality;
5. exact-title post-upload search returning exactly one canonical artifact;
6. post-publication branch/compare/PR re-read showing no head drift;
7. PR remaining draft/open/unmerged.

Canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

After closure, no source materialization is inherited beyond the exact one-file dormant-owner ceiling above.
