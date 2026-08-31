# Phase 152 C03e-HA — Production Reachability Typed Durable Snapshot Integration Source Materialization

Status: `STAGING`

Target gate:
`C03E_HA_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_MATERIALIZED`

Target closure:
`CLOSED_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_MATERIALIZATION`

## Purpose

C03e-HA source-materializes the existing C03e-GW typed durable-snapshot integration law under the compiler-complete path ceiling selected by canonically closed C03e-GZ. It does not inherit blocked GY branch state as lineage: HA is rooted directly at exact closed GZ and reuses only byte-identical, audited source content where that content independently matches the selected semantics.

HA changes the bridge-owned durable snapshot from a live `PeerConnectivityPlan` carrier to provider-neutral `PeerConnectivityPlanDurableState`; projects live plans immediately before existing durable compare-and-commit calls; restores loaded state only through `PeerConnectivityPlan::from_durable_state(...)`; and performs only compiler-required test fixture adaptations outside the owner/focused-test seam.

## Exact predecessor

- predecessor PR: `#330` C03e-GZ — canonical `CLOSED`;
- exact GZ branch: `phase-152-c03e-gz-production-reachability-typed-durable-snapshot-integration-source-scope-correction-ii-semantics-selection-staging`;
- exact GZ head: `569f44e76ab07bcb4d53cd79c489f4a1cf851f1f`;
- exact GZ tree: `09bb288c1e888261caa9016801b48434e1fc485b`;
- GZ gate: `C03E_GZ_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_SCOPE_CORRECTION_II_SEMANTICS_SELECTED`.

Blocked GY PRs are evidence only and are not HA ancestors.

## Fresh exact-GZ audit

The exact-GZ source topology remains consistent with GZ:

1. `crates/prw-remote-bridge/src/reachability_owner.rs` still owns `ReachabilityDurableSnapshot`, `ReachabilitySnapshotError`, `ReachabilityDurableStore`, candidate commit, retirement, recover, and reload.
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` still owns the focused in-memory production-owner persistence tests and direct snapshot construction.
3. `crates/prw-agent/src/production_reachability_owner_custody.rs` production code remains generic; its existing `#[cfg(test)]` fixture requires only `plan.durable_state()` when constructing the typed snapshot.
4. `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs` has exactly the two compiler-proven live-plan snapshot-constructor callsites recorded by GZ. Both are test-only projections to `fixture.plan.durable_state()`.
5. Production `crates/prw-remote-bridge/src/reachability_freshness_wire.rs` does not construct or restore snapshots and requires no source change.
6. No manifest, lockfile, workflow, codec, schema, provider, runtime, networking, deployment, or sixth source path is selected by the current audit.

Any sixth path remains a stop-and-re-audit condition.

## Materialized source law

### Typed durable snapshot carriage

`ReachabilityDurableSnapshot` owns:
- `PeerConnectivityPlanDurableState` as durable plan state; and
- the existing `CandidatePublicationFreshnessRecord`.

Its constructor rejects exact-peer mismatch between durable plan state and freshness before persistence can observe the pair. `plan()` returns the provider-neutral durable state, not a live plan.

### Restoration failure classification

`ReachabilitySnapshotError` gains the narrow semantic classification:
`PlanRestoration(ConnectivityError)`.

This remains distinct from persistence unavailability/ambiguity, missing durable state, stale expected-current CAS, candidate-publication validation, and exact-peer mismatch. No peer-visible wire-error taxonomy is added.

### Candidate publication ordering

The existing owner ordering remains:
1. require current owner;
2. validate authenticated publication/currentness;
3. require exact current presented freshness;
4. clone and fully validate staged live plan;
5. issue exactly one verifier-owned replacement token;
6. reject exact-current replacement;
7. construct staged freshness;
8. project the validated staged live plan through `durable_state()`;
9. construct typed durable snapshot;
10. await existing expected-current CAS;
11. only after definite `Committed`, install the original staged live plan/freshness and invalidate old traversal.

Projection does not create authority, perform I/O, mint a token, retry, or select a provider.

### Retirement

Retirement projects `self.plan.durable_state()` and combines it with the existing retired freshness tombstone before the existing durable CAS. Retirement does not mint a token or create a replacement lifecycle.

### Recovery

After authoritative `load_current(...)`, recovery:
- rejects plan/freshness/requested-peer mismatch;
- destructures the typed snapshot;
- restores only through `PeerConnectivityPlan::from_durable_state(...)`;
- maps semantic restoration failure to `ReachabilityOwnerError::Snapshot(ReachabilitySnapshotError::PlanRestoration(...))`;
- constructs an owner only after complete restoration succeeds.

No partial owner is returned. Missing state never implies new-lifecycle eligibility.

### Reload

`reload_from_store(...)` retains the current exact peer key, loads one authoritative snapshot, validates exact-peer coherence, restores the live plan, and only then installs plan plus freshness as one pair. Missing, ambiguous, mismatched, or semantically invalid durable state enters `RecoveryRequired` and drops traversal. Loaded freshness is not installed independently after plan restoration failure.

### Historical anti-reuse

The durable snapshot now carries the connectivity-owned historical candidate-ID high-watermark through `PeerConnectivityPlanDurableState`. Recovery/reload therefore preserve removed-ID anti-reuse history rather than reconstructing only from active candidates.

### Transient observations

Reachability observations remain absent from durable state. Restored candidate observations return as `Unknown` through the connectivity-owned restoration boundary.

## Focused validation

The bridge production-seam tests retain prior coverage and add/strengthen proofs that:
- successful commit persists typed projected candidates/high-watermark plus replacement freshness;
- historical high-watermark survives durable recovery and prevents reuse of removed IDs;
- invalid persisted high-watermark fails recovery under `Snapshot(PlanRestoration(InvalidCandidateIdHighWatermark))`;
- invalid durable plan state during reload enters `RecoveryRequired`, drops traversal, and does not partially install loaded freshness;
- stale durable state reload continues through typed state;
- retirement persists typed plan state plus retired freshness.

The Agent custody test fixture changes only snapshot construction to `plan.durable_state()`; production custody semantics are unchanged.

The freshness-wire test fixture changes only its two direct snapshot-constructor arguments from cloned live plans to `durable_state()` projections. Production resynchronization semantics remain authoritative durable read plus exact-token redelivery with no generation or CAS.

## Exact authorized path set

HA may change exactly these five paths and no others:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`
2. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`
3. `crates/prw-agent/src/production_reachability_owner_custody.rs`
4. `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`
5. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HA_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_MATERIALIZATION_STAGING.md`

The Agent and freshness-wire paths are authorized only for existing test-fixture adaptations. Any production semantic change in those files beyond formatter movement is a scope violation.

## Explicit non-selection / non-activation

HA selects or activates no:
- persistence byte codec or framing;
- schema/version/migration law;
- database key encoding or keyspace;
- etcd, SQL, embedded database, or other concrete durable provider;
- provider revisions, leases, TTLs, watch loops, retries, or executor/runtime ownership;
- credentials, TLS, RBAC, secrets, or connection bootstrap;
- owner-map population or Agent startup recovery orchestration;
- new-lifecycle/bootstrap freshness callsite;
- candidate handoff/current-Mesh terminal response activation;
- listener/readiness/traversal/dialing/networking activation;
- deployment, restart operation, merge, branch deletion, or repository-visibility mutation.

## Validation gate

HA may close only on one exact final head when:
1. exact GZ parent/merge base and ahead-only lineage are proven;
2. exactly the five authorized paths changed;
3. Agent and freshness-wire diffs remain test-fixture-only;
4. no manifest/lockfile/workflow/provider/runtime/network/deployment path changed;
5. locked dependency graph passes;
6. rustfmt passes;
7. Clippy with warnings denied passes;
8. workspace tests pass;
9. workspace build passes;
10. Android validation, if automatically triggered, passes on the same exact final head;
11. no failing or pending automatically triggered exact-head workflow remains;
12. immutable Drive audit is stored in the canonical PRW folder and verified by raw byte/hash readback.

Until all conditions hold, HA remains `STAGING`, draft, open, and unmerged.

## Safe successor boundary

After canonical HA closure, begin with a fresh exact-HA read-only audit. Do not assume codec/schema/keyspace/provider materialization is automatically authorized. Determine the narrowest unresolved prerequisite and keep concrete provider/runtime activation separately gated.
