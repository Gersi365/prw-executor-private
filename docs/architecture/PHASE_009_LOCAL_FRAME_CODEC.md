# Phase 009 Pure Local Frame Header Codec

Status: approved under standing project authorization for build-phase implementation

## Purpose

Turn the Phase 007 fixed frame-header contract into a deterministic pure-Rust encode/decode boundary before implementing any socket reader.

## Decision

Phase 009 implements only the fixed 24-byte header codec.

Encoder input:

- validated `LocalIpcFrameHeader`.

Encoder output:

- exactly 24 bytes matching the Phase 007 big-endian wire layout.

Decoder input:

- exactly 24 bytes.

Decoder output:

- validated `LocalIpcFrameHeader`, or a bounded framing error.

## Fail-closed validation order

The decoder validates structural fixed fields before returning typed metadata:

1. `PRW\0` magic.
2. zero version-1.0 flags.
3. zero version-1.0 reserved field.
4. known message kind.
5. non-zero request ID.
6. exact supported protocol version.
7. payload length at or below the 1 MiB global ceiling.

Version/request/payload constraints are routed through the existing typed constructors where applicable.

## No payload allocation

The header codec does not allocate based on the declared payload length and does not receive payload bytes.

This separation matters because an untrusted local peer must not be able to force an allocation merely by providing an unchecked length field.

A later stream-reader phase must decode and validate the header before any bounded payload acquisition.

## Truncated streams remain deferred

A function whose input type is `[u8; 24]` cannot represent a truncated header.

A future socket reader must explicitly handle:

- EOF before 24 header bytes;
- EOF before the validated payload length;
- read timeouts/cancellation;
- connection closure between frames.

These concerns are deliberately not mixed into the pure codec.

## No dependency

The implementation uses only Rust standard-library integer byte-order operations.

No serialization, networking, async-runtime, cryptographic, or parser dependency is added.

## Error taxonomy

The pure decoder distinguishes:

- InvalidMagic
- UnknownMessageKind
- NonZeroFlags
- NonZeroReserved
- ZeroRequestId
- UnsupportedVersion
- PayloadTooLarge

These errors are protocol-control metadata only and do not echo arbitrary peer data.

## Explicit deferrals

Phase 009 does not implement:

- socket reads/writes;
- payload acquisition;
- payload serialization;
- Phase 008 command encoding;
- runtime command dispatch;
- timeouts or cancellation;
- privileged-helper IPC;
- systemd activation;
- cryptographic provider selection;
- remote networking.

## Validation requirements

Phase 009 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- valid header encode/decode round trip;
- exact locked byte layout;
- rejection of wrong magic;
- rejection of unknown kind;
- rejection of non-zero flags/reserved bytes;
- rejection of zero request ID;
- rejection of unsupported version;
- rejection of payload length above the global bound.
