# Phase 152 C03e-AC — Single-Worker Cancellation Pair Selection Staging

Status: STAGED

Target gate:

`C03E_AC_SINGLE_WORKER_CANCELLATION_PAIR_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-AB:

- branch: `phase-152-c03e-ab-lexically-contained-spawned-worker-source-materialization-staging`
- head: `fb0d8a0302b9cc883e83ce7a63e4b16281d3cf35`
- tree: `230b870cdd80478cfec5dbd2941053aba29c2cad`
- gate: `C03E_AB_LEXICALLY_CONTAINED_SPAWNED_WORKER_SOURCE_MATERIALIZED`

C03e-AC preserves exact AB lineage.

## Purpose

Select only the first concrete orderly cancellation-controller ownership shape for the already-proven single spawned-and-joined remote-session worker.

C03e-AB already accepts a caller-supplied `Future<Output = ()> + Send + 'static` and moves that future into exactly one lexically-contained worker task. C03e-AC selects a bounded Agent-owned pair capable of producing that future without selecting worker collections, shutdown fan-out, process-signal wiring, concurrent authenticated-session admission or task abort.

The selected pair is conceptually:

- one `RemoteSessionWorkerCancellationController`;
- one `RemoteSessionWorkerCancellationSignal`;
- one private shared state containing a monotonic cancellation flag plus one Tokio async wake primitive.

The pair is one-controller / one-signal and is not a multi-worker broadcast primitive.

## Selection rationale

Existing Linux runtime code already uses a monotonic `Arc<AtomicBool>` shutdown gate for scheduler control. That precedent establishes that shutdown state should be durable and independently observable rather than represented only by an edge-triggered notification.

Existing `LocalLinuxRuntimeWake` is deliberately only a runtime wake transport and explicitly does not own shutdown semantics. Existing `LocalLinuxWorkerCancellation` is specific to terminal shutdown of an authenticated local Unix stream and is not the remote-session task cancellation authority.

Therefore C03e-AC selects a new remote-session cancellation pair rather than reusing either Linux-specific type.

## Shared cancellation state

The future source shape owns one private state equivalent in responsibility to:

- `requested: AtomicBool` initialized `false`;
- `wake: tokio::sync::Notify`.

The state is shared internally by controller and signal through one `Arc`.

The `Arc` is an implementation detail only. It is not PRW identity, is not exposed as a generic shared-state handle, and does not imply controller or signal cloneability.

## Controller semantics

`RemoteSessionWorkerCancellationController` owns one internal reference to the private state.

Its selected cancellation operation is equivalent in responsibility to:

`request_cancellation(&self)`

The operation:

1. stores `true` into the monotonic flag with release ordering;
2. posts one wake for the single signal waiter;
3. returns without waiting for worker completion.

Cancellation request is idempotent. Repeated requests do not create additional lifecycle states and do not fail merely because cancellation was already requested.

The controller does not expose:

- raw `Notify`;
- raw `Arc`;
- task handle;
- abort handle;
- transport close handle;
- runtime handle;
- logical or transport identity.

Dropping the controller without calling `request_cancellation` is **not** cancellation.

## Single-signal semantics

`RemoteSessionWorkerCancellationSignal` owns the other internal reference to the private state.

It is consumed to create exactly one `'static` cancellation future suitable for the C03e-AB spawned drive. The future is equivalent in responsibility to:

`into_cancelled(self) -> impl Future<Output = ()> + Send + 'static`

The future completes only after the monotonic flag is observed true.

If cancellation has already been requested before the future begins polling, the future completes promptly from the flag without requiring a new notification.

If cancellation is requested while the future is pending, the Tokio wake causes it to observe the monotonic flag and complete.

The selected implementation must avoid a lost-wake race between checking the flag and awaiting notification. For the single-waiter shape, `Notify::notify_one()` or an equivalently durable single-waiter notification primitive may be used together with the monotonic flag.

Dropping the signal is not a cancellation request and has no effect on the controller other than releasing the private shared-state reference.

## No channel-close cancellation

C03e-AC deliberately rejects sender-drop/channel-close as cancellation semantics.

The cancellation future must not complete merely because the controller was dropped. This avoids accidental session cancellation caused by ownership teardown, panic unwinding or scope exit unrelated to an explicit orderly cancellation request.

This is one reason the selected state is monotonic flag + wake rather than a bare oneshot receiver whose sender disappearance has terminal receive semantics.

## Clone boundary

C03e-AC does not select `Clone` for either public half.

- the controller is one explicit authority for this single worker;
- the signal is one explicit waiter for this single worker.

The private state may use `Arc` internally only so the two halves can move independently.

Future multi-worker shutdown fan-out, controller cloning or multiple signals must be selected separately.

## Integration with C03e-AB

C03e-AC does not change the C03e-AB executor API at the selection checkpoint.

A future source materialization may construct one pair, retain the controller outside the bounded drive, and pass:

`signal.into_cancelled()`

as the existing generic cancellation future consumed by:

`RemoteSessionExecutorRuntime::drive_spawned_capability_request_worker(...)`.

The C03e-AB worker remains the authority for orderly cancellation consequences:

- cancellation wins only while the C03e-Q request loop remains pending;
- the request-loop future is dropped first;
- the same retained peer is then closed exactly once with code `4` / `remote capability session shutdown`;
- terminal result remains `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

The new pair only supplies the cancellation readiness event; it does not close transport or classify worker termination itself.

## No hard abort

C03e-AC does not select or expose:

- `JoinHandle::abort()`;
- `AbortHandle`;
- `JoinSet::shutdown()`;
- task cancellation by dropping a join handle;
- runtime shutdown as worker cancellation.

Orderly cancellation continues through the C03e-S future race selected earlier.

## No worker collection or fan-out

The selected pair controls exactly one worker.

It does not select:

- a `Vec`, map, slab or `JoinSet` of workers;
- multiple cancellation signals;
- broadcast/watch fan-out;
- worker IDs;
- DeviceId-keyed cancellation;
- TransportIdentity-keyed cancellation;
- capacity/fairness policy;
- multi-worker drain ordering.

No collection key or duplicate-session policy is selected.

## Concurrency boundary

C03e-AC does not make concurrent authenticated-session admission available.

The proven C03e-AB bounded drive still holds `&mut RemoteSessionExecutorRuntime` and joins its one spawned worker before returning.

The pair only makes the already-existing single worker cancellable by an explicit external authority. It does not create a second active remote worker or listener loop.

## Dependency boundary

C03e-AC selects no new crate and no Cargo feature.

Agent already depends directly on Tokio with `sync` and `rt` support. The selected monotonic state uses only:

- `std::sync::atomic::AtomicBool`;
- `std::sync::Arc`;
- `tokio::sync::Notify`.

These must remain byte-stable through the selection checkpoint:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

No `rt-multi-thread`, `macros`, external cancellation-token crate or new synchronization dependency is selected.

## Memory ordering

The future source materialization should use release ordering when requesting cancellation and acquire ordering when the signal observes the monotonic flag.

The flag carries only the cancellation state; no registry, policy, session or identity payload is published through it.

The exact ordering exists to make the monotonic state transfer explicit, not to create a general-purpose synchronization channel.

## Identity invariants

Cancellation ownership creates no PRW identity.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- cancellation-state Arc identity, Notify identity, task ID, runtime ID, thread ID, PID/UID/GID and lock identity are not logical identity.

The cancellation controller must not be keyed or addressed by those implementation identities in this checkpoint.

## Error boundary

Requesting cancellation is selected as infallible local state transition for this pair.

No public cancellation error type is required merely for setting the monotonic flag or posting the in-process wake.

Worker execution and join errors remain the existing C03e-S/AB domain results. Cancellation pair construction is also selected as infallible because it allocates only ordinary in-process synchronization state.

Any future resource-bounded or externally backed cancellation mechanism would require a separate contract and error model.

## Exact selection scope

C03e-AC is docs-only.

Its final AB -> AC net diff must contain exactly this contract path and no source, manifest, workflow, application or host changes.

## Validation requirements

Closure requires on the final exact AC head:

- exact AB merge base;
- one-path docs-only net scope;
- permanent Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- Android validation is not required to trigger for the docs-only exact head; no Android PASS may be claimed if it does not run;
- skipped workflows recorded as skipped, never PASS;
- immutable Drive audit raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AB prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Explicit non-selection

C03e-AC does not select or perform:

- source materialization of the pair;
- controller/signal `Clone`;
- broadcast/watch cancellation fan-out;
- worker collections;
- persistent join-handle custody;
- multi-worker shutdown or drain;
- task abort;
- concurrent authenticated-session admission;
- duplicate DeviceId policy;
- capacity/fairness scheduling;
- process signal wiring;
- Agent `main.rs` wiring;
- remote listener/bind activation;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge.

## Completion meaning

Closure of C03e-AC means only that the concrete single-worker cancellation ownership shape is selected: one controller explicitly flips one monotonic cancellation state and wakes one signal future, while controller drop alone does not cancel.

It does not mean the pair exists in Rust source, the controller is wired into a supervisor, workers are stored persistently, multiple authenticated sessions run concurrently, process shutdown reaches remote workers, Agent `main.rs` is wired, transport is activated or readiness may be published.

The next checkpoint may materialize only this single-worker cancellation pair and its focused tests before any persistent worker collection or multi-worker shutdown ownership is selected.

Target gate:

`C03E_AC_SINGLE_WORKER_CANCELLATION_PAIR_SELECTED`
