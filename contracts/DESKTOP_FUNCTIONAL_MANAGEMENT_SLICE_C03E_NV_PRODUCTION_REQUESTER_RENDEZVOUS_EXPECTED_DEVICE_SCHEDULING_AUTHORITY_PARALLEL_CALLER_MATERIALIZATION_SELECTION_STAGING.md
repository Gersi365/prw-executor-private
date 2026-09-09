# C03e-NV — Production requester/rendezvous expected-device scheduling-authority parallel-caller materialization selection

Status: **STAGING — SELECTION ONLY — SOURCE MATERIALIZATION BLOCKED**

## 1. Gate

`C03E_NV_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_PARALLEL_CALLER_MATERIALIZATION_SELECTED`

Closure token reserved for validated evidence closure:

`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_PARALLEL_CALLER_MATERIALIZATION_SELECTION`

## 2. Authoritative closed predecessor

This checkpoint is rooted exactly at the last closed authority boundary, C03e-NT.

Exact C03e-NT head:

`0e7bc4b1d5f1327b139e51bdd8bc379e047e4aa8`

Exact C03e-NT tree:

`275189a7ece2bdf4e950acbfa8c90399caeee845`

Exact C03e-NT contract blob:

`dac07cbba8cf6988006694af4ebd7bc8ffccd0c5`

C03e-NT selected the production-durable requester worker as the first scheduling-authority derivation caller and fixed the following semantic law:

- requester/rendezvous DR registration runs first;
- DR failure skips scheduling derivation;
- DR success is followed by scheduling derivation before requester acknowledgement framing/I/O;
- scheduling derivation and requester acknowledgement disposition are retained as orthogonal terminal result channels;
- a non-Clone scheduling grant survives acknowledgement failure;
- derivation failure does not rewrite successful requester/rendezvous registration or the requester acknowledgement;
- once a scheduling result exists, cancellation does not override it and no next requester ingress cycle begins;
- historical requester lifecycle variants remain unchanged;
- request construction/send and runtime activation remain separately gated.

## 3. C03e-NU blocker evidence is informative, not authoritative predecessor state

C03e-NU attempted the C03e-NT one-file source materialization by changing the return shape of:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

Blocked NU candidate head:

`937230da5e1956bb5fbf4aa23c7d2de3d12bad39`

Blocked NU candidate tree:

`af4c5f80603087a2f96b9c20a770468ac5216b7b`

Blocked target blob:

`bc6307e5debdd3adbd3e0201f3a8ccc0c1d9d727`

Blocked NU PR:

`#509`

NU is explicitly:

`SOURCE MATERIALIZATION — BLOCKED — SECOND-PATH REQUIRED — EVIDENCE_RECORDED — NOT CLOSED`

NU must not be used as the base authority for this checkpoint.

Durable NU blocker evidence:

`C03E_NU_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_DERIVATION_CALLER_RESULT_CUSTODY_SOURCE_MATERIALIZATION_BLOCKER_AUDIT_2026-09-09.txt`

Drive ID:

`1Cham0QupPzFIn6PJPqWkDq0tJBoN3eYO`

Exact bytes:

`15352`

SHA-256:

`7145d945c7a43c569bbe5f6055aa880883060af47852f61fd8327ac1f5118245`

## 4. Compiler-proven blocker

Exact-head Rust #1679 on blocked NU candidate head `937230da5e1956bb5fbf4aa23c7d2de3d12bad39` passed locked graph and formatting, then failed during Clippy/compile with two `E0308` ownership/type-custody mismatches.

### 4.1 Recoverable spawned completion mismatch

Outside the NT one-file ceiling:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Existing production-durable recoverable spawned driver returns the historical:

`RecoverableSpawnedRequesterRendezvousWorkerCompletion`

whose `result` is fixed to:

`Result<RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

The blocked scheduling-aware worker instead returned a production-specific non-Copy scheduling result type.

### 4.2 Persistent worker-entry mismatch

Outside the NT one-file ceiling:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

The production-durable spawn helper currently returns:

`RecoverableRequesterAwareWorkerEntry`

which is specialized to the historical worker stop.

The blocked scheduling-aware worker instead yields a `JoinHandle` whose terminal result carries the non-Clone scheduling outcome.

### 4.3 Transitive production-durable repeated/endpoint owner

Fresh C03e-NT source audit also shows a third higher-owner path beyond the two direct compiler diagnostics:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

That path currently:

- uses the historical active-worker specialization;
- publishes `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`;
- consumes completion through the existing historical FW peer disposer;
- exposes only historical stop/join result to the endpoint completion callback.

Therefore changing the existing production-durable worker return type is not a local one-file edit. It reaches the higher-owner graph.

## 5. Generic persistent primitive finding

Fresh exact C03e-NT source audit confirms:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`

already defines generic custody primitives:

`RecoverablePersistentWorkerEntry<O, T>`

and:

`RecoverablePersistentWorkerCompletion<K, O, T>`

The generic reap/drain/cancellation machinery is already generic over terminal result type `T`.

The current historical aliases bind that generic primitive to:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

but the primitive itself does not require a source change to represent a future production-specific scheduling result.

Selection consequence:

**the generic persistent primitive file is not selected for modification by the immediate source successor and should remain byte/semantic-equivalent.**

## 6. Historical stop compatibility finding

Exact C03e-NT source defines:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

as:

`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`

with historical variants:

- `Cancelled`;
- `Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError)`.

Historical higher-owner completion accessors return this stop/result by value and rely on the Copy-compatible shape.

The scheduling terminal outcome must own a non-Clone `ExpectedDeviceSchedulingAuthorityGrant` on success.

Therefore adding a scheduling-grant-bearing variant to the historical stop would either:

- remove or invalidate existing Copy/Clone semantics; or
- require cloning/copying/reconstructing the scheduling grant.

Neither is selected.

The historical stop type remains unchanged.

## 7. Alternatives considered

### Alternative A — immediate atomic four-file higher-owner propagation

Potential future paths:

1. requester retained-custody DR continuation / worker;
2. recoverable spawned requester worker custody;
3. repeated real-admission persistent integration;
4. production-durable repeated/endpoint collection.

This would permit one source checkpoint to carry the new non-Clone result through the entire currently materialized dormant higher-owner graph.

Rejected for the immediate successor because source audit exposes an additional unresolved semantic decision at the endpoint boundary: the historical endpoint path disposes the recovered authenticated-session owner before the boundary-safe callback, while C03e-NT selected no new scheduling-terminal peer disposition or close-code law.

A type-only four-file rewrite would therefore force an unselected peer-lifecycle decision merely to make the graph compile.

### Alternative B — widen historical worker stop/completion types

Rejected.

Historical Copy/Clone and by-value result semantics must remain unchanged.

### Alternative C — hide scheduling result behind `Arc`, global/static state, side channel, or reconstructed token

Rejected.

The grant is one-shot, non-Clone, and must move by value exactly once.

### Alternative D — drop scheduling result at one compatibility boundary

Rejected.

C03e-NT explicitly requires the grant or derivation error to survive acknowledgement disposition and move upward.

### Alternative E — parallel scheduling-aware production-durable worker, existing worker unchanged

Selected.

This preserves all C03e-NT ordering and custody semantics in one dormant source surface without changing the already-existing production-durable worker signature or forcing higher-owner propagation/peer-disposition decisions into the same checkpoint.

## 8. Selected revised caller materialization seam

The immediate source successor must **not** replace the return type or behavior of:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

That existing function remains byte/semantic-equivalent in the immediate source successor.

Instead, one new sibling production-durable scheduling-aware requester worker is selected in the same module.

Semantic future identity:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`

The exact final Rust identifier may be refined only for local naming consistency, but it must be a distinct sibling surface rather than a signature replacement of the existing production-durable worker.

## 9. Selected source ceiling for the immediate successor

Exactly one Rust source path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

No second path is authorized by the immediate source successor.

The successor must STOP if implementation requires any other source file.

## 10. Selected one-file materialized shapes

Within the one selected path, the future source checkpoint may materialize only:

1. one non-`Copy`, non-`Clone` terminal caller-result carrier preserving:
   - scheduling derivation `Result<ExpectedDeviceSchedulingAuthorityGrant, ExpectedDeviceSchedulingAuthorityDerivationError>` by value;
   - requester acknowledgement `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>` by value;
2. one production-specific worker stop able to represent:
   - cancellation before any scheduling result exists;
   - existing historical ingress/requester-response failure before a scheduling result exists;
   - one scheduling-terminal result carrier after successful requester/rendezvous DR;
3. one new sibling scheduling-aware production-durable worker implementing the exact C03e-NT order;
4. focused same-file tests proving result orthogonality, grant non-duplication and historical-function non-regression.

No higher-owner completion or persistent collection type is materialized in this source checkpoint.

## 11. Exact scheduling-aware worker ordering preserved from NT

The new sibling worker must implement:

1. run the exact existing production-durable post-auth requester ingress;
2. receive one requester/rendezvous handoff;
3. preserve requester `SessionId` and target `DeviceId` as non-authorizing selectors before consuming the start intent;
4. run exact existing requester/rendezvous DR continuation once;
5. if DR fails:
   - do not call scheduling derivation;
   - send the existing rejected requester acknowledgement exactly as today;
   - after successful rejected acknowledgement, preserve historical cancellation/next-ingress behavior;
6. if DR succeeds:
   - call exact existing `derive_expected_device_scheduling_authority(...)` once;
   - retain success grant or typed derivation failure by value;
   - then attempt the existing requester acknowledgement exactly once from the unchanged DR result only;
   - retain acknowledgement disposition independently;
   - return scheduling-terminal custody before any cancellation poll or next ingress cycle.

No response transport result may become scheduling authority.

## 12. Non-Clone grant law remains exact

If derivation succeeds, the grant:

- is moved exactly once into the terminal caller-result carrier;
- is not cloned;
- is not copied;
- is not reconstructed;
- is not reminted;
- is not hidden behind `Arc` or global/static storage;
- is not dropped because acknowledgement framing/I/O fails;
- is not replaced by cancellation after it exists.

## 13. Derivation failure law remains exact

If requester/rendezvous DR succeeds but scheduling derivation fails:

- requester registration remains committed;
- provider state remains unchanged;
- requester acknowledgement remains the existing accepted acknowledgement;
- exact derivation error is retained internally;
- no peer-visible scheduling-specific error frame is introduced;
- worker terminates with the scheduling-terminal result after acknowledgement disposition is known.

## 14. Acknowledgement failure law remains exact

If a scheduling result already exists and requester acknowledgement framing or response I/O fails:

- scheduling result is retained unchanged;
- grant is not dropped;
- derivation error is not replaced;
- acknowledgement error is retained independently;
- no resend/retry/replacement stream is created.

## 15. Historical function preservation

The immediate source successor must preserve byte/semantic behavior of:

- `run_requester_rendezvous_post_terminal_response_serial_lifecycle(...)`;
- `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`;
- `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`.

Only the new sibling scheduling-aware surface may contain the scheduling-derivation caller behavior.

Existing shared historical worker stop and error types remain unchanged.

## 16. Higher-owner propagation remains separately gated

Fresh source evidence proves that a later migration from the existing production-durable worker to the new scheduling-aware sibling will require a separately selected higher-owner result-custody propagation seam.

The later gate must examine at least:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`.

The generic persistent primitive path is not currently proven necessary for modification.

## 17. Later higher-owner gate must decide peer/session-owner disposition

The existing production-durable endpoint lifecycle currently consumes recovered worker completion through the historical FW disposer before invoking its callback.

A scheduling-terminal result introduces a new semantic state not represented by historical:

- cancellation;
- terminal ingress/requester-response failure;
- abnormal task join.

C03e-NT selected no new scheduling-specific close code or peer-visible scheduling failure mapping.

Therefore the immediate source successor must not invent how `SchedulingTerminal` disposes the authenticated-session owner.

A later documentation gate must explicitly select whether and how owner disposition occurs before any production-specific endpoint callback.

That selection must remain independent from scheduling derivation success/failure and must not silently turn scheduling derivation state into peer authorization or a peer-visible scheduling response.

## 18. No request construction or send

This selection does not authorize:

- `RemoteSessionExpectedDeviceAdmissionRequest` construction from the scheduling grant;
- expected-device sender/channel creation or send;
- target admission `SessionId` selection/generation;
- authentication PRWM request-ID selection/generation;
- admission timing selection;
- dispatcher/listener/bootstrap caller migration;
- runtime activation;
- endpoint reachability/dial selection;
- retry/remint/replay;
- persistence/timer;
- deployment;
- merge.

## 19. No authority widening

The new sibling worker does not create new authority.

It may only invoke the existing C03e-NS scheduling derivation method after the existing C03e-NT requester/rendezvous DR-success condition.

The scheduling derivation method remains the authority boundary for:

- provider relationship witness;
- exact requester-session/target match;
- fresh registry currentness;
- fresh requester policy reauthorization;
- terminal one-shot consumption commit;
- non-Clone scheduling grant construction.

The caller carrier is custody only.

## 20. Focused validation requirements for the immediate source successor

The source successor should prove at minimum:

1. historical three requester lifecycle surfaces retain unchanged behavior/signatures;
2. the new sibling does not derive scheduling authority on DR failure;
3. DR success invokes derivation before acknowledgement composition/I/O;
4. grant + ACK success preserves both results;
5. grant + ACK failure preserves grant and exact ACK error;
6. derivation failure + ACK success preserves both facts;
7. derivation failure + ACK failure preserves both facts;
8. cancellation cannot override a scheduling result after derivation;
9. no next ingress cycle occurs after a scheduling result exists;
10. no request/channel/runtime activation occurs because the sibling exists.

## 21. Source-diff hard ceiling

The immediate materialization checkpoint is one-file and dormant.

Permitted path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Not permitted:

- `shared_requester_rendezvous_authority.rs` changes;
- `remote_session_capability_runtime` export changes;
- recoverable spawned higher-owner changes;
- persistent integration changes;
- production-durable collection/endpoint changes;
- generic persistent primitive changes;
- `linux_bootstrap.rs` changes;
- Cargo manifest or lockfile changes;
- workflow changes;
- Android source changes;
- packaging/service/database/control-plane changes;
- runtime activation;
- deployment.

If any second source path becomes necessary, STOP and return to a fresh documentation selection.

## 22. Selection

`PARALLEL_PRODUCTION_DURABLE_SCHEDULING_AWARE_REQUESTER_WORKER_SELECTED / EXISTING_PRODUCTION_DURABLE_WORKER_SIGNATURE_AND_HIGHER_OWNER_GRAPH_REMAIN_UNCHANGED / ONE_FILE_DORMANT_CALLER_MATERIALIZATION_RESELECTED / FUTURE_HIGHER_OWNER_PROPAGATION_REQUIRES_THREE_ADDITIONAL_EXISTING_SOURCE_PATHS / GENERIC_PERSISTENT_PRIMITIVES_REMAIN_UNCHANGED / NON_CLONE_SCHEDULING_RESULT_MOVES_BY_VALUE_WITHOUT_ARC_GLOBAL_OR_RECONSTRUCTION / PEER_AND_SESSION_OWNER_DISPOSITION_REMAINS_SEPARATELY_GATED / REQUEST_CONSTRUCTION_AND_SEND_REMAIN_BLOCKED / SOURCE_MATERIALIZATION_BLOCKED`

## 23. Next separately gated boundary after NV closure

If this selection closes with exact evidence, the next source checkpoint may be:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_PARALLEL_CALLER_SOURCE_MATERIALIZATION`

Likely successor token:

`C03e-NW`

Likely one-file source ceiling:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

After that one-file source checkpoint closes, higher-owner propagation must still return to a separately gated documentation selection before any additional source path is changed.

## 24. Explicit non-actions

This checkpoint is documentation-only.

No Rust source is changed.
No blocked NU source is adopted as authority.
No NU branch is rebased, force-updated, merged or deleted.
No higher-owner source path is changed.
No historical stop type is widened.
No scheduling grant is cloned/copied/reconstructed.
No expected-device request is constructed or sent.
No channel, SessionId, request-ID or timing source is added.
No runtime/listener/bootstrap path is activated.
No deployment occurs.
No PR is merged.

`STOP`
