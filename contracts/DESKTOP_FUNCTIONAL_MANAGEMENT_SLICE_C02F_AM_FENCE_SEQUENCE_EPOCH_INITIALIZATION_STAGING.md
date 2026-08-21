# Phase 152 C02f-AM — Fence-Sequence Epoch Initialization Staging Contract

## Purpose

C02f-AM materializes the provider-neutral deterministic planning boundary required to initialize or
roll the existing PRWF within-epoch fence-sequence head to one already-selected globally-current
recovery epoch.

This tranche exists because C02f-AJ materialized PRWF/PRWR codecs, allocation planning,
re-observation classification, and a one-reissue allocation budget, but it did not materialize the
recovery-time PRWF head initialization/rollover decision selected by C02f-AI.

C02f-AM does not contact etcd or Spanner and does not activate recovery/runtime authority.

## Exact base

- base tranche: C02f-AL
- exact base head: `358633f7ef6299c398c932c10d01bc711b1c7395`
- base PR #57 remains draft and unmerged

## Selected deterministic initialization behavior

Given a globally-current target recovery epoch `E` and one fresh exact PRWF-head observation:

1. **Head absent**
   - plan one mutation guarded by head-key `version == 0`;
   - success writes exactly `PRWF(E, 0)` to the canonical head key;
   - failure branch performs one exact default-linearizable Get of the canonical head key.

2. **Observed head epoch < E**
   - retain the exact predecessor bytes and exact positive `mod_revision`;
   - plan exactly two compares in order: exact `mod_revision`, exact value bytes;
   - success replaces the canonical head with exactly `PRWF(E, 0)`;
   - failure branch performs one exact default-linearizable Get of the canonical head key.

3. **Observed head epoch == E**
   - perform no mutation;
   - preserve the existing high-water exactly;
   - never reset a same-epoch head to zero.

4. **Observed head epoch > E**
   - perform no mutation;
   - classify the retained target as superseded.

Malformed PRWF bytes and non-positive revisions remain rejected by the existing AJ observation
constructor before C02f-AM planning.

## Indeterminate initialization re-observation

C02f-AM materializes a narrow provider-neutral classifier for a future adapter/orchestrator after an
indeterminate initialization mutation:

- observed epoch == target `E` => `Current`, even if high-water is now greater than zero because a
  later same-epoch allocator may already have advanced it;
- observed epoch > `E` => `Superseded`;
- create plan + head still absent => `ProvenNotCommitted`;
- replace plan + exact same predecessor bytes **and** exact same predecessor revision =>
  `ProvenNotCommitted`;
- every other absent/lower/changed-revision/changed-value state => contradictory and fail closed.

This classifier does **not** authorize a retry policy. No new reissue budget is selected here.

## Source scope

C02f-AM adds only:

- `crates/prw-control-plane/src/fence_sequence_initialization.rs`;
- `crates/prw-control-plane/tests/c02f_am_fence_sequence_epoch_initialization.rs`;
- this contract.

C02f-AM does not modify:

- `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- root `Cargo.lock`;
- existing AJ `fence_sequence.rs`;
- existing AK Spanner adapter;
- existing AL recovery-epoch orchestrator.

## Explicit non-goals

C02f-AM does not:

- create or connect an etcd client;
- translate the plan to a real etcd `Txn`;
- select endpoints, TLS, auth, RBAC, credentials, leases, TTLs, Watches, or tasks;
- initialize a real production PRWF head;
- allocate any real fence sequence;
- construct or contact Spanner;
- issue a real recovery epoch;
- execute snapshot restore or disaster recovery;
- export the new module through the production public library surface;
- implement R1-R4 lower-epoch effect rejection;
- activate production authority;
- deploy;
- merge any draft PR.

## Validation gate

The tranche is valid only if canonical Rust and Android workflows pass on the exact C02f-AM head and
final compare proves the scope remains the three intended files with Cargo/lib.rs byte-stable.

Expected gate after validation:

`C02F_AM_FENCE_SEQUENCE_EPOCH_INITIALIZATION_VALIDATED`
