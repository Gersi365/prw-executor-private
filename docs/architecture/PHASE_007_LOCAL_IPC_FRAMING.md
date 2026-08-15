# Phase 007 Local IPC Framing and Versioning

Status: approved under standing project authorization for build-phase implementation

## Purpose

Define a deterministic, bounded frame boundary for the Phase 006 Unix-domain stream transport before selecting command serialization or implementing socket runtime code.

## Decision

The initial PRW local IPC frame uses:

- fixed 24-byte header;
- magic bytes `PRW\0`;
- protocol version `1.0`;
- three message kinds: Request, Response, Error;
- non-zero unsigned 64-bit request ID;
- unsigned 32-bit payload length;
- maximum payload length 1 MiB;
- network byte order for multi-byte integers;
- opaque payload bytes.

## Wire header

Version 1.0 header layout:

```text
0               4  magic = PRW\0
4               2  major = 1
6               2  minor = 0
8               1  kind
9               1  flags = 0
10              2  reserved = 0
12              8  request_id != 0
20              4  payload_length <= 1 MiB
24                 payload begins
```

Message-kind codes:

- 1 = Request
- 2 = Response
- 3 = Error

All multi-byte integers use big-endian byte order.

## Why an explicit fixed header

`SOCK_STREAM` preserves byte order but not application message boundaries. PRW therefore needs a framing contract before any runtime reader can safely determine where one request ends and the next begins.

The header keeps the minimum control metadata independent of the eventual payload serialization:

- a fixed magic value prevents arbitrary bytes from being silently interpreted as PRW frames;
- explicit version fields prevent implicit schema guessing;
- message kind distinguishes request and terminal response classes;
- request ID provides bounded correlation for concurrent logical exchanges;
- payload length permits exact stream reads and an enforceable memory/processing bound.

## Version policy

Only exact version `1.0` is accepted by the current contract.

This is intentionally conservative. No forward-minor compatibility policy is assumed before one is specified and tested.

A future decoder must reject unsupported versions before interpreting payload bytes.

## Request ID policy

Request ID zero is reserved.

On one connection, a client must not reuse a request ID while the earlier request with that ID remains outstanding.

Response and Error frames echo the originating request ID.

Request IDs are correlation metadata only. They are not credentials, capabilities, nonces, authentication tokens, or durable object identifiers.

## Bound rationale

The maximum opaque payload is 1 MiB.

The Agent IPC channel is intended for bounded control operations and metadata, not as a replacement bulk-transfer pipe. A global ceiling prevents an attacker or defective local client from declaring arbitrarily large frame allocations before more granular command limits exist.

Large file content belongs to the dedicated transfer architecture. Future command contracts may impose substantially smaller limits.

## Fail-closed decoder requirements

A future decoder must reject malformed frames before dispatch when any of these conditions is present:

- wrong magic;
- unsupported version;
- unknown message kind;
- non-zero version-1.0 flags;
- non-zero version-1.0 reserved field;
- zero request ID;
- payload length above the global maximum;
- truncated header;
- truncated payload.

The decoder must not attempt version guessing or unbounded buffering after a framing error.

## Typed implementation boundary

`prw-agent` records:

- frame magic/header/max-payload constants;
- current protocol version type;
- message-kind enum;
- non-zero request ID type;
- validated frame-header metadata;
- focused validation tests.

The Rust types do not yet encode/decode the 24-byte wire header. That runtime/parser implementation is deliberately separate from the contract lock.

## Explicit deferrals

Phase 007 does not select or implement:

- payload serialization/schema;
- command identifiers;
- command capability mapping;
- error-code schema;
- actual socket read/write loops;
- frame encoder/decoder;
- timeout behavior;
- cancellation behavior;
- unsolicited event messages;
- authentication beyond Phase 006 local peer credentials;
- privileged-helper protocol;
- systemd activation;
- cryptographic provider;
- remote control-plane protocol.

## Validation requirements

Phase 007 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- magic/header/payload limits are locked;
- current version is `1.0`;
- unsupported versions are rejected;
- request ID zero is rejected;
- payload length is bounded;
- validated headers preserve version, kind, request ID, and payload length.
