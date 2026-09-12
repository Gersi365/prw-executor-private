# C03e-RA — Production-Durable Post-Auth Fallible Verifier-Time Requester/Rendezvous Scheduling-Aware Recoverable Repeated Real-Admission Completion Custody Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COMPLETION_CUSTODY_SELECTION`

Selected future boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COMPLETION_CUSTODY_SOURCE_MATERIALIZATION`

## 1. Exact predecessor

This selection begins only from evidence-closed C03e-QZ.

Exact QZ authority:

- branch `phase-152-c03e-qz-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-scheduling-aware-recoverable-persistent-worker-entry-source-materialization`;
- head `8d7bd3b57441106d471f32c19bcd778025094a67`;
- tree `6774ebb121a6f3156d567bd2adf2982a8298d547`;
- selected integration source blob `db1398a64e51898df072bb49511dd6f0c99dbeb7`;
- recoverable parent blob `7603b207b2efdd6c90a886c6e6f0abc232fa3bbc`;
- QX lifecycle blob `39f91dc8510df49620cf3336e98656532b46c158`;
- generic persistent custody blob `4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`;
- PR #590 remains draft/open/unmerged/mergeable with external status `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- canonical QZ audit Drive ID `1CaEO4QCqukAp9lO0NPlrs5QxC1amBz8T`, 20749 bytes, SHA-256 `08d075677d637e8d733faf7fd4869639232e52b181a3547524c3202ddf33d59b`.

`main` remains unchanged at head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

## 2. Fresh post-QZ source finding

QZ materialized only the dormant fallible scheduling-specific persistent worker entry in:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

The exact QZ entry returns:

`RecoverablePersistentWorkerEntry<AuthenticatedRemoteSessionRuntimeOwner, RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop>`.

The historical scheduling-aware collection path still publishes:

`RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion`

whose terminal result is:

`Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The recoverable parent file already establishes the exact owner-bearing completion-envelope law used by both historical requester-aware and historical scheduling-aware repeated-admission paths:

1. authenticated logical `DeviceId` remains by-value custody;
2. exact recovered `AuthenticatedRemoteSessionRuntimeOwner` remains by-value custody;
3. exact worker stop or bounded join failure remains by-value custody;
4. accessors borrow identity/owner/result without cloning terminal custody;
5. `into_parts()` transfers all custody by value;
6. the envelope itself performs no peer close, owner disposition, retry, requester cleanup, scheduling request construction, or higher propagation.

Therefore the immediate missing dependency after QZ is not collection migration itself. It is one parallel owner-bearing completion envelope capable of carrying the exact QX/QZ fallible scheduling stop through the existing recoverable completion topology.

## 3. Selected future RB source ceiling

Future C03e-RB may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact predecessor blob:

`7603b207b2efdd6c90a886c6e6f0abc232fa3bbc`

RB must keep byte-stable:

- QZ integration file `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs` at blob `db1398a64e51898df072bb49511dd6f0c99dbeb7`;
- QX lifecycle file `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs` at blob `39f91dc8510df49620cf3336e98656532b46c158`;
- generic persistent custody file `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs` at blob `4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`;
- production-durable repeated-admission collection file at current QZ blob `08dc8160d72c41bc9a211c2c2e15f0e65d067b74`.

If correctness requires a second Rust path, collection integration, generic custody mutation, worker-entry mutation, disposer mutation, endpoint migration, producer propagation, Cargo/workflow widening, or visibility widening outside this parent, STOP and return to selection.

## 4. Selected future RB type

RB may import the already-existing exact stop:

`RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop`

into the recoverable parent and add exactly one owner-bearing repeated-admission completion envelope:

`RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion`

Selected fields, exactly by value:

- `device_id: DeviceId`;
- `session_owner: AuthenticatedRemoteSessionRuntimeOwner`;
- `result: Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The new envelope is parallel to the existing historical scheduling-aware completion and must not replace, rename, or mutate it.

## 5. Selected future RB constructor/accessor law

RB may add only the minimal constructor/accessor surface required to preserve exact custody:

1. `new(device_id, session_owner, result) -> Self` consumes all three values by value;
2. `device_id(&self) -> &DeviceId` borrows authenticated identity;
3. `session_owner(&self) -> &AuthenticatedRemoteSessionRuntimeOwner` borrows exact recovered owner;
4. `result(&self) -> &Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>` borrows the exact terminal result;
5. `into_parts(self)` transfers `DeviceId`, recovered owner and exact result by value;
6. no `Clone` or `Copy` requirement is introduced for the envelope or fallible scheduling stop;
7. no scheduling-terminal grant, derivation error, acknowledgement disposition, nested lifecycle error, verifier-time error, ingress error, requester-response error, or abnormal join is flattened, stringified, reconstructed, reminted, copied, defaulted, or diverted to a side channel.

The exact fallible scheduling stop remains the sole worker-terminal type inside the envelope.

## 6. Explicit non-selection of owner disposition

RB must not add or modify a peer/owner disposition classifier or disposer.

In particular RB does not select:

- a fallible scheduling-specific analogue of `select_recoverable_requester_aware_scheduling_peer_disposition(...)`;
- a fallible scheduling-specific completion disposer;
- any call to `close_for_orderly_shutdown()`;
- any call to `close_for_requester_aware_terminal_failure()`;
- any interpretation of scheduling derivation success/failure for peer disposition;
- any mapping of exact QX `Failed(...)` or `SchedulingTerminal(...)` into historical stop types.

Owner disposition remains a separate later checkpoint after exact fallible completion custody exists.

## 7. Explicit non-selection of collection migration

RB must not migrate the repeated real-admission supervisor or collection.

RB therefore does not select:

- a fallible scheduling active-map alias;
- a fallible scheduling recoverable completion alias in the integration file;
- fallible scheduling publish/reap/poll/drain helpers;
- in-flight admission drain migration;
- any replacement of historical `spawn_recoverable_requester_aware_worker_with_production_durable_scheduling(...)` inside the collection;
- any call to QZ `spawn_recoverable_fallible_verifier_time_requester_aware_worker_with_production_durable_scheduling(...)` from a collection;
- any change to the historical `T: FnMut() -> u64 + Send + 'static` collection bound;
- any expected-request channel type migration;
- any collection callback type migration.

These remain later gates after completion custody is materialized.

## 8. Exact nested custody to preserve

When the future envelope carries:

`Ok(RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop::Failed(error))`

it must retain the exact QV/QX lifecycle error by value, including nested typed provenance from:

- fallible verifier-time source failure;
- authenticated post-auth ingress failure;
- requester terminal acknowledgement framing failure;
- requester terminal acknowledgement response I/O failure.

When it carries:

`Ok(RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop::SchedulingTerminal(outcome))`

it must retain the existing non-Clone scheduling terminal carrier by value, with the scheduling derivation channel and requester acknowledgement channel still independent.

`Err(RemoteSessionSpawnedWorkerJoinError)` remains the exact existing bounded abnormal-join channel.

## 9. Historical surfaces preserved unchanged

RB must preserve unchanged:

- `RecoverableSpawnedRequesterRendezvousWorkerCompletion`;
- `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`;
- `RecoverableRepeatedRealAdmissionRequesterAwareSchedulingWorkerCompletion`;
- all existing requester-aware and scheduling-aware peer disposition selectors;
- `dispose_recoverable_repeated_real_admission_requester_aware_worker_completion(...)`;
- `dispose_recoverable_repeated_real_admission_requester_aware_scheduling_worker_completion(...)`;
- generic recoverable persistent entry/completion primitives;
- QZ fallible scheduling worker entry and constructor;
- QX fallible scheduling lifecycle worker/stop;
- historical infallible scheduling worker/stop and collection.

## 10. Later gates explicitly not selected

RA does not select or authorize:

- fallible scheduling owner-disposition selection/materialization;
- active fallible scheduling-map migration;
- fallible scheduling poll/reap/drain adapter migration;
- repeated real-admission collection migration;
- repeated real-admission completion callback migration;
- cooperative scheduling producer propagation;
- executor endpoint-lifecycle propagation;
- higher endpoint-owner propagation;
- concrete producer specialization/reintegration;
- expected-device scheduling request construction/send;
- target admission `SessionId` generation;
- expected-device PRWM request-ID allocation;
- new timing acquisition outside the existing fallible verifier-time source;
- process/executable caller migration;
- listener/readiness/network/runtime activation;
- new task/channel/queue topology;
- peer reuse expansion;
- retry/reconnect/respawn/restart;
- fallback/default/cached verifier time;
- auth/trust/RBAC mutation;
- DB/schema/control-plane mutation;
- Cargo/lockfile/workflow mutation;
- Android source mutation;
- packaging/service/systemd mutation;
- repository configuration mutation;
- merge, deployment, ready-for-review transition, or PR close;
- branch deletion/reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

## 11. Validation law for RA

RA itself is docs-only selection.

Exact QZ -> RA validation must prove:

- ahead exactly one commit;
- behind zero;
- merge base exact QZ final head;
- exactly one changed documentation path;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging/deployment/repository-config changes.

All CI PASS claims must bind only to exact final RA head. `SKIPPED` is not PASS. No Android PASS may be inherited if no Android workflow registers on the docs-only exact head.

## 12. Evidence and closure law

RA closes only after:

1. exact-final-head topology validation;
2. exact-final-head CI completion;
3. immutable markdown audit freeze;
4. exact-title Drive pre-upload collision check;
5. single canonical upload under parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw Drive readback byte-count/SHA-256/final-LF verification;
7. exact-title post-upload singleton verification;
8. Drive metadata and revision-lineage verification;
9. PR body metadata updated to `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
10. PR remains draft/open/unmerged.

The frozen audit must retain internal status `SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING` and must not be rewritten after publication.

## 13. STOP boundary

C03e-RA stops after selection evidence closure.

Do not materialize C03e-RB inside RA closure.
Do not add the fallible completion envelope in RA.
Do not add owner disposition in RA.
Do not migrate active-map/poll/reap/drain helpers in RA.
Do not migrate the repeated real-admission collection in RA.
Do not propagate to producer/executor/endpoint/higher-owner surfaces.
Do not merge, deploy, activate runtime behavior, rewrite history, or destructively clean evidence.
