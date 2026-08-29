# Phase 152 C03e-FC — Requester/Rendezvous Retained-Custody DR Response Materialization Semantics Selection (Staging)

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-FC selects only the semantics and ownership boundary for the next requester/rendezvous response-materialization step after the exact C03e-FB retained-custody DR continuation.

The selected boundary consumes no source custody in this checkpoint. It defines what a later separately gated source materialization may project from the exact FB terminal `Result<(), RequesterRendezvousStartCompositionError>` while preserving the exact retained `PostAuthRequesterRendezvousTransaction` and its same-stream response custody.

C03e-FC is docs-only. It constructs no requester response frame, performs no response write, consumes no stream, resumes no ingress loop, selects no candidate/endpoint/relay, dials no traffic, and activates no runtime/listener/bootstrap/deployment path.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fb-requester-rendezvous-retained-custody-dr-continuation-source-materialization-staging`
- head: `8707ca79f478624064c99294468316cb440949c7`
- tree: `1dd93d89eb6033a61e8cc3a9f19faaf4b01193df`
- FB contract blob: `53d55dec1a73e830b98db4fe77d0c0b95c214ba2`
- FB isolated continuation blob: `9550148740b654a79eca8e51956bf37a351ac802`

FC must remain an exact docs-only descendant of that head.

## 3. Audited predecessor source guards

The FC selection is bounded by these exact FB-head blobs:

- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
  - `9550148740b654a79eca8e51956bf37a351ac802`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
  - `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`
- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - `d5562a9587bdbde7d05e38fdd704d42f9d20f3c8`
- `crates/prw-remote-bridge/src/requester_rendezvous_target_request_wire.rs`
  - `2bfb2d6119a0bc3e1278fb361955093361949db1`
- `crates/prw-remote-bridge/src/candidate_publication_result_wire.rs`
  - `b7aa6b05a037cdc5db1bcd454f5c076890d95c96`

The candidate-publication result codec is audited only as an existing fail-closed projection precedent. It is not selected for reuse by requester/rendezvous.

## 4. Exact predecessor facts

At the exact FB head:

- the FB terminal envelope retains by value the exact `PostAuthRequesterRendezvousTransaction` plus the exact `Result<(), RequesterRendezvousStartCompositionError>`;
- `PostAuthRequesterRendezvousTransaction` retains the exact strict `RequesterRendezvousTargetWireRequest` plus the exact already-accepted `MeshControlStream`;
- the strict requester request preserves the exact outer PRWM `request_id` as correlation only and the exact nominated logical target `DeviceId`;
- DR success means the existing requester/rendezvous start validation, requester-aware authorization, and live requester registration mutation completed successfully;
- DR success does not prove candidate selection, target reachability, rendezvous completion, endpoint selection, relay selection, transport establishment, dial success, port-forward availability, terminal availability, or remote-session establishment;
- no requester/rendezvous terminal response codec exists in the exact FB source tree;
- candidate-publication has a separate terminal result codec, but FB explicitly forbids candidate-publication result-codec reuse for requester/rendezvous.

These facts constrain the FC selection.

## 5. Selected semantic boundary

C03e-FC selects one requester-specific terminal **DR acknowledgement** semantic family for later pure source materialization.

The future projection boundary must consume the exact FB DR terminal result and classify it only as:

1. **accepted for requester/rendezvous continuation** when the exact DR result is `Ok(())`; or
2. **rejected** when the exact DR result is any `Err(RequesterRendezvousStartCompositionError)`.

This is deliberately a DR-stage acknowledgement, not a rendezvous-completion result.

The exact Rust enum/type names, payload magic, operation numbers, byte layout, and encode/decode function names remain source-materialization details for the next separately gated checkpoint. FC selects semantics, not bytes.

## 6. Accepted semantic law

A future requester-visible accepted acknowledgement may mean only:

- the exact authenticated-session-derived requester intent passed the existing DR validation;
- the separately supplied requester-aware policy authorized that exact requester/target start intent;
- the existing requester/rendezvous runtime owner accepted the existing registration mutation; and
- the request is accepted for the separately gated continuation that may later perform candidate/reachability work.

Accepted must not mean or imply:

- target online;
- candidate available;
- candidate selected;
- target reachable;
- endpoint resolved;
- relay selected;
- direct path available;
- QUIC/TCP connected;
- port-forward established;
- terminal established;
- remote session established;
- rendezvous complete; or
- end-to-end operation success.

No later code may strengthen this accepted meaning without a new explicit gate.

## 7. Rejected semantic law

Every exact `RequesterRendezvousStartCompositionError` remains available internally at the FB boundary, but the requester-specific terminal DR acknowledgement selected here is fail-closed and coarse externally.

A future pure projector must map any DR error to one generic requester-visible rejection unless a later separately gated contract explicitly selects a richer stable external taxonomy.

The response surface must not accidentally expose:

- registry lookup internals;
- policy evaluator internals;
- provider mutation internals;
- requester registration implementation details;
- internal error discriminants;
- internal debug/display strings; or
- implementation-specific retry or recovery hints.

FC selects no retry, fallback, replacement, fabricated success, or alternate authorization path.

## 8. Correlation and identity law

The future response must preserve the exact requester transaction's original non-zero PRWM `request_id` as echo correlation only.

The request ID must never become:

- requester identity;
- target identity;
- authenticated-session identity;
- capability identity;
- transport identity;
- authorization evidence; or
- provider/rendezvous ownership evidence.

Requester identity remains the authenticated PRW application session already captured in the exact EZ start intent consumed by DR. Target identity remains the exact nominated logical `DeviceId` from that intent/request lineage.

No IP address, port, stream order, endpoint, peer socket, connection tuple, or `TransportIdentity` alone may replace those logical identities.

## 9. Response-family ownership selection

C03e-FC selects a requester/rendezvous-specific result family rather than candidate-publication codec reuse.

A later source-materialization checkpoint should keep pure requester/rendezvous response projection/framing at the bridge protocol boundary beside the existing strict requester request codec, while Agent-owned continuation code supplies only the already-completed exact FB DR result and retained transaction custody needed for that projection.

This ownership split must preserve these boundaries:

- Agent code owns the semantic provenance of the exact DR result;
- bridge protocol code owns requester/rendezvous wire framing semantics;
- the retained bridge transaction owns the exact same-stream response custody until a later separately gated write consumes it;
- no layer may reconstruct a second requester identity or target intent from transport metadata.

FC does not itself add a module, export, adapter, transaction method, frame encoder, or stream writer.

## 10. Candidate-publication codec non-reuse

The existing candidate-publication result codec demonstrates a useful architectural precedent: internal semantic failures can project to a generic external rejection without leaking internal classes.

FC selects only that fail-closed projection principle.

It explicitly does **not** select reuse of:

- candidate-publication wire magic;
- candidate-publication operation tags;
- candidate-publication result enum/types;
- candidate-publication freshness fields;
- candidate-publication encode/decode functions; or
- candidate-publication request/result protocol family.

Requester/rendezvous requires its own protocol semantics because its accepted state means only DR-stage authorization plus requester registration, not candidate-publication commit semantics.

## 11. Framing selection boundary

FC selects the following framing invariants for later materialization:

- one requester-specific terminal DR acknowledgement family;
- exact original PRWM request correlation echoed unchanged;
- accepted and rejected remain distinguishable at the requester-facing protocol boundary;
- accepted carries no fabricated candidate, endpoint, relay, reachability, transport, or session-success data;
- rejected carries no internal DR error detail by default;
- framing is bounded and pure before any I/O;
- frame construction failure remains a local materialization failure and must not be converted into semantic accepted/rejected success.

FC intentionally does not assign exact payload magic, version increments, operation numbers, reserved bits, payload byte counts, or Rust symbol names. Those require exact-source materialization under the next gate.

## 12. Same-stream custody law

The exact `PostAuthRequesterRendezvousTransaction` must remain retained until the future response path deliberately consumes it.

Pure response projection/framing must not itself:

- read the stream;
- write the stream;
- accept another stream;
- clone or duplicate stream custody;
- close the peer;
- retry I/O; or
- resume mixed-family ingress.

A later separately gated write-ownership checkpoint may consume the exact transaction to send exactly one already-materialized requester/rendezvous terminal DR acknowledgement on the same control stream.

FC does not select that write implementation yet.

## 13. Repeated-ingress barrier

The existing requester path remains terminal while retained response custody is unresolved.

No second control-stream accept/read and no second frame receive may occur before the separately gated requester response path has resolved custody according to later contracts.

FC creates no:

- task;
- queue;
- channel;
- worker;
- concurrent reader;
- speculative pre-accept;
- fairness policy;
- backpressure policy;
- retry policy; or
- loop-resume policy.

## 14. Failure-boundary preservation

The future pure materializer must preserve three distinct conceptual layers:

1. the exact completed DR semantic result;
2. the coarse requester-visible accepted/rejected semantic projection; and
3. any local response-frame construction error.

These layers must not be flattened into one ambiguous success/failure value.

In particular:

- an internal DR error must project to semantic rejected, not to a local codec error;
- a local codec/frame-construction error must not become semantic rejected and must not fabricate a frame;
- semantic accepted must originate only from exact DR `Ok(())`;
- semantic rejected must not be retried through DR; and
- no local failure may cause a second registration mutation.

## 15. Source-stability guard

C03e-FC is docs-only.

All predecessor Rust, Android, dependency, workflow, protocol implementation, and runtime source must remain byte-stable in this checkpoint. In particular the exact source guards in section 3 must remain unchanged.

Any source incompatibility discovered by validation is a contradiction requiring a new explicit decision; it is not permission to widen FC into implementation.

## 16. Explicit non-goals

C03e-FC does not materialize or authorize:

- requester/rendezvous response Rust source;
- exact response magic/version/operation numbers;
- response frame construction;
- response frame decode;
- response stream write;
- transaction send adapter;
- candidate-publication result-codec reuse;
- detailed external DR error taxonomy;
- retry/fallback/replacement/fabricated success;
- stream close or peer-close policy;
- second accept/read or mixed-family loop resume;
- provider query/select beyond the existing completed DR registration mutation;
- candidate selection;
- reachability selection;
- endpoint selection;
- relay selection;
- QUIC/TCP dialing;
- port-forward activation;
- terminal activation;
- remote-session activation;
- runtime/listener/bootstrap activation;
- Android behavior changes;
- dependency/workflow widening;
- packaging;
- deployment;
- restart;
- recovery; or
- merge.

## 17. Next-gate recommendation

If FC closes cleanly, the next separately gated checkpoint may materialize only the selected pure requester/rendezvous terminal DR acknowledgement projection/framing source.

That next checkpoint should remain pure and no-I/O unless its own contract explicitly and narrowly proves that response-write ownership belongs in the same gate. The preferred decomposition is to materialize pure requester-specific response semantics first and keep same-stream write ownership separately gated.

No later checkpoint may infer permission for candidate/reachability continuation, dialing, loop resume, runtime activation, deployment, or merge from FC.

## 18. Validation contract

Closure requires exact-final-head validation.

Because FC is docs-only:

- Rust validation must pass the repository's exact required locked dependency graph, rustfmt, Clippy with warnings denied, workspace tests, and workspace build if that workflow is triggered for the final FC head;
- Android validation is claimed only if the workflow is actually triggered and passes on the exact final FC head;
- source byte-stability must be verified against the exact FB guards;
- branch ancestry must show exact FB merge base, ahead only by the FC docs commit(s), behind zero; and
- durable audit evidence must record the exact final FC head/tree, changed-path count, source guards, workflow evidence, closure marker, and gate marker.

## 19. Canonical closure target

Intended closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_RETAINED_CUSTODY_DR_RESPONSE_MATERIALIZATION_SEMANTICS_SELECTION`

Intended gate marker:

`C03E_FC_REQUESTER_RENDEZVOUS_RETAINED_CUSTODY_DR_RESPONSE_MATERIALIZATION_SEMANTICS_SELECTED`
