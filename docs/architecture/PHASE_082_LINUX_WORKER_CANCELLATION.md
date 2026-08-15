# Phase 082 — Linux Worker Cancellation Authority and Registry Pairing

Status: implementation payload awaiting CI validation

## Purpose

Implement the Phase 081 shutdown authority before the first accept-and-spawn scheduler exists.

## Cancellation authority

`LocalLinuxWorkerCancellation` can be constructed only from an already-authenticated `AuthenticatedLocalLinuxConnection<UnixStream>`.

Construction calls `UnixStream::try_clone()` and retains that independently owned handle solely for terminal cancellation. The wrapper exposes no application read/write API.

`cancel()` issues `shutdown(Shutdown::Both)` and returns a bounded shutdown error if the operating system rejects the call.

## Registry pairing

The Phase 080 registry now stores one entry containing:

- one Phase 078 `ScopedJoinHandle`;
- one matching `LocalLinuxWorkerCancellation`.

`cancel_all()` issues shutdown on every retained cancellation authority without removing any handle.

`reap_finished()` joins/classifies finished workers and drops the matching cancellation authority with the removed entry.

`join_all()` joins/classifies all remaining workers and drops the paired cancellation authorities.

## Shutdown semantics

The intended future shutdown sequence remains:

1. stop accept/scheduling;
2. `cancel_all()`;
3. `join_all()`;
4. leave thread scope;
5. listener/path/instance-lock cleanup.

A cancellation syscall failure remains visible in the `cancel_all()` result and does not remove the obligation to join that worker.

## Scope

Phase 082 does not:

- implement an accept scheduler;
- accept a connection in production code;
- acquire worker capacity;
- bind a policy evaluator to runtime identity;
- activate Agent bootstrap/systemd/service state;
- add application I/O through the cancellation clone.

## Validation target

CI must prove:

- cancellation authority can be created after authentication;
- shutdown is terminal for the underlying stream;
- registry retains cancellation authority for running workers after finished-worker reaping;
- `cancel_all()` leaves handles registered for mandatory joining;
- a worker blocked on a deliberately long Request-read deadline is woken and joined substantially before that deadline after cancellation;
- the worker permit returns to zero after cancellation/join;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
