# Private Remote Workspace Local Frame Writer Contract

Version: `0.1.0`

Status: Phase 012 generic frame-writer baseline

## Scope

This contract defines how one complete validated Phase 010 local IPC frame is emitted to a generic Rust `std::io::Write` byte stream.

Phase 012 does not bind, connect to, configure, or authenticate a Unix socket. Tests use memory buffers and deterministic synthetic writers only.

## Write order

The writer must emit one frame in this exact order:

1. encode the validated header through the Phase 009 codec;
2. write all 24 encoded header bytes;
3. only after the header is complete, write all bounded payload bytes;
4. return without adding separators, padding, trailers, or bytes belonging to another frame.

The writer uses the already validated Phase 010 frame and must not recompute, clamp, or silently modify payload length metadata.

## Header write failure

If the complete 24-byte header cannot be written, the operation fails as:

- `HeaderIo`.

No protocol-facing raw operating-system error text is exposed through this bounded classification.

The caller must treat the underlying stream state after a partial failed write as connection/runtime state requiring its own recovery policy. Phase 012 does not attempt to rewind or repair a partially written stream.

## Payload write failure

If the header is complete but the bounded payload cannot be fully written, the operation fails as:

- `PayloadIo`.

The writer does not retry indefinitely, pad missing bytes, or emit a replacement frame.

Connection retry/recovery semantics remain a future runtime concern.

## Zero-length payload

For a validated zero-length payload, the writer emits exactly the 24-byte header and no payload bytes.

## Flush policy

The generic writer deliberately does not call `flush()`.

Flush, buffering, batching, and socket backpressure policy belong to the future connection/runtime layer. A frame write therefore means the bytes were accepted by the provided `Write` implementation, not that they were durably transmitted to a peer or flushed through every underlying buffer.

## Exact byte representation

The writer must use the Phase 009 header encoder rather than constructing an alternate byte layout.

The payload is emitted byte-for-byte as stored by the Phase 010 bounded payload object.

No payload serialization, compression, encryption, transformation, or canonicalization is performed in Phase 012.

## Reader/writer interoperability

A frame emitted by the Phase 012 generic writer must round-trip through the Phase 011 generic reader when both operate over an in-memory byte stream.

This validates that the writer and reader share the locked Phase 007 framing contract without requiring a real socket.

## Error disclosure

The public write error boundary exposes only:

- HeaderIo;
- PayloadIo.

It must not expose raw payload bytes, credentials, private keys, filesystem secrets, stack traces, or raw operating-system error text.

## Forbidden interpretation

Phase 012 does not authorize or implement:

- Unix socket binding/listening/connection;
- SO_PEERCRED retrieval;
- production connection retry policy;
- command payload serialization;
- command dispatch;
- shell/PTY execution;
- file/network/DNS mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic private-key operations;
- systemd activation;
- database changes;
- deployment.
