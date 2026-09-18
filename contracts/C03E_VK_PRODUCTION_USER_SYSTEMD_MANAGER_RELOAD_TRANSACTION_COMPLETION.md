# C03e-VK Production User-Systemd Manager Reload Transaction Completion

Status: `PRODUCTION_USER_SYSTEMD_MANAGER_RELOAD_COMPLETED — MANAGED_20_AND_30_LOADED — LOCAL_ONLY_MODE_MANAGER_VISIBLE — NEED_DAEMON_RELOAD_NO — EXTERNAL_CONFIGURATION_CLEAR — VALIDATED — EVIDENCE_PENDING — AGENT_NOT_ACTIVATED`

Boundary: `PRW_PRODUCTION_USER_SYSTEMD_MANAGER_RELOAD_TRANSACTION`

Authoritative predecessor: evidence-closed C03e-VJ / PR #700 at exact head `93ee9adfb24fb937b99aee22799b8ea3d0c67cab`.

## Authorized scope

The separately authorized production mutation was limited to one user-systemd manager reload after the C03e-VJ managed `local_only` filesystem state had been evidence-closed and freshly preflighted.

The authorization did not include:

- Agent start, restart or activation;
- `prw-agent-configure reconfigure-active`;
- any managed file write or remote-input creation;
- credential read, decrypt, replace or enrollment mutation;
- networking, DNS, forwarding or relay mutation;
- merge, close, ready-for-review transition or branch deletion.

## Immediate pre-reload proof

Immediately before the reload at `2026-09-18T18:28:04+02:00`, PowerCode was re-proved:

- canonical binary `/usr/lib/private-remote-workspace/prw-agent-configure`: regular root-owned `0755`, size `104941656`, SHA-256 `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- fixed `20-device-identity-credential.conf`: regular user-owned `0600`, size `170`, SHA-256 `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- managed `30-agent-execution-mode.conf`: regular user-owned `0600`, size `58`, SHA-256 `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- exact canonical `local_only` bytes verification: PASS;
- managed `40-configured-remote-inputs.conf`: ABSENT;
- writer temporary residue count: `0`;
- user service `LoadState=loaded`, `ActiveState=failed`, `SubState=failed`, `MainPID=0`, `Result=exit-code`, `NRestarts=6`;
- manager-visible `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `NeedDaemonReload=yes`;
- configure executable process count `0`;
- Agent executable process count `0`.

A separately authorized read-only preflight immediately before this transaction had also re-proved 9 effective user-unit paths, zero external configuration conflicts, process quiescence, exact managed bytes/custody, and a successful read-only `systemd-analyze --user verify` of `prw-agent.service`.

## Production daemon-reload transaction

The exact manager mutation was:

`systemctl --user daemon-reload`

Transaction timing:

- begin `2026-09-18T18:28:22+02:00`;
- return code `0`;
- end `2026-09-18T18:28:24+02:00`.

No start, restart, stop, enable/disable, `reconfigure-active`, managed-file write, credential mutation, enrollment mutation or networking mutation was invoked as part of the transaction.

## Immediate post-reload proof

Fresh read-only verification at `2026-09-18T18:28:47+02:00` proved:

- fixed `20` remained regular user-owned `0600`, size `170`, exact SHA-256 unchanged;
- managed `30` remained regular user-owned `0600`, size `58`, exact SHA-256 unchanged;
- exact canonical `local_only` bytes verification: PASS;
- managed `40`: ABSENT;
- writer temporary residue count `0`;
- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- manager-visible `DropInPaths=` became exactly the fixed `20` and managed `30` paths;
- `UnitFileState=enabled`;
- `NeedDaemonReload=no`;
- `MainPID=0`;
- `Result=exit-code`;
- `NRestarts=6`;
- manager-visible service environment contains `PRW_AGENT_EXECUTION_MODE=local_only`;
- configure executable process count `0`;
- Agent executable process count `0`;
- canonical binary SHA-256 remained `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`.

The reload therefore caused the user manager to consume the already-written `20`/`30` configuration while preserving the intentionally inactive service state.

## Post-reload external and generator guard

A subsequent read-only guard at `2026-09-18T18:29:18+02:00` proved:

- effective user-unit path count `9`;
- writer-style external scanned fragment count `2`;
- external conflict count `0`;
- external configuration preflight PASS;
- manager-global `PRW_*` environment count `0`;
- user-manager queued job count `0`.

This guard was performed after `daemon-reload` because systemd generators may run during manager reload.

## Fresh evidence snapshot

A later read-only closure snapshot beginning at `2026-09-18T18:31:19+02:00` re-proved durable post-reload state:

- canonical binary remained regular root-owned `0755`, size `104941656`, exact SHA-256 unchanged;
- fixed `20` remained regular user-owned `0600`, size `170`, exact SHA-256 unchanged;
- managed `30` remained regular user-owned `0600`, size `58`, exact SHA-256 unchanged;
- exact canonical `local_only` bytes verification PASS;
- managed `40` remained ABSENT;
- no writer temporary file was present;
- user service remained `loaded`, `failed/failed`, `MainPID=0`, `Result=exit-code`, `NRestarts=6`;
- manager-visible `DropInPaths=` remained exactly fixed `20` plus managed `30`;
- `NeedDaemonReload=no`;
- manager-visible service environment remained `PRW_AGENT_EXECUTION_MODE=local_only`;
- manager-global `PRW_*` environment remained empty;
- user-manager queued jobs remained absent;
- configure executable process count `0`;
- Agent executable process count `0`;
- effective user-unit path list remained the same 9 paths;
- the two writer-relevant external fragments remained regular files and contained no conflicting selected PRW environment directives or continuation lines.

## Runtime boundary proof

No Agent process was started by the reload transaction. The service remained failed/inactive with `MainPID=0` and unchanged `NRestarts=6` before and after reload.

The manager has now consumed the canonical local-only mode, but first Agent activation has not occurred.

No configured-remote input drop-in exists. No credential, enrollment or networking mutation occurred.

## Git evidence scope

C03e-VK is documentation/evidence only. It introduces no Rust source, workflow, packaging implementation, Agent unit, credential, enrollment or networking source mutation.

The evidence branch is stacked directly on the exact C03e-VJ head.

## Hard boundary

C03e-VK closes only the production user-systemd manager reload transaction.

It does not authorize or perform:

- Agent start, restart or first activation;
- `prw-agent-configure reconfigure-active`;
- managed configured-remote input creation;
- credential mutation;
- enrollment, revocation, networking, DNS, forwarding or relay mutation;
- merge, close, ready-for-review transition or branch deletion.

## STOP

`STOP_BEFORE_FIRST_AGENT_ACTIVATION`

Next safe boundary: a separately authorized fresh read-only preflight for first Agent activation. It must re-prove the manager-loaded canonical local-only configuration, fixed credential-drop-in custody, managed `40` absence, process quiescence, service state and absence of unexpected external configuration, and must stop before any Agent start/restart or `reconfigure-active` unless separately authorized.

`NO_RACE_FREE_CLAIM`
