# PRW Phase 152 Real Host Reconciliation Preview Audit

Status: `REAL_HOST_PREVIEW_VERIFIED / APPLY_BLOCKED_PENDING_DEFERRED_RUNTIME_GUARDRAIL / CORRECTION_PUBLISHED`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_authority_commit: `01f5466504684ea6a2c504613901d24018485887`
- real_host_workspace: `/home/gersi365/private-remote-workspace`
- real_host_preview_utc: `20260817T192509Z`
- source_apply_performed: `NO`

## Real-host preview result

The verified Drive -> local reconciler was bootstrapped on the real Ubuntu host and run in preview mode only.

- authority files verified: `93`
- local changes initially reported: `89`
- `MATCH`: `4`
- `ABSENT`: `88`
- `DIFF`: `1`
- reconciliation audit SHA-256: `128359315c2324b7fcbf89cc513848136218f2b61a09b1b49a1b2688a2810d76`
- preview TSV SHA-256: `5fca12f6b72178c413025f8d57302bd3046fe018e181e7c3c33b79e21d45e0d8`

The single differing file was:

- `crates/prw-agent/src/main.rs`
  - frozen authority Git blob: `db6b8028c6df100a961a0fb5818347bea2fdc5c1`
  - current host Git blob: `d3124af74881f58535963a7bd0b790e49eba4d4b`

The two frozen binary-bootstrap integration tests were absent on the host:

- `crates/prw-agent/tests/phase125_device_identity_bootstrap.rs`
- `crates/prw-agent/tests/phase_102_binary_bootstrap.rs`

These three paths are coupled to the Agent binary bootstrap/runtime boundary. The frozen `main.rs` loads the Ubuntu enrollment signer from a systemd credential before entering the Linux bootstrap. Runtime signing and systemd credential loading remain not authorized. Therefore the initial 93-file apply boundary was too broad even though all authority bytes were valid.

`--apply` was not executed.

## Host topology observation

`git status --short --untracked-files=all` returned:

`fatal: not a git repository (or any of the parent directories): .git`

This is consistent with the established Host Mirror topology, which excludes VCS metadata. It is not classified as source corruption and does not weaken Git-blob verification, which is computed directly from file bytes.

## Corrective guardrail

The reconciliation tooling was corrected GitHub-first without changing the frozen source authority bundle or its 79-file Agent manifest.

The corrected reconciler continues to verify all `93 / 93` authority inputs but classifies these three paths as `DEFERRED_RUNTIME_GATE` and categorically skips them during `--apply`:

- `crates/prw-agent/src/main.rs`
- `crates/prw-agent/tests/phase125_device_identity_bootstrap.rs`
- `crates/prw-agent/tests/phase_102_binary_bootstrap.rs`

Corrected reconciliation boundary:

- authority files verified: `93`
- apply-eligible files: `90`
- deferred runtime-gate files: `3`
- expected real-host changes before apply: `86`
- expected already-matching files: `4`

## Corrected tooling identity

### Reconciler

- GitHub path: `tools/workspace-sync/prw-reconcile-from-drive.sh`
- Git blob: `9844e7a717e80ffa58e8962ebf5248962af0e30b`
- SHA-256: `fb835a6e69e860e4ad1d7a0c1862f24a4cb8da05c80c57c084670d000b99c9bb`
- size: `12033`
- Drive file ID: `1vHcj0lbWsHRN5_YfY_uFuf8kHyL50Jdc`
- Drive raw readback: `MATCH`
- `bash -n`: `PASS`

### Bootstrap

- GitHub path: `tools/workspace-sync/prw-bootstrap-drive-reconciliation.sh`
- Git blob: `bd9ffcab696e067e03f64779a1b3e6e45991febc`
- SHA-256: `c6efc345a6ab572749dd194d89f5732b5d6700454189205e7c54ef06e3eb6da1`
- size: `2909`
- Drive file ID: `1X3mrJSSs8lhykdJbKGuiJWT2cu3P6Xtt`
- Drive raw readback: `MATCH`
- `bash -n`: `PASS`

## Preserved gates

- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`

## Next safe host action

Install the corrected Drive-pinned bootstrap and run preview again. The second preview must report `93` verified files, `90` apply-eligible files, `3` deferred runtime-gate files, and no `DIFF` entry eligible for apply before any controlled source reconciliation is authorized.
