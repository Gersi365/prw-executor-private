# Phase 152 C02e — Actual Phase 141 Integration-Test Edge Review

Status: `PASS_STATIC_DEPENDENCY_REVIEW / FUTURE_DEV_EDGE_SHAPE_IDENTIFIED / MANIFEST_AND_LOCKFILE_MUTATION_DEFERRED / VALIDATED_GRAPH_REQUIRED / NO_PRODUCTION_OWNERSHIP_INFERENCE / BUILD_GATE_CLOSED / NO_NETWORK_IO`

Review base head: `baaf419bdd565bc3a9fe67d88032f139adfae105`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The C02e test-only reachability reference harness currently proves lifecycle ordering with an opaque test traversal marker because `prw-remote-bridge` has no dependency on `prw-nat-traversal`.

This review determines the safe shape and validation obligations for a future integration test that uses the actual Phase 141 `IceConnectivitySession`, without conflating a test dependency with production ownership.

## Current dependency facts

`crates/prw-remote-bridge/Cargo.toml` currently has no dependency or dev-dependency on `prw-nat-traversal`.

`crates/prw-nat-traversal` is already a workspace member and its own manifest depends on:

- `prw-connectivity`;
- exact `rtc-ice = 0.20.2`;
- exact `rtc-stun = 0.20.2`;
- exact `rtc-shared = 0.20.2`;
- exact `sansio = 1.0.0`.

The production Phase 141 contract explicitly keeps this crate Sans-I/O and leaves socket/runtime ownership to a later caller.

## Phase 141 dependency-materialization precedent

The authoritative Phase 141 validation record treats dependency state as validated implementation evidence, not an incidental manifest edit.

It records:

- an exact dependency probe under Rust/Cargo 1.97.1;
- an exact scratch lock hash;
- focused Clippy/tests;
- full workspace Clippy/tests/build;
- an exact validated `Cargo.lock` SHA-256;
- a later materialization run that reapplied the exact validated state to the then-current main head and reverified the hashes.

The Phase 141 contract likewise requires validation of the exact locked dependency graph and full workspace validation.

Therefore a future C02e manifest edge must follow the same evidence-preserving principle.

## Future test-edge shape

When the Cargo/build validation gate is explicitly opened, the narrowest intended edge is:

```toml
[dev-dependencies]
prw-nat-traversal = { path = "../prw-nat-traversal" }
```

in `crates/prw-remote-bridge/Cargo.toml` only, unless then-current Cargo resolution proves another change is necessary.

This edge would exist solely to let integration-test source instantiate the actual Phase 141 Sans-I/O traversal session alongside the C02e private semantic adapter and actual connectivity plan.

It must not be added under `[dependencies]` merely to make the test compile.

## Required future mutation procedure

The future authorized step must be audit-first and deterministic:

1. re-read current C02e head, `prw-remote-bridge/Cargo.toml`, root workspace manifest and `Cargo.lock`;
2. add only the dev-dependency edge;
3. let the locked Cargo toolchain resolve/materialize the lockfile rather than hand-editing `Cargo.lock`;
4. inspect the resulting manifest/lock diff to ensure no unrelated dependency upgrade or graph drift;
5. run focused formatting/Clippy/test validation for the new integration surface;
6. run the separately required full-workspace formatting/Clippy/tests/build under the resolved graph;
7. record exact hashes and environment/tooling failures separately from source defects;
8. commit only the validated graph/source/evidence.

If Cargo resolution changes unrelated versions or cannot reproduce the locked graph, the mutation must fail closed rather than broaden into dependency upgrades.

## Actual Phase 141 test objective

The eventual integration test should replace only the opaque test traversal marker with actual Phase 141 lifecycle ownership sufficient to prove:

- the current owner holds one actual `IceConnectivitySession` for the current accepted candidate state;
- a successful candidate refresh makes that session stale even for an exactly retained candidate;
- any already-polled or queued old `CandidateReachabilityUpdate` is rejected by upper lifecycle currentness before application;
- a newly constructed session from the refreshed state may produce current observations;
- failed candidate refresh preserves the previously current actual traversal session;
- transport rotation invalidates the old plan and traversal lifecycle;
- all traffic remains in-memory/Sans-I/O during disposable validation.

The test must not open a UDP socket or activate persistent STUN/ICE/TURN traffic merely because it imports Phase 141.

## Production ownership boundary

A dev-dependency from `prw-remote-bridge` to `prw-nat-traversal` would be **test composition only**.

It must not be cited as evidence that:

- `prw-remote-bridge` is the production traversal owner;
- production `prw-remote-bridge` should gain a corresponding normal dependency;
- Agent/bootstrap should activate traversal;
- the upper runtime owner has been selected.

The concrete production owner remains a later architecture/runtime decision.

## Why mutation is deferred now

The current C02e branch explicitly keeps the build/Cargo gate closed.

Adding the dev-dependency now would alter a Cargo manifest and potentially the root lockfile without authorization to run the required dependency resolution and validation steps. That would create unvalidated repository state and violate the repository's own Phase 141 materialization precedent.

Therefore no manifest or lockfile mutation is made by this checkpoint.

## Next safe seam

Continue source/design work on the still-missing verifier-owned candidate-publication freshness authority placement and lifecycle scope.

The actual Phase 141 integration edge remains ready for a later separately authorized Cargo/build validation step.

## Validation boundary

Static dependency/contract review only. No Cargo manifest, lockfile, source dependency graph, build, test, workflow, socket, traversal runtime, Agent/bootstrap, deployment, signing, privileged state, PR or merge is modified or executed.
