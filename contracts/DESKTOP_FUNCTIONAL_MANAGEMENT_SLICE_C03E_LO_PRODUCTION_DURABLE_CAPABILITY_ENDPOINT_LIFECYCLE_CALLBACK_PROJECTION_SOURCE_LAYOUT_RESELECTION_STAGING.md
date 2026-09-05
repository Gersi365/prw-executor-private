# Desktop Functional Management Slice C03e-LO — Production Durable Capability Endpoint Lifecycle Callback Projection Source Layout Reselection

Status: `STAGING_RESELECTION`
Date: `2026-09-05`
Repository: `Gersi365/prw-executor-private`

## 1. Gate

Selected gate after exact-head validation and immutable evidence publication:

`C03E_LO_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SOURCE_LAYOUT_RESELECTED`

Selected closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SOURCE_LAYOUT_RESELECTION`

C03e-LO is documentation-only. It does not materialize Rust/source behavior. It corrects only the physical source layout required to preserve the exact C03e-LN callback-projection semantics after fresh source audit proved that the LN-selected completion enum cannot be made genuinely crate-visible by editing the private endpoint child module alone.

## 2. Exact predecessor authority

C03e-LO is rooted only at the evidence-closed C03e-LN selection checkpoint:

- branch: `phase-152-c03e-ln-production-durable-capability-endpoint-lifecycle-callback-projection-selection`
- exact predecessor head / merge base: `d0cbdc28b23337ed700e8f79ed1f75cf05f70370`
- exact predecessor tree: `0cffde491606415788f0457df6babba4b71d11cc`
- LN contract blob: `c357c43ab6cfc459e1d3b5ebdaed43876e4f05a0`
- endpoint child blob: `59859c2659b94f68267eae105e3bcce928b77dc9`
- parent remote-session capability module blob: `ed60fa3673d24f4ed0a73dd8ae1cef4e9dd04411`
- crate root blob: `53e6b9c33d1a3be644fb6645289f6854cc096eee`

C03e-LN remains draft/open/unmerged and evidence-closed. No merge, deployment, runtime activation, or repository configuration mutation is implied by using it as the exact predecessor.

## 3. Corrective source-layout finding

### 3.1 Endpoint child is private to its parent module

Exact LN parent source:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

contains:

```rust
mod remote_session_endpoint_lifecycle_runtime;
```

The endpoint child module is therefore private beneath the public `remote_session_capability_runtime` parent.

### 3.2 Existing endpoint public types cross the child boundary only by explicit parent re-export

The same parent source explicitly re-exports the existing endpoint-facing types:

```rust
pub use remote_session_endpoint_lifecycle_runtime::{
    RemoteSessionEndpointBoundAddressError,
    RemoteSessionEndpointLifecycleRuntime,
    RemoteSessionEndpointLifecycleStartupError,
    RemoteSessionEndpointLifecycleStartupFailure,
    RemoteSessionSupervisorShutdownController,
};
```

The C03e-LN-selected `RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection` does not exist yet and therefore is not re-exported.

### 3.3 Public parent module is already registered at crate root

Exact LN `crates/prw-agent/src/lib.rs` contains:

```rust
pub mod remote_session_capability_runtime;
```

No crate-root registration change is required.

### 3.4 Why the LN one-file ceiling is insufficient

C03e-LN selected the completion projection enum as `pub(crate)` and described it as a crate-visible endpoint callback surface. Defining that enum only inside the private endpoint child does not provide a crate-visible name through the public parent module.

The adapter method itself is attached to the already parent-re-exported `RemoteSessionEndpointLifecycleRuntime`, but the selected projection enum still needs one explicit parent re-export so crate sibling callers can name and match its bounded variants without exposing requester-private lifecycle types.

C03e-LO therefore preserves every LN semantic choice and changes only the future physical source ceiling from one path to two paths.

## 4. Reselected immediate source layout

The immediate future source materialization may mutate exactly two Rust paths.

### 4.1 Endpoint child source

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact LN predecessor blob:

`59859c2659b94f68267eae105e3bcce928b77dc9`

This path may add only the LN-selected:

1. `RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection` with visibility exactly `pub(crate)` and variants exactly:
   - `Cancelled`
   - `IngressFailure`
   - `RequesterResponseFailure`
   - `AbnormalTaskCompletion`
2. `RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection(...)` with visibility exactly `pub(crate)`;
3. the requester-private error import needed only to perform the LN-selected four-family mapping;
4. mechanical attributes/doc comments/formatting required for compilation and lint cleanliness.

The adapter must invoke the unchanged C03e-LM `pub(super)` durable endpoint lifecycle exactly once and project only the completion callback.

### 4.2 Parent module source

`crates/prw-agent/src/remote_session_capability_runtime.rs`

Exact LN predecessor blob:

`ed60fa3673d24f4ed0a73dd8ae1cef4e9dd04411`

This path may add only one crate-private parent re-export of the selected enum, conceptually:

```rust
pub(crate) use remote_session_endpoint_lifecycle_runtime::
    RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection;
```

plus only a mechanical lint attribute if the exact compiler/Clippy surface requires one while the enum remains intentionally dormant.

No other parent-module item, module declaration, public re-export, type, function, impl, test, or behavior may change.

## 5. C03e-LN semantics preserved exactly

C03e-LO does not alter the selected terminal projection law.

The exact mapping remains:

- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled)` -> `Cancelled`;
- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError::Ingress(_)))` -> `IngressFailure`;
- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError::RequesterResponse(_)))` -> `RequesterResponseFailure`;
- `Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)` -> `AbnormalTaskCompletion`.

The authenticated `DeviceId` is forwarded unchanged beside the projection.

Requester-private error payloads remain private and are not stringified, cloned into a new API, widened, or re-exported.

## 6. Ownership and lifecycle law preserved

The future adapter must preserve the exact C03e-LM ownership surface:

- consume `self` exactly once;
- move exactly one caller-owned `Arc<ProductionDurableCapabilityAuthority>` unchanged into the existing LM method;
- keep `SharedCurrentCapabilityAuthority<P>` as the requester-DR/current admission authority lane;
- keep the requester-aware policy source and `SharedRequesterRendezvousAuthority` unchanged;
- forward rejection and admission-failure callbacks unchanged;
- return `RemoteSessionPersistentCollectionConfigError` unchanged.

The adapter must not independently destructure the endpoint owner, convert supervisor shutdown, close transport, wait idle, invoke the executor lifecycle directly, or duplicate lower lifecycle behavior.

## 7. Existing lower peer disposition remains authoritative

The C03e-FW peer-disposition consumer and C03e-LK durable executor boundary already dispose the recovered authenticated peer before the outward requester-aware completion callback.

C03e-LO selects no:

- FW visibility widening;
- new peer-disposition classifier;
- new close code;
- duplicate peer close;
- peer reuse/restart/reconnect;
- requester-record cleanup;
- retry or implicit re-admission.

The new projection remains observation-only after the existing lower disposition.

## 8. Frozen source and behavior outside the reselection

The immediate future source checkpoint must leave unchanged:

- C03e-LM durable endpoint method body/signature/`pub(super)` visibility;
- legacy public endpoint lifecycle;
- requester retained-custody stop/error visibility;
- FW disposer and classifier;
- LK durable executor lifecycle;
- `production_durable_capability_higher_owner_custody.rs`;
- `linux_bootstrap.rs`;
- `lib.rs`;
- `main.rs`;
- manifests, lockfile, workflows and Android source.

If any third source path or any semantic widening is required, STOP and open another independently selected gate.

## 9. Explicit non-selection

C03e-LO does not select or authorize:

- source materialization in LO itself;
- public (`pub`) exposure of the projection enum;
- requester-lifecycle stop/error visibility widening;
- re-export of requester-private error payloads;
- callback projection outside the endpoint child;
- legacy aggregate construction;
- higher-owner operation caller migration;
- `linux_bootstrap.rs` mutation;
- durable-authority bootstrap/population;
- executable callback/logging/counter policy;
- candidate/reachability continuation;
- target dialing, retry, reconnect or peer reuse;
- endpoint/listener/readiness/runtime activation;
- merge, ready-for-review conversion, deploy, restart/recovery;
- repository configuration mutation;
- PR close, branch deletion, force update or history rewrite;
- destructive cleanup.

## 10. Validation requirements

C03e-LO closure authority is only its exact final docs-only head.

Before closure:

1. verify exact predecessor remains C03e-LN head `d0cbdc28b23337ed700e8f79ed1f75cf05f70370`;
2. verify LO is ahead only by documentation commit(s), behind by zero;
3. verify exactly one changed path: this contract;
4. verify zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/executable changes;
5. require exact-final-head `PRW Rust Validation` success;
6. classify path-inapplicable workflows as `SKIPPED`, never PASS;
7. claim Android only if an exact-final-head Android workflow actually runs and succeeds;
8. re-read exact branch/PR state before immutable evidence publication;
9. freeze audit bytes and SHA-256 before Drive upload;
10. require exact-title pre-upload uniqueness, raw Drive byte/hash readback, and exact-title post-upload uniqueness;
11. keep the PR draft/open/unmerged.

## 11. Gate and successor boundary

After exact-final-head validation and immutable evidence publication, record:

`SOURCE_LAYOUT_RESELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Gate:

`C03E_LO_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SOURCE_LAYOUT_RESELECTED`

Closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SOURCE_LAYOUT_RESELECTION`

After C03e-LO closure: STOP.

The immediate later source checkpoint may materialize only the two-path layout above: the LN-selected bounded completion enum + projection adapter in the endpoint child, and one crate-private parent re-export of that enum. No caller migration, higher-owner mutation, durable-authority population, runtime activation, merge, deploy or visibility widening is implied.