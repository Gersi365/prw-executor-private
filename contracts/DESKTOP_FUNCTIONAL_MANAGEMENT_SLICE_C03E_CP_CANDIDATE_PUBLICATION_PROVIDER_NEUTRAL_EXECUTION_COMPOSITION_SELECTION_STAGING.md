# Phase 152 C03e-CP — Candidate Publication Provider-Neutral Execution Composition Selection

Status: STAGED SELECTION

Target gate:
`C03E_CP_CANDIDATE_PUBLICATION_PROVIDER_NEUTRAL_EXECUTION_COMPOSITION_SELECTED`

## 1. Exact predecessor

Closed C03e-CO is the authoritative predecessor:
- branch: `phase-152-c03e-co-candidate-publication-post-auth-prwc-command-receive-seam-source-materialization-staging`
- head: `ca83f2304a002dcec42cb6f036346ed723f5c3df`
- tree: `1d253a3023b72e06496129453c25b7a40d663dc3`
- gate: `C03E_CO_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_RECEIVE_SEAM_SOURCE_MATERIALIZED`
- PR #212: body `Status: CLOSED`, draft/open/unmerged

C03e-CP preserves exact CO lineage. It does not amend the CO receive seam, CM requester/rendezvous authority carrier, existing candidate-reachability authority, durable reachability-owner semantics, PRWC request-ID lifecycle, or any runtime/listener behavior.

## 2. Fresh post-CO prerequisite-order audit

At exact closed CO head, the provider-neutral inputs required for one bounded candidate-publication semantic execution already exist:

- `AuthenticatedPrwcConnection` retains the authenticated publisher session and keeps `ControlTlsServerStream` private;
- `AuthenticatedPrwcConnection::receive_candidate_publication_command(...)` now produces one typed `CandidatePublicationControlFrame` using exactly one bounded frame read while preserving outer PRWC `request_id` only as peer-originated correlation;
- `CandidatePublicationWireSubmission` already exposes the presented `TransportIdentity`, verifier-owned presented freshness token, and bounded typed candidate vector;
- `publish_current_candidates(...)` already revalidates the authenticated publisher and presented transport identity before creating `AuthenticatedCandidatePublication`;
- `RequesterRendezvousAuthorityProvider` already defines the provider-neutral, fail-closed authorization port for exactly one current server-side requester/rendezvous selection;
- `AuthorizedRequesterRendezvous` already carries one owned authenticated requester session plus the exact expected publisher `DeviceId` and is intentionally neither `Copy` nor `Clone`;
- `ProductionReachabilityOwner::commit_candidate_publication(...)` already owns requester/publisher/workspace/exact-target revalidation, presented freshness comparison, staged candidate validation, replacement verifier freshness, durable expected-current CAS, local installation and traversal invalidation.

The remaining source gap is therefore composition, not a missing semantic authority primitive: no bounded bridge-owned helper currently connects one already-received authenticated candidate-publication Command to the existing CM authority port and existing reachability-owner commit seam in the previously selected order.

A concrete requester/rendezvous provider lifecycle/representation is still mandatory before production runtime can instantiate trustworthy requester/rendezvous authority. It is not a prerequisite for materializing the provider-neutral composition itself because CM deliberately exposed the trait boundary for that purpose.

Therefore the safe prerequisite order selected by C03e-CP is:
1. select the provider-neutral execution composition in this docs-only checkpoint;
2. materialize and validate only that composition against the existing CM port and existing reachability owner;
3. perform a fresh audit after source materialization;
4. select/materialize concrete requester/rendezvous provider lifecycle/representation before any production runtime activation that supplies requester/rendezvous grants;
5. keep response/Error-frame semantics, frame loops, listener activation, networking activation and deployment separately gated.

## 3. Selected ownership boundary

Candidate-publication semantic composition remains in `prw-remote-bridge` but is selected as a separate module rather than expanding the authentication/receive module into durable reachability ownership.

Future source materialization shall introduce one narrow module equivalent to:

`crates/prw-remote-bridge/src/candidate_publication_execution.rs`

and register it through the existing production crate root.

The composition accepts an `AuthenticatedPrwcConnection` only by shared borrow so publisher logical identity is obtained from `AuthenticatedPrwcConnection::session()` while raw transport ownership remains private.

C03e-CP does not move, expose, borrow or otherwise surface `ControlTlsServerStream`.

## 4. Selected execution shape

Future source materialization shall provide one bounded helper equivalent to:

```text
execute_authenticated_candidate_publication<S, T, P>(
    connection: &AuthenticatedPrwcConnection,
    command: &CandidatePublicationControlFrame,
    registry: &WorkspaceDeviceRegistry,
    requester_authority: &mut P,
    owner: &mut ProductionReachabilityOwner<S, T>,
) -> Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>
where
    S: ReachabilityDurableStore,
    T: CandidatePublicationFreshnessTokenSource,
    P: RequesterRendezvousAuthorityProvider
```

Exact identifiers may vary only where compiler/repository conventions require a narrowly equivalent form. The ordering and authority boundaries in this contract are authoritative.

The helper performs no frame read and no frame write. It composes one command already returned by the CO receive seam.

## 5. Exact selected execution order

One invocation must preserve this order:

1. obtain the publisher `AuthenticatedDeviceSession` only from `connection.session()`;
2. obtain presented transport identity, presented freshness and candidate vector only from the typed `command.submission()`;
3. call `publish_current_candidates(...)` with the current registry, authenticated publisher session, presented transport identity and candidate vector;
4. only after successful publisher/session/transport/candidate construction, call `RequesterRendezvousAuthorityProvider::authorize_current_for_publisher(...)` exactly once using the authenticated publication's logical publisher `DeviceId` as lookup selector;
5. take ownership of the returned one-shot `AuthorizedRequesterRendezvous` grant for this execution attempt;
6. require `grant.expected_publisher_device_id()` to equal the authenticated publication's publisher `DeviceId` exactly;
7. call `ProductionReachabilityOwner::commit_candidate_publication(...)` exactly once using:
   - the current registry;
   - the grant's authenticated requester session;
   - the authenticated candidate publication;
   - the PRWP presented freshness token;
8. return the existing `ReachabilityCommitOutcome` only after the existing durable commit seam succeeds.

No later step may execute after an earlier failure.

This preserves the previously selected order from CN while reusing existing authorities rather than duplicating their validation logic.

## 6. Publisher identity is connection-bound, not payload-derived

The publisher logical identity for execution is the authenticated session retained by `AuthenticatedPrwcConnection`.

The composition must not derive publisher authority from:
- PRWP bytes;
- presented `TransportIdentity`;
- candidate endpoints;
- `CandidateId` values;
- outer PRWC `request_id`;
- requester/rendezvous provider output.

`publish_current_candidates(...)` remains authoritative for current publisher-session and lower transport-identity validation.

The provider lookup selector must be the logical publisher `DeviceId` derived from the successfully authenticated/current publication path, never a new payload field.

## 7. One-shot requester/rendezvous grant semantics

Each execution invocation may request at most one current requester/rendezvous grant.

The returned `AuthorizedRequesterRendezvous` is owned by the composition attempt and must not be cloned, cached, retained for later commands, placed in connection state, or reused after success/failure.

The composition must explicitly compare:

`grant.expected_publisher_device_id() == authenticated_publication.publisher DeviceId`

before invoking the reachability owner.

A mismatch fails closed before any candidate commit attempt.

The grant's `requester_session()` is the only requester session passed to `ProductionReachabilityOwner::commit_candidate_publication(...)`.

Provider errors remain provider authority failures and must not be converted into implicit requester authority.

## 8. Existing reachability owner remains authoritative

C03e-CP does not duplicate or bypass `ProductionReachabilityOwner::commit_candidate_publication(...)`.

That owner remains authoritative for:
- current owner mode;
- requester registry currentness;
- publisher registry currentness;
- requester/publisher workspace equality;
- exact publication peer versus target plan match;
- current target transport identity;
- exact presented verifier freshness;
- complete staged candidate refresh validation;
- distinct replacement verifier token issuance;
- durable expected-current compare-and-commit;
- recovery transition on stale/ambiguous durable state;
- local plan/freshness installation after commit;
- invalidation of an older traversal lifecycle after accepted publication.

The CP composition performs no direct durable-store call and no direct plan mutation.

## 9. Selected failure surface

Future materialization shall define a stable fail-closed composition error surface distinguishing at least:

- authenticated candidate publication construction/admission failure represented by existing `CandidateReachabilityError`;
- requester/rendezvous authority failure represented by existing `RequesterRendezvousAuthorityError`;
- authorized expected-publisher mismatch;
- reachability-owner commit failure represented by existing `ReachabilityOwnerError`.

The composition must preserve source errors where appropriate through `std::error::Error::source()` rather than flattening distinct authority failures into one generic success/failure flag.

`AuthenticatedPrwcCommandReceiveError` is not part of this execution error because CO receive completes before this helper is invoked.

## 10. Outer PRWC request ID remains correlation only

`CandidatePublicationControlFrame::request_id()` is not consumed as semantic authority by CP.

The execution composition must not:
- allocate a replacement local request ID;
- insert the inbound ID into `PrwcRequestIdLifecycle`;
- use the ID as requester or publisher authority;
- use the ID as freshness/replay authority;
- use the ID as a durable-store expected version;
- use the ID as candidate identity.

The command is selected to be borrowed rather than consumed, so its outer correlation remains available to later separately gated response/Error-frame composition.

CP itself writes no response.

## 11. No receive, retry or loop semantics

The CP helper operates on one already-decoded command and therefore must not:
- call `receive_candidate_publication_command(...)` internally;
- read another PRWC frame;
- write a PRWC frame;
- loop over commands;
- retry requester/rendezvous authorization;
- retry a failed reachability commit;
- reconnect or replace a connection;
- spawn a task or thread;
- poll a socket or runtime.

One function invocation is one bounded semantic execution attempt.

Any retry/reconnect/response lifecycle remains separately selected later.

## 12. No concrete requester/rendezvous provider selected

C03e-CP deliberately remains generic over `RequesterRendezvousAuthorityProvider`.

This checkpoint does not select:
- a database/storage product for rendezvous state;
- a persistent schema/serialization;
- a server process that owns requester-awaits-publisher state;
- TTL duration or wall-clock source;
- abandonment/retirement timing;
- cleanup scheduling;
- synchronization primitive;
- distributed transaction implementation;
- bootstrap wiring;
- provider credentials;
- runtime lifecycle.

Those choices require a separate concrete-provider checkpoint before product runtime activation can claim requester/rendezvous authorization.

## 13. No response/Error-frame semantics selected

CP returns only the existing semantic `ReachabilityCommitOutcome` on success and a typed execution error on failure.

It selects no PRWC response representation and no write behavior, including:
- Response versus Error outer kind;
- response payload schema;
- success/failure status code;
- whether replacement freshness is returned to the publisher;
- write timing;
- connection terminalization after write failure;
- retry/reconnect rules.

The existing peer-originated `request_id` remains available as correlation for a future separately gated response checkpoint.

## 14. Freshness custody remains unchanged

The PRWP presented freshness token is supplied directly to the existing reachability owner.

CP does not:
- generate presented freshness;
- reinterpret it as a timestamp/counter;
- compare it directly outside the existing owner;
- persist it directly;
- generate replacement freshness;
- expose the owner's verifier token source.

Replacement freshness generation and durable expected-current semantics remain exclusively within the existing reachability owner.

## 15. Identity separation

C03e-CP preserves all existing distinct identities and authorities:
- authenticated `DeviceId` in `AuthenticatedDeviceSession` = publisher logical PRW identity;
- PRWP `TransportIdentity` = lower transport identity requiring current registry validation;
- one-shot `AuthorizedRequesterRendezvous` = server-side requester/rendezvous operation authority;
- grant requester authenticated session = requester logical identity for reachability admission;
- `CandidatePublicationFreshnessToken` = verifier-owned publication currentness/replay state;
- `CandidateId` = candidate-plan correlation identity only;
- PRWC `request_id` = outer message correlation only.

None may substitute for another.

## 16. No new dependency selected

All selected composition types are already dependencies of `prw-remote-bridge` and already present in the crate's validated source graph.

C03e-CP selects no Cargo manifest or lockfile change.

Future source materialization should require only:
- one new provider-neutral execution module;
- one existing `root.rs` module registration/export line;
- one bounded source-materialization contract.

Any manifest or lockfile change blocks that successor unless a fresh audit proves an unavoidable dependency need.

## 17. Audit-basis source remains byte-stable

The C03e-CP selection is based on these exact closed-CO source blobs:
- `crates/prw-remote-bridge/src/prwc_connection_authentication.rs` — `1af3aa2851e87e3a4f7990c98e105e62141d8db1`;
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — `20ff7d2bc5f32596a3c0696aa387e6735f8f2031`;
- `crates/prw-remote-bridge/src/candidate_publication_wire.rs` — `299042938b38b65b78f737926f74b8567e5046fb`;
- `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs` — `260024b7aca2aea6109dc72e778bcda3dcca8038`;
- `crates/prw-remote-bridge/src/candidate_reachability.rs` — `51b294cfb3772925651a05bdcb034cd051204efb`;
- `crates/prw-remote-bridge/src/reachability_owner.rs` — `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
- `crates/prw-remote-bridge/src/root.rs` — `6c3c3f038eb8a5abf217e18b645d38f5312bfb34`.

No audit-basis source path may change in this docs-only checkpoint.

## 18. Exact C03e-CP diff boundary

C03e-CP is docs-only.

The exact CO -> CP diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CP_CANDIDATE_PUBLICATION_PROVIDER_NEUTRAL_EXECUTION_COMPOSITION_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, provider/database implementation, Agent/Desktop/Android application source, runtime/listener implementation, networking configuration or deployment path blocks CP closure.

## 19. Explicitly rejected shortcuts

C03e-CP rejects:
- deriving publisher identity from candidate-publication payload bytes;
- calling requester/rendezvous authority before current publisher/transport publication construction succeeds;
- using an arbitrary requester session supplied by the caller instead of the CM grant;
- skipping the grant expected-publisher equality check;
- cloning/caching/reusing a one-shot grant;
- bypassing `ProductionReachabilityOwner::commit_candidate_publication(...)`;
- mutating a connectivity plan directly from the composition helper;
- comparing or replacing durable freshness outside the existing owner;
- using inbound PRWC `request_id` as authority/currentness state;
- combining receive, semantic execution and response write into one new loop;
- adding automatic retry/reconnect;
- selecting a concrete requester/rendezvous store in this checkpoint;
- activating a listener, accepted-stream loop or product runtime;
- changing production networking;
- deployment or merge.

## 20. Safe source-materialization successor

After durable C03e-CP closure, the next safe checkpoint may materialize only the selected provider-neutral execution composition.

That successor is authorized to change only a bounded set equivalent to:
1. a source-materialization contract for the CP-selected composition;
2. `crates/prw-remote-bridge/src/candidate_publication_execution.rs` as the new composition module;
3. `crates/prw-remote-bridge/src/root.rs` only to register/export that module.

It must:
- preserve CO `prwc_connection_authentication.rs` bytes unless a concrete compiler contradiction requires a separately reviewed corrective;
- preserve CM requester/rendezvous authority semantics;
- preserve existing reachability-owner semantics;
- preserve PRWC request-ID custody;
- require no manifest/lock change;
- include focused tests proving ordering, one provider call, expected-publisher mismatch fail-closed behavior, authority-error propagation and no owner commit after prerequisite failure;
- validate the exact final head through the canonical Rust and Android workflows;
- keep any non-applicable disposable provider workflows recorded as SKIPPED rather than PASS.

After source materialization validates, a fresh prerequisite audit is required before choosing concrete requester/rendezvous provider lifecycle, response/Error-frame composition, or any later runtime assembly.

No successor may jump directly to listener/runtime activation, production networking, deployment or merge.

## 21. Validation and closure

C03e-CP may close only after:
- exact closed CO predecessor remains unchanged;
- CO -> CP compare is ahead 1 / behind 0 with exact CO merge base and exactly one docs-only path;
- every audit-basis source blob remains byte-stable;
- root and Android Cargo locks remain byte-stable;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- skipped workflows are recorded as SKIPPED, never PASS;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive predecessor guard and append-only byte-prefix proof pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged.

## 22. Completion meaning

Closure means only that the next prerequisite has been selected as provider-neutral candidate-publication semantic execution composition and that its exact ordering, authority boundaries, failure surface and source-materialization boundary are locked.

It does not mean the composition exists in Rust source, a concrete requester/rendezvous provider exists, a response is written, a frame loop runs, a listener is active, reachability networking is activated in product runtime, production networking changes, anything is deployed, or any PR is merged.

Target gate:
`C03E_CP_CANDIDATE_PUBLICATION_PROVIDER_NEUTRAL_EXECUTION_COMPOSITION_SELECTED`
