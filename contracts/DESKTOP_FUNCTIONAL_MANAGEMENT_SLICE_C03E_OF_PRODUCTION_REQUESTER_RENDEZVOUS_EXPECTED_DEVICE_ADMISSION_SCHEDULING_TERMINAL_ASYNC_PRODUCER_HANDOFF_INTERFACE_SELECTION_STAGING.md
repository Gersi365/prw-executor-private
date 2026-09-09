# Desktop Functional Management Slice C03e-OF

## Production requester/rendezvous expected-device admission scheduling-terminal async producer handoff interface selection

Status: `SELECTION — VALIDATION_PENDING`
Date: 2026-09-09

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_INTERFACE_SELECTION`

Authoritative predecessor: exact closed C03e-OE head `71280ac6d60a5b6493ea908b9f287a0d6e60f6ca`, tree `e36e9bec309f02361288a513fce66406d1588106`.

This checkpoint changes documentation only. It selects a borrowed async producer interface, one retained in-flight producer future, cooperative supervisor progress, and the first independently materializable extraction prerequisite. It does not construct a production channel/request, install a producer, change Rust, or activate a runtime caller.

## 1. Decision

`BORROWED_LENDING_ASYNC_PRODUCER_INTERFACE_SELECTED / ONE_SCOPED_PENDING_FUTURE / RECEIVER_ADMISSION_AND_SHUTDOWN_PROGRESS_PRESERVED / SINGLE_READY_COMPLETION_EXTRACTION_REQUIRED / FIRST_SOURCE_STAGE_ONE_FILE_ADDITIVE_GENERIC_EXTRACTION / COMPLETE_HANDOFF_WIRING_DEFERRED`

Keep the sole sender owned by the future Agent higher-owner producer composition. Pass a temporary mutable borrow of that producer into a new dormant scheduling-aware sibling; do not move sender ownership into endpoint/executor/worker state.

The eventual producer uses a lending async callback, conceptually:

`H: AsyncFnMut(DeviceId, Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>) -> HandoffReceipt`

The sibling receives `producer: &mut H`. The callback is invoked with the exact disposed completion values by value. Its one call future may borrow the producer, including its sole sender. That future is pinned and polled within the existing runtime invocation, alongside receiver/admission/shutdown progress. It is never spawned, detached, rebuilt on every poll, or awaited in isolation from its own receiver.

This is a semantic interface selection, not a compiled full repository interface claim. HandoffReceipt below defines the selected custody/result law; source declaration and callback propagation remain later stages.

## 2. Exact source observations

All observations use exact OE, whose documentation change left OD source unchanged.

| Existing path under crates/prw-agent/src/remote_session_capability_runtime/ | Exact blob |
| --- | --- |
| remote_session_endpoint_lifecycle_runtime.rs | 530ed80783023bbd03a1f1d7784a1aeec53d0090 |
| remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs | eea51672c46aa83d50e6294a36e912fd0aa51928 |
| remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs | 4d2389bfc5e671f44b23438e258d328d92484895 |
| remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs | 264d18d57aafe9c6f67683843ded656e40d2d8cb |
| remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs | 7603b207b2efdd6c90a886c6e6f0abc232fa3bbc |
| requester_rendezvous_retained_custody_dr_continuation.rs | d1a68ea88d6721a622e0fd8ec54ef23ab136726f |

The endpoint-owner scheduling sibling accepts synchronous FnMut completion and forwards it unchanged.

The lower scheduling collection owns the expected-request receiver inside one existing runtime.block_on invocation. It calls reap_requester_aware_scheduling_workers before polling shutdown/expected-request and before polling shutdown/in-flight admission.

The endpoint disposer closes recovered peer/session-owner custody under the existing NX acknowledgement-only disposition law before passing (requester DeviceId, exact result) to completion.

The generic reap_ready_recoverable_workers scans all entries, collects all ready joins, then removes each entry, recovers its exact owner and publishes every completion synchronously. Its entry fields and owner recovery helper are private to that module.

The request poller checks explicit shutdown first; it polls the receiver only while the source is open and active.len() is below the worker bound. Receiver None merely marks the source closed.

## 3. Rejected direct callback conversion

Changing FnMut to an async callback and awaiting each completion inline is insufficient.

Concrete full-channel cycle:
1. One request already occupies the capacity-one channel.
2. A newly completed requester yields another construction-eligible grant.
3. Its producer awaits send(request) for capacity.
4. If the supervisor awaits that producer in isolation, it cannot poll its own receiver.
5. Capacity cannot become available: no timeout, retry, sender clone or hidden task is a valid repair.

The existing batch reaper also cannot feed a single lending async callback without either retaining several futures or staging completion values outside the active map. Both defeat the selected one-action custody bound.

Therefore both single-completion extraction and cooperative polling are required. A callback-bound change alone is rejected.

## 4. Selected owner and borrowing law

The producer composition is within the already-private remote-session capability ownership graph, where exact scheduling terminal/grant types are accessible. Do not widen them to linux_bootstrap, crate-public or public visibility merely to build this handoff.

The higher owner retains the concrete producer and sole sender. New endpoint/executor siblings borrow that producer for the duration of the synchronous outer drive. Borrowing its callback state is not sender ownership transfer.

No endpoint field, executor field, worker field, global/static slot, Arc<Mutex<_>>, or cloned sender is introduced.

The call future may borrow &mut H. At most one such future exists. Complete and destroy the entire pinned-future storage scope before invoking H again; dropping only a Pin<&mut _> handle does not release the underlying future's borrow.

Use stable AsyncFnMut bound/call syntax and ordinary lexical pinning. Do not directly name nightly-only AsyncFnMut associated types/methods or add nightly features.

A local Rust 1.97.1 language-shape compile probe verified successive borrowed async invocations with mutable captured state after the first pinned future's lexical scope ends. This is language-shape evidence only, not a production sender, Tokio integration, shutdown test or repository CI substitute.

## 5. Single pending action and existing active-map custody

Normal running state permits:
- the existing bounded active-worker map;
- at most one existing in-flight admission;
- at most one detached scheduling completion being converted into the one scoped producer future;
- the OE capacity-one expected-request channel.

There is no additional completion queue, Vec backlog, mailbox, second channel, detached task, or second producer future.

When no producer future is active, extract at most one ready completion. Remove its active entry and recover exact owner/result once. Invoke the unchanged scheduling peer disposer before producer delivery.

While a producer future is active, do not extract another completion for production delivery. Other completed joins remain in their existing bounded active entries until this future terminates. Their grants are not cloned, projected away or moved into a second backlog.

The private extraction prerequisite must stop scanning at the first Ready join. Do not poll a second Ready join and then discard its result; a completed JoinHandle must not later be polled again.

## 6. Cooperative progress law

The future scheduling-aware driver must integrate producer progress into both existing supervisor phases:
- awaiting the next expected request;
- awaiting the one in-flight admission.

Retain one pinned producer future across polls. Every relevant poll must give explicit shutdown priority, poll pending producer work with the current Context, and continue eligible receiver or in-flight admission progress. Pending producer work is not permission to return Pending before polling the consumer side.

Do not consume more than one request into admission at a time. Existing preflight, duplicate-active-device, authenticated DeviceId, admission timing, worker bound and worker creation rules remain unchanged.

A producer starts only after one active entry has been detached. With no second producer or sender clone, the selected topology permits an existing queued request to be received and free capacity for that one send. If admission is already in flight, it continues to be polled. This is a structural progress argument, not a claim that remote authentication or external peers always complete promptly.

Do not spin/self-wake merely because a slot is pending. Register real future/receiver/join wakeups. Bound ready work per outer iteration and continue shutdown polling so a stream of ready completions cannot monopolize the loop.

The old synchronous collection and endpoint siblings remain unchanged. The new behavior belongs in separately named dormant siblings.

## 7. Exact producer input authority

The producer receives the exact:
`(requester DeviceId, Result<SchedulingWorkerStop, JoinError>)`
after peer disposition.

Only SchedulingTerminal with scheduling_result Ok(grant) is eligible. The target DeviceId is extracted only from that consumed grant. The callback DeviceId remains requester-side correlation and never substitutes for target identity.

Requester acknowledgement result remains orthogonal: it neither creates nor revokes scheduling eligibility. Preserve it by value in terminal producer reporting.

No grant clone/copy/reconstruction/remint/replay, requester-session substitution, PRWC identifier reuse, synthetic dispatcher, frozen/default time or scheduling-consumption rollback is permitted.

## 8. Handoff receipt and failure propagation

Select one private receipt envelope with requester correlation and these semantic outcomes:

- Ineligible: retain the exact original non-eligible scheduling-stop/join result by value, including its typed derivation/acknowledgement channels where present. This variant must never carry an eligible Ok(grant).
- Eligible terminal: retain the exact acknowledgement result by value plus exactly one disposition: Enqueued, ConstructionFailed, ChannelClosed, or SuppressedOnShutdown.

These dispositions are bounded zero-data classifications. They contain no request bytes, path, endpoint, credential, grant, sender, request retry handle or producer reconstruction token.

For an eligible terminal, consume or deterministically dispose the grant once before reporting. No eligible grant returns in the receipt. ConstructionFailed means input acquisition/validation or request construction failed before enqueue; it is terminal for that one-shot continuation and must not be translated to success.

ChannelClosed means send failed or closure was known before construction/send. Any request returned by SendError is disposed once. There is no retry, fallback channel, remint or rollback.

Enqueued means only that Sender::send returned Ok(()); it does not prove receiver consumption, admission, authentication or worker success.

A private synchronous higher-owner receipt observer receives each receipt exactly once and does no blocking/async work. Dropping a receipt is not an implicit success. Endpoint Result<(), RemoteSessionPersistentCollectionConfigError> remains configuration/lifecycle status and must not absorb or erase producer failures.

The concrete future constructor-input errors remain governed by their own source gates; this receipt deliberately classifies the production stage failure without inventing those inputs or their error implementations.

## 9. Shutdown and send cancellation

OE's explicit supervisor shutdown remains the only endpoint-supervisor shutdown authority. Sender drop, ChannelClosed and request-source closure do not themselves request supervisor shutdown.

On observed explicit shutdown:
1. Mark producer quiescence; no new production handoff or enqueue may start.
2. Close the existing receiver to wake a pending send, and retire any buffered but unadmitted request deterministically. Never start a queued admission after quiescence.
3. Preserve and poll any already-started producer future to its terminal receipt alongside existing in-flight admission and worker cancellation/drain.
4. Do not drop and recreate the pending send future as a select-loser shortcut. Receiver closure permits the selected send to finish with ChannelClosed; a send that already committed may still report Enqueued.
5. Completions recovered during drain use the unchanged peer disposer. Non-eligible results retain their typed outcome; eligible grants are disposed with SuppressedOnShutdown and exact acknowledgement custody. They never invoke production request construction/send.
6. After the pending future and drain reporting finish, end the mutable producer borrow. The higher owner then drops its sole sender as cleanup.
7. Preserve endpoint close and wait_idle through the existing lifecycle.

The production callback must have no suspension dependency on another completion, a future grant, endpoint teardown or sender drop. Its enqueue wait is the selected send(request).await. Any future extra await requires a fresh lifecycle proof.

Closing a receiver does not create another shutdown authority: it is an effect of already-observed explicit shutdown. Buffered retirement is a shutdown disposition, not a full-channel drop policy.

## 10. First independently materializable source prerequisite

The smallest immediate source successor is exactly one existing path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`

It may add one dormant pub(super) helper with the conceptual signature:

`poll_one_ready_recoverable_worker<K, O, T>(&mut HashMap<K, RecoverablePersistentWorkerEntry<O, T>>, &mut Context<'_>) -> Poll<RecoverablePersistentWorkerCompletion<K, O, T>>`

Constraints:
- K: Eq + Hash + Clone, as existing map custody requires.
- Return at most one detached ready completion per call.
- Scan pending handles with the supplied Context; on first Ready, stop scanning, remove that entry, recover exact owner through the existing private helper, and use the existing join-result mapper.
- Clone only the map key for detachment as needed. Do not clone owner, worker result or grant.
- Empty or all-pending maps return Pending without fabricated output, synchronous wait or self-wake loop.
- No FIFO/order guarantee is selected for HashMap iteration.
- Add focused same-file tests and only narrowly required dormant lint acknowledgement.
- Existing entry/completion fields, visibility, constructors, batch reaper, cancellation helper and drain behavior remain unchanged.
- No caller migration, scheduling-specialized type import, sender/channel, async producer, request construction or runtime activation is permitted in this first source stage.

This revisits the earlier statement that generic primitives needed no change: exact OE now proves that the existing all-ready callback primitive lacks the required one-at-a-time extraction. An additive helper inside its own private-field module is necessary; widening entry fields or reimplementing custody outside that module is rejected.

Future source tests must cover empty/all-pending behavior with real wakeup registration; two ready entries yielding exactly one per call; untouched remaining entry and later exact recovery; non-Clone owner/result movement; and existing join-error mapping. Tests must not depend on HashMap iteration order or process-global environment mutation.

## 11. Full handoff source graph remains staged

The eventual dormant handoff graph touches at least these four existing paths:
1. The one-file generic extraction prerequisite above.
2. remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs: scheduling-specific single-completion adaptation using the generic helper.
3. remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs: parallel driver, cooperative producer/receiver/admission polling and shutdown/drain reporting.
4. remote_session_endpoint_lifecycle_runtime.rs: borrowed-producer forwarding sibling and boundary-private receipt interface.

All paths are under crates/prw-agent/src/remote_session_capability_runtime/. The existing scheduling disposer and scheduling terminal/grant definitions are reused unchanged.

This four-path graph is a dependency inventory, not authorization to implement all four now. The immediate ceiling is only the generic extraction file in section 10. After it closes, audit the exact new head before selecting the next driver/receipt propagation source stage.

## 12. Remaining construction dependencies

Actual channel construction, production callback installation, request constructor invocation and send remain deferred until all required sources are materially available:
- independent target admission SessionId source under OB's server-local CSPRNG law;
- independent nonzero expected-device PRWM authentication request-ID source with live-transaction uniqueness/exhaustion law;
- NB status-only dispatcher construction/transfer;
- verifier-time source/interface compatible with the existing request-carried timing bound.

No complete production factory or live expected-device request is claimed by this interface selection.

## 13. Required future integration validation

Before any future handoff wiring can close, validate at minimum:
- full capacity-one channel plus one pending send still allows receiver progress;
- a send remains one retained future across Pending polls;
- multiple ready workers do not create a second future/backlog or lose terminal custody;
- in-flight admission continues while producer work is pending;
- explicit shutdown wins over starting new producer/admission work;
- receiver closure completes a pending send with bounded failure and deterministic disposal;
- a send that committed before shutdown is reported as enqueue-only success;
- eligible drain completions are suppressed without construction or remint;
- non-eligible typed results and acknowledgement disposition remain intact;
- caller-owned producer/sender outlives the loan, and its next mutable invocation begins only after the prior future storage is destroyed;
- historical sibling behavior remains unchanged.

OF documentation CI does not substitute for these future source/integration tests.

## 14. Documentation scope, validation and durable closure

Only this contract path may differ from exact OE:
`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_OF_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_TERMINAL_ASYNC_PRODUCER_HANDOFF_INTERFACE_SELECTION_STAGING.md`

Validate exact OE merge base, one added docs file, zero Rust/Cargo/lockfile/workflow/runtime changes, and actual terminal exact-head CI conclusions. SKIPPED is never PASS.

Publish one immutable raw audit to canonical Drive parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT` only after exact-title presearch zero. Require raw readback byte/SHA-256 equality and postsearch exactly one before PR closure metadata.

Keep the PR draft/open/unmerged. No merge, close/ready conversion, deployment, branch deletion, history rewrite or repository configuration mutation.

After this documentation checkpoint closes, STOP. A fresh authority/concurrency check is required before assigning a successor token or materializing the selected single-completion extraction prerequisite.

## 15. Primary API references and limits

- Rust AsyncFnMut: https://doc.rust-lang.org/std/ops/trait.AsyncFnMut.html
- Tokio Sender::send: https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html#method.send
- Tokio Receiver::close: https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.close

The Tokio pages identified version 1.53.1 when consulted. The Rust web page identified 1.98.1; stable bound/call syntax was separately compiled locally with the repository-pinned Rust 1.97.1. No toolchain/dependency change is selected.
