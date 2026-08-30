# Phase 152 C03e-GI — Candidate Publication Production Reachability Owner Authenticated Peer Mapping Lookup Source Materialization

Status: VALIDATING

Target gate:
`C03E_GI_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_LOOKUP_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-GH is the authoritative predecessor:

- branch: `phase-152-c03e-gh-candidate-publication-production-reachability-owner-authenticated-peer-mapping-handoff-consumption-semantics-selection-staging`;
- head: `4f464556cb109a1c4db9a85678fc9f397afb1785`;
- tree: `4611dc6308657beba02f60ac91718315d43679ae`;
- exact parent/merge base: closed C03e-GG head `e6b47949c91d190134ab7acd960aea30053caf4d`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_HANDOFF_CONSUMPTION_SEMANTICS_SELECTION`;
- gate: `C03E_GH_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_HANDOFF_CONSUMPTION_SEMANTICS_SELECTED`;
- PR #310: `Status: CLOSED`, draft/open/unmerged.

GH remains frozen. GI starts exactly from the GH commit and does not amend any closed predecessor.

## 2. Fresh exact-GH source finding

GH proved that current-Mesh candidate publication needs exact production-owner selection keyed by:

`PeerConnectivityIdentity::new(authenticated_publisher_device_id, submission.presented_transport_identity())`

At exact GH head:

- GE already retains one strict `PostAuthCandidatePublicationTransaction` and exact same Mesh stream;
- the dormant Agent post-auth ingress still returns `CandidatePublicationHandoffNotSelected` for that family;
- GG owns one recovered `ProductionReachabilityOwnerCustody<S,T>` around one exact production owner;
- no Agent source owns a multi-peer association/lookup boundary over those already-recovered custodies.

Therefore the first source prerequisite after GH is the narrow exact peer-keyed custody association/lookup representation. GI does not yet consume the GE transaction or execute candidate semantics.

## 3. Narrow representation selection after source inspection

`PeerConnectivityIdentity` derives exact `PartialEq/Eq` but does not currently provide a `Hash` contract.

No existing Agent lifetime topology proves a need for:

- `HashMap`;
- `BTreeMap`;
- `Arc`;
- mutex/Tokio mutex;
- actor/mailbox;
- background owner task;
- persistent lookup table.

GI therefore materializes a private `Vec`-backed logical map over already-recovered custodies.

This is a representation choice only for the dormant crate-internal boundary. It does not create a runtime scaling promise or forbid a later separately audited representation change if source topology proves one necessary.

## 4. Materialized association owner

GI adds:

`ProductionReachabilityOwnerCustodyMap<S,T>`

inside the existing Agent crate-internal production-owner custody module.

The map stores only:

`Vec<ProductionReachabilityOwnerCustody<S,T>>`

It does not store a second persistent peer-key copy beside each owner.

Exact peer keys are observed transiently through GG's bounded `with_owner_mut(...)` seam and remain authoritative only inside the existing production owner's plan.

## 5. Composition law

GI materializes:

`ProductionReachabilityOwnerCustodyMap::try_new(...)`

The constructor consumes already-recovered custodies by value.

It performs no:

- `ProductionReachabilityOwnerCustody::recover(...)`;
- durable load/reload;
- token issuance;
- durable compare-and-commit;
- candidate execution;
- requester/rendezvous mutation;
- response I/O;
- runtime activation.

For each supplied custody it observes the exact current owner plan peer only through GG's bounded closure seam.

## 6. Duplicate exact peer is rejected at composition

GI adds typed:

`ProductionReachabilityOwnerCustodyAssociationError::DuplicatePeer`

If two supplied custodies resolve to the same exact `PeerConnectivityIdentity`, construction fails closed.

GI does not:

- keep both and choose first;
- keep both and choose last;
- merge their stores/freshness state;
- treat one as standby;
- normalize duplicates by `DeviceId` only;
- normalize duplicates by transport identity only.

A successful map therefore establishes one retained custody at most for each exact two-part peer key.

## 7. Exact lookup seam

GI materializes:

`with_owner_mut_for_peer(&PeerConnectivityIdentity, operation)`

The operation is invoked only after exactly one full `PeerConnectivityIdentity` match is proven.

Lookup equality includes both:

1. logical `DeviceId`;
2. `TransportIdentity`.

A same-device/different-transport entry is not a match.

A same-transport/different-device entry is not a match.

No single-entry fallback exists.

## 8. Typed lookup failures

GI adds:

`ProductionReachabilityOwnerCustodyLookupError`

with exact classes:

- `Missing` — zero exact matches;
- `Ambiguous` — more than one exact match.

`Ambiguous` is retained defensively even though the public constructor rejects duplicate keys. Future internal representation drift must still fail closed rather than silently selecting an arbitrary owner.

Neither lookup failure is a candidate semantic `Rejected` result because candidate semantic execution has not occurred.

## 9. GG encapsulation remains unchanged

Successful lookup does not return:

- raw `ProductionReachabilityOwner<S,T>`;
- raw store;
- raw token source;
- raw vector entry;
- peer snapshot reference;
- mutex guard;
- cloneable owner handle.

The supplied synchronous operation is delegated only through the existing GG:

`ProductionReachabilityOwnerCustody::with_owner_mut(...)`.

The higher-ranked closure boundary continues to prevent a reference tied to the production owner borrow from escaping the lexical operation.

## 10. No duplicated peer authority retained

GI does not add a stored `(PeerConnectivityIdentity, custody)` tuple.

Construction uses temporary cloned peer identities only to detect duplicates, then drops that temporary validation set.

Lookup compares the caller-provided exact key against each retained production owner's existing plan peer.

Thus the production owner remains the sole retained source of its exact peer lifecycle identity inside this mapping boundary.

## 11. GH key-source law remains mandatory but uninvoked

GI's lookup accepts an already-typed `&PeerConnectivityIdentity`.

This source checkpoint does not yet remove the current-Mesh candidate handoff barrier and therefore does not construct that key from a live GE transaction.

When a later separately gated handoff adapter is selected, its lookup key must still be formed exactly from:

- authenticated publisher logical `DeviceId` from the retained current application session;
- strict candidate submission `presented_transport_identity()`.

GI does not authorize a key sourced from:

- outer request ID;
- bound-session transport identity substitution;
- candidate endpoint bytes;
- requester grant;
- cleanup identity;
- peer socket/address;
- lower transport identity bytes outside the typed submission.

## 12. Presented transport remains nomination before semantic validation

Association lookup is not current-registry validation.

Even if a map entry exists for the exact presented key, `submission.presented_transport_identity()` remains caller-presented nomination until the existing candidate semantic path revalidates it for the authenticated publisher.

GI therefore does not treat successful owner lookup as:

- transport ownership proof;
- registry currentness;
- publisher authorization;
- freshness authority;
- requester/rendezvous authority;
- durable commit authority.

## 13. Owner target equality remains existing production law

GI does not retarget an owner.

The map never rewrites:

- owner plan peer;
- publisher device identity;
- presented transport identity;
- candidate publication peer.

A later semantic execution adapter must still reach the existing production-owner exact target equality checks through the existing commit path.

## 14. GE transaction remains unconsumed

GI does not change:

`PostAuthCandidatePublicationTransaction`

and does not remove:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`.

No current-Mesh stream is consumed, cloned, duplicated, re-read, replaced, or exposed by GI.

The exact same Mesh stream remains dormant in GE custody for a later separately gated handoff/execution/response lineage.

## 15. Historical PRWC ownership is not fabricated

GI does not construct or wrap:

- `AuthenticatedPrwcConnection`;
- historical `ControlTlsServerStream`;
- historical `CandidatePublicationControlFrame`;
- historical candidate `Command` frame.

The existing historical CQ/FY APIs remain unchanged and dormant.

## 16. FY/GA/GC remain dormant

GI does not call or adapt current-Mesh input into:

- `execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`;
- `project_candidate_publication_terminal_result(...)`;
- `compose_candidate_publication_terminal_result_frame(...)`.

No candidate publication, requester grant, durable commit, post-commit cleanup, terminal semantic projection, or result frame composition is activated.

## 17. No response write semantics

GI adds no current-Mesh candidate response send surface.

It does not select or materialize:

- Accepted/Rejected response write;
- response-stream consumption;
- send-direction finish;
- wire I/O error mapping;
- fallback Rejected;
- retry/re-encoding;
- peer close;
- repeated candidate command continuation.

## 18. No recovery/bootstrap schedule

GI operates only on already-recovered GG custodies supplied by value.

It does not select or invoke:

- owner recovery startup order;
- per-session recovery;
- per-command recovery;
- `reload_from_store()`;
- production-owner bootstrap collection population;
- live-owner authority equivalence;
- process recovery.

The future owner population/lifetime schedule remains separately gated.

## 19. No concurrency invention

The map itself requires `&mut self` for lookup that may later mutate one exact retained owner through the supplied closure.

GI adds no synchronization primitive around it.

If future source topology requires shared access across tasks, that requirement must be selected separately rather than inferred from this dormant mapping type.

## 20. Focused tests

GI extends the existing GG custody tests to prove:

1. duplicate exact peer custodies are rejected during map composition;
2. same logical device with different transport identities remains two distinct exact keys;
3. lookup for an absent alternate transport fails `Missing` rather than falling back to another owner for the same device;
4. defensive ambiguous state fails `Ambiguous` rather than selecting first match;
5. map construction and lookup do not perform additional durable loads;
6. map construction and lookup do not perform durable compare-and-commit;
7. map construction and lookup do not issue freshness tokens.

Existing GG recovery success/missing/ambiguous-durable-load tests remain unchanged in semantic meaning.

## 21. Authorized path set

GI is authorized to change exactly two paths:

1. this GI contract;
2. `crates/prw-agent/src/production_reachability_owner_custody.rs`.

No other Rust source, `Cargo.toml`, `Cargo.lock`, workflow, Android/Kotlin/Gradle, bridge source, ingress source, listener, bootstrap, readiness, persistence, deployment, configuration, or other contract path is authorized.

## 22. Validation requirements

Canonical closure requires exact-final-head evidence for:

- exact GH merge base;
- ahead-only branch state;
- only the two authorized paths;
- no manifest/lock/workflow/runtime activation changes;
- locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS with warnings denied;
- workspace tests PASS;
- workspace build PASS;
- Android validation recorded only if actually triggered;
- AD/AE and other path-filtered workflows recorded exactly as observed.

Any correction creates a new candidate head and supersedes prior validation evidence.

## 23. Immutable evidence

After exact-final-head CI is terminal and acceptable:

1. freeze GI source/contract state;
2. record final commit/tree/compare/path/blob evidence;
3. create immutable GI audit locally;
4. compute exact local bytes and SHA-256;
5. upload directly to canonical Drive folder `Private Remote Workspace` / `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch the exact Drive object;
7. recompute bytes/SHA and require equality;
8. update the GI PR to `Status: CLOSED` while keeping it draft/open/unmerged;
9. independently re-read PR and branch state.

No My Drive root upload is permitted.

## 24. Explicit non-goals

GI does not:

- derive a live lookup key from a GE transaction;
- consume `PostAuthCandidatePublicationTransaction`;
- remove `CandidatePublicationHandoffNotSelected`;
- source-materialize current-Mesh semantic execution adaptation;
- recover/populate production owners at runtime;
- fabricate historical PRWC ownership;
- execute candidate publication;
- mutate requester/rendezvous authority;
- commit reachability state;
- perform post-commit cleanup;
- project/encode a terminal result;
- send/write a candidate response;
- activate traversal;
- bind/accept a listener;
- publish readiness;
- dial;
- deploy;
- restart/recover a process;
- merge;
- delete branches.

## 25. Canonical source law

**Agent may compose a dormant private collection of already-recovered production reachability-owner custodies only when each custody has a unique exact `PeerConnectivityIdentity`; exact lookup requires full logical-device plus transport equality, fails distinctly for missing or ambiguous ownership, never falls back by DeviceId/transport/single-entry heuristics, and exposes the selected owner only through GG's bounded mutable closure without performing recovery, candidate semantics, response I/O or runtime activation.**

## 26. Closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_LOOKUP_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_GI_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_AUTHENTICATED_PEER_MAPPING_LOOKUP_SOURCE_MATERIALIZED`

## 27. Successor rule

After canonical GI closure, perform a fresh exact-final-head source audit before selecting the next prerequisite.

Likely remaining prerequisites include the current-Mesh authenticated-session + strict candidate-request key/handoff adaptation into this exact owner lookup, provider-neutral semantic execution without fabricating historical PRWC ownership, and terminal same-stream response custody. GI does not pre-authorize their ordering or source scope.
