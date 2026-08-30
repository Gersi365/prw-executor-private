# Phase 152 C03e-GF — Candidate Publication Production Reachability Owner Custody/Recovery Semantics Selection

Status: VALIDATING

Target gate:
`C03E_GF_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SEMANTICS_SELECTED`

## 1. Exact predecessor

Closed C03e-GE is the authoritative predecessor:

- branch: `phase-152-c03e-ge-candidate-publication-mesh-post-auth-control-stream-same-stream-custody-source-materialization-staging`;
- head: `a2cc8edcfe8055651e4ea1e9c302532d6ed21b75`;
- tree: `7de256340ef03e61b57cef993647b0cae9d552f5`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SOURCE_MATERIALIZATION`;
- gate: `C03E_GE_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SOURCE_MATERIALIZED`;
- PR #307: `Status: CLOSED`, draft/open/unmerged.

GE remains frozen. GF starts exactly from the GE commit and does not amend any closed predecessor.

## 2. Fresh post-GE prerequisite finding

GE materialized current-Mesh `PRWP` family recognition, strict structural decode, exact peer request correlation and same-stream custody. It intentionally stops at the Agent fail-closed `CandidatePublicationHandoffNotSelected` barrier.

The dormant FY semantic execution seam already requires a mutable:

`ProductionReachabilityOwner<S, T>`

where:

- `S: ReachabilityDurableStore`;
- `T: CandidatePublicationFreshnessTokenSource`.

Exact Agent source at GE head contains live-owner custody based on:

`ReachabilityLiveOwnerComposedAsyncAuthority`

but no Agent owner that stores, recovers or lends the durable/freshness `ProductionReachabilityOwner<S,T>` required by FY.

Those authority domains are distinct and must not be treated as interchangeable.

Therefore production reachability-owner custody/recovery is the first missing prerequisite before current-Mesh candidate execution adaptation.

## 3. Existing production-owner law remains authoritative

GF does not redefine `ProductionReachabilityOwner<S,T>`.

Existing `prw-remote-bridge::reachability_owner` law remains authoritative:

- one owner represents one exact `PeerConnectivityIdentity` lifecycle;
- `&mut self` serializes in-process owner operations;
- durable expected-current compare-and-commit is the cross-owner/process arbitration seam;
- stale expected durable state or ambiguous persistence puts the local owner into fail-closed recovery;
- secure replacement freshness is verifier-owned and is not derived from publisher input, clocks, request IDs, candidate IDs or endpoints;
- traversal is subordinate to exact current durable plan/freshness state.

GF selects only higher-owner custody/recovery semantics around that existing owner.

## 4. Recovery source of truth

Future Agent custody construction MUST create a production owner only by invoking existing:

`ProductionReachabilityOwner::recover(store, token_source, &peer)`

for one exact expected `PeerConnectivityIdentity`.

Recovery means authoritative reload from existing durable current state.

GF explicitly selects:

- durable state is source of truth for production-owner construction;
- storage absence is not new-lifecycle authority;
- storage absence must preserve existing `ReachabilityOwnerError::DurableStateMissing`;
- ambiguous/unavailable storage must preserve existing persistence failure semantics;
- a durable snapshot for a different peer must fail exact peer consistency checks;
- retired durable lifecycle remains retired and must not be silently revived;
- recovery does not fabricate a current freshness token, plan, candidate set or traversal session.

No empty/default owner fallback is permitted.

## 5. Bootstrap/recovery separation

`ProductionReachabilityOwner::recover(...)` explicitly assumes that separate lifecycle bootstrap authority has already durably established the peer lifecycle.

GF therefore does not authorize:

- creating durable bootstrap state when recovery finds none;
- treating missing state as `NewLifecycleEligible`;
- same-byte transport identity reuse as implicit rebaseline;
- creating a new freshness token merely because owner custody is requested;
- merging lifecycle bootstrap authorization into candidate publication execution.

Any missing production lifecycle bootstrap prerequisite remains a separate earlier/later gate if exact source topology exposes one during source materialization.

## 6. Selected custody cardinality

For one exact production peer lifecycle, future Agent source must retain exactly one mutable `ProductionReachabilityOwner<S,T>` inside one higher-owner custody boundary.

GF rejects:

- cloning the production owner;
- reconstructing/recovering a fresh owner for each candidate command;
- two concurrently mutable owners for the same exact peer lifecycle inside the same higher-owner custody domain;
- copying durable plan/freshness into a second local semantic owner;
- exposing raw store or token source as replacement mutation authority.

Later callers may obtain only bounded exclusive mutable access required by the existing owner API.

## 7. Recovery timing

Recovery is a custody-construction operation, not a per-command candidate execution phase.

The selected ordering is:

1. caller provides one exact expected `PeerConnectivityIdentity`, durable store instance and verifier-owned token source;
2. Agent custody construction calls `ProductionReachabilityOwner::recover(...)` exactly once for that construction attempt;
3. only successful recovery creates a retained production-owner custody object;
4. later candidate commands borrow the retained owner mutably and do not call `recover(...)` as part of ordinary request processing.

If the retained owner later enters `RecoveryRequired`, the existing explicit `reload_from_store()` law remains authoritative. GF does not invent automatic reload loops or hidden per-request recovery.

## 8. Construction failure is fail-closed

If initial production-owner recovery fails, future custody construction must fail and return/preserve the exact typed owner/recovery failure.

Failure must not:

- create an owner with guessed local state;
- suppress `DurableStateMissing`;
- fabricate freshness;
- discard peer mismatch evidence;
- create an empty plan;
- invoke candidate publication;
- select requester authority;
- retry indefinitely;
- activate listener/readiness;
- publish an owner as usable.

No partially initialized production-owner custody may escape.

## 9. Live-owner authority is not production-owner authority

Current Agent source has `ReachabilityLiveOwnerComposedAsyncAuthority` and `ReachabilityAuthorityRuntimeOwner` for live-owner admission/currentness/custody.

GF selects an explicit non-equivalence law:

- live-owner lease/currentness is not the durable reachability plan/freshness owner;
- possession of live-owner authority does not construct `ProductionReachabilityOwner<S,T>`;
- production durable currentness does not fabricate or imply a live-owner lease;
- neither owner may be substituted for the other by type erasure, wrapper conversion or naming convention.

GF does not select any new ordering or transactional coupling between these two authority domains.

Any future composition requiring both must be separately selected from source evidence.

## 10. Publisher identity remains separate

Production-owner custody does not determine publisher logical identity.

Candidate publication publisher authority must continue to derive from the authenticated logical PRW session.

GF does not allow publisher identity to be derived from:

- the durable owner peer alone;
- current Mesh request ID;
- PRWP payload fields;
- candidate endpoints;
- transport bytes;
- requester/rendezvous grant;
- freshness token;
- live-owner lease identity.

A future current-Mesh execution adapter must still prove exact authenticated publisher/session composition separately.

## 11. Requester authority lock ordering remains unchanged

FY already fixes requester/rendezvous lock ordering:

- candidate admission occurs without requester mutex custody;
- one requester grant is selected under requester mutex;
- requester mutex is released before durable reachability commit;
- exact requester cleanup reacquires requester authority only after definite commit success.

GF preserves that law.

Production-owner custody construction/recovery must not occur while requester/rendezvous mutex custody is held.

A future candidate request must not acquire requester authority merely to recover or rebuild the production owner.

## 12. Candidate execution remains dormant

GF does not invoke or activate:

`SharedRequesterRendezvousAuthority::execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`

It does not remove GE's:

`CandidatePublicationHandoffNotSelected`

barrier.

It selects only the owner prerequisite needed before a later execution-adaptation checkpoint may replace that barrier with a typed handoff/composition.

## 13. GA/GC remain dormant

GF does not invoke:

- `project_candidate_publication_terminal_result(...)`;
- `compose_candidate_publication_terminal_result_frame(...)`.

No Accepted/Rejected projection, cleanup disposition projection, terminal result frame composition or current-Mesh response write occurs in GF.

## 14. No current-Mesh response write

GE candidate same-stream custody remains retained but has no candidate send API.

GF does not select or materialize:

- `MeshControlStream::send_frame(...)` for candidate publication;
- send-direction finish semantics;
- local candidate result-write error mapping;
- retry/re-encoding;
- fallback Rejected;
- response-loop continuation;
- peer-close policy after candidate response.

Those remain later separately gated boundaries.

## 15. No traversal activation

Although `ProductionReachabilityOwner` can own at most one Phase 141 traversal session, GF selects no traversal activation.

GF does not call or authorize:

- `provision_current_traversal(...)`;
- `poll_and_apply_current_reachability(...)`;
- traversal factory construction;
- network socket creation;
- target selection/dialing.

Initial `recover(...)` naturally restores durable plan/freshness with no traversal session, as existing owner source specifies.

## 16. No retirement/reload automation

GF does not automatically call:

- `retire_noncurrent_lifecycle(...)`;
- `reload_from_store()`.

Those existing owner operations remain explicit semantic actions under their established preconditions.

No background recovery worker, sweep, timer, TTL, retry loop or hidden lifecycle transition is selected.

## 17. Future source-materialization shape

After GF canonical closure, a separately gated source-materialization checkpoint may add only the minimum Agent-owned production-owner custody boundary needed to implement this selection.

Expected minimal shape:

- one generic Agent owner/custody type parameterized by `S: ReachabilityDurableStore` and `T: CandidatePublicationFreshnessTokenSource`;
- one fail-closed constructor/recovery seam consuming exact store, token source and expected peer;
- private retention of exactly one recovered `ProductionReachabilityOwner<S,T>`;
- one narrow crate-internal exclusive mutable access/composition seam sufficient for a later candidate execution adapter;
- focused tests for one-time recovery, exact failure preservation, no clone/duplicate owner, and no semantic execution during construction.

The source checkpoint must not itself wire GE candidate ingress into FY execution unless a separate semantics gate explicitly authorizes that composition.

## 18. Ownership/API constraints

Future custody source should prefer ownership transfer and narrow closures/methods over raw mutable-owner escape.

GF does not authorize public API expansion beyond what is required for crate-internal composition.

The durable store and token source should remain encapsulated inside `ProductionReachabilityOwner` rather than being separately exposed by the Agent wrapper.

No serialization or persistence-product API is added.

## 19. Error preservation

GF preserves existing typed production-owner errors.

In particular, future Agent custody construction must not flatten:

- `ReachabilityOwnerError::DurableStateMissing`;
- `ReachabilityOwnerError::Persistence(...)`;
- `ReachabilityOwnerError::Snapshot(...)`;
- retired/current/recovery mode distinctions.

A thin Agent wrapper may introduce one construction error wrapper only if required by ownership composition, but it must retain the exact owner error as a distinguishable source and may not translate failure into candidate wire `Rejected` at custody-construction time.

## 20. Thread/runtime boundary

GF does not require a new async runtime merely to recover the synchronous production owner.

Existing production-owner durable store/token source traits are synchronous at this source boundary.

GF does not authorize:

- spawning a task for owner recovery;
- moving the owner onto the existing live-owner Tokio runtime by assumption;
- wrapping it in an async mutex without a separately proven concurrency requirement;
- blocking an async executor on hidden network/database I/O beyond the existing trait contract.

Exact executor/concurrency placement remains a source-materialization concern and must stay minimal.

## 21. Validation scope

GF is semantics-selection only.

The authorized repository delta is exactly one new contract file.

No Rust/Kotlin source, Cargo/Gradle manifest, lockfile, workflow, configuration, deployment source, listener/readiness source or unrelated contract may change.

Exact-head CI is required before canonical closure.

## 22. Explicit non-goals

GF does not:

- materialize production-owner custody source;
- change current Mesh ingress;
- remove the candidate handoff-not-selected barrier;
- execute candidate publication;
- select or validate requester grant;
- mutate requester records;
- commit reachability state;
- write candidate response frames;
- activate traversal;
- bind/accept a listener;
- publish readiness;
- dial a target;
- deploy;
- restart/recover a process;
- merge;
- delete branches.

## 23. Closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SEMANTICS_SELECTION`

Canonical gate:

`C03E_GF_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SEMANTICS_SELECTED`

Canonical law:

**current-Mesh candidate execution cannot proceed from GE custody into FY semantics until Agent owns exactly one fail-closed production `ProductionReachabilityOwner<S,T>` recovered from existing authoritative durable state for the exact peer lifecycle; missing or ambiguous durable state creates no owner, live-owner authority is not a substitute, ordinary candidate requests do not reconstruct the owner, and GF performs no execution or response I/O.**

## 24. Successor rule

GF does not authorize runtime activation.

After GF source materialization closes, a fresh exact-head audit must determine the next prerequisite among:

- current authenticated logical-session + GE candidate request adaptation into dormant FY execution;
- any exact peer-to-production-owner lookup/selection requirement exposed by source materialization;
- current-Mesh terminal-result response-write custody;
- any earlier prerequisite exposed by exact source topology.

No later checkpoint may fabricate historical `AuthenticatedPrwcConnection` or `CandidatePublicationControlFrame` transport custody merely to reuse older APIs.