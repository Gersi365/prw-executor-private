# C03e-IB — Production Reachability Endpoint Lifecycle Runtime Drive Custody Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_IB_PRODUCTION_REACHABILITY_ENDPOINT_LIFECYCLE_RUNTIME_DRIVE_CUSTODY_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_ENDPOINT_LIFECYCLE_RUNTIME_DRIVE_CUSTODY_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-IA head:

`d49a36a4030e6dff0a7cc9d128802598fec270ef`

IA materializes one non-cloneable production endpoint wrapper:

```text
ProductionReachabilityEndpointLifecycleRuntime {
    RemoteSessionEndpointLifecycleRuntime,
    ProductionReachabilityEtcdOwnerCustody,
}
```

IA also materializes the HZ-selected startup transaction that moves durable owner custody beside the successfully bound endpoint and reconstructs complete `ProductionReachabilityRuntimeCustody` on startup failure.

IA intentionally exposes only read-only `bound_addr()` on the production endpoint wrapper and does not select or activate runtime drive.

## 2. Current lower endpoint-drive evidence

The retained existing `RemoteSessionEndpointLifecycleRuntime` already exposes:

```text
drive_repeated_real_remote_admission_endpoint_lifecycle(
    self,
    max_active_workers,
    authority,
    session_authentication,
    expected_requests,
    admission_timing,
    on_completion,
    on_rejection,
    on_admission_failure,
)
-> Result<(), RemoteSessionPersistentCollectionConfigError>
```

That existing method consumes the endpoint lifecycle exactly once, consumes its stored supervisor-shutdown signal exactly once, and delegates all admission, worker, shutdown, endpoint-close and idle-drain behavior to the existing repeated-admission executor lifecycle.

Its existing contract states that even the persistent-collection configuration error is returned only after the lower lifecycle has closed the bound endpoint and driven it idle.

No new lower transport, executor, worker, shutdown, or close state machine is required for the production wrapper.

## 3. Durable owner API evidence

The retained `ProductionReachabilityEtcdOwnerCustody` is an alias of the existing `ProductionReachabilityOwnerCustody<ReachabilityDurableSnapshotEtcdStore, ProductionReachabilityFreshnessTokenSource>`.

That custody type already owns the durable production owner and exposes bounded sync/async owner-operation seams. However, no existing endpoint lifecycle transition requires or invokes any durable-owner operation merely to drive the endpoint to terminal close/idle state.

Therefore IB selects no new durable load, commit, refresh, candidate-publication, or other owner mutation during endpoint drive.

The selected role of durable custody in this checkpoint is lifetime retention only.

## 4. Selected successor boundary

IB selects one additive production-wrapper drive seam that consumes the complete `ProductionReachabilityEndpointLifecycleRuntime` exactly once and delegates to the retained existing endpoint lifecycle exactly once while keeping durable owner custody alive for the complete lower drive call.

Selected semantic entrypoint shape:

```text
ProductionReachabilityEndpointLifecycleRuntime
    ::drive_repeated_real_remote_admission_endpoint_lifecycle<P, D, T, F, C, R, E>(
        self,
        max_active_workers: NonZeroUsize,
        authority: &SharedCurrentCapabilityAuthority<P>,
        session_authentication: &mut SessionAuthenticationService,
        expected_requests: mpsc::Receiver<
            RemoteSessionExpectedDeviceAdmissionRequest<D, T>
        >,
        admission_timing: F,
        on_completion: C,
        on_rejection: R,
        on_admission_failure: E,
    )
-> Result<(), RemoteSessionPersistentCollectionConfigError>
```

The exact Rust formatting may be rustfmt-normalized, but the ownership, bounds, argument order, and result semantics must remain aligned with the existing lower endpoint lifecycle method.

## 5. Selected custody-retention law

The production wrapper must be consumed by value.

Selected ownership law:

```text
ProductionReachabilityEndpointLifecycleRuntime
 -> private destructure inside its defining module
 -> RemoteSessionEndpointLifecycleRuntime endpoint
    + ProductionReachabilityEtcdOwnerCustody owner_custody
 -> retain owner_custody lexically
 -> endpoint.drive_repeated_real_remote_admission_endpoint_lifecycle(...)
    exactly once
 -> lower lifecycle completes terminal endpoint close + idle drain
 -> only then release owner_custody
 -> return the exact lower Result unchanged
```

The source successor should make the lifetime ordering explicit enough to audit: durable owner custody must not be dropped before the existing lower drive call returns.

No owner reference, raw owner, provider handle, store, executor, credential material, or mutable durable handle may escape this wrapper.

## 6. Selected success semantics

If the existing lower drive returns `Ok(())`:

1. the lower endpoint lifecycle has consumed the endpoint and completed its existing shutdown/close/idle-drain law;
2. durable owner custody has remained alive for the full duration of that lower call;
3. durable owner custody may then be dropped normally;
4. the production wrapper returns `Ok(())` unchanged.

IB does not add a post-drive durable commit, final snapshot write, retry, re-recovery, close acknowledgment, or replacement owner.

## 7. Selected lower-error semantics

If the existing lower drive returns `Err(RemoteSessionPersistentCollectionConfigError)`:

1. the existing lower contract still owns endpoint close and idle-drain completion before returning;
2. durable owner custody remains alive until that lower call returns;
3. durable owner custody is then released normally;
4. the exact existing lower error is returned unchanged.

The production wrapper must not convert this error into startup failure, reconstruct pre-bind runtime custody, restart the endpoint, or retry the lifecycle. Startup recovery is an IA concern; terminal drive completion is a distinct ownership phase.

## 8. Existing shutdown controller remains unchanged

The `RemoteSessionSupervisorShutdownController` returned by IA startup remains the only selected external shutdown-control authority for this endpoint lifecycle.

IB adds no new shutdown controller, clone, channel, signal, registry, cancellation primitive, process signal hook, or global handle.

The wrapper drive seam consumes only the endpoint wrapper. The existing controller continues to wake the lower endpoint lifecycle through its already-retained shutdown signal.

## 9. No durable operation during drive

IB explicitly selects **retention without invocation** for `ProductionReachabilityEtcdOwnerCustody` during this drive seam.

The successor must not call:

- `with_owner_mut(...)`;
- `with_owner_mut_async(...)`;
- any durable load/recovery/compare-and-commit operation;
- candidate-publication freshness mutation;
- requester/rendezvous mutation;
- any provider or network operation through durable custody.

Any future need for a durable owner operation during endpoint runtime must be selected in a separate checkpoint with its exact trigger, ordering, persistence semantics, and failure law.

## 10. Existing lower-drive semantics remain authoritative

The production wrapper must delegate rather than copy or reimplement the existing lower lifecycle.

IB selects no changes to:

- worker admission;
- persistent worker collection;
- session authentication;
- expected-device admission;
- worker completion/rejection/failure callbacks;
- supervisor shutdown signaling;
- endpoint close;
- idle drain;
- executor ownership;
- transport ownership.

All such behavior remains defined by the existing `RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle(...)` and its lower executor lifecycle.

## 11. Identity/security invariants

IB preserves all existing PRW identity and transport separation invariants.

In particular:

- logical device identity remains independent of dynamic IP;
- request IDs remain correlation only;
- bind address and observed bound address are not logical identity;
- transport identity remains lower-transport/certificate derived;
- production live authority and durable owner custody remain separate semantic owners;
- role-isolated durable provider custody remains encapsulated;
- no raw provider client, credential bytes, store, owner, freshness-token source, or mutable guard is exposed.

## 12. First source-successor ceiling

The first source-materialization successor is authorized to modify only:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

The expected change is one production-wrapper drive delegation method plus bounded source/type/lifetime tests inside the same file.

No modification is authorized to:

- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/production_reachability_runtime_custody.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/main.rs`;
- Cargo/lockfiles;
- reachability custody/control-plane/bridge crates;
- systemd units/packages;
- workflows;
- deployment/security material.

If the source successor cannot compile within this exact one-file ceiling, stop and select a separate extension checkpoint instead of widening scope implicitly.

## 13. Test obligations for first source successor

Tests must not perform real systemd credential reads, real endpoint binds, real endpoint drive/network I/O, provider connections, durable recovery, durable commits, or production-state mutation.

Required evidence is source/type/lifetime based:

1. the new method consumes `ProductionReachabilityEndpointLifecycleRuntime` by value;
2. the method has the same existing lower-drive inputs/result shape;
3. durable custody remains lexically retained until the lower drive call returns;
4. the exact lower `Result<(), RemoteSessionPersistentCollectionConfigError>` is returned unchanged;
5. no owner operation is invoked merely by the wrapper drive seam;
6. no generic split/extraction API is added;
7. existing workspace tests remain green.

A pure local helper may be used to prove drop ordering without invoking a real endpoint lifecycle, but it must not introduce a broader reusable ownership API.

## 14. Source-level side effects versus executable activation

The future source-materialized method will delegate to an existing real endpoint-drive method **only if invoked**.

IB does not add any invocation from `linux_bootstrap`, `main.rs`, a background task, a process companion, readiness code, or any other executable path.

Therefore the first source successor remains dormant from the executable process even though it materializes the typed production drive capability.

Executable activation must remain a separately selected checkpoint.

## 15. Explicitly deferred executable integration

IB does not select how a running Linux Agent obtains or composes all inputs required by the production drive seam.

Still deferred are, at minimum:

- executable source of the exact `PeerConnectivityIdentity` used by production reachability bootstrap;
- executable sequencing of production systemd custody/bootstrap -> runtime custody -> endpoint bind -> endpoint drive;
- integration with existing remote process companion ownership;
- bind-address source invocation;
- shutdown-controller publication/finalization in a production reachability lane;
- readiness/listener publication policy;
- expected-device request producer and policy/registry composition;
- candidate-publication/traversal activation;
- process exit/error mapping.

Those concerns must not be pulled into the first IB source successor.

## 16. Explicit non-authorization

IB authorizes no:

- executable endpoint activation;
- `linux_bootstrap` or `main.rs` wiring;
- background task/thread/process-companion activation;
- readiness/listener publication;
- candidate publication or traversal activation;
- peer dialing;
- new shutdown primitive or signal integration;
- durable owner mutation during endpoint drive;
- retry/fallback/re-bootstrap;
- systemd unit/package credential wiring;
- credential/certificate/trust/RBAC provisioning;
- service restart/deployment;
- production-state mutation;
- repository visibility/configuration change;
- merge or branch deletion.

The IB PR must remain draft/open/unmerged after closure.