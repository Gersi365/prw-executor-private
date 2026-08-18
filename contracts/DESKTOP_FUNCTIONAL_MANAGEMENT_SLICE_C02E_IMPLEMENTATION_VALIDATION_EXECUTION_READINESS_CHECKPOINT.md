# Phase 152 C02e — Implementation Validation Execution Readiness Checkpoint

Status: `VALIDATION_EXECUTION_READINESS_LOCK / LOCKED_DEPENDENCY_GRAPH_REQUIRED / LOCKED_METADATA_PREFLIGHT_REQUIRED / CARGO_GATES_MUST_USE_LOCKED / NATIVE_RUNNER_PREREQUISITES_PRECHECK_REQUIRED / TRANCHE_BOUNDARIES_UNCHANGED / EXECUTION_NOT_AUTHORIZED / BUILD_GATE_CLOSED / NO_NETWORK_IO`

Readiness base head: `c5a8cd2db18bda189062a4bcbacbb9d886d29854`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The existing C02e implementation-validation tranche plan correctly separates current-source validation from the later optional Phase 141 dev-dependency tranche, but a current-head readiness review found that its command examples omit repository-wide locked-dependency enforcement that is already authoritative under Phase 056 and the standard Rust validation workflow.

This checkpoint corrects execution-readiness details only. It does not run validation, modify Cargo manifests or `Cargo.lock`, select a production freshness representation, add the Phase 141 dependency edge, or activate runtime/network behavior.

Historical validation-planning files remain unchanged. Where their command examples omit `--locked` or the locked metadata preflight, this checkpoint is the current execution authority.

## Authoritative repository precedent

The current repository contains an explicit lockfile CI policy:

1. once `Cargo.lock` exists, standard Rust CI fails closed if the committed lockfile cannot satisfy current workspace manifests;
2. validation runs `cargo metadata --locked --no-deps --format-version 1` before build-oriented gates;
3. Clippy runs with `--locked`;
4. tests run with `--locked`;
5. workspace build runs with `--locked`;
6. `cargo fmt` does not resolve dependencies and therefore does not require `--locked`.

The current standard Rust workflow implements exactly that policy.

C02e Tranche 1 has an even stronger stated invariant — zero dependency-state mutation — so omitting the repository's locked-dependency flags from a future validation run would weaken, rather than merely restyle, the planned gate.

## Toolchain baseline

The current repository pins:

```toml
[toolchain]
channel = "1.97.1"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

The workspace lint baseline remains:

```toml
[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
```

No alternative toolchain or lint profile is selected by this checkpoint.

## Runner/environment readiness

Full-workspace Rust validation includes the desktop workspace member, whose current manifest depends on GTK4 and libadwaita.

The standard Linux workflow therefore installs/records native prerequisites including:

- `pkg-config`;
- `libgtk-4-dev`;
- `libadwaita-1-dev`;
- `pkg-config --modversion gtk4`;
- `pkg-config --modversion libadwaita-1`.

A future authorized C02e validation may run on another equivalent prepared environment, but it must prove required native build prerequisites before classifying compiler/Clippy/test/build failures as source defects.

Missing native prerequisites are `EnvironmentToolingFailure`, not evidence that C02e source is invalid.

This checkpoint does not authorize package installation now.

## Tranche 1 pre-execution readiness sequence

Immediately before any future authorized execution:

1. re-read the branch head and require an explicitly recorded starting head;
2. re-read `rust-toolchain.toml`;
3. re-read root `Cargo.toml`;
4. re-read `Cargo.lock`;
5. re-read `crates/prw-remote-bridge/Cargo.toml`;
6. re-read the C02d -> current C02e compare;
7. confirm no Cargo manifest or lockfile belongs to the C02e source delta;
8. record SHA-256 for at least root `Cargo.toml`, `Cargo.lock`, and `crates/prw-remote-bridge/Cargo.toml`;
9. record `rustc --version`, `cargo --version`, `rustfmt --version`, and `cargo clippy --version`;
10. prove required native runner prerequisites for the selected validation environment;
11. execute the locked metadata preflight before any build-oriented Cargo gate.

The locked metadata preflight is:

```bash
cargo metadata --locked --no-deps --format-version 1 > /dev/null
```

Failure here is a dependency-state or environment/tooling failure. Do not allow Cargo to regenerate/normalize `Cargo.lock` and continue.

## Tranche 1 focused validation — corrected command authority

The focused current-source sequence is:

```bash
cargo fmt --all -- --check
cargo clippy --locked -p prw-connectivity -p prw-remote-bridge --all-targets --all-features -- -D warnings
cargo test --locked -p prw-connectivity --all-targets
cargo test --locked -p prw-remote-bridge --all-targets
```

`--locked` is mandatory on the Cargo gates because Tranche 1 forbids dependency-state mutation.

Focused validation remains diagnostic/locality evidence only; it does not replace the full-workspace gate.

## Corrective-loop rule

If a source corrective is required during a future authorized validation:

- preserve the exact failing command/output;
- apply only the smallest correction within the failing C02e source/test seam;
- do not edit manifests, dependency versions/features, or `Cargo.lock` incidentally;
- record the corrective commit;
- re-read the new head;
- re-check the three manifest/lock hashes;
- rerun `cargo metadata --locked --no-deps --format-version 1` before resuming build-oriented validation;
- restart the affected focused sequence from its first relevant gate.

A source corrective that changes dependency state is outside Tranche 1 and fails closed pending separate review.

## Full-workspace authoritative validation — corrected command authority

Only after focused gates are clean, the authoritative workspace sequence is:

```bash
cargo metadata --locked --no-deps --format-version 1 > /dev/null
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-targets
cargo build --locked --workspace --all-targets
```

A Tranche 1 PASS requires:

- the locked metadata preflight to pass;
- all four workspace gates to pass on one clearly identified final head;
- required native prerequisites to have been proven for the environment;
- root manifest, bridge manifest, and lockfile SHA-256 values to remain byte-stable from the accepted pre-run baseline.

Unexpected manifest/lock mutation is not a formatting artifact and must not be committed automatically.

## Tranche 2 relationship

The existing Tranche 2 boundary is unchanged.

Only under a separately opened Cargo/dependency gate may `prw-remote-bridge` gain the reviewed test-only `prw-nat-traversal` dev-dependency and allow pinned Cargo to materialize any required lockfile change.

For Tranche 2:

- lockfile materialization is an intentional reviewed dependency change rather than an incidental validation effect;
- the resulting dependency/lock diff must be inspected explicitly;
- unrelated resolution/version drift fails closed;
- subsequent validation again uses `--locked` against the newly committed/reviewed dependency state;
- Tranche 1 and Tranche 2 evidence remain separate.

This checkpoint does not add that dependency edge.

## Failure classification refinement

### `DependencyStateFailure`

Examples:

- `cargo metadata --locked` reports that current manifests cannot be satisfied by committed `Cargo.lock`;
- a supposedly zero-mutation Tranche 1 command changes dependency state;
- manifest/lock hashes differ unexpectedly.

Response: stop; preserve evidence; do not regenerate/commit lock state automatically.

### `EnvironmentToolingFailure`

Examples:

- wrong/missing Rust component;
- missing `pkg-config`, GTK4 or libadwaita native development prerequisites on the selected Linux runner;
- resource/runner failure unrelated to source semantics.

Response: preserve evidence and correct the environment separately; do not patch C02e source to compensate.

### `FormattingOnlyDefect`

Only deterministic `rustfmt --check` differences.

Response: exact formatter output only under an opened execution/corrective gate.

### `SourceDefect`

Rust typing, lint or test behavior fails with locked dependency state and a proven-ready environment.

Response: minimal C02e source/test corrective, followed by locked preflight and rerun.

## Evidence required when execution is eventually authorized

At minimum record:

- starting and final heads;
- exact tool versions;
- native prerequisite versions/readiness evidence;
- locked metadata preflight result;
- focused command results;
- full-workspace command results;
- all corrective commits;
- pre/post SHA-256 for root manifest, bridge manifest and `Cargo.lock`;
- exact C02d -> validated-C02e compare;
- explicit failure classification for every failed attempt.

Static source review remains distinct from this executable evidence.

## Supersession boundary

This checkpoint supersedes only the **execution command details and readiness preflight** in the earlier implementation-validation tranche plan/audit where they omit repository-wide locked dependency enforcement or native runner readiness.

It does not supersede:

- the two-tranche separation;
- Tranche 1 zero-dependency-mutation intent;
- Tranche 2's separately gated dev-edge;
- the four authoritative workspace gates;
- source-defect minimality rules;
- the closed production runtime/protocol/deployment boundaries.

## Not executed

No Cargo metadata command, Cargo resolution, package installation, rustfmt, compiler/type check, Clippy, tests, build, workflow dispatch, network I/O, STUN/ICE/TURN, QUIC activity, Agent/bootstrap activation, deployment, signing, privileged/system mutation, PR creation/merge, or Host Mirror synchronization is performed by this checkpoint.

## Result

`C02E_VALIDATION_EXECUTION_READINESS_LOCKED / PHASE056_LOCKFILE_POLICY_APPLIED / LOCKED_METADATA_PREFLIGHT_REQUIRED / CLIPPY_TEST_BUILD_LOCKED_REQUIRED / NATIVE_ENVIRONMENT_READINESS_REQUIRED / TRANCHE1_ZERO_DEPENDENCY_MUTATION_PRESERVED / TRANCHE2_STILL_SEPARATE / EXECUTION_GATE_STILL_CLOSED / C02D_UNTOUCHED`
