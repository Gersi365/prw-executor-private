# Private Remote Workspace Local Frame Codec Contract

Version: `0.1.0`

Status: Phase 009 pure frame-header codec baseline

## Scope

This contract governs the pure encode/decode implementation for the fixed 24-byte local IPC header locked by Phase 007.

The codec operates on one complete header value in memory.

It does not:

- read from or write to a socket;
- allocate or parse the opaque payload;
- dispatch a command;
- select a payload serializer;
- perform authentication or authorization beyond validating frame metadata.

## Encoder

The encoder accepts an already validated `LocalIpcFrameHeader` and produces exactly 24 bytes.

It must encode:

- offsets 0..4: magic `PRW\0`;
- offsets 4..6: major version, big-endian;
- offsets 6..8: minor version, big-endian;
- offset 8: message-kind code;
- offset 9: zero flags;
- offsets 10..12: zero reserved field;
- offsets 12..20: non-zero request ID, big-endian;
- offsets 20..24: payload length, big-endian.

Because the typed input has already passed Phase 007 metadata validation, encoding is infallible.

## Decoder

The decoder accepts exactly 24 bytes and must fail closed before returning typed frame metadata if any invariant is violated.

Required rejection classes:

- invalid magic;
- unknown message kind;
- non-zero version-1.0 flags;
- non-zero version-1.0 reserved field;
- zero request ID;
- unsupported protocol version;
- payload length above 1 MiB.

The decoder must reconstruct all multi-byte values from big-endian bytes.

It must route reconstructed version, request ID, and payload length through the existing typed validators rather than duplicating weaker acceptance rules.

## Truncation boundary

The pure Phase 009 decoder takes an exact 24-byte array. Therefore header truncation is outside this function's input domain.

A future stream reader is responsible for acquiring exactly 24 header bytes and must classify EOF before 24 bytes as a truncated-header protocol failure.

The future stream reader must likewise acquire exactly the declared payload bytes and classify early EOF as a truncated-payload failure.

## Payload boundary

Phase 009 does not read, allocate, copy, deserialize, or interpret the payload.

The decoded payload length is metadata only.

A future runtime reader must enforce the validated length before allocating or reading payload storage.

## No protocol guessing

The decoder must not:

- guess alternate magic values;
- accept unknown message kinds as a generic/raw command;
- ignore non-zero reserved fields;
- downgrade unknown protocol versions;
- clamp an oversized declared payload into the supported range.

Invalid metadata is rejected explicitly.

## Error disclosure

`LocalIpcFrameDecodeError` identifies only bounded framing classes.

It contains no raw payload, filesystem path, process credential, stack trace, or secret-bearing diagnostic data.

## Forbidden interpretation

Phase 009 does not authorize or implement:

- local socket runtime;
- command execution;
- shell/PTY execution;
- file mutation;
- network/DNS mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic operations;
- systemd activation;
- database changes;
- deployment.
