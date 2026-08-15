# Phase 102 — Linux Agent Standalone Binary Bootstrap

Status: `BOOTSTRAP_SURFACE_INTEGRATED / AWAITING_PERMANENT_CI_AND_BINARY_SUBPROCESS_PROOFS`

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

A03 run `31904436866` applied the accumulated mechanical corrections and passed:

- locked metadata;
- rustfmt;
- Clippy with `-D warnings`;
- all workspace/all-target tests;
- all workspace/all-target builds;
- `git diff --check`.

Only after that full PASS did the workflow commit `937927fd2f6e6c9a71937769e061ee10734a1de8` and remove all four temporary surface-integration workflows.

## Required remaining Phase 102 evidence

The surface is not yet classified `IMPLEMENTED_AND_VALIDATED`.

Before Phase 102 closes, controlled standalone-binary tests must still prove:

1. valid temporary XDG runtime-root startup;
2. real SIGTERM -> success exit -> no stale socket;
3. real SIGINT -> success exit -> no stale socket;
4. one same-UID bounded local read request with correlated successful response;
5. deterministic second-instance rejection while the first instance remains functional;
6. invalid/missing runtime-root failure before usable listener activation;
7. bounded stderr contract;
8. no systemd/install dependency in any proof.

A permanent PRW Rust Validation run on the integrated surface is also required before those binary-behavior proofs are treated as the next authoritative checkpoint.

## Boundary preserved

Phase 102 has not created or modified a systemd unit, installed or enabled a service, started an Agent on a user host, exposed remote/public networking, added a privileged helper, enabled TUN/relay/database work, made private DNS mandatory, or reintroduced Wake-on-LAN.
