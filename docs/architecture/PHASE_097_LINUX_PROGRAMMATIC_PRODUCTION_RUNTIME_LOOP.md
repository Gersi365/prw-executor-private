# Phase 097 — Linux Programmatic Production Runtime Loop

Status: `IMPLEMENTED_AND_VALIDATED`

## Purpose

Phase 097 implements the first callable long-running production-local runtime loop authorized by Phase 094-A01. It repeatedly invokes the already-validated finite Phase 092 orchestration primitive while retaining one scoped worker registry, memory-bounded process-lifetime counters, and the Phase 095 connection-local versus fail-stop error-disposition policy.

The loop remains below the product bootstrap boundary. It is not wired into `main.rs`, does not process OS signals, and does not install, enable, or start systemd.

## Integrated source

Integrated source commit:

`b9583c3757003d2cfbfb8e38db9d6e247faf29bf`

## Runtime input boundary

The initial Phase 097 preflight exposed an API ergonomics issue: the loop took eight arguments and passed the small copyable `BoundedLocalReadPolicy` by reference.

Phase 097-A01 replaced those individual operational values with immutable `LocalLinuxProductionRuntimeInputs<'a>` containing:

- `LocalLinuxProductionRuntimeConfig`;
- `BoundedLocalReadPolicy` by value;
- `LocalAgentStatusSnapshot`;
- a borrowed `LocalPrivateDnsSnapshot`.

The core loop therefore takes exactly five inputs: listener, runtime wake, worker capacity, scheduler control, and the immutable runtime-input bundle.

## Long-running loop semantics

`run_local_linux_production_runtime_loop(...)` owns one `std::thread::scope` and one scoped worker registry for the loop lifetime.

Each iteration performs exactly one Phase 092 finite orchestration invocation.

Successful outcomes are disposed as locked by Phase 094-A01:

- `ShutdownObserved` establishes terminal `ShutdownRequested`;
- `RuntimeWake` returns to readiness after wake/state/completion processing;
- `WaitInterrupted` returns to readiness without hiding a syscall retry loop;
- all successful `Scheduling(...)` stop reasons return to capacity-aware blocking readiness.

Runtime errors are classified through Phase 095:

- same-UID peer-authorization rejection records bounded rejection evidence and continues;
- every other initial readiness/scheduling error establishes a typed fatal terminal reason and enters teardown.

There is no sleep, periodic timeout, polling cadence, or unbounded retry/backoff policy around errors.

## Process-lifetime evidence

Phase 097 extends the Phase 095 fixed-size counters to retain evidence that may accumulate before a scheduling error:

- the listener-ready readiness step;
- scheduling attempts including the failed attempt;
- workers registered before the failure;
- worker completions reaped before the failure.

Final worker completions returned by `join_all()` are also added to the saturating completion counter.

Only final cancellation outcomes and final joined completions remain as vectors, and both are bounded by configured worker capacity.

## Terminal teardown

When the loop establishes shutdown or fatal termination:

1. `registry.cancel_all()` is called for every retained worker;
2. `registry.join_all()` joins/classifies every remaining worker;
3. final completions are added to bounded counters;
4. the worker scope exits;
5. Phase 096 performs explicit listener/socket cleanup;
6. loop evidence and cleanup evidence are combined into `LocalLinuxProductionRuntimeTerminalReport`.

The listener cleanup result remains independent from the original loop terminal reason.

## Programmatic control seam

`run_local_linux_production_runtime_from_env(...)` is callable below `main.rs`.

After Phase 096 lifecycle assembly and before the blocking loop begins, the supplied `on_started` callback receives a cloneable `LocalLinuxRuntimeShutdownHandle`. This is the Phase 097 pre-signal control seam and allows deterministic shutdown testing without process-global signal mutation.

OS signal integration remains Phase 098 work.

## Focused runtime proofs

### Shutdown before first blocking wait

A temporary-root integration test requests shutdown and runtime wake from `on_started` before the first loop wait. It proves:

- exactly one readiness/orchestration step observes terminal shutdown;
- zero workers are registered;
- no final cancellation/completion entries exist;
- terminal reason is `ShutdownRequested`;
- listener/socket cleanup is `Clean`;
- the socket pathname is absent after return.

### One client round-trip then shutdown

A second integration test starts a local client after lifecycle assembly. The client:

1. connects to the Agent Unix socket;
2. writes one `GetAgentStatus` request;
3. receives and decodes the correctly correlated success response;
4. requests programmatic shutdown and wake.

The runtime then returns through the full terminal transaction. The test proves:

- at least one listener-ready scheduling attempt occurred;
- exactly one worker was registered;
- the worker completion is represented in bounded counters;
- peer-rejection count remains zero;
- terminal reason is `ShutdownRequested`;
- listener/socket cleanup is `Clean`;
- the socket pathname is absent after return.

## Preflight history

Initial Phase 097 integration run `31901904093` stopped safely at Clippy before source integration. It reported only API ergonomics findings: too many loop arguments and trivial-copy policy passed by reference.

The source was refactored in commit `867ecb9fb8ae9cda1e2cb2db2537119f7a1bded7` to introduce `LocalLinuxProductionRuntimeInputs` rather than suppressing lints.

Phase 097-A01 run `31901993038` then passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and `git diff --check`. Only after full PASS did A01 commit the integrated source and delete both temporary Phase 097 workflows.

## Authoritative permanent validation

Permanent PRW Rust Validation run:

`31902081956`

Validated commit containing the integrated source:

`469a5cfb0680687c49f7eec8151120b7c5b56365`

The permanent workflow passed:

- `cargo metadata --locked --no-deps --format-version 1`;
- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --locked --workspace --all-targets`;
- `cargo build --locked --workspace --all-targets`.

## Boundary preserved

Phase 097 does not implement or activate:

- OS signal handling;
- a signal dependency;
- `main.rs` runtime bootstrap;
- process exit-code mapping;
- systemd installation/enable/start;
- deployment;
- remote/public networking;
- auth/enrollment expansion;
- TUN/relay/database/private-key changes;
- Wake-on-LAN.

Phase 097 is therefore `IMPLEMENTED_AND_VALIDATED` below the production bootstrap boundary.
