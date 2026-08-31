# Phase 152 C03e-GV — Production Reachability Durable Plan-State Projection/Restoration Source Materialization

Status: `STAGING`

Target gate:
`C03E_GV_PRODUCTION_REACHABILITY_DURABLE_PLAN_STATE_PROJECTION_RESTORATION_SOURCE_MATERIALIZED`

## Purpose

C03e-GV source-materializes only the C03e-GU-selected provider-neutral typed durable state for
`PeerConnectivityPlan`, together with exact projection/restoration semantics that preserve historical
candidate-ID anti-reuse state while deliberately discarding transient reachability observations.

GV does not select or materialize a persistence byte codec, database keyspace, etcd/database
provider, credentials, production owner recovery/population, candidate handoff, runtime/networking,
deployment, or merge.

## Exact predecessor

GV is rooted directly at canonically closed C03e-GU:

- predecessor PR: `#323`;
- exact GU head: `264cc44357b8969cf3b2a49dfcb86e6dbdfcc940`;
- exact GU tree: `d633148334dcf7bc55363c6743071e4b7e2276d6`;
- GU gate: `C03E_GU_PRODUCTION_REACHABILITY_DURABLE_PLAN_STATE_PROJECTION_RESTORATION_SEMANTICS_SELECTED`;
- GU remains intentionally draft/open/unmerged.

No intermediate branch, merge result, `main`, or reconstructed local state is a permitted GV base.

## Fresh exact-GU-head source audit

The exact GU head confirms:

1. `PeerConnectivityPlan` and all candidate anti-reuse invariants remain owned by
   `crates/prw-connectivity/src/lib.rs`;
2. there is no existing `PeerConnectivityPlanDurableState` or equivalent projection/restoration API;
3. the exact plan representation still owns private current `CandidateState` values plus private
   `candidate_id_high_watermark`;
4. no third source/product path is required to materialize the GU semantics;
5. no manifest or lockfile change is required;
6. `ConnectivityError` is already `#[non_exhaustive]`, so one narrow durable-state consistency
   failure can be represented without forcing external exhaustive-match changes.

Exact audited GU connectivity blob:
`68635ae87735e6abb055cda21f8232f39b81a63e`.

No existing C03e-GV branch/PR was present before this checkpoint was created.

## Authorized final path scope

GV authorizes exactly these two paths unless compiler evidence proves an unavoidable contradiction:

1. `crates/prw-connectivity/src/lib.rs` — typed durable state, projection/restoration API, one narrow
   consistency error, and focused pure tests;
2. this source-materialization contract.

Any third repository path is a stop-and-re-audit condition. No Cargo manifest/lockfile mutation is
expected or authorized by default.

## Materialized API law

### Typed durable carrier

`PeerConnectivityPlanDurableState` is connectivity-owned and provider-neutral. It contains exactly:

- one `PeerConnectivityIdentity`;
- the complete current `Vec<ConnectivityCandidate>` in plan order;
- the exact historical optional `CandidateId` high-watermark.

The carrier contains no `ReachabilityObservation`, selected-path cache, freshness state, timestamps,
request/session IDs, provider revisions, database keys, or serialized bytes.

The carrier exposes typed read-only accessors and a parts constructor so a later separately gated
codec can construct decoded typed state without gaining authority over connectivity invariants.
Validation of persisted semantic consistency remains the plan restoration boundary.

### Projection

`PeerConnectivityPlan::durable_state()`:

- clones the exact peer identity;
- projects each current private candidate state to only its typed `ConnectivityCandidate`;
- preserves current candidate vector order exactly;
- preserves exact historical high-watermark;
- ignores all transient observations;
- performs no mutation, allocation, provider call, I/O, freshness change, or candidate refresh.

### Restoration

`PeerConnectivityPlan::from_durable_state(...)` consumes only the typed durable state and validates
before constructing a plan:

- candidate count remains within `MAX_CONNECTIVITY_CANDIDATES`;
- duplicate candidate IDs are rejected through the existing classification;
- duplicate exact `(path kind, endpoint)` candidates are rejected through the existing
  classification;
- exact peer identity is preserved;
- non-empty candidates require historical high-water to be present and at least the maximum current
  candidate ID;
- empty candidates allow either `None` or `Some(previous_high_water)`;
- high-water missing for non-empty candidates or below the current maximum fails closed through the
  narrow `InvalidCandidateIdHighWatermark` classification;
- every restored observation is initialized to `Unknown`;
- no partially restored plan is returned on failure.

The restored internal numeric high-water is exactly the typed durable value, or zero only when the
carrier contains `None`.

### Existing construction and refresh semantics remain unchanged

`PeerConnectivityPlan::new(...)` retains its existing genuinely-new-plan semantics and still derives
its initial high-watermark from initial current candidates.

`refresh_candidates(...)` retains the existing removed-ID anti-reuse/rebinding law. GV does not
allocate candidate IDs or weaken the rule that a newly introduced ID must exceed the complete
plan-lifetime high-watermark.

## Focused validation expectations

Focused pure tests in `prw-connectivity` must prove at least:

- empty never-used plan projects/restores with no high-water;
- active candidates round-trip exact peer/candidate order/high-water;
- transient observations do not survive restoration and restored selection is `Offline`;
- historical high-water greater than active maximum survives round trip;
- zero active candidates with historical high-water survives round trip;
- high-water below active maximum fails closed;
- missing high-water with active candidates fails closed;
- post-restore refresh rejects reuse of an identifier that is historical but no longer active.

## Explicit exclusions

GV does not authorize:

- byte codec/schema/version or migration policy;
- persistence key encoding/keyspace;
- etcd or another concrete durable store;
- provider client connection/construction;
- TLS/auth/RBAC or credential material;
- new-lifecycle/bootstrap freshness callsite;
- production owner recovery/population/synchronization;
- GP owner-map population;
- Agent candidate handoff/current-Mesh response activation;
- worker/cancellation integration;
- traversal/listener/readiness/dialing/networking activation;
- deployment, restart/recovery operation, merge, branch deletion, or repository-visibility mutation.

## Validation gate

GV may close only when all of the following hold on one exact final head:

1. exact GU merge base and ahead-only lineage;
2. final changed-path set is exactly the authorized two paths unless a separately documented compiler
   contradiction is explicitly handled;
3. locked dependency graph succeeds without lock mutation;
4. rustfmt succeeds;
5. Clippy with warnings denied succeeds;
6. workspace tests succeed;
7. workspace build succeeds;
8. Android validation, if automatically triggered, succeeds on the same exact head;
9. no failing or pending automatically triggered exact-head workflow remains;
10. immutable Drive audit is stored under the canonical Private Remote Workspace folder and verified
    by raw byte/hash readback.

Successful closure classification:
`CLOSED_PRODUCTION_REACHABILITY_DURABLE_PLAN_STATE_PROJECTION_RESTORATION_SOURCE_MATERIALIZATION`.

Until closure, GV remains `STAGING`, draft, open, and unmerged.