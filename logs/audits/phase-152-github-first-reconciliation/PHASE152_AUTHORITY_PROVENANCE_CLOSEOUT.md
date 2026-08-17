# PRW Phase 152 GitHub-First Authority Provenance Closeout

Status: `PASS / GITHUB_DRIVE_AUTHORITY_BYTE_CHAIN_VERIFIED / REAL_HOST_PREVIEW_COMPLETED / CORRECTED_PREVIEW_PENDING`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_source_commit: `01f5466504684ea6a2c504613901d24018485887`
- frozen_source_branch: `phase-152-desktop-functional-management`
- reconciliation_branch: `phase-152-github-first-reconciliation`
- reconciliation_head_observed_before_this_update: `85537595352d3658c18a773bdd5d357471e8a612`

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

## Corrected reconciliation tooling chain

The reconciler continues to verify the full immutable `93`-file authority boundary, but three Agent binary-bootstrap paths are now explicitly classified `DEFERRED_RUNTIME_GATE` and are categorically skipped by `--apply` while runtime/systemd gates remain closed.

- authority files verified by design: `93`
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

Source mutation still requires explicit `--apply`. Host Mirror refresh additionally requires `--sync-host-mirror` and reuses the existing checksum-verified local-to-Drive sync transaction. Neither has been run on the real host in this reconciliation sequence.

## Real-host preview evidence

The verified v1 bootstrap/reconciler was executed on the real Ubuntu workspace in preview mode only.

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
- source apply: `NOT_PERFORMED`

The single `DIFF` was `crates/prw-agent/src/main.rs`, where frozen authority blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1` differed from host blob `d3124af74881f58535963a7bd0b790e49eba4d4b`. The two frozen binary-bootstrap integration tests were absent. Because these three paths cross the runtime/systemd boundary, the initial apply boundary was rejected before mutation and the reconciler was corrected GitHub-first.

The dedicated evidence record is `PHASE152_REAL_HOST_PREVIEW_AUDIT.md`, Git blob `80b5bd8f272eb9ed2fb243d32f922e0ed75c2a8f`.

`LOCAL_RECONCILIATION=REAL_PREVIEW_COMPLETED / APPLY_NOT_PERFORMED / CORRECTED_SECOND_PREVIEW_REQUIRED`

## Historical local topology corroboration

Uploaded archive `20260817T115615Z.zip` was independently verified as historical evidence predating the reconciliation preview.

- archive SHA-256: `0fd5e6e5596e4cd15ab93136a382ea8c54829390174f786fa10e8309adfe15e4`
- embedded `SHA256SUMS`: `24 / 24 PASS`
- historical `git_worktree=0`
- historical `tracked_file_count=0`
- historical `current_file_count=22`
- historical `component_count=7`

This corroborates that the Ubuntu workspace is an intentional non-Git Host Mirror. The later `fatal: not a git repository` result is not classified as reconciliation damage. The dedicated evidence record is `PHASE152_LOCAL_LAYOUT_CORROBORATION.md`, Git blob `d6eeaecc5688ed5f609979b2a15e179ca17b882f`.

## Audit evidence mirror

- `PHASE152_AGENT_AUTHORITY_AUDIT.md` GitHub/Drive Git blob: `31b8cba791e46ea580972d72df6f3e767ee35625`
- `PHASE152_AGENT_AUTHORITY_AUDIT.md` Drive SHA-256: `96a5f8027bd05454959f3d59159b17c37e1b4d814c4b0e3aee9c7ec35dcc4999`
- `PHASE152_LOCAL_BOOTSTRAP_READINESS.md` GitHub/Drive Git blob: `7d7dfbedfdef2b53e919992f1d74480d2b2704b5`
- `PHASE152_LOCAL_BOOTSTRAP_READINESS.md` Drive SHA-256: `31a308e66fe64c4f1555e0705973e6766e7de231519b9d0e144dd4a76089e12c`
- `RECONCILIATION_BOOTSTRAP_AUDIT.md` GitHub/Drive Git blob: `a3e6976fb8a8d20b9ead78ee4afdde87e2690698`
- `PHASE152_REAL_HOST_PREVIEW_AUDIT.md` Git blob: `80b5bd8f272eb9ed2fb243d32f922e0ed75c2a8f`
- `PHASE152_REAL_HOST_PREVIEW_AUDIT.md` Drive file ID: `1M0wLlzLP5bwrrOIFZApNDKENuBs6BlhQ`
- `PHASE152_LOCAL_LAYOUT_CORROBORATION.md` Git blob: `d6eeaecc5688ed5f609979b2a15e179ca17b882f`
- `PHASE152_LOCAL_LAYOUT_CORROBORATION.md` Drive file ID: `1-dWYKy-2lipmNfniLwaPEmoD5GEAbegx`

## Review-container corrective event

Draft PR `#2` was briefly created as a review container with base `phase-152-desktop-functional-management` and head `phase-152-github-first-reconciliation`. It was immediately closed after the repository's pre-existing pull-request-triggered `PRW Rust Validation` workflow started automatically.

- PR `#2`: `CLOSED / DRAFT / UNMERGED`
- PR base SHA: `01f5466504684ea6a2c504613901d24018485887`
- PR head SHA at close: `ea3085eb6360d7d7c1e6b0448ae68439d659dcc4`
- automatically triggered run: `32059250680`
- workflow: `PRW Rust Validation`
- job: `95476493438 / Validate Rust workspace`
- final run status: `COMPLETED / SUCCESS`
- completed UTC: `2026-08-17T19:18:59Z`
- job completed Clippy, tests, and workspace build successfully
- classification: `AUTOMATIC_EXISTING_CI / NOT_AUTHORIZED_AS_BUILD_GATE_EVIDENCE`
- build gate after corrective action: `CLOSED`

The successful automatic run is retained only as incident/provenance evidence. It does not retroactively authorize build/test/clippy and is not used as Phase 152 build-gate validation. PR `#2` remains closed and must not be reopened unless pull-request CI behavior is explicitly authorized or isolated from build/test/clippy execution.

## Preserved gates

- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`

## Next safe host action

Run the Drive-pinned corrected bootstrap in preview mode. Before any controlled source apply, the second preview must establish:

- `verified_files = 93`
- `apply_eligible_files = 90`
- `deferred_runtime_gate_files = 3`
- `DEFERRED_RUNTIME_GATE = 3`
- no apply-eligible `DIFF`
- source apply still `NOT_PERFORMED`

Only after that preview is inspected may the non-deferred 90-file boundary be considered for controlled materialization. Root Cargo/build/runtime/systemd/deployment gates remain independent and closed.
