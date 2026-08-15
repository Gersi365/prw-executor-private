# Phase 098 — Linux Signal-Aware Production Runtime

Status: `DEPENDENCY_BASELINE_INTEGRATED_AWAITING_AUTHORITATIVE_CI_VALIDATION`

## Purpose

Phase 098 adds the safe synchronous Linux termination-signal boundary authorized by Phase 094-A01, while preserving workspace `unsafe_code = "forbid"`, the validated Phase 091 capacity-aware semantics, and the Phase 097 callable runtime below `main.rs`.

Signal runtime source is not yet integrated at this status point. This first checkpoint records the dependency precheck and exact Cargo graph baseline before signal-source implementation.

## Primary-source dependency precheck

Phase 098-A00 audit:

`logs/audits/phase-098-linux-signal-aware-runtime/PRW-PHASE-098-A00-LINUX-SIGNAL-DEPENDENCY-PRECHECK.txt`

Audit commit:

`11967246f1f332746cbce8de67806bacb82bee89`

The selected exact candidate is:

```toml
nix = { version = "=0.31.3", default-features = false, features = ["signal"] }
```

Published nix 0.31.3 metadata documents Rust MSRV 1.69, MIT licensing, empty default features, and `signal` implying `process`. Its safe `SigSet`, `SignalFd`, `SfdFlags`, `read_signal()`, and `AsFd` APIs cover the Phase 094-A01 design without local unsafe code or direct libc FFI.

## Controlled Cargo preflight

Temporary preflight run:

`31902253525`

Integrated dependency baseline commit:

`fc71e9ded88f3963f8000a1a5ec05d6b299e8745`

The preflight temporarily added the exact manifest dependency, resolved the graph with repository-pinned Cargo 1.97.1, and rejected any existing-package lockfile churn.

The actual added locked packages were exactly:

- `nix 0.31.3`;
- `cfg-if 1.0.4`;
- `cfg_aliases 0.2.2`.

Existing locked package records remained unchanged. `prw-agent` gained exactly one dependency entry: `nix`.

The resulting nix lock entry depends on the already-existing compatible `bitflags` and `libc` packages plus the newly locked `cfg-if` and `cfg_aliases` packages.

After dependency resolution, the preflight passed:

- `cargo metadata --locked --no-deps --format-version 1`;
- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --locked --workspace --all-targets`;
- `cargo build --locked --workspace --all-targets`;
- `git diff --check`.

Only after full PASS did the workflow commit `Cargo.lock` and `crates/prw-agent/Cargo.toml`, then self-delete.

## Signal source requirements for the next Phase 098 step

The source implementation may proceed only after permanent CI validates the dependency baseline.

The source must:

- synchronously block exactly SIGTERM and SIGINT on the runtime thread;
- preserve the prior calling-thread signal mask;
- create a CLOEXEC + NONBLOCK `SignalFd` for the same mask;
- expose it through safe `AsFd`/`read_signal()` APIs only;
- establish the mask before the Phase 097 worker scope creates threads;
- preserve signal > runtime-wake > listener precedence;
- suppress listener interest at full worker capacity exactly as Phase 091 does;
- restore the prior thread mask after listener/socket cleanup;
- best-effort restore on unwind;
- preserve original runtime/lifecycle terminal evidence independently from mask-restoration failure;
- validate real SIGTERM/SIGINT delivery only inside an isolated subprocess test boundary.

## Boundary preserved

This dependency checkpoint does not yet implement signal masking, signalfd polling, signal-aware runtime iteration, `main.rs` bootstrap, process exit mapping, systemd activation, deployment, or public networking.

Permanent PRW Rust Validation is required before signal runtime source integration proceeds.
