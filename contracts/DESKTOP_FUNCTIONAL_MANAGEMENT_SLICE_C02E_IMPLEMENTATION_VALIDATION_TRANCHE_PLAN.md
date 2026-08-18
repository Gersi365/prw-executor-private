# Phase 152 C02e — Implementation Validation Tranche Plan

Status: `VALIDATION_PLAN_LOCK / TRANCHE_1_CURRENT_SOURCE_NO_DEPENDENCY_MUTATION / TRANCHE_2_ACTUAL_PHASE141_DEV_EDGE_SEPARATE / RUST_1_97_1 / EXECUTION_NOT_AUTHORIZED_BY_THIS_PLAN / BUILD_GATE_CLOSED / NO_NETWORK_IO`

Planning base head: `1a55e026c14e7a2fc755f7e6970bd9e6f2916331`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The C02e source/design gap review found no further runtime-neutral semantic gap under the current authority. The next implementation-phase work is executable validation, but validation must not be mixed with an optional Cargo dependency change or production runtime activation.

This plan separates those concerns into two independently auditable tranches.

The plan itself does not open the build/Cargo gate or execute any command/workflow.

## Authoritative validation baseline

Current repository state pins:

```toml
[toolchain]
channel = "1.97.1"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

Workspace lint policy includes:

```toml
[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
```

Prior authoritative implementation audits repeatedly use the gate sequence:

1. `cargo fmt --all -- --check`;
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
3. `cargo test --workspace --all-targets`;
4. `cargo build --workspace --all-targets`.

C02e must preserve that full-workspace baseline rather than treating focused tests as sufficient completion evidence.

## Tranche 1 — validate the current C02e source exactly as staged

### Objective

Validate the current C02e Rust/test source without changing any Cargo manifest, dependency edge or lockfile.

This tranche covers:

- `crates/prw-connectivity/src/lib.rs` transactional refresh/candidate-ID lifetime changes;
- private `crates/prw-remote-bridge/src/candidate_reachability.rs` publication/admission seams;
- existing C02e registry/provenance/semantic-adapter tests;
- `reachability_composition_reference.rs`;
- `reachability_freshness_authority_reference.rs`;
- `reachability_freshness_bootstrap_reference.rs`.

It does not import actual Phase 141 into `prw-remote-bridge` tests.

### Pre-execution audit

Immediately before execution, the future authorized validation must re-read:

- branch head;
- `rust-toolchain.toml`;
- root `Cargo.toml`;
- `Cargo.lock`;
- `crates/prw-remote-bridge/Cargo.toml`;
- C02d -> current C02e compare.

The validation must confirm that no Cargo manifest or lockfile is already part of the C02e source delta before commands run.

Record exact pre-run SHA-256 for at least:

- `Cargo.lock`;
- root `Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

### Focused validation sequence

Run under the repository-pinned Rust/Cargo 1.97.1 toolchain:

```bash
cargo fmt --all -- --check
cargo clippy -p prw-connectivity -p prw-remote-bridge --all-targets --all-features -- -D warnings
cargo test -p prw-connectivity --all-targets
cargo test -p prw-remote-bridge --all-targets
```

Focused failure handling:

- formatter-only differences may be corrected only by exact rustfmt output, with no semantic cleanup;
- compiler/Clippy/test failures must be classified as source defect vs environment/tooling failure;
- corrections must remain confined to the failing C02e source/test seam;
- no dependency upgrade, manifest edit, broad refactor or unrelated lint cleanup is authorized as an incidental fix.

After every source corrective, rerun the failed focused gate from the beginning of the affected sequence rather than recording a partial pass.

### Full-workspace authoritative validation

Only after focused C02e gates pass:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --all-targets
```

A Tranche 1 implementation-validation PASS requires all four full-workspace gates to pass on one clearly recorded validated head.

### Dependency-state invariant

Tranche 1 must not change dependency state.

After validation, verify exact pre/post hashes for the root manifest, `prw-remote-bridge` manifest and `Cargo.lock`.

Any unexpected mutation of those files is a validation failure requiring investigation; it must not be normalized or committed automatically.

### Required evidence

Record:

- starting head;
- every corrective commit if any;
- final validated head;
- `rustc --version` / `cargo --version` evidence showing the pinned toolchain;
- focused command results;
- full-workspace command results;
- pre/post manifest/lock hashes;
- source defects separately from tooling/environment failures;
- exact changed-path compare against frozen C02d after validation.

No runtime/network activity is necessary for Tranche 1.

## Tranche 2 — actual Phase 141 integration test edge

### Objective

Only after Tranche 1 is clean and only under a separately opened Cargo/dependency validation gate, replace the opaque test traversal lifecycle with an integration-test path that can instantiate actual Phase 141 `IceConnectivitySession` state.

This tranche is intentionally separate because it changes dependency metadata.

### Intended minimal manifest mutation

The reviewed future edge is only:

```toml
[dev-dependencies]
prw-nat-traversal = { path = "../prw-nat-traversal" }
```

inside `crates/prw-remote-bridge/Cargo.toml`, unless then-current Cargo validation proves a narrower equivalent or a required correction.

It must not become a normal production dependency merely for test convenience.

### Lockfile rule

Do not hand-edit `Cargo.lock`.

Use the pinned Cargo toolchain to materialize any required lockfile update, then inspect the exact diff.

Because `prw-nat-traversal` is already a workspace member with locked external dependencies, the expected change should not opportunistically upgrade unrelated crates.

Any unrelated version/resolution drift fails closed and requires a separate dependency review.

### Actual Phase 141 integration objectives

The new/updated integration test must prove using actual Sans-I/O traversal state that:

- one current `IceConnectivitySession` belongs to one accepted candidate-state lifecycle;
- a successful full candidate refresh invalidates that session even for an exact retained candidate;
- an old already-produced/queued `CandidateReachabilityUpdate` is rejected by upper lifecycle currentness before `PeerConnectivityPlan::set_observation(...)`;
- a replacement traversal session built from the refreshed current state may produce current observations;
- failed candidate refresh preserves the current traversal lifecycle;
- transport rotation invalidates old plan/traversal state;
- all test traffic remains synthetic/in-memory; no UDP socket or persistent STUN/ICE/TURN activity is activated.

### Tranche 2 validation

After the dev-edge and actual test source are staged, run focused manifest/dependency and test validation first, followed by the same authoritative full-workspace four-gate sequence.

Record exact pre/post manifest and lock hashes plus dependency diff evidence.

Tranche 2 must have its own validated head and audit; it must not overwrite Tranche 1 evidence.

## Not part of either tranche

Neither validation tranche authorizes:

- production freshness token/protocol selection;
- candidate wire codec/message kind;
- durable freshness database/replication/re-baselining;
- production upper-owner selection;
- async/thread/task architecture changes;
- socket/network adapter activation;
- production STUN/ICE/TURN traffic;
- QUIC migration/runtime wiring;
- Agent/bootstrap activation;
- deployment/signing/privileged system mutation.

Those remain separately gated work.

## Failure classification discipline

Future validation evidence must distinguish:

### Source defect

The staged C02e source violates Rust typing, lint policy, unit/integration behavior or existing workspace contracts.

Response: smallest source corrective, audit it, rerun relevant gates.

### Formatting-only defect

`rustfmt --check` reports deterministic formatting differences.

Response: apply exact formatter output only; do not bundle cleanup/refactor.

### Dependency-resolution defect

Only relevant to Tranche 2. Cargo cannot materialize the intended narrow dev edge without unrelated graph drift or version conflict.

Response: stop dependency mutation and review the graph; do not upgrade broadly.

### Environment/tooling failure

Runner/toolchain/system package/resource problem unrelated to source correctness.

Response: preserve evidence, fix/retry environment separately, and do not misclassify source as failed.

## Result

`C02E_VALIDATION_TRANCHE_PLAN_LOCKED / TRANCHE1_VALIDATES_CURRENT_SOURCE_WITH_ZERO_DEPENDENCY_MUTATION / TRANCHE2_OWNS_OPTIONAL_ACTUAL_PHASE141_DEV_EDGE_AND_LOCKFILE_VALIDATION / EXECUTION_REMAINS_SEPARATELY_AUTHORIZED / NO_RUNTIME_ACTIVATION`
