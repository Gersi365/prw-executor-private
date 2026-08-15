# Private Remote Workspace Local Frame Reader Contract

Version: `0.1.0`

Status: Phase 011 bounded generic frame-reader baseline

## Scope

This contract defines how one complete local IPC frame is acquired from a generic byte stream after the Phase 007 framing, Phase 009 header codec, and Phase 010 in-memory frame invariants are locked.

The implementation is generic over Rust `std::io::Read`.

Phase 011 does not bind, accept, connect to, configure, or authenticate a Unix socket. Tests use in-memory cursors only.

## Required read order

The reader must process one frame in this order:

1. acquire exactly 24 header bytes;
2. decode and validate the header through the Phase 009 codec;
3. reject any invalid header before allocating payload receive storage;
4. convert the already bounded payload length to the target allocation/index type;
5. allocate exactly the validated payload length;
6. acquire exactly that many payload bytes;
7. construct the Phase 010 bounded payload;
8. construct the Phase 010 complete frame;
9. return without consuming bytes belonging to the next frame.

The implementation must not allocate payload storage from an unchecked wire length.

## Header truncation

If EOF occurs before all 24 header bytes are acquired, the reader returns:

- `TruncatedHeader`.

A non-EOF I/O failure while acquiring the header returns:

- `HeaderIo`.

The bounded public error taxonomy does not expose the underlying operating-system error text.

## Invalid complete header

A complete 24-byte header is passed to the Phase 009 fail-closed decoder.

An invalid decoded header returns:

- `InvalidHeader(LocalIpcFrameDecodeError)`.

This validation occurs before payload allocation or payload read.

## Payload allocation bound

The Phase 009/007 header validator already guarantees that declared payload length does not exceed 1 MiB.

The reader must also ensure the validated unsigned 32-bit length can be represented by the target's `usize` before allocation.

If not representable, it returns:

- `PayloadLengthUnsupported`.

On the intended 64-bit Ubuntu target this condition is not expected, but the conversion remains explicit rather than relying on an unchecked cast.

## Payload truncation

After the validated allocation is created, the reader acquires exactly the declared payload byte count.

If EOF occurs early, it returns:

- `TruncatedPayload`.

A non-EOF I/O failure while acquiring payload bytes returns:

- `PayloadIo`.

The reader must not return a partial `LocalIpcFrame`.

## Internal invariants

After exact acquisition, Phase 010 construction should succeed by construction.

Unexpected failure at those internal boundaries is classified as:

- `PayloadInvariant`;
- `FrameInvariant`.

Those classifications remain bounded and do not expose raw payload bytes.

## Stream preservation

The reader consumes exactly one frame.

Bytes following the declared payload remain unread for the caller or the next frame read.

This permits multiple sequential frames on one future stream without treating trailing bytes as part of the current payload.

## Zero-length payload

A validated payload length of zero is supported.

The reader must not consume bytes from the following frame merely because the current frame has no payload.

## Error disclosure

The public read error boundary contains bounded categories only.

It must not expose:

- raw peer payload;
- credentials;
- private keys;
- filesystem secrets;
- stack traces;
- raw operating-system error messages.

Protected diagnostic logging may be designed separately.

## Forbidden interpretation

Phase 011 does not authorize or implement:

- Unix socket binding/listening/connection;
- `SO_PEERCRED` runtime retrieval;
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
