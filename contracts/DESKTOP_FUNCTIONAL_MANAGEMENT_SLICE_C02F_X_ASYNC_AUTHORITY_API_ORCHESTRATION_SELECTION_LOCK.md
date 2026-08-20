# Phase 152 C02f-X — Async Authority API / Orchestration Selection Lock

Status: `SELECTED / SEPARATE_ASYNC_PRODUCTION_AUTHORITY_PORT / IMPL_FUTURE_PLUS_SEND / STATIC_DISPATCH / MUTABLE_PROVIDER_RECEIVER / CONTROL_PLANE_PROVIDER_OWNER / REMOTE_BRIDGE_ORCHESTRATION_OWNER / NO_ASYNC_TRAIT / NO_BOXED_FUTURE_REQUIREMENT / NO_AGENT_TOKIO_EXPANSION / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
Predecessor C02f-W head: `c8c705d2140b69042a91ec385257754d669b16ee`
Predecessor C02f-W tree: `b9f773fe511ef3edec48cb7da937bb1dd4057200`

## Approval basis

The user explicitly approved the architecture recommendation after C02f-W compiler validation.

This lock converts the previously preferred-but-unselected C02f-T/U/V directions into selected architecture for the narrow async-authority integration boundary only.

It does not select storage schema, etcd compare/CAS semantics, TLS/auth/RBAC, cluster topology, recovery epoch/provider, production endpoints, executor bootstrap ownership, or R1-R4 network activation.

## Selected architecture

### 1. Keep the deterministic synchronous semantic seam

The existing synchronous `ReachabilityLiveOwnerAuthority` and the live-owner semantic types remain valid deterministic/Sans-I/O reference machinery.

Production etcd network I/O must not be hidden behind those synchronous methods through `block_on`, nested runtimes, helper threads, local watch caches, or other opaque execution mechanisms.

A separate explicit asynchronous production authority port is selected.

### 2. Async port representation

The selected public production port representation is native Rust return-position opaque Future syntax with an explicit Send guarantee:

```rust
fn operation(&mut self, ...)
    -> impl Future<Output = Result<...>> + Send;
```

Equivalent explicit Future-returning methods will be used for the selected production authority operations.

This selection is based on C02f-W compiler evidence on Rust 1.97.1 with `etcd-client = 0.19.0`.

### 3. Static dispatch

The initial production authority path uses generic/static dispatch.

No current requirement for `dyn` authority dispatch has been proven. Therefore the initial design does not pay for dynamic-dispatch support speculatively.

If a future architecture proves a real runtime need for heterogeneous authority implementations behind a trait object, that later gate may reconsider boxed futures or another dyn-compatible boundary.

### 4. Receiver model

The initial provider/authority operation receiver is selected as:

```rust
&mut self
```

This keeps mutation ownership explicit and avoids introducing `Arc<Mutex<_>>`, channels, detached workers, or hidden internal concurrency merely to make the first adapter concurrent.

This selection does not claim that all namespaces must be globally serialized forever. A later measured concurrency requirement may introduce a more granular provider handle or synchronization model without changing the authority safety invariants.

### 5. No `async-trait` and no mandatory boxing

The initial production path must not add `async-trait` as a direct dependency merely for this port.

The selected Future contract does not require `Pin<Box<dyn Future<...>>>` or another mandatory heap-allocated Future form.

The existing transitive presence of `async-trait` in third-party dependency graphs is not an authorization to use it in PRW source.

### 6. Provider ownership

Provider-specific etcd authority implementation remains owned by `prw-control-plane`.

This includes, once separately selected and authorized:

- etcd client operation mapping;
- Get/Txn/CAS behavior;
- indeterminate mutation reconciliation;
- provider key/value codec use;
- provider error classification;
- TLS/auth/RBAC client configuration.

`prw-control-plane` must not depend upward on `prw-remote-bridge` merely to implement the provider.

### 7. Orchestration ownership

`prw-remote-bridge` is selected as the asynchronous reachability/live-owner orchestration boundary.

Its role is to coordinate when authority acquisition/currentness/release/reconciliation is required and to map only definitive provider outcomes into the existing live-owner semantic grant/fence model.

This selection does not make `prw-remote-bridge` the owner of a Tokio runtime or process-level executor.

### 8. Transport boundary

`prw-remote-transport` remains responsible for QUIC/TLS transport mechanics.

It must not become the etcd/control-plane authority owner merely because it already has Tokio/Quinn dependencies.

Future R1-R4 effect fencing may require transport/NAT/runtime effect sinks to consume a fence, but that is separate from owning authority allocation.

### 9. Agent boundary

`prw-agent` must not gain ad hoc direct Tokio, Quinn, or etcd responsibilities solely for live-owner authority integration.

Process-level executor/bootstrap ownership remains a separate later selection.

### 10. Dependency direction

The selected layering direction is:

```text
executor/bootstrap (deferred)
        |
        v
prw-remote-bridge orchestration
        |
        v
prw-control-plane provider adapter
        |
        v
etcd-client 0.19.0 -> etcd v3.7
```

A normal `prw-remote-bridge -> prw-control-plane` dependency is permitted when required by the approved source tranche.

The inverse `prw-control-plane -> prw-remote-bridge` dependency is not selected.

The exact type boundary for `DeviceId + TransportIdentity` entering the control-plane provider remains deferred: this lock does not choose a new `prw-control-plane -> prw-connectivity` dependency versus primitive/provider-neutral inputs.

## Inherited safety invariants

This lock preserves all prior C02f safety requirements:

- exact live-owner namespace is `DeviceId + TransportIdentity`;
- `ReachabilityLiveOwnerFence(NonZeroU128)` remains the logical ordered fencing generation;
- ambiguity/unavailability fails closed;
- stale/member-local serializable reads cannot prove currentness;
- Watch remains advisory only;
- stale release cannot clear newer authority;
- clocks/TTL/heartbeat are liveness aids, not primary safety authority;
- recovery cannot reuse or roll back fencing generations;
- R1-R4 must reject stale authority at or atomically with the actual effect boundary;
- a one-time bridge-level currentness check is not sufficient effect fencing.

## C02f-W evidence inherited by this selection

Disposable PR #42 validated the selected API family without merging probe code into production.

The successful targeted workflow demonstrated:

- Rust 1.97.1 accepts the explicit `impl Future + Send` form;
- `etcd-client 0.19.0` `Client`, `get`, and `txn` are compatible with the selected Send Future boundary;
- a statically dispatched authority trait with borrowed `&mut self`, peer and grant inputs compiles under the current workspace;
- Clippy passes with `-D warnings` after formatting/lint-only corrections;
- both targeted tests pass;
- the full PRW Rust Validation workflow also passes on the disposable probe branch.

The probe PR was closed unmerged.

## First authorized source tranche after this lock

The next source tranche may now materialize only the selected async integration boundary:

1. preserve the synchronous semantic/reference seam;
2. add a separate production async authority port using `fn -> impl Future + Send` and `&mut self`;
3. use static dispatch;
4. add deterministic runtime-independent reference tests for that port;
5. if needed for the selected bridge/provider layering, promote `prw-control-plane` from dev-only to a normal `prw-remote-bridge` dependency;
6. do not add `async-trait`;
7. do not add Tokio to Agent/bridge/control-plane merely to define or test the port;
8. do not construct or contact etcd endpoints;
9. do not select or encode storage keys/values/CAS guards;
10. do not activate R1-R4 effects.

## Still deferred / requires separate selection

The following remain explicitly unselected by C02f-X:

- exact production async method/result surface beyond the minimum authority semantics needed for the first staging tranche;
- exact provider identity input representation across the bridge/control-plane boundary;
- key prefix/version/framing/DeviceId length policy;
- authority record value schema;
- stable owner/attempt identifier representation;
- exact etcd Txn compare guard (`value`, `mod_revision`, or another proven pattern);
- indeterminate Txn reconciliation details;
- TLS feature selection and crypto-provider divergence acceptance;
- CA/certificate identities/credentials/RBAC/endpoints;
- three-voter versus five-voter topology and deployment platform;
- recovery epoch representation and external immutable ledger provider;
- process-level executor/bootstrap ownership;
- R1-R4 sink-side implementation;
- runtime/network/deployment activation.

## Selection conclusion

PRW selects a separate honest asynchronous production live-owner authority port using native `impl Future + Send`, static dispatch and an initial `&mut self` receiver. Provider-specific etcd implementation remains in `prw-control-plane`; async live-owner/reachability orchestration belongs in `prw-remote-bridge`; remote transport remains transport-only; Agent does not gain ad hoc Tokio/etcd ownership.

This lock authorizes the narrow source staging tranche above and nothing beyond it. C02d remains frozen and untouched.
