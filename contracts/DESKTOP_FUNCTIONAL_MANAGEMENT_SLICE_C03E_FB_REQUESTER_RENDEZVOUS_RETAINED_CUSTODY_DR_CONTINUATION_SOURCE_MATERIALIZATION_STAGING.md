# Phase 152 C03e-FB — Requester/Rendezvous Retained-Custody DR Continuation Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FB materializes only the C03e-FA-selected Agent-owned continuation seam that consumes one exact C03e-EZ requester/rendezvous response-stream custody handoff, runs exactly the existing C03e-DR authority composition under the existing shared-current authority read pattern, and returns the exact requester transaction together with the exact DR terminal result.

This checkpoint does not construct or write a requester response, does not resume the mixed-family ingress loop, does not select candidates/endpoints/relays, does not dial traffic, and does not activate a runtime/listener/bootstrap/deployment path.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fa-requester-rendezvous-retained-custody-dv-continuation-ownership-selection-staging`
- head: `51ae3d2c54eef5e22ac7e6cb0c57a7a4150236e2`
- tree: `4ba4c81c1221ae13c65edb1c98aec2ced15815f0`
- FA contract blob: `72c0a65ab07b6d481222fceb6b718d328b71b8c6`

FB must remain an exact descendant of that head.

## 3. Source guards

The source materialization is bounded by these exact FA-predecessor blobs:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - `66f8435314d1ede2474292d811276e16919ddcad`
- `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
  - `50356b47d3c5304b67edd424e9286beb028ace16`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
  - `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`
- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - `d5562a9587bdbde7d05e38fdd704d42f9d20f3c8`

FB may modify only the narrow Agent ownership seam needed to materialize the FA selection plus this contract. The DR composition, shared-current authority implementation and bridge requester transaction remain byte-stable unless validation proves a concrete source incompatibility.

## 4. Materialized ownership law

FB must preserve this exact by-value chain:

`RequesterRendezvousResponseStreamCustodyHandoff`
`-> exact PostAuthRequesterRendezvousTransaction retained by value`
`-> exact C03e-EZ RequesterRendezvousStartIntent consumed by DR`
`-> SharedCurrentCapabilityAuthority::with_current_authority(...) once`
`-> validate_authorize_and_register_requester_rendezvous_start(...) once`
`-> exact PostAuthRequesterRendezvousTransaction + exact Result<(), RequesterRendezvousStartCompositionError>`

The requester transaction must survive both DR success and DR failure unchanged.

## 5. Authority semantics

The continuation must:

- use current registry state only from `SharedCurrentCapabilityAuthority`;
- ignore the principal-agnostic capability policy yielded beside that registry for requester/rendezvous policy purposes;
- use the separately supplied `RequesterRendezvousStartPolicySource` as the sole DP requester-aware policy source;
- invoke exactly the existing DI -> DP -> DK -> DN composition;
- preserve the exact `RequesterRendezvousStartCompositionError` without retry, fallback, flattening, replacement or fabricated success;
- release the shared-current read guard before any future response I/O.

## 6. Provenance preservation

FB must not extract only `target_device_id` and call the target-only C03e-DV convenience method, because EZ already owns the exact authenticated-session-derived `RequesterRendezvousStartIntent`.

Requester identity remains the authenticated PRW application session captured in that intent. Target identity remains the exact nominated logical `DeviceId` carried in that intent. PRWM `request_id` remains correlation only inside the retained strict request.

No IP address, port, transport endpoint, stream order or `TransportIdentity` alone may become requester or target identity.

## 7. Response-stream custody

FB does not expose raw `MeshControlStream` to unrelated Agent code. The terminal FB envelope retains the bridge-owned `PostAuthRequesterRendezvousTransaction`; a later separately gated response-materialization checkpoint may use that transaction for correlation and same-stream response custody.

FB selects and materializes no requester response schema, result code, frame, write, retry, close policy or loop-resume behavior.

## 8. Repeated-ingress barrier

The requester path remains terminal at the retained-custody continuation boundary. No second control-stream accept/read may occur while DR continuation and later response handling remain unresolved.

FB creates no task, queue, channel, fairness policy, backpressure policy, concurrent reader or speculative pre-accept.

## 9. Explicit non-goals

FB does not materialize or authorize:

- requester response mapping or write;
- candidate-publication result-codec reuse;
- provider query/select operations beyond the existing DR registration mutation;
- candidate, reachability, endpoint or relay selection;
- QUIC/TCP dialing;
- port-forward or terminal activation;
- second ingress acceptance/read;
- mixed-family loop resume;
- runtime/listener/bootstrap activation;
- Android behavior changes;
- dependency/workflow widening;
- packaging, deployment, restart, recovery or merge.

## 10. Validation contract

Closure requires exact-final-head validation. Rust validation must pass locked dependency graph, rustfmt, Clippy with warnings denied, workspace tests and workspace build. Android validation is claimed only if the workflow is actually triggered and passes on the exact final head.

## 11. Canonical closure target

Intended closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_RETAINED_CUSTODY_DR_CONTINUATION_SOURCE_MATERIALIZATION`

Intended gate marker:

`C03E_FB_REQUESTER_RENDEZVOUS_RETAINED_CUSTODY_DR_CONTINUATION_SOURCE_MATERIALIZED`
