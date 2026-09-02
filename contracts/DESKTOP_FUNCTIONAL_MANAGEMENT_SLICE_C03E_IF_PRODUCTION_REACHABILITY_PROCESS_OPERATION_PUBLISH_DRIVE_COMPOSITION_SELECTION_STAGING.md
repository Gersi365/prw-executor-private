# C03e-IF — Production Reachability Process-Operation Publish-and-Drive Composition Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_IF_PRODUCTION_REACHABILITY_PROCESS_OPERATION_PUBLISH_DRIVE_COMPOSITION_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_PROCESS_OPERATION_PUBLISH_DRIVE_COMPOSITION_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-IE head:

`a5197aafd736c488c4b55315bab0f0d1c75a937b`

IE materializes the dormant same-executor production bridge:

1. `RemoteSessionExecutorRuntime::bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&self, peer)` drives the existing async production custody/bootstrap exactly once on the already-owned private current-thread Tokio runtime and returns `ProductionReachabilityRuntimeCustody`;
2. `ProductionReachabilityRuntimeCustody::bind_remote_endpoint_with_executor_from_systemd_credentials(self, executor, bind_addr)` consumes that exact executor through the existing lower supplied-executor endpoint bind and retains durable production-owner custody beside a successful endpoint.

IE does not invoke either seam from an executable or process-companion caller.

## 2. Existing process-operation composition evidence

`crates/prw-agent/src/linux_bootstrap.rs` already owns the generic private helper:

```text
run_remote_process_operation_composition(
    construct_executor,
    bootstrap_authority,
    start_endpoint,
    publish_controller,
    drive_lifecycle,
) -> bool
```

Its fixed ordering is:

```text
construct executor
 -> bootstrap typed authority/custody
 -> start endpoint with exact executor + typed authority/custody
 -> publish exact shutdown controller
 -> drive endpoint lifecycle
```

The helper fails closed before each later stage. It performs no retry, fallback or replacement construction.

The same module also contains the existing public two-role operation:

```text
linux_agent_remote_process_operation(inputs)
 -> FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

That public operation currently uses the older two-role reachability authority bootstrap and lower endpoint lifecycle. C03e-IF does not replace or alter this public path.

## 3. Existing process-companion finalization law

`RemoteSessionProcessLifecycleOwner::spawn(...)` creates exactly one join-owned OS thread around a supplied one-shot operation and gives the lane one one-shot `RemoteSessionSupervisorShutdownPublisher`.

Process-side finalization blocks until either:

- the exact endpoint shutdown controller is published, after which orderly shutdown is requested and the exact thread is joined; or
- the remote operation terminates before controller publication, in which case controller finalization is `UnavailableBeforeEndpointStartup` and the exact thread is joined.

Therefore executor/bootstrap/bind failure before publication can terminate the production operation without inventing shutdown authority, retrying startup or detaching a thread.

If publication itself finds process-side ownership already gone, the existing publisher recovers the exact controller and requests orderly shutdown before endpoint drive proceeds.

## 4. Existing production endpoint-drive evidence

The closed C03e-IC source seam remains available at IE head:

```text
ProductionReachabilityEndpointLifecycleRuntime
    ::drive_repeated_real_remote_admission_endpoint_lifecycle(...)
```

It delegates to the existing lower repeated-admission lifecycle while retaining `ProductionReachabilityEtcdOwnerCustody` for the entire delegated call. Durable custody is dropped only after the lower endpoint close + idle-drain law returns.

No durable-owner mutation is performed by this drive seam.

## 5. Missing boundary after IE

A future production remote process operation needs one internal ownership composition that can use the already-materialized IE seams in the existing synchronous remote-thread operation boundary.

The missing composition is not an executable startup decision. It is only a dormant factory that owns all already-typed process-operation inputs plus the typed production peer identity and returns the same existing one-shot operation shape accepted by `run_with_remote_process_companion(...)`.

## 6. Selected production operation-input owner

C03e-IF selects one new crate-private non-cloneable input owner in `linux_bootstrap.rs` with semantic shape:

```text
LinuxAgentProductionRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    peer: PeerConnectivityIdentity,
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
}
```

The owner consumes one already-typed `PeerConnectivityIdentity` by value and one existing `LinuxAgentRemoteProcessOperationInputs` by value.

The owner does not accept raw device IDs, IP addresses as identity, provider endpoints, credential bytes, certificate material, provider clients, durable store handles, request IDs or runtime handles.

A crate-private constructor may be added solely to build this owner without starting work.

## 7. Selected production process-operation factory

C03e-IF selects one new crate-private sibling in `linux_bootstrap.rs`:

```text
linux_agent_production_remote_process_operation(
    inputs: LinuxAgentProductionRemoteProcessOperationInputs<...>,
)
-> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

Factory construction itself is side-effect-free. Credential reads, provider I/O and endpoint bind occur only if a later separately gated caller invokes the returned closure.

The existing public `linux_agent_remote_process_operation(...)` remains unchanged.

## 8. Selected exact production operation ordering

When the returned production operation is invoked, it must use the existing `run_remote_process_operation_composition(...)` ordering with exactly these stages:

```text
1. RemoteSessionExecutorRuntime::new()

2. executor
     .bootstrap_production_reachability_runtime_custody_from_systemd_credentials(
         &peer,
     )

3. runtime_custody
     .bind_remote_endpoint_with_executor_from_systemd_credentials(
         executor,
         bind_addr,
     )

4. publisher.publish(exact_shutdown_controller)

5. production_endpoint
     .drive_repeated_real_remote_admission_endpoint_lifecycle(
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

Exactly one `RemoteSessionExecutorRuntime` is constructed. The same executor is borrowed for async production bootstrap and then moved into endpoint bind. No second Tokio runtime is created.

## 9. Controller publication law

The exact shutdown controller returned by the successful IE supplied-executor bind is published exactly once through the existing `LinuxAgentRemoteSupervisorShutdownPublisher`.

Publication occurs before production endpoint drive.

The existing bounded publication result is not promoted into readiness, authentication, identity or durable-state evidence.

If process ownership has disappeared before publication, the existing publisher requests orderly shutdown on the recovered exact controller. The production endpoint lifecycle is still driven through its existing shutdown/close/idle-drain behavior; no alternate controller or hard cancellation is introduced.

## 10. Failure law before controller publication

The selected process operation remains fail-closed at each pre-publication stage:

- executor creation failure: operation returns without bootstrap, endpoint bind or publication;
- production bootstrap/custody failure: the borrowed executor remains locally owned until operation teardown, no endpoint bind occurs, and no retry/fallback occurs;
- endpoint startup failure: the existing IE failure reconstructs complete production runtime custody, but because this process operation has no selected recovery action, the failure owner is dropped as the operation terminates; no rebind, replacement executor or provider re-bootstrap occurs.

In all three cases the operation drops its one-shot publisher without publishing a controller. Existing process finalization then reports `UnavailableBeforeEndpointStartup` and joins the exact remote thread.

C03e-IF selects no new public error surface for these internal operation failures.

## 11. Production endpoint lifetime law

After successful controller publication, the selected operation drives exactly the C03e-IC production endpoint wrapper.

This preserves:

- lower live-authority ownership inside the endpoint lifecycle;
- durable production-owner custody beside the endpoint for the full drive;
- existing repeated admission behavior;
- existing supervisor shutdown behavior;
- existing endpoint close + idle-drain completion before durable custody is released.

No durable-owner operation is added.

## 12. Identity invariants

C03e-IF preserves the PRW identity law:

- logical device/peer identity is not fixed-IP based;
- dynamic IP remains transient reachability only;
- request IDs remain correlation only;
- `bind_addr` is transport configuration, not identity;
- the production bootstrap peer input remains the typed `PeerConnectivityIdentity`;
- transport identity remains authenticated lower-transport/certificate identity.

The selected production operation owns `PeerConnectivityIdentity` directly so the borrowed bootstrap input remains valid for the operation invocation without deriving identity from bind address or network observation.

## 13. Existing remote-operation inputs remain authoritative

The selected production operation reuses the existing `LinuxAgentRemoteProcessOperationInputs` for:

- bind address;
- max active workers;
- capability authority;
- session authentication;
- expected request receiver;
- admission timing;
- worker completion callback;
- expected-device rejection callback;
- repeated-admission failure callback.

C03e-IF selects no new producer/source for any of these values.

In particular, it does not select production bind-address loading, expected-device production, capability registry construction, session-authentication construction, dispatcher construction, policy-source construction, timing-source construction or requester/rendezvous assembly.

## 14. Existing requester/rendezvous wrapper remains separate

The existing crate-private `LinuxAgentRequesterRendezvousRemoteProcessOperationInputs` and `linux_agent_requester_rendezvous_remote_process_operation(...)` remain unchanged by the first source successor.

C03e-IF does not combine requester/rendezvous custody with the new production operation yet. Any such assembly remains separately gated.

## 15. First source-successor ceiling

The first source-materialization successor is authorized to modify exactly one file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized changes are only:

1. import or fully qualify the existing `PeerConnectivityIdentity` type as needed;
2. add the crate-private production remote-process input owner and side-effect-free constructor;
3. add the crate-private production remote-process operation sibling implementing the selected ordering through existing IE/IC seams and the existing generic composition helper;
4. add bounded no-I/O type/ordering tests or pure-helper tests required to demonstrate the selected composition.

No other file may change in the first source successor.

## 16. Test obligations

Tests must not perform real production credential reads, provider connections, durable recovery, endpoint binds, listener activation or network I/O.

Required evidence is source/type/pure-composition based:

1. the new input owner consumes exact `PeerConnectivityIdentity` plus existing remote-process inputs;
2. the new factory has the same one-shot publisher operation shape as the existing companion boundary;
3. the production operation uses the existing generic ordering construct→bootstrap→bind→publish→drive;
4. failure at executor/bootstrap/bind prevents later stages;
5. publication precedes drive;
6. no second executor/runtime, retry, fallback, re-bootstrap or replacement controller is introduced;
7. existing workspace tests remain green.

Existing generic composition-helper tests may be reused or extended only within `linux_bootstrap.rs`.

## 17. Source-level capability versus activation

The first source successor may contain dormant calls which read production credentials, perform provider bootstrap and bind an endpoint only when its returned operation is explicitly invoked.

It must not invoke the production factory from:

- `run()`;
- `run_with_remote_process_companion(...)` internally;
- `main.rs`;
- readiness code;
- systemd/package startup wiring;
- any other currently running Agent path.

Therefore the first source successor remains dormant from executable behavior.

## 18. Explicitly deferred executable and production assembly work

C03e-IF does not select or authorize:

- replacing `main.rs` call to `linux_bootstrap::run()`;
- automatically calling `run_with_remote_process_companion(...)`;
- production creation/loading of `PeerConnectivityIdentity`;
- production creation of `LinuxAgentRemoteProcessOperationInputs`;
- requester/rendezvous production assembly;
- executable exit-policy changes based on remote companion evidence;
- readiness semantics for remote endpoint success/failure;
- systemd `LoadCredential=` or package wiring;
- credential/certificate/trust/RBAC provisioning;
- deployment or service restart.

Those require later separately selected checkpoints.

## 19. Security and durable-state non-authorization

C03e-IF authorizes no:

- generic `block_on`, Tokio handle or arbitrary future-driving API;
- second Tokio runtime;
- raw provider client or store exposure;
- credential/certificate/trust disclosure;
- durable snapshot protocol change;
- runtime durable-owner mutation;
- candidate-publication mutation;
- traversal activation or peer dialing beyond the existing endpoint lifecycle when a future caller explicitly invokes the dormant operation;
- retry, fallback or re-bootstrap;
- repository visibility/configuration change;
- merge or branch deletion.

The IF PR must remain draft/open/unmerged after closure.
