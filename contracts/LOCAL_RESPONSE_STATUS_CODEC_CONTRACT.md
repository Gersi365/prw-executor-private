# Private Remote Workspace Local Response Status Codec Contract

Version: `0.1.0`

Status: Phase 016 response-status prefix baseline

## Scope

This contract defines the mandatory status prefix at the start of a future local Agent terminal response payload.

It builds on the Phase 008 typed response-status taxonomy but does not yet define command-specific response body schemas.

## Status prefix

Every terminal local Agent response payload begins with exactly two bytes containing one unsigned 16-bit response-status identifier in network byte order (big-endian).

Current status mappings:

- `Ok` = `00 00`
- `InvalidRequest` = `00 01`
- `Unauthorized` = `00 02`
- `UnsupportedCommand` = `00 03`
- `Conflict` = `00 04`
- `InternalError` = `00 05`

Any bytes after the two-byte prefix are command-specific response body bytes and remain opaque to the Phase 016 codec.

## Decode rules

Decoding fails when:

- fewer than two bytes are available: `MissingStatus`;
- the unsigned 16-bit identifier is not part of the active response taxonomy: `UnknownStatus`.

The decoder preserves all body bytes after the prefix exactly and does not parse or copy them into a new schema.

## Success semantics

`Ok` is the only successful status in the current taxonomy.

All other statuses represent terminal failure conditions.

This distinction is typed through `LocalAgentResponseStatus::is_success()`.

## Frame-kind relationship

Phase 016 does not inspect the outer Phase 007 frame kind.

A future combined protocol decoder must lock and enforce the relationship between:

- `Response` frame kind and success/failure statuses;
- `Error` frame kind and failure statuses.

Phase 016 does not silently assume that relationship before it is specified.

## Body schema boundary

The status prefix is intentionally independent of command-specific body bytes.

The following remain separate future schemas:

- `GetAgentStatus` successful body;
- `GetPrivateDnsConfig` successful body;
- any structured bounded error detail.

A failed response is not authorized to include arbitrary stack traces, raw operating-system errors, credentials, private keys, or other secret-bearing implementation detail.

## No general serializer

The status prefix uses only the existing stable numeric status identifiers and standard-library big-endian conversion.

Phase 016 does not add JSON, CBOR, Protocol Buffers, MessagePack, or another serialization dependency.

## Forbidden interpretation

Phase 016 does not authorize or implement:

- response dispatch;
- command execution;
- Unix socket runtime;
- shell/PTY execution;
- file/network/DNS mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic operations;
- systemd activation;
- database changes;
- deployment.
