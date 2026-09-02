# C03e-IF — Production Reachability Remote-Process Operation Composition Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_IF_PRODUCTION_REACHABILITY_REMOTE_PROCESS_OPERATION_COMPOSITION_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_REMOTE_PROCESS_OPERATION_COMPOSITION_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-IE head:

`a5197aafd736c488c4b55315bab0f0d1c75a937b`

Exact IE tree:

`4fe4ba348a4d8904f547c2610b4380b76e3cfb20`

IE materializes the ID-selected same-executor production bootstrap and endpoint-startup seams while remaining dormant from executable/process callsites.

IE gate:

`C03E_IE_PRODUCTION_REACHABILITY_SAME_EXECUTOR_BOOTSTRAP_ENDPOINT_STARTUP_SOURCE_MATERIALIZED`

IE closure:

`CLOSED_PRODUCTION_REACHABILITY_SAME_EXECUTOR_BOOTSTRAP_ENDPOINT_STARTUP_SOURCE_MATERIALIZATION`

IE immutable Drive audit ID:

`10kEfqr-WU9NwCzk9Cm_lRSX774v9pxov`

## 2. Existing remote-process composition evidence

The exact current `crates/prw-agent/src/linux_bootstrap.rs` remains at blob:

`b0fb368d95f35fb034b7cb51c76510fdfcbd7613`

It already contains the generic fail-closed internal composition helper:

```text
run_remote_process_operation_composition(
    construct_executor,
    bootstrap_authority,
    start_endpoint,
    publish_controller,
    drive_lifecycle,
) -> bool
```

Its ordering is exact and sequential:

```text
construct executor
 -> bootstrap authority/custody
 -> start endpoint with that executor
 -> publish exact shutdown controller
 -> drive complete endpoint lifecycle
```

Executor construction, bootstrap or endpoint-startup failure returns `false` before controller publication or lifecycle drive. The helper performs no retry/fallback and constructs no second executor.

## 3. Existing public two-role operation is frozen

The same file already exposes the existing public injected operation:

```text
linux_agent_remote_process_operation(
    LinuxAgentRemoteProcessOperationInputs<...>,
)
-> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

That operation currently composes:

```text
RemoteSessionExecutorRuntime::new
 -> RemoteSessionExecutorRuntime
      ::bootstrap_reachability_authority_from_systemd_credentials
 -> RemoteSessionEndpointLifecycleRuntime
      ::bind_with_executor_from_systemd_credentials
 -> publisher.publish(controller)
 -> lower endpoint lifecycle drive
```

This is the established two-role/live-authority path. IF does not select replacement, retargeting, visibility change or semantic mutation of this existing public function.

The first source successor must preserve the existing public operation unchanged and add a crate-private production sibling instead.

## 4. Existing injected remote-operation inputs remain authoritative

`LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` already owns exactly the typed values needed after reachability/endpoint startup:

- `bind_addr: SocketAddr`
- `max_active_workers: NonZeroUsize`
- `SharedCurrentCapabilityAuthority<P>`
- `SessionAuthenticationService`
- expected-device request receiver
- admission timing source
- completion callback
- rejection callback
- admission-failure callback

Construction of this carrier performs no credential read, provider I/O, endpoint bind, authentication, task spawn, readiness publication or process lifecycle mutation.

IF selects reuse of this exact existing carrier. It does not duplicate these inputs or select their future production sources.

## 5. Production one-executor seams now exist

At exact IE head the executor source blob is:

`ef370ca500f118bc067097ddb8f5c37ab597b214`

It now contains the crate-private production bootstrap seam:

```text
RemoteSessionExecutorRuntime
    ::bootstrap_production_reachability_runtime_custody_from_systemd_credentials(
        &self,
        peer: &PeerConnectivityIdentity,
    )
-> Result<
       ProductionReachabilityRuntimeCustody,
       ProductionReachabilityCustodyBootstrapError,
   >
```

At exact IE head the production runtime-custody blob is:

`ffcddc0253de2b5430be798061ddad8e920a07ac`

It now contains the crate-private supplied-executor endpoint seam:

```text
ProductionReachabilityRuntimeCustody
    ::bind_remote_endpoint_with_executor_from_systemd_credentials(
        self,
        executor: RemoteSessionExecutorRuntime,
        bind_addr: SocketAddr,
    )
-> Result<
       (
           ProductionReachabilityEndpointLifecycleRuntime,
           RemoteSessionSupervisorShutdownController,
       ),
       ProductionReachabilityEndpointLifecycleStartupFailure,
   >
```

Therefore IF requires no new provider bootstrap, durable recovery, runtime owner, endpoint startup protocol or second executor.

## 6. Production endpoint drive seam already retains durable custody

The exact current production endpoint lifecycle remains at blob:

`4be58d66dddccc03e1f0d932b2805aba524ead0c`

It already exposes:

```text
ProductionReachabilityEndpointLifecycleRuntime
    ::drive_repeated_real_remote_admission_endpoint_lifecycle(...)
```

The production wrapper is consumed exactly once and retains `ProductionReachabilityEtcdOwnerCustody` for the complete delegated lower endpoint lifecycle. Durable custody is released only after the lower lifecycle has completed its existing admission/worker/supervisor-shutdown/endpoint-close/idle-drain law.

IF therefore selects this existing production wrapper drive rather than extracting or dropping durable custody before remote operation completion.

## 7. Typed peer identity input

The exact current `PeerConnectivityIdentity` remains an owned typed pair of:

- logical `DeviceId`;
- independently rotatable opaque `TransportIdentity`.

It contains no IP address and no request ID.

IF selects one owned `PeerConnectivityIdentity` as the only new semantic input required by the production remote-process operation. The operation may borrow it only while executing the IE production bootstrap seam.

IF does not select how the executable process obtains this peer identity. That source remains separately gated.

## 8. Selected production remote-process input carrier

IF selects one new crate-private non-cloneable wrapper in `linux_bootstrap.rs`:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    peer: PeerConnectivityIdentity,
}
```

Selected constructor:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(
    remote_process_inputs,
    peer,
)
```

The carrier owns only already-typed values. Construction performs no credential read, provider I/O, endpoint bind, listener activation, authentication, readiness publication, candidate publication, traversal, peer dial or process-lifecycle mutation.

No getter/extraction API is selected. The carrier exists only to move complete typed custody into the selected operation factory.

## 9. Selected production remote-process operation sibling

IF selects one new crate-private sibling in `linux_bootstrap.rs`:

```text
linux_agent_production_reachability_remote_process_operation<P, D, T, F, C, R, E>(
    inputs: LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
)
-> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

The factory itself is side-effect-free. Production credential/provider I/O and endpoint startup occur only when a later caller invokes the returned closure.

The outward operation shape remains exactly compatible with the existing process companion:

```text
FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

No new process-thread result or error channel is selected.

## 10. Exact selected execution ordering

When the returned operation is invoked, IF selects exactly:

```text
owned production operation inputs
 -> destructure existing LinuxAgentRemoteProcessOperationInputs
 -> RemoteSessionExecutorRuntime::new() exactly once
 -> executor.bootstrap_production_reachability_runtime_custody_from_systemd_credentials(
      &peer,
    ) exactly once while borrowing that executor
 -> runtime_custody.bind_remote_endpoint_with_executor_from_systemd_credentials(
      exact same executor,
      bind_addr,
    ) exactly once
 -> ProductionReachabilityEndpointLifecycleRuntime
    + exact RemoteSessionSupervisorShutdownController
 -> publisher.publish(exact controller) exactly once
 -> production endpoint wrapper
      .drive_repeated_real_remote_admission_endpoint_lifecycle(
          existing injected admission/session inputs,
      ) exactly once
 -> operation returns
```

The executor is created once, borrowed for production bootstrap, then moved into endpoint startup. A second executor or generic runtime driver is forbidden.

Durable production-owner custody remains inside the production runtime/endpoint wrappers throughout the sequence and is not separately extracted.

## 11. Selected use of existing composition helper

The first source successor should reuse `run_remote_process_operation_composition(...)` rather than duplicate its fail-closed sequencing.

For the production sibling its semantic type substitution is:

```text
Executor   = RemoteSessionExecutorRuntime
Authority  = ProductionReachabilityRuntimeCustody
Endpoint   = ProductionReachabilityEndpointLifecycleRuntime
Controller = RemoteSessionSupervisorShutdownController
Publication = LinuxAgentRemoteSupervisorShutdownPublish
```

The five closures are selected as:

```text
construct_executor:
    RemoteSessionExecutorRuntime::new

bootstrap_authority:
    |executor| {
        executor
            .bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&peer)
    }

start_endpoint:
    |executor, runtime_custody| {
        runtime_custody
            .bind_remote_endpoint_with_executor_from_systemd_credentials(
                executor,
                bind_addr,
            )
    }

publish_controller:
    |controller| publisher.publish(controller)

drive_lifecycle:
    |lifecycle, _publication| {
        lifecycle.drive_repeated_real_remote_admission_endpoint_lifecycle(...)
    }
```

No alteration to the generic helper itself is selected unless compilation requires only a local formatting/type annotation with no semantic widening. If semantic alteration would be required, stop and select a separate checkpoint.

## 12. Fail-closed startup law

The selected operation preserves the existing remote-process fail-closed protocol:

- executor construction failure: no bootstrap, endpoint, publication or drive;
- production bootstrap failure: no endpoint, publication or drive;
- production endpoint startup failure: no publication or drive;
- none of those failures causes retry, fallback, re-bootstrap, replacement executor or two-role downgrade.

The operation closure continues to return `()`; the internal helper's `false` result remains intentionally non-public process-operation evidence exactly as in the existing two-role operation.

Because no controller is published on pre-endpoint failure, the existing process-side finalization remains authoritative for classifying controller unavailability before endpoint startup.

IF selects no new error enum that could accidentally expose provider/custody internals across the process boundary.

## 13. Shutdown-controller publication law

After successful production endpoint startup, the exact existing controller is published exactly once through `LinuxAgentRemoteSupervisorShutdownPublisher::publish`.

The existing publication law remains unchanged:

- `Published`: exact controller moves to process-side ownership;
- receiver gone: publication recovers the controller, requests orderly shutdown, and reports `ReceiverGoneShutdownRequested`.

The production lifecycle drive still runs after publication exactly as the existing two-role operation does. If the process-side receiver is gone, the recovered controller has already been asked to shut down before lifecycle drive proceeds.

IF selects no second publisher, broadcast channel, task or global shutdown registry.

## 14. Complete endpoint lifecycle law

After publication, the production endpoint wrapper is consumed by the existing IC drive seam with the existing injected admission/session inputs.

The lower lifecycle remains solely responsible for:

- expected-device admission;
- authenticated worker lifetime;
- supervisor shutdown observation;
- endpoint close;
- idle drain;
- existing persistent-collection configuration error.

The selected operation does not intercept, reorder or retry those semantics.

The drive result remains process-operation-internal exactly as in the existing two-role operation; IF does not select a new executable exit mapping.

## 15. Bind-address and admission-source deferral

IF does not select any new source for:

- `bind_addr`;
- max-active-worker capacity;
- capability authority;
- session authentication service;
- expected-device requests;
- dispatcher values;
- admission timing;
- completion/rejection/admission-failure callbacks.

Those values remain supplied through the existing `LinuxAgentRemoteProcessOperationInputs` carrier.

The existing `load_linux_agent_remote_bind_addr_from_env()` is not invoked or wired by IF. Executable bind-address selection remains separately gated.

## 16. Requester/rendezvous composition remains separate

The existing crate-private `LinuxAgentRequesterRendezvousRemoteProcessOperationInputs` and `linux_agent_requester_rendezvous_remote_process_operation(...)` remain unchanged.

IF does not re-point that wrapper to production reachability and does not select requester/rendezvous policy-source activation.

A later checkpoint may decide how requester/rendezvous lifetime custody composes with the new production remote-process sibling. That is not required to prove the production one-executor operation itself.

## 17. Executable process integration remains deferred

IF does not select a call from:

- `run()`;
- `run_with_remote_process_companion(...)` construction sites;
- `main.rs`;
- readiness code;
- systemd service startup.

The selected factory remains dormant until a separately gated caller passes the returned operation into the existing remote process companion.

No process exit policy is selected for production remote failures in IF.

## 18. Identity and security invariants

IF preserves all PRW identity laws:

- logical device identity is not IP-based;
- dynamic IP remains transient reachability only;
- request IDs remain correlation only;
- bind address is not logical identity;
- `PeerConnectivityIdentity` carries logical `DeviceId` plus distinct opaque transport identity;
- no raw endpoint, IP, request ID, provider client, credential bytes, trust material or durable store handle may substitute for peer identity.

Production live-owner and durable-snapshot provider custody remain role-separated and opaque.

## 19. No durable-owner operation

IF selects custody retention only.

The new process operation must not invoke:

- `with_owner_mut(...)`;
- `with_owner_mut_async(...)`;
- durable compare-and-commit;
- candidate-publication freshness mutation;
- requester/rendezvous durable mutation;
- a second durable recovery.

The existing bootstrap recovery remains the only durable recovery in the selected operation sequence.

## 20. First source-successor ceiling

The first source-materialization successor is authorized to modify exactly one file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized changes are only:

1. import the existing `PeerConnectivityIdentity` type if needed;
2. add the crate-private production remote-process input carrier and constructor;
3. add the crate-private production remote-process operation sibling using the exact existing helper and IE/IC seams;
4. add bounded source/type/ordering tests that perform no production I/O.

No modification is authorized to:

- existing public `linux_agent_remote_process_operation(...)` semantics or visibility;
- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/lib.rs`;
- `production_reachability_*` modules;
- remote endpoint/transport modules;
- reachability custody/control-plane/bridge crates;
- Cargo/lockfiles;
- workflows;
- systemd units/packages;
- deployment/security material.

If the production sibling cannot compile inside this exact one-file ceiling, stop and select a separate extension checkpoint instead of widening scope implicitly.

## 21. First-successor test obligations

Tests must perform no real production credential read, provider connection, durable recovery, endpoint bind, listener activation, process spawn or network I/O.

Required evidence:

1. the new carrier owns exact existing remote-operation inputs plus one `PeerConnectivityIdentity`;
2. factory construction is side-effect-free;
3. returned closure has the exact existing process-operation `FnOnce(...)+Send+'static` shape;
4. existing generic helper ordering remains executor → production bootstrap → supplied-executor endpoint → publish → production drive;
5. exactly one executor is created and the same executor reaches endpoint startup;
6. pre-endpoint failure prevents publication/drive without retry/fallback;
7. existing public two-role operation remains unchanged;
8. workspace tests remain green.

Pure injected helper tests may be used to prove ordering and fail-closed behavior without invoking the real production seams.

## 22. Explicit non-authorization

IF authorizes no:

- executable production reachability activation;
- `main.rs` wiring;
- change to `run()`;
- automatic call to `run_with_remote_process_companion(...)`;
- bind-address source activation;
- peer-identity source activation;
- expected-device producer activation;
- requester/rendezvous activation;
- readiness/listener publication;
- candidate publication or traversal activation;
- peer dialing;
- second Tokio runtime;
- generic executor/future-driving API;
- retry/fallback/re-bootstrap;
- durable-owner mutation beyond existing bootstrap recovery;
- systemd unit/package credential wiring;
- credential/certificate/trust/RBAC provisioning;
- service restart/deployment;
- repository visibility/configuration change;
- merge or branch deletion.

The IF PR must remain draft/open/unmerged after closure.
