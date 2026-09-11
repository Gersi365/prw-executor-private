# C03e-PS — Production requester/rendezvous expected-device admission fallible verifier-time endpoint lifecycle runtime composition source-layout reselection

Status: `SOURCE_LAYOUT_RESELECTION — STAGED`

## Purpose

C03e-PS is a documentation-only corrective selection checkpoint above the exact safely restored C03e-PR state. It does not materialize Rust source.

C03e-PQ selected a one-file endpoint-wrapper materialization. C03e-PR attempted that exact one-file layout, and canonical Rust validation proved that the selected endpoint wrapper cannot call the already-materialized C03e-PP fallible executor lifecycle under the executor method's current nested-module visibility. The candidate was restored forward-only to the exact PQ source tree and C03e-PR closed as blocked.

C03e-PS therefore reselects only the minimum source/access layout required to make the previously selected wrapper composition expressible without widening a public API, changing runtime semantics, or crossing into provider/caller activation.

## Exact predecessor authority

Authoritative predecessor is the final safe C03e-PR head:

`2e491a0a2e52f4fa0fce9f491d6351d4914f7c2b`

Predecessor tree:

`ba21f00b9a54cede08b6b3419918394eaa2d2ac1`

The predecessor tree is byte-for-byte the same Git tree as C03e-PQ after C03e-PR's forward-only safe restore.

C03e-PR PR #557 remains draft/open/unmerged with administrative status:

`SOURCE MATERIALIZATION — BLOCKED — SAFE RESTORE VALIDATED — EVIDENCE_RECORDED — SELECTION REFINEMENT REQUIRED`

Canonical blocked-outcome audit:

`C03E_PR_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_RUNTIME_COMPOSITION_SOURCE_MATERIALIZATION_AUDIT_2026-09-11.md`

Drive ID:

`1PaDPK7rirP_SFlp7QkBqISfEZ9WhtA6e`

Exact bytes:

`16784`

SHA-256:

`80a3d6e838fae287d7bc38c12e50725144842f204eca9010f0ba0ff02cf60de0`

## Exact safe source identity

Endpoint lifecycle source:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact predecessor blob:

`e9f82f32ba875c2461513e445f81962720dc3298`

Executor source:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Exact predecessor blob:

`7ea5dbe8d6e7844d4c38ddd78c42e84311ac6d54`

Parent capability-runtime module:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

Exact predecessor blob:

`de66532f18ebbca30ac6bd6b9da4983ded4b8bbe`

## Canonical failure evidence requiring reselection

The C03e-PR candidate head was:

`055922095bf95d21b400bdc236c1d94ec0273301`

Rust validation #1759 / run `34579174641` / job `103198494555` failed at compile/Clippy after locked dependency graph and formatting passed.

The exact compiler evidence was:

1. `E0425`: the candidate referred to `super::RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`, but the parent module does not re-export that carrier.
2. `E0624`: `RemoteSessionExecutorRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle` is private from the endpoint-lifecycle sibling under its current visibility.

The candidate was not retained. Final safe C03e-PR Rust #1762 / run `34579758755` / job `103200685879` succeeded on exact safe head `2e491a0a2e52f4fa0fce9f491d6351d4914f7c2b` after the source tree was restored.

## Fresh privacy/layout audit

Exact executor source proves the C03e-PP lifecycle method is defined inside the private nested module:

`repeated_real_admission_supervisor`

Inside that nested module, the method currently has:

`pub(super)`

At that lexical location, `super` is `remote_session_executor_runtime`, not the common parent `remote_session_capability_runtime`. Therefore the endpoint-lifecycle sibling cannot call it.

Exact executor source also proves the completion carrier:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

is defined at the root of `remote_session_executor_runtime` with:

`pub(super)`

At that lexical location, `super` is the common parent `remote_session_capability_runtime`. That visibility already permits bounded access from descendants of the common parent; the C03e-PR `E0425` arose because the candidate used an absent parent re-export (`super::Type`) instead of naming/importing the sibling executor-module item directly.

Therefore C03e-PS does **not** select a parent-module re-export and does **not** select completion-carrier visibility widening.

## Corrective source-layout precedent

C03e-LO previously established the project pattern for this exact class of problem: when a prior documentation selection proves physically insufficient under the real module layout, the next checkpoint is a documentation-only source-layout reselection with an exact revised path ceiling before source mutation.

C03e-PS follows that pattern.

## Reselected immediate source successor

The next separately gated source materialization is ceilinged to exactly two Rust paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

No third source path is selected.

### Path 1 — executor visibility correction only

The future source checkpoint may change only the visibility of the existing C03e-PP method:

`RemoteSessionExecutorRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

from its current nested-module-local `pub(super)` boundary to the narrow common-parent capability-runtime scope.

Selected semantic visibility is:

`pub(in crate::remote_session_capability_runtime)`

or the exact rustfmt-equivalent ancestor-restricted form if the compiler requires a mechanically equivalent spelling.

This visibility is intentionally narrower than `pub(crate)` and much narrower than `pub`.

The future source checkpoint must not:

- change the method name;
- change parameters, generic bounds, callback shapes, return type, body, teardown behavior, runtime custody, or error semantics;
- move the method;
- add a generic executor bridge;
- expose a raw Tokio runtime or handle;
- make the method public outside `remote_session_capability_runtime`;
- widen any historical infallible method as part of this correction.

The only selected executor-file semantic change is the minimum ancestor-scoped visibility required for the endpoint-lifecycle sibling to call the already-existing PP method.

### Completion carrier access law

The future endpoint wrapper must use the existing completion carrier unchanged:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

The carrier's existing executor-root `pub(super)` visibility remains unchanged.

The endpoint source may name/import it through the sibling module, for example semantically equivalent to:

`super::remote_session_executor_runtime::RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

No new parent-module re-export is selected.

No change to:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

is selected.

If canonical Rust compilation disproves direct sibling access under the existing carrier visibility, the future materialization must STOP and reselect again; it must not silently add a parent re-export or a third path.

### Path 2 — endpoint wrapper materialization

The future source checkpoint may add only the previously selected dormant sibling:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

The wrapper must:

- remain no wider than the narrow existing endpoint-owner internal boundary required by current source;
- consume the existing endpoint lifecycle owner exactly once;
- destructure/retain the existing executor, bound transport, and supervisor-shutdown signal exactly as the historical infallible wrapper does;
- delegate exactly once to the existing C03e-PP executor fallible endpoint lifecycle;
- borrow the same retained transport as `&transport`;
- supply the existing endpoint-owner shutdown future through `supervisor_shutdown.into_shutdown()` or the exact current equivalent;
- preserve the exact provider bound:
  `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`;
- preserve the exact existing fallible completion carrier unchanged;
- forward admission timing, rejection, and admission-failure callbacks unchanged;
- return the exact existing `Result<(), RemoteSessionPersistentCollectionConfigError>` unchanged;
- add no second endpoint close;
- add no second `wait_idle()`;
- add no second shutdown source;
- perform no verifier-time sampling, installation, retry, default, cache, clamp, or error translation.

C03e-PP remains the sole owner of close -> `wait_idle()` -> exact-result ordering.

## Historical infallible path freeze

The existing historical infallible wrapper:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle`

must remain byte-semantically unchanged except for formatting that is provably mechanically forced by rustfmt within a touched region; no behavioral modification is selected.

The existing infallible executor lifecycle and repeated collection remain unchanged.

## Identity, authority, and concurrency invariants

C03e-PS changes no runtime behavior. The future two-file source checkpoint must preserve:

- pre-auth expected `DeviceId` as scheduling intent only;
- authenticated owner-derived `DeviceId` as post-auth worker-map authority;
- duplicate expected-device rejection before timing/network work;
- no request polling while the active collection is full;
- at most one AJ future in flight;
- retained AJ drain across supervisor shutdown;
- post-shutdown AJ success orderly authenticated-owner close without worker spawn;
- active worker cancellation/drain under existing controller/join-handle custody;
- `Cancelled` and exact fallible worker failure as normal completion values;
- abnormal Tokio completion as the existing bounded join-error family;
- one private current-thread executor runtime;
- no nested `block_on` while the repeated supervisor is alive;
- existing endpoint close then idle-drain sequencing.

## Explicitly deferred

C03e-PS does not select or authorize:

- concrete verifier-time provider installation;
- verifier-time sampling at the endpoint wrapper;
- requester/rendezvous producer migration;
- higher-owner caller migration;
- production expected-device source construction;
- dispatcher population or capability-authority population changes;
- session-authentication population changes;
- listener or bind invocation;
- startup/readiness publication;
- process-signal handling;
- `main.rs` mutation;
- `linux_bootstrap.rs` mutation;
- production reachability wrapper migration;
- service/systemd mutation;
- host credential mutation;
- database/schema/control-plane mutation;
- authentication/authorization redesign;
- retry/reconnect/rebootstrap behavior;
- generic runtime access;
- public API widening;
- deployment;
- merge;
- ready-for-review conversion;
- PR closure;
- branch deletion;
- force-push, reset, rebase, squash, or history rewrite.

## Accidental branch boundary

The unrelated accidental administrative branch:

`tmp-never-use`

points to the failed C03e-PR candidate head. C03e-PS does not delete, retarget, merge, or otherwise mutate that branch.

## Validation law for C03e-PS itself

C03e-PS is documentation-only.

Its final diff must contain exactly one contract path and zero Rust/source/runtime/workflow/manifest/lockfile/Android/packaging/executable changes.

All CI claims must bind only to the exact final C03e-PS head.

`SKIPPED` is not PASS. A workflow that does not trigger is not PASS.

## Future materialization fail-closed law

Before the later two-file source checkpoint mutates anything, it must freshly reread:

- exact C03e-PS final head;
- exact executor blob;
- exact endpoint wrapper blob;
- exact parent capability-runtime blob;
- exact target branch/PR/artifact namespace.

The materialization must enforce an exact two-path net ceiling.

If compilation requires:

- a parent-module re-export;
- completion-carrier visibility widening;
- a third source path;
- a new bridge method;
- a changed error/completion/provider surface;
- a caller/provider/startup mutation;

then STOP and reselect; do not widen silently.

## C03e-PS gate

Selected gate:

`C03E_PS_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_RUNTIME_COMPOSITION_SOURCE_LAYOUT_RESELECTED`

Selected closure:

`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_RUNTIME_COMPOSITION_SOURCE_LAYOUT_RESELECTION`

After durable evidence publication and administrative closure of this documentation-only selection checkpoint: **STOP**.
