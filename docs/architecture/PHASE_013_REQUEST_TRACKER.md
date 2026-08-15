# Phase 013 Bounded Outstanding Request State

Status: approved under standing project authorization for build-phase implementation

## Purpose

Lock a bounded, deterministic correlation-state model before a real connection loop or concurrent local IPC runtime exists.

## Decision

Each future local IPC connection may have at most:

`64` simultaneously outstanding requests.

A pure in-memory `LocalRequestTracker` records typed non-zero request IDs that are waiting for one terminal Response or Error.

## State transitions

### Register

`register(request_id)` succeeds only when:

- the ID is not already outstanding;
- the tracker contains fewer than 64 IDs.

Failure classes:

- `DuplicateRequestId`;
- `TooManyOutstandingRequests`.

### Complete

`complete(request_id)` succeeds only when the ID is currently outstanding and removes it from the tracker.

Failure class:

- `UnknownRequestId`.

After completion, the same numeric request ID may be registered again.

## Why bound correlation state now

Phase 007 defined a request ID but did not define how many logical exchanges may coexist.

Before a future connection loop supports multiple frames, PRW needs an explicit resource bound so an authenticated local process cannot create unbounded in-memory request-tracking state.

A limit of 64 is intentionally modest for a local control channel while still allowing UI/CLI operations to overlap. It is not intended for bulk transfer concurrency.

Bulk file-transfer state remains a separate product capability and must not be represented by opening unlimited local control requests.

## Duplicate and unsolicited terminal responses

A duplicate outstanding request ID is rejected rather than replacing the earlier request.

A terminal Response/Error for an unknown request ID is rejected rather than being treated as an unsolicited event or silently ignored by the correlation layer.

Unsolicited event messages remain outside the current protocol baseline.

## Ephemeral scope

The tracker is per connection and in memory only.

It is not persisted and does not survive Agent restart or connection closure.

A future connection object will own lifecycle cleanup; Phase 013 does not implement that object yet.

## No scheduling semantics

The tracker does not decide:

- processing order;
- whether requests execute concurrently;
- thread/task ownership;
- fairness;
- priority;
- timeout duration;
- cancellation;
- retries;
- idempotency.

It only protects correlation uniqueness and a fixed state bound.

## Dependency boundary

The implementation uses only Rust standard-library collections/types and existing PRW typed request IDs.

No runtime, networking, async, serializer, cryptographic, or persistence dependency is introduced.

## Explicit deferrals

Phase 013 does not implement:

- actual local IPC connection loop;
- Unix socket runtime;
- SO_PEERCRED runtime enforcement;
- request scheduling/execution;
- timeouts/cancellation;
- command payload serialization;
- unsolicited events;
- privileged-helper protocol;
- systemd activation;
- crypto-provider selection;
- remote networking.

## Validation requirements

Phase 013 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- tracker starts empty;
- registration is observable;
- duplicate active IDs are rejected;
- the 64-request bound is enforced;
- completion removes an ID and permits later reuse;
- unknown completion is rejected.
