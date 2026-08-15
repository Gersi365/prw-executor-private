# Private Remote Workspace Local Command Request Codec Contract

Version: `0.1.0`

Status: Phase 015 read-only command request payload baseline

## Scope

This contract defines the exact opaque Phase 007 payload bytes used to request one of the two Phase 008 read-only local Agent commands.

It does not define response-body serialization, command execution, socket runtime, authentication, or mutating operations.

## Request payload

The Phase 015 request payload is exactly:

`2 bytes`

The two bytes contain one unsigned 16-bit command identifier in network byte order (big-endian).

No additional bytes are permitted in the current request payload.

Current encodings:

- `GetAgentStatus` (code 1) -> `00 01`
- `GetPrivateDnsConfig` (code 2) -> `00 02`

The two current commands have no request arguments.

## Decode rules

A request payload is accepted only when:

1. its length is exactly two bytes;
2. the big-endian unsigned 16-bit value maps to a command in the active local command namespace.

Failure classes:

- `InvalidLength`;
- `UnknownCommand`.

The decoder must not:

- ignore trailing bytes;
- pad a short request;
- treat an unknown identifier as a raw/generic command;
- guess an alternate byte order;
- interpret text command names.

## Encoding rules

The encoder receives a typed `LocalAgentCommand` and returns exactly two bytes.

Encoding is infallible for a valid typed command.

The stable Phase 008 command-code mapping remains the authoritative numeric namespace; Phase 015 does not create a second independent identifier table.

## Frame relationship

The request command payload lives inside a Phase 007 Request frame.

A future higher-level decoder must therefore enforce both boundaries:

- frame kind must be `Request`;
- payload bytes must satisfy this exact Phase 015 command request codec.

Phase 015 itself operates only on payload bytes and does not inspect the surrounding frame.

## Authorization relationship

Successful payload decoding means only that a command identifier is structurally recognized.

It does not establish authorization.

The future Agent dispatcher remains responsible for checking the authenticated connection state and any command-specific capability policy before executing a command.

## Response serialization

Phase 015 deliberately does not define response bytes.

The typed Phase 008 `LocalAgentResponseStatus` exists, but its response-body encoding and the concrete schemas for:

- Agent status;
- private-DNS configuration;

remain separate future contracts.

## Dependency boundary

The implementation uses only Rust standard-library integer byte-order and slice conversion operations.

No JSON, CBOR, Protocol Buffers, MessagePack, parser, async runtime, socket, or cryptographic dependency is introduced.

## Forbidden interpretation

Phase 015 does not authorize or implement:

- command dispatch or execution;
- shell/PTY execution;
- file mutation;
- DNS/network mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic operations;
- socket activation;
- systemd activation;
- database changes;
- deployment.
