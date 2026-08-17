# PRW Phase 152 GitHub-First Authority Provenance Closeout

Status: `PASS / GITHUB_DRIVE_AUTHORITY_BYTE_CHAIN_VERIFIED / REAL_HOST_RECONCILED / HOST_MIRROR_SYNC_VERIFIED`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_source_commit: `01f5466504684ea6a2c504613901d24018485887`
- frozen_source_branch: `phase-152-desktop-functional-management`
- reconciliation_branch: `phase-152-github-first-reconciliation`
- reconciliation_head_before_completion_closeout: `acb1e849f88c3de61f03bf69711cf31c06e66bf2`

## Remaining non-Agent closure authority

- source files: `14`
- Drive raw readback result: `14 / 14 GIT_BLOB_MATCH`
- authority manifest Git blob: `c8e19f7f45ad0becc5c3156943f957b8831df56a`
- authority manifest Drive SHA-256: `1652503a018188f5a8836e552b603516eb52c94a3d8f79f7b4879a5d56e958bb`

## Agent authority chain

- Agent authority files: `79`
- Agent source tree: `59973a1dbc7698906dc30bea50f2369e2f83d9b1`
- Agent tests tree: `1729b632a0260971362246ba70724f8b71b49693`
- Agent manifest Git blob: `1ec700b289fdf56effeeca2f4d3c8bfc459c8981`
- Agent manifest Drive SHA-256: `2182d5225c33663a10a94cfdfee84083fa35007e7a738449c33fd4b9a33547b7`
- GitHub Actions packaging run: `32057739058`
- GitHub Actions artifact: `9296971091`
- GitHub-reported artifact digest: `sha256:5c1d01ebebd7c33eba3bd813d501d7b39b3554f3ba4c2e8fd7f92ff0b2377771`
- Drive Agent bundle ID: `1ILrfBpNUkia8PTONJSK3Z8tTSjGHKrec`
- Drive Agent bundle SHA-256: `5c1d01ebebd7c33eba3bd813d501d7b39b3554f3ba4c2e8fd7f92ff0b2377771`
- independently downloaded GitHub artifact vs Drive bundle: `BYTE_IDENTICAL`
- bundle ZIP entries: `85`
- Agent manifest file verification: `79 / 79 GIT_BLOB_MATCH`
- internal bundle `SHA256SUMS`: `84 / 84 MATCH`

## Corrected reconciliation boundary

The reconciler verifies the full immutable `93`-file authority boundary while three Agent binary-bootstrap paths remain explicitly classified `DEFERRED_RUNTIME_GATE` and categorically excluded from apply while runtime/systemd gates remain closed.

- authority files verified: `93`
- apply-eligible files: `90`
- deferred runtime-gate files: `3`
- deferred paths:
  - `crates/prw-agent/src/main.rs`
  - `crates/prw-agent/tests/phase125_device_identity_bootstrap.rs`
  - `crates/prw-agent/tests/phase_102_binary_bootstrap.rs`

### Reconciler v2

- path: `tools/workspace-sync/prw-reconcile-from-drive.sh`
- Git blob: `9844e7a717e80ffa58e8962ebf5248962af0e30b`
- SHA-256: `fb835a6e69e860e4ad1d7a0c1862f24a4cb8da05c80c57c084670d000b99c9bb`
- size: `12033`
- Drive file ID: `1vHcj0lbWsHRN5_YfY_uFuf8kHyL50Jdc`
- Drive raw readback: `MATCH`
- `bash -n`: `PASS`

### Bootstrap v2

- path: `tools/workspace-sync/prw-bootstrap-drive-reconciliation.sh`
- Git blob: `bd9ffcab696e067e03f64779a1b3e6e45991febc`
- SHA-256: `c6efc345a6ab572749dd194d89f5732b5d6700454189205e7c54ef06e3eb6da1`
- size: `2909`
- Drive file ID: `1X3mrJSSs8lhykdJbKGuiJWT2cu3P6Xtt`
- Drive raw readback: `MATCH`
- `bash -n`: `PASS`

### Controlled apply runner v2

The first controlled-apply runner attempt failed before source mutation because the reconciler was executed from a temporary path and therefore resolved the workspace root as `/`. The corrected runner installs/executes the verified reconciler at its canonical workspace path and performs the local-to-Drive Host Mirror sync only after the reconciler has exited and released its sync lock.

- path: `tools/workspace-sync/prw-run-controlled-reconciliation-apply.sh`
- Git blob: `a0efb18a460381b334706f10e71c3c8ea0add302`
- SHA-256: `b45a1bf58f70e5e28d897d3942ba32a8208978ce7ffe7c50cd4c0ce69cc8942c`
- size: `12295`
- Drive file ID: `1aNEe2q9VDaCcr7_oCS42wdIcrQJtANz_`
- Drive raw readback: `MATCH`
- `bash -n`: `PASS`

## Historical first real-host preview

The verified first bootstrap/reconciler was executed on the real Ubuntu workspace in preview mode only.

- workspace: `/home/gersi365/private-remote-workspace`
- bootstrap evidence stamp: `20260817T192502Z`
- reconciliation preview stamp: `20260817T192509Z`
- preview status: `STAGED / VERIFIED / LOCAL_SOURCE_NOT_MUTATED`
- authority files verified: `93`
- local changes initially reported: `89`
- `MATCH`: `4`
- `ABSENT`: `88`
- `DIFF`: `1`
- reconciliation audit SHA-256: `128359315c2324b7fcbf89cc513848136218f2b61a09b1b49a1b2688a2810d76`
- preview TSV SHA-256: `5fca12f6b72178c413025f8d57302bd3046fe018e181e7c3c33b79e21d45e0d8`

The single `DIFF` was `crates/prw-agent/src/main.rs`, where frozen authority blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1` differed from host blob `d3124af74881f58535963a7bd0b790e49eba4d4b`. The two frozen binary-bootstrap integration tests were absent. Because these three paths cross the runtime/systemd boundary, the initial 93-file apply boundary was rejected before mutation and corrected to the 90-file apply-eligible boundary.

The dedicated evidence record is `PHASE152_REAL_HOST_PREVIEW_AUDIT.md`, Git blob `80b5bd8f272eb9ed2fb243d32f922e0ed75c2a8f`.

## Real-host controlled reconciliation completion

The corrected controlled-apply runner was executed on the real Ubuntu host. Terminal evidence supplied from the host records the following sequence:

- controlled transaction stamp: `20260817T203643Z`
- pre-apply reconciliation audit: `logs/audits/drive-reconciliation/20260817T203649Z/RECONCILIATION_AUDIT.md`
- apply reconciliation audit: `logs/audits/drive-reconciliation/20260817T203907Z/RECONCILIATION_AUDIT.md`
- checksum Host Mirror sync audit: `logs/audits/workspace-sync/20260817T204206Z/SYNC_AUDIT.md`
- post-apply reconciliation audit: `logs/audits/drive-reconciliation/20260817T204259Z/RECONCILIATION_AUDIT.md`
- controlled audit: `logs/audits/controlled-reconciliation-apply/20260817T203643Z/CONTROLLED_APPLY_AUDIT.md`
- controlled runner exit code: `0`
- Drive check: `0 differences found`
- Drive check: `318 matching files`

The controlled runner contract exits successfully only after its exact pre-apply boundary checks, non-deferred apply, per-file Git-blob verification, separate Host Mirror sync, exact post-apply reconciliation checks, deferred-path preservation checks, and root `Cargo.toml` / `Cargo.lock` before-vs-after checks have all passed.

Therefore the real-host reconciliation state is closed as:

- `LOCAL_RECONCILIATION=PASS`
- `APPLY_ELIGIBLE_FILES=90 / MATCH_AUTHORITY_AFTER_APPLY`
- `DEFERRED_RUNTIME_GATE_FILES=3 / NOT_APPLIED`
- `LOCAL_CHANGES_REMAINING_IN_APPLY_ELIGIBLE_BOUNDARY=0`
- `USER_HOST_MIRROR=SYNCED / CHECKSUM_VERIFIED`
- `DRIVE_POST_SYNC_DIFFERENCES=0`
- `DRIVE_POST_SYNC_MATCHING_FILES=318`

The source candidate itself remains frozen at `01f5466504684ea6a2c504613901d24018485887`; reconciliation completion does not authorize production runtime activation.

## Historical local topology corroboration

Uploaded archive `20260817T115615Z.zip` was independently verified as historical evidence predating reconciliation.

- archive SHA-256: `0fd5e6e5596e4cd15ab93136a382ea8c54829390174f786fa10e8309adfe15e4`
- embedded `SHA256SUMS`: `24 / 24 PASS`
- historical `git_worktree=0`
- historical `tracked_file_count=0`
- historical `current_file_count=22`
- historical `component_count=7`

This corroborates that the Ubuntu workspace is an intentional non-Git Host Mirror. The later `fatal: not a git repository` result is not reconciliation damage. The dedicated evidence record is `PHASE152_LOCAL_LAYOUT_CORROBORATION.md`, Git blob `d6eeaecc5688ed5f609979b2a15e179ca17b882f`.

## Review-container corrective event

Draft PR `#2` was briefly created as a review container with base `phase-152-desktop-functional-management` and head `phase-152-github-first-reconciliation`. It was immediately closed after the repository's pre-existing pull-request-triggered `PRW Rust Validation` workflow started automatically.

- PR `#2`: `CLOSED / DRAFT / UNMERGED`
- PR base SHA: `01f5466504684ea6a2c504613901d24018485887`
- PR head SHA at close: `ea3085eb6360d7d7c1e6b0448ae68439d659dcc4`
- automatically triggered run: `32059250680`
- workflow: `PRW Rust Validation`
- job: `95476493438 / Validate Rust workspace`
- final run status: `COMPLETED / SUCCESS`
- classification: `AUTOMATIC_EXISTING_CI / NOT_AUTHORIZED_AS_BUILD_GATE_EVIDENCE`
- build gate after corrective action: `CLOSED`

The automatic run is retained only as incident/provenance evidence. It does not retroactively authorize build/test/clippy and is not used as Phase 152 build-gate validation. PR `#2` remains closed.

## Preserved gates

- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`
- C03 production activation: `NOT_AUTHORIZED`

## Project continuation

Host reconciliation is no longer the active blocker. The next project work returns to the Phase 152 reviewed authority boundary described by PR #1: local-management principal semantics for terminal/forwarding, trusted Agent-owned filesystem-root configuration and ownership, provider lifecycle ownership/cleanup/rollback, and exact production policy configuration that may grant management capabilities. C03 remains separate and closed.
