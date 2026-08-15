# Phase 086 — Linux Bounded Scheduling Cycle

Status: formatter-corrected integrated source awaiting authoritative CI validation

## Purpose

Implement the Phase 085 finite scheduling-cycle contract without creating a long-running wait loop or activating the Agent bootstrap.

## Scheduler control

`LocalLinuxSchedulerControl` is a cloneable shared monotonic stop control backed by `Arc<AtomicBool>`.

- new/default starts in `Running`;
- `request_shutdown()` performs the one-way transition to shutdown requested;
- `is_shutdown_requested()` is a read-only observation;
- there is no reset/resume operation.

The control does not cancel active workers. Phase 082 cancellation remains a separate shutdown step.

## Bounded cycle

`run_bounded_scheduling_cycle` receives a caller-supplied `NonZeroUsize` attempt budget.

For each possible attempt it performs:

1. `registry.reap_finished()` and appends completions to cycle evidence;
2. checks the shutdown control;
3. if shutdown is observed, returns before Phase 084 and therefore before any accept;
4. otherwise performs exactly one Phase 084 scheduling transaction.

Phase 084 outcomes are handled as follows:

- `WorkerRegistered`: increment registration count and continue only if attempt budget remains;
- `AtCapacity`: terminate the cycle immediately;
- `NoConnectionReady`: terminate the cycle immediately;
- bounded Phase 084 error: terminate immediately and return prior completion/registration evidence with the error.

## Evidence

A successful cycle report contains:

- ordered worker completions reaped during the cycle;
- workers successfully registered;
- actual Phase 084 attempt count;
- stop reason (`AttemptBudgetExhausted`, `AtCapacity`, `NoConnectionReady`, or `ShutdownRequested`).

An error report preserves prior completions, prior successful registrations, the attempt count including the failed attempt, and the bounded Phase 084 error.

## Scope

Phase 086 does not:

- sleep/poll/epoll or wait for readiness;
- create an unbounded loop;
- choose a production attempt budget;
- automatically call worker cancellation/join on shutdown;
- activate Agent bootstrap/systemd/service state.

## Validation target

CI must prove:

- shutdown requested before a cycle results in zero scheduling attempts and leaves a queued client unconsumed;
- `NoConnectionReady` stops after one attempt even with a larger budget;
- `AtCapacity` stops after one attempt and does not consume a queued connection;
- exact attempt budget limits multiple queued worker registrations;
- a finished worker is reaped before the next scheduling attempt and its completion appears in cycle evidence;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
