# Phase 152 C03e-AD — Single-Worker Cancellation Pair Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AD_SINGLE_WORKER_CANCELLATION_PAIR_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-AC:

- branch: `phase-152-c03e-ac-single-worker-cancellation-pair-selection-staging`
- head: `2eb91422df424fcf00ac77632307a45fb0387b29`
- tree: `bdbf9a14bbf19a60e46c97075f1211672fd09f3c`
- gate: `C03E_AC_SINGLE_WORKER_CANCELLATION_PAIR_SELECTED`

C03e-AD preserves exact AC lineage and materializes only the single-worker cancellation pair selected there.

## Purpose

Materialize one Agent-owned cancellation controller/signal pair that can produce exactly one `Future<Output = ()> + Send + 'static` for the already-existing C03e-AB spawned-worker drive.

This checkpoint must not add persistent worker storage, multi-worker shutdown, controller cloning, cancellation fan-out, process-signal wiring, concurrent authenticated-session admission, Agent `main.rs`, readiness, runtime activation, deployment or merge.

## Materialized source seam

C03e-AD adds one module:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_worker_cancellation.rs`

The module exposes:

- `RemoteSessionWorkerCancellationController`;
- `RemoteSessionWorkerCancellationSignal`;
- one infallible pair constructor equivalent in responsibility to `remote_session_worker_cancellation_pair()`.

The parent `remote_session_capability_runtime` module re-exports only these bounded Agent-domain symbols.

No generic `Arc`, `Notify`, atomic state, task handle, runtime handle or transport close handle is exposed.

## Private state

The pair shares one private state containing:

- `requested: AtomicBool`, initialized `false`;
- `wake: tokio::sync::Notify`;
- one private `Arc` so controller and signal can move independently.

The private `Arc` exists only for ownership separation between the two public halves. It is not a public clone surface and is not PRW identity.

## Pair construction

Pair construction is infallible and performs only ordinary in-process synchronization allocation.

The constructor creates exactly one private state, gives the controller one `Arc` reference and gives the signal the other reference.

Neither public half implements or derives `Clone` in this checkpoint.

No worker, task, transport, session, listener or runtime operation occurs during pair construction.

## Controller behavior

`RemoteSessionWorkerCancellationController::request_cancellation(&self)` performs the exact AC-selected transition:

1. store `true` into the monotonic flag with `Ordering::Release`;
2. call the single-waiter Tokio wake operation equivalent to `Notify::notify_one()`;
3. return without waiting for worker completion.

The operation is infallible and idempotent.

Repeated requests do not create new lifecycle states, task handles, identities or errors.

Dropping the controller without calling `request_cancellation()` is not cancellation.

## Signal future

`RemoteSessionWorkerCancellationSignal::into_cancelled(self)` consumes the one signal and returns an opaque future satisfying:

`Future<Output = ()> + Send + 'static`

The future loops only until the monotonic flag is observed true using `Ordering::Acquire`.

While the flag remains false, it awaits the private `Notify` wake.

Because Tokio `Notify::notify_one()` retains a permit when no waiter is currently registered, the selected flag-check / notified-await loop must not lose a cancellation request that races between the flag check and waiter registration.

If cancellation was requested before the future is first polled, the future completes from the durable flag without depending on a new wake.

If cancellation is requested while the future is pending, the registered Tokio waiter is woken and then observes the durable flag.

## No channel-close semantics

Controller destruction does not alter the monotonic flag.

Therefore controller drop alone cannot complete the signal future.

Signal destruction also does not request cancellation.

C03e-AD must not introduce oneshot sender-drop, watch sender-drop or equivalent channel-close-as-cancellation behavior.

## No public clone or fan-out

C03e-AD does not implement `Clone` for either public half.

There is exactly one public controller and one public signal for the pair.

The module does not expose a method that creates a second signal or clones the controller.

Broadcast, watch fan-out, cancellation tokens and multi-worker shutdown remain separately gated.

## Integration with C03e-AB

The existing C03e-AB executor source remains unchanged.

The materialized signal future is type-compatible with the existing generic cancellation parameter of:

`RemoteSessionExecutorRuntime::drive_spawned_capability_request_worker(...)`

No executor overload or new generic bound is needed.

A focused compile-time test must prove the returned cancellation future is `Future<Output = ()> + Send + 'static`.

This checkpoint does not construct an authenticated session or invoke the executor seam merely to prove type compatibility.

## Existing worker semantics remain authoritative

The pair supplies readiness only.

It does not close transport, classify worker completion or override C03e-S semantics.

After the future resolves, the existing worker remains authoritative for:

- cancellation winning only while the request loop is pending;
- dropping the request-loop future first;
- closing the retained peer exactly once with code `4` / `remote capability session shutdown`;
- returning `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

## Focused tests

The new source module must include focused tests that prove at least:

1. cancellation requested before future polling completes promptly;
2. a pending future is woken by an explicit cancellation request and then completes;
3. repeated cancellation requests remain idempotent;
4. dropping the controller without an explicit request leaves the signal future pending;
5. the produced future satisfies `Future<Output = ()> + Send + 'static`.

Tests may manually poll the future with a safe custom `Waker`; no Tokio macros or new test dependency is required.

## Lost-wake boundary

The source must not implement a check-then-sleep pattern whose wake can be permanently lost.

For the selected single waiter, the allowed pattern is the durable monotonic flag combined with Tokio `Notify` permit semantics.

The flag is the lifecycle state. The notification is only a wake mechanism.

No semantic event count is exposed.

## Memory ordering

Cancellation request uses `Ordering::Release`.

Cancellation observation uses `Ordering::Acquire`.

The atomic carries only the boolean cancellation state. It carries no registry, policy, session, identity, authorization or transport payload.

## Dependency boundary

C03e-AD adds no crate and no Cargo feature.

Existing Agent Tokio features already contain `rt` and `sync`.

These files must remain byte-stable:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

No `rt-multi-thread`, `macros`, cancellation-token crate or synchronization dependency may be added.

## Identity invariants

Cancellation state creates no PRW identity.

- DeviceId / authenticated PRW session identity remain logical identity;
- TransportIdentity remains lower-transport identity;
- IP remains a transient endpoint;
- `Arc`, `Notify`, atomic, waker, task, runtime, thread, PID/UID/GID and lock identities are not logical identity.

No implementation identity may be returned, logged as protocol identity or selected as a worker key.

## Exact source scope

The intended final AC -> AD net diff contains exactly three paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_worker_cancellation.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime.rs`.

No executor source, manifest, lockfile, workflow, Android application, bridge, transport, Agent binary or host file belongs to the final AD scope.

## Validation requirements

Closure requires on the final exact AD head:

- exact AC merge base;
- exact three-path final net-scope review;
- manifest/lockfile byte stability through their absence from the final diff;
- permanent Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- canonical Android native/application validation FULL PASS because Rust source changes are present;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AC prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Explicit non-selection

C03e-AD does not select or perform:

- controller or signal `Clone`;
- multiple signals;
- broadcast/watch cancellation fan-out;
- cancellation-token dependencies;
- persistent worker collections;
- persistent join-handle custody;
- multi-worker shutdown or drain;
- hard task abort;
- process-signal wiring;
- concurrent authenticated-session admission;
- duplicate DeviceId policy;
- capacity/fairness scheduling;
- Agent `main.rs` wiring;
- remote listener/bind activation;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge.

## Completion meaning

Closure means only that the C03e-AC single-worker cancellation pair exists in Rust source, explicit cancellation is durable and wakeable without controller-drop semantics, and the signal can supply the existing C03e-AB cancellation future type.

It does not mean the controller is wired to process shutdown, workers are persistent, multiple authenticated sessions run concurrently, a worker collection exists, Agent `main.rs` is wired, remote transport is activated or readiness may be published.

The next checkpoint must explicitly select the next lifecycle ownership boundary after this pair is proven; it must not infer worker collection or multi-worker shutdown from C03e-AD alone.

Target gate:

`C03E_AD_SINGLE_WORKER_CANCELLATION_PAIR_SOURCE_MATERIALIZED`
