# Phase 088 — Linux Runtime Readiness Architecture Decision

Status: `APPROVED_AND_LOCKED / IMPLEMENTATION_AUTHORIZED_BELOW_BOOTSTRAP_BOUNDARY`

Approval source: explicit user continuation after the Phase 088 architecture gate was presented on 2026-08-15.

## Scope

Phase 088 locks the Linux local-runtime readiness architecture used to build the next finite implementation layers after the validated Phase 086 bounded scheduling cycle.

This decision authorizes implementation of the readiness/wake primitives and finite orchestration proofs below the production Agent bootstrap boundary. It does **not** authorize `main.rs` activation, a production long-running Agent loop, signal/service-manager wiring, systemd installation/enablement/start, remote/public listeners, deployment, or unrelated product/runtime changes.

## Locked decision 1 — readiness model

Use a capacity-aware blocking `poll` model.

- Exactly one outer readiness owner consumes readiness.
- The runtime wake descriptor is always armed for readable events.
- The accept-ready listener is armed only while `active_workers() < max_workers()`.
- Normal readiness waiting uses no periodic timeout.
- No initial `epoll` object, async runtime, or separate listener-management thread is introduced.

The listener MUST NOT remain in the wait set solely because a connection is queued while worker capacity is full.

## Locked decision 2 — shared runtime wake transport

Use one Linux `eventfd` as a coalescing wake transport.

Semantic state remains elsewhere:

- `LocalLinuxSchedulerControl` owns monotonic shutdown state;
- `LocalLinuxWorkerCapacity` owns worker-capacity accounting;
- `LocalLinuxScopedWorkerRegistry` owns worker completion/cancellation/join state;
- the eventfd carries no semantic payload.

Wake producers are:

- terminal shutdown request handling; and
- scoped worker completion.

The single outer readiness owner is the only wake consumer/drainer.

## Locked decision 3 — eventfd configuration

Create the eventfd with:

- initial counter `0`;
- `CLOEXEC`;
- `NONBLOCK`;
- no `SEMAPHORE` flag.

Multiple notifications may coalesce. The numeric eventfd counter MUST NOT be interpreted as worker count, shutdown count, queued-client count, scheduling attempts, request IDs, or protocol data.

## Locked decision 4 — wake write semantics

A wake write uses exactly:

```text
1_u64.to_ne_bytes()
```

with one fixed 8-byte write operation.

Result policy:

- `Ok(8)` => wake queued;
- `Err(EAGAIN)` => wake already pending and therefore notification intent is satisfied;
- `Err(EINTR)` => retry only that same fixed write operation until it succeeds or returns a non-`EINTR` result;
- any short/non-8-byte successful result => invariant failure;
- any other errno => bounded notifier failure.

This EINTR-only retry is syscall completion, not a scheduler/accept retry and must not contain sleep or cadence.

Worker-teardown notification must never panic.

## Locked decision 5 — wake drain semantics

When runtime-wake `POLLIN` is reported, the single consumer performs one successful non-semaphore 8-byte eventfd read.

- `Err(EINTR)` retries only that same drain operation.
- `Ok(8)` is decoded with `u64::from_ne_bytes`.
- The decoded value must be nonzero.
- Decoded `u64::MAX`, short I/O, `EAGAIN` after reported `POLLIN`, or other errno are fail-closed structured errors.
- A successful drain is followed by re-observation of shutdown, worker completions, and capacity.

No read-until-empty loop is required for ordinary user-space posts because a non-semaphore eventfd read drains the current aggregate counter.

## Locked decision 6 — poll semantics

The normal one-step wait calls blocking `poll(..., None)`.

- `Ok(n > 0)` => inspect every returned `revents` value before dispatch.
- `Ok(0)` => invalid/unexpected readiness state for the no-timeout path.
- `Err(EINTR)` => return structured `WaitInterrupted`; do not retry `poll` internally.
- Any other poll error => bounded `WaitFailed` outcome/error.

Descriptor error readiness (`POLLERR`, `POLLHUP`, `POLLNVAL`) fails closed.

## Locked decision 7 — simultaneous readiness precedence

If runtime wake and listener readiness are reported together:

1. process and drain runtime wake first;
2. re-observe shutdown;
3. reap finished workers and accumulate completion evidence;
4. re-observe capacity;
5. only if shutdown is false and capacity is available may the listener readiness be dispatched to the existing Phase 086 bounded scheduling cycle.

Phase 086 retains its own shutdown check before Phase 084, preserving a second fail-closed gate.

Array ordering returned by `poll` MUST NOT determine semantic precedence.

## Locked decision 8 — worker completion notification placement

Do not add eventfd I/O to `LocalLinuxWorkerPermit::drop()`.

Use a runtime-specific completion wake guard in the scoped worker wrapper frame around `run_authenticated_session_worker(...)`.

The ordering requirement is:

1. worker-body-owned state, including the Phase 075 capacity permit, is released during normal return or panic unwinding;
2. the wrapper-level completion guard then attempts the runtime wake;
3. notification is non-panicking even during unwind.

Focused tests must prove that capacity has already been released when wake-driven readiness recomputation occurs.

## Locked decision 9 — completion evidence ownership

Runtime-wake handling immediately calls `LocalLinuxScopedWorkerRegistry::reap_finished()` and accumulates the resulting `LocalLinuxScopedWorkerCompletion` evidence at the finite orchestration layer.

This avoids retaining already-finished scoped handles indefinitely while the runtime is otherwise idle.

Final shutdown still performs:

1. `cancel_all()`;
2. `join_all()`;
3. classification of all residual completions;
4. scope exit only after no scoped worker handle remains unjoined.

## Locked decision 10 — scoped-thread lifetime

All scoped session workers remain structurally inside one controlled `std::thread::scope` lifetime.

- `LocalLinuxScopedWorkerRegistry` remains inside that scope.
- No `ScopedJoinHandle` may detach or escape the scope.
- Wake objects borrowed/shared by workers must outlive all worker guards.
- Shutdown cancellation and joining complete before scope exit.

## Locked decision 11 — shutdown request ordering

A future shutdown handle must preserve:

1. set `LocalLinuxSchedulerControl` to `ShutdownRequested`;
2. notify runtime wake;
3. outer readiness owner wakes;
4. wake processing precedes listener dispatch;
5. no new Phase 086 cycle starts after shutdown is observed;
6. registered workers are cancelled then joined;
7. listener/socket lifecycle is explicitly cleaned;
8. instance-lock/runtime-directory ownership is released according to existing lifecycle contracts.

Wake-before-state ordering is forbidden.

The production signal/service-manager source of shutdown remains a later bootstrap decision.

## Locked decision 12 — implementation decomposition

Implementation proceeds as separately validated layers.

### Phase 089 — runtime wake primitive

- enable only rustix feature `event`;
- implement owned/shared eventfd wake transport;
- implement exact create/notify/drain semantics and deterministic failure classification tests;
- no listener, worker integration, outer loop, or `main.rs` wiring.

### Phase 090 — scoped worker completion wake

- add the completion wake guard at the existing scoped worker wrapper boundary;
- prove normal completion and panic/unwind wake behavior;
- prove capacity is released before notification observation;
- no listener wait loop or `main.rs` wiring.

### Phase 091 — one-step capacity-aware readiness wait

- borrow `AcceptReadyAgentSocket` through `AsFd`;
- construct dynamic poll set from capacity;
- enforce wake-first precedence and structured outcomes;
- no long-running loop or bootstrap wiring.

### Phase 092 — finite readiness + Phase 086 orchestration proof

- compose the one-step readiness primitive and Phase 086 only in a caller-bounded orchestration/test boundary;
- prove the full-capacity + queued-client state blocks instead of spinning;
- prove completion wake restores listener eligibility;
- prove shutdown causes zero post-shutdown accepts;
- still no production long-running runtime loop.

## Dependency/lockfile invariant

`prw-agent` remains pinned to `rustix = 1.1.4`.

The Phase 089 feature addition is limited to enabling rustix feature `event`. The dependency package set is expected to remain unchanged; `Cargo.lock` must remain byte-identical. Any lockfile change caused by Phase 089 is a stop-and-audit condition.

## Validation requirements

Every source-changing implementation phase must pass the permanent GitHub Rust validation workflow under exact Rust/Cargo `1.97.1`:

- `cargo metadata --locked --no-deps --format-version 1`;
- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --locked --workspace --all-targets`;
- `cargo build --locked --workspace --all-targets`.

Focused phase tests must additionally prove the semantics locked above.

## Explicitly deferred

Phase 088 does not authorize:

- a production infinite/long-running Agent event loop;
- `main.rs` runtime activation;
- production capacity/backlog/attempt-budget configuration;
- OS signal handling;
- systemd unit creation, installation, enablement or start;
- remote/public network listeners;
- auth/enrollment changes;
- DNS/TUN/relay/database/private-key/deployment changes.

Those remain separate future approval boundaries.
