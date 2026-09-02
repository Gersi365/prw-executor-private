# C03e-HX — Production Reachability Runtime Custody Adaptation Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_HX_PRODUCTION_REACHABILITY_RUNTIME_CUSTODY_ADAPTATION_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_RUNTIME_CUSTODY_ADAPTATION_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-HW head:

`ceb38882c1ebbe233a2772154d01c83ea5d3b346`

C03e-HW already materializes one crate-internal Agent facade that performs:

```text
&PeerConnectivityIdentity
 -> fixed production systemd reachability credential custody
 -> opaque three-role production etcd bootstrap config
 -> production provider bootstrap
 -> authoritative durable-owner recovery
 -> live reachability-authority composition
 -> ProductionReachabilityBootstrapComposition
```

HW remains draft/open/unmerged and is not merged or activated by HX.

## 2. Evidence that motivates HX

At the exact HW head, `ProductionReachabilityBootstrapComposition` owns two semantic values:

1. `ReachabilityLiveOwnerComposedAsyncAuthority`
2. `ProductionReachabilityEtcdOwnerCustody`

Its consuming `into_parts(self)` exposes exactly those two values.

The existing remote transport/runtime path does not currently retain the second value.

`ReachabilityAuthorityRuntimeOwner` owns only a live authority admission token. The existing remote transport runtime consumes and retains only `ReachabilityAuthorityRuntimeOwner` for endpoint lifetime.

The existing endpoint lifecycle failure path similarly returns only the retained live-authority owner.

Therefore a direct replacement of the existing two-role remote bootstrap with HW at the current process-operation callsite would have no selected owner for the recovered durable production owner.

HX treats that missing lifetime owner as a custody gap, not as permission to drop the durable owner, hide it in a global, clone it, or activate runtime wiring.

## 3. Selected boundary

HX selects one new crate-internal, side-effect-free Agent owner:

`ProductionReachabilityRuntimeCustody`

Selected source module for the first implementation successor:

`crates/prw-agent/src/production_reachability_runtime_custody.rs`

The owner semantically contains exactly:

```text
ReachabilityAuthorityRuntimeOwner
ProductionReachabilityEtcdOwnerCustody
```

Both fields remain private.

The owner is intentionally non-cloneable.

It owns no runtime handle, task, channel, socket, endpoint, listener, systemd credential bytes, provider client, raw durable store, request ID, IP identity, or readiness token.

## 4. Selected adaptation law

The first implementation successor must provide a pure ownership adaptation with this semantic order:

```text
ProductionReachabilityBootstrapComposition
 -> ProductionReachabilityBootstrapComposition::into_parts()
 -> (ReachabilityLiveOwnerComposedAsyncAuthority,
     ProductionReachabilityEtcdOwnerCustody)
 -> crate-internal live-authority adaptation
 -> ReachabilityAuthorityRuntimeOwner
 -> ProductionReachabilityRuntimeCustody {
      authority owner,
      durable owner custody,
    }
```

No credential read, provider I/O, durable read/write, endpoint bind, listener operation, Tokio drive, task spawn, retry, fallback, readiness publication, or process mutation occurs during this adaptation.

## 5. Live-authority adaptation seam

The existing `ReachabilityAuthorityRuntimeOwner` constructor accepts the private admission wrapper, whose authority constructor is currently module-private.

HX selects one additive crate-internal constructor on the existing runtime owner:

```text
ReachabilityAuthorityRuntimeOwner::from_composed_authority(
    ReachabilityLiveOwnerComposedAsyncAuthority,
) -> ReachabilityAuthorityRuntimeOwner
```

Semantic implementation:

```text
ReachabilityLiveOwnerComposedAsyncAuthority
 -> ReachabilityLiveOwnerAuthorityAdmission::from_authority(...)
 -> ReachabilityAuthorityRuntimeOwner::new(...)
```

This constructor performs ownership composition only.

It must be `pub(crate)`, not public.

It must not expose `ReachabilityLiveOwnerAuthorityAdmission`, raw provider state, credentials, or authority internals.

The existing public/two-role admission/bootstrap APIs remain unchanged.

## 6. Runtime-custody construction seam

The first implementation successor selects one crate-internal constructor with semantic shape:

```text
ProductionReachabilityRuntimeCustody::from_bootstrap_composition(
    ProductionReachabilityBootstrapComposition,
) -> ProductionReachabilityRuntimeCustody
```

The constructor consumes the composition exactly once.

It may only split the composition to immediately re-own both parts in the new runtime-custody owner.

It must not return either part separately during construction.

No `Clone`, `Copy`, shared-global registration, singleton, service locator, static mutable slot, thread-local owner, detached task, or hidden process-global custody is selected.

## 7. No extraction seam selected yet

HX intentionally does **not** select a public or crate-internal `into_parts`/take/extract method on `ProductionReachabilityRuntimeCustody`.

Reason: the next endpoint/process integration checkpoint must first define how live authority moves into endpoint lifetime while durable owner custody remains retained across endpoint startup, runtime drive, failure, and deterministic shutdown.

A generic extraction method would permit accidental durable-custody drop before that ownership transaction is selected.

Therefore the first source successor keeps both fields private with no general extraction API.

## 8. Downstream endpoint law deferred

HX does not select endpoint binding.

A later checkpoint must separately decide a bounded transaction with properties equivalent to:

```text
ProductionReachabilityRuntimeCustody
 + existing RemoteSessionExecutorRuntime
 + existing bind address
 -> existing endpoint lifecycle using the retained live authority
 + durable owner custody retained for the same process/endpoint lifetime
```

That later checkpoint must also specify non-lossy failure ownership before any source implementation.

HX makes no choice today about the exact endpoint wrapper type, failure carrier, shutdown order, process companion carrier, or binary exit policy.

## 9. Existing two-role path remains unchanged

The following existing surfaces remain untouched and callable exactly as before:

- `ReachabilityAuthorityCustodyBootstrapError`
- `bootstrap_reachability_live_owner_authority_from_systemd_credentials()`
- `bootstrap_and_admit_reachability_live_owner_authority_from_systemd_credentials()`
- `RemoteSessionExecutorRuntime::bootstrap_reachability_authority_from_systemd_credentials(...)`
- `linux_agent_remote_process_operation(...)`

HX does not redirect any current caller from two-role bootstrap to production three-role bootstrap.

HX does not remove, deprecate, alias, or silently reinterpret the two-role path.

## 10. Current executable/runtime state remains unchanged

At the HW predecessor, the Linux Agent binary still performs device-identity custody and then calls:

`prw_agent::linux_bootstrap::run()`

HX does not alter `main.rs`.

HX does not switch the binary to `run_with_remote_process_companion(...)`.

HX does not create a remote process companion.

HX does not start or bind the remote endpoint.

HX does not alter local readiness semantics.

HX does not publish remote readiness.

HX does not change startup failure or terminal exit classification.

## 11. Exact first source-successor ceiling

The first source implementation successor after HX is limited to exactly these paths:

1. `crates/prw-agent/src/production_reachability_runtime_custody.rs`
2. `crates/prw-agent/src/reachability_authority_admission.rs`
3. `crates/prw-agent/src/lib.rs`

Permitted changes:

- add the new crate-internal runtime-custody module;
- add the new non-cloneable owner and pure constructor;
- add only the crate-internal `ReachabilityAuthorityRuntimeOwner::from_composed_authority(...)` adaptation constructor;
- register the new module crate-internally;
- add compile/type-shape tests that perform no real provider/systemd/network I/O.

No other source path is authorized by HX.

If compilation unexpectedly requires Cargo, lockfile, endpoint lifecycle, remote transport, linux bootstrap, main, custody crate, control-plane, bridge, systemd unit/package, workflow, or deployment changes, the successor must stop and open a separate selection checkpoint.

## 12. Test obligations for the first source successor

The first source successor must at minimum prove:

1. the runtime-custody constructor consumes exactly `ProductionReachabilityBootstrapComposition`;
2. the live-authority adapter accepts exactly `ReachabilityLiveOwnerComposedAsyncAuthority` and returns exactly `ReachabilityAuthorityRuntimeOwner`;
3. the new owner is constructed without credential/provider/runtime/network calls in tests;
4. no general extraction method is added;
5. existing two-role tests remain passing;
6. workspace formatting, Clippy, tests, and build pass on the exact final successor SHA;
7. Android validation is closure evidence if triggered on the source successor.

No real production credentials, etcd endpoints, sockets, listeners, or systemd service operations are permitted in tests.

## 13. Security and identity invariants

HX preserves all existing project invariants:

- logical device identity is not IP-based;
- dynamic IP is transient reachability only;
- request IDs remain correlation only;
- authenticated transport identity remains separate from logical device identity;
- durable owner custody remains peer-lifecycle keyed;
- no raw provider client or credential material escapes;
- no live-owner/fence/durable credential or client reuse is introduced;
- no fallback or degraded production owner is introduced.

## 14. Explicit non-authorization

HX authorizes no:

- runtime/startup/readiness activation;
- `main.rs` wiring;
- endpoint bind;
- remote listener activation;
- candidate publication activation;
- traversal activation;
- dialing;
- provider retry;
- fallback authority/owner/store;
- global singleton/service locator;
- systemd unit/package credential wiring;
- credential, certificate, trust, RBAC, or etcd provisioning;
- service restart;
- deployment;
- production-state mutation;
- repository visibility change;
- merge;
- branch deletion.

The HX PR must remain draft/open/unmerged.

## 15. Closure criteria

HX may close only when all of the following hold on its exact docs-only head:

- predecessor merge base is exact closed HW head `ceb38882c1ebbe233a2772154d01c83ea5d3b346`;
- branch is ahead only by the HX contract commit;
- exactly one Markdown contract path changed;
- no source/Cargo/workflow/runtime/security/deployment path changed;
- exact-head Rust validation succeeds;
- any disposable-etcd/suppression workflows resolve to their expected skipped state unless the exact docs diff legitimately triggers them;
- no required exact-head check remains pending or failing;
- immutable Drive audit is uploaded to the canonical PRW audit parent;
- raw Drive readback matches local byte size and SHA-256;
- exact-title post-upload search identifies the unique canonical audit;
- PR body records the closed gate, lineage, validation evidence, audit identity/hash, and non-authorization;
- PR remains draft/open/unmerged.
