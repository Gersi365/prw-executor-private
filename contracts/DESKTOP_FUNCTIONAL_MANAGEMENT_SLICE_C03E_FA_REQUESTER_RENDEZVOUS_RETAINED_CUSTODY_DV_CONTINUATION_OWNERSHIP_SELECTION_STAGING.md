# Phase 152 C03e-FA — Requester/Rendezvous Retained-Custody DV Continuation Ownership Selection (Staging)

Status: SELECTION_STAGING

## 1. Purpose

C03e-FA selects only the ownership and authority-composition boundary that may consume the C03e-EZ requester/rendezvous response-stream custody handoff and run the already-materialized requester/rendezvous start authority composition while preserving the exact requester transaction for a separately gated response mapping step.

This checkpoint is selection-only. It adds no Rust source, does not invoke C03e-DV or C03e-DR at runtime, does not construct or write requester/rendezvous responses, does not resume the repeated ingress loop, does not select candidates or dial traffic, and does not activate a listener, task, bootstrap path, deployment, restart, recovery path, or merge.

## 2. Exact predecessor

The canonical predecessor is C03e-EZ:

- branch: `phase-152-c03e-ez-requester-rendezvous-response-stream-custody-source-materialization-staging`
- head: `1b434f54e2d6e34b9329e38838307cebf6a31d79`
- tree: `10c635cb11cbdbcb09c8294e202d0a77eac83096`

C03e-FA must remain a docs-only descendant of that exact head.

## 3. Fresh source audit

The exact C03e-EZ source boundaries audited before this selection are:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - blob `66f8435314d1ede2474292d811276e16919ddcad`
  - owns the crate-private `RequesterRendezvousResponseStreamCustodyHandoff` typing introduced by C03e-EZ.
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
  - blob `083bf83fd1827f6175c9eb62ff93b40147fa9271`
  - contains the already-materialized C03e-DV current-authority caller composition.
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
  - blob `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`
  - contains the already-materialized C03e-DR fail-closed start composition.
- `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
  - blob `50356b47d3c5304b67edd424e9286beb028ace16`
  - owns the bounded current-registry/current-capability-policy read operation used by C03e-DV.
- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - blob `d5562a9587bdbde7d05e38fdd704d42f9d20f3c8`
  - owns the strict requester request plus exact same `MeshControlStream` inside `PostAuthRequesterRendezvousTransaction`.

C03e-FA changes none of those blobs.

## 4. Existing C03e-EZ handoff

C03e-EZ already retains one crate-private requester/rendezvous handoff containing:

1. the exact strict C03e-ET requester transaction, which owns:
   - the decoded `RequesterRendezvousTargetWireRequest`; and
   - the exact same already-accepted `MeshControlStream` by value; and
2. the exact `RequesterRendezvousStartIntent` derived by the existing EO -> EJ authenticated-session path.

The outer PRWM `request_id` remains only inside the retained strict request and remains correlation only.

The handoff is a barrier: C03e-EX does not accept another stream after producing it.

## 5. Existing C03e-DV and C03e-DR authority behavior

C03e-DV already exposes the Agent-owned asynchronous method:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

Its current source shape accepts a caller-supplied `target_device_id`, reconstructs one `RequesterRendezvousStartIntent` from the authenticated session retained by the owner, then enters one `SharedCurrentCapabilityAuthority::with_current_authority(...)` read and invokes C03e-DR.

C03e-DR is:

`validate_authorize_and_register_requester_rendezvous_start(...)`

Its exact current terminal shape is:

`Result<(), RequesterRendezvousStartCompositionError>`

Its execution order is fixed and fail-closed:

1. DI validates the raw start intent against current registry state.
2. DP resolves requester-aware policy using only the exact DI-held authenticated requester session.
3. DK consumes the unchanged DI carrier and authorizes the dedicated requester/rendezvous-start capability.
4. DN consumes the exact DK carrier and performs the existing private requester/rendezvous registration mutation.

The four failure classes remain distinct:

- `RegistryValidation(...)`
- `PolicySource(...)`
- `PolicyAuthorization(...)`
- `Registration(...)`

Success carries no endpoint, candidate, route, transport, relay, or response payload. It is exactly `()`.

## 6. Provenance mismatch that C03e-FA must resolve

The pre-existing C03e-DV convenience method accepts only `target_device_id` and then reconstructs a new `RequesterRendezvousStartIntent` from its own authenticated-session owner.

C03e-EZ, however, already carries the exact `RequesterRendezvousStartIntent` produced for the requester transaction before the C03e-EX handoff barrier.

C03e-FA therefore rejects a continuation design that would:

1. extract only the target `DeviceId` from the EZ handoff;
2. discard the already-derived EZ start intent; and
3. call the current target-only C03e-DV convenience method to construct a second start intent.

That path would be semantically redundant and would weaken the exact ownership/provenance chain established by EO -> EJ -> EZ.

C03e-FA instead selects preservation of the exact existing EZ start-intent object across the continuation boundary.

## 7. Selected continuation owner

The requester/rendezvous continuation remains Agent-owned.

The selected future source seam belongs under the existing authenticated remote-session runtime ownership line because that line already owns:

- the authenticated requester session provenance that produced the EZ start intent;
- the EV/EX requester handoff barrier;
- access to the shared-current authority owner used by C03e-DV; and
- the crate-private requester/rendezvous runtime owner and requester-aware policy source needed by C03e-DR.

The bridge remains owner of raw stream/frame I/O and does not become owner of registry, requester policy, or provider mutation authority.

No new authority owner is selected.

## 8. Selected authority invocation pattern

A future source-materialization successor must consume the exact EZ handoff and run the existing DR composition under the same current-authority read pattern already established by C03e-DV.

Semantically, the selected authority operation is:

`EZ start_intent -> SharedCurrentCapabilityAuthority::with_current_authority(...) -> existing DR composition`

The operation must preserve all current C03e-DV/C03e-DR authority rules:

- current registry state comes only from `SharedCurrentCapabilityAuthority`;
- the principal-agnostic capability policy stored beside that registry is not requester/rendezvous policy and remains ignored by this requester-specific path;
- the separately supplied `RequesterRendezvousStartPolicySource` remains the sole DP requester-aware policy source;
- C03e-DR remains the only selected DI -> DP -> DK -> DN composition;
- the provider remains private behind `CandidatePublicationRequesterRendezvousRuntimeOwner`;
- no direct provider reference or second registration path is introduced;
- no retry, fallback evaluator, replacement intent, or fabricated success is introduced.

The current shared-authority read guard must continue to span only the synchronous DR call. Network response I/O must not execute while that read guard is held.

## 9. Selected retained-custody rule across DR

The strict requester transaction must survive the authority operation unchanged on both success and failure.

The future source seam therefore must not model continuation as a plain:

`Result<PostAuthRequesterRendezvousTransaction, RequesterRendezvousStartCompositionError>`

unless the error branch itself also retains the exact transaction.

The selected semantic result is instead one terminal custody envelope containing:

1. the exact same `PostAuthRequesterRendezvousTransaction`; and
2. the exact terminal C03e-DR result:
   - success `()`; or
   - the unchanged `RequesterRendezvousStartCompositionError`.

Exact Rust type names are intentionally deferred to source materialization.

This requirement ensures that registry, policy, authorization, or registration failure does not destroy the same-stream custody needed by a future correlated requester error response.

## 10. Request correlation remains outside authority

The outer requester wire `request_id` remains correlation only.

C03e-FA does not move `request_id` into:

- requester identity;
- target identity;
- session identity;
- transport identity;
- registry keys;
- policy principal selection;
- provider identity;
- authorization provenance; or
- registration authority.

The future response mapper may read the request ID from the retained strict requester transaction after C03e-DR finishes, but the authority operation itself must not use that ID.

## 11. Logical target and requester identity

The logical target remains exactly the `DeviceId` already carried in the EZ-derived `RequesterRendezvousStartIntent`.

Requester identity remains exactly the authenticated PRW application session already carried in that same start intent.

Neither identity may be inferred or widened from:

- IP address;
- port;
- transport endpoint;
- `TransportIdentity` alone;
- PRWM `request_id`;
- stream number/order; or
- lower transport connection metadata.

C03e-FA preserves the canonical identity model:

`authenticated PRW session identity -> current registry validation -> requester-aware policy -> provider mutation`

## 12. Success semantics deliberately remain narrow

C03e-DR success is only confirmation that the exact requester/rendezvous start passed current DI/DP/DK authority and was accepted by the current DN provider registration mutation.

C03e-FA does not reinterpret `Ok(())` as proof of:

- a selected endpoint;
- candidate availability;
- reachability freshness;
- successful peer rendezvous;
- successful relay selection;
- transport establishment;
- successful dialing;
- target online state; or
- requester response delivery.

No such claims are selected.

## 13. Failure semantics remain exact

The future continuation seam must preserve the exact `RequesterRendezvousStartCompositionError` without:

- flattening all failures into one generic class;
- retrying automatically;
- falling back to another registry snapshot;
- selecting a default policy;
- replacing the requester session;
- replacing the target;
- repeating provider registration; or
- converting failure into success.

A later response-mapping checkpoint may select a bounded wire projection for these classes, but C03e-FA does not select that mapping.

## 14. Response-stream custody after authority completion

After the DR call returns, the exact same bridge-owned requester transaction remains the sole response-stream custody object.

A later response mapper may consume that transaction to obtain:

- the strict request/correlation metadata; and
- the exact same stream for one bounded response write.

C03e-FA does not expose raw stream access to unrelated Agent code and does not select a public `MeshControlStream` surface.

Any future bridge response adapter should preserve bridge ownership of raw stream I/O.

## 15. Repeated ingress barrier remains active

The C03e-EX requester handoff remains a terminal barrier for the repeated ingress loop.

C03e-FA does not allow the same authenticated session worker to accept another post-auth control stream while:

- DR authority continuation is unresolved;
- requester response mapping is unresolved;
- requester response transmission is unresolved; or
- requester continuation terminal ownership has not been separately selected.

No speculative pre-accept, second reader, concurrent family queue, or loop resume is selected.

## 16. No requester response protocol selected

C03e-FA does not select:

- success response magic;
- error response magic;
- response payload schema;
- error-code mapping;
- response version;
- response flags;
- candidate list encoding;
- endpoint encoding;
- retry advice;
- replay token;
- idempotency token;
- response deadline;
- stream close behavior; or
- post-response loop-resume behavior.

The existing candidate-publication result codec remains unrelated and must not be reused as requester/rendezvous response semantics without a separate explicit checkpoint.

## 17. No candidate or dialing behavior

C03e-FA does not select or activate:

- target candidate lookup;
- provider result lookup beyond the existing start registration mutation;
- endpoint resolution;
- route choice;
- relay choice;
- current reachability selection;
- QUIC dialing;
- TCP dialing;
- port forwarding;
- terminal transport;
- retry/reconnect; or
- network activation.

Dynamic IP/port values remain transient reachability only and never become logical identity.

## 18. Source-materialization successor boundary

A later source-materialization successor may add only the narrow isolated Agent continuation seam selected here.

That successor should:

1. consume one `RequesterRendezvousResponseStreamCustodyHandoff` by value;
2. retain the exact bridge requester transaction without clone/replacement;
3. consume the exact EZ `RequesterRendezvousStartIntent` rather than reconstructing it from a target-only value;
4. enter one current-authority read using the existing `SharedCurrentCapabilityAuthority` operation;
5. invoke exactly the existing C03e-DR composition once;
6. release the current-authority read before any future response I/O;
7. return the exact requester transaction plus exact DR terminal result; and
8. stop before requester response mapping/write and before repeated-loop resume.

The source successor should avoid modifying the existing C03e-DV target-only convenience method unless exact source constraints force a small compatibility-preserving refactor. A sibling/isolated seam is preferred so the already-closed DV contract remains stable.

## 19. Explicit non-goals

C03e-FA does not materialize or authorize:

- runtime invocation of C03e-DV/C03e-DR from EZ/EX;
- any new provider operation;
- any provider query/select API;
- requester response construction;
- requester response write;
- response retry/replay/idempotency;
- candidate publication result codec reuse;
- candidate selection;
- reachability selection;
- endpoint or relay selection;
- dialing;
- second stream acceptance/read;
- mixed-family loop resume;
- task/channel/queue creation;
- fairness/backpressure policy;
- peer close policy;
- listener/bootstrap activation;
- Android behavior changes;
- dependency or workflow changes;
- packaging/deployment/restart/recovery;
- merge.

## 20. Security invariants

The following remain mandatory:

- PID/UID/GID never fabricate PRW logical identity.
- PRWM `request_id` never becomes logical identity or authorization principal.
- IP/port never become device identity.
- `TransportIdentity` remains transport evidence, not requester identity.
- target `DeviceId` remains requester nomination until DI/DP/DK pass.
- requester identity comes only from the authenticated PRW session already captured by EZ.
- provider mutation occurs only after current-registry validation and dedicated requester-aware policy authorization.
- no request-selected executable/argv/env/cwd/root or privilege expansion is introduced.
- no public/LAN bind, route, firewall, TUN/TAP, DNS or arbitrary socket-option expansion is introduced.

## 21. Selection conclusion

C03e-FA selects one Agent-owned retained-custody continuation law:

`EZ exact handoff`
`-> retain exact requester transaction`
`-> consume exact EZ start intent`
`-> existing shared-current authority read`
`-> existing DR (DI -> DP -> DK -> DN) exactly once`
`-> release authority read`
`-> retain exact requester transaction + exact DR terminal result`
`-> stop for separately gated requester response mapping`

The existing target-only C03e-DV convenience method is not the selected direct caller from EZ because it would reconstruct a second start-intent object. Its established current-authority/DR ownership pattern is preserved and reused.

No runtime behavior is activated by this selection.

## 22. Canonical closure target

When exact-head validation and durable evidence complete, the intended closure marker is:

`CLOSED_REQUESTER_RENDEZVOUS_RETAINED_CUSTODY_DV_CONTINUATION_OWNERSHIP_SELECTION`

and the intended gate marker is:

`C03E_FA_REQUESTER_RENDEZVOUS_RETAINED_CUSTODY_DV_CONTINUATION_OWNERSHIP_SELECTED`
