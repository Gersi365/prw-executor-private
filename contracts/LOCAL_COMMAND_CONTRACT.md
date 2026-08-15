# Private Remote Workspace Local Command Contract

Version: `0.1.0`

Status: Phase 008 bounded read-only local command baseline

## Scope

This contract defines the first typed command namespace carried by the Phase 007 local IPC framing contract.

It applies only after the Phase 006 local peer-authentication boundary has accepted the connection.

Phase 008 does not define a serializer, byte-level command payload, socket runtime, privileged-helper command surface, remote protocol, or mutating host operation.

## Initial command namespace

The only commands admitted by the Phase 008 baseline are read-only:

| Code | Command | Meaning |
| ---: | --- | --- |
| 1 | `GetAgentStatus` | Read the Agent's local runtime status snapshot. |
| 2 | `GetPrivateDnsConfig` | Read the effective private-DNS configuration snapshot. |

The command namespace is intentionally narrow.

Phase 008 does not include:

- shell execution;
- terminal execution;
- file writes/deletes/moves;
- DNS mutation;
- network mutation;
- service start/stop/restart;
- policy mutation;
- enrollment mutation;
- privileged-helper commands.

Adding a future mutating command requires an explicit command contract with capability/authorization semantics and command-specific validation.

## Request envelope

A typed local request envelope contains:

- the non-zero request ID defined by the Phase 007 framing contract;
- one typed `LocalAgentCommand`.

Request ID remains correlation metadata only.

It is not:

- authentication;
- authorization;
- a capability token;
- a replay-defense nonce;
- a persistent object identifier.

## Response envelope

A terminal local response envelope contains:

- the same request ID as the request being answered;
- one typed `LocalAgentResponseStatus`.

The Phase 008 response-status namespace is:

| Code | Status | Meaning |
| ---: | --- | --- |
| 0 | `Ok` | Request completed successfully. |
| 1 | `InvalidRequest` | Request metadata or future payload is invalid. |
| 2 | `Unauthorized` | Authenticated peer lacks authority for the operation. |
| 3 | `UnsupportedCommand` | Command is not supported by the active protocol. |
| 4 | `Conflict` | Request conflicts with current Agent state. |
| 5 | `InternalError` | Internal failure without exposing sensitive implementation detail. |

The status code is terminal for one request/response exchange.

## Authorization boundary

Same-UID peer authentication from Phase 006 establishes which local user process is connected. It does not automatically grant every future Agent capability.

The Agent remains an authorization enforcement point.

For Phase 008, the admitted commands are read-only and do not alter host, network, policy, enrollment, file, service, or cryptographic state.

Future commands must be mapped to explicit capability requirements rather than relying solely on a successful socket connection.

## Error disclosure

`InternalError` is deliberately bounded and generic.

A wire protocol must not expose stack traces, filesystem secrets, private keys, credentials, environment secrets, or implementation-sensitive detail merely because a local request failed.

Detailed diagnostics may be recorded in appropriately protected local logs under a later logging/audit policy.

## Payload serialization

Phase 008 does not define how command/status identifiers or response bodies are serialized into the opaque Phase 007 frame payload.

It does not select:

- JSON;
- CBOR;
- Protocol Buffers;
- MessagePack;
- another serialization format.

It also does not yet define the concrete fields of the Agent status snapshot or private-DNS configuration response body.

Those schema decisions remain separate so that the command authorization surface is locked before a serializer is introduced.

## Response correlation

Every Response or Error frame must correlate to the same non-zero request ID as the originating Request.

A response must not silently substitute a different request ID.

The local runtime must eventually reject malformed request/response correlation rather than guessing intent.

## Forbidden interpretation

Phase 008 does not authorize or implement:

- arbitrary command execution;
- shell or PTY execution;
- file mutation;
- network/DNS/TUN/WireGuard mutation;
- systemd control;
- privileged-helper invocation;
- account authentication;
- private-key operations;
- database changes;
- deployment.
