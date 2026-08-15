# Phase 016 Pure Local Response Status Prefix

Status: approved under standing project authorization for build-phase implementation

## Purpose

Map the Phase 008 terminal response-status taxonomy onto deterministic bytes while leaving command-specific success/error bodies separately reviewable.

## Decision

Every terminal local Agent response payload begins with a two-byte big-endian unsigned status code.

Mappings:

- `Ok` -> `00 00`
- `InvalidRequest` -> `00 01`
- `Unauthorized` -> `00 02`
- `UnsupportedCommand` -> `00 03`
- `Conflict` -> `00 04`
- `InternalError` -> `00 05`

Bytes after this prefix remain opaque body bytes.

## Why a prefix instead of a full serializer

The current protocol needs a stable terminal outcome before command-specific body formats are selected.

Separating the status prefix permits:

- one bounded response taxonomy across commands;
- independent review of Agent status and private-DNS body schemas;
- no general serialization dependency;
- body bytes to remain untouched by the status codec.

## Fail-closed decoding

The decoder rejects fewer than two bytes as `MissingStatus` and unknown numeric codes as `UnknownStatus`.

It never treats an unknown status as a generic internal error and never guesses a byte order.

## Typed inverse mapping

`LocalAgentResponseStatus::from_code()` is added as the inverse of the existing `code()` mapping.

`is_success()` explicitly defines only `Ok` as success.

The codec therefore reuses one authoritative status namespace instead of duplicating a mapping table.

## Frame-kind relationship remains explicit future work

The Phase 007 header distinguishes `Response` and `Error` frames, but Phase 016 does not yet enforce which status may appear under each kind.

A later combined protocol-envelope phase must define that invariant rather than relying on convention.

## Response bodies remain deferred

The next safe work can define bounded typed schemas for:

- Agent status;
- private-DNS configuration.

Those schemas should be reviewed before byte codecs are introduced, especially because the existing domain `PrivateDnsConfig` contains variable-length string/vector fields.

## Runtime remains inactive

Phase 016 is pure in-memory byte conversion only.

It does not create or use a Unix socket, dispatch a command, inspect live host state, or mutate any state.

## Dependency boundary

Only Rust standard-library byte conversion and slice operations are used.

No serializer, async runtime, socket wrapper, cryptographic provider, or persistence dependency is added.

## Validation requirements

Phase 016 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- status prefix remains exactly two bytes;
- status codes use stable big-endian representation;
- all current statuses round-trip;
- body bytes after the prefix are preserved unchanged;
- missing prefix is rejected;
- unknown status is rejected.
