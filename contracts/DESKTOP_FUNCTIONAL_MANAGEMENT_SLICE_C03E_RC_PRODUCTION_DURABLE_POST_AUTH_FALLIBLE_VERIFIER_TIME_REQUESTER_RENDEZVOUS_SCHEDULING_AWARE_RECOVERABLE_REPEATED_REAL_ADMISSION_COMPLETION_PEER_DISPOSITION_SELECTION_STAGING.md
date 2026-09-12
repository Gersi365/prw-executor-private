# Desktop Functional Management Slice C03e-RC — Production-Durable Post-Auth Fallible Verifier-Time Requester/Rendezvous Scheduling-Aware Recoverable Repeated Real-Admission Completion Peer Disposition Selection — Staging

**Checkpoint:** C03e-RC  
**Boundary:** `PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COMPLETION_PEER_DISPOSITION_SELECTION`  
**Status:** `SELECTION — VALIDATION PENDING`

## 1. Purpose

C03e-RC selects the next minimal source boundary after evidence-closed C03e-RB.

RB materialized one dormant owner-bearing completion envelope carrying exact fallible-verifier-time production-durable scheduling terminal custody. RB explicitly stopped before owner/peer disposition, active-map migration, poll/reap/drain adapters, repeated real-admission collection migration, producer/executor/endpoint propagation, request construction, runtime activation, merge or deployment.

RC is documentation-only. It selects only the exact peer/session-owner disposition law for the RB completion envelope and the future one-file source ceiling needed to materialize that law.

RC does not mutate Rust source.

## 2. Exact predecessor authority

C03e-RC begins only from exact evidence-closed C03e-RB:

- branch: `phase-152-c03e-rb-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-scheduling-aware-recoverable-repeated-real-admission-completion-custody-source-materialization`;
- head: `4517bb49c551060208b270db1dd494e262c5da78`;
- tree: `e39736ab07794960627ed218f002c6ddc2b0b36a`;
- selected parent source blob: `246f69cb92b66b5415bfbd7d073f911171d0bd79`;
- PR: #592, preserved draft/open/unmerged/mergeable and evidence-closed;
- canonical RB audit Drive ID: `1YPfPy8D-IjHKQ5YjHtiCImEq3YRbwS7S`;
- canonical RB audit bytes: `18839`;
- canonical RB audit SHA-256: `046bac65a25e76921e462973468cbe69cc8a4955656cb50168268bb192421a6e`.

Stable repository `main` at RC selection start remains:

- head: `7c993fa93977a0bb84e0d030874eee7fd0cae77f`;
- tree: `63b8e59ca53797fdea6b95432e16f35eaf473604`.

## 3. Fresh post-RB authority and collision finding

Fresh live checks before RC creation established:

- PR #592 remained the newest repository PR;
- RB branch remained exact head `4517bb49c551060208b270db1dd494e262c5da78`;
- `main` remained unchanged;
- canonical RB evidence remained the expected singleton;
- no `c03e-rc` branch existed;
- no `C03e-RC` PR existed;
- no canonical RC audit with the selected title existed.

No concurrent newer checkpoint existed. RC therefore branches directly from exact RB authority.

## 4. Immediate live source finding

Exact RB source path inspected:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact RB blob:

`246f69cb92b66b5415bfbd7d073f911171d0bd79`

RB adds exactly one owner-bearing fallible scheduling completion envelope:

`RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion`

with fields retained by value:

- `device_id: DeviceId`;
- `session_owner: AuthenticatedRemoteSessionRuntimeOwner`;
- `result: Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The same parent file already contains the historical scheduling-aware higher-owner peer disposition seam:

- `RecoverableRequesterAwarePeerDisposition` with `OrderlyShutdown` and `TerminalFailure`;
- `select_scheduling_terminal_acknowledgement_peer_disposition(...)`;
- `select_recoverable_requester_aware_scheduling_peer_disposition(...)`;
- `dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(...)`.

The historical scheduling disposer consumes recovered owner custody before returning authenticated identity and terminal result. It uses only requester acknowledgement success/failure for `SchedulingTerminal`; scheduling derivation success/failure does not control peer disposition.

## 5. Historical law verified from C03e-NX / C03e-NY

Historical C03e-NX selection and C03e-NY source materialization establish the scheduling-aware peer-disposition law:

1. `Cancelled` -> existing orderly-shutdown close seam;
2. pre-scheduling `Failed(...)` -> existing requester-aware terminal-failure close seam;
3. abnormal spawned/join failure -> existing requester-aware terminal-failure close seam;
4. `SchedulingTerminal` with requester ACK success -> existing orderly-shutdown close seam;
5. `SchedulingTerminal` with requester ACK failure -> existing requester-aware terminal-failure close seam;
6. scheduling derivation success/failure never controls peer disposition;
7. recovered authenticated-session owner is consumed before boundary-safe result publication;
8. scheduling terminal custody moves by value without clone/copy/reconstruction/remint/side-channel substitution.

RC selects the exact parallel law for the fallible-verifier-time scheduling stop already materialized by QX/QZ/RB.

## 6. Why owner disposition is the next minimal dependency

RB already provides the exact owner-bearing envelope required to dispose the recovered authenticated-session owner while preserving:

- authenticated `DeviceId`;
- exact `RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop`;
- exact abnormal join error.

The parent file already has:

- the disposition enum;
- the ACK-only terminal helper;
- the two consuming owner close seams;
- the historical scheduling classifier/disposer shape.

Therefore no collection migration, active-map mutation, worker-entry mutation, generic custody mutation, lifecycle mutation, visibility widening or second Rust path is required to materialize the immediate parallel disposition seam.

Collection migration remains downstream because the historical collection still owns historical scheduling completion and callback types. RC does not authorize changing that collection.

## 7. Selected future source boundary

Future source-materialization checkpoint may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact predecessor blob for that path:

`246f69cb92b66b5415bfbd7d073f911171d0bd79`

No other Rust/source path is selected.

If correctness requires any second source path, collection integration, visibility widening outside this parent, generic custody mutation, QZ worker-entry mutation, QX lifecycle mutation, Cargo/workflow mutation, Android source mutation, endpoint propagation or higher-owner propagation, STOP and return to a fresh selection audit.

## 8. Selected future fallible scheduling disposition classifier

Future source materialization may add one parallel classifier conceptually equivalent to:

`select_recoverable_requester_aware_fallible_verifier_time_scheduling_peer_disposition(...)`

Input shape must be an ordinary borrow of exact terminal result custody:

`&Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

Output must reuse existing:

`RecoverableRequesterAwarePeerDisposition`.

No new disposition enum is selected.

## 9. Selected classifier law

The classifier must preserve exact terminal provenance and select disposition as follows:

1. `Ok(RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop::Cancelled)` -> `OrderlyShutdown`;
2. `Ok(RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop::Failed(_))` -> `TerminalFailure`;
3. `Err(RemoteSessionSpawnedWorkerJoinError::...)` -> `TerminalFailure`;
4. `Ok(RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop::SchedulingTerminal(outcome))` -> reuse existing `select_scheduling_terminal_acknowledgement_peer_disposition(outcome.acknowledgement_result().is_ok())`;
5. scheduling derivation success/failure inside the scheduling terminal carrier must not influence disposition;
6. nested fallible verifier-time, ingress, requester-response and scheduling terminal errors remain unflattened and unmodified;
7. the classifier borrows terminal result custody and does not consume, clone, copy, reconstruct, stringify, remint, replay, default, cache or divert it.

## 10. Selected future completion disposer

Future source materialization may add one parallel disposer conceptually equivalent to:

`dispose_recoverable_repeated_real_admission_requester_aware_fallible_verifier_time_scheduling_worker_completion(...)`

Input must be exact RB envelope by value:

`RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion`.

Return shape must be exactly:

`(DeviceId, Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>)`.

No authenticated-session owner may cross the disposer boundary.

## 11. Selected disposer ownership law

Future materialization must:

1. consume the exact RB completion envelope by value;
2. split it only through existing `into_parts()`;
3. retain `device_id`, exact recovered `session_owner`, and exact terminal `result` locally;
4. classify disposition by borrowing `&result` through the selected parallel classifier;
5. on `OrderlyShutdown`, consume exact owner through existing `session_owner.close_for_orderly_shutdown()`;
6. on `TerminalFailure`, consume exact owner through existing `session_owner.close_for_requester_aware_terminal_failure()`;
7. perform exactly one owner disposition;
8. return only `device_id` and unchanged exact `result` after owner disposition;
9. return no owner, peer, peer-reuse token, requester cleanup authority, retry token, worker restart token, scheduling sender/channel or request-construction authority;
10. perform no requester-record cleanup, reachability continuation, dialing, reconnect, retry, remint, replay or replacement worker creation.

## 12. Existing helpers and visibility must be reused

Future materialization must reuse unchanged where possible:

- `RecoverableRequesterAwarePeerDisposition`;
- `select_scheduling_terminal_acknowledgement_peer_disposition(...)`;
- `AuthenticatedRemoteSessionRuntimeOwner::close_for_orderly_shutdown()`;
- `AuthenticatedRemoteSessionRuntimeOwner::close_for_requester_aware_terminal_failure()`;
- RB completion `into_parts()`;
- existing terminal carrier `acknowledgement_result()` accessor.

No visibility widening is selected.

No new close code, peer-close variant or owner-disposition enum is selected.

## 13. Same-file tests permitted by the selected ceiling

Because future source ceiling is exactly one parent Rust file and that file already owns the relevant unit tests, future materialization may add same-file tests necessary to prove the selected parallel law.

Permitted test coverage includes:

- fallible scheduling `Cancelled` selects orderly shutdown;
- fallible scheduling `Failed(...)` selects terminal failure;
- abnormal join selects terminal failure;
- disposer signature consumes exact RB completion and returns exact `(DeviceId, Result<...>)` shape;
- terminal ACK success/failure behavior remains bound to existing ACK-only helper.

Tests must not require mutation outside the selected parent path.

If a test requires a second production path, visibility widening or construction of unrelated runtime state, STOP and return to selection.

## 14. Byte-stable guard paths for future materialization

Future source materialization must keep byte-stable:

1. QZ fallible scheduling worker entry:
   `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`
   - blob `db1398a64e51898df072bb49511dd6f0c99dbeb7`;
2. QX fallible scheduling lifecycle:
   `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `39f91dc8510df49620cf3336e98656532b46c158`;
3. generic persistent custody:
   `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`
   - blob `4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`;
4. production-durable repeated real-admission collection:
   `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`
   - blob `08dc8160d72c41bc9a211c2c2e15f0e65d067b74`.

Historical completion envelopes and historical scheduling disposer must remain semantically unchanged.

## 15. Explicitly deferred later gates

RC does not select or materialize:

- source materialization itself;
- active fallible scheduling worker map;
- fallible scheduling publish adapter;
- fallible scheduling poll/reap/drain adapters;
- in-flight fallible scheduling admission drain;
- repeated real-admission collection migration;
- collection completion callback migration;
- replacement of historical scheduling collection types;
- collection invocation of QZ fallible scheduling worker entry;
- migration of historical `T: FnMut() -> u64 + Send + 'static` collection bounds;
- cooperative scheduling producer propagation;
- executor endpoint-lifecycle propagation;
- higher endpoint-owner propagation;
- expected-device admission request construction or send;
- target admission `SessionId` generation;
- authentication/request ID allocation;
- new timing acquisition outside the existing fallible verifier-time source;
- listener/readiness/network/runtime activation;
- task spawn/join beyond existing historical seams;
- new queue/channel/sender construction;
- retry/reconnect/respawn/restart;
- fallback/default/cached verifier time;
- requester-record cleanup;
- candidate/reachability continuation;
- target dial;
- auth/trust/RBAC mutation;
- database/schema/control-plane mutation;
- Cargo/lockfile/workflow mutation;
- Android source mutation;
- packaging/service/systemd mutation;
- repository configuration mutation;
- merge, deployment, ready-for-review transition or PR close;
- branch deletion, reset, rebase, squash, force update or history rewrite;
- destructive evidence cleanup.

## 16. Future source checkpoint closure law

A future source-materialization checkpoint is valid only if all of the following hold:

- exact predecessor is final evidence-closed RC head;
- direct RC -> source checkpoint merge base is exact RC;
- changed source paths remain exactly within the one selected parent file;
- net diff contains only the selected parallel classifier/disposer and same-file proof required by that law;
- all guard blobs remain byte-stable;
- no collection/runtime propagation occurs;
- all PASS claims bind only to exact final source head;
- `SKIPPED` remains distinct from PASS;
- immutable Drive evidence is frozen before publication, published once, raw-read back, singleton-verified and revision-verified;
- PR remains draft/open/unmerged;
- no next checkpoint is selected inside source closure.

## 17. RC validation and evidence law

RC itself is docs-only.

Closure requires:

1. exact RB -> RC topology: ahead only, merge base exact RB;
2. exactly one added contract path;
3. zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/packaging/deployment/repository-config changes;
4. exact-final-head CI claims only for workflows actually registered on RC head;
5. immutable canonical Drive audit publication/readback/singleton/revision verification;
6. read-after-write verification of RC branch, PR, `main`, and newest-PR chronology;
7. PR remains draft/open/unmerged;
8. STOP before source materialization.

## 18. Selected future boundary

The next separately gated source boundary selected by RC is:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COMPLETION_PEER_DISPOSITION_SOURCE_MATERIALIZATION`

A likely future token is `C03e-RD` only after a fresh exact-authority/concurrency/collision audit. RC closure does not create or assume RD.

## 19. STOP boundary

C03e-RC stops after docs-only selection validation and immutable evidence closure.

Do not materialize the selected peer disposition inside RC.  
Do not mutate the RB parent Rust file inside RC.  
Do not migrate active-map/poll/reap/drain helpers.  
Do not migrate repeated real-admission collection.  
Do not propagate to producer/executor/endpoint/higher-owner surfaces.  
Do not construct or send a new admission request.  
Do not allocate new IDs or timing.  
Do not activate listener/network/runtime behavior.  
Do not merge, deploy, mark ready for review, delete branches or rewrite history.  
Do not destructively clean evidence.

**Selection status:** `SELECTION — VALIDATION PENDING`
