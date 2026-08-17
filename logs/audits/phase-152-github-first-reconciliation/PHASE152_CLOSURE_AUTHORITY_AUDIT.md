# PRW Phase 152 Remaining Closure Authority Audit

Status: `GITHUB_VERIFIED / DRIVE_VERIFIED / LOCAL_EXECUTION_PENDING`

- authoritative_repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_source_commit: `01f5466504684ea6a2c504613901d24018485887`
- frozen_source_branch: `phase-152-desktop-functional-management`
- reconciliation_branch: `phase-152-github-first-reconciliation`
- authority_bundle_run: `32056557259`
- authority_bundle_job: `95467944496`
- authority_bundle_artifact: `9296576124`
- authority_bundle_artifact_sha256: `5eed347d31436f4d3eff80ed41b6f351159fcb3717327d9d52bfb45882cbb7a1`
- drive_snapshot_folder_id: `1hDuMnlzH6k2zMJYQ4cMByasG62St934B`
- drive_manifest_id: `1hDQSquQ0mKkBTCbrNpH4hThePZC303Bt`
- reconciler_drive_id: `1vHcj0lbWsHRN5_YfY_uFuf8kHyL50Jdc`
- reconciler_git_blob: `e41d4deda32418147c3f019cb0d27f67442ba23e`
- reconciler_sha256: `b79c1a2733e671d69a601143c9e8714685d6f3b8211cb19dbe5591c8dabee931`

## Verified source boundary

The remaining six non-Agent crates in the Desktop + Agent 20-member closure are fully represented by fourteen raw source files. GitHub Actions checked out the frozen commit, verified every allowlisted file with `git hash-object`, staged only those files, and uploaded the authority artifact. The artifact digest was independently verified after download. Every Drive source file was then downloaded as raw bytes and its Git blob SHA was verified against the frozen GitHub manifest.

| crate | files | result |
|---|---:|---|
| `prw-session` | 2 | `DRIVE_RAW_READBACK_MATCH` |
| `prw-remote-transport` | 3 | `DRIVE_RAW_READBACK_MATCH` |
| `prw-registry` | 2 | `DRIVE_RAW_READBACK_MATCH` |
| `prw-terminal` | 2 | `DRIVE_RAW_READBACK_MATCH` |
| `prw-forwarding` | 2 | `DRIVE_RAW_READBACK_MATCH` |
| `prw-remote-bridge` | 3 | `DRIVE_RAW_READBACK_MATCH` |

Total: `14 / 14` source files verified.

## Reconciler boundary

`tools/workspace-sync/prw-reconcile-from-drive.sh` is dry-run by default and now allowlists only:

- `crates/prw-session/*`
- `crates/prw-remote-transport/*`
- `crates/prw-registry/*`
- `crates/prw-terminal/*`
- `crates/prw-forwarding/*`
- `crates/prw-remote-bridge/*`

It explicitly rejects root `Cargo.toml`, `Cargo.lock`, VCS metadata, target paths, absolute paths, path traversal, and any manifest path outside this allowlist. Apply mode backs up existing differing files and verifies the final Git blob after replacement. Optional Host Mirror sync reuses the existing local-to-Drive checksum-verified sync transaction.

## Preserved gates

- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`

## Local execution

The authoritative Ubuntu workspace `/home/gersi365/private-remote-workspace` is not exposed to the current connected-tool runtime. Therefore Drive -> Ubuntu reconciliation has not been executed and is not classified complete.

## Next safe action

Audit and snapshot the remaining pre-existing `prw-agent` source substrate against the same frozen commit. Do not activate the root workspace or build gate. After Agent source closure is verified in Drive, the local reconciler can be extended to that exact Agent manifest and executed on Ubuntu when a host execution channel is available.
