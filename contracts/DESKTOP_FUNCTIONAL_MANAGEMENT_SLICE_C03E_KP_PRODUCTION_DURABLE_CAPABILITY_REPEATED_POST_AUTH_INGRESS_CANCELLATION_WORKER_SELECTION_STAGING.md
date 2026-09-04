# C03e-KP Production Durable Capability Repeated Post-Auth Ingress Cancellation Worker Selection

Status: `SELECTION_STAGING`

Gate:

`C03E_KP_PRODUCTION_DURABLE_CAPABILITY_REPEATED_POST_AUTH_INGRESS_CANCELLATION_WORKER_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_REPEATED_POST_AUTH_INGRESS_CANCELLATION_WORKER_SELECTION`

## 1. Purpose

C03e-KP selects one future additive dormant executor-neutral cancellation-aware worker around the C03e-KO production-durable repeated post-auth ingress loop.

KP is selection-only. It does not materialize the worker, does not replace the existing legacy EX worker, does not modify FI/FQ/FU requester-aware callers, does not introduce dual-authority propagation, does not replace requester DR authority, does not populate production executable inputs, and does not activate runtime/network behavior.

## 2. Exact predecessor authority

Predecessor checkpoint:

`C03e-KO — Production durable capability repeated post-auth ingress loop source materialization`

Exact KO branch:

`phase-152-c03e-ko-production-durable-capability-repeated-post-auth-ingress-loop-source-materialization`

Exact KO head / required merge base:

`1dc3ff0c0a06af9a845cc86e911ddd978ded02e4`

Exact KO tree:

`bcd44e2ed5241574e191dfc4eb99247eca21c953`

Exact KO target source blob:

`50be786be7fe8283f5dd8276f8d2054ff608f9dd`

KO PR #426 remains draft/open/unmerged and evidence-closed.

## 3. Fresh source finding

Exact KO source path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

KO already materializes:

`run_repeated_post_auth_control_stream_ingress_with_production_durable_capability(...)`

The existing legacy cancellation-aware worker remains:

`run_repeated_post_auth_control_stream_ingress_worker(...)`

Its semantics are already suitable as the structural model for one durable analogue:

1. own exactly one repeated-ingress future;
2. own exactly one caller-supplied cancellation future;
3. poll ingress first on each wake;
4. if ingress is ready with a requester handoff, return `Ok(Some(handoff))`;
5. if ingress is ready with an error, return that exact error;
6. only if ingress is pending, poll cancellation;
7. if cancellation is ready, return `Ok(None)`;
8. otherwise remain pending;
9. keep the race in a lexical block so the in-flight ingress future is dropped before the cancellation result leaves the method and the exclusive mutable owner borrow is released first.

The existing worker performs no task spawn and no whole-peer close.

## 4. Higher-caller finding

Exact FI source remains separately responsible for requester/rendezvous DR continuation under `SharedCurrentCapabilityAuthority<P>`.

The failed KI evidence already established:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

Therefore KP does not modify FI/FQ/FU and does not attempt to replace the existing shared-current requester/admission authority with durable capability authority.

The selected worker remains dormant until a later separately selected higher caller-composition gate determines whether and how both authority lanes are supplied.

## 5. Selected future method

A later source checkpoint may add exactly one Agent-internal async method owned by `AuthenticatedRemoteSessionRuntimeOwner`:

`run_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability(...)`

Selected generic shape:

```rust
pub(crate) async fn run_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability<
    D: CapabilityDispatcher + Send,
    T: FnMut() -> u64 + Send,
    C: Future<Output = ()> + Send,
>(
    &mut self,
    authority: &ProductionDurableCapabilityAuthority,
    verifier_time_unix_seconds: T,
    dispatcher: &mut D,
    cancellation: C,
) -> Result<
    Option<RequesterRendezvousResponseStreamCustodyHandoff>,
    AuthenticatedRemoteSessionPostAuthIngressTransactionError,
>
```

Exact formatting is rustfmt-owned.

No policy generic `P` is selected for the durable worker.

## 6. Selected worker composition

The future worker must construct exactly one pinned KO repeated-loop future:

`self.run_repeated_post_auth_control_stream_ingress_with_production_durable_capability(...)`

using:

- the exact borrowed `ProductionDurableCapabilityAuthority`;
- the exact verifier-time callback transferred into the KO loop;
- the exact mutable dispatcher.

It must construct exactly one pinned caller-supplied cancellation future.

It must not call the legacy shared-current repeated loop.

## 7. Exact polling priority

The future worker must preserve loop-first priority exactly.

On every wake:

1. poll the KO durable ingress-loop future first;
2. `Poll::Ready(Ok(handoff))` returns `Poll::Ready(Ok(Some(handoff)))` immediately;
3. `Poll::Ready(Err(error))` returns `Poll::Ready(Err(error))` immediately;
4. only `Poll::Pending` permits polling cancellation;
5. cancellation `Poll::Ready(())` returns `Poll::Ready(Ok(None))`;
6. otherwise return `Poll::Pending`.

An already-ready requester handoff or ingress error therefore outranks same-wake cancellation.

KP selects no alternative fairness policy.

## 8. Cancellation semantics

Cancellation is caller-owned and is recognized only while the KO repeated ingress loop is pending.

Cancellation must not:

- be converted into an ingress error;
- fabricate a requester handoff;
- close the retained authenticated peer;
- emit a response frame;
- trigger retry/reconnect/rebind;
- restart the worker;
- launch another task;
- select candidate/reachability state;
- run requester DR/provider work.

`Ok(None)` remains the sole cancellation result.

## 9. Drop / exclusive-owner invariant

The future worker must retain the existing lexical race-block pattern.

When cancellation wins, the pinned KO repeated-loop future must be dropped before the method returns `Ok(None)`.

This releases the repeated-loop future's exclusive mutable borrow of `AuthenticatedRemoteSessionRuntimeOwner` before the cancellation result leaves the method.

No leaked, detached or concurrently-polled ingress future is selected.

## 10. Verifier-time invariant

The durable cancellation worker does not sample verifier time itself.

It transfers the caller-supplied verifier-time callback by value into the KO repeated loop.

The KO loop remains the only selected sampling owner and continues to sample exactly once immediately before each KM transaction invocation.

If cancellation wins while the KO loop is pending, no new verifier-time sample may be taken after cancellation readiness for a new transaction.

## 11. Stream ownership invariant

The future cancellation worker performs no direct:

- `accept_control_stream()`;
- `receive_post_auth_control_stream_ingress(...)`;
- family-specific frame read;
- stream replacement.

Stream accept/read ownership remains inside KM, reached only through the KO loop.

The worker introduces no second acceptor, queue, channel, speculative pre-accept or parallel transaction.

## 12. Capability-family chain

For capability traffic the selected composed path remains:

`durable cancellation worker`
→ `KO durable repeated loop`
→ `KM one-transaction durable wrapper`
→ one authenticated stream accept
→ one typed family-ingress read
→ `KK` typed-ingress processor
→ `KG` durable capability transaction helper
→ `ProductionDurableCapabilityAuthority`
→ authorized dispatch
→ exact same transaction response stream.

The cancellation worker itself performs no authorization, dispatch or response write.

No `SharedCurrentCapabilityAuthority<P>` fallback is selected inside the durable worker.

## 13. Requester/rendezvous chain

For requester/rendezvous traffic, KO returns one exact:

`RequesterRendezvousResponseStreamCustodyHandoff`

The durable worker returns that exact handoff as `Ok(Some(handoff))` immediately when the ingress future is ready.

The worker performs no requester DR, policy evaluation, registration, terminal acknowledgement response, provider execution, candidate selection or dialing.

Outer PRWM `request_id` remains correlation only.

## 14. Candidate-publication chain

Candidate publication remains fail-closed through the existing:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

The durable worker returns that exact ingress error unchanged when KO produces it.

No candidate authority/provider invocation, response write, retry or reinterpretation is selected.

## 15. Error preservation

The durable worker introduces no new error enum.

Its `Err(...)` arm must preserve the exact first:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError`

including existing distinctions for:

- `Accept(...)`;
- `Ingress(...)`;
- `ProductionDurableCapability(...)`;
- nested KG `Authority` / `Dispatch` / `Response` failures;
- `CandidatePublicationHandoffNotSelected`.

No flattening, suppression, retry or fabricated success is selected.

## 16. Existing legacy worker preservation

The future source checkpoint must leave unchanged:

- `process_one_post_auth_control_stream_ingress(...)`;
- `run_repeated_post_auth_control_stream_ingress(...)`;
- `run_repeated_post_auth_control_stream_ingress_worker(...)`.

The legacy worker continues using:

`SharedCurrentCapabilityAuthority<P>`

exactly as before.

No existing caller is redirected to the new durable worker in the same source checkpoint.

## 17. Exact source-successor ceiling

The later KP source materialization may change at most one source path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

No second source path is selected.

The following must remain byte-identical unless a new gate is opened:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`;
- FQ recoverable spawned requester-aware worker source;
- FU repeated real-admission requester-aware integration source;
- KG helper source;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests/lockfile;
- workflows;
- Android source.

If compilation requires a second path, public API expansion, FI/FQ/FU mutation, production aggregate mutation or runtime activation, STOP and open a separate gate.

## 18. Source-successor proof obligations

A later materialization must prove at minimum:

1. exactly one selected source path changed;
2. the KO loop remains byte-semantically unchanged;
3. legacy EV/EX worker signatures remain unchanged;
4. FI/FQ/FU remain byte-identical;
5. durable worker calls only the KO durable repeated loop;
6. ingress is polled before cancellation on every wake;
7. ready requester handoff outranks same-wake cancellation;
8. ready ingress error outranks same-wake cancellation;
9. cancellation returns only `Ok(None)`;
10. no peer close occurs on cancellation;
11. no direct accept/read occurs in the worker;
12. no new verifier-time sampling occurs in the worker;
13. cancellation drops the in-flight loop future before return;
14. no task/channel/queue is created;
15. no legacy shared-current fallback is used by the durable worker;
16. the new worker remains dormant and uninvoked.

## 19. Explicit exclusions

KP does not select or authorize:

- source materialization in KP itself;
- replacement of legacy EX worker;
- FI serial lifecycle migration;
- FI higher cancellation worker migration;
- FQ spawned requester-aware worker migration;
- FU real-admission requester-aware migration;
- dual-authority FI/FQ/FU parameters or aggregate;
- requester DR authority replacement;
- production aggregate replacement;
- executable durable-authority population;
- positive production capability grants;
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

## 20. Validation rule

KP selection may be closed only on exact-final-head validation authority.

Required evidence:

- exact branch head/tree re-read;
- exact KO -> KP topology;
- exactly one docs contract path;
- PR remains draft/open/unmerged;
- exact-final-head Rust validation terminal-success;
- any other exact-head workflow that triggers must be recorded accurately; `SKIPPED` is never represented as PASS;
- immutable Drive audit publication with exact-title presearch, frozen bytes/SHA-256, raw readback verification, exact-title postsearch uniqueness, and post-publication GitHub guards.

## 21. STOP boundary

STOP after KP selection closure.

Do not materialize the durable cancellation-aware worker in KP.

Do not modify FI/FQ/FU, introduce dual-authority propagation, populate production executable inputs, activate runtime/network behavior, merge, deploy, restart/recover or mutate repository configuration without a separately selected successor gate.
