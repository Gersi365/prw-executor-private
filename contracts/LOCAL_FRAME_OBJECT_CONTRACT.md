# Private Remote Workspace In-Memory Frame Contract

Version: `0.1.0`

Status: Phase 010 bounded in-memory frame baseline

## Scope

This contract defines the typed in-memory relationship between a validated Phase 007 frame header and its opaque payload bytes.

It sits after header decoding/validation and before any future payload deserialization or command dispatch.

Phase 010 performs no socket I/O.

## Bounded payload

`LocalIpcPayload` owns opaque bytes and records a validated unsigned 32-bit length.

Construction must fail when:

- the platform byte-vector length cannot be represented by the protocol's unsigned 32-bit length field; or
- the length exceeds the Phase 007 global payload ceiling of 1,048,576 bytes.

Empty payloads are valid.

The payload type does not deserialize or interpret its bytes.

## Complete frame relationship

`LocalIpcFrame` couples:

- one already validated `LocalIpcFrameHeader`;
- one bounded `LocalIpcPayload`.

Construction succeeds only when:

`header.payload_length == payload.len`

A mismatch is rejected explicitly as `PayloadLengthMismatch`.

The implementation must not:

- truncate a longer payload to the declared length;
- pad a shorter payload;
- rewrite the header silently;
- ignore extra bytes.

## Allocation boundary

Phase 010 validates an already-owned byte vector. It does not claim to prevent an upstream caller from allocating an oversized vector before calling the constructor.

A future stream reader must enforce the decoded payload limit before allocating or extending receive storage.

That future pre-allocation/runtime boundary remains separate and mandatory.

## Payload opacity

The bounded payload remains opaque.

Phase 010 does not select or implement:

- JSON;
- CBOR;
- Protocol Buffers;
- MessagePack;
- command serialization;
- response-body schemas;
- cryptographic payload processing.

## Ownership

The payload API permits:

- borrowed read access to the validated bytes;
- transfer of ownership back to the caller.

The complete frame can be split back into its validated header and bounded payload without changing either value.

## Error disclosure

The frame-object error boundary contains only bounded length metadata.

It does not contain arbitrary payload bytes, filesystem paths, credentials, keys, stack traces, or implementation secrets.

## Forbidden interpretation

Phase 010 does not authorize or implement:

- socket reads/writes;
- runtime listener activation;
- payload deserialization;
- local command dispatch;
- shell/PTY execution;
- file mutation;
- DNS/network mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic private-key operations;
- systemd activation;
- database changes;
- deployment.
