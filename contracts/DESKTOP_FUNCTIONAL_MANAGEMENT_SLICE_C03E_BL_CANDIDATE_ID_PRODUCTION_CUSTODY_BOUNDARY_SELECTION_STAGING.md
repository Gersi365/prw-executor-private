# Phase 152 C03e-BL — Candidate ID Production Custody Boundary Selection

Status: STAGED SELECTION

Gate target:
`C03E_BL_CANDIDATE_ID_PRODUCTION_CUSTODY_BOUNDARY_SELECTED`

## 1. Exact predecessor

Closed C03e-BK:
- branch: `phase-152-c03e-bk-candidate-id-high-water-observation-source-materialization-staging`;
- head: `28d532808ffa86af65454f2793452c85cfbf79e5`;
- tree: `23817f9423e8bab98446d361b9e521b52eb46268`;
- gate: `C03E_BK_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SOURCE_MATERIALIZED`.

BK materialized only read-only observation of the existing `PeerConnectivityPlan` candidate-ID high-water state. It did not materialize a candidate-ID allocator, reservation API, production endpoint provider, path-kind classifier, publication producer, networking activation, readiness, deployment or merge.

## 2. Exact audited source seams

BL is a docs-only selection checkpoint over the exact closed-BK source.

Audited source seams include:

1. `crates/prw-connectivity/src/lib.rs`
   - blob: `68635ae87735e6abb055cda21f8232f39b81a63e`;
   - `PeerConnectivityPlan` owns the plan-lifetime candidate-ID high-water state;
   - `refresh_candidates(...)` remains authoritative for retained-ID identity, no rebound/reuse, and monotonic high-water validation;
   - `candidate_id_high_watermark()` is observation only.

2. `crates/prw-remote-bridge/src/candidate_reachability.rs`
   - blob: `51b294cfb3772925651a05bdcb034cd051204efb`;
   - `AuthenticatedCandidatePublication` already contains `Vec<ConnectivityCandidate>`;
   - `publish_current_candidates(...)` validates a supplied candidate vector but does not issue candidate IDs;
   - `assemble_explicit_connectivity_candidate(...)` consumes an already-typed `CandidateId` and explicitly does not allocate it.

3. `crates/prw-remote-bridge/src/reachability_owner.rs`
   - blob: `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
   - `ReachabilityDurableSnapshot` persists the complete current `PeerConnectivityPlan` together with publication freshness;
   - `ProductionReachabilityOwner` owns one current plan snapshot and stages candidate publication against a clone of that exact plan;
   - `commit_candidate_publication(...)` applies `refresh_candidates(...)` before durable commit;
   - `ReachabilityDurableStore::compare_and_commit(...)` is required to be linearizable for one exact peer lifecycle;
   - stale or ambiguous durable state fails closed and requires authoritative recovery.

4. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`
   - production-seam tests construct `CandidateId` values in disposable fixture helpers before publication;
   - the owner admits/validates/commits those already-constructed candidates;
   - the fixture IDs are test data only and are not production allocation authority.

## 3. Audit conclusion: no production CandidateId producer exists

The exact closed-BK tree contains no selected production `CandidateId` allocator/custodian that issues identifiers for newly discovered candidates.

Existing components consume or validate already-supplied IDs:
- candidate assembly consumes an already-typed ID;
- authenticated publication carries already-built candidates;
- NAT traversal consumes already-built PRW candidates;
- relay consumes already-selected candidate semantics;
- Android native candidate IDs are disposable/fixed validation fixtures;
- `PeerConnectivityPlan` validates accepted history but does not issue an ID;
- the BK high-water accessor observes history but does not issue an ID.

Therefore BL must not pretend that an allocator already exists.

## 4. Selected production custody boundary

BL selects only this boundary rule:

> Production issuance or reservation of any new `CandidateId` for one exact peer connectivity-plan lifetime must be serialized against the authoritative current durable `PeerConnectivityPlan` history at or behind the `ProductionReachabilityOwner` / `ReachabilityDurableStore` compare-and-commit boundary.

This means a future production ID producer must not mint a new candidate ID solely from publisher-local, discovery-local, Android-local, NAT-local, relay-local, request-local or endpoint-local state.

The producer must have authoritative access to the exact current plan-lifetime history that enforces candidate-ID non-reuse and must participate in a serialization/commit path that cannot race an independently current writer into reusing or rebinding a historical ID.

## 5. What this selection does not claim

BL does **not** claim that `ProductionReachabilityOwner` currently allocates candidate IDs.

The current owner still:
- receives publications whose candidates already carry IDs;
- validates the publication against current identity/workspace/transport state;
- clones the exact current plan;
- applies transactional `refresh_candidates(...)` validation;
- issues a separate verifier-owned publication-freshness token;
- durable-CAS commits the complete replacement snapshot;
- fails closed on stale/ambiguous durable state.

A future candidate-ID issuance seam may be implemented inside the owner, behind the durable store, or through another bounded component that is serialized by the same authoritative durable lifecycle. BL selects the custody/serialization requirement only, not the concrete class/module/API placement.

## 6. Publisher and discovery are not allocation authority

`AuthenticatedCandidatePublication` is not candidate-ID production authority merely because it carries candidate IDs.

Candidate discovery/provider logic is also not automatically candidate-ID production authority merely because it observes or constructs:
- a `SocketAddr`;
- a `ConnectivityEndpoint`;
- a STUN/server-reflexive address;
- an ICE candidate class;
- a relay route;
- a successful reachability observation.

Those components may eventually request an ID from the selected authority path, but they may not autonomously define plan-lifetime identifier history.

## 7. High-water observation is necessary state, not an allocation algorithm

Closed BK makes the plan-lifetime high-water observable as:

```rust
candidate_id_high_watermark(&self) -> Option<CandidateId>
```

BL does not convert that observation into the rule:

```text
next_id = high_water + 1
```

That arithmetic remains unselected.

The high-water value is authoritative historical anti-reuse state inside the plan. A future issuance algorithm must respect it, but BL does not select how the next ID is numerically chosen, reserved, encoded, persisted or recovered.

## 8. Concurrency requirement

Because multiple process/owner instances could otherwise race on the same peer lifecycle, production ID issuance must not rely only on an in-process mutex, local counter or cached high-water observation.

The existing durable reachability boundary already requires linearizable expected-current compare-and-commit for the exact peer lifecycle. BL selects that authoritative serialization domain as the boundary against which future issuance/reservation must be coordinated.

A stale or ambiguous durable state must not authorize issuance from cached history.

BL does not select a specific transaction shape for issuance.

## 9. Lifecycle scope remains exact

`CandidateId` remains plan-scoped candidate correlation only under the current connectivity semantics.

BL does not promote it to:
- `DeviceId`;
- `TransportIdentity`;
- `SessionId`;
- PRWM request ID;
- global workspace identity;
- globally durable object identity;
- relay-session identity;
- ICE foundation/component identity;
- database primary key.

The selected custody rule applies only to preserving valid candidate-ID history for the exact authoritative peer connectivity-plan lifecycle.

## 10. Freshness and candidate ID remain separate

`CandidatePublicationFreshnessToken` is verifier-owned publication-currentness state and remains semantically distinct from `CandidateId`.

A freshness token:
- must not be derived from candidate IDs;
- does not allocate a candidate ID;
- does not replace plan-lifetime high-water history.

A candidate ID:
- does not prove publication freshness;
- does not prove registry currentness;
- does not prove reachability;
- does not prove public routability.

BL introduces no mapping between these domains.

## 11. Path-kind provenance remains separately gated

`ConnectivityPathKind` remains explicit product classification only.

BL does not select whether a future candidate is `LocalDirect`, `InternetDirect` or `Relay`, and does not infer path kind from:
- IP shape/scope;
- interface name;
- observed bound address;
- STUN result;
- ICE host/server-reflexive class;
- relay availability;
- reachability success.

Candidate-ID custody selection must not accidentally become path-kind classification authority.

## 12. Production endpoint provenance remains separately gated

BL does not select:
- interface enumeration;
- host/LAN/public address discovery;
- STUN provider ownership;
- TURN/relay provider ownership;
- endpoint publication scheduling;
- rendezvous/broker discovery;
- public-routability proof.

An endpoint may be discovered before an ID is requested, but endpoint discovery is not itself candidate-ID authority.

## 13. No restart/persistence policy is inferred

The durable reachability snapshot currently carries the complete `PeerConnectivityPlan`, including its internal candidate-ID history while that exact snapshot is persisted.

BL does not extrapolate this into a final product policy for:
- process restart;
- host reboot;
- lifecycle replacement;
- transport rotation;
- database migration;
- snapshot loss/corruption;
- disaster recovery;
- durable retention duration;
- candidate-ID history reset rules.

Those lifecycle/persistence semantics require separate selection before a production allocator can be considered complete.

## 14. Explicit non-selections

BL does not select or materialize:
- a concrete candidate-ID allocator;
- `high_water + 1` or any other numeric algorithm;
- random/UUID/database-sequence allocation;
- reservation count or batching;
- leases for candidate IDs;
- overflow/wraparound behavior;
- restart/recovery/reset semantics;
- distributed uniqueness beyond the exact plan lifetime;
- allocator wire protocol;
- allocator database schema/product;
- candidate path-kind classifier/provenance;
- endpoint discovery/provider custody;
- candidate publication scheduling;
- publication freshness/currentness changes;
- STUN/ICE/TURN/relay activation;
- private DNS or server-pushed DNS activation;
- exit-node/full-tunnel/NAT routing;
- registry/provider mutation;
- expected-device provenance;
- `SessionId` or request-ID production;
- remote operation dispatcher/provider backends;
- Agent `main.rs` activation;
- readiness/process-exit policy;
- systemd/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 15. Exact intended BK -> BL scope

BL must differ from closed BK by exactly one docs-only path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BL_CANDIDATE_ID_PRODUCTION_CUSTODY_BOUNDARY_SELECTION_STAGING.md`

No Rust/Kotlin/Java/Cargo/lockfile/workflow/Agent/registry/provider/networking/packaging/systemd/deployment source change is authorized by this checkpoint.

## 16. Validation requirements

BL can close only after:
- exact BK predecessor lineage remains unchanged;
- exact BK -> BL compare is one docs-only path only;
- no production source blob changes relative to closed BK;
- canonical Rust validation on the exact final BL head reaches terminal success;
- every other automatically triggered workflow reaches a terminal non-failing verdict;
- immutable audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes the fresh predecessor guard for closed BK, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No allocator/source/network/readiness/deployment behavior is authorized merely by BL closure.

Gate target remains:
`C03E_BL_CANDIDATE_ID_PRODUCTION_CUSTODY_BOUNDARY_SELECTED`
