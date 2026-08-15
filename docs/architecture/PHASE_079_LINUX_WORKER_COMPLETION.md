# Phase 079 — Linux Scoped Worker Completion Classification

Status: implementation payload awaiting CI validation

## Purpose

Ensure every explicitly joined Phase 078 scoped worker has one bounded terminal classification before a multi-worker registry or scheduler exists.

## Classification

`join_authenticated_session_worker` consumes exactly one scoped join handle and returns:

- `Stopped(LocalLinuxSessionWorkerStop)` for a normal finite Phase 076 stop;
- `WorkerError(LocalLinuxSessionWorkerError)` for a bounded Phase 076 processing failure;
- `Panicked` when the OS thread panicked before producing its worker result.

The panic payload itself is intentionally not part of the local runtime API surface.

## Scope

Phase 079:

- spawns no production worker;
- accepts no connection;
- owns no handle collection/registry;
- implements no scheduler;
- implements no shutdown signal;
- activates no Agent bootstrap or service.

## Validation target

CI must prove all three terminal classes using scoped test threads and preserve the full locked workspace validation gates.
