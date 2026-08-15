# Phase 102 — Linux Agent Standalone Binary Bootstrap

Status: `BINARY_SUBPROCESS_PROOFS_INTEGRATED / AWAITING_AUTHORITATIVE_CI`

## Locked predecessor

Phase 101 bootstrap-contract lock:

`f6334af4618429390cf32fa91a789614c9ad7ad2`

Phase 101 authorizes standalone binary wiring only. Systemd packaging/activation and deployment remain separately gated.

## Integrated bootstrap surface

Integrated source commit:

`937927fd2f6e6c9a71937769e061ee10734a1de8`

Phase 102 adds one narrow Linux-only public facade:

`prw_agent::linux_bootstrap`

The existing internal Linux implementation remains crate-private:

```rust
#[cfg(target_os = "linux")]
pub mod linux_bootstrap;

#[cfg(target_os = "linux")]
pub(crate) mod linux_identity;
```

The binary therefore does not expose the Phase 057–098 lifecycle, socket, readiness, signal, scheduling, or worker implementation graph as public API.

## Fixed first bootstrap profile

The facade encodes the Phase 101 locked initial profile:

- worker capacity: 2;
- Unix listener backlog: 8;
- scheduling-attempt budget: 2;
- request budget per session: 1;
- absolute read I/O budget: 2 seconds;
- absolute write I/O budget: 2 seconds;
- `BoundedLocalReadPolicy::allow_local_reads()`;
- immutable `Ready` status snapshot;
- `PrivateDnsConfig::default()` projected into the bounded local snapshot.

No configuration-file parser, CLI override, environment-based capability expansion, live reload, system-DNS mutation, remote listener, or service-manager behavior is introduced.

## Narrow bootstrap result model

The public facade maps internal runtime evidence into bounded bootstrap-level classifications only.

Terminal classes:

- programmatic shutdown;
- SIGTERM;
- SIGINT;
- readiness fatal;
- runtime fatal.

Startup classes include bounded categories for signal source, XDG/runtime-directory, instance-lock/AlreadyRunning, bind/listen/accept-ready, and runtime-wake failures.

The facade also exposes fixed-size lifetime counters, listener cleanup classification, and signal-mask restoration classification without exposing internal worker/error object graphs.

## Thin binary adapter

`crates/prw-agent/src/main.rs` now owns only:

1. one call to `prw_agent::linux_bootstrap::run()`;
2. one bounded stderr startup-failure or terminal-summary record;
3. the Phase 101 simple success/failure `ExitCode` mapping.

It does not implement socket creation, signal handlers, worker threads, runtime loops, configuration parsing, systemd operations, private-DNS mutation, or public/remote networking.

Success requires a normal handled terminal class plus clean listener/socket cleanup plus restored signal mask. Every startup/runtime fatal class, cleanup failure, or signal-mask restoration failure maps to failure.

## Surface-integration validation history

Initial workflow run `31904318295` stopped safely before integration because Rust 1.97.1 did not permit the staged `Result::expect` calls inside the proposed `const fn`, and one focused test import was missing.

A01 run `31904357017` corrected those compile-only findings, then stopped at two Clippy cleanliness findings: needless pass-by-value and explicit default-type access.

A02 run `31904396246` corrected those findings and stopped only because Clippy identified one mapper as eligible for `const fn`.

A03 run `31904436866` applied the accumulated mechanical corrections and passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and `git diff --check`.

Only after that full PASS did the workflow commit `937927fd2f6e6c9a71937769e061ee10734a1de8` and remove all four temporary surface-integration workflows.

Permanent surface checkpoint `31904558606` independently passed the permanent PRW Rust Validation workflow.

## Standalone binary subprocess proof

Integrated proof commit:

`a2dc8574b4df8e7df8e54312a0d5e6be50475594`

Proof source:

`crates/prw-agent/tests/phase_102_binary_bootstrap.rs`

The proof runs the actual Cargo-built `prw-agent` executable in controlled child processes. It does not call the bootstrap facade directly as a substitute for the binary boundary.

One sequential integration test proves six isolated scenarios with unique temporary XDG runtime roots:

1. **SIGTERM** — the real Agent socket becomes connectable, SIGTERM is delivered to the child PID, the binary exits successfully, stderr reports `terminal=sigterm`, cleanup is clean, signal-mask restoration is restored, and the socket pathname is absent after exit.
2. **SIGINT** — the same binary-level proof is repeated for SIGINT.
3. **Real same-UID local request** — the test sends a real `GetAgentStatus` frame through the public request writer, reads the real status response through the public response decoder, proves request correlation and `Ready` runtime state, then terminates cleanly.
4. **Second-instance exclusion** — a first Agent remains live; a second Agent against the same XDG runtime root exits failure with `kind=already_running`; the first Agent still answers a real status request and then shuts down cleanly.
5. **Missing runtime root** — an Agent child with `XDG_RUNTIME_DIR` removed fails startup with `kind=runtime_root`, failure exit, and restored signal mask.
6. **Wrong-mode runtime root** — a temporary root changed to mode `0755` fails before usable listener activation, reports `kind=runtime_root`, and leaves no Agent socket pathname.

The test also enforces the Phase 101 bounded stderr contract: each child outcome emits exactly one stderr record and the validated terminal record excludes request payload output.

### Proof-workflow corrective history

Initial proof workflow run `31904663853` encountered runner toolchain-provisioning failures before Cargo/Clippy/test diagnostics when `cargo fmt` was the first Rust invocation. Two reruns reproduced the same infrastructure-only failure. The permanent workflow on the same source successfully provisioned exact Rust/Cargo 1.97.1 by recording tool versions first, proving the source/toolchain pin was not defective.

A01 run `31904782482` adopted the proven toolchain-record order and reached proof generation. It then exposed a workflow mutation-guard defect: a newly generated integration test is untracked, while `git diff --name-only` lists tracked changes only. No Rust/test failure occurred on that run.

A02 run `31904847101` corrected the guard to require zero tracked mutations and exactly one expected untracked test file. It reached real Clippy and stopped only because `AgentChild::child_mut` was eligible for `const fn`.

A03 run `31904884041` applied only that mechanical Clippy correction and passed the complete chain:

- exact pinned toolchain record;
- locked metadata;
- rustfmt;
- workspace/all-target Clippy with `-D warnings`;
- focused standalone binary subprocess proof with `--test-threads=1`;
- all workspace/all-target tests;
- all workspace/all-target builds;
- `git diff --check`.

Only after that complete PASS did the workflow commit `a2dc8574b4df8e7df8e54312a0d5e6be50475594` and delete all four temporary binary-proof workflows.

The repository workflow directory now contains only the permanent `.github/workflows/phase-001-rust-validation.yml` workflow.

## Authoritative validation state

The bootstrap surface and binary subprocess proof are integrated. Phase 102 still requires one permanent PRW Rust Validation PASS on a commit containing `a2dc8574b4df8e7df8e54312a0d5e6be50475594` before classification as `IMPLEMENTED_AND_VALIDATED`.

That permanent run must therefore execute the committed `phase_102_binary_bootstrap` integration test as part of normal `cargo test --locked --workspace --all-targets` coverage.

## Boundary preserved

Phase 102 has not created or modified a systemd unit, installed or enabled a service, started an Agent on a user host, exposed remote/public networking, added a privileged helper, enabled TUN/relay/database work, made private DNS mandatory, or reintroduced Wake-on-LAN.
