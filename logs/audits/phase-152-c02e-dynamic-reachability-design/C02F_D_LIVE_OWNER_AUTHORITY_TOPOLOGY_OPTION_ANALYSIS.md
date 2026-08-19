# Phase 152 C02f-D — Live-Owner Authority Topology Option Analysis

Status: `TOPOLOGY_OPTION_ANALYSIS_COMPLETE / UI_LOCAL_AND_PROCESS_LOCAL_AUTHORITY_REJECTED / AGENT_LOCAL_AUTHORITY_CONDITIONAL / CENTRAL_OR_SHARED_AUTHORITY_ELIGIBLE / AUTHORITY_PLACEMENT_NOT_SELECTED / PROVIDER_NOT_SELECTED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`

Repository ID: `1334911207`

Active branch: `phase-152-c02e-dynamic-reachability-design`

Exact predecessor head: `4e45d8b03bdead43a115c7ad71a0211f2f1f419d`

Exact predecessor tree: `9f39272596254731abc983a06f8eac4a06982904`

Predecessor checkpoint: `C02f-C durable live-owner backend decision-readiness audit`

## Purpose

C02f-D performs repository-native topology analysis for the future durable `ReachabilityLiveOwnerAuthority` required by C02e/C02f-A.

This checkpoint narrows which authority placements are structurally compatible with already locked PRW process/transport boundaries. It does **not** select a concrete backend, database, cloud product, consensus service, control-plane deployment, region, availability-zone layout, persistence encoding, schema, migration, wire protocol or production runtime.

The goal is to separate topology choices from provider choices so a later provider-selection decision does not silently relocate authority into an incompatible process or failure domain.

## Inherited safety contract

C02f-D inherits without weakening:

- authority namespace = exact `DeviceId + TransportIdentity`;
- non-zero ordered logical `u128` fence;
- each replacement in one namespace receives a strictly newer durable fence;
- older grants remain permanently stale for authority purposes;
- stale release cannot clear a newer grant;
- restart/failover/recovery cannot roll back or reuse fencing history;
- ambiguous or unavailable authority fails closed;
- TTL/heartbeat/clock expiry is not the primary safety mechanism;
- candidate-publication freshness remains a separate authority domain;
- externally visible side effects that can race with replacement must reject stale fences at or atomically with the effect boundary.

The production seam remains provider-neutral and explicitly leaves persistence/failover to a future concrete authority implementation.

## Repository topology facts

### Ubuntu host / desktop

The locked desktop architecture establishes:

- desktop UI is a separate unprivileged process from the headless PRW Agent;
- desktop UI must not own production Agent lifecycle or host runtime authority;
- the Agent remains authoritative for host runtime state, policy enforcement, local privileged boundaries, remote capability execution and production lifecycle ownership;
- desktop-to-Agent communication uses the authenticated fixed local Unix-domain IPC boundary;
- the host must remain remotely reachable independently of desktop UI lifecycle.

The historical and current Agent evidence further proves deterministic second-instance exclusion for the Agent runtime and a long-running identity-aware service lifecycle. Therefore an authority implementation that depends on the desktop process being alive is inconsistent with locked product architecture.

### Android

The locked Android architecture establishes:

- Compose UI does not own the transport session;
- an application connection controller/native adapter owns the explicit PRW connection state machine;
- a user-visible foreground service owns persistent active connectivity where Android policy requires it;
- process death is fail-closed and reconnect reconstructs validated state and reauthenticates rather than assuming prior session authority;
- no hidden always-on daemon or boot-time remote activation is introduced.

Therefore a UI/ViewModel-local authority is not stable enough to be durable live-owner authority.

### Remote transport plane

The Phase 139 architecture locks:

- control plane remains separate from data plane;
- Agent control-plane transport is outbound TCP/TLS;
- data plane is peer-to-peer QUIC/TLS over explicit selected candidates;
- either peer may act as QUIC connection initiator according to deterministic `TransportIdentity` ordering;
- successful transport establishment does not grant application authority;
- transport identity rotation creates a new `TransportIdentity` and registry binding lifecycle.

This means live-owner authority cannot be inferred from socket direction, IP endpoint, connection initiator, selected candidate, relay route or UI process identity.

### Capability / side-effect ownership

The Phase 143 bridge locks that the bridge itself does not own filesystem, PTY, port-forward socket, QUIC endpoint or production Agent runtime effects. Existing typed capability providers own those effects.

Therefore live-owner fencing ultimately requires integration with the concrete effect-owning component(s); placing authority in a different service does not eliminate sink-side stale-fence rejection.

### Relay

The Phase 142 relay reference service is deliberately in-memory/Sans-network and owns no persistence. Relay route tokens are routing metadata, not application authorization.

The relay service is therefore not an existing durable live-owner authority candidate and relay path selection must not determine authority placement.

## Topology option analysis

### T0 — UI-local authority

Placement examples:

- desktop process/UI state;
- Android Compose/ViewModel state.

Classification: `REJECTED`

Reasons:

1. desktop UI is explicitly non-authoritative for Agent/runtime state;
2. host reachability must survive desktop UI lifecycle;
3. Android UI does not own persistent transport lifecycle;
4. UI/process restart loses authority history unless delegated to another durable authority, in which case the UI is no longer the authority;
5. UI state is presentation projection and cannot be treated as capability or ownership proof.

No future provider-selection checkpoint may place authoritative fencing state solely in UI process state.

### T1 — arbitrary process-local authority

Placement examples:

- one `prw-remote-bridge` process memory table;
- one worker-local mutex/counter;
- one client process cache.

Classification: `REJECTED`

Reasons:

- process restart loses durable fence history;
- multiple independently running owners cannot be serialized safely;
- stale owners could regain apparent currentness from reconstructed local state;
- C02f-A explicitly requires restart/failover-safe authority.

Process-local caches may exist later only as non-authoritative acceleration and must fail closed on uncertainty.

### T2 — Ubuntu Agent-local durable authority

Placement:

- one durable authority colocated with the authoritative headless Agent on one host.

Classification: `CONDITIONALLY_ELIGIBLE_FOR_A_LOCKED_SINGLE-HOST_AUTHORITY_SCOPE`

Positive architectural fit:

- Agent already owns host runtime state and capability execution;
- Agent lifecycle is independent from desktop UI;
- second-instance exclusion exists for the local Agent runtime;
- colocating authority with an effect-owning Agent can simplify some side-effect fencing boundaries.

Conditions that must be proved before selection:

1. all contenders for the exact live-owner namespace must serialize through this one Agent authority;
2. no Android/desktop/other host can independently claim the same authority namespace without consulting the Agent;
3. host loss/failover semantics are explicitly declared;
4. durable state survives process restart and the in-scope host recovery model without fence rollback;
5. any side effect outside the same durable transaction boundary still performs stale-fence rejection;
6. transport rotation and registry currentness remain separate higher-level admission authorities.

Limitation:

A per-host embedded authority cannot by itself prove cross-host failover or active/active distributed ownership. If the required authority scope includes replacement across hosts/services, T2 alone is insufficient.

C02f-D does not select T2.

### T3 — central control-plane authority

Placement:

- durable live-owner authority colocated with a future PRW control-plane service/backend.

Classification: `ELIGIBLE_TOPOLOGY_FAMILY / NOT_SELECTED`

Architectural fit:

- PRW already has a separate authenticated control plane used for signaling/session orchestration/registry coordination;
- both hosts and clients can conceptually consult a common authority independent of ephemeral endpoint/IP state;
- exact `DeviceId + TransportIdentity` is already a control-plane-visible logical identity boundary.

Required new decisions before selection:

1. whether expanding the control plane to own live-owner fencing is acceptable product architecture;
2. control-plane service replication/failover/partition model;
3. exact durable backend and linearization primitive;
4. authenticated authority-request protocol and caller authorization;
5. availability policy when the control plane is unreachable;
6. provider credentials/trust placement;
7. stale-fence propagation to data-plane effect sinks;
8. production deployment topology and recovery procedure.

Important boundary:

The current Phase 129 control-plane transport does not itself implement this authority. Treating its existence as proof of central authority would be an architecture invention.

C02f-D does not select T3.

### T4 — dedicated shared live-owner authority service

Placement:

- a distinct service whose narrow responsibility is durable live-owner acquisition/currentness/release and fencing generation.

Classification: `ELIGIBLE_TOPOLOGY_FAMILY / NOT_SELECTED`

Potential advantages:

- narrow authority trust boundary;
- clear separation from signaling/control-plane business logic;
- one shared linearization point for multiple processes/hosts;
- explicit opportunity to expose only bounded typed authority operations.

Costs/new architecture surface:

- new runtime service and deployment dependency;
- service authentication/authorization protocol;
- replication/failover/partition policy;
- credentials and operational lifecycle;
- monitoring/backup/recovery requirements;
- side-effect sinks still need stale-fence enforcement.

A dedicated service is not justified merely because it is cleanly separated. It must be compared against control-plane colocation and Agent-local placement using the intended failure scope and operational burden.

C02f-D does not select T4.

### T5 — replicated per-device/per-host authorities without one linearization domain

Placement:

- independent local stores on each contender with asynchronous synchronization;
- caches that can each declare ownership during partition.

Classification: `REJECTED_UNLESS_THE_REPLICATION_MECHANISM_ITSELF_PROVIDES_THE_REQUIRED_LINEARIZABLE_AUTHORITY`

Reasons:

- asynchronous replication alone permits split-brain fencing allocation;
- independent counters can issue non-comparable or reused generations;
- later reconciliation cannot undo stale side effects already performed.

If a future replicated design uses a consensus/transaction mechanism that provides one authoritative linearization domain, the consensus/transaction layer—not the replicas individually—is the authority and must be reviewed as such.

### T6 — hybrid shared authority plus local caches

Placement:

- T3 or T4 authoritative backend plus Agent/client caches for liveness/latency hints.

Classification: `CONDITIONALLY_ELIGIBLE / CACHE_NEVER_AUTHORITY`

Rules:

- cache hit cannot manufacture `Current` after authoritative state becomes unavailable;
- cached old grant cannot regain authority after replacement;
- acquisition and replacement always linearize at authoritative durable state;
- release remains conditional at authoritative state;
- side-effect fencing still evaluates the actual fence at the sink boundary.

This topology may be useful operationally later but cache design is not part of C02f-D provider selection.

## Cross-platform authority-placement constraints

Any selected topology must preserve the same semantics on Ubuntu, desktop and Android participation:

1. UI lifecycle is never authority lifecycle.
2. Process restart cannot reset fencing history.
3. Android process death cannot resurrect a previous owner.
4. Desktop restart cannot displace or recreate Agent authority from presentation state.
5. Network path migration or relay fallback cannot change authority namespace.
6. QUIC initiator role cannot decide who owns live-owner authority.
7. transport-key rotation creates a distinct `DeviceId + TransportIdentity` authority namespace and old transport admission remains governed by registry currentness/revocation.
8. unavailable shared authority must fail closed unless a separately reviewed topology explicitly proves a safe degraded mode.

## Side-effect topology constraint

Authority placement and side-effect placement are coupled but not identical.

C02f-D locks the following rule:

> A component may hold a valid live-owner grant only if every in-scope side-effect path it can drive is either atomically fenced with the authority state or carries the grant fence to a sink that rejects stale generations.

Consequences:

- central authority plus unfenced Agent/socket/transfer effect is insufficient;
- Agent-local authority plus remote/unfenced effect is insufficient;
- a preflight `currentness()` call followed by asynchronous work is insufficient;
- relay success or current connectivity path is insufficient;
- session/capability authorization remains separate from fencing currentness.

## Failure-model options to be locked before authority placement selection

A later topology-selection checkpoint must explicitly answer:

### F1 — process failure

Must ownership remain safe across owner process crash/restart? C02f-A already requires yes.

### F2 — host failure

Must a different host/process be able to replace ownership after one host becomes unavailable while preserving strict fencing history?

### F3 — authority-service failure

If the authority backend/service is unavailable, does the product fail closed for new/currentness-sensitive work or is a bounded degraded mode required?

Any degraded mode must not use stale cached state as authority.

### F4 — network partition

Can two partitions both continue acquiring owners? Under C02f-A the answer must be no unless a single linearizable/quorum authority prevents split brain.

### F5 — durable-storage loss/restore

What recovery evidence prevents restoring an older fencing generation from snapshot/backup or recreating a namespace at a lower generation?

### F6 — region/zone failure

Is multi-zone or multi-region continuity an initial product requirement or explicitly deferred?

Provider selection cannot answer this implicitly.

## Authority-placement decision criteria

A future C02f-E topology-selection checkpoint may select T2, T3, T4 or another equivalent topology only after documenting:

1. exact authority owner component;
2. all processes/hosts that can contend for one exact namespace;
3. failure scope F1-F6;
4. acquisition availability policy;
5. one authoritative linearization domain;
6. durable monotonic fence-history ownership;
7. exact side-effect sink(s) and stale-fence rejection path;
8. registry/current transport interaction;
9. Android/desktop/Agent lifecycle interaction;
10. operational/deployment/trust consequences;
11. why rejected topologies cannot accidentally reappear as fallback behavior.

Only after authority placement is locked should a concrete backend/provider be selected against that topology.

## Explicit non-decisions

C02f-D does not decide:

- T2 versus T3 versus T4;
- SQL versus KV versus consensus/coordination backend;
- local embedded database technology;
- cloud provider/product;
- region/AZ count;
- quorum size;
- lease/heartbeat duration;
- persistence schema or byte encoding;
- backup/restore tooling;
- wire protocol for authority requests;
- runtime process count;
- production endpoint/port;
- deployment packaging;
- credential provisioning;
- Phase 154 activation transaction.

## Production-source byte-stability baseline

This checkpoint is audit-only. The following predecessor blobs must remain unchanged:

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

No new build, rustfmt, Clippy, test, workflow dispatch or runtime validation is required for this documentation-only checkpoint. C02e Tranche 6 remains the latest executable evidence for unchanged production source.

## Mutation boundary

The only repository mutation authorized by C02f-D is this audit file.

No production Rust source, Cargo manifest, lockfile, workflow, database/schema/migration, runtime wiring, control-plane implementation, network/QUIC/ICE/STUN/TURN/relay behavior, Agent/bootstrap/systemd behavior, Android/desktop runtime, credential, deployment or privileged host state is changed or authorized.

## Classification

`C02F_D_TOPOLOGY_OPTION_ANALYSIS_COMPLETE / UI_LOCAL_REJECTED / PROCESS_LOCAL_REJECTED / AGENT_LOCAL_SINGLE_HOST_ONLY_CONDITIONAL / CENTRAL_CONTROL_PLANE_AUTHORITY_ELIGIBLE_NOT_SELECTED / DEDICATED_SHARED_AUTHORITY_ELIGIBLE_NOT_SELECTED / ASYNC_REPLICATED_SPLIT_AUTHORITY_REJECTED / SHARED_AUTHORITY_WITH_NONAUTHORITATIVE_CACHE_CONDITIONAL / AUTHORITY_PLACEMENT_REQUIRED_BEFORE_PROVIDER_SELECTION / SIDE_EFFECT_SINK_FENCING_MANDATORY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
