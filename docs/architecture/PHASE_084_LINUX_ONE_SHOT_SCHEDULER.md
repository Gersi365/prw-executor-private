# Phase 084 — Linux One-Shot Capacity-Gated Scheduler Transaction

Status: implementation payload awaiting CI validation

## Purpose

Compose the validated Linux runtime primitives into exactly one scheduling attempt without introducing an accept loop, runtime poller, or Agent bootstrap activation.

## Ordering

One call follows this fixed order:

1. acquire one Phase 075 worker permit;
2. if capacity is full, return `AtCapacity` **without touching the listener**;
3. perform exactly one Phase 070 authenticated accept attempt;
4. if no connection is queued, release the permit and return `NoConnectionReady`;
5. borrow the authenticated accepted connection and create its Phase 082 cancellation clone;
6. consume the Phase 070 outcome through the Phase 071 authenticated-session bridge;
7. spawn exactly one Phase 078 scoped worker using the Phase 076 worker configuration;
8. register the scoped handle together with the matching Phase 082 cancellation authority in the Phase 080 registry;
9. return `WorkerRegistered`.

## Policy boundary

The scheduler accepts only `prw_policy::BoundedLocalReadPolicy` from Phase 083. It does not accept an arbitrary `PolicyEvaluator` implementation.

This preserves the Phase 081 requirement that the initial runtime worker path cannot block inside an unreviewed external policy evaluator.

## Failure behavior

- capacity full: no accept syscall;
- no-ready: acquired permit is dropped;
- accept failure: permit and any accepted ownership are dropped;
- cancellation-clone failure: connection is not spawned and owned connection/permit are dropped;
- scoped-spawn failure: captured session/permit are disposed by the fallible spawn path and the local cancellation clone is dropped;
- registry mutation occurs only after successful scoped spawn.

No pre-registration failure leaves a half-registered worker.

## Scope

Phase 084 does not:

- loop or retry;
- reap existing workers automatically;
- issue shutdown/cancellation except in tests;
- choose production capacity/backlog/deadline/Request-count values;
- activate Agent bootstrap/systemd/service state;
- add remote networking.

## Validation target

CI must prove:

- capacity-full path does not consume a connection already queued in the kernel listener backlog;
- after capacity is released, that same queued connection can be scheduled and answered;
- no-ready releases the temporary worker permit and creates no registry entry;
- successful scheduling creates one registry entry backed by one active worker permit and one cancellation authority;
- cancellation/join of the scheduled worker returns capacity to zero;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
