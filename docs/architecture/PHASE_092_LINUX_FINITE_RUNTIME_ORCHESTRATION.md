# Phase 092 — Linux Finite Runtime Orchestration

Status: `PREFLIGHT_VALIDATED_INTEGRATED_SOURCE_AWAITING_AUTHORITATIVE_CI_VALIDATION`

## Purpose

Phase 092 composes the validated Phase 091 one-step capacity-aware readiness primitive with a runtime-specific caller-bounded scheduling cycle whose workers always use the Phase 090 completion-wake spawn path.

It remains below the production Agent bootstrap boundary. It does not introduce a production long-running runtime loop, `main.rs` activation, signal handling, systemd/service activation, remote/public networking, or deployment.

## Integrated source

Integrated source commit:

`4162b1ceeba14dbab8ef33cecd036f2e93e8b4d2`

The integration was committed only after the Phase 092 A02 preflight passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and the mutation-surface guard.

## Runtime-specific scheduling

`LocalLinuxRuntimeSchedulerContext` carries the already-validated finite worker inputs plus one Phase 089 completion-wake notifier.

`run_bounded_runtime_scheduling_cycle(...)` mirrors the Phase 085/086 bounded-cycle invariants while ensuring every newly registered worker is spawned through `spawn_authenticated_session_worker_with_completion_wake(...)`.

The runtime-specific scheduling cycle:

- reaps already-finished workers before every possible scheduling attempt;
- rechecks terminal shutdown before every possible accept;
- uses a caller-supplied `NonZeroUsize` attempt budget;
- preserves capacity-before-accept ordering;
- stops immediately on `AtCapacity` or `NoConnectionReady`;
- terminates on the first bounded scheduling error;
- records completion, registration, attempt, and stop evidence;
- contains no outer readiness loop.

The older Phase 084/086 APIs remain intact and are not silently changed to require runtime wake state.

## Finite readiness + scheduling composition

`run_finite_linux_runtime_orchestration(...)` performs exactly one Phase 091 readiness invocation.

Readiness outcomes map as follows:

- `ShutdownObserved` => return immediately with zero scheduling attempts;
- `RuntimeWake` => return immediately with zero scheduling attempts;
- `WaitInterrupted` => return immediately with zero scheduling attempts;
- `ListenerReady` => enter exactly one caller-bounded runtime-specific scheduling cycle.

This is a finite orchestration primitive, not a production event loop.

## Completion-wake scheduling proof

A focused real-kernel test schedules one authenticated local worker through Phase 092 and proves:

- exactly one worker is registered;
- the worker returns a valid bounded local status response;
- the worker completion path emits Phase 090/089 runtime wake;
- subsequent Phase 091 wake handling observes released worker capacity;
- residual scoped-worker ownership is still bounded by registry cancellation/join semantics.

## Capacity-restoration proof

A deterministic test avoids relying on a race between worker completion and wait-set construction:

1. first client is scheduled with capacity one;
2. capacity is observed full and no completion wake is pending;
3. first client is closed;
4. Phase 091 waits with no second client queued and receives the completion wake;
5. released capacity is observed;
6. only then is a second client connected and a bounded request written;
7. a later finite Phase 092 invocation schedules exactly one second completion-wake worker;
8. the second response is validated;
9. the second completion wake returns capacity to zero.

This proves completion wake restores scheduling eligibility without introducing a polling cadence or busy retry.

## Shutdown ordering proof

`LocalLinuxRuntimeShutdownHandle::request_shutdown_and_wake()` commits monotonic `ShutdownRequested` state before posting runtime wake.

A focused test queues a client, requests shutdown and wake, then runs finite orchestration. It proves:

- shutdown is already committed before orchestration observes the wake boundary;
- orchestration returns `ShutdownObserved`;
- zero scheduling attempts occur;
- zero workers are registered;
- worker capacity remains zero;
- the queued client remains in the listener and can be accepted afterward by an explicit test operation.

This demonstrates that Phase 092 does not consume a queued client after terminal shutdown has been requested.

## At-capacity scheduling proof

A focused test holds the sole Phase 075 permit and queues a client, then directly invokes the runtime-specific bounded scheduling cycle with a larger attempt budget.

The cycle:

- performs exactly one scheduling attempt;
- returns `AtCapacity`;
- registers no worker;
- does not consume the queued client.

This preserves the Phase 084/086 capacity-before-accept invariant.

## Eventfd semantic separation

A focused test posts multiple runtime wake notifications and proves they may coalesce into one drainable eventfd state. The eventfd counter is not interpreted as worker-result count, scheduling-attempt count, or protocol data.

## Phase 086 deterministic test corrective

The Phase 092 preflight exposed a latent timing assumption in the existing Phase 086 test `reaps_finished_worker_before_next_schedule_attempt`.

The old test waited until `LocalLinuxWorkerCapacity::active_workers() == 0` and then assumed `ScopedJoinHandle::is_finished()` was necessarily already true. That implication is not guaranteed: the worker-owned capacity permit can be dropped before the scoped thread has completed the final transition observed by `ScopedJoinHandle::is_finished()`.

The production Phase 086/registry behavior was not changed.

A test-only observation helper was added under `#[cfg(test)]`:

`LocalLinuxScopedWorkerRegistry::has_finished_worker_for_test()`

The Phase 086 test now waits for the actual precondition used by `reap_finished()`—a finished scoped handle—then additionally verifies worker capacity is zero before invoking the cycle. This removes the timing proxy without changing production runtime semantics or public API.

## Preflight history

### Initial preflight — run `31900494375`

The first integration preflight stopped safely before commit because a test-fixture cleanup method borrowed `&self` while still attempting to move owned fixture fields. No runtime source was committed from that failed preflight.

### A01 — run `31900571690`

A YAML-safe corrective fixed the test-fixture cleanup and reached the full Rust test stage.

A01 established that:

- Phase 092 source compiled and passed Clippy;
- every new Phase 092 test passed;
- all Phase 089/090/091 tests passed;
- the only failure was the existing Phase 086 reap-first test described above.

### A02 — run `31900636680`

A02 applied the test-only deterministic Phase 086 correction and reran the complete validation chain.

A02 passed:

- `cargo metadata --locked --no-deps --format-version 1`;
- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --locked --workspace --all-targets`;
- `cargo build --locked --workspace --all-targets`;
- `git diff --check`.

The A02 source-mutation guard allowed exactly:

- `crates/prw-agent/src/linux_runtime_orchestration.rs`;
- `crates/prw-agent/src/linux_identity.rs`;
- `crates/prw-agent/src/linux_worker_registry.rs` — test-only observation helper;
- `crates/prw-agent/src/linux_bounded_scheduler_cycle.rs` — test-only deterministic wait correction.

Only after full PASS did A02 commit the integrated source as `4162b1ceeba14dbab8ef33cecd036f2e93e8b4d2`.

All Phase 092 temporary preflight workflows were removed in that integration commit. The repository currently contains only the permanent PRW Rust Validation workflow.

## Dependency and toolchain invariants

The repository remains pinned to exact Rust/Cargo `1.97.1`.

`Cargo.lock` remains byte-identical with blob SHA:

`76af6bd831191309ac904dfe02ef76729de9a4fb`

Phase 092 adds no package dependency and no Cargo feature change.

## Authoritative validation state

The integrated source has passed the complete A02 preflight, but Phase 092 is not yet classified `IMPLEMENTED_AND_VALIDATED` until the permanent PRW Rust Validation workflow passes on a commit containing the integrated source.

## Explicitly deferred

Phase 092 does not authorize or implement:

- a production long-running outer runtime loop;
- Agent `main.rs` runtime bootstrap;
- OS signal handling;
- systemd unit creation, installation, enablement, or start;
- production capacity/backlog/attempt-budget configuration;
- remote/public network listeners;
- authentication/enrollment changes;
- DNS/TUN/relay/database/private-key/deployment changes.

Those remain a new architecture/runtime approval boundary after Phase 092.
