# Phase 152 C02f-AJ — Provider / Client Source Materialization Staging Contract

Status: `SOURCE_MATERIALIZATION_STAGED / AI_EXACT_HEAD_96AD285E / P1_CONTROL_PLANE_PLACEMENT_SELECTED / GOOGLE_CLOUD_SPANNER_0_34_4_PREVIEW_EXACT_PIN_MATERIALIZED / GOOGLE_CLOUD_GAX_1_13_0_EXACT_PIN_MATERIALIZED / DEFAULT_FEATURES_DISABLED / PRWF_PRWR_U16_BE_VERSION_SELECTED / BOOTSTRAP_ZERO_NON_ATTEMPT_MARKER_SELECTED / PROVIDER_NEUTRAL_RECOVERY_EPOCH_SOURCE_MATERIALIZED / PROVIDER_NEUTRAL_SEQUENCE_ALLOCATOR_SOURCE_MATERIALIZED / GITHUB_BRANCH_CREATED / CARGO_LOCK_GENERATION_PENDING_CANONICAL_TOOLCHAIN / COMPILE_VALIDATION_PENDING / NO_PROVIDER_IO / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative AI predecessor: `96ad285ebbc51de0f62d667ec019f0f49b3e5cde`
AI PR: `#54` (`open / draft / unmerged / mergeable`)
Branch: `phase-152-c02f-aj-provider-client-source-materialization-staging`
Contract path: `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02F_AJ_PROVIDER_CLIENT_SOURCE_MATERIALIZATION_STAGING.md`

## Authorized gate interpretation

The user explicitly authorized C02f-AJ branch creation, commit, push and draft PR publication for this bounded source/dependency materialization gate. The gate materializes exact dependency pins plus provider-neutral recovery epoch and sequence allocator source while keeping cloud resources, provider I/O, credentials, runtime activation, recovery execution and merge outside scope.

## Source placement selected

`P1 / prw-control-plane ownership` is selected.

Rationale:
- preserves symmetry with the existing `reachability_live_owner_etcd` adapter ownership;
- avoids adding a workspace crate solely to isolate one preview SDK;
- provider-neutral domain/codec/plan modules remain free of Google SDK types;
- only the future provider-I/O adapter needs `google-cloud-spanner` / `google-cloud-gax` imports.

## Exact dependency selection

`crates/prw-control-plane/Cargo.toml` materializes:

```toml
google-cloud-gax = "=1.13.0"
google-cloud-spanner = { version = "=0.34.4-preview", default-features = false }
```

The exact pins are selected because the Spanner crate is preview. Default features remain disabled so this gate does not silently select a rustls crypto provider. Concrete crypto-provider installation remains a runtime/security gate.

MSRV evidence: `google-cloud-rust release-20260730` declares `rust-version = "1.88.0"`; canonical PRW CI is Rust `1.97.1`, so the selected release train is not blocked by declared MSRV. This is compatibility evidence only, not a compile PASS.

## Retry constraints retained

Any later Spanner issuance adapter must prove:
- transaction-level `BasicTransactionRetryPolicy::new().with_max_attempts(1)`;
- Commit RPC `NeverRetry`;
- initial authority-changing DML also `NeverRetry` for reviewable one-submit semantics;
- `ABORTED` returns to PRW orchestration for fresh authoritative read/replan;
- transport/UNKNOWN/commit-time deadline ambiguity returns `MutationIndeterminate` and requires strong head+history re-observation before one bounded exact reissue.

No provider I/O adapter is materialized by this tranche.

## Exact source-level encoding selections

### Spanner bootstrap head marker

C02f-AI specified `LastAttemptId BYTES(32)` and a bootstrap epoch-zero head, while production `RecoveryEpochAttemptId` must never be all zero.

AJ selects:
- `EpochBe == 0` bootstrap head -> `LastAttemptId == [0; 32]` exactly;
- this byte pattern is a bootstrap-only **non-attempt marker**, not a `RecoveryEpochAttemptId`;
- every issued epoch `>=1` requires a non-zero exact 32-byte `RecoveryEpochAttemptId`.

### `PRWF` / `PRWR` version field

C02f-AI selected logical `version 1` without an exact byte width.

AJ selects one unsigned big-endian `u16` version field with value `1` for both records.

Therefore:
- `PRWF` = 4-byte magic + 2-byte version + 8-byte epoch + 8-byte high-water = 22 bytes;
- `PRWR` = 4-byte magic + 2-byte version + 8-byte epoch + 8-byte sequence + 32-byte attempt ID = 54 bytes.

No reserved flags or implicit host-endian fields are introduced.

## Source materialization

New provider-neutral source files:
- `crates/prw-control-plane/src/recovery_epoch.rs`
- `crates/prw-control-plane/src/fence_sequence.rs`

The source files remain isolated from runtime/provider wiring in this staging tranche. A validation integration-test harness may compile them directly before public library exposure is finalized.

Materialized logic includes:
- full unsigned u64 recovery epoch domain with explicit bootstrap sentinel;
- provider-neutral `RecoveryEpochLedgerAuthority` async port using `impl Future + Send`, static dispatch and no Google SDK types;
- exact `BYTES(8)` encoding/decoding;
- non-zero exact 32-byte recovery attempt ID;
- bootstrap head marker validation;
- exact issuance plan `(H,N,A)` and append-only issuance row validation;
- strong head+history re-observation classification;
- one bounded exact reissue budget / no third submit;
- exact fixed sequence head/reservation namespace;
- exact PRWF/PRWR v1 codecs;
- non-zero exact 32-byte sequence allocation attempt ID;
- deterministic reservation key: epoch BE8 || sequence BE8;
- exact head observation with positive `mod_revision`;
- exact three compares: head revision + exact head bytes + reservation version zero;
- exactly two success Puts and two compare-failure linearizable Gets;
- reservation key/value binding validation;
- committed/superseded/proven-not-committed classification;
- ABA same bytes at new revision fails closed;
- one bounded exact reissue / no third submit.

## Validation boundary

The local execution environment has no `cargo` or `rustc` binary, so this commit cannot pre-generate or honestly claim a Cargo-authored lockfile, rustfmt, Clippy, compile or workspace-test PASS before GitHub CI.

Local patch transport validation against exact AI context: **PASS**. This proves patch serialization/context integrity only.

Because the direct dependency graph changes, canonical `cargo metadata --locked` is expected to require a Cargo-generated `Cargo.lock` update. The lockfile must not be hand-authored. If canonical CI stops at the locked-graph gate, that is a materialization blocker to resolve under a Cargo-capable validation environment; it is not evidence against the selected provider-neutral source semantics.

No compile claim is made until canonical CI executes successfully on the exact branch head.

## Non-claims

No cloud resource/schema/DDL, credential, IAM/FGAC binding, endpoint contact, Spanner request, etcd RBAC mutation, recovery execution, epoch issuance, sequence allocation, TLS runtime selection, R1-R4 fencing, merge, retarget, deployment or production activation is authorized or performed here.
