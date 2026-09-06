# C03e-MG — Production Remote-Process Current Capability Authority Fail-Closed Population Selection

## Status

`SELECTION — VALIDATION_PENDING`

## Gate

`C03E_MG_PRODUCTION_REMOTE_PROCESS_CURRENT_CAPABILITY_AUTHORITY_FAIL_CLOSED_POPULATION_SELECTED`

## Exact predecessor authority

Evidence-closed predecessor:

`C03e-MF — Production remote-process session-authentication authority population source materialization`

Exact predecessor branch:

`phase-152-c03e-mf-production-remote-process-session-authentication-authority-population-source-materialization`

Exact predecessor head:

`779cc19dcf453f9e8fecfb4d828ea9dddc3ddc3e`

Exact predecessor tree:

`729354ac533b1e640a25ece4171620881cab0497`

Exact predecessor higher-owner blob:

`8bfa387edd90c26fa35414370368d0875c1da16a`

C03e-MF materialized exactly one dormant session-authentication population wrapper and left concrete production provenance for current capability authority, expected-request ingress, admission timing, callbacks and requester/rendezvous separately gated.

## Selection question

Select the smallest remaining current-capability-authority production population boundary that is explicitly supported by inherited production source while remaining fail-closed and without inventing allow-bearing policy, registry hydration, expected-request ingress, timing, callbacks, requester/rendezvous state, executable caller wiring or runtime activation.

## Fresh exact-source audit

### Production-safe remote capability policy

Exact source:

`crates/prw-policy/src/lib.rs`

Exact C03e-MF blob:

`3056b53e81c4429314d9f890dcf2bf3e80d433b8`

Existing exact type:

`ProductionRemoteCapabilityDenyAllPolicy`

is documented by the source as the production-safe remote capability baseline that grants no capability.

It has:

- no external policy source;
- no mutable grant state;
- no allow-bearing configuration;
- no credential dependency;
- no network/provider dependency;
- no runtime activation behavior.

Its `PolicyEvaluator` implementation returns `Decision::Deny` for every represented capability.

The source explicitly states that this baseline exists so production durable capability-authority composition can remain fail-closed until an allow-bearing production policy source is separately selected and reviewed.

C03e-MG does not reinterpret that deny-all baseline as a final allow-bearing production policy.

### Empty bounded current registry

Exact source:

`crates/prw-registry/src/lib.rs`

Exact C03e-MF blob:

`98efd22858960cd441237049a5578a78eecc13ab`

Existing exact type:

`WorkspaceDeviceRegistry`

is the bounded in-memory current membership/device registry used to revalidate authenticated application-session identity against current membership/device state.

Existing constructor:

`WorkspaceDeviceRegistry::new()`

creates exactly one empty registry.

An empty registry contains no active membership and no registered device, so current-session registry revalidation fails closed until separately reviewed population/hydration is selected.

C03e-MG does not select any membership insertion, device registration, lifecycle transition, transport binding, durable-registry hydration, synchronization, refresh or mutation source.

### Shared current capability authority

Exact source:

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`

Exact C03e-MF blob:

`60307fff4dd0fd573192ba6e6fab9dedd3321dda`

Existing exact type:

`SharedCurrentCapabilityAuthority<P>`

retains exactly one `WorkspaceDeviceRegistry` and one policy `P` inside one shared current-state `Arc<RwLock<_>>`.

Existing constructor:

`SharedCurrentCapabilityAuthority::new(registry, policy)`

performs ownership composition only. It performs no authorization, registry mutation, network I/O, task spawn or readiness publication.

Clones share only the outer current-authority allocation; registry/policy snapshots are not cloned per worker.

### Existing C03e-MF population wrapper

Exact source:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact C03e-MF blob:

`8bfa387edd90c26fa35414370368d0875c1da16a`

Existing C03e-MF wrapper:

`linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_session_authentication(...)`

still accepts one already-populated:

`SharedCurrentCapabilityAuthority<P>`

beside the still-unresolved expected-request receiver, admission timing and callbacks.

The MF wrapper already constructs exactly one process-local `SessionAuthenticationService` and delegates exactly once to the MD same-custody population path.

C03e-MG selects only the current capability-authority input immediately above this MF seam.

### Existing test construction is not production provenance

Exact inherited `linux_bootstrap.rs` blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

Synthetic tests construct examples such as:

- `WorkspaceDeviceRegistry::new()`;
- `BoundedLocalReadPolicy::allow_local_reads()`;
- `mpsc::channel(..., 1)`;
- fixed test admission timing;
- no-op test callbacks.

C03e-MG does not promote those test-only allow/channel/timing/callback values into production provenance.

The MG policy selection instead relies on the separately named production-specific `ProductionRemoteCapabilityDenyAllPolicy` and the explicit empty-registry fail-closed behavior.

## Selected production population boundary

C03e-MG selects exactly one fail-closed current capability-authority population rule:

1. construct exactly one fresh `WorkspaceDeviceRegistry::new()`;
2. construct/use exactly one `ProductionRemoteCapabilityDenyAllPolicy` value;
3. compose those exact values exactly once through `SharedCurrentCapabilityAuthority::new(registry, policy)`;
4. move that exact `SharedCurrentCapabilityAuthority<ProductionRemoteCapabilityDenyAllPolicy>` by value into the existing C03e-MF session-authentication population wrapper exactly once;
5. return the exact MF result unchanged;
6. add no invocation site.

The selected baseline is intentionally fail-closed twice:

- the current registry starts empty;
- the production policy grants no represented capability.

No positive grant can be inferred from this selection.

## Distinction from production durable capability authority

C03e-MG does not modify or replace:

`ProductionDurableCapabilityAuthority`

and does not adapt its durable registry custody into `WorkspaceDeviceRegistry`.

The two authority lanes remain distinct:

- `SharedCurrentCapabilityAuthority<ProductionRemoteCapabilityDenyAllPolicy>` is the existing current in-memory registry/policy authority used by the staged remote worker path;
- `ProductionDurableCapabilityAuthority` retains the separately materialized durable-registry custody and its own production deny-all bridge policy.

C03e-MG selects no synchronization, mirroring, snapshot conversion or shared inner store between those authorities.

No durable authority is downgraded into an in-memory snapshot.

## Allow-bearing production policy remains separately gated

C03e-MG intentionally selects no allow-bearing production capability policy.

It does not select:

- `BoundedLocalReadPolicy::allow_local_reads()`;
- a configured `BoundedLocalManagementPolicy`;
- role-to-capability translation;
- remote grant configuration;
- policy file/database/provider source;
- dynamic policy reload;
- policy mutation authority.

A future transition away from the deny-all baseline requires a separately reviewed production policy/provenance gate.

## Current registry hydration remains separately gated

C03e-MG intentionally selects no production population of membership/device entries into `WorkspaceDeviceRegistry`.

It does not select:

- loading current membership/device state from the durable registry;
- copying durable registry state into memory;
- live watch/refresh;
- startup snapshot hydration;
- membership/device mutation source;
- transport identity hydration;
- retry/fallback/cache semantics.

A future non-empty current registry population requires a separately reviewed authority/provenance gate.

## Expected-request ingress remains separately unresolved

C03e-MG does not select:

- expected-request sender/producer;
- channel capacity;
- producer lifetime;
- dispatcher production source;
- verifier-time production source.

Observed `mpsc::channel(...)` values in tests remain synthetic only.

## Admission timing remains separately unresolved

Existing `RemoteSessionRealAdmissionTiming` requires caller-supplied:

- challenge-validity range;
- authentication current time;
- application-lease range.

C03e-MG does not select any of those values, their clock source or timing policy.

Synthetic `1..2`/`1` test timing remains test-only.

## Callbacks remain separately unresolved

C03e-MG does not select production semantics for:

- completion callback;
- rejection callback;
- repeated-admission-failure callback.

Existing no-op test callbacks are not production provenance.

## Requester/rendezvous remains separately unresolved

C03e-MG does not select:

- requester/rendezvous policy bindings;
- policy-source production population;
- provider type;
- provider capacity;
- provider population;
- registration strategy;
- final requester/rendezvous higher-owner join.

## Immediate later source-materialization ceiling

A later separately executed source checkpoint may materialize this selection in exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

The additive source shape is ceilinged to one crate-private dormant composition wrapper around the existing C03e-MF wrapper.

Conceptually the future wrapper may accept the same still-unresolved typed inputs as MF except `SharedCurrentCapabilityAuthority<P>`, then:

1. construct exactly one `WorkspaceDeviceRegistry::new()`;
2. construct/use exactly one `ProductionRemoteCapabilityDenyAllPolicy`;
3. construct exactly one `SharedCurrentCapabilityAuthority::new(registry, policy)`;
4. pass that exact authority into the existing MF wrapper exactly once;
5. return the existing MF result/error unchanged.

Its output policy type is fixed to:

`ProductionRemoteCapabilityDenyAllPolicy`

rather than exposing an arbitrary caller-selected `P` at this new population seam.

Because all three selected constructors/compositions are infallible, the later wrapper must not invent a new population error solely for this baseline construction.

Mechanical helper spelling may adjust only for Rust naming/formatting while preserving this semantic boundary.

The later source checkpoint must remain dormant and add no invocation site.

## Explicit non-selection

C03e-MG does not select or authorize:

- positive capability grants;
- allow-bearing production policy provenance;
- current registry hydration/population;
- durable-to-current registry snapshot conversion;
- registry synchronization/watch/refresh;
- expected-request sender/producer;
- expected-request channel capacity;
- dispatcher production source;
- verifier-time production source;
- challenge-validity timing;
- authentication clock/current-time source;
- application-lease timing;
- completion callback policy;
- rejection callback policy;
- admission-failure callback policy;
- requester/rendezvous production bindings/provider population;
- final requester/rendezvous higher-owner join;
- LX companion invocation;
- mutation of `linux_bootstrap.rs`;
- mutation of `prw-policy` or `prw-registry` algorithms;
- `main.rs` or `run()` mutation;
- executable caller/input assembly;
- endpoint bind/listener/readiness/publication changes;
- runtime/network activation;
- authentication/authorization/trust weakening;
- identity/address/correlation conflation;
- manifest/lockfile/workflow/Android/packaging/systemd/credential/certificate/trust/RBAC/repository-configuration changes;
- merge or ready-for-review conversion;
- deploy/restart/recovery;
- PR close;
- branch deletion;
- force update/rebase/squash/history rewrite;
- destructive cleanup.

## Identity and authority invariants

Remain distinct:

- `SharedCurrentCapabilityAuthority<ProductionRemoteCapabilityDenyAllPolicy>`;
- `WorkspaceDeviceRegistry` current in-memory state;
- `ProductionDurableRegistryRuntimeCustody`;
- raw `ProductionDurableCapabilityAuthority`;
- `SessionAuthenticationService`;
- authenticated `DeviceId` / `AuthenticatedDeviceSession`;
- requester/rendezvous authority;
- current `PeerConnectivityIdentity`;
- reachability/socket address;
- PRWM `request_id` correlation.

Session authentication establishes authenticated application-session identity; the MG current authority remains fail-closed and does not itself establish positive authorization.

`DeviceId` remains logical identity.

`PeerConnectivityIdentity` remains current transport-authority output.

IP/port remains reachability/addressing, not logical identity.

PRWM `request_id` remains correlation only.

## Validation and closure requirement

C03e-MG is documentation-only selection.

Exact-final-head validation is required before closure. Skipped workflows are not PASS and superseded-head workflows are not closure evidence.

Closure also requires immutable canonical Google Drive evidence with exact-title uniqueness and raw-byte/hash readback verification.

## STOP boundary

After verified C03e-MG closure: STOP.

C03e-MG selects only the fail-closed current capability-authority population boundary above. It does not pre-authorize its later source materialization or any other unresolved production provenance domain.
