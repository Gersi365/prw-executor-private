# Phase 008 Bounded Local Agent Commands

Status: approved under standing project authorization for build-phase implementation

## Purpose

Introduce the first typed local command namespace only after the local transport, peer-authentication, frame boundary, version, and request-correlation rules are locked.

The phase remains provider-neutral and runtime-inactive.

## Decision

Phase 008 admits exactly two read-only local Agent commands:

1. `GetAgentStatus` — code 1
2. `GetPrivateDnsConfig` — code 2

It also locks terminal response-status identifiers:

- `Ok` — 0
- `InvalidRequest` — 1
- `Unauthorized` — 2
- `UnsupportedCommand` — 3
- `Conflict` — 4
- `InternalError` — 5

A request envelope carries a Phase 007 non-zero request ID plus one command identifier.

A response envelope carries the same request ID plus one terminal response status.

## Why read-only first

The first command surface intentionally avoids host mutation.

This provides a narrow way to validate future protocol serialization and runtime dispatch before introducing operations that can:

- change files;
- alter private DNS;
- change policy;
- control services;
- open terminal sessions;
- affect network state;
- invoke privileged helpers.

A successful same-user local connection is not treated as blanket authority for future mutating operations.

## Authorization direction

Phase 006 authenticates the local peer using kernel-reported credentials and a same-UID baseline.

Phase 008 keeps the Agent as an independent authorization enforcement point. Future command additions must identify their required capabilities explicitly.

This preserves the existing product security direction that authorization is capability-oriented rather than represented by one administrator flag.

## Command namespace discipline

Command identifiers are stable typed values rather than arbitrary command-name strings.

The baseline contains no generic variants such as:

- `RunShell`;
- `RunCommand`;
- `Execute`;
- `Eval`;
- `RawRequest`.

A future terminal capability may exist, but it must be designed as a separately bounded session protocol and authorization surface rather than smuggled into the local control protocol as unrestricted execution.

## Response discipline

Response status is deliberately small and bounded.

`InternalError` is generic so a future wire implementation does not need to expose implementation-sensitive diagnostics to the requester.

Detailed protected diagnostics and user-facing structured error detail can be designed separately.

## Serialization remains deferred

Phase 008 defines domain-level command/status identifiers and envelopes only.

The bytes inside the Phase 007 opaque payload are still undefined. This prevents a serialization dependency from being introduced before the command/authorization boundary is clear.

The next phase may evaluate a deterministic bounded serialization strategy for these envelopes and their read-only response schemas.

## Runtime remains inactive

Phase 008 does not:

- bind/listen/connect a socket;
- parse frame bytes;
- dispatch a command;
- read actual Agent status;
- read actual DNS state;
- modify any state;
- add a serializer dependency.

## Validation requirements

Phase 008 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- command identifiers are stable;
- response-status identifiers are stable;
- request envelopes preserve request ID and command;
- response envelopes preserve request correlation and status.
