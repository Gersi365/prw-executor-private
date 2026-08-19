# Phase 152 C02f-T — Async etcd Adapter Placement / Integration Readiness Audit

Status: `ASYNC_ETCD_ADAPTER_PLACEMENT_READINESS_COMPLETE / SYNC_DOMAIN_SEAM_VS_ASYNC_PROVIDER_IO_MISMATCH_ISOLATED / NO_BLOCK_ON_OR_HIDDEN_RUNTIME_SELECTED / CONTROL_PLANE_OWNS_ETCD_DEPENDENCY / REMOTE_BRIDGE_OWNS_PROVIDER_NEUTRAL_LIVE_OWNER_SEMANTICS / PRODUCTION_ASYNC_ORCHESTRATION_BOUNDARY_REQUIRES_EXPLICIT_SELECTION / NO_SCHEMA_SELECTION / NO_TLS_SELECTION / NO_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-S predecessor head: `30085eb73c0d60341f8f641340e998d90426b61f`
C02f-S predecessor tree: `15d66a673d87d2c1ec3b4964b27dbcfb09e833b3`
Review date: `2026-08-19`

## Purpose

C02f-S consolidated the remaining architecture selections and gave a safe implementation order beginning with pure codecs/state helpers and then an etcd Txn adapter.

Before production adapter code is added, C02f-T checks whether the current crate/API shape can host asynchronous etcd I/O without violating existing PRW boundaries or hiding blocking/runtime behavior behind a synchronous authority interface.

This checkpoint does not select schema bytes, CAS compare target, owner/attempt token representation, TLS features, runtime framework, endpoint configuration, cluster topology, recovery provider or production activation.

## Source facts

### Current live-owner seam is synchronous

`crates/prw-remote-bridge/src/reachability_live_owner.rs` currently defines:

- `ReachabilityLiveOwnerAuthority::acquire(&mut self, ...)`;
- `ReachabilityLiveOwnerAuthority::currentness(&mut self, ...)`;
- `ReachabilityLiveOwnerAuthority::release(&mut self, ...)`.

All three methods return ordinary `Result` values and expose no Future/async boundary.

The module explicitly states that it performs no socket, task, traversal, persistence or deployment operation and that real runtime side effects require separate fencing.

### Current remote-bridge ownership model is intentionally runtime-neutral

`crates/prw-remote-bridge/src/reachability_owner.rs` also defines synchronous provider-neutral seams such as `ReachabilityDurableStore`, while explicitly owning no socket or async runtime.

`crates/prw-nat-traversal` is intentionally Sans-I/O and owns protocol state rather than the UDP runtime.

Therefore the present synchronous seams are coherent for deterministic semantic/state-machine testing, but they are not by themselves a safe place to conceal network I/O.

### etcd client placement is already materialized in control-plane

`crates/prw-control-plane/Cargo.toml` currently contains:

`etcd-client = { version = "=0.19.0", default-features = false }`

and only `prw-core` as a PRW runtime dependency.

`crates/prw-control-plane/src` currently contains enrollment/session-auth modules and no live-owner etcd implementation module.

### Current dependency direction

`prw-remote-bridge` depends normally on connectivity/file/forwarding/NAT/session/transport/etc. and currently carries `prw-control-plane` only as a dev-dependency.

`prw-agent` depends on `prw-remote-bridge` but does not currently declare `prw-control-plane` or an async runtime dependency in its manifest.

Consequently no production crate currently owns the complete chain:

`runtime executor -> async etcd client -> live-owner semantic grant -> effect-boundary fencing`.

## Provider I/O reality

The selected `etcd-client 0.19.0` API is asynchronous and backed by Tokio/Tonic. Connection, KV Get/Put and Txn operations are asynchronous operations.

Therefore a production etcd live-owner adapter cannot be implemented honestly as network I/O directly inside the existing synchronous `ReachabilityLiveOwnerAuthority` methods without choosing an additional execution strategy.

## Unsafe shortcuts rejected for selection review

### Hidden per-call runtime / `block_on`

Classification: `NOT_RECOMMENDED`.

Do not create a Tokio runtime inside `acquire/currentness/release` or call an ambient runtime through an opaque `block_on` merely to preserve the current synchronous signature.

Risks include:

- runtime nesting failures;
- blocking an executor worker;
- unclear cancellation semantics;
- timeout ambiguity hidden behind a synchronous API;
- accidental coupling of domain semantics to one executor lifecycle.

### Per-operation helper thread

Classification: `NOT_RECOMMENDED`.

Spawning a thread per authority operation only moves the blocking boundary and complicates cancellation/shutdown without improving authority semantics.

### Treating cached/watch state as synchronous currentness proof

Classification: `REJECTED_BY_INHERITED_LOCKS`.

A local cache or Watch event cannot replace authoritative linearizable currentness. C02f-J already locks Watch as advisory only.

## Viable integration families

### T1 — make the live-owner authority production seam asynchronous

Classification: `ELIGIBLE / REQUIRES_EXPLICIT_API_SELECTION`.

Conceptually change or complement the current trait so production acquisition/currentness/release return Futures and can await the selected etcd operations directly.

Benefits:

- honest I/O boundary;
- natural timeout/cancellation propagation;
- direct mapping to `etcd-client`;
- no hidden blocking runtime.

Costs:

- changes the currently synchronous provider-neutral seam;
- requires updating reference harnesses/callers;
- requires an explicit runtime owner later;
- must preserve the deterministic semantic tests rather than making them depend on a real executor/backend.

### T2 — preserve the synchronous semantic model and add a separate asynchronous production authority port

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Keep `ReachabilityLiveOwnerFence`, grant/currentness/release classifications and pure invariants as provider-neutral semantic types, but introduce an explicit async I/O port for production authority operations.

The async port would be implemented by the control-plane etcd provider and consumed by the future runtime orchestration layer. The current synchronous trait can remain as a deterministic reference/test seam unless a later cleanup chooses to replace it.

Benefits:

- preserves Sans-I/O/reference semantics;
- prevents network I/O from being disguised as synchronous local state;
- makes async ownership/cancellation explicit;
- supports mockable async Txn integration tests before real endpoints;
- avoids forcing the pure NAT/reachability state machines to own a Tokio runtime.

Costs/open questions:

- exact location/name of the async port remains to be selected;
- the runtime/orchestrator crate that owns awaiting it remains unselected;
- translation between control-plane provider records and remote-bridge semantic grants must remain exact and fail closed.

### T3 — long-lived dedicated authority worker behind a synchronous command channel

Classification: `ELIGIBLE_ONLY_IF_RUNTIME_CONSTRAINTS_REQUIRE_IT / NOT_PREFERRED`.

A dedicated task/thread could own `etcd-client`, while synchronous callers send commands and wait for responses.

This can be made correct, but it introduces queueing, shutdown, timeout and backpressure semantics that PRW does not currently need to accept merely to retain a synchronous trait.

It should not be selected unless a later runtime constraint demonstrates a concrete need.

## Crate-placement direction supported by current evidence

The current dependency materialization strongly supports keeping provider-specific etcd code under the shared control-plane side rather than moving `etcd-client` into NAT traversal or remote transport.

A future implementation can preserve acyclic dependencies by following this direction:

1. `prw-control-plane` owns provider-specific etcd client/Txn/reconciliation implementation and provider record/key codecs after Group A is selected;
2. exact peer identity input is passed into that layer through a deliberately selected dependency/type boundary;
3. `prw-remote-bridge` continues to own the PRW reachability live-owner semantic types and effect-fencing contract;
4. a runtime/orchestration layer awaits control-plane authority operations and constructs/validates `ReachabilityLiveOwnerGrant` only from unambiguous authoritative results;
5. R1-R4 effect sinks consume the grant/fence and must reject stale authority at or atomically with the effect boundary.

This direction does not yet select whether the runtime/orchestration layer is `prw-agent`, a new integration crate, or a future expanded bridge layer.

## Dependency-cycle guard

A direct `prw-control-plane -> prw-remote-bridge` dependency is not preferred for the provider implementation because the remote/Agent integration path already points toward the bridge and could create undesirable upward coupling/cycles as production dependencies expand.

Preferred layering principle for selection review:

`low-level identity/domain inputs -> control-plane provider` and `runtime integration -> control-plane + remote-bridge semantics`, not `control-plane provider -> remote-bridge runtime glue`.

If exact `PeerConnectivityIdentity` is admitted directly into control-plane, the likely dependency addition would be from control-plane to the lower-level `prw-connectivity` crate, not to `prw-remote-bridge`. That dependency is not selected by this audit.

## Minimum test plan after async placement selection

Before any real etcd endpoint is contacted:

1. compile-time API test proving the provider port is explicitly async;
2. mock/in-memory async KV boundary for Get/Txn outcome scripting;
3. acquisition success -> exact grant mapping;
4. compare failure -> contention/stale re-observation;
5. timeout after possibly committed Txn -> fail closed + reconciliation;
6. matching attempt identity -> committed reconciliation;
7. non-matching successor -> original attempt not current;
8. stale release cannot clear newer owner;
9. authority unavailability cannot produce `Current`;
10. cancellation during request cannot manufacture a grant;
11. shutdown/restart cannot reuse a fence;
12. deterministic reference semantic tests remain independent of a real runtime/backend.

After Group A/B selections and mock validation:

13. disposable etcd v3.7 integration;
14. selected TLS/auth/RBAC integration;
15. quorum/restart/restore scenarios;
16. R1-R4 effect-boundary tests.

## What C02f-T does not authorize

This audit does not authorize:

- changing `ReachabilityLiveOwnerAuthority` to async;
- adding Tokio or another runtime dependency to `prw-agent` or `prw-remote-bridge`;
- adding a new crate;
- moving `etcd-client`;
- adding `prw-connectivity` to `prw-control-plane`;
- selecting schema/record bytes or CAS target;
- selecting TLS features or credentials;
- creating endpoints/cluster/cloud resources;
- contacting etcd;
- activating R1-R4 network effects.

## C02f-T conclusion

The next production-code gate is not merely “write the etcd adapter.” The current synchronous semantic authority seam and the selected asynchronous etcd client require an explicit integration boundary.

The strongest direction supported by current evidence is:

- preserve deterministic Sans-I/O/live-owner semantic types;
- keep provider-specific etcd I/O in the shared control-plane side;
- expose an explicit asynchronous production authority port rather than hiding async network I/O behind synchronous methods;
- select the runtime/orchestrator owner separately;
- preserve acyclic crate layering;
- only construct live-owner grants from unambiguous authoritative outcomes;
- keep R1-R4 sink fencing as a separate mandatory implementation step.

This is a recommendation for explicit selection review only. No runtime/API architecture is silently locked by C02f-T.

C02d remains frozen and must not be modified.
