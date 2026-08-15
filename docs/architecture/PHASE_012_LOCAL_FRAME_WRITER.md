# Phase 012 Generic Local Frame Writer

Status: approved under standing project authorization for build-phase implementation

## Purpose

Complete the provider-neutral in-memory framing path by adding a generic `std::io::Write` boundary that emits one validated Phase 010 frame without activating the real Unix socket runtime.

## Decision

The writer sequence is fixed:

1. encode the validated Phase 010 frame header with the Phase 009 encoder;
2. `write_all` exactly the 24-byte header;
3. `write_all` exactly the bounded payload bytes;
4. return without implicit flush.

## Why generic Write first

The local protocol now has:

- pure header encoder/decoder;
- bounded complete frame object;
- generic reader;
- generic writer.

All of these can be validated with memory buffers before the project takes on filesystem socket lifecycle, peer credential retrieval, connection loops, backpressure, and service activation.

This keeps wire correctness distinct from OS endpoint correctness.

## Partial write semantics

Rust `write_all` may internally perform multiple writes.

If header writing fails before all 24 bytes are accepted, Phase 012 returns `HeaderIo`.

If the header is complete and payload writing then fails, Phase 012 returns `PayloadIo`.

The writer does not attempt to repair or rewind a partially written byte stream. A future connection/runtime policy must treat such a stream carefully, normally by terminating the affected connection rather than trying to resynchronize blindly.

## No implicit flush

`write_frame` does not call `flush()`.

This prevents the low-level frame codec from silently imposing a batching/backpressure policy on future buffered transports. The connection/runtime layer will own that policy explicitly.

## Exact representation

The header bytes come only from the Phase 009 encoder.

The payload bytes are emitted unchanged from `LocalIpcPayload`.

No serializer or transform is introduced.

## Round-trip proof

A focused test writes a complete frame to `Vec<u8>` and feeds those bytes to the Phase 011 reader through `Cursor`.

The decoded frame must equal the original typed frame.

This is the first end-to-end framing round trip, but it remains entirely in memory.

## Failure tests

Synthetic `Write` implementations deterministically fail:

- before any header byte is accepted, proving `HeaderIo`;
- immediately after exactly 24 header bytes are accepted, proving `PayloadIo`.

No real socket or filesystem endpoint is required.

## Dependency boundary

Only Rust standard-library I/O traits are used.

No async runtime, socket crate, serializer, cryptographic provider, or parser dependency is added.

## Explicit deferrals

Phase 012 does not implement:

- Unix socket runtime;
- SO_PEERCRED runtime enforcement;
- connection loops or concurrent requests;
- timeout/backpressure/cancellation policy;
- flush policy;
- command payload serialization;
- runtime command dispatch;
- privileged-helper protocol;
- systemd activation;
- cryptographic provider selection;
- remote networking.

## Validation requirements

Phase 012 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- exact header-then-payload bytes;
- generic writer → generic reader round trip;
- zero payload emits header only;
- header write failure classification;
- payload write failure classification after a complete header.
