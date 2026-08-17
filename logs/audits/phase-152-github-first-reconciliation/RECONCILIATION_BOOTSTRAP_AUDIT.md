# PRW Phase 152 GitHub-First Reconciliation Bootstrap Audit

Status: `GITHUB_READY / DRIVE_AUTHORITY_14_OF_14_VERIFIED / LOCAL_EXECUTION_PENDING`

- authoritative_repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_source_commit: `01f5466504684ea6a2c504613901d24018485887`
- frozen_source_branch: `phase-152-desktop-functional-management`
- reconciliation_branch: `phase-152-github-first-reconciliation`
- reconciliation_branch_verified_head: `0ba3d830f4e26ef178311699410e8f3796a9d2e2`
- drive_project_root_id: `1ro9kl1LxtM7vfr_69WppkWFrxri2GTek`
- drive_authority_root_id: `1jkCfK6iCoIYennaaG7WWH9kAcvComZcG`
- drive_repository_id_folder: `1IOCuSK-6TfeUks-H0IlSuubSbwaroy3F`
- drive_commit_snapshot_folder: `1hDuMnlzH6k2zMJYQ4cMByasG62St934B`
- drive_manifest_id: `1hDQSquQ0mKkBTCbrNpH4hThePZC303Bt`

## Authority manifest integrity

- GitHub manifest path: `logs/audits/phase-152-github-first-reconciliation/PHASE152_NEXT_AUTHORITY_MANIFEST.tsv`
- GitHub manifest Git blob: `c8e19f7f45ad0becc5c3156943f957b8831df56a`
- Drive manifest raw readback Git blob: `c8e19f7f45ad0becc5c3156943f957b8831df56a`
- Drive manifest raw readback SHA-256: `1652503a018188f5a8836e552b603516eb52c94a3d8f79f7b4879a5d56e958bb`
- manifest entries: `14`
- classification: `MATCH / IMMUTABLE_SOURCE_COMMIT_BOUND`

## Verified Phase 152 remaining closure source boundary

Every listed file was downloaded as raw bytes from the Drive authority snapshot after upload. The Git-style blob SHA of each Drive readback matched the corresponding blob at frozen GitHub commit `01f5466504684ea6a2c504613901d24018485887`.

| path | Git blob SHA | Drive readback SHA-256 | result |
|---|---|---|---|
| `crates/prw-session/Cargo.toml` | `5b0e0ccad709cbbd0eb3f5f1d33759514cd692d5` | `109da6fb94f9c97e2061ffcd525985fc4b42ae0d1b46a24b523dd3b2ca586fa1` | MATCH |
| `crates/prw-session/src/lib.rs` | `9eccba113e852a658189d76d892fd4b9822d2e83` | `8fd8781d3a99090684a3d15af0dc15eeda659f7df31598d8e784a70213cb9a61` | MATCH |
| `crates/prw-remote-transport/Cargo.toml` | `37055b7371cd6325438d8a2cbff00bd37773f6f6` | `ee46b65061806723b7e1306253e2091a9318024365b52821fa6a8d5afc354bf7` | MATCH |
| `crates/prw-remote-transport/src/lib.rs` | `35ffebccaf237fc6892dac0991a7c7fcd23576c8` | `ea064d1dd856223f72731a5a349f62e3d8ecef3c23819068ae86c48dcdff77f1` | MATCH |
| `crates/prw-remote-transport/tests/loopback.rs` | `97b8fb97e150ae14ddcc24457664a8ab77c11c72` | `24d3322aa1bc75ff0ed082a06a72be11df431df7c2464f731722cdc17592196e` | MATCH |
| `crates/prw-registry/Cargo.toml` | `ec9215d9bad86ac9601e2f2d1bc0ed8461e724c1` | `d6cd5131e5dfeaa1b8f9ab608ae22373a4e66e1b3131017ae5fce75f728de0dd` | MATCH |
| `crates/prw-registry/src/lib.rs` | `cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6` | `6ee1f002f76ae506f0cfb7e3049ef1707cdb07ce6c8694dae7eb09848dce1c54` | MATCH |
| `crates/prw-terminal/Cargo.toml` | `990d644bdaca761b71f3cac7fb8c8980fa466611` | `e8abf9f576c67d24a2e771f052d5cf8ebc13c28444b37a20659f7b1734029b73` | MATCH |
| `crates/prw-terminal/src/lib.rs` | `093d44bdc57cd0872cd830a7af44afe709a3d9bf` | `d55a28c8997c3772bf23873ab3d13fc729305b6ebb78ccedb5415e16c2b5190a` | MATCH |
| `crates/prw-forwarding/Cargo.toml` | `ca94f03a2fadbadb2dcce71c4c006c48d3f323c3` | `d3bd1cec562ad5bc3808d0780b81de9976a5d5f38f4ca374560ce43247d485ef` | MATCH |
| `crates/prw-forwarding/src/lib.rs` | `904a2422ba94d138122874af0b23906f4f68f7eb` | `52241c3883af26f9984d46c4a21c0a3b44cb32a90ac82df00a117b8294751347` | MATCH |
| `crates/prw-remote-bridge/Cargo.toml` | `c060de750d88bc1cc4400a3948e08e8618bc4d59` | `51227c7f0a1216df83c1e2ac7526961d096b7508ff1aaa1c7410f15102ba182e` | MATCH |
| `crates/prw-remote-bridge/src/lib.rs` | `1573a12f39d75ec80f25adc6360ca108d2009af0` | `def171f775dd62fd74d020f98f4aab3ba25d05877afde75322617e797b1d4f51` | MATCH |
| `crates/prw-remote-bridge/tests/end_to_end.rs` | `be7cfbd7ae377bde9ffa3e2f42b7b674168d9fb8` | `3a3312b0a89a3ddc1e1af632d6f5a6d5bbd007e76d404f0c76a3f85bd72fef5b` | MATCH |

Result: `14 / 14 DRIVE_RAW_READBACK_GIT_BLOB_MATCH`.

## Reconciliation tooling

- GitHub path: `tools/workspace-sync/prw-reconcile-from-drive.sh`
- GitHub source commit: `0ba3d830f4e26ef178311699410e8f3796a9d2e2`
- GitHub blob: `e41d4deda32418147c3f019cb0d27f67442ba23e`
- Drive file id: `1vHcj0lbWsHRN5_YfY_uFuf8kHyL50Jdc`
- Drive raw readback Git blob: `e41d4deda32418147c3f019cb0d27f67442ba23e`
- Drive raw readback SHA-256: `b79c1a2733e671d69a601143c9e8714685d6f3b8211cb19dbe5591c8dabee931`
- `bash -n`: PASS
- `shellcheck`: NOT_AVAILABLE_IN_VALIDATION_RUNTIME

The reconciler is dry-run by default, takes the same workspace-sync lock as the existing local-to-Drive transaction, stages files from the immutable authority snapshot, verifies each downloaded Git blob before comparison or mutation, backs up differing local files before replacement, and refuses paths outside the six-crate remaining Phase 152 closure boundary represented by the authority manifest.

## Gates preserved

- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`

## Local state

`/home/gersi365/private-remote-workspace` is not mounted or exposed to the current connected-tool runtime. Therefore Drive -> Ubuntu reconciliation has not been executed from this session and must not be reported as complete.

The Drive authority snapshot is ready for controlled local dry-run/apply when an execution channel to that Ubuntu workspace is available. The existing local -> Drive `prw-sync.sh` remains a separate checksum-verified transaction and is not reinterpreted as Drive -> local automation.

## Next safe work

1. Audit and package the remaining pre-existing `prw-agent` source substrate required by the already-established 20-member Desktop + Agent closure, still against frozen source commit `01f5466504684ea6a2c504613901d24018485887`.
2. Keep the root Cargo workspace activation and build gate closed until that Agent substrate is complete and audited.
3. When the Ubuntu workspace becomes accessible through an execution channel, run the reconciler dry-run first, inspect the generated `PREVIEW.tsv`, then apply only the verified authority manifest and re-run the existing local -> Drive checksum sync.
