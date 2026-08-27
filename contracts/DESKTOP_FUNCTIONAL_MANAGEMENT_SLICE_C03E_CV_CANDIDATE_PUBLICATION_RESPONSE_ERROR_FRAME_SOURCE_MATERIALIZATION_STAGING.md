# Phase 152 C03e-CV — Candidate Publication Response/Error Frame Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Target gate:
`C03E_CV_CANDIDATE_PUBLICATION_RESPONSE_ERROR_FRAME_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-CU is the authoritative predecessor:

- branch: `phase-152-c03e-cu-candidate-publication-response-error-frame-composition-selection-staging`
- head: `642973eb364bf0778ad79ff1c478783582a04fd8`
- tree: `e241aa3a940aeb57bdbe39cc18f741d940598170`
- gate: `C03E_CU_CANDIDATE_PUBLICATION_RESPONSE_ERROR_FRAME_COMPOSITION_SELECTED`
- PR #218: body `Status: CLOSED`, draft/open/unmerged

C03e-CV may materialize only the exact CU-selected pure result representation and composition.

## 2. Fresh post-CU source prerequisite audit

Exact-CU inspection confirms:

- `candidate_publication_wire.rs` publicly exposes the existing PRWP magic/version/header constants without requiring any submission-format mutation;
- `CandidatePublicationFreshnessToken` already exposes exact 32-byte non-zero verifier token material through `as_bytes()` and validates decoded bytes through `new(...)`;
- `ControlFrame` and `ControlMessageKind::{Response, Error}` already provide the bounded outer envelope and non-zero request-ID validation;
- `CandidatePublicationControlFrame::request_id()` already exposes the peer-originated correlation selected by CU;
- `CandidatePublicationExecutionError` is already the complete internal semantic execution failure surface needed for one generic rejection projection;
- `ReachabilityCommitOutcome::replacement_freshness()` already exposes the only successful wire field selected by CU;
- representative top-level execution failure variants can be constructed in focused unit tests without changing execution, reachability-owner, requester-provider, registry, or transport source;
- `root.rs` requires only one additional public module registration;
- `prw-remote-bridge` already depends on every crate required by the pure codec.

No new dependency, manifest change, lockfile change, runtime owner, stream getter, listener, task, database, synchronization primitive, or network operation is required.

## 3. Exact C03e-CV authorized diff

The exact CU -> CV diff is authorized to contain only:

1. this source-materialization contract:
   `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CV_CANDIDATE_PUBLICATION_RESPONSE_ERROR_FRAME_SOURCE_MATERIALIZATION_STAGING.md`
2. new source:
   `crates/prw-remote-bridge/src/candidate_publication_result_wire.rs`
3. `crates/prw-remote-bridge/src/root.rs` only to add:
   `pub mod candidate_publication_result_wire;`

Any other changed path blocks CV closure.

## 4. Materialized result message

The new module must expose a typed message semantically equivalent to:

`CandidatePublicationResultMessage::{ Accepted { replacement_freshness: CandidatePublicationFreshnessToken }, Rejected }`

No field for requester identity, publisher identity, transport identity, candidate set, traversal invalidation, durable-store state, runtime state, or internal error text is authorized.

## 5. Accepted encoding

Accepted must encode:

- outer kind: exactly `ControlMessageKind::Response`
- caller-supplied request ID unchanged
- PRWP magic/version unchanged
- operation: exactly `2`
- reserved: exactly zero
- exactly one 32-byte replacement freshness token
- exact payload size: 44 bytes

The replacement freshness is the existing verifier-issued token represented by `CandidatePublicationFreshnessToken`.

## 6. Rejected encoding

Rejected must encode:

- outer kind: exactly `ControlMessageKind::Error`
- caller-supplied request ID unchanged
- PRWP magic/version unchanged
- operation: exactly `3`
- reserved: exactly zero
- no body
- exact payload size: 12 bytes

No detailed rejection code or internal failure classification may be serialized.

## 7. Strict decoding

The decoder must fail closed on:

- wrong magic;
- unsupported major/minor version;
- unknown operation;
- non-zero reserved field;
- accepted payload length other than exactly 44 bytes;
- rejected payload length other than exactly 12 bytes;
- zero/invalid accepted freshness token;
- accepted operation carried by any outer kind other than `Response`;
- rejected operation carried by any outer kind other than `Error`;
- truncation or trailing bytes.

Successful decode proves only result wire structure. It does not establish semantic authorization or durable commit provenance.

## 8. Stable local codec errors

The module may expose only a bounded local classification equivalent to:

- `InvalidOuterKind`
- `InvalidPayload`
- `Frame(ControlFrameError)`

Frame-construction failure returns locally. It must not recursively create another Error frame.

## 9. Execution-result projection

The module must expose a pure projection equivalent to:

`Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError> -> CandidatePublicationResultMessage`

Rules:

- `Ok(outcome)` -> `Accepted { replacement_freshness: outcome.replacement_freshness() }`
- every `Err(_)` -> exactly `Rejected`
- nested error variants must not be inspected for distinct external codes
- `outcome.invalidated_traversal()` must not be read or serialized

This projection performs no candidate-publication execution and no I/O.

## 10. Exact decoded-command correlation helper

A pure composition helper may accept:

- `&CandidatePublicationControlFrame`
- `Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>`

and return a framed terminal result.

It must use exactly `command.request_id()` and must not:

- allocate a request ID;
- register it in `PrwcRequestIdLifecycle`;
- mutate connection-local request-ID state;
- derive correlation from freshness or identity fields.

## 11. No success before semantic success

The execution-result projection must produce Accepted only from `Ok(ReachabilityCommitOutcome)`.

Requester-authority success alone, validation success alone, persistence-attempt initiation, or any error result must never project to Accepted.

The standalone typed codec remains a pure representation utility; later write composition remains responsible for using the execution-result projection on the live path.

## 12. Focused required tests

CV source tests must prove at least:

1. accepted result uses outer `Response`;
2. accepted result preserves exact request ID;
3. accepted result carries exactly the replacement freshness and is exactly 44 payload bytes;
4. rejected result uses outer `Error`;
5. rejected result preserves exact request ID;
6. rejected result is exactly the 12-byte generic PRWP header;
7. representative top-level `CandidatePublicationExecutionError` classes all project to the same `Rejected`;
8. accepted decoder rejects wrong outer kind;
9. rejected decoder rejects wrong outer kind;
10. malformed magic/version/operation/reserved fields fail closed;
11. truncation/trailing bytes fail closed;
12. zero accepted freshness fails closed through the existing freshness-token constructor;
13. direct zero request ID fails through existing `ControlFrame` validation;
14. decoded-command execution-result framing echoes the existing command request ID and allocates no replacement;
15. the execution projection signature consumes only the existing semantic result type and exposes no traversal-invalidation field.

Tests must use no socket, listener, database, thread, async runtime, product bootstrap, or deployment.

## 13. Explicitly unchanged source

CV must not modify:

- `candidate_publication_wire.rs`;
- `candidate_publication_control_frame.rs`;
- `candidate_publication_execution.rs`;
- `candidate_publication_freshness.rs`;
- `prwc_connection_authentication.rs`;
- `prwc_request_id_lifecycle.rs`;
- `requester_rendezvous_authority.rs`;
- `requester_rendezvous_in_memory_provider.rs`;
- `reachability_owner.rs`;
- any Cargo manifest or lockfile;
- any workflow;
- any runtime/listener/network/deployment source.

## 14. Explicitly not materialized

CV does not materialize:

- `ControlTlsServerStream::write_frame`;
- response write composition;
- command loop or retry;
- keepalive/connection-close policy;
- runtime/process ownership of the in-memory requester/rendezvous provider;
- provider instance lifetime;
- provider registration ingress;
- synchronization primitive;
- production reachability-owner lookup;
- listener/accept-loop wiring;
- process shutdown behavior;
- database/persistence schema;
- TTL/clock/cleanup;
- production networking;
- credentials/bootstrap;
- deployment or merge.

## 15. No manifest or lock change

All required types are already available to `prw-remote-bridge`.

No Cargo manifest or lockfile change is expected or authorized.

## 16. Closure requirements

C03e-CV may close only if one exact final head proves:

1. exact CU merge base;
2. only the three authorized paths changed;
3. no manifest/lock/workflow/runtime/networking/unrelated change;
4. canonical exact-head Rust validation is terminal and successful;
5. Android validation is recorded according to actual trigger/result and never inferred;
6. both root and Android-native lock blobs remain byte-stable;
7. any strictly mechanical rustfmt/Clippy correction remains within the same authorized three-path boundary and is separately evidenced;
8. immutable Drive audit is raw-read back exactly;
9. rolling Drive predecessor is freshly guarded and preserved byte-for-byte as prefix;
10. PR body changes to `Status: CLOSED` only after durable evidence succeeds;
11. PR remains draft/open/unmerged.

Until these conditions are satisfied, C03e-CV remains staged and activates no production behavior.

## 17. Safe successor after durable CV closure

After durable CV closure, a fresh exact-head audit must choose between:

- candidate-publication response write composition on an already-authenticated connection; or
- runtime/process ownership of the bounded in-memory requester/rendezvous provider.

Those must remain separately gated.

No successor may jump directly to command loop, listener activation, production networking, deployment, or merge.
