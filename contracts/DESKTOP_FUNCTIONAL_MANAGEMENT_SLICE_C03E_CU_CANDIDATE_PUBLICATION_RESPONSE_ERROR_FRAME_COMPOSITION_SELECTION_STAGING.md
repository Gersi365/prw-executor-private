# Phase 152 C03e-CU — Candidate Publication Response/Error Frame Composition Selection

Status: STAGED SELECTION

Target gate:
`C03E_CU_CANDIDATE_PUBLICATION_RESPONSE_ERROR_FRAME_COMPOSITION_SELECTED`

## 1. Exact predecessor

Closed C03e-CT is the authoritative predecessor:
- branch: `phase-152-c03e-ct-candidate-publication-requester-rendezvous-bounded-in-memory-provider-source-materialization-staging`
- head: `df2349f3ef4000219c06b0556d95b449a332c341`
- tree: `4fd60a2840d59bd807872c49c57ec1c810ae2e3d`
- gate: `C03E_CT_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_BOUNDED_IN_MEMORY_PROVIDER_SOURCE_MATERIALIZED`
- PR #217: body `Status: CLOSED`, draft/open/unmerged

C03e-CU is docs-only and preserves exact CT source bytes.

## 2. Fresh post-CT prerequisite audit

Exact-CT inspection establishes the following existing facts:
- `prw-control-transport` already defines bounded `ControlMessageKind::{Command, Response, Error}` frames and rejects request ID zero;
- `candidate_publication_control_frame.rs` already preserves the peer-originated outer request ID in `CandidatePublicationControlFrame` and exposes it through `request_id()`;
- `candidate_publication_execution.rs` deliberately does not read, allocate, mutate or persist request IDs and returns `Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>` after semantic execution;
- `ReachabilityCommitOutcome` already exposes the verifier-issued `replacement_freshness` that became current and an internal `invalidated_traversal` observation;
- `control_session_auth_wire.rs` provides an existing bridge-local protocol precedent: one magic/version family maps operation tags to `Response` for terminal success and a generic `Error` operation with no detailed external rejection reason, while preserving the caller-supplied request ID exactly;
- `authorized_request_dispatch.rs` provides a second existing precedent that constructs a `Response` using the exact already-authorized inbound request ID rather than allocating a new one;
- no current candidate-publication Response/Error inner payload codec or response writer exists;
- runtime/process ownership of the new in-memory requester/rendezvous provider remains unresolved and would require separate process lifetime/concurrency/shared-instance decisions.

Therefore response/error frame representation is the lower-level prerequisite that can be selected without runtime ownership, listener activation, shared mutable runtime state or network I/O.

## 3. Ordering relative to runtime ownership

C03e-CU selects candidate-publication Response/Error frame composition before runtime/process ownership.

Reason:
- the wire result is a pure deterministic projection of an already-decoded command correlation and an already-completed semantic execution result;
- it requires no concrete process owner and no synchronization primitive;
- selecting runtime ownership first would force unrelated decisions about instance lifetime, registration ingress, concurrency and bootstrap wiring before the protocol has a terminal result representation.

C03e-CU does not reject future runtime ownership. It only establishes prerequisite order.

## 4. Preserve the existing PRWP protocol family

Candidate-publication result messages remain in the existing `PRWP` v1.0 inner protocol family.

Existing values remain unchanged:
- magic: `PRWP`;
- major: `1`;
- minor: `0`;
- operation `1`: publisher candidate-set submission carried by outer `Command`.

C03e-CU selects two additional operation tags only for terminal result representation:
- operation `2`: publisher candidate-set accepted;
- operation `3`: publisher candidate-set rejected.

No existing submission bytes, operation tag or decoder behavior may be reinterpreted.

## 5. Accepted result representation

The accepted result is equivalent to:

`CandidatePublicationResultMessage::Accepted { replacement_freshness: CandidatePublicationFreshnessToken }`

Its outer PRWC kind is exactly `ControlMessageKind::Response`.

The inner PRWP v1.0 payload is exactly:
- 4-byte `PRWP` magic;
- u16 major `1`;
- u16 minor `0`;
- u16 operation `2`;
- u16 reserved zero;
- 32-byte verifier-issued replacement freshness token.

Exact accepted payload size: `44` bytes.

The replacement freshness is sourced only from `ReachabilityCommitOutcome::replacement_freshness()` after a successful durable reachability commit.

## 6. Do not expose traversal invalidation

`ReachabilityCommitOutcome::invalidated_traversal()` remains internal server-side execution evidence.

C03e-CU selects no wire field for it because exact-CT source provides no protocol requirement for a publisher to observe whether a previously current server-side traversal object was invalidated.

The successful wire contract exposes only the new verifier freshness token required for the next publication freshness transition.

## 7. Rejected result representation

The rejected result is exactly a generic terminal rejection equivalent to:

`CandidatePublicationResultMessage::Rejected`

Its outer PRWC kind is exactly `ControlMessageKind::Error`.

The inner PRWP v1.0 payload is exactly the 12-byte common header:
- 4-byte `PRWP` magic;
- u16 major `1`;
- u16 minor `0`;
- u16 operation `3`;
- u16 reserved zero.

Exact rejected payload size: `12` bytes.

No detailed semantic failure reason is exposed on the wire by this checkpoint.

## 8. Generic rejection is deliberate

Every `CandidatePublicationExecutionError` maps to the same external `Rejected` result.

This preserves fail-closed behavior and follows the existing PRWA `Rejected` precedent. It avoids exposing distinctions such as:
- requester/rendezvous authority missing, stale, ambiguous or unavailable;
- expected-publisher mismatch;
- registry/currentness details;
- candidate admission details;
- persistence/recovery state;
- token-source state;
- internal durable-owner lifecycle details.

Internal typed errors remain available to server-side callers/logging policy outside the wire codec. C03e-CU does not select logging behavior.

## 9. Request-ID custody

Both accepted and rejected result frames must preserve exactly:

`command.request_id()`

from the already-decoded peer-originated `CandidatePublicationControlFrame`.

The result codec/composition must never:
- allocate a replacement request ID;
- register the peer-originated ID in the local `PrwcRequestIdLifecycle`;
- mutate or increment connection-local request-ID state;
- derive a request ID from freshness, session, publisher or candidate fields.

This is echo correlation only, consistent with existing PRWA and authorized-response precedents.

## 10. Result projection boundary

A future pure result adapter may accept semantically equivalent inputs:

`request_id: u64`

and

`Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>`

and return one typed/result `ControlFrame`.

Rules:
- `Ok(outcome)` -> `Response`, operation `2`, exact same request ID, body containing only `outcome.replacement_freshness()`;
- `Err(_)` -> generic `Error`, operation `3`, exact same request ID, no detailed body.

The adapter does not itself execute candidate publication and does not inspect nested errors to choose externally distinct codes.

## 11. Receive/decode failures are outside this mapping

C03e-CU does not map `AuthenticatedPrwcCommandReceiveError` into a candidate-publication result frame.

Reason:
- the current receive seam terminalizes on frame/decode failure;
- only a successfully decoded `CandidatePublicationControlFrame` exposes the typed candidate-publication correlation consumed by this result composition;
- changing malformed-frame response behavior would require a separate receive/error-correlation contract and must not be smuggled into post-execution framing.

Thus CU result framing begins only after successful candidate-publication Command decode.

## 12. No frame I/O yet

C03e-CU selects pure frame construction/decoding semantics only.

It does not authorize:
- `ControlTlsServerStream::write_frame` calls;
- a new public stream getter;
- response write retry;
- reconnect;
- a command loop;
- task spawning;
- connection keepalive/close policy after response;
- listener/runtime activation.

A later checkpoint must separately select the write-composition seam.

## 13. Future source boundary

A future source-materialization checkpoint may introduce one narrow module equivalent to:

`crates/prw-remote-bridge/src/candidate_publication_result_wire.rs`

and register/export it through `root.rs`.

The source checkpoint should not need to modify:
- `candidate_publication_execution.rs`;
- `prwc_connection_authentication.rs`;
- `candidate_publication_wire.rs` submission bytes;
- requester/rendezvous provider source;
- reachability owner source;
- Cargo manifests or lockfiles.

The new result module may reuse public PRWP magic/version constants and `CandidatePublicationFreshnessToken`.

## 14. Selected public result surface

The future module should expose a typed result message equivalent to:

`CandidatePublicationResultMessage::{ Accepted { replacement_freshness }, Rejected }`

and pure helpers equivalent to:
- encode one result message with caller-supplied request ID into a bounded `ControlFrame`;
- decode one result `ControlFrame` back into the typed message while validating exact outer-kind/operation pairing;
- optionally project one semantic execution result into the typed message/frame without performing I/O.

Exact naming may follow repository conventions, but semantics may not change silently.

## 15. Strict decoder behavior

A future result decoder must reject:
- wrong magic;
- unsupported major/minor version;
- unknown operation;
- non-zero reserved field;
- operation `2` with outer kind other than `Response`;
- operation `3` with outer kind other than `Error`;
- zero/invalid freshness token for accepted result;
- missing/truncated/trailing accepted bytes;
- any payload bytes after generic rejection;
- outer Command/Event/Heartbeat/Authentication for terminal result operations.

Successful decode proves wire structure only; it does not establish semantic authorization or durable commit provenance.

## 16. Frame-construction failures

The pure result codec may expose a stable local codec error for invalid payload/outer-frame construction.

A frame-construction failure is not converted recursively into another Error frame. It returns to the caller as a local composition failure.

For normal post-decode candidate-publication use, the echoed request ID is already non-zero by the Phase 129 frame contract, but the pure codec remains defensive for direct callers/tests.

## 17. No new dependency

All selected semantics use existing:
- `prw-control-transport` frame types;
- `CandidatePublicationFreshnessToken`;
- existing PRWP constants/format conventions;
- standard-library facilities.

No Cargo manifest or lockfile change is selected.

## 18. Focused future tests

A later source checkpoint must prove at least:
1. accepted result encodes outer `Response`;
2. accepted result preserves exact caller-supplied request ID;
3. accepted result carries exactly the replacement freshness and has 44-byte PRWP payload;
4. rejected result encodes outer `Error`;
5. rejected result preserves exact caller-supplied request ID;
6. rejected result has exactly the 12-byte generic PRWP header and no detailed error body;
7. every representative `CandidatePublicationExecutionError` projects to the same generic rejection;
8. accepted decoder rejects wrong outer kind;
9. rejected decoder rejects wrong outer kind;
10. malformed magic/version/operation/reserved/truncation/trailing data fail closed;
11. accepted zero freshness bytes fail closed through the existing freshness-token constructor;
12. direct zero request ID fails through existing `ControlFrame` validation;
13. projection does not read or mutate `PrwcRequestIdLifecycle`;
14. no result path exposes `invalidated_traversal`.

Tests require no socket, listener, database, thread, runtime or product bootstrap.

## 19. Runtime/process ownership remains separately gated

C03e-CU does not choose:
- which product process owns `InMemoryRequesterRendezvousAuthorityProvider`;
- provider instance lifetime;
- how registration/retirement/removal commands reach that same instance;
- synchronization primitive or task ownership;
- how a `ProductionReachabilityOwner` is located for a publisher;
- listener/accept-loop wiring;
- runtime shutdown behavior.

Those remain later prerequisites.

## 20. No success before durable commit

A `Response`/Accepted result is valid only after `execute_authenticated_candidate_publication(...)` has returned `Ok(ReachabilityCommitOutcome)`.

No pre-commit acknowledgement, optimistic acceptance, provider authorization success alone, candidate validation success alone or persistence-attempt initiation may emit Accepted.

## 21. No rejection detail as authority side channel

The generic Error/Rejected result is not an authorization oracle.

Externally observable candidate-publication failure remains one terminal generic class at this checkpoint. Detailed internal error taxonomy is deliberately not serialized.

## 22. Exact C03e-CU diff boundary

C03e-CU is docs-only.

The exact CT -> CU diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CU_CANDIDATE_PUBLICATION_RESPONSE_ERROR_FRAME_COMPOSITION_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, runtime/listener, networking, deployment, provider-state mutation or unrelated contract blocks CU closure.

## 23. Explicitly rejected shortcuts

C03e-CU rejects:
- allocating a new response request ID;
- exposing local request-ID lifecycle state;
- serializing internal error strings;
- exposing requester/rendezvous authority classifications;
- exposing persistence/recovery classifications;
- exposing `invalidated_traversal` without a protocol need;
- acknowledging before durable commit;
- mapping malformed receive/decode errors through the post-execution result contract;
- writing frames in the pure result codec;
- adding retries or loops;
- choosing runtime ownership implicitly;
- activating listener/network/runtime paths;
- deployment or merge.

## 24. Safe successor after durable CU closure

After CU is durably closed, a fresh exact-head audit may authorize a bounded source-materialization checkpoint equivalent to:
1. a source-materialization contract;
2. new `candidate_publication_result_wire.rs`;
3. `root.rs` only for module registration/export.

That successor should require no manifest/lock change and no runtime ownership.

After result-wire source validates, a new prerequisite audit is required before selecting response write composition versus runtime/process ownership. No direct jump to command loop, listener activation, production networking, deployment or merge is allowed.

## 25. Closure requirements

C03e-CU may close only if one exact final head proves:
1. exact CT merge base and a bounded docs-only commit;
2. only the CU contract path changed;
3. canonical automatically-triggered validation is terminal and non-failing; non-applicable workflows are recorded as `SKIPPED`, not `PASS`;
4. root and Android-native lock blobs remain byte-stable;
5. immutable Drive audit is raw-read back exactly;
6. rolling Drive predecessor is freshly guarded and preserved byte-for-byte as prefix;
7. PR body changes to `Status: CLOSED` only after durable evidence succeeds;
8. PR remains draft/open/unmerged.

Until those conditions are satisfied, C03e-CU remains staged and selects no production behavior.
