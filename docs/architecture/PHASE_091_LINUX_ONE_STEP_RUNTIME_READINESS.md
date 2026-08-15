# Phase 091 — Linux One-Step Runtime Readiness

Status: `IMPLEMENTED_AND_VALIDATED`

## Purpose

Phase 091 implements one finite capacity-aware blocking readiness wait below the Agent bootstrap boundary.

It composes only already-locked Phase 088 readiness semantics with the validated Phase 089 runtime wake, Phase 090 completion-wake model, Phase 075 capacity accounting, Phase 080/082 registry ownership, and Phase 085/086 shutdown control.

## Implemented boundary

`wait_once_for_linux_runtime_readiness(...)` performs at most one `poll(..., None)` call.

Before blocking it:

- returns immediately if terminal shutdown is already requested;
- always arms the Phase 089 runtime-wake descriptor for `PollFlags::IN`;
- arms the Phase 070 accept-ready listener only while `active_workers() < max_workers()`.

After `poll` returns it:

- validates the ready count against observed `revents` state;
- fails closed on `ERR`, `HUP`, `NVAL`, unsupported readiness flags, invalid ready counts, or unexpected wake drain state;
- surfaces `EINTR` as a structured `WaitInterrupted` outcome rather than retrying `poll` internally;
- processes runtime wake before simultaneous listener readiness;
- drains wake, calls `registry.reap_finished()`, then re-observes shutdown and capacity;
- returns listener readiness only while shutdown remains false and capacity remains available.

## Busy-loop prevention

The listener is not present in the `poll` set while worker capacity is full.

A focused real-kernel test queues a client while holding the sole worker permit, posts one runtime wake, and proves the one-step wait reports:

- runtime wake;
- `listener_armed == false`.

After releasing the permit, a second finite invocation proves that the still-queued client makes the listener eligible/readable again.

This directly addresses the full-capacity queued-listener spin identified by Phase 087-A01.

## Wake-first precedence

A focused test makes both eventfd and listener readable before the wait. The result exposes listener readiness only after the wake has been drained. The test confirms the eventfd is empty afterward.

## Completion evidence

Wake handling immediately calls `LocalLinuxScopedWorkerRegistry::reap_finished()` and returns any observed `LocalLinuxScopedWorkerCompletion` values in the Phase 091 report.

This call is an observation point, not a claim that the particular thread which emitted a completion wake is guaranteed to have reached `ScopedJoinHandle::is_finished()` before the outer owner wakes. Final completion ownership remains bounded by later Phase 086 scheduling/reaping or shutdown `join_all()` semantics.

## Poll API

Phase 091 uses rustix 1.1.4:

- `PollFd::new`;
- `PollFlags::{IN, ERR, HUP, NVAL}`;
- `poll(&mut fds, None)`;
- `PollFd::revents()`.

## Authoritative validation

Validated source/trigger head:

`a07bb5f3fffebe0e4f9951fe21e9c3df5acd3781`

Semantic corrective source head incorporated by that validation:

`b783686affc6889913a3221b9026af07c07bd0a0`

Authoritative permanent GitHub Actions run:

`31900246130`

The run passed under exact Rust/Cargo 1.97.1:

- `cargo metadata --locked --no-deps --format-version 1`;
- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --locked --workspace --all-targets`;
- `cargo build --locked --workspace --all-targets`.

`Cargo.lock` remained byte-identical with blob SHA:

`76af6bd831191309ac904dfe02ef76729de9a4fb`

The net Phase 091 mutation surface from the Phase 090 authoritative report commit through the validated Phase 091 head contains only:

- `crates/prw-agent/src/linux_identity.rs`;
- `crates/prw-agent/src/linux_runtime_readiness.rs`;
- `docs/architecture/PHASE_091_LINUX_ONE_STEP_RUNTIME_READINESS.md`.

No temporary corrective workflow remains on `main`; only the permanent PRW Rust Validation workflow remains.

## Corrective history

Initial integrated run `31900029359` passed exact Rust/Cargo 1.97.1 toolchain resolution and locked metadata, then failed safely only at `cargo fmt --all -- --check`.

Temporary workflow `phase-091-rustfmt-fix.yml` ran `cargo fmt --all` with a guard that the only pre-self-delete source change was `crates/prw-agent/src/linux_runtime_readiness.rs`. Corrective run `31900068830` succeeded and produced formatter commit `3f3eb2b95b02a711584d8a4ab86910bfff53c8ee`; the workflow self-deleted in that commit.

Permanent validation run `31900113946` then passed toolchain, locked metadata and rustfmt and reached Clippy. Clippy reported only two compatibility/style findings: one elidable named lifetime and one private helper eligible for `const fn`.

Temporary workflow `phase-091-clippy-fix.yml` used exact single-occurrence text guards, restricted source mutation to `crates/prw-agent/src/linux_runtime_readiness.rs`, and applied only those two mechanical changes. Corrective run `31900151699` succeeded and produced commit `b783686affc6889913a3221b9026af07c07bd0a0`; the workflow self-deleted.

Final permanent run `31900246130` passed every required validation stage.

## Explicitly not implemented

Phase 091 does not add:

- a readiness loop;
- a scheduler loop;
- Phase 086 dispatch inside the wait primitive;
- `main.rs` activation;
- signal handling;
- systemd/service activation;
- remote/public network listeners;
- production runtime cadence/configuration;
- unrelated auth, DNS, TUN, relay, database, key, or deployment changes.

## Next bounded layer

Phase 092 may now compose the Phase 091 finite readiness step with a runtime-specific finite scheduling transaction that preserves the Phase 090 completion wake. Phase 092 remains below the production bootstrap boundary and must not introduce an unbounded outer runtime loop.
