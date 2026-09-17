# C03e-UN — Privileged package reconciliation transaction

## 1. Status and boundary

`TRANSACTION — IMMEDIATE REPROOF PASS — BLOCKED ON INTERACTIVE AUTHENTICATION CHANNEL — NO PACKAGE MUTATION`

Boundary:

`PRIVILEGED_CURRENT_AGENT_PACKAGE_RECONCILIATION_TRANSACTION`

C03e-UN records the explicitly authorized attempt to execute the exact C03e-UM frozen target-root sudo-rs argv. The mandatory immediate read-only reproof passed. The exact sudo process then reached an interactive authentication boundary and remained blocked without producing transaction output. Because this execution surface provides no secure credential-entry channel, the blocked process was terminated. Independent post-block readback proves that no Agent replacement, root staging residue, vendor-unit mutation, or service mutation occurred.

UN does not select or authorize any alternate privilege mechanism, stored password, stdin password, askpass, NOPASSWD rule, sudoers change, direct root shell, `su`, `pkexec`, setuid PRW helper, or environment-injected caller identity.

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UM / PR #677.

Exact predecessor identity:

- branch `phase-152-c03e-um-final-transaction-readiness`;
- head `f302935896044b6d151b9d1be73408a0428f1a60`;
- tree `0e1fd168f64c0f8d4db3596359d3362fd26a94f4`;
- status `FINAL READINESS — VALIDATED — EVIDENCE_RECORDED — CLOSED — PRIVILEGED TRANSACTION NOT AUTHORIZED`;
- retained open, draft and unmerged.

UM froze one exact future target-root sudo-rs argv and required a complete immediate reproof before execution.

## 3. Authorization scope

User authorization for UN permitted only:

1. the mandatory immediate read-only reproof;
2. execution of the exact frozen sudo-rs argv if and only if every guard passed;
3. the bounded root-owned current-Agent package replacement performed by the fixed-purpose reconciler;
4. independent post-transaction readback and evidence closure.

It did not authorize service start/stop/restart/reload, enablement/linger mutation, vendor-unit rewrite, sudoers/alternatives mutation, identity/20/30/40 mutation, arbitrary root shell, alternate privilege path, merge, ready conversion, PR close, or `main` mutation.

## 4. Exact frozen argv attempted

The attempted argv was exactly:

1. `/usr/bin/sudo`
2. `-u`
3. `root`
4. `--`
5. `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile`
6. `reconcile-current-agent`
7. `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

Display-only rendering:

`/usr/bin/sudo -u root -- /home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile reconcile-current-agent /home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

No sudo flag, command argument, executable path, stage path, environment assignment, preserved-environment option, cwd override, pipe, redirection, command substitution or alternate executable was added.

The launcher replaced its shell process with `/usr/bin/sudo` using `exec`, so the privileged handoff process itself had the frozen sudo argv.

## 5. Mandatory immediate reproof

Immediately before the sudo attempt, a read-only fail-closed reproof returned:

`IMMEDIATE_REPROOF=PASS`

It proved all of the following.

### Operator

- real UID/GID `1000:1000`.

### Stage

Stage path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

- UID/GID `1000:1000`;
- mode `0700`;
- directory and non-symlink;
- exact child set:
  - `C03E_UF_DEPLOYMENT_MANIFEST`;
  - `candidate-prw-agent`;
  - `prw-agent-package-reconcile`.

Staged reconciler:

- UID/GID `1000:1000`;
- mode `0500`;
- bytes `2898840`;
- SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`.

Staged Agent candidate:

- UID/GID `1000:1000`;
- mode `0500`;
- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

Staged manifest:

- UID/GID `1000:1000`;
- mode `0600`;
- bytes `602`;
- SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`.

The complete private stage parent chain remained non-symlink.

### Active sudo-rs

- `/usr/bin/sudo` resolved to `/usr/lib/cargo/bin/sudo`;
- reported version `sudo-rs 0.2.13-0ubuntu1.2`;
- resolved binary root:root mode `4755`;
- bytes `1090848`;
- SHA-256 `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`.

Root account still resolved UID/GID `0:0`.

### Fixed package state

Installed Agent remained:

- path `/usr/lib/private-remote-workspace/prw-agent`;
- root:root;
- mode `0755`;
- bytes `2865776`;
- SHA-256 `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`.

Vendor unit remained:

- path `/usr/lib/systemd/user/prw-agent.service`;
- root:root;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

Root reconciliation residue count was zero.

### Immediate service state

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `Result=exit-code`;
- exact `prw-agent` process count for UID 1000 was zero.

Therefore the selected immediate pre-execution boundary passed.

`NO_RACE_FREE_CLAIM` remains mandatory because service enablement plus linger is still an external activation surface.

## 6. Authentication boundary result

After the PASS reproof, the exact frozen sudo argv was started.

Observed execution state:

- process remained running;
- captured stdout: none;
- captured stderr: none;
- remote process state reported `Blocked: true`;
- no reconciler success or bounded reconciliation-error output was observed.

This is classified as:

`BLOCKED_ON_INTERACTIVE_AUTHENTICATION_CHANNEL`

The execution environment available to this checkpoint does not expose a secure credential-entry channel suitable for transmitting a sudo password. No password was requested in chat, stored, inferred, injected, or transmitted through an alternate channel.

The blocked sudo process was explicitly terminated.

UN does not claim that sudo authentication succeeded, that target-root execution began, or that the reconciler entered the production transaction.

## 7. Post-block non-mutation proof

Immediately after terminating the blocked sudo process, independent readback proved the following.

### Installed Agent unchanged

`/usr/lib/private-remote-workspace/prw-agent`

- root:root;
- mode `0755`;
- bytes `2865776`;
- SHA-256 `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`;
- exact old-Agent identity retained.

No selected new Agent bytes were installed.

### Vendor unit unchanged

`/usr/lib/systemd/user/prw-agent.service`

- root:root;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

### Sudo binary unchanged

`/usr/lib/cargo/bin/sudo`

- root:root;
- mode `4755`;
- bytes `1090848`;
- SHA-256 `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`.

### Stage unchanged

Stage remained `1000:1000`, mode `0700`, with the exact three selected children.

Staged reconciler remained:

- `1000:1000`;
- mode `0500`;
- bytes `2898840`;
- SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`.

Staged Agent remained:

- `1000:1000`;
- mode `0500`;
- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

Staged manifest remained:

- `1000:1000`;
- mode `0600`;
- bytes `602`;
- SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`.

### Root staging residue absent

Post-block root reconciliation residue count:

`0`

### Service state unchanged

Post-block readback:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- exact vendor `FragmentPath`;
- empty `DropInPaths`;
- `UnitFileState=enabled`;
- `Result=exit-code`;
- exact `prw-agent` process count for UID 1000: `0`.

No systemd manager mutation occurred.

## 8. Canonical transaction result

`IMMEDIATE_REPROOF_PASS / EXACT_FROZEN_SUDO_ARGV_STARTED / INTERACTIVE_AUTHENTICATION_CHANNEL_UNAVAILABLE / SUDO_PROCESS_TERMINATED / AUTHENTICATION_SUCCESS_NOT_CLAIMED / RECONCILER_PRODUCTION_EXECUTION_NOT_CLAIMED / OLD_AGENT_RETAINED_4DBB114E / VENDOR_UNIT_UNCHANGED_24F646DC / PRIVATE_STAGE_UNCHANGED / ROOT_RESIDUE_ZERO / SERVICE_FAILED_FAILED_MAINPID0 / NO_PACKAGE_MUTATION / NO_SERVICE_MUTATION / NO_RACE_FREE_CLAIM`

C03e-UN is not a deployment-success checkpoint. It records a clean block before any provable privileged package transaction.

## 9. Prohibited fallback paths

No fallback was attempted or selected using:

- `sudo -S`;
- `sudo -A`;
- `sudo -n` as a substitute transaction path;
- environment-injected `SUDO_*` identity;
- stdin password transfer;
- password stored in a file or environment variable;
- NOPASSWD rule;
- sudoers mutation;
- sudo alternatives mutation;
- direct root login or root shell;
- `su`;
- `pkexec`;
- setuid PRW helper;
- alternate executable or stage path;
- service stop/restart as an implicit workaround.

Any future privilege-channel design requires a separately authorized selection checkpoint.

## 10. Explicit non-actions / STOP

C03e-UN performed no successful sudo authentication and no provable target-root reconciler execution.

It performed no:

- installed Agent replacement;
- vendor-unit rewrite;
- root reconciliation staging creation retained after the attempt;
- systemd manager mutation;
- service start/stop/restart/reload;
- enablement mutation;
- linger mutation;
- sudoers or alternatives mutation;
- password storage or transmission;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation;
- repository `main` mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- history rewrite;
- destructive evidence cleanup.

STOP at this authentication boundary.

## 11. Next safe boundary

The next safe checkpoint is a separately authorized selection/recovery checkpoint for a secure interactive privilege-authentication channel that preserves the exact C03e-UM transaction semantics without storing or exposing credentials.

That successor must not weaken the fixed argv, SUDO_UID/GID/USER custody model, package hashes, stage law, service non-running guard, or no-race-free limitation.

It must not silently introduce stdin passwords, askpass scripts, NOPASSWD, sudoers mutation, direct root shells, alternate setuid helpers, or credential persistence.
