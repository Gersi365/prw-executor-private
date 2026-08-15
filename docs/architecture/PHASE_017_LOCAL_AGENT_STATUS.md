# Phase 017 Minimal Local Agent Status Snapshot

Status: approved under standing project authorization for build-phase implementation

## Purpose

Define the smallest useful successful response body for `GetAgentStatus` before introducing a byte codec or live runtime collection.

## Decision

The typed `LocalAgentStatusSnapshot` contains exactly:

- `LocalAgentRuntimeState`;
- `LocalIpcProtocolVersion`.

The snapshot constructor always records the compiled current local IPC protocol version.

## Runtime-state taxonomy

Stable identifiers:

- 1 = Starting
- 2 = Ready
- 3 = Degraded
- 4 = Stopping

Only `Ready` reports normal readiness.

## Why the schema is intentionally small

A status endpoint can easily become an accidental metadata-disclosure surface. The first schema therefore excludes fields that are not necessary for the initial local UI/CLI readiness decision.

Excluded from Phase 017:

- hostname and username;
- PID/process details;
- runtime/filesystem paths;
- environment values;
- network addresses;
- device/public identity material;
- source/commit/build identifiers;
- timestamps;
- arbitrary health/error strings.

Additional fields must have a concrete consumer and a bounded disclosure rationale before they are added.

## Separate private-DNS command

Private DNS remains optional and already has its own `GetPrivateDnsConfig` command.

Status therefore does not duplicate DNS configuration or make Agent readiness semantics depend on DNS state.

## Degraded state

`Degraded` is included so a future Agent can remain responsive while signaling that a non-fatal capability is impaired.

Phase 017 deliberately does not define degradation reason strings or subcodes. That prevents unbounded diagnostics from being smuggled into the minimal status body.

## Current protocol version by construction

`LocalAgentStatusSnapshot::current(state)` records `LocalIpcProtocolVersion::current()` internally.

There is no public constructor accepting an arbitrary protocol version in Phase 017. This keeps locally generated snapshots consistent with the compiled protocol contract.

## Runtime remains inactive

The types do not inspect the process or operating system.

No actual status command is dispatched, no socket is bound, and no service is started.

## Dependency boundary

No dependency is added. The snapshot reuses the existing local IPC protocol version type.

## Explicit deferrals

Phase 017 does not implement:

- status-body byte codec;
- live runtime-state source/state machine;
- degradation reason taxonomy;
- private-DNS response schema;
- command dispatcher;
- actual Unix socket runtime;
- SO_PEERCRED runtime enforcement;
- systemd activation;
- crypto-provider selection;
- remote networking.

## Validation requirements

Phase 017 changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove:

- runtime-state identifiers are stable and invertible;
- only Ready reports readiness;
- the snapshot uses the current local IPC protocol version.
