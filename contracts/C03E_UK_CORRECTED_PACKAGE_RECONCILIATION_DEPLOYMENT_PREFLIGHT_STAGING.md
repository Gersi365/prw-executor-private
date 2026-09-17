# C03e-UK — Corrected package reconciliation deployment preflight

## 1. Status and boundary

`PREFLIGHT — VALIDATED — PRIVATE STAGE MATERIALIZATION REQUIRED — PRIVILEGED EXECUTION NOT AUTHORIZED`

Boundary:

`CORRECTED_PACKAGE_RECONCILIATION_DEPLOYMENT_PREFLIGHT`

C03e-UK is a read-only host/package preflight plus disposable unprivileged build-provenance checkpoint. It re-proves the corrected package reconciler selected and materialized through C03e-UJ, but deliberately does not create the intended-user deployment stage and does not perform authenticated sudo or any privileged package transaction.

The immediate successor is a separately authorized unprivileged private-stage materialization checkpoint. Root/package/systemd mutation remains outside this boundary.

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UJ / PR #674.

Exact predecessor identity:

- branch: `phase-152-c03e-uj-invoking-user-identity-source-correction`;
- head: `7526958f61e011dcc84f38dfc04320e026fdd5c7`;
- tree: `d06ba38f16fcbb9a6fc37f2e7543dcba6020da4f`;
- status: `SOURCE CORRECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- retained open, draft and unmerged.

UJ corrected only `crates/prw-agent-package-reconciliation/src/lib.rs` and materialized the fixed sudo-rs caller-identity law selected by UI.

## 3. Repository and main state

Canonical repository:

`Gersi365/prw-executor-private`

Repository stable ID:

`1334911207`

At UK start, `main` remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

No UK predecessor/successor collision existed before materialization:

- no PR titled `C03e-UK`;
- no `phase-152-c03e-uk*` branch;
- no exact UK audit filename in the canonical Drive parent.

UK does not mutate `main`.

## 4. Read-only host identity

Host/device:

`PowerCode`

Observed unprivileged operator:

- user: `gersi365`;
- UID: `1000`;
- GID: `1000`;
- sudo group membership present.

The ordinary session had no ambient `SUDO_*` variables.

No authenticated sudo command was executed during UK.

## 5. Active sudo-rs implementation continuity

Read-only host inspection established:

- `/usr/bin/sudo -> /etc/alternatives/sudo`;
- `/etc/alternatives/sudo -> /usr/lib/cargo/bin/sudo`;
- active target owner/group: root:root;
- active target mode: `4755`;
- active target bytes: `1090848`;
- active target filesystem device: `66306`;
- active target inode: `2240838`;
- active target SHA-256: `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`;
- active implementation reports `sudo-rs 0.2.13-0ubuntu1.2`;
- local `sudo(8)` manual identifies `sudo-rs 0.2.13`;
- `dpkg -V sudo-rs` reported no package-file drift.

Installed package inventory also includes classic `sudo 1.9.17p2-1ubuntu3` and `sudo-common 1.2ubuntu`, but the active alternatives chain resolves to sudo-rs.

This is continuity evidence relative to the UI selection. UK does not claim that an unauthenticated read can dynamically observe the post-authentication SUDO_* environment. The source-level sudo-rs identity semantics are inherited from evidence-closed UI and no implementation/version drift was observed.

`sudo -n true` remained unavailable and returned interactive-authentication-required. UK did not add NOPASSWD, store a password, alter sudoers, or authenticate sudo.

## 6. Corrected reconciler exact source

Exact build source:

- head: `7526958f61e011dcc84f38dfc04320e026fdd5c7`;
- tree: `d06ba38f16fcbb9a6fc37f2e7543dcba6020da4f`;
- Cargo.lock SHA-256: `2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`;
- package: `prw-agent-package-reconciliation`;
- binary: `prw-agent-package-reconcile`;
- release profile;
- locked dependency graph.

Observed toolchain:

- rustc `1.97.1 (8bab26f4f 2026-07-14)`;
- cargo `1.97.1 (c980f4866 2026-06-30)`.

The disposable exact-source worktree was clean before and after the builds.

No observed build override was present for the checked variables `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `SOURCE_DATE_EPOCH`, or `CARGO_BUILD_TARGET`.

## 7. Fixed-target reconciler reproducibility

Canonical UK disposable target:

`/tmp/prw-c03e-uk-reconciler-target`

Exact build semantic:

`CARGO_TARGET_DIR=/tmp/prw-c03e-uk-reconciler-target cargo build --locked --release -p prw-agent-package-reconciliation --bin prw-agent-package-reconcile`

The target directory was deleted completely between build A and build B.

Build A:

- bytes: `2898840`;
- SHA-256: `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`;
- GNU build ID: `4010d38d9ea37d1bcc2f0ac916954c75b267e82e`.

Build B:

- bytes: `2898840`;
- SHA-256: `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`;
- GNU build ID: `4010d38d9ea37d1bcc2f0ac916954c75b267e82e`.

Result:

`FIXED_TARGET_REPEAT_BYTE_IDENTICAL=YES`

Classification:

`PATH_BOUND_REPRODUCIBLE_CORRECTED_RECONCILER_CANDIDATE`

No path-independent reproducibility claim is made.

The observed `/tmp` build file was user-owned mode `0775` on filesystem device `40`. It is build/provenance evidence only and is not selected as the future privileged-execution custody artifact.

## 8. Unprivileged corrected-binary fail-closed check

The exact release binary was invoked unprivileged only with the selected CLI shape and a future absolute stage locator.

Observed:

- exit code: `1`;
- stdout: empty;
- stderr: `prw-agent-package-reconcile error=root_required`.

This proves the corrected production path still refuses unprivileged execution before SUDO_* caller identity can authorize stage custody.

No package path was mutated by this negative check.

## 9. UF Agent candidate provenance

Existing UF provenance artifact:

`/tmp/prw-c03e-uf-canonical-target/release/prw-agent`

Observed:

- UID/GID: `1000:1000`;
- mode: `0775`;
- bytes: `11068384`;
- SHA-256: `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- filesystem device: `40`;
- regular file.

The bytes/hash remain exact UF provenance.

Because the current artifact is group-writable and lives beneath `/tmp`, it remains provenance only. It is not an eligible prepared stage candidate under the package reconciler's group/other-write rejection and private-stage custody law.

UK did not chmod, copy, move or replace it.

## 10. Exact deployment manifest

The frozen manifest remains unchanged by UJ.

Required filename:

`C03E_UF_DEPLOYMENT_MANIFEST`

Exact identity:

- bytes: `602`;
- SHA-256: `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`;
- final LF: present;
- production-required mode: `0600`;
- production-required owner UID: exact selected stage/SUDO UID.

The deterministic manifest binds the selected UF Agent candidate provenance and fixed package hashes/action.

No deployment manifest was materialized under intended-user state during UK.

## 11. Installed package readback

Fixed package parent:

`/usr/lib/private-remote-workspace`

Observed:

- directory;
- root:root;
- mode `0755`;
- filesystem device `66306`.

Installed Agent:

`/usr/lib/private-remote-workspace/prw-agent`

Observed:

- regular file;
- root:root;
- mode `0755`;
- bytes `2865776`;
- SHA-256 `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`;
- filesystem device `66306`.

Vendor unit parent:

`/usr/lib/systemd/user`

Observed root-owned mode `0755` on filesystem device `66306`.

Vendor unit:

`/usr/lib/systemd/user/prw-agent.service`

Observed:

- regular file;
- root:root;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

No `.prw-agent.c03e-ug.*.candidate` root staging residue was observed in the fixed Agent parent.

## 12. Service state at observation time

Read-only user-manager query through `/run/user/1000/bus` observed:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `Result=exit-code`;
- `is-active=failed`;
- `is-failed=failed`.

Linger remained `yes`.

An exact process-table probe by command name found no `prw-agent` process for UID 1000.

This is only an observation-time non-running proof. Enabled+linger remains an external activation race surface. UK makes `NO_RACE_FREE_CLAIM`.

A future privileged transaction requires fresh immediate service-state proof after stage materialization and immediately before any authenticated sudo handoff.

## 13. Intended-user private state parent

Read-only chain:

- `/home`: root:root, mode `0755`, device `66306`;
- `/home/gersi365`: `1000:1000`, mode `0750`, device `66306`;
- `/home/gersi365/.local`: `1000:1000`, mode `0700`, device `66306`;
- `/home/gersi365/.local/state`: `1000:1000`, mode `0700`, device `66306`;
- `/home/gersi365/.local/state/private-remote-workspace`: `1000:1000`, mode `0700`, device `66306`.

The PRW state parent and fixed package parent are both on filesystem device `66306`.

No stage exists at the reserved successor path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

UK intentionally did not create it.

## 14. Selected successor stage law

The immediate separately authorized successor should materialize only an unprivileged private deployment stage at the exact reserved path above.

Selected stage directory custody:

- owner UID `1000`;
- group GID `1000`;
- exact mode `0700`;
- absolute normalized path;
- no symlink components;
- existing private PRW state parent only;
- no privileged mutation.

Selected fixed children:

### Corrected reconciler

Filename:

`prw-agent-package-reconcile`

Required bytes/hash:

- bytes `2898840`;
- SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`.

Selected stage mode:

`0500`

Required owner/group:

`1000:1000`

The successor must copy from a freshly verified exact UK build/provenance artifact, set exact mode, sync, and independently rehash/read back. The `/tmp` build path itself is not the execution path.

### Agent candidate

Filename:

`candidate-prw-agent`

Required bytes/hash:

- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

Selected stage mode:

`0500`

Required owner/group:

`1000:1000`

The source UF provenance artifact must be freshly hash-verified before copying; its current `0775` mode must not be propagated.

### Deployment manifest

Filename:

`C03E_UF_DEPLOYMENT_MANIFEST`

Required bytes/hash:

- bytes `602`;
- SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`.

Required mode:

`0600`

Required owner/group:

`1000:1000`

Manifest bytes must be exact deterministic UTF-8 with final LF.

## 15. Stage materialization is not privileged execution

The successor private-stage checkpoint may create and verify only the intended-user stage and the three fixed child artifacts above.

It must not:

- run sudo;
- obtain EUID 0;
- touch `/usr/lib/private-remote-workspace`;
- touch `/usr/lib/systemd/user`;
- create root reconciliation siblings;
- run the package reconciler against production;
- start/stop/reload/restart the service;
- change enablement or linger;
- change sudoers or alternatives.

Source materialization, stage materialization, and privileged execution remain distinct approval boundaries.

## 16. Future authenticated invocation shape

UK does not authorize or hand off a real deployment command.

After the private stage is evidence-closed, a separately authorized final transaction checkpoint must first refresh:

- stage directory UID/GID/mode/no-symlink custody;
- corrected reconciler exact bytes/hash/mode;
- Agent candidate exact bytes/hash/mode;
- manifest exact bytes/hash/mode;
- active sudo implementation/version/path/custody;
- installed old Agent exact hash/mode/owner;
- vendor unit exact hash/mode/owner;
- no root staging residue;
- immediate `failed/failed`, `MainPID=0` service state.

Only then may an explicit interactive sudo command be considered.

The source-selected semantic shape remains target-root sudo-rs with real/effective UID root and sudo-rs-injected `SUDO_UID/SUDO_GID/SUDO_USER` used solely for stage custody.

No SUDO_* value selects package destination, expected hashes, candidate/manifest name, semantic action, systemd behavior, network/database/auth behavior, or arbitrary command execution.

## 17. Threat-model and race limits

The corrected SUDO_* law is not a cryptographic identity boundary against hostile root.

The private stage is intended to prevent accidental/cross-user custody drift and to bind the authenticated operator's selected artifacts. UK does not claim that a malicious already-authorized local administrator is constrained by user-owned staging bytes.

Enabled service state plus linger remains a possible activation race between preflight and future package exchange.

The reconciler performs package-file mutation only and no systemd manager action.

Therefore:

`NO_RACE_FREE_CLAIM`

remains mandatory.

## 18. Canonical UK classification

`EXACT_UJ_SOURCE / CORRECTED_RECONCILER_FIXED_TARGET_REPRODUCIBLE_A73A82A7 / SUDO_RS_0_2_13_CONTINUITY / SUDO_BINARY_EXACT_2EB5D31F / UF_AGENT_CANDIDATE_EXACT_9DB768C1_PROVENANCE_ONLY / MANIFEST_EXACT_56A37904 / OLD_AGENT_EXACT_4DBB114E / VENDOR_UNIT_EXACT_24F646DC / SERVICE_FAILED_FAILED_MAINPID0 / ENABLED_AND_LINGER_RACE_SURFACE / STATE_PARENT_PRIVATE_AND_SAME_DEVICE / PRIVATE_STAGE_ABSENT / NO_ROOT_STAGING_RESIDUE / PRIVATE_STAGE_MATERIALIZATION_REQUIRED / PRIVILEGED_EXECUTION_NOT_AUTHORIZED / NO_RACE_FREE_CLAIM`

UK is a successful preflight to the next unprivileged staging boundary; it is not a deployment-success claim.

## 19. Validation requirements for this checkpoint

Because UK is docs-only repository materialization, exact-head CI must bind the final UK commit before evidence publication.

Required repository checks:

- exact UJ→UK topology;
- exactly one docs-only changed path;
- `git diff --check` PASS;
- exact-head Rust validation SUCCESS;
- any exact-head Android validation must be reported by its actual terminal conclusion;
- skipped workflows must not be called PASS.

Host observations above are read-only and must be represented as observation-time evidence only.

## 20. Explicit non-actions / STOP

C03e-UK performs no:

- reconciler Rust/source mutation;
- Cargo dependency mutation;
- intended-user deployment-stage creation;
- candidate chmod/copy/move into deployment state;
- manifest creation under intended-user state;
- reconciler deployment staging;
- authenticated sudo execution;
- sudoers mutation;
- sudo alternatives mutation;
- root package replacement;
- vendor-unit rewrite;
- root sibling creation;
- systemd manager mutation;
- service start/stop/restart/try-restart;
- enable/disable mutation;
- linger mutation;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation;
- main mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force-push/history rewrite;
- destructive evidence cleanup.

STOP after immutable evidence closure of this preflight checkpoint.

The next safe boundary is a separately authorized C03e-UL unprivileged private deployment-stage materialization checkpoint using only the exact stage path and fixed artifacts/custody selected above. That successor still does not authorize authenticated sudo or the privileged package transaction.
