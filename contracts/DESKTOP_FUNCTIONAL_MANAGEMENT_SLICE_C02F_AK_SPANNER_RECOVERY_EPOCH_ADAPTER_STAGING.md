# Phase 152 C02f-AK — Spanner Recovery-Epoch Adapter Staging Contract

Status: `SOURCE_ADAPTER_STAGED / AJ_VALIDATED_BASE_78BB48B1 / GOOGLE_CLOUD_SPANNER_0_34_4_PREVIEW / INJECTED_DATABASE_CLIENT / FIXED_LEDGER_ID / EXPLICIT_BEGIN / TRANSACTION_ATTEMPT_LIMIT_ONE / BEGIN_NEVER_RETRY / DML_NEVER_RETRY / COMMIT_NEVER_RETRY / STRONG_REOBSERVATION / EXACT_ROW_COUNTS / PROVIDER_IO_IMPLEMENTED_NOT_EXECUTED / NO_CLIENT_CONSTRUCTION / NO_CREDENTIAL_LOOKUP / NO_SCHEMA_DDL / NO_PROVIDER_CONTACT / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative AJ predecessor: `78bb48b1b4cf788567ad06c080af545e788992a7`
AJ PR: `#55` (`open / draft / unmerged`, canonical Rust #777 + Android #285 PASS)
AK branch: `phase-152-c02f-ak-spanner-recovery-epoch-adapter-staging`

## Purpose

C02f-AK materializes the concrete Google Cloud Spanner data-plane adapter for the provider-neutral recovery-epoch authority port created in C02f-AJ.

The adapter is deliberately dependency-injected with an already-constructed `google_cloud_spanner::client::DatabaseClient`. AK therefore implements the provider I/O path while keeping cloud resource selection, client construction, credentials, endpoint selection, schema creation, IAM/FGAC binding, recovery execution and runtime activation outside this tranche.

The adapter source is compiled through an AK staging integration harness and is not yet exported through `prw-control-plane::lib.rs`. This prevents accidental runtime reachability before validation closure and a later explicit runtime-construction tranche.

## Fixed provider surface

The implementation fixes these identifiers in source; none is request-selected:

- singleton ledger ID: `prw-recovery-epoch-v1`;
- head table: `PrwRecoveryEpochHeadV1`;
- append-only issuance table: `PrwRecoveryEpochIssuanceV1`.

The canonical epoch and attempt encodings remain those validated in AJ:

- epoch: exact unsigned big-endian `BYTES(8)`;
- issued epoch zero is forbidden; zero is bootstrap only;
- attempt ID: exact non-zero `BYTES(32)` for issued epochs;
- bootstrap `LastAttemptId`: exact 32-byte zero non-attempt marker.

## Single logical issuance attempt

For one retained AJ `RecoveryEpochIssuancePlan (H, N, A)`, AK uses the exact pinned first-party Spanner client with the following controls:

1. `BeginTransactionOption::ExplicitBegin`;
2. Begin RPC retry policy = `NeverRetry`;
3. transaction retry policy = `BasicTransactionRetryPolicy::with_max_attempts(1)`;
4. every authority-changing DML statement retry policy = `NeverRetry`;
5. Commit RPC retry policy = `NeverRetry`.

This prevents `TransactionRunner` from replaying an `ABORTED` transaction closure and prevents GAX RPC retry policy from replaying the authority-changing DML or commit request after transport ambiguity.

The Spanner SDK may perform protocol continuation for a returned precommit token. That is treated as part of one logical commit attempt: the SDK source explicitly does not resend mutations in the continuation request. Any failure after both PRW DML statements have returned exact success is still classified conservatively as commit-boundary ambiguity unless Spanner definitively reports `ABORTED`.

## Exact transaction body

Within the single read-write transaction attempt:

1. read the fixed singleton head row;
2. require exactly one row;
3. decode canonical `EpochBe` and `LastAttemptId`;
4. require observed head epoch to equal exact retained predecessor `H`;
5. execute an `INSERT` into `PrwRecoveryEpochIssuanceV1` with exact `(N,H,A,PENDING_COMMIT_TIMESTAMP())`;
6. require exact affected-row count `1`;
7. execute an `UPDATE` of the singleton head from `H` to `(N,A,PENDING_COMMIT_TIMESTAMP())`;
8. require exact affected-row count `1`;
9. mark the transaction as ready for commit;
10. let the one-attempt runner perform the commit with Commit RPC `NeverRetry`.

If a local canonical-state guard fails, the closure returns an internal adapter error and the runner performs only best-effort rollback; no authority result is produced.

## Provider result classification

AK preserves AI/AJ semantics:

- successful logical commit -> `CommittedCurrent`;
- Spanner status `ABORTED`, at any submit stage -> `Aborted` and no authority;
- any non-ABORTED error after both authority-changing DML statements returned exact success -> `MutationIndeterminate`;
- provider/transport error before the commit boundary -> adapter error, no authority outcome;
- no hidden transaction replay and no third submit behavior is introduced.

The outer recovery orchestrator still owns strong re-observation and the one bounded exact reissue selected by AI/AJ.

## Strong reads

`strong_head` uses a strong single-use read-only Spanner transaction and fixed singleton query.

`strong_reobserve(N)` uses one strong single-use query that returns, in one provider snapshot:

- the singleton head; and
- the exact optional history row keyed by proposed epoch `N`.

The query uses a left join so absence of the exact history row is represented explicitly. Partial-null history projections, malformed BYTES, multiple singleton rows, bootstrap history epochs and other non-canonical provider states fail closed.

All strong-read statements also use `NeverRetry` so the adapter does not silently replace one authoritative observation with a later one inside the same PRW operation.

## Validation boundary

AK validation must prove at minimum:

1. canonical locked dependency graph remains stable;
2. rustfmt PASS;
3. Clippy `-D warnings` PASS;
4. full workspace tests/build PASS;
5. Android native/application validation remains PASS because `prw-control-plane` is in the Android native dependency graph;
6. adapter unit tests prove fixed schema surface and provider-error classification;
7. no Cargo dependency or lockfile delta unless compiler evidence demonstrates one is required.

No real Spanner endpoint, credential, project, instance, database or schema is needed or contacted by this source validation.

## Non-claims

AK does not claim or perform:

- Google Cloud project/instance/database creation;
- Spanner DDL/schema materialization;
- service account, ADC, Workload Identity or other credential construction;
- IAM or Spanner database-role binding;
- production endpoint/network selection or contact;
- actual epoch issuance;
- disaster-recovery execution;
- etcd sequence allocator I/O;
- etcd allocator RBAC materialization;
- runtime provider construction/activation;
- R1-R4 stale-side-effect rejection;
- merge or deployment.

Authorization/execution boundary:

`C02F_AK_SPANNER_ADAPTER_SOURCE_ONLY / PROVIDER_IO_CODE_MATERIALIZED_NOT_EXECUTED / NO_RESOURCE / NO_CREDENTIAL / NO_ENDPOINT_CONTACT / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION / NO_DEPLOYMENT`
