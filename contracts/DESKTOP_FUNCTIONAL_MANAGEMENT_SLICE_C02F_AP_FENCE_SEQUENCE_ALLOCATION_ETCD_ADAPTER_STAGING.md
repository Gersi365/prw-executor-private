# Phase 152 C02f-AP — Fence-Sequence Allocation etcd Adapter Staging Contract

## Purpose

C02f-AP materializes the bounded real `etcd-client = 0.19.0` translation layer for the already-
validated C02f-AJ within-epoch fence-sequence allocation plan.

C02f-AP does not own recovery-epoch issuance, PRWF initialization, allocation-attempt identifier
generation, retry/reissue orchestration, endpoint/client construction, security configuration, or
production authority activation. It accepts an already-created `etcd_client::KvClient` and one exact
retained `FenceSequenceAllocationPlan`.

## Exact base

- base tranche: C02f-AO
- exact validated base/head: `9f9efb84b218470197c4cf9a9611faec99d65fb9`
- base PR #60 remains draft and unmerged

## Existing retained allocation contract

C02f-AJ already defines the canonical allocator state and deterministic transaction plan:

- canonical PRWF head key: `/prw/reachability/fence-sequence/v1/head`;
- canonical immutable PRWR reservation namespace under
  `/prw/reachability/fence-sequence/v1/reservation/`;
- exact head predecessor observation containing canonical bytes and positive `mod_revision`;
- next sequence = predecessor high-water + 1, with zero and overflow rejected;
- exact 32-byte non-zero `SequenceAllocationAttemptId` retained by the caller;
- compare order:
  1. exact PRWF head `mod_revision`;
  2. exact PRWF predecessor bytes;
  3. exact PRWR reservation key `version == 0`;
- success branch:
  1. Put exact successor PRWF head;
  2. Put exact immutable PRWR reservation;
- failure branch:
  1. default-linearizable Get of exact PRWF head key;
  2. default-linearizable Get of the exact retained reservation key;
- retained re-observation classifier: `Committed`, `Superseded`, or `ProvenNotCommitted`;
- a separate one-reissue budget exists in the provider-neutral layer but is not consumed by C02f-AP.

C02f-AP translates this retained plan exactly; it does not re-plan or substitute another allocator
schema.

## Adapter construction boundary

`FenceSequenceAllocationEtcdStore` owns only an already-created `etcd_client::KvClient`.

Construction:

- performs no provider I/O;
- does not call `Client::connect`;
- does not accept or select endpoint strings;
- does not select TLS roots, certificates, keys, authentication credentials, RBAC identities,
  leases, TTLs, Watches, cluster members, DNS names, or deployment topology.

The caller remains responsible for every later runtime/security construction decision.

## Fresh head observation

The adapter exposes one default-linearizable exact-key PRWF head read so a later caller can obtain the
canonical predecessor required by the existing AJ planner.

The read must:

- use the canonical PRWF head key only;
- return zero or one key/value pair only;
- reject any returned key different from the canonical key;
- preserve provider `mod_revision` and require it to be positive;
- decode through the existing canonical AJ PRWF decoder;
- return absence as absence rather than manufacturing a bootstrap head.

C02f-AP does not interpret an absent head as permission to initialize. C02f-AM/AN/AO remain the
separate initialization boundary.

## Exact real etcd transaction translation

For one retained `FenceSequenceAllocationPlan`, C02f-AP maps the canonical compare set in order:

1. `HeadModRevision` -> etcd `mod_revision == expected` on the canonical PRWF head key;
2. `HeadExactValue` -> etcd exact value compare on the canonical PRWF head key;
3. `ReservationVersionZero` -> etcd key-version compare `== 0` on the exact retained PRWR key.

The success branch maps to exactly two Put operations in order:

1. exact successor PRWF head Put;
2. exact retained PRWR reservation Put.

The compare-failure branch maps to exactly two default-linearizable Gets in order:

1. canonical PRWF head key;
2. exact retained PRWR reservation key.

One `execute` invocation submits at most one etcd Txn.

The adapter validates the public retained-plan branch/compare shape before provider submission. A
mutated plan with another compare order/type, branch operation type, or canonical key placement fails
closed before I/O.

## Definitive transaction result handling

A definitive compare-success response is valid only when the response contains exactly two Put
operation responses. It returns `Applied`.

A definitive compare-failure response is valid only when it contains exactly two Get operation
responses in the selected order.

The returned failure observations are decoded as follows:

- PRWF head must be exactly one canonical key/value with positive `mod_revision`;
- missing PRWF head fails closed because normal allocation requires an initialized head;
- PRWR reservation may be absent or exactly one canonical key/value;
- a present reservation must use the exact retained reservation key, have positive provider
  `mod_revision`, and decode through the existing canonical AJ PRWR decoder;
- impossible cardinality, key mismatch, malformed bytes, zero epoch/sequence/attempt ID, or other
  structural inconsistency fails closed.

The decoded head plus optional reservation are passed unchanged to AJ `classify_reobservation`.
Therefore compare-failure classification remains exactly:

- exact retained reservation + compatible advanced/current head -> `Committed`;
- another attempt occupying the exact reservation -> `Superseded`;
- exact unchanged predecessor revision+bytes with no reservation -> `ProvenNotCommitted`;
- contradictory state -> hard failure.

C02f-AP does not reinterpret those states.

## Indeterminate mutation boundary

An etcd/provider/RPC error that returns no definitive Txn response is surfaced only as
`MutationIndeterminate`.

C02f-AP does not:

- immediately retransmit;
- consume the AJ reissue budget;
- generate a replacement attempt ID;
- derive a new allocation plan;
- spawn a task or detached retry;
- treat provider failure as committed authority.

A later orchestration tranche must freshly re-observe the exact retained plan before any deliberate
reissue policy can run.

## Explicit re-observation

The adapter exposes explicit fresh re-observation for one retained allocation plan.

Re-observation performs:

1. one fresh default-linearizable exact-key PRWF head Get;
2. one fresh default-linearizable exact-key Get for the retained PRWR reservation key;
3. the existing AJ `classify_reobservation` over the decoded observations.

The two reads are intentionally fail-closed under concurrent movement: an interleaving that cannot be
reconciled with the exact retained plan becomes contradictory rather than safe-to-reissue authority.

This method performs no mutation, retry, or reissue.

## Source scope

C02f-AP adds only:

- `crates/prw-control-plane/src/fence_sequence_allocation_etcd.rs`;
- `crates/prw-control-plane/tests/c02f_ap_fence_sequence_allocation_etcd_adapter.rs`;
- this contract.

C02f-AP does not modify:

- AJ `crates/prw-control-plane/src/fence_sequence.rs`;
- AM `crates/prw-control-plane/src/fence_sequence_initialization.rs`;
- AN `crates/prw-control-plane/src/fence_sequence_initialization_etcd.rs`;
- AO `crates/prw-control-plane/src/fence_sequence_initialization_orchestrator.rs`;
- `crates/prw-control-plane/src/recovery_epoch.rs`;
- `crates/prw-control-plane/src/recovery_epoch_orchestrator.rs`;
- `crates/prw-control-plane/src/recovery_epoch_spanner.rs`;
- public `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- root `Cargo.lock`;
- any `prw-remote-bridge` source.

No dependency or lockfile change is authorized by AP.

## Explicit non-goals

C02f-AP does not:

- export AP through the production public `lib.rs` surface;
- generate `SequenceAllocationAttemptId` values;
- choose the current recovery epoch;
- initialize PRWF state;
- own the allocation retry/reissue state machine;
- allocate a real production fence sequence during validation;
- create/connect an etcd client or select endpoints;
- configure TLS, authentication, RBAC, credentials, leases, TTLs, Watches, users, roles,
  permissions, or cluster membership;
- construct/contact Spanner or issue a recovery epoch;
- bridge a completed allocation into live-owner acquisition;
- activate `ReachabilityLiveOwnerAsyncAuthority`;
- execute snapshot restore or disaster recovery;
- implement R1-R4 stale-fence effect rejection;
- select deployment/runtime process lifecycle;
- deploy;
- merge any draft PR.

## Validation gate

The tranche is valid only if canonical Rust and Android workflows pass on the exact final AP head and
a fresh AO -> AP compare proves the diff remains exactly the three intended added files.

AJ allocation source, AM/AN/AO initialization source, recovery source, public `lib.rs`, Cargo manifest,
root lockfile, and remote-bridge source must remain byte-stable from the validated AO base.

Expected gate after validation:

`C02F_AP_FENCE_SEQUENCE_ALLOCATION_ETCD_ADAPTER_VALIDATED`
