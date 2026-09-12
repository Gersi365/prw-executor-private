# Desktop Functional Management Slice C03e-QS

## Production Durable Post-Auth Fallible Verifier-Time Cancellation-Aware Worker Selection — Staging Contract

Date: 2026-09-12

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_CANCELLATION_AWARE_WORKER_SELECTION`

Selected future boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_CANCELLATION_AWARE_WORKER_SOURCE_MATERIALIZATION`

## 1. Purpose

C03e-QS selects exactly one next prerequisite after evidence-closed C03e-QR: a dormant fallible-verifier-time sibling of the existing C03e-KQ cancellation-aware production-durable post-authenticated ingress worker.

QS is selection only. It materializes no Rust source, invokes no runtime path, and changes no existing authority/custody relationship.

The selection exists because QR removed the first infallible verifier-time boundary by adding a fallible repeated-ingress loop, while the immediately higher cancellation worker still accepts only `T: FnMut() -> u64 + Send` and internally owns the infallible repeated-ingress future.

## 2. Exact predecessor

Evidence-closed C03e-QR branch:

`phase-152-c03e-qr-production-durable-post-auth-fallible-verifier-time-repeated-ingress-source-materialization`

Exact QR head:

`561bf959463da84667abf62a1042287415dc3366`

Exact QR tree:

`666b9557e78d731ca25f7dbfad74ba84051953ff`

Exact QR source blob:

`82639cb68881b5d569552339a65fcd737b0a2173`

QR PR:

`#582`

QR immutable audit Drive ID:

`1TN_ZAnpDxTBsaeiDu3v8uGCBTK710hVQ`

QR immutable audit exact bytes:

`21006`

QR immutable audit SHA-256:

`b62b17811bdcd2e371bf2a6a7483d11df997b41cd849fc4f01d2bb2b9bd50d7a`

Fresh closure audit before QS branch creation proved QR remained draft/open/unmerged/mergeable, exact-head CI-valid, evidence-singleton, and source/topology exact.

## 3. Why selection advances above QR

QR materialized:

`run_fallible_verifier_time_repeated_post_auth_control_stream_ingress_with_production_durable_capability(...)`

with:

```rust
T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send
```

and return type:

```rust
Result<
    RequesterRendezvousResponseStreamCustodyHandoff,
    AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError,
>
```

The exact next existing owner seam in the same source file is the unchanged C03e-KQ worker:

`run_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability(...)`

which still accepts:

```rust
T: FnMut() -> u64 + Send
```

and owns exactly one infallible C03e-KO repeated-ingress future plus one cancellation future.

Therefore C03e-KQ is the first remaining verifier-time infallibility barrier above QR. Higher requester/rendezvous lifecycle, scheduling, persistent-worker and cooperative-producer seams cannot consume QR's fallible source law until this cancellation boundary is crossed without erasing error provenance.

## 4. Hard future source ceiling

A future source-materialization checkpoint selected by QS may modify exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

No other source path is selected.

In particular the future source checkpoint must not modify:

- `authenticated_remote_session_runtime.rs`;
- requester/rendezvous scheduling lifecycle source;
- persistent recoverable worker source;
- cooperative producer/executor source;
- endpoint lifecycle source;
- `linux_bootstrap.rs`;
- higher-owner custody source;
- Cargo manifests;
- lockfile;
- workflows;
- Android source;
- packaging;
- deployment;
- repository configuration.

## 5. Selected future symbol

The future source checkpoint may add one dormant sibling with the semantic identity:

`run_fallible_verifier_time_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability<D, T, C>(...)`

The exact final spelling may follow rustfmt and local source conventions, but it must remain a sibling rather than a replacement of C03e-KQ.

## 6. Selected verifier-time source bound

The future worker must accept the same fallible source law already materialized by QR:

```rust
T: FnMut() -> Result<
    u64,
    prw_session::prwa_verifier_source::PrwaVerifierSourceError,
> + Send
```

The worker must not adapt this source back into `FnMut() -> u64` through defaulting, panic, caching, retry, or error suppression.

## 7. Selected cancellation bound

The future worker must preserve the existing executor-neutral cancellation shape:

```rust
C: Future<Output = ()> + Send
```

QS selects no cancellation token type, task handle, channel, watch receiver, timeout primitive, peer-close authority, or executor-specific cancellation API.

## 8. Selected return type

The future worker must return:

```rust
Result<
    Option<RequesterRendezvousResponseStreamCustodyHandoff>,
    AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError,
>
```

The existing QR typed error is reused exactly.

No new error enum is selected for this boundary.

No cancellation error variant is selected: cancellation remains `Ok(None)` exactly as in existing KQ.

## 9. Selected owned race inputs

The future worker must own exactly:

1. one QR fallible repeated-ingress future; and
2. one caller-supplied cancellation future.

It must not create or own:

- a second ingress future;
- an infallible KO future;
- a second cancellation future;
- a timeout future;
- a peer-close future;
- a background task;
- a queue/channel;
- a retry controller.

## 10. Selected QR delegation law

The future worker must delegate ingress progression only to the existing QR sibling:

`run_fallible_verifier_time_repeated_post_auth_control_stream_ingress_with_production_durable_capability(...)`

It must not duplicate QR's loop, sample verifier time directly, call C03e-KM directly, accept a control stream directly, or decode family ingress directly.

This preserves one acquisition site for each verifier-time sample and one bounded transaction site inside KM.

## 11. Selected poll order

The future worker must preserve existing C03e-KQ race priority:

1. poll the QR ingress future first on every wake;
2. only while QR ingress is pending, poll cancellation.

This order is semantic, not stylistic.

An already-ready requester handoff or already-ready verifier/ingress failure must win over cancellation when both are observable in the same wake.

QS does not select fair/random polling or cancellation-first bias.

## 12. Selected requester-handoff mapping

If the QR ingress future is ready with:

```rust
Ok(handoff)
```

the future worker must return:

```rust
Ok(Some(handoff))
```

The exact `RequesterRendezvousResponseStreamCustodyHandoff` moves by value unchanged.

No requester DR work, acknowledgement I/O, scheduling derivation, target dialing, response fabrication, or additional ingress iteration is selected here.

## 13. Selected verifier-time failure mapping

If the QR ingress future is ready with:

```rust
Err(
    AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError::VerifierTime(error)
)
```

the future worker returns the exact same typed error unchanged.

It must not:

- retry verifier-time acquisition;
- poll cancellation first and hide the ready error;
- convert it to `Ok(None)`;
- convert it to an ingress error;
- default a timestamp;
- panic;
- stringify/box/flatten it.

## 14. Selected ingress failure mapping

If the QR ingress future is ready with:

```rust
Err(
    AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError::Ingress(error)
)
```

the future worker returns the exact same typed wrapper and nested ingress provenance unchanged.

All existing nested KM classifications remain preserved.

## 15. Selected cancellation mapping

Only if the QR ingress future is pending may the future worker poll cancellation.

If cancellation is ready, the future worker returns exactly:

```rust
Ok(None)
```

Cancellation must not manufacture a typed verifier-time or ingress error.

Cancellation must not close the authenticated peer, send a response, retry, reconnect, or perform requester/rendezvous work.

## 16. Selected pending mapping

If both:

- QR ingress future is pending; and
- cancellation is pending;

then the combined worker remains pending.

No spin, yield loop, sleep, timeout or busy polling is selected.

## 17. Selected lexical drop-before-return law

The future worker must preserve existing KQ's scoped-race lifetime discipline.

The in-flight QR future owns an exclusive mutable borrow of the authenticated session owner. On cancellation, that in-flight QR future must be dropped before the cancellation result leaves the method.

Therefore the race should remain inside a lexical scope equivalent in effect to existing KQ, so the exclusive `&mut self` borrow is released before method return.

QS does not select leaking, forgetting, detaching or background-retaining the ingress future.

## 18. No direct verifier-time acquisition in worker

The future worker must not invoke `verifier_time_unix_seconds()` itself.

Verifier-time acquisition remains exclusively inside the QR repeated-ingress sibling, exactly once immediately before each prospective KM invocation.

This prevents:

- duplicate sampling;
- sample-before-race semantics;
- cached samples surviving cancellation;
- worker-specific time fallback;
- drift from QR's fail-before-stream-accept law.

## 19. No direct stream ownership in worker

The future worker must not directly call:

- `accept_control_stream()`;
- `receive_post_auth_control_stream_ingress(...)`;
- C03e-KM;
- the existing typed-ingress processor.

Those operations remain nested under QR -> KM.

## 20. Existing C03e-KQ remains unchanged

QS selects a sibling only.

The existing infallible worker:

`run_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability(...)`

must remain source-identical in the future source checkpoint except for incidental surrounding rustfmt only if unavoidable; the intended source materialization is additive and must not migrate existing callers.

No historical infallible behavior is removed in this gate.

## 21. Existing C03e-KO remains unchanged

The infallible repeated-ingress loop remains unchanged and continues to accept `T: FnMut() -> u64 + Send`.

The future fallible worker must not call KO.

It calls QR instead.

## 22. QR typed error remains authoritative

The future worker reuses:

`AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError`

with exactly its current semantic categories:

- `VerifierTime(...)`;
- `Ingress(...)`.

QS selects no third variant and no change to `Display`, `Error::source`, or `From` implementations.

## 23. No peer-close semantics

Existing C03e-KQ itself does not perform peer close as part of the selected cancellation race.

The fallible sibling likewise must not add:

- close code emission;
- close reason emission;
- transport shutdown;
- connection teardown;
- readiness mutation.

Any higher peer-close policy remains separately gated.

## 24. No task semantics

The future worker is an async method but QS selects no spawn.

It must not call or create equivalents of:

- `tokio::spawn`;
- `spawn_local`;
- detached task ownership;
- join-handle custody;
- task cancellation token ownership.

Higher worker/task custody remains outside QS.

## 25. No retry/reconnect semantics

The future worker must perform no:

- verifier-time retry;
- ingress retry outside QR's normal next-capability iteration;
- reconnect;
- alternate authenticated session acquisition;
- request replay;
- stream replay;
- error suppression.

## 26. No requester/rendezvous lifecycle propagation yet

QS does not select or materialize the next higher requester/rendezvous serial lifecycle counterpart.

The future worker returns `Option<RequesterRendezvousResponseStreamCustodyHandoff>` only.

It does not consume that handoff into requester DR, acknowledgement, scheduling, terminal response, or target admission.

## 27. No scheduling-aware requester propagation yet

QS does not modify any seam that owns production-durable scheduling grant derivation or scheduling-aware requester completion.

No `RequesterRendezvousProductionDurableSchedulingWorkerStop` behavior changes in QS.

## 28. No persistent recoverable worker propagation yet

QS does not modify persistent authenticated-session worker custody or active-map ownership.

No recoverable worker is spawned or specialized by this selection.

## 29. No cooperative producer propagation yet

QS does not modify or select invocation of:

- `finish_cooperative_scheduling_admission(...)`;
- cooperative scheduling producer drivers;
- executor endpoint producer wrapper;
- endpoint-owner producer wrapper.

Those remain blocked on higher fallible propagation.

## 30. No return to QN specialization yet

Although QN already consumes a fallible verifier-time request-construction source, QS does not select concrete generic producer specialization.

Returning to QN before the intervening infallible boundaries are replaced would either lose verifier-time errors or exceed the selected scope.

## 31. QJ/QK/QL/QN custody laws remain unchanged

QS does not:

- capture the QJ production dispatcher source;
- mint another status snapshot;
- construct or split QL capacity-one channel custody;
- clone sender authority;
- create a second/unbounded channel;
- move the receiver into QF;
- invoke QN's dormant producer helper;
- observe the returned QN receipt.

## 32. Identity and authority laws remain unchanged

QS changes no identity law:

- requester `DeviceId` remains correlation only;
- target `DeviceId` remains sourced only from scheduling grant where that later seam is active;
- target admission `SessionId` remains QH-owned;
- auth request ID remains QH-owned;
- requester scheduling `SessionId` is not target admission `SessionId`;
- fallible verifier-time provider remains the exact PRWA source family.

QS creates no new authority.

## 33. No runtime activation

QS selects dormant source only for a later checkpoint.

No process caller, executable caller, listener, readiness path, runtime loop, network path or deployment becomes active.

## 34. Selected future source-validation requirement

A future source-materialization checkpoint must prove on its exact final head:

1. final diff is confined to the selected one Rust path;
2. only the fallible KQ sibling and strictly necessary local source support are added;
3. existing KQ/KO/QR laws are unchanged;
4. formatting succeeds;
5. Clippy succeeds;
6. workspace tests succeed;
7. workspace build succeeds;
8. any Android workflow that registers for that exact head is reported exactly as run;
9. no CI result is inherited from a superseded head.

## 35. Selected forward-only correction law

If future validation exposes formatter, lint or compilation defects, corrections must be forward-only commits.

No reset, rebase, squash, force update, branch rewind or history rewrite is selected.

Superseded failures must remain transparent in immutable evidence.

## 36. Selection stage order after QS

The prerequisite sequence remains:

1. QR fallible repeated-ingress loop — complete;
2. fallible cancellation-aware worker counterpart — selected by QS;
3. fallible requester/rendezvous serial lifecycle counterpart — deferred;
4. fallible scheduling-aware requester counterpart — deferred;
5. fallible persistent scheduling worker counterpart — deferred;
6. fallible cooperative producer/executor/endpoint counterparts — deferred;
7. concrete QN producer specialization — deferred until those infallible boundaries are crossed.

QS does not collapse these gates.

## 37. Explicitly deferred scope

Still separately gated after QS:

1. source materialization of the QS-selected fallible cancellation-aware worker;
2. fallible requester/rendezvous serial lifecycle;
3. fallible scheduling-aware requester lifecycle;
4. fallible persistent recoverable scheduling worker;
5. fallible cooperative scheduling producer driver;
6. fallible executor endpoint producer wrapper;
7. fallible endpoint-owner producer wrapper;
8. renewed concrete QN specialization;
9. QJ dispatcher-source capture;
10. QL channel construction/split;
11. higher-owner composition;
12. receiver-to-QF wiring;
13. receipt observation policy;
14. process/runtime caller migration;
15. executable caller;
16. listener/readiness/runtime/network activation;
17. merge/deployment.

## 38. Explicit non-actions in QS

C03e-QS performs no:

- Rust source mutation;
- source materialization;
- cancellation worker invocation;
- requester lifecycle mutation;
- scheduling mutation;
- persistent worker mutation;
- cooperative producer mutation;
- endpoint mutation;
- QN invocation;
- QJ capture;
- QL channel operation;
- receiver transfer;
- receipt observation;
- runtime activation;
- Cargo/lockfile/workflow/Android mutation;
- authentication/trust/RBAC/database/schema/control-plane policy mutation;
- repository configuration mutation;
- merge;
- deployment;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update/history rewrite.

## 39. Closure condition for C03e-QS

QS may close only if exact-head validation proves:

- branch starts from exact QR head `561bf959463da84667abf62a1042287415dc3366`;
- direct QR -> QS topology is ahead one / behind zero;
- merge base is exact QR head;
- exactly one contract path is added;
- zero Rust/source/runtime/workflow/Cargo/lockfile/Android/packaging/deployment/repository-config changes;
- exact-final-head CI is terminal and reported without inherited claims;
- integrated `main` is unchanged;
- immutable audit publication succeeds with collision-free canonical title and byte-identical readback;
- successor namespace remains uncreated at closure.

## 40. STOP law

After C03e-QS is evidence-closed, STOP.

Do not materialize the selected fallible cancellation-aware worker inside QS.

Do not create the next source checkpoint inside QS.
