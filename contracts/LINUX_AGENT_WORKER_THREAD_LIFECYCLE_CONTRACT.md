# Private Remote Workspace Linux Agent Worker Thread Lifecycle Contract

Version: `0.1.0`

Status: Phase 077 lifecycle decision lock — no production thread spawn in this phase

## Purpose

Lock the ownership and join semantics for the first Linux Agent OS session workers before any production-side thread is created.

Phase 072 selected native OS threads as the initial worker model and prohibited unbounded or detached workers. Phase 075/076 now provide bounded capacity accounting and a finite session worker body. This phase chooses the thread lifetime mechanism that will contain those workers.

## Evidence

Rust 1.97.1 standard-library semantics establish:

- a normal `std::thread::JoinHandle` detaches its thread when the handle is dropped;
- `std::thread::Builder::spawn` returns an `io::Result<JoinHandle<_>>` and therefore permits bounded handling of OS thread-creation failure;
- threads created inside `std::thread::scope` cannot outlive the scope;
- a dropped scoped join handle is still joined when the scope ends;
- `std::thread::Builder::spawn_scoped` returns an `io::Result<ScopedJoinHandle<_>>`.

## Decision 1 — scoped workers only for the initial Agent runtime

The initial Linux Agent runtime will create authenticated session workers only inside one enclosing `std::thread::scope` owned by the Agent runtime orchestration call.

The initial runtime will not create unscoped session workers with `std::thread::spawn` or `Builder::spawn`.

Reason: dropping a normal `JoinHandle` would detach the worker and permanently lose join authority. Scoped workers provide a structural lifetime boundary that guarantees every spawned worker terminates or is joined before the Agent runtime scope can return.

## Decision 2 — Builder::spawn_scoped

The worker-spawn adapter will use `std::thread::Builder::spawn_scoped`, not `Scope::spawn`.

Reason: thread creation is an OS operation that can fail. The adapter must surface a bounded typed spawn failure rather than panic because thread creation failed.

Thread names may be supplied by a later orchestration layer only after naming rules are reviewed. Phase 077 does not choose a naming format or custom stack size.

## Decision 3 — capacity permit acquired before spawn

The Phase 075 `LocalLinuxWorkerPermit` must be acquired before the worker spawn attempt.

The permit is moved into the worker closure on successful spawn and remains live for the entire Phase 076 worker body.

If `spawn_scoped` fails, the captured worker inputs/permit must be recovered or dropped locally so the capacity slot is released and the authenticated connection is not left half-registered.

The scheduler must not perform an accept operation when no worker capacity is available, per Phase 072.

## Decision 4 — no detached-result policy

Although dropping a scoped join handle cannot detach the thread beyond the enclosing scope, the final runtime scheduler must still account for worker results.

The scheduler will retain scoped join handles until either:

- a finished worker is explicitly joined/reaped during runtime; or
- shutdown joins the remaining handles before the scope exits.

Worker return values and worker panics must not be silently discarded in the final runtime.

A separate phase will define the exact result registry/reaping structure before the accept scheduler is activated.

## Decision 5 — shutdown structural baseline

The Agent runtime scope is the outer lifetime boundary for all session worker threads.

Shutdown ordering will eventually be:

1. stop scheduling/accepting new sessions;
2. initiate the separately reviewed connection-worker shutdown/cancellation mechanism if needed;
3. join/reap every remaining scoped worker;
4. only then permit the thread scope to return;
5. only after worker completion clean up listener/path/instance-lock lifecycle.

Phase 077 locks only this structural order. It does not yet implement the shutdown signal/cancellation mechanism.

## Decision 6 — panic classification

A worker panic is not a normal worker result.

The future result-reaping layer must distinguish:

- normal Phase 076 worker stop;
- bounded Phase 076 worker error;
- thread panic/join failure.

The final Agent runtime must not intentionally propagate an unexamined worker panic by relying only on implicit scope panic behavior.

## Next safe implementation

Phase 078 may implement a crate-internal **single scoped-worker spawn adapter** that:

- accepts an existing `std::thread::Scope`;
- consumes one authenticated session and one already-acquired Phase 075 permit;
- captures the Phase 076 worker context;
- uses `Builder::spawn_scoped` exactly once;
- returns the `ScopedJoinHandle<Result<LocalLinuxSessionWorkerStop, LocalLinuxSessionWorkerError>>` to the caller;
- maps OS spawn failure to a typed error;
- performs no accept operation, no scheduler loop, and no Agent bootstrap activation.

Phase 078 tests must explicitly join every returned handle.

## Deferred

Phase 077 does not choose or implement:

- accept scheduling;
- worker result registry/reaping container;
- shutdown signal/cancellation primitive;
- worker thread naming;
- custom stack size;
- production worker capacity/deadline/Request-count values;
- Agent bootstrap/systemd activation.

## Forbidden interpretation

This decision does not authorize:

- unscoped/detached session workers;
- unbounded thread creation;
- ignoring worker results in the final runtime;
- application I/O on the accept/control thread;
- service activation or deployment.
