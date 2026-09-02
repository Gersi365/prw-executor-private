# C03e-IJ — Production Reachability Requester/Rendezvous Process-Companion Executable Assembly Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_IJ_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_EXECUTABLE_ASSEMBLY_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_EXECUTABLE_ASSEMBLY_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-II head:

`30862f5b928cf88792edf473ed3787d54c937164`

Exact predecessor tree:

`37bd725077e65dbdb0e88a82cf4564bd67322450`

Exact predecessor `crates/prw-agent/src/linux_bootstrap.rs` blob:

`99eaa8f9343845036291297dd87a387f0d51aef8`

II gate:
`C03E_II_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_CUSTODY_JOIN_SOURCE_MATERIALIZED`

II closure:
`CLOSED_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_CUSTODY_JOIN_SOURCE_MATERIALIZATION`

II immutable Drive audit:
`1xSNlA6XrxiRjivZaU-9-zSOyksjhkAKi`

II materializes the IH-selected production requester/rendezvous custody join as a dormant crate-private process operation. It explicitly leaves executable assembly separately gated.

## 2. Why IJ is the next bounded seam

The exact II source now exposes two already-compatible boundaries in `linux_bootstrap.rs`:

1. `linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs)` returns the existing one-shot remote process-operation shape; and
2. `run_with_remote_process_companion(operation)` already accepts that exact one-shot shape.

II deliberately does not compose those boundaries.

IJ therefore selects only the smallest executable-assembly composition between the already-materialized II operation factory and the already-existing companion runner.

IJ selects no executable caller, no production input-source construction, no requester/rendezvous behavior invocation and no runtime activation.

## 3. Existing II operation evidence

At the exact II head, `linux_bootstrap.rs` contains the crate-private input owner:

```text
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
> {
    production_inputs:
        LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source:
        BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_runtime_owner:
        CandidatePublicationRequesterRendezvousRuntimeOwner,
}
```

and the crate-private factory:

```text
linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs)
 -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

Factory construction remains side-effect free. Requester/rendezvous custody values are retained in the returned operation and released without requester/rendezvous invocation before delegation exactly once to the existing production reachability operation.

The II source comment explicitly states that this materialization occurs before separately gated executable assembly.

## 4. Existing companion runner evidence

At the exact II head, `linux_bootstrap.rs` already contains:

```text
run_with_remote_process_companion<F>(operation: F)
 -> Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>
where
    F: FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

This runner is the existing process-companion execution boundary. It accepts exactly the operation shape returned by the II factory.

IJ does not modify the runner, its report/failure types, local lifecycle semantics, remote process-thread semantics or shutdown-controller publication behavior.

## 5. Existing executable remains local-only

At the exact II head:

- `run()` still enters only the existing local signal-aware production runtime path; and
- `main.rs` still performs its existing preflight and calls `linux_bootstrap::run()`.

Neither path invokes `run_with_remote_process_companion(...)`.

IJ preserves this state exactly.

## 6. Selected assembly wrapper

IJ selects one future crate-private wrapper in `crates/prw-agent/src/linux_bootstrap.rs` with this responsibility:

```text
run_with_production_reachability_requester_rendezvous_remote_process_companion(inputs)
 -> Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>
```

The wrapper consumes exactly one:

```text
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
>
```

and performs only this composition when a future separately gated caller invokes the wrapper:

```text
inputs
 -> linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs)
 -> run_with_remote_process_companion(operation)
```

The wrapper must preserve the generic bounds already required by the II factory. It introduces no new authority type, runtime type, report type or failure type.

## 7. Exact delegation law

The selected wrapper must:

1. consume the II typed input owner by value;
2. construct the II operation exactly once;
3. pass that exact operation exactly once to the existing `run_with_remote_process_companion(...)` runner;
4. return the runner result unchanged.

It must not duplicate production reachability bootstrap logic, requester/rendezvous custody logic, local runtime logic or remote companion lifecycle logic.

There is no fallback to the older public two-role operation and no alternate runner.

## 8. Source materialization remains dormant

A future source successor may define the selected wrapper without creating an executable caller for it.

Merely compiling an uncalled crate-private wrapper does not select invocation from `run()`, `main.rs`, systemd startup or any other executable entrypoint.

The first source successor must therefore leave the wrapper uncalled by production executable paths.

Any change that makes the wrapper reachable from `run()`, `main.rs` or service startup is a separate activation boundary and is not authorized by IJ.

## 9. No production input-source assembly

IJ selects composition of already-typed II inputs only. It does not select how real executable values populate those inputs.

Still separately gated are all concrete executable sources for:

- `PeerConnectivityIdentity`;
- remote bind address;
- dispatcher;
- timing policy;
- capability authority;
- session authentication service;
- expected-device admission request construction;
- remote-process callbacks;
- requester/rendezvous start-policy population;
- requester/rendezvous runtime-provider construction/population;
- provider capacity or lifecycle provenance.

The future assembly wrapper receives the already-constructed typed owner; it does not manufacture those values.

## 10. No requester/rendezvous behavior activation

IJ does not select any requester/rendezvous method invocation.

The selected wrapper may only obtain the II operation and pass it to the existing companion runner when a later caller invokes the wrapper. It does not add requester/rendezvous work before, around or after that delegation.

IJ therefore does not select:

- requester-aware policy evaluation;
- requester/rendezvous registry validation;
- provider registration;
- current-grant selection;
- provider lifecycle cleanup;
- target-intent construction or ingestion;
- requester/rendezvous DR execution;
- candidate-publication authorization or activation.

Those boundaries remain separately gated.

## 11. Production operation remains authoritative

The selected assembly wrapper must use the exact II factory rather than reconstructing its nested IG production operation.

Therefore the established production order remains owned by the existing production operation:

```text
one RemoteSessionExecutorRuntime
 -> production reachability bootstrap
 -> same-executor production endpoint startup
 -> exact shutdown-controller publication
 -> production endpoint lifecycle drive with durable production-owner custody retained
```

IJ neither changes nor restates this sequence as new wrapper-owned behavior.

## 12. Companion runner remains authoritative

The selected wrapper must use the existing `run_with_remote_process_companion(...)` boundary rather than duplicating local bootstrap, remote thread spawn, lifecycle join or report assembly.

Its existing result remains authoritative:

```text
Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>
```

No new executable result envelope, retry result, fallback result or background-runtime abstraction is selected.

## 13. Existing public and requester-only paths remain frozen

IJ selects no mutation to:

```text
LinuxAgentRemoteProcessOperationInputs
linux_agent_remote_process_operation(...)
LinuxAgentRequesterRendezvousRemoteProcessOperationInputs
linux_agent_requester_rendezvous_remote_process_operation(...)
LinuxAgentProductionReachabilityRemoteProcessOperationInputs
linux_agent_production_reachability_remote_process_operation(...)
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs
linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)
run_with_remote_process_companion(...)
```

The future assembly wrapper remains crate-private and additive.

## 14. Identity invariants

IJ preserves the established identity law:

- `PeerConnectivityIdentity` is logical connectivity identity, not a fixed IP address;
- dynamic IP/port is transient reachability only;
- bind address is not identity;
- requester identity comes only from authenticated application-session custody in the existing requester/rendezvous chain;
- requester/rendezvous target identity remains explicit logical `DeviceId` subject to the existing validation/authorization chain;
- repeated-admission `expected_device_id` is not rendezvous target identity;
- candidate publisher identity is not rendezvous target identity;
- `SessionId`, transport identity, request IDs, PRWM `request_id`, endpoint addresses and candidate addresses are not substitute logical device identity;
- PRWM `request_id` remains correlation only.

The assembly boundary derives no identity from any endpoint or transport coordinate.

## 15. Authority separation

IJ preserves separate custody for:

1. generic capability authority;
2. session authentication service;
3. production reachability live-owner authority and durable snapshot custody;
4. requester-aware start-policy source;
5. private requester/rendezvous runtime-provider custody.

The selected wrapper receives only the already-typed II aggregate owner. It exposes no raw provider, policy backing, credential, key, certificate, registry or mutable global authority.

## 16. Failure law

The selected wrapper adds no new failure policy.

When later invoked by a separately gated caller:

- II operation construction remains the exact existing side-effect-free construction;
- `run_with_remote_process_companion(...)` remains authoritative for local bootstrap and remote companion execution/reporting;
- nested production operation failures retain their existing suppression/publication/lifecycle semantics;
- no retry, fallback, re-bootstrap, rebind, replacement runtime or downgrade is introduced.

IJ does not activate any of these failure paths because IJ itself contains no Rust source mutation and no executable invocation.

## 17. First source-successor ceiling

The first source-materialization successor after IJ is authorized to modify exactly one source file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized source change is limited to:

1. one crate-private wrapper named `run_with_production_reachability_requester_rendezvous_remote_process_companion`;
2. input type exactly `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`;
3. construction of the existing II factory exactly once;
4. delegation of the returned operation exactly once to existing `run_with_remote_process_companion(...)`;
5. unchanged propagation of `Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>`;
6. bounded compile/type/no-I/O evidence that does not invoke the wrapper or any production credential/provider/network path.

The source successor must not modify `run()`, `main.rs` or any second source file.

If compilation or testing requires a second source path, public API widening, real input construction, wrapper invocation or executable behavior change, the successor must stop and select a separate checkpoint rather than widen scope.

## 18. Test obligations for first source successor

Validation must prove the wrapper is source-present but executable-dormant.

Tests or compile-time evidence must not invoke the wrapper and must perform no real:

- systemd credential read;
- provider connection or mutation;
- requester/rendezvous policy evaluation;
- registry lookup;
- endpoint bind;
- listener activation;
- candidate publication;
- NAT traversal;
- peer dialing;
- network I/O.

Required evidence is limited to:

1. exact accepted II input-owner type;
2. exact existing companion-runner result type;
3. exact factory-to-runner delegation in source;
4. no caller added from `run()` or `main.rs`;
5. existing public and crate-private operation factories remain source-compatible;
6. workspace validation remains green.

## 19. Explicitly deferred executable activation

IJ does not select:

- construction/population of real production inputs;
- invocation of `run_with_production_reachability_requester_rendezvous_remote_process_companion(...)`;
- invocation of `run_with_remote_process_companion(...)` from `run()`;
- `run()` mutation;
- `main.rs` mutation;
- executable argument/environment/config selection for production inputs;
- requester/rendezvous policy/provider execution;
- target-intent or DR execution;
- readiness/listener publication semantics;
- candidate publication/traversal/dialing;
- systemd `LoadCredential=` wiring;
- credential/certificate/trust/RBAC provisioning;
- package/service configuration;
- service restart or deployment.

Every such boundary remains separately gated.

## 20. Explicit non-authorization

IJ authorizes no:

- Rust source mutation in IJ itself;
- executable caller selection beyond the dormant wrapper boundary;
- production runtime activation;
- requester/rendezvous authorization execution;
- provider or registry mutation;
- public API widening;
- replacement/removal of existing operation factories;
- alternate runner;
- generic runtime/future-driving API;
- new task/thread/background runtime;
- second Tokio runtime;
- retry/fallback/re-bootstrap/rebind;
- readiness publication;
- candidate publication/traversal activation;
- peer dialing;
- durable-owner mutation;
- credential/certificate/trust/RBAC creation or mutation;
- systemd unit/package mutation;
- deployment/restart;
- repository visibility/configuration change;
- merge;
- branch deletion.

The IJ PR must remain draft/open/unmerged after closure.