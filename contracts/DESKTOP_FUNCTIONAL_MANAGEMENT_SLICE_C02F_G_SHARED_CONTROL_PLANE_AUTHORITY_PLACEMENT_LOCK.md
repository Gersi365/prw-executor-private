# Phase 152 C02f-G — Shared Control-Plane Live-Owner Authority Placement Lock

Status: `ARCHITECTURE_PLACEMENT_LOCK / T3_SHARED_CONTROL_PLANE_AUTHORITY_SELECTED / CROSS_HOST_REPLACEMENT_REQUIRED / AUTHORITY_AMBIGUITY_FAIL_CLOSED / PROVIDER_BACKEND_UNSELECTED / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `91fb5553e537a435adb7ac00dc0bce0db6913e31`
Exact predecessor tree: `a77f294ed48d31ac4e7dc6b27b08866426402877`
Predecessor checkpoint: `C02f-F live-owner effect-sink and failure-scope evidence audit`

## Architecture approval

The project architecture owner explicitly approved the following direction on 2026-08-19:

- live-owner authority is placed in the shared control-plane authority domain;
- cross-host replacement must be possible;
- authority ambiguity or unavailability fails closed;
- concrete provider/backend selection remains deferred to the next checkpoint.

C02f-G materializes that approval as a repository contract. It does not extend the approval beyond those points.

## Locked placement decision

### T3 — shared control-plane authority

Classification: `SELECTED`.

The authoritative live-owner state for one exact peer lifecycle belongs to a shared control-plane authority domain rather than to one Agent-local authority domain or a separate dedicated authority service.

This is a logical authority-placement decision. It does **not** mean that the current `prw-control-plane` crate already implements the authority, nor does it authorize an immediate listener, database, deployment, credential, replication or runtime mutation.

The current control-plane source remains provider-neutral typed source until a later implementation checkpoint explicitly materializes the selected architecture.

### T2 — Agent-local durable authority

Classification: `ELIMINATED_FOR_INITIAL_ARCHITECTURE`.

A standalone Agent-local authority cannot satisfy the newly locked requirement that another surviving host must be able to establish a newer live owner when the previous authority host is unavailable.

An Agent may later host caches, clients, adapters or effect-sink enforcement, but it must not become an independently authoritative live-owner domain for the same exact namespace.

### T4 — dedicated shared authority service

Classification: `NOT_SELECTED`.

A dedicated service remains a conceivable future architecture alternative only under a separately approved redesign. It is not the selected initial placement and must not be introduced implicitly by provider convenience.

## Placement predicates after approval

### P1 — cross-host replacement during authority-host loss

Locked value: `YES`.

A surviving authorized contender on another host must be able to obtain a strictly newer live-owner fence through the shared authority domain when replacement is permitted by higher-level identity/session/registry policy.

The old host must not retain or regain authority merely because it later reconnects.

Consequence: standalone T2 is eliminated.

### P2 — authority placement relative to control-plane failure domain

Locked placement: `CONTROL_PLANE_AUTHORITY_DOMAIN`.

C02f-G accepts that live-owner authority is a control-plane responsibility. It does not yet select the internal HA/failover topology of that domain or require a particular region/AZ/service-process decomposition.

If the shared authority domain is unavailable, partitioned or unable to establish an unambiguous result, currentness-sensitive acquisition/effects fail closed.

A later provider/topology decision may improve availability but may not weaken this safety rule.

### P3 — dedicated new authority service

Locked value for initial architecture: `NOT_SELECTED`.

Provider evaluation must target the selected T3 authority domain and must not silently create T4 as a side effect of choosing a product.

### P4 — deliberately single-host authority scope

Locked value: `NO`.

The live-owner authority cannot be scoped to one designated Agent host in a way that prevents cross-host replacement.

### P5 — liveness while authority is unreachable

Locked safety value: `FAIL_CLOSED`.

No stale cache, expired lease, previous grant, local Agent memory, candidate state or transport connection may be treated as current authority when the shared control-plane authority result is unavailable or ambiguous.

This checkpoint does not define an availability SLO, degraded mode, retry cadence or timeout budget.

### P6 — effect-sink consumption of the fence

Classification: `INHERITED / IMPLEMENTATION_BOUNDARY_PENDING`.

C02f-F identified the future reachability sink classes that must be fenced when production runtime integration exists:

- R1 — actual traversal UDP transmit boundary;
- R2 — traversal timer/check/retransmission task ownership;
- R3 — selected-path QUIC connection establishment/acceptance/retirement;
- R4 — reachability-owned current connection/task registry.

The selected control-plane authority must issue/validate state in a form that can be propagated to those sinks or atomically coupled to an equivalent enforcement boundary.

## Shared authority semantics

The selected T3 design must preserve all C02f-A safety requirements and the executable C02e Tranche 6 semantics.

### Exact namespace

The authority namespace remains exactly:

`DeviceId + TransportIdentity`

IP address, port, candidate, relay route, QUIC initiator, request ID, process ID, host address and UI/session-local state are not substitutes for this identity.

### One shared linearization domain

All contenders that may acquire or replace live ownership for the same exact namespace must serialize through one shared authoritative linearization domain.

Multiple control-plane replicas are permitted only if their provider/topology semantics present one authoritative linearization history for the namespace.

Independently authoritative asynchronous replicas are forbidden.

### Strictly monotonic durable fencing

Every replacement owner for one exact namespace must receive a strictly newer non-zero logical `u128` fence than every fence previously issued for that namespace.

Cross-host replacement must preserve this ordering across:

- process restart;
- host loss;
- service failover;
- backend failover;
- retry;
- reconnect;
- stale release;
- restore/recovery.

No host may restart local allocation from an older value.

### Atomic replacement

The shared authority must provide an operation equivalent in safety to:

`observe current -> allocate strictly newer fence -> atomically install replacement -> return installed grant`

Split client-side read/modify/write without backend-enforced serialization is insufficient.

### Permanent stale-owner rejection

After a newer fence is installed, every older grant remains stale permanently for authority purposes.

The old host returning after a partition or outage must not resume currentness based on cached memory, a still-open connection, TTL, heartbeat, or prior lease.

### Stale release isolation

Release must be conditional on the exact current grant/fence. A stale host or delayed request must not clear a newer owner.

### Ambiguous outcomes

An unavailable, timed-out, indeterminate or otherwise ambiguous authority result is never interpreted as success or currentness.

A caller may retry according to a later bounded retry contract, but after an ambiguous mutation outcome it must first recover authoritative state rather than blindly issuing side effects under an assumed grant.

### Recovery

Backup/restore, replica replacement and disaster recovery must preserve enough history to guarantee that a previously issued fence cannot become current again or be reissued.

If that cannot be established, the authority remains fail closed until an approved recovery procedure re-establishes monotonic history.

## Cross-host replacement contract

Cross-host replacement is now an explicit product/architecture requirement.

For one exact namespace:

1. host A may hold fence `F`;
2. host A may crash, disappear or become partitioned;
3. host B may later acquire through the shared authority domain;
4. if acquisition succeeds, host B must receive `F2` where `F2 > F`;
5. every effect boundary governed by the live-owner domain must reject later work from host A carrying `F`;
6. host A reconnecting must reload/reacquire and must not revive `F`;
7. a delayed release from host A must not clear `F2`.

The concrete mechanism for deciding *when* host B is eligible to attempt replacement remains governed by authenticated identity/session/registry policy and future liveness/runtime contracts. C02f-G does not authorize endpoint/IP-based takeover.

## Control-plane responsibility expansion

Selecting T3 makes live-owner fencing an explicit future control-plane responsibility in addition to existing identity/session/coordination concepts.

A later implementation must therefore define, at minimum:

- a typed live-owner authority API/operation set;
- authenticated authorization for authority callers;
- durable state and strictly monotonic fence allocation;
- atomic acquire/replace/current/release semantics;
- bounded unavailable/ambiguous/stale result classes;
- retry/idempotency behavior for indeterminate outcomes;
- replication/failover/partition semantics;
- recovery/backup/restore semantics;
- credential/configuration ownership;
- propagation of fences to R1-R4 effect boundaries.

None of those concrete mechanisms are selected by C02f-G.

## Provider/backend selection gate

Provider/backend evaluation is now **eligible** because authority placement and cross-host failure scope are locked.

The next checkpoint may compare concrete provider/backend candidates, but every candidate must be evaluated against the selected T3 topology and the inherited safety contract.

At minimum, provider proof must cover:

1. multi-client linearizable conditional mutation or transaction semantics;
2. strictly monotonic durable generation allocation without reuse;
3. cross-host contender serialization;
4. leader/primary/quorum failover behavior;
5. network partition behavior and split-brain prevention;
6. indeterminate commit/outcome handling;
7. stale conditional release behavior;
8. durable recovery and stale-restore prevention;
9. representation/storage strategy capable of preserving the logical non-zero `u128` fence without changing its semantics;
10. authenticated access and credential footprint;
11. deployment dependency and operational failure surface;
12. ability to support fence propagation/enforcement at R1-R4 sinks.

Provider choice remains `UNSELECTED` in this checkpoint.

## Explicit non-decisions

C02f-G does not select or authorize:

- SQL vs KV vs consensus product family;
- a named database/cloud product/provider;
- table/key/schema layout;
- external fence byte encoding or database column type;
- RPC/HTTP/gRPC/custom wire protocol;
- control-plane listener address/port;
- region, zone, quorum or replica count;
- lease/heartbeat/TTL duration;
- retry/backoff constants;
- cloud account/project/resource;
- credentials/secrets;
- migration/rollback procedure;
- production runtime activation;
- Agent systemd/bootstrap changes;
- Android/desktop runtime changes;
- QUIC/ICE/STUN/TURN/relay activation.

## Production-source byte-stability baseline

This architecture lock is documentation-only. The following predecessor blobs must remain unchanged:

- `crates/prw-remote-bridge/src/reachability_live_owner.rs` — `ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`
- `crates/prw-remote-bridge/src/reachability_owner.rs` — `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`
- `crates/prw-remote-bridge/src/root.rs` — `591320cbba4b7c3bdfcfd37a8176d82db33c1db6`
- `crates/prw-remote-bridge/src/lib.rs` — `1573a12f39d75ec80f25adc6360ca108d2009af0`
- `crates/prw-nat-traversal/src/lib.rs` — `86d7fddc0d6f833ab1b26949b058efbfa1487509`
- `crates/prw-remote-transport/src/lib.rs` — `35ffebccaf237fc6892dac0991a7c7fcd23576c8`
- `crates/prw-control-plane/src/lib.rs` — `668619338b1e085a4ac42bc27f793014e8a03df2`
- `crates/prw-control-plane/Cargo.toml` — `a940a7eb23764452b9ef1fb24b8d20a91ba712c9`
- `crates/prw-remote-bridge/Cargo.toml` — `5e59862f0a2ee120e05c5b4569ebe25d85ffd79d`
- root `Cargo.toml` — `fbbd220348e3008b38d4cfb1ec5721f8c12199e2`
- `Cargo.lock` — `4d69f7c6ec5a779615595f7dac8e02b2a660dc5d`

No new executable validation claim is created. C02e Tranche 6 remains the latest executable authority evidence for unchanged production source.

## Mutation boundary

The only repository mutation authorized by C02f-G is this architecture contract.

No production Rust source, Cargo/workflow, persistence dependency, database/schema/migration, listener, network socket, runtime task, Agent/bootstrap/systemd behavior, desktop/Android runtime, credential, cloud resource, deployment or privileged host state is changed or activated by this checkpoint.

## Classification

`C02F_G_SHARED_CONTROL_PLANE_AUTHORITY_PLACEMENT_LOCKED / T3_SELECTED / P1_CROSS_HOST_REPLACEMENT_YES / T2_ELIMINATED / T4_NOT_SELECTED / P5_AUTHORITY_AMBIGUITY_UNAVAILABILITY_FAIL_CLOSED / P6_R1_R4_EFFECT_SINK_OBLIGATION_INHERITED / PROVIDER_BACKEND_SELECTION_NOW_ELIGIBLE_BUT_UNSELECTED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
