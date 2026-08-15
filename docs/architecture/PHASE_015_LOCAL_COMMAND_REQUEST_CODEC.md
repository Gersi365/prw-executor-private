# Phase 015 Pure Local Command Request Codec

Status: approved under standing project authorization for build-phase implementation

## Purpose

Map the already locked Phase 008 read-only command namespace onto deterministic bytes without introducing a general-purpose serializer or activating the local socket runtime.

## Decision

A local read-only command request payload is exactly two bytes:

- unsigned 16-bit command code;
- big-endian/network byte order;
- no argument bytes for the two current commands.

Mappings:

- `GetAgentStatus` -> `00 01`
- `GetPrivateDnsConfig` -> `00 02`

## Why a fixed two-byte request payload

Both current commands are parameterless read-only introspection operations.

A general serializer would add dependency/supply-chain and schema complexity before it provides value. The fixed representation keeps the first command request surface deterministic and bounded while preserving room for later command-specific payload contracts.

This does not mean all future commands must fit in two bytes. A future protocol version or command schema may define structured request bodies explicitly.

## Fail-closed decoding

The decoder rejects:

- any payload whose length is not exactly two bytes;
- any numeric command code not present in the active typed command namespace.

It does not accept raw command strings or unknown pass-through identifiers.

## Single authoritative namespace

`LocalAgentCommand::code()` remains the stable typed-to-numeric mapping.

Phase 015 adds `LocalAgentCommand::from_code()` as the inverse mapping used by the codec.

The codec does not maintain a separate independent command table.

## Authorization remains separate

Decoding a known command only establishes structural validity.

It does not prove:

- authenticated peer state;
- capability authorization;
- command execution eligibility;
- current Agent lifecycle compatibility.

Those remain later dispatcher/connection-state responsibilities.

## Response body remains deferred

The Phase 008 response status taxonomy is typed but not yet serialized here.

Agent status and private-DNS response schemas remain separate future work so response data can be bounded and reviewed independently.

## Runtime remains inactive

Phase 015 performs only pure in-memory byte conversion.

It does not:

- read or write a socket;
- create a connection loop;
- dispatch a command;
- inspect host state;
- modify any state.

## Dependency boundary

Only Rust standard-library operations are used.

No serializer, networking, async-runtime, cryptographic, or persistence dependency is added.

## Explicit deferrals

Phase 015 does not implement:

- response payload codec;
- Agent status response schema;
- private-DNS response schema;
- frame-kind + command-payload combined decoder;
- runtime dispatch;
- Unix socket runtime;
- timeout/cancellation policy;
- privileged-helper protocol;
- systemd activation;
- crypto-provider selection;
- remote networking.

## Validation requirements

Phase 015 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- payload length remains exactly two bytes;
- command bytes use big-endian stable codes;
- both current commands round-trip;
- short/trailing payloads are rejected;
- unknown command codes are rejected.
