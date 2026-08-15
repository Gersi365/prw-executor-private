# Phase 095 — Linux Production Runtime Types

Status: `INTEGRATED_SOURCE_AWAITING_AUTHORITATIVE_CI_VALIDATION`

## Purpose

Phase 095 implements the first source layer authorized by the locked Phase 094-A01 production-runtime architecture decision. It defines immutable validated production-local runtime configuration, memory-bounded saturating evidence counters, terminal evidence types, and the initial connection-local versus fail-stop error-disposition classifier.

It does not start a listener, perform signal handling, add an outer runtime loop, change `main.rs`, or activate systemd.

## Integrated source

Integrated source commit:

`598a99378341f2800d81cc68ff6d98692fd54bde`

## Locked behavior represented in types

- explicit non-zero worker capacity;
- explicit non-zero listener backlog;
- explicit non-zero per-readiness scheduling attempt budget;
- finite per-worker request/read/write budgets;
- saturating process-lifetime counters rather than unbounded completion retention;
- same-UID peer-authorization rejection classified as connection-local continuation;
- other initial readiness/scheduling failures classified fail-stop;
- exact fatal readiness/scheduling classification retained;
- terminal reason preserved independently from listener/socket cleanup outcome;
- final cancellation and joined-completion vectors remain bounded by configured worker capacity.

## Preflight history

Initial integration run `31901374645` stopped safely at Clippy because three pure constructor/classifier functions were eligible for `const fn`.

Phase 095-A01 run `31901410245` changed only those three functions to `const fn` and then passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and `git diff --check` before committing the integrated source and deleting both temporary workflows.

## Boundary preserved

Phase 095 does not implement:

- production lifecycle resource assembly;
- a long-running readiness loop;
- OS signals;
- `main.rs` bootstrap;
- systemd installation/activation;
- deployment or public networking.

Permanent PRW Rust Validation is now required on a commit containing the integrated source before Phase 095 is classified `IMPLEMENTED_AND_VALIDATED`.
