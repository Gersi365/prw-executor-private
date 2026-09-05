# C03e-LU — Production Durable Capability Higher-Owner Callback Projection Caller Migration Selection

Status: `SELECTION — VALIDATION_PENDING`
Date: `2026-09-05` (Europe/Tirane)
Repository: `Gersi365/prw-executor-private`

## 1. Gate and checkpoint boundary

Gate:

`C03E_LU_PRODUCTION_DURABLE_CAPABILITY_HIGHER_OWNER_CALLBACK_PROJECTION_CALLER_MIGRATION_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_HIGHER_OWNER_CALLBACK_PROJECTION_CALLER_MIGRATION_SELECTION`

C03e-LU is documentation-only selection. It performs no Rust/source/runtime mutation.

The checkpoint selects only the immediate higher-owner caller adaptation above evidence-closed C03e-LT. It does not select executable population, startup callback policy, runtime activation, merge, deployment, restart/recovery, or repository configuration mutation.

## 2. Authority order

For this checkpoint, authority remains:

1. exact GitHub repository/ref/file state;
2. exact-final-head CI validation;
3. immutable Google Drive audit after byte/hash readback;
4. ChatGPT Project Sources for continuity only.

Any mismatch against live GitHub state requires STOP and re-audit before source materialization.

## 3. Integrated main guard

Fresh pre-selection integrated `main` remained:

- head: `7c993fa93977a0bb84e0d030874eee7fd0cae77f`
- tree: `63b8e59ca53797fdea6b95432e16f35eaf473604`
- commit: `Restore main after accidental connector file mutation`

C03e-LU is staged independently and does not modify `main`.

## 4. Exact predecessor

C03e-LT branch:

`phase-152-c03e-lt-linux-production-requester-rendezvous-durable-callback-projection-operation-source-materialization`

Exact LT head / required merge base:

`9ee799bd5322a5cb4d4eef5fe9d64f4a9cd2b00b`

Exact LT tree:

`e3d4e4dd69da7576aed7ebbd0d6f2cf0e2698823`

Exact LT Linux bootstrap blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

C03e-LT is evidence-closed, draft/open/unmerged, and materializes the dormant projection-capable Linux operation:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(...)`

Fresh namespace audit immediately before LU creation found no existing `phase-152-c03e-lu*` branch.

## 5. Fresh higher-owner source audit

Exact current higher-owner path at LT:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact predecessor blob:

`9ab6023eee49e5987f34409d3f37e63d753d73bd`

The module already owns the non-cloneable aggregate:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

with exactly two custody fields:

1. `requester_rendezvous_inputs` — the existing production/reachability/requester-rendezvous aggregate by value;
2. `capability_authority` — one outer `Arc<ProductionDurableCapabilityAuthority>`.

The existing constructor consumes one raw `ProductionDurableCapabilityAuthority` and performs exactly one outer:

`Arc::new(capability_authority)`

No Clone/Copy/accessor/extraction seam is present.

## 6. Existing legacy higher-owner operation

The same higher-owner module currently defines:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation(...)`

Its current callback bounds remain the legacy completion/rejection/admission-failure surface:

- `FnMut(RemoteSessionRegisteredWorkerCompletion)`;
- `FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>)`;
- `FnMut(RemoteSessionRepeatedAdmissionFailure)`.

It destructures the higher-owner aggregate, calls exactly the legacy Linux requester/rendezvous production operation:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation(requester_rendezvous_inputs)`

and retains the durable authority only through a wrapper closure whose final action is:

`drop(capability_authority)`

The legacy function remains valid for its existing dormant surface and is not selected for mutation or deletion.

## 7. LT projection-capable operation evidence

Exact LT Linux operation:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(...)`

accepts:

1. the same `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>` aggregate;
2. one explicit `Arc<ProductionDurableCapabilityAuthority>` by value.

Its callback surface is already the bounded projection-capable shape:

- completion: `FnMut(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)`;
- rejection: `FnMut(RemoteSessionExpectedDeviceAdmissionRejectionReason, RemoteSessionExpectedDeviceAdmissionRequest<D,T>)`;
- admission failure: `FnMut(DeviceId, RemoteSessionRealAdmissionError)`.

LT consumes the requester/rendezvous runtime owner once into `SharedRequesterRendezvousAuthority`, wraps the bounded requester policy source once in `Arc`, preserves production bootstrap/bind/controller-publication ordering, and delegates exactly once to the production durable callback projection lifecycle.

Because LT takes the durable capability-authority `Arc` by value and moves it into the returned operation closure/lifecycle composition, a higher owner can transfer its retained outer Arc directly without an additional retention wrapper.

## 8. Selected immediate source successor

A later source-materialization checkpoint may change exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

No second source path is selected.

The source successor may add one additive dormant crate-private sibling conceptually named:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(...)`

The sibling must consume exactly one existing:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

by value.

It must destructure that aggregate into exactly:

- `requester_rendezvous_inputs`;
- `capability_authority`.

It must call exactly once:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(requester_rendezvous_inputs, capability_authority)`

and return the resulting one-shot operation directly.

## 9. Selected ownership law

The future sibling must preserve the existing higher-owner outer-Arc custody law while transferring ownership into LT:

- no new `Arc::new`;
- no `Arc::clone`;
- no explicit `drop(capability_authority)` wrapper around the LT operation;
- no accessor or extraction API;
- no Clone/Copy implementation;
- no authority aggregate redesign.

The one outer Arc created by the existing higher-owner constructor moves by value into the LT operation, which owns it for the delegated operation lifetime.

The existing legacy higher-owner operation and constructor remain source- and behavior-unchanged.

## 10. Selected callback law

The future higher-owner sibling must use exactly the LT projection-capable callback families:

### Completion

`FnMut(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)`

The bounded completion projection remains exactly the already-materialized families:

- `Cancelled`;
- `IngressFailure`;
- `RequesterResponseFailure`;
- `AbnormalTaskCompletion`.

No requester-private error payload is widened or reconstructed.

### Rejection

`FnMut(RemoteSessionExpectedDeviceAdmissionRejectionReason, RemoteSessionExpectedDeviceAdmissionRequest<D,T>)`

The rejection reason and untouched expected request are forwarded unchanged.

### Admission failure

`FnMut(DeviceId, RemoteSessionRealAdmissionError)`

Authenticated `DeviceId` and the existing real-admission error are forwarded unchanged.

C03e-LU selects no new logging, metrics, process-exit, restart, retry, reconnect, or callback translation policy.

## 11. Authority lanes remain distinct

The future sibling must preserve distinct authority roles:

1. `SharedCurrentCapabilityAuthority<P>` remains the existing shared-current/requester-DR capability authority lane carried inside the production inputs;
2. `Arc<ProductionDurableCapabilityAuthority>` remains the explicit production durable capability-authority lane;
3. `SharedRequesterRendezvousAuthority` remains requester/rendezvous runtime authority inside LT;
4. authenticated `DeviceId` remains logical/session identity;
5. bind address remains transient reachability data;
6. PRWM `request_id` remains transaction correlation only.

No identity, authorization, transport, or reachability role is collapsed.

## 12. One-file source ceiling

The immediate later source checkpoint is ceilinged to exactly:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Allowed edits are limited to:

- imports required for the LT projection-capable function and callback types;
- one additive dormant crate-private higher-owner sibling described above;
- a narrowly required mechanical lint allowance only if exact source CI requires it and only if it preserves this selected semantic surface.

If source materialization requires any second path, existing legacy-function mutation, visibility widening, callback-policy composition, or executable caller change, STOP and open a new selection gate.

## 13. Explicitly frozen source

The immediate source successor must not modify:

- `crates/prw-agent/src/linux_bootstrap.rs` — exact LT blob `7940a69e598355176a61b0bef5c7571dab9fb530`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`;
- durable registry/provider/bootstrap modules;
- requester/rendezvous provider/population modules;
- remote-session executor/endpoint modules;
- manifests or lockfiles;
- workflows;
- Android source or packaging;
- systemd/credential/certificate/private-key/trust/RBAC configuration.

## 14. Executable/runtime boundary remains closed

C03e-LU does not select a real caller of the future higher-owner sibling.

It does not select:

- executable aggregate population;
- production durable capability-authority bootstrap invocation at a real executable owner;
- callback logging/counter/exit policy;
- startup error composition;
- `run()` or `main.rs` migration;
- listener/readiness activation;
- bind or network activation;
- restart/recovery behavior.

After future one-file source materialization closes, another fresh exact-head audit is required before selecting any executable or runtime boundary.

## 15. Security and product invariants

Continue to preserve:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Do not reinterpret endpoint reachability as identity or authorization.

Do not treat decode, lookup, correlation, transport connection, or target resolution as capability authorization.

No new cryptographic primitive, key handling, trust relationship, privilege transition, filesystem authority, forwarding authority, DNS dependency, or database/control-plane mutation is selected.

## 16. Validation requirements for LU

C03e-LU itself is docs-only.

Closure requires exact-final-head CI tied only to the final LU head.

Required evidence classification:

- `PRW Rust Validation`: must complete successfully on exact final LU head;
- path-filtered workflows: classify `SKIPPED` as skipped, never PASS;
- Android: do not claim PASS unless an exact-head Android workflow actually runs successfully.

A successful workflow tied to LT or any superseded LU head does not validate final LU.

## 17. Evidence requirements

After exact-final-head CI succeeds, publish one immutable Markdown audit to the canonical Google Drive evidence folder:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The evidence publication must record:

- exact final LU head/tree;
- exact LT predecessor/merge base;
- exact one-doc-path diff;
- exact target source blobs audited for the selected successor;
- exact-final-head workflow run/job IDs and conclusions;
- integrated `main` guard;
- successor namespace guard;
- Drive ID, exact bytes, SHA-256, final LF, and raw readback verification.

The audit becomes immutable after verified publication.

## 18. Repository mutation exclusions

C03e-LU does not authorize:

- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update/history rewrite;
- repository visibility/configuration mutation;
- deployment;
- restart/recovery activation;
- destructive cleanup.

The LU PR must remain draft/open/unmerged.

## 19. STOP boundary

C03e-LU selects only the one-file higher-owner callback-projection caller adaptation described above.

After LU closure: **STOP**.

A later source checkpoint may materialize only that selected one-file dormant higher-owner sibling after a fresh namespace/head/source audit. It must not simultaneously activate an executable caller, populate production runtime inputs, add callback process policy, merge, deploy, restart, or cross another architectural/security boundary.
