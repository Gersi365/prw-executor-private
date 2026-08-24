# Phase 152 C03e-Y — Shared-Current Single-Worker Transaction Integration Selection Staging

Status: STAGED

Target gate:

`C03E_Y_SHARED_CURRENT_SINGLE_WORKER_TRANSACTION_INTEGRATION_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-X:

- branch: `phase-152-c03e-x-shared-current-capability-authority-source-materialization-staging`
- head: `eb297039abbf5aa29e09597a1c18c83baf8e91de`
- tree: `5c02eab23c6947542a2c58b73b946c8d12a6e7ca`
- gate: `C03E_X_SHARED_CURRENT_CAPABILITY_AUTHORITY_SOURCE_MATERIALIZED`

C03e-Y preserves exact X lineage.

## Purpose

Select only the integration boundary required by C03e-X before any spawned-task ownership is selected:

1. the existing single-worker request transaction consumes the C03e-X `SharedCurrentCapabilityAuthority<P>` instead of a caller-supplied borrowed `CapabilityBridge`;
2. each received request performs fresh current authorization under exactly one shared-authority read guard;
3. the guard yields one owned `AuthorizedCapabilityRequest` and is released before capability dispatch, response framing, response send or further network/lifecycle work;
4. the existing C03e-Q loop, C03e-S cancellation-aware worker and C03e-V executor drive seam forward the same borrowed shared-current authority owner;
5. stop before task spawn, authority cloning for task ownership, cancellation-controller construction, join-handle collection, concurrent authenticated-session admission, Agent binary wiring, readiness or runtime activation.

C03e-Y is decision-only. It does not change Rust source.

## Selected one-request transaction shape

The future source materialization changes the existing:

`AuthenticatedRemoteSessionRuntimeOwner::process_one_capability_request(...)`

so its authority input becomes a borrowed:

`&SharedCurrentCapabilityAuthority<P>`

instead of a borrowed:

`&CapabilityBridge<'_, P>`.

The transaction remains serialized by `&mut self` and continues to process exactly one newly accepted control stream.

The selected order is:

1. accept one bounded control stream from the retained authenticated peer;
2. receive exactly one bounded request frame using the existing C03e-N wire adapter;
3. acquire one fresh read operation through `SharedCurrentCapabilityAuthority::with_current_authority(...)`;
4. inside that synchronous read operation, construct one ephemeral `CapabilityBridge::new(registry, policy)` borrowing the current combined state;
5. call the retained `BoundRemoteSession::authorize(...)` exactly once with the ephemeral bridge, current verifier time and received frame;
6. return the owned `AuthorizedCapabilityRequest` from the shared-authority read operation;
7. release the shared-authority read guard;
8. call bridge-owned `authorized_request_dispatch::dispatch_authorized_request(...)` exactly once outside the authority guard;
9. send the resulting response frame exactly once on the same retained control stream;
10. return the existing typed transaction result unchanged.

No authorization result is reused for another request.

## Ephemeral bridge rule

`CapabilityBridge` remains the existing bridge-owned authorization implementation, but the Agent single-worker path no longer receives a bridge constructed by an outer caller.

For this selected path, `CapabilityBridge` is ephemeral and exists only inside the synchronous shared-current authority read operation.

The bridge borrows exactly:

- the current `WorkspaceDeviceRegistry`; and
- the current policy evaluator `P`.

It is not stored in `AuthenticatedRemoteSessionRuntimeOwner`, `RemoteSessionExecutorRuntime`, a task, a channel, a global/static value or a cache.

This preserves the bridge's existing authorization semantics while preventing stale caller-selected registry/policy borrows from becoming the future task boundary.

## Authorization path retained

Inside the authority read operation, authorization continues through the existing retained chain:

`BoundRemoteSession::authorize(...)`

-> `CapabilityBridge::authorize(...)`

That retains all current per-request checks:

- outer request control-message kind;
- verifier-owned remote-session lease time;
- current `WorkspaceDeviceRegistry::validate_authenticated_session(...)`;
- current transport-identity binding validation;
- request decode;
- exact required capability derivation;
- current policy evaluation.

C03e-Y selects no new authorization rule and removes none.

## Owned authorization evidence boundary

The only value crossing out of the shared-authority read operation is the existing owned:

`AuthorizedCapabilityRequest`.

That value remains one-request authorization evidence only.

It carries the existing validated principal snapshot, presented transport identity, granted capability, request ID and typed command needed by the already-authorized dispatch boundary.

It must not be cached, reused for a later request, treated as an authenticated-session replacement or treated as evidence that future registry/policy state is unchanged.

## Dispatch boundary

After the authority read operation returns, the future transaction calls the existing C03e-X helper:

`prw_remote_bridge::authorized_request_dispatch::dispatch_authorized_request(...)`.

Dispatch remains outside the shared-current authority guard.

The helper retains exactly the existing post-authorization behavior:

- dispatcher invocation exactly once;
- dispatcher failure -> `RemoteBridgeError::DispatchFailed`;
- existing `MAX_CONTROL_PAYLOAD_BYTES` response bound;
- oversize -> `RemoteBridgeError::DispatchResponseTooLarge`;
- response kind = `ControlMessageKind::Response`;
- original request ID preserved;
- frame construction failure -> `RemoteBridgeError::ResponseFrameRejected`.

No negative-response invention, retry or replacement stream/session is selected.

## Transaction error classification

The existing `AuthenticatedRemoteSessionCapabilityTransactionError` classification remains authoritative.

- stream acceptance failure remains `Accept(...)`;
- receive/send failure remains `Wire(...)`;
- current authorization failure remains `Bridge(...)`;
- already-authorized dispatch/response-frame failure also remains `Bridge(...)`.

No existing `RemoteBridgeError` is reclassified as recoverable.

The C03e-Q loop therefore keeps its existing behavior: the first transaction failure closes the same retained peer exactly once with code `3` / `remote capability session terminated` and returns the original typed failure.

## Lock hold boundary

The shared-current read guard must not be held across any of these operations:

- control-stream accept;
- request-frame receive;
- dispatcher execution;
- response-frame send;
- filesystem side effects;
- terminal side effects;
- forwarding side effects;
- cancellation wait;
- task spawn;
- task join/drain;
- readiness publication.

The only work under the guard is synchronous current authorization and creation of the owned one-request authorization result.

No raw Tokio read/write guard is exposed.

## Cancellation while waiting for authority

The C03e-S worker race remains authoritative.

If the request-loop future is waiting for `SharedCurrentCapabilityAuthority` read access and external cancellation wins while the request loop remains pending:

1. the request-loop future is dropped first;
2. any pending read-lock acquisition future is therefore dropped before peer close;
3. the retained peer is then closed exactly once with existing code `4` / `remote capability session shutdown`;
4. worker stop remains `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

If current authorization or dispatch has already produced a real terminal Q error in the same poll, the existing Q-before-cancellation priority remains unchanged and that failure wins.

C03e-Y adds no hard task abort and no new cancellation primitive.

## C03e-Q loop signature selection

The future source materialization changes:

`AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_loop(...)`

so it receives the same borrowed:

`&SharedCurrentCapabilityAuthority<P>`

and forwards it to every one-request transaction.

The loop still samples the caller-supplied verifier-time provider exactly once immediately before every request transaction.

It does not snapshot registry or policy outside the request transaction.

## C03e-S worker signature selection

The future source materialization changes:

`AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_worker(...)`

so it receives the same borrowed:

`&SharedCurrentCapabilityAuthority<P>`.

The worker continues to own only the existing race between the C03e-Q request loop and caller-supplied cancellation.

C03e-Y does not select an owned authority clone for a spawned task because no task is selected yet.

## C03e-V executor drive signature selection

The future source materialization changes:

`RemoteSessionExecutorRuntime::drive_capability_request_worker(...)`

so its current bridge argument is replaced by the same borrowed:

`&SharedCurrentCapabilityAuthority<P>`.

The drive seam remains one synchronous private-runtime `block_on(...)` over one borrowed worker body.

It remains `&mut self`, admits no second session and exposes no generic `block_on`, Tokio `Runtime`, `Handle` or task-spawn API.

## Policy trait boundary

Because the C03e-X shared-current authority read operation is available for:

`P: PolicyEvaluator + Send + Sync`,

the future integrated single-worker path may tighten its generic policy bounds from the current `PolicyEvaluator + Sync` shape to:

`PolicyEvaluator + Send + Sync`.

No `P: Clone` bound is permitted.

`WorkspaceDeviceRegistry: Clone` is not required.

This trait-bound tightening is a consequence of using the already-materialized shared-current owner, not a policy-semantic change.

## Shared mutation boundary remains separate

C03e-Y does not select or materialize registry/policy mutation APIs.

Future management mutations must linearize through the same combined state write lock selected in C03e-W, but exact write operations remain separately gated.

No currentness cache, authorization cache or separate registry/policy lock is selected.

## Dependency boundary

The selected integration requires no new crate, dependency, feature or lockfile change.

Existing dependencies already provide:

- `prw-registry` and `prw-policy` to Agent;
- Tokio `sync` to Agent;
- `CapabilityBridge`, `BoundRemoteSession`, `AuthorizedCapabilityRequest`, `CapabilityDispatcher` and `authorized_request_dispatch` through `prw-remote-bridge`.

Therefore future materialization must keep unchanged:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

No direct `prw-remote-transport` dependency may be added to Agent.

## Identity invariants

No runtime, task, lock or authority-owner value becomes PRW identity.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- PID/UID/GID/thread/runtime/lock/task identifiers are not logical identity.

The immutable transport identity already retained by `BoundRemoteSession` remains paired with the authenticated logical session lease and is revalidated against current registry state on every protected request.

## Explicit non-selection

C03e-Y does not select or perform:

- Rust source materialization;
- task spawn;
- `tokio::spawn`;
- `JoinHandle` / `JoinSet` ownership;
- cancellation-controller construction;
- worker collection;
- concurrent authenticated-session admission;
- duplicate-DeviceId admission policy;
- fairness/capacity policy;
- authority write APIs;
- Agent `main.rs` wiring;
- remote transport bind/listen/accept activation;
- readiness publication;
- systemd/host mutation;
- deployment.

## Intended Y net diff boundary

Final X -> Y net diff is intended to contain exactly one docs-only path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_Y_SHARED_CURRENT_SINGLE_WORKER_TRANSACTION_INTEGRATION_SELECTION_STAGING.md`

No Rust source, Cargo manifest/lockfile, workflow, Android application source, Agent `main.rs`, readiness, packaging/systemd or host-network path may change.

## Validation requirements

Closure requires on the final exact head:

- exact X merge base;
- exact one-path docs-only net-scope review;
- permanent Rust validation FULL PASS;
- Android validation is not required to trigger for a docs-only change and no Android PASS may be claimed if it does not run;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback verification;
- append-only rolling Drive update preserving the complete post-X prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-Y means only that the single-worker shared-current authorization/dispatch integration shape is selected.

It does not mean the worker path has been changed yet, registry/policy mutation is wired, authority ownership has been adapted for spawned tasks, tasks are spawned, worker handles are collected, multiple authenticated sessions are admitted concurrently, the Agent binary is wired, remote transport is activated or readiness may be published.

The next checkpoint may materialize only this selected single-worker integration by changing the existing one-request/loop/worker/executor-drive authority parameter and splitting current authorization from already-authorized dispatch exactly as selected here.

Target gate:

`C03E_Y_SHARED_CURRENT_SINGLE_WORKER_TRANSACTION_INTEGRATION_SELECTED`
