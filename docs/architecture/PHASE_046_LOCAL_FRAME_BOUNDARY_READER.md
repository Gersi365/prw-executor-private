# Phase 046 — Local Frame-Boundary Reader

## Objective

Add a pure generic-I/O boundary reader that lets a future connection loop distinguish clean EOF before a new frame from truncation after a frame has begun.

## Design

`read_frame_at_boundary()` performs one first-byte probe over caller-supplied `std::io::Read`.

- `read == 0` before any frame byte → `CleanEof`.
- `Interrupted` → retry the first-byte probe.
- other first-byte I/O error → `LocalIpcFrameReadError::HeaderIo`.
- one byte acquired → prepend that byte back into a temporary chained reader and delegate to the existing Phase 011 `read_frame()` implementation.

This preserves a single authoritative complete-frame decoder and its existing validation/error taxonomy.

## Why this is separate from `read_frame()`

Phase 011 defines acquisition of one required frame; for that API, empty input is correctly a truncated required header. A connection loop has different lifecycle information: empty input before the next frame begins is an orderly peer close. Keeping the APIs separate avoids weakening or silently changing the Phase 011 contract.

## State interaction

Phase 046 itself owns no connection state. A later pure loop layer may map:

- `CleanEof` → normal connection stop;
- `TruncatedHeader`, `TruncatedPayload`, invalid frame, or I/O failure → existing inbound failure/connection-discard handling.

## Runtime boundary

No socket API, file descriptor, peer credential, filesystem path, timer, task, thread, service activation, DNS/network mutation, database work, or deployment is introduced.
