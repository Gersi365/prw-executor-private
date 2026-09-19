# C03e-WN — Command-3 Authority / Semantics Rebinding Preflight

Status: `COMMAND3_AUTHORITY_SEMANTICS_REBOUND — SERVER_SIDE_AGENTSTATUS_PATH_PROVEN — DEDICATED_ONE_SHOT_PROBE_NOT_YET_MATERIALIZED — NO_COMMAND3_REQUEST`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`COMMAND3_AUTHORITY_SEMANTICS_REBINDING_PREFLIGHT`

This checkpoint is read-only with respect to production runtime.

It rebinds exactly what "command-3" means in the selected Phase-152 WA production activation sequence, proves the installed NEW Agent contains the already-selected server-side AgentStatus-only path, and determines whether a production command-3 probe is executable under the existing transaction law.

WN sends no command-3 request.

## Exact predecessor

Evidence-closed C03e-WM / PR #729 is the exact predecessor:

- branch:
  `phase-152-c03e-wm-exact-production-command-1-probe`
- exact head:
  `a9c6598e19f5d1d68493a40d2944d321b9994a9b`
- exact tree:
  `a7543b0ea4f4abd1e6f301184b2aab65e98b4511`
- canonical WM evidence ID:
  `1xFj7nTz8vhICCyvi5TWcde8pdnsvB0Hr`

WM proved one exact legacy command-1 GetAgentStatus request and explicitly stopped before command-3.

## Terminology correction: outer command 3 vs inner PRWC operation

"Command-3" in this activation sequence means:

- outer local Agent management command code:
  `3`

It does NOT mean PRWC operation code 3.

The selected inner canonical PRWC operation is:

`BridgeCommand::AgentStatus`

whose stable PRWC operation code is:

`1`

PRWC operation code `3` is `FileStat` and is not selected or authorized by this checkpoint.

This distinction is security-significant and must remain explicit in any future probe.

## Selected transaction authority

Exact WB contract:

`contracts/C03E_WB_WA_AGENT_PRODUCTION_ACTIVATION_TRANSACTION_SELECTION_STAGING.md`

Blob:

`b6173440ab65ba84ab14fdbbea582e013c76b4c9`

WB Gate 10 selected one dedicated production command-3 AgentStatus probe and explicitly rejected:

- ad-hoc hand-crafted command-3 bytes;
- raw shell socket writes;
- desktop command-3 activation by implication;
- widening desktop management authority merely to run one deployment probe.

WB requires a separately gated source checkpoint to select and materialize one bounded same-UID one-shot AgentStatus probe before any production command-3 request.

Required future probe composition from WB:

- trusted LocalIpcContract endpoint derivation and same-UID socket custody checks;
- `BridgeCommand::AgentStatus`;
- canonical PRWC encoding;
- Agent-owned command-3 local framing;
- existing LocalIpcFrame writer/reader;
- existing terminal-response validation;
- exact request-ID correlation;
- terminal status `Ok`;
- management success body tag `1`;
- existing five-byte Agent status snapshot codec.

## Command-3 framing authority

Exact source:

`crates/prw-agent/src/local_commands/management_request.rs`

Blob:

`a29f0a0d25769e413ebc934239f5ccfa6f5f0c15`

Locked local command-3 constants:

- command code = `3`;
- fixed prefix length = `6`;
- canonical PRWC embedded maximum = `65,536` bytes;
- local management payload = two-byte command code + four-byte big-endian embedded length + exact PRWC bytes.

The Agent-owned builder is:

`build_local_management_request_frame(request_id, bridge_payload)`

It builds a current-version local IPC `Request` and preserves the supplied non-zero request ID.

The decoder requires:

- outer local kind `Request`;
- command code exactly `3`;
- non-zero embedded PRWC length;
- embedded length <= 65,536;
- declared embedded length equal to exact remaining payload bytes.

## Canonical inner AgentStatus operation

Exact source:

`crates/prw-remote-bridge/src/lib.rs`

Blob:

`ad6833cc4e71a372810b260f157126a3df6645e5`

Canonical PRWC constants:

- magic `PRWC`;
- protocol `1.0`;
- fixed PRWC header length `12` bytes;
- `BridgeCommand::AgentStatus.operation_code() = 1`;
- `BridgeCommand::AgentStatus.required_capability() = AgentStatusRead`;
- AgentStatus has no operation-specific request body.

Exact canonical AgentStatus PRWC request bytes:

`505257430001000000010000`

Decoded:

- magic `50 52 57 43` = `PRWC`;
- protocol major `00 01`;
- protocol minor `00 00`;
- operation `00 01` = AgentStatus;
- reserved `00 00`;
- no trailing operation body.

## Exact local command-3 payload

Embedding the exact 12-byte AgentStatus PRWC payload inside the Agent-owned command-3 envelope produces:

`00030000000c505257430001000000010000`

Length:

`18` bytes

Decoded:

- outer local management code:
  `00 03`;
- embedded PRWC length:
  `00 00 00 0c`;
- exact canonical PRWC AgentStatus bytes:
  `505257430001000000010000`.

A future request must place these 18 payload bytes inside the existing 24-byte PRW local IPC Request header.

Therefore a complete request is 42 bytes once a non-zero request ID is selected by the dedicated probe.

WN does not select or send the future transaction request ID.

## Authorization and policy authority

Exact VT contract:

`contracts/C03E_VT_COMMAND3_MANAGEMENT_BOUNDARY_TRANSACTION_SOURCE_MATERIALIZATION.md`

Blob:

`bf31873a70655187d786c88da8cceeebd24f84ed`

VT selected the first live management slice as:

`BridgeCommand::AgentStatus` only.

Exact VU contract:

`contracts/C03E_VU_AGENT_STATUS_ONLY_MANAGEMENT_RUNTIME_OWNERSHIP_SOURCE_MATERIALIZATION.md`

Blob:

`68d038d02ff4dab8fd08f359129339e53aadc755`

VU selected a narrow runtime ownership path with:

- `AgentStatusRead = Allow`;
- `PrivateDnsConfigRead = Deny`;
- `TerminalOpen = Deny`;
- `TerminalExec = Deny`;
- `FilesRead = Deny`;
- `FilesWrite = Deny`;
- `ForwardingCreate = Deny`.

The caller cannot supply or widen this management policy.

Exact runtime source:

`crates/prw-agent/src/local_commands/management_agent_status_runtime.rs`

Blob:

`037ebc94547d4c3f3b7fc3bde88befedc75c75a5`

It constructs the fixed policy internally, admits only through the authenticated same-UID Linux connection, and returns `UnsupportedCommand` defensively if a policy-admitted command is not AgentStatus.

Current represented non-AgentStatus management operations require denied capabilities and therefore cannot acquire an AgentStatus-only success result.

## Same-UID authentication boundary

Command-3 admission is bound to the existing:

`AuthenticatedLocalLinuxConnection`

The admission token copies peer PID/UID/GID only from kernel-authenticated local connection state.

Request bytes cannot supply or override peer identity.

The selected production probe must therefore run as the same normal UID and connect through the trusted local Unix socket boundary.

No sudo/root client is selected.

## Server-side production runtime wiring

Exact WA / PR #717 source authority:

- WA head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- WA tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`;
- `linux_bootstrap.rs` blob:
  `aa827c05b518f3cf4c7571127391b132b1ccedf0`.

WA changed local-only `linux_bootstrap::run()` to call exactly once:

`run_signal_aware_linux_production_runtime_from_env_with_agent_status_management(...)`

The signal-aware runtime exact blob is:

`f1350a7bfa01931cbc42a7c049139b057b31a645`

and selects:

`AgentStatusOnlyManagement`

which constructs the scheduler context through:

`new_with_agent_status_management(...)`

Exact narrow server sources at the WM head additionally include:

- `management_agent_status_boundary.rs`
  `9d070b5fed2c814121a85d3341bc5ea775678bb2`;
- `linux_authenticated_session/management_agent_status.rs`
  `c61bfc1530ae79a0ada26a7e136d4bc8ca1d99a8`;
- `linux_session_worker/management_agent_status.rs`
  `f5ff8c2752d996466f59d08d4e914edfac0e3811`.

The worker preserves finite request budget, read deadline, deferred write deadline and permit RAII.

No filesystem authority, provider lifecycle, terminal backend, forwarding backend or transfer-provider authority is supplied to the AgentStatus-only path.

## Installed NEW Agent provenance

Exact WC contract:

`contracts/C03E_WC_WA_AGENT_CANDIDATE_PROVENANCE_STAGING.md`

Blob:

`11c35fdf189b6decb8e408c59efa8ea69bc879ee`

WC froze the exact WA-derived candidate as:

- source head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- source tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`;
- bytes:
  `11089904`;
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

The currently installed production Agent independently hashes to this exact candidate identity.

Therefore the running NEW Agent is the WA-derived binary containing the selected server-side command-3 AgentStatus-only path.

This source/binary provenance statement does not itself prove a live command-3 exchange.

## Success response semantics

Exact response source:

`crates/prw-agent/src/local_commands/management_response.rs`

Blob:

`80c05fa23f9ecf51bf6ed9d17bc3e24a126da997`

For a successful AgentStatus result:

- common terminal status = `Ok`;
- management body tag = `1`;
- tag is followed by the existing five-byte Agent status snapshot codec.

For the current `Ready / protocol 1.0` snapshot, the existing five-byte snapshot is:

`0200010000`

Therefore the exact successful command-specific management body is:

`010200010000`

and the complete local terminal-response payload after the common two-byte `Ok` prefix is:

`0000010200010000`

Payload length:

`8` bytes

A successful future probe must also require:

- outer local kind `Response`;
- exact same request ID as the request;
- terminal status `Ok(0)`;
- management body tag `1`;
- status snapshot `Ready(2)`;
- protocol `1.0`;
- no trailing body bytes.

A complete successful Response frame is 32 bytes including the existing 24-byte local IPC header.

## Expected side effects

The selected AgentStatus operation is read-only.

A successful probe is expected to cause only bounded same-UID local IPC request/response processing.

It does not authorize or require:

- filesystem read/write authority beyond the already-running socket path custody checks;
- filesystem management operation;
- terminal PTY/process;
- transfer provider;
- forwarding socket;
- network egress;
- DNS mutation;
- service lifecycle mutation;
- configured-remote activation;
- credential change;
- database/control-plane mutation.

## Desktop is not the selected dispatcher

Exact desktop helper:

`apps/desktop/src/local_management_ipc.rs`

Blob:

`b8908ee573e74a25cc74c51d473820a702c6e349`

It can canonical-encode a typed `BridgeCommand` and wrap it in the Agent-owned local command-3 envelope.

It explicitly performs no:

- socket I/O;
- authorization;
- dispatch;
- acknowledgement;
- completion.

WB therefore rejects using desktop dispatch by implication for this production probe.

WN does not mutate desktop source/runtime authority.

## Dedicated one-shot probe materialization status

WB requires a separately gated dedicated same-UID one-shot AgentStatus probe before production command-3 traffic.

Fresh recursive exact-WM-tree inspection identified no dedicated command-3/status-probe source path or executable source materialization.

The exact tree contains the server runtime and pure desktop request builder, but no selected production one-shot command-3 probe implementation.

Therefore:

`DEDICATED_ONE_SHOT_COMMAND3_PROBE_NOT_YET_MATERIALIZED`

and:

`NOT_READY_TO_SEND_COMMAND3_UNDER_WB_TRANSACTION_LAW`

This is a source/materialization blocker, not a server-side command-3 semantics blocker.

## Live production read-only guard

At:

`2026-09-19T17:14:08+02:00`

PowerCode proved:

- user `gersi365`, uid/gid `1000:1000`;
- service `active/running`;
- `MainPID=3033677`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `PRW_AGENT_EXECUTION_MODE=local_only`;
- exact Agent process count `1`;
- configure process count `0`;
- user-manager jobs `NONE`;
- Agent socket user-owned mode `0600`;
- Unix listener count `1`;
- Agent TCP count `0`;
- Agent UDP count `0`;
- installed NEW Agent exact:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- vendor unit exact:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- fixed 20 exact:
  `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- local-only 30 exact:
  `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- managed 40 absent;
- runtime credential user-owned mode `0400`, nlink `1`, bytes `138`;
- public identity marker:
  `b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d`.

Desktop remained:

- process count `1`;
- executable:
  `/usr/lib/private-remote-workspace/prw-desktop`;
- SHA-256:
  `1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`;
- TCP count `0`;
- UDP count `0`.

WM command-1 remained the only production command probe completed in this activation sequence.

Command-3 remained unsent.

## Readiness result

Server-side command-3 AgentStatus semantics and authority are fully rebound.

The current running NEW Agent source/binary provenance includes the selected fixed AgentStatus-only command-3 runtime.

However the transaction is not yet ready to send command-3 because WB requires a separately selected/materialized one-shot same-UID probe and no such production probe source is yet materialized.

Classification:

`COMMAND3_AUTHORITY_SEMANTICS_REBOUND / OUTER_LOCAL_COMMAND_CODE_3 / INNER_PRWC_AGENT_STATUS_OPERATION_1 / INNER_AGENTSTATUS_PRWC_EXACT_505257430001000000010000 / LOCAL_COMMAND3_PAYLOAD_EXACT_00030000000C505257430001000000010000 / SAME_UID_AUTHENTICATED_LOCAL_AUTHORITY / AGENT_STATUS_READ_ALLOW / ALL_OTHER_MANAGEMENT_DENY / WA_SERVER_RUNTIME_WIRED / INSTALLED_NEW_AGENT_WA_DERIVED / SUCCESS_RESPONSE_OK_TAG1_READY_PROTOCOL1_0 / SUCCESS_PAYLOAD_EXACT_0000010200010000 / READ_ONLY_SIDE_EFFECT_PROFILE / DESKTOP_DISPATCH_NOT_SELECTED / DEDICATED_ONE_SHOT_PROBE_NOT_YET_MATERIALIZED / NOT_READY_TO_SEND_COMMAND3 / NO_COMMAND3_REQUEST / NO_RACE_FREE_CLAIM`

## Security and mutation ceiling

WN performs no:

- command-3 request;
- second command-1 request;
- raw/ad-hoc socket write;
- desktop command-3 dispatch;
- probe-source materialization;
- production executable build/install/replacement;
- service start/stop/restart/reload;
- sudo/root action;
- managed configuration write;
- execution-mode transition;
- configured-remote activation;
- private credential read;
- credential/enrollment mutation;
- filesystem management operation;
- terminal/PTY/process operation;
- transfer operation;
- forwarding operation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation.

## Immediate successor

The immediate successor must be separately gated source materialization for the bounded same-UID one-shot command-3 AgentStatus production probe required by WB.

That source checkpoint must reuse canonical components and stop before executing the probe.

## STOP

`STOP_AFTER_COMMAND3_AUTHORITY_SEMANTICS_REBINDING_PREFLIGHT_AND_BEFORE_ONE_SHOT_PROBE_SOURCE_MATERIALIZATION_OR_COMMAND3_REQUEST`

`NO_RACE_FREE_CLAIM`
