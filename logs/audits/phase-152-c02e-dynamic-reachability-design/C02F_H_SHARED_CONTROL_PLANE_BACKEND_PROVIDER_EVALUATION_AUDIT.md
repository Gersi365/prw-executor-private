# Phase 152 C02f-H — Shared Control-Plane Backend Provider Evaluation Audit

Status: `PROVIDER_EVALUATION_COMPLETE / T3_SHARED_CONTROL_PLANE_SCOPE_LOCKED / ETCD_V3_7_PREFERRED_FOR_SELECTION_REVIEW / COCKROACHDB_V26_2_ELIGIBLE / POSTGRESQL_18_ELIGIBLE_WITH_EXTERNAL_HA_DEPENDENCY / PROVIDER_SELECTION_NOT_LOCKED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `8c1da97354ac4b752215ce81a1e75227aff1bb64`
Exact predecessor tree: `7f5cad6b1c95a972417b9fd898913f1bbede66c0`
Predecessor checkpoint: `C02f-G shared control-plane live-owner authority placement lock`
Review date: `2026-08-19`

## Purpose

C02f-G selected T3: one shared control-plane authority domain, cross-host replacement required, ambiguity/unavailability fail closed, provider/backend deferred.

C02f-H performs the now-authorized provider/backend evaluation without selecting or installing a provider. It compares supported/current implementation families against the locked C02f-A safety contract and C02f-G topology.

The evaluation is deliberately limited to three concrete candidates with documented transactional/consensus semantics:

1. etcd v3.7;
2. PostgreSQL 18;
3. CockroachDB v26.2 supported release line.

CockroachDB v26.3 testing releases are not treated as production candidates because official release material classifies them as testing/not production-qualified as of this review date.

## Source policy

Provider claims in this audit were checked against official primary documentation only on 2026-08-19.

Reviewed source families:

- etcd v3.7 API guarantees, transaction API and disaster-recovery documentation;
- PostgreSQL 18 transaction isolation, explicit locking, warm-standby/synchronous replication, failover and numeric-type documentation;
- CockroachDB supported-release overview, transaction/serializability documentation, transaction-layer/locking documentation and replication/Raft documentation.

This audit does not rely on third-party benchmark, blog, forum or vendor-comparison claims.

## Locked T3 requirements used for evaluation

Every candidate must support a design that preserves all of the following:

1. exact namespace `DeviceId + TransportIdentity`;
2. one shared linearization history for every contender of one exact namespace;
3. strictly increasing, non-zero logical `u128` fencing generation;
4. atomic replacement/install of the new current grant;
5. permanent stale-owner rejection;
6. stale release isolation;
7. fail-closed unavailable/ambiguous authority semantics;
8. cross-host replacement after previous host loss or partition;
9. partition behavior that does not permit split authoritative histories;
10. recovery that never makes an older fence current again or reuses it;
11. bounded indeterminate mutation handling;
12. authenticated control-plane access;
13. no TTL/lease/heartbeat dependency as primary stale-owner safety authority;
14. future propagation/enforcement of the fence at R1-R4 effect boundaries.

A candidate is not accepted merely because it offers persistence. The decisive primitive is a durable shared linearization boundary whose failure semantics can be reconciled with the PRW fencing contract.

## Candidate A — etcd v3.7

Classification: `ELIGIBLE / SEMANTIC_FIT_HIGHEST / PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

### Relevant official semantics

Official etcd v3.7 API guarantees document a KV API with durability and strict serializability. Atomic KV operations participate in one total order consistent with real-time ordering. Default range reads are linearizable; the optional serializable/member-local read mode may return stale data.

The v3 transaction API provides atomic `If / Then / Else` transactions with comparisons over key state such as value/version/create revision/modification revision. This directly supports compare-and-swap style ownership replacement.

The cluster uses quorum-based consensus. Loss of quorum prevents new authoritative updates rather than allowing independent writable histories, which aligns naturally with PRW fail-closed safety.

### PRW-compatible authority shape

A provider-neutral PRW design on etcd can be expressed conceptually as:

1. key by canonical exact peer namespace;
2. perform a linearizable authoritative read or an atomic transaction against current record state;
3. derive/allocate the next logical PRW fence under the same authoritative transaction boundary;
4. conditionally install the new owner/fence record;
5. return success only after the transaction commits;
6. on timeout/transport ambiguity, treat result as indeterminate and recover via authoritative linearizable state before permitting side effects.

The exact record layout, fence encoding and client library remain unselected.

### Important prohibitions

#### Do not use stale/serializable reads for currentness

The etcd `serializable` range-read option trades consistency for availability by permitting a member-local stale read. It is therefore unsuitable as live-owner currentness authority.

Authority reads must use semantics that preserve the shared linearization history.

#### Do not use Watch as authority currentness

etcd Watch is useful for ordered/reliable change delivery, but the documented Watch interface is not itself a linearizable currentness decision boundary. A watch notification/cache cannot replace an authoritative transaction/read when deciding whether a grant is current.

#### Do not use Lease/TTL as primary safety

etcd lease facilities may later support liveness/cleanup, but C02f-A forbids clock/TTL expiry as the primary stale-owner safety authority. Correctness must come from monotonic fencing and authoritative conditional mutation.

#### Do not substitute etcd revision for the PRW fence without a separate proof

etcd exposes a monotonic 64-bit revision in normal cluster operation. PRW has already locked a logical non-zero `u128` fencing generation and stronger recovery requirements.

C02f-H therefore does not authorize using raw etcd revision as `ReachabilityLiveOwnerFence`. A later representation checkpoint must decide whether the PRW fence is stored as application data and how monotonicity survives restore/recovery.

### Ambiguous mutation outcome

A client can lose connectivity or time out without knowing whether a modifying request committed. This maps directly to the existing PRW `UnavailableOrAmbiguous` / recovery-required model.

The safe client contract must never translate timeout into `not committed` or `current`. It must reload authoritative state and resolve the exact namespace before issuing currentness-sensitive effects.

### Recovery caveat

Official etcd disaster-recovery documentation warns that restoring from snapshot can move the cluster revision backwards. Revision-bump facilities exist for restoring revision continuity, but a chosen bump value does not by itself prove preservation of PRW's last-issued logical `u128` fence.

Therefore a production etcd design still requires a reviewed PRW recovery/high-water procedure. Snapshot rollback must never allow an older stored owner/fence to become valid or a previously issued fence to be reissued.

### Deployment/auth footprint

A real etcd deployment introduces a consensus cluster, authenticated client access, certificates/credentials, endpoint discovery/configuration, snapshot/restore operations and quorum monitoring.

Those are material deployment choices and are not authorized by this audit.

### Evaluation

Strengths:

- narrow KV authority model closely matches PRW ownership state;
- strict-serializable KV semantics;
- atomic transaction/CAS primitive;
- consensus/quorum behavior naturally supports one T3 linearization domain;
- no need to rely on leases or wall clocks for safety;
- small conceptual schema surface compared with a general distributed SQL database.

Residual proof obligations:

- exact `u128` application representation and monotonic allocation;
- ambiguous-request recovery protocol;
- snapshot/restore high-water preservation;
- authentication/credential/deployment topology;
- R1-R4 fence propagation.

Result: `PREFERRED_FOR_SELECTION_REVIEW`, not selected.

## Candidate B — CockroachDB v26.2

Classification: `ELIGIBLE / STRONG_DISTRIBUTED_SQL_FIT / HIGHER_OPERATIONAL_AND_SCHEMA_SCOPE / NOT_SELECTED`.

### Version boundary

Official CockroachDB release documentation lists v26.2 as a supported Regular release line. v26.3 material available on the review date is testing/not qualified for production and is therefore excluded from the production candidate set.

### Relevant official semantics

CockroachDB runs transactions at `SERIALIZABLE` isolation by default. Its distributed write path is backed by Raft replication and majority acknowledgement under the normal replication model.

A live-owner record can therefore be updated through a serializable transaction with conditional state checks and retry handling.

### PRW-compatible authority shape

A provider-neutral design could use a canonical namespace row and a serializable transaction that:

1. reads the exact current owner/fence record;
2. computes a strictly newer PRW logical fence;
3. conditionally replaces current ownership;
4. commits the transaction;
5. reports current ownership only after committed success.

Serialization/retry failures must be handled explicitly and boundedly.

### Retry and ambiguity requirements

Serializable distributed transactions may require client retries. A PRW adapter must distinguish a definite retryable serialization failure from an indeterminate transport/commit outcome.

Blindly replaying a mutation after an ambiguous outcome is unsafe unless the operation is made idempotent or authoritative state is first recovered.

### Do not rely on explicit row-lock survivability for correctness

CockroachDB documentation describes explicit locking behavior and transaction-layer details where lock durability/survival is not the primitive that should define application correctness in all modes/configurations.

PRW correctness must derive from committed serializable transaction semantics/conditional update and persistent fence state, not from the assumption that an explicit lock survives every replica/node failure.

### Recovery and cross-host behavior

Consensus replication and serializable transactions make CockroachDB structurally compatible with T3 shared authority. However backup/restore, cluster recovery, multi-region topology and surviving historical fence high-water still require a PRW-specific proof.

A stale backup restore cannot be accepted simply because the database normally supplies serializable transactions.

### Representation

Distributed SQL gives several possible exact representations for a logical `u128` fence, but C02f-H does not select a column type, decimal/string/bytes mapping, endianness, schema or migration.

The representation must preserve total ordering, the full non-zero `u128` domain required by the PRW contract, and recovery monotonicity.

### Deployment/auth footprint

CockroachDB introduces a full distributed SQL cluster, SQL schema/migrations, authenticated DB access, certificates/credentials, node/range replication topology, backup/restore and operational management.

For the narrowly scoped live-owner authority workload, this is a broader product/deployment surface than a KV authority store.

### Evaluation

Strengths:

- distributed serializable transactions;
- consensus-replicated shared authority domain;
- natural multi-client/cross-host serialization;
- expressive schema/transaction model.

Residual costs/risks:

- broader database and schema/migration surface;
- mandatory transaction retry discipline;
- ambiguous outcome recovery still required;
- explicit row locks must not become correctness dependency;
- backup/restore monotonic-fence proof still required;
- unsupported/testing release lines must not be used for production.

Result: `ELIGIBLE`, below etcd for the narrow live-owner authority use case; not selected.

## Candidate C — PostgreSQL 18

Classification: `ELIGIBLE_WITH_EXTERNAL_HA_FENCING_AND_DURABILITY_TOPOLOGY / NOT_SELECTED`.

### Relevant official semantics

PostgreSQL 18 supports transactions including `SERIALIZABLE` isolation and explicit row locking such as `SELECT ... FOR UPDATE`. These primitives can serialize concurrent updates to an authority row on one current primary.

PostgreSQL therefore has a viable transactional primitive for the record-level acquire/replace/release operation itself.

### Cross-host HA limitation relevant to T3

The C02f-G requirement is not only transaction correctness on one primary; it requires a single safe shared authority across host/process failure and cross-host replacement.

Official PostgreSQL failover documentation explicitly warns that after promoting a standby, a mechanism such as STONITH is necessary to ensure the old primary does not return while both systems believe they are primary. PostgreSQL itself does not provide the entire external failure-detection/failover orchestration system.

This makes the correctness of a self-managed PostgreSQL T3 topology depend on additional HA/fencing infrastructure beyond the database engine.

### Replication durability

Streaming replication is asynchronous by default and may lose recently committed transactions on failover. Synchronous replication can strengthen durability by waiting for required standby acknowledgement, but introduces explicit availability/latency and topology choices.

For PRW live-owner fencing, losing an acknowledged latest fence during failover can resurrect an older generation/state and is unacceptable unless the chosen topology/procedure proves monotonicity.

Bare default asynchronous replication is therefore insufficient as the cross-host safety story.

### Transaction retries

Serializable PostgreSQL transactions can fail with serialization errors and must be retried according to bounded application logic.

As with the other candidates, network failure after commit may leave the caller uncertain. An ambiguous result must trigger authoritative recovery, not side effects under assumed ownership.

### Representation

PostgreSQL `numeric/decimal` supports exact selectable precision and is one technically plausible way to preserve the value range of a logical `u128`. Byte/string representations are also conceivable.

C02f-H selects none of them. Schema, ordering semantics, check constraints, migration and wire mapping remain separate decisions.

### Deployment/auth footprint

A robust T3 design using PostgreSQL requires not only the database but also an explicitly reviewed HA topology: primary/standby selection, synchronous durability policy, failure detection, safe promotion, old-primary fencing/STONITH, credential ownership, routing/endpoint policy, backup/restore and recovery.

A managed PostgreSQL service could package some of this, but that would be a distinct provider/product evaluation and cannot be inferred from PostgreSQL engine semantics alone.

### Evaluation

Strengths:

- mature transactional row-level primitives;
- serializable isolation available;
- straightforward authority-table model;
- exact high-precision numeric representation is technically possible.

Residual costs/risks:

- safe distributed failover is not supplied by core PostgreSQL alone;
- asynchronous replication can lose acknowledged state;
- synchronous topology/STONITH/failover orchestration must be selected and proven;
- larger external HA dependency chain for the specific T3 requirement;
- ambiguous outcome and stale restore protections still required.

Result: `ELIGIBLE_WITH_EXTERNAL_HA_DEPENDENCY`; not preferred over consensus-native candidates for this narrowly scoped authority; not selected.

## Comparative matrix

| Criterion | etcd v3.7 | CockroachDB v26.2 | PostgreSQL 18 |
| --- | --- | --- | --- |
| Shared multi-client linearization | strong KV strict-serializable domain | strong distributed SERIALIZABLE transactions | strong on current primary; HA topology external |
| Native conditional mutation | atomic Txn/CAS | serializable conditional transaction | transaction + row/conditional update |
| Consensus-native cross-host authority | yes | yes | not by engine alone |
| Quorum/partition fail-closed fit | strong | strong | depends on HA/failover design |
| Client retry discipline | required | required | required |
| Ambiguous commit recovery | required | required | required |
| TTL/lease needed for safety | no | no | no |
| PRW u128 representation selected | no | no | no |
| Stale backup/restore proof still required | yes | yes | yes |
| Operational/schema surface for narrow authority | lowest of compared distributed candidates | high | medium plus external HA |
| R1-R4 fence propagation solved by provider | no | no | no |
| C02f-H result | preferred for selection review | eligible | eligible with external HA dependency |

## Why etcd is preferred for the next selection checkpoint

C02f-H's preference is architectural fit, not a final provider decision.

The live-owner authority is a small keyed state machine whose core operations are conditional current-state observation, monotonic replacement, currentness and conditional release. etcd's strict-serializable KV domain and atomic comparison transaction directly match this shape without requiring the project to adopt a general SQL schema/runtime as the authority abstraction.

It also makes the selected T3 shared linearization domain explicit through consensus/quorum semantics.

That advantage does **not** remove the hardest PRW-specific obligations: fence representation, monotonic recovery after restore, ambiguous-result reconciliation, credentials/deployment topology and R1-R4 stale-fence enforcement.

Therefore the correct classification is:

`ETCD_V3_7_PREFERRED_FOR_SELECTION_REVIEW`, not `ETCD_SELECTED`.

## Provider-independent implementation constraints now clarified

Regardless of which eligible provider is later selected:

### Acquire/replace

A grant may be returned only after one authoritative atomic/serializable mutation has installed the exact new owner and strictly newer logical fence.

### Currentness

Currentness must be obtained from the authoritative linearization domain. Stale caches, watches/changefeeds, member-local stale reads, replica-local reads or previously open transport connections cannot substitute.

### Indeterminate result

Timeout/disconnect/unknown commit outcome enters recovery-required state. The caller reloads authoritative state before emitting owner-sensitive effects.

### Release

Release must condition on the exact current owner/fence. A delayed stale release becomes a no-op/stale result and cannot erase a newer owner.

### Recovery

Restore/failover cannot reduce the logical last-issued fence. If high-water cannot be proven, the namespace/domain fails closed until reviewed recovery re-establishes a safe generation history.

### Effect sinks

Provider correctness at acquisition does not automatically fence R1-R4. The generation must still reach actual traversal send/task/QUIC/current-task boundaries or be atomically coupled to an equivalent sink-side authority mechanism.

## Explicit rejections from this evaluation

C02f-H rejects the following designs as insufficient for the locked T3 safety contract:

- etcd member-local stale/`serializable` reads as currentness authority;
- etcd Watch/cache state as currentness authority;
- etcd Lease/TTL expiry as primary stale-owner safety;
- directly equating etcd's 64-bit revision with the PRW `u128` fence without a representation/recovery proof;
- treating snapshot restore that rewinds authority state as safe without PRW high-water recovery;
- bare PostgreSQL asynchronous replication as sufficient preservation of acknowledged fencing state across failover;
- PostgreSQL HA that omits old-primary fencing/STONITH or equivalent split-brain prevention;
- relying on CockroachDB explicit row-lock survivability instead of committed serializable transaction semantics;
- using CockroachDB v26.3 testing releases as a production provider candidate at this review date;
- treating any provider's TTL, local cache, session, endpoint or transport connection as ownership identity/authority;
- assuming provider selection alone solves R1-R4 side-effect fencing.

## Next decision gate

The architecture is now sufficiently narrow for one explicit provider lock.

The next provider-selection checkpoint may choose **etcd v3.7** if the project approves its consensus-cluster/deployment/credential footprint and accepts the required PRW-specific recovery/high-water design.

Alternatively it may choose CockroachDB v26.2 or a fully specified PostgreSQL 18 HA topology if a broader database surface is justified.

Generic continuation after C02f-H may prepare a provider-selection decision package and representation/recovery design, but must not add a dependency, schema, cluster, credential, listener or runtime implementation until the provider selection is explicitly locked.

## Production-source byte-stability baseline

The following C02f-G predecessor blobs must remain unchanged by this audit-only checkpoint:

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

The only repository mutation authorized by C02f-H is this audit file.

No production Rust source, Cargo/workflow, dependency, schema/migration, database/cluster, control-plane listener, network socket, Agent/bootstrap/systemd behavior, desktop/Android runtime, credential, cloud resource, deployment or privileged host state is modified or activated.

## Classification

`C02F_H_SHARED_CONTROL_PLANE_BACKEND_PROVIDER_EVALUATION_COMPLETE / ETCD_V3_7_PREFERRED_FOR_SELECTION_REVIEW_NOT_SELECTED / COCKROACHDB_V26_2_ELIGIBLE_NOT_SELECTED / POSTGRESQL_18_ELIGIBLE_WITH_EXTERNAL_HA_DEPENDENCY_NOT_SELECTED / STALE_READ_WATCH_TTL_NOT_AUTHORITY / AMBIGUOUS_OUTCOME_RECOVERY_REQUIRED / STALE_RESTORE_HIGH_WATER_PROOF_REQUIRED / R1_R4_FENCE_PROPAGATION_STILL_REQUIRED / PROVIDER_SELECTION_NOT_LOCKED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
