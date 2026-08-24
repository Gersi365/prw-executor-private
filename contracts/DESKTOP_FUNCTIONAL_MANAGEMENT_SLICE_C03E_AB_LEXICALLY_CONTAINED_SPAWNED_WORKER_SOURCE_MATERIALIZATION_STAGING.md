# Phase 152 C03e-AB — Lexically-Contained Spawned Worker Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AB_LEXICALLY_CONTAINED_SPAWNED_WORKER_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-AA:

- branch: `phase-152-c03e-aa-lexically-contained-spawned-worker-ownership-selection-staging`
- head: `e820cc3da50ac51167733df9f3f642ac98c5ac60`
- tree: `9acc4d415fa9810c50308aa372ef110752285900`
- gate: `C03E_AA_LEXICALLY_CONTAINED_SPAWNED_WORKER_OWNERSHIP_SELECTED`

C03e-AB preserves exact AA lineage and materializes only the task-custody shape selected there.

## Purpose

Materialize one bounded `RemoteSessionExecutorRuntime` seam that spawns exactly one authenticated remote-session worker, retains the one join handle lexically, awaits it exactly once before returning, and exposes only a bounded Agent-domain abnormal-join classification.

The checkpoint must not introduce persistent worker storage, a second worker, concurrent authenticated-session admission, a concrete cancellation controller, Agent `main.rs` wiring, readiness, runtime activation, deployment or merge.

## Materialized API shape

The existing borrowed seam:

`RemoteSessionExecutorRuntime::drive_capability_request_worker(...)`

remains source- and behavior-stable.

C03e-AB adds a second bounded operation equivalent to:

`RemoteSessionExecutorRuntime::drive_spawned_capability_request_worker(...)`

The method:

- takes `&mut self` for the entire synchronous drive;
- consumes one `AuthenticatedRemoteSessionRuntimeOwner` by value;
- receives `&SharedCurrentCapabilityAuthority<P>` and clones the authority exactly once before task creation;
- consumes dispatcher `D` by value;
- consumes verifier-time provider `T` by value;
- consumes caller-supplied cancellation future `C` by value;
- returns `Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

Expected generic bounds are exactly those selected by AA:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> u64 + Send + 'static`;
- `C: Future<Output = ()> + Send + 'static`.

No `Clone` bound may be added to `P`, the session owner, dispatcher, verifier-time provider or cancellation future.

## Shared-current authority capture

The method clones the existing `SharedCurrentCapabilityAuthority<P>` by value before entering the spawned future.

That clone must preserve C03e-X semantics:

- only the outer `Arc` is cloned;
- `WorkspaceDeviceRegistry` is not cloned;
- policy `P` is not cloned;
- no registry or policy snapshot is created;
- no cached authorization decision is moved into task state.

The spawned worker borrows the task-owned authority clone when invoking the existing C03e-S/Z worker body.

## Private runtime and spawn boundary

The existing Agent-owned Tokio current-thread runtime remains private and non-cloneable.

The new method uses one private `Runtime::block_on(...)` supervisor future. Inside that runtime context the supervisor creates exactly one Tokio task with one `async move` worker future.

The task owns:

- one mutable local `AuthenticatedRemoteSessionRuntimeOwner`;
- one task-owned shared-current authority clone;
- one mutable local dispatcher `D`;
- verifier-time provider `T`;
- cancellation future `C`.

The worker calls the existing:

`AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_worker(...)`

and does not reimplement request-loop, authorization, dispatch or cancellation behavior.

No runtime handle is returned, cloned into persistent state or exposed to callers.

## Join ownership and result

The supervisor retains exactly one local:

`tokio::task::JoinHandle<AuthenticatedRemoteSessionWorkerStop>`

and awaits it exactly once before the supervisor future returns.

Successful join returns the existing `AuthenticatedRemoteSessionWorkerStop` unchanged.

Abnormal Tokio join completion maps to exactly one Agent-domain error variant:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`

The public error must not retain or expose raw `tokio::task::JoinError`, panic payloads, backtraces, Tokio task IDs, runtime IDs or thread IDs.

No retry, replacement task, replacement session, re-authentication or fabricated worker stop follows abnormal task completion.

## Public error surface

`RemoteSessionSpawnedWorkerJoinError` is a public Agent-domain type and must be re-exported alongside the existing executor runtime types from `remote_session_capability_runtime`.

It is expected to be:

- `Debug`;
- `Clone`;
- `Copy`;
- `PartialEq`;
- `Eq`;
- `Display`;
- `std::error::Error`.

Its only selected variant in this checkpoint is `AbnormalTaskCompletion`.

## Existing worker semantics retained

The task-owned worker delegates to the existing C03e-S/Z path unchanged.

Therefore:

- fresh shared-current registry/policy authorization still occurs per protected request;
- no authority guard is retained across dispatcher execution or response I/O;
- the C03e-Q request loop is still polled before cancellation on each wake;
- a real request-loop failure still wins a same-poll tie;
- request-loop failure retains code `3` / `remote capability session terminated` and the original typed transaction failure;
- cancellation still wins only while the request loop remains pending;
- cancellation still drops the request-loop future first and then closes the retained peer once with code `4` / `remote capability session shutdown`;
- successful worker completion remains `AuthenticatedRemoteSessionWorkerStop::Cancelled` or `Failed(...)` before the join layer.

C03e-AB adds only task custody and abnormal-join classification around that existing worker body.

## Compile-time ownership proof

The materialized source must compile under Tokio's spawned-future requirements without weakening ownership.

The `tokio::spawn(...)` call itself must prove the captured future is `Send + 'static`.

A focused compile-time test may additionally assert that `AuthenticatedRemoteSessionRuntimeOwner: Send + 'static`.

No `Clone` implementation is added to the session owner or dispatcher.

## No hard abort

C03e-AB must not call or expose:

- `JoinHandle::abort()`;
- `AbortHandle::abort()`;
- `JoinSet::shutdown()`;
- equivalent hard-cancellation primitives.

The caller-supplied cancellation future remains the only orderly cancellation signal selected for this worker.

## No detached or persistent task

The local join handle must not be:

- returned;
- stored in `RemoteSessionExecutorRuntime`;
- stored in any other persistent owner;
- inserted into a collection;
- converted to an abort handle;
- forgotten or detached;
- keyed by DeviceId, TransportIdentity or a local worker ID.

The bounded drive returns only after the one local handle resolves.

## No concurrent admission

One internal task does not constitute concurrent authenticated-session support.

The `&mut self` executor borrow spans the whole private-runtime drive and the method waits for the spawned worker before returning.

C03e-AB therefore does not select:

- a second active worker through the same executor owner;
- a remote accept loop while the worker is active;
- persistent worker capacity;
- fairness/backpressure policy;
- duplicate logical-device admission policy;
- completion polling across multiple workers.

## Dependency boundary

C03e-AB adds no dependency and no Tokio feature.

Existing Agent Tokio features already include `rt`, which is sufficient for the selected current-thread spawn path.

These files must remain byte-stable:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

No `rt-multi-thread` or `macros` feature may be added.

## Identity invariants

Task ownership creates no PRW identity.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- task/runtime/thread/PID/UID/GID/lock identifiers are not logical identity.

No task ID or join error metadata may enter protocol or authorization state.

## Exact source scope

The intended final AA -> AB net diff contains exactly three paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime.rs`.

No workflow, manifest, lockfile, bridge, transport, Android, Agent binary or host file is part of the final AB scope.

## Validation requirements

Closure requires on the final exact AB head:

- exact AA merge base;
- exact three-path final net-scope review;
- dependency/manifest byte stability;
- permanent Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy with warnings denied, workspace tests and workspace build;
- canonical Android native/application validation FULL PASS because Rust source changes are present;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AA prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Explicit non-selection

C03e-AB does not select or perform:

- persistent worker collections;
- `JoinSet`;
- detached task handles;
- task abort;
- concrete cancellation-controller/channel construction;
- shutdown fan-out or multi-worker drain;
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

Closure means only that the AA-selected single worker can be moved into exactly one lexically-contained Tokio task under the existing private current-thread runtime, with one local handle awaited before return and abnormal join completion mapped to one bounded Agent-domain error.

It does not mean workers persist after the bounded drive, multiple sessions run concurrently, a cancellation controller exists, Agent `main.rs` is wired, remote transport is activated or readiness may be published.

The next checkpoint must explicitly select the next lifecycle boundary after this single spawned-and-joined worker is proven; it must not infer a worker collection or concurrent admission from AB alone.

Target gate:

`C03E_AB_LEXICALLY_CONTAINED_SPAWNED_WORKER_SOURCE_MATERIALIZED`
