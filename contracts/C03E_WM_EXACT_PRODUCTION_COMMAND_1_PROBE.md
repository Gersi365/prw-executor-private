# C03e-WM — Exact Production Command-1 Probe

Status: `EXACT_PRODUCTION_COMMAND_1_PROBE_PASS — GET_AGENT_STATUS_OK_READY_PROTOCOL_1_0 — POSTPROBE_HEALTH_STABLE — COMMAND_3_NOT_SENT`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`EXACT_PRODUCTION_COMMAND_1_PROBE`

This checkpoint records one separately authorized production-local read-only command-1 probe against the already running exact NEW Agent.

The probe is exactly the existing local `GetAgentStatus` command, code `1`.

WM authorizes and records exactly one command-1 request/response exchange. It does not authorize or send command-3.

## Authoritative predecessor

Evidence-closed C03e-WL / PR #728 is the exact predecessor:

- branch `phase-152-c03e-wl-post-start-production-health-probe-readiness`;
- exact head `1c191db562f86f88bd043e045178a3d04ba2f8e3`;
- exact tree `7da74a6315ca75f304fe820cb98cd59ffb2e0b53`;
- exact WK parent / merge base `947f34b06595883821f7f99ae977e9e141c8bc7a`;
- canonical WL evidence ID `10SOWUlSEXfqlj6gt3SlADm_aCGGiDVYM`.

WL explicitly stopped before any production command probe and required a separately gated exact probe with exact request shape, success semantics, and STOP boundary.

## Rebound command-1 protocol authority

All protocol files were read at exact WL head `1c191db562f86f88bd043e045178a3d04ba2f8e3`.

### Command namespace

`contracts/LOCAL_COMMAND_CONTRACT.md`

Blob:

`0f44793c4e57c519425dae4dbb2acb0d22b92057`

The locked command namespace defines:

- code `1` = `GetAgentStatus`;
- meaning = read the Agent local runtime status snapshot;
- request ID = correlation metadata only;
- command is read-only.

### Request payload

`contracts/LOCAL_COMMAND_REQUEST_CODEC_CONTRACT.md`

Blob:

`c31aa8149def7f1ca6db22a7d7d17d74dd49173d`

The exact command-1 payload is:

`00 01`

It is exactly two bytes, unsigned big-endian `u16`, with no request arguments.

### Local IPC frame

`contracts/LOCAL_IPC_CONTRACT.md`

Blob:

`5fd1f44355488b82bfb53c55319c60d31fe84cdf`

The exact frame is:

- fixed 24-byte header;
- magic `PRW\0`;
- protocol `1.0`;
- Request kind code `1`;
- flags `0`;
- reserved `0`;
- non-zero client-assigned `u64` request ID;
- payload length `2`;
- payload immediately follows.

`contracts/LOCAL_FRAME_CODEC_CONTRACT.md`

Blob:

`1ffe62f935308fdb493fa3ce0102d9f19c38c91a`

confirms all multi-byte fields are big-endian and the exact header offsets.

### Successful status response

`contracts/LOCAL_AGENT_STATUS_RESPONSE_CONTRACT.md`

Blob:

`769cc32c229a0edc8e43e4f59ae8fbb3d39a0e2d`

The exact successful payload for `Ready / protocol 1.0` is seven bytes:

`00 00 02 00 01 00 00`

where:

- response status `00 00` = `Ok`;
- runtime state `02` = `Ready`;
- protocol major `00 01` = `1`;
- protocol minor `00 00` = `0`.

`contracts/LOCAL_AGENT_STATUS_FRAME_CONTRACT.md`

Blob:

`8e674daf1eb97d2463ec85a54c2e5b73d7f6dbf3`

requires a correlated outer `Response` frame with the same request ID and payload length exactly `7`.

### Prior real-host precedent

`logs/audits/phase-109-real-host-activation/PRW-PHASE-109-A03-ENABLE-START-TRANSACTION.txt`

Blob:

`0a598741f0b0239837142c9bf3c8732adf227f02`

records the prior same-UID production-local readiness pattern:

- connect to `/run/user/1000/private-remote-workspace/agent.sock`;
- write one `GetAgentStatus` Request;
- require correlated Response;
- require `Ok`;
- require `Ready`;
- require protocol `1.0`.

WM reuses this existing bounded local IPC contract and does not widen the command surface.

## Exact WM request selection

Endpoint:

`/run/user/1000/private-remote-workspace/agent.sock`

Selected request ID:

- hex:
  `0x574d000000000001`
- decimal:
  `6290684254506450945`

The request ID is correlation metadata only.

Exact request wire bytes:

`505257000001000001000000574d000000000001000000020001`

Decoded:

- magic `50 52 57 00` = `PRW\0`;
- major `00 01` = 1;
- minor `00 00` = 0;
- kind `01` = Request;
- flags `00`;
- reserved `00 00`;
- request ID `57 4d 00 00 00 00 00 01`;
- payload length `00 00 00 02`;
- command payload `00 01` = `GetAgentStatus`.

Expected success payload:

`00000200010000`

Expected success semantics:

- outer kind Response `2`;
- exact same request ID;
- payload length exactly `7`;
- status Ok `0`;
- runtime state Ready `2`;
- protocol `1.0`.

Any mismatch was defined as probe failure.

## Pre-probe evidentiary typo and correction

An initial read-only pre-probe guard at:

`2026-09-19T16:54:28+02:00`

printed the correct:

- request ID hex `0x574d000000000001`;
- request wire hex;
- expected success payload hex;

but printed an incorrect decimal rendering of the request ID.

No production command request had been sent at that point.

The mismatch was detected before probe execution.

A corrected final immediate pre-probe guard was performed at:

`2026-09-19T16:54:58+02:00`

and recorded the correct decimal value:

`6290684254506450945`

The probe used the hex/integer value `0x574d000000000001`, not the incorrect discarded decimal text.

No request was sent under the incorrect decimal record.

## Corrected final immediate pre-probe drift guard

At:

`2026-09-19T16:54:58+02:00`

PowerCode freshly proved:

- service `active/running`;
- `MainPID=3033677`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- manager-visible `local_only`;
- exact Agent process count `1`;
- socket owner/group `gersi365:gersi365`;
- socket uid/gid `1000:1000`;
- socket mode `0600`;
- Unix listener count `1`;
- unexpected Agent TCP count `0`;
- unexpected Agent UDP count `0`;
- installed Agent exact NEW SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

Before the exact probe:

- command-1 had not yet been sent by WM;
- command-3 had not been sent.

## Exact production command-1 probe

Probe timestamp:

`2026-09-19T16:55:26+02:00`

The probe ran as the normal user, not root, using one ephemeral same-UID Python Unix-domain socket client.

It created no script file.

It connected only to:

`/run/user/1000/private-remote-workspace/agent.sock`

It sent exactly one request:

`505257000001000001000000574d000000000001000000020001`

It then read exactly one 24-byte response header and the declared response payload.

Client timeout was bounded to five seconds.

No retry loop was used.

## Exact response

Response header bytes:

`505257000001000002000000574d00000000000100000007`

Response payload bytes:

`00000200010000`

Complete response wire bytes:

`505257000001000002000000574d0000000000010000000700000200010000`

Decoded result:

- magic exact `PRW\0`;
- protocol `1.0`;
- kind `Response(2)`;
- flags `0`;
- reserved `0`;
- request ID exact `0x574d000000000001`;
- payload length `7`;
- response status `Ok(0)`;
- runtime state `Ready(2)`;
- status protocol `1.0`;
- request correlation exact.

Probe classification:

`wm_command_1_probe=success`

Probe process result:

`wm_probe_rc=0`

No Error frame was returned.

No alternate response was accepted.

## Independent post-probe health proof

At:

`2026-09-19T16:55:54+02:00`

a separate read-only health check, which sent no command request, proved:

- `ActiveState=active`;
- `SubState=running`;
- `MainPID=3033677`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- manager-visible `local_only`;
- exact Agent process count `1`;
- exact configure process count `0`;
- user-manager jobs `NONE`;
- socket still user-owned mode `0600`;
- Unix listener count `1`;
- Agent TCP count `0`;
- Agent UDP count `0`.

Exact identities remained:

- installed NEW Agent:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- vendor unit:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- fixed 20:
  `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- local-only 30:
  `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`.

Managed 40 remained absent.

Runtime credential file remained:

- owner/group `gersi365:gersi365`;
- mode `0400`;
- nlink `1`;
- inode `251`;
- bytes `138`.

Private credential bytes were not read.

Device-identity marker remained:

`b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d`

WM sent exactly one command-1 probe.

Command-3 remained unsent.

## Probe classification

`EXACT_PRODUCTION_COMMAND_1_PROBE_PASS / GET_AGENT_STATUS_CODE_1 / SAME_UID_UNIX_CLIENT / ONE_REQUEST_ONLY / REQUEST_ID_574D000000000001 / REQUEST_WIRE_EXACT / CORRELATED_RESPONSE / RESPONSE_KIND_2 / RESPONSE_STATUS_OK_0 / RUNTIME_STATE_READY_2 / PROTOCOL_1_0 / RESPONSE_PAYLOAD_EXACT_00000200010000 / PROBE_RC_ZERO / ACTIVE_RUNNING_PRESERVED / MAINPID_3033677_STABLE / NRESTARTS_ZERO / EXACT_NEW_AGENT_6A229C76 / UNIX_LISTENER_ONE / TCP_ZERO / UDP_ZERO / RUNTIME_CREDENTIAL_CUSTODY_INTACT / COMMAND_3_NOT_SENT / NO_RACE_FREE_CLAIM`

## Security and mutation ceiling

WM authorizes and records only the one completed read-only command-1 probe.

WM does not authorize or perform:

- command-3;
- desktop command-3 dispatch;
- a second command-1 probe;
- arbitrary command code;
- shell/PTY command;
- service mutation;
- daemon reload;
- execution-mode transition;
- configured-remote activation;
- private credential read;
- credential replacement;
- enrollment/revocation mutation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation.

## Explicit non-actions

WM performed no:

- command-3 request;
- desktop command-3 dispatch;
- command retry;
- second command-1 request;
- service start/stop/restart;
- daemon reload;
- reset-failed;
- sudo/root command;
- reconciler invocation;
- Agent replacement;
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

It must define exactly what `command-3` means in the current Phase 152 production activation sequence before any such request is sent.

It must rebind:

- the exact typed operation;
- exact request wire/request mechanism;
- authorization/policy expectations;
- exact success semantics;
- expected side effects, if any;
- post-request health proof;
- exact STOP boundary.

WM itself authorizes no command-3 request.

## STOP

`STOP_AFTER_EXACT_PRODUCTION_COMMAND_1_PROBE_AND_BEFORE_COMMAND_3`

`NO_RACE_FREE_CLAIM`
