# C02e Tranche 6 — Static Clippy Preflight Corrective Audit

Status: `STATIC_CLIPPY_PREFLIGHT_CORRECTIVE_APPLIED / MANUAL_CONTAINS_CORRECTED / OPTION_IF_LET_ELSE_REMOVED / ASSIGNING_CLONES_REMOVED / TEST_ONLY_MUTATION / PRODUCTION_SOURCE_BYTE_STABLE / EXECUTABLE_VALIDATION_UNRESOLVED / NO_RUNTIME_ACTIVATION`

Starting branch head: `51bf5462b4549424a941b734607613164315b3a1`

Frozen predecessor C02d: `857583b25ed1206317641a93fd8f927819c954d8`

First test-only corrective commit: `c5821a1f6626c5a8364cb6435c33079e34893ac8`

Second test-only corrective commit: `00c78fa746ddaf604beb41e994a79a04478f1824`

## Purpose

This audit records a narrow static Clippy-oriented corrective discovered after the existing-workflow validation-route preflight. It does not claim that Clippy, tests, build, formatting or locked metadata have executed for the corrected head.

The correction is limited to the test-only peer-namespace reference harness at:

`crates/prw-remote-bridge/tests/reachability_live_owner_peer_namespace.rs`

No production live-owner source, production reachability owner, crate manifest, Cargo lockfile, Agent/bootstrap, persistence backend, network adapter, deployment or service-manager path is changed by these two corrective commits.

## Authoritative lint policy

The workspace root `Cargo.toml` enables:

- `clippy::all = "warn"`;
- `clippy::pedantic = "warn"`;
- `clippy::nursery = "warn"`.

The canonical repository Rust validation workflow invokes workspace Clippy with `-D warnings`.

Therefore a statically identifiable lint in any of those enabled groups is an executable-validation risk and should be removed before the exact-head run when the correction is deterministic and semantics-preserving.

No `.clippy.toml`, `clippy.toml`, or local allow override for the corrected lint patterns was found during this preflight.

## Corrective 1 — `manual_contains`

The original peer-scoped reference test blob was:

`4ebb7b053c6b0b28d6aade8bd1e2604a1801b931`

Its `currentness` implementation used:

`self.current.iter().any(|current| current == grant)`

For a `Vec<ReachabilityLiveOwnerGrant>`, this is the direct `manual_contains` pattern. The first corrective replaced it with:

`self.current.contains(grant)`

Commit:

`c5821a1f6626c5a8364cb6435c33079e34893ac8`

That commit changed only the peer-namespace integration test by one added and one deleted line.

## Corrective 2 — `option_if_let_else` and `assigning_clones`

The same test-only `acquire` implementation used an `if let Some(...) { ... } else { ... }` branch over `iter_mut().find(...)` and assigned a cloned grant with:

`*current = grant.clone()`

Under the workspace-enabled nursery/pedantic groups these are static lint risks corresponding to `option_if_let_else` and `assigning_clones`.

The second corrective rewrote only that test-only replacement block to:

- iterate explicitly over `&mut self.current`;
- replace the matching same-peer grant with `current.clone_from(&grant)`;
- record whether replacement occurred;
- append `grant.clone()` only when no current same-peer entry was replaced.

This preserves the same exact-peer namespace semantics and does not alter acquisition ordering, fencing generation, currentness, release semantics or registry-currentness separation.

Commit:

`00c78fa746ddaf604beb41e994a79a04478f1824`

Final corrected peer-namespace test blob:

`d384a455f0ba1d98f97578c8f90977c82fa40ca2`

## Production byte stability

After both corrective commits, the production live-owner seam remains Git blob:

`ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`

at:

`crates/prw-remote-bridge/src/reachability_live_owner.rs`

The representation decision therefore remains unchanged:

- `ReachabilityLiveOwnerFence` retains `NonZeroU128` as the reviewed logical in-memory fencing generation;
- exact authority namespace remains `DeviceId + TransportIdentity`;
- persistence encoding remains unselected;
- wire encoding remains unselected;
- concrete live-owner backend remains unselected;
- no runtime/network ownership is activated.

## Validation classification

This static corrective removes three concrete lint-pattern risks from the newly added peer-namespace test surface. It is not executable evidence.

The exact corrected branch must still receive an observable run of the canonical locked metadata / rustfmt / workspace Clippy / workspace tests / workspace build validation route before Tranche 6 can be closed.

No PASS or FAIL is inferred from the absence of an executable runner in the current connected environment.

## Stop line

`TRANCHE6_STATIC_CLIPPY_PREFLIGHT_CORRECTED / TEST_ONLY_LINT_RISKS_REMOVED / PRODUCTION_SEAM_BYTE_STABLE / EXECUTABLE_VALIDATION_STILL_REQUIRED / SOURCE_FAILURE_NOT_ESTABLISHED / PRODUCTION_RUNTIME_CLOSED`
