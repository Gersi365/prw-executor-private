# Private Remote Workspace Linux Agent Bounded Scheduling Cycle Contract

Version: `0.1.0`

Status: Phase 085 scheduler-orchestration decision lock — no long-running Agent loop/bootstrap activation

## Purpose

Lock the finite orchestration semantics that may repeatedly invoke the Phase 084 one-shot scheduler without introducing busy polling, unbounded accept retries, or post-shutdown accepts.

## Existing baseline

The validated repository already establishes:

- Phase 075 bounded active-worker capacity;
- Phase 080/082 worker registry, nonblocking finished-worker reaping, cancellation, and mandatory joining;
- Phase 083 bounded in-memory local policy;
- Phase 084 exactly one capacity-gated accept/authenticate/cancel-clone/spawn/register transaction per call.

## Decision 1 — caller-bounded scheduling attempts

One scheduling-cycle invocation receives a caller-supplied `NonZeroUsize` attempt budget.

The cycle may invoke Phase 084 at most that many times.

No internal default, unbounded loop, recursive retry, sleep, poll, or wall-clock wait is authorized by this cycle.

## Decision 2 — reap before each scheduling attempt

Before every potential Phase 084 call, the cycle invokes the Phase 080 registry's nonblocking `reap_finished()`.

All returned worker completions are retained in the cycle result in observation order.

This allows completed worker permits to be released before capacity is checked for a new connection and prevents finished handles from accumulating unnecessarily.

## Decision 3 — explicit caller-controlled shutdown flag

The initial scheduler control is a small shared atomic stop flag.

It has exactly two monotonic states:

- `Running`;
- `ShutdownRequested`.

The transition is one-way for one runtime instance. No reset/resume operation exists.

The scheduling cycle checks the shutdown flag after finished-worker reaping and immediately before every Phase 084 scheduling transaction.

If shutdown is requested, the cycle returns without calling Phase 084 and therefore performs no new accept/spawn operation.

The shutdown flag itself does not cancel existing workers. Existing workers remain governed by the Phase 082 `cancel_all()` + `join_all()` shutdown boundary.

## Decision 4 — no busy-loop on no-work conditions

A Phase 084 result of either:

- `AtCapacity`; or
- `NoConnectionReady`

terminates the current bounded scheduling cycle immediately.

The cycle does not retry these conditions internally, even if unused attempt budget remains.

A future outer runtime wait/readiness mechanism must decide when to invoke another cycle.

## Decision 5 — successful registrations may continue within the finite budget

`WorkerRegistered` consumes exactly one scheduling attempt.

After a successful registration, if attempt budget remains, the cycle begins the next iteration by reaping finished workers and checking shutdown again before another possible Phase 084 call.

Thus one cycle may schedule multiple already-queued connections, but never more than the caller-provided attempt count and never beyond worker capacity.

## Decision 6 — errors terminate the cycle

The first Phase 084 scheduling error terminates the cycle immediately.

The error result must retain:

- the worker completions already reaped earlier in that cycle;
- the number of workers successfully registered earlier in that cycle;
- the bounded Phase 084 error.

No automatic retry occurs after accept/auth/cancellation-clone/spawn failure.

## Decision 7 — cycle result is evidence-bearing

A successful bounded cycle result records:

- ordered worker completions reaped during the cycle;
- number of workers registered during the cycle;
- number of Phase 084 scheduling attempts actually performed;
- terminal stop reason.

Stop reasons are at least:

- `AttemptBudgetExhausted`;
- `AtCapacity`;
- `NoConnectionReady`;
- `ShutdownRequested`.

This is runtime state/evidence returned to the caller, not a persistent audit log by itself.

## Decision 8 — shutdown boundary remains separate

Phase 085 does not merge scheduling-cycle return with final shutdown execution.

Future shutdown orchestration remains:

1. request shutdown on the shared scheduler control;
2. stop invoking new scheduling cycles;
3. `cancel_all()` active registered workers;
4. `join_all()` and classify every remaining worker;
5. leave the scoped-thread lifetime;
6. clean listener/path/instance-lock lifecycle.

## Next safe implementation

Phase 086 may implement:

- a crate-internal monotonic `LocalLinuxSchedulerControl` backed by `AtomicBool`;
- a pure bounded `run_scheduling_cycle` composition over Phase 080/082/084;
- structured success/error evidence as locked above;
- tests proving zero accept after shutdown, no retry on `AtCapacity`/`NoConnectionReady`, finished-worker reaping before capacity checking, and exact attempt-budget enforcement.

Phase 086 must not implement a long-running outer wait loop, poll/epoll, sleep cadence, Agent bootstrap, or service activation.

## Deferred

This contract does not select:

- the production scheduling attempt budget;
- poll/epoll/eventfd/condition-variable runtime wake mechanism;
- how the long-running Agent waits between cycles;
- production shutdown signal source;
- production listener backlog/capacity/deadline/Request-count values.

## Forbidden interpretation

Phase 085 does not authorize:

- an unbounded accept loop;
- busy polling;
- accepts after shutdown is observed;
- retries after scheduling errors;
- arbitrary `PolicyEvaluator` binding;
- Agent bootstrap/systemd activation;
- remote networking/DNS/TUN/database/private-key/deployment mutation.
