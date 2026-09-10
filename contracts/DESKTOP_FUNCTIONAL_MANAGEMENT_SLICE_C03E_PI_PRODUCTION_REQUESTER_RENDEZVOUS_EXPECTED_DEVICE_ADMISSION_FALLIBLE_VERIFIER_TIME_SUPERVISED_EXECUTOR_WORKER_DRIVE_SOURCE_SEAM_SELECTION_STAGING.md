# C03e-PI — Production Requester/Rendezvous Expected-Device Admission Fallible Verifier-Time Supervised Executor Worker Drive Source-Seam Selection

Status: `SELECTION / STAGING`
Date: `2026-09-10`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_SUPERVISED_EXECUTOR_WORKER_DRIVE_SOURCE_SEAM_SELECTION`

Selected later boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_SUPERVISED_EXECUTOR_WORKER_DRIVE_SOURCE_MATERIALIZATION`

## 1. Decision and scope

C03e-PI performs the documentation-only supervisor-compatibility source-seam selection authorized only after closed C03e-PH.

Selection result:

`ONE_DORMANT_FALLIBLE_VERIFIER_TIME_SUPERVISED_EXECUTOR_WORKER_DRIVE_SIBLING / EXISTING_GENERIC_SUPERVISOR_HELPER_REUSED / PERSISTENT_PROPAGATION_DEFERRED`

The selected later materialization adds one lexically-contained supervisor analogue for the already-materialized fallible verifier-time worker path. It must preserve the historical single-worker supervisor ownership, cancellation and join semantics while substituting only the fallible verifier-time worker terminal type and provider callback.

PI changes documentation only. It does not modify Rust/source, create the future sibling, retype persistent collection state, migrate admission/request-carrier types, install a concrete verifier-time provider, activate a production caller, wire startup, merge, deploy or enable runtime behavior.

## 2. Exact authoritative predecessor

Repository: `Gersi365/prw-executor-private`

Closed predecessor: `C03e-PH`

PH branch:
`phase-152-c03e-ph-production-requester-rendezvous-expected-device-admission-fallible-verifier-time-spawned-executor-worker-drive-source-materialization`

Exact PH head:
`0104aadebeb938804222d3af04570688ba97f127`

Exact PH tree:
`53c6ca4f676f7e5c9ad9b78635def43ef9da69c9`

Exact executor source blob:
`161b8182d1697f9f0d9f06110b03e633ff701919`

PH PR:
`#547 — C03e-PH: materialize fallible verifier-time spawned executor worker drive`

PH remains draft/open/unmerged and its body records:
`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

PH immutable Drive audit ID:
`12kgEAejq_OaMa9UMdi7OjMSmyWdqW47E`

Canonical evidence parent:
`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

## 3. Fresh exact-source findings

At exact PH source, `RemoteSessionExecutorRuntime` preserves the historical staged decomposition:

1. borrowed synchronous worker drive;
2. lexically-contained spawned-and-joined worker drive;
3. bounded current-thread single-worker supervisor;
4. persistent worker collection;
5. later admission/request-carrier and lifecycle layers.

PH adds only the fallible verifier-time analogue for stage 2.

The next unfilled compatibility layer is therefore stage 3: the bounded single-worker supervisor.

## 4. Existing generic supervisor helper is sufficient

Exact PH source already contains:

```rust
async fn await_supervised_worker<T, S>(
    mut worker_handle: JoinHandle<T>,
    cancellation_controller: RemoteSessionWorkerCancellationController,
    supervisor_shutdown: S,
) -> Result<T, RemoteSessionSpawnedWorkerJoinError>
where
    S: Future<Output = ()> + Send,
```

The helper is generic over worker terminal `T`. It therefore already accepts the fallible worker terminal type without retyping or adding another helper.

The helper's ordering law is retained:

- poll the worker handle first on every race wake;
- only if the worker is still pending, poll supervisor shutdown;
- if the worker completes first, return that exact worker result after bounded join mapping;
- if supervisor shutdown wins while the worker is pending, request cancellation once through the retained cancellation controller;
- then await the same existing worker handle to terminal completion;
- map only abnormal Tokio task completion to `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

PI selects no modification to `await_supervised_worker`.

## 5. Historical supervisor seam used as shape authority

Exact PH source contains historical:

`drive_supervised_capability_request_worker`

Its current shape:

```rust
pub fn drive_supervised_capability_request_worker<
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> u64 + Send + 'static,
    S: Future<Output = ()> + Send,
>(
    &mut self,
    session_owner: AuthenticatedRemoteSessionRuntimeOwner,
    authority: &SharedCurrentCapabilityAuthority<P>,
    verifier_time_unix_seconds: T,
    dispatcher: D,
    supervisor_shutdown: S,
) -> Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>
```

It clones the shared-current authority once, enters one private-runtime `block_on`, creates one cancellation controller/signal pair, spawns one worker, moves the cancellation signal into that worker, then passes the same worker handle and retained controller to `await_supervised_worker`.

PI selects that ownership/race shape and nothing wider.

## 6. Selected future source ceiling

A later separately gated C03e-PJ source materialization may modify exactly one existing Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No other Rust/source path, Cargo manifest, lockfile, workflow, Android source, host configuration, database, authorization, packaging, endpoint, listener, startup or `main.rs` path is selected.

If implementation unexpectedly requires another source path, the later checkpoint must stop and report the mismatch rather than widen scope automatically.

## 7. Selected dormant sibling

The selected later sibling is conceptually:

`RemoteSessionExecutorRuntime::drive_supervised_fallible_verifier_time_capability_request_worker`

Maximum selected visibility:

`pub(super)`

The future sibling remains dormant and crate-internal staging surface. PI does not select production caller migration.

## 8. Selected future signature law

Conceptual signature:

```rust
pub(super) fn drive_supervised_fallible_verifier_time_capability_request_worker<
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static,
    S: Future<Output = ()> + Send,
>(
    &mut self,
    session_owner: AuthenticatedRemoteSessionRuntimeOwner,
    authority: &SharedCurrentCapabilityAuthority<P>,
    verifier_time_unix_seconds: T,
    dispatcher: D,
    supervisor_shutdown: S,
) -> Result<
    AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop,
    RemoteSessionSpawnedWorkerJoinError,
>
```

The future materialization may use the existing fully-qualified fallible verifier error/worker stop paths if that is the narrowest import-compatible implementation. PI does not require import widening solely for aesthetics.

## 9. Spawn and ownership law

The future sibling must:

- clone the shared-current authority exactly once;
- consume `AuthenticatedRemoteSessionRuntimeOwner` by value;
- consume the dispatcher by value;
- consume the fallible verifier-time provider by value;
- consume the supervisor-shutdown future by value into the enclosing private-runtime async drive;
- create exactly one `RemoteSessionWorkerCancellationController` / cancellation-signal pair through the existing `remote_session_worker_cancellation_pair()` helper;
- retain the cancellation controller beside the local worker handle;
- move the paired cancellation signal into the one worker task;
- create exactly one `tokio::spawn(async move { ... })` worker;
- reconstruct mutable owner/dispatcher custody inside that task;
- retain the local join handle until `await_supervised_worker` returns.

No detached task, task registry, channel, extra join handle, replacement worker or runtime-handle clone is selected.

## 10. Direct worker delegation law

The spawned task must invoke exactly once:

`AuthenticatedRemoteSessionRuntimeOwner::run_fallible_verifier_time_capability_request_worker`

The task must pass:

- the cloned shared-current authority by reference;
- the exact fallible verifier-time callback;
- the mutable dispatcher;
- `cancellation_signal.into_cancelled()` from the supervisor-owned cancellation pair.

The future supervisor sibling must not call:

- PF's synchronous borrowed fallible bridge;
- PH's synchronous spawned-and-joined bridge;
- the historical infallible worker;
- persistent collection entrypoints.

Direct async worker delegation preserves the existing supervisor's runtime and cancellation ownership model without nested `block_on` or nested synchronous executor calls.

## 11. Supervisor shutdown / worker-terminal precedence

The existing `await_supervised_worker` helper remains the sole supervisor race authority.

The selected ordering is:

1. on each wake, poll the worker join handle first;
2. if the worker is ready, its result wins even if supervisor shutdown is also ready on that wake;
3. only while the worker remains pending may supervisor shutdown win;
4. if supervisor shutdown wins, request cancellation once;
5. continue awaiting the same worker handle rather than fabricating a supervisor-level terminal value.

Supervisor shutdown therefore does not convert itself directly into `Cancelled`.

The C03e-PD fallible worker remains authoritative for the internal request-loop-versus-cancellation ordering after the cancellation signal becomes ready.

## 12. Fallible worker terminal law

The selected result type is:

`Result<AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

Normal joined worker terminals remain exact:

- `Ok(AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Cancelled)`;
- `Ok(AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Failed(exact_PB_error))`.

Only abnormal Tokio task completion becomes:

`Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)`.

The supervisor layer must not:

- convert `Failed` into a join error;
- convert verifier-time failure into `Cancelled`;
- stringify or erase the PB error;
- introduce a new supervisor error type;
- expose raw `JoinError`, panic payload, task ID or runtime identity.

## 13. Verifier-time law

Verifier time remains supplied only through:

`FnMut() -> Result<u64, PrwaVerifierSourceError>`

The supervisor bridge must not sample verifier time itself.

No eager sample, retry, fallback, default, cache, clamp, saturation, stale-value reuse or conversion back to `FnMut() -> u64` is selected.

The exact provider callback moves into the spawned fallible worker and remains governed by the previously materialized PB/PD failure semantics.

## 14. Cancellation and peer-close authority

The supervisor layer owns only the external supervisor cancellation pair and join custody.

It must not close the peer directly.

When supervisor shutdown wins, it requests cancellation through the existing controller. The C03e-PD worker remains authoritative for whether its request loop or cancellation wins and for the existing code-4 cancellation close.

Verifier-time/request processing failures remain governed by PB/PD failure-close behavior.

No duplicate close, alternate close code or supervisor-level peer shutdown is selected.

## 15. Bound discipline

Selected generic bounds mirror only the already-proven spawned/supervised requirements:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`;
- `S: Future<Output = ()> + Send`.

The supervisor shutdown future does not require `'static` because it is not moved into the spawned worker; it remains inside the enclosing synchronous private-runtime drive.

PI does not select gratuitous `Clone`, `Copy`, `Default`, `Unpin`, extra `Sync`, or other unrelated bounds.

## 16. Existing PH spawned seam remains distinct

PH's:

`drive_spawned_fallible_verifier_time_capability_request_worker`

accepts a caller-supplied cancellation future and performs a direct lexical spawn/join.

The selected future supervisor sibling is different: it constructs and retains a cancellation controller, gives the paired signal to the worker, races the worker against supervisor shutdown, and requests cancellation only if shutdown wins.

PI therefore does not implement the supervisor by calling the PH synchronous spawned bridge. Doing so would surrender the supervisor's required cancellation-controller custody and race visibility.

## 17. Persistent collection remains deferred

Exact PH source still types persistent collection state around the historical infallible terminal:

- `RemoteSessionPersistentWorkerEntry<T>` is generic internally;
- `RemoteSessionRegisteredWorkerCompletion` currently stores `Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`;
- `RemoteSessionWorkerAdmission<D,T>` carries the historical provider shape through later collection/admission code;
- persistent worker spawning still delegates to the historical infallible request worker.

PI does not retype or migrate these surfaces.

After the future supervised source materialization closes, persistent result/ownership compatibility remains the next separate graph layer to assess.

## 18. Historical paths preserved

The future materialization must leave existing historical methods unchanged, including:

- `drive_capability_request_worker`;
- `drive_spawned_capability_request_worker`;
- `drive_supervised_capability_request_worker`;
- `drive_persistent_remote_worker_collection`;
- historical admission/rejection types;
- historical registered completion typing;
- repeated real-admission supervision;
- endpoint lifecycle;
- integrated expected-device admission/request-carrier path.

No existing production caller is redirected by PI or its selected later materialization.

## 19. Explicit non-actions

PI does not authorize:

- Rust/source materialization in PI itself;
- modifying `await_supervised_worker` merely to add a fallible specialization;
- persistent entry/result retyping;
- persistent collection migration;
- admission/request-carrier migration;
- requester/rendezvous authority changes;
- concrete verifier-time provider installation;
- production caller invocation;
- endpoint/listener/startup/readiness/network activation;
- dependency changes;
- database/auth/security mutations;
- retry/fallback/default/cache behavior;
- merge or deploy;
- mark-ready transition;
- branch deletion;
- force push, squash, rebase or history rewrite.

Runtime activation remains unchanged.

## 20. Later source-materialization validation ceiling

A later C03e-PJ materialization must prove at minimum:

- exact predecessor PI head unchanged;
- exactly one selected Rust path changed;
- additive dormant sibling only;
- one existing cancellation pair creation;
- one authority clone;
- one `tokio::spawn`;
- one direct `run_fallible_verifier_time_capability_request_worker` invocation;
- one `await_supervised_worker` invocation;
- exact fallible provider and worker-stop types preserved;
- no persistent/admission/provider/caller propagation;
- exact-head formatting, Clippy, tests and workspace build success;
- Android validation reported with its actual trigger/result state;
- every skipped workflow represented as `SKIPPED`, not PASS.

If a same-file lint expectation is required solely to preserve a selected custody signature, it must remain method-local and justified. No broad lint suppression is selected.

## 21. PI repository scope

PI itself may add only this documentation contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_PI_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_SUPERVISED_EXECUTOR_WORKER_DRIVE_SOURCE_SEAM_SELECTION_STAGING.md`

Exact PH -> PI must contain:

- one commit;
- one added contract path;
- zero deletions;
- zero Rust/source/Cargo/lock/workflow/Android/runtime changes.

## 22. PI validation and durable closure

PI closes only after:

- exact PH predecessor remains `0104aadebeb938804222d3af04570688ba97f127`;
- PR #547 remains draft/open/unmerged and closed in its evidence body;
- PH immutable Drive evidence remains available under exact ID/metadata;
- PH -> PI is exactly one added documentation path;
- the existing Rust workflow on exact final PI head reaches terminal success;
- every automatically registered workflow is reported with its actual terminal result;
- `SKIPPED` is not represented as PASS;
- an immutable Markdown PI audit is uploaded once to canonical Drive evidence parent after exact-title presearch zero;
- metadata and raw Drive readback reproduce exact frozen bytes/hash;
- post-upload exact-title search resolves only the canonical object;
- only after evidence acceptance may the PI PR body record `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- the PI PR remains draft/open/unmerged;
- final PH/PI/main/successor guards remain exact.

## 23. Successor ceiling

After PI closes, stop at PI.

The selected future source materialization is expected to be C03e-PJ, but it requires a fresh exact-head/source/concurrency audit before any Rust mutation.

After a separately validated PJ closure, later work may assess, one layer at a time:

1. persistent result/ownership compatibility;
2. admission/request-carrier compatibility;
3. concrete verifier-time provider installation;
4. production caller invocation.

None is pre-authorized by PI.

## 24. Closure meaning

C03e-PI closes only the supervised-executor fallible verifier-time source-seam selection.

It proves that the existing generic supervisor helper can preserve the historical worker-first race, cancellation-controller custody and bounded join mapping without retyping that helper, while a future one-file sibling substitutes the fallible verifier-time worker/provider types.

PI itself introduces no runtime behavior and performs no source materialization.
