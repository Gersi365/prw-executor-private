# PRW Phase 152 Local Reconciliation Bootstrap Readiness

Status: `READY / DRIVE_VERIFIED / REAL_HOST_EXECUTION_PENDING`

- repository_id: `1334911207`
- authority_commit: `01f5466504684ea6a2c504613901d24018485887`
- reconciliation_branch: `phase-152-github-first-reconciliation`
- bootstrap_path: `tools/workspace-sync/prw-bootstrap-drive-reconciliation.sh`
- bootstrap_git_blob: `ec5a463d63e3662982c038c0fca1decc108592f8`
- bootstrap_sha256: `61adf7ff77f106d736a1dddc1f85dd03154fe0baf107bbef0470f5ee17c3c578`
- bootstrap_size: `2909`
- bootstrap_drive_id: `1X3mrJSSs8lhykdJbKGuiJWT2cu3P6Xtt`
- bootstrap_drive_raw_readback: `MATCH`
- bootstrap_bash_syntax: `PASS`

## Bootstrap behavior

The bootstrap requires the existing local `.prw-sync/config.env`, takes the same
workspace-sync lock, retrieves the unified reconciler from the immutable Drive authority
snapshot, verifies both the expected SHA-256 and Git blob, validates shell syntax, backs
up any prior reconciler, installs the verified reconciler, records bootstrap audit
evidence, and immediately runs the reconciler in preview mode.

The bootstrap does not apply Phase 152 source changes. Source apply remains a separate,
explicit reconciler operation after the real-host preview has been inspected.

## Authority covered by the reconciler

- remaining six non-Agent closure crates: `14` files
- frozen `prw-agent` source/test authority: `79` files
- total verified reconciliation source boundary: `93` files

## Preserved gates

- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`

## Real-host status

The current ChatGPT tool runtime does not expose `/home/gersi365/private-remote-workspace`
or an Ubuntu/SSH execution connector. Therefore no real-host command has been executed
and local source reconciliation remains pending.
