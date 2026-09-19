# C03e-WL — Post-Start Production-Health / Probe Readiness

Status: `POST_START_PRODUCTION_HEALTH_PROBE_READINESS_PASS — ACTIVE_RUNNING_STABLE — EXACT_NEW_AGENT_REPROVED — NO_PRODUCTION_PROBES`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`POST_START_PRODUCTION_HEALTH_PROBE_READINESS`

This checkpoint is read-only with respect to production command traffic.

It freshly re-proves the evidence-closed WK authority, sustained active/running service health, stable MainPID, exact NEW Agent identity, effective local-only configuration, runtime credential custody, local Unix listener health, zero unexpected Agent TCP/UDP activity, and device-identity marker continuity.

It does not send production command-1 or command-3 requests.

## Authoritative predecessor

Evidence-closed C03e-WK / PR #727 is the exact predecessor:

- branch `phase-152-c03e-wk-service-start-transaction`;
- exact head `947f34b06595883821f7f99ae977e9e141c8bc7a`;
- exact tree `27fedf4cca46321c43a38ef8c78f2ba331605606`;
- exact WJ parent / merge base `a8d2cadcd370170eb2dc40b6a48550d4251701e4`;
- canonical WK evidence ID `1giU7SWd-S35iqx1-lz-61cb6VpmWt0dc`.

WK closed the exact service-start transaction and explicitly stopped before production command probes.

## Fresh authority and duplicate guard

Immediately before WL materialization:

- WK / PR #727 remained draft/open/unmerged/mergeable;
- WK exact head remained `947f34b06595883821f7f99ae977e9e141c8bc7a`;
- canonical WK evidence remained exact and one-revision;
- no C03e-WL branch existed;
- no C03e-WL PR existed;
- no canonical C03e-WL audit-title collision existed.

## Primary live readiness sampling

Host:

`PowerCode`

Timestamp:

`2026-09-19T16:41:41+02:00`

## Stable service health

Fresh manager state:

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

Fresh process state:

- exact Agent process count `1`;
- exact configure process count `0`;
- MainPID executable `/usr/lib/private-remote-workspace/prw-agent`;
- user-manager jobs `NONE`.

This retains the same MainPID observed throughout WK closure.

## Exact installed NEW Agent

Installed path:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh SHA-256:

`6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`

This remains the exact frozen NEW payload identity established by WC/WD/WE/WI and run by WK.

## Effective configuration remains exact

Vendor unit SHA-256:

`24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`

Fixed `20-device-identity-credential.conf` SHA-256:

`42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`

Local-only managed `30-agent-execution-mode.conf` SHA-256:

`00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`

Managed `40-configured-remote-inputs.conf` remained:

`ABSENT`

No execution-mode transition or configured-remote activation occurred.

## Runtime credential custody

Without reading private key bytes, live metadata proved:

Runtime credentials directory:

`/run/user/1000/credentials/prw-agent.service`

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0500`;
- directory.

Runtime credential:

`/run/user/1000/credentials/prw-agent.service/prw.device-identity.private-key.v1`

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0400`;
- nlink `1`;
- inode `251`;
- regular file;
- bytes `138`.

Private credential bytes were not read.

## Local listener and network posture

Agent Unix listener count:

`1`

Runtime socket:

`PRESENT_SOCKET`

Unexpected Agent TCP activity count:

`0`

Unexpected Agent UDP activity count:

`0`

This is consistent with the selected `local_only` execution mode.

## Device-identity continuity

The live journal retained the startup marker:

`prw-agent event=device_identity_loaded public_spki_sha256=b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d`

Timestamp:

`2026-09-19T16:29:33+02:00`

PID:

`3033677`

This exactly matches the established public SPKI marker.

No private credential bytes were read.

## Production-probe boundary

WL did not send:

- production command-1;
- production command-3;
- desktop command-3 dispatch.

Observed assertions:

`production_command_1_not_sent=yes`

`production_command_3_not_sent=yes`

No claim is made about command-path behavior beyond readiness.

## Readiness classification

`POST_START_PRODUCTION_HEALTH_PROBE_READINESS_PASS / WK_AUTHORITY_REPROVED / ACTIVE_RUNNING / MAINPID_3033677_STABLE / EXACT_ONE_AGENT_PROCESS / RESULT_SUCCESS / NRESTARTS_ZERO / NEED_DAEMON_RELOAD_NO / INSTALLED_NEW_AGENT_6A229C76 / LOCAL_ONLY_PRESERVED / FIXED_20_EXACT / LOCAL_ONLY_30_EXACT / MANAGED_40_ABSENT / RUNTIME_CREDENTIAL_DIR_0500 / RUNTIME_CREDENTIAL_FILE_0400 / PRIVATE_CREDENTIAL_BYTES_NOT_READ / UNIX_LISTENER_ONE / AGENT_SOCKET_PRESENT / TCP_ZERO / UDP_ZERO / DEVICE_IDENTITY_MARKER_B417E6B0 / READY_FOR_SEPARATELY_AUTHORIZED_PRODUCTION_COMMAND_PROBE / NO_PRODUCTION_PROBE / NO_RACE_FREE_CLAIM`

## Security and mutation ceiling

WL does not authorize or perform:

- production command-1 probe;
- production command-3 probe;
- desktop command-3 dispatch;
- service start/stop/restart;
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

WL performed no:

- production command request;
- desktop command dispatch;
- service mutation;
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

The next checkpoint is separately gated.

It may authorize a narrowly scoped production command probe only after another immediate drift guard.

The next gate must specify exactly which production command is authorized, its exact request shape, expected success semantics, and exact STOP boundary.

No command-1 or command-3 probe is authorized by WL itself.

## STOP

`STOP_AFTER_POST_START_PRODUCTION_HEALTH_PROBE_READINESS_AND_BEFORE_PRODUCTION_COMMAND_PROBE`

`NO_RACE_FREE_CLAIM`
