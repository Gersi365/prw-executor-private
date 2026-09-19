# C03e-WH — Privileged Reconciliation Readiness Preflight

Status: `READINESS_PREFLIGHT_PASS — SERVICE_STOPPED — USER_ATTENDED_INTERACTIVE_TERMINAL_REQUIRED — NO_SUDO — NO_RECONCILER_EXECUTION`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`PRIVILEGED_RECONCILIATION_READINESS_PREFLIGHT`

This checkpoint is read-only with respect to production runtime. It re-proves the stopped/quiescent state closed by C03e-WG, exact stage/OLD Agent/reconciler/vendor-unit identities, package-parent custody, selected future sudo argv, and the exact invoking-user identity law compiled into the WD reconciler.

It performs no sudo authentication, no sudo command, no root process, no reconciler execution, no Agent replacement, no service start, and no production probes.

## Authoritative predecessor

Evidence-closed C03e-WG / PR #723 is the exact predecessor:

- branch `phase-152-c03e-wg-wa-agent-service-stop-transaction`;
- exact head `51f6fc096305bf62d38824e06e1f53bdb1024796`;
- exact tree `0ab2b6c31061ea11abd1609121ccb29e3aac7095`;
- exact WF parent / merge base `0c35b4ec31d9455caad57195e1d678fd16aecc93`;
- canonical WG evidence ID `1bVNoplnkwzJN8AbDNY-MskzeM5Cgi2YW`.

C03e-WB / PR #718 at exact head `8fdce3efb4bd6013a1710bc5514132afd5e0d3df` remains the ordered activation transaction authority.

C03e-WD exact reconciliation source authority remains:

- WD head `ff8cbd518d22c92930fda2cda0cb8788d8d61e4a`;
- reconciliation source blob `871d01eebd4694ec4aa0a6e3e5acfbf3dd4b20b2`;
- main blob `e973038d2281f9b51f3fbce9d8763158b5ffc6dc`.

## Fresh authority and duplicate guard

Immediately before WH materialization:

- WG / PR #723 remained draft/open/unmerged/mergeable;
- exact WG head remained `51f6fc096305bf62d38824e06e1f53bdb1024796`;
- no C03e-WH branch existed;
- no C03e-WH PR existed;
- no canonical C03e-WH Drive audit-title collision existed;
- production service remained stopped/quiescent.

## Fresh stopped-state proof

Host:

`PowerCode`

Primary WH live sampling:

`2026-09-19T15:33:59+02:00`

Follow-up:

`2026-09-19T15:34:25+02:00`

Final readiness guard:

`2026-09-19T15:34:51+02:00`

Pre-write guard:

`2026-09-19T15:35:14+02:00`

Fresh manager state remained:

- `LoadState=loaded`;
- `ActiveState=inactive`;
- `SubState=dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- manager-loaded drop-ins exactly fixed `20` plus managed `30`.

Fresh quiescence remained:

- exact `prw-agent` process count `0`;
- exact `prw-agent-configure` process count `0`;
- user-manager jobs `NONE`;
- Agent Unix listener count `0`;
- runtime `agent.sock` absent;
- Agent TCP activity count `0`;
- Agent UDP activity count `0`.

No automatic restart was observed during WH preflight.

## Invoking user identity

Fresh host identity:

- user `gersi365`;
- uid `1000`;
- gid `1000`;
- passwd entry resolves uid/gid `1000:1000`;
- login shell `/bin/bash`.

The selected stage owner is the same uid/gid.

## Exact stage custody

Stage:

`/home/gersi365/.prw-c03e-we-stage`

Path-chain inspection remained free of symlink components.

Stage directory:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0700`;
- nlink `2`;
- inode `6293644`;
- direct children `3`;
- regular files `3`;
- symlink children `0`;
- directory children `0`.

### Frozen deployment manifest

`C03E_WD_DEPLOYMENT_MANIFEST`

- owner/group `gersi365:gersi365`;
- mode `0600`;
- nlink `1`;
- bytes `597`;
- SHA-256 `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`.

Manifest readback remained:

- schema `c03e-wd-wa-agent-package-reconciliation-v1`;
- source head `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- source tree `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`;
- Cargo.lock SHA-256 `2258f178ab0076cc2899c50074503f7936491af566aa8ec2d35a2c247f4e5ac7`;
- canonical target `/tmp/prw-c03e-wc-canonical-target`;
- candidate bytes `11089904`;
- candidate SHA-256 `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- OLD Agent SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- vendor-unit SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- operation `reconcile-current-agent`.

### Candidate Agent

`candidate-prw-agent`

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0500`;
- nlink `1`;
- bytes `11089904`;
- SHA-256 `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

### Exact reconciler

`prw-agent-package-reconcile`

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0500`;
- nlink `1`;
- bytes `2898840`;
- SHA-256 `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`.

The reconciler binary was not invoked.

## Installed OLD Agent identity

`/usr/lib/private-remote-workspace/prw-agent`

Fresh readback:

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0755`;
- nlink `1`;
- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

This matches WD compiled `OLD_AGENT_SHA256` and `OLD_AGENT_BYTES`.

## Verify-only vendor unit

`/usr/lib/systemd/user/prw-agent.service`

Fresh readback:

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

This matches WD compiled `VENDOR_UNIT_SHA256`.

## Package-parent custody

The exact package parents required by WD were freshly re-proved:

`/usr/lib/private-remote-workspace`

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0755`;
- directory.

`/usr/lib/systemd/user`

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0755`;
- directory.

WD requires those fixed parents to be root-owned mode `0755`.

## Transaction sibling guard

Read-only scan of:

`/usr/lib/private-remote-workspace/.prw-agent.c03e-ug.*.candidate`

found:

`stale_transaction_sibling_count=0`

No previous transaction-owned candidate sibling was observed.

## Filesystem capacity observation

Read-only `df` at the target and stage paths reported the common filesystem with approximately:

- available blocks `20158956` KiB;
- usage `92%`.

The available capacity is materially larger than the exact `11089904`-byte candidate and therefore does not present an observed capacity blocker for the selected root-sibling staging step.

This is a capacity observation only, not a guarantee against future disk-state changes.

## Exact WD privileged law

The fixed-purpose reconciler exposes only:

`reconcile-current-agent <absolute-stage-directory>`

The exact future argv selected by WB remains:

1. `/usr/bin/sudo`;
2. `-u`;
3. `root`;
4. `--`;
5. `/home/gersi365/.prw-c03e-we-stage/prw-agent-package-reconcile`;
6. `reconcile-current-agent`;
7. `/home/gersi365/.prw-c03e-we-stage`.

WD source requires:

- real UID `0`;
- effective UID `0`;
- `SUDO_UID` present and a nonzero canonical decimal ID;
- `SUDO_GID` present and a nonzero canonical decimal ID;
- `SUDO_USER` present, nonempty, and not `root`;
- stage owner uid/gid to match the resolved invoking uid/gid;
- stage path to be an absolute normal path with no symlink traversal;
- stage mode `0700`;
- manifest exactness;
- candidate exact bytes/hash and safe custody;
- fixed package-parent custody;
- installed OLD Agent exact bytes/hash/custody;
- verify-only vendor-unit exact hash/custody.

The reconciler then stages the candidate to a root-owned sibling in the fixed Agent parent, revalidates exact old/new/unit identities, performs a same-directory `RENAME_EXCHANGE`, verifies new/rollback/unit state, and rolls back on bounded post-exchange failure when possible.

No race-free binding to the external service-stop preflight is claimed by the reconciler or WH.

## Sudo executable metadata

No sudo command was executed.

Fresh metadata-only readback:

`/usr/bin/sudo`

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `4755`;
- regular file;
- bytes `1090848`;
- SHA-256 `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`;
- installed package metadata `sudo 1.9.17p2-1ubuntu3`.

WH makes no claim about future authentication success or sudo policy beyond the existence/custody of this executable.

## Interactive-terminal constraint

The connector shell used for WH reported:

- `stdin_tty=no`;
- `stdout_tty=no`;
- `tty=not a tty`.

The current shell also contained no `SUDO_UID`, `SUDO_GID`, or `SUDO_USER` variables.

This is expected because no sudo transaction was entered.

The WB-selected future transaction is explicitly user-attended and same-terminal. Therefore:

`CONNECTOR_SHELL_NOT_AUTHORIZED_FOR_USER_ATTENDED_SUDO`

The future sudo command must be launched from a user-attended interactive terminal where the user can personally handle any authentication prompt. ChatGPT/automation must not request, transport, store, inject, or relay the user's password.

No `sudo -S`, askpass, NOPASSWD mutation, password piping, root shell, `su`, `pkexec`, or credential injection is selected.

## Readiness classification

`PRIVILEGED_RECONCILIATION_READINESS_PREFLIGHT_PASS / WG_STOPPED_STATE_REPROVED / INACTIVE_DEAD / MAINPID_ZERO / AGENT_PROCESS_ZERO / AGENT_UNIX_LISTENER_ZERO / USER_MANAGER_JOBS_NONE / NRESTARTS_ZERO / EXACT_WE_STAGE_REPROVED / EXACT_MANIFEST_REPROVED / EXACT_CANDIDATE_6A229C76 / EXACT_RECONCILER_F2792BB1 / INSTALLED_OLD_AGENT_9DB768C1 / VERIFY_ONLY_VENDOR_UNIT_24F646DC / PACKAGE_PARENTS_ROOT_0755 / NO_STALE_TRANSACTION_SIBLING / SUDO_EXECUTABLE_METADATA_REPROVED / INVOKING_USER_1000_1000_MATCHES_STAGE_CUSTODY / EXACT_SUDO_ARGV_REBOUND / WD_ROOT_AND_SUDO_IDENTITY_LAW_REBOUND / INTERACTIVE_USER_TERMINAL_REQUIRED / CONNECTOR_SHELL_NOT_TTY / READY_FOR_SEPARATELY_AUTHORIZED_USER_ATTENDED_PRIVILEGED_RECONCILIATION / NO_SUDO / NO_RECONCILER_EXECUTION / NO_AGENT_REPLACEMENT / NO_SERVICE_START / NO_PRODUCTION_PROBE / NO_RACE_FREE_CLAIM`

## Security and mutation ceiling

WH does not authorize or perform:

- sudo authentication;
- sudo command execution;
- root process execution;
- privileged reconciler execution;
- installed Agent replacement;
- service start/restart;
- daemon reload;
- managed configuration mutation;
- production probes;
- private credential read;
- credential/enrollment mutation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation.

## Explicit non-actions

WH performed no:

- sudo authentication;
- sudo invocation;
- root command;
- reconciler invocation;
- installed Agent replacement;
- vendor-unit replacement;
- service start/restart;
- daemon reload;
- production command request;
- desktop command-3 dispatch;
- execution-mode transition;
- configured-remote activation;
- private credential read;
- credential/enrollment mutation;
- network mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite.

## Immediate successor

The next checkpoint is separately gated and would be the actual user-attended privileged reconciliation transaction.

It must begin with another immediate stopped-state and exact-identity drift guard.

Only after that guard passes may the user, in an interactive local terminal, execute the exact WB-selected sudo argv.

That successor must not use this non-TTY connector shell for password handling.

After reconciliation returns, it must independently prove the installed Agent exact NEW identity before any service start is considered.

Service start remains a later, separately gated action.

## STOP

`STOP_AFTER_PRIVILEGED_RECONCILIATION_READINESS_PREFLIGHT_AND_BEFORE_SUDO_AUTHENTICATION_OR_RECONCILER_EXECUTION`

`NO_RACE_FREE_CLAIM`
