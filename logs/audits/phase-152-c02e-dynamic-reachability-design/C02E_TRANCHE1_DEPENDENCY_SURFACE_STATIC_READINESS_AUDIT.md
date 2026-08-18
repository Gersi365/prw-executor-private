# Phase 152 C02e — Tranche 1 Dependency-Surface Static Readiness Audit

Status: `PASS_STATIC_DEPENDENCY_SURFACE_REVIEW / CURRENT_IMPORTS_CLOSED_BY_EXISTING_MANIFESTS / NO_PHASE141_IMPORT_IN_TRANCHE1 / ZERO_MANIFEST_MUTATION_BASELINE_CONFIRMED / EXECUTION_NOT_PERFORMED / BUILD_GATE_CLOSED / C02D_UNTOUCHED`

Reviewed head: `1284b47b1ecf6af38d59cb7c95df26d1c85c7850`

Reviewed tree: `3907162dbc8a5ee59621228c97543801b0734260`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Audit purpose

This static audit verifies that the C02e source/tests assigned to validation Tranche 1 do not already require an undeclared crate edge that would force a Cargo manifest mutation before executable validation.

No compiler, Cargo resolution, Clippy, test or build command was run.

## Evidence reviewed

- `crates/prw-connectivity/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`;
- `crates/prw-remote-bridge/src/candidate_reachability.rs`;
- `crates/prw-remote-bridge/tests/authenticated_candidate_provenance.rs`;
- `crates/prw-remote-bridge/tests/candidate_reachability_semantic_adapter.rs`;
- `crates/prw-remote-bridge/tests/dynamic_reachability_registry.rs`;
- `crates/prw-remote-bridge/tests/reachability_composition_reference.rs`;
- `crates/prw-remote-bridge/tests/reachability_freshness_authority_reference.rs`;
- `crates/prw-remote-bridge/tests/reachability_freshness_bootstrap_reference.rs`;
- the existing C02e Tranche 1/Tranche 2 validation plan and execution-readiness checkpoint.

## Connectivity finding

`prw-connectivity` currently declares only `prw-core` as a crate dependency.

The current C02e connectivity source remains within that declared crate boundary plus the Rust standard library.

No static evidence requires a new dependency for the connectivity portion of Tranche 1.

## Private adapter finding

The private `candidate_reachability.rs` source imports only:

- standard library `fmt`;
- `prw-connectivity`;
- `prw-registry`;
- `prw-session`.

Those PRW crates are already normal `prw-remote-bridge` dependencies.

No production module export or new crate dependency is needed merely for the existing test path-inclusion model.

## Base integration-test finding

The base C02e bridge tests import only crate families already present in the bridge manifest:

- `aws-lc-rs`;
- `prw-connectivity`;
- `prw-control-plane`;
- `prw-core`;
- `prw-device-identity-signer`;
- `prw-registry`;
- `prw-session`.

The required test-only crates are already declared under `[dev-dependencies]`; connectivity/registry/session are already normal dependencies.

No additional direct crate import was observed in the reviewed source headers.

## Reference-test finding

All three upper reference tests deliberately avoid actual Phase 141 imports.

They path-include the private candidate adapter and use test-local traversal/freshness/bootstrap markers.

`reachability_composition_reference.rs` explicitly documents that `prw-nat-traversal` is not added while the Cargo/build gate is closed.

No `prw_nat_traversal` or `IceConnectivitySession` import appears in the reviewed Tranche 1 reference source.

## Tranche boundary conclusion

The current committed source surface is structurally compatible with the intended Tranche 1 invariant:

`validate current C02e source/tests with zero dependency-state mutation`

The actual Phase 141 import remains a separate Tranche 2 operation because it intentionally requires the reviewed `prw-nat-traversal` dev-edge.

No evidence supports moving that edge earlier.

## Limits of this audit

Static dependency closure is not compiler evidence.

This audit does not prove:

- parsing/type checking;
- feature unification correctness;
- rustfmt cleanliness;
- Clippy cleanliness;
- test behavior;
- native environment readiness;
- locked dependency graph satisfiability.

Those remain part of the separately locked executable validation sequence.

## Safety implication

A future authorized Tranche 1 must not preemptively edit `Cargo.toml` or `Cargo.lock` in anticipation of a problem that static source inspection does not show.

If executable validation later exposes an actual missing declaration/feature, preserve that failure as evidence and review the minimal dependency correction rather than broadening manifests opportunistically.

## Mutation surface

Added only:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_TRANCHE1_DEPENDENCY_SURFACE_STATIC_READINESS_CHECKPOINT.md`;
- this audit record.

No existing contract, Rust source/test, Cargo manifest, lockfile, workflow, C02d path, runtime/network/deployment state, or immutable authority is changed.

## Not executed

- Cargo metadata/resolution;
- rustfmt;
- compiler/type check;
- Clippy;
- tests;
- build;
- dependency materialization;
- package installation;
- workflow dispatch;
- network I/O;
- STUN/ICE/TURN activation;
- QUIC activity;
- Agent/bootstrap activation;
- deployment;
- signing;
- privileged/system mutation;
- PR creation/merge;
- Host Mirror synchronization.

## Result

`STATIC_TRANCHE1_DEPENDENCY_SURFACE_PASS / EXISTING_MANIFESTS_COVER_CURRENT_C02E_IMPORTS / NO_PHASE141_EDGE_NEEDED_BEFORE_TRANCHE1 / ZERO_DEPENDENCY_MUTATION_BASELINE_PRESERVED / EXECUTION_GATE_CLOSED / C02D_UNTOUCHED`
