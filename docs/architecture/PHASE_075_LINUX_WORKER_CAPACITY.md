# Phase 075 — Linux Bounded Worker Capacity

Status: implementation payload awaiting CI validation

## Purpose

Implement thread-safe RAII accounting for the Phase 072 maximum-active-worker bound without spawning any worker thread or accepting any connection.

## State model

`LocalLinuxWorkerCapacity` owns shared state through `Arc`:

- caller-supplied strictly positive maximum worker count;
- atomic current active-worker count.

`try_acquire()` performs a nonblocking compare-exchange acquisition:

- below capacity → returns one non-cloneable `LocalLinuxWorkerPermit`;
- at capacity → returns typed `AtCapacity` without waiting or side effects.

The permit owns an `Arc` reference to the same accounting state. Its `Drop` releases exactly one active slot. No public release/reset operation exists.

The capacity handle is `Clone + Send + Sync`, and the permit is `Send`, allowing a later phase to move one already-accounted permit into an OS worker thread without creating an accounting gap.

## Scope

Phase 075 does not:

- spawn a thread/task;
- accept/authenticate a connection;
- process a Request;
- select a production worker-capacity value;
- implement an accept scheduler;
- implement shutdown/join behavior;
- activate Agent bootstrap or systemd.

## Validation target

CI must prove:

- initial active count is zero;
- acquisition stops exactly at the caller bound;
- a dropped permit releases exactly one slot;
- a released slot can be reacquired;
- cloned capacity handles share one accounting state;
- capacity is `Send + Sync` and permit is `Send`;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
