# C03e-PG — Production Requester/Rendezvous Expected-Device Admission Fallible Verifier-Time Spawned Executor Worker Drive Source-Seam Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_SPAWNED_EXECUTOR_WORKER_DRIVE_SOURCE_SEAM_SELECTION`

Selected future boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_SPAWNED_EXECUTOR_WORKER_DRIVE_SOURCE_MATERIALIZATION`

## 1. Authoritative predecessor

C03e-PG starts only from the exact closed C03e-PF head:

`e21cace3b21d2e2bd4e87777d0d33c00be599a6c`

Exact PF tree at selection start:

`20b6e2a60f048243ec353061de579b45548bf560`

PF already materialized the dormant borrowed executor compatibility seam:

`RemoteSessionExecutorRuntime::drive_fallible_verifier_time_capability_request_worker`

Its exact changed-path blob at selection start is:

`f27ace4190d83f14ac0071d2b46279c5a1200e84`

This PG checkpoint is documentation-only. It does not modify Rust/source/runtime/manifest/workflow/Android code.

## 2. Fresh source finding

The existing executor file already contains the historical staged decomposition:

1. borrowed synchronous drive: `drive_capability_request_worker`;
2. lexically-contained spawned-and-joined drive: `drive_spawned_capability_request_worker`;
3. supervised spawned drive: `drive_supervised_capability_request_worker`;
4. persistent worker collection and later admission/lifecycle layers.

The historical spawned seam:

- consumes `AuthenticatedRemoteSessionRuntimeOwner` by value;
- consumes dispatcher and verifier-time provider by value;
- requires `P`, `D`, `T`, and the cancellation future to be spawn-compatible where needed;
- clones the shared-current authority exactly once;
- enters the existing private runtime through one synchronous `block_on`;
- creates exactly one `tokio::spawn(async move { ... })` task;
- recreates mutable local owner/dispatcher custody inside that task;
- delegates to the async authenticated-session worker;
- awaits the same local join handle before returning;
- maps only abnormal Tokio join completion to the existing bounded `RemoteSessionSpawnedWorkerJoinError`.

PF's new fallible verifier-time borrowed seam is deliberately narrower and does not itself spawn. The immediate compatibility gap is therefore the spawned-and-joined layer only.

## 3. Selected PH source ceiling

The first source successor after this selection may modify exactly one existing Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No other Rust/source path, Cargo manifest, lockfile, workflow, Android, packaging, host, database, authorization, or configuration path is selected.

## 4. Selected dormant sibling

PH may add exactly one dormant sibling on `RemoteSessionExecutorRuntime`, conceptually named:

`drive_spawned_fallible_verifier_time_capability_request_worker`

Maximum selected visibility:

`pub(super)`

Conceptual signature:

```rust
pub(super) fn drive_spawned_fallible_verifier_time_capability_request_worker<
    P: PolicyEvaluator + Send + Sync + 'static,
    D: CapabilityDispatcher + Send + 'static,
    T: FnMut() -> Result<
            u64,
            prw_session::prwa_verifier_source::PrwaVerifierSourceError,
        > + Send
        + 'static,
    C: Future<Output = ()> + Send + 'static,
>(
    &mut self,
    session_owner: AuthenticatedRemoteSessionRuntimeOwner,
    authority: &SharedCurrentCapabilityAuthority<P>,
    verifier_time_unix_seconds: T,
    dispatcher: D,
    cancellation: C,
) -> Result<
    super::authenticated_remote_session_runtime::AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop,
    RemoteSessionSpawnedWorkerJoinError,
>
```

Formatting may follow exact `rustfmt` output; semantic bounds and ownership are the selected contract.

## 5. Selected implementation semantics

The successor must mirror the historical lexically-contained spawned-and-joined ownership shape, while substituting only the already-materialized fallible verifier-time worker semantics.

Conceptually:

```rust
let authority = (*authority).clone();

self.runtime.block_on(async move {
    let worker_handle = tokio::spawn(async move {
        let mut session_owner = session_owner;
        let mut dispatcher = dispatcher;

        session_owner
            .run_fallible_verifier_time_capability_request_worker(
                &authority,
                verifier_time_unix_seconds,
                &mut dispatcher,
                cancellation,
            )
            .await
    });

    worker_handle
        .await
        .map_err(|_| RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)
})
```

The exact PD worker remains sole authority for:

- fallible verifier-time sampling;
- verifier-source error classification;
- request-loop versus cancellation ordering;
- peer close behavior;
- exact `AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop` classification.

The executor layer adds only the already-existing bounded Tokio join-failure envelope around that exact worker stop.

## 6. Ownership and bound selection

Unlike PF's borrowed sibling, the spawned sibling must own values moved into one `'static` Tokio task. Therefore PG selects only the spawn-driven lifetime widening already demonstrated by the historical spawned seam:

- `P: ... + 'static`;
- `D: ... + 'static`;
- `T: ... + 'static`;
- `C: ... + 'static`.

This is not permission to add `Clone`, `Copy`, `Unpin`, `Default`, `Sync`, or any other new bound beyond what the historical spawned shape or the exact async worker requires.

The authority handle may be cloned exactly once before task creation, matching the historical spawned seam. The session owner, dispatcher, verifier-time provider, and cancellation future remain single-owner values moved into the task.

## 7. Join-result contract

The selected return shape is:

`Result<AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

This means:

- `Ok(Cancelled)` remains a normal worker terminal result;
- `Ok(Failed(exact_PB_error))` remains a normal worker terminal result containing the exact PD/PB failure classification;
- only an abnormal Tokio task join becomes `Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)`.

PH must not flatten worker failure into join failure, must not wrap the PD stop in a new executor enum, and must not expose raw Tokio `JoinError`, panic payload, task ID, runtime identity, or other Tokio implementation detail.

## 8. Exact delegation rule

The spawned task must delegate directly to:

`AuthenticatedRemoteSessionRuntimeOwner::run_fallible_verifier_time_capability_request_worker`

It must not call `drive_fallible_verifier_time_capability_request_worker` from inside the async task. PF's method is a synchronous private-runtime `block_on` bridge and is not the async worker body. Calling that bridge from inside the executor task would cross the selected runtime/ownership boundary and is not authorized.

## 9. Explicit non-actions

PG does not select, and PH must not add:

- supervisor-shutdown racing;
- construction of a cancellation controller/signal pair;
- persistent worker collection propagation;
- persistent completion/result retyping;
- repeated real-admission propagation;
- expected-device request-carrier migration;
- requester/rendezvous authority changes;
- concrete verifier-time provider installation;
- verifier-time fallback/default/retry/cache behavior;
- production caller invocation;
- endpoint/listener/startup wiring;
- readiness publication;
- network activation;
- `main.rs` activation;
- dependency changes;
- authorization or security-policy mutation;
- merge, deployment, branch deletion, force push, or history rewrite.

Historical infallible borrowed/spawned/supervised/persistent/admission paths remain untouched.

## 10. Deferred compatibility graph

Only after PH source materialization closes on a fresh exact-head audit may later checkpoints consider, one boundary at a time:

1. supervisor compatibility;
2. persistent worker result/ownership compatibility;
3. admission/request-carrier compatibility;
4. concrete verifier-time provider installation;
5. production caller invocation.

No later item is pre-authorized by PG.

## 11. Validation and closure protocol

PG must close as a docs-only selection checkpoint:

1. exact PF predecessor remains unchanged;
2. PG contains exactly one new contract file and zero source/runtime changes;
3. exact-final-head CI results are recorded precisely;
4. `SKIPPED` is never represented as `PASS`;
5. immutable Google Drive audit evidence is published once with raw readback equality;
6. the PG PR remains draft/open/unmerged;
7. `main` remains unchanged;
8. PH successor namespace remains empty at closure;
9. closure records selection only and stops.

PH source materialization requires a later continuation and fresh concurrency/source audit.