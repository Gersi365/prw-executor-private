# Private Remote Workspace Linux Agent Worker Cancellation Contract

Version: `0.1.0`

Status: Phase 081 shutdown/cancellation decision lock — no scheduler/bootstrap activation

## Purpose

Lock the cancellation authority that must be registered together with every future scoped session worker before the first accept-and-spawn scheduler is implemented.

## Evidence baseline

The validated repository already establishes:

- Phase 070 accepted `UnixStream` objects are authenticated before application bytes are exposed;
- Phase 071 composes an authenticated connection into an authenticated session;
- Phase 073/074 bound blocking Request/response I/O with absolute deadlines;
- Phase 075 bounds active workers with a permit;
- Phase 076 consumes a session/permit in a finite worker body;
- Phase 077/078 use scoped native worker threads;
- Phase 079 classifies worker completion;
- Phase 080 retains/reaps scoped worker handles.

Rust 1.97.1 `UnixStream::try_clone()` returns an independently owned handle referring to the same underlying stream. Both handles share the same data stream and socket options.

Rust 1.97.1 `UnixStream::shutdown(Shutdown::Both)` shuts down both halves so pending and future I/O on those portions return promptly with the platform-appropriate EOF/error behavior.

## Decision 1 — cancellation authority is a cloned authenticated stream handle

Immediately after a connection has passed Phase 070/059 same-UID authentication, and before the authenticated connection is moved into a worker session, the scheduler must create one `UnixStream::try_clone()` handle dedicated to cancellation.

The cancellation clone:

- is not an application reader/writer;
- is not exposed to policy or command processing;
- exists only to issue connection shutdown and retain independent descriptor ownership until worker completion/reaping;
- must be associated with the exact scoped worker created from that authenticated connection.

If cancellation-handle cloning fails, that connection must not be spawned as a worker. The scheduler drops/closes the accepted connection and releases the already-acquired worker permit.

## Decision 2 — registry owns handle plus cancellation authority

The Phase 080 registry model will be extended so each registered worker entry contains:

- one `ScopedJoinHandle`;
- one cancellation authority for the same underlying authenticated `UnixStream`.

When a finished worker is reaped normally, its cancellation authority is dropped together with the entry after the worker result is classified.

No orphan cancellation clone or orphan join handle is allowed.

## Decision 3 — shutdown operation

Runtime shutdown ordering for active workers is:

1. stop new scheduling/accept operations;
2. call `shutdown(Shutdown::Both)` through every registered worker cancellation authority;
3. join/classify every remaining worker through the Phase 079/080 path;
4. only after all worker entries are gone may the thread scope return;
5. listener/path/instance-lock cleanup remains after worker completion.

A shutdown call failure is recorded as bounded shutdown evidence but does not skip joining that worker. Join/reap remains mandatory.

## Decision 4 — cancellation is connection-scoped and terminal

Worker cancellation is terminal for that connection.

The runtime never attempts to resume/reuse a session after its cancellation authority has issued shutdown.

Existing inbound/write poisoning and Phase 076 finite-session drop semantics remain valid. Cancellation is not a retry signal.

## Decision 5 — absolute deadlines remain defense in depth

Phase 073/074 per-Request deadlines remain required even with explicit shutdown authority.

Cancellation handles Agent shutdown responsiveness; deadlines bound ordinary slow/stalled peers during normal runtime. Neither replaces the other.

## Decision 6 — policy evaluator blocking is a separate runtime boundary

Socket shutdown can interrupt socket I/O, but it cannot interrupt arbitrary computation or unrelated blocking I/O performed inside a `PolicyEvaluator` implementation.

The current `PolicyEvaluator` trait is synchronous and does not itself encode an I/O/time bound.

Therefore the initial Agent runtime scheduler may bind only a separately validated local policy evaluator whose `evaluate` path is bounded and performs no external/network/filesystem blocking I/O.

An arbitrary plugin/dynamic evaluator that may block is not authorized for the initial runtime path. Supporting such evaluators requires a separate cancellation/isolation design.

## Decision 7 — no concrete shutdown timing value yet

Phase 081 does not choose a shutdown grace duration.

Because active socket I/O receives explicit shutdown and every Request already has absolute deadlines, a future runtime shutdown phase may measure/join workers and define escalation/reporting policy separately.

## Required next implementation

Phase 082 may implement only:

1. a crate-internal cancellation-authority wrapper constructed via `UnixStream::try_clone()` from an already-authenticated connection;
2. a terminal `shutdown(Both)` operation with bounded error classification;
3. extension of the Phase 080 registry entry to own the cancellation authority beside the scoped join handle;
4. registry `cancel_all()` followed by existing `join_all()` semantics;
5. tests proving shutdown wakes a worker blocked in socket read well before a long normal read deadline, worker capacity returns to zero, and completed-worker reaping drops the matching cancellation authority.

Phase 082 must not implement accept scheduling, policy binding, Agent bootstrap, or service activation.

## Forbidden interpretation

Phase 081 does not authorize:

- application I/O through the cancellation clone;
- connection reuse after cancellation;
- arbitrary blocking policy evaluators in the initial runtime;
- an accept loop/scheduler;
- unbounded thread creation;
- Agent bootstrap/systemd activation;
- network/DNS/TUN/database/private-key/deployment mutation.
