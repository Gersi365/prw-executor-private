# Phase 152 C03e-BL — Candidate ID Production Authority Boundary Selection

Status: STAGED SELECTION

Gate target:
`C03E_BL_CANDIDATE_ID_PRODUCTION_AUTHORITY_BOUNDARY_SELECTED`

## 1. Exact predecessor

Closed C03e-BK:
- branch: `phase-152-c03e-bk-candidate-id-high-water-observation-source-materialization-staging`;
- head: `28d532808ffa86af65454f2793452c85cfbf79e5`;
- tree: `23817f9423e8bab98446d361b9e521b52eb46268`;
- gate: `C03E_BK_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SOURCE_MATERIALIZED`.

BK materialized only read-only observation of the existing `PeerConnectivityPlan` candidate-ID high-water state. It did not select an allocator, persistence/restart policy, distributed uniqueness, path-kind provenance, candidate publication transport, networking activation, readiness, deployment or merge.

## 2. Repository audit result

The exact BK head contains no production `CandidateId` allocator/custody authority that can be reused without introducing a new semantic responsibility.

The relevant current boundaries are:

- `CandidateId::new(u64)` validates representation only: non-zero is required. It is not an allocator.
- `PeerConnectivityPlan::new(...)` initializes historical high-water from an already-constructed candidate vector.
- `PeerConnectivityPlan::refresh_candidates(...)` remains the final plan-local anti-reuse/rebinding validator and advances historical high-water only after a valid refresh.
- `PeerConnectivityPlan::candidate_id_high_watermark()` exposes the accepted historical high-water state read-only.
- `assemble_explicit_connectivity_candidate(...)` consumes an already-produced `CandidateId`; it does not allocate one.
- `AuthenticatedCandidatePublication` contains already-constructed candidates.
- `publish_current_candidates(...)` validates authenticated publisher provenance and a complete already-ID-bearing candidate set.
- `ProductionReachabilityOwner::commit_candidate_publication(...)` stages the current plan and delegates candidate-ID anti-reuse/rebinding enforcement to `PeerConnectivityPlan::refresh_candidates(...)`; it does not issue candidate IDs.
- Phase 141 NAT traversal correlates already-existing candidate IDs and does not allocate product candidate IDs.
- Phase 136 relay consumes an already-selected candidate ID and does not allocate product candidate IDs.
- Android native candidate IDs are disposable/fixed validation fixtures, not production authority.
- `DisposableRelayService::allocate_handle(...)` allocates `RelayProviderHandle`, not `CandidateId`, and is explicitly disposable provider state.
- publication freshness tokens and distributed live-owner fencing generations are distinct currentness/fencing state and are not candidate identifiers.

Therefore BL must not reinterpret any of those existing values or components as a production candidate-ID allocator.

## 3. Selected authority boundary

BL selects only this ownership boundary:

> Production `CandidateId` issuance is a distinct, separately reviewed authority/coordination responsibility that occurs before explicit candidate assembly. The produced ID is proposed into the existing typed candidate path; `PeerConnectivityPlan` remains the independent final authority for whether that proposed ID is admissible in the current plan lifetime.

The selected ordering is:

```text
authoritative candidate-ID production / custody
    -> CandidateId
    -> explicit candidate assembly
    -> authenticated candidate publication
    -> current-plan refresh validation
    -> durable publication commit
```

This ordering deliberately separates **production** from **admission**:

- the producer/custodian is responsible for supplying a candidate identifier from an authoritative source;
- `CandidateId::new(...)` remains representation validation only;
- assembly remains typed composition only;
- authenticated publication remains publisher/identity provenance only;
- `PeerConnectivityPlan::refresh_candidates(...)` remains the final anti-reuse/rebinding admission authority;
- durable publication commit remains freshness/currentness persistence authority only.

No producer result can bypass plan validation.

## 4. High-water relationship

The existing high-water observation is an admissibility constraint signal, not an allocator algorithm.

A future concrete producer must be designed so that proposed new identifiers can satisfy the current plan's anti-reuse rules. BL does **not** select how the producer obtains or synchronizes the current high-water state.

In particular, BL does not select:

```text
next = high_water + 1
```

or any other numeric rule.

The source of truth for whether a proposed identifier is valid remains `PeerConnectivityPlan::refresh_candidates(...)` on the current plan state.

## 5. Existing plan-scoped semantics remain unchanged

`CandidateId` remains plan-scoped candidate correlation state only.

BL does not widen it into:
- a logical device identity;
- a transport identity;
- an authentication/session identity;
- a globally unique database key;
- a cross-workspace identifier;
- a relay route token;
- a live-owner fence;
- a publication freshness token;
- a PRWM request identifier.

Existing anti-reuse semantics remain bounded to the plan lifetime unless a later separately reviewed contract explicitly selects stronger persistence/recovery semantics.

## 6. Exact non-authorities

The following are explicitly **not** selected as the production candidate-ID source:

- `CandidateId::new(...)`;
- `ConnectivityCandidate::new(...)`;
- `assemble_explicit_connectivity_candidate(...)`;
- `PeerConnectivityPlan::new(...)`;
- `PeerConnectivityPlan::refresh_candidates(...)`;
- `PeerConnectivityPlan::candidate_id_high_watermark()`;
- `ProductionReachabilityOwner` merely by virtue of owning the committed plan;
- `CandidatePublicationFreshnessTokenSource`;
- `CandidatePublicationFreshnessToken` bytes;
- `ReachabilityLiveOwnerFence` or etcd lease/revision/fencing state;
- `SessionId`;
- PRWM request IDs;
- STUN transaction IDs;
- ICE candidate foundation strings or ICE candidate classes;
- relay session IDs, relay route tokens or relay provider handles;
- endpoint IP addresses/ports or hashes derived from them;
- Android/JNI fixture constants;
- clocks/timestamps;
- random/UUID/database sequence generators that have not been separately selected.

## 7. No derivation from endpoint or path metadata

BL does not derive `CandidateId` from:
- `SocketAddr`;
- `ConnectivityEndpoint`;
- `ConnectivityPathKind`;
- IP scope;
- interface name/index;
- DNS names;
- STUN XOR-mapped addresses;
- ICE host/server-reflexive classes;
- relay allocation state;
- reachability observations.

Candidate correlation remains independent of transient endpoint/path state.

## 8. Path-kind provenance remains separately gated

`ConnectivityPathKind` remains explicit product classification only.

BL does not select a classifier or provenance source for `LocalDirect`, `InternetDirect`, or `Relay`.

No product path kind may be inferred merely from IP address shape, `SocketAddr`, ICE candidate class, successful connectivity, or relay availability.

## 9. Publication/currentness separation

Candidate-ID production does not prove publication freshness or currentness.

Existing publication ordering remains:
- current authenticated requester/publisher checks;
- workspace and exact peer identity checks;
- exact current publication freshness;
- complete staged plan validation, including candidate-ID rules;
- fresh verifier token;
- durable compare-and-commit;
- local current-state installation.

A candidate-ID producer does not gain authority over any of those checks.

## 10. Identity and security invariants

BL preserves:
- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower-transport certificate identity only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `ConnectivityPathKind` as product path classification only;
- `SessionId` as authentication correlation only;
- PRWM request IDs as message correlation only.

Candidate-ID production is not authentication, authorization, membership, currentness, liveness, reachability, public-routability or readiness evidence.

## 11. Explicit non-selections

BL does not select or materialize:
- a concrete candidate-ID allocator implementation;
- `high_water + 1` allocation;
- counter ownership;
- allocator persistence/restart/recovery semantics;
- cross-process/distributed uniqueness;
- database sequencing;
- random/UUID allocation;
- reservation/batching/leases;
- overflow/wrap policy;
- candidate-ID wire reservation protocol;
- producer-to-owner synchronization protocol;
- candidate-ID remapping by the server;
- path-kind classifier/provenance;
- endpoint discovery/interface enumeration;
- candidate publication transport or rendezvous protocol;
- STUN/ICE/TURN/relay activation;
- registry/provider/database mutation;
- Agent `main.rs` activation;
- readiness/process-exit policy;
- retry/reconnect/rebind/rebootstrap/replacement;
- systemd/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 12. Next materialization rule

A source-materialization successor to BL is allowed only if a concrete production candidate-ID source/custodian can be selected from authoritative project architecture without inventing persistence, distributed uniqueness, restart, or synchronization semantics.

If no such concrete source exists, the next work must remain a bounded audit/selection seam rather than fabricate an allocator.

A future source materialization must preserve all of these constraints:
- no client/request-controlled bypass of plan validation;
- no reuse of unrelated freshness/fence/session/request identifiers;
- no inference from endpoint/path metadata;
- no weakening of `PeerConnectivityPlan::refresh_candidates(...)` anti-reuse rules;
- no production networking activation merely from candidate-ID issuance.

## 13. Exact intended BK -> BL scope

The final BL branch must differ from closed BK only by this contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BL_CANDIDATE_ID_PRODUCTION_AUTHORITY_BOUNDARY_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent, remote-bridge implementation, registry/provider, packaging/systemd, networking or deployment change blocks BL closure.

## 14. Validation requirements

BL can close only after:
- exact BK predecessor lineage remains unchanged;
- exact BK -> BL compare is one docs-only path;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- immutable audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No allocator, path-kind classifier, production networking, readiness or deployment materialization is authorized merely by BL closure.

Gate target remains:
`C03E_BL_CANDIDATE_ID_PRODUCTION_AUTHORITY_BOUNDARY_SELECTED`
