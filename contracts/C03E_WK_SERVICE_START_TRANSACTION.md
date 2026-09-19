# C03e-WK — Service-Start Transaction

Status: `SERVICE_START_TRANSACTION_COMPLETED — ACTIVE_RUNNING — EXACT_NEW_AGENT_RUNNING — POSTSTART_HEALTH_PROVEN — NO_PRODUCTION_PROBES`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`EXACT_SERVICE_START_TRANSACTION`

This checkpoint records one separately authorized user-manager service-start mutation followed by independent read-only post-start health proof.

It does not authorize or perform production command-1 or command-3 probes, execution-mode transition, configured-remote activation, credential mutation, network mutation, database/control-plane mutation, service restart, or daemon reload.

## Authoritative predecessor

Evidence-closed C03e-WJ / PR #726 is the exact predecessor:

- branch `phase-152-c03e-wj-service-start-readiness-preflight`;
- exact head `a8d2cadcd370170eb2dc40b6a48550d4251701e4`;
- exact tree `f122c1f03a765f8be4452aea32c3b66a7579ac18`;
- exact WI parent / merge base `5e6ce6f772ae68e6baa24e273ed6873fa02ee41b`;
- canonical WJ evidence ID `1uCdjxTd9y2X4_uGl9Ton_UUIqJxx614o`.

WJ authorized no start itself. It required a separately gated immediate drift guard, then exactly:

`systemctl --user start prw-agent.service`

followed by independent post-start proof and STOP before production command probes.

## Immediate pre-start drift guard

At:

`2026-09-19T16:29:18+02:00`

PowerCode freshly re-proved:

- WJ / PR #726 remained draft/open/unmerged/mergeable;
- exact WJ head remained `a8d2cadcd370170eb2dc40b6a48550d4251701e4`;
- canonical WJ evidence remained one-revision and exact;
- service `inactive/dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- Agent process count `0`;
- configure process count `0`;
- user-manager jobs `NONE`;
- Unix listener count `0`;
- runtime `agent.sock` absent;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- fixed `20` plus managed `30` loaded;
- managed `40` absent;
- installed Agent exact NEW SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- vendor unit exact SHA-256:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- fixed `20` exact SHA-256:
  `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- local-only `30` exact SHA-256:
  `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- stage manifest exact:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`;
- staged candidate exact:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- reconciler exact:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`;
- transaction sibling count `0`;
- runtime credential directory absent while stopped.

No drift blocker was observed.

## Exact service-start mutation

The only service-manager mutation executed in WK was:

`systemctl --user start prw-agent.service`

Invocation timestamp:

`2026-09-19T16:29:31+02:00`

The command returned:

`start_rc=0`

at:

`2026-09-19T16:29:33+02:00`

No `restart`, `try-restart`, `reload`, `daemon-reload`, `reset-failed`, stop, or alternate service-manager mutation was executed.

## Immediate independent post-start proof

Independent read-only proof began at:

`2026-09-19T16:29:53+02:00`

### User-manager state

Fresh service state:

- `LoadState=loaded`;
- `ActiveState=active`;
- `SubState=running`;
- `UnitFileState=enabled`;
- `NeedDaemonReload=no`;
- `MainPID=3033677`;
- `Result=success`;
- `NRestarts=0`;
- `ExecMainStartTimestamp=Sat 2026-09-19 16:29:31 CEST`;
- `ActiveEnterTimestamp=Sat 2026-09-19 16:29:33 CEST`;
- `ExecStart=/usr/lib/private-remote-workspace/prw-agent`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`.

Exact Agent process count:

`1`

Exact configure process count:

`0`

Process executable for the service MainPID:

`/usr/lib/private-remote-workspace/prw-agent`

User-manager jobs:

`NONE`

## Installed NEW Agent remains exact

Installed path:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh metadata:

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0755`;
- nlink `1`;
- inode `3017271`;
- exact bytes `11089904`;
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

The running process therefore resolves to the exact installed NEW Agent path, and the installed bytes remain the exact frozen NEW payload identity.

## Vendor unit and effective drop-ins remain exact

Vendor unit SHA-256 remained:

`24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`

Fixed `20` SHA-256 remained:

`42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`

Local-only `30` SHA-256 remained:

`00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`

Managed `40-configured-remote-inputs.conf` remained:

`ABSENT`

No execution-mode transition occurred.

## Runtime credential materialization

Without reading private key bytes, immediate post-start proof observed:

Runtime credentials directory:

`/run/user/1000/credentials/prw-agent.service`

- present;
- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0500`;
- type directory.

Runtime credential file:

`/run/user/1000/credentials/prw-agent.service/prw.device-identity.private-key.v1`

- present;
- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0400`;
- nlink `1`;
- inode `251`;
- regular file;
- bytes `138`.

Private credential bytes were not read.

## Agent local listener health

Immediate post-start Unix listener count:

`1`

Runtime socket path:

`PRESENT_SOCKET`

Observed listener:

`/proc/self/fd/5/agent.sock`

owned by:

- process `prw-agent`;
- PID `3033677`;
- fd `7`.

Unexpected Agent TCP activity count:

`0`

Unexpected Agent UDP activity count:

`0`

This is consistent with the selected `local_only` execution mode.

## Device-identity load marker

Recent service journal contained:

`prw-agent event=device_identity_loaded public_spki_sha256=b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d`

at:

`2026-09-19T16:29:33+02:00`

for PID:

`3033677`

No private credential bytes were read to establish this marker.

## Stage and transaction state after start

Exact private-stage hashes remained:

- manifest:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`;
- candidate:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- reconciler:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`.

Transaction sibling count remained:

`0`

## Sustained post-start guard

A second read-only guard after the immediate proof sampled:

`2026-09-19T16:30:29+02:00`

It retained:

- `ActiveState=active`;
- `SubState=running`;
- `MainPID=3033677`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `PRW_AGENT_EXECUTION_MODE=local_only`;
- exact Agent process count `1`;
- user-manager jobs `NONE`;
- Unix listener count `1`;
- Agent TCP count `0`;
- Agent UDP count `0`;
- installed Agent exact NEW SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- runtime credential file still present with mode `0400`, nlink `1`, bytes `138`.

No restart-loop signal was observed during the sampled interval.

This is an observed interval only and is not a race-free future guarantee.

## Start transaction classification

`EXACT_SERVICE_START_COMPLETED / START_RC_ZERO / ACTIVE_RUNNING / MAINPID_3033677 / EXACT_ONE_AGENT_PROCESS / RESULT_SUCCESS / NRESTARTS_ZERO / NEED_DAEMON_RELOAD_NO / INSTALLED_NEW_AGENT_6A229C76 / INSTALLED_NEW_AGENT_BYTES_11089904 / RUNNING_PROC_EXE_EXACT_INSTALLED_PATH / LOCAL_ONLY_PRESERVED / FIXED_20_EXACT / LOCAL_ONLY_30_EXACT / MANAGED_40_ABSENT / RUNTIME_CREDENTIAL_DIR_0500 / RUNTIME_CREDENTIAL_FILE_0400 / PRIVATE_CREDENTIAL_BYTES_NOT_READ / UNIX_LISTENER_ONE / AGENT_SOCKET_PRESENT / TCP_ZERO / UDP_ZERO / DEVICE_IDENTITY_MARKER_B417E6B0 / PRIVATE_STAGE_EXACT / NO_TRANSACTION_SIBLING / NO_RESTART_LOOP_OBSERVED / NO_PRODUCTION_PROBE / NO_RACE_FREE_CLAIM`

## Security and mutation ceiling

WK authorizes and records only the exact service-start transaction already completed.

WK does not authorize or perform:

- production command-1 probe;
- production command-3 probe;
- desktop command-3 dispatch;
- service restart;
- service stop;
- daemon reload;
- reset-failed;
- execution-mode transition;
- configured-remote activation;
- private credential read;
- credential replacement;
- enrollment/revocation mutation;
- provider/backend activation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation.

## Explicit non-actions

WK performed no:

- production command-1 request;
- production command-3 request;
- desktop command-3 dispatch;
- service restart;
- service stop after start;
- daemon reload;
- reset-failed;
- sudo/root command;
- reconciler invocation;
- installed Agent replacement;
- vendor-unit replacement;
- managed configuration write;
- private credential read;
- execution-mode transition;
- configured-remote activation;
- credential/enrollment mutation;
- network mutation;
- database/control-plane mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite.

## Immediate successor

The next checkpoint is separately gated and must be post-start production-health/probe readiness before any production command probe.

It may freshly re-prove:

- exact WK Git/evidence authority;
- service remains `active/running`;
- MainPID remains stable/nonzero;
- exactly one Agent process;
- `Result=success`;
- `NRestarts=0`;
- installed Agent exact NEW identity;
- fixed `20`, local-only `30`, managed `40` absent;
- runtime credential private custody;
- Unix listener healthy;
- no unexpected Agent TCP/UDP activity;
- device-identity marker continuity.

It must STOP before sending production command-1 or command-3 requests unless a later checkpoint explicitly authorizes those probes.

## STOP

`STOP_AFTER_EXACT_SERVICE_START_AND_POSTSTART_HEALTH_PROOF_AND_BEFORE_PRODUCTION_COMMAND_PROBES`

`NO_RACE_FREE_CLAIM`
