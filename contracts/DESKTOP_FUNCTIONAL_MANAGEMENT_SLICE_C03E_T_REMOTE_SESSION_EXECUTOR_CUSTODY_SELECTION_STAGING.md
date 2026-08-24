# Phase 152 C03e-T — Remote Session Executor Custody Selection Staging

Status: STAGED

Target gate:

`C03E_T_REMOTE_SESSION_EXECUTOR_CUSTODY_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-S:

- branch: `phase-152-c03e-s-cancellation-aware-remote-session-worker-seam-source-materialization-staging`
- head: `4ad8f97025ed07fef0eaa4fbaa3c06d71ac74667`
- tree: `0c2cdcf0da6f1aedfee8ca6948f6dc0b6e13a339`
- gate: `C03E_S_CANCELLATION_AWARE_REMOTE_SESSION_WORKER_SEAM_SOURCE_MATERIALIZED`

C03e-T preserves exact S lineage. It is a selection-only checkpoint and does not add runtime, task or network source.

## Purpose

Select the narrow executor-custody boundary required before the already-materialized C03e-S cancellation-aware remote-session worker body can ever run inside the Agent.

C03e-T answers only:

1. which executor implementation the Agent may own for the remote QUIC/runtime path;
2. who owns that executor and its lifetime;
3. how the executor relates to `RemoteServerTransportRuntime` and C03e-S futures;
4. which Tokio dependency/version/features may be added later;
5. why per-session `tokio::spawn` is not yet selected;
6. what current-authority/task-lifetime prerequisite must be solved before spawned workers or concurrent remote sessions.

This checkpoint does not create a runtime, spawn a task, bind UDP, accept a peer, publish readiness, wire `main.rs`, add a dependency, or activate production behavior.

## Existing facts constraining the selection

### C03e-S is executor-neutral

C03e-S materialized `AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_worker(...)` using only:

- `std::future::Future`;
- `std::future::poll_fn`;
- `std::task::Poll`;
- `Box::pin`.

C03e-S intentionally owns no executor, task, channel, runtime, join handle or listener.

### Lower remote transport already selects Tokio

`crates/prw-remote-transport/Cargo.toml` currently pins:

`tokio = { version = "=1.53.1", default-features = false, features = ["rt", "macros", "net", "time", "sync", "io-util"] }`

Quinn is compiled with `runtime-tokio` and the reusable real-socket transport wrapper constructs Quinn endpoints with `TokioRuntime`.

Therefore the future Agent executor must be compatible with Tokio 1.53.1. C03e-T does not introduce a second executor family or a second Tokio version.

### `prw-agent` has no direct Tokio dependency today

`crates/prw-agent/Cargo.toml` currently has no direct Tokio dependency.

The lower transport's Tokio dependency is an implementation detail of that crate and is not treated as implicit Agent runtime ownership.

Any direct Agent dependency must be an explicit later source checkpoint.

### Current registry authority is not a spawned-task sharing primitive

`WorkspaceDeviceRegistry` is currently a bounded in-memory authority containing mutable `HashMap` state and ordinary `&mut self` mutation APIs.

It is not currently wrapped in `Arc<RwLock<_>>`, an actor/mailbox, an `ArcSwap`-like indirection, or another independently selected shared-current-authority primitive.

### `CapabilityBridge` borrows the concrete current registry

The current bridge shape is:

- `CapabilityBridge<'a, P>`;
- stores `&'a WorkspaceDeviceRegistry`;
- stores `&'a P` policy evaluator.

C03e-S accepts one borrowed `&CapabilityBridge` for the worker lifetime.

This is sufficient for the current source/disposable sequential boundary, because every request re-evaluates the live object reached by that borrow. It is not by itself a safe basis for inventing a `'static tokio::spawn` worker plus concurrent registry mutation.

### Current-registry semantics must not be weakened

Phase 130 requires authenticated-session identity to be revalidated against current registry state before protected operations. Membership suspension/removal and device revocation must invalidate an otherwise valid older authenticated session snapshot.

A task design must therefore not obtain a stale cloned registry snapshot merely to satisfy `'static` task bounds.

## Selected executor family

The future Agent-owned remote executor is Tokio, pinned to the already-present exact workspace version:

`=1.53.1`

No alternate executor family is selected.

The first Agent direct dependency, when separately materialized, may enable only the features required for explicit runtime custody and the already-selected lower transport path. The selected baseline is:

- `rt`;
- `net`;
- `time`;
- `sync`.

The Agent does not need Tokio macros for this boundary and C03e-T does not select `macros` as a direct Agent requirement.

`rt-multi-thread` is not selected for the first executor shape.

Cargo feature unification with the existing lower transport dependency remains ordinary Cargo behavior; C03e-T does not modify lower transport features.

## Selected executor shape

The first Agent-owned remote executor is one non-cloneable owner around one Tokio **current-thread** runtime.

Conceptual source name for the later materialization:

`RemoteSessionExecutorRuntime`

The runtime is constructed explicitly with the Tokio current-thread builder and with I/O/time drivers enabled for the already-selected Quinn/Tokio transport path.

The owner:

- owns the Tokio runtime by value;
- is not a global/static runtime;
- is not lazily created through hidden process-global state;
- is not cloned into domain objects;
- does not expose the raw Tokio runtime as public API;
- does not itself bind a socket, accept a peer, authenticate a session, authorize a capability or publish readiness.

## Runtime custody and transport relationship

The same future executor owner must outlive every remote transport endpoint, accepted peer and remote-session worker future that it drives.

The intended ordering for later separately gated composition is:

1. construct the Agent-owned remote executor;
2. enter/run the selected executor context;
3. construct/bind `RemoteServerTransportRuntime` only inside that executor context;
4. perform asynchronous transport acceptance/authentication inside the same executor custody;
5. compose C03e-H/L/S session ownership inside the same executor custody;
6. drive session worker futures inside that same executor custody;
7. stop admitting work before executor shutdown;
8. close/drain remote transport resources before dropping the executor.

C03e-T does not materialize any of those operations.

## Why current-thread is selected first

The first executor source checkpoint is intended to establish **custody**, not concurrency policy.

A current-thread runtime provides the narrowest initial ownership surface because it:

- avoids selecting a new runtime worker-thread pool size;
- avoids selecting multi-thread scheduling/fairness behavior prematurely;
- allows a later `Runtime::block_on(...)`-style narrow operation to drive non-`'static` borrowed futures without fabricating shared authority;
- keeps executor lifetime explicit and inspectable;
- remains compatible with Quinn's Tokio runtime adapter.

This selection does not claim that a current-thread executor is the final concurrent production scheduling policy.

If later concurrent remote-session execution requires a multi-thread executor or a separate blocking-dispatch strategy, that requires a separately reviewed contract and exact resource bounds.

## Spawned per-session tasks are deliberately NOT selected here

C03e-R selected the semantic lifecycle desired for a future retained worker collection. C03e-S materialized the cancellation-aware worker body. However C03e-T does **not** yet select `tokio::spawn` for one session.

The reason is a concrete source constraint, not a generic deferral:

- `tokio::spawn` requires an owned `'static` future;
- C03e-S currently borrows one `CapabilityBridge` for the worker lifetime;
- `CapabilityBridge` borrows the concrete `WorkspaceDeviceRegistry` and policy evaluator;
- the registry currently has ordinary mutable ownership, not a selected shared-current-authority indirection;
- cloning the registry into the task would break current-registry revocation/suspension semantics;
- holding a hypothetical read lock for the whole session would block current registry mutation and also fail the intended per-request current-authority model.

Therefore no spawned session task is selected until current authority sharing is separately designed.

## No silent authority wrapper

C03e-T explicitly rejects silently wrapping the registry/policy in one of the following merely to satisfy task bounds:

- `Arc<Mutex<_>>`;
- `Arc<RwLock<_>>`;
- detached snapshots;
- process-global statics;
- unsafe raw pointers;
- leaked `'static` references;
- task-local cached authorization decisions.

Any shared-current-authority owner must define:

- mutation ownership;
- read/write synchronization;
- poison/failure behavior if applicable;
- lock/await ordering;
- per-request freshness;
- policy authority freshness;
- interaction with session cancellation;
- testable revocation/suspension visibility.

That is a separate checkpoint.

## Selected first operation boundary after executor source materialization

After the executor owner itself is separately materialized, the first allowed execution seam is a **single-worker borrowed execution** path that can drive one existing C03e-S worker future without requiring `'static` task spawn.

That future seam must retain:

- borrowed current registry/policy through the existing `CapabilityBridge`;
- caller-owned mutable dispatcher;
- caller-owned verifier-time provider;
- caller-owned cancellation future;
- C03e-S code-3/code-4 race semantics unchanged.

It must not create a worker collection or concurrent session admission.

This keeps the next source steps incremental:

1. executor owner source only;
2. one borrowed single-worker drive seam;
3. current-authority sharing selection/materialization;
4. only then spawned task + retained join/completion ownership;
5. only after that concurrent session collection/admission.

## Executor shutdown semantics selected at custody level

The executor owner may be dropped only after all remote operations it owns have been driven to terminal cleanup by a separately gated outer lifecycle.

C03e-T does not select forceful Tokio runtime shutdown as worker cancellation.

C03e-R/S cancellation remains session-specific and must occur before executor teardown.

No future orderly shutdown may rely on dropping the runtime while managed session work is intentionally live.

## Failure boundary

Executor construction failure, when source is later materialized, must be represented by a narrow Agent-owned error rather than panic or process exit inside a library constructor.

C03e-T does not invent the final enum name, but selects these semantics:

- runtime construction failure is distinct from transport bind/accept failure;
- no lower Tokio diagnostic is sent over the remote protocol;
- no retry loop is automatic;
- failure occurs before any remote listener/readiness publication in the later composition order.

## Security invariants preserved

C03e-T does not change identity semantics:

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity only;
- IP address remains a transient endpoint;
- PID/UID/GID/task/thread/runtime identifiers never become PRW logical identity.

C03e-T does not weaken capability authority:

- current registry validation remains required;
- current policy evaluation remains required;
- application lease validation remains required;
- verifier time remains caller-owned and fresh per request;
- no authorization result is cached into executor/task metadata.

## Explicit non-goals

C03e-T does not:

- modify `Cargo.toml` or `Cargo.lock`;
- construct Tokio runtime source;
- add `tokio::spawn`;
- add `JoinHandle` or `JoinSet`;
- add cancellation channels/tokens;
- add `Arc<Mutex<_>>` or `Arc<RwLock<_>>` authority state;
- modify `CapabilityBridge`;
- modify `WorkspaceDeviceRegistry`;
- modify policy source;
- modify dispatcher source;
- bind or activate remote UDP/QUIC;
- add concurrent session acceptance;
- modify `main.rs`;
- publish readiness;
- modify systemd, firewall, NAT, routing or host-network configuration;
- deploy or merge.

## Required validation for closure

Because this checkpoint is docs-only, closure requires:

- exact predecessor merge base;
- exact one-path docs-only diff;
- canonical PRW Rust Validation FULL PASS on the exact head;
- any skipped workflows recorded as skipped, not PASS;
- immutable Drive audit with raw readback verification;
- append-only rolling Drive update with exact predecessor prefix preservation;
- PR remains draft/open/unmerged.

Android validation need not be claimed if the docs-only head does not trigger it.

## Completion meaning

Closure of C03e-T means only:

- Tokio 1.53.1 is selected as the future Agent-owned remote executor family;
- the first executor custody shape is a non-cloneable current-thread runtime owner;
- future remote transport/session async work must run under that explicit owner;
- direct spawned session tasks are blocked until a separate current-authority sharing boundary exists;
- the next implementation checkpoint may materialize only the executor owner source/dependency surface.

Target gate:

`C03E_T_REMOTE_SESSION_EXECUTOR_CUSTODY_SELECTED`
