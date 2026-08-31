# Phase 152 C03e-GU — Production Reachability Durable Plan-State Projection/Restoration Semantics Selection

Status: `STAGING`

Target gate:
`C03E_GU_PRODUCTION_REACHABILITY_DURABLE_PLAN_STATE_PROJECTION_RESTORATION_SEMANTICS_SELECTED`

## Purpose

C03e-GU selects exactly one missing persistence prerequisite exposed by a fresh exact-C03e-GT
source audit: a provider-neutral typed projection/restoration boundary that can round-trip the
complete durable semantic state of `PeerConnectivityPlan` without persisting transient reachability
observations and without losing historical candidate-ID anti-reuse state.

GU is documentation-only. It materializes no Rust source, serialization bytes, database key, etcd
client, provider, bootstrap callsite, owner-map population, candidate handoff, listener, networking,
deployment, or merge.

## Exact predecessor

GU is rooted directly at canonically closed C03e-GT:

- predecessor PR: `#322` — `Phase 152 C03e-GT: materialize production reachability freshness token source`;
- exact GT head: `c4fcb910e6017c14097d6b9a2454b72cf1b0c4c3`;
- exact GT tree: `8a53137d8b2b41ce55307b36fc3a6acb701297b6`;
- GT gate: `C03E_GT_PRODUCTION_REACHABILITY_FRESHNESS_TOKEN_SOURCE_MATERIALIZED`;
- GT remains intentionally draft/open/unmerged.

No intermediate branch, merge result, `main`, or reconstructed local state is a permitted GU base.

## Fresh exact-GT-head audit findings

### `ReachabilityDurableSnapshot` requires a complete plan round trip

`crates/prw-remote-bridge/src/reachability_owner.rs` defines the durable snapshot as exactly:

`PeerConnectivityPlan + CandidatePublicationFreshnessRecord`.

The production owner persists snapshots at accepted publication and retirement commit points and
recovers an owner only by loading an existing authoritative durable snapshot. Transient reachability
observations are intentionally not durable truth.

Exact audited GT blob:
`fb7543361ea3a144ae9275284b41bf0ef63df2ad`.

The existing `ReachabilityDurableStore` is already awaitable after GQ/GR, but explicitly still owns
no database product or serialization format.

### Freshness state is already reconstructible

`crates/prw-remote-bridge/src/candidate_publication_freshness.rs` exposes:

- exact peer identity;
- exact durable lifecycle;
- exact non-zero 32-byte verifier token where a lifecycle carries one;
- constructors for `NewLifecycleEligible`, `Established`, `RecoveryRequired`, and `Retired`.

Exact audited GT blob:
`fd7c2f095999b6a6479be79c562637fe5f46634c`.

GU therefore does not redesign freshness representation or lifecycle reconstruction.

### `PeerConnectivityPlan` is not currently persistence-round-trip capable

Exact GT `crates/prw-connectivity/src/lib.rs` stores:

- exact `PeerConnectivityIdentity`;
- private `Vec<CandidateState>` containing current configured candidates plus transient observations;
- private historical `candidate_id_high_watermark: u64`.

The current public API exposes the exact peer and historical high-watermark, but it does not expose
one complete provider-neutral projection of the current configured candidate vector.

More importantly, `PeerConnectivityPlan::new(peer, candidates)` derives its high-watermark only from
the maximum currently active candidate identifier. That constructor cannot faithfully restore a
valid plan whose historical high-watermark is greater than every currently active candidate ID, or
a plan with zero current candidates and a non-empty historical candidate-ID namespace.

This state is reachable through ordinary validated `refresh_candidates(...)`: a higher-ID candidate
may become historical after later removal while the high-watermark remains non-decreasing.

Reconstructing such a plan through `PeerConnectivityPlan::new(...)` would lower historical
anti-reuse state and could later admit a candidate identifier that was already used and removed.
That would violate the existing C03e-BJ/BK anti-reuse semantics and is not an acceptable persistence
implementation shortcut.

Exact audited GT connectivity blob:
`bdefd6302fde130330be0c51073aa07345501249`.

### Transient observations must not become durable state

`ReachabilityDurableSnapshot` documentation already states that transient reachability observations
are not written by observation admission and recovery falls back to the last committed publication
snapshot rather than treating prior `Reachable` state as durable truth.

A restoration boundary must therefore reconstruct all candidate observations as `Unknown`.

Persisting or restoring `Reachable`/`Unreachable` observations is explicitly rejected by GU.

## Selected canonical law

### 1. New provider-neutral typed durable plan state

A later source-materialization checkpoint shall add a connectivity-owned typed durable state carrier
conceptually equivalent to:

`PeerConnectivityPlanDurableState`.

The carrier must represent exactly:

1. one exact `PeerConnectivityIdentity`;
2. the complete bounded vector of currently configured `ConnectivityCandidate` values;
3. the exact historical candidate-ID high-watermark as typed optional `CandidateId` state.

The carrier does not contain reachability observations, selected-path cache, timestamps, request IDs,
session IDs, freshness tokens, provider revision numbers, database keys, or serialized bytes.

### 2. Ownership and layering

The typed durable-plan state belongs to `prw-connectivity`, because:

- candidate anti-reuse/high-water invariants are connectivity-plan invariants;
- only `prw-connectivity` owns the private plan representation today;
- an external codec in `prw-remote-bridge` must not reconstruct private connectivity state through
  ad-hoc or duplicated rules;
- a database/provider layer must not gain authority to reinterpret plan invariants.

The future type must be provider-neutral and contain no etcd/database/runtime dependency.

### 3. Exact projection semantics

A future read-only projection from one `PeerConnectivityPlan` shall preserve exactly:

- the exact peer identity;
- every currently configured candidate with exact `CandidateId`, `ConnectivityPathKind`, and
  `ConnectivityEndpoint`;
- the exact historical candidate-ID high-watermark.

Projection must ignore all current `ReachabilityObservation` values.

Projection performs no mutation, allocation, candidate refresh, network I/O, provider call, or
freshness change.

The projected candidate order must be deterministic and lossless relative to the plan's current
configured vector. GU does not reinterpret order as preference or authority; existing deterministic
path selection remains based on typed path kind and candidate ID.

### 4. Exact restoration semantics

A future restoration constructor shall accept only the typed durable state and construct one
`PeerConnectivityPlan` after validating all existing candidate-plan invariants plus the durable
high-water invariant.

Required validation:

1. current candidate count remains `0..=MAX_CONNECTIVITY_CANDIDATES`;
2. every current candidate identifier is non-zero by existing `CandidateId` typing;
3. duplicate candidate IDs are rejected;
4. duplicate exact `(path kind, endpoint)` candidates are rejected;
5. the exact peer identity is preserved unchanged;
6. when current candidates are non-empty, historical high-water must be present and must be greater
   than or equal to the maximum current candidate ID;
7. when current candidates are empty, both `None` and `Some(previous_high_water)` are valid durable
   states: `None` means no non-zero candidate ID has ever been accepted, while `Some(...)` preserves
   a historical namespace after all current candidates were removed;
8. a supplied historical high-water smaller than any current candidate ID fails closed;
9. all restored candidate observations become `Unknown`;
10. restoration performs no provider observation, path probe, reachability inference, or selected
    path fabrication.

Restoration must preserve the exact historical high-water so subsequent
`refresh_candidates(...)` continues rejecting reuse/rebinding exactly as before persistence.

### 5. Existing construction remains unchanged

`PeerConnectivityPlan::new(peer, candidates)` retains its current meaning for a genuinely new plan
whose historical high-water is derived from its initial candidates.

GU does not overload or weaken `new(...)` to accept arbitrary persistence state.

The persistence restoration path is a distinct explicitly validated constructor/boundary.

### 6. Existing anti-reuse law remains authoritative

GU preserves the current law:

- a removed candidate ID cannot later be reused;
- a retained ID is valid only for the exact same path kind and endpoint;
- a newly introduced candidate ID must exceed the complete plan-lifetime high-watermark;
- failed refresh does not advance or mutate high-water state.

Durable projection/restoration is required to preserve these rules across process restart/recovery.
It is not an allocator and does not select `high_water + 1` as a production issuance mechanism.

### 7. No observation durability

The durable carrier owns no `ReachabilityObservation` field.

This is intentional rather than an omission:

- `Reachable`/`Unreachable` are provider observations, not durable publication authority;
- recovery must not resurrect a pre-restart reachability observation;
- path selection after restoration remains `Offline` until fresh provider observations are admitted
  through the existing observation path.

### 8. No byte codec or database schema selected by GU

GU selects a typed semantic boundary only.

It does not select:

- magic/version bytes;
- field widths/tags/endian layout;
- candidate ordering on disk;
- key encoding or key prefix;
- etcd versus another database product;
- provider revisions/mod-revisions;
- TLS/auth/RBAC/credentials;
- snapshot replication or retention;
- migration/version-upgrade policy.

A later separately gated checkpoint may select a canonical durable snapshot codec only after this
typed plan state is source-materialized and validated.

## Expected bounded source successor

If a fresh exact-GU-head audit still supports the topology, the expected source-materialization
successor may change only:

1. `crates/prw-connectivity/src/lib.rs` — typed durable state, projection/restoration APIs, and
   focused pure tests;
2. one exact source-materialization contract.

No manifest/lockfile change is expected.

Any required third product/source path is a stop-and-re-audit condition rather than automatic scope
expansion.

Focused tests must cover at least:

- empty never-used plan -> durable state with no high-water -> restore exact peer/empty plan;
- active candidates -> exact candidates/high-water round trip;
- historical high-water greater than active maximum survives round trip;
- zero active candidates with historical high-water survives round trip;
- invalid high-water below active maximum fails closed;
- restored observations are all effectively `Unknown`/selected path `Offline` before fresh
  observations;
- post-restore refresh still rejects reuse of a removed historical candidate ID;
- failed restoration does not construct a partially valid plan.

## Explicit exclusions

GU does not authorize:

- Rust/source materialization;
- persistence byte codec/schema/version;
- key encoding/keyspace;
- etcd or another concrete durable provider;
- provider connection/client construction;
- production durable-store credentials;
- new-lifecycle/bootstrap freshness callsite;
- owner recovery/population/synchronization;
- GP production owner-map population;
- Agent candidate handoff/current-Mesh response activation;
- worker/cancellation integration;
- traversal, STUN/ICE/TURN/relay activation;
- listener/readiness/runtime bootstrap;
- production networking or host mutation;
- deployment, restart/recovery operation, merge, branch deletion, or repository-visibility change.

## Validation gate

GU may close only when:

1. exact GT merge base and ahead-only lineage are proven;
2. exactly one docs-only contract path changed;
3. canonical Rust validation succeeds on the exact final head;
4. no failing or pending automatically triggered exact-head workflow remains;
5. Android is reported only if it actually triggers on the docs-only head;
6. immutable Drive audit is stored under the canonical Private Remote Workspace folder and raw
   readback matches independently recorded bytes/SHA-256.

Successful closure classification:
`CLOSED_PRODUCTION_REACHABILITY_DURABLE_PLAN_STATE_PROJECTION_RESTORATION_SEMANTICS_SELECTION`.

Until closure, GU remains `STAGING`, draft, open, and unmerged.
