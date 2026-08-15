# Phase 076 — Linux Finite Authenticated Session Worker Body

Status: implementation payload awaiting CI validation

## Purpose

Implement the finite per-session worker body required by Phase 072 while still spawning no OS thread and accepting no connection.

## Ownership

The worker body consumes:

- one `AuthenticatedLocalLinuxSession<UnixStream>`;
- one non-cloneable Phase 075 `LocalLinuxWorkerPermit`;
- caller-supplied policy/snapshots;
- a strictly positive maximum Request count;
- Phase 073/074 read and write I/O budgets.

The session and permit are never returned to the caller.

## Processing semantics

For each permitted Request:

1. invoke Phase 074 `process_one_with_deadlines`;
2. each Request gets a fresh absolute read deadline and an independent deferred response-write deadline;
3. a written response increments the completed-response count;
4. clean EOF terminates the worker normally;
5. the first processing error terminates the worker with the count of prior successful responses;
6. consuming the final permitted Request terminates with `RequestBudgetExhausted`.

Every terminal path returns from the worker body and therefore drops the authenticated session stream. Runtime orchestration must not resume that connection with a fresh Request budget.

The Phase 075 permit remains owned for the complete worker-body call and releases its slot through RAII on return or unwind.

## Scope

Phase 076 does not:

- spawn a thread/task;
- accept/authenticate a new connection;
- implement a scheduler;
- choose production Request/deadline values;
- implement shutdown/join accounting;
- activate Agent bootstrap or systemd.

## Validation target

CI must prove:

- Request-budget exhaustion writes the final response, then closes the session and releases the permit;
- policy evaluation observes the worker slot as active;
- clean EOF releases the permit without policy evaluation;
- a read-deadline failure releases the permit and reports the correct prior-response count;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
