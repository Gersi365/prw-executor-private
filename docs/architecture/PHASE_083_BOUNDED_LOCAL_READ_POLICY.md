# Phase 083 — Bounded In-Memory Local Read Policy

Status: implementation payload awaiting CI validation

## Purpose

Provide the concrete bounded/non-blocking `PolicyEvaluator` required by Phase 081 before any Agent accept-and-spawn scheduler is authorized.

## Model

`BoundedLocalReadPolicy` is a small value type in `prw-policy` containing exactly two decisions:

- `AgentStatusRead`;
- `PrivateDnsConfigRead`.

Evaluation is a fixed `match` over the requested capability:

- the two local read capabilities return their stored decision;
- every other represented capability returns `Deny`.

The evaluator performs no allocation, filesystem/network I/O, synchronization, callbacks, dynamic dispatch beyond the existing trait call, or external state lookup.

## Security interpretation

This type does **not** authenticate a principal. Runtime code may select/use it only after the relevant transport/principal boundary has been established according to higher-level contracts.

For the initial local same-UID Agent scheduler, this is the only concrete policy implementation authorized for worker binding. The generic `PolicyEvaluator` trait remains available to non-runtime/test code, but arbitrary potentially blocking implementations are not authorized for initial runtime orchestration.

## Scope

Phase 083 does not:

- accept a connection;
- spawn a worker;
- select default Allow permissions;
- change device/user enrollment;
- activate Agent bootstrap/systemd/service state.

## Validation target

CI must prove:

- both local read capabilities are independently configurable;
- all other capabilities fail closed to `Deny`;
- deny-all and allow-both constructors/state are deterministic;
- the evaluator is `Copy + Send + Sync`;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
