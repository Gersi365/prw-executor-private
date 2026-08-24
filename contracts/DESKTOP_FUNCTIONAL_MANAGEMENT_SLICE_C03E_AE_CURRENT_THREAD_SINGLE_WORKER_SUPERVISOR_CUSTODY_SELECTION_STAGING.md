# Phase 152 C03e-AE — Current-Thread Single-Worker Supervisor Custody Selection Staging

Status: STAGED

Target gate:

`C03E_AE_CURRENT_THREAD_SINGLE_WORKER_SUPERVISOR_CUSTODY_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-AD:

- branch: `phase-152-c03e-ad-single-worker-cancellation-pair-source-materialization-staging`
- head: `3a01b0a0731e5669c2d73623811e5a02423004fa`
- tree: `9b9ee62a5e5323af6592d3902e58624fc366e064`
- gate: `C03E_AD_SINGLE_WORKER_CANCELLATION_PAIR_SOURCE_MATERIALIZED`

C03e-AE preserves exact AD lineage. It is a selection-only checkpoint.

## Purpose

Select only the next lifecycle ownership boundary needed after C03e-AD proved one concrete orderly cancellation controller/signal pair for the already-existing C03e-AB spawned worker.

The missing boundary is not worker collection yet. The existing executor is a private Tokio current-thread runtime, and the existing C03e-AB spawned seam keeps that runtime actively driven only because one lexical supervisor `block_on` remains in progress until its one local `JoinHandle` completes.

Therefore the next selected shape is one bounded current-thread single-worker supervisor that:

1. keeps the existing private current-thread runtime actively driven for the whole supervised worker lifetime;
2. creates exactly one C03e-AD cancellation pair for exactly one worker;
3. moves the signal future into exactly one C03e-AB-shaped spawned worker;
4. retains the matching controller and exactly one `JoinHandle` in the lexical supervisor future;
5. races worker completion against one caller-supplied orderly supervisor-shutdown future;
6. gives already-completed worker termination precedence in a same-poll tie;
7. if shutdown wins while the worker is pending, requests cancellation exactly through the retained AD controller and then awaits the same worker handle to terminal completion;
8. returns only the already-selected worker stop / bounded abnormal-join result;
9. returns only after the worker has completed and no task remains detached.

This checkpoint selects that one-worker supervisor custody only. It does not select a persistent worker collection, a second worker, concurrent authenticated-session admission, process-signal wiring, Agent `main.rs`, readiness, remote listener activation, deployment or merge.

## Why completion custody alone is not the missing boundary

C03e-AB already materialized and validated:

- one local `JoinHandle<AuthenticatedRemoteSessionWorkerStop>`;
- exactly-once join before the bounded drive returns;
- normal `AuthenticatedRemoteSessionWorkerStop` propagation;
- bounded `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion` classification;
- no detached handle and no hard abort.

C03e-AD then materialized the concrete single-worker cancellation pair.

The remaining first-order problem before any retained collection can be designed is current-thread runtime-driving custody: a Tokio current-thread task does not become a persistent independently executing worker merely because a join handle exists. The private runtime must remain actively driven while that task is expected to make progress.

C03e-AE therefore does not invent a second completion type. It selects how existing completion and AD cancellation custody coexist under one actively driven current-thread supervisor.

## Selected future source seam

A separately gated source checkpoint may add one bounded `RemoteSessionExecutorRuntime` operation equivalent in responsibility to:

`drive_supervised_capability_request_worker(...)`

The operation remains a synchronous bounded drive on the existing non-cloneable executor owner and takes `&mut self` for the whole call.

It consumes by value:

- one `AuthenticatedRemoteSessionRuntimeOwner`;
- one dispatcher `D`;
- one verifier-time provider `T`;
- one caller-supplied orderly supervisor-shutdown future `S`.

It receives the existing `SharedCurrentCapabilityAuthority<P>` by borrowed reference and may clone it exactly as already validated by C03e-AB for the one spawned worker.

The selected return shape remains:

`Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

No new public lifecycle result or raw Tokio type is selected.

## Caller-supplied supervisor shutdown is not worker cancellation authority

The caller-supplied `S: Future<Output = ()>` is only the readiness input telling the lexical supervisor to begin orderly shutdown of its one worker.

The supervisor itself owns the actual worker-cancellation authority by retaining the C03e-AD `RemoteSessionWorkerCancellationController`.

When `S` becomes ready while the worker remains pending, the supervisor performs exactly one semantic action:

`controller.request_cancellation()`

The paired signal future remains the only cancellation future moved into the worker.

This preserves the AD boundary:

- shutdown readiness carries no PRW identity or request data;
- controller drop alone is not cancellation;
- cancellation is explicit and monotonic;
- no transport is closed by the controller;
- the existing C03e-S worker remains authoritative for the code-4 close and `Cancelled` terminal stop.

## Supervisor construction order

The future source materialization must preserve one explicit lexical custody order equivalent in responsibility to:

1. enter one bounded `Runtime::block_on(...)` supervisor future on the already-owned private current-thread runtime;
2. construct exactly one AD cancellation controller/signal pair inside or immediately for that lexical supervisor lifetime;
3. clone the shared-current authority exactly once for the worker task;
4. spawn exactly one worker task under the same runtime custody;
5. move `signal.into_cancelled()` into that task as the worker cancellation future;
6. retain the controller, caller shutdown future and one local join handle in the supervisor;
7. drive the completion-vs-shutdown race while the same current-thread runtime remains active;
8. if completion wins, return its existing bounded classification;
9. if shutdown wins while completion is pending, request cancellation and continue driving the same join handle until it resolves;
10. return only after that handle has been consumed exactly once.

No task handle, runtime handle, cancellation controller or shutdown future escapes the bounded drive.

## Completion-first race ordering

The selected supervisor race is deterministic at the semantic boundary.

On every wake while both worker completion and supervisor shutdown are candidates:

1. poll worker completion first;
2. if worker completion is ready, classify and return it without issuing a new cancellation request;
3. only while worker completion remains pending, poll the supervisor-shutdown future;
4. if shutdown is ready, request cancellation exactly once and leave the race phase;
5. after cancellation has been requested, await only the retained worker handle to terminal completion.

This ordering prevents a same-poll completed worker from being retrospectively reclassified as cancelled merely because the supervisor-shutdown future is also ready.

No fairness or repeated alternating tie policy is selected for more than this one worker.

## Worker result remains authoritative

Supervisor shutdown readiness does not fabricate a `Cancelled` result.

After the supervisor requests cancellation, the worker still owns the C03e-S race between its request loop and the AD signal future.

Therefore the final worker result may still be:

- `AuthenticatedRemoteSessionWorkerStop::Cancelled` when cancellation wins while the request loop is pending; or
- `AuthenticatedRemoteSessionWorkerStop::Failed(...)` when a real request-loop failure wins first.

The supervisor returns whichever existing worker stop is actually produced.

A late shutdown request must not rewrite a real earlier request-loop failure as cancellation.

Abnormal Tokio task completion remains mapped only to existing `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

## Existing C03e-S cancellation ordering remains authoritative

When the AD signal wins inside the worker, C03e-S remains authoritative for:

1. dropping the pending C03e-Q request-loop future first;
2. releasing its mutable borrow before shutdown close;
3. closing the same retained peer exactly once with code `4` and fixed reason `remote capability session shutdown`;
4. returning `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

Request-loop failure retains code `3` / `remote capability session terminated` and the original typed transaction error.

C03e-AE does not move peer-close authority into the supervisor.

## Current-thread runtime custody

The existing C03e-T/U executor choice remains unchanged:

- one Agent-owned Tokio runtime;
- current-thread flavor;
- non-cloneable owner;
- private raw runtime;
- I/O/time drivers enabled;
- no public `Runtime` or `Handle` accessor.

The selected supervisor must keep one `Runtime::block_on(...)` call active for the complete one-worker supervision lifetime.

It must not spawn a worker and then return from `block_on` while expecting that current-thread worker to continue making progress independently.

This checkpoint does not select `rt-multi-thread`, a background runtime thread, a second executor family, or a runtime-handle escape.

## Shared-current authorization remains unchanged

The task-owned worker continues to use the C03e-X/Z shared-current authority semantics already validated by C03e-AB:

- one outer `Arc` clone may move into the worker;
- `WorkspaceDeviceRegistry` is not cloned;
- policy `P` is not cloned;
- every protected request obtains current registry/policy authority;
- one ephemeral `CapabilityBridge` is constructed for current authorization;
- authority guard is released before dispatcher side effects and response I/O;
- authorization evidence is one-request only.

Supervisor ownership adds no registry/policy snapshot and no cached authorization result.

## No controller clone or fan-out

C03e-AE retains exactly one controller for exactly one worker.

It does not select `Clone` for controller or signal and does not construct a second signal.

No broadcast/watch fan-out, cancellation token, cancellation map or multi-worker shutdown primitive is selected.

The caller shutdown future is not cloned or distributed.

## No persistent worker handle yet

The selected join handle remains lexically contained in one supervisor future.

It is not:

- returned to the caller;
- stored in `RemoteSessionExecutorRuntime`;
- inserted into `Vec`, map, slab, queue or `JoinSet`;
- keyed by `DeviceId`, `TransportIdentity` or a local worker identifier;
- converted to an abort handle;
- detached or forgotten.

This is intentionally still pre-collection.

## No hard abort

C03e-AE does not select:

- `JoinHandle::abort()`;
- `AbortHandle::abort()`;
- `JoinSet::shutdown()`;
- runtime shutdown as worker cancellation;
- dropping a live join handle as a lifecycle mechanism.

After orderly cancellation request, the supervisor continues to drive and await the same worker.

No deadline-based forced termination is selected.

## No concurrent authenticated-session admission

The supervisor remains semantically single-worker.

Because `&mut RemoteSessionExecutorRuntime` remains borrowed for the whole synchronous drive and the one handle is joined before return:

- no second supervised drive can execute concurrently through the same executor owner;
- no persistent worker is retained after return;
- no remote accept loop is active alongside a retained worker through this seam;
- no active-worker capacity or admission backpressure exists;
- no duplicate logical-device admission policy is selected.

Internal task spawn does not by itself constitute concurrent authenticated-session support.

## Dependency boundary

C03e-AE selects no dependency or Cargo feature change.

The future source materialization must use only the already-present Agent Tokio surface (`rt`, `net`, `time`, `sync`) and ordinary `std::future` polling primitives if a manual race is needed.

Tokio `macros` and `rt-multi-thread` remain unselected.

These files must remain byte-stable through AE:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

## Identity invariants

Supervisor/runtime/cancellation/task state creates no PRW identity.

- DeviceId / authenticated PRW session identity remain logical identity;
- TransportIdentity remains lower-transport identity;
- IP remains a transient endpoint;
- controller state, signal state, Tokio task ID, join handle, runtime ID, thread ID, PID/UID/GID and lock identities are not logical identity.

No implementation identity may be exposed as protocol identity, authorization identity or future collection key by this checkpoint.

## Exact selection scope

C03e-AE is docs-only.

Its final AD -> AE net diff must contain exactly this contract path and no source, manifest, lockfile, workflow, Android application, bridge, transport, Agent binary, packaging or host mutation.

## Validation requirements

Closure requires on the final exact AE head:

- exact AD merge base;
- exact one-path docs-only net scope;
- permanent PRW Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- Android validation is not required to trigger for the docs-only exact head; no Android PASS may be claimed if it does not run;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit with raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AD predecessor prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Explicit non-selection

C03e-AE does not select or perform:

- source materialization of the supervisor seam;
- persistent worker collections;
- persistent join-handle custody after the bounded drive returns;
- multiple cancellation controllers or signals;
- controller/signal `Clone`;
- multi-worker shutdown or drain;
- concurrent authenticated-session admission;
- collection key selection;
- duplicate DeviceId policy;
- worker-capacity/fairness scheduling;
- process-signal wiring;
- Agent `main.rs` wiring;
- remote listener/bind activation;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge.

## Completion meaning

Closure of C03e-AE means only that the next one-worker lifecycle custody is selected: one current-thread lexical supervisor keeps the private runtime actively driven, retains the matching AD cancellation controller and one local join handle, gives worker completion precedence in a same-poll tie, requests orderly cancellation only if supervisor shutdown wins while the worker is pending, and joins the worker before returning.

It does not mean the supervisor seam exists in Rust source, a worker collection exists, multiple authenticated sessions can run concurrently, process shutdown is wired, Agent `main.rs` is wired, remote transport is activated or readiness may be published.

The next checkpoint may materialize only this bounded current-thread single-worker supervisor seam and focused tests. Persistent worker collection/admission remains separately gated after that source is proven.

Target gate:

`C03E_AE_CURRENT_THREAD_SINGLE_WORKER_SUPERVISOR_CUSTODY_SELECTED`
