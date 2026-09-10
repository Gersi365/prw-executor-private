# C03e-PE — Fallible verifier-time borrowed executor worker-drive source seam selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_BORROWED_EXECUTOR_WORKER_DRIVE_SOURCE_SEAM_SELECTION`

Selected future source boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_BORROWED_EXECUTOR_WORKER_DRIVE_SOURCE_MATERIALIZATION`

C03e-PE is documentation-only. It selects the narrow first executor compatibility prerequisite after closed C03e-PD. It does not materialize Rust behavior, assign a production caller, widen a public runtime surface, or activate any worker path.

## 1. Exact authority and source observations

Authoritative predecessor is closed C03e-PD, PR #543, draft/open/unmerged:

- exact head: `8dcea3563df0a4b7e3d6d81919a019642a249cd0`;
- exact tree: `f3a4a4a9b5ce9129c04353aaa926e89cae4581d4`;
- exact changed authenticated-session blob: `20d93c729ce50f1905fbe94fb9a75004b2c6c204`;
- PC-to-PD: ahead 2 / behind 0, one Rust path, +414/-0;
- `main`: `7c993fa93977a0bb84e0d030874eee7fd0cae77f`.

PD immutable evidence is recorded at canonical Drive ID `1MJLtsEK_ySDOM7uh2S15DFvakz-Xx4t4`, parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`, raw Markdown bytes `12298`, SHA-256 `e2403651718fef064d392399c5c6ee3b1d5afe2654a70df6c19273971978c417`.

Fresh source reads at exact PD establish:

| Path | Exact Git blob | Observation |
| --- | --- | --- |
| `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` | `20d93c729ce50f1905fbe94fb9a75004b2c6c204` | contains the closed PD fallible verifier-time cancellation-aware worker |
| `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` | `ef370ca500f118bc067097ddb8f5c37ab597b214` | still contains only the historical infallible executor propagation chain |

The executor source is 80,259 bytes at exact PD. Its current worker graph is deliberately layered:

1. borrowed synchronous executor drive: `drive_capability_request_worker`;
2. spawned-and-joined drive: `drive_spawned_capability_request_worker`;
3. supervised worker drive: `drive_supervised_capability_request_worker`;
4. persistent worker collection and registered completion ownership;
5. integrated admission/request-carrier path.

The current borrowed driver accepts `T: FnMut() -> u64 + Send`, synchronously uses the private Tokio runtime through `block_on`, delegates to the historical infallible `run_capability_request_worker`, and returns `AuthenticatedRemoteSessionWorkerStop`.

The current spawned, supervised, persistent and integrated layers also retain the historical infallible verifier-time provider and historical worker-stop typing. They therefore form later compatibility propagation gates; their existence does not authorize changing them in the immediate successor.

C03e-PD introduced the dormant sibling `run_fallible_verifier_time_capability_request_worker` on `AuthenticatedRemoteSessionRuntimeOwner`. That worker accepts `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send`, owns no executor, and returns exactly `AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop`, whose only worker terminal families are `Cancelled` and `Failed(AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError)`.

PD remains the authority for loop-first race behavior, cancellation drop-before-close ordering, exact fallible-loop error preservation, and code-4 cancellation close. The executor compatibility seam selected here must not duplicate or reinterpret those semantics.

Fresh pre-selection concurrency guards found no C03e-PE branch, no exact-title `C03e-PE:` pull request, and no semantically equivalent `fallible-verifier-time-borrowed-executor-worker-drive` branch. No concurrent successor was adopted.

## 2. Why the immediate executor gate is borrowed-only

The historical executor path was intentionally decomposed instead of materialized as one broad caller migration. The existing source documentation itself records the staged sequence from the borrowed driver through spawned, supervised and persistent ownership.

The same decomposition must be preserved for fallible verifier-time compatibility. The smallest independently reviewable next prerequisite is not the spawned worker, supervisor, persistent collection, admission carrier, or production provider installation. It is one additive borrowed executor drive that can prove the executor owner can synchronously drive the already-materialized PD worker without changing ownership or lifetime laws.

Selecting spawned compatibility now would add `'static` ownership, task creation, join behavior and authority cloning before the lower borrowed bridge is established. Selecting supervisor compatibility would additionally entangle supervisor shutdown and cancellation-controller custody. Selecting persistent compatibility would also change stored worker-result typing and collection completion surfaces. Selecting integrated admission would mix all of those concerns with request-carrier/provider propagation and production ownership.

Those are separate semantic boundaries and remain separately gated.

Selected decomposition:

`FIRST_EXECUTOR_COMPATIBILITY_STAGE_BORROWED_SYNCHRONOUS_DRIVE_ONLY / EXACTLY_ONE_EXECUTOR_SOURCE_PATH / DELEGATE_ONCE_TO_CLOSED_PD_WORKER / PRESERVE_EXACT_PD_STOP / NO_NEW_ERROR_WRAPPER / NO_SPAWN_OR_JOIN / NO_SUPERVISOR_PROPAGATION / NO_PERSISTENT_RESULT_RETYPING / NO_ADMISSION_OR_REQUEST_CARRIER_MIGRATION / NO_PROVIDER_INSTALLATION / NO_RUNTIME_ACTIVATION`

## 3. Selected immediate future source ceiling

Exactly one existing Rust source path may change in the later materialization:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No other Rust path, Cargo manifest, lockfile, workflow, Android source, packaging path, deployment path, contract, generated artifact or runtime configuration belongs to that future source ceiling except narrowly necessary same-file tests/documentation and local imports in the selected executor file.

Select one additive dormant sibling method on `RemoteSessionExecutorRuntime`, conceptually:

`drive_fallible_verifier_time_capability_request_worker`

The sibling is an executor-neutral compatibility bridge only in the sense that it exposes no raw Tokio handle or generic future-driving API; it still uses the already-owned private Tokio runtime exactly as the historical borrowed executor seam does.

The future sibling is synchronous, not `async`. Its purpose is to synchronously drive exactly one already-borrowed PD worker future through the private runtime's existing `block_on` custody.

Conceptual signature:

```rust
pub(super) fn drive_fallible_verifier_time_capability_request_worker<
    P: PolicyEvaluator + Send + Sync,
    D: CapabilityDispatcher + Send,
    T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send,
    C: Future<Output = ()> + Send,
>(
    &mut self,
    session_owner: &mut AuthenticatedRemoteSessionRuntimeOwner,
    authority: &SharedCurrentCapabilityAuthority<P>,
    verifier_time_unix_seconds: T,
    dispatcher: &mut D,
    cancellation: C,
) -> AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop
```

`pub(super)` is the maximum selected visibility for this dormant compatibility stage. No crate-public or public API widening is selected. If a later production caller requires wider visibility, that requirement must be demonstrated and selected separately.

Required same-file imports may include only the existing `PrwaVerifierSourceError` type and the PD sibling worker-stop type needed by the signature. No new dependency is selected.

## 4. Exact delegation law

The future borrowed sibling must delegate exactly once to the already-materialized PD worker:

```rust
self.runtime.block_on(
    session_owner.run_fallible_verifier_time_capability_request_worker(
        authority,
        verifier_time_unix_seconds,
        dispatcher,
        cancellation,
    ),
)
```

Equivalent formatting is allowed; semantic broadening is not.

The executor bridge must forward unchanged:

- the same mutable authenticated-session owner borrow;
- the same borrowed shared-current capability authority;
- the exact caller-supplied fallible verifier-time provider by value;
- the same mutable dispatcher borrow;
- the exact caller-supplied cancellation future by value.

The exact `AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop` returned by PD must be returned directly. Do not introduce:

- a new executor error enum;
- a new worker-stop enum;
- an error wrapper;
- string/error projection;
- `map_err` or semantic conversion;
- failure-to-cancellation conversion;
- cancellation-to-success conversion;
- retry tokens or success sentinels.

The borrowed executor sibling does not close the peer. PB/PD retain sole authority for verifier-time/transaction failure closure and cancellation closure. No duplicate close, alternate close diagnostic, rollback or replacement connection is selected.

## 5. Verifier-time and transaction law

The future executor sibling does not sample verifier time itself. It forwards the provider unchanged to PD.

No eager read may occur before `block_on`. No sample may be taken in executor cancellation handling because this selected bridge has no separate cancellation handling. No second sample, retry, fallback, default, cached value, frozen value, saturation, clamp or conversion to the historical infallible `FnMut() -> u64` callback is selected.

A `PrwaVerifierSourceError` remains validation-source failure, not an executor failure and not cancellation. A capability transaction failure remains its exact PB nested transaction error. The exact distinction preserved by PB and PD must survive this bridge without new taxonomy.

Already-ready cancellation retains PD's established loop-first semantics. PE does not select shutdown-first behavior, an eager cancellation precheck, `select!` replacement, or polling-order changes.

## 6. Borrow and lifetime law

The future sibling keeps the historical borrowed-driver ownership shape:

- `&mut self` borrows the executor owner for the synchronous drive;
- `&mut AuthenticatedRemoteSessionRuntimeOwner` remains borrowed, not moved;
- authority remains borrowed;
- dispatcher remains mutably borrowed;
- verifier-time provider and cancellation future are moved only into the one PD worker future created inside the one `block_on` call.

Do not add `'static`, `Clone`, `Copy` or `Unpin` bounds merely for this borrowed stage. Those concerns belong to later spawned ownership if and when separately selected.

No authority clone is needed for this borrowed stage. No runtime handle clone, `tokio::spawn`, `spawn_local`, join handle, cancellation controller, task registry, channel, persistent map entry or detached future may be introduced.

The private Tokio runtime remains inaccessible to callers. No generic `block_on` API or generic executor surface is selected.

## 7. Historical-path preservation

The later source materialization must leave the historical infallible path semantically unchanged, including:

- `drive_capability_request_worker`;
- `drive_spawned_capability_request_worker`;
- `drive_supervised_capability_request_worker`;
- existing persistent worker entry/result typing;
- `RemoteSessionRegisteredWorkerCompletion`;
- existing admission/rejection wrappers;
- existing persistent collection mechanics;
- existing integrated expected-device admission/request-carrier path.

No historical method is replaced, renamed, retyped or redirected to the fallible path in the immediate materialization.

No call site is migrated merely because the sibling exists.

## 8. Deferred executor propagation graph

After the future borrowed source materialization closes, exact source must be re-audited before selecting any of these later stages:

1. **Spawned compatibility** — an owned `'static` fallible-provider worker path and exact fallible stop under existing join-error custody.
2. **Supervisor compatibility** — explicit supervisor-shutdown behavior around the fallible spawned worker, preserving worker-first/shutdown law already selected for the historical supervisor.
3. **Persistent ownership compatibility** — active-entry and completion-result typing for the fallible worker, including exact join-error separation.
4. **Admission/request-carrier compatibility** — moving the fallible provider through the existing admission carrier and retained worker registration path.
5. **Concrete verifier-time provider installation** — choosing the existing fallible clock authority at the actual producer/caller boundary without fallback or panic adaptation.
6. **Production caller invocation** — only after all required ownership and error propagation seams are independently selected/materialized/validated.

No later layer is implicitly authorized by successful borrowed compatibility.

## 9. Validation requirements for later source materialization

The future source checkpoint must prove at minimum:

- exact PD merge base and expected predecessor SHA;
- exactly the selected executor source path changed;
- the new sibling is additive and dormant;
- the historical borrowed/spawned/supervised/persistent/integrated paths remain unchanged;
- callback type remains exactly fallible `Result<u64, PrwaVerifierSourceError>`;
- no `'static` bound is added to the borrowed sibling;
- no spawn/join/supervisor/persistence/admission behavior is introduced;
- the method contains one private-runtime `block_on` of one PD worker invocation and no verifier-time sampling or error conversion;
- return type is the exact PD worker stop;
- formatting, Clippy, workspace tests and workspace build succeed on the exact final source head.

A narrowly justified dormant-code lint acknowledgement is allowed if required by the compiler. It must not conceal a broader unused public surface.

If an existing same-file fixture can exercise the bridge without creating wider production fixtures, a focused test may be added. If a live authenticated owner/transport fixture would require wider production source changes, do not create that fixture merely to satisfy this stage. Report the limitation and substantiate the one-line custody bridge through exact source review plus exact-head workspace CI. Do not claim end-to-end transport or production caller coverage.

`SKIPPED` remains distinct from `PASS`. Predecessor CI does not validate the future source head.

## 10. Identity, authority and correlation laws

This selection does not alter PRW identity semantics.

Canonical separation remains:

`PRW logical device/session identity`
`-> registry/discovery`
`-> current reachable endpoint/candidates`
`-> authenticated transport`

Verifier time is validation input only. It is not logical identity, requester identity, target identity, transport identity, capability authority or PRWM correlation.

Requester callback `DeviceId` remains requester-side correlation. Target expected `DeviceId` remains derived only from the consumed target scheduling grant where that flow applies. Target admission `SessionId` remains distinct from requester scheduling `SessionId`.

A PRWM `request_id` remains transaction correlation only and is not identity or authorization evidence.

PE opens, clones, reconstructs, remints, consumes or disposes no scheduling grant. It constructs no request and sends no request.

## 11. Explicit non-actions

C03e-PE performs or authorizes none of the following:

- Rust source materialization;
- modification of `remote_session_executor_runtime.rs`;
- modification of the PD authenticated-session source;
- spawned fallible worker materialization;
- supervisor fallible propagation;
- persistent worker result retyping;
- admission/request-carrier migration;
- verifier-time provider installation;
- production expected-device request construction/send;
- runtime/listener/network activation;
- endpoint/startup/main wiring;
- production caller invocation;
- deployment or service restart;
- merge;
- PR close or ready-for-review conversion;
- branch deletion;
- force push or history rewrite;
- Cargo/lockfile/workflow/Android/packaging mutation;
- repository configuration/ruleset/visibility change;
- architecture redesign;
- database/schema change;
- authentication cutover;
- service/systemd/package mutation;
- credential/certificate/private-key/trust/RBAC change.

## 12. C03e-PE documentation scope and closure

Only this added documentation path may differ from exact PD:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_PE_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_BORROWED_EXECUTOR_WORKER_DRIVE_SOURCE_SEAM_SELECTION_STAGING.md`

Prove exact PD merge base, one forward commit and one added contract with zero Rust/Cargo/lockfile/workflow/Android-source/runtime changes.

Review terminal exact-final-head workflow conclusions. PASS claims bind only to the exact final PE head. `SKIPPED` is never PASS. Do not inherit PD workflow results.

After validation, freeze one immutable raw Markdown audit. Before publication, perform a global non-trashed exact-title Drive search and require zero matches. Upload exactly once to canonical parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`, preserve Markdown source type, verify metadata and exact raw bytes/SHA-256, and require post-publication exact-title search to return exactly one canonical artifact before recording closure metadata in the PR.

Keep the PR draft/open/unmerged. No merge, deployment, activation, branch deletion, history rewrite or repository configuration mutation occurs in PE.

After C03e-PE selection closes, stop at that checkpoint. The selected borrowed executor source materialization requires a fresh exact-head/concurrency audit in a later continuation. C03e-PE itself does not assign or materialize a source-successor token beyond naming the separately gated future boundary.
