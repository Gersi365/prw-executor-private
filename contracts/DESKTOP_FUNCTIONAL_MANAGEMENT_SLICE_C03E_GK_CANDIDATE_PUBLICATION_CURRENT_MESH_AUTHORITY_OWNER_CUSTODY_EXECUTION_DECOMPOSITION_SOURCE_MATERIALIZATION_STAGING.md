# Phase 152 C03e-GK — Candidate Publication Current-Mesh Authority / Owner-Custody Execution Decomposition Source Materialization

Status: IMPLEMENTING

Target gate:
`C03E_GK_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Canonically CLOSED C03e-GJ is the authoritative predecessor:

- branch: `phase-152-c03e-gj-candidate-publication-current-mesh-authority-owner-custody-execution-decomposition-semantics-selection-staging`;
- final head: `311a190019ac0474d0996bb9817624e28f3ba89e`;
- final tree: `fade76c1dc42b5835e8d91d1a5d0d6971645a00e`;
- PR #312: `Status: CLOSED`, draft/open/unmerged;
- immutable GJ Drive object: `1AJqLhbuFV0AiJWaPjQnlPbgeafuxKZyx`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SEMANTICS_SELECTION`;
- gate: `C03E_GJ_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SEMANTICS_SELECTED`.

GJ remains frozen. GK starts exactly from the GJ final commit and does not amend any closed predecessor.

## 2. Fresh exact-GJ source audit

The post-GJ audit re-read the exact Agent source rather than assuming the source-materialization path set.

The audit established:

1. `SharedRequesterRendezvousAuthority` already owns the exact async requester grant selection and exact post-commit cleanup seams.
2. The same module already owns the historical FY candidate execution path and the GA/GC terminal projection/composition helpers.
3. `SharedCurrentCapabilityAuthority::with_current_authority(...)` is accessible from this sibling module and preserves a synchronous bounded operation after async read-lock acquisition.
4. `ProductionReachabilityOwnerCustodyMap<S,T>` already provides the exact GI synchronous peer-keyed owner lookup; no map change is required.
5. `ProductionReachabilityOwner::commit_candidate_publication(...)` is already synchronous and performs the mandatory commit-time requester/publisher/workspace/target currentness, freshness and durable CAS law.
6. Current-Mesh bridge source already provides `CandidatePublicationWireSubmission` without requiring historical control-envelope reconstruction.
7. Therefore the GJ-selected dormant semantic decomposition can be materialized inside the existing shared requester/rendezvous authority module without touching ingress, stream custody, the GI map implementation, runtime workers, transport, listener, readiness, manifests or lockfiles.

## 3. Narrow source shape

GK adds exactly one dormant envelope-neutral Agent method to `SharedRequesterRendezvousAuthority`:

`execute_current_mesh_candidate_publication_with_post_commit_cleanup(...)`

The method accepts only:

- `&SharedCurrentCapabilityAuthority<P>`;
- `&AuthenticatedDeviceSession` for the already-authenticated publisher;
- `&CandidatePublicationWireSubmission` from the already-strict current-Mesh request;
- `&mut ProductionReachabilityOwnerCustodyMap<S,T>`.

It does not accept or construct:

- `AuthenticatedPrwcConnection`;
- `CandidatePublicationControlFrame`;
- historical `prw_control_transport::ControlFrame`;
- `MeshControlStream`;
- current-Mesh request ID;
- socket address;
- a raw production owner;
- a raw requester provider or mutex guard.

## 4. New typed Agent-local error composition

GK adds:

`CurrentMeshCandidatePublicationExecutionError`

with exactly two layers:

- `Semantic(CandidatePublicationExecutionError)`;
- `OwnerLookup(ProductionReachabilityOwnerCustodyLookupError)`.

This preserves the lower taxonomies rather than flattening them.

GI `Missing` and `Ambiguous` remain exact owner-association failures, not requester authority failures and not production reachability commit failures.

No wire-level Accepted/Rejected classification is selected by this error type.

## 5. First current-authority phase

The new method first performs one bounded:

`SharedCurrentCapabilityAuthority::with_current_authority(...)`

operation that invokes the existing:

`publish_current_candidates(...)`

using exactly:

- the supplied authenticated publisher session;
- `submission.presented_transport_identity()`;
- `submission.candidates().to_vec()`.

Successful return is one owned `AuthenticatedCandidatePublication`.

The first current-authority read is released before requester authority is awaited.

No registry reference, read guard or per-request registry clone escapes.

## 6. Exact publisher and owner-key authority

Publisher logical identity comes only from the supplied authenticated session.

The exact production-owner key is the owned publication's:

`publication.peer()`

which preserves GJ/GH/GI law:

`PeerConnectivityIdentity::new(authenticated publisher DeviceId, submission.presented_transport_identity())`.

Forbidden lookup substitutions remain:

- `BoundRemoteSession::transport_identity()`;
- DeviceId-only;
- transport-only;
- request ID;
- requester identity;
- candidate endpoint;
- socket/peer address;
- single-owner fallback;
- same-device alternate-transport fallback.

## 7. Requester grant phase

After the first registry read is released, the existing async requester authority selects exactly one current grant for:

`publication.peer().device_id()`.

The requester mutex remains held only for the existing synchronous provider authorization call and is released before the grant-selection async method returns.

The grant's `expected_publisher_device_id()` must exactly equal the publication publisher device before commit proceeds.

Mismatch preserves existing:

`CandidatePublicationExecutionError::ExpectedPublisherMismatch`.

No durable commit or cleanup occurs after mismatch.

## 8. Cleanup identity preservation

Before durable commit, GK preserves the existing exact non-authorizing cleanup identity derived from the selected grant:

- requester `SessionId`;
- expected publisher `DeviceId`.

This identity does not authorize a second publication, reconnect, retry, dial or new requester lifecycle.

It exists only so a definite successful durable publication can later retire and remove the same exact requester record.

## 9. Fresh commit-time current-authority phase

After requester grant selection and expected-publisher equality, GK performs a second fresh:

`SharedCurrentCapabilityAuthority::with_current_authority(...)`

read.

This second read is the commit-time currentness authority and is intentionally distinct from initial publication construction.

Registry drift between initial admission and commit is therefore re-evaluated by existing production-owner commit law rather than hidden behind a stale cloned registry snapshot.

No requester mutex is held while the second current-authority read is awaited or while durable commit executes.

## 10. Exact GI lookup and lexical production-owner custody

Inside the second current-authority synchronous operation, GK invokes:

`ProductionReachabilityOwnerCustodyMap::with_owner_mut_for_peer(publication.peer(), operation)`.

The operation receives one exact lexical `&mut ProductionReachabilityOwner<S,T>` and performs only the existing synchronous:

`commit_candidate_publication(...)`.

Commit inputs are exactly:

- the fresh current registry reference;
- the selected grant's requester authenticated session;
- the owned authenticated publication;
- `submission.presented_freshness()`.

No production-owner mutable reference, store, token source, map entry or future escapes the GI closure.

No `.await` occurs while the inner production-owner mutable borrow exists.

## 11. Outer map custody

The new async method accepts `&mut ProductionReachabilityOwnerCustodyMap<S,T>` for the duration of the dormant semantic composition.

Because the second shared-current read is acquired asynchronously, the outer map mutable borrow may remain reserved while awaiting that read.

This is only the GJ-selected narrow outer-map exclusivity required by current Rust lifetime composition.

GK does not add:

- `Arc` around the map;
- mutex/Tokio mutex around the map;
- per-peer locks;
- actor/mailbox ownership;
- background owner tasks;
- concurrent peer execution promises.

## 12. Semantic error preservation

Initial publication admission maps only to existing:

`CandidatePublicationExecutionError::Candidate`.

Requester authority maps only to:

`CandidatePublicationExecutionError::RequesterAuthority`.

Expected publisher mismatch remains:

`CandidatePublicationExecutionError::ExpectedPublisherMismatch`.

Production-owner commit failure maps only to:

`CandidatePublicationExecutionError::Reachability`.

Exact GI map failure maps only to:

`CurrentMeshCandidatePublicationExecutionError::OwnerLookup`.

No automatic retry, fallback, response, close or replacement owner selection occurs.

## 13. Definite commit remains the cleanup gate

The second current-authority + GI lookup phase returns one definite `ReachabilityCommitOutcome` only after the existing production owner confirms durable commit.

No cleanup occurs after:

- initial candidate admission failure;
- requester authority failure;
- expected-publisher mismatch;
- GI `Missing`;
- GI `Ambiguous`;
- currentness failure;
- stale freshness;
- token-source failure;
- durable stale expected state;
- persistence ambiguity/failure;
- any other pre/at-commit semantic error.

## 14. Post-commit cleanup

Only after definite durable commit and after current-authority/production-owner custody has been released does GK invoke the existing exact requester cleanup:

`cleanup_committed_requester_rendezvous_record(...)`.

Cleanup remains async because it reacquires requester authority.

The returned outcome preserves:

- exact `ReachabilityCommitOutcome`;
- independent `Result<(), RequesterRendezvousLifecycleError>` cleanup disposition.

Cleanup failure cannot rewrite committed semantic success.

## 15. Historical FY path remains unchanged in meaning

GK does not remove or activate the historical:

`execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`.

The historical path continues to accept its existing historical `AuthenticatedPrwcConnection` and `CandidatePublicationControlFrame` inputs.

GK adds a separate envelope-neutral current-Mesh semantic seam rather than fabricating those historical types.

Existing historical tests and semantics remain valid.

## 16. Current-Mesh transaction and stream remain outside this method

The GK method receives no current-Mesh transaction and no `MeshControlStream`.

A later higher-owner handoff may retain the exact `PostAuthCandidatePublicationTransaction` while borrowing only its typed submission and authenticated session into the GK semantic seam.

Therefore GK:

- does not consume stream custody;
- does not clone a stream;
- does not read a second frame;
- does not reaccept a stream;
- does not build a response;
- does not send/write/finish a response.

`CandidatePublicationHandoffNotSelected` remains unchanged and active at the dormant current-Mesh higher-owner ingress seam.

## 17. Request ID remains correlation only

The GK semantic method has no request-ID parameter.

Consequently current-Mesh request correlation cannot enter:

- publisher identity;
- owner lookup;
- requester authority;
- registry validation;
- freshness authority;
- durable keying;
- cleanup identity.

A later response-framing checkpoint remains responsible for exact request-ID preservation.

## 18. `Send` bounds are not map synchronization

The method may require `S: Send` and `T: Send` only because the existing shared-current authority accepts a `Send` synchronous closure and the closure captures the mutable outer map.

These bounds do not:

- make the map shared;
- create concurrent access;
- add a runtime task;
- add synchronization;
- authorize cross-thread owner movement outside the caller's existing future ownership.

No stronger concurrency promise is selected.

## 19. Focused tests

GK extends only the existing shared requester/rendezvous authority tests to prove narrow source properties, including:

1. current-Mesh execution error preserves exact GI `Missing` as owner lookup error;
2. exact GI `Ambiguous` remains distinguishable;
3. existing semantic errors remain separately wrapped;
4. existing requester grant selection still releases requester mutex before return;
5. existing commit phase still runs without requester mutex custody;
6. existing cleanup failure still preserves a committed value separately;
7. existing commit failure still prevents cleanup.

The source signature itself proves no historical PRWC/Command/current-Mesh stream/request-ID input is required by the new semantic seam.

## 20. Authorized path set

GK is authorized to change exactly two paths:

1. this GK contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`.

No other Rust source, `Cargo.toml`, `Cargo.lock`, workflow, Android/Kotlin/Gradle, bridge source, ingress source, GI custody-map source, runtime worker, listener, bootstrap, readiness, persistence, deployment, configuration or other contract path is authorized.

## 21. Validation requirements

Canonical closure requires exact-final-head evidence for:

- exact GJ merge base;
- ahead-only branch state;
- exactly the two authorized paths;
- no manifest/lock/workflow/runtime activation changes;
- locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS with warnings denied;
- workspace tests PASS;
- workspace build PASS;
- Android validation recorded only if actually triggered;
- AD/AE and other path-filtered workflows recorded exactly as observed.

Any correction creates a new candidate head and supersedes prior validation evidence.

## 22. Immutable evidence

After exact-final-head CI is terminal and acceptable:

1. freeze GK source/contract state;
2. record final commit/tree/compare/path/blob evidence;
3. create immutable GK audit locally;
4. compute exact local bytes and SHA-256;
5. upload directly to canonical Drive folder `Private Remote Workspace` / `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch the exact Drive object;
7. recompute bytes/SHA and require equality;
8. update the GK PR to `Status: CLOSED` while keeping it draft/open/unmerged;
9. independently re-read PR and branch state.

No My Drive root upload is permitted.

## 23. Explicit non-goals

GK does not:

- remove `CandidatePublicationHandoffNotSelected`;
- consume `PostAuthCandidatePublicationTransaction`;
- perform response projection or framing for current Mesh;
- send/write/finish a candidate response;
- integrate the repeated mixed-family loop;
- select cancellation semantics;
- synchronize or populate the production-owner map at runtime;
- recover owners per command;
- activate traversal;
- bind/accept a listener;
- publish readiness;
- dial;
- deploy;
- restart/recover a process;
- merge;
- delete branches;
- change repository visibility.

## 24. Canonical source law

**Agent may execute one dormant envelope-neutral current-Mesh candidate-publication semantic composition only by first constructing an owned registry-current publication from the authenticated publisher session and strict submitted transport/candidates, then selecting one exact requester grant with no registry/owner custody held, then performing a fresh current-registry read whose synchronous operation performs exact GI `publication.peer()` owner lookup and one lexical production-owner durable commit with no owner borrow crossing an await, and only after definite commit success reacquiring requester authority for exact record cleanup; GI lookup errors remain separate from existing semantic errors, cleanup remains separate from committed success, historical PRWC/Command envelopes are not fabricated, and no stream/request-ID/response/runtime activation surface participates.**

## 25. Closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_GK_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SOURCE_MATERIALIZED`

## 26. Successor rule

After canonical GK closure, perform a fresh exact-final-head source audit before selecting any successor.

Likely remaining prerequisites include higher-owner current-Mesh candidate transaction handoff into this dormant semantic seam, current-Mesh terminal result projection/framing tied to exact request correlation, consuming same-stream response I/O, cancellation/worker integration, and production-owner population/runtime ownership. GK does not pre-authorize their ordering or source scope.