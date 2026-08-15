# Phase 011 Bounded Generic Local Frame Reader

Status: approved under standing project authorization for build-phase implementation

## Purpose

Implement the first I/O-shaped local IPC component without activating the real Unix socket: a generic reader that acquires one complete bounded frame from any `std::io::Read` implementation.

Tests use `std::io::Cursor` only.

## Decision

The reader sequence is fixed:

1. `read_exact` 24 header bytes;
2. Phase 009 decode/validation;
3. explicit conversion of validated payload length to `usize`;
4. allocation of exactly the validated payload length;
5. `read_exact` the payload;
6. Phase 010 bounded-payload construction;
7. Phase 010 header/payload coupling;
8. return exactly one frame.

This order ensures untrusted length bytes are validated before receive allocation.

## Why a generic reader first

Using `std::io::Read` separates framing correctness from Unix socket lifecycle, filesystem safety, peer credentials, async/runtime selection, and service activation.

The same protocol logic can be validated deterministically with memory cursors before any operating-system endpoint is created.

## Truncation semantics

EOF before all 24 header bytes:

- `TruncatedHeader`.

EOF after a valid header but before the full declared payload:

- `TruncatedPayload`.

No partial frame is returned in either case.

Other I/O failures are categorized separately as `HeaderIo` or `PayloadIo` without surfacing raw OS error text through the protocol-facing error value.

## Invalid header before allocation

The reader sends the complete header through the Phase 009 decoder before allocating payload storage.

Thus wrong magic, unsupported versions, invalid flags/reserved fields, zero request IDs, unknown message kinds, and payload lengths above 1 MiB fail before payload acquisition begins.

A focused test mutates only the wire payload-length field above the maximum and proves the result is `InvalidHeader(PayloadTooLarge)`, not a payload-read error.

## Stream preservation

The reader consumes only the current frame.

Bytes following the declared payload remain in the underlying stream for subsequent processing.

A zero-length payload therefore consumes no bytes from the next frame/data segment.

## Platform length conversion

The wire length is `u32`; memory indexing/allocation uses `usize`.

The reader uses an explicit checked conversion. A target unable to represent the validated length fails with `PayloadLengthUnsupported` rather than using an unchecked cast.

## Runtime still inactive

Phase 011 does not instantiate the reader over a Unix socket in production code.

It does not:

- create an XDG runtime directory;
- bind/listen/accept/connect a socket;
- retrieve `SO_PEERCRED`;
- start the Agent as a service;
- choose sync versus async socket runtime for production.

## Dependency boundary

Only the Rust standard library is used.

No async runtime, socket crate, serializer, cryptographic provider, or parser dependency is introduced.

## Explicit deferrals

Phase 011 does not implement:

- generic frame writer;
- actual Unix socket runtime;
- peer-credential runtime enforcement;
- connection loop/multiple-frame policy;
- timeout/cancellation policy;
- command payload serialization;
- runtime command dispatch;
- privileged-helper protocol;
- systemd activation;
- cryptographic provider selection;
- remote networking.

## Validation requirements

Phase 011 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- one valid frame is read and following bytes remain unread;
- truncated header is rejected;
- oversized declared payload is rejected during header validation before payload read;
- truncated payload is rejected;
- zero-length payload does not consume following bytes.
