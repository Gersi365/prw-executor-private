# C03e-QC — Production durable reachability fallible verifier-time higher-owner companion assembly selection

Status: `SELECTION — VALIDATION PENDING`

Date: `2026-09-11`

## 1. Scope

C03e-QC is a documentation-only selection checkpoint above evidence-closed C03e-QB.

C03e-QB materialized one dormant crate-private pre-requester higher-owner operation that retains the exact raw `ProductionDurableCapabilityAuthority` across the complete C03e-PZ Linux fallible-verifier-time operation invocation.

C03e-QC selects the next narrow boundary only: one dormant higher-owner companion-assembly sibling in the same higher-owner source file. It performs no Rust/source/runtime mutation itself.

## 2. Exact predecessor authority

Repository: `Gersi365/prw-executor-private`.

Closed C03e-QB branch:

`phase-152-c03e-qb-production-durable-reachability-fallible-verifier-time-higher-owner-operation-source-materialization`

Exact QB head:

`9d76b24b2138d918110090c05a6b133bc99803bb`

Exact QB tree:

`8a2c4380eda42d5b6f1badf1abdca645218315fe`

Exact higher-owner source blob:

`ca9dfef2794859b9cb772cac7e59c9a33329168b`

PR #567 remains draft/open/unmerged and evidence-closed.

Immutable QB audit:
- Drive ID `1CKdYkPscqqqVbJDNWlbxNFNSnDsyplQQ`;
- bytes `17118`;
- SHA-256 `09b1c4bd2dbc6fa925781cf9393fc24f5b6f1712478bfef161443f7349ceb576`.

Exact-final-head QB validation:
- PRW Rust Validation #1772 / run `34602932918` / job `103274381859`: `SUCCESS`;
- PRW Android Validation #1707 / run `34602933008` / job `103274382071`: `SUCCESS`;
- C02f-AD #1022 and C02f-AE #1013: `SKIPPED`.

`SKIPPED` is not PASS.

## 3. Fresh source finding

Exact QB source:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

already contains:

`linux_agent_production_durable_reachability_remote_process_operation_with_fallible_verifier_time_completion_projection(...)`

The existing sibling consumes one:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

by value, retains the exact raw `ProductionDurableCapabilityAuthority`, constructs the C03e-PZ lower Linux operation exactly once, invokes it exactly once inside the returned one-shot closure, and releases the raw authority only after lower return.

The same exact source file already imports:
- `LinuxAgentBootstrapStartFailure`;
- `LinuxAgentBootstrapWithRemoteReport`;
- `run_with_remote_process_companion`.

The same source already contains the historical higher-owner companion assembly:

`run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_with_production_durable_capability_projection(...)`

That established seam composes one higher-owner operation with existing `run_with_remote_process_companion(...)` without mutating `linux_bootstrap.rs`, `run()`, or `main.rs`.

## 4. Historical layering precedent

Closed C03e-LW selected a one-file dormant higher-owner companion assembly only after the corresponding higher-owner operation had been materialized.

Closed C03e-LX then materialized that companion assembly in the same higher-owner source file and still did not add a concrete production caller, executable entrypoint migration, configured-source population, or runtime activation.

C03e-QC follows that layering discipline for the distinct fallible-verifier-time lane.

## 5. Selected immediate future source ceiling

The immediate later source materialization is ceilinged to exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

No second Rust/source path is selected.

The future source stage may add only one additive dormant crate-private companion-assembly sibling plus only mechanical import changes if exact compilation proves they are required. Because the exact QB source already imports the existing report, start-failure and generic companion runner symbols, no import change is expected.

Any requirement for a second source path requires STOP and reselection.

## 6. Selected future companion sibling

Selected name:

`run_with_production_durable_reachability_remote_process_companion_with_fallible_verifier_time_completion_projection`

Selected visibility:

`pub(crate)`

Selected input:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

Selected return:

`Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>`

## 7. Exact delegation law

The future companion sibling must:

1. accept exactly one existing pre-requester same-custody input aggregate by value;
2. invoke the existing C03e-QB higher-owner operation exactly once;
3. receive exactly one one-shot operation from that call;
4. pass that operation directly to existing `run_with_remote_process_companion(...)` exactly once;
5. return the exact existing `Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>` unchanged;
6. remain dormant and crate-private;
7. add no invocation site in the same source checkpoint.

Selected conceptual body:

```rust
let operation =
    linux_agent_production_durable_reachability_remote_process_operation_with_fallible_verifier_time_completion_projection(
        inputs,
    );
run_with_remote_process_companion(operation)
```

No additional wrapper, match, retry, result translation, error variant, report projection, logging policy, callback policy, or teardown is selected.

## 8. Generic law

The future companion must preserve the QB generic families:

`P: PolicyEvaluator + Send + Sync + 'static`

`D: CapabilityDispatcher + Send + 'static`

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`

`F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`

`C: FnMut(DeviceId, RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection) + Send + 'static`

`R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D,T>) + Send + 'static`

`E: FnMut(RemoteSessionRepeatedAdmissionFailure) + Send + 'static`

No generic family may be broadened merely for companion assembly.

## 9. Durable-authority custody preservation

The future companion must not inspect, clone, wrap, extract, authorize with, or otherwise adapt the raw durable authority.

The exact authority lifetime remains owned solely by the existing C03e-QB operation.

The future companion therefore adds no:
- `Arc::new`;
- `Arc::clone`;
- authority accessor;
- `ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)` call;
- durable-registry lock/provider operation;
- policy evaluation;
- alternate durable authority.

## 10. Verifier-time provider preservation

The verifier-time provider remains request-carried through generic `T`.

The concrete production-capable source remains:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds() -> Result<u64, PrwaVerifierSourceError>`

C03e-QC does not install it.

The future companion must not sample, preflight, invoke, cache, default, clamp, saturate, retry, translate, or otherwise alter verifier-time acquisition.

Concrete provider installation remains separately gated with expected-request production.

## 11. Completion and identity preservation

The future companion performs no completion mapping.

C03e-PV remains sole bounded completion classifier with variants:
- `Cancelled`;
- `VerifierTimeFailure`;
- `TransactionFailure`;
- `AbnormalTaskCompletion`.

Authenticated owner-derived `DeviceId` remains post-auth worker authority.

Expected pre-auth `DeviceId` remains scheduling intent.

`request_id` remains correlation only.

Durable capability-authority custody is not identity or authentication authority.

## 12. Requester/rendezvous separation

The future companion must not construct, consume or convert into the separate requester/rendezvous durable lane.

It must not use:
- `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- `LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- `SharedRequesterRendezvousAuthority`;
- requester policy source/provider/runtime custody;
- requester-aware completion projection.

The historical requester/durable companion assembly remains independent and unchanged.

## 13. Expected-request production remains deferred

Neither QC nor the selected future companion may:
- create the expected-request channel;
- retain/clone a sender;
- construct `RemoteSessionExpectedDeviceAdmissionRequest`;
- generate target admission `SessionId`;
- generate PRWM expected-device request IDs;
- construct a concrete request dispatcher;
- install the concrete verifier-time source;
- call `sender.send(request).await`;
- add retry/remint/replay/rollback.

The companion receives only already-populated typed inputs.

## 14. Runtime and teardown preservation

The future companion must reuse existing `run_with_remote_process_companion(...)` unchanged.

It adds no:
- second Tokio runtime;
- generic runtime handle;
- nested/generic `block_on`;
- endpoint close;
- `wait_idle()`;
- alternate shutdown source;
- detached task;
- retry/reconnect loop;
- new bootstrap report;
- new startup error envelope.

Existing lower layers remain sole runtime/lifecycle/teardown authority.

## 15. Configured-source population remains deferred

QC explicitly does not select composition with:

`run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_from_configured_production_sources(...)`

or any new fallible-verifier-time configured-source analog.

A configured-source population seam must be separately selected only after the companion assembly itself is materialized and validated.

## 16. Executable activation remains deferred

QC does not select:
- `run()` migration;
- `main.rs` mutation;
- executable callback/logging/metrics/process-exit policy;
- listener/readiness activation;
- process-signal integration;
- service/systemd mutation;
- deployment.

No invocation site is selected by QC.

## 17. Historical paths preserved

The future source materialization must leave unchanged:
- the C03e-QB higher-owner operation;
- historical requester/rendezvous higher-owner operations;
- historical requester/durable companion assembly;
- configured-production population helpers;
- C03e-PZ Linux operation;
- C03e-PX production wrapper;
- C03e-PV completion projection;
- `run_with_remote_process_companion(...)` itself.

## 18. Source stop conditions

The later source checkpoint must STOP and reselect if correctness requires:
- more than `production_durable_capability_higher_owner_custody.rs`;
- `linux_bootstrap.rs` mutation;
- `main.rs` or `run()` mutation;
- endpoint/runtime projection mutation;
- `prw-session` mutation;
- requester/rendezvous fusion;
- provider invocation or installation;
- expected-request construction/send;
- durable authorization invocation;
- new completion mapping;
- new error/report type;
- broader than `pub(crate)` visibility;
- second runtime/teardown;
- configured-source population;
- executable invocation.

No silent widening is authorized.

## 19. Exact QC topology law

C03e-QC itself must remain documentation-only:
- one commit;
- exactly one contract path;
- zero Rust/source/runtime/workflow/manifest/lockfile/Android/packaging/executable changes.

## 20. Validation law

All PASS claims must bind only to the exact final QC head.

Rust validation must reach terminal `SUCCESS`.

Path-filtered `SKIPPED` workflows must be reported as `SKIPPED`, not PASS.

Android may be claimed only if an exact-final-head Android run actually triggers and reaches terminal `SUCCESS`.

No predecessor PASS may be inherited as exact-head validation.

## 21. Immutable evidence law

After exact-head validation:
1. freeze one local raw Markdown audit artifact;
2. record exact byte count and SHA-256;
3. exact-title canonical-parent Drive precheck must return zero;
4. upload exactly once;
5. fetch raw Drive bytes;
6. verify byte count and SHA-256 exact;
7. exact-title postcheck must return exactly one canonical artifact;
8. update only the QC PR body with evidence metadata and closed administrative status;
9. keep the PR draft/open/unmerged;
10. verify the immediate successor namespace remains absent;
11. STOP.

Canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

## 22. Frozen exclusions

QC performs no source materialization itself and does not authorize provider installation, expected-request production, requester/rendezvous fusion, durable authorization invocation, executable population, configured-source population, runtime activation, listener/readiness/process-signal changes, service/systemd changes, DB/schema/control-plane changes, authentication redesign, retry/reconnect/rebootstrap, public API widening, generic runtime exposure, merge, deployment, ready-for-review conversion, PR closure, branch deletion, force update, reset/rebase/squash/history rewrite, or destructive cleanup.

## 23. Gate

Gate:

`C03E_QC_PRODUCTION_DURABLE_REACHABILITY_FALLIBLE_VERIFIER_TIME_HIGHER_OWNER_COMPANION_ASSEMBLY_SELECTED`

Expected administrative closure after exact-head validation and immutable evidence:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

After QC closure: **STOP at C03e-QC**.

Do not create C03e-QD in the same closure.
