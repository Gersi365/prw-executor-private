# Phase 152 — C03e-GO Candidate Publication Current-Mesh Same-Stream Terminal Response Custody Source Materialization Staging

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-GO source-materializes only the bridge-owned candidate-publication same-stream terminal-response custody primitive selected by canonically CLOSED C03e-GN.

It does not activate the Agent candidate higher-owner handoff, invoke candidate semantic execution, compose terminal result semantics, populate production reachability owners, integrate a worker/loop, alter transport runtime, activate traversal/listener/readiness/dialing, deploy, restart/recover a process, merge, delete branches, or change repository visibility.

## 2. Canonical predecessor

C03e-GN is canonically CLOSED and frozen.

- GN branch:
  `phase-152-c03e-gn-candidate-publication-current-mesh-same-stream-terminal-response-custody-semantics-selection-staging`
- GN final head:
  `a60bb6d48b46a920be68cc8265eb8d972c070e67`
- GN final tree:
  `fa9b0a6fb8db5e0e1b9593849883e58fd4cbd01d`
- GN PR:
  `#316`
- GN state:
  draft / open / unmerged / `Status: CLOSED`
- canonical GN immutable Drive object:
  `1IWVVtLHnG8E-QsJ6UkE0yAgCiqR3K-9w`
- GN audit bytes:
  `13019`
- GN audit SHA-256:
  `2babcc6779b278f36f6ca440b95ad182fdc37fbb9adc3c8d6585dcd0f4449e01`

GO starts exactly from the GN final head and does not amend GN.

## 3. Fresh exact-GN source audit

Fresh source readback at exact GN head confirmed:

1. `PostAuthCandidatePublicationTransaction` retains exactly one `CandidatePublicationMeshRequest` plus the exact same already-accepted `MeshControlStream`.
2. The transaction already exposes consuming `into_parts()` custody transfer.
3. `CandidatePublicationMeshRequest::request_id()` exposes exact retained current-Mesh correlation only.
4. Existing requester/capability response seams prove the crate already owns consuming same-stream response I/O.
5. `MeshControlStream::send_frame(...)` already owns bounded encode/write/send-direction finish and typed `MeshQuicRuntimeError` failure.
6. GM already owns pure current-Mesh candidate terminal frame composition.
7. Agent `CandidatePublicationHandoffNotSelected` remains active.
8. No manifest, lockfile or transport runtime change is required.

Exact pre-GO ingress source blob:
`8294cd236dcc497da87e859afdf675b79aa24085`

Exact pre-GO `root.rs` blob:
`aa63339c93b0ffe53a54e0b5267002d04d2ca00d`

## 4. Fresh-audit scope refinement

GN expected the likely source focus to be the existing large `post_auth_control_stream_ingress.rs` plus one contract, while explicitly requiring any broader path need to be justified by fresh audit.

Fresh GO audit selected a safer three-path representation:

1. one new dedicated bridge module containing the candidate-specific response-custody error and inherent implementation on the existing transaction type;
2. one single-line crate-root module exposure;
3. this GO contract.

This avoids replacing the large ingress file while preserving the same public inherent-method semantics through the existing public `into_parts()` custody transfer.

The refinement increases changed-path count by one but materially reduces accidental full-file rewrite risk and introduces no new dependency, ownership layer, runtime mechanism or authority surface.

## 5. Materialized response-custody type

GO adds:
`CandidatePublicationTerminalResponseIoError`

with distinct variants:
- `CorrelationMismatch`;
- `Runtime(MeshQuicRuntimeError)`.

The classification preserves GN's separation between:
- semantic execution failure;
- GM frame-construction failure;
- exact response correlation mismatch;
- lower stream response-I/O failure.

`From<MeshQuicRuntimeError>` preserves the typed lower cause.

Correlation mismatch is not converted into `MeshQuicRuntimeError` and is not peer-visible semantic Rejected.

## 6. Materialized consuming send seam

GO adds the inherent async method:
`PostAuthCandidatePublicationTransaction::send_terminal_response_frame(...)`

The method:
1. consumes `self` by value;
2. transfers the retained request + exact retained stream through existing `into_parts()`;
3. validates exact retained request-ID equality before any call to `send_frame(...)`;
4. fails locally on mismatch;
5. on equality, sends the supplied already-composed `ControlFrame` exactly once on the exact retained stream;
6. delegates write + send-direction finish to existing `MeshControlStream::send_frame(...)`;
7. returns no stream custody;
8. performs no second read, retry, fallback response, reconnect, alternate stream open/accept, peer close or loop continuation.

## 7. Exact correlation law

GO compares only:

`response_frame.request_id() == retained_request.request_id()`

before stream I/O.

Mismatch performs no `send_frame(...)` call and does not:
- rewrite the response request ID;
- allocate a replacement request ID;
- build a fallback Rejected frame;
- rerun candidate semantic execution;
- rerun durable commit;
- replay requester cleanup;
- reopen/reaccept another stream;
- return stream custody.

Request ID remains correlation only and is never logical identity, authenticated publisher authority, transport identity, owner identity, requester authority, freshness authority, durable authority, endpoint identity or socket identity.

The dynamic-network identity law remains:

`logical device/session identity -> registry/discovery -> current reachable endpoint/candidates -> authenticated transport`

not static IP and not request ID.

## 8. Payload and semantic non-ownership

GO accepts one already-composed current-Mesh `ControlFrame`.

It does not:
- decode or reparse PRWP terminal bytes;
- inspect replacement freshness;
- reconstruct Accepted/Rejected;
- rebuild `ControlMessageKind`;
- convert to historical `prw_control_transport::ControlFrame`;
- fabricate `CandidatePublicationControlFrame`;
- fabricate `AuthenticatedPrwcConnection`;
- mutate terminal payload bytes.

GM remains the pure frame-composition boundary.

## 9. Failure/replay law

A GO correlation mismatch or runtime response-I/O failure does not authorize:
- semantic replay;
- second durable commit;
- freshness reissue;
- owner lookup replay;
- requester grant replay;
- requester cleanup replay;
- second GM composition;
- fallback Rejected construction;
- alternate request correlation;
- reconnect or alternate stream custody.

Definite durable semantic success remains durable even if response I/O later fails.

## 10. Focused tests

GO source includes focused unit coverage for:
- exact correlation acceptance;
- mismatched correlation fail-closed classification before the send seam;
- typed `MeshQuicRuntimeError` preservation;
- existence of the consuming inherent send surface.

The tests require no listener, deployed network, traversal, Agent bootstrap or production owner population.

## 11. Exact authorized GO path ceiling

GO authorizes exactly three paths:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GO_CANDIDATE_PUBLICATION_CURRENT_MESH_SAME_STREAM_TERMINAL_RESPONSE_CUSTODY_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-remote-bridge/src/candidate_publication_same_stream_response_custody.rs`
3. `crates/prw-remote-bridge/src/root.rs`

No other path is authorized.

In particular no `Cargo.toml`, `Cargo.lock`, workflow, Android/Kotlin/Gradle, Agent source, existing ingress source, transport runtime, listener, bootstrap, readiness, persistence, deployment or configuration path may change.

## 12. Current source evidence before validation

New response-custody module semantic commit:
`76e4f3baac0cd609fd743ce93bd48f35337a8f1d`

New module blob before any later correction:
`85bf842f40f7ae73f57b23953e55e070f9886d57`

Root exposure commit:
`f39de51c3b99d4e35b391741b51489301a3dc22b`

Current root blob:
`7619fed9f1de890bda2c43ec909f63dc50bfa927`

The root change is exactly one `pub mod candidate_publication_same_stream_response_custody;` declaration.

## 13. Validation law

Only the exact final GO head may provide closure evidence.

Required observations:
- PRW Rust Validation must pass locked graph, rustfmt, Clippy, workspace tests and workspace build on exact final head;
- Android result must be observed if the workflow triggers for this source delta;
- path-filtered workflows must be recorded accurately as success/skipped/failure;
- no superseded head PASS may be reused as final closure evidence;
- no manual workflow dispatch is authorized.

Any formatter/Clippy correction must remain inside the exact authorized source path set and be proven as narrow correction evidence.

## 14. Immutable evidence closure law

After exact-final-head CI:
1. freeze branch/source state;
2. record exact head/tree/compare/path/blob evidence;
3. create immutable GO audit locally;
4. record exact bytes and SHA-256;
5. upload exact bytes directly to canonical Drive folder `Private Remote Workspace / 1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch the exact Drive object;
7. require exact byte-count/SHA equality;
8. reread PR + branch before closure metadata mutation;
9. set PR body `Status: CLOSED` while keeping draft/open/unmerged;
10. independently reread PR + branch after closure.

No My Drive root staging is authorized.

## 15. Explicit exclusions

GO does not authorize:
- Agent candidate higher-owner handoff activation;
- removal of `CandidatePublicationHandoffNotSelected`;
- calling GK semantic execution from ingress;
- calling GM composition from ingress;
- repeated ingress worker integration;
- cancellation/peer-close policy;
- owner-map population/recovery/synchronization;
- transport runtime changes;
- traversal activation;
- listener/readiness activation;
- dialing;
- deployment;
- process restart/recovery;
- merge;
- branch deletion;
- repository visibility change.

## 16. Canonical closure target

Canonical closure:
`CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_SAME_STREAM_TERMINAL_RESPONSE_CUSTODY_SOURCE_MATERIALIZATION`

Canonical gate:
`C03E_GO_CANDIDATE_PUBLICATION_CURRENT_MESH_SAME_STREAM_TERMINAL_RESPONSE_CUSTODY_SOURCE_MATERIALIZED`

## 17. Successor rule

After canonical GO closure, perform a fresh exact-final-head source audit before naming or mutating a successor.

The next missing prerequisite may concern higher-owner orchestration of retained candidate transaction + authenticated publisher + GK semantic execution + GM frame composition + GO response custody, or production-owner runtime population required before such orchestration can be sound. GO does not pre-authorize the ordering between those concerns.
