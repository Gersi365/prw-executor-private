# C03e-QU Production Durable Post-Auth Fallible Verifier-Time Requester/Rendezvous Serial Lifecycle Selection

Date: 2026-09-12
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`
Checkpoint: `C03e-QU`
Boundary: `PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SERIAL_LIFECYCLE_SELECTION`
Status: `SELECTION — SOURCE MATERIALIZATION BLOCKED`

## 1. Purpose

C03e-QU selects only the next dormant source boundary after evidence-closed C03e-QT: a fallible-verifier-time production-durable requester/rendezvous serial lifecycle sibling that preserves QT ingress/cancellation semantics and the existing KS requester DR/terminal-response lifecycle semantics.

QU is a documentation-only selection checkpoint. It does not materialize Rust source, migrate a caller, activate runtime behavior, alter authentication/authorization semantics, construct scheduling requests, merge, deploy, or clean historical evidence.

## 2. Exact predecessor

Evidence-closed C03e-QT branch:

`phase-152-c03e-qt-production-durable-post-auth-fallible-verifier-time-cancellation-aware-worker-source-materialization`

QT exact final head:

`ec56da73dd7855f3597188a165c145e1e483d6b3`

QT exact final tree:

`94e3f0115d4e9348d0e86448536f9b7023783804`

QT final requester ingress source blob:

`8b28d22d80ec59e707b2e6b6a44a87fe207231af`

QT PR:

`#584`

QT canonical audit:

`C03E_QT_PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_CANCELLATION_AWARE_WORKER_SOURCE_MATERIALIZATION_AUDIT_2026-09-12.md`

QT canonical Drive ID:

`14wuSCiAdnhv0qwtBDZcgI59FxHv7lJ_h`

QT canonical parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

QT exact frozen/readback bytes:

`13334`

QT SHA-256:

`5a08d8a4f4ef83bebe0ce5be5ec154ac3759bbebbb08aa5459a6fc4305d8411a`

Fresh startup verification before QU creation proved QT branch/head remained exact, PR #584 remained draft/open/unmerged/mergeable, exact-head Rust and Android validation remained successful, the canonical QT audit remained an exact-title singleton, and integrated `main` remained unchanged.

## 3. Stable integrated main

Fresh pre-selection read proved `main` remains:

- head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

QU does not modify `main`.

## 4. Exact QT ingress worker prerequisite

QT materialized exactly one dormant sibling on `AuthenticatedRemoteSessionRuntimeOwner`:

`run_fallible_verifier_time_repeated_post_auth_control_stream_ingress_worker_with_production_durable_capability<D, T, C>(...)`

Its verifier-time source law is:

```rust
T: FnMut() -> Result<
    u64,
    prw_session::prwa_verifier_source::PrwaVerifierSourceError,
> + Send
```

Its cancellation law is:

```rust
C: Future<Output = ()> + Send
```

Its result law is:

```rust
Result<
    Option<RequesterRendezvousResponseStreamCustodyHandoff>,
    AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError,
>
```

QT polls its exact QR fallible ingress future first on every wake, propagates exact typed `VerifierTime` / `Ingress` failures unchanged, polls cancellation only while ingress remains pending, maps cancellation to `Ok(None)`, and preserves lexical drop-before-return of the in-flight QR future.

QU does not modify that worker or its child-module source.

## 5. Existing production-durable requester lifecycle prerequisite

Exact QT source inspection shows the existing KS requester/rendezvous production-durable serial lifecycle worker remains:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

It currently accepts:

```rust
T: FnMut() -> u64 + Send
```

and delegates pre-handoff ingress/cancellation to the existing infallible production-durable ingress worker.

It then preserves the existing lifecycle law:

1. retain one pinned caller cancellation future across serial cycles;
2. before requester handoff, delegate the ingress/cancellation race to the durable ingress worker;
3. `Ok(None)` means caller cancellation;
4. ingress failure becomes the historical requester lifecycle `Failed(...)` result;
5. after requester handoff, do not poll cancellation during requester DR continuation or terminal acknowledgement response composition;
6. requester DR continuation uses only the existing requester DR authority lane;
7. terminal response failure stops the worker with exact requester-response provenance;
8. after terminal response success, poll cancellation exactly once before another ingress cycle;
9. if cancellation is not ready, begin the next strictly serial ingress cycle.

QU preserves this historical infallible worker byte-for-byte in the selected successor.

## 6. Existing scheduling-aware worker remains a later gate

The current scheduling-aware production-durable sibling:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`

also still accepts infallible verifier time:

```rust
T: FnMut() -> u64 + Send
```

and owns scheduling-result custody after successful requester DR registration.

QU does not select a fallible scheduling-aware sibling, does not alter `RequesterRendezvousProductionDurableSchedulingWorkerStop`, and does not alter scheduling derivation, expected-device request construction, sender/channel custody, admission `SessionId`, PRWM request ID, timing, or higher-owner scheduling propagation.

## 7. Typed-error visibility prerequisite

The QT typed ingress error is declared in the private child module:

`authenticated_remote_session_runtime::requester_rendezvous_one_shot_transaction`

as:

`AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError`

The child module itself is declared privately by:

```rust
mod requester_rendezvous_one_shot_transaction;
```

and the authenticated-runtime parent currently provides no parent-level re-export of the QT error.

The requester lifecycle source is a sibling module outside that private child. A future lifecycle sibling must preserve the exact QT error by value; it must not erase, stringify, box, default, flatten away, or reconstruct verifier-time/ingress provenance.

Therefore the minimum source-materialization boundary requires one narrow visibility seam in the authenticated-runtime parent in addition to the requester lifecycle source change.

## 8. Selected future boundary

QU selects exactly this future boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SERIAL_LIFECYCLE_SOURCE_MATERIALIZATION`

Expected successor checkpoint after a fresh authority check:

`C03e-QV`

Expected successor branch:

`phase-152-c03e-qv-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-serial-lifecycle-source-materialization`

QU itself does not create QV.

## 9. Exact two-path source ceiling

Future QV source materialization may change at most these two Rust paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
2. `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

No third source path is selected.

Exact QT blobs used as source guards:

- authenticated-runtime parent blob: `20d93c729ce50f1905fbe94fb9a75004b2c6c204`;
- requester lifecycle/FI source blob: `d1a68ea88d6721a622e0fd8ec54ef23ab136726f`;
- QT ingress child source blob: `8b28d22d80ec59e707b2e6b6a44a87fe207231af`.

The QT ingress child source is a guard only and must remain unchanged in QV.

## 10. Selected parent visibility seam

In `authenticated_remote_session_runtime.rs`, QV may add only a narrow parent-visible re-export of the existing QT error, semantically equivalent to:

```rust
pub(super) use requester_rendezvous_one_shot_transaction::
    AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError;
```

Formatting may follow rustfmt.

This is visibility adaptation only. It must not:

- move or duplicate the error declaration;
- add variants;
- change `Display`, `Error`, or `From` behavior;
- widen the child module itself;
- widen the error outside the `remote_session_capability_runtime` parent boundary;
- alter QT worker behavior;
- alter any authenticated-session runtime method.

## 11. Selected fallible lifecycle error surface

In `requester_rendezvous_retained_custody_dr_continuation.rs`, QV may add one sibling error family:

`RequesterRendezvousFallibleVerifierTimePostTerminalResponseSerialLifecycleError`

with exactly two semantic variants:

```rust
Ingress(
    AuthenticatedRemoteSessionFallibleVerifierTimeProductionDurablePostAuthIngressError,
)
```

and:

```rust
RequesterResponse(
    RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError,
)
```

The `Ingress(...)` channel retains the exact QT nested distinction between:

- `VerifierTime(PrwaVerifierSourceError)`;
- `Ingress(AuthenticatedRemoteSessionPostAuthIngressTransactionError)`.

The `RequesterResponse(...)` channel retains the existing exact FH distinction between:

- `Frame(RequesterRendezvousDrAcknowledgementWireError)`;
- `ResponseIo(RequesterRendezvousDrAcknowledgementResponseIoError)`.

No verifier-time error is translated into requester response failure. No requester response failure is translated into ingress failure. No new DR-semantic error category is selected.

The sibling error may implement the same bounded `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Display`, `std::error::Error`, and exact `From` conversion style already used by adjacent lifecycle errors where supported by the contained types.

Historical `RequesterRendezvousPostTerminalResponseSerialLifecycleError` remains unchanged.

## 12. Selected fallible lifecycle stop surface

QV may add one sibling stop family:

`RequesterRendezvousFallibleVerifierTimePostTerminalResponseSerialLifecycleWorkerStop`

with exactly:

```rust
Cancelled
```

and:

```rust
Failed(RequesterRendezvousFallibleVerifierTimePostTerminalResponseSerialLifecycleError)
```

No scheduling-terminal variant is selected here.

Historical `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop` remains unchanged.

Historical `RequesterRendezvousProductionDurableSchedulingWorkerStop` remains unchanged.

## 13. Selected future lifecycle sibling

QV may add exactly one dormant sibling:

`run_fallible_verifier_time_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

Its generic verifier-time bound must be exactly:

```rust
T: FnMut() -> Result<
    u64,
    prw_session::prwa_verifier_source::PrwaVerifierSourceError,
> + Send
```

Its cancellation bound remains:

```rust
C: Future<Output = ()> + Send
```

It retains the same explicit authority lanes as KS:

1. `ProductionDurableCapabilityAuthority` only for pre-handoff durable capability ingress;
2. `SharedCurrentCapabilityAuthority<P>` only for requester/rendezvous DR continuation;
3. `SharedRequesterRendezvousAuthority` for requester/rendezvous registration authority;
4. the existing requester-aware policy source remains the requester policy source;
5. dispatcher remains caller-owned mutable custody.

No authority aggregate or Clone adaptation is selected.

## 14. Selected serial-cycle law

The fallible lifecycle sibling must retain one pinned caller cancellation future across serial cycles.

For each pre-handoff cycle it must:

1. create only a temporary polling adapter over that exact retained cancellation future;
2. call the exact QT fallible cancellation-aware production-durable ingress worker;
3. pass the fallible verifier-time source by mutable reborrow so one caller-owned source survives across cycles;
4. let QT own verifier-time sampling and the ingress/cancellation race;
5. perform no direct verifier-time sample itself;
6. perform no direct stream accept/read itself.

QT result handling is selected exactly as:

- `Ok(Some(handoff))` -> transfer the exact handoff into the existing requester DR continuation;
- `Ok(None)` -> return `Cancelled`;
- `Err(exact_qt_error)` -> return `Failed(Ingress(exact_qt_error))`.

No retry, fallback, second verifier-time sample, default time, cache, panic conversion, error stringification, or peer close is selected.

## 15. Requester DR and response custody law

After `Ok(Some(handoff))`, the future sibling must preserve the existing KS lifecycle exactly:

1. cancellation is not polled while requester DR continuation is in progress;
2. call `continue_requester_rendezvous_retained_custody_through_dr(...)` exactly once with the existing requester DR authority/policy/rendezvous authority lanes;
3. run `complete_requester_rendezvous_terminal_dr_acknowledgement_response(...)` exactly once;
4. exact requester-response failure returns `Failed(RequesterResponse(exact error))`;
5. no next ingress cycle begins after a requester-response failure;
6. successful generic rejected acknowledgement remains transaction-complete success exactly as historically;
7. successful response does not itself imply scheduling or rendezvous transport success.

No cancellation-abandonment path is introduced after requester handoff.

## 16. Post-response cancellation law

After exact terminal acknowledgement response success:

1. poll the retained cancellation future exactly once before another ingress cycle;
2. if ready, return `Cancelled`;
3. if not ready, begin the next serial QT ingress cycle;
4. no verifier-time sample, accept, or receive for the next cycle may occur before this post-response cancellation check.

This preserves the existing KS/FK requester-aware cancellation discipline while replacing only the pre-handoff ingress worker with the QT fallible counterpart.

## 17. Explicitly unchanged behavior

QV must leave unchanged:

- QT fallible ingress error definition;
- QT fallible ingress worker source and semantics;
- QR fallible repeated-ingress loop;
- KQ/KO infallible worker/loop;
- existing FI/FJ/FL requester lifecycle types/functions;
- existing KS production-durable requester worker;
- existing NW scheduling-aware requester worker;
- DR continuation;
- acknowledgement framing and I/O;
- requester registration semantics;
- scheduling derivation semantics;
- candidate publication behavior;
- peer-close policy.

## 18. Focused test allowance

Within the requester lifecycle source path only, QV may add focused tests required to prove the selected typed surface, including:

- exact conversion from the parent-re-exported QT error into the new lifecycle `Ingress(...)` channel;
- exact conversion from existing requester-response error into `RequesterResponse(...)`;
- stop type retains exact failure by value;
- no historical type or test expectation is changed except formatter/import adjustments mechanically required by the additive sibling.

No integration/runtime activation test is selected.

## 19. Explicit non-actions

QU/QV do not select or authorize:

- fallible scheduling-aware requester sibling;
- scheduling-result custody migration;
- persistent recoverable scheduling-worker propagation;
- cooperative scheduling producer propagation;
- executor/endpoint wrapper propagation;
- concrete QN specialization;
- QJ dispatcher capture;
- QL channel construction/split;
- receiver-to-QF wiring;
- receipt observation policy;
- expected-device request construction/send;
- admission `SessionId` construction;
- PRWM request-ID allocation;
- new timing acquisition outside the existing verifier-time source;
- process/executable caller migration;
- listener/readiness/network activation;
- task spawn or join ownership;
- new channel/queue;
- peer-close policy expansion;
- retry/reconnect;
- fallback/default verifier time;
- authentication/trust/RBAC mutation;
- database/schema/control-plane mutation;
- Cargo/lockfile/workflow mutation;
- Android source mutation;
- packaging/service/systemd mutation;
- repository visibility/configuration mutation;
- merge;
- deployment;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force update/history rewrite;
- destructive cleanup of duplicate or historical Drive evidence.

## 20. QU validation law

QU is docs-only. Validation authority must bind only to the exact final QU head.

Required closure evidence:

- direct QT -> QU topology with exact QT merge base;
- exactly one added contract path;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging/deployment/repository-config changes;
- PRW Rust Validation success on exact final QU head;
- any path-filtered workflow reported as `SKIPPED` remains `SKIPPED`, not PASS;
- no Android PASS may be inherited when no exact-head Android workflow runs.

## 21. Immutable evidence target

Canonical QU audit filename:

`C03E_QU_PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SERIAL_LIFECYCLE_SELECTION_AUDIT_2026-09-12.md`

Canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

Before QU closure the exact frozen audit must be uploaded once, raw-read back, byte-counted, re-hashed, and exact-title post-search must return exactly one canonical QU artifact.

Historical QS duplicate evidence remains untouched and does not authorize destructive cleanup.

## 22. Closure boundary

If exact-final-head CI and immutable evidence succeed, QU may close as:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Selected future boundary remains:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SERIAL_LIFECYCLE_SOURCE_MATERIALIZATION`

STOP after QU closure.

Do not materialize QV source inside the QU checkpoint.
Do not select the fallible scheduling-aware requester sibling inside QU.
Do not merge, deploy, activate runtime behavior, or destructively clean evidence.
