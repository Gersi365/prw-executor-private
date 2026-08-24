# Phase 152 C03e-R — Remote Session Worker / Lifecycle Ownership Selection Staging

Status: STAGED

Target gate:

`C03E_R_REMOTE_SESSION_WORKER_LIFECYCLE_OWNERSHIP_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-Q:

- branch: `phase-152-c03e-q-authenticated-session-request-loop-source-materialization-staging`
- head: `e1f4f70151a74d723a60fca5c5f0bf5f114071a8`
- tree: `e7ae89af92b3805a6d8dc9d6b0dbc2b5f762634b`
- gate: `C03E_Q_AUTHENTICATED_SESSION_REQUEST_LOOP_SOURCE_MATERIALIZED`

C03e-R preserves exact Q lineage. It is a selection-only checkpoint and does not add task/runtime source.

## Purpose

Select the narrow lifecycle ownership contract for one future authenticated remote-session worker after C03e-Q materialized the borrowed serial request loop.

The selected worker boundary must answer:

1. who owns one `AuthenticatedRemoteSessionRuntimeOwner` while its request loop runs;
2. how external shutdown is delivered without fabricating a transport or logical identity;
3. how external shutdown avoids double-closing the peer through the C03e-Q code-3 terminal-error path;
4. how completed/failed workers are retained, reaped and joined;
5. which state is allowed to cross the worker boundary;
6. what remains separately gated before any runtime activation.

This checkpoint does not spawn a task, create a Tokio runtime, add dependencies, modify `main.rs`, publish readiness or activate a listener.

## Existing facts constraining the selection

### C03e-Q request-loop behavior

The existing `AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_loop(...)`:

- borrows the owner through `&mut self`;
- samples caller-owned verifier time once before every C03e-O transaction;
- serializes one request transaction at a time;
- treats every transaction `Accept`, `Wire` or `Bridge` error as session-terminal;
- closes the retained peer with fixed code `3` / reason `remote capability session terminated` before returning that original typed error;
- owns no external cancellation path.

### Current peer ownership

`AuthenticatedRemotePeerConnection` owns one bridge-wrapped validated `MeshQuicConnection` and exposes only:

- `transport_identity()`;
- `accept_control_stream()`;
- `close(code, reason)`.

It is not publicly cloneable and raw Quinn transport state is not exposed.

The lower `MeshQuicConnection` is internally cloneable, but C03e-R does not widen that lower transport surface or expose it through Agent.

### Current Agent async dependency boundary

`crates/prw-agent/Cargo.toml` currently has no direct Tokio dependency.

The lower remote transport already uses Tokio internally, but C03e-R does not silently turn that implementation detail into an Agent runtime dependency.

Concrete async runtime construction, task-spawn primitive and direct dependency changes remain separately gated.

### Existing local Linux lifecycle precedent

The already-validated local Linux runtime provides a semantic precedent only:

- worker ownership is separate from the accepted authenticated session;
- cancellation authority is paired with the retained worker handle;
- finished workers are reaped explicitly;
- shutdown cancels workers before joining them;
- join/completion classification is explicit;
- no detached worker is silently abandoned.

C03e-R does not reuse local `UnixStream`, scoped-thread or local-worker types for remote QUIC sessions.

## Selected worker ownership

One future remote-session worker owns by value exactly one already-composed `AuthenticatedRemoteSessionRuntimeOwner`.

The worker is the exclusive owner of that session owner for its lifetime. No concurrent code path may independently call request-loop operations on the same owner.

The worker also owns the per-worker mutable execution inputs required by C03e-Q rather than sharing mutable request state across workers:

- one dispatcher instance or other separately validated dispatcher owner supplied for that worker;
- one caller-owned verifier-time provider;
- stable shared handles/references to the current registry and policy evaluator sufficient to construct/use the existing `CapabilityBridge` without caching authorization decisions.

The worker must not cache:

- registry authorization results;
- policy decisions;
- transport-binding validation outcomes beyond the already-bound session state;
- lease-validity decisions;
- capability decisions;
- request success responses.

Every request continues through existing C03e-Q -> C03e-O -> `BoundRemoteSession::process_request(...)` dynamic authority.

## Selected cancellation model

External cancellation is a monotonic worker-control signal, not a transport-identity or logical-session mutation.

The controller side and worker side of the cancellation primitive are paired with exactly one remote-session worker.

The cancellation signal carries no user data, request data, identity, capability, path, dispatcher result or lower transport diagnostic.

The concrete channel type remains source-gated. C03e-R selects these semantics:

- cancellation can transition only from not-requested to requested;
- the worker can await cancellation concurrently with the C03e-Q loop;
- cancellation delivery must not require exposing raw Quinn/transport objects;
- controller drop must fail closed rather than leave an intentionally managed worker silently detached;
- repeated logical cancellation requests must not create multiple peer-close side effects.

A future implementation may use a Tokio synchronization primitive only after the direct Agent dependency/runtime surface is explicitly materialized and validated.

## Race ownership: request-loop failure vs external cancellation

The worker owns the race between:

1. the existing borrowed C03e-Q request loop; and
2. the worker cancellation signal.

### Request-loop wins

If C03e-Q returns its first `AuthenticatedRemoteSessionCapabilityTransactionError` before cancellation wins:

- the existing Q behavior remains authoritative;
- Q closes the same retained peer exactly once with code `3` / reason `remote capability session terminated`;
- the worker records a failed terminal completion containing the original typed transaction error;
- no retry or reclassification is performed.

### Cancellation wins

If external cancellation wins first:

1. the in-flight C03e-Q loop future is dropped/cancelled before the worker performs shutdown close;
2. after the mutable borrow held by that loop future is released, the worker closes the same retained authenticated peer exactly once using the external-shutdown diagnostic selected below;
3. the worker returns a normal `Cancelled` lifecycle completion rather than fabricating an `Accept`, `Wire` or `Bridge` failure;
4. no C03e-Q code-3 close is executed for that externally cancelled path.

This ordering is required specifically to prevent the external cancellation close from inducing an `Accept`/`Wire` error that Q would then close again as code `3`.

## External-shutdown peer close diagnostic

The selected remote-session worker cancellation close is:

- code: `4`;
- fixed reason bytes: `b"remote capability session shutdown"`.

The diagnostic is private, bounded and non-secret.

It contains no:

- `DeviceId`;
- `TransportIdentity`;
- IP address;
- request identifier;
- capability name;
- policy result;
- path;
- dispatcher diagnostic;
- certificate data;
- lower transport error.

Existing close-code allocations remain unchanged:

- code `1`: logical-session authentication transaction failure;
- code `2`: post-authentication binding failure;
- code `3`: capability-session transaction-loop terminal failure;
- code `4`: externally requested remote-session worker shutdown.

## No task abort in the first worker shape

C03e-R does not select forceful task abort as a normal shutdown mechanism.

After cancellation is delivered, the worker owns peer close and then exits through its normal lifecycle completion path.

The future worker registry/owner must join or otherwise await every retained worker completion before dropping the registry at orderly shutdown.

A hard abort/kill primitive, forced deadline expiration or detached task is not selected because those could interrupt dispatcher/domain operations at an unvalidated point.

If bounded forced-abort semantics are ever required, they need a separate contract covering domain-operation cancellation safety.

## Completion classification

The first remote worker completion shape is selected semantically as exactly these terminal classes:

- `Cancelled` — external cancellation won, the worker performed the code-4 close, and the worker exited normally;
- `Failed(AuthenticatedRemoteSessionCapabilityTransactionError)` — C03e-Q failed first and returned the original typed error after its code-3 close;
- `Panicked` / task-join failure — the async execution context terminated abnormally before yielding one of the expected worker results.

There is no fabricated clean-success completion from the request loop itself because C03e-Q is intentionally long-running and returns only on terminal transaction failure.

No `RemoteBridgeError` variant is reclassified as cancellation merely because shutdown was requested later.

## Retained worker-handle ownership

A future remote worker registry/collection entry must pair:

- one task/join handle;
- one matching cancellation-controller handle;
- no raw peer connection;
- no independent authenticated session copy;
- no cached registry/policy decision.

During normal operation the collection reaps workers whose task handles are already finished and classifies their completion explicitly.

At orderly shutdown the selected sequence is:

1. stop admitting new remote-session workers at the outer runtime boundary;
2. request cancellation for every currently retained worker;
3. retain every task handle while cancellation propagates;
4. await/join every remaining task;
5. classify every completion;
6. only then release the collection/runtime owner.

No worker handle is dropped intentionally while its task remains live.

## Concurrent collection remains separately gated

C03e-R selects the per-entry ownership relationship and shutdown/reap sequence only.

It does not yet select:

- collection key type;
- maximum concurrent remote sessions;
- one-session-per-`DeviceId` policy;
- replacement-session policy;
- duplicate logical-session admission;
- fairness;
- task scheduling budget;
- readiness interaction;
- listener backpressure.

Those belong to the later concurrent authenticated-session collection/admission checkpoint.

## Identity preservation

No lifecycle primitive is identity authority.

- `DeviceId` / authenticated PRW session identity remains the logical identity.
- `TransportIdentity` remains the lower-transport certificate identity already validated and bound to the session.
- IP addresses remain transient endpoints only.
- PID/UID/GID remain unrelated to remote logical identity.

Worker handles, cancellation handles, task IDs, runtime IDs and thread IDs must never be projected into PRW identity.

## Dynamic authority preservation

The worker may retain stable references/owned shared handles to registry/policy authorities, but may not snapshot their authorization outcomes for later requests.

Every successful iteration continues to perform current validation through the existing bridge/bound-session stack, including:

- verifier-owned application lease time;
- current authenticated-session registry validity;
- current logical-device to transport binding validity;
- current capability policy evaluation;
- current dispatcher execution.

External cancellation does not widen or bypass those checks.

## Explicitly unselected source/runtime changes

C03e-R selects no mutation to:

- `crates/prw-agent/Cargo.toml`;
- `Cargo.lock`;
- Agent `lib.rs` or `main.rs`;
- current remote-session runtime source;
- bridge peer wrapper;
- lower remote transport;
- registry/policy implementation;
- workflows;
- Android application source;
- readiness;
- systemd/packaging;
- host firewall/NAT/reachability state.

It also does not create:

- Tokio runtime construction;
- `tokio::spawn` call;
- direct `JoinHandle` type in Agent source;
- concrete cancellation channel;
- worker registry source;
- concurrent remote-session accept loop.

## Selected source-materialization order after R

The selected lifecycle cannot be safely materialized as one large mutation.

The next implementation sequence is deliberately narrow:

1. materialize only the cancellation-aware single-worker execution seam needed to race Q against external cancellation, including the fixed code-4 shutdown close, without spawning or collecting workers;
2. validate that source exactly;
3. separately select/materialize the concrete async task/runtime dependency and task-handle owner;
4. only after that materialize concurrent worker collection/admission.

If the implementation proves that a direct Agent Tokio dependency is required for the cancellation-aware worker seam, that dependency change must be explicit in that source checkpoint and validated as part of its exact diff. It must not be smuggled into this selection checkpoint.

## Validation requirements

C03e-R is docs-only.

Canonical completion requires exact-head PR validation appropriate to the docs-only diff. If Rust validation is triggered, it must finish successfully before closeout. Android validation is not claimed unless it is actually triggered and finishes successfully.

Skipped C02f-AD/C02f-AE workflows are never PASS evidence.

## Drive closeout requirements

After exact-head validation:

1. publish immutable `C03E_R_REMOTE_SESSION_WORKER_LIFECYCLE_OWNERSHIP_SELECTION_AUDIT.md` in the existing evidence folder;
2. raw-readback verify exact byte size and SHA-256;
3. re-fetch authoritative rolling `C02E_BRANCH_STATUS.md` immediately before mutation and require exact closed-Q baseline;
4. append R evidence only while preserving the full Q prefix byte-for-byte;
5. raw-readback verify final rolling size/hash and the full predecessor prefix;
6. update the R PR body to CLOSED checkpoint metadata while keeping the PR draft/open/unmerged.

## Deliberate stopping point

After C03e-R closes, the immediate next boundary is the cancellation-aware single-worker execution source seam only.

Still separately gated after R:

- concrete async runtime/task-spawn dependency;
- task-handle source ownership;
- concurrent remote-session collection/admission;
- listener accept loop;
- Agent `main.rs` remote runtime composition;
- remote readiness publication;
- listener/reachability activation;
- external NAT/ICE/STUN/TURN/relay integration;
- credential provisioning;
- deployment/restart/merge.

Gate on successful canonical closeout:

`C03E_R_REMOTE_SESSION_WORKER_LIFECYCLE_OWNERSHIP_SELECTED`
