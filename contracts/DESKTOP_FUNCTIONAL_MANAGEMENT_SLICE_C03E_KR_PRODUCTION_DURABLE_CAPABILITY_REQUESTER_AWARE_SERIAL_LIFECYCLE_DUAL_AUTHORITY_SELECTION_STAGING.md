# C03e-KR Production Durable Capability Requester-Aware Serial Lifecycle Dual-Authority Selection

Status: `SELECTION_STAGING`

Gate:

`C03E_KR_PRODUCTION_DURABLE_CAPABILITY_REQUESTER_AWARE_SERIAL_LIFECYCLE_DUAL_AUTHORITY_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_REQUESTER_AWARE_SERIAL_LIFECYCLE_DUAL_AUTHORITY_SELECTION`

## 1. Purpose

C03e-KR selects one future additive dormant requester-aware higher-owner worker that combines the existing C03e-KQ production-durable ingress cancellation worker with the existing FI requester/rendezvous DR and terminal-response continuation while preserving the two authority lanes as distinct inputs.

KR is selection-only. It does not materialize the selected worker, does not replace the legacy FI lifecycle/worker, does not modify FQ/FU, does not create a production authority aggregate, does not populate executable authority inputs, does not activate runtime/network behavior, and does not merge/deploy/restart/recover anything.

## 2. Exact predecessor authority

Predecessor checkpoint:

`C03e-KQ — Production durable capability repeated post-auth ingress cancellation worker source materialization`

Exact KQ branch:

`phase-152-c03e-kq-production-durable-capability-repeated-post-auth-ingress-cancellation-worker-source-materialization`

Exact KQ head / required merge base:

`9cfdfaebf27f04c336795fca699c43fae594f230`

Exact KQ tree:

`d9d57f344400114f290ff312c261e3e63b6e961e`

Exact KQ durable worker source blob:

`06c64d6ff69ef6862ea3d9c3dd3b4c0c1e34051d`

KQ PR #428 remains draft/open/unmerged and evidence-closed.

## 3. Fresh FI authority finding

Exact FI path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact KQ FI blob:

`bc0b9c49471d515b721c9cf47cd27ec3111f32ca`

The existing FI requester-aware serial lifecycle and cancellation-aware worker still receive one:

`&SharedCurrentCapabilityAuthority<P>`

and use that same input for two different responsibilities:

1. legacy EV/EX mixed-family capability ingress authorization;
2. requester/rendezvous DR continuation through `SharedRequesterRendezvousAuthority`.

The failed KI evidence already established the critical distinction:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

Therefore KR does not replace the shared-current authority input globally and does not reinterpret durable capability authority as requester/admission authority.

## 4. Fresh KQ capability-ingress finding

KQ already materializes the dormant executor-neutral durable ingress cancellation worker:

`AuthenticatedRemoteSessionRuntimeOwner::run_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability(...)`

That worker accepts:

- `&ProductionDurableCapabilityAuthority`;
- a verifier-time callback;
- a mutable capability dispatcher;
- a caller-owned cancellation future.

It returns:

`Result<Option<RequesterRendezvousResponseStreamCustodyHandoff>, AuthenticatedRemoteSessionPostAuthIngressTransactionError>`

Its polling law is already fixed:

1. ingress is polled first;
2. ready requester handoff outranks same-wake cancellation;
3. ready ingress failure outranks same-wake cancellation;
4. cancellation is polled only while ingress is pending;
5. cancellation returns only `Ok(None)`;
6. the in-flight KO loop future is dropped before cancellation return leaves KQ.

KR reuses that worker unchanged.

## 5. Fresh requester DR finding

Existing FI DR continuation remains:

`continue_requester_rendezvous_retained_custody_through_dr(...)`

It accepts:

- `&SharedCurrentCapabilityAuthority<P>`;
- requester-specific policy source;
- `&SharedRequesterRendezvousAuthority`;
- exact requester response-stream custody handoff.

That shared-current authority lane remains the requester/admission authority input for DR.

KR does not substitute `ProductionDurableCapabilityAuthority` into DR and does not change requester policy-source authority.

## 6. Selected future worker

A later source checkpoint may add exactly one dormant Agent-internal async function in FI:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

Selected generic shape:

```rust
pub(super) async fn run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability<
    P: PolicyEvaluator + Send + Sync,
    D: CapabilityDispatcher + Send,
    T: FnMut() -> u64 + Send,
    S: RequesterRendezvousStartPolicySource + Sync + ?Sized,
    C: Future<Output = ()> + Send,
>(
    session_owner: &mut AuthenticatedRemoteSessionRuntimeOwner,
    capability_authority: &ProductionDurableCapabilityAuthority,
    requester_dr_authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    requester_rendezvous_authority: &SharedRequesterRendezvousAuthority,
    verifier_time_unix_seconds: T,
    dispatcher: &mut D,
    cancellation: C,
) -> RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop
```

Exact formatting is rustfmt-owned.

## 7. Two authority lanes are explicit

The selected worker has two distinct authority inputs with non-overlapping responsibilities.

### Lane A — production durable capability ingress

`capability_authority: &ProductionDurableCapabilityAuthority`

This input may be passed only into the existing KQ durable ingress worker.

It authorizes capability-family ingress through the existing KG durable capability path.

It must not be supplied to requester DR, requester policy evaluation, admission, registration, candidate authority or transport identity derivation.

### Lane B — requester DR current authority

`requester_dr_authority: &SharedCurrentCapabilityAuthority<P>`

This input may be passed only into the existing requester/rendezvous DR continuation.

It remains the shared-current registry/policy authority consumed by existing requester DR semantics.

It must not be used as a fallback capability authority inside the new durable ingress path.

## 8. No aggregate authority selected

KR selects explicit parameters rather than a new aggregate/context type.

No structure such as:

`ProductionRemoteSessionAuthorities`

or equivalent is selected.

No new cloneable wrapper, global state, trait object, dynamic authority map, registry accessor or authority getter is selected.

If higher FQ/FU integration later requires an owned or cloneable durable authority handle, that ownership adaptation must be selected separately.

## 9. Selected cancellation ownership

The new FI worker must pin the caller-owned cancellation future exactly once and retain it across multiple requester-aware serial cycles.

The KQ worker consumes one cancellation future per ingress cycle, so FI must pass a temporary adapter future that polls the same retained pinned cancellation future without transferring or replacing the underlying cancellation ownership.

The selected adapter may be implemented with the existing `poll_fn` primitive around the retained pinned cancellation future.

The adapter must not create a task, channel, cancellation token replacement or second cancellation source.

## 10. Exact selected per-cycle order

Each FI durable requester-aware cycle must perform exactly:

1. invoke the existing KQ durable ingress cancellation worker once;
2. pass only `capability_authority` to that ingress worker;
3. pass the current verifier-time callback by mutable borrow so KQ/KO remain the verifier-time sampling owners;
4. pass one temporary cancellation adapter over the same retained FI cancellation future;
5. await KQ to one terminal ingress-cycle result;
6. if KQ returns `Ok(None)`, return `Cancelled` immediately;
7. if KQ returns an ingress error, return `Failed(Ingress(error))` immediately;
8. if KQ returns one requester handoff, stop polling cancellation;
9. run existing requester DR using only `requester_dr_authority`;
10. run existing terminal DR acknowledgement response composition;
11. if response composition fails, return `Failed(RequesterResponse(error))`;
12. after successful response completion, poll the retained FI cancellation future once before another KQ cycle begins;
13. if cancellation is ready, return `Cancelled` before the next verifier-time sample or stream accept;
14. otherwise begin the next serial cycle.

## 11. Cancellation priority before requester handoff

Before requester handoff, KQ remains the cancellation-race authority.

Therefore:

- already-ready requester handoff outranks same-wake cancellation;
- already-ready capability/requester/candidate ingress failure outranks same-wake cancellation;
- cancellation wins only while ingress remains pending;
- cancellation returns no ingress error and no fabricated handoff.

FI must not add a second competing ingress/cancellation race around KQ.

## 12. Cancellation suppression during requester DR and response

After KQ yields one requester/rendezvous handoff, cancellation must not be polled while:

1. exact requester DR continuation runs;
2. exact terminal acknowledgement framing/write runs.

This preserves existing FI law that caller cancellation cannot create a response-abandonment path after requester authorization/registration may already have committed.

Existing bounded transport timeouts remain authoritative inside terminal response I/O.

## 13. Cancellation checkpoint after terminal response

After successful requester DR acknowledgement response completion, FI must poll the retained cancellation future exactly once before another durable ingress cycle starts.

If cancellation became ready during DR/response, the worker returns:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled`

before:

- another verifier-time sample;
- another KQ invocation;
- another stream accept;
- another family-ingress read.

## 14. Verifier-time invariant

The FI dual-authority worker does not sample verifier time itself.

It retains the caller-supplied verifier-time callback and passes it by mutable borrow into KQ for each ingress cycle.

KO remains the exact sampling owner and samples once immediately before each KM transaction invocation.

Requester DR and terminal response composition perform no additional capability verifier-time sampling.

## 15. Capability-family chain

For capability traffic, the selected path is:

`FI dual-authority requester-aware worker`
→ `KQ durable cancellation worker`
→ `KO durable repeated loop`
→ `KM one-transaction durable accept/read wrapper`
→ one authenticated stream accept
→ one typed family-ingress read
→ `KK` typed-ingress processor
→ `KG` durable transaction helper
→ `ProductionDurableCapabilityAuthority`
→ authorized dispatch
→ exact same transaction response stream.

The FI worker performs no capability authorization, dispatch or response I/O itself.

`SharedCurrentCapabilityAuthority<P>` is not used as capability fallback on this path.

## 16. Requester/rendezvous chain

For requester traffic, KQ returns one exact:

`RequesterRendezvousResponseStreamCustodyHandoff`

The selected FI worker then uses existing:

`continue_requester_rendezvous_retained_custody_through_dr(...)`

with only:

`requester_dr_authority: &SharedCurrentCapabilityAuthority<P>`

plus the existing requester-specific policy source and shared requester authority.

The exact requester transaction remains retained across DR and is then consumed exactly once by existing terminal acknowledgement response composition.

No second stream is accepted for requester response.

## 17. Requester identity and correlation invariants

Requester identity continues to come only from the retained authenticated PRW application session.

The nominated logical target continues to come only from the strict decoded requester transaction.

Outer PRWM `request_id` remains correlation only.

Neither authority input may be derived from or selected by request-controlled correlation fields.

## 18. Candidate-publication behavior

Candidate-publication ingress remains fail-closed before any FI requester work through:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

KQ returns that exact error and FI maps it only into the existing `Ingress(...)` lifecycle failure category.

No candidate authority/provider invocation, candidate response, reachability continuation or retry is selected.

## 19. Error preservation

The future FI worker introduces no new error enum.

Ingress failure from KQ remains:

`RequesterRendezvousPostTerminalResponseSerialLifecycleError::Ingress(error)`

using the existing conversion from:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError`

Requester terminal response failure remains:

`RequesterRendezvousPostTerminalResponseSerialLifecycleError::RequesterResponse(error)`

The outer worker stop remains:

- `Cancelled`;
- `Failed(existing_error)`.

No failure is flattened, retried, suppressed or converted into fabricated success.

## 20. Existing FI lifecycle preservation

The later source materialization must leave unchanged:

- `continue_requester_rendezvous_retained_custody_through_dr(...)`;
- `complete_requester_rendezvous_terminal_dr_acknowledgement_response(...)`;
- `run_requester_rendezvous_post_terminal_response_serial_lifecycle(...)`;
- `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`.

The legacy FI lifecycle/worker remain available on the existing shared-current ingress path.

The new durable FI worker is additive and dormant.

## 21. FQ preservation

Exact FQ path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact KQ FQ blob:

`bc4520b2c13308b446230b43a2650d02e5b42cc2`

KR selects no FQ mutation.

`RemoteSessionExecutorRuntime::drive_recoverable_spawned_requester_rendezvous_worker(...)` remains on the legacy FI worker and existing `SharedCurrentCapabilityAuthority<P>` input.

No durable authority ownership/clone semantics are selected at FQ in KR.

## 22. FU preservation

Exact FU path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact KQ FU blob:

`2a07f03bb3c1739e4963a16c0ba7c30ae753d24e`

KR selects no FU mutation.

Real admission continues to use the existing shared-current authority for admission and legacy requester-aware worker spawning.

No durable capability authority parameter is propagated through the repeated admission collection in KR.

## 23. Durable authority custody preservation

Exact durable authority source path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact KQ blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

`ProductionDurableCapabilityAuthority` is already `Send + Sync` and internally retains durable-registry custody through `Arc<Mutex<_>>`, but the authority type itself is not currently selected as `Clone`.

KR does not add `Clone`, does not expose the inner `Arc`, and does not add a generic custody getter.

Higher task ownership remains separately gated.

## 24. Exact source-successor ceiling

The later KR source materialization may change at most one source path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

No second source path is selected.

The following must remain unchanged unless a new gate is opened:

- KQ durable worker source path;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- FQ recoverable spawned requester-aware worker;
- FU repeated real-admission requester-aware integration;
- durable authority custody source;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests/lockfile;
- workflows;
- Android source.

If correct compilation requires a second path, public API expansion, authority clone/ownership adaptation, FQ/FU mutation, production aggregate mutation or runtime activation, STOP and open a separate gate.

## 25. Source-successor proof obligations

A later source checkpoint must prove at minimum:

1. exactly one FI source path changed;
2. KQ worker source remains byte-identical;
3. legacy FI lifecycle and worker remain unchanged;
4. FQ/FU remain byte-identical;
5. durable authority custody remains byte-identical;
6. new worker accepts two explicit authority inputs;
7. durable authority is used only for KQ ingress;
8. shared-current authority is used only for requester DR within the new worker;
9. no capability fallback to shared-current authority exists;
10. one retained cancellation future spans all FI cycles;
11. KQ owns the pre-handoff ingress/cancellation race;
12. cancellation is not polled during requester DR/terminal response;
13. cancellation is checked after successful terminal response before a new ingress cycle;
14. FI samples no verifier time directly;
15. no task/channel/queue is created;
16. no peer close is added;
17. no requester/candidate provider scope is widened;
18. new worker remains dormant/uninvoked by FQ/FU.

## 26. Explicit exclusions

KR does not select:

- source materialization of the selected FI worker;
- mutation/replacement of legacy FI lifecycle/worker;
- FQ spawned-worker migration;
- FU repeated-admission migration;
- durable authority clone/ownership adaptation;
- authority aggregate/context creation;
- requester DR authority replacement;
- production aggregate replacement;
- executable durable-authority population;
- positive production capability grants;
- candidate authority/provider execution;
- registry/provider mutation;
- retry/reconnect/rebind/rebootstrap;
- peer-close policy change;
- task/channel/queue creation;
- listener/bind/readiness/runtime/network activation;
- `run()` or `main.rs` mutation;
- manifest/lockfile/workflow/Android-source mutation;
- deployment/restart/recovery;
- database/schema/control-plane mutation;
- repository visibility/configuration mutation;
- merge;
- PR close;
- ready-for-review conversion;
- branch deletion;
- history rewrite.

## 27. Validation law

Only the exact final KR head may be used as validation authority.

Because KR is docs-only, Android validation may be absent due to path filtering. An absent or skipped Android workflow must not be claimed as PASS.

Skipped workflows are never PASS.

## 28. Closure evidence law

KR may be recorded closed only after:

1. exact final branch head/tree re-read;
2. exact KQ -> KR topology proves ahead `1`, behind `0`, one docs path only;
3. exact-final-head required CI is terminal-success;
4. PR remains draft/open/unmerged on exact final head;
5. canonical Drive audit exact-title pre-upload search is zero;
6. frozen local raw Markdown bytes and SHA-256 are recorded;
7. upload uses canonical audit parent;
8. raw Drive readback matches exact frozen bytes/hash;
9. exact-title post-upload search returns exactly one canonical artifact;
10. PR metadata is updated to selection closure only after verified evidence publication.

## 29. STOP boundary

STOP after KR selection closure.

Do not materialize the FI dual-authority worker in KR.

Do not propagate durable authority into FQ/FU, add authority clone/ownership adaptation, populate production executable inputs, activate runtime/network behavior, merge, deploy, restart/recover or mutate repository configuration without a separately selected successor gate.
