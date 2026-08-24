# Phase 152 C03e-AA — Lexically-Contained Spawned Worker Ownership Selection Staging

Status: STAGED

Target gate:

`C03E_AA_LEXICALLY_CONTAINED_SPAWNED_WORKER_OWNERSHIP_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-Z:

- branch: `phase-152-c03e-z-shared-current-single-worker-transaction-integration-source-materialization-staging`
- head: `d123c647d7a328e8eb12465986ca3cd9a6e7ad7e`
- tree: `1cd8cbadbea7d1ebb581a982aee19a514c84081f`
- gate: `C03E_Z_SHARED_CURRENT_SINGLE_WORKER_TRANSACTION_INTEGRATION_SOURCE_MATERIALIZED`

C03e-AA preserves exact Z lineage.

## Purpose

Select only the first concrete task-ownership boundary after C03e-Z proved fresh shared-current authorization in the borrowed single-worker path.

The selected next source seam is one lexically-contained spawned worker:

1. one Agent-owned private current-thread runtime remains the executor custody;
2. one existing authenticated-session owner moves into exactly one spawned task by value;
3. one clone of `SharedCurrentCapabilityAuthority<P>` moves into that same task, cloning only the outer `Arc` selected in C03e-W/X;
4. one dispatcher, verifier-time provider and caller-supplied cancellation future move into the task by value;
5. the spawning supervisor retains the one `JoinHandle` locally and awaits it exactly once before the enclosing private-runtime drive returns;
6. no raw task handle, runtime handle or cancellation primitive escapes;
7. no worker collection, second worker, concurrent authenticated-session admission or production activation is selected.

This checkpoint selects task custody without selecting detached tasks or multi-worker scheduling.

## Selected executor seam

The future source seam is a bounded `RemoteSessionExecutorRuntime` operation equivalent in responsibility to:

`drive_spawned_capability_request_worker(...)`

It remains a method on the existing non-cloneable `RemoteSessionExecutorRuntime` and takes `&mut self`.

The `&mut self` borrow remains held for the whole drive, preserving explicit serialization at this checkpoint.

The operation consumes by value:

- one `AuthenticatedRemoteSessionRuntimeOwner`;
- one dispatcher `D`;
- one verifier-time provider `T`;
- one cancellation future `C`.

It receives the current shared authority by borrowed reference and makes exactly one `SharedCurrentCapabilityAuthority<P>` clone before task creation.

That clone must clone only the existing outer `Arc`; it must not require `P: Clone` and must not clone `WorkspaceDeviceRegistry`.

## Spawn primitive

The selected task primitive is Tokio task spawn under the already-owned current-thread Tokio runtime.

The implementation may use `tokio::spawn(...)` only while executing inside the private runtime context established by the same bounded drive operation, or an equivalent runtime-owned spawn primitive with identical custody.

No global executor, second runtime, thread pool or second executor family is selected.

The existing direct Agent Tokio dependency already has `rt` and therefore no dependency/feature expansion is selected by C03e-AA.

Direct Agent `rt-multi-thread` and `macros` remain unselected.

## Spawned future ownership

The spawned `async move` worker owns exactly the resources required by the existing C03e-S/Z worker body:

- the `AuthenticatedRemoteSessionRuntimeOwner` by value;
- the shared-current authority clone by value;
- dispatcher `D` by value;
- verifier-time provider `T` by value;
- caller-supplied cancellation future `C` by value.

Inside the task, the existing C03e-S worker remains authoritative.

The task calls the existing:

`AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_worker(...)`

with a borrowed reference to the task-owned shared-current authority, mutable task-owned dispatcher, task-owned verifier-time provider and task-owned cancellation future.

No raw peer, second bound session, second logical identity, registry snapshot, policy snapshot or authorization decision is copied into task metadata.

## Required task bounds

A separately gated source materialization must prove the exact task capture is accepted by Tokio without weakening existing ownership.

Expected generic constraints are bounded to what the spawned future actually requires:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> u64 + Send + 'static`;
- `C: Future<Output = ()> + Send + 'static`.

The source materialization must also compile-time prove the existing `AuthenticatedRemoteSessionRuntimeOwner` can move into the selected spawned future.

No `Clone` bound is selected for the session owner, dispatcher, verifier-time provider or cancellation future.

## Join ownership

The spawning supervisor future owns exactly one:

`tokio::task::JoinHandle<AuthenticatedRemoteSessionWorkerStop>`

The handle is local implementation detail only.

It is not:

- returned to the caller;
- stored in a persistent field;
- exposed through an accessor;
- converted to an abort handle;
- detached;
- inserted into a collection;
- keyed by DeviceId or transport identity.

The supervisor awaits the handle exactly once before the bounded drive returns.

On every normal code path, no spawned task may outlive the lexical supervisor future that created it.

## Abnormal task completion

The selected public/domain result must distinguish existing worker stop from abnormal Tokio join failure without exposing Tokio internals.

A future bounded Agent error equivalent in responsibility to:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`

may classify any `JoinError` produced by the one local handle.

The mapping must not expose:

- panic payloads;
- backtraces;
- executor identifiers;
- task identifiers;
- raw Tokio `JoinError` as public PRW identity or protocol data.

No retry, replacement task, replacement session, replacement peer or automatic re-authentication follows abnormal task completion.

## No hard abort

C03e-AA does not select task abort as a lifecycle primitive.

The future implementation must not call:

- `JoinHandle::abort()`;
- `AbortHandle::abort()`;
- `JoinSet::shutdown()`;
- any equivalent hard-cancellation primitive.

The existing C03e-S caller-supplied cancellation future remains the only selected orderly worker cancellation signal.

## Cancellation-controller boundary remains separate

C03e-AA deliberately does not construct a concrete cancellation controller/channel.

The first spawned seam receives the already-existing generic cancellation future `C` by value and moves it into the worker task unchanged.

Therefore this checkpoint does not yet select:

- oneshot/watch/broadcast channel construction;
- cancellation token ownership;
- controller cloning;
- shutdown fan-out;
- process-signal wiring.

Those remain separately gated with persistent worker collection and orderly multi-worker shutdown.

## Existing shared-current authorization remains authoritative

Task ownership does not change C03e-Z authorization semantics.

For every protected request the task-owned worker still:

1. accepts one request stream through the retained authenticated peer;
2. receives one bounded request frame;
3. acquires one fresh `SharedCurrentCapabilityAuthority` read operation;
4. constructs one ephemeral `CapabilityBridge` over current registry/policy state;
5. calls retained `BoundRemoteSession::authorize(...)` exactly once;
6. releases the authority read guard;
7. dispatches the owned authorized request outside the guard;
8. sends the bounded response outside the guard.

No guard is retained in task state between requests.

## Existing worker semantics remain authoritative

The spawned task does not redefine C03e-Q/S lifecycle behavior.

The existing worker still owns the race between:

- the C03e-Q serial request loop; and
- the caller-supplied cancellation future.

Existing semantics remain:

- request loop polled before cancellation on each wake;
- real request-loop failure wins a same-poll tie;
- request-loop failure retains code `3` / `remote capability session terminated` and original typed transaction failure;
- cancellation wins only while the request loop remains pending;
- cancellation drops the request-loop future first;
- cancellation then closes the same retained peer exactly once with code `4` / `remote capability session shutdown`;
- terminal stop remains `AuthenticatedRemoteSessionWorkerStop::Cancelled` or `Failed(...)` before the outer join layer classifies abnormal task failure.

## Current-thread runtime retained

C03e-AA does not change the executor selected in C03e-T/U.

The runtime remains:

- Agent-owned;
- non-cloneable;
- Tokio current-thread;
- private;
- built with I/O/time drivers enabled.

The bounded spawned drive still uses one private `Runtime::block_on(...)` supervisor call.

The spawned task executes under that same runtime custody.

No `rt-multi-thread` feature or background runtime thread is selected.

## No concurrent admission

Although one task is spawned internally, the selected seam is still semantically single-worker.

The method's `&mut self` borrow and mandatory join-before-return mean:

- a second bounded spawned drive cannot be entered concurrently through the same executor owner;
- no persistent second worker is retained;
- no network accept loop admits another authenticated session while this seam is active;
- no collection capacity, fairness or scheduling policy is selected.

C03e-AA therefore does not claim concurrent authenticated-session support.

## No worker collection

No `Vec<JoinHandle<_>>`, `HashMap`, `BTreeMap`, `JoinSet`, slab, queue or other persistent worker collection is selected.

No collection key is selected.

In particular C03e-AA does not decide:

- DeviceId duplicate-session policy;
- transport-identity keying;
- local worker-ID generation;
- maximum active worker count;
- admission backpressure;
- fairness;
- completion polling order;
- shutdown drain ordering across multiple workers.

Those remain a later explicit selection checkpoint.

## Existing borrowed seam retained

The existing C03e-V/Z borrowed:

`RemoteSessionExecutorRuntime::drive_capability_request_worker(...)`

remains unchanged at the AA selection checkpoint.

The later source materialization must add the spawned-and-joined seam without silently deleting or behavior-changing the already-validated borrowed seam unless a separate contract explicitly selects that removal.

This preserves a regression reference and avoids conflating task-ownership materialization with API cleanup.

## Dependency boundary

C03e-AA selects no dependency, feature or lockfile change.

These must remain unchanged through the selection checkpoint:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

The future source materialization is expected to use existing Tokio `rt` support and the already-materialized shared-current authority.

## Identity invariants

Task ownership creates no PRW identity.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- Tokio task IDs, runtime IDs, thread IDs, PID/UID/GID and lock identities are not logical identity.

The task-owned session continues to revalidate current transport binding on every protected request through C03e-Z.

## Explicit non-selection

C03e-AA does not select or perform:

- persistent/detached task handles;
- hard task abort;
- worker collections;
- concrete cancellation-controller construction;
- shutdown fan-out;
- multi-worker drain;
- concurrent authenticated-session admission;
- duplicate logical-device admission policy;
- capacity/fairness scheduling;
- Agent `main.rs` wiring;
- remote listener/bind activation;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge.

## Exact selection scope

C03e-AA is docs-only.

Its final Z -> AA net diff must contain exactly this contract path and no source/manifest/workflow/application/host changes.

## Validation requirements

Closure requires on the final exact AA head:

- exact Z merge base;
- one-path docs-only net scope;
- permanent Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- Android validation not required to trigger for a docs-only exact head; no Android PASS may be claimed if it does not run;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-Z prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-AA means only that the first task-ownership shape is selected: one task owns one session worker's runtime resources by value, the shared authority is Arc-cloned into that task, and one local JoinHandle is awaited before the same bounded private-runtime drive returns.

It does not mean the spawned seam exists in Rust source, a concrete cancellation controller exists, workers are stored persistently, multiple authenticated sessions run concurrently, Agent `main.rs` is wired, remote transport is activated or readiness may be published.

The next checkpoint may materialize only this lexically-contained spawned-and-joined single-worker seam and its bounded abnormal-join classification.

Target gate:

`C03E_AA_LEXICALLY_CONTAINED_SPAWNED_WORKER_OWNERSHIP_SELECTED`
