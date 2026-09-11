# C03e-PQ — Production requester/rendezvous expected-device admission fallible verifier-time endpoint lifecycle runtime composition selection

Status: `SELECTION — SOURCE MATERIALIZATION DEFERRED`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_RUNTIME_COMPOSITION_SELECTION`

## 1. Purpose

C03e-PQ selects one later, separately gated source-materialization boundary above the closed C03e-PP executor-level fallible verifier-time endpoint lifecycle.

This checkpoint is documentation-only. It does not modify Rust source, runtime behavior, workflow configuration, manifests, lockfiles, Android source, requester/rendezvous producers, process lifecycle, startup/readiness, service state, host state, database/control-plane state, repository configuration, or deployment state.

The selected later source seam is one dormant sibling on the existing Agent-owned endpoint lifecycle wrapper:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

The selected future source ceiling is exactly:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

## 2. Exact predecessor

Closed predecessor checkpoint:

`C03e-PP — Production requester/rendezvous expected-device admission fallible verifier-time endpoint lifecycle composition source materialization`

Exact PP branch:

`phase-152-c03e-pp-production-requester-rendezvous-expected-device-admission-fallible-verifier-time-endpoint-lifecycle-composition-source-materialization`

Exact PP head:

`73c5aad56f5badf8ec7536ac501a4f57539a9542`

Exact PP tree:

`cb45d89b6f4812b9d38e047565698d222b2e0050`

Exact PP executor source blob:

`7ea5dbe8d6e7844d4c38ddd78c42e84311ac6d54`

Exact endpoint-lifecycle wrapper blob observed before PQ selection:

`e9f82f32ba875c2461513e445f81962720dc3298`

PP PR #555 remains draft/open/unmerged with administrative status:

`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

## 3. Fresh gap audit

The exact PP source was re-audited before this selection.

The executor now contains the C03e-PP dormant fallible sibling:

`RemoteSessionExecutorRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

The existing `RemoteSessionEndpointLifecycleRuntime` already exposes the historical infallible wrapper that delegates to the historical infallible executor endpoint lifecycle and supplies its owned supervisor-shutdown future.

No corresponding fallible verifier-time wrapper sibling exists on the exact PP head.

Therefore the narrow unresolved composition gap immediately above PP is the endpoint-lifecycle owner/wrapper delegation seam. Concrete verifier-time provider installation and higher production caller migration remain separate later concerns.

## 4. Historical decomposition retained

Historical C03e-AM/C03e-AN selected/materialized executor-level supervisor shutdown and endpoint teardown ordering.

Historical C03e-AO/C03e-AP then selected/materialized the Agent-owned endpoint lifecycle owner around executor-before-bind startup, recoverable reachability-authority custody, one explicit remote-supervisor shutdown controller/signal pair, and delegation into the executor endpoint lifecycle.

Historical C03e-AQ/C03e-AR separately selected/materialized process-level/single-executor composition above that owner.

The fallible verifier-time migration preserves this layering rather than collapsing it:

1. C03e-PM selected the fallible repeated-real-admission persistent collection;
2. C03e-PN materialized that collection;
3. C03e-PO selected the executor-level fallible endpoint-lifecycle composition;
4. C03e-PP materialized that executor-level composition;
5. C03e-PQ selects only the corresponding `RemoteSessionEndpointLifecycleRuntime` wrapper composition;
6. any source materialization of that wrapper is a later checkpoint;
7. provider installation, producer migration and production invocation remain later separately gated boundaries.

## 5. Selected future sibling

A later source-materialization checkpoint may add exactly one dormant sibling:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

The sibling must be structurally parallel to the existing historical infallible endpoint-lifecycle wrapper only where that preserves already-selected semantics.

It must delegate exactly once to:

`RemoteSessionExecutorRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

It must not duplicate the executor lifecycle algorithm inside the wrapper.

## 6. Selected parameter forwarding

The future wrapper must forward unchanged the existing fallible endpoint-lifecycle inputs required by PP, including:

- `max_active_workers`;
- retained `AgentRemoteTransportRuntime`;
- shared current capability authority;
- mutable session-authentication service;
- expected-device admission request receiver carrying the fallible verifier-time provider;
- real-admission timing callback;
- fallible registered-worker completion callback;
- expected-device admission rejection callback;
- repeated-admission failure callback.

No wrapper-local replacement identity, timing value, retry state, cache, or derived authorization signal is selected.

## 7. Supervisor shutdown ownership

The existing `RemoteSessionEndpointLifecycleRuntime` owns one remote-specific supervisor-shutdown controller/signal pair established by the historical endpoint startup/lifecycle seam.

The future fallible wrapper must supply that existing owned supervisor future to the PP executor lifecycle exactly as the historical infallible wrapper supplies its existing supervisor shutdown future.

Selected form:

`supervisor_shutdown.cancelled()`

or the exact equivalent already used by the existing wrapper at source-materialization time.

The wrapper must not create a second shutdown source.

A shutdown request remains only a signal that makes the existing paired supervisor future ready. It does not itself close the endpoint, cancel workers directly, abort tasks, publish readiness, or become authorization evidence.

## 8. Fallible verifier-time provider surface

The future wrapper preserves the PP/PN provider surface unchanged:

`T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`

The wrapper does not select or install a concrete verifier-time provider.

It must not:

- sample verifier time;
- cache verifier time;
- default a verifier timestamp;
- clamp a verifier timestamp;
- retry provider access;
- translate the provider error into a new wrapper error;
- stringify the provider error as control flow;
- derive authorization from time-source success or failure.

Provider ownership and sampling remain below this wrapper in the existing fallible admission/worker path.

## 9. Fallible worker completion surface

The future wrapper preserves the existing fallible completion surface unchanged:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

Normal worker terminals remain worker-completion values, including cancellation and exact fallible verifier-time worker failure.

The wrapper must not promote worker completion into a new endpoint-lifecycle error family.

Abnormal spawned-task completion remains governed by the existing worker/join semantics below the selected wrapper.

## 10. Outer result law

The future wrapper returns the exact PP executor-lifecycle result without adding a new error envelope:

`Result<(), RemoteSessionPersistentCollectionConfigError>`

The existing collection configuration error remains the only selected outer result error at this layer.

The wrapper must not convert AJ failures, worker completions, verifier-time source failures, shutdown requests, endpoint close state, or idle-drain completion into new outer lifecycle errors.

## 11. Endpoint teardown authority

C03e-PP remains authoritative for executor-level endpoint teardown composition.

The future wrapper must not call `transport_runtime.close(...)` directly and must not call `transport_runtime.wait_idle()` directly as a second teardown path.

The wrapper delegates to PP and therefore inherits PP's ordering:

1. fallible repeated collection fully returns;
2. existing endpoint close runs exactly once;
3. existing `wait_idle()` runs only after close on the same private current-thread executor;
4. the exact captured collection result is returned unchanged.

No new endpoint teardown helper, close code, close reason, idle-drain runtime, detached task, or retry path is selected.

## 12. Identity invariant

The selected wrapper preserves:

`logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Expected pre-authentication `DeviceId` remains scheduling intent only.

Authenticated owner-derived `DeviceId` remains post-authentication worker authority.

`request_id` remains correlation only.

Transport address, endpoint state, verifier timestamp, Tokio task identity, session correlation, shutdown state, or successful teardown must not become replacement capability authorization evidence.

## 13. Capability and authorization invariant

The future wrapper does not alter capability authority population or policy evaluation.

Successful endpoint bind, successful AJ authentication, worker collection membership, verifier-time provider success, endpoint shutdown, or wrapper completion do not independently imply capability authorization.

Existing shared-current capability authority remains authoritative at the already-selected capability boundaries.

## 14. Runtime custody

The wrapper continues to use the single retained private current-thread `RemoteSessionExecutorRuntime` already owned by `RemoteSessionEndpointLifecycleRuntime`.

The later materialization must not add:

- a second Tokio runtime;
- `rt-multi-thread`;
- a public/generic runtime handle;
- generic `block_on` exposure;
- a nested private-runtime drive while another lifecycle drive is live;
- a detached endpoint idle-drain task;
- hard task abort as lifecycle policy.

## 15. Historical infallible seam remains untouched

The existing historical method:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle`

must remain behaviorally and textually untouched unless a strictly necessary source-local import/list accommodation is proven by the later materialization.

PQ selects an additive fallible sibling; it does not replace, rename, widen, remove, genericize, or refactor the historical infallible wrapper.

No shared generic abstraction is selected merely to deduplicate the two wrappers.

## 16. Future source ceiling

The later source-materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

No contract, module registration, parent re-export, Cargo manifest, lockfile, workflow, Android, transport, requester/rendezvous, executor, startup binary, packaging, service, host, database, schema, control-plane, authentication, or authorization path is selected for that future materialization.

If exact source conditions prove that a second path is required, that is a new boundary and must stop for a fresh selection rather than silently widening the ceiling.

## 17. Visibility ceiling

The later sibling must use the minimum visibility required by its separately selected caller boundary.

PQ does not authorize a public API expansion.

Until a concrete higher owner/caller is separately selected, the sibling should remain private to the narrowest existing module/crate boundary consistent with the current endpoint-lifecycle implementation.

Any visibility widening is a separately justified change, not implied by PQ.

## 18. Explicitly deferred provider/caller wiring

PQ does not select:

- a concrete production verifier-time provider;
- verifier-time provider installation;
- a requester/rendezvous producer migration;
- a requester/rendezvous higher-owner caller migration;
- a production expected-device request source;
- production dispatcher construction;
- production capability-authority population;
- session-authentication population changes;
- endpoint listener activation;
- endpoint bind invocation from executable startup;
- production bind-address selection;
- readiness publication;
- process-signal ownership changes;
- `main.rs` wiring;
- `linux_bootstrap.rs` wiring;
- service/systemd mutation;
- host credential mutation;
- database/schema/control-plane mutation;
- deployment;
- merge.

## 19. No recovery-policy expansion

The future wrapper must not introduce:

- retry;
- reconnect;
- replacement endpoint creation;
- re-bootstrap;
- alternate bind;
- fallback provider;
- cached verifier time;
- default verifier time;
- time clamping;
- detached recovery worker;
- fail-open behavior.

Any recovery policy requires its own selection checkpoint.

## 20. C03e-PQ path ceiling

C03e-PQ itself may change exactly one documentation path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_PQ_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_RUNTIME_COMPOSITION_SELECTION_STAGING.md`

PQ must contain zero Rust/source/runtime/workflow/manifest/lockfile/Android/requester-rendezvous/transport/startup/host/deployment changes.

## 21. Validation semantics

All PASS claims for PQ must bind only to the exact final PQ head.

Every exact-head workflow that actually triggers must be reported with its actual terminal conclusion.

`SKIPPED` is not PASS.

A workflow that does not trigger is not reported as PASS.

Documentation-only scope does not waive exact-head CI requirements for workflows that do trigger.

## 22. Durable evidence semantics

After exact-final-head validation, publish one immutable Markdown audit under the canonical PRW Drive audit parent.

Required evidence discipline:

1. exact-title pre-upload search must return zero matches;
2. freeze local bytes and SHA-256 before upload;
3. upload once as raw `text/markdown`;
4. raw Drive readback must match exact byte count and SHA-256;
5. exact-title post-upload search must return exactly one canonical artifact;
6. reread exact PQ branch/tree/contract blob after publication;
7. reread PR state after publication;
8. closure may update only PR body evidence/status metadata;
9. PR remains draft/open/unmerged.

## 23. Closure rule

C03e-PQ may close only as:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Closure means only that the later dormant endpoint-lifecycle wrapper composition is selected and durably evidenced.

Closure does not mean source materialization, caller migration, provider installation, runtime activation, readiness, deployment, or merge.

After C03e-PQ closure: **STOP**.

Do not create the source-materialization successor as part of PQ closure.

A fresh exact-head, exact-source, concurrency and evidence audit is required before any later Rust mutation.
