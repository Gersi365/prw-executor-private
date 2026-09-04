# Phase 152 C03e-KF — Production Durable Capability Post-Auth Transaction Composition Selection

Status: `STAGING_SELECTION`

Gate on successful exact-final-head validation and immutable evidence publication:

`C03E_KF_PRODUCTION_DURABLE_CAPABILITY_POST_AUTH_TRANSACTION_COMPOSITION_SELECTED`

Intended closure token:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_POST_AUTH_TRANSACTION_COMPOSITION_SELECTION`

## 1. Purpose

C03e-KF selects only the first narrow Agent-side transaction composition that may consume the already-materialized dormant `ProductionDurableCapabilityAuthority` after one capability-family frame has already been read into exact same-stream custody.

This checkpoint is selection-only. It changes no Rust source and activates no runtime, listener, worker, dispatcher, provider, registry mutation, service, deployment, or production network path.

The selected later source seam must preserve the existing ownership and authority boundaries instead of adapting durable authority into the legacy in-memory `SharedCurrentCapabilityAuthority<P>` model.

## 2. Exact predecessor authority

Exact predecessor checkpoint:

`C03e-KE — Production durable capability authority population composition source materialization`

Exact predecessor branch:

`phase-152-c03e-ke-production-durable-capability-authority-population-composition-source-materialization`

Exact predecessor head / branch authority at selection audit:

`3e1c571d1492a51a2a5866ef13ae51270e675e8e`

Exact predecessor tree:

`8cb1ed9f335db37175533a1755fcfdaf6d5d9292`

KE remains draft, open and unmerged. Its exact final CI and immutable Drive evidence are already closed separately. C03e-KF does not alter or reopen KE.

## 3. Exact source observations at KE head

The selection is grounded in the exact KE source state, not in historical handoff assumptions.

### 3.1 Durable authority population already exists

Path:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Exact KE blob:

`c48003712ac20b86fc09ebdfb2ddb67afd44f649`

Existing helper:

`bootstrap_production_durable_capability_authority_from_systemd_credentials()`

It performs the already-selected one-shot composition:

`production durable-registry systemd custody -> provider bootstrap -> DurableRegistryEtcdStore -> ProductionDurableRegistryRuntimeCustody -> ProductionDurableCapabilityAuthority`

and returns dormant capability authority without performing semantic registry reads, authorization, dispatch, response I/O, repeated ingress, or runtime activation.

### 3.2 Durable authorization invocation already exists

Path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact KE blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

Existing operation-specific method:

`ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)`

Its selected input shape is already exact:

- one presented `TransportIdentity`;
- one borrowed `RemoteSessionLease`;
- verifier-owned `now_unix_seconds`;
- one borrowed `PostAuthCapabilityTransaction`.

It acquires the retained durable-registry mutex only across one `DurableCapabilityBridge::authorize(...)` invocation and returns `AuthorizedCapabilityRequest` or `DurableCapabilityBridgeError`. The mutex guard is released before return.

### 3.3 Exact same-stream post-auth capability custody already exists

Path:

`crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`

Exact KE blob:

`8294cd236dcc497da87e859afdf675b79aa24085`

Existing `PostAuthCapabilityTransaction` retains:

- the exact already-read bounded `ControlFrame`;
- the exact same already-accepted `MeshControlStream`.

Existing surfaces:

- `request_frame()` borrows the exact already-read frame;
- `send_response_frame(...)` consumes the transaction and sends one already-constructed response on the exact retained stream.

Therefore the selected durable composition must not accept another stream and must not read a second frame.

### 3.4 Bound authenticated session already owns transport evidence and lease

Path:

`crates/prw-remote-bridge/src/remote_session_binding.rs`

Exact KE blob:

`fcaa4960c7ec150d317e8aea197b5e936f3529a4`

Existing `BoundRemoteSession` retains the pair:

- immutable bound `TransportIdentity`;
- verifier-owned `RemoteSessionLease` containing the authenticated application session.

Existing accessors:

- `transport_identity()`;
- `lease()`;
- `session()`.

The selected production durable transaction composition must source presented transport evidence and lease only from this existing bound session. It must not accept a second caller-selected transport identity or lease.

### 3.5 Dispatch and response framing are already split from authorization

Path:

`crates/prw-remote-bridge/src/authorized_request_dispatch.rs`

Exact KE blob:

`d3c25ce18aa56a3924fe2ab2b5f82e3e81bea2aa`

Existing `dispatch_authorized_request(...)` consumes only a borrowed `AuthorizedCapabilityRequest` plus mutable `CapabilityDispatcher` and returns the bounded response `ControlFrame` or `RemoteBridgeError`.

This split allows durable-registry authorization custody to end before dispatcher side effects and response I/O.

### 3.6 The current executable aggregate still carries the legacy authority type

Path:

`crates/prw-agent/src/linux_bootstrap.rs`

At exact KE head, `LinuxAgentRemoteProcessOperationInputs<P, ...>` still contains:

`capability_authority: SharedCurrentCapabilityAuthority<P>`

and existing worker/executor paths remain generic over `P: PolicyEvaluator` and invoke the legacy in-memory `CapabilityBridge` route.

C03e-KF therefore does not select aggregate replacement, a wrapper from durable authority to `PolicyEvaluator`, an in-memory registry mirror, or any synthetic `SharedCurrentCapabilityAuthority` population.

## 4. Selected boundary

C03e-KF selects one later Agent-internal async operation on `AuthenticatedRemoteSessionRuntimeOwner` that processes exactly one already-read `PostAuthCapabilityTransaction` through the existing production durable capability authority.

Selected semantic name for the later helper:

`process_production_durable_capability_transaction`

Selected effective visibility:

crate-internal only. Exact Rust visibility spelling may use the narrowest form accepted by the existing private module structure and workspace Clippy without increasing effective API reachability.

The first source-materialization successor is limited to exactly one path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

No second repository path is authorized by this selection.

## 5. Exact selected call order

The later one-file source successor must preserve exactly this order:

1. receive from the caller one already-owned `PostAuthCapabilityTransaction`;
2. borrow the existing exact `BoundRemoteSession` retained by `AuthenticatedRemoteSessionRuntimeOwner`;
3. obtain presented transport evidence only through `BoundRemoteSession::transport_identity()`;
4. obtain the authenticated lease only through `BoundRemoteSession::lease()`;
5. use the caller/verifier-supplied `now_unix_seconds` exactly as provided, without deriving time from request payload, transport state, registry state, or system wall-clock inside the helper;
6. invoke `ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)` exactly once;
7. only after successful durable authorization, invoke existing `dispatch_authorized_request(...)` exactly once;
8. only after successful dispatch/response-frame construction, consume the exact `PostAuthCapabilityTransaction` through `send_response_frame(...)` exactly once;
9. return success only after that exact same-stream response send succeeds.

No durable-authority mutex may be held during dispatcher execution or response I/O; the existing authorization method already releases its guard before returning and the successor must not introduce another outer lock.

## 6. Selected ownership and provenance rules

### 6.1 Transport evidence

The helper must not accept `TransportIdentity` as a free caller argument.

The exact transport evidence comes from the existing `BoundRemoteSession` retained by the authenticated owner. This preserves the established distinction:

`logical device/session identity != transport identity != transient endpoint addressing`

Transport evidence remains an input to current durable validation, not a substitute for logical PRW identity.

### 6.2 Session lease

The helper must not construct, replace, extend, refresh, or reinterpret a lease.

It borrows exactly the `RemoteSessionLease` already retained by `BoundRemoteSession`.

### 6.3 Verifier time

`now_unix_seconds` remains verifier/caller provenance. C03e-KF does not select a system-clock source, time service, environment source, request timestamp, monotonic/wall-clock conversion, skew policy, retry, or refresh mechanism.

### 6.4 Request correlation

The `ControlFrame::request_id` inside the already-read capability transaction remains transaction correlation only. It is not requester identity, target identity, device identity, session identity, transport identity, authorization evidence, or registry authority.

### 6.5 Same-stream custody

The helper consumes the existing `PostAuthCapabilityTransaction` only at the response step. It must not clone, extract, duplicate, replace, reopen, or independently accept the retained stream.

Failure before response send drops the local transaction custody without fabricating a success response or retrying on another stream.

## 7. Selected bounded failure surface

The later one-file source successor may add one bounded Agent-local error:

`ProductionDurableCapabilityTransactionError`

with exactly these semantic stages:

- `Authority(DurableCapabilityBridgeError)`;
- `Dispatch(RemoteBridgeError)`;
- `Response(CapabilityRequestWireError)`.

Required semantics:

- `Display` is stage-bounded and must not disclose registry/provider/credential/request payload details;
- `std::error::Error::source()` preserves the exact underlying stage error;
- minimal exact `From` plumbing may be added for the three selected stage errors;
- no success/error remapping across stages;
- no retry, fallback, suppression, degraded authority, alternate dispatcher, alternate stream, negative-response fabrication, or process-exit classification.

This selection does not absorb stream acceptance or post-auth family-ingress errors because the selected helper begins only after one `PostAuthCapabilityTransaction` already exists.

## 8. Relationship to existing legacy capability path

The existing `SharedCurrentCapabilityAuthority<P>` and `CapabilityBridge` path remains unchanged in C03e-KF.

The later source successor selected here is a dormant sibling transaction seam. It must not:

- delete or rewrite the legacy path;
- make `ProductionDurableCapabilityAuthority` implement `PolicyEvaluator`;
- create an empty/default/synthetic `WorkspaceDeviceRegistry`;
- mirror durable registry state into `SharedCurrentCapabilityAuthority`;
- replace `LinuxAgentRemoteProcessOperationInputs.capability_authority`;
- change worker generic parameters;
- change executable aggregate constructors;
- choose which path is runtime-active.

A later separately reviewed checkpoint must decide any aggregate/interface replacement or runtime caller migration.

## 9. Authorization invariant

Successful post-auth ingress, transport binding, lease ownership, durable-registry lookup, PRWC decoding, request correlation, or dispatcher availability must never be upgraded into authorization by implication.

Only successful `ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)` may yield the `AuthorizedCapabilityRequest` consumed by the selected dispatch step.

The current production policy retained by that authority remains the fail-closed `ProductionRemoteCapabilityDenyAllPolicy`. C03e-KF selects no positive production grant source and does not weaken that policy.

## 10. First source-successor ceiling

After C03e-KF closes, the immediate materialization successor may change only:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

Permitted source scope is limited to:

1. exact imports required for the already-existing durable authority, transaction, dispatcher helper and bounded errors;
2. one Agent-local `ProductionDurableCapabilityTransactionError` with exactly the three selected semantic variants;
3. bounded `Display`, `Error::source`, and minimal `From` implementations;
4. one async `AuthenticatedRemoteSessionRuntimeOwner::process_production_durable_capability_transaction(...)` helper with effective crate-internal visibility;
5. exact extraction of transport identity and lease only from the retained `BoundRemoteSession`;
6. exactly one existing durable authority authorization invocation;
7. exactly one existing authorized-request dispatch invocation after authorization success;
8. exactly one same-stream response send after dispatch success;
9. focused same-file source/error/ownership-shape tests that require no provider, credential, process-global environment, listener, network, or production mutation;
10. strictly local rustfmt/Clippy acknowledgement only if exact selected source shape requires it.

The successor must stop if correct materialization requires any second repository path, public API expansion, manifest/lockfile change, new dependency, new authority owner, new registry/policy source, new stream read/accept path, new runtime task, or aggregate/executable mutation.

## 11. Explicit exclusions

C03e-KF does not perform or authorize:

- Rust/source materialization;
- aggregate replacement or `LinuxAgentRemoteProcessOperationInputs` mutation;
- executable caller/session-auth/expected-request/timing/callback population;
- production invocation of the KE authority bootstrap;
- repeated post-auth ingress or request loop migration;
- stream acceptance or second frame read;
- requester/rendezvous or candidate-publication execution changes;
- positive capability grants or production policy-source selection;
- registry mutation, scan, watch, cache, snapshot, mirror or background refresh;
- provider bootstrap changes, credential changes, RBAC changes, systemd/service/package changes;
- listener, bind, readiness, endpoint lifecycle, worker spawn, cancellation or process-lifecycle activation;
- `run()` or `main.rs` changes;
- environment provisioning or concrete deployed configuration;
- database/schema/control-plane mutation;
- deployment, restart or recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 12. Validation requirements for this selection checkpoint

C03e-KF is documentation-only. Closure requires validation tied only to the exact final KF head.

Required evidence:

- exact KE -> KF topology proves KE is the merge base and KF is ahead only;
- changed paths are exactly one contract file;
- zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/service/host changes;
- automatically triggered Rust validation reaches terminal success for the exact final head;
- path-filtered skipped workflows are recorded as `SKIPPED`, never PASS;
- Android PASS is claimed only if an Android workflow actually runs on the exact final head;
- immutable Google Drive audit is published only after exact-final-head validation;
- Drive readback must match frozen bytes and SHA-256 exactly;
- post-publication branch and PR guards must show no head drift.

## 13. STOP rule

After C03e-KF selection closure: **STOP**.

The immediate next checkpoint may only materialize the one-file dormant durable post-auth capability transaction seam selected above. After that source materialization is validated and durably closed, perform a fresh exact-head audit before selecting any aggregate replacement, session-auth/expected-request/timing/callback production population, repeated ingress migration, executable caller, startup/exit policy, listener/runtime/network activation, or allow-bearing production policy.
