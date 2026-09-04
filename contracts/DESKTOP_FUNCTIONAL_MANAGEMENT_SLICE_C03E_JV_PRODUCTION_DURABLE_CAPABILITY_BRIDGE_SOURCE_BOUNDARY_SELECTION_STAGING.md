# C03e-JV — Production Durable Capability Bridge Source Boundary Selection

Status: `SELECTION_STAGING`

Target gate:

`C03E_JV_PRODUCTION_DURABLE_CAPABILITY_BRIDGE_SOURCE_BOUNDARY_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_BRIDGE_SOURCE_BOUNDARY_SELECTION`

## 1. Purpose

C03e-JV is a documentation-only selection checkpoint after closed C03e-JU.

It performs the fresh exact-head audit required by JU/JT and selects the smallest next source boundary for the provider-aware Phase 143 durable capability bridge.

JV does not materialize Rust/source code.

## 2. Exact predecessor authority

Closed predecessor:

`C03e-JU — Production Durable Session-Transport Validator Source Materialization`

Predecessor branch:

`phase-152-c03e-ju-production-durable-session-transport-validator-source-materialization`

Exact predecessor head / intended merge base:

`489736d248804740c0a17e2c6b6ca1b148cfd783`

Exact predecessor tree:

`e75c4a56378560b30f00711ff8ffb6032331ddd7`

Exact final JU durable-registry store blob:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

`216e0dc184f8fd8b4c4ba94ea31295e55f962cd1`

JU gate:

`C03E_JU_PRODUCTION_DURABLE_SESSION_TRANSPORT_VALIDATOR_SOURCE_MATERIALIZED`

JU closure:

`CLOSED_PRODUCTION_DURABLE_SESSION_TRANSPORT_VALIDATOR_SOURCE_MATERIALIZATION`

## 3. Fresh audit findings

The exact JU head still contains the existing synchronous/in-memory Phase 143 bridge in:

`crates/prw-remote-bridge/src/lib.rs`

Exact audited blob:

`7b1c5c62339983da6ae2556f73510d7582ec0c5b`

The production crate root remains:

`crates/prw-remote-bridge/src/root.rs`

Exact audited blob:

`8b829f503380b3d02e8a91a9743017046d8c0b92`

`root.rs` already privately mounts `lib.rs` as `legacy_bridge` and publicly re-exports its public API with:

```text
pub use legacy_bridge::*;
```

Therefore a new public sibling bridge type added to `lib.rs` requires no `root.rs` change.

The exact `prw-remote-bridge` manifest blob is:

`5fd48263be415aac28dee1c71a4031a4a02ad36c`

It already depends on `prw-registry` and `prw-policy`.

Therefore the selected durable bridge can be materialized without manifest or lockfile mutation.

The exact audited `prw-policy` source blob is:

`3745024b5b222fcb36244222fad3c9c05a59cece`

The existing `PolicyEvaluator` remains principal-agnostic and evaluates exactly one `Capability` to `Decision`.

JV does not change that interface or select an allow-bearing production policy.

## 4. Existing synchronous bridge remains unchanged

The exact current synchronous bridge is:

`CapabilityBridge<'a, P>`

with custody over:

- `&WorkspaceDeviceRegistry`;
- `&P` where `P: PolicyEvaluator`.

Its existing synchronous authorization sequence performs:

1. request-kind validation;
2. lease-time validation;
3. in-memory authenticated-session validation;
4. a separate in-memory transport-identity validation;
5. PRWC decode;
6. capability derivation;
7. policy evaluation;
8. private `AuthorizedCapabilityRequest` construction.

JV does not remove, rename, repurpose or weaken this bridge.

The durable bridge is selected as a sibling provider-backed semantic path.

## 5. Durable prerequisite now exists

Closed JU materialized exactly:

`DurableRegistryEtcdStore::validate_authenticated_session_and_transport_identity(...)`

The exact JU implementation uses one authoritative `linearizable_pair_get(...)` and validates authenticated-session binding plus presented transport from the same decoded current device observation.

The durable bridge must use this combined validator directly.

It must not recreate the rejected two-read sequence:

```text
validate_authenticated_session(...).await
then
validate_transport_identity(...).await
```

## 6. Selected source owner

The immediate Rust/source successor after JV may change exactly one repository path:

`crates/prw-remote-bridge/src/lib.rs`

No other source, test, manifest, lockfile, root module, workflow or executable path is selected for the immediate materialization successor.

## 7. Selected durable bridge type

JV retains the JT-selected public type name exactly:

`DurableCapabilityBridge<'a, P>`

with exact conceptual custody:

```text
DurableCapabilityBridge<'a, P: PolicyEvaluator> {
    registry: &'a mut DurableRegistryEtcdStore,
    policy: &'a P,
}
```

The durable bridge accepts semantic durable-registry custody only.

It must not accept or expose:

- a raw etcd client;
- `DurableRegistryEtcdExecutor`;
- provider endpoint strings;
- credentials;
- TLS/auth/RBAC material;
- service URLs;
- registry snapshots/mirrors/caches;
- Agent runtime ownership.

## 8. Selected constructor

The immediate source successor may add exactly one constructor with semantics equivalent to:

```text
pub const fn new(
    registry: &'a mut DurableRegistryEtcdStore,
    policy: &'a P,
) -> Self
```

Construction performs no I/O and does not validate or authorize any request.

## 9. Selected async authorization signature

The immediate source successor may add exactly one public async authorization operation with semantic signature:

```text
pub async fn authorize(
    &mut self,
    presented_transport_identity: TransportIdentity,
    lease: &RemoteSessionLease,
    now_unix_seconds: u64,
    frame: &ControlFrame,
) -> Result<AuthorizedCapabilityRequest, DurableCapabilityBridgeError>
```

Rustfmt may choose exact line layout, but the argument authority and return shape above may not widen.

The method does not accept caller-selected principal, capability, device ID, workspace/user ID, registry key, policy decision or authorized-request constructor data.

## 10. Exact authorization order

The selected durable `authorize(...)` must preserve this order exactly:

1. reject any outer frame whose kind is not `ControlMessageKind::Request`;
2. validate the verifier-owned `RemoteSessionLease` time bounds;
3. call `DurableRegistryEtcdStore::validate_authenticated_session_and_transport_identity(...)` exactly once using:
   - `lease.session()`;
   - the caller-presented `TransportIdentity`;
4. only after durable session+transport validation succeeds, decode the bounded PRWC command from `frame.payload()`;
5. derive the exact required `Capability` only from the decoded `BridgeCommand`;
6. evaluate that exact capability through the already-held `PolicyEvaluator`;
7. only after an explicit `Decision::Allow`, construct the existing private-field `AuthorizedCapabilityRequest` inside the same module.

No operation may be dispatched by this selected method.

## 11. No second durable read

The durable bridge must call the combined JU validator exactly once per authorization attempt.

It must not additionally call:

- `membership(...)`;
- `device(...)`;
- `validate_authenticated_session(...)`;
- `current_transport_identity(...)`;
- `validate_transport_identity(...)`;
- another combined validator call;
- a raw `linearizable_get(...)`;
- a raw `linearizable_pair_get(...)`.

The bridge owns authorization ordering, not provider-read composition.

## 12. Existing private authorized-request construction remains authoritative

The current `AuthorizedCapabilityRequest` fields remain private.

The immediate successor must construct it only inside the existing `prw-remote-bridge/src/lib.rs` module after all gates succeed.

JV explicitly rejects adding:

- a public `AuthorizedCapabilityRequest::new(...)`;
- a public unchecked constructor;
- an Agent-side duplicate authorization constructor;
- a dispatcher API that can fabricate authorization.

## 13. Selected durable bridge error envelope

The immediate source successor may add the public bounded error enum named exactly:

`DurableCapabilityBridgeError`

with exactly two semantic categories:

```text
Bridge(RemoteBridgeError)
Authority(DurableRegistryEtcdStoreError)
```

The enum may derive bounded comparison/debug traits already supported by its contained types.

It may implement `Display` and `std::error::Error` without exposing provider endpoint, credential, key material or record bytes.

It may add only the minimal conversion/helper plumbing required by this exact two-category envelope.

## 14. Exact semantic mapping from durable registry errors

The selected mapping is:

```text
DurableRegistryEtcdStoreError::Semantic(RegistryError::TransportIdentityMissing)
    -> Bridge(RemoteBridgeError::TransportIdentityRejected)

DurableRegistryEtcdStoreError::Semantic(RegistryError::TransportIdentityMismatch)
    -> Bridge(RemoteBridgeError::TransportIdentityRejected)

DurableRegistryEtcdStoreError::Semantic(any other RegistryError)
    -> Bridge(RemoteBridgeError::RegistryRejected)

any non-Semantic DurableRegistryEtcdStoreError
    -> Authority(original durable-store error)
```

This preserves the existing coarse Phase 143 external rejection taxonomy while retaining provider/canonical-authority failures distinctly.

`DeviceRevoked`, `DeviceUnknown`, `MembershipUnknown`, `MembershipNotActive` and `SessionBindingMismatch` remain registry rejection because the combined validator proves those failures before successful transport-currentness validation.

No durable provider/canonical failure may become:

- authorization success;
- policy denial;
- fabricated request-codec failure;
- fabricated transport mismatch.

## 15. Selected bridge-local error conversion

Existing `RemoteBridgeError` failures from request-kind, lease, PRWC decode and capability denial may be wrapped directly as:

`DurableCapabilityBridgeError::Bridge(...)`

The source successor may add one minimal exact `From<RemoteBridgeError>` implementation if useful.

No broad generic conversion trait or provider abstraction is selected.

## 16. No durable `process_request(...)` yet

JV deliberately does not select a provider-aware `process_request(...)` in the immediate successor.

The immediate durable bridge source boundary ends at successful construction of `AuthorizedCapabilityRequest`.

Existing `authorized_request_dispatch` remains the separately reviewed dispatch helper.

Any composition that holds durable registry custody, authorizes asynchronously, releases custody and then dispatches is a later Agent/runtime-custody checkpoint.

This keeps the future durable-registry mutex outside dispatcher execution.

## 17. Policy remains generic and non-production-enabling

The immediate bridge remains generic over `P: PolicyEvaluator` so its authorization semantics can be compiled and tested independently of production policy custody.

JV does not materialize:

`ProductionRemoteCapabilityDenyAllPolicy`

and does not select any policy capable of returning `Decision::Allow` as production authority.

The later deny-all production policy prerequisite remains separately gated.

A generic policy parameter in the bridge is not authority to select or activate an allow-bearing production policy.

## 18. Focused test ceiling

The immediate one-file source successor may add only focused tests inside the same `lib.rs` path for the selected durable-bridge seam.

Permitted tests include:

1. pure error-mapping tests proving transport-missing/mismatch -> `TransportIdentityRejected`;
2. pure error-mapping tests proving membership/device/session semantic errors -> `RegistryRejected`;
3. pure error-mapping tests proving non-semantic durable-store failures remain `Authority(...)`;
4. bounded `Display`/`Error::source` shape if implemented;
5. compile/type-shape checks that require no live provider, process-global environment mutation or network activation.

The immediate successor need not invent a fake production provider or alter durable-store construction solely for tests.

## 19. One-file materialization ceiling

After JV closes, the immediate source-materialization successor may change only:

`crates/prw-remote-bridge/src/lib.rs`

It may add only:

1. imports for the already-existing durable registry store/error and `RegistryError`;
2. `DurableCapabilityBridgeError` with the exact two-category envelope;
3. minimal bounded `Display` / `Error` / exact conversion-helper plumbing;
4. `DurableCapabilityBridge<'a, P>` with exact mutable semantic-store + policy custody;
5. the exact async `authorize(...)` sequence selected above;
6. focused same-file tests under section 18;
7. strictly local rustfmt/lint corrections required by the exact added source shape.

## 20. Immediate successor exclusions

The immediate materialization successor must not change:

- `crates/prw-remote-bridge/src/root.rs`;
- any other `prw-remote-bridge` module;
- `crates/prw-remote-bridge/tests/*`;
- `crates/prw-policy/*`;
- `crates/prw-registry/*`;
- `crates/prw-agent/*`;
- any `Cargo.toml`;
- `Cargo.lock`;
- workflows;
- contracts other than a later checkpoint's own selection artifact;
- Android/application code;
- packaging/service/systemd paths;
- `run()` or `main.rs`.

It must not add durable dispatch composition, production policy custody, Agent authority custody, aggregate input replacement or runtime activation.

## 21. Identity and authorization invariants

JV preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Specifically:

- authenticated `DeviceId` remains logical device identity;
- `TransportIdentity` remains current transport evidence;
- transport-key rotation does not redefine logical device identity;
- IP/port is not identity or authorization;
- PRWM request ID remains transaction correlation only;
- lease/session correlation is not device identity;
- successful durable read is not capability authorization;
- successful transport match is not capability authorization;
- successful PRWC decode is not capability authorization;
- capability derivation is not capability authorization;
- only explicit `Decision::Allow` after all prior gates permits private `AuthorizedCapabilityRequest` construction.

## 22. No runtime or network activation

Neither JV nor its immediate one-file materialization successor authorizes:

- listener/bind/readiness activation;
- remote-process spawn;
- terminal/forwarding/file execution;
- requester/rendezvous invocation;
- candidate publication/traversal/dialing;
- retry/reconnect/rebind/rebootstrap;
- service/systemd mutation;
- deployment/restart/recovery.

## 23. JV repository scope

JV itself is documentation-only.

The exact JU -> JV compare must contain exactly one changed path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_JV_PRODUCTION_DURABLE_CAPABILITY_BRIDGE_SOURCE_BOUNDARY_SELECTION_STAGING.md`

Any Rust/source, manifest, lockfile, workflow, Android, packaging, Agent, registry, policy, bridge implementation, `main.rs` or runtime path change blocks JV closure.

## 24. Validation semantics

Exact-final-head CI must be tied only to the exact final JV head.

Path-filtered workflows reporting `SKIPPED` remain `SKIPPED` and must not be represented as PASS.

A successful docs-only JV validation proves only repository integrity plus the selection artifact. It does not validate the future durable bridge implementation.

## 25. Successor rule

After JV closure: **STOP**.

The immediate successor may only materialize the one-file durable bridge source boundary selected in sections 7–20.

After that source materialization closes, a fresh exact-head audit is required before selecting/materializing `ProductionRemoteCapabilityDenyAllPolicy`, Agent durable capability-authority custody, aggregate input replacement, executable caller wiring, startup/exit policy or runtime/network activation.

## 26. Explicit exclusions

C03e-JV does not perform or authorize:

- Rust/source materialization in JV;
- a second durable provider read;
- registry mutation, scan, Watch, Lease, TTL, cache or mirror authority;
- production policy materialization;
- any allow-bearing production policy;
- role-to-capability mapping;
- local policy promotion;
- `ProductionDurableCapabilityAuthority` materialization;
- `SharedCurrentCapabilityAuthority` mutation;
- `LinuxAgentRemoteProcessOperationInputs` mutation;
- session/expected-request/dispatcher/timing/callback production population;
- requester/rendezvous custody population/invocation;
- operation-factory invocation;
- remote-process companion spawn;
- `run()` or `main.rs` mutation;
- listener/readiness/runtime/network activation;
- service/systemd/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment/restart/recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 27. Closure requirements

C03e-JV may close only after:

1. exact JV branch head is re-read;
2. exact JU -> JV merge base equals `489736d248804740c0a17e2c6b6ca1b148cfd783`;
3. aggregate changed-file set contains exactly the single JV contract path;
4. exact-final-head CI reaches terminal expected conclusions;
5. immutable Drive audit evidence is published and raw-readback verified;
6. exact-title Drive uniqueness is verified under the canonical audit parent;
7. the JV PR remains draft/open/unmerged at the exact audited head.

After closure, STOP before any Rust/source materialization.