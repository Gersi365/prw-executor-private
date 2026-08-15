# Private Remote Workspace Local Frame-Boundary Reader Contract

Version: `0.1.0`

Status: Phase 046 provider-neutral clean-EOF boundary

## Scope

Phase 046 defines how a future connection loop distinguishes an orderly peer close at a frame boundary from a truncated next frame while remaining generic over Rust `std::io::Read`.

It does not bind, accept, connect, configure, authenticate, or close a Unix socket.

## Compatibility with Phase 011

The existing Phase 011 `read_frame()` contract remains unchanged.

Calling `read_frame()` when fewer than 24 header bytes are available continues to return `TruncatedHeader`, including an empty input stream.

Phase 046 adds a separate boundary-aware entry point for callers that own connection-loop semantics.

## Boundary classification

Before delegating to the existing complete-frame reader, the boundary-aware reader acquires at most the first byte of the next frame.

The outcomes are:

- EOF before any byte is acquired: `CleanEof`;
- one byte acquired: re-prefix that byte and delegate complete-frame acquisition to Phase 011;
- non-EOF I/O failure before the first byte: bounded `HeaderIo`;
- interrupted first-byte read: retry without changing classification.

After one byte has been acquired, all existing Phase 011 validation and error classes remain authoritative. Therefore EOF before the rest of the 24-byte header completes is `TruncatedHeader`, and EOF inside a validated payload is `TruncatedPayload`.

## Stream preservation

A successful boundary read consumes exactly one frame and must not consume bytes belonging to the following frame.

`CleanEof` consumes no application bytes.

## Security behavior

The boundary distinction is protocol lifecycle metadata only. It does not authenticate a peer, authorize a command, weaken frame validation, expose raw operating-system errors, or convert malformed/truncated input into a clean shutdown.

## Forbidden interpretation

Phase 046 does not authorize or implement:

- Unix socket bind/listen/accept/connect/close;
- `SO_PEERCRED` retrieval;
- XDG runtime-directory or socket-path mutation;
- timeout/timer behavior;
- concurrent connection handling;
- authentication or identity binding;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- deployment.
