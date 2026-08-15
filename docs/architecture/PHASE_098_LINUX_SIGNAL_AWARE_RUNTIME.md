# Phase 098 — Linux Signal-Aware Production Runtime

Status: `DEPENDENCY_BASELINE_VALIDATED / SIGNAL_RUNTIME_SOURCE_IN_PROGRESS`

## Purpose

Phase 098 adds the safe synchronous Linux termination-signal boundary authorized by Phase 094-A01, while preserving workspace `unsafe_code = "forbid"`, the validated Phase 091 capacity-aware semantics, and the Phase 097 callable runtime below `main.rs`.

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

After dependency resolution, the preflight passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and `git diff --check`. Only after full PASS did the workflow commit `Cargo.lock` and `crates/prw-agent/Cargo.toml`, then self-delete.

## Authoritative dependency-baseline validation

Permanent PRW Rust Validation run:

`31902304269`

Validated dependency/documentation head:

`a2b8343cd136d7a9eda9eb49c1339749bcb827ad`

The permanent workflow passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, and all workspace/all-target builds.

The dependency baseline is therefore authoritative and signal runtime source integration may proceed.

## Signal source requirements

The source implementation must:

- synchronously block exactly SIGTERM and SIGINT on the runtime thread;
- preserve the prior calling-thread signal mask;
- create a CLOEXEC + NONBLOCK `SignalFd` for the same mask;
- expose it through safe `AsFd`/`read_signal()` APIs only;
- remain thread-affine so mask restoration cannot migrate to another thread;
- establish the mask before the Phase 097 worker scope creates threads;
- preserve signal > runtime-wake > listener precedence;
- suppress listener interest at full worker capacity exactly as Phase 091 does;
- restore the prior thread mask after listener/socket cleanup;
- best-effort restore on unwind;
- preserve original runtime/lifecycle terminal evidence independently from mask-restoration failure;
- validate real SIGTERM/SIGINT delivery only inside an isolated subprocess test boundary.

## Boundary preserved

The dependency checkpoint itself does not wire `main.rs`, define process exit mapping, activate systemd, deploy anything, or expose public networking. Those boundaries remain excluded throughout Phase 098.
