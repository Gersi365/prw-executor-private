# C03e-QA — Production durable reachability fallible verifier-time higher-owner operation selection

Status: `SELECTION — VALIDATION PENDING`

Date: `2026-09-11`

## 1. Purpose

C03e-QA is a documentation-only selection checkpoint above evidence-closed C03e-PZ.

C03e-PZ materialized one dormant Linux production-reachability operation whose verifier-time provider is fallible and whose completion callback receives the bounded C03e-PV projection.

C03e-QA selects the next narrow ownership boundary only: one dormant pre-requester higher-owner operation in `production_durable_capability_higher_owner_custody.rs` that consumes the already-existing same-custody durable reachability owner, delegates exactly once to the C03e-PZ Linux operation, and retains the exact raw `ProductionDurableCapabilityAuthority` until after the delegated operation returns.

QA performs no Rust source mutation, provider installation, expected-request production, requester/rendezvous join, companion assembly, executable invocation, merge, or deployment.

## 2. Canonical repository authority

Repository:

`Gersi365/prw-executor-private`

Repository ID:

`1334911207`

Canonical `main` remains:

`7c993fa93977a0bb84e0d030874eee7fd0cae77f`

Canonical `main` tree:

`63b8e59ca53797fdea6b95432e16f35eaf473604`

C03e-QA does not mutate `main`.

## 3. Exact predecessor C03e-PZ

Predecessor branch:

`phase-152-c03e-pz-linux-production-reachability-fallible-verifier-time-callback-projection-operation-source-materialization`

Exact predecessor head:

`4f1d4e78763f2bb1c775f9ed441af0a4eeb61e31`

Exact predecessor tree:

`7c84728b4ded4ac4889c9920a1e529cf6a8e1cac`

Exact Linux source blob:

`4c6e414e163bcadb5066e575ec418f2e4fe3d8ac`

C03e-PZ PR #565 remains draft/open/unmerged and administratively closed.

Canonical PZ evidence:
- Drive ID `1vNrCF94SMnvq-b-qLNQa6oDFEcpWpmLh`;
- bytes `18018`;
- SHA-256 `917fd4ab83d74b3b8fcdeb004934f0e6cc08fabba7f204d60fa60d8ed652aafa`.

Exact-final-head PZ validation:
- Rust #1770 / run `34597317067` / job `103256001487`: `SUCCESS`;
- Android #1706 / run `34597317187` / job `103256001391`: `SUCCESS`;
- C02f-AD #1020 and C02f-AE #1011: `SKIPPED`.

`SKIPPED` is not PASS.

## 4. Fresh higher-owner source audit

Exact higher-owner path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact C03e-PZ blob:

`5c8943a43f100bcfea2e63056ed368ac3dbdeb2b`

The file already defines the non-cloneable pre-requester same-custody owner:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

It owns exactly:

1. `production_inputs: LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`; and
2. `capability_authority: ProductionDurableCapabilityAuthority`.

The owner is populated by existing production-source helpers and intentionally keeps the production peer/reachability inputs and durable capability authority from one exact same-custody lineage.

The owner is currently consumed only by later requester-policy/requester-rendezvous joining. There is no pre-requester operation wrapper for the C03e-PZ fallible verifier-time lane.

The selected future target function is absent at exact PZ head.

## 5. Durable-authority type audit

Exact source path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact C03e-PZ blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

`ProductionDurableCapabilityAuthority` owns:
- one `Arc<tokio::sync::Mutex<ProductionDurableRegistryRuntimeCustody>>`;
- one concrete fail-closed `ProductionRemoteCapabilityDenyAllPolicy`.

Its source contains an explicit compile-time test that the authority is `Send + Sync`.

The authority exposes only its operation-specific durable authorization method; no generic registry/store extraction seam is exposed.

QA therefore selects lifetime retention only. It does not select an authority clone, Arc adaptation, authorization invocation, registry extraction, or policy reinterpretation.

## 6. Exact PZ lower operation

C03e-PZ materialized in `linux_bootstrap.rs`:

`linux_agent_production_reachability_remote_process_operation_with_fallible_verifier_time_completion_projection(...)`

The operation:
- consumes one `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`;
- returns one `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static`;
- preserves one private executor across production reachability bootstrap, endpoint bind, controller publication and lifecycle drive;
- invokes the C03e-PX production wrapper exactly once;
- preserves the fallible verifier-time provider type;
- preserves the bounded completion projection;
- performs no concrete provider installation/sampling;
- performs no expected-request construction;
- performs no requester/rendezvous or durable-capability fusion.

## 7. Why a higher-owner seam is selected now

Calling the PZ Linux operation directly from the pre-requester same-custody owner would require destructuring away the raw durable capability authority.

Dropping that authority at factory construction would destroy the existing same-custody lifetime relationship before the delegated production operation runs.

The existing historical higher-owner operation pattern already demonstrates the required custody law: build one lower one-shot operation, retain the authority in the returned closure, invoke the lower operation, then release retained authority only after the lower operation returns.

QA selects the same ownership ordering for the distinct pre-requester fallible-verifier-time lane, without importing requester/rendezvous semantics.

## 8. Selected future source ceiling

Immediate future source materialization is ceilinged to exactly one path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

No second Rust path is selected.

The future source stage may add only:

1. the C03e-PZ Linux fallible projection operation to the existing `crate::linux_bootstrap` imports;
2. `RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection` to the existing remote-session capability imports if required by the selected signature; and
3. one dormant crate-private higher-owner operation sibling.

If correctness requires any second source path, the future source checkpoint must STOP and reselect.

## 9. Selected future higher-owner sibling

Selected name:

`linux_agent_production_durable_reachability_remote_process_operation_with_fallible_verifier_time_completion_projection`

Selected visibility:

`pub(crate)`

Selected input owner:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

Selected return:

`impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static`

The future sibling is additive and dormant.

## 10. Selected generic law

The future sibling must preserve:

`P: PolicyEvaluator + Send + Sync + 'static`

`D: CapabilityDispatcher + Send + 'static`

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`

`F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`

`C: FnMut(DeviceId, RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection) + Send + 'static`

`R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>) + Send + 'static`

`E: FnMut(RemoteSessionRepeatedAdmissionFailure) + Send + 'static`

No new generic parameter or error envelope is selected.

## 11. Exact ownership/delegation law

The future sibling must consume the higher-owner input exactly once and destructure exactly:

- `production_inputs`;
- `capability_authority`.

It must construct the C03e-PZ lower operation exactly once using `production_inputs`.

It must return one closure that:

1. invokes the lower PZ operation exactly once with the supplied `LinuxAgentRemoteSupervisorShutdownPublisher`;
2. retains the exact raw `ProductionDurableCapabilityAuthority` across that complete invocation; and
3. releases that exact authority only after the lower operation returns.

The selected reference ordering is conceptually:

```rust
let operation =
    linux_agent_production_reachability_remote_process_operation_with_fallible_verifier_time_completion_projection(
        production_inputs,
    );

move |publisher| {
    operation(publisher);
    drop(capability_authority);
}
```

An exact semantically equivalent ownership form is acceptable only if the compiler proves the authority remains owned through the complete lower invocation.

## 12. No Arc adaptation

The pre-requester owner contains one raw `ProductionDurableCapabilityAuthority`.

QA selects no:
- `Arc::new`;
- `Arc::clone`;
- `Clone` implementation;
- accessor exposing the authority;
- conversion to the requester/rendezvous durable higher-owner aggregate.

The exact raw authority is retained by value in the future one-shot closure.

## 13. No durable authorization invocation

The future higher-owner sibling must not call:

`ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)`

It preserves authority lifetime only.

No registry lock, provider I/O, durable-registry read/write, policy evaluation or capability authorization occurs during factory construction or solely because this wrapper exists.

## 14. Verifier-time provider law

The provider remains request-carried and caller-supplied through generic `T`.

QA does not install:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds`

The future sibling must not:
- invoke the provider;
- sample time;
- preflight provider availability;
- cache time;
- retry;
- default;
- clamp;
- saturate;
- translate `PrwaVerifierSourceError`.

Concrete provider installation remains separately gated with expected-request production.

## 15. Completion and identity law

The future sibling performs no completion mapping.

C03e-PV remains sole bounded fallible completion-classification authority.

Authenticated owner-derived `DeviceId` remains post-auth worker authority and is forwarded unchanged through the existing lower layers.

Expected pre-auth `DeviceId` remains scheduling intent only.

`request_id` remains correlation only.

Durable capability-authority custody is not logical identity and is not substituted for authenticated worker identity.

## 16. Requester/rendezvous separation

QA explicitly rejects requester/rendezvous fusion.

The future sibling must not use or construct:
- `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- `LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- `SharedRequesterRendezvousAuthority`;
- `BoundedRequesterRendezvousStartPolicySource`;
- requester/rendezvous provider/runtime custody;
- requester-aware completion projection.

The existing requester/durable lineage remains unchanged.

## 17. Expected-request producer remains deferred

QA selects no:
- expected-request channel creation;
- sender clone/retention;
- `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- concrete dispatcher construction for a request;
- target admission `SessionId` generation;
- PRWM request-ID generation;
- verifier-time provider installation;
- `sender.send(request).await`;
- producer retry/remint/replay/rollback.

The future higher-owner sibling receives only already-populated typed inputs.

## 18. Runtime and teardown preservation

The future sibling adds no runtime behavior of its own beyond invoking the already-selected lower one-shot operation when the returned closure is called.

It adds no:
- second Tokio runtime;
- runtime handle;
- generic or nested `block_on`;
- endpoint close;
- `wait_idle()`;
- shutdown source;
- detached worker;
- retry loop;
- listener/readiness publication;
- process-signal handling;
- new process-level error envelope.

Existing lower layers remain sole endpoint lifecycle/teardown authority.

## 19. Historical-path preservation

The future source checkpoint must leave unchanged:
- `linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation(...)`;
- `linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(...)`;
- `run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_with_production_durable_capability_projection(...)`;
- all configured requester/rendezvous population helpers;
- the C03e-PZ Linux operation;
- the C03e-PX production endpoint wrapper;
- the C03e-PV endpoint projection adapter;
- production durable authority implementation.

## 20. Companion/executable boundaries deferred

QA does not select:
- a `run_with_remote_process_companion(...)` wrapper for this lane;
- higher-owner companion assembly;
- executable input population;
- callback logging/metrics/process-exit policy;
- `run()` migration;
- `main.rs` mutation;
- listener/readiness activation;
- process-signal integration;
- service/systemd changes;
- deployment.

Those remain separately gated after the higher-owner operation itself is materialized and validated.

## 21. Stop conditions for future source materialization

The future source checkpoint must STOP and reselect if correctness requires:
- any source path beyond `production_durable_capability_higher_owner_custody.rs`;
- mutation of `linux_bootstrap.rs`;
- mutation of the endpoint/runtime projection layers;
- mutation of `prw-session`;
- requester/rendezvous join/fusion;
- `Arc` wrapping or cloning of the raw authority;
- durable authorization invocation;
- concrete verifier-time provider invocation;
- expected-request construction/send;
- new completion mapping;
- broader than `pub(crate)` visibility;
- a second runtime/teardown path;
- companion assembly;
- executable or `main.rs` activation.

No silent widening is authorized.

## 22. C03e-QA branch

Branch:

`phase-152-c03e-qa-production-durable-reachability-fallible-verifier-time-higher-owner-operation-selection`

The branch was created from exact C03e-PZ head:

`4f1d4e78763f2bb1c775f9ed441af0a4eeb61e31`

QA must remain documentation-only.

## 23. Validation law

All QA PASS claims must bind only to the exact final QA head.

Required canonical validation:
- PRW Rust Validation: terminal `SUCCESS`.

Android is claimable only if an exact-head Android run triggers and reaches terminal `SUCCESS`.

Path-filtered `SKIPPED` workflows are not PASS.

## 24. Evidence law

After exact-head validation, publish one immutable raw Markdown audit under canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

Required evidence discipline:
1. exact-title pre-upload search: zero matches;
2. freeze local bytes and SHA-256;
3. one real upload;
4. raw Drive readback;
5. exact byte/hash equality;
6. exact-title postcheck: exactly one canonical artifact;
7. update only the QA PR body with evidence metadata and administrative closure;
8. keep PR draft/open/unmerged.

## 25. Frozen exclusions

C03e-QA performs or authorizes no:
- Rust source materialization in QA itself;
- provider installation;
- expected-request producer;
- requester/rendezvous fusion;
- durable authorization invocation;
- companion assembly;
- executable caller migration;
- listener/readiness/process-signal activation;
- `main.rs` mutation;
- service/systemd mutation;
- DB/schema/control-plane mutation;
- authentication/authorization redesign;
- retry/reconnect/rebootstrap;
- public API widening;
- generic runtime exposure;
- merge;
- deployment;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update;
- reset/rebase/squash/history rewrite;
- destructive cleanup.

## 26. Gate / closure

Gate:

`C03E_QA_PRODUCTION_DURABLE_REACHABILITY_FALLIBLE_VERIFIER_TIME_HIGHER_OWNER_OPERATION_SELECTED`

Expected administrative closure after exact-head CI and immutable evidence:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

After closure: **STOP at C03e-QA**.

Do not create C03e-QB in the same closure.
