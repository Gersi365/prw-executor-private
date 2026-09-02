# C03e-ID — Production Reachability Same-Executor Bootstrap-to-Endpoint Startup Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_ID_PRODUCTION_REACHABILITY_SAME_EXECUTOR_BOOTSTRAP_ENDPOINT_STARTUP_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_SAME_EXECUTOR_BOOTSTRAP_ENDPOINT_STARTUP_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-IC head:

`4d53237157706aadc2f1db28e59b4fc26177fe14`

IC materializes the production endpoint runtime-drive custody seam. The production endpoint wrapper now retains durable production-owner custody for the entire existing lower endpoint lifecycle and releases it only after the lower endpoint close + idle-drain law returns.

IC intentionally remains dormant from executable/runtime callsites.

## 2. Current production bootstrap evidence

The existing production systemd custody bootstrap is async:

```text
bootstrap_production_reachability_from_systemd_credentials(
    peer: &PeerConnectivityIdentity,
)
-> Future<Output = Result<
       ProductionReachabilityBootstrapComposition,
       ProductionReachabilityCustodyBootstrapError,
   >>
```

That existing function already owns the fixed production credential read, provider bootstrap, durable owner recovery and live-authority composition sequence. It performs no retry/fallback and is not invoked from the running Agent.

The existing side-effect-free ownership adapter is:

```text
ProductionReachabilityRuntimeCustody::from_bootstrap_composition(
    ProductionReachabilityBootstrapComposition,
)
-> ProductionReachabilityRuntimeCustody
```

No new production bootstrap protocol or custody format is needed.

## 3. Existing private-executor precedent

`RemoteSessionExecutorRuntime` owns one private current-thread Tokio runtime and deliberately exposes no generic `block_on`, runtime handle or arbitrary future-driving API.

The exact existing two-role reachability path already establishes the required precedent:

```text
RemoteSessionExecutorRuntime
    ::bootstrap_reachability_authority_from_systemd_credentials(&self)
```

That crate-private domain-specific method drives the existing async reachability bootstrap exactly once using the already-owned private runtime and immediately converts success into a typed Agent runtime owner.

Therefore production integration must follow the same domain-specific pattern. ID must not select a generic executor escape or a second Tokio runtime.

## 4. Current process-companion constraint

The existing process companion accepts a synchronous one-shot operation:

```text
FnOnce(RemoteSessionSupervisorShutdownPublisher) + Send + 'static
```

and the process-lifecycle owner itself creates no Tokio runtime.

Because production bootstrap is async while this operation boundary is synchronous, jumping directly to `linux_bootstrap`/`main.rs` would force an implicit runtime decision. ID resolves only that missing internal runtime boundary first.

## 5. Selected production private-executor bootstrap seam

ID selects one new crate-private domain-specific method on the existing `RemoteSessionExecutorRuntime`:

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

Selected semantic sequence:

```text
already-created RemoteSessionExecutorRuntime
 -> private runtime.block_on(
      bootstrap_production_reachability_from_systemd_credentials(peer)
    ) exactly once
 -> ProductionReachabilityBootstrapComposition
 -> ProductionReachabilityRuntimeCustody::from_bootstrap_composition(...)
 -> ProductionReachabilityRuntimeCustody
```

The private runtime remains owned by the same `RemoteSessionExecutorRuntime`; only a temporary immutable borrow exists during bootstrap.

No generic future driver, runtime handle, provider client, secret material, raw live authority or durable owner escapes.

## 6. Selected bootstrap error law

The production executor bootstrap seam returns the existing `ProductionReachabilityCustodyBootstrapError` unchanged.

On custody/provider/durable-recovery/composition failure:

- no production runtime custody is returned;
- the same executor owner remains caller-owned after the borrowed bootstrap call returns;
- no retry or second bootstrap is attempted;
- no endpoint bind is attempted by this method;
- no two-role fallback is selected.

Executor construction failure remains separately classified by the existing `RemoteSessionExecutorRuntimeCreateError` at the future caller boundary. ID does not combine these error domains prematurely.

## 7. Current supplied-executor endpoint evidence

The existing lower endpoint lifecycle already exposes the crate-private same-executor bind:

```text
RemoteSessionEndpointLifecycleRuntime
    ::bind_with_executor_from_systemd_credentials(
        executor: RemoteSessionExecutorRuntime,
        authority_owner: ReachabilityAuthorityRuntimeOwner,
        bind_addr: SocketAddr,
    )
```

It consumes the exact supplied executor, attempts the existing fixed-credential/TLS/socket bind once, and never constructs a replacement executor.

On lower bind failure, the exact reachability authority is recoverable through the existing startup failure owner; the supplied executor is consumed/dropped by the existing lower failure path and is not reconstructed.

## 8. Selected production supplied-executor endpoint seam

ID selects one new crate-private sibling on `ProductionReachabilityRuntimeCustody`:

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

Selected success sequence:

```text
ProductionReachabilityRuntimeCustody
 -> private destructure into
      ReachabilityAuthorityRuntimeOwner
      + ProductionReachabilityEtcdOwnerCustody
 -> existing lower bind_with_executor_from_systemd_credentials(
      exact supplied executor,
      authority owner,
      bind_addr,
    ) exactly once
 -> lower endpoint lifecycle + unchanged shutdown controller
 -> ProductionReachabilityEndpointLifecycleRuntime {
      endpoint,
      owner_custody,
    }
 -> return production endpoint wrapper + unchanged controller
```

The durable production-owner custody remains associated with the successfully bound endpoint exactly as selected by HZ/IA and later retained across drive by IB/IC.

## 9. Selected same-executor failure law

On supplied-executor endpoint bind failure:

```text
existing RemoteSessionEndpointLifecycleStartupFailure
 -> observe existing bounded startup error
 -> recover exact ReachabilityAuthorityRuntimeOwner
 + untouched ProductionReachabilityEtcdOwnerCustody
 -> reconstruct ProductionReachabilityRuntimeCustody
 -> ProductionReachabilityEndpointLifecycleStartupFailure
```

The selected production failure continues to expose only the existing bounded startup classification and complete production runtime custody.

The supplied executor is not part of `ProductionReachabilityRuntimeCustody`; the existing lower supplied-executor failure path consumes/drops it. ID therefore selects no executor recovery, replacement executor, retry or rebind after failure.

## 10. Selected future one-executor ordering

ID establishes the exact internal ordering that a later separately gated process-operation composition may use:

```text
RemoteSessionExecutorRuntime::new()
 -> executor.bootstrap_production_reachability_runtime_custody_from_systemd_credentials(peer)
 -> runtime_custody.bind_remote_endpoint_with_executor_from_systemd_credentials(
      executor,
      bind_addr,
    )
 -> production endpoint wrapper + shutdown controller
```

This ordering creates exactly one remote-session executor runtime and moves that same executor into the endpoint lifecycle after production bootstrap completes.

No second runtime is created for endpoint startup. No generic runtime API is exposed.

## 11. Explicitly deferred publish + drive composition

ID does **not** select or materialize the later process-operation sequence that would:

- publish `RemoteSessionSupervisorShutdownController` through the existing process publisher;
- invoke IC's production endpoint drive seam;
- integrate expected-device request/policy/registry inputs;
- join the operation to `run_with_remote_process_companion(...)`.

Those are a distinct future composition checkpoint.

## 12. Identity and security invariants

ID preserves all PRW identity and transport separation invariants:

- logical device identity remains independent of dynamic IP;
- request IDs remain correlation only;
- bind address is not logical identity;
- transport identity remains authenticated lower-transport/certificate identity;
- production live-owner and durable-snapshot provider custody remain role-separated;
- no raw provider client, credential bytes, trust material, store, token source or owner handle escapes;
- the existing two-role reachability executor bootstrap remains unchanged.

The only peer input to the production bootstrap seam remains the already-typed `PeerConnectivityIdentity`.

## 13. No durable owner operation

ID selects no runtime durable-owner mutation.

Neither selected seam may invoke:

- `with_owner_mut(...)`;
- `with_owner_mut_async(...)`;
- durable compare-and-commit;
- candidate-publication freshness mutation;
- requester/rendezvous mutation;
- a second durable recovery.

Durable ownership remains custody only until some separately selected operation requires mutation.

## 14. First source-successor ceiling

The first source-materialization successor is authorized to modify exactly two files:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - add only the production domain-specific private-runtime bootstrap-to-runtime-custody seam and bounded signature/error-routing tests;
2. `crates/prw-agent/src/production_reachability_runtime_custody.rs`
   - add only the supplied-executor production endpoint startup sibling and bounded ownership/type-shape tests.

No modification is authorized to:

- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/production_reachability_custody_bootstrap.rs`;
- `crates/prw-agent/src/production_reachability_bootstrap.rs`;
- `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`;
- lower endpoint lifecycle/transport files;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/main.rs`;
- Cargo/lockfiles;
- reachability-custody/control-plane/bridge crates;
- workflows;
- systemd units/packages;
- deployment/security material.

If the selected source shape cannot compile within this exact two-file ceiling, stop and select a separate extension checkpoint rather than widening scope implicitly.

## 15. Test obligations for first source successor

Tests must perform no real production credential read, provider connection, durable recovery, endpoint bind, listener activation or network I/O.

Required evidence is source/type/ownership based:

1. the new executor method has exact `&self + &PeerConnectivityIdentity -> ProductionReachabilityRuntimeCustody / ProductionReachabilityCustodyBootstrapError` shape;
2. it remains domain-specific and does not expose generic `block_on` or runtime handle access;
3. the new production endpoint method consumes `ProductionReachabilityRuntimeCustody` and exact supplied `RemoteSessionExecutorRuntime` by value;
4. success routes exact supplied executor + live authority into the existing lower same-executor bind while retaining durable custody beside the endpoint;
5. failure reconstructs complete production runtime custody from exact recovered authority + untouched durable custody;
6. no retry/replacement executor/fallback is introduced;
7. existing workspace tests remain green.

Pure injected ownership helpers may be used if needed, but must remain private and narrowly tied to these laws.

## 16. Source-level capability versus executable activation

The future source successor may contain calls that perform credential/provider I/O or endpoint bind **only when explicitly invoked**.

ID selects no invocation from `linux_bootstrap`, `main.rs`, process companion construction, readiness code or any running Agent path.

Therefore the first source successor remains dormant from the executable process.

## 17. Explicit non-authorization

ID authorizes no:

- executable production reachability activation;
- `linux_bootstrap` or `main.rs` wiring;
- process-companion publish/drive composition;
- new thread/task/background runtime;
- generic executor/future-driving API;
- second Tokio runtime;
- readiness/listener publication;
- candidate publication or traversal activation;
- peer dialing;
- durable owner mutation beyond the already-existing bootstrap recovery;
- retry/fallback/re-bootstrap;
- systemd unit/package credential wiring;
- credential/certificate/trust/RBAC provisioning;
- service restart/deployment;
- production-state mutation outside the already-existing dormant bootstrap semantics when invoked;
- repository visibility/configuration change;
- merge or branch deletion.

The ID PR must remain draft/open/unmerged after closure.