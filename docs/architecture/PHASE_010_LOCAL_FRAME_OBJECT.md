# Phase 010 Bounded In-Memory Local Frame

Status: approved under standing project authorization for build-phase implementation

## Purpose

Introduce a typed boundary that proves a validated Phase 007 header and its in-memory opaque payload agree on length before any future payload parser or command dispatcher receives them.

## Decision

Phase 010 adds two pure in-memory types:

- `LocalIpcPayload` — owned opaque bytes whose length is representable by the wire field and does not exceed the 1 MiB global control-channel ceiling;
- `LocalIpcFrame` — a validated header plus a bounded payload whose actual length exactly equals the header's declared payload length.

## Why this boundary exists

Phase 009 validates the fixed header independently of payload bytes. A future stream reader will then acquire the declared payload.

Before payload interpretation, PRW needs one explicit invariant:

`declared payload length == actual payload length`

Without this boundary, downstream logic could accidentally process truncated data, ignore trailing bytes, or trust stale length metadata.

## Fail-closed rules

`LocalIpcPayload` rejects bytes when:

- their platform length cannot fit a `u32`; or
- the length exceeds `LOCAL_IPC_MAX_PAYLOAD_LENGTH`.

`LocalIpcFrame` rejects a header/payload pair when their lengths differ.

It never repairs a mismatch by truncating, padding, or rewriting metadata.

## Allocation caveat

The Phase 010 constructor receives an already allocated `Vec<u8>`.

Therefore this phase does not by itself prevent an upstream component from allocating too much memory before validation. The future stream-reader implementation must validate the Phase 009 header first and enforce the 1 MiB bound before allocating or extending payload receive storage.

This limitation is explicit rather than hidden.

## Payload remains opaque

No payload serializer or schema is selected.

The Phase 008 command namespace still exists only as typed domain metadata. Phase 010 does not map it onto bytes.

## Runtime remains inactive

No Unix-domain socket is bound or read. No Agent listener is activated. No command is dispatched.

## Dependency boundary

The implementation uses only Rust standard-library types and conversions.

No networking, async runtime, serializer, crypto provider, or parser dependency is introduced.

## Explicit deferrals

Phase 010 does not implement:

- bounded socket payload acquisition;
- truncated stream handling;
- frame serialization as one contiguous byte vector;
- command payload serialization;
- runtime command dispatch;
- timeout/cancellation semantics;
- privileged-helper protocol;
- systemd activation;
- crypto-provider selection;
- remote networking.

## Validation requirements

Phase 010 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- payload at exactly 1 MiB is accepted;
- payload above 1 MiB is rejected;
- matching header/payload length is accepted;
- mismatched length is rejected;
- zero-length payload is valid when declared length is zero.
