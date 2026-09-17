# C03e-UH — Package reconciliation deployment preflight

## 1. Status and boundary

`PREFLIGHT — VALIDATED — PRIVILEGE INVOCATION UID BINDING BLOCKED — NO HOST MUTATION`

Boundary:

`PACKAGE_RECONCILIATION_DEPLOYMENT_PREFLIGHT`

C03e-UH is a docs-only checkpoint that records the separately authorized read-only deployment preflight required by C03e-UF after evidence-closed reconciler source materialization.

UH performs no real privileged package transaction. It does not authorize or perform root-owned package replacement, sudo-authenticated helper execution, vendor-unit rewrite, systemd manager mutation, service activation, identity mutation, 20/30/40 mutation, deployment, merge, ready conversion, PR close, or main mutation.

The preflight found the expected package/service baseline but also found one blocking privilege-invocation identity mismatch. Because that mismatch prevents a deterministic exact interactive invocation from being frozen under the currently selected sudo boundary, real deployment remains STOPPED.

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UG / PR #671.

Exact predecessor identity:

- branch: `phase-152-c03e-ug-package-reconciliation-source-materialization`;
- head: `7cb9ea50c882d139b191ade82bd00f7b3421b030`;
- tree: `e4fa7310fa053d63cce45e41106e10658f875b20`;
- base: evidence-closed C03e-UF `e2a97d449376992798a074b537b3b6edc06dec3c`;
- PR #671 remained open, draft, unmerged and mergeable;
- PR body status was `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED` immediately before UH materialization.

C03e-UG materialized only the fixed-purpose package reconciliation crate and did not execute it as root.

## 3. Repository and main state

Canonical repository:

`Gersi365/prw-executor-private`

Repository stable ID:

`1334911207`

`main` remained outside the checkpoint and was not mutated by this preflight.

Observed main identity at the predecessor closure boundary:

- head: `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree: `63b8e59ca53797fdea6b95432e16f35eaf473604`.

## 4. UH read-only host scope

Host/device inspected:

`PowerCode`

Intended unprivileged operator:

- user: `gersi365`;
- UID: `1000`;
- GID: `1000`.

UH used ordinary unprivileged reads for host package/systemd evidence and disposable unprivileged builds under `/tmp`.

No command in UH authenticated sudo or obtained effective UID 0.

## 5. Exact reconciler source identity

The reconciler source was built from exact UG:

- source head: `7cb9ea50c882d139b191ade82bd00f7b3421b030`;
- source tree: `e4fa7310fa053d63cce45e41106e10658f875b20`;
- root `Cargo.lock` SHA-256: `2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`;
- package: `prw-agent-package-reconciliation`;
- binary: `prw-agent-package-reconcile`;
- profile: Cargo `release`;
- locked graph required.

Observed build toolchain:

- rustc: `1.97.1 (8bab26f4f 2026-07-14)`;
- cargo: `1.97.1 (c980f4866 2026-06-30)`;
- cc: Ubuntu GCC `15.2.0-16ubuntu1`;
- GNU ld: Binutils `2.46`.

Observed unset build variables:

- `RUSTFLAGS`;
- `CARGO_ENCODED_RUSTFLAGS`;
- `RUSTC_WRAPPER`;
- `RUSTC_WORKSPACE_WRAPPER`;
- `SOURCE_DATE_EPOCH`;
- `CARGO_BUILD_TARGET`.

Tracked source remained unchanged before and after builds.

## 6. Exact reconciler build identity

UH selected one disposable canonical reconciler target path for preflight reproducibility:

`/tmp/prw-c03e-uh-reconciler-target`

Exact command semantic:

`CARGO_TARGET_DIR=/tmp/prw-c03e-uh-reconciler-target cargo build --locked --release -p prw-agent-package-reconciliation --bin prw-agent-package-reconcile`

The target directory was removed completely before each build.

Build A:

- bytes: `2897432`;
- SHA-256: `248be565cee943417ed77f84e4d6e9d71257da24e9db1fe5c1a17bf3838d7f95`;
- GNU build ID: `7b93b312af7a0f6fa612d6d22f42ade658ccc5f1`.

Build B after complete target deletion:

- bytes: `2897432`;
- SHA-256: `248be565cee943417ed77f84e4d6e9d71257da24e9db1fe5c1a17bf3838d7f95`;
- GNU build ID: `7b93b312af7a0f6fa612d6d22f42ade658ccc5f1`.

Result:

`FIXED_TARGET_REPEAT_BYTE_IDENTICAL=YES`

Classification:

`PATH_BOUND_REPRODUCIBLE_RECONCILER_CANDIDATE`

UH does not claim path-independent reconciler reproducibility because different-target-path equivalence was not selected or tested here.

The `/tmp` binary is a disposable build/provenance artifact only. It is not a privileged-custody deployment artifact.

## 7. Exact Agent candidate provenance artifact

The UF canonical Agent candidate remained present at:

`/tmp/prw-c03e-uf-canonical-target/release/prw-agent`

Observed read-only identity:

- owner UID/GID: `1000:1000`;
- mode: `0775`;
- bytes: `11068384`;
- SHA-256: `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- filesystem device: `40` (`/tmp` tmpfs).

The exact bytes/hash remain the UF-selected current-Agent candidate.

However this existing `/tmp` file is explicitly NOT an eligible future stage candidate because:

1. it is group-writable (`0775`), while UG rejects a candidate whose mode has group/other write bits; and
2. `/tmp` is retained as build provenance, not the separately authorized intended-user private transaction stage.

UH does not chmod, copy, move or otherwise alter that artifact.

## 8. Exact deployment manifest identity

UG source deterministically requires `C03E_UF_DEPLOYMENT_MANIFEST` beneath the prepared stage.

Exact required UTF-8 content is:

```text
schema=c03e-uf-current-agent-package-reconciliation-v1
source_head=10714024a4df71bd3b5d0232bb0c6b6d7c9fb71f
source_tree=cd0218229280c470e8995b3341f2461afd660523
cargo_lock_sha256=e2d650e7a60663b651f8dfa3013eda729d1b02e763d2c3d8cf1823f43121e909
canonical_target=/tmp/prw-c03e-uf-canonical-target
candidate_bytes=11068384
candidate_sha256=9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7
old_agent_sha256=4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e
vendor_unit_sha256=24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909
operation=reconcile-current-agent
```

Exact manifest identity:

- bytes: `602`;
- SHA-256: `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`;
- final LF: present;
- required source-enforced mode: `0600`;
- required owner: exact stage owner UID.

## 9. Intended-user state parent viability

Existing intended-user state chain was read-only inspected:

- `/home`: root-owned directory mode `0755`, non-symlink;
- `/home/gersi365`: UID/GID `1000:1000`, mode `0750`, non-symlink;
- `/home/gersi365/.local`: `1000:1000`, mode `0700`, non-symlink;
- `/home/gersi365/.local/state`: `1000:1000`, mode `0700`, non-symlink;
- `/home/gersi365/.local/state/private-remote-workspace`: `1000:1000`, mode `0700`, non-symlink.

The PRW state parent and fixed package parent are both on filesystem device `66306`.

The preselected successor-stage pathname is reserved but remains absent:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ui-package-reconciliation-stage`

The following future children are also absent:

- `candidate-prw-agent`;
- `C03E_UF_DEPLOYMENT_MANIFEST`;
- `prw-agent-package-reconcile`.

UH does not create any of them.

A separately authorized successor may materialize them only after the privilege-UID blocker below is corrected/evidence-closed or the successor is explicitly limited to non-executable staging evidence.

## 10. Current installed Agent readback

Fixed parent:

`/usr/lib/private-remote-workspace`

Observed parent identity:

- directory;
- root:root;
- mode `0755`;
- device `66306`;
- non-symlink.

Installed Agent:

`/usr/lib/private-remote-workspace/prw-agent`

Observed exact identity:

- regular file;
- root:root;
- mode `0755`;
- bytes `2865776`;
- SHA-256 `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`;
- non-symlink.

This is the exact UF-selected old PRW-managed Agent identity.

## 11. Current vendor-unit readback

Fixed vendor unit:

`/usr/lib/systemd/user/prw-agent.service`

Observed exact identity:

- regular file;
- root:root;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- non-symlink.

The vendor unit remains verification-only and no rewrite is selected.

## 12. Root sibling collision guard

Read-only listing of `/usr/lib/private-remote-workspace` found no existing sibling matching the reconciler transaction naming family such as `.prw-agent...candidate`, reconciliation, rollback, staging, new or old residue.

The source creates fresh exclusive root siblings named from:

`.prw-agent.c03e-ug.<pid>.<attempt>.candidate`

with a bounded 32-attempt collision loop.

No root-owned staging file was created by UH.

## 13. Current user-service state

With the existing user-manager DBus endpoint `/run/user/1000/bus`, read-only systemd inspection proved:

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

This state is non-running eligible at observation time only.

Because the unit is enabled and linger is enabled, external activation remains possible. UH therefore makes no race-free claim between this read-only observation and any future privileged transaction. A future real transaction would still require a fresh immediately-before-handoff non-running/MainPID proof.

## 14. Noninteractive privilege state

`sudo -n true` remained unavailable and returned failure.

UH preserves UF's interactive privilege boundary:

- no sudoers mutation;
- no NOPASSWD grant;
- no stored password;
- no root runner;
- no arbitrary privileged shell;
- no automatic privilege escalation.

## 15. Bounded CLI behavior proved unprivileged

The exact reconciler release binary was executed only as UID/EUID `1000`, never as root.

Observed bounded parser behavior:

- no arguments -> `invalid_invocation`, exit `2`;
- wrong action -> `invalid_invocation`, exit `2`;
- relative stage path -> `invalid_invocation`, exit `2`;
- exact valid CLI shape with an absolute future stage path -> `root_required`, exit `1`.

This proves the source rejects invalid CLI widening and refuses the production transaction without effective UID 0.

No package path was modified by these unprivileged negative checks.

## 16. Privilege invocation UID-binding blocker

UG production policy sets:

- required effective UID from `geteuid()` to `0`;
- stage owner UID from `getuid()`.

Stage validation then explicitly rejects a stage owner UID of `0` and requires the stage directory to be owned by the exact `getuid()` value.

That shape is compatible with a set-user-ID style helper where real UID remains the invoking user and effective UID becomes root.

It is not deterministically compatible with ordinary default sudo execution. Standard sudo behavior executes the command with real and effective user IDs set to the target user (root by default) unless the `stay_setuid` sudoers option changes that behavior.

On this host:

- installed sudo package is `1.9.17p2-1ubuntu3`;
- `/usr/bin/sudo` is root-owned setuid mode `4755`;
- `/etc/sudoers` is root-owned mode `0440` and is not readable by the unprivileged preflight user;
- noninteractive sudo is unavailable;
- UH did not authenticate sudo merely to probe child credentials;
- no evidence in this preflight proves `stay_setuid` is enabled for the intended invocation;
- UF explicitly prohibits adding/changing sudoers merely to make the transaction work.

Therefore UH cannot freeze a valid exact interactive command that simultaneously proves:

1. `geteuid()==0` for the reconciler; and
2. `getuid()==1000` so the prepared stage is bound to the intended user.

If ordinary sudo sets both IDs to root, current UG source deterministically reaches `StageCustodyInvalid` because the source rejects `stage_owner_uid == 0`.

Classification:

`BLOCKED_ON_PRIVILEGE_INVOCATION_UID_BINDING`

This is a preflight/source-boundary mismatch, not permission to weaken stage custody or mutate sudoers.

## 17. No exact interactive invocation is frozen

The nominal command shape would otherwise be:

`sudo -- <exact-reconciler-path> reconcile-current-agent <exact-stage-path>`

UH deliberately does NOT promote a nominal command into an authorized/frozen execution command because real-UID preservation is unproved and ordinary sudo semantics conflict with the current `getuid()` binding.

No user should be handed a real deployment command from UH.

## 18. Expected terminal classifications remain bounded

Current source terminal classifications remain:

- `invalid_invocation`;
- `root_required`;
- `stage_custody_invalid`;
- `candidate_invalid`;
- `manifest_invalid`;
- `package_parent_invalid`;
- `installed_agent_invalid`;
- `vendor_unit_invalid`;
- `root_staging_failed`;
- `exchange_failed`;
- `post_exchange_failure_rolled_back`;
- `rollback_failed_or_ambiguous`;
- `cleanup_failed`;
- `durability_failed`.

Success stdout remains bounded to:

`prw-agent-package-reconcile result=current_agent_payload_reconciled`

UH does not treat helper stdout as sufficient deployment evidence; UF requires independent post-privilege readback after any future real transaction.

## 19. Historical helper remains non-authoritative

Historical Phase126 helper remains present read-only at:

`/home/gersi365/.local/state/private-remote-workspace/phase126/staging/1e0b5eec93538bcbf120fa800cf85227eeb32410/phase126-a05-root-agent-replace`

Observed identity remained:

- user-owned mode `0700`;
- bytes `2723`;
- SHA-256 `a75bcc1d44f788e34b12303d4446a8c31de5cb60858d56b59956ea990905ea64`.

It is stale historical evidence only. UH does not execute, delete, rename, copy or reuse it.

## 20. Canonical UH result

UH proves:

`EXACT_UG_SOURCE / PATH_BOUND_RECONCILER_BUILD_248BE565 / EXACT_AGENT_CANDIDATE_9DB768C1 / EXACT_MANIFEST_56A37904 / EXACT_OLD_AGENT_4DBB114E / EXACT_VENDOR_UNIT_24F646DC / ROOT_PACKAGE_PARENTS_EXACT / SERVICE_FAILED_FAILED_MAINPID0 / ENABLED_AND_LINGER_RACE_SURFACE / NONINTERACTIVE_SUDO_UNAVAILABLE / STAGE_PARENT_VIABLE_BUT_STAGE_ABSENT / NO_ROOT_STAGING_RESIDUE / BOUNDED_CLI_NEGATIVE_CHECKS / PRIVILEGE_INVOCATION_UID_BINDING_BLOCKED / NO_RACE_FREE_CLAIM / NO_HOST_MUTATION`

UH does NOT prove:

- deployability of the current source under the selected interactive sudo boundary;
- a valid real privileged invocation;
- prepared intended-user stage custody;
- real root-owned package replacement;
- service-manager activation or readiness;
- path-independent build reproducibility.

## 21. Required successor before staging or deployment

The immediate safe successor must be separately authorized and must resolve the privilege invocation identity binding without broadening the privilege model.

It must not rely on:

- sudoers mutation or `stay_setuid` enablement;
- NOPASSWD;
- stored passwords;
- arbitrary root shell/command execution;
- caller-controlled destination/hashes/modes;
- weakening stage owner validation to accept root-owned or arbitrary-user stages without a selected trust binding.

A successor selection should evaluate a narrow source-bound way to identify the authenticated invoking user under interactive sudo, with an explicit trust model and tests, before any candidate/reconciler stage is materialized for real execution.

No correction is authorized by UH itself.

## 22. Evidence/validation requirements for UH

UH is docs-only. Closure requires:

- exact one-contract-path delta from evidence-closed UG;
- exact predecessor/head/tree proof;
- exact contract bytes/blob/SHA-256;
- `git diff --check`;
- exact-head CI actually registered for the docs-only head;
- skipped checks represented as skipped, never PASS;
- immutable raw Markdown Drive evidence;
- byte-identical raw readback;
- exact-title singleton proof;
- one-revision lineage with previous revision `null`;
- metadata-only PR closure binding;
- PR retained open, draft and unmerged.

## 23. Explicit non-actions / STOP

C03e-UH performs no:

- runtime/Rust source correction;
- intended-user transaction stage creation;
- candidate copy/chmod/move into a deployment stage;
- reconciler copy/chmod/move into a deployment stage;
- deployment manifest creation under intended-user state;
- root-owned package replacement;
- sudo-authenticated helper execution;
- historical helper execution;
- vendor-unit rewrite;
- root sibling creation;
- identity mutation;
- 20/30/40 mutation;
- daemon reload;
- service start/stop/restart/try-restart;
- enable/disable mutation;
- linger mutation;
- network/listener/database/auth/control-plane mutation;
- main mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force-push/history rewrite;
- destructive evidence cleanup.

STOP after evidence closure of this preflight blocker checkpoint.