# Desktop Functional Management Slice C03e-KB — Production Durable Capability-Authority Authorization Invocation Boundary Selection

Status: **SELECTION STAGING — VALIDATION PENDING**

Target gate:

`C03E_KB_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_AUTHORIZATION_INVOCATION_BOUNDARY_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_AUTHORIZATION_INVOCATION_BOUNDARY_SELECTION`

## 1. Scope

C03e-KB is a **selection-only** checkpoint.

It selects the exact dormant Agent-side operation-specific invocation seam that may later call the already-materialized provider-aware `DurableCapabilityBridge::authorize(...)` through the C03e-KA production durable capability-authority custody.

This checkpoint performs no Rust/source/runtime materialization.

It does not authorize aggregate replacement, executable caller wiring, request receive-loop activation, dispatcher execution, response I/O, listener activation, policy widening, deployment, merge, or production enablement.

## 2. Exact predecessor authority

Closed predecessor:

`C03e-KA — Production Durable Capability-Authority Custody Source Materialization`

Predecessor branch:

`phase-152-c03e-ka-production-durable-capability-authority-custody-source-materialization`

Exact predecessor head:

`d9b90a76f419631321c3c5980639f991dbcba33a`

Exact predecessor tree:

`bd7a3f0ed98c18baa3fffdc042b21ba9ae0accf1`

Exact predecessor source blob:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

`ef52cb0a85c89ec5470e5d549e488e33cf5a6d80`

Predecessor gate:

`C03E_KA_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_CUSTODY_SOURCE_MATERIALIZED`

Predecessor closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_CUSTODY_SOURCE_MATERIALIZATION`

KA materialized dormant custody only:

```text
ProductionDurableCapabilityAuthority {
    registry_custody:
        Arc<tokio::sync::Mutex<ProductionDurableRegistryRuntimeCustody>>,
    policy:
        ProductionRemoteCapabilityDenyAllPolicy,
}
```

KA did not materialize an authorization invocation method.

## 3. Fresh successor namespace audit

Before this selection branch was created:

- branch search for `phase-152-c03e-kb` returned zero results;
- PR search for `C03e-KB` returned zero results;
- the exact contract path selected by this checkpoint did not exist at the exact KA head;
- KA branch was freshly re-read at exact head `d9b90a76f419631321c3c5980639f991dbcba33a` and tree `bd7a3f0ed98c18baa3fffdc042b21ba9ae0accf1`.

This KB branch is based only on that exact KA head.

## 4. Fresh source authorities audited at exact KA head

The following exact source authorities were re-read before this selection.

### 4.1 Agent durable-registry/capability custody

Path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Blob:

`ef52cb0a85c89ec5470e5d549e488e33cf5a6d80`

Relevant facts:

- `ProductionDurableRegistryRuntimeCustody` privately owns one `DurableRegistryEtcdStore`;
- no generic store getter/extraction seam exists;
- `ProductionDurableCapabilityAuthority` privately owns one `Arc<tokio::sync::Mutex<ProductionDurableRegistryRuntimeCustody>>`;
- it also privately owns concrete `ProductionRemoteCapabilityDenyAllPolicy`;
- it currently has only a side-effect-free constructor;
- it performs no authorization and no mutex lock acquisition.

### 4.2 Existing provider-aware durable capability bridge

Path:

`crates/prw-remote-bridge/src/lib.rs`

Blob:

`ad6833cc4e71a372810b260f157126a3df6645e5`

Relevant existing public authority:

```text
DurableCapabilityBridge<'a, P: PolicyEvaluator + Sync> {
    registry: &'a mut DurableRegistryEtcdStore,
    policy: &'a P,
}
```

Existing authorization signature:

```text
pub async fn authorize(
    &mut self,
    presented_transport_identity: TransportIdentity,
    lease: &RemoteSessionLease,
    now_unix_seconds: u64,
    frame: &ControlFrame,
) -> Result<AuthorizedCapabilityRequest, DurableCapabilityBridgeError>
```

Existing authorization order is already authoritative and must not be duplicated in Agent:

1. outer `ControlMessageKind::Request` validation;
2. lease-time validation;
3. exactly one combined durable current authenticated-session + presented-transport validation;
4. PRWC decode;
5. exact capability derivation;
6. policy evaluation;
7. private-field `AuthorizedCapabilityRequest` construction only after `Decision::Allow`.

`prw-remote-bridge` remains the sole constructor authority for `AuthorizedCapabilityRequest`.

### 4.3 Existing post-auth capability same-stream custody

Path:

`crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`

Blob:

`8294cd236dcc497da87e859afdf675b79aa24085`

Existing bridge-owned type:

```text
PostAuthCapabilityTransaction {
    request_frame: ControlFrame,
    stream: MeshControlStream,
}
```

Existing narrow borrow:

```text
pub const fn request_frame(&self) -> &ControlFrame
```

Existing response path consumes the transaction only when sending one response frame:

```text
pub async fn send_response_frame(
    self,
    response_frame: &ControlFrame,
) -> Result<(), CapabilityRequestWireError>
```

Therefore an authorization operation can borrow the transaction without consuming or duplicating same-stream custody.

### 4.4 Remote-bridge production root

Path:

`crates/prw-remote-bridge/src/root.rs`

Blob:

`8b829f503380b3d02e8a91a9743017046d8c0b92`

The production root already exposes:

`pub mod post_auth_control_stream_ingress;`

and re-exports the legacy bridge public API, including `DurableCapabilityBridge`, `DurableCapabilityBridgeError`, `RemoteSessionLease`, and `AuthorizedCapabilityRequest`.

No new bridge re-export is required by the selected Agent seam.

### 4.5 Agent dependency boundary

Path:

`crates/prw-agent/Cargo.toml`

Blob:

`4c70d6be9b56f39edc10810eefa3428314ed7559`

The Agent already directly depends on:

- `prw-connectivity`;
- `prw-policy`;
- `prw-registry`;
- `prw-remote-bridge`;
- `prw-session`;
- `tokio` with `sync` support.

The Agent does **not** directly depend on `prw-remote-transport`.

C03e-KB deliberately preserves that boundary.

## 5. Problem statement

The durable bridge requires `&ControlFrame`, but adding a direct `prw-remote-transport` dependency to Agent solely to name `ControlFrame` would widen the dependency graph and likely require manifest/lockfile mutation.

That widening is unnecessary because the existing bridge-owned `PostAuthCapabilityTransaction` already owns the exact received `ControlFrame` and exposes it by immutable borrow.

The safest production-authority invocation seam is therefore an Agent method that accepts the existing bridge-owned transaction, not a raw transport frame.

## 6. Selected Agent operation-specific method

A later source-materialization checkpoint may add exactly one dormant method to `ProductionDurableCapabilityAuthority`:

```text
pub async fn authorize_capability_transaction(
    &self,
    presented_transport_identity: TransportIdentity,
    lease: &RemoteSessionLease,
    now_unix_seconds: u64,
    transaction: &PostAuthCapabilityTransaction,
) -> Result<AuthorizedCapabilityRequest, DurableCapabilityBridgeError>
```

The exact source spelling may use fully qualified imports, but the semantic input/output boundary above is selected and normative.

Because the containing Agent module remains private to the crate, `pub` source spelling does not create an externally reachable `prw-agent` API.

## 7. Selected input provenance law

The method parameters have strict provenance semantics.

### 7.1 Presented transport identity

`presented_transport_identity` is transport evidence only.

A future production caller may supply it only from the already-authenticated lower-transport peer/session custody.

It must not be:

- a logical `DeviceId` substitute;
- request-decoded arbitrary bytes;
- an IP address or socket endpoint;
- a cached historical transport identity;
- fabricated from PID/UID/GID;
- caller-selected alternate transport state.

Transport-key rotation does not redefine logical device identity.

### 7.2 Remote session lease

`lease` must be an existing authenticated logical-session lease produced by the reviewed session-authentication boundary.

This method does not construct, renew, extend, reinterpret, or fabricate the lease.

Authentication success remains distinct from authorization.

### 7.3 Current-time input

`now_unix_seconds` is verifier/runtime-owned time input used only by the existing bridge lease check.

It must not come from request payload, requester-controlled metadata, PRWM correlation, device clock claims, or remote peer data.

KB does not yet select the executable production clock source; that remains a later caller-composition checkpoint.

The immediate source successor only plumbs this explicitly supplied verifier-owned scalar to the existing bridge unchanged.

### 7.4 Capability transaction

`transaction` must be the exact existing `PostAuthCapabilityTransaction` that owns the already-read capability-family `ControlFrame` and same authenticated stream.

The authorization method only borrows it.

It must not:

- clone or reconstruct the frame;
- consume the transaction;
- call `into_parts()`;
- send a response;
- accept another stream;
- perform another read;
- retry ingress;
- reclassify requester/rendezvous or candidate-publication families.

## 8. Selected mutex/registry custody law

The method must acquire exactly one guard through:

```text
self.registry_custody.lock().await
```

No `try_lock` fallback, timeout fallback, second mutex, retry loop, detached lock holder, cloned durable snapshot, alternate store, or generic inner-store extraction is selected.

After acquisition, the method may access the existing private `store` field only because both custody types live in the same private Agent module.

No getter is added.

## 9. Selected bridge construction and invocation law

Inside one lexical lock scope, the method must construct exactly one bridge:

```text
DurableCapabilityBridge::new(
    &mut registry_custody.store,
    &self.policy,
)
```

and invoke exactly once:

```text
bridge.authorize(
    presented_transport_identity,
    lease,
    now_unix_seconds,
    transaction.request_frame(),
).await
```

The Agent method must not duplicate any of the bridge's request-kind, lease, registry, transport, decode, capability, policy, or authorized-request construction logic.

It must not call the durable registry validator separately before or after the bridge.

It must not perform a second provider read for authorization.

## 10. Selected lock lifetime

The mutex guard is held only for the bounded bridge authorization transaction.

The selected lexical lifetime is conceptually:

```text
let result = {
    let mut registry_custody = self.registry_custody.lock().await;
    let mut bridge = DurableCapabilityBridge::new(
        &mut registry_custody.store,
        &self.policy,
    );
    bridge
        .authorize(
            presented_transport_identity,
            lease,
            now_unix_seconds,
            transaction.request_frame(),
        )
        .await
};

result
```

The exact formatting is non-normative; the lock-lifetime semantics are normative.

The guard must be dropped before any later:

- dispatcher execution;
- filesystem/terminal/forwarding operation;
- response encoding;
- response stream write/finish;
- worker cancellation/join;
- repeated ingress;
- runtime shutdown;
- unrelated registry operation.

No lock guard may escape in a returned value.

## 11. Selected error surface

The Agent method returns the existing:

`DurableCapabilityBridgeError`

**unchanged**.

No new Agent-specific wrapper or semantic remap is selected in the immediate successor.

This preserves the bridge's already-reviewed distinction:

- `Bridge(RemoteBridgeError)` for request/lease/semantic registry-or-transport/codec/policy rejection;
- `Authority(DurableRegistryEtcdStoreError)` for non-semantic provider/currentness/canonical-authority failure.

In particular, the Agent must not collapse provider authority failure into ordinary capability denial and must not expose provider endpoint or credential detail in display text.

A later response-framing checkpoint may select how these authorization failures map to wire-visible response semantics; KB does not.

## 12. Dependency selection

The immediate source successor must use only dependencies already present in `prw-agent`.

Selected imports may come from:

```text
prw_connectivity::TransportIdentity
prw_remote_bridge::{
    AuthorizedCapabilityRequest,
    DurableCapabilityBridge,
    DurableCapabilityBridgeError,
    RemoteSessionLease,
    post_auth_control_stream_ingress::PostAuthCapabilityTransaction,
}
```

No direct `prw-remote-transport` dependency is selected.

No bridge API re-export/type alias for `ControlFrame` is selected.

No manifest or lockfile change is selected.

## 13. Same-stream custody preservation

Authorization borrows `&PostAuthCapabilityTransaction` and therefore does not consume the same-stream custody envelope.

On both authorization success and failure, ownership of the transaction remains with the caller.

This checkpoint does not decide whether a later failure path sends an error response, closes the stream, drops the transaction, or continues any loop.

Those are later response/lifecycle decisions.

The authorization method itself performs none of them.

## 14. Fail-closed production policy consequence

The stored policy remains exactly:

`ProductionRemoteCapabilityDenyAllPolicy`

It always returns:

`Decision::Deny`

Therefore, under the currently selected production policy, a structurally valid request that reaches policy evaluation cannot produce an `AuthorizedCapabilityRequest`; it is rejected with the existing capability-denied bridge error.

Materializing the selected method does **not** enable any remote capability.

Any future production policy capable of returning `Decision::Allow` still requires a new explicit checkpoint selecting its principal scope, decision source, currentness/reload semantics, missing/invalid-source behavior, custody/lifetime, binding dimension, and failure semantics.

## 15. Identity invariants preserved

The selected seam preserves the canonical identity law:

```text
PRW logical device/session identity
    -> registry/discovery
    -> current reachable endpoint/candidates
    -> authenticated transport
```

Specifically:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains transport evidence;
- PRWM request ID remains correlation only;
- IP/port remains transient reachability only;
- decode success is not authorization;
- connection success is not authorization;
- correlation success is not authorization;
- authentication success is not authorization;
- durable-registry currentness validation is necessary but not sufficient for authorization;
- only bridge policy `Decision::Allow` may construct the authorized request.

## 16. Rejected alternatives

C03e-KB explicitly rejects the following immediate designs.

### 16.1 Direct Agent `ControlFrame` dependency widening

Rejected:

- add `prw-remote-transport` to `prw-agent/Cargo.toml`;
- modify `Cargo.lock` solely to name `ControlFrame` in the Agent method.

Reason: existing bridge-owned transaction already provides the exact frame by immutable borrow.

### 16.2 Bridge `ControlFrame` re-export widening

Rejected:

- add a new `pub use prw_remote_transport::ControlFrame` or alias to `prw-remote-bridge` solely for Agent.

Reason: unnecessary bridge API expansion.

### 16.3 Generic store callback/extraction

Rejected:

- `with_store(...)` generic closure;
- `store()` / `store_mut()` / `into_store()`;
- raw executor/client extraction.

Reason: would widen durable-provider authority beyond one operation-specific authorization transaction.

### 16.4 Duplicate Agent authorization logic

Rejected:

- pre-validating request kind or lease in Agent;
- decoding PRWC in Agent;
- deriving capability in Agent;
- evaluating policy in Agent separately;
- constructing `AuthorizedCapabilityRequest` in Agent;
- direct registry session+transport validation outside the bridge.

Reason: `prw-remote-bridge` already owns the reviewed authorization sequence and constructor authority.

### 16.5 Lock across downstream work

Rejected:

- hold durable registry mutex while dispatching;
- hold it during response I/O;
- hold it during worker waits/cancellation;
- return any guard/bridge borrowing the guard.

### 16.6 Consuming capability transaction during authorization

Rejected:

- consume same-stream custody merely to authorize;
- split stream/frame during authorization;
- second stream read;
- response send inside the authority method.

## 17. Immediate source-materialization successor ceiling

After KB closes, the **first** source-materialization successor may change exactly one repository path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Permitted changes are limited to:

1. imports from already-direct dependencies required by the selected signature;
2. materialization of `authorize_capability_transaction(...)` on `ProductionDurableCapabilityAuthority`;
3. exactly one mutex acquisition and one lexical bridge invocation as selected above;
4. unchanged propagation of `DurableCapabilityBridgeError`;
5. focused same-file compile/type-shape tests or lint/rustfmt fixes needed for that exact method.

The first source successor must not change:

- `crates/prw-agent/Cargo.toml`;
- `Cargo.lock`;
- `crates/prw-remote-bridge/src/lib.rs`;
- `crates/prw-remote-bridge/src/root.rs`;
- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`;
- `crates/prw-registry`;
- `crates/prw-policy`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/main.rs`;
- workflows;
- Android sources;
- packaging/service/systemd/runtime configuration.

No second repository path is permitted in that immediate source successor.

## 18. Immediate source-successor validation requirements

The source successor must prove, from an exact KA/KB predecessor head:

- expected predecessor source blob guard before write;
- exact predecessor -> candidate compare;
- behind `0`;
- only the one selected source path changed;
- no manifest/lockfile drift;
- exact-final-head Rust validation SUCCESS;
- exact-final-head Android validation SUCCESS if triggered;
- conditional workflows reported only according to their actual terminal conclusions;
- draft/open/unmerged PR;
- immutable Drive audit with exact raw readback bytes/hash;
- post-publication branch/compare/PR re-read.

If CI requires a correction commit, only the corrected final head may carry final PASS authority.

## 19. STOP after immediate source successor

After the one-file authorization-method materialization closes: **STOP** again.

A fresh audit is required before selecting any of the following:

- replacement of `SharedCurrentCapabilityAuthority<P>` in `LinuxAgentRemoteProcessOperationInputs`;
- production population of the new durable authority into the remote-process aggregate;
- exact production source of `TransportIdentity` at the caller;
- exact production `RemoteSessionLease` custody/population;
- executable production clock source for `now_unix_seconds`;
- expected-request receiver/producer integration;
- dispatcher invocation after authorization;
- authorization-failure response framing;
- success-response same-stream write;
- repeated ingress or connection loop policy;
- completion/rejection/repeated-failure callbacks;
- startup/exit policy;
- listener/readiness/runtime/network activation.

## 20. Explicit exclusions for C03e-KB itself

C03e-KB does not authorize or perform:

- Rust/source materialization;
- manifest/lockfile mutation;
- direct `prw-remote-transport` Agent dependency;
- bridge API widening;
- provider read/write;
- registry mutation;
- mutex acquisition;
- authorization invocation;
- capability enablement;
- allow-bearing policy;
- aggregate replacement;
- expected-request production;
- dispatcher/provider production assembly;
- response I/O;
- repeated ingress;
- operation-factory invocation;
- companion spawn;
- `run()` or `main.rs` mutation;
- listener/bind/readiness/runtime/network activation;
- candidate publication/traversal/dialing/retry/reconnect/rebind/rebootstrap;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment/restart/recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion, or history rewrite.

## 21. Selection decision

C03e-KB selects:

```text
PostAuthCapabilityTransaction
    --immutable request-frame borrow-->
ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)
    --one mutex guard-->
ProductionDurableRegistryRuntimeCustody.store
    --borrowed only inside one lexical scope-->
DurableCapabilityBridge::authorize(...)
    -> Result<AuthorizedCapabilityRequest, DurableCapabilityBridgeError>
```

with same-stream transaction custody retained by the caller and no direct Agent dependency on `prw-remote-transport`.

Gate upon exact validation and immutable evidence:

`C03E_KB_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_AUTHORIZATION_INVOCATION_BOUNDARY_SELECTED`

Closure upon exact validation and immutable evidence:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_AUTHORIZATION_INVOCATION_BOUNDARY_SELECTION`
