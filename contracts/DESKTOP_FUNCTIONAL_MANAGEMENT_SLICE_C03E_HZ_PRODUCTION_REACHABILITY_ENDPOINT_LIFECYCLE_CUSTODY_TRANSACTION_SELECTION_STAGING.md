# C03e-HZ — Production Reachability Endpoint Lifecycle Custody Transaction Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_HZ_PRODUCTION_REACHABILITY_ENDPOINT_LIFECYCLE_CUSTODY_TRANSACTION_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_ENDPOINT_LIFECYCLE_CUSTODY_TRANSACTION_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-HY head:

`eaabb480bc5d77dfca57bc2182d651215be94c97`

HY materializes one side-effect-free joint runtime-custody owner:

```text
ProductionReachabilityRuntimeCustody {
    ReachabilityAuthorityRuntimeOwner,
    ProductionReachabilityEtcdOwnerCustody,
}
```

HY intentionally exposes no generic extraction/split API and does not activate any endpoint or process callsite.

## 2. Current endpoint evidence

The existing `RemoteSessionEndpointLifecycleRuntime::bind_from_systemd_credentials(...)` consumes one `ReachabilityAuthorityRuntimeOwner` and either:

- succeeds with an endpoint lifecycle owner plus `RemoteSessionSupervisorShutdownController`, or
- fails with `RemoteSessionEndpointLifecycleStartupFailure`, which retains and can return the exact `ReachabilityAuthorityRuntimeOwner`.

The existing failure does **not** know about or retain `ProductionReachabilityEtcdOwnerCustody`.

Therefore directly splitting HY custody at a caller and invoking the existing endpoint startup would create a new loss boundary unless the durable custody is explicitly re-associated on both success and failure.

## 3. Selected successor boundary

HZ selects one additive Agent-internal endpoint startup transaction that consumes the whole HY runtime-custody owner exactly once and never exposes its two components to an external caller.

Selected crate-internal entrypoint semantic shape:

```text
ProductionReachabilityRuntimeCustody
    ::bind_remote_endpoint_from_systemd_credentials(
        self,
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

The exact final Rust spelling may be rustfmt-normalized, but the ownership/error semantics below are mandatory.

## 4. Selected success transaction

The selected success law is exactly:

```text
ProductionReachabilityRuntimeCustody
 -> private destructure inside its defining module only
 -> ReachabilityAuthorityRuntimeOwner
    + ProductionReachabilityEtcdOwnerCustody
 -> existing RemoteSessionEndpointLifecycleRuntime::bind_from_systemd_credentials(
        authority_owner,
        bind_addr,
    )
 -> existing endpoint lifecycle runtime
    + existing shutdown controller
 -> ProductionReachabilityEndpointLifecycleRuntime {
        endpoint,
        owner_custody,
    }
 -> return wrapper + unchanged shutdown controller
```

The durable custody must remain owned by the new production endpoint wrapper immediately after successful bind. It must not be dropped, cloned, registered globally, moved to a detached task, or returned independently.

## 5. Selected failure transaction

On existing endpoint startup failure:

```text
RemoteSessionEndpointLifecycleStartupFailure
 -> observe bounded existing RemoteSessionEndpointLifecycleStartupError
 -> recover exact ReachabilityAuthorityRuntimeOwner
 -> recombine recovered live authority owner
    + untouched ProductionReachabilityEtcdOwnerCustody
 -> reconstruct ProductionReachabilityRuntimeCustody
 -> ProductionReachabilityEndpointLifecycleStartupFailure {
        runtime_custody,
        error,
    }
```

Failure must return the complete pre-bind production runtime custody exactly once. No component may be lost.

The selected production failure exposes only:

```text
error(&self) -> RemoteSessionEndpointLifecycleStartupError
into_runtime_custody(self) -> ProductionReachabilityRuntimeCustody
```

It must not expose raw provider clients, credentials, live authority internals, durable executor/store internals, endpoint internals, or a generic tuple split.

## 6. New production endpoint lifecycle owner

HZ selects one new crate-internal/non-cloneable owner:

`ProductionReachabilityEndpointLifecycleRuntime`

Private semantic ownership:

```text
RemoteSessionEndpointLifecycleRuntime
ProductionReachabilityEtcdOwnerCustody
```

The first source successor may expose only the existing read-only bound-address observation:

```text
bound_addr(&self)
 -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError>
```

implemented by exact delegation to the retained existing endpoint owner.

No drive/consume/extraction API is selected yet.

## 7. Explicitly deferred endpoint drive

HZ does **not** select the runtime-drive transaction yet.

A later checkpoint must separately define how `ProductionReachabilityEndpointLifecycleRuntime` is consumed through the existing repeated-admission endpoint lifecycle while proving that durable owner custody remains alive until deterministic endpoint close + idle drain complete.

That later drive checkpoint must also decide whether any bounded durable-owner operation is needed during the running endpoint lifetime. HZ must not guess or pre-authorize that behavior.

## 8. Existing shutdown controller

The existing `RemoteSessionSupervisorShutdownController` is returned unchanged from successful startup.

HZ selects no new shutdown primitive, no clone, no global registry and no signal integration.

Returning the controller does not publish readiness and does not activate a process callsite.

## 9. No re-bootstrap/retry/fallback

The selected transaction attempts the existing endpoint bind exactly once.

On failure:
- no second executor is created by HY/HZ code,
- no replacement endpoint bind is attempted,
- no production reachability provider bootstrap is repeated,
- no durable recovery is repeated,
- no two-role fallback is attempted,
- no degraded runtime owner is produced.

The caller receives the fully reconstructed `ProductionReachabilityRuntimeCustody` plus the existing bounded endpoint startup classification.

## 10. Identity/security invariants

HZ preserves all existing PRW identity and transport separation invariants.

In particular:
- logical device identity remains independent of dynamic IP,
- request IDs remain correlation only,
- transport identity remains certificate/lower-transport derived,
- endpoint bind address is not logical identity,
- live-owner and durable-snapshot provider custody remain role-separated,
- no credential bytes or broad provider clients are exposed by the production endpoint wrapper.

## 11. First source-successor ceiling

The first source-materialization successor is authorized to modify only:

1. `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs` — new
2. `crates/prw-agent/src/production_reachability_runtime_custody.rs` — narrow endpoint-startup transaction only
3. `crates/prw-agent/src/lib.rs` — crate-internal module registration only

No modification is authorized to:
- `remote_session_endpoint_lifecycle_runtime.rs`
- `remote_transport_runtime.rs`
- `linux_bootstrap.rs`
- `main.rs`
- Cargo/lockfiles
- reachability custody crate
- control-plane or bridge crates
- systemd units/packages
- workflows
- deployment/security material

If the selected source shape cannot compile within this exact three-path ceiling, stop and select a separate extension checkpoint instead of widening scope implicitly.

## 12. Test obligations for first source successor

Tests must not perform a real systemd credential read, endpoint bind, provider connect, durable recovery or network operation.

Required evidence is source/type/ownership based:

1. the production endpoint wrapper is non-cloneable by construction and has the selected bound-address delegation shape;
2. the endpoint-startup method consumes `ProductionReachabilityRuntimeCustody` by value;
3. the success composition law retains durable custody with the endpoint wrapper;
4. the failure composition law reconstructs complete production runtime custody and preserves the existing bounded endpoint startup error;
5. a pure injected helper may be used to prove success/failure ownership routing without real endpoint I/O;
6. no generic extraction/split API is added to production runtime custody.

Existing workspace tests must remain green.

## 13. Source-level side effects versus executable activation

The future source-materialized transaction will call an existing real endpoint-bind function **only if invoked**.

HZ does not add any invocation from an executable/runtime path. Therefore the first source successor must remain unreachable from `main.rs`/`linux_bootstrap` and must not itself execute during tests.

This distinction is mandatory: source capability may be materialized, executable activation is not authorized.

## 14. Explicit non-authorization

HZ authorizes no:
- endpoint activation from a running process
- `main.rs` or `linux_bootstrap` wiring
- readiness publication
- remote listener activation
- candidate publication or traversal activation
- peer dialing
- endpoint runtime drive
- retry/fallback
- systemd unit/package credential wiring
- credential/certificate/trust/RBAC provisioning
- service restart/deployment
- production-state mutation
- repository visibility/configuration change
- merge or branch deletion

The HZ PR must remain draft/open/unmerged after closure.
