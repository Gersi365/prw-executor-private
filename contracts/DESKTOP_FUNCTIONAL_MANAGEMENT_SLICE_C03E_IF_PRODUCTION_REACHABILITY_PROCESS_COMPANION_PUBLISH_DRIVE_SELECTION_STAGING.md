# C03e-IF — Production Reachability Process-Companion Publish/Drive Composition Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_IF_PRODUCTION_REACHABILITY_PROCESS_COMPANION_PUBLISH_DRIVE_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_PROCESS_COMPANION_PUBLISH_DRIVE_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-IE head:

`a5197aafd736c488c4b55315bab0f0d1c75a937b`

IE materializes two dormant crate-private production seams:

1. `RemoteSessionExecutorRuntime::bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&self, peer)`;
2. `ProductionReachabilityRuntimeCustody::bind_remote_endpoint_with_executor_from_systemd_credentials(self, executor, bind_addr)`.

IE preserves exactly one remote-session executor across async production bootstrap and supplied-executor endpoint startup. It does not invoke those seams from an executable process path.

Exact IE audit:
`10kEfqr-WU9NwCzk9Cm_lRSX774v9pxov`

## 2. Existing Linux process-operation composition evidence

At the exact IE head, `crates/prw-agent/src/linux_bootstrap.rs` remains blob:

`b0fb368d95f35fb034b7cb51c76510fdfcbd7613`

It already owns the generic private composition helper:

```text
run_remote_process_operation_composition(
    construct_executor,
    bootstrap_authority,
    start_endpoint,
    publish_controller,
    drive_lifecycle,
) -> bool
```

Its locked stage order is:

```text
construct executor
 -> bootstrap authority/custody
 -> start endpoint
 -> publish shutdown controller
 -> drive endpoint lifecycle
```

Failure at executor construction, bootstrap or endpoint startup suppresses every later stage. A controller publication result of `ReceiverGoneShutdownRequested` still proceeds into the lifecycle-drive stage so the endpoint can observe the already-requested orderly shutdown.

The helper is generic over the bootstrap owner and endpoint type; therefore no new generic orchestration primitive is required for production reachability.

## 3. Existing two-role remote operation is frozen

The public existing factory:

```text
linux_agent_remote_process_operation(inputs)
```

currently uses:

```text
RemoteSessionExecutorRuntime::new
 -> RemoteSessionExecutorRuntime::bootstrap_reachability_authority_from_systemd_credentials
 -> RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials
 -> publisher.publish(controller)
 -> RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle
```

This is the existing two-role live-authority path.

IF explicitly selects **no replacement or mutation of this public factory**. Existing callers and behavior remain unchanged. Production reachability must be introduced as a separate crate-private sibling until a later activation checkpoint explicitly selects which operation is supplied to the process companion.

## 4. Existing production endpoint-drive evidence

At the exact IE head, the existing production endpoint wrapper remains at blob:

`4be58d66dddccc03e1f0d932b2805aba524ead0c`

It exposes:

```text
ProductionReachabilityEndpointLifecycleRuntime
    ::drive_repeated_real_remote_admission_endpoint_lifecycle(...)
```

The wrapper consumes itself once and retains `ProductionReachabilityEtcdOwnerCustody` for the complete delegated lower endpoint lifecycle. Durable custody is released only after the lower lifecycle returns following its existing endpoint-close and idle-drain behavior.

No durable-owner mutation is performed by this drive seam.

## 5. Existing IE same-executor evidence

The exact final IE blobs are:

- `production_reachability_runtime_custody.rs`: `ffcddc0253de2b5430be798061ddad8e920a07ac`
- `remote_session_executor_runtime.rs`: `ef370ca500f118bc067097ddb8f5c37ab597b214`

The selected production operation may therefore use exactly:

```text
RemoteSessionExecutorRuntime::new()
 -> executor.bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&peer)
 -> runtime_custody.bind_remote_endpoint_with_executor_from_systemd_credentials(
      executor,
      bind_addr,
    )
 -> ProductionReachabilityEndpointLifecycleRuntime
    + RemoteSessionSupervisorShutdownController
```

No second runtime, generic `block_on`, executor recovery, retry or fallback is required.

## 6. Selected production process-operation input owner

IF selects one new crate-private non-cloneable carrier in `linux_bootstrap.rs`:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    peer: PeerConnectivityIdentity,
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
}
```

The carrier owns exactly:

- one already-typed logical `PeerConnectivityIdentity` by value;
- the existing injected remote-process inputs by value.

It does not own raw endpoint strings, provider clients, credentials, trust material, durable-store handles or runtime handles.

The peer identity is logical identity only. Dynamic IP remains transient reachability and is not promoted into peer identity.

## 7. Selected production process-operation factory

IF selects one new crate-private factory in `linux_bootstrap.rs`:

```text
linux_agent_production_reachability_remote_process_operation(inputs)
 -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

Factory construction is side-effect-free. It only owns typed values and returns a one-shot operation closure.

Credential/provider I/O, endpoint bind and lifecycle work occur only if some later separately gated caller invokes the returned closure.

## 8. Exact selected operation order

When the returned production operation is explicitly invoked, IF selects exactly this sequence through the existing `run_remote_process_operation_composition` helper:

```text
1. RemoteSessionExecutorRuntime::new()

2. exact executor borrow:
   executor.bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&peer)

3. exact executor move + complete production custody move:
   runtime_custody.bind_remote_endpoint_with_executor_from_systemd_credentials(
       executor,
       bind_addr,
   )

4. exact existing controller publication:
   publisher.publish(controller)

5. exact production wrapper drive:
   production_endpoint.drive_repeated_real_remote_admission_endpoint_lifecycle(
       max_active_workers,
       &capability_authority,
       &mut session_authentication,
       expected_requests,
       admission_timing,
       on_completion,
       on_rejection,
       on_admission_failure,
   )
```

This preserves one executor from bootstrap through endpoint ownership and preserves durable production-owner custody through the entire endpoint drive.

## 9. Failure law

IF reuses the existing fail-closed composition law.

### Executor construction failure

- no production bootstrap;
- no endpoint bind;
- no controller publication;
- no lifecycle drive.

### Production bootstrap failure

- existing `ProductionReachabilityCustodyBootstrapError` terminates the operation stage;
- the borrowed executor is dropped when the operation returns;
- no endpoint bind or later stage occurs;
- no two-role fallback or retry occurs.

### Production endpoint startup failure

- existing `ProductionReachabilityEndpointLifecycleStartupFailure` retains reconstructed complete production runtime custody until the failure value is dropped;
- no controller publication occurs;
- no lifecycle drive occurs;
- no rebind, replacement executor or provider re-bootstrap occurs.

The process-operation helper continues to expose only its existing boolean completion/suppression semantics internally; IF selects no new public error surface.

## 10. Controller publication law

IF preserves the existing one-shot shutdown-controller publication behavior.

On `Published`, the production endpoint lifecycle is driven normally.

On `ReceiverGoneShutdownRequested`, the recovered controller has already received orderly shutdown. The operation still enters the production endpoint drive exactly once so the lower lifecycle observes shutdown and completes its existing close + idle-drain path while durable production custody remains retained.

No readiness claim is inferred from controller publication.

## 11. Existing remote-process inputs remain authoritative

The production sibling reuses the exact existing `LinuxAgentRemoteProcessOperationInputs` fields for:

- bind address;
- worker bound;
- capability authority;
- session authentication service;
- expected-device admission requests;
- admission timing;
- worker completion callback;
- expected-device rejection callback;
- repeated-admission failure callback.

IF selects no new producer/source for any of those values.

In particular IF does not select:

- `PRW_REMOTE_BIND_ADDR_ENV` as an executable source;
- a peer-identity source;
- a registry/policy bootstrap source;
- expected-device request production;
- dispatcher construction;
- timing source construction.

Those remain caller responsibilities for a later composition/activation checkpoint.

## 12. Requester/rendezvous custody remains separate

The existing requester/rendezvous wrapper:

```text
LinuxAgentRequesterRendezvousRemoteProcessOperationInputs
```

and its operation factory remain unchanged.

IF does not fold requester-policy/source custody into the production sibling. If production activation requires requester/rendezvous ownership, that join must be separately selected after the production process operation exists.

## 13. No executable activation

IF explicitly does not select any caller from:

- `run_with_remote_process_companion(...)`;
- `run()`;
- `main.rs`;
- signal-aware readiness construction;
- service startup.

The future production operation factory remains dormant unless a later checkpoint constructs it and supplies it to the existing companion facade.

Therefore source materialization of IF does not by itself activate a remote listener in the running Agent.

## 14. Identity and security invariants

IF preserves all current PRW invariants:

- logical peer/device identity is independent of dynamic IP;
- request IDs remain correlation only;
- bind address is not logical identity;
- transport identity remains lower authenticated transport/certificate identity;
- production live-owner authority and durable snapshot owner custody remain role-separated;
- no raw provider client, store, token, private key or trust material escapes;
- no generic runtime handle or arbitrary future driver is exposed;
- durable snapshot protocol semantics remain unchanged.

The only new production-bootstrap identity input is the existing typed `PeerConnectivityIdentity`.

## 15. No durable-owner operation

The selected production process operation treats durable ownership as retained custody only after bootstrap recovery.

It does not invoke:

- `with_owner_mut(...)`;
- `with_owner_mut_async(...)`;
- compare-and-commit;
- candidate publication freshness mutation;
- requester/rendezvous durable mutation;
- a second durable recovery.

## 16. First source-successor ceiling

The first source-materialization successor after IF is authorized to modify exactly one file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized changes are only:

1. one crate-private `LinuxAgentProductionReachabilityRemoteProcessOperationInputs` carrier owning exact typed peer identity + existing remote-process inputs;
2. one crate-private side-effect-free production operation factory;
3. use of the existing `run_remote_process_operation_composition` helper with the exact IE bootstrap/bind seams and IC production lifecycle drive;
4. bounded no-I/O type/ordering tests.

No modification is authorized to:

- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/production_reachability_runtime_custody.rs`;
- `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`;
- `crates/prw-agent/src/production_reachability_custody_bootstrap.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`;
- lower endpoint/transport/process-lifecycle files;
- Cargo/lockfiles;
- workflows;
- systemd units/packages;
- deployment/security material.

If compilation cannot be achieved inside this exact one-file ceiling, stop and select a separate extension checkpoint rather than widening scope.

## 17. Test obligations for first source successor

Tests must perform no real production credential read, provider connection, durable recovery, endpoint bind, listener activation or network I/O.

Required evidence is source/type/injected-composition based:

1. production input carrier consumes typed `PeerConnectivityIdentity` + existing remote inputs by value;
2. production factory returns exact `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` shape;
3. existing public two-role factory remains callable and unchanged;
4. synthetic composition preserves exact executor→production bootstrap→production endpoint→publish→production drive ordering;
5. executor/bootstrap/endpoint failure suppresses all later stages;
6. receiver-gone publication still drives the lifecycle stage once;
7. no generic runtime or durable-owner mutation API is added;
8. existing workspace tests remain green.

## 18. Explicitly deferred activation/assembly

IF does not select:

- construction of the production input carrier from real process configuration;
- loading bind address from environment at executable startup;
- deriving/loading the typed peer identity;
- capability authority/registry/policy construction;
- session authentication source construction;
- expected-device request producer wiring;
- requester/rendezvous policy-source join;
- supplying the production operation to `run_with_remote_process_companion`;
- changing `run()`;
- changing `main.rs`;
- readiness/listener publication semantics;
- candidate publication/traversal activation;
- peer dialing;
- systemd `LoadCredential=` wiring;
- credential/certificate/trust/RBAC provisioning;
- deployment or service restart.

Each remains a separately gated successor decision.

## 19. Explicit non-authorization

IF authorizes no:

- replacement/removal of the existing two-role remote operation;
- public API widening for the new production operation;
- executable production reachability activation;
- `main.rs` or `run()` callsite mutation;
- new thread/task/background runtime;
- second Tokio runtime;
- generic executor/future-driving API;
- retry/fallback/re-bootstrap;
- readiness publication;
- candidate publication/traversal activation;
- peer dialing;
- runtime durable-owner mutation;
- credential/certificate/trust/RBAC creation or mutation;
- systemd unit/package mutation;
- deployment/restart;
- repository visibility/configuration change;
- merge or branch deletion.

The IF PR must remain draft/open/unmerged after closure.
