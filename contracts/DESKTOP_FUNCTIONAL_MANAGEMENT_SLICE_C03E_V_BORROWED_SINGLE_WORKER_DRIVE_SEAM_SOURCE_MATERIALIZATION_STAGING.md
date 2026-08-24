# Phase 152 C03e-V — Borrowed Single-Worker Drive Seam Source Materialization Staging

Status: STAGED

Target gate:

`C03E_V_BORROWED_SINGLE_WORKER_DRIVE_SEAM_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-U:

- branch: `phase-152-c03e-u-remote-session-executor-owner-source-materialization-staging`
- head: `3e569630a87b658e0e17ddee5a73e74f16cfe2b0`
- tree: `5ef2f1a64058123f480696d17c11a44d2dc8052e`
- gate: `C03E_U_REMOTE_SESSION_EXECUTOR_OWNER_SOURCE_MATERIALIZED`

C03e-V preserves exact U lineage.

## Purpose

Materialize only the next boundary selected by C03e-T and left open by C03e-U:

1. use the private Agent-owned current-thread Tokio runtime;
2. drive exactly one already-materialized C03e-S worker future;
3. borrow both executor custody and authenticated-session custody for the duration of the drive;
4. preserve the C03e-S worker's existing cancellation/failure classification unchanged;
5. stop before task spawn, join ownership, shared-current-authority synchronization, concurrent session admission, Agent binary wiring, readiness or runtime activation.

## Exact source placement

The drive seam is added only to:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No new module is created.

No parent-module re-export is required because `RemoteSessionExecutorRuntime` is already exposed through the existing `remote_session_capability_runtime` surface.

## Materialized operation

The executor owner gains exactly one domain-specific synchronous drive method:

`RemoteSessionExecutorRuntime::drive_capability_request_worker(...)`

Its inputs remain the already-selected authorities and worker inputs:

- `&mut self` for exclusive mutable executor custody during the drive;
- `&mut AuthenticatedRemoteSessionRuntimeOwner` for one borrowed authenticated remote-session owner;
- current caller-supplied `&CapabilityBridge<'_, P>`;
- caller-supplied verifier-time provider `T: FnMut() -> u64 + Send`;
- caller-owned mutable dispatcher `&mut D` with `D: CapabilityDispatcher + Send`;
- caller-supplied cancellation future `C: Future<Output = ()> + Send`.

Policy remains `P: PolicyEvaluator + Sync`, matching the existing C03e-O/Q/S Send-safe async boundary.

The return value is the existing:

`AuthenticatedRemoteSessionWorkerStop`

No new completion/error enum is introduced.

## Exact drive sequence

The method performs exactly one internal future drive:

1. retain exclusive mutable borrow of the executor owner;
2. retain the caller's mutable borrow of the authenticated-session owner;
3. call the existing `AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_worker(...)` exactly once with the current bridge, verifier-time provider, dispatcher and cancellation future;
4. drive that one returned future through the private Tokio runtime with one internal `Runtime::block_on(...)` call;
5. return the exact `AuthenticatedRemoteSessionWorkerStop` produced by C03e-S unchanged.

No second worker future is constructed by the drive seam.

## Why the seam is domain-specific

C03e-V does **not** expose generic executor access.

Specifically it does not expose:

- `Runtime` by reference or value;
- Tokio `Handle`;
- generic `block_on<F>`;
- arbitrary future execution;
- generic task submission;
- task-local state;
- runtime metrics or driver internals.

The only newly exposed operation is the already-gated remote-session worker drive.

## Borrowing and serialization

The drive method uses `&mut self` rather than `&self` even though Tokio's underlying runtime can technically drive through an immutable reference.

This is deliberate contract-level serialization:

- one executor owner cannot be used through this API to drive two workers concurrently;
- no concurrent authenticated-session admission policy is selected;
- no scheduling/fairness/capacity semantics are implied;
- the current-thread runtime remains one explicit custody object rather than a process-global executor.

The authenticated-session owner is also borrowed mutably, preserving the existing C03e-O/Q/S serialization boundary.

## C03e-S semantics remain authoritative

C03e-V does not duplicate or reinterpret worker lifecycle behavior.

The existing C03e-S worker still owns the race between:

- the C03e-Q request loop; and
- caller-supplied cancellation.

Therefore existing behavior remains exact:

- Q failure first: existing code `3` / `remote capability session terminated` close remains authoritative and the original typed transaction failure is returned inside `AuthenticatedRemoteSessionWorkerStop::Failed`;
- cancellation first while Q is pending: Q future is dropped before peer close, then existing code `4` / `remote capability session shutdown` close occurs exactly once and stop is `AuthenticatedRemoteSessionWorkerStop::Cancelled`;
- a same-poll ready Q terminal failure continues to win because C03e-S polls Q before cancellation;
- no drive-layer close, retry, replacement session, or stop reclassification is added.

## Current authorization remains dynamic

C03e-V introduces no authorization cache or authority snapshot.

For every request actually processed by the existing worker path:

- retained `BoundRemoteSession` still supplies its bound transport identity and lease;
- `CapabilityBridge` still performs current registry/policy/transport-binding validation;
- caller-supplied verifier time remains fresh through the existing provider;
- caller-owned dispatcher remains mutable current execution authority.

C03e-V does not change registry, policy, lease or dispatcher ownership.

## Identity invariants

No executor/runtime/thread/task value becomes PRW identity.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- PID/UID/GID/thread/runtime identifiers are not logical identity.

## Synchronous owner-context boundary

The new drive operation is a synchronous owner-context seam around one current-thread Tokio runtime.

C03e-V does not select nested runtime driving from inside an already-running Tokio task. Future production invocation must preserve the selected executor-owner context rather than introducing hidden or nested runtime ownership.

This checkpoint does not add alternate nested-runtime detection, fallback executors, or re-entrant drive semantics.

## No task ownership yet

C03e-V intentionally does not materialize the later C03e-R task/collection model.

Absent here:

- `tokio::spawn`;
- `spawn_local`;
- `spawn_blocking`;
- `JoinHandle`;
- `JoinSet`;
- cancellation token/channel construction;
- worker registry or collection;
- task panic/join classification;
- concurrent authenticated-session admission;
- duplicate-DeviceId admission policy;
- worker capacity/fairness policy.

Those remain separately gated because current registry/policy authority is still borrowed and no shared-current-authority synchronization mechanism has been selected.

## No remote transport or production activation

Calling the drive seam can only drive the caller-supplied already-composed worker path.

C03e-V itself does not:

- construct `RemoteServerTransportRuntime`;
- bind UDP/QUIC;
- accept the first transport peer outside the existing worker path;
- authenticate or compose a replacement logical session;
- wire Agent `main.rs`;
- publish readiness;
- modify systemd/firewall/NAT/routing;
- deploy or merge.

## Dependency boundary

C03e-V requires no Cargo dependency change.

C03e-U already materialized the exact direct Agent Tokio dependency and exact lockfile edge. Therefore V must leave byte-stable unless a concrete validation defect proves otherwise:

- `crates/prw-agent/Cargo.toml`;
- `Cargo.lock`.

No new Tokio feature is selected.

## Expected final diff boundary

Intended U -> V net diff is exactly two paths:

1. this V contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`.

Expected unchanged paths include:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- Agent `lib.rs` and `main.rs`;
- bridge/transport source;
- Android application source;
- workflows;
- readiness, packaging/systemd and host-network source.

## Validation requirements

Closure requires on the final exact head:

- exact U merge base;
- exact two-path scope review unless a concrete validation defect proves otherwise;
- locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS;
- workspace tests PASS;
- workspace build PASS;
- canonical Android native/application validation if triggered by the Rust source change;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback verification;
- append-only rolling Drive update preserving the complete post-U prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-V means only that the private Agent-owned executor can synchronously drive exactly one borrowed existing cancellation-aware remote-session worker to its existing terminal classification.

It does **not** mean remote transport is activated, multiple sessions can run concurrently, worker tasks are spawned, current authority is safely shareable across `'static` tasks, the Agent binary is wired, or readiness can be published.

The next allowed checkpoint must separately select the next ownership boundary rather than silently introducing task concurrency.

Target gate:

`C03E_V_BORROWED_SINGLE_WORKER_DRIVE_SEAM_SOURCE_MATERIALIZED`
