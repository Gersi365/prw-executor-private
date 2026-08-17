# PRW Phase 152 GitHub-First Authority Provenance Closeout

Status: `PASS / GITHUB_DRIVE_AUTHORITY_BYTE_CHAIN_VERIFIED / REAL_HOST_RECONCILIATION_PENDING`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_source_commit: `01f5466504684ea6a2c504613901d24018485887`
- frozen_source_branch: `phase-152-desktop-functional-management`
- reconciliation_branch: `phase-152-github-first-reconciliation`
- reconciliation_head_before_closeout: `810fc2faed58ae961a402311471a7b121d1d044e`

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
- GitHub Actions run: `32057739058`
- GitHub Actions artifact: `9296971091`
- GitHub-reported artifact digest: `sha256:5c1d01ebebd7c33eba3bd813d501d7b39b3554f3ba4c2e8fd7f92ff0b2377771`
- Drive Agent bundle ID: `1ILrfBpNUkia8PTONJSK3Z8tTSjGHKrec`
- Drive Agent bundle SHA-256: `5c1d01ebebd7c33eba3bd813d501d7b39b3554f3ba4c2e8fd7f92ff0b2377771`
- independently downloaded GitHub artifact vs Drive bundle: `BYTE_IDENTICAL`
- bundle ZIP entries: `85`
- Agent manifest file verification: `79 / 79 GIT_BLOB_MATCH`
- internal bundle `SHA256SUMS`: `84 / 84 MATCH`

## Reconciliation tooling chain

- reconciler path: `tools/workspace-sync/prw-reconcile-from-drive.sh`
- reconciler Git blob: `8ea2c7ec4d9d0a731260d35573acde9010954d92`
- reconciler SHA-256: `37c04bdd7893f9b6f3497516116734d6f4085ca34d0f6c75646a9188852ffbc6`
- reconciler Drive raw readback: `MATCH`
- reconciler `bash -n`: `PASS`
- bootstrap path: `tools/workspace-sync/prw-bootstrap-drive-reconciliation.sh`
- bootstrap Git blob: `ec5a463d63e3662982c038c0fca1decc108592f8`
- bootstrap SHA-256: `61adf7ff77f106d736a1dddc1f85dd03154fe0baf107bbef0470f5ee17c3c578`
- bootstrap Drive raw readback: `MATCH`
- bootstrap `bash -n`: `PASS`

The unified reconciler covers `93` source files: `14` remaining non-Agent closure files plus `79` Agent source/test authority files. It is dry-run by default. Source mutation requires explicit `--apply`; Host Mirror refresh additionally requires `--sync-host-mirror` and reuses the existing checksum-verified local-to-Drive sync transaction.

## Audit evidence mirror

- `PHASE152_AGENT_AUTHORITY_AUDIT.md` GitHub/Drive Git blob: `31b8cba791e46ea580972d72df6f3e767ee35625`
- `PHASE152_AGENT_AUTHORITY_AUDIT.md` Drive SHA-256: `96a5f8027bd05454959f3d59159b17c37e1b4d814c4b0e3aee9c7ec35dcc4999`
- `PHASE152_LOCAL_BOOTSTRAP_READINESS.md` GitHub/Drive Git blob: `7d7dfbedfdef2b53e919992f1d74480d2b2704b5`
- `PHASE152_LOCAL_BOOTSTRAP_READINESS.md` Drive SHA-256: `31a308e66fe64c4f1555e0705973e6766e7de231519b9d0e144dd4a76089e12c`
- `RECONCILIATION_BOOTSTRAP_AUDIT.md` GitHub/Drive Git blob: `a3e6976fb8a8d20b9ead78ee4afdde87e2690698`

## Preserved gates

- root Cargo workspace activation: `NOT_AUTHORIZED`
- build/test/clippy: `NOT_AUTHORIZED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment: `NOT_AUTHORIZED`
- privileged/system changes: `NOT_AUTHORIZED`

## Real-host boundary

The current connected-tool runtime does not expose `/home/gersi365/private-remote-workspace` and no Ubuntu/SSH execution connector is available. Therefore this closeout does not claim a real-host preview or apply.

`LOCAL_RECONCILIATION=READY_BUT_NOT_EXECUTED`

The next real-host action is the verified bootstrap in preview mode. Apply must remain separate and only follow inspection of the generated `PREVIEW.tsv` and reconciliation audit.
