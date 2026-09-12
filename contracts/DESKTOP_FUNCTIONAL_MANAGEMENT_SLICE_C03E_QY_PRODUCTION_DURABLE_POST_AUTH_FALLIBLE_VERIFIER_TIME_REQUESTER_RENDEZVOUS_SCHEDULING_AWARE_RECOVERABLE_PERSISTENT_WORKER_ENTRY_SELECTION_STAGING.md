# Desktop Functional Management Slice C03e-QY — Production-Durable Post-Auth Fallible Verifier-Time Requester/Rendezvous Scheduling-Aware Recoverable Persistent Worker Entry Selection Staging

**Checkpoint:** C03e-QY  
**Boundary:** `PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_PERSISTENT_WORKER_ENTRY_SELECTION`  
**Status:** `SELECTION — VALIDATION PENDING`

## 1. Authority and predecessor

This selection begins only from evidence-closed C03e-QX.

Exact predecessor:

- branch: `phase-152-c03e-qx-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-scheduling-aware-lifecycle-source-materialization`
- head: `e41a63187d91f372e6333ba58c77c72bed855740`
- tree: `1b05d6b78c24105bebe89b558ff121897c1d492a`
- lifecycle source blob: `39f91dc8510df49620cf3336e98656532b46c158`
- PR: #588, preserved draft/open/unmerged/mergeable
- canonical QX evidence Drive ID: `1ulfFh-YEODY56_JITPR3sK5a0eUGYPpF`
- canonical QX evidence bytes: `21950`
- canonical QX evidence SHA-256: `2d8ea06942324e1d9af793b5d7dfe72cc84df6d2c6b67625405dd250945d2e73`

QX materialized the dormant fallible scheduling-aware requester lifecycle and explicitly left persistent recoverable scheduling-worker propagation as a later gate.

## 2. Live selection finding

At exact QX source state, the first existing historical consumer layer after the scheduling-aware requester lifecycle is the recoverable persistent worker-entry constructor in:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact QX blob:

`e52c248463ca57c51a487df2357f72df0d2212dc`

The file already contains the historical infallible scheduling-specific specialization:

`RecoverableSchedulingRequesterAwareWorkerEntry = RecoverablePersistentWorkerEntry<AuthenticatedRemoteSessionRuntimeOwner, RequesterRendezvousProductionDurableSchedulingWorkerStop>`

and dormant constructor:

`spawn_recoverable_requester_aware_worker_with_production_durable_scheduling(...)`

That constructor retains exact authenticated-session owner custody in the existing recoverable owner cell, creates exactly one existing cooperative cancellation pair, spawns exactly one task, borrows the retained owner mutably inside that task, and delegates to the historical infallible scheduling-aware lifecycle worker. It returns the existing generic recoverable persistent entry containing owner-cell, cancellation-controller, and join-handle custody.

The parent recoverable completion/disposition module does not need mutation for this immediate boundary. The repeated real-admission collection does not need migration yet. Therefore executor-wide, collection-wide, producer, endpoint, or higher-owner propagation would exceed the narrow first post-QX seam.

## 3. Selected future boundary

C03e-QY selects exactly one future source-materialization boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_PERSISTENT_WORKER_ENTRY_SOURCE_MATERIALIZATION`

Provisional future checkpoint: **C03e-QZ**.

Provisional future branch:

`phase-152-c03e-qz-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-scheduling-aware-recoverable-persistent-worker-entry-source-materialization`

## 4. Exact QZ source ceiling

Future QZ may modify exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Required predecessor blob:

`e52c248463ca57c51a487df2357f72df0d2212dc`

Guard-only paths that must remain byte-stable:

1. QX lifecycle source:
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `39f91dc8510df49620cf3336e98656532b46c158`
2. recoverable spawned requester/rendezvous parent:
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`
   - blob `7603b207b2efdd6c90a886c6e6f0abc232fa3bbc`
3. generic recoverable persistent custody primitives:
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`
   - blob `4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`

If QZ requires a second Rust path, parent completion-envelope mutation, generic persistent custody mutation, repeated real-admission loop migration, producer/executor/endpoint/higher-owner mutation, Cargo/workflow widening, or runtime activation, STOP and return to selection.

## 5. Selected future type seam

QZ may add one dormant scheduling-specific recoverable entry specialization over the exact QX stop, conceptually:

`RecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkerEntry`

with exact underlying custody:

`RecoverablePersistentWorkerEntry<AuthenticatedRemoteSessionRuntimeOwner, RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop>`

No new owner-cell type, cancellation controller, join-handle abstraction, completion envelope, scheduling terminal carrier, or error family is selected.

## 6. Selected future constructor

QZ may add exactly one dormant sibling constructor, conceptually:

`spawn_recoverable_fallible_verifier_time_requester_aware_worker_with_production_durable_scheduling(...)`

The selected verifier-time provider bound is:

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`

The selected constructor keeps the historical explicit authority inputs:

- `RemoteSessionWorkerAdmission<D, T>` by value;
- `Arc<ProductionDurableCapabilityAuthority>` by value for spawned-task lifetime;
- `&SharedCurrentCapabilityAuthority<P>` for requester DR/current scheduling authority, cloned only through its existing shared handle;
- `&Arc<S>` requester-aware policy source, cloned only as the existing shared handle;
- `&SharedRequesterRendezvousAuthority`, cloned only through its existing shared handle.

## 7. Selected QZ worker-entry law

Future QZ must preserve the historical recoverable scheduling entry law while replacing only the worker body and stop type with the exact QX fallible scheduling-aware equivalents:

1. consume one existing `RemoteSessionWorkerAdmission<D, T>` by value;
2. preserve the exact fallible verifier-time provider by value; do not sample it in the entry constructor;
3. retain the exact authenticated-session owner inside one existing `Arc<Mutex<Option<_>>>` owner cell;
4. retain one supervisor-side owner-cell handle and move one cloned handle into the spawned task;
5. create exactly one existing `remote_session_worker_cancellation_pair()`;
6. retain the cancellation controller in the returned persistent entry;
7. move the cancellation signal into the spawned task and pass only `into_cancelled()` to the exact QX worker;
8. acquire the owner-cell guard inside the spawned task and borrow the exact owner mutably without taking ownership out of the cell;
9. call exactly `run_fallible_verifier_time_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`;
10. pass the exact fallible verifier-time provider by value into that QX worker; no default, fallback, cache, reconstruction, or independent time sampling;
11. preserve production-durable capability authority and requester DR authority as distinct lanes;
12. preserve requester-aware policy and requester/rendezvous authority through their existing shared handles;
13. preserve caller-owned dispatcher by moving it into the one task and mutably borrowing it for exact QX execution;
14. after QX worker completion, drop the owner guard and return the exact `RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop` from the task;
15. return only the existing generic `RecoverablePersistentWorkerEntry` custody containing owner-cell, cancellation-controller, and join-handle;
16. do not close or reuse the peer;
17. do not publish a completion envelope;
18. do not reap/drain a persistent map;
19. do not migrate the repeated real-admission supervisor;
20. do not clone/copy/remint/reconstruct scheduling terminal custody;
21. do not allocate an admission `SessionId`, PRWM request ID, timing value, new channel, queue, or task beyond the one already inherent in the historical entry constructor;
22. do not retry, reconnect, respawn, restart, panic-convert, flatten errors, or fabricate success.

## 8. Why repeated real-admission migration is not selected yet

The same source file also contains active scheduling maps, completion publication adapters, drain/reap helpers, and repeated real-admission collection logic. Those surfaces consume and publish the historical infallible scheduling stop and therefore will eventually require their own fallible propagation gate.

QY deliberately stops before that migration. The immediate selected seam is only the dormant persistent worker entry specialization and constructor. This keeps the exact QX stop recoverable across task completion without yet changing collection APIs, completion envelopes, higher-owner disposition, producer channels, or endpoint behavior.

## 9. Explicit later gates

QY does not select or authorize:

- repeated real-admission collection migration to the fallible scheduling stop;
- fallible scheduling completion-envelope publication;
- fallible scheduling owner-disposition migration;
- cooperative scheduling producer propagation;
- executor endpoint-lifecycle propagation;
- higher endpoint-owner propagation;
- concrete QN producer specialization/reintegration;
- QJ dispatcher capture;
- QL channel construction/ownership split;
- receiver-to-QF wiring;
- higher receipt observation policy;
- expected-device admission request construction or send;
- target admission `SessionId` generation;
- expected-device PRWM authentication request-ID allocation;
- new timing acquisition outside the existing fallible verifier-time source;
- process/executable caller migration;
- listener/readiness/network/runtime activation;
- task-spawn topology changes beyond the historical one-task entry constructor;
- new queue/channel construction;
- peer-close expansion;
- retry/reconnect;
- fallback/default/cached verifier time;
- authentication/trust/RBAC mutation;
- database/schema/control-plane mutation;
- Cargo/lockfile/workflow mutation;
- Android source mutation;
- packaging/service/systemd mutation;
- repository configuration mutation;
- merge, deployment, ready-for-review transition, or PR close;
- branch deletion/reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

## 10. Validation and evidence law

QY is a docs-only selection checkpoint. All validation claims must bind only to its exact final head.

- Rust CI success on exact final QY head may be claimed only if the workflow registers and completes successfully.
- Android PASS must not be inherited from QX; claim it only if an Android workflow registers for exact QY head and succeeds.
- `SKIPPED` is never PASS.
- Before immutable audit publication, exact-title Drive search under canonical parent must return zero QY audit artifacts.
- Freeze audit bytes before upload.
- Upload once.
- Raw-read back and verify byte count, SHA-256, and final LF.
- Require exact-title singleton after upload unless a new anomaly must be preserved/classified.
- Verify Drive revision lineage.
- Only then may the PR body record external `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

## 11. STOP boundary

C03e-QY stops after selection validation and immutable evidence closure.

Do not materialize QZ inside QY closure.  
Do not migrate repeated real-admission collection here.  
Do not mutate any Rust/source path here.  
Do not merge, deploy, activate runtime/network behavior, mark ready for review, rewrite history, delete branches, or destructively clean evidence.
