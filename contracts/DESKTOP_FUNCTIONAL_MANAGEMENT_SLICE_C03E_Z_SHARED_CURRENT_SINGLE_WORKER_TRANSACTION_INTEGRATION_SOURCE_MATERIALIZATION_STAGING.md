# Phase 152 C03e-Z — Shared-Current Single-Worker Transaction Integration Source Materialization Staging

Status: STAGED

Target gate:

`C03E_Z_SHARED_CURRENT_SINGLE_WORKER_TRANSACTION_INTEGRATION_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-Y:

- branch: `phase-152-c03e-y-shared-current-single-worker-transaction-integration-selection-staging`
- head: `5b432205151c0db6a63fee84815b95b6918a157c`
- tree: `5cb606af5929995d612365d59d60fd7bd89ac9e0`
- gate: `C03E_Y_SHARED_CURRENT_SINGLE_WORKER_TRANSACTION_INTEGRATION_SELECTED`

C03e-Z preserves exact Y lineage.

## Purpose

Materialize only the C03e-Y-selected integration of the existing single-worker request path with the C03e-X shared-current authority.

The source change is bounded to:

1. replace the caller-supplied borrowed `CapabilityBridge` parameter in the existing C03e-O/Q/S request/worker path with borrowed `SharedCurrentCapabilityAuthority<P>`;
2. perform one fresh current authorization operation per received request under the shared authority read boundary;
3. create one ephemeral `CapabilityBridge` only inside that synchronous authority operation;
4. return one existing owned `AuthorizedCapabilityRequest` from authorization;
5. release the authority read guard before existing already-authorized dispatch and response send;
6. replace the C03e-V executor-drive bridge parameter with the same borrowed shared-current authority owner;
7. preserve all existing request-loop, cancellation, error, identity and response semantics;
8. stop before task spawn, authority cloning for task ownership, worker collection, concurrent authenticated-session admission, Agent binary wiring, readiness or runtime activation.

## Materialized one-request transaction

`AuthenticatedRemoteSessionRuntimeOwner::process_one_capability_request(...)` retains `&mut self` serialization and the existing bounded transport/wire transaction.

Its authority parameter becomes:

`&SharedCurrentCapabilityAuthority<P>`

with policy bound:

`P: PolicyEvaluator + Send + Sync`.

For each request the materialized order is:

1. accept exactly one control stream from the retained authenticated peer;
2. receive exactly one bounded request frame;
3. invoke `SharedCurrentCapabilityAuthority::with_current_authority(...)` exactly once;
4. inside that synchronous closure, construct `CapabilityBridge::new(registry, policy)` over the current combined state;
5. invoke retained `BoundRemoteSession::authorize(...)` exactly once with current verifier time and the received frame;
6. return the existing owned `AuthorizedCapabilityRequest` from the closure;
7. allow the shared-authority guard to release when the authority operation returns;
8. invoke `prw_remote_bridge::authorized_request_dispatch::dispatch_authorized_request(...)` exactly once outside the guard;
9. send the resulting response frame exactly once on the same control stream.

No dispatcher execution or response I/O occurs under the authority read guard.

## Current authorization retained

The ephemeral bridge continues to execute the existing authorization chain without semantic replacement:

- request outer-kind check;
- verifier-owned lease validity check;
- current authenticated-session registry validation;
- current transport-identity validation;
- request decode;
- exact capability derivation;
- current policy evaluation.

No authorization cache, registry snapshot, policy snapshot or reusable decision is introduced.

## Owned request evidence

The existing `AuthorizedCapabilityRequest` is the only authorization result crossing the authority-read boundary.

It remains evidence for exactly one received request and is consumed immediately by the existing already-authorized dispatch boundary. It is not stored for later requests and does not replace the retained logical session or future current-state validation.

## Request loop and worker

`AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_loop(...)` receives the same borrowed shared-current authority owner and forwards it to every one-request transaction.

The verifier-time callback is still sampled exactly once immediately before each request transaction.

`AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_worker(...)` receives the same borrowed shared-current authority and otherwise preserves C03e-S arbitration unchanged:

- request-loop future is polled before cancellation on every wake;
- a ready real request-loop failure wins a same-poll tie;
- cancellation wins only while the request loop remains pending;
- cancellation drops the request-loop future first, which also drops any pending shared-authority read acquisition before peer close;
- cancellation then closes the retained peer once with existing code `4` / `remote capability session shutdown`;
- request-loop failure retains existing code `3` / `remote capability session terminated` and the original typed transaction error.

No new cancellation primitive or hard abort is introduced.

## Executor drive seam

`RemoteSessionExecutorRuntime::drive_capability_request_worker(...)` replaces its borrowed `CapabilityBridge` parameter with:

`&SharedCurrentCapabilityAuthority<P>`.

It retains the existing private current-thread Tokio runtime and exactly one internal `Runtime::block_on(...)` over the borrowed C03e-S worker body.

It still exposes no generic `block_on`, runtime handle, task-spawn primitive, second-session admission or readiness state.

## Lock hold boundary

The materialized authority read guard is held only for synchronous authorization.

It is not held across:

- control stream accept;
- request receive;
- dispatcher execution;
- response send;
- filesystem/terminal/forwarding side effects;
- cancellation wait;
- task spawn;
- join/drain;
- readiness publication.

No raw Tokio lock guard is exposed.

## Dependency boundary

C03e-Z adds no dependency or feature and does not change any manifest or lockfile.

Required surfaces already exist from C03e-X:

- Agent `SharedCurrentCapabilityAuthority<P>`;
- Agent Tokio `sync` support;
- bridge `CapabilityBridge`;
- `BoundRemoteSession::authorize(...)`;
- owned `AuthorizedCapabilityRequest`;
- public bridge `authorized_request_dispatch::dispatch_authorized_request(...)`.

These must remain byte-stable:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

No direct `prw-remote-transport` dependency may be added to Agent.

## Identity invariants

Identity semantics are unchanged:

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- runtime/task/lock/PID/UID/GID/thread identifiers are not logical identity.

The retained transport identity is revalidated against current registry state on every protected request through the existing bridge authorization path.

## Source files

The intended final Y -> Z net diff contains exactly three paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`.

No parent module export change is required because `SharedCurrentCapabilityAuthority` is already re-exported by C03e-X and the dispatch helper is already public from the bridge crate.

## Corrective materialization guard

Because `authenticated_remote_session_runtime.rs` is a large established source file and the connected GitHub contents API only exposes whole-file replacement, a branch-local self-removing corrective workflow may be used only to apply exact bounded text replacements.

If used, it must:

- run only on this exact Z branch;
- verify the expected predecessor text occurs exactly once before each replacement;
- change only the two intended Rust source paths plus remove its own temporary workflow;
- run canonical `cargo fmt` before committing;
- verify the locked dependency graph;
- enforce an exact changed-path allow-list;
- remove itself in the same corrective commit;
- push only a fast-forward commit to this Z branch.

Any CI run on a head containing that temporary workflow is diagnostic/superseded only. Completion evidence must come from the final exact head after the workflow is absent.

## Explicit non-selection

C03e-Z does not select or perform:

- `tokio::spawn` or any task spawn;
- authority cloning into a task;
- `JoinHandle` / `JoinSet` ownership;
- cancellation-controller construction;
- worker collection;
- concurrent authenticated-session admission;
- duplicate logical-device admission policy;
- capacity/fairness scheduling;
- registry/policy write APIs;
- Agent `main.rs` wiring;
- remote listener/bind activation;
- readiness publication;
- systemd/host mutation;
- deployment.

## Validation requirements

Closure requires on the final exact Z head after any temporary workflow is absent:

- exact Y merge base;
- exact three-path final net-scope review;
- `Cargo.lock` and relevant manifests unchanged;
- permanent Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy with `-D warnings`, workspace tests and workspace build;
- canonical Android native/application validation FULL PASS because Rust source changes are present;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-Y prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-Z means only that the existing borrowed single-worker transaction/loop/worker/executor-drive path now performs fresh shared-current authorization and releases the authority guard before already-authorized dispatch and response I/O.

It does not mean shared mutation APIs are wired, authority ownership for spawned tasks is selected, tasks are spawned, workers are collected, multiple authenticated sessions are admitted concurrently, Agent `main.rs` is wired, remote transport is activated or readiness may be published.

The next checkpoint must explicitly select task/worker ownership after this shared-current single-worker integration is proven.

Target gate:

`C03E_Z_SHARED_CURRENT_SINGLE_WORKER_TRANSACTION_INTEGRATION_SOURCE_MATERIALIZED`
