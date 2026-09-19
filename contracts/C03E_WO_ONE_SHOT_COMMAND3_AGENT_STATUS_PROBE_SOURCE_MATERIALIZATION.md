# C03e-WO — One-Shot Command-3 AgentStatus Probe Source Materialization

Status: `SOURCE_MATERIALIZATION — PRODUCTION_COMMAND3_EXECUTION_NOT_AUTHORIZED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Exact predecessor

Evidence-closed C03e-WN / PR #730 is the exact predecessor:

- exact head:
  `ed0da8ff1ecf303fdd02b057bc7156e0f7521401`
- exact tree:
  `d7d20d111413b32bab33a869df42b1025aa7f63a`
- canonical WN evidence:
  `1pTxUCUPzDMtFjKCMeF25LTCIAsoZiW7H`

WN proved the installed NEW Agent contains the selected server-side AgentStatus-only command-3 path and stopped because WB requires a dedicated same-UID one-shot probe before any production command-3 request.

## Selected source ceiling

WO adds exactly one executable source target to the existing `prw-agent` package:

`crates/prw-agent/src/bin/prw-agent-command3-agent-status-probe.rs`

No Cargo manifest or lockfile edit is selected.

The existing package already owns every dependency needed by this source:

- `prw-agent` library protocol/runtime types;
- `prw-remote-bridge`;
- `rustix`;
- the Rust standard library Unix socket/filesystem APIs.

The executable is not installed into a production path by WO.

## Why this placement is selected

A `prw-agent` package bin target is the narrowest source placement that can directly reuse the existing public Agent local-IPC protocol surface without:

- widening desktop runtime authority;
- creating a generic management client library;
- adding a new workspace package;
- adding a dependency;
- adding a configuration surface;
- changing the production Agent binary;
- changing service/systemd packaging.

Desktop remains outside the execution path.

## Exact operation ceiling

The source constructs exactly:

`BridgeCommand::AgentStatus`

It contains no caller-selected `BridgeCommand`, no generic operation parser and no management operation argument.

The inner canonical PRWC encoding remains operation code `1`.

The outer Agent-owned management command remains code `3`.

The source obtains the request frame only through:

`build_local_management_request_frame(...)`

No raw command-3 byte writer is materialized.

## Request-ID law

WO selects one fixed non-zero request ID for this one-shot executable:

`0x574f000000000001`

The value is process-static and exists only for this one-shot deployment probe.

The probe requires the response to carry the exact same request ID.

A fresh Unix-stream connection is created for the one request/response transaction.

## Endpoint and same-UID custody

The source accepts no endpoint path argument.

It resolves only `XDG_RUNTIME_DIR` and rejects:

- missing value;
- empty value;
- relative value.

The Agent socket path is derived only through:

`LocalIpcContract::socket_path(...)`

Before connect, the source validates with `symlink_metadata`:

1. XDG runtime root:
   - directory;
   - owner = current effective UID;
   - exact locked runtime-directory mode;

2. PRW runtime directory:
   - directory;
   - owner = current effective UID;
   - exact locked runtime-directory mode;

3. Agent socket:
   - Unix socket;
   - owner = current effective UID;
   - exact locked socket mode.

After the Unix connection and timeout setup, the exact endpoint checks run again before any request write.

This reduces pathname drift exposure but is not a race-free pathname proof.

The production Agent still owns authoritative peer authentication through its kernel-backed same-UID connection boundary.

No sudo/root path exists in the probe.

## Exact one-shot I/O path

One process invocation can perform at most one selected request transaction:

1. resolve/validate trusted endpoint;
2. construct fixed request ID;
3. canonical-encode `BridgeCommand::AgentStatus`;
4. build the Agent-owned command-3 local frame;
5. connect one Unix stream;
6. apply two-second read and write timeouts;
7. revalidate endpoint custody;
8. write exactly one complete frame with existing `write_frame`;
9. flush once;
10. read exactly one complete frame with existing `read_frame`;
11. validate through existing terminal-response validation;
12. require exact request-ID correlation;
13. require terminal `Ok`;
14. decode common response status through the existing response codec;
15. require management result tag `1`;
16. decode exactly five status-snapshot bytes through the existing status codec;
17. require `Ready`;
18. return success and drop the stream/process.

There is no retry and no second command request.

## Response success ceiling

Success requires all of:

- terminal outer kind consistent with `Ok`;
- exact request ID;
- terminal status exactly `Ok`;
- management result tag exactly `1`;
- exactly five bytes accepted by `decode_status_snapshot`;
- current supported local IPC protocol;
- runtime state `Ready`;
- no trailing management-body bytes.

The source therefore retains the WN success semantics:

`0000010200010000`

for the current Ready/protocol-1.0 response payload.

## Error behavior

Probe errors are coarse and do not serialize implementation-sensitive provider details.

Any endpoint-trust, connect, timeout, build, write, flush, read, terminal-validation, correlation, response-tag, status-codec or non-Ready failure exits non-zero.

No fallback operation is attempted.

## Deterministic source tests

WO source contains tests that perform no production socket I/O and prove:

- exact canonical AgentStatus PRWC request bytes;
- exact Agent-owned command-3 payload;
- complete request frame length of 42 bytes;
- exact Ok/tag-1/Ready response acceptance;
- wrong result tag fails closed;
- non-Ready status fails closed;
- mismatched request ID fails closed.

The tests use in-memory frame objects only.

## Production ceiling

Source materialization and CI do not execute the binary.

WO performs no production command-3 request.

WO performs no command-1 request.

WO does not install the probe executable into:

- `/usr/bin`;
- `/usr/lib/private-remote-workspace`;
- the desktop installation;
- the Agent service unit;
- any systemd drop-in.

A read-only temporary build may establish exact binary identity, but the binary must not be executed in WO.

## Explicit non-actions

WO performs no:

- production command-3 request;
- production command-1 request;
- raw/ad-hoc socket write;
- desktop command-3 dispatch;
- production Agent mutation;
- production desktop mutation;
- service start/stop/restart/reload;
- sudo/root action;
- managed configuration write;
- configured-remote activation;
- credential/private-key read or mutation;
- terminal/files/transfer/forwarding operation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation;
- probe installation into a production path;
- probe execution;
- merge/ready/close;
- branch deletion;
- reset/rebase/squash/force/history rewrite.

## Validation requirement

The exact final WO head must pass Rust workspace validation.

A separate read-only build of the exact final source may record:

- exact source head/tree;
- exact probe source blob;
- exact Cargo.lock SHA-256;
- build command;
- build host/toolchain;
- exact temporary probe binary bytes/SHA-256.

That build is provenance only and grants no execution authority.

## Classification target

`ONE_SHOT_COMMAND3_AGENT_STATUS_PROBE_SOURCE_MATERIALIZED / EXISTING_PRW_AGENT_PACKAGE_BIN_TARGET / NO_CARGO_MANIFEST_CHANGE / NO_LOCKFILE_CHANGE / FIXED_AGENT_STATUS_ONLY / NO_GENERIC_MANAGEMENT_COMMAND_INPUT / FIXED_REQUEST_ID / TRUSTED_LOCAL_IPC_ENDPOINT_DERIVATION / SAME_UID_OWNERSHIP_AND_MODE_CHECKS / ENDPOINT_REVALIDATED_BEFORE_WRITE / EXISTING_FRAME_IO_REUSED / EXISTING_TERMINAL_RESPONSE_VALIDATION_REUSED / EXISTING_STATUS_CODEC_REUSED / EXACT_CORRELATION_REQUIRED / OK_TAG1_READY_REQUIRED / NO_RETRY / NO_DESKTOP_DISPATCH / NO_PRODUCTION_INSTALL / NO_PROBE_EXECUTION / NO_COMMAND3_REQUEST / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_ONE_SHOT_COMMAND3_AGENT_STATUS_PROBE_SOURCE_MATERIALIZATION_AND_VALIDATION_AND_BEFORE_PROBE_INSTALL_OR_PRODUCTION_COMMAND3_EXECUTION`

`NO_RACE_FREE_CLAIM`
