# Phase 152 C03e-GH — Candidate Publication Production Reachability Owner Authenticated Peer Mapping / Handoff Consumption Semantics Selection

Status: VALIDATING

Target gate:
`C03E_GH_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_HANDOFF_CONSUMPTION_SEMANTICS_SELECTED`

## 1. Exact predecessor

Closed C03e-GG is the authoritative predecessor:

- branch: `phase-152-c03e-gg-candidate-publication-production-reachability-owner-custody-recovery-source-materialization-staging`;
- head: `e6b47949c91d190134ab7acd960aea30053caf4d`;
- tree: `2180e527243e5ffff8a9b40e79ae53ea5e060e0e`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SOURCE_MATERIALIZATION`;
- gate: `C03E_GG_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SOURCE_MATERIALIZED`;
- PR #309: `Status: CLOSED`, draft/open/unmerged.

GG remains frozen. GH starts exactly from the GG commit and does not amend any closed predecessor.

## 2. Fresh post-GG source finding

The exact GG head leaves the current-Mesh candidate transaction stopped at:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

GE already owns one strict current-Mesh candidate transaction:

`PostAuthCandidatePublicationTransaction`

which retains exactly:

1. one `CandidatePublicationMeshRequest`;
2. the exact same already-accepted `MeshControlStream` by value.

GG separately materializes Agent-owned:

`ProductionReachabilityOwnerCustody<S,T>`

around exactly one recovered:

`ProductionReachabilityOwner<S,T>`.

No source at exact GG head maps the current authenticated candidate transaction to the one exact production owner whose peer lifecycle may accept that publication.

That missing association is a prerequisite to current-Mesh semantic execution and therefore precedes any source materialization that removes `CandidatePublicationHandoffNotSelected`.

## 3. Exact peer identity is two-part

The existing production reachability owner is scoped to one exact:

`PeerConnectivityIdentity`

whose components are:

1. logical `DeviceId`;
2. `TransportIdentity`.

GH preserves those two authorities separately.

The logical device component MUST come only from the already-authenticated logical application session retained by the current Agent remote-session owner.

The transport component for candidate-publication target selection MUST remain the candidate submission's existing:

`presented_transport_identity()`.

GH does not substitute the `BoundRemoteSession` transport identity for that presented candidate-publication transport identity.

## 4. Why bound-session transport is not the candidate target key

Existing provider-neutral candidate semantics already call:

`publish_current_candidates(registry, publisher_session, submission.presented_transport_identity(), ...)`

That existing function:

1. revalidates the authenticated publisher session against current registry state;
2. separately revalidates the presented transport identity for that exact publisher device;
3. only after those checks constructs:
   `PeerConnectivityIdentity::new(publisher_device_id, presented_transport_identity)`.

Therefore current-Mesh adaptation may not silently replace `submission.presented_transport_identity()` with the transport identity on `BoundRemoteSession`.

The bound-session transport remains transport-binding evidence for that session's existing capability path. It is not selected by GH as the candidate-publication reachability-plan target.

## 5. Presented transport is nomination, not authority

Before existing registry validation succeeds, `submission.presented_transport_identity()` is caller-presented nomination only.

Using the typed value to form an exact lookup key MUST NOT be interpreted as:

- currentness proof;
- publisher authorization;
- transport ownership proof;
- freshness authority;
- requester/rendezvous authority;
- durable commit authority.

The existing registry validation inside candidate semantic execution remains mandatory and cannot be skipped merely because an owner custody entry exists for the same typed key.

## 6. Selected lookup key

For candidate-owner association, GH selects exactly one lookup key shape:

`PeerConnectivityIdentity::new(authenticated_publisher_device_id, submission.presented_transport_identity())`

where:

- `authenticated_publisher_device_id` comes only from the current authenticated logical session;
- `presented_transport_identity` comes only from the strict decoded candidate submission.

The outer PRWM request ID, candidate endpoints, requester grant, cleanup identity, peer socket/address, and lower transport bytes are not inputs to this key.

No local replacement `DeviceId`, request ID, transport identity, or peer key may be fabricated.

## 7. Selected cardinality

The future Agent owner-association boundary is keyed by the exact two-part `PeerConnectivityIdentity` and has cardinality:

**zero or one retained production-owner custody for one exact peer lifecycle key.**

Consequences:

- a lookup for an absent exact peer fails closed;
- a `DeviceId`-only lookup is forbidden;
- a transport-only lookup is forbidden;
- a lookup may not fall back from one presented transport identity to another transport identity for the same logical device;
- one custody may not be aliased as authority for two distinct `PeerConnectivityIdentity` keys;
- owner selection may not fall back to "the only owner currently present";
- duplicate/ambiguous ownership must not be normalized into arbitrary first-match behavior.

GH does not yet choose a concrete Rust collection type, synchronization primitive, capacity constant, persistence schema, or recovery/bootstrap schedule.

## 8. Existing GG custody remains the owner boundary

A successful future lookup yields bounded access to the existing:

`ProductionReachabilityOwnerCustody<S,T>`

for the exact peer key.

The future handoff must preserve GG's encapsulation:

- no raw production-owner getter;
- no store getter;
- no token-source getter;
- no clone;
- no `Arc`/mutex invention solely for this mapping;
- semantic mutation occurs, if separately authorized later, only through GG's bounded `with_owner_mut(...)` seam.

GH does not reopen GG recovery semantics and does not select per-command recovery.

## 9. Lookup is not semantic success

Finding a custody entry for the nomination key does not authorize candidate publication.

A later current-Mesh execution adapter must still preserve the existing provider-neutral semantic order, including:

1. current publisher-session registry validation;
2. current presented-transport validation for that publisher;
3. bounded candidate validation;
4. requester/rendezvous authority;
5. exact expected-publisher equality;
6. existing production-owner admission/freshness/durable commit.

If any existing stage rejects the publication, owner lookup cannot turn that failure into success.

## 10. Owner target equality remains mandatory

Existing production-owner admission requires the publication peer to match the owner's current plan peer exactly.

GH preserves that law.

A future adapter MUST NOT retarget an owner by rewriting:

- publisher `DeviceId`;
- presented transport identity;
- production plan peer;
- publication peer.

If semantic validation produces a peer that does not exactly match the selected owner's peer lifecycle, the existing fail-closed target mismatch remains authoritative.

## 11. Current-Mesh transaction consumption

GH selects one-way candidate handoff ownership semantics:

- the exact `PostAuthCandidatePublicationTransaction` is consumed by value at most once by the future candidate handoff adapter;
- its exact `CandidatePublicationMeshRequest` remains the structural/correlation source;
- its exact same `MeshControlStream` remains in that opaque transaction lineage;
- no clone, duplicate stream, second accept, second read, retry, or resynchronization is permitted.

GH itself is semantics selection only and does not remove `CandidatePublicationHandoffNotSelected` from source.

## 12. Current-Mesh request adaptation must not fabricate historical transport ownership

The historical provider-neutral public entry point still accepts:

`&AuthenticatedPrwcConnection`

and historical:

`CandidatePublicationControlFrame`.

Current Mesh instead owns:

- authenticated `BoundRemoteSession` / logical session state;
- `CandidatePublicationMeshRequest`;
- `MeshControlStream`.

GH does not authorize construction, wrapping, casting, or fabrication of:

- `AuthenticatedPrwcConnection`;
- historical `ControlTlsServerStream`;
- historical candidate `Command` frame;
- a fake historical request carrier.

A later separately gated execution-adaptation checkpoint must consume current-Mesh typed fields directly while preserving existing semantic authorities.

## 13. Correlation remains non-authorizing

The peer-originated current-Mesh request ID remains correlation state only.

It must not select:

- production-owner custody;
- publisher identity;
- transport identity;
- requester authority;
- freshness currentness;
- retry/replay behavior.

No replacement local request ID is allocated by GH.

## 14. Requester/rendezvous authority remains separate

Owner lookup does not replace or precompute requester/rendezvous authority.

The existing requester/rendezvous provider path remains separately authoritative for the requester session and expected publisher check during semantic execution.

No requester grant is used as the owner lookup key.

No requester cleanup state is used as the owner lookup key.

## 15. No current-Mesh response write selected

GH does not select or materialize:

- candidate terminal frame send;
- Accepted/Rejected write custody;
- local send error classification;
- send-direction finish;
- fallback Rejected;
- retry/re-encoding;
- peer-close policy;
- continuation to another candidate command.

The exact same stream remains retained for a later separately gated response-write boundary.

## 16. FY/GA/GC remain dormant

GH does not invoke or materialize current-Mesh calls to:

- `execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`;
- `project_candidate_publication_terminal_result(...)`;
- `compose_candidate_publication_terminal_result_frame(...)`.

Execution, post-commit cleanup, terminal semantic projection and frame composition remain dormant.

## 17. No owner recovery or runtime startup

GH does not call:

- `ProductionReachabilityOwnerCustody::recover(...)`;
- `ProductionReachabilityOwner::reload_from_store()`;
- reachability live-owner bootstrap;
- traversal provisioning/polling;
- listener bind/accept;
- readiness publication;
- target dialing;
- deployment;
- process restart/recovery.

It selects association semantics only for already-existing custody.

## 18. No concrete collection selected

Although the proven logical cardinality is zero-or-one custody per exact peer key, GH intentionally does not yet choose:

- `HashMap`;
- `BTreeMap`;
- slab/index table;
- mutex-protected registry;
- actor/mailbox;
- global singleton;
- persistent database table.

Source materialization must first inspect the existing Agent lifetime topology and choose the narrowest representation that preserves this selected law without inventing unnecessary concurrency or lifetime machinery.

## 19. Existing active worker key is not reused as owner authority

The current persistent remote-session worker collection is keyed by authenticated logical `DeviceId` for duplicate-active-session control.

That worker key has different semantics from candidate reachability owner identity because production owner identity also includes a transport identity.

GH therefore forbids treating the current `DeviceId` worker key as sufficient production-owner lookup authority.

Existing duplicate-active-device behavior remains unchanged.

## 20. No transport equality invented

GH does not require `submission.presented_transport_identity()` to equal `BoundRemoteSession::transport_identity()` merely because both typed values exist.

Any such equality rule would be a new semantic restriction not present in existing candidate publication admission.

Current registry validation remains the authority that decides whether the presented transport identity is current for the authenticated publisher device.

## 21. Exact failure posture for future materialization

Future source materialization must preserve distinct failure layers:

- current-Mesh structural/ingress failure;
- exact peer-owner lookup absence/ambiguity failure;
- existing candidate semantic execution failure;
- later cleanup lifecycle result;
- later terminal frame composition/wire failure;
- later response-write failure.

GH does not flatten one layer into another and does not select concrete enum names for layers not yet source-materialized.

An owner lookup failure must not be reframed as candidate semantic `Rejected`, because semantic execution has not occurred.

## 22. Authorized path set

GH is a semantics-selection checkpoint.

It is authorized to change exactly one path:

1. this GH contract.

No Rust source, Cargo manifest, `Cargo.lock`, workflow, Android/Kotlin/Gradle, ingress, listener, bootstrap, readiness, persistence, deployment, configuration or other contract path is authorized.

## 23. Validation requirements

Canonical closure requires exact-final-head evidence for:

- exact GG merge base;
- ahead-only branch state;
- exactly one changed path;
- no source/manifest/lock/workflow/runtime changes;
- Rust validation PASS if triggered for the exact head;
- Android validation recorded only if actually triggered;
- AD/AE and any other path-filtered workflows recorded exactly as observed, never inferred.

Any correction creates a new candidate head and supersedes prior validation evidence.

## 24. Immutable evidence

After exact-final-head CI is terminal and acceptable:

1. freeze GH source/contract state;
2. record final commit/tree/compare/path/blob evidence;
3. create immutable GH audit locally;
4. compute exact local bytes and SHA-256;
5. upload directly to canonical Drive folder `Private Remote Workspace` / `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch the exact Drive object;
7. recompute bytes/SHA and require equality;
8. update the GH PR to `Status: CLOSED` while keeping it draft/open/unmerged;
9. independently re-read PR and branch state.

No My Drive root upload is permitted.

## 25. Explicit non-goals

GH does not:

- source-materialize an owner collection/lookup;
- recover production owners;
- choose recovery startup order;
- remove `CandidatePublicationHandoffNotSelected`;
- source-materialize current-Mesh execution adaptation;
- fabricate historical PRWC ownership;
- execute candidate publication;
- mutate requester/rendezvous authority;
- commit reachability state;
- perform post-commit cleanup;
- project or encode a terminal result;
- write/send a candidate response;
- activate traversal;
- bind/accept a listener;
- publish readiness;
- dial;
- deploy;
- restart/recover a process;
- merge;
- delete branches.

## 26. Canonical selected law

**For current-Mesh candidate publication, production-owner selection is keyed by the exact `PeerConnectivityIdentity` formed from the authenticated publisher's logical `DeviceId` plus the strict candidate submission's presented `TransportIdentity`; that presented transport remains nomination until existing current-registry validation succeeds, lookup cardinality is zero-or-one custody per exact two-part peer lifecycle with no DeviceId-only or fallback aliasing, and the exact GE same-stream candidate transaction may later be consumed by value only through a separately gated adapter that preserves GG custody and all existing semantic authorities.**

## 27. Closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_HANDOFF_CONSUMPTION_SEMANTICS_SELECTION`

Canonical gate:

`C03E_GH_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_HANDOFF_CONSUMPTION_SEMANTICS_SELECTED`

## 28. Successor rule

After canonical GH closure, perform a fresh exact-final-head audit before source materialization.

The likely next prerequisite is the narrow Agent representation/source boundary for exact peer-keyed production-owner custody lookup. That successor must not pre-authorize current-Mesh semantic execution, response write, runtime activation or recovery startup merely because the association law is now selected.
