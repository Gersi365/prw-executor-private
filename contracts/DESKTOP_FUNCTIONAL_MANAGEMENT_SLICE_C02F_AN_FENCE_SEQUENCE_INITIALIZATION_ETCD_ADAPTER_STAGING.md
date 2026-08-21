# Phase 152 C02f-AN — Fence-Sequence Initialization etcd Adapter Staging Contract

## Purpose

C02f-AN materializes the bounded real `etcd-client` translation layer for the already-selected
C02f-AM fence-sequence epoch-head initialization plan.

C02f-AM closed deterministic initialization and re-observation semantics but intentionally performed
no provider RPC. C02f-AN binds that exact retained plan to `etcd-client = 0.19.0` while preserving
provider connection, retry policy, runtime construction, and authority activation as separate later
boundaries.

## Exact base

- base tranche: C02f-AM
- exact validated base head: `757cd2b851a290d4e00876aa6dfabfe690e288ab`
- base PR #58 remains draft and unmerged

## Selected adapter behavior

C02f-AN accepts an already-created `etcd_client::KvClient`.

Construction therefore:

- performs no endpoint selection;
- performs no `Client::connect`;
- performs no credential lookup;
- performs no TLS/auth/RBAC configuration;
- performs no network I/O by itself.

For one retained C02f-AM initialization mutation plan, the adapter translates the plan exactly:

- `HeadVersionZero` -> etcd key-version compare `== 0`;
- `HeadModRevisionEquals` -> etcd `mod_revision == expected` compare;
- `HeadExactValueEquals` -> etcd exact-value compare;
- success branch -> exactly the retained one Put;
- compare-failure branch -> exactly the retained one default-linearizable exact-key Get.

The adapter submits the translated Txn at most once per `execute` call.

## Definitive response handling

For a definitive etcd Txn response:

- compare success requires exactly one Put response and returns `Applied`;
- compare failure requires exactly one Get response;
- the failure-branch Get is decoded as zero-or-one exact canonical PRWF head observation;
- more than one returned key/value fails closed;
- a returned key different from the requested key fails closed;
- malformed PRWF bytes or non-positive `mod_revision` fail closed through the existing AJ observation boundary;
- the resulting fresh observation is classified only through the existing AM
  `classify_initialization_reobservation` contract;
- contradictory state remains a hard failure.

No provider failure is promoted to authority.

## Indeterminate mutation handling

An etcd mutation RPC error that returns no definitive Txn response is classified only as
`MutationIndeterminate`.

C02f-AN performs no automatic retry after such an error.

The adapter exposes one explicit fresh default-linearizable re-observation operation that:

1. Gets the canonical PRWF head exactly once;
2. validates/decode the exact provider observation;
3. applies the retained AM re-observation classifier.

This operation itself performs no mutation and consumes no reissue budget.

## Retry boundary

C02f-AN selects **no retry policy** and **no reissue budget**.

In particular, this tranche does not:

- blindly retransmit an indeterminate mutation;
- consume or create a one-reissue allowance;
- loop on `ProvenNotCommitted`;
- create a third-submit path;
- spawn detached/background recovery work.

Any future retry/orchestration policy must be a separate bounded tranche and must reuse the exact
retained AM plan rather than silently re-planning initialization.

## Source scope

C02f-AN adds only:

- `crates/prw-control-plane/src/fence_sequence_initialization_etcd.rs`;
- `crates/prw-control-plane/tests/c02f_an_fence_sequence_initialization_etcd_adapter.rs`;
- this contract.

C02f-AN does not modify:

- `crates/prw-control-plane/src/fence_sequence_initialization.rs`;
- `crates/prw-control-plane/src/fence_sequence.rs`;
- `crates/prw-control-plane/src/recovery_epoch.rs`;
- `crates/prw-control-plane/src/recovery_epoch_spanner.rs`;
- `crates/prw-control-plane/src/recovery_epoch_orchestrator.rs`;
- `crates/prw-control-plane/src/lib.rs`;
- `crates/prw-control-plane/Cargo.toml`;
- root `Cargo.lock`.

The existing exact-pinned `etcd-client = 0.19.0` dependency is reused without dependency or lockfile
mutation.

## Explicit non-goals

C02f-AN does not:

- create or connect an etcd client;
- select etcd endpoints;
- configure TLS, authentication, RBAC, leases, TTLs, Watches, or background tasks;
- create or alter etcd users, roles, permissions, or cluster membership;
- initialize a real production PRWF head in validation;
- allocate a real fence sequence;
- construct or contact Spanner;
- issue a real recovery epoch;
- execute snapshot restore or disaster recovery;
- export the AN module through the production public library surface;
- select a retry/reissue policy;
- activate production authority;
- implement R1-R4 lower-epoch effect rejection;
- deploy;
- merge any draft PR.

## Validation gate

The tranche is valid only if canonical Rust and Android workflows pass on the exact C02f-AN head
and the final AM -> AN compare proves the scope remains exactly the three intended added files with
AM source, AJ sequence source, public `lib.rs`, Cargo manifest, and lockfile byte-stable.

Expected gate after validation:

`C02F_AN_FENCE_SEQUENCE_INITIALIZATION_ETCD_ADAPTER_VALIDATED`
