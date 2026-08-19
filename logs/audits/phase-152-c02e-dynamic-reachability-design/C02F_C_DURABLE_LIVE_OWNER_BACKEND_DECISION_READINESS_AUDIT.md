# Phase 152 C02f-C — Durable Live-Owner Backend Decision-Readiness Audit

Status: `DECISION_READINESS_GATE_LOCKED / PROVIDER_NOT_SELECTED / FAILURE_TOPOLOGY_NOT_YET_LOCKED / FAMILY_LEVEL_ELIGIBILITY_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`

Repository ID: `1334911207`

Active branch: `phase-152-c02e-dynamic-reachability-design`

Exact predecessor head: `c7f7db0930d664c954b432a21d1ed0078056d71f`

Exact predecessor tree: `141c07f765cc3fdf8bf23fa8e0aacb1ce109c9f1`

Predecessor checkpoint: `C02f-B durable live-owner backend preselection audit`

## Audit purpose

C02f-C determines whether the repository contains enough locked architecture to choose a concrete durable live-owner authority provider without inventing an undeclared deployment/failure model.

This checkpoint does **not** select a database, key-value store, consensus service, coordination service, cloud product, storage engine, persistence encoding, migration strategy, wire protocol, runtime topology, network path, or deployment target.

It performs only family-level eligibility analysis against the already locked C02f-A safety contract and identifies the exact architecture inputs that must be fixed before a concrete provider can be selected responsibly.

## Authoritative inputs inherited unchanged

C02f-C inherits without reinterpretation:

- C02f-A durable live-owner backend authority contract;
- C02f-B finding that no existing concrete durable live-owner backend is present in the repository;
- C02e Tranche 6 provider-neutral `ReachabilityLiveOwnerAuthority` seam;
- exact live-owner namespace `DeviceId + TransportIdentity`;
- non-zero ordered `u128` logical fencing generation;
- strictly newer fence for every replacement in one exact peer namespace;
- permanent stale-owner rejection;
- stale release may not clear a newer owner;
- ambiguous or unavailable authority fails closed;
- recovery may not reuse or roll back a fencing generation;
- clock/TTL/heartbeat behavior is not the primary safety authority;
- future side effects that can race with replacement require stale-fence rejection at, or atomically with, their effect boundary.

Candidate-publication freshness remains a separate authority domain and is not eligible for reuse as the live-owner fencing generation.

## Repository topology audit

Current repository material does not lock a concrete durable-authority deployment topology.

The root product architecture requires separation of control plane and encrypted data plane, prefers direct peer-to-peer connectivity, and keeps the PRW Agent independent of the desktop UI lifecycle. Those are product architecture principles, not a declaration of where durable live-owner authority runs or how many authority replicas/processes exist.

The Phase 002 scope explicitly deferred database/persistent storage and deployment architecture while keeping `prw-control-plane` transport-agnostic and I/O-free.

The private-mesh connectivity foundation similarly locks provider-neutral peer/candidate semantics while deferring production networking and migration/failover timing.

Therefore the repository does not currently establish any of the following as an authoritative C02f assumption:

- single host versus multiple hosts;
- single process versus multiple authority workers;
- active/passive versus active/active authority service;
- single availability zone versus multi-zone failover;
- single region versus multi-region authority;
- whether authority state is colocated with a future control-plane service;
- whether authority state is colocated with the future side-effect sink;
- whether network partition tolerance is required for acquisition liveness;
- whether an authority service may continue after losing a quorum/primary;
- intended durability/RPO/RTO behavior;
- exact recovery procedure after durable authority-state loss.

Without those inputs, selecting a specific provider would silently choose a failure model and deployment architecture that the repository has not authorized.

## Family-level eligibility matrix

The following matrix is a safety-capability classification only. It does not select a provider.

### A. Process-local in-memory authority

Examples include ordinary maps, mutex-protected counters, reference-counted test stores, or process-local ownership tables.

Classification: `REJECTED_FOR_CONCRETE_DURABLE_AUTHORITY`

Reason:

- fence history is lost on process restart;
- no host-failure durability;
- no backend failover authority;
- cannot preserve the C02f-A no-reuse requirement across recovery.

The existing test/reference authorities remain valid executable semantic harnesses only.

### B. Single-host embedded durable transactional store

Classification: `CONDITIONALLY_ELIGIBLE_ONLY_FOR_AN_EXPLICIT_SINGLE-HOST_FAILURE_MODEL`

Potentially satisfiable properties:

- durable exact-peer record;
- atomic local transaction/CAS;
- monotonic stored fencing generation;
- stale-release compare condition.

Unproven/insufficient without an explicit topology decision:

- host-loss failover;
- multi-process/multi-host ownership arbitration;
- split-brain rejection across independently running authority instances;
- side-effect fencing outside the host/store transaction boundary.

C02f-C therefore does not permit an embedded store to be treated as distributed-authority proof merely because it is crash-durable.

### C. Transactional relational/SQL authority

Classification: `ELIGIBLE_FAMILY_SUBJECT_TO_PROVIDER_AND_TOPOLOGY_PROOF`

A concrete design could satisfy C02f-A if the selected transaction mechanism proves, for one exact peer namespace:

- linearizable or otherwise correctly serialized current-row replacement;
- atomic allocation and installation of a strictly newer durable fence;
- no generation rollback or reuse after database/process/host failover;
- conditional stale release against the exact current generation;
- fail-closed handling of indeterminate commit outcomes;
- documented behavior when replicas/primaries fail or leadership changes.

A transaction-capable SQL product is not automatically sufficient. Isolation level, failover semantics, counter representation, retry behavior and side-effect fencing must be reviewed for the exact selected deployment.

### D. Transactional key-value / consensus-backed authority

Classification: `ELIGIBLE_FAMILY_SUBJECT_TO PROVIDER_AND_TOPOLOGY_PROOF`

A concrete design could satisfy C02f-A if the backend exposes a linearizable transaction/CAS boundary that can atomically:

1. read/compare the exact current authority record;
2. allocate a strictly newer durable fencing generation;
3. install that generation as current;
4. return success only after authoritative commit.

Consensus or replication alone is not sufficient. The exact API used for acquisition/currentness/release must preserve the contract under leader change, retries, partitions and ambiguous client outcomes.

Lease/watch/TTL features, if present, remain liveness mechanisms and do not replace monotonic fencing safety.

### E. Dedicated lease/coordination service

Classification: `CONDITIONALLY_ELIGIBLE / TTL_ONLY_DESIGNS_REJECTED`

A coordination service is eligible only if its reviewed mechanism provides or can safely anchor:

- strictly monotonic non-reused fencing generations;
- linearizable current-owner replacement;
- conditional stale release;
- durable/recoverable generation history under the intended failover model.

A lease identifier, expiry timestamp, heartbeat sequence, lock presence bit, ephemeral node or TTL by itself is not C02f-A authority.

If a service cannot produce a fencing value that a future side-effect sink can reject when stale, it cannot close the live-owner safety contract by itself.

### F. Managed conditional/transactional cloud data store

Classification: `ELIGIBLE_FAMILY_SUBJECT_TO EXACT PROVIDER SEMANTICS AND DEPLOYMENT AUTHORITY`

Potentially usable mechanisms include a provider-enforced conditional update/transaction over the exact peer key plus durable generation state.

Before selection, the provider-specific proof must cover:

- exact consistency level of the read/conditional-write path;
- transaction/conditional-write linearization guarantee;
- retry and idempotency behavior after unknown client outcomes;
- failover/replication behavior;
- whether counters/versions can ever reset, wrap, be restored to an older snapshot, or be recreated from stale state;
- operational dependency, credentials, region/topology and outage implications.

Provider-managed durability does not eliminate the side-effect fencing requirement.

## Cross-family invariants

Any concrete provider selected later must implement the same abstract durable authority record semantics. At minimum the reviewed state model must preserve:

- exact namespace key: `DeviceId + TransportIdentity`;
- last-issued / current fencing generation sufficient to prevent reuse;
- current grant identity/state sufficient for exact-currentness checks;
- atomic replacement semantics;
- conditional release semantics;
- explicit ambiguous/unavailable classification;
- recoverable monotonic history.

No family is permitted to substitute endpoint, IP address, candidate ID, request/session ID, freshness token, PID/UID/GID or lease expiry as the authority key/generation.

## Side-effect fencing remains a separate mandatory decision

Provider selection cannot be completed solely by proving safe acquisition.

The future design must identify which externally visible side effects can race with owner replacement and where stale fences are rejected. Depending on the future topology, that may require one of the following classes of integration:

- same authoritative transaction as the state mutation;
- sink-side conditional mutation on the current fencing generation;
- protocol message carrying the fence with receiver-side stale rejection;
- another separately reviewed mechanism with equivalent stale-effect exclusion.

A one-time `currentness()` check before asynchronous/network work is not sufficient evidence that an old owner cannot complete a later stale side effect.

## Missing architecture inputs that block concrete provider selection

C02f-C identifies the following decisions as required inputs to the provider-selection checkpoint:

1. **Authority deployment topology** — single-process, multi-process single-host, or multi-host/distributed.
2. **Failure scope** — process crash only, host loss, storage loss, availability-zone loss, region loss, and/or network partition.
3. **Availability policy** — whether acquisition must remain available during specific backend/partition failures or must fail closed.
4. **Replication/failover model** — no replication, active/passive, quorum/consensus, managed primary/failover, or another reviewed model.
5. **Recovery authority** — what durable evidence survives and what operator/recovery procedure is permitted if authority history is lost.
6. **Side-effect sink boundary** — the exact component(s) that must reject stale fences.
7. **Operational trust boundary** — where backend credentials/configuration live and which PRW process may mutate authority state.
8. **Deployment dependency policy** — whether an external service/cloud dependency is acceptable for PRW's product architecture.

These are not implementation details. They determine whether a backend family can satisfy the safety contract at all.

## Provider-selection entry criteria

A future C02f provider-selection checkpoint may name a concrete technology only when it documents:

1. the locked topology/failure model above;
2. the exact provider primitive used for linearization;
3. the exact durable state representation strategy for a non-zero logical `u128` generation;
4. atomic acquire/replacement pseudotransaction and retry semantics;
5. currentness query semantics;
6. stale-release conditional semantics;
7. crash/restart/failover/partition behavior;
8. ambiguous-result mapping into bounded PRW errors;
9. side-effect fencing integration point;
10. dependency/build/runtime/credential/deployment footprint;
11. migration and rollback implications;
12. why no older fence can become authoritative again after any in-scope failure.

Naming a provider before these items are locked is explicitly outside C02f-C authority.

## Production-source byte-stability baseline

C02f-C is documentation/audit only. The following predecessor production/dependency blobs must remain unchanged:

- `crates/prw-remote-bridge/src/reachability_live_owner.rs`
  - `ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`
- `crates/prw-remote-bridge/src/reachability_owner.rs`
  - `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`
- `crates/prw-remote-bridge/src/root.rs`
  - `591320cbba4b7c3bdfcfd37a8176d82db33c1db6`
- `crates/prw-remote-bridge/Cargo.toml`
  - `5e59862f0a2ee120e05c5b4569ebe25d85ffd79d`
- root `Cargo.toml`
  - `fbbd220348e3008b38d4cfb1ec5721f8c12199e2`
- `Cargo.lock`
  - `4d69f7c6ec5a779615595f7dac8e02b2a660dc5d`

No new executable validation claim is created by this audit-only checkpoint. C02e Tranche 6 remains the latest executable evidence for unchanged production source.

## Mutation boundary

The only repository mutation authorized by this checkpoint is this audit file.

No production Rust source, Cargo manifest, lockfile, validation workflow, database schema/migration, runtime wiring, network behavior, Agent/bootstrap behavior, systemd/service-manager behavior, signing/credential material, cloud resource or privileged host state is changed or authorized.

## Classification

`C02F_C_DECISION_READINESS_GATE_LOCKED / CONCRETE_PROVIDER_NOT_SELECTED / FAILURE_AND_DEPLOYMENT_TOPOLOGY_REQUIRED_BEFORE_SELECTION / IN_MEMORY_REJECTED / EMBEDDED_DURABLE_SINGLE_HOST_ONLY_CONDITIONAL / TRANSACTIONAL_SQL_CONDITIONAL / TRANSACTIONAL_KV_OR_CONSENSUS_CONDITIONAL / TTL_ONLY_COORDINATION_REJECTED / MANAGED_CONDITIONAL_STORE_CONDITIONAL / SIDE_EFFECT_FENCING_REMAINS_MANDATORY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
