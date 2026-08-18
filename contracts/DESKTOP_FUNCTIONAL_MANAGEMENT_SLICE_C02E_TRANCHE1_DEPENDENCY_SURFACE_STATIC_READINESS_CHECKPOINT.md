# Phase 152 C02e — Tranche 1 Dependency-Surface Static Readiness Checkpoint

Status: `TRANCHE1_DEPENDENCY_SURFACE_LOCK / CURRENT_C02E_TEST_IMPORTS_ALREADY_DECLARED / NO_PHASE141_IMPORT_IN_TRANCHE1 / NO_MANIFEST_MUTATION_REQUIRED_FOR_CURRENT_SOURCE_SURFACE / PRIVATE_ADAPTER_PATH_INCLUDED_ONLY / EXECUTION_NOT_AUTHORIZED / BUILD_GATE_CLOSED / C02D_UNTOUCHED`

Readiness base head: `1284b47b1ecf6af38d59cb7c95df26d1c85c7850`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The C02e validation readiness checkpoint now locks `--locked` dependency behavior and runner prerequisites. This follow-up static review answers the narrower question needed before any future Tranche 1 execution:

**Does the current C02e source/test surface already fit the committed Cargo dependency declarations, or would compilation require an undeclared dependency mutation before validation can even begin?**

This checkpoint is source/manifest inspection only. It does not run Cargo, compile source, add a dev-dependency, modify `Cargo.lock`, or claim executable success.

## Current Tranche 1 source surface reviewed

The planned focused source/test surface is:

- `crates/prw-connectivity/src/lib.rs`;
- private `crates/prw-remote-bridge/src/candidate_reachability.rs`;
- `crates/prw-remote-bridge/tests/authenticated_candidate_provenance.rs`;
- `crates/prw-remote-bridge/tests/candidate_reachability_semantic_adapter.rs`;
- `crates/prw-remote-bridge/tests/dynamic_reachability_registry.rs`;
- `crates/prw-remote-bridge/tests/reachability_composition_reference.rs`;
- `crates/prw-remote-bridge/tests/reachability_freshness_authority_reference.rs`;
- `crates/prw-remote-bridge/tests/reachability_freshness_bootstrap_reference.rs`.

The current `prw-remote-bridge` and `prw-connectivity` manifests were reviewed against those source imports.

## `prw-connectivity` dependency closure

The current `crates/prw-connectivity/Cargo.toml` declares only:

```toml
[dependencies]
prw-core = { path = "../prw-core" }
```

The C02e connectivity changes remain inside the crate's existing standard-library + `prw-core` dependency boundary.

No C02e connectivity test/source surface reviewed here requires a new crate edge.

This is static import/declaration evidence only; actual Rust name/type/lint behavior remains executable evidence for Tranche 1.

## Private candidate adapter dependency closure

`crates/prw-remote-bridge/src/candidate_reachability.rs` remains private/unexported and directly imports only:

- standard library `fmt`;
- `prw-connectivity`;
- `prw-registry`;
- `prw-session`.

All three PRW crates are already normal `prw-remote-bridge` dependencies.

The adapter therefore does not require a new dependency edge merely to be compiled through its current integration-test path inclusion.

## Base C02e integration-test dependency closure

The three base C02e integration tests use combinations of:

- `aws-lc-rs`;
- `prw-connectivity`;
- `prw-control-plane`;
- `prw-core`;
- `prw-device-identity-signer`;
- `prw-registry`;
- `prw-session`.

The current `prw-remote-bridge` manifest already supplies:

### Normal dependencies

- `prw-connectivity`;
- `prw-registry`;
- `prw-session`;
- the existing unrelated production bridge dependencies.

### Dev dependencies

- `aws-lc-rs`;
- `prw-control-plane`;
- `prw-core`;
- `prw-device-identity-signer`.

Therefore the current base C02e tests do not require any additional manifest dependency.

## Test-only reference dependency closure

The three test-only upper-reference files use the same already-declared dependency family and include the private adapter with:

```rust
#[path = "../src/candidate_reachability.rs"]
mod candidate_reachability;
```

They do not import `prw_nat_traversal`, `IceConnectivitySession`, or any other Phase 141 type.

`reachability_composition_reference.rs` explicitly documents that it uses an opaque test-only traversal lifecycle marker specifically to avoid creating the Phase 141 dev-dependency while the Cargo/build gate is closed.

The freshness authority/bootstrap references likewise use test-local lifecycle/freshness markers, not an undeclared production or Phase 141 crate.

## Tranche 1 manifest invariant

Under the current source surface, a future authorized Tranche 1 must begin with the assumption:

`no Cargo.toml dependency change is required to compile the source/tests already staged`

That assumption is now grounded by static import/declaration review rather than convenience.

Therefore:

- do not add `prw-nat-traversal` before Tranche 1 merely because a later Tranche 2 intends to use it;
- do not promote any test-only dependency to a normal dependency to simplify validation;
- do not add duplicate direct dependencies that are not actually referenced by the current source/test surface;
- do not change dependency versions/features as a pre-validation cleanup;
- do not edit `Cargo.lock` during Tranche 1 preparation.

Any compiler failure that later demonstrates a genuinely undeclared dependency or feature requirement must be preserved as evidence and reviewed as a source/dependency-surface defect. It must not be preemptively hidden by broad manifest edits.

## Phase 141 boundary remains Tranche 2

The actual Phase 141 integration seam is still separate.

A future test that directly imports actual `prw-nat-traversal` types such as `IceConnectivitySession` requires the previously reviewed test-only dev-edge in `crates/prw-remote-bridge/Cargo.toml`.

That dependency is intentionally absent now.

Adding it remains a Tranche 2 Cargo/dependency mutation with its own:

- explicit authorization;
- pinned-Cargo materialization;
- lockfile diff review;
- locked validation;
- separate evidence head.

This checkpoint does not move actual Phase 141 validation into Tranche 1.

## Private module/export boundary

Path-including `candidate_reachability.rs` from integration tests does not make the module part of the production `prw-remote-bridge` module graph.

No `lib.rs` export is authorized or required for Tranche 1.

A compiler problem caused by the deliberate private path-inclusion model would need a minimal test/source corrective under the execution gate, not an automatic production export.

## Static readiness conclusion

No undeclared crate dependency was identified in the current C02e Tranche 1 source/test imports.

This means the intended zero-dependency-mutation validation tranche is structurally coherent with the committed manifests.

It does **not** prove:

- that Rust parsing/type checking succeeds;
- that all warnings satisfy `-D warnings`;
- that tests pass;
- that formatting passes;
- that native runner prerequisites are installed;
- that the lock graph satisfies current manifests at execution time.

Those remain executable readiness/validation evidence and must use the separately locked `cargo metadata --locked` / `--locked` gate sequence.

## Mutation surface

This checkpoint adds documentation/evidence only. It does not alter:

- any Rust source or test;
- root or crate `Cargo.toml`;
- `Cargo.lock`;
- workflow files;
- C02d;
- production runtime/network/deployment state.

## Not executed

No Cargo metadata/resolution, rustfmt, compiler/type check, Clippy, tests, build, dependency materialization, package installation, workflow dispatch, network I/O, STUN/ICE/TURN, QUIC activity, Agent/bootstrap activation, deployment, signing, privileged/system mutation, PR creation/merge, or Host Mirror synchronization is performed by this checkpoint.

## Result

`C02E_TRANCHE1_DEPENDENCY_SURFACE_STATIC_READY / CURRENT_IMPORTS_MATCH_COMMITTED_DEPENDENCIES / PRW_NAT_TRAVERSAL_NOT_REQUIRED_OR_IMPORTED / ZERO_MANIFEST_MUTATION_BASELINE_PRESERVED / ACTUAL_PHASE141_EDGE_REMAINS_TRANCHE2 / EXECUTION_GATE_CLOSED / C02D_UNTOUCHED`
