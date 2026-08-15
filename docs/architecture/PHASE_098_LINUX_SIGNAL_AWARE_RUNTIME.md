# Phase 098 — Linux Signal-Aware Production Runtime

Status: `SIGNAL_SOURCE_VALIDATED / SIGNAL_AWARE_READINESS_INTEGRATED_AWAITING_AUTHORITATIVE_CI`

## Purpose

Phase 098 adds the safe synchronous Linux termination-signal boundary authorized by Phase 094-A01, while preserving workspace `unsafe_code = "forbid"`, the validated Phase 091 capacity-aware semantics, and the Phase 097 callable runtime below `main.rs`.

## Dependency baseline

Exact dependency:

```toml
nix = { version = "=0.31.3", default-features = false, features = ["signal"] }
```

Phase 098-A00 dependency audit commit:

`11967246f1f332746cbce8de67806bacb82bee89`

Controlled Cargo preflight run:

`31902253525`

Integrated dependency baseline:

`fc71e9ded88f3963f8000a1a5ec05d6b299e8745`

Permanent dependency-baseline validation:

`31902304269`

The lockfile delta was restricted to `nix 0.31.3`, `cfg-if 1.0.4`, `cfg_aliases 0.2.2`, and the `prw-agent` dependency edge to `nix`; existing locked package records remained unchanged.

## Safe termination signal source

Integrated signal-source commit:

`ce5c429cb8f0147fd1db950c27be52d729a92cca`

Permanent signal-source validation:

`31902722135`

`LocalLinuxTerminationSignalSource`:

- synchronously blocks exactly SIGTERM and SIGINT on the creating thread;
- preserves the exact previous calling-thread mask;
- creates one CLOEXEC + NONBLOCK `SignalFd` for the same mask;
- uses only safe nix APIs and preserves workspace `unsafe_code = "forbid"`;
- is deliberately thread-affine (`!Send`/`!Sync`) so mask restoration cannot migrate to another thread;
- classifies `SIGTERM` and `SIGINT` explicitly;
- surfaces nonblocking no-data and EINTR without hidden retry loops;
- explicitly closes `SignalFd` before restoring the previous mask;
- performs best-effort same-thread restoration during unwind/Drop.

### Signal-source preflight history

Initial run `31902474463` compiled against nix successfully but stopped at Clippy-only style/documentation findings. No nix API mismatch was reported.

A01 run `31902589850` corrected those mechanical findings and reached the real signal test. It exposed a test-isolation defect: process-directed `kill(Pid::this(), SIGTERM)` could be delivered to a pre-existing unblocked Rust test-harness thread before the test function had a chance to modify that thread's mask.

The test strategy was corrected rather than weakening production semantics. The isolated child test uses safe nix `raise(...)`, targeting the current test thread after its termination mask is installed. This avoids unrelated harness-thread delivery while still exercising the real kernel pending-signal -> `SignalFd` path.

A02 run `31902660895` passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and `git diff --check` before committing the signal source and deleting all three temporary signal-source workflows.

The child-process proof validates mask installation, worker-thread mask inheritance, SIGTERM/SIGINT SignalFd consumption, exact explicit mask restoration, and Drop restoration without contaminating the parent test harness.

## One-step signal-aware readiness

Integrated readiness commit:

`300cea789300b9356202bb240ba3049164067267`

The production-specific one-step readiness adapter preserves Phase 091 rather than replacing it.

Its poll set is exactly:

1. termination `SignalFd` — always armed;
2. Phase 089 runtime wake — always armed;
3. listener — armed only while worker capacity is available.

Semantic precedence for one kernel result is exactly:

1. termination signal;
2. runtime wake;
3. listener readiness.

A consumed termination signal commits monotonic scheduler shutdown before returning its typed readiness outcome, ensuring no simultaneous listener dispatch occurs after the signal wins precedence.

The wake branch preserves Phase 091 behavior: drain wake, reap finished workers, re-observe shutdown, then allow simultaneous listener dispatch only if capacity remains available.

The adapter contains no outer loop and surfaces poll EINTR as `WaitInterrupted` rather than hiding an internal syscall retry.

### Readiness preflight history

Initial readiness integration run `31902826688` reached compile/Clippy and stopped only on two mechanical findings: an elidable lifetime name and one helper eligible for `const fn`.

A01 run `31902865634` applied exactly those mechanical changes and passed the complete locked metadata/fmt/Clippy/test/build/diff-check chain before committing integrated readiness and deleting both temporary readiness workflows.

Permanent CI is now required on a commit containing `300cea789300b9356202bb240ba3049164067267` before the signal-aware long-running runtime wrapper is integrated.

## Remaining Phase 098 layer

After permanent readiness validation, Phase 098 may compose:

- the Phase 098 termination signal source;
- signal-aware one-step readiness;
- Phase 092 runtime-specific bounded scheduling;
- Phase 097 memory-bounded error/counter/teardown semantics;
- Phase 096 lifecycle cleanup;
- explicit signal-mask restoration after listener cleanup.

The resulting callable runtime must remain below `main.rs` and systemd.

## Boundary preserved

Phase 098 does not wire `main.rs`, define process exit mapping, activate systemd, deploy anything, or expose public networking. Those boundaries remain excluded throughout Phase 098.
