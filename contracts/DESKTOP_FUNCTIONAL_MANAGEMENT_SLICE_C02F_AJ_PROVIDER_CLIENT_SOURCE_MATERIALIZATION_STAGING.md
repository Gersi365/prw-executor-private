# Phase 152 C02f-AJ — Provider / Client Source Materialization Staging Contract

Status: `SOURCE_MATERIALIZATION_STAGED / AI_EXACT_HEAD_96AD285E / P1_CONTROL_PLANE_PLACEMENT_SELECTED / GOOGLE_CLOUD_SPANNER_0_34_4_PREVIEW_EXACT_PIN_MATERIALIZED / GOOGLE_CLOUD_GAX_1_13_0_EXACT_PIN_MATERIALIZED / DEFAULT_FEATURES_DISABLED / LOCKED_DEPENDENCY_GRAPH_PASS_NO_LOCKFILE_DELTA / PRWF_PRWR_U16_BE_VERSION_SELECTED / BOOTSTRAP_ZERO_NON_ATTEMPT_MARKER_SELECTED / PROVIDER_NEUTRAL_RECOVERY_EPOCH_SOURCE_MATERIALIZED / PROVIDER_NEUTRAL_SEQUENCE_ALLOCATOR_SOURCE_MATERIALIZED / FORMAT_CORRECTION_STAGED / COMPILE_CLIPPY_TEST_BUILD_PENDING / NO_PROVIDER_IO / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative AI predecessor: `96ad285ebbc51de0f62d667ec019f0f49b3e5cde`
AI PR: `#54` (`open / draft / unmerged`)
AJ branch: `phase-152-c02f-aj-provider-client-source-materialization-staging`
AJ draft PR: `#55`
Initial AJ source commit: `7f7cf25a60d1d7fb59979e06ff01e3c592418d95`

## Gate scope

This bounded source/dependency tranche materializes exact provider-client dependency pins plus provider-neutral recovery epoch and sequence allocator source. It does not create a cloud resource, contact Spanner or etcd for the new recovery operations, create credentials, execute recovery, activate runtime authority, implement R1-R4, merge, or deploy.

## Source placement

`P1 / prw-control-plane ownership` is selected.

Provider-neutral domain, codec and plan types remain free of Google SDK types. A later provider-I/O adapter may use the selected SDK inside `prw-control-plane`, symmetric with the existing etcd live-owner adapter and without adding a workspace crate.

## Exact dependency selection

`crates/prw-control-plane/Cargo.toml` materializes:

```toml
google-cloud-gax = "=1.13.0"
google-cloud-spanner = { version = "=0.34.4-preview", default-features = false }
```

The Spanner dependency is exact-pinned because the selected first-party crate is preview. Default features remain disabled so this tranche does not silently select a rustls crypto provider. Concrete TLS/crypto-provider/runtime construction remains separately gated.

The inspected Google release train declares Rust `1.88.0`; canonical PRW validation uses Rust `1.97.1`.

## Canonical locked dependency result

Canonical `PRW Rust Validation #762` on initial AJ head `7f7cf25a60d1d7fb59979e06ff01e3c592418d95` executed:

`cargo metadata --locked --no-deps --format-version 1`

and it **passed**.

Therefore the previously anticipated Cargo.lock blocker did not materialize. The existing Cargo.lock already resolves the exact selected dependency graph sufficiently for the locked metadata gate, and AJ requires **no hand-authored or generated lockfile delta at this checkpoint**.

The same run then failed only at `cargo fmt --all -- --check`; Clippy, tests and build were skipped. The current corrective commit is limited to formatting/test-module visibility plus this factual contract correction. No compile PASS is claimed until the follow-up canonical run completes.

## Retry semantics retained for the later Spanner adapter

Any later provider-I/O adapter must explicitly preserve the AI contract:

- full transaction attempt count bounded to one so `ABORTED` returns to PRW orchestration;
- Commit RPC `NeverRetry`;
- initial authority-changing DML `NeverRetry` preferred for one-submit auditability;
- transport/`UNKNOWN`/commit-time deadline ambiguity -> `MutationIndeterminate`;
- fresh strong head+history re-observation before one bounded exact reissue;
- no third submit.

No Spanner provider-I/O adapter is part of AJ source materialization.

## Recovery epoch source

`crates/prw-control-plane/src/recovery_epoch.rs` materializes provider-neutral:

- full unsigned u64 recovery epoch domain with explicit epoch-zero bootstrap sentinel;
- exact 8-byte unsigned big-endian epoch codec;
- exact 32-byte non-zero `RecoveryEpochAttemptId`;
- deterministic retained issuance plan `(H, N, A)` with checked increment;
- canonical head/history logical records;
- provider-neutral `RecoveryEpochLedgerAuthority` port using `impl Future + Send` and static dispatch;
- strong head+history re-observation classification;
- one bounded reissue budget / no third submit.

### Bootstrap head marker

AI specifies an epoch-zero bootstrap head while production attempt IDs must be non-zero. AJ selects exactly 32 zero bytes as the bootstrap-only `LastAttemptId` **non-attempt marker**. It is never a valid `RecoveryEpochAttemptId`; every issued epoch requires a non-zero 32-byte attempt ID.

## Within-epoch sequence source

`crates/prw-control-plane/src/fence_sequence.rs` materializes provider-neutral:

- fixed head key `/prw/reachability/fence-sequence/v1/head`;
- fixed reservation prefix `/prw/reachability/fence-sequence/v1/reservation/`;
- deterministic reservation suffix `epoch_be8 || sequence_be8`;
- exact `PRWF` head codec;
- exact `PRWR` immutable reservation codec;
- exact 32-byte non-zero `SequenceAllocationAttemptId`;
- exact positive predecessor `mod_revision` plus predecessor bytes;
- deterministic three-compare / two-put / two-get transaction plan;
- committed/superseded/proven-not-committed re-observation classification;
- ABA-like same bytes at a new revision fails closed;
- one bounded exact reissue / no third submit.

## Exact PRWF / PRWR version width

AI selected logical record version `1` but did not materialize its byte width. AJ selects an unsigned big-endian `u16` version field, exact value `1`.

Therefore:

- `PRWF` = 4 magic + 2 version + 8 epoch + 8 high-water = **22 bytes**;
- `PRWR` = 4 magic + 2 version + 8 epoch + 8 sequence + 32 attempt ID = **54 bytes**.

No host-endian, variable-width or implicit version representation is permitted.

## Validation harness

`crates/prw-control-plane/tests/c02f_aj_materialization.rs` includes the two provider-neutral modules as public validation modules so their unit tests and selected byte-width constants participate in the AJ test target before any provider I/O or runtime wiring exists.

Canonical follow-up validation must still prove:

1. locked metadata;
2. rustfmt;
3. Clippy with `-D warnings` under workspace `all + pedantic + nursery` lints;
4. full workspace tests;
5. full workspace build.

## Non-claims

AJ does not authorize or perform Spanner schema/DDL, project/instance/database creation, ADC or credentials, IAM/FGAC binding, endpoint contact, provider I/O, etcd allocator RBAC mutation, recovery execution, epoch issuance, sequence allocation, TLS/crypto runtime selection, R1-R4 stale-side-effect fencing, merge, retargeting, deployment or production activation.
