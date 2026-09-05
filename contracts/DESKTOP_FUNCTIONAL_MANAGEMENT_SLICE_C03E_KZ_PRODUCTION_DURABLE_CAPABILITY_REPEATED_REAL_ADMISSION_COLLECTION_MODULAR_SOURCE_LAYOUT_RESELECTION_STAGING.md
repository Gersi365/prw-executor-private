# C03e-KZ Production Durable Capability Repeated Real-Admission Collection Modular Source Layout Reselection Staging

Status: `SELECTION_STAGING`
Date: `2026-09-05`

Gate:

`C03E_KZ_PRODUCTION_DURABLE_CAPABILITY_REPEATED_REAL_ADMISSION_COLLECTION_MODULAR_SOURCE_LAYOUT_RESELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_REPEATED_REAL_ADMISSION_COLLECTION_MODULAR_SOURCE_LAYOUT_RESELECTION`

## 1. Purpose

C03e-KZ is a corrective source-layout reselection after the evidence-closed C03e-KX semantic selection and a guarded C03e-KY materialization attempt that stopped before any source mutation.

KX selected the production-durable repeated real-admission collection caller migration semantics correctly. KY then reconstructed the exact FU predecessor and an additive candidate locally, but the available connected GitHub whole-blob transport did not reproduce the frozen candidate Git blob SHA. The mismatch was detected before any tree, commit, or branch-ref source mutation.

KZ does not change the KX semantic selection. It only reselects a safer modular source layout so the same dormant overload can be materialized without replacing the full existing FU source blob through an unverified transport surface.

## 2. Exact predecessor authority

Repository:

`Gersi365/prw-executor-private`

Exact KX head:

`9dfba4e00e1663346f16a3b1dd2de9e4af7f0574`

Exact KX tree:

`32ce98199486c2ecb3a2a12d654d0bc78fdf9e22`

Exact KX contract blob:

`0427b0931ed4729df29daaee6ee8b00c678d0665`

KX PR #435 remains draft/open/unmerged and evidence-closed.

Exact KX FU blob:

`be83931ba1fd5698c1a007e785bdfe6140498e4e`

## 3. Guarded KY non-materialization finding

KY branch:

`phase-152-c03e-ky-production-durable-capability-repeated-real-admission-collection-caller-migration-source-materialization`

The branch was created from exact KX head but no source commit was attached.

At the stop boundary, KY still pointed exactly to:

`9dfba4e00e1663346f16a3b1dd2de9e4af7f0574`

and its FU path still resolved to exact KX blob:

`be83931ba1fd5698c1a007e785bdfe6140498e4e`

Unreferenced Git blob objects created while testing connector transport are not reachable from the KY branch or any repository tree and are not checkpoint authority.

No force-push, history rewrite, source mutation, PR merge, runtime activation, or repository configuration mutation occurred.

## 4. Exact local source proof from KY attempt

The exact KX FU source was reconstructed byte-for-byte before any attempted materialization.

Reconstructed predecessor:

- bytes: `25328`;
- Git blob SHA: `be83931ba1fd5698c1a007e785bdfe6140498e4e`;
- final newline: present.

The locally frozen one-file KX semantic candidate was additive-only:

- additions: `210`;
- deletions: `0`;
- intended candidate Git blob SHA: `4c9a282389d2a0df896d9d612e9563e43c3d2a8f`.

Because connector-created transport blobs did not equal that SHA, none was attached to KY.

This is a transport-integrity stop, not a source-semantic failure.

## 5. Semantic selection remains KX

The future source still materializes exactly one dormant method:

`RemoteSessionExecutorRuntime::drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_capability(...)`

It preserves the existing repeated real-admission supervisor law and adds exactly one higher-owner:

`Arc<ProductionDurableCapabilityAuthority>`

The existing:

`SharedCurrentCapabilityAuthority<P>`

remains both:

- AJ admission authority; and
- requester-DR authority after worker admission.

The two authority lanes remain explicit and non-substitutable.

## 6. Selected modular source layout

The next source materialization may change exactly two source paths.

### Path A — existing FU parent module

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Selected mutation is only one private child-module declaration, formatted by rustfmt as required:

`mod production_durable_repeated_real_admission_collection;`

No existing import, type alias, helper, legacy repeated collection method, KW durable worker helper, or test body is selected for mutation.

### Path B — new child module

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

This new file owns only the KX-selected dormant production-durable repeated-admission overload.

It may use ancestor-private FU helpers and types through ordinary Rust child-module visibility, preferably with:

`use super::*;`

No new public API is selected.

## 7. Child-module visibility law

The child module remains an implementation detail of FU.

The new method must retain equivalent Agent-internal visibility to the KX-selected surface. An explicit ancestor path such as:

`pub(in crate::remote_session_capability_runtime::remote_session_executor_runtime)`

is acceptable if required by the additional module nesting.

No crate-external visibility expansion is selected.

## 8. Exact repeated supervisor law

The child-module overload must preserve the existing FU collection ordering exactly:

1. reap ready persistent completions first;
2. poll supervisor shutdown;
3. poll expected request only while the source is open and capacity permits;
4. reject duplicate active expected device before timing or AJ work;
5. sample existing admission timing only for a prepared request;
6. start exactly one existing AJ transaction;
7. while AJ is pending, reap ready active workers and poll shutdown before AJ completion;
8. on normal AJ success, derive authenticated `DeviceId` from the returned session owner;
9. create exactly one `RemoteSessionWorkerAdmission<D, T>`;
10. require a vacant authenticated-device slot under the existing preflight invariant;
11. insert exactly one persistent worker entry;
12. on shutdown during AJ, cancel active workers, retain/drain AJ, orderly-close any post-shutdown successful session, then drain active workers.

No retry, fairness, capacity, shutdown, identity, or admission policy change is selected.

## 9. Successful worker insertion law

Only the successful vacant-slot worker constructor differs from the legacy collection.

The new overload must call exactly:

`spawn_recoverable_requester_aware_worker_with_production_durable_capability(...)`

with:

- exact `RemoteSessionWorkerAdmission<D, T>`;
- `Arc::clone(&capability_authority)`;
- the existing shared-current authority as requester-DR authority;
- the existing requester policy source;
- the existing requester/rendezvous authority.

The existing KW helper remains the sole persistent-task constructor on the durable path.

## 10. Outer Arc cloning law

The future overload retains one higher-owner outer:

`Arc<ProductionDurableCapabilityAuthority>`

It may clone only that outer Arc and only for a successfully admitted worker that is actually inserted into a vacant authenticated-device slot.

No durable-authority Arc clone is selected for:

- duplicate expected-device rejection;
- AJ failure;
- shutdown-drained AJ success;
- an occupied-slot invariant failure;
- no worker insertion.

The authority object itself remains non-Clone.

## 11. Admission/requester authority law

`SharedCurrentCapabilityAuthority<P>` remains the authority passed to:

`admit_expected_remote_device_session(...)`

The same shared-current authority is then passed to the KW worker helper only in its requester-DR authority parameter.

It must not become fallback production-durable capability authority.

`ProductionDurableCapabilityAuthority` must not enter AJ or requester-DR.

KZ preserves:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

## 12. Identity law

Authenticated PRW application-session `DeviceId` remains the active worker-map key.

Expected `DeviceId` remains preflight intent only until authenticated AJ success.

Static IP is never identity.

Transport identity/evidence is not logical device identity.

Outer PRWM `request_id` remains correlation only.

Authority custody is not identity.

## 13. Persistent custody law

The new repeated collection overload must not create worker tasks, owner cells, cancellation pairs, or persistent entry types itself.

Those remain owned by the exact KW helper and existing FS persistent-custody layer.

The collection only inserts the exact returned:

`RecoverableRequesterAwareWorkerEntry`

into the existing authenticated-device active map.

## 14. Verifier-time law

The repeated collection overload samples no capability verifier time.

It preserves the verifier-time provider carried through:

`RemoteSessionWorkerAdmission<D, T>`

The lower KS/KQ/KO/KM chain remains verifier-time sampling authority.

Admission timing is not capability verifier time.

## 15. Candidate/publication boundary

Candidate-publication processing remains separately gated and fail-closed through the existing lower typed-ingress chain.

KZ selects no candidate provider execution, reachability continuation, response publication, retry, target dialing, or runtime activation.

## 16. Legacy FU preservation

The existing legacy method:

`drive_recoverable_repeated_real_remote_admission_collection(...)`

must remain textually unchanged except for unavoidable line-number displacement caused by the single child-module declaration.

The existing legacy helper:

`spawn_recoverable_requester_aware_worker(...)`

must remain unchanged.

The KW helper:

`spawn_recoverable_requester_aware_worker_with_production_durable_capability(...)`

must remain unchanged.

Existing FU tests must remain unchanged.

## 17. Exact future source ceiling

The immediate source successor may change only the two paths named in section 6.

The following remain byte-identical unless a new gate is opened:

- KU FQ source;
- KS FI source;
- FS persistent-custody source;
- production durable-authority custody;
- lower KQ/KO/KM/KK/KG source;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests and lockfile;
- workflows;
- Android source.

If compilation requires a third source path, authority-type mutation, executable population, or runtime activation, STOP and open a new gate.

## 18. Source successor proof obligations

A later source checkpoint must prove:

1. exactly two selected source paths changed;
2. parent FU net mutation is only the child-module declaration;
3. new child module contains only the durable repeated-admission overload;
4. legacy FU method body is unchanged;
5. legacy FU helper is unchanged;
6. KW helper is unchanged;
7. existing FU tests are unchanged;
8. AJ still receives only shared-current authority;
9. durable authority is cloned only as outer Arc for successful vacant-slot insertion;
10. KW helper receives the exact Arc clone;
11. requester-DR authority remains shared-current;
12. no authority aggregate or Clone implementation is added;
13. no extra task/channel/queue is added;
14. no verifier-time sample is added at FU;
15. no candidate/reachability/dialing behavior is added;
16. overload remains dormant/uninvoked by executable/runtime source.

## 19. Executable/runtime boundary

KZ does not select where the first production outer durable-authority Arc is created or retained.

It does not select a production aggregate/context or how a higher runtime caller receives that Arc.

It does not modify `run()`, `main.rs`, bootstrap, listener, readiness, deployment, restart, recovery, or service-manager behavior.

Those remain later independently gated decisions.

## 20. Exact source-layout STOP

KZ is selection-only.

Do not materialize either selected Rust path in KZ.

Do not mutate the existing empty KY branch.

Do not attach any previously rejected orphan blob to a branch or tree.

Do not populate executable/runtime durable-authority inputs.

Do not activate runtime/network behavior.

Do not merge, close PRs, mark ready, delete branches, or rewrite history.
