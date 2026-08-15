# Phase 073 — Linux Absolute-Deadline Blocking I/O Adapter

Status: implementation payload awaiting CI validation

## Purpose

Implement the first Phase 072 wait-bound primitive without changing accepted-stream blocking mode, spawning workers, accepting connections, or activating the Agent runtime.

## Model

Two crate-internal adapters borrow an already-authenticated `UnixStream`:

- `LocalLinuxDeadlineReader`;
- `LocalLinuxDeadlineWriter`.

Each adapter is constructed from a strictly positive `LocalLinuxIoBudget` and computes one immutable monotonic absolute deadline at construction time.

Before every underlying blocking read/write call, the adapter recomputes the remaining duration until that same deadline and programs the corresponding Unix socket timeout to no more than that remainder.

Partial progress therefore cannot create a fresh full-duration budget.

Linux timeout-style `WouldBlock` / `TimedOut` results are normalized to `std::io::ErrorKind::TimedOut` inside this adapter.

## Scope

Phase 073 does not:

- change the listener or accepted-stream `O_NONBLOCK` mode;
- accept a connection;
- authenticate a peer;
- process a PRW Request;
- create a worker/thread/task;
- choose production timeout values;
- bind the adapter into Phase 060 yet;
- activate Agent bootstrap or systemd.

## Validation target

CI must prove:

- zero duration is rejected;
- unrepresentable monotonic deadlines are rejected;
- read/write round-trip succeeds over `UnixStream::pair()`;
- partial progress leaves the stored absolute deadline unchanged;
- an idle peer read expires as `TimedOut`;
- workspace remains clean under locked metadata, rustfmt, Clippy `-D warnings`, tests, and build.
