# Phase 078 — Linux Single Scoped Worker Spawn

Status: implementation payload awaiting CI validation

## Purpose

Implement exactly one fallible scoped OS-thread spawn around the already validated Phase 076 finite authenticated-session worker body.

## Entry conditions

The adapter receives:

- an existing `std::thread::Scope` owned by future runtime orchestration;
- one already-authenticated `AuthenticatedLocalLinuxSession<UnixStream>`;
- one already-acquired Phase 075 `LocalLinuxWorkerPermit`;
- borrowed policy and private-DNS snapshot context valid for the scope;
- status snapshot and typed Phase 076 worker config.

It does not acquire capacity and does not accept/authenticate a connection itself.

## Spawn semantics

`std::thread::Builder::spawn_scoped` is invoked exactly once.

On success:

- session and permit are moved into the scoped worker closure;
- the closure runs `run_authenticated_session_worker` exactly once;
- the caller receives the `ScopedJoinHandle<Result<LocalLinuxSessionWorkerStop, LocalLinuxSessionWorkerError>>`;
- the caller remains responsible for explicit result accounting/joining in later orchestration;
- the enclosing thread scope structurally prevents the worker from outliving runtime scope.

On OS thread-creation failure:

- the adapter returns typed `SpawnFailed`;
- captured session/permit inputs are disposed as part of the failed spawn path;
- the stream therefore cannot remain half-scheduled and the worker-capacity permit is not intentionally retained.

## Scope

Phase 078 does not:

- accept a connection;
- acquire worker capacity;
- schedule multiple workers;
- create a result registry;
- implement shutdown/cancellation;
- choose thread name/stack size;
- activate Agent bootstrap/systemd/service state.

## Validation target

CI must prove:

- a scoped worker holds its capacity permit while policy evaluation is in progress;
- explicit join returns the exact Phase 076 worker result;
- permit count returns to zero after the joined worker completes;
- response bytes are emitted and the session stream closes after finite worker completion;
- scoped workers can borrow policy/snapshot data from the thread-scope environment;
- all tests remain safe Rust under workspace `unsafe_code = "forbid"`;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
