# Phase 152 C03e-AF — Current-Thread Single-Worker Supervisor Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AF_CURRENT_THREAD_SINGLE_WORKER_SUPERVISOR_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-AE:

- branch: `phase-152-c03e-ae-current-thread-single-worker-supervisor-custody-selection-staging`
- head: `0b9e1857d00240e5aba248371f953ba46d362e5e`
- tree: `7416f2e7dba1a8f730db800d3b0f5b71190add8b`
- gate: `C03E_AE_CURRENT_THREAD_SINGLE_WORKER_SUPERVISOR_CUSTODY_SELECTED`

C03e-AF preserves exact AE lineage and materializes only the bounded supervisor custody selected there.

## Purpose

Materialize one `RemoteSessionExecutorRuntime` seam that keeps the existing private Tokio current-thread runtime actively driven while exactly one spawned authenticated-session worker is supervised by exactly one C03e-AD cancellation controller and one caller-supplied orderly supervisor-shutdown future.

This checkpoint remains single-worker. It must not introduce persistent worker storage, a second worker, concurrent authenticated-session admission, cancellation fan-out, process-signal wiring, Agent `main.rs`, readiness, runtime activation, deployment or merge.

## Materialized API

C03e-AF adds one inherent method equivalent in responsibility to:

`RemoteSessionExecutorRuntime::drive_supervised_capability_request_worker(...)`

The method:

- takes `&mut self` for the whole synchronous drive;
- consumes one `AuthenticatedRemoteSessionRuntimeOwner` by value;
- receives `&SharedCurrentCapabilityAuthority<P>` and clones only the existing outer authority `Arc` exactly once for the worker;
- consumes dispatcher `D` by value;
- consumes verifier-time provider `T` by value;
- consumes orderly supervisor-shutdown future `S` by value;
- constructs exactly one C03e-AD controller/signal pair;
- moves the signal future into exactly one spawned worker;
- retains the matching controller and one local `JoinHandle` inside one lexical `Runtime::block_on(...)` supervisor;
- returns `Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

Worker-captured bounds remain the C03e-AB bounds:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> u64 + Send + 'static`.

The supervisor-shutdown future needs only the bounded drive requirement:

- `S: Future<Output = ()> + Send`.

It is not spawned, persisted or returned, so C03e-AF does not add an unnecessary `'static` bound to `S`.

## Runtime-driving custody

The complete supervisor lifecycle occurs inside one private `Runtime::block_on(...)` call on the existing non-cloneable Tokio current-thread runtime.

The method must not create a worker task and then return from `block_on` while that task is live.

The raw Tokio runtime and runtime handle remain private. No `Handle` accessor, handle clone, background runtime thread, `rt-multi-thread` feature or second executor family is added.

## Worker construction

Inside the lexical supervisor:

1. create exactly one `remote_session_worker_cancellation_pair()`;
2. clone `SharedCurrentCapabilityAuthority<P>` exactly once;
3. spawn exactly one `async move` worker under the current runtime;
4. move the authenticated session owner, authority clone, dispatcher, verifier-time provider and `signal.into_cancelled()` into the task;
5. delegate directly to existing `AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_worker(...)`;
6. retain only the matching cancellation controller and the one local `JoinHandle` in the supervisor.

No worker request-loop, authorization, dispatch or transport-close behavior is reimplemented.

## Completion-first supervisor race

The supervisor must implement deterministic completion-first polling without adding Tokio macro features.

On every wake before supervisor shutdown has won:

1. poll the one worker `JoinHandle` first;
2. if the handle is ready, return its mapped existing result immediately and do not issue a cancellation request;
3. only if the worker handle remains pending, poll the supervisor-shutdown future;
4. if shutdown is ready, request cancellation once through the retained controller and leave the race phase;
5. after cancellation request, await only the same worker handle until it resolves.

A manual `std::future::poll_fn` / `Poll` implementation is allowed and preferred over adding Tokio `select!`/macro features.

The supervisor-shutdown future is not polled again after it wins.

## Existing result authority

Supervisor shutdown readiness does not fabricate `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

After cancellation is requested, existing C03e-S worker semantics remain authoritative. A real request-loop failure may still win before the cancellation signal is observed, and that original `Failed(...)` result must be returned unchanged.

Abnormal task completion remains mapped only to the existing:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`

No raw Tokio `JoinError`, panic payload, task ID, runtime ID or backtrace escapes.

## Existing worker cancellation semantics remain authoritative

When cancellation wins inside the worker, the existing C03e-S path still:

1. drops the pending C03e-Q request-loop future first;
2. releases its mutable borrow;
3. closes the retained peer exactly once with code `4` / `remote capability session shutdown`;
4. returns `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

A request-loop failure retains code `3` / `remote capability session terminated` and its original typed transaction error.

The supervisor itself never closes the peer.

## Shared-current authority remains authoritative

The worker continues through C03e-Z shared-current authorization:

- only the outer authority `Arc` is cloned;
- no `WorkspaceDeviceRegistry` clone;
- no policy clone;
- no registry/policy snapshot;
- fresh current authority on every protected request;
- authority read guard released before dispatcher execution and response I/O.

No authority guard is retained by the supervisor or across cancellation/join waits.

## Existing seams remain stable

C03e-AF must not remove or behavior-change:

- `drive_capability_request_worker(...)`;
- `drive_spawned_capability_request_worker(...)`;
- `RemoteSessionSpawnedWorkerJoinError`;
- C03e-AD cancellation pair public surface.

The new supervisor seam is additive and bounded.

## No persistent handle or collection

The one `JoinHandle` remains lexical to the method's private supervisor future.

It is not returned, stored in `RemoteSessionExecutorRuntime`, inserted into a collection, converted to an abort handle, forgotten or detached.

No `Vec`, map, slab, queue or `JoinSet` is introduced.

No collection key, duplicate DeviceId policy, capacity, fairness or admission backpressure is selected.

## No cancellation fan-out

Exactly one non-cloneable controller and one non-cloneable signal are created.

No second signal, controller clone, watch/broadcast channel, cancellation token or multi-worker fan-out exists.

Controller drop without explicit request remains non-cancellation.

## No hard abort

C03e-AF must not call or expose:

- `JoinHandle::abort()`;
- `AbortHandle::abort()`;
- `JoinSet::shutdown()`;
- runtime shutdown as worker cancellation.

Once orderly shutdown wins, the same worker is allowed to terminate through its existing cancellation path and the supervisor continues to drive it until join completion.

## Focused tests

Source materialization must include focused tests for the private supervisor primitive proving at least:

1. a pending worker is cancelled when supervisor shutdown becomes ready and the same handle is joined;
2. a worker that completes before a permanently pending supervisor-shutdown future returns its result without cancellation;
3. when worker completion is already ready and shutdown is also ready, completion-first ordering avoids issuing a cancellation request;
4. abnormal task completion maps to `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

Tests may use the existing current-thread Tokio runtime and ordinary `std::future` utilities. No Tokio macros or new dependency is required.

## Dependency boundary

C03e-AF adds no crate, version or feature.

These files must remain byte-stable:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

Tokio `macros` and `rt-multi-thread` remain absent from the direct Agent requirement.

## Identity invariants

Supervisor, cancellation and task state create no PRW identity.

- DeviceId / authenticated PRW session identity remain logical identity;
- TransportIdentity remains lower-transport identity;
- IP remains transient endpoint;
- controller/signal/Arc/task/join/runtime/thread/PID/UID/GID/lock identities are not logical identity.

No implementation identity may become a protocol field, authorization identity or collection key.

## Exact source scope

The intended final AE -> AF net diff contains exactly two paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`.

No parent-module change is required because the new seam is an inherent method on an already-exported type and no new public type is added.

No workflow, manifest, lockfile, bridge, transport, Android, Agent binary, packaging or host path belongs to final AF scope.

## Validation requirements

Closure requires on the final exact AF head:

- exact AE merge base;
- exact two-path final net scope;
- manifest/lockfile byte stability through absence from the diff;
- permanent PRW Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- canonical Android native/application validation FULL PASS because Rust source changes are present;
- skipped workflows recorded only as skipped;
- immutable Drive audit with raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AE prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Explicit non-selection

C03e-AF does not select or perform:

- persistent worker collection;
- persistent task-handle custody after method return;
- multiple active remote workers;
- concurrent authenticated-session admission;
- collection key or duplicate-session policy;
- cancellation fan-out;
- multi-worker shutdown/drain;
- hard abort;
- process-signal wiring;
- Agent `main.rs` wiring;
- remote listener/bind activation;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge.

## Completion meaning

Closure means only that the AE-selected single-worker supervisor exists in Rust source and deterministically keeps the private current-thread runtime active while pairing one worker completion handle with one orderly cancellation controller until terminal completion.

It does not mean workers are persistent, multiple authenticated sessions can run concurrently, process shutdown is wired, a collection exists, Agent `main.rs` is wired, transport is activated or readiness may be published.

The next checkpoint must explicitly select the first persistent worker-collection/admission boundary, including current-thread runtime-driving implications, before any multi-worker source is introduced.

Target gate:

`C03E_AF_CURRENT_THREAD_SINGLE_WORKER_SUPERVISOR_SOURCE_MATERIALIZED`
