# Phase 152 C03e-CN — Candidate Publication Post-Authentication PRWC Command Execution Seam Selection

Status: STAGED SELECTION

Target gate:
`C03E_CN_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_EXECUTION_SEAM_SELECTED`

## 1. Exact predecessor

Closed C03e-CM is the authoritative predecessor:
- branch: `phase-152-c03e-cm-candidate-publication-requester-rendezvous-authority-carrier-source-materialization-staging`
- head: `2784c56d09ba63cd6121581834ca2205b4e12858`
- tree: `3ab33a66c106ccaae66226c9e86513d6fb4f7b96`
- gate: `C03E_CM_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SOURCE_MATERIALIZED`
- PR #210: body `Status: CLOSED`, draft/open/unmerged

C03e-CN preserves exact CM lineage and does not amend the CM requester/rendezvous authority carrier, CL semantics, CK authentication transaction, or any existing reachability authority.

## 2. Fresh post-CM prerequisite-order audit

At exact closed CM head, all provider-neutral candidate-publication semantic inputs already exist:
- `AuthenticatedPrwcConnection` owns one authenticated publisher session plus the retained `ControlTlsServerStream` and connection-local request-ID custody;
- CK intentionally keeps the retained stream private and exposes only authenticated session plus locally-originated request-ID custody;
- `ControlTlsServerStream` already provides bounded synchronous `read_frame` / `write_frame` primitives;
- `decode_candidate_publication_control_frame(...)` strictly requires the existing Phase 129 `Command` kind and decodes the existing PRWP submission while preserving outer request correlation;
- `RequesterRendezvousAuthorityProvider` and one-shot `AuthorizedRequesterRendezvous` now exist in validated CM source;
- `publish_current_candidates(...)`, `validate_authenticated_publication_admission(...)`, and `ProductionReachabilityOwner::commit_candidate_publication(...)` already provide the downstream provider-neutral semantic and durable-commit boundaries.

The remaining direct source gap is that no bridge-owned API can receive one post-authenticated candidate-publication Command from the CK-owned stream without exposing that raw stream.

A concrete requester/rendezvous provider representation is not a prerequisite for selecting or materializing this receive seam: later provider-neutral execution composition can depend on the CM trait. A concrete provider remains mandatory before production runtime wiring can provide authoritative grants.

Therefore the safe prerequisite order is:
1. select this post-authenticated one-frame Command receive seam;
2. materialize that seam in validated source;
3. then perform a fresh audit before choosing generic candidate-publication execution composition versus concrete requester/rendezvous provider lifecycle work;
4. concrete provider authority must exist before any production runtime activation that claims requester/rendezvous authorization.

## 3. Selected ownership boundary

The seam remains owned by `AuthenticatedPrwcConnection` in `prw-remote-bridge`.

C03e-CN does **not** expose:
- `ControlTlsServerStream` directly;
- `&mut ControlTlsServerStream`;
- an `into_stream` / `into_inner` escape hatch;
- the underlying `TcpStream`, rustls state, raw reader/writer, socket, or listener.

The authenticated connection remains the sole owner of the retained post-auth PRWC stream.

## 4. Selected narrow receive responsibility

Future source materialization shall add one responsibility equivalent to:

```text
receive_candidate_publication_command(
    &mut self
) -> Result<CandidatePublicationControlFrame, AuthenticatedPrwcCommandReceiveError>
```

Exact names may vary only if repository/compiler conventions require a narrowly equivalent form. The semantics in this contract are authoritative.

The method:
- borrows the authenticated connection mutably rather than consuming it;
- performs exactly one bounded frame read per successful/failed receive attempt;
- performs no accept loop, background task, retry loop, polling loop, or multi-frame batching;
- calls the existing `ControlTlsServerStream::read_frame()` internally;
- passes the resulting frame directly to `decode_candidate_publication_control_frame(...)`;
- returns the existing typed `CandidatePublicationControlFrame` on success;
- never returns or exposes the raw transport stream.

## 5. Why `&mut self` is selected

The authenticated PRWC connection is a connection-local protocol context, not a one-command transport.

Mutable borrowing is selected because it:
- serializes frame reads for one connection without introducing a lock;
- preserves the authenticated session and connection-local state across sequential bounded Commands;
- avoids forcing one TCP/TLS connection per publication attempt;
- prevents concurrent mutable stream access through this API;
- does not clone, duplicate, or transfer transport ownership.

C03e-CN does not select concurrent multiplexing of peer-originated Commands on one stream.

## 6. Fail-closed receive state

Future source materialization must add private connection-local receive state sufficient to prevent reuse after an unrecoverable receive/protocol error.

Selected semantics:
- a successful candidate-publication Command receive leaves the connection eligible for a later separately gated sequential receive;
- any underlying `ControlFrameError` during post-auth receive marks the connection receive side terminal before returning an error;
- wrong Command kind or invalid PRWP candidate-publication payload likewise marks the candidate-publication receive side terminal;
- after terminalization, later receive attempts fail immediately without another frame read;
- terminal receive state is local protocol safety state only; it is not requester authority, authentication state, a freshness token, or a new identity;
- C03e-CN does not select automatic socket close, Error-frame transmission, reconnect, retry, or replacement connection creation.

This prevents parser-resynchronization guesses or repeated reads after partial/malformed transport input while keeping close/error-response policy separately gated.

## 7. Selected receive error surface

The future receive error must distinguish at least:
- bounded frame transport/codec failure from `ControlFrameError`;
- candidate-publication Command decode failure from `CandidatePublicationControlFrameError`;
- already-terminal post-auth receive state.

All classes fail closed and return no candidate-publication command.

Provider, registry, freshness, reachability-owner and durable-store failures do not belong to this receive error surface because they occur after successful structural Command receipt.

## 8. Authenticated publisher binding remains separate

The returned `CandidatePublicationControlFrame` continues to carry only:
- outer PRWC `request_id` correlation;
- the decoded PRWP submission.

It does not duplicate the authenticated publisher session.

Later execution composition must obtain publisher logical identity from `AuthenticatedPrwcConnection::session()` and must not derive publisher authority from PRWP payload fields, request ID, transport bytes or candidate data.

## 9. Request-ID custody remains unchanged

C03e-CJ `PrwcRequestIdLifecycle` is explicitly for **locally originated** PRWC request identifiers.

Peer-originated candidate-publication Command `request_id`:
- is preserved exactly as outer correlation by the existing control-frame adapter;
- is not allocated through `PrwcRequestIdLifecycle`;
- is not inserted into its outstanding set;
- is not completed through that lifecycle;
- is not requester authority, publisher authority, replay authority, freshness authority, or rendezvous authority.

C03e-CN introduces no new request-ID allocator or correlation namespace.

## 10. No response/write semantics selected

Although `ControlTlsServerStream` already has `write_frame`, C03e-CN selects no candidate-publication response or Error-frame behavior.

This checkpoint does not define:
- success response payload;
- failure response payload;
- Response versus Error envelope choice;
- write timing;
- terminal close behavior after response;
- retry/reconnect behavior.

Those semantics remain later and separately gated. The receive seam exists only to make one authenticated connection-local Command available to later semantic composition.

## 11. Concrete requester/rendezvous provider ordering

CM's provider-neutral trait is sufficient for later generic source composition to name and call requester/rendezvous authority without selecting a concrete backend.

Therefore C03e-CN does not require or select a concrete provider before this receive seam.

A later concrete-provider checkpoint still must select:
- authoritative representation of current requester-awaits-publisher rendezvous state;
- staleness/abandonment/retirement lifecycle;
- uniqueness/ambiguity handling;
- synchronization or transactional linearization;
- provider availability/indeterminate classification mapping;
- bootstrap/runtime ownership.

None of those choices are inferred from the receive seam.

## 12. Candidate-publication execution remains blocked

C03e-CN does not execute publication semantics.

After successful receive, later execution still must preserve the previously selected order:
1. use publisher `AuthenticatedDeviceSession` from the authenticated connection;
2. create `AuthenticatedCandidatePublication` through `publish_current_candidates(...)` using the presented transport identity and decoded candidate set;
3. authorize exactly one current requester/rendezvous grant through the CM provider-neutral port using only authenticated publisher logical `DeviceId` as lookup selector;
4. require grant expected publisher `DeviceId` equals authenticated publisher device;
5. revalidate requester/publisher/workspace/exact target through existing candidate/reachability authorities;
6. require existing publication freshness semantics;
7. only then allow existing durable candidate compare-and-commit ordering.

No step above is materialized by this docs-only checkpoint.

## 13. No new dependency required

The selected receive seam uses only already-present dependencies/types:
- `prw_control_transport::{ControlFrameError, ControlTlsServerStream}` already used by `prwc_connection_authentication.rs`;
- `CandidatePublicationControlFrame` and `CandidatePublicationControlFrameError` already exist in the same crate.

No Cargo manifest or lockfile change is selected.

## 14. Identity separation

C03e-CN preserves all existing non-interchangeable identities:
- authenticated session / logical `DeviceId` = publisher logical PRW identity;
- `TransportIdentity` = lower transport certificate identity, revalidated later by publication semantics;
- PRWC `request_id` = outer correlation only;
- requester/rendezvous grant = server-side one-shot operation authority;
- candidate publication freshness token = verifier-owned currentness/replay state;
- `CandidateId` = candidate-plan correlation only.

Receive terminal state is none of the above.

## 15. Explicitly rejected shortcuts

C03e-CN rejects:
- exposing the raw `ControlTlsServerStream` from `AuthenticatedPrwcConnection`;
- generic raw-frame escape solely to bypass candidate-publication gating;
- allocating a new local request ID for an inbound Command;
- using inbound request ID as requester/rendezvous authority;
- deriving publisher identity from PRWP payload;
- reading multiple frames in one call;
- adding a background frame loop or async runtime;
- retrying after malformed/partial frame input;
- auto-closing or auto-reconnecting the connection;
- writing a success/error response in this checkpoint;
- selecting or embedding a concrete rendezvous store/provider;
- directly committing reachability state from the receive method.

## 16. Audit-basis source remains byte-stable

The C03e-CN selection is based on these exact closed-CM source blobs:
- `crates/prw-remote-bridge/src/prwc_connection_authentication.rs` — `952ad7e8d0027e2acc8d05b6526b4ebaf8212e69`;
- `crates/prw-control-transport/src/lib.rs` — `88f70e187e865119ff6401d05019cdac7b5392ad`;
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — `20ff7d2bc5f32596a3c0696aa387e6735f8f2031`;
- `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs` — `260024b7aca2aea6109dc72e778bcda3dcca8038`;
- `crates/prw-remote-bridge/src/candidate_reachability.rs` — `51b294cfb3772925651a05bdcb034cd051204efb`;
- `crates/prw-remote-bridge/src/reachability_owner.rs` — `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
- `crates/prw-remote-bridge/src/prwc_request_id_lifecycle.rs` — `905aa5d658b6b912474cc2b91048bd8a51798148`.

No audit-basis source path may change in this docs-only checkpoint.

## 17. Exact C03e-CN diff boundary

C03e-CN is docs-only.

The exact CM -> CN diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CN_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_EXECUTION_SEAM_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent/Desktop/Android implementation, provider/database file, transport implementation, networking configuration or deployment path blocks CN closure.

## 18. Safe successor rule

After durable C03e-CN closure, the next safe checkpoint may materialize only the selected post-authenticated candidate-publication receive seam in `prwc_connection_authentication.rs` plus a bounded source-materialization contract.

That materialization must:
- keep the raw stream private;
- use exactly one existing bounded frame read per receive attempt;
- preserve authenticated session ownership;
- preserve inbound request ID as correlation only;
- implement private terminal receive state for frame/protocol failure;
- add no concrete provider, response write, frame loop, runtime activation or reachability mutation;
- require no manifest/lock change.

After that source exists and validates, a fresh audit is required before selecting generic candidate-publication execution composition versus concrete requester/rendezvous provider lifecycle work.

No successor may jump directly to product runtime activation, listener cutover, production networking, deployment or merge.

## 19. Validation and closure

C03e-CN may close only after:
- exact closed CM predecessor remains unchanged;
- CM -> CN compare is ahead 1 / behind 0 with exact CM merge base and exactly one docs-only path;
- every audit-basis source blob remains byte-stable;
- root and Android Cargo locks remain byte-stable;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- skipped workflows are recorded as SKIPPED, never PASS;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive predecessor guard and append-only byte-prefix proof pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged.

## 20. Completion meaning

Closure means only that the bridge-owned, raw-stream-private, one-frame post-authenticated candidate-publication Command receive seam is selected, including fail-closed terminal receive state and request-ID non-interference.

It does not mean the seam exists in Rust source, a concrete requester/rendezvous provider exists, candidate publication executes, a response is written, reachability mutates, a listener is activated, product runtime is wired, or anything is deployed.

Target gate:
`C03E_CN_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_EXECUTION_SEAM_SELECTED`
