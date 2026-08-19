# Phase 152 C02f-V — Async Authority Orchestrator Placement Readiness Audit

Status: `ASYNC_AUTHORITY_ORCHESTRATOR_PLACEMENT_READINESS_COMPLETE / TRANSPORT_RUNTIME_DEPENDENCY_OWNERSHIP_RECONCILED / AGENT_AD_HOC_TOKIO_EXPANSION_NOT_PREFERRED / REMOTE_TRANSPORT_ETCD_ORCHESTRATION_REJECTED_AS_WRONG_LAYER / REMOTE_BRIDGE_ASYNC_ORCHESTRATION_BOUNDARY_PREFERRED_FOR_SELECTION_REVIEW / REMOTE_BRIDGE_NOT_RUNTIME_EXECUTOR_OWNER_BY_THIS_REVIEW / NEW_INTEGRATION_CRATE_ELIGIBLE_ONLY_IF_LATER_COUPLING_REQUIRES / CONTROL_PLANE_REMAINS_ETCD_PROVIDER_OWNER / NORMAL_REMOTE_BRIDGE_TO_CONTROL_PLANE_DEPENDENCY_NOT_SELECTED / EXECUTOR_BOOTSTRAP_OWNER_UNSELECTED / NO_SOURCE_MUTATION / NO_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-U predecessor head: `9cfd61f3eae7800be625646b8b1f8d16a32b4d9b`
C02f-U predecessor tree: `abe38e77c3afe47870d0b163e791dc0d78de2633`
Review date: `2026-08-19`

## Purpose

C02f-T proved that real etcd authority I/O needs an explicit asynchronous production port. C02f-U narrowed the preferred Rust representation for later selection review to a statically dispatched `impl Future + Send` style port, while leaving runtime ownership and crate placement unselected.

C02f-V reviews where asynchronous **orchestration** should live in the existing PRW crate architecture. It deliberately separates:

- provider ownership;
- reachability/capability orchestration ownership;
- executor/runtime bootstrap ownership.

Those three responsibilities need not live in the same crate.

This audit makes no source/dependency/runtime selection.

## Existing architecture facts

### `prw-control-plane` already owns the etcd provider dependency

`crates/prw-control-plane/Cargo.toml` contains the selected exact dependency:

`etcd-client = { version = "=0.19.0", default-features = false }`

C02f-G/J already lock shared control-plane authority placement and etcd v3.7 as the backend provider.

Therefore provider-specific Get/Txn/reconciliation code remains naturally owned by the control-plane side.

### `prw-remote-transport` owns Quinn/Tokio transport mechanics

The locked Phase 139 transport architecture assigns the dedicated remote-transport component ownership of Quinn/Tokio and QUIC-specific rustls configuration.

Current `prw-remote-transport` direct runtime dependencies include:

- `quinn = =0.11.11` with `runtime-tokio` and `rustls-aws-lc-rs`;
- `tokio = =1.53.1` with `rt`, `macros`, `net`, `time`, `sync`, `io-util`;
- `rustls = =0.23.43` with AWS-LC.

The same Phase 139 decision explicitly states that the Agent must not gain ad hoc Quinn/Tokio usage in unrelated modules and that Agent integration belongs to the later authenticated capability bridge.

The remote-transport source itself states that it owns disposable QUIC/TLS mesh transport mechanics and does not integrate the production Agent or grant application capabilities.

### `prw-remote-bridge` is the application/reachability bridge

The production root of `prw-remote-bridge` states that it preserves the Phase 143 authenticated capability bridge and adds reviewed dynamic-reachability ownership plus provider-neutral live-owner fencing.

The legacy bridge source describes itself as the end-to-end authenticated capability bridge that joins:

- transport identity;
- application-session proof;
- current registry state;
- capability policy;
- typed existing Agent capability commands.

It owns no socket/process/filesystem/PTY/firewall/TUN/TAP or production runtime activation.

Its normal dependencies already include `prw-remote-transport`, connectivity, registry, session, policy and the capability crates.

It currently includes `prw-control-plane` only as a **dev-dependency**, proving a current test-time dependency direction from bridge to control-plane without an inverse control-plane-to-bridge dependency.

### `prw-agent` currently has no direct Tokio/control-plane dependency

The current Agent manifest depends on `prw-remote-bridge` and capability/domain crates but does not directly depend on:

- Tokio;
- Quinn;
- `prw-control-plane`.

Adding those solely to host live-owner etcd orchestration would therefore expand the Agent's direct infrastructure responsibilities rather than reuse a pre-existing Agent runtime ownership contract.

## Responsibility split required by live-owner safety

The future production path conceptually needs three layers:

1. **provider adapter** — authoritative etcd Get/Txn/reconciliation and selected key/value codecs;
2. **reachability orchestrator** — decides when acquisition/currentness/release/reconciliation is required and maps only unambiguous provider outcomes into PRW live-owner grants/fences;
3. **executor/bootstrap owner** — polls async work, owns shutdown/cancellation and eventually binds the orchestrator to real Agent/network lifecycle.

Keeping those layers distinct avoids treating “Tokio exists somewhere in the dependency graph” as proof that one crate owns all runtime behavior.

## Candidate V1 — direct Agent ownership of etcd orchestration

Classification: `ELIGIBLE_WITH_EXPLICIT_RUNTIME_ARCHITECTURE / NOT_PREFERRED_FOR_INITIAL_SELECTION`.

Conceptually the Agent would gain direct control-plane/async runtime dependencies and invoke etcd authority operations itself.

Benefits:

- top-level process has eventual access to shutdown/bootstrap lifecycle;
- fewer apparent intermediary calls.

Costs/risks:

- expands Agent dependency and responsibility surface;
- conflicts with the existing Phase 139 direction against ad hoc Quinn/Tokio use in unrelated Agent modules;
- couples the process shell directly to provider behavior instead of the existing remote bridge abstraction;
- risks duplicating reachability policy/currentness translation outside the crate that already owns those semantics.

This candidate should only be selected if a later explicit Agent runtime architecture demonstrates that the Agent is intentionally becoming the central async executor/integration owner.

## Candidate V2 — put etcd authority orchestration inside `prw-remote-transport`

Classification: `REJECTED_FOR_SELECTION_REVIEW`.

Reasons:

- remote transport is explicitly the QUIC/TLS transport mechanics layer;
- transport authentication establishes `TransportIdentity` but does not grant PRW application authority;
- live-owner namespace includes logical DeviceId + TransportIdentity and is a reachability/control-plane concern;
- importing etcd authority policy into the transport crate would mix control-plane distributed tenancy with QUIC mechanics;
- the existence of Tokio in the transport crate is not sufficient architectural justification.

`prw-remote-transport` may later expose real network effect boundaries that require a fence, but it should not become the authoritative etcd owner merely because it already depends on Tokio.

## Candidate V3 — `prw-remote-bridge` owns async live-owner orchestration, executor remains external

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Proposed responsibility direction:

- `prw-control-plane`: provider-specific etcd adapter, Txn/reconciliation, selected storage codecs;
- `prw-remote-bridge`: asynchronous reachability/live-owner orchestration and translation into existing semantic grants/fences;
- `prw-remote-transport`: QUIC/TLS mechanics and eventual fenced network effect boundary;
- future bootstrap/executor boundary: polls bridge futures and owns process-level cancellation/shutdown.

Why this fits current architecture:

1. remote-bridge already owns Phase 143 transport-to-capability composition;
2. its production root explicitly owns Phase 152 dynamic reachability and live-owner semantic modules;
3. it already depends on remote transport, connectivity, registry and session semantics needed for exact-peer orchestration;
4. it already uses control-plane as a dev-dependency, so the dependency direction `remote-bridge -> control-plane` is established in test scope and does not require an inverse control-plane dependency;
5. asynchronous Rust functions can return/poll Futures without the bridge itself owning a Tokio runtime or spawning tasks;
6. this keeps provider details below the bridge and Agent process details above it.

Required separate approvals before materialization:

- promote/add the exact normal `prw-control-plane` dependency if selected;
- select the async production authority port API/receiver model;
- decide whether exact peer identity enters the control-plane adapter through `prw-connectivity` or through provider-neutral primitive inputs;
- select the process-level executor/bootstrap owner;
- select Group A schema/CAS semantics.

V3 must not be interpreted as “remote-bridge owns Tokio.” It is an orchestration boundary recommendation only.

## Candidate V4 — new dedicated reachability-runtime/integration crate

Classification: `ELIGIBLE_IF_LATER_COUPLING_REQUIRES / NOT_PREFERRED_NOW`.

A new crate could depend on both `prw-control-plane` and `prw-remote-bridge`, leaving both existing crates narrower.

Benefits:

- explicit dependency inversion boundary;
- isolated async orchestration tests;
- avoids expanding remote-bridge if future responsibilities become large.

Costs:

- new crate/API/lifecycle surface;
- additional wiring and ownership questions;
- current remote-bridge already exists specifically as the cross-capability/transport integration layer, so a new crate would duplicate that architectural role unless a concrete dependency-cycle or complexity problem appears.

No such problem has yet been proven.

## Executor/runtime owner remains a separate unresolved gate

Even if V3 is later selected, one component must eventually drive the asynchronous bridge Future to completion in production.

C02f-V does not select:

- Tokio current-thread versus multi-thread runtime;
- whether an existing future transport bootstrap runtime is reused;
- whether Agent main/bootstrap owns the runtime handle;
- whether a dedicated service/task supervisor owns it;
- task spawning policy;
- shutdown ordering;
- retry scheduling.

A deliberate bootstrap/runtime selection is required before production activation, but the provider/orchestration layers can be implemented and mock-tested without selecting production endpoints.

## No implicit runtime from transitive dependencies

The following are explicitly invalid inferences:

- `etcd-client` depends on Tokio/Tonic transitively, therefore control-plane owns a Tokio Runtime — **false**;
- remote-transport directly depends on Tokio, therefore all PRW async work should live there — **false**;
- Agent depends on remote-bridge which depends on remote-transport, therefore Agent may call Tokio APIs without a direct architecture/dependency decision — **false**.

Dependency presence and runtime lifecycle ownership are distinct.

## Effect-fencing implications

The orchestration boundary must eventually feed fence authority into R1-R4 effect sinks:

- R1 actual UDP traversal transmit boundary;
- R2 traversal timers/tasks;
- R3 selected-path QUIC establish/retire;
- R4 reachability-owned connection/task registry.

V3's preferred layering is useful because the bridge can coordinate the authority grant with reachability state while the lower transport/NAT components remain explicit effect sinks.

However, bridge-level currentness alone is not sufficient. The inherited rule remains: stale fence rejection must occur at or atomically with each actual effect boundary.

## Minimal source tranche if V3 is later approved

A safe first materialization tranche should be intentionally narrow:

1. add only the approved normal dependency edge needed for the async authority port;
2. add provider-neutral async authority port/result types without real endpoints;
3. add a mock/in-memory provider implementation for deterministic async outcome tests;
4. add remote-bridge orchestration that maps definitive provider results to existing live-owner semantic types;
5. prove cancellation/ambiguous outcomes cannot manufacture a grant;
6. keep real etcd network connection construction disabled;
7. keep R1-R4 effect activation disabled until their own source tranche.

This ordering allows executable validation before TLS/endpoints/cluster deployment exist.

## What C02f-V does not authorize

This audit does not authorize:

- making `prw-control-plane` a normal `prw-remote-bridge` dependency;
- adding `prw-connectivity` to control-plane;
- adding or changing Rust APIs;
- adding Tokio to Agent/bridge/control-plane;
- creating a new crate;
- choosing V3 or another placement;
- selecting executor/bootstrap ownership;
- schema/CAS/attempt-token selection;
- TLS/auth/RBAC selection;
- cluster/recovery provider selection;
- contacting etcd;
- opening sockets or starting tasks;
- R1-R4 production effects.

## C02f-V conclusion

Current source and locked architecture support the following preferred direction for explicit selection review:

- provider-specific etcd authority remains in `prw-control-plane`;
- `prw-remote-bridge` becomes the asynchronous live-owner/reachability **orchestration boundary**;
- `prw-remote-transport` remains QUIC/TLS transport mechanics rather than an etcd/control-plane layer;
- the Agent does not gain ad hoc Tokio/etcd responsibilities merely to host this feature;
- the process-level async executor/bootstrap owner is selected separately.

This preserves the existing control-plane/data-plane separation and the Phase 143 bridge role while avoiding speculative new crates or direct Agent infrastructure coupling.

No placement or runtime architecture is selected by C02f-V. C02d remains frozen.
