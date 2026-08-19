# Phase 152 C02f-I — etcd Provider Selection Readiness Audit

Status: `SELECTION_READINESS_COMPLETE / T3_SHARED_CONTROL_PLANE_LOCKED / ETCD_V3_7_RECOMMENDED_PENDING_EXPLICIT_SELECTION / PROVIDER_NOT_SELECTED / RECOVERY_HIGH_WATER_PROOF_REQUIRED / LINEARIZABLE_KV_ONLY_FOR_AUTHORITY / WATCH_NOT_AUTHORITY / LEASE_TTL_NOT_SAFETY / TLS_AUTH_REQUIRED_IF_SELECTED / CLIENT_LIBRARY_UNSELECTED / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `8cb7777690a4fbcd93cb5722de45292593103685`
Exact predecessor tree: `7fe03419fa8395d99e7e723123f4e69f8eb0d73f`
Predecessor checkpoint: `C02f-H shared control-plane backend provider evaluation audit`
Review date: `2026-08-19`

## Purpose

C02f-H evaluated etcd v3.7, PostgreSQL 18 and CockroachDB v26.2 against the already selected T3 shared control-plane live-owner authority placement. It classified etcd v3.7 as the preferred candidate for a later provider-selection review while explicitly leaving provider selection unlocked.

C02f-I closes the remaining provider-selection-readiness questions that can be answered without choosing a concrete deployment, dependency, key encoding, client implementation or production topology.

This checkpoint does **not** select etcd, install etcd, add a Rust client dependency, create an etcd cluster, choose a member count, choose an AZ/region layout, create credentials, define a production endpoint, define the external fence encoding, mutate production Rust source, or activate any runtime/network behavior.

## Inherited architecture and safety locks

The following are already authoritative and are not reopened here:

- T3 shared control-plane authority is selected;
- cross-host replacement is required;
- standalone T2 Agent-local authority is eliminated for the initial architecture;
- T4 dedicated authority service is not selected;
- live-owner namespace is exact `DeviceId + TransportIdentity`;
- every replacement must receive a strictly newer non-zero logical `u128` fence;
- authority ambiguity or unavailability fails closed;
- stale release cannot clear newer authority;
- restart/failover/recovery cannot make an older fence authoritative again;
- clocks, TTLs and heartbeats are not primary stale-owner safety authority;
- any racing reachability side effect must reject a stale fence at, or atomically with, its effect boundary;
- candidate-publication freshness remains a separate authority domain.

## Exact-head repository evidence

At the C02f-H predecessor head:

- `crates/prw-control-plane/Cargo.toml` contains only the existing `prw-core` dependency and no provider client;
- `Cargo.lock` remains at blob `4d69f7c6ec5a779615595f7dac8e02b2a660dc5d` before this audit;
- the live-owner seam remains provider-neutral;
- no etcd client, endpoint, credential, cluster configuration, schema or runtime adapter has been materialized.

Therefore C02f-I is a pure selection-readiness audit, not implementation.

## Current official etcd baseline

Official etcd documentation reviewed on `2026-08-19` identifies v3.7 as the latest stable documentation line and v3.8 as draft.

Primary source:

- `https://etcd.io/docs/v3.7/`

Selection review must target the stable v3.7 line unless a later explicit checkpoint re-evaluates a newer stable release.

## 1. Linearization primitive

### Requirement

C02f-A requires an operation safety-equivalent to:

`observe current -> allocate strictly newer fence -> atomically install replacement -> return installed grant`

with no split read/write race.

### etcd v3.7 evidence

Official etcd v3.7 API documentation states that KV API calls are durable and strictly serializable. Transaction comparisons and their success/failure operation blocks are applied atomically.

Primary sources:

- `https://etcd.io/docs/v3.7/learning/api_guarantees/`
- `https://etcd.io/docs/v3.7/learning/api/`

### PRW interpretation

If etcd is selected, the authority adapter must express ownership acquisition/replacement as one authoritative KV transaction whose comparison is against the exact current namespace record and whose successful branch writes the newer authoritative state.

A caller-side read followed by a separate unconditional put is forbidden.

Classification:

`MEETS_REQUIRED_PRIMITIVE_IF_IMPLEMENTED_AS_ATOMIC_KV_TXN`

## 2. Authoritative read mode

Official etcd documentation distinguishes default linearizable reads from lower-cost serializable reads that may be stale with respect to quorum.

For PRW:

- currentness-sensitive reads must use the linearizable/strictly-serializable KV path;
- a serializable/stale member-local read must never return `Current`;
- a stale or cached value may be used only for non-authoritative diagnostics/telemetry where explicitly safe.

Classification:

`LINEARIZABLE_KV_REQUIRED / SERIALIZABLE_STALE_READS_FORBIDDEN_FOR_CURRENTNESS`

## 3. Watch semantics

Official etcd v3.7 documentation explicitly does not guarantee linearizability for Watch operations. Watches are ordered and resumable within their retained history window, but delivery may be delayed.

Therefore:

- Watch may be used for cache invalidation, wakeup, cleanup or observability;
- Watch must not be the proof that a grant is current;
- after a Watch event, any authority-sensitive action still requires an authoritative transaction/read or sink-side fence check.

Classification:

`WATCH_ADVISORY_ONLY / WATCH_NOT_AUTHORITY`

## 4. Lease and TTL semantics

etcd exposes a Lease API and TTL-based key expiry. C02f-A already forbids clocks/TTL/heartbeat cadence as primary stale-owner safety authority.

Therefore, if etcd is selected:

- a lease may later support liveness, garbage collection or bounded abandoned-owner cleanup;
- lease expiry must not be the event that makes an older fence current again;
- live-owner replacement safety must remain monotonic-fence + authoritative transaction + sink-side stale-fence rejection;
- loss or extension of a lease cannot authorize stale side effects.

Classification:

`LEASE_OPTIONAL_FOR_LIVENESS / LEASE_TTL_NOT_SAFETY`

## 5. Quorum and partition behavior

Official etcd documentation describes a majority/quorum model. A minority partition is unavailable; a majority side can continue. Permanent loss of quorum prevents new consensus until quorum is recovered or disaster recovery is performed.

Primary sources:

- `https://etcd.io/docs/v3.7/faq/`
- `https://etcd.io/docs/v3.7/op-guide/recovery/`

This matches the locked PRW partition rule provided the adapter does not introduce fallback authority outside the cluster.

Required PRW behavior:

- no quorum => acquisition/replacement/currentness-sensitive operations fail closed;
- minority partition => no independent owner issuance;
- no local Agent cache or stale etcd read may substitute for quorum authority;
- cross-host replacement is permitted only through the surviving authoritative quorum.

Classification:

`QUORUM_MODEL_COMPATIBLE_WITH_T3_FAIL_CLOSED`

## 6. Fence generation must remain PRW-owned logical state

etcd revisions are ordered cluster metadata, but C02e/C02f define `ReachabilityLiveOwnerFence` as a PRW logical non-zero `u128` generation whose external representation is still unselected.

C02f-I does not authorize using:

- etcd global revision;
- key version;
- create revision;
- mod revision;
- lease ID;
- member ID

as the PRW live-owner fence merely because those values are monotonic in some provider scope.

The future authoritative record must contain enough PRW-owned state to allocate and preserve a strictly newer logical fence for each exact `DeviceId + TransportIdentity` namespace.

Classification:

`PROVIDER_REVISION_NOT_PRW_FENCE / EXTERNAL_U128_ENCODING_STILL_UNSELECTED`

## 7. Disaster recovery is the principal residual safety gate

Official etcd v3.7 disaster-recovery documentation supports snapshot/restore and documents revision-difference concerns. It provides revision bumping and compaction marking for ecosystems whose clients depend on etcd revisions.

Primary source:

- `https://etcd.io/docs/v3.7/op-guide/recovery/`

For PRW, provider revision bumping alone does not prove live-owner fence monotonicity because the PRW fence is its own logical namespace generation.

A restored snapshot can be older than fences that were issued after that snapshot. Therefore a naive restore could resurrect an older PRW `last_fence` value and violate the permanent stale-owner rule.

Before any production selection/implementation is considered complete, a separate recovery design must prove one of the following safety-equivalent properties:

1. restored authority state contains a PRW high-water value strictly at or above every fence ever issued for each namespace; or
2. an independent durable monotonic high-water authority survives the restore and forces allocation above all historical fences; or
3. affected namespaces remain fail-closed and cannot issue a new grant until an explicitly reviewed recovery procedure re-establishes a non-reusable fence range.

It is insufficient to restore the latest available snapshot and simply continue incrementing from the restored value.

Classification:

`RECOVERY_HIGH_WATER_PROOF_REQUIRED_BEFORE_PRODUCTION_AUTHORITY`

## 8. Ambiguous request outcome

C02f-A requires ambiguous authority outcomes to fail closed.

For a client request that times out, disconnects or loses its response after submission, the caller must not assume that an ownership transaction definitely failed merely because no response was received.

Future adapter behavior must surface a bounded state such as `RecoveryRequired` / `Indeterminate` and re-read authoritative state before retrying any transition whose first execution may have committed.

Automatic retry is safe only when its idempotency/compare semantics prove that a duplicate request cannot allocate or install an unintended additional generation.

Classification:

`INDETERMINATE_RESULT_MUST_FAIL_CLOSED_AND_RECONCILE`

## 9. Security/trust boundary

Official etcd documentation supports TLS for client/server and peer communication and authentication/RBAC, but those protections are not automatically equivalent to PRW's desired production policy and are not all enabled by default.

Primary sources:

- `https://etcd.io/docs/v3.7/op-guide/authentication/`
- stable transport-security material corresponding to the v3.7 documentation line.

If etcd is later selected, production implementation must require an explicitly reviewed security profile including:

- encrypted client-to-member transport;
- authenticated client identity;
- authenticated/encrypted peer transport;
- least-privilege authority credentials scoped to the PRW live-owner keyspace;
- credential rotation/revocation procedure;
- no unauthenticated authority endpoint;
- explicit decision for storage-at-rest protection at the host/storage layer if required by the deployment trust model.

C02f-I does not select certificate authority, secret manager, certificate lifetime, RBAC subject naming or storage encryption mechanism.

Classification:

`SECURITY_CAPABILITIES_ELIGIBLE / PRODUCTION_SECURITY_PROFILE_STILL_REQUIRES_DESIGN`

## 10. Cluster topology remains deployment-unselected

etcd quorum semantics are compatible with T3, but C02f-G did not select:

- three vs five members;
- AZ placement;
- region placement;
- self-hosted vs managed infrastructure;
- control-plane process colocation;
- backup storage target;
- RPO/RTO;
- disaster-recovery operator/process.

Provider selection does not silently choose these deployment semantics.

Classification:

`PROVIDER_CAN_BE_SELECTED_BEFORE_EXACT_DEPLOYMENT_TOPOLOGY / PRODUCTION_DEPLOYMENT_REQUIRES_SEPARATE_LOCK`

## 11. Rust client/dependency remains unselected

The server/provider choice and the in-repository client-library choice are separate decisions.

C02f-I does not add or choose:

- `etcd-client` or any other Rust crate;
- raw generated gRPC/protobuf client code;
- tonic transport settings;
- retry middleware;
- connection pooling;
- endpoint discovery;
- TLS implementation details.

A later implementation-readiness checkpoint must review the exact client library/version/API before adding it to `prw-control-plane`.

Classification:

`CLIENT_LIBRARY_UNSELECTED`

## 12. Keyspace and external encoding remain unselected

C02f-I does not select the concrete key or value encoding.

Future design must preserve:

- exact namespace `DeviceId + TransportIdentity`;
- unambiguous canonical encoding of both identity components;
- PRW logical non-zero `u128` fence;
- current grant identity sufficient for conditional release/currentness;
- bounded versioning/migration semantics;
- no collision with candidate freshness or registry state.

No path prefix, byte order, protobuf schema, JSON schema, CBOR representation or decimal representation is authorized here.

Classification:

`KEY_VALUE_SCHEMA_UNSELECTED`

## 13. Side-effect fencing remains independent of provider selection

Selecting etcd would only decide where ownership is linearized. It does not fence asynchronous effects by itself.

The C02f-F future reachability sink classes remain:

- R1 actual UDP transmission of traversal datagrams;
- R2 traversal timer/check/retransmission task ownership;
- R3 QUIC connection establishment/acceptance/retirement;
- R4 current reachability connection/task registry mutation.

Each racing sink must either participate in an atomic authority/effect boundary or consume a fence and reject stale generations at the sink.

Classification:

`SIDE_EFFECT_FENCING_STILL_MANDATORY`

## Selection-readiness matrix

| Criterion | etcd v3.7 status | PRW consequence |
|---|---|---|
| Shared cross-host authority | Eligible | Fits T3 |
| Atomic compare/replace | Meets via KV Txn | Required implementation pattern |
| Strong authoritative reads | Meets via linearizable KV | Stale serializable reads forbidden for currentness |
| Partition split-brain prevention | Meets under quorum | No quorum/minority fails closed |
| Monotonic PRW `u128` fence | Not supplied directly | PRW-owned value/state required |
| Stale release isolation | Implementable via conditional Txn | Must compare exact current grant/fence |
| Watch currentness authority | Does not meet | Watch advisory only |
| TTL as primary safety | Forbidden by PRW | Lease only optional liveness |
| Snapshot restore monotonicity | Residual risk | PRW high-water recovery proof mandatory |
| Auth/TLS capability | Available | Explicit secure production profile required |
| Rust client | Not part of server provider | Separate review required |
| Effect-sink fencing | Not supplied by backend | R1-R4 obligation remains |

## C02f-I decision

C02f-I concludes:

`ETCD_V3_7_IS_SELECTION_READY_AS_THE_RECOMMENDED_T3_PROVIDER_CANDIDATE_SUBJECT_TO_EXPLICIT_ARCHITECTURE_SELECTION`

This means the current evidence is sufficient to put etcd v3.7 in front of the architecture owner for an explicit provider lock.

It does **not** mean etcd is selected.

The recommendation is conditioned on preserving all of the following non-negotiable rules after selection:

1. authority mutations use strict/linearizable KV transaction semantics;
2. serializable stale reads never prove currentness;
3. Watch never proves currentness;
4. Lease/TTL never becomes stale-owner safety authority;
5. PRW owns the logical `u128` fence rather than aliasing an etcd revision;
6. no quorum/ambiguity fails closed;
7. indeterminate request outcomes reconcile against authoritative state before unsafe retry;
8. snapshot/restore cannot resume authority until PRW fence high-water monotonicity is proven;
9. production transport/authentication is explicitly secured;
10. side-effect sinks R1-R4 remain fenced independently of backend acquisition.

## Exact approval boundary for the next checkpoint

A provider lock requires an explicit architecture approval materially equivalent to:

> Select etcd v3.7 as the T3 shared control-plane live-owner authority backend. Preserve fail-closed ambiguity/no-quorum behavior, linearizable KV transaction authority, PRW-owned monotonic `u128` fencing, recovery high-water safety and sink-side stale-fence rejection. Client library, schema/encoding, cluster deployment and runtime activation remain deferred.

Without that explicit selection, implementation/provider dependency mutation remains closed.

## Production mutation boundary

C02f-I is audit-only.

It does not authorize modification of:

- `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- `crates/prw-remote-bridge/src/reachability_live_owner.rs`;
- `crates/prw-remote-bridge/src/reachability_owner.rs`;
- any `Cargo.toml` or `Cargo.lock`;
- Agent runtime;
- network listeners;
- deployment manifests;
- secret/configuration stores;
- database/provider endpoints.

No build, rustfmt, Clippy, test or workflow execution is required for this audit-only checkpoint because production executable source must remain byte-stable.

## Final classification

`C02F_I_SELECTION_READINESS_COMPLETE / T3_SHARED_CONTROL_PLANE_LOCKED / ETCD_V3_7_RECOMMENDED_PENDING_EXPLICIT_SELECTION / PROVIDER_NOT_SELECTED / LINEARIZABLE_KV_TXN_REQUIRED / SERIALIZABLE_STALE_READ_FORBIDDEN_FOR_CURRENTNESS / WATCH_ADVISORY_ONLY / LEASE_TTL_NOT_SAFETY / PRW_U128_FENCE_PROVIDER_INDEPENDENT / RECOVERY_HIGH_WATER_PROOF_REQUIRED / INDETERMINATE_RESULTS_FAIL_CLOSED / SECURITY_PROFILE_REQUIRED_IF_SELECTED / CLIENT_LIBRARY_UNSELECTED / KEY_VALUE_SCHEMA_UNSELECTED / R1_R4_SIDE_EFFECT_FENCING_REQUIRED / PRODUCTION_SOURCE_BYTE_STABLE / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
