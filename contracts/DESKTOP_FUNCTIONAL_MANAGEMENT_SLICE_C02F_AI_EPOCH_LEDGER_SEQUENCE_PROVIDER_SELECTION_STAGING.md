# Phase 152 C02f-AI — Epoch Ledger / Sequence High-Water Provider Selection Contract

Status: `ARCHITECTURE_SELECTION_STAGED / GOOGLE_CLOUD_SPANNER_GOOGLESQL_LEDGER_SELECTED / EXTERNAL_CONSISTENCY_REQUIRED / APPEND_ONLY_EPOCH_HISTORY / STRONG_REOBSERVATION / ONE_BOUNDED_EXACT_REISSUE / ETCD_SEQUENCE_HIGH_WATER_SELECTED / EXACT_HEAD_PLUS_RESERVATION_TXN / U128_FENCE_ENCODING_PRESERVED / DOCS_ONLY / NO_CLOUD_RESOURCE / NO_SCHEMA_MATERIALIZATION / NO_CARGO_MUTATION / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative predecessor validation head: `2dd0b712aaec2605608cd4c8452bcb162ed516f2`
Predecessor PR: `#53` (`open / draft / unmerged`)
Predecessor canonical Rust validation: run `#760` / run ID `32389664346` — PASS
Predecessor Drive PASS evidence: `1SUkAx6i-YmFedXaz1obNw02ahjIXnE4r`
Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This contract advances the recovery architecture boundary selected by C02f-AH.

It selects:

1. the concrete external durable provider for global recovery-epoch authority;
2. the provider-level epoch issuance and indeterminate-result re-observation contract;
3. the durable placement and exact transaction semantics for the within-epoch sequence high-water.

It does not create a cloud resource, materialize a schema, select a Rust SDK crate, add a Cargo dependency, create credentials, contact a provider, execute a recovery, allocate a production epoch/fence, or activate runtime authority.

The existing canonical PRW fence remains a non-zero `u128` encoded in exactly 16 big-endian bytes. C02f-AI preserves the C02f-AH interpretation:

`fence = (epoch << 64) | sequence`

with an unsigned 64-bit recovery epoch and an unsigned 64-bit within-epoch sequence. Neither component is logical peer identity; persisted live-owner identity remains `DeviceId + TransportIdentity`.

## Concrete external epoch-ledger provider selected

The external recovery-epoch ledger provider is selected as **Google Cloud Spanner using the GoogleSQL dialect**.

The provider is selected because the required recovery primitive is a very small correctness-critical ledger rather than a high-throughput application database. The selected behavior relies on Spanner's default serializable read-write transactions and external consistency, strong reads, atomic multi-row transaction commit, commit timestamps, and fine-grained database roles.

The Spanner ledger must remain lifecycle-independent from the live-owner etcd cluster. Restoring, replacing, rebuilding, or rolling back the etcd authority cluster must not restore or roll back the Spanner epoch ledger with it.

This selection does not yet choose a Google Cloud project, Spanner instance, instance configuration, region/multi-region placement, node/processing-unit size, database name, networking path, service account, workload identity mechanism, CMEK setting, backup policy, or billing configuration. Those are later deployment/security bindings.

## Canonical epoch representation in Spanner

Spanner `INT64` is not selected as the canonical epoch storage type because C02f-AH selected the full unsigned 64-bit epoch domain.

The canonical provider representation of an epoch is therefore exactly **8 big-endian bytes** stored as `BYTES(8)`.

Rules:

- decode as unsigned 64-bit big-endian;
- epoch `0` is the reserved unissued/bootstrap sentinel;
- issued epochs begin at `1`;
- increment is performed with checked unsigned arithmetic outside SQL expression semantics;
- overflow is fail-closed;
- no signed reinterpretation, decimal string encoding, clock-derived epoch, or provider commit timestamp substitutes for the canonical epoch value.

Commit timestamps are audit/order metadata only. They are not PRW fence values and are not logical identity.

## Selected Spanner ledger schema contract

The future materialized GoogleSQL schema must preserve two logical tables.

### `PrwRecoveryEpochHeadV1`

Exactly one logical head row for the PRW authority epoch ledger:

- `LedgerId STRING(...)` — fixed singleton identifier selected by implementation, never request-selected;
- `EpochBe BYTES(8)` — canonical current global epoch high-water;
- `LastAttemptId BYTES(32)` — exact opaque attempt identifier for the issuance that established `EpochBe`;
- `CommitTs TIMESTAMP` with commit-timestamp capability — provider commit metadata.

The initial bootstrap head is epoch `0`, with no production authority implied and no corresponding production issuance-history row.

### `PrwRecoveryEpochIssuanceV1`

One append-only row for every successfully issued production epoch:

- `EpochBe BYTES(8)` — primary key, exact issued epoch;
- `PreviousEpochBe BYTES(8)` — exact predecessor epoch observed for this issuance;
- `AttemptId BYTES(32)` — exact opaque issuance attempt identifier;
- `CommitTs TIMESTAMP` with commit-timestamp capability.

There is exactly one history row per issued epoch. Normal recovery issuance never updates or deletes a history row.

A `RecoveryEpochAttemptId` is selected as exactly 32 opaque bytes and must never be all zero. Exact generation/custody source remains a later implementation/security gate; no RNG or secret source is materialized here.

## Selected Spanner privilege shape

A future fine-grained database role named conceptually `prw_recovery_epoch_issuer` is selected with the minimum data privileges needed by the issuance protocol:

- `SELECT` on the exact head and issuance-history tables;
- `UPDATE` only on the mutable head row/columns required by issuance;
- `INSERT` on the issuance-history table;
- no `UPDATE` on issuance history;
- no `DELETE` on either ledger table;
- no schema-administration privilege;
- no unrelated database-object privilege.

Initial schema/bootstrap administration remains a separate principal/role and is not the normal recovery issuer.

The normal C02f-AG etcd principal `prw-live-owner-runtime` receives no Spanner privilege from this selection. Google IAM principal binding, database-role binding, credentials, network access and resource creation remain deferred.

## Exact epoch issuance transaction contract

Epoch issuance is an explicit control-plane recovery operation. It is not a normal live-owner runtime request.

For one issuance attempt with attempt ID `A`:

1. begin a default-serializable Spanner read-write transaction;
2. read the exact singleton head row;
3. validate canonical shape and decode predecessor epoch `H`;
4. require `H < u64::MAX` and compute exactly `N = H + 1` with checked unsigned arithmetic;
5. retain the exact plan `(H, N, A)` for the lifetime of the attempt;
6. insert one new `PrwRecoveryEpochIssuanceV1` row containing exactly `(N, H, A, pending commit timestamp)`;
7. update the singleton head from exact predecessor `H` to exactly `(N, A, pending commit timestamp)` inside the same read-write transaction;
8. require exactly one head row to have been updated and require the history insert not to collide with an existing `N` row;
9. commit once;
10. only a definitive successful commit may return `CommittedCurrent` without re-observation.

The history insert and head advance are one atomic transaction. A state where one is committed without the other is invalid and fails closed if ever observed.

The issuance path must not let a high-level client library silently change `(H, N, A)` through automatic transaction retries. Hidden automatic retry of an `ABORTED` transaction is not selected for this operation. The outer PRW recovery orchestrator owns retry/replan decisions.

## Provider result classification

The selected result semantics are deliberately conservative.

### Definitive success

A successful Spanner commit response means the exact transaction committed.

### Definitive abort/contention

A provider result that definitively reports the read-write transaction as `ABORTED` is treated as not committed. No authority is granted from that attempt. The orchestrator must perform a fresh authoritative read before constructing any later issuance plan.

It may then discover that another issuer advanced the head; in that case the old `(H, N, A)` plan is superseded and must not be replayed as a new plan.

### Indeterminate commit result

`UNKNOWN`, commit-time `DEADLINE_EXCEEDED`, transport loss after commit dispatch, or any other provider result that does not prove either successful commit or definite non-commit is classified as `MutationIndeterminate`.

No blind commit retry is permitted from `MutationIndeterminate`.

## Strong epoch re-observation contract

After `MutationIndeterminate`, the recovery orchestrator performs one fresh strong Spanner read-only transaction that reads, in one consistent snapshot:

- the exact singleton head row; and
- the exact history row keyed by proposed epoch `N`.

The result is classified against retained `(H, N, A)`:

1. history `N` exists with exact `(PreviousEpochBe = H, AttemptId = A)` and head is exactly `N` with `LastAttemptId = A` -> `CommittedCurrent`;
2. history `N` exists with exact `(H, A)` but head is strictly greater than `N` -> `CommittedButSuperseded`; the issuance existed, but this recovery attempt must not activate authority under `N`;
3. history `N` exists with another attempt or predecessor -> `Superseded`; no retry and no authority under `N`;
4. head remains exact predecessor `H` and history `N` is absent -> `ProvenNotCommitted`;
5. head is less than `H`, head/history bytes are malformed, head is `N` without the matching history row, history exists while head remains `H`, or any other state violates the selected atomic monotonic history -> fail closed as contradictory/unverifiable state.

Because every valid epoch advances by exactly one and every successful issuance creates its append-only history row atomically with the head update, observing `head > N` while history `N` is absent is also contradictory and fails closed.

## One bounded exact epoch reissue

Only `ProvenNotCommitted` permits retransmission of the retained exact issuance plan.

The orchestrator may perform **one** deliberate reissue using the exact same `(H, N, A)` values and the exact same logical transaction shape.

If that second commit result is indeterminate, the orchestrator must perform the same strong head+history re-observation again.

- if the re-observation proves the exact history `(N, H, A)` committed, classify as committed/current or committed/superseded according to the head;
- if it proves the exact predecessor still remains and `N` is still absent, fail closed with a reissue-limit result;
- no third submit is permitted.

This bounded rule mirrors the already-validated PRW principle that indeterminate authority mutations are re-observed before any retransmission and are never blindly retried.

## Recovery activation rule after epoch issuance

Before a recovered etcd authority cluster can be initialized for epoch `N`, the recovery orchestrator must perform a fresh strong Spanner head read and prove that `N` is still the current global epoch high-water.

If a later epoch is already current, the `N` recovery attempt is superseded and must not activate.

No recovered live-owner authority is exposed before this proof and the within-epoch sequence head is initialized/validated as specified below.

## Within-epoch sequence high-water placement selected

The within-current-epoch sequence high-water is selected to live **inside the current live-owner etcd authority cluster**, not in Spanner.

Rationale and boundary:

- normal fence allocation is a high-frequency authority operation and remains co-located with the existing linearizable etcd authority provider;
- process restart, provider reconnect, member replacement and leader change retain durable etcd state and therefore do not reset sequence high-water;
- if the etcd cluster is restored/replaced as a disaster-recovery event, C02f-AH already requires issuance of a strictly later external Spanner epoch before authority can resume, so a rolled-back old-epoch sequence high-water cannot authorize new fences under the new epoch;
- Spanner remains the global cross-recovery epoch authority; etcd is authoritative only for sequence allocation inside the currently activated epoch.

## Selected etcd sequence namespace

A new dedicated etcd namespace is selected:

- fixed head key: `/prw/reachability/fence-sequence/v1/head`;
- fixed reservation prefix: `/prw/reachability/fence-sequence/v1/reservation/`;
- each reservation key appends exactly 8 big-endian epoch bytes followed by exactly 8 big-endian sequence bytes.

These keys are control-plane allocator state. They are not peer keys and do not alter the canonical `/prw/reachability/live-owner/` peer namespace.

No request-selected host/path/key is permitted.

## Selected etcd allocator record encodings

The sequence allocator uses explicit canonical records separate from the existing `PRWL` live-owner record.

### Head record `PRWF` v1

Exact logical fields:

- magic `PRWF`;
- version `1`;
- epoch: 8-byte unsigned big-endian;
- high-water sequence: 8-byte unsigned big-endian.

The head may contain `(epoch = E, high_water = 0)` only as the initialized state before the first production sequence in epoch `E` is allocated.

### Reservation record `PRWR` v1

Exact logical fields:

- magic `PRWR`;
- version `1`;
- epoch: 8-byte unsigned big-endian;
- sequence: 8-byte unsigned big-endian;
- `SequenceAllocationAttemptId`: exactly 32 opaque non-zero bytes.

A reservation record is immutable under the normal allocator contract. It proves which exact attempt owns one allocated `(epoch, sequence)` pair, including after later allocations advance the head.

Exact Rust types/codecs remain a later source gate.

## Sequence-head initialization after recovery

After Spanner proves newly issued epoch `E` is still current, the recovered etcd cluster may initialize/validate the sequence head using a linearizable exact-key read.

Selected behavior:

1. if the head key is absent, create exact `PRWF(E, 0)` only with an etcd `version == 0` compare;
2. if an existing canonical head has an epoch lower than `E`, replace it with exact `PRWF(E, 0)` only under both exact observed `mod_revision` and exact observed value compares;
3. if the existing head already has epoch `E`, preserve its existing high-water and do not reset it to zero;
4. if the existing head has an epoch greater than `E`, the recovery attempt is superseded and fails closed;
5. malformed state, ABA-like same bytes at a different unapproved revision, provider ambiguity or inability to prove the exact predecessor fails closed.

Initialization does not create live-owner authority by itself.

## Exact normal sequence-allocation transaction

For active epoch `E`, allocation of one sequence proceeds from one linearizable exact head observation.

Given observed canonical head `PRWF(E, H)` at positive `mod_revision R`:

1. require the locally active epoch to equal `E`;
2. require `H < u64::MAX`;
3. compute exactly `N = H + 1`;
4. construct the exact reservation key for `(E, N)`;
5. generate/retain one exact `SequenceAllocationAttemptId A`;
6. build one etcd Txn with three conjunctive compares:
   - head `mod_revision == R`;
   - head value bytes exactly equal the observed `PRWF(E, H)` bytes;
   - reservation key `version == 0`;
7. success branch performs exactly two Puts:
   - head -> exact successor `PRWF(E, N)`;
   - reservation key -> exact `PRWR(E, N, A)`;
8. compare-failure branch performs linearizable Gets for both the exact head and exact reservation key;
9. no lease, TTL, Watch, random key, wildcard range, or delete is part of allocation.

A committed reservation permanently consumes `N`. If later live-owner acquisition never commits, `N` is burned and must not be reused.

Gaps are permitted; reuse is not.

## Sequence allocation re-observation

An etcd RPC error after transaction dispatch is indeterminate and must not be blindly retried.

The allocator retains exact predecessor `(R, PRWF(E,H))`, exact successor `(E,N)`, exact reservation key and exact attempt `A`, then performs a fresh linearizable observation of both head and reservation.

Classification:

1. reservation exists as exact `PRWR(E,N,A)` and head is canonical with same epoch and `high_water >= N` -> `Committed`; this exact attempt owns sequence `N` even if later allocations advanced the head;
2. reservation exists for `(E,N)` with another attempt ID -> `Superseded`; this attempt must never use `N`;
3. reservation is absent and head remains byte-identical at the exact same predecessor `mod_revision R` -> `ProvenNotCommitted`;
4. reservation is absent but the head revision changed, the head advanced to or beyond `N`, the epoch changed, or an ABA-like predecessor value reappeared at a new revision -> fail closed; absence does not prove non-commit;
5. malformed head/reservation bytes, key mismatch, rollback, overflow, unavailable re-observation or contradictory state -> fail closed.

Only `ProvenNotCommitted` permits one deliberate exact reissue of the same transaction plan with the same `A`.

After a second indeterminate submit, re-observe once more. If the exact reservation is not proven committed and exact non-commit is again observed, return a reissue-limit failure. No third Txn is permitted.

## Sequence reservation retention

Normal allocator authority may create reservation records but does not delete or rewrite them.

Reservation compaction/archival after an epoch is permanently retired is not selected by this checkpoint and requires a later bounded retention design. No cleanup mechanism may make an in-flight or delayed indeterminate allocation ambiguous.

## Allocator privilege separation

The existing C02f-AG `prw-live-owner-runtime` principal and its exact `/prw/reachability/live-owner/` role remain unchanged by this docs-only checkpoint.

A separate future etcd principal/role is selected conceptually for sequence allocation, for example `prw-fence-allocator-runtime`, with read/write access limited to `/prw/reachability/fence-sequence/` and no cluster administration or unrelated PRW keyspace.

Concrete certificate issuance, etcd user/role creation, auth mutation and Cargo/TLS materialization remain separately gated.

## Split-brain and stale-cluster boundary

A later recovery epoch makes every older-epoch fence globally stale under unsigned `u128` ordering, but C02f-AI does not by itself stop an isolated old etcd cluster from continuing to allocate old-epoch sequences.

Therefore this selection does not claim end-to-end stale-side-effect prevention. R1-R4 consumers/effect boundaries must eventually reject lower-epoch fences before production disaster recovery can be considered fully fenced.

No lower-epoch cluster may be treated as current merely because its local sequence allocator remains internally consistent.

## Provider capability alignment

The reviewed provider behavior supports this selection:

- Spanner default serializable read-write transactions provide external consistency;
- strong reads observe all transactions committed before the read begins;
- a read-write transaction commits its writes atomically;
- provider documentation states that an `ABORTED` transaction does not affect the database and is retried from the beginning by normal client libraries;
- Spanner commit can rarely return `UNKNOWN`, for which provider guidance is to read the database to determine current state;
- commit-time `DEADLINE_EXCEEDED` can occur even when a state-changing operation completed, so it is treated as indeterminate;
- fine-grained database roles can grant `SELECT`, `INSERT`, `UPDATE`, and `DELETE` independently, enabling an issuer role with append-only history privileges;
- commit timestamps can be written atomically as provider metadata.

These capabilities are architecture evidence only. No network call or provider resource is created by this contract.

## Explicitly deferred

C02f-AI does not materialize or authorize:

- Google Cloud project/organization/resource creation;
- Spanner instance/database/schema creation or DDL execution;
- Spanner region/multi-region configuration or capacity sizing;
- service account, Workload Identity, OAuth credential, IAM binding or database-role creation;
- concrete Rust Spanner SDK/client crate or Cargo dependency;
- source implementation of the Spanner ledger port, schema codec or retry machinery;
- source implementation of `PRWF`/`PRWR`, sequence allocator, new etcd prefix role, or recovery bootstrap;
- any Spanner or etcd production endpoint contact;
- actual epoch issuance, sequence allocation, snapshot restore or disaster-recovery execution;
- C02f-AG TLS feature materialization, certificate/private-key generation, existing etcd auth/RBAC mutation or secret distribution;
- concrete cloud/platform placement for the etcd authority cluster;
- R1-R4 stale-side-effect fencing implementation;
- merge, retargeting, deployment or production activation.

## Next dependency

After C02f-AI is validated and frozen, the next bounded source/design gate is the **provider/client materialization contract**:

- select the concrete Rust Spanner client/dependency path and feature set;
- materialize provider-neutral epoch-ledger types/port and canonical Spanner row codecs without creating resources or contacting production;
- materialize provider-neutral `PRWF`/`PRWR` codecs and deterministic sequence-allocation plan types;
- keep actual cloud credentials/resources, etcd RBAC changes, recovery execution and runtime activation separately gated.

## Authorization boundary

`C02F_AI_PROVIDER_SELECTION_ONLY / SPANNER_LEDGER_SELECTED / ETCD_SEQUENCE_PLACEMENT_SELECTED / U128_CODEC_BYTE_STABLE / NO_CLOUD_RESOURCE / NO_SCHEMA_DDL / NO_CARGO / NO_SOURCE_MATERIALIZATION / NO_CREDENTIAL / NO_PROVIDER_CONTACT / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Any Spanner resource/schema creation, SDK/Cargo selection or materialization, source implementation, service-account/IAM/database-role mutation, etcd allocator RBAC mutation, credential/secret creation, provider endpoint contact, recovery execution, runtime activation, deployment, retargeting or merge requires a separate explicit authorization.