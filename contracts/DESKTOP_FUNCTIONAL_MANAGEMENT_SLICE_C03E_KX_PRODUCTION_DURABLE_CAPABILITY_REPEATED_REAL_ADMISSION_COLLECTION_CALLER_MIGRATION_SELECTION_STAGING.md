# C03e-KX Production Durable Capability Repeated Real-Admission Collection Caller Migration Selection Staging

Status: `SELECTION_STAGING`
Date: `2026-09-05`

Gate:

`C03E_KX_PRODUCTION_DURABLE_CAPABILITY_REPEATED_REAL_ADMISSION_COLLECTION_CALLER_MIGRATION_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_REPEATED_REAL_ADMISSION_COLLECTION_CALLER_MIGRATION_SELECTION`

## 1. Purpose

C03e-KX selects the narrowest source successor after evidence-closed C03e-KW.

KW materialized one dormant FU helper:

`spawn_recoverable_requester_aware_worker_with_production_durable_capability(...)`

The helper is validated and preserves exact FS persistent custody, KS FI durable dual-authority execution, requester policy, requester/rendezvous authority, cancellation pair, dispatcher transfer, verifier-time transfer, and recoverable owner-cell semantics.

KX selects only a future additive dormant repeated real-admission collection overload that invokes that KW helper after successful admission.

KX itself is docs-only. It performs no Rust source mutation and no runtime activation.

## 2. Exact predecessor authority

Repository:

`Gersi365/prw-executor-private`

Exact predecessor branch:

`phase-152-c03e-kw-production-durable-capability-recoverable-persistent-requester-aware-worker-entry-spawn-source-materialization`

Exact predecessor head / required merge base:

`b57eb1abf468e0849f8a6dfb5f6d712359632030`

Exact predecessor tree:

`a03267fa93d8c25aec9187f87925be633cbd7f74`

Exact FU predecessor blob for KX selection:

`be83931ba1fd5698c1a007e785bdfe6140498e4e`

KW PR #434 remains draft/open/unmerged and evidence-closed.

## 3. Fresh exact-source finding

The FU file remains:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

The existing repeated collection method remains:

`drive_recoverable_repeated_real_remote_admission_collection(...)`

That existing method owns the full repeated admission supervisor law:

1. recover ready persistent worker completions first;
2. observe supervisor shutdown before accepting more work;
3. enforce active-worker capacity;
4. reject duplicate expected `DeviceId` before timing or AJ;
5. run exactly one in-flight AJ transaction;
6. preserve shutdown-vs-inflight-admission handling;
7. derive the active key from authenticated session-owner `DeviceId`;
8. create one `RemoteSessionWorkerAdmission<D, T>` after AJ success;
9. insert one recoverable persistent requester-aware worker into a vacant active-map slot;
10. request cooperative cancellation and drain exact FS custody on shutdown.

The existing method still calls the legacy:

`spawn_recoverable_requester_aware_worker(...)`

The KW durable helper exists in the same FU file but remains dormant/uninvoked.

## 4. Authority finding

The existing shared-current authority parameter is still required by the AJ admission transaction.

The same shared-current authority lane is also the requester-DR authority consumed by the KS FI durable lifecycle after worker spawn.

The durable capability authority is a distinct lane:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

KX therefore selects no authority replacement.

The future overload must carry both lanes explicitly:

- one outer `Arc<ProductionDurableCapabilityAuthority>` for production durable capability ingress;
- one `&SharedCurrentCapabilityAuthority<P>` for AJ admission and requester DR.

## 5. Selected future method

A later source checkpoint may add exactly one dormant Agent-internal method on:

`RemoteSessionExecutorRuntime`

Selected name:

`drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_capability(...)`

The generic families must remain aligned with the existing FU repeated collection:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> u64 + Send + 'static`;
- `PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static`;
- existing shutdown/timing/completion/rejection/admission-failure generic callbacks unchanged.

The selected parameter surface is the existing FU repeated collection surface plus exactly one durable capability-authority input:

`capability_authority: Arc<ProductionDurableCapabilityAuthority>`

The existing shared-current authority lane remains separately supplied as:

`&SharedCurrentCapabilityAuthority<P>`

No authority aggregate is selected.

## 6. Selected repeated-loop law

The future durable overload must preserve the existing FU loop structure and priority exactly.

It must retain:

- the same capacity validation;
- the same expected-request receiver ownership;
- the same current-thread runtime custody;
- the same active worker map type;
- the same supervisor-shutdown future;
- the same request-source-open state;
- the same ready-completion-first reaping;
- the same shutdown/request polling order;
- the same duplicate-device preflight;
- the same admission timing callback;
- the same one-AJ-at-a-time law;
- the same shutdown-vs-inflight-AJ polling order;
- the same AJ drain-on-shutdown behavior;
- the same authenticated-device-id assertion;
- the same occupied-slot unreachable invariant;
- the same active-worker cancellation and drain semantics.

No loop policy or fairness change is selected.

## 7. Admission authority law

For every AJ transaction, the future durable overload must continue to pass only the existing shared-current authority into:

`admit_expected_remote_device_session(...)`

The production durable capability authority must not enter AJ admission.

Successful AJ remains the only source of authenticated session-owner custody.

Static endpoint information, transport identity, request correlation, task ownership, or durable authority custody must not become logical admission identity.

## 8. Successful-admission worker construction law

After exact AJ success, the future durable overload must preserve the current sequence:

1. derive `authenticated_device_id` from the returned session owner;
2. debug-assert equality with the expected device id;
3. construct one exact `RemoteSessionWorkerAdmission::new(session_owner, dispatcher, verifier_time_unix_seconds)`;
4. acquire the active-map entry keyed by authenticated device id;
5. require `Entry::Vacant` under the existing single-inflight preflight law;
6. insert exactly one persistent worker entry.

Only step 6 changes constructor selection in the new overload.

Instead of the legacy helper, the future overload must call:

`spawn_recoverable_requester_aware_worker_with_production_durable_capability(...)`

with:

- the exact worker admission;
- `Arc::clone(&capability_authority)`;
- the existing shared-current authority as requester-DR authority;
- the same policy-source Arc;
- the same requester/rendezvous authority.

## 9. Outer Arc cloning law

The repeated collection retains one higher-owner:

`Arc<ProductionDurableCapabilityAuthority>`

For each successfully admitted worker that is actually inserted, it may clone only this outer Arc:

`Arc::clone(&capability_authority)`

That cloned handle moves into the KW persistent worker helper and then into the exact worker task.

No clone of the authority object itself is selected.

No inner registry Arc is exposed.

No durable registry state is copied or reconstructed.

No Arc clone occurs for rejected, failed-admission, occupied-slot, or shutdown-drained transactions.

## 10. Requester-DR authority law

The same shared-current authority that remains AJ admission authority is passed to the KW helper as the requester-DR authority lane.

Inside the KW/KS chain it must remain distinct from durable capability authority.

The future overload must not use shared-current authority as fallback durable capability authority.

The future overload must not use durable capability authority as AJ or requester-DR authority.

## 11. Persistent worker custody law

The future overload must not construct its own task, owner cell, cancellation pair, or persistent entry.

Those responsibilities remain exclusively inside the existing KW helper and FS custody.

The repeated collection receives the returned:

`RecoverableRequesterAwareWorkerEntry`

and inserts it into the existing active map exactly once.

## 12. Verifier-time law

The repeated collection does not sample capability verifier time itself.

It preserves the existing `verifier_time_unix_seconds` provider carried by the admitted request into `RemoteSessionWorkerAdmission`.

The provider then flows through KW -> KS -> KQ -> KO, where the selected durable ingress chain owns per-transaction verifier-time sampling.

No AJ timing or supervisor clock becomes capability verifier time.

## 13. Cancellation and shutdown law

No new cancellation mechanism is selected.

Existing supervisor shutdown continues to:

- stop accepting new request work according to the existing poll order;
- request cancellation on all active persistent entries;
- drain in-flight AJ if shutdown wins while AJ is pending;
- orderly-close a post-shutdown successfully admitted session instead of inserting a worker;
- drain all retained active entries through FS.

Per-worker cancellation remains the existing KW/KS/KQ chain.

## 14. Completion law

Ready worker completions remain reaped before same-wake shutdown/request admission.

Completion publication remains the existing:

`RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`

No new completion or error family is selected.

## 15. Legacy method preservation

The existing:

`drive_recoverable_repeated_real_remote_admission_collection(...)`

must remain available and behaviorally unchanged.

The future durable overload is additive and dormant.

No current caller may be redirected in the immediate source-materialization successor.

## 16. KW helper preservation

The existing KW helper:

`spawn_recoverable_requester_aware_worker_with_production_durable_capability(...)`

must remain unchanged.

The future repeated-collection overload is only its caller.

No duplicate persistent worker spawn logic may be introduced.

## 17. Exact future source ceiling

The immediate source-materialization successor may change at most:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

No second source path is selected.

Guarded unchanged paths include:

- KU FQ source;
- FS persistent-custody source;
- KS FI source;
- production durable-authority custody;
- remote-session parent runtime/error source;
- authenticated-session lower durable ingress chain;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests/lockfile;
- workflows;
- Android source.

If correct compilation requires a second path, STOP and open a fresh gate.

## 18. Future source proof obligations

A future KX source successor must prove at minimum:

1. exactly one FU source path changed;
2. legacy repeated collection remains unchanged;
3. legacy FU helper remains unchanged;
4. KW helper remains unchanged;
5. new overload remains dormant/uninvoked;
6. one higher-owner outer durable-authority Arc is accepted by value;
7. only outer Arc clones are made per successfully inserted worker;
8. shared-current authority remains AJ + requester-DR authority;
9. durable authority never enters AJ;
10. shared-current authority never becomes durable capability fallback;
11. existing request/shutdown/admission polling order is preserved;
12. existing capacity and duplicate-device laws are preserved;
13. existing authenticated-device-id active-map law is preserved;
14. existing FS cancellation/reaping/drain law is preserved;
15. no task/channel/queue/global authority state is added by the new overload;
16. no executable/runtime caller migration occurs.

## 19. Higher-owner/executable boundary

KX does not select where the first production outer:

`Arc<ProductionDurableCapabilityAuthority>`

is constructed or retained.

KX does not select how that handle reaches a production executor/bootstrap caller.

KX does not mutate `run()`, `main.rs`, Linux bootstrap, production aggregate ownership, listener/readiness state, or network activation.

Those remain later separately gated decisions.

## 20. Candidate-publication boundary

Candidate publication remains separately gated and fail-closed through the existing lower typed-ingress chain.

KX selects no candidate provider execution, reachability continuation, endpoint publication, retry or target dialing.

## 21. Production policy boundary

No positive production capability grant is selected.

The existing production durable authority remains coupled to the concrete deny-all policy baseline.

Caller migration does not imply authorization.

## 22. Explicit exclusions

KX does not perform or authorize:

- source materialization of the new repeated collection overload;
- mutation of the existing repeated collection;
- executable/runtime durable-authority population;
- construction-location selection for the first production outer durable-authority Arc;
- `Clone` on `ProductionDurableCapabilityAuthority`;
- private inner Arc exposure;
- authority aggregate/context creation;
- requester-DR authority replacement;
- AJ authority replacement;
- FQ/FS/FI mutation;
- lower KQ/KO/KM/KK/KG mutation;
- positive production grants;
- candidate provider execution;
- reachability/dialing;
- peer reuse/restart/reconnect;
- listener/readiness/network activation;
- `run()`/`main.rs` mutation;
- manifest/lockfile/workflow/Android-source mutation;
- deployment/restart/recovery;
- database/schema/control-plane mutation;
- repository configuration/visibility mutation;
- merge;
- PR close;
- ready-for-review conversion;
- branch deletion;
- history rewrite.

## 23. STOP boundary

STOP after C03e-KX selection closure.

Do not materialize the selected repeated collection overload in KX.

The immediate source successor requires a separately created branch based on the exact final KX head and must obey the one-FU-path source ceiling above.