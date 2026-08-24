# Phase 152 C03e-U — Remote Session Executor Owner Source Materialization Staging

Status: STAGED

Target gate:

`C03E_U_REMOTE_SESSION_EXECUTOR_OWNER_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-T:

- branch: `phase-152-c03e-t-remote-session-executor-custody-selection-staging`
- head: `3b05b172f3c6a46e0d5c8ce93e657cdc60c985a7`
- tree: `d0946d6de132cc3b94fc12cc9fe2a1be8ed1cd48`
- gate: `C03E_T_REMOTE_SESSION_EXECUTOR_CUSTODY_SELECTED`

C03e-U preserves exact T lineage.

## Purpose

Materialize only the executor-custody source selected by C03e-T:

1. add the exact direct Tokio dependency required by the Agent-owned executor owner;
2. materialize one non-cloneable current-thread runtime owner;
3. materialize one bounded construction error;
4. expose those types only through the existing `remote_session_capability_runtime` module;
5. record the direct dependency edge in `Cargo.lock` without package/version/checksum drift;
6. stop before any future-driving, task-spawn, network, authority-sharing or production-activation surface.

## Exact dependency surface

`crates/prw-agent/Cargo.toml` adds exactly:

`tokio = { version = "=1.53.1", default-features = false, features = ["rt", "net", "time", "sync"] }`

This matches the exact Tokio version already selected by `prw-remote-transport`.

C03e-U does not add direct Agent:

- `macros`;
- `rt-multi-thread`;
- a second executor family;
- a second Tokio version.

Canonical validation proved that the direct Agent dependency must also be represented in the workspace lockfile package metadata. Therefore `Cargo.lock` changes only by adding one existing-package dependency edge under the `prw-agent` package:

`"tokio",`

No Tokio package entry, version, checksum or transitive package selection changes. Tokio remains exactly `1.53.1`.

## Source placement

Executor source is placed under the existing remote-session capability runtime module:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

The existing parent module:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

adds a private child-module declaration and re-exports only:

- `RemoteSessionExecutorRuntime`;
- `RemoteSessionExecutorRuntimeCreateError`.

No new root `lib.rs` module is required.

## Materialized runtime owner

`RemoteSessionExecutorRuntime` owns exactly one private `tokio::runtime::Runtime` by value.

Construction is:

- Tokio `Builder::new_current_thread()`;
- I/O/time drivers enabled through the selected builder surface;
- `build()` result mapped into the bounded Agent-owned construction error.

The owner is intentionally not `Clone`.

The raw Tokio runtime field remains private and has no accessor.

## Materialized error

`RemoteSessionExecutorRuntimeCreateError` contains exactly the bounded initial construction class:

- `Construction`.

The error implements `Display` and `std::error::Error`.

Lower Tokio/OS construction diagnostics are not sent to the remote peer and are not exposed as a remote protocol payload.

The constructor:

`RemoteSessionExecutorRuntime::new() -> Result<Self, RemoteSessionExecutorRuntimeCreateError>`

performs no retry and does not terminate the process.

## Deliberately absent executor operations

C03e-U does not expose:

- raw `Runtime` access;
- `Handle` access or cloning;
- generic `block_on`;
- a C03e-S-specific drive method;
- `tokio::spawn`;
- `spawn_local`;
- `spawn_blocking`;
- `JoinHandle`;
- `JoinSet`;
- cancellation token/channel construction;
- worker registry/collection;
- runtime thread-pool configuration.

The private runtime field may carry a narrow dead-code annotation because executor custody is staged before the separately gated borrowed drive seam.

## No remote transport activation

Constructing the executor owner does not:

- construct `RemoteServerTransportRuntime`;
- bind UDP/QUIC;
- accept a peer;
- validate a `TransportIdentity`;
- authenticate a logical session;
- construct a `BoundRemoteSession`;
- run C03e-S worker code;
- publish readiness.

## No current-authority sharing change

C03e-U does not modify:

- `WorkspaceDeviceRegistry`;
- `CapabilityBridge`;
- policy evaluator types;
- dispatcher types;
- registry/policy ownership or synchronization.

The C03e-T restriction remains exact: no spawned `'static` per-session task may be introduced until current-authority sharing is separately selected and materialized.

## Identity and authorization invariants

No executor/runtime value becomes PRW identity.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains transient endpoint;
- PID/UID/GID/thread/task/runtime IDs are not identity.

Current registry/policy/lease/verifier-time authority remains unchanged.

## Required source tests

C03e-U tests may prove only the materialized construction surface:

- constructor has the exact fallible signature;
- selected current-thread runtime can construct and drop in the test environment without running remote work.

Tests must not:

- bind a real remote listener as evidence of U;
- spawn remote tasks;
- claim readiness;
- claim production activation.

## Canonical corrective history

The initial source candidate exposed two validation defects, both corrected without widening runtime semantics:

1. Rust validation #1005 passed the locked graph and failed only canonical rustfmt in the executor-owner test; corrective commit `c95bb6ef80aeac377814d95b4cc869786b9353ac` applied only the canonical line wrapping.
2. Rust validation #1006 then passed locked graph and rustfmt, but `cargo clippy --locked` refused to proceed because the new direct `prw-agent -> tokio` dependency was not yet represented in `Cargo.lock`; tests/build were therefore skipped and are not PASS evidence.

The lockfile defect was corrected by a branch-scoped, self-removing one-shot workflow staged at `fe0d712653b8a4d15f1eeafe3e518a4bada8481e`. Its resulting commit `4c6152f587b36f33920645bbb28da4d3ccd312e8`:

- adds exactly one line, `"tokio",`, to the `prw-agent` dependency list in `Cargo.lock`;
- proves the corrected graph through `cargo metadata --locked --format-version 1` before committing;
- removes the temporary corrective workflow in the same resulting commit;
- leaves no corrective workflow path in the final tree;
- introduces no package/version/checksum/transitive dependency drift.

The push produced with the workflow token is not used as canonical completion evidence. Final completion requires fresh exact-head canonical PR validation after this contract update.

## Expected final diff boundary

The intended final T -> U diff is limited to:

1. this U contract;
2. `Cargo.lock`, with only the direct `prw-agent -> tokio` dependency edge;
3. `crates/prw-agent/Cargo.toml` direct Tokio dependency;
4. `crates/prw-agent/src/remote_session_capability_runtime.rs` child-module declaration/re-export;
5. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` source.

`apps/android`, `main.rs`, root `lib.rs`, bridge/transport source, permanent workflow, readiness, packaging/systemd and host-network paths must remain unchanged.

## Validation requirements

Closure requires on the final exact head:

- exact T merge base;
- final scope review;
- locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS;
- workspace tests PASS;
- workspace build PASS;
- canonical Android native/application validation if triggered by source/dependency change;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback verification;
- append-only rolling Drive update with exact post-T prefix preservation;
- PR remains draft/open/unmerged.

## Explicit non-goals

C03e-U does not:

- drive a future;
- spawn a worker;
- add concurrent session admission;
- select shared-current-authority synchronization;
- modify session authentication/binding/capability semantics;
- modify transport identity semantics;
- bind/listen/accept remote transport;
- wire `main.rs`;
- publish readiness;
- modify systemd/firewall/NAT/routing;
- deploy or merge.

## Completion meaning

Closure of C03e-U means only that the C03e-T-selected executor custody exists in source with the exact direct dependency, corrected lockfile dependency edge and bounded constructor surface.

The next allowed checkpoint is one borrowed single-worker drive seam using the private runtime without task spawn. Current-authority sharing, spawned task/join ownership, concurrent sessions, `main.rs`, readiness and runtime activation remain separately gated.

Target gate:

`C03E_U_REMOTE_SESSION_EXECUTOR_OWNER_SOURCE_MATERIALIZED`
