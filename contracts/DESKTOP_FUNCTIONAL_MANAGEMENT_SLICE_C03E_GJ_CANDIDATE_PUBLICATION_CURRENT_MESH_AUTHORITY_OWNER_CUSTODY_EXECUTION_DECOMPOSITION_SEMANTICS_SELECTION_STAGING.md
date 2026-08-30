# Phase 152 C03e-GJ — Candidate Publication Current-Mesh Authority / Owner-Custody Execution Decomposition Semantics Selection

Status: VALIDATING

Target gate:
`C03E_GJ_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SEMANTICS_SELECTED`

## 1. Exact predecessor

Canonically CLOSED C03e-GI is the authoritative predecessor:

- branch: `phase-152-c03e-gi-candidate-publication-production-reachability-owner-authenticated-peer-mapping-lookup-source-materialization-staging`;
- final head: `36c27bb630a09d743c2da96c7ae3cde18eca26f5`;
- final tree: `8434bc5ebdf21f9f67fb5c04844863e60da8ab35`;
- exact predecessor GH: `4f464556cb109a1c4db9a85678fc9f397afb1785`;
- PR #311: `Status: CLOSED`, draft/open/unmerged;
- immutable GI Drive object: `1WpukIK_OXXmDdd1AaNTprzE03w5sJdSy`;
- GI closure: `CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_LOOKUP_SOURCE_MATERIALIZATION`;
- GI gate: `C03E_GI_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_LOOKUP_SOURCE_MATERIALIZED`.

GI remains frozen. GJ starts exactly from the GI final commit and does not amend any closed predecessor.

## 2. Fresh exact-GI-head source finding

The exact GI source topology exposes four independently correct boundaries that cannot yet be composed directly without a new semantics selection.

### 2.1 Current-Mesh same-stream candidate custody already exists

C03e-GE materialized `PostAuthCandidatePublicationTransaction` in `prw-remote-bridge`.

That transaction retains:

- one strict `CandidatePublicationMeshRequest`;
- its exact peer-originated non-zero outer request ID as correlation only;
- one already-decoded `CandidatePublicationWireSubmission`;
- the exact same already-accepted `MeshControlStream` by value.

The bridge transaction exposes only request borrowing and by-value `into_parts()` custody transfer. It exposes no semantic execution or send surface for candidate publication.

### 2.2 Agent candidate handoff remains deliberately blocked

At exact GI head, `AuthenticatedRemoteSessionRuntimeOwner::process_one_post_auth_control_stream_ingress(...)` recognizes the current-Mesh candidate family but still returns:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

The transaction is not reinterpreted as capability or requester/rendezvous traffic. No second read occurs.

### 2.3 Production owner lookup is exact but synchronously borrow-bounded

C03e-GI materialized:

`ProductionReachabilityOwnerCustodyMap<S,T>::with_owner_mut_for_peer(...)`

Its lookup key is exact `PeerConnectivityIdentity` equality over both:

1. authenticated logical `DeviceId`;
2. `TransportIdentity`.

The successful operation is a higher-ranked synchronous closure over one exact `&mut ProductionReachabilityOwner<S,T>`.

That higher-ranked return type deliberately prevents a reference or future borrowing the production owner from escaping the lexical lookup operation.

### 2.4 Existing post-commit candidate execution is async and historical-envelope-shaped

`SharedRequesterRendezvousAuthority::execute_authenticated_candidate_publication_with_post_commit_cleanup(...)` is async because it:

1. performs publisher/candidate admission;
2. asynchronously acquires requester/rendezvous authority to select one exact current grant;
3. releases requester authority;
4. performs one synchronous durable production-owner commit;
5. only after definite commit success asynchronously reacquires requester authority for exact record cleanup.

Its current signature accepts historical:

- `AuthenticatedPrwcConnection`;
- `CandidatePublicationControlFrame`.

Current-Mesh source does not own those historical envelope types and must not fabricate them.

## 3. Additional current-authority constraint

`SharedCurrentCapabilityAuthority<P>::with_current_authority(...)` acquires the current registry/policy read lock asynchronously, but its operation closure is deliberately synchronous and its result cannot borrow the registry or policy.

Therefore current source does not provide an authorized way to:

- hold a `WorkspaceDeviceRegistry` borrow across requester-authority `.await`;
- return an async future borrowing current registry state;
- clone a per-request registry snapshot;
- expose an `RwLockReadGuard`;
- keep current registry custody while waiting on unrelated requester authority.

This is intentional existing lock/lifetime law and remains preserved by GJ.

## 4. Direct composition options rejected by the exact source audit

The following shortcuts are rejected.

### 4.1 No historical PRWC fabrication

GJ does not authorize creating an `AuthenticatedPrwcConnection` merely to satisfy the existing historical execution function.

Current-Mesh authenticated identity already exists in the retained `BoundRemoteSession` / authenticated application session. Fabricating a historical connection envelope would duplicate transport ownership semantics and blur current-Mesh versus historical pre-Mesh custody.

### 4.2 No historical candidate Command fabrication

GJ does not authorize wrapping `CandidatePublicationMeshRequest` into a historical `prw_control_transport::ControlFrame` or `CandidatePublicationControlFrame` solely to reuse CQ/FY signatures.

The current-Mesh request has already passed its own exact outer-kind and PRWP decoding law. Re-encoding and re-decoding into a historical envelope would add a second authority representation and unnecessary failure surface.

### 4.3 No async future escaping GI owner lookup

GJ does not widen `with_owner_mut_for_peer(...)` so a future borrowing `ProductionReachabilityOwner` can escape the lexical closure.

Production-owner mutable custody must remain bounded to the exact synchronous durable-commit operation.

### 4.4 No registry guard across requester-authority await

GJ does not select holding `SharedCurrentCapabilityAuthority`'s read guard while awaiting requester/rendezvous authority.

### 4.5 No requester-authority guard across registry/owner commit

The existing FY lock ordering remains mandatory: requester authority is released before durable reachability commit and is reacquired only after definite commit success for exact cleanup.

## 5. Selected decomposition

GJ selects a current-Mesh execution decomposition with distinct owned/synchronous/async phases rather than one monolithic borrow graph.

The selected semantic order is:

1. retain current-Mesh candidate transaction and exact same stream;
2. derive authenticated publisher provenance from the retained authenticated application session only;
3. perform current publisher/transport/candidate admission under one bounded current-registry read;
4. release that current-registry read;
5. asynchronously select one exact current requester/rendezvous grant;
6. verify exact expected-publisher equality;
7. perform a fresh current-registry read and exact GI owner lookup;
8. inside that same synchronous lexical commit operation, revalidate and durably commit through the existing production owner;
9. release production-owner borrow and current-registry read;
10. only after definite durable commit success, asynchronously perform exact requester-record cleanup;
11. preserve semantic result and cleanup disposition separately;
12. retain the original current-Mesh transaction/stream custody for later separately gated result framing and response I/O.

No phase may infer authority from the request ID or lower transport bytes.

## 6. Publisher identity source

The current-Mesh publisher logical identity comes only from the exact authenticated application session retained by `AuthenticatedRemoteSessionRuntimeOwner` through its bound session.

GJ does not authorize publisher identity from:

- current-Mesh request ID;
- PRWP payload bytes other than typed submission fields;
- socket address;
- lower QUIC connection identity bytes as a logical identity substitute;
- requester grant;
- owner-map entry;
- candidate endpoint;
- historical PRWC envelope fabrication.

The authenticated session may be cloned only as the existing owned session-domain value required by current semantic publication construction. Such a clone is not reauthentication or a new authority snapshot.

## 7. First current-registry read: owned publication admission

The first current-authority operation is selected to invoke the existing semantic admission equivalent of:

`publish_current_candidates(registry, authenticated_publisher_session, submission.presented_transport_identity(), submission.candidates().to_vec())`

Successful return yields one owned `AuthenticatedCandidatePublication`.

That owned publication retains:

- the authenticated publisher session snapshot;
- exact `PeerConnectivityIdentity` derived from authenticated publisher `DeviceId` plus submission `presented_transport_identity()`;
- the validated bounded candidate vector.

The first current-registry guard is released immediately after this owned publication is produced.

The publication remains a semantic snapshot, not durable commit authority. A later fresh current-registry read is mandatory before commit.

## 8. Exact GI lookup key source

The production-owner lookup key is exactly:

`publication.peer()`

This is equivalent to the already-selected GH/GI law:

`PeerConnectivityIdentity::new(authenticated publisher DeviceId, submission.presented_transport_identity())`

No alternative lookup key is allowed.

In particular GJ forbids:

- `BoundRemoteSession::transport_identity()` substitution;
- DeviceId-only lookup;
- transport-only lookup;
- lookup by request ID;
- lookup by requester identity;
- lookup by candidate endpoint;
- single-owner fallback;
- same-device alternate-transport fallback.

## 9. Requester/rendezvous grant phase

After the first current-registry read has been released, one exact current requester/rendezvous grant may be selected asynchronously for:

`publication.peer().device_id()`

The existing requester/rendezvous authority remains the sole source of that grant.

Grant issuance alone does not consume or clean up the provider record.

The expected publisher carried by the grant must exactly equal `publication.peer().device_id()` before any owner commit is attempted.

Mismatch remains fail-closed and performs no durable commit or cleanup.

## 10. Fresh commit-time currentness is mandatory

Because the first registry borrow cannot and must not span requester-authority waiting, GJ explicitly selects a second fresh current-registry read before durable commit.

This second read is not an optimization artifact; it is the commit-time currentness authority.

Inside that read, the existing production owner must revalidate requester, publisher, workspace and exact target transport currentness through its existing `commit_candidate_publication(...)` path.

Therefore registry drift between initial publication construction and commit is fail-closed at commit rather than being silently hidden behind a stale per-request registry snapshot.

No cloned `WorkspaceDeviceRegistry` snapshot is selected.

## 11. GI owner lookup and commit are one synchronous lexical operation

Within the second current-registry read, the exact GI map performs:

`with_owner_mut_for_peer(publication.peer(), operation)`

The operation may perform only the existing synchronous production-owner commit using:

- the fresh current registry reference;
- the exact requester session from the selected grant;
- the owned authenticated candidate publication;
- the exact submission `presented_freshness()` value.

No `.await` occurs while the exact `ProductionReachabilityOwner` mutable borrow exists.

No raw owner, custody, map entry, store, token source or guard escapes.

## 12. Outer map custody versus inner owner borrow

A later source adapter may require exclusive `&mut ProductionReachabilityOwnerCustodyMap<S,T>` custody while awaiting acquisition of the second current-registry read because current source has no shared map synchronization primitive.

GJ permits only that narrow outer-map exclusivity if required by Rust lifetime composition.

It does not equate outer map custody with an escaped production-owner borrow.

GJ does not select:

- `Arc<Mutex<ProductionReachabilityOwnerCustodyMap<...>>>`;
- Tokio mutex around the map;
- per-peer mutexes;
- actor/mailbox ownership;
- background owner tasks;
- lock-free maps;
- concurrent candidate execution across peer entries.

Any runtime concurrency/scaling ownership remains separately gated.

## 13. Owner lookup failure remains Agent-local and pre-commit

GI lookup failures are not existing bridge candidate semantic errors.

A later current-Mesh adapter must preserve exact distinction between:

- zero exact owner matches: `ProductionReachabilityOwnerCustodyLookupError::Missing`;
- multiple exact owner matches: `ProductionReachabilityOwnerCustodyLookupError::Ambiguous`.

Neither may be flattened into:

- `CandidatePublicationExecutionError::RequesterAuthority`;
- `CandidatePublicationExecutionError::Reachability`;
- generic wire `Rejected` before result-mapping semantics are separately selected;
- retry or fallback owner selection.

A narrow Agent-side execution wrapper error may compose GI lookup failure beside existing `CandidatePublicationExecutionError` without changing either lower taxonomy.

## 14. Existing semantic error taxonomy remains authoritative

For phases already represented by `CandidatePublicationExecutionError`, GJ preserves the existing classifications:

- publisher/transport/candidate admission -> `Candidate`;
- requester authority failure -> `RequesterAuthority`;
- expected publisher mismatch -> `ExpectedPublisherMismatch`;
- production reachability commit failure -> `Reachability`.

GJ adds no new lower bridge semantic meaning.

The current-Mesh Agent adapter may wrap these existing classes only to add the separate GI lookup class.

## 15. Definite commit remains the cleanup trigger

Exact FY law remains unchanged.

No cleanup occurs on:

- initial current-registry admission failure;
- requester authority failure;
- expected publisher mismatch;
- GI owner `Missing`;
- GI owner `Ambiguous`;
- production-owner currentness/admission failure;
- stale freshness;
- token-source failure;
- durable stale expected value;
- ambiguous persistence failure;
- any other pre/at-commit error.

Only definite `Ok(ReachabilityCommitOutcome)` permits requester authority to be reacquired for exact `retire -> remove_retired` cleanup.

## 16. Cleanup failure does not rewrite committed success

After definite commit success, exact requester-record cleanup disposition remains independent:

`Result<(), RequesterRendezvousLifecycleError>`

Cleanup failure cannot:

- roll back durable reachability state;
- restore previous freshness;
- reactivate a requester record;
- trigger a second commit;
- become `CandidatePublicationExecutionError`;
- automatically become wire-level `Rejected`;
- trigger retry/reconnect.

The existing FY/GA semantic separation remains authoritative.

## 17. Current-Mesh transaction custody is retained across semantic execution

GJ does not consume or discard the exact `PostAuthCandidatePublicationTransaction` merely to run semantics.

A later source handoff may own that transaction while semantic execution borrows only the typed request data needed for publication construction/correlation.

The exact same `MeshControlStream` remains in the transaction lineage until a separately gated current-Mesh result-frame/write checkpoint consumes it.

No stream clone, duplicate read, replacement stream, reaccept, or historical stream wrapper is selected.

## 18. Request ID remains correlation only

The exact current-Mesh request ID remains unchanged correlation state.

It is not used for:

- publisher identity;
- owner lookup;
- requester authority;
- registry validation;
- transport ownership;
- freshness authority;
- durable keying;
- cleanup identity.

GJ performs no result-frame construction, so it does not yet select how that request ID is encoded into a current-Mesh terminal response beyond requiring exact preservation for the later framing checkpoint.

## 19. Existing historical CQ/FY source remains byte-semantically authoritative for its own path

GJ does not delete, rewrite, or activate the historical pre-Mesh APIs:

- `execute_authenticated_candidate_publication(...)`;
- `SharedRequesterRendezvousAuthority::execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`;
- `CandidatePublicationControlFrame`;
- `AuthenticatedPrwcConnection`.

The later current-Mesh source adapter must reuse their underlying semantic laws, not fabricate their envelope ownership.

A bounded internal refactor may be selected later only if it preserves historical behavior and tests while exposing envelope-neutral owned semantic phases.

## 20. Current-authority lock ordering

The selected lock/borrow order is fail-closed and non-nested across unrelated async authorities:

1. first `SharedCurrentCapabilityAuthority` read -> owned publication -> release;
2. requester/rendezvous mutex -> exact grant -> release;
3. second `SharedCurrentCapabilityAuthority` read;
4. inside that synchronous read only, exact GI map lookup -> exact production-owner mutable borrow -> durable commit -> release owner/map operation -> release current-authority read;
5. after definite commit only, requester/rendezvous mutex -> exact cleanup -> release.

No requester mutex and current-registry guard are held simultaneously.

No production-owner mutable borrow exists across an await.

No response I/O occurs while any authority guard or production-owner borrow is held.

## 21. Cancellation semantics are not selected

GJ does not define cancellation behavior at intermediate semantic phases.

In particular it does not decide whether a future integrated worker may be cancelled:

- after initial publication admission;
- after requester grant selection but before commit;
- while awaiting current-registry read;
- after commit but before cleanup;
- during cleanup.

Those runtime cancellation boundaries may have lifecycle consequences and remain separately gated.

The dormant source materialization following GJ must not be integrated into a cancellation-aware active worker unless cancellation semantics are separately selected.

## 22. Response framing remains separate

GJ does not adapt current-Mesh semantic results into historical `CandidatePublicationControlFrame` framing.

The existing GA/GC path proves semantic projection and historical frame composition, but current-Mesh response framing still requires a separately selected bridge-owned current-Mesh correlation/frame adapter using the exact `CandidatePublicationMeshRequest::request_id()`.

GJ performs no:

- Accepted/Rejected current-Mesh frame construction;
- same-stream write;
- send-direction finish;
- response I/O error mapping;
- fallback Rejected;
- retry/re-encode;
- peer close.

## 23. Runtime integration remains blocked

The existing `CandidatePublicationHandoffNotSelected` barrier remains in place during GJ.

GJ is selection-only and does not alter:

- `process_one_post_auth_control_stream_ingress(...)`;
- repeated mixed-family ingress loop;
- requester-aware worker;
- capability-only worker;
- remote-session executor;
- endpoint lifecycle;
- Agent main/bootstrap;
- listener/readiness publication.

## 24. Production owner population remains separate

GJ operates semantically on an already-composed GI custody map.

It does not select or invoke:

- startup recovery schedule;
- per-peer durable owner population;
- owner insertion/removal after transport rotation;
- lifecycle retirement scheduling;
- `reload_from_store()` orchestration;
- process recovery;
- owner-map synchronization.

## 25. No dynamic-network invariant regression

The project dynamic-network identity invariant remains mandatory.

GJ never treats a fixed IP/socket endpoint as stable device identity.

Candidate publication continues to represent dynamic reachable endpoint information under the exact logical-device plus transport lifecycle identity and current registry authority.

## 26. Source-materialization target selected after GJ

If GJ closes canonically and a fresh source audit shows no drift, the next source checkpoint may materialize only the minimum dormant current-Mesh semantic decomposition required by this contract.

Likely source responsibilities include:

- envelope-neutral candidate publication preparation from authenticated session + `CandidatePublicationWireSubmission`;
- exact Agent-side composition of existing requester grant ordering with GI exact owner lookup;
- fresh commit-time current-authority revalidation;
- preservation of existing post-commit cleanup separation;
- narrow typed wrapping of GI lookup error beside existing semantic error;
- focused tests for lock/borrow ordering and no historical-envelope fabrication.

The exact file/path set must be re-audited before source mutation. GJ does not pre-authorize a broad refactor.

## 27. Focused validation expectations for later source materialization

A later source checkpoint must prove at minimum:

1. current-Mesh execution does not construct `AuthenticatedPrwcConnection`;
2. current-Mesh execution does not construct or decode a historical `CandidatePublicationControlFrame`;
3. request ID never enters publisher/owner/requester authority;
4. initial admission derives peer from authenticated session + submitted transport;
5. requester authority is released before durable commit;
6. commit uses a fresh current-registry read after requester grant selection;
7. exact GI `Missing` and `Ambiguous` remain distinguishable;
8. no production-owner borrow crosses `.await`;
9. definite commit alone triggers cleanup;
10. cleanup failure preserves committed semantic success;
11. exact current-Mesh transaction/stream custody remains available for later response handling;
12. no response I/O or runtime activation occurs.

## 28. GJ authorized path set

GJ is docs-only.

Exactly one path is authorized:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GJ_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SEMANTICS_SELECTION_STAGING.md`

No Rust source, `Cargo.toml`, `Cargo.lock`, workflow, Android/Kotlin/Gradle, listener, ingress, bootstrap, readiness, persistence, deployment, configuration, or other contract path is authorized.

## 29. GJ validation requirements

Canonical closure requires exact-final-head evidence for:

- exact GI merge base;
- ahead-only branch state;
- exactly one changed path;
- contract-only delta;
- no source/manifest/lock/workflow/runtime changes;
- locked dependency graph PASS when Rust workflow is triggered;
- rustfmt PASS when Rust workflow is triggered;
- Clippy PASS when Rust workflow is triggered;
- workspace tests PASS when Rust workflow is triggered;
- workspace build PASS when Rust workflow is triggered;
- Android recorded only if actually triggered;
- AD/AE and any path-filtered workflow recorded exactly as observed;
- immutable Drive raw byte/SHA equality.

Any correction creates a new exact final head and supersedes earlier validation evidence.

## 30. Immutable evidence procedure

After exact-final-head CI is terminal and acceptable:

1. freeze GJ source/contract state;
2. record final commit/tree/compare/path/blob evidence;
3. create one immutable GJ audit locally;
4. compute exact local bytes and SHA-256;
5. upload directly to canonical Drive folder `Private Remote Workspace` / `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch that exact Drive object;
7. recompute byte count and SHA-256 and require equality;
8. only then update the GJ PR body to `Status: CLOSED` while keeping it draft/open/unmerged;
9. independently re-read PR and branch state.

No My Drive root upload is permitted.

## 31. Explicit GJ non-goals

GJ does not:

- source-materialize the selected decomposition;
- remove `CandidatePublicationHandoffNotSelected`;
- execute a live current-Mesh candidate publication;
- fabricate historical PRWC or candidate Command ownership;
- add map synchronization;
- recover/populate production owners;
- construct a current-Mesh candidate response;
- write/send/finish a candidate response stream;
- resume a repeated loop after candidate handling;
- select candidate-session cancellation semantics;
- activate traversal;
- bind/accept a listener;
- publish readiness;
- dial;
- deploy;
- restart/recover a process;
- merge;
- delete branches;
- change repository visibility.

## 32. Canonical selected law

**For current-Mesh candidate publication, semantic execution must be decomposed so the exact authenticated application session and strict PRWP submission first produce an owned registry-current `AuthenticatedCandidatePublication`; requester/rendezvous grant selection then occurs with no current-registry guard or production-owner borrow held; durable commit occurs only under a fresh current-registry read and one exact GI peer-keyed synchronous production-owner borrow; no production-owner borrow crosses an await, no requester-authority guard spans commit, definite commit alone permits exact requester-record cleanup, cleanup remains independent from committed semantic success, historical PRWC/Command envelopes are never fabricated, request ID remains correlation only, and the original current-Mesh same-stream transaction remains retained for separately gated result framing/I/O.**

## 33. Canonical closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SEMANTICS_SELECTION`

Canonical gate:

`C03E_GJ_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SEMANTICS_SELECTED`

## 34. Successor rule

After canonical GJ closure, perform a fresh exact-final-head source audit before naming or materializing the source successor.

The likely next checkpoint is a narrow source materialization of this dormant current-Mesh authority/owner-custody execution decomposition. Current-Mesh terminal result framing/write custody, worker-loop integration, cancellation semantics, production-owner population/runtime ownership, traversal, listener/readiness, dialing, deployment, restart/recovery and merge remain separately gated.