# Phase 152 C03e-S — Cancellation-Aware Remote Session Worker Seam Source Materialization Staging

Status: STAGED

Target gate:

`C03E_S_CANCELLATION_AWARE_REMOTE_SESSION_WORKER_SEAM_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-R:

- branch: `phase-152-c03e-r-remote-session-worker-lifecycle-ownership-selection-staging`
- head: `00b54f9f411b6fe4e25f64a96cea803f78b32761`
- tree: `59ebd52447097fd63829e3fc283d784a4e30629e`
- gate: `C03E_R_REMOTE_SESSION_WORKER_LIFECYCLE_OWNERSHIP_SELECTED`

C03e-S must preserve exact R lineage and materialize only the cancellation-aware single-worker execution seam selected by R.

## Purpose

Materialize a worker-body operation on the existing `AuthenticatedRemoteSessionRuntimeOwner` that can race:

1. the existing C03e-Q serial capability request loop; and
2. one caller-owned external cancellation future.

C03e-S does not spawn a task, create a cancellation channel, construct an async runtime, retain a join handle, collect workers, modify `main.rs`, publish readiness or activate a listener.

## Executor-neutral cancellation boundary

The new source accepts a generic caller-owned cancellation future equivalent to:

`C: Future<Output = ()> + Send`

The cancellation future carries no request, capability, identity, policy, path, dispatcher or transport diagnostic payload.

C03e-S uses only `std` future primitives for arbitration:

- `std::future::Future`;
- `std::future::poll_fn`;
- `Box::pin`;
- `std::task::Poll`.

No direct Tokio dependency is added to `prw-agent`.

No concrete cancellation sender/receiver type is selected or created by this checkpoint.

## Materialized worker operation

The existing `AuthenticatedRemoteSessionRuntimeOwner` gains a borrowed async method equivalent to:

`run_capability_request_worker(...)`

The method receives:

- `&mut self`;
- current `&CapabilityBridge`;
- caller-owned `FnMut() -> u64 + Send` verifier-time provider;
- caller-owned mutable `CapabilityDispatcher + Send`;
- caller-owned cancellation `Future<Output = ()> + Send`.

The method returns one source-level worker stop classification and does not itself create a task.

## Worker stop classification

C03e-S materializes:

- `AuthenticatedRemoteSessionWorkerStop::Cancelled`;
- `AuthenticatedRemoteSessionWorkerStop::Failed(AuthenticatedRemoteSessionCapabilityTransactionError)`.

Panic/task-join failure is intentionally not part of this source-level stop enum because no task/join owner exists yet. That classification remains at the future task-handle owner boundary selected by C03e-R.

## Exact arbitration ordering

Every poll of the worker race checks the existing C03e-Q request-loop future first.

This creates deterministic Q-failure precedence when both Q and cancellation are ready on the same worker poll.

### Q terminal failure ready

If the existing Q loop returns `Err(error)`:

- Q has already performed its selected code-3 close;
- the worker race returns `Failed(error)`;
- the exact `AuthenticatedRemoteSessionCapabilityTransactionError` is preserved unchanged;
- cancellation is not allowed to reclassify that already-ready failure.

This preserves the C03e-R rule that a real transaction failure is not converted to cancellation merely because shutdown is also requested.

### Q pending, cancellation ready

If Q remains pending and cancellation returns ready:

1. the arbitration returns the internal cancellation outcome;
2. the pinned Q future leaves its inner scope and is dropped;
3. dropping Q releases the `&mut self` borrow held by the request loop;
4. only after that drop does the worker operation access the retained peer;
5. the peer is closed exactly once with the C03e-R code-4 diagnostic;
6. the method returns `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

Because cancellation itself does not close the transport, it cannot induce an `Accept`/`Wire` error inside Q before the Q future is dropped.

## Code-4 shutdown diagnostic

C03e-S materializes exactly:

- close code: `4`;
- reason: `b"remote capability session shutdown"`.

The diagnostic remains fixed, bounded and non-secret.

It contains no:

- `DeviceId`;
- `TransportIdentity`;
- IP address;
- request identifier;
- capability name;
- policy result;
- path;
- dispatcher diagnostic;
- certificate material;
- lower transport error.

Existing close-code meanings remain unchanged:

- `1`: logical-session authentication transaction failure;
- `2`: post-authentication binding failure;
- `3`: C03e-Q capability-session terminal transaction failure;
- `4`: externally requested worker shutdown.

## Current Q clean-return defense

The current C03e-Q loop has no reachable `Ok(())` exit path, but its Rust signature remains `Result<(), ...>`.

C03e-S does not fabricate a new clean lifecycle completion for that type-level possibility.

If a future Q implementation returns `Ok(())`, the arbitration marks that Q future as cleanly completed, stops polling it, and remains pending solely on the caller-owned cancellation future.

This preserves the C03e-R selected stop classes without introducing an unselected clean-success variant.

## Identity and dynamic authority preservation

C03e-S does not change logical or transport identity semantics.

- `DeviceId` / authenticated PRW session identity remains logical identity.
- `TransportIdentity` remains lower-transport certificate identity.
- IP remains a transient endpoint only.
- PID/UID/GID/task/runtime IDs remain non-authoritative for remote identity.

The worker seam delegates all request processing to the existing C03e-Q loop. Therefore every successful request continues to use:

- caller-owned fresh verifier time;
- retained session lease validation;
- current authenticated-session registry state;
- current logical-device to transport binding validation;
- current policy evaluation;
- current dispatcher execution.

No authorization result is cached by S.

## No task/runtime ownership

C03e-S does not materialize:

- Tokio dependency in `crates/prw-agent/Cargo.toml`;
- Cargo lockfile change;
- Tokio runtime construction;
- `tokio::spawn`;
- `JoinHandle`;
- cancellation channel/sender;
- worker registry;
- concurrent authenticated-session collection;
- session capacity/admission policy;
- listener accept loop;
- `main.rs` remote runtime wiring;
- readiness publication.

The new method is only a future-producing worker-body seam that a later task owner may execute.

## No retry, replacement or protocol widening

C03e-S introduces no:

- retry;
- reconnect;
- replacement stream;
- replacement logical session;
- replacement application lease;
- replacement transport identity;
- re-authentication;
- pending-session abort;
- authenticated-session deletion;
- negative response envelope;
- concurrent request processing.

## Exact source scope

Expected final net diff relative to closed C03e-R is exactly two paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
2. this contract.

No mutation is selected for:

- `crates/prw-agent/Cargo.toml`;
- `Cargo.lock`;
- Agent `lib.rs` or `main.rs`;
- parent `remote_session_capability_runtime.rs` export surface;
- bridge peer wrapper;
- lower remote transport;
- registry/policy implementation;
- workflows;
- Android application source;
- readiness;
- systemd/packaging;
- host-network/reachability activation.

## Validation requirements

Because C03e-S changes Rust source, canonical completion requires exact-final-head:

- PRW Rust Validation FULL PASS: locked graph, rustfmt, Clippy, workspace tests and workspace build;
- PRW Android Validation FULL PASS when triggered by the source change, including exact toolchains, native adapter and Android application;
- skipped C02f-AD/C02f-AE workflows must not be counted as PASS evidence.

Any corrective formatter/lint commit must remain within the same exact two-path source/contract scope and be recorded in the immutable audit.

## Drive closeout requirements

After exact-head validation:

1. publish immutable `C03E_S_CANCELLATION_AWARE_REMOTE_SESSION_WORKER_SEAM_SOURCE_MATERIALIZATION_AUDIT.md` in the existing evidence folder;
2. raw-readback verify exact byte size and SHA-256;
3. immediately re-fetch authoritative rolling `C02E_BRANCH_STATUS.md` and require exact closed-R baseline;
4. append S evidence only, preserving the entire R prefix byte-for-byte;
5. raw-readback verify final rolling size/hash and predecessor prefix;
6. update the S PR body to CLOSED checkpoint metadata while keeping it draft/open/unmerged.

## Deliberate stopping point

After C03e-S closes, the next boundary is selection of the concrete async task/runtime ownership required to spawn one S worker and retain its cancellation-controller/task-handle pair.

Still separately gated after S:

- direct Agent async runtime/task dependency and construction;
- task spawning;
- task/join-handle owner source;
- concurrent authenticated-session worker collection/admission;
- duplicate logical-session policy and capacity;
- listener accept loop;
- `main.rs` remote runtime wiring;
- remote readiness;
- listener/reachability activation;
- deployment/restart/merge.

Gate on successful canonical closeout:

`C03E_S_CANCELLATION_AWARE_REMOTE_SESSION_WORKER_SEAM_SOURCE_MATERIALIZED`
