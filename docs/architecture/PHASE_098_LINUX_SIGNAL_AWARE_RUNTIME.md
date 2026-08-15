# Phase 098 — Linux Signal-Aware Production Runtime

Status: `SIGNAL_SOURCE_INTEGRATED_AWAITING_AUTHORITATIVE_CI_VALIDATION`

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

## Signal-source preflight history

Initial run `31902474463` compiled against nix successfully but stopped at Clippy-only style/documentation findings. No nix API mismatch was reported.

A01 run `31902589850` corrected those mechanical findings and reached the real signal test. It exposed a test-isolation defect: process-directed `kill(Pid::this(), SIGTERM)` could be delivered to a pre-existing unblocked Rust test-harness thread before the test function had a chance to modify that thread's mask.

The test strategy was corrected rather than weakening production semantics. The isolated child test now uses safe nix `raise(...)`, which targets the current test thread after its termination mask is installed. This avoids unrelated harness-thread delivery while still exercising the real kernel pending-signal -> `SignalFd` path.

A02 run `31902660895` passed locked metadata, rustfmt, Clippy with `-D warnings`, all workspace/all-target tests, all workspace/all-target builds, and `git diff --check` before committing the signal source and deleting all three temporary signal-source workflows.

The real child-process proof validates:

- the creating thread has SIGTERM/SIGINT blocked;
- a subsequently created child thread inherits that blocked mask;
- thread-directed SIGTERM becomes readable from `SignalFd` and decodes as `SigTerm`;
- thread-directed SIGINT becomes readable and decodes as `SigInt`;
- explicit restore returns the exact original mask;
- Drop-based restoration also returns the exact original mask;
- the parent parallel test harness is not left with mutated signal-mask state.

## Next Phase 098 source layer

After permanent CI validates this signal-source checkpoint, Phase 098 may implement tri-source readiness and the signal-aware callable runtime wrapper with semantic precedence:

1. termination signal;
2. Phase 089 runtime wake;
3. listener readiness.

Capacity-aware listener suppression from Phase 091 must remain unchanged.

## Boundary preserved

The signal-source checkpoint does not wire `main.rs`, define process exit mapping, activate systemd, deploy anything, or expose public networking. Those boundaries remain excluded throughout Phase 098.
