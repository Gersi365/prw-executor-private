# Phase 152 C02f-E — Live-Owner Authority Placement Decision Analysis

Status: `PLACEMENT_DECISION_ANALYSIS_COMPLETE / PLACEMENT_NOT_SELECTED / PROVIDER_NOT_SELECTED / FAILURE_SCOPE_DECISION_PREDICATES_LOCKED / SIDE_EFFECT_FENCING_REQUIRED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `422a861c57c7744342331f7bb502cbe525ae0dcd`
Exact predecessor tree: `3ccb125ff41ab4e8c2e6d6294d16ed59eb6505c9`
Predecessor checkpoint: `C02f-D live-owner authority topology option analysis`

## Purpose

C02f-E converts the C02f-D topology inventory into explicit placement decision predicates for the three still-eligible families:

- T2 — Ubuntu Agent-local durable authority;
- T3 — central control-plane authority;
- T4 — dedicated shared live-owner authority service.

This checkpoint does **not** select T2/T3/T4 or a provider/database/consensus product, schema, wire protocol, region/AZ topology, runtime listener, credential model or deployment target. Its purpose is to prevent backend choice from silently deciding architecture.

## Inherited safety contract

C02f-E preserves without reinterpretation:

1. authority namespace is exact `DeviceId + TransportIdentity`;
2. every replacement receives a strictly newer non-zero logical `u128` fence;
3. restart/failover/recovery cannot make an older fence authoritative again;
4. stale release cannot clear a newer grant;
5. ambiguous/unavailable authority fails closed;
6. TTL/heartbeat/clock expiry is not primary safety authority;
7. candidate-publication freshness is separate from live-owner fencing;
8. UI/process-local authority is rejected;
9. independently authoritative asynchronous replicas are rejected unless one linearizable authority layer serializes them;
10. every racing side-effect path must reject stale fences at or atomically with its effect boundary.

## Repository constraints on placement

### Control plane

Phase 139 already uses the control plane for signaling, enrollment/session orchestration, registry state, candidate exchange and coordination. However current `prw-control-plane` remains provider-neutral typed source: persistence, retries/idempotency, listener/runtime topology and live-owner fencing are not implemented or selected.

Therefore T3 is an eligible placement family, not an existing durable authority. Selecting T3 later is a real architecture expansion.

### Data plane

The remote data plane remains peer-to-peer QUIC/TLS over explicit selected candidates, separate from the control plane. Connection initiator, IP/port, candidate and relay path are transport/routing facts, not ownership authority.

### Side effects

Phase 143 keeps authorization separate from concrete effect owners such as filesystem, PTY/terminal, forwarding sockets, transport/runtime and other providers. Safe `acquire()` alone therefore cannot satisfy C02f-A; the fence must reach the actual effect boundary when replacement can race with work.

### Agent

The headless Ubuntu Agent is stable host runtime authority and independent of desktop UI lifecycle. That makes T2 structurally plausible only for an explicitly single-host authority domain. It does not prove cross-host failover, partition arbitration or distributed continuity.

## Failure-scope decisions

### F1 — process crash/restart

Classification: `MANDATORY_SAFETY_SCOPE`.

All eligible placements must preserve fencing history across owner/authority process restart and must fail closed on ambiguous recovery.

### F2 — host loss and cross-host replacement

Classification: `PLACEMENT_DISCRIMINATOR / NOT_YET_LOCKED`.

Question: must another surviving host/process establish a newer owner while the original authority host is unavailable?

- `NO`: T2 may remain eligible under an explicit single-host authority scope.
- `YES`: standalone T2 is insufficient; one shared linearization domain such as T3/T4/equivalent is required.

A provider cannot answer F2 implicitly.

### F3 — authority/backend unavailability

Classification: `LIVENESS_POLICY_NOT_YET_LOCKED / SAFETY_FAIL_CLOSED_ALREADY_LOCKED`.

Ambiguous or unavailable authority is never `Current`. What remains to decide is whether product liveness accepts a fail-closed pause or requires a separately proven degraded mode. Stale cache state cannot become authority.

### F4 — network partition / split brain

Classification: `MANDATORY_SAFETY_SCOPE`.

Two partitions must not independently issue authoritative owners for one exact namespace. Multi-replica T3/T4 designs must reduce to one reviewed linearization domain.

### F5 — durable-state loss / stale restore

Classification: `MANDATORY_RECOVERY_SAFETY / PROCEDURE_NOT_YET_LOCKED`.

Restore/recreation must not lower the last-issued fencing generation. If monotonicity cannot be established after recovery, authority fails closed until a reviewed recovery procedure establishes a safe history.

### F6 — zone/region continuity

Classification: `PRODUCT_AVAILABILITY_DECISION_NOT_YET_LOCKED`.

C02f-A does not itself require multi-zone or multi-region continuity. If either becomes an initial requirement, standalone T2 is eliminated and T3/T4 must be evaluated using exact distributed provider semantics.

## T2 — Agent-local durable authority

Classification: `CONDITIONAL / SINGLE_HOST_SCOPE_ONLY`.

Strengths:

- aligns with existing headless Agent host-runtime authority;
- independent of desktop UI;
- may place authority near host-local effect sinks;
- avoids introducing a new shared service when every contender serializes through one designated Agent.

Required conditions:

1. one exact namespace has one designated Agent authority domain;
2. every contender consults that authority;
3. F2 cross-host continuity is not required;
4. durable state survives process restart without fence reset;
5. restore cannot make an older fence current;
6. non-local effects still consume/reject the fence;
7. Agent loss causes fail-closed unavailability, not independent local takeover elsewhere.

T2 is eliminated if cross-host replacement during authority-host loss, active/active multi-host authority, or multi-zone/region continuity is an initial requirement.

Architectural tradeoff: smallest new shared-service surface, but authority availability is coupled to one Agent-host failure domain.

C02f-E does not select T2.

## T3 — central control-plane authority

Classification: `CONDITIONAL_SHARED_AUTHORITY / CONTROL_PLANE_EXPANSION_REQUIRED`.

Strengths:

- existing common coordination plane;
- logical device/transport identity and registry context already belong conceptually to this plane;
- all hosts/clients can consult one authority independent of NAT/relay path;
- a future shared durable backend can serialize multiple contenders.

Required new architecture if selected later:

1. live-owner fencing becomes an explicit control-plane responsibility;
2. durable linearizable backend;
3. authenticated bounded authority API;
4. replication/failover/partition model;
5. explicit outage policy;
6. fence propagation to effect sinks;
7. authority credentials/configuration;
8. monotonic backup/recovery procedure.

T3 is eliminated if the control plane must remain non-durable/provider-neutral, if authority must continue independently during control-plane failure, or if its trust/deployment boundary may not mutate live-owner state.

Architectural tradeoff: reuses an existing conceptual coordination plane but materially expands its persistence/runtime/failure responsibilities.

C02f-E does not select T3.

## T4 — dedicated shared live-owner authority service

Classification: `CONDITIONAL_SHARED_AUTHORITY / NEW_SERVICE_DEPENDENCY_REQUIRED`.

Strengths:

- narrow authority responsibility and trust boundary;
- one shared linearization point across multiple hosts/processes;
- failure/scaling policy can be isolated from signaling logic;
- can decouple authority failure domain from control-plane failure if deliberately deployed that way.

Required new architecture if selected later:

1. new service/runtime deployment;
2. authenticated typed authority protocol;
3. durable backend/replication model;
4. credentials, configuration, monitoring and lifecycle ownership;
5. partition/outage policy;
6. fence propagation to effect sinks;
7. monotonic backup/recovery semantics.

T4 is eliminated if no new shared service is acceptable, if architecture requires colocation in the control plane, or if a single-host authority scope is explicitly chosen and separate-service complexity is rejected.

Architectural tradeoff: cleanest separation, largest explicit new service/deployment surface.

C02f-E does not select T4.

## Placement decision matrix

| Requirement | T2 Agent-local | T3 Control-plane | T4 Dedicated shared |
| --- | --- | --- | --- |
| F1 process restart safety | required | required | required |
| F2 cross-host replacement | standalone `NO` | conditional `YES` | conditional `YES` |
| F3 outage coupling | Agent host | control-plane authority | dedicated authority service |
| F4 partition safety | one Agent must remain sole linearization point | provider/topology proof | provider/topology proof |
| F5 stale restore prevention | local durable recovery proof | shared recovery proof | shared recovery proof |
| F6 multi-zone/region continuity | standalone `NO` | conditional | conditional |
| New shared service | no | conceptual plane exists, responsibility expands | yes |
| Side-effect sink fencing | mandatory | mandatory | mandatory |
| Existing concrete provider | no | no | no |

## Decision predicates for the next architecture checkpoint

### P1 — Is cross-host replacement required during authority-host loss?

- `NO`: T2/T3/T4 remain possible.
- `YES`: standalone T2 is eliminated.

### P2 — Must authority failure be isolated from control-plane failure?

- `NO`: T3/T4 remain possible.
- `YES`: T3 is constrained or eliminated as owner; T4/equivalent independent shared authority becomes the distributed family to evaluate.

### P3 — Is a new dedicated service acceptable for initial productization?

- `NO`: T4 is eliminated.
- `YES`: T4 remains available.

### P4 — Is a deliberately single-host authority scope acceptable?

- `YES`: T2 remains available.
- `NO`: T2 is eliminated.

### P5 — What liveness is required while authority is unreachable?

Safe baseline is fail-closed. Continued currentness-sensitive work during authority outage requires separate proof and may not use stale cache state as authority.

### P6 — Which exact effect sinks consume the fence?

Placement cannot be locked until the first concrete racing effect paths are named and stale rejection is located at the sink or equivalent atomic boundary. A one-time `currentness()` precheck is insufficient.

## Neutral decision tree

1. If single-host scope is acceptable and cross-host replacement is not required, T2 remains a valid candidate.
2. If cross-host replacement is required, standalone T2 is eliminated and one shared linearization domain is mandatory.
3. If the control plane may own durable live-owner mutation and its failure coupling is acceptable, T3 remains a shared candidate.
4. If authority must be isolated from control-plane failure/responsibility, T4/equivalent independent shared authority remains the shared candidate family.
5. If T2 is eliminated and T4 is forbidden, T3 or another already-approved shared authority plane must be explicitly authorized; a backend choice cannot resolve that architecture contradiction.
6. Provider selection occurs only after placement/failure scope is explicitly locked.

## Side-effect fencing boundary

All three candidate placements share the same rule:

> possession of a previously issued grant is insufficient for an effect that can complete after a newer grant is installed.

Eligible mechanism classes include:

- same authoritative transaction as the effect;
- sink-side conditional mutation requiring current fence;
- typed protocol carrying the fence with receiver-side stale rejection;
- another reviewed equivalent.

Insufficient mechanisms include:

- `currentness()` followed by unfenced async work;
- cached `Current`;
- TTL/lease expiry alone;
- QUIC connection ownership;
- candidate/relay path ownership;
- UI/session state.

## Provider-selection consequence

If T2 is selected later, embedded/single-host transactional backends may be evaluated only against the explicitly locked single-host failure model.

If T3/T4 is selected later, provider proof must additionally cover multi-client linearization, leader/primary/quorum failover, partition behavior, indeterminate commit handling, authenticated service access, replication/recovery semantics and deployment/credential footprint.

No specific provider is named or preferred by C02f-E.

## Architecture approval boundary

Selecting T2, T3 or T4 fixes authoritative service/process ownership, failure domain, remote dependency topology, operational trust boundary and later deployment footprint. It is therefore an architecture decision and is not authorized silently by generic continuation.

C02f-E stops before that selection.

## Production-source byte-stability baseline

The following predecessor blobs must remain unchanged:

- `crates/prw-remote-bridge/src/reachability_live_owner.rs` — `ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`
- `crates/prw-remote-bridge/src/reachability_owner.rs` — `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`
- `crates/prw-remote-bridge/src/root.rs` — `591320cbba4b7c3bdfcfd37a8176d82db33c1db6`
- `crates/prw-remote-bridge/Cargo.toml` — `5e59862f0a2ee120e05c5b4569ebe25d85ffd79d`
- root `Cargo.toml` — `fbbd220348e3008b38d4cfb1ec5721f8c12199e2`
- `Cargo.lock` — `4d69f7c6ec5a779615595f7dac8e02b2a660dc5d`
- `crates/prw-control-plane/src/lib.rs` — `668619338b1e085a4ac42bc27f793014e8a03df2`
- `crates/prw-control-plane/Cargo.toml` — `a940a7eb23764452b9ef1fb24b8d20a91ba712c9`

No new executable validation claim is created by this audit-only checkpoint. C02e Tranche 6 remains the latest executable authority evidence for unchanged production source.

## Mutation boundary

The only repository mutation authorized by C02f-E is this audit file.

No production Rust source, Cargo/workflow, database/schema/migration, network listener, QUIC/ICE/STUN/TURN/relay behavior, Agent/bootstrap/systemd behavior, control-plane implementation, Android/desktop runtime, credential, cloud resource, deployment or privileged host state is changed or authorized.

## Classification

`C02F_E_PLACEMENT_DECISION_ANALYSIS_COMPLETE / T2_AGENT_LOCAL_SINGLE_HOST_CONDITIONAL / T3_CONTROL_PLANE_SHARED_CONDITIONAL / T4_DEDICATED_SHARED_CONDITIONAL / F1_F4_F5_SAFETY_REQUIREMENTS_LOCKED / F2_F3_LIVENESS_F6_AVAILABILITY_REQUIRE_EXPLICIT_DECISION / SIDE_EFFECT_SINK_IDENTIFICATION_REQUIRED / PLACEMENT_NOT_SELECTED / PROVIDER_NOT_SELECTED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
