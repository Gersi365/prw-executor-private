# Phase 152 C02f-F — Live-Owner Effect-Sink and Failure-Scope Evidence Audit

Status: `EVIDENCE_AUDIT_COMPLETE / REACHABILITY_EFFECT_SINKS_NOT_YET_PRODUCTION_MATERIALIZED / P1_P4_PRODUCT_DECISIONS_UNRESOLVED / P5_FAIL_CLOSED_BASELINE_LOCKED / P6_FUTURE_REACHABILITY_SINK_CLASSES_IDENTIFIED / PLACEMENT_NOT_SELECTED / PROVIDER_NOT_SELECTED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `21504cc359921a8ace81c311e8e45ebfea3c0be0`
Exact predecessor tree: `38b1d6dd1c9ce0a22a5a16dc28bf2d563fb580f5`
Predecessor checkpoint: `C02f-E live-owner authority placement decision analysis`

## Purpose

C02f-F performs the last safe repository-native evidence pass before any live-owner authority placement decision.

C02f-E proved that T2 Agent-local, T3 control-plane shared authority and T4 dedicated shared authority are architecture choices whose selection depends on predicates P1-P6. Generic continuation does not authorize choosing those product/deployment/failure-domain semantics.

This checkpoint therefore asks only two evidence questions:

1. which P1-P6 predicates can the current repository actually answer without inventing product requirements; and
2. which concrete or future effect boundaries belong to the C02e/C02f live-owner reachability tenancy domain.

C02f-F does not choose T2/T3/T4, a database/provider, service topology, schema, wire protocol, lease duration, runtime adapter, production endpoint or deployment target.

## Inherited authority invariants

The following remain unchanged:

- live-owner namespace is exact `DeviceId + TransportIdentity`;
- every replacement for one exact namespace receives a strictly newer non-zero logical `u128` fence;
- restart/failover/recovery cannot make an older fence authoritative again;
- stale release cannot clear a newer grant;
- authority ambiguity/unavailability fails closed;
- TTL, heartbeat and wall-clock expiry are not primary stale-owner safety authority;
- candidate-publication freshness is a separate authority domain;
- transport endpoint, IP/port, candidate, relay route and QUIC initiator role are not identity;
- a one-time `currentness()` check before asynchronous work is insufficient;
- any racing side effect in the live-owner domain must reject stale fences at, or atomically with, the effect boundary.

## Exact-head repository evidence

### Phase 141 NAT traversal is Sans-I/O

`crates/prw-nat-traversal/src/lib.rs` explicitly states that Phase 141 owns STUN/ICE protocol state only and owns no socket, async runtime, DNS resolver, process, tunnel, route or firewall mutation surface.

Its `TraversalDatagram` is a bounded value object. `StunDiscovery::poll_transmit()` and the ICE session expose datagrams for a caller to send, but the crate itself does not transmit them.

Therefore the current Phase 141 source is not yet the external network side-effect sink at which a live-owner fence can be enforced against an actual UDP send.

### Phase 140 remote transport is disposable and not Agent-integrated

`crates/prw-remote-transport/src/lib.rs` explicitly describes itself as a disposable QUIC/TLS mesh transport foundation and states that it does not integrate the production Agent.

The current source builds endpoint/TLS/transport configuration and inspects established `quinn::Connection` identity. In the reviewed source it does not own the future production Agent reachability orchestration that binds traversal selection, live-owner tenancy and long-lived QUIC task lifecycle together.

Therefore the current Phase 140 crate is not evidence that the final production live-owner side-effect boundary has already been selected.

### Phase 143 bridge is capability authorization, not reachability tenancy

`crates/prw-remote-bridge/src/lib.rs` explicitly owns no socket, process, shell, filesystem root, PTY, DNS resolver, firewall, route, TUN/TAP or production runtime activation.

It validates authenticated session, registry/current transport identity and capability policy before yielding typed operations. Those operations may later drive file, terminal or forwarding providers, but bridge authorization is a separate authority domain from C02f live-owner reachability tenancy.

C02f-F therefore does not reinterpret every capability effect as automatically requiring the reachability live-owner fence.

### File-service proves why effect-domain scoping matters

`crates/prw-file-service/src/lib.rs` contains real local filesystem effects such as descriptor-anchored `openat`, `mkdirat`, writes, `sync_all` and postcondition checks.

Those are application capability effects governed by file authorization contracts. Their existence proves that PRW has concrete side effects, but does not by itself prove that the C02f reachability live-owner fence must be threaded into every file operation.

A later integration may require a live-owner fence to guard some higher-level remote session or transport-owned work, but that coupling must be explicitly reviewed rather than inferred from the existence of a side effect.

## P1-P6 evidence classification

### P1 — cross-host replacement during authority-host loss

Question: must another surviving host/process establish a newer live owner while the original authority host is unavailable?

Repository evidence: `UNRESOLVED_PRODUCT_REQUIREMENT`.

The repository demonstrates multi-device remote connectivity and separate host/client roles, but no current contract requires the live-owner authority itself to remain available by moving to another authority host during host loss.

Multi-device product topology is not equivalent to cross-host authority failover.

Result: P1 cannot be set to `YES` or `NO` from current source without an architecture/product decision.

### P2 — authority failure isolated from control-plane failure

Question: must live-owner authority remain available independently of control-plane service failure?

Repository evidence: `UNRESOLVED_ARCHITECTURE_REQUIREMENT`.

The control plane is already separate from the mesh data plane, but `prw-control-plane` remains provider-neutral typed source and does not currently own durable live-owner state. No current contract requires live-owner authority to share or avoid the control-plane failure domain.

Result: P2 cannot be set from current repository evidence.

### P3 — new dedicated service acceptable

Question: is a new dedicated shared authority service acceptable for initial productization?

Repository evidence: `UNRESOLVED_PRODUCT_DEPLOYMENT_DECISION`.

No current source or locked contract grants permission to add that service, its credentials, replication, monitoring or deployment lifecycle.

Result: P3 cannot be inferred as `YES`; T4 remains conditional only.

### P4 — deliberately single-host authority scope acceptable

Question: may one exact live-owner namespace deliberately depend on one designated Agent-host authority, failing closed if that host is lost?

Repository evidence: `UNRESOLVED_PRODUCT_AVAILABILITY_DECISION`.

The Agent is an authoritative host runtime and its UI-independent lifecycle makes T2 structurally plausible. That fact does not prove that single-host live-owner availability is acceptable as product policy.

Result: P4 cannot be set from current repository evidence.

### P5 — liveness while authority is unreachable

Repository evidence: `SAFETY_BASELINE_RESOLVED / PRODUCT_LIVENESS_NOT_RESOLVED`.

C02e/C02f already lock that ambiguous/unavailable authority never becomes `Current`. Therefore currentness-sensitive acquisition/effects must fail closed unless a later separately reviewed degraded mode proves equivalent safety.

What remains unresolved is only the product liveness requirement: whether a fail-closed pause is acceptable operationally or whether an additional safe continuity design is required.

Result: the safety answer is fixed; the availability/product answer is not.

### P6 — exact effect sinks consuming the fence

Repository evidence: `PRODUCTION_REACHABILITY_SINK_NOT_YET_MATERIALIZED / FUTURE_SINK_CLASSES_IDENTIFIABLE`.

The repository does not yet contain the production Agent/runtime adapter that performs real reachability networking under C02e live-owner tenancy. Therefore C02f-F cannot name an already-existing final production function and claim it is the complete sink.

It can, however, identify the minimum future sink classes that must be reviewed when runtime integration is staged.

## Future reachability effect-sink classes

### R1 — traversal UDP transmit boundary

When a future runtime consumes `TraversalDatagram` from Phase 141 and actually transmits bytes through a UDP socket, that adapter is a racing external side-effect boundary.

If multiple old/new owners can race, stale generations must be rejected before or atomically with actual transmission/task ownership. Checking `currentness()` once and then allowing an unfenced long-running send loop is insufficient.

Current status: `FUTURE_RUNTIME_ADAPTER / NOT_PRESENT_IN_PHASE141_SANS_IO_SOURCE`.

### R2 — traversal session timer/task ownership

A future Agent/runtime may schedule ICE/STUN timeout processing, retransmission, candidate-check loops or refresh work.

Those long-lived tasks can outlive an owner replacement. The live-owner generation must therefore be bound to task ownership such that a stale task cannot continue producing externally applied traversal work after replacement.

Current status: `FUTURE_RUNTIME_TASK_BOUNDARY / NOT_PRODUCTION_MATERIALIZED`.

### R3 — selected-path QUIC connection establishment/retirement

A future reachability owner may create or retire QUIC connections when the selected connectivity path changes.

Connection creation, acceptance into the current peer runtime, and retirement/replacement are live-owner-sensitive effects. QUIC initiator ordering is transport mechanics and cannot substitute for the live-owner fence.

Current status: `FUTURE_AGENT_REMOTE_TRANSPORT_INTEGRATION_BOUNDARY`.

### R4 — reachability-owned connection/task registry

If production runtime keeps a map of active traversal/connection tasks per exact peer, installing or replacing the current task is an effect boundary that must be consistent with the authoritative fence.

A stale task must not be able to re-register itself as current after a newer owner is installed.

Current status: `FUTURE_RUNTIME_STATE_BOUNDARY`.

### R5 — candidate/publication state is explicitly separate

`ReachabilityOwner` durable candidate-publication compare-and-commit and candidate freshness tokens remain separate from live-owner tenancy.

They must not be reclassified as the R1-R4 live-owner fence sink simply because they are durable/linearizable state.

Current status: `SEPARATE_AUTHORITY_DOMAIN / NOT_A_SUBSTITUTE_FOR_LIVE_OWNER_FENCING`.

### R6 — application capability effects are not automatically live-owner sinks

Filesystem, upload, terminal and forwarding operations are real side effects with their own capability and provider contracts.

C02f-F does not globally require the reachability fence at every such operation. A later runtime design must state explicitly whether a specific capability effect can race with live-owner replacement in a way that requires fence propagation beyond the transport/reachability runtime.

Current status: `OUTSIDE_AUTOMATIC_C02F_SCOPE / EXPLICIT_INTEGRATION_REVIEW_REQUIRED_IF_COUPLED`.

## Consequence for authority placement

Current repository evidence cannot safely eliminate or select T2/T3/T4 because the deciding product predicates remain unresolved:

- T2 remains possible only if P1=`NO` and P4=`YES` under an explicit single-host authority failure model;
- T3 remains possible only if control-plane responsibility expansion and failure coupling are explicitly accepted;
- T4 remains possible only if a new dedicated service is explicitly accepted;
- P5 safety remains fail closed in every case;
- P6 cannot be considered closed until a production reachability runtime adapter and its concrete R1-R4 effect sinks are staged/reviewed.

A provider/database comparison before those placement decisions would risk selecting architecture by dependency convenience.

## What generic continuation may safely do next

Without architecture approval, a later audit may:

1. preserve T2/T3/T4 as conditional options;
2. prepare an explicit architecture decision record template for P1-P6;
3. inventory provider-independent semantic requirements for R1-R4;
4. verify current source remains byte-stable;
5. preserve Drive evidence.

Generic continuation must not:

- select T2, T3 or T4;
- add a persistence/database dependency;
- add a service/listener;
- introduce authority wire messages;
- bind sockets or start runtime tasks;
- alter Agent bootstrap/systemd;
- deploy credentials/resources;
- treat file/terminal/forwarding capability effects as automatically fenced by C02f without a reviewed coupling contract.

## Production-source byte-stability baseline

The following predecessor blobs must remain unchanged by this audit-only checkpoint:

- `crates/prw-remote-bridge/src/reachability_live_owner.rs` — `ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`
- `crates/prw-remote-bridge/src/reachability_owner.rs` — `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`
- `crates/prw-remote-bridge/src/root.rs` — `591320cbba4b7c3bdfcfd37a8176d82db33c1db6`
- `crates/prw-remote-bridge/src/lib.rs` — `1573a12f39d75ec80f25adc6360ca108d2009af0`
- `crates/prw-nat-traversal/src/lib.rs` — `86d7fddc0d6f833ab1b26949b058efbfa1487509`
- `crates/prw-remote-transport/src/lib.rs` — `35ffebccaf237fc6892dac0991a7c7fcd23576c8`
- `crates/prw-file-service/src/lib.rs` — `5990403687923da93b7bc258b650784b1a41fa47`
- `crates/prw-remote-bridge/Cargo.toml` — `5e59862f0a2ee120e05c5b4569ebe25d85ffd79d`
- root `Cargo.toml` — `fbbd220348e3008b38d4cfb1ec5721f8c12199e2`
- `Cargo.lock` — `4d69f7c6ec5a779615595f7dac8e02b2a660dc5d`
- `crates/prw-control-plane/src/lib.rs` — `668619338b1e085a4ac42bc27f793014e8a03df2`
- `crates/prw-control-plane/Cargo.toml` — `a940a7eb23764452b9ef1fb24b8d20a91ba712c9`

No build, rustfmt, Clippy, test, workflow dispatch, network or runtime validation is required for this audit-only checkpoint. C02e Tranche 6 remains the latest executable evidence for unchanged production source.

## Mutation boundary

The only repository mutation authorized by C02f-F is this audit file.

No production Rust source, Cargo/workflow, database/schema/migration, control-plane implementation, network/QUIC/ICE/STUN/TURN/relay behavior, Agent/bootstrap/systemd behavior, Android/desktop runtime, credentials, deployment or privileged host state is changed or authorized.

## Classification

`C02F_F_EFFECT_SINK_AND_FAILURE_SCOPE_EVIDENCE_AUDIT_COMPLETE / P1_CROSS_HOST_REPLACEMENT_UNRESOLVED / P2_CONTROL_PLANE_FAILURE_COUPLING_UNRESOLVED / P3_NEW_SERVICE_ACCEPTANCE_UNRESOLVED / P4_SINGLE_HOST_ACCEPTANCE_UNRESOLVED / P5_SAFETY_FAIL_CLOSED_LOCKED_LIVENESS_UNRESOLVED / P6_PRODUCTION_REACHABILITY_SINK_NOT_YET_MATERIALIZED / FUTURE_R1_R4_REACHABILITY_SINK_CLASSES_IDENTIFIED / APPLICATION_CAPABILITY_EFFECTS_NOT_AUTOMATICALLY_RECLASSIFIED / PLACEMENT_NOT_SELECTED / PROVIDER_NOT_SELECTED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
