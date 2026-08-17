# PRW Phase 152 Corrected Preview Runner Readiness

Status: `READY / GITHUB_DRIVE_MATCH / PREVIEW_ONLY / REAL_HOST_EXECUTION_PENDING`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_authority_commit: `01f5466504684ea6a2c504613901d24018485887`
- reconciliation_branch: `phase-152-github-first-reconciliation`

## Runner identity

- path: `tools/workspace-sync/prw-run-corrected-reconciliation-preview.sh`
- Git blob: `109c96744f5b6e0ae143d4cd226c74db24fdee87`
- SHA-256: `cc036f60fdd8b91b3e5ff1ea54bc36a4bc32a005bb5dc3a0d48b5da68049a220`
- size: `6813`
- Drive file ID: `1nIoPxWGQsZikQK8P3DgOY_AIDE-Ip6Xg`
- Drive raw readback Git blob: `109c96744f5b6e0ae143d4cd226c74db24fdee87`
- Drive raw readback SHA-256: `cc036f60fdd8b91b3e5ff1ea54bc36a4bc32a005bb5dc3a0d48b5da68049a220`
- `bash -n`: `PASS`

## Pinned corrected bootstrap

- bootstrap path: `tools/workspace-sync/prw-bootstrap-drive-reconciliation.sh`
- bootstrap Git blob: `bd9ffcab696e067e03f64779a1b3e6e45991febc`
- bootstrap SHA-256: `c6efc345a6ab572749dd194d89f5732b5d6700454189205e7c54ef06e3eb6da1`

The runner downloads this bootstrap from the Drive authority namespace and verifies both SHA-256 and Git blob before execution.

## Preview gate enforced by runner

The runner performs no source apply. It executes the corrected bootstrap in preview mode and requires all of the following before returning success:

- `verified_files = 93`
- `apply_eligible_files = 90`
- `deferred_runtime_gate_files = 3`
- `local_changes_required = 86`
- `MATCH = 4`
- `ABSENT = 86`
- `DIFF = 0`
- `DEFERRED_RUNTIME_GATE = 3`
- `BLOCKED = 0`
- exact deferred path set match
- reconciliation status remains `STAGED / VERIFIED / LOCAL_SOURCE_NOT_MUTATED`

Exact deferred paths:

- `crates/prw-agent/src/main.rs`
- `crates/prw-agent/tests/phase125_device_identity_bootstrap.rs`
- `crates/prw-agent/tests/phase_102_binary_bootstrap.rs`

On any mismatch, the runner returns non-zero and records `FAIL / SOURCE_NOT_APPLIED` evidence. On success, it records `PASS / SOURCE_NOT_APPLIED` in `logs/audits/corrected-reconciliation-preview/<STAMP>/SECOND_PREVIEW_GATE.md`.

## Preserved gates

- source apply: `NOT_PERFORMED_BY_RUNNER`
- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`

## Next boundary

`CORRECTED_SECOND_PREVIEW=READY_BUT_NOT_EXECUTED_ON_REAL_HOST`

The next real-host action is to execute this runner once. A PASS gate is required before considering controlled materialization of the 90 non-deferred files.
