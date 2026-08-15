# Phase 080 — Linux Scoped Worker Registry / Reaper

Status: implementation payload awaiting CI validation

## Purpose

Retain every Phase 078 scoped worker handle until its result is explicitly classified through Phase 079, before any accept scheduler or Agent runtime is activated.

## Registry model

`LocalLinuxScopedWorkerRegistry<'scope>` owns a bounded-runtime collection of scoped join handles. It does not duplicate the Phase 075 worker-capacity counter; the future scheduler remains responsible for registering only handles associated with already-acquired worker permits.

Operations:

- `register(handle)`: retain one already-spawned scoped worker handle;
- `reap_finished()`: use `ScopedJoinHandle::is_finished()` to identify completed workers without blocking on still-running workers, remove only those handles, and classify them through Phase 079;
- `join_all(self)`: consume the registry, block until every remaining worker terminates, and classify every result through Phase 079.

The registry is `#[must_use]` to make accidental result-accounting loss visible at compile time where possible. The enclosing `std::thread::scope` remains the structural no-detach boundary even if orchestration is incorrect, but the final runtime contract still requires explicit reaping/joining.

## Scope

Phase 080 does not:

- spawn a production worker;
- acquire worker capacity;
- accept a connection;
- implement an accept scheduler;
- implement shutdown signaling/cancellation;
- activate Agent bootstrap/systemd/service state.

## Validation target

CI must prove:

- empty/registration accounting;
- `reap_finished()` removes a finished worker while leaving a deliberately blocked worker registered;
- `join_all()` waits for and classifies remaining workers;
- registration-order classification for `join_all()`;
- panic classification is routed through Phase 079 rather than escaping as an unexamined join error;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
