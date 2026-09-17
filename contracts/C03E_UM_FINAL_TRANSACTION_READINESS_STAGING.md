# C03e-UM — Final transaction readiness

## 1. Status and boundary

`FINAL READINESS — VALIDATED — PRIVILEGED TRANSACTION NOT AUTHORIZED`

Boundary:

`FINAL_TRANSACTION_READINESS`

C03e-UM is a read-only final readiness checkpoint over the evidence-closed C03e-UL private stage. It freshly re-proves the exact staged tree, active sudo-rs implementation/custody, installed Agent/vendor-unit identities, root-staging absence, operator identity and current user-service state. It also freezes one exact future privilege-handoff argv vector.

UM does not authenticate sudo, does not execute the privileged reconciler, does not create root staging, does not replace the installed Agent, and does not mutate systemd, sudoers, alternatives, identity, repository main, or deployment state.

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UL / PR #676.

Exact predecessor identity:

- branch `phase-152-c03e-ul-private-deployment-stage-materialization`;
- head `d0794f91d72df896b1a56a4096fc42b3276b3b60`;
- tree `afc53bdf911546d38e66da7d8f21dfef7bda6786`;
- status `STAGE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED — PRIVILEGED EXECUTION NOT AUTHORIZED`;
- retained open, draft and unmerged.

UL created the exact private stage and three selected children unprivileged, then stopped before any sudo execution.

## 3. Repository and duplicate guards

Canonical repository:

`Gersi365/prw-executor-private`

Repository stable ID:

`1334911207`

At UM start, canonical `main` remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

Before UM materialization:

- no all-state PR titled `C03e-UM` existed;
- no `phase-152-c03e-um*` branch existed;
- exact audit-title search for `C03E_UM_FINAL_TRANSACTION_READINESS_AUDIT_2026-09-17.md` in the canonical Drive parent returned zero objects;
- planned contract path was absent at exact UL head.

UM does not mutate `main`.

## 4. Operator identity

Host/device:

`PowerCode`

Fresh readback:

- current real UID `1000`;
- current GID `1000`;
- user `gersi365`;
- primary group `gersi365`;
- passwd entry `gersi365:x:1000:1000:Gersi:/home/gersi365:/bin/bash`;
- root passwd entry resolves UID/GID `0:0`.

Ordinary session environment had no ambient `SUDO_UID`, `SUDO_GID`, `SUDO_USER`, or `SUDO_COMMAND`.

No sudo authentication was attempted by UM.

## 5. Private parent and stage-chain custody

Fresh `lstat` readback:

- `/home`: UID/GID `0:0`, mode `0755`, device `66306`, non-symlink;
- `/home/gersi365`: UID/GID `1000:1000`, mode `0750`, device `66306`, non-symlink;
- `/home/gersi365/.local`: UID/GID `1000:1000`, mode `0700`, device `66306`, non-symlink;
- `/home/gersi365/.local/state`: UID/GID `1000:1000`, mode `0700`, device `66306`, non-symlink;
- `/home/gersi365/.local/state/private-remote-workspace`: UID/GID `1000:1000`, mode `0700`, device `66306`, non-symlink.

Selected stage:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

Fresh exact stage readback:

- UID/GID `1000:1000`;
- mode `0700`;
- directory;
- non-symlink;
- device `66306`;
- inode `6183861`;
- exact child set of three names only.

Exact child names:

- `C03E_UF_DEPLOYMENT_MANIFEST`;
- `candidate-prw-agent`;
- `prw-agent-package-reconcile`.

## 6. Staged corrected reconciler

Exact path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile`

Fresh readback:

- UID/GID `1000:1000`;
- mode `0500`;
- regular file;
- non-symlink;
- bytes `2898840`;
- device `66306`;
- inode `6189545`;
- SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`.

The staged bytes are exact UK/UL corrected-reconciler bytes. The retained GNU build ID is:

`4010d38d9ea37d1bcc2f0ac916954c75b267e82e`

An unprivileged negative invocation using the exact selected action and stage returned:

- exit code `1`;
- stdout empty;
- stderr `prw-agent-package-reconcile error=root_required`.

Thus the staged binary still rejects the production operation before privileged process identity and sudo-rs caller identity are established.

## 7. Staged Agent candidate

Exact path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/candidate-prw-agent`

Fresh readback:

- UID/GID `1000:1000`;
- mode `0500`;
- regular file;
- non-symlink;
- bytes `11068384`;
- device `66306`;
- inode `6189549`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

## 8. Staged deployment manifest

Exact path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/C03E_UF_DEPLOYMENT_MANIFEST`

Fresh readback:

- UID/GID `1000:1000`;
- mode `0600`;
- regular file;
- non-symlink;
- bytes `602`;
- device `66306`;
- inode `6189550`;
- SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`.

Manifest semantics and bytes remain exactly those selected by UF/UK/UL.

## 9. Active sudo-rs identity and custody

Active sudo path chain freshly resolves:

`/usr/bin/sudo -> /etc/alternatives/sudo -> /usr/lib/cargo/bin/sudo`

Resolved binary:

`/usr/lib/cargo/bin/sudo`

Fresh readback:

- UID/GID `0:0`;
- mode `4755`;
- regular file;
- bytes `1090848`;
- device `66306`;
- inode `2240838`;
- SHA-256 `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`;
- version `sudo-rs 0.2.13-0ubuntu1.2`.

This exactly matches the implementation identity observed by UK and the sudo-rs version family whose caller-identity semantics were selected/evidence-closed by UI.

No implementation/version drift was observed.

## 10. Sudo command-line boundary

Fresh local `sudo --help` reports:

- `-u, --user=user` runs a command as the specified user name or ID;
- `--` stops processing command-line arguments.

UM therefore freezes an explicit target-root invocation rather than relying on a configurable/default target-user assumption.

No shell, login shell, command substitution, `env`, preserved-environment option, working-directory override, stdin password, askpass, non-interactive flag, or caller-selected executable is part of the selected handoff.

## 11. Exact source CLI confirmation

At exact UL head, `crates/prw-agent-package-reconciliation/src/main.rs` delegates directly to:

`prw_agent_package_reconciliation::run_cli(env::args_os())`

Exact inherited corrected source accepts only:

`prw-agent-package-reconcile reconcile-current-agent <absolute-stage-directory>`

The parser:

- rejects missing action;
- rejects missing stage;
- rejects extra arguments;
- requires action exactly `reconcile-current-agent`;
- requires the stage to be a normal absolute path;
- calls `reconcile_current_agent` only after those checks.

The production path first requires real UID `0` and effective UID `0`, then resolves only fixed environment keys `SUDO_UID`, `SUDO_GID`, and `SUDO_USER` for invoking-user stage custody.

No SUDO_* value selects package destination, expected hashes, semantic action, candidate/manifest filename, systemd behavior, network/database/auth behavior, cwd, or executable path.

## 12. Exact future privilege-handoff argv

UM freezes the following exact argv vector for a separately authorized future transaction checkpoint:

- argv[0] `/usr/bin/sudo`
- argv[1] `-u`
- argv[2] `root`
- argv[3] `--`
- argv[4] `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile`
- argv[5] `reconcile-current-agent`
- argv[6] `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

Equivalent display-only shell rendering:

`/usr/bin/sudo -u root -- /home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile reconcile-current-agent /home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

This rendering is evidence/documentation only. UM does not authorize executing it.

The argv vector is the authority. A future transaction must not add flags, environment assignments, shell wrappers, redirects, pipes, substitutions, alternate paths, extra arguments, `-D`, `-i`, `-s`, `-S`, `-A`, `-n`, or preserved-environment controls.

## 13. Sudo-rs identity continuity requirement

The future transaction may use the exact argv only if the immediate pre-execution readback still proves:

- `/usr/bin/sudo` resolves through the same alternatives chain to `/usr/lib/cargo/bin/sudo`;
- resolved sudo binary remains root-owned setuid mode `4755`;
- bytes remain `1090848`;
- SHA-256 remains `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`;
- reported version remains `sudo-rs 0.2.13-0ubuntu1.2`.

Any drift must STOP the transaction and return to selection/readiness review.

No fallback to classic sudo semantics, direct root shell, explicit SUDO_* environment injection, `su`, `pkexec`, setuid PRW helper, or alternate privilege mechanism is selected.

## 14. Fixed package destination readiness

Package parent:

`/usr/lib/private-remote-workspace`

Fresh readback:

- UID/GID `0:0`;
- mode `0755`;
- directory;
- device `66306`.

Installed Agent:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh readback:

- UID/GID `0:0`;
- mode `0755`;
- bytes `2865776`;
- regular file;
- non-symlink;
- device `66306`;
- inode `3035845`;
- SHA-256 `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`.

This remains the exact old-Agent identity compiled into the reconciler.

## 15. Vendor unit readiness

Unit parent:

`/usr/lib/systemd/user`

Fresh readback:

- UID/GID `0:0`;
- mode `0755`;
- directory;
- device `66306`.

Vendor unit:

`/usr/lib/systemd/user/prw-agent.service`

Fresh readback:

- UID/GID `0:0`;
- mode `0644`;
- bytes `332`;
- regular file;
- non-symlink;
- device `66306`;
- inode `2231750`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

This remains the exact vendor-unit identity compiled into the reconciler.

## 16. Root staging residue

Fresh read-only search under `/usr/lib/private-remote-workspace` found no sibling matching:

`.prw-agent.c03e-ug.*.candidate`

No root reconciliation staging residue is present at UM observation time.

## 17. Disk-space observation

Fresh filesystem readback for the package filesystem reported approximately:

`36551049216` bytes available.

No disk-space pressure blocker was observed for the bounded one-Agent replacement transaction.

This is observation-time evidence only.

## 18. Immediate user-service state

Fresh user-manager readback through `/run/user/1000/bus`:

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

Fresh process-name probe found no `prw-agent` process owned by UID 1000.

Fresh login-manager readback:

- user state `active`;
- sessions `2 1`;
- linger `yes`.

## 19. Race boundary

Enabled service state plus linger remains an external activation surface.

UM therefore does not claim that the observation-time `MainPID=0` state stays true after this checkpoint.

`NO_RACE_FREE_CLAIM`

remains mandatory.

The reconciler intentionally performs no systemd-manager operation. Therefore a future privileged transaction must perform one final immediate read-only service/process recheck directly before privilege handoff.

If that recheck does not prove all of the following, the command must not be executed:

- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- exact vendor `FragmentPath`;
- empty `DropInPaths`;
- no exact `prw-agent` process for UID 1000.

No automatic stop/restart/disable/linger mutation is selected as a fallback.

## 20. Mandatory immediate pre-execution reproof

A separately authorized future transaction must re-prove, after authorization and immediately before invoking argv[0]:

1. operator is still UID/GID `1000:1000`;
2. complete private stage parent chain remains non-symlink and selected custody is unchanged;
3. stage remains `1000:1000`, mode `0700`, exact three-child set;
4. reconciler remains `1000:1000`, mode `0500`, bytes/hash exact;
5. Agent candidate remains `1000:1000`, mode `0500`, bytes/hash exact;
6. manifest remains `1000:1000`, mode `0600`, bytes/hash exact;
7. sudo alternatives path, resolved binary custody/version/bytes/hash remain exact;
8. `root` still resolves UID/GID `0:0`;
9. package parent remains root-owned mode `0755`;
10. installed Agent remains exact old bytes/hash/mode/owner;
11. unit parent remains root-owned mode `0755`;
12. vendor unit remains exact bytes/hash/mode/owner;
13. no root reconciliation staging sibling exists;
14. immediate service/process state satisfies the non-running proof above.

Any mismatch is a STOP condition. No repair or fallback is implicitly authorized.

## 21. Expected transaction terminal semantics

The exact source returns process success only after the fixed Agent payload reconciliation completes and prints:

`prw-agent-package-reconcile result=current_agent_payload_reconciled`

Bounded source errors produce exit code `1`; malformed CLI produces exit code `2`.

The future transaction checkpoint must treat any non-zero exit as failure and perform read-only post-transaction reconciliation/evidence before making any success claim.

UM does not simulate or predict a privileged success result.

## 22. Post-transaction evidence requirement

If the future privileged transaction is separately authorized and executed, closure requires fresh readback proving at minimum:

- installed Agent now has selected new bytes/hash and root-owned mode `0755`;
- vendor unit remains exact and unchanged;
- root temporary reconciliation sibling is absent after cleanup;
- stage bytes/custody are recorded as observed post-transaction;
- service state is reported exactly as observed, without implying start/restart occurred;
- exact command exit code/stdout/stderr classification is recorded;
- no race-free service claim is made;
- immutable Drive evidence and GitHub closure binding are completed before STOP.

A successful package-byte exchange does not itself authorize service start/restart or any further activation mutation.

## 23. Canonical UM classification

`EXACT_UL_PREDECESSOR / PRIVATE_STAGE_EXACT / RECONCILER_A73A82A7_0500 / AGENT_CANDIDATE_9DB768C1_0500 / MANIFEST_56A37904_0600 / SUDO_RS_0_2_13_EXACT_2EB5D31F / EXPLICIT_TARGET_ROOT_ARGV / NO_SHELL / NO_ENV_WRAPPER / NO_PRESERVED_ENV / OLD_AGENT_EXACT_4DBB114E / VENDOR_UNIT_EXACT_24F646DC / NO_ROOT_STAGING_RESIDUE / SERVICE_FAILED_FAILED_MAINPID0_AT_OBSERVATION / ENABLED_AND_LINGER_RACE_SURFACE / IMMEDIATE_RECHECK_REQUIRED / NO_SUDO_AUTH / NO_PRIVILEGED_EXECUTION / NO_PACKAGE_TRANSACTION / NO_RACE_FREE_CLAIM`

UM establishes a bounded transaction-readiness handoff shape. It does not establish that the future service state will remain unchanged and does not authorize the package mutation.

## 24. Explicit non-actions / STOP

C03e-UM performs no:

- Rust/source/Cargo mutation;
- private-stage mutation;
- sudo authentication;
- privileged reconciler execution;
- sudoers mutation;
- sudo alternatives mutation;
- NOPASSWD/stored-password configuration;
- direct root shell;
- root staging creation;
- installed Agent replacement;
- vendor-unit rewrite;
- systemd manager mutation;
- service start/stop/restart/try-restart/reload;
- enable/disable mutation;
- linger mutation;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation;
- repository main mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- history rewrite;
- destructive evidence cleanup.

STOP after evidence closure of this final transaction-readiness checkpoint.

The next safe boundary is a separately and explicitly authorized C03e-UN privileged current-Agent package reconciliation transaction. Such authorization must cover the root-owned Agent replacement and exact interactive sudo-rs invocation. It must still begin with the mandatory immediate read-only reproof; any drift causes STOP. It does not authorize service activation/restart or any unrelated privileged mutation.