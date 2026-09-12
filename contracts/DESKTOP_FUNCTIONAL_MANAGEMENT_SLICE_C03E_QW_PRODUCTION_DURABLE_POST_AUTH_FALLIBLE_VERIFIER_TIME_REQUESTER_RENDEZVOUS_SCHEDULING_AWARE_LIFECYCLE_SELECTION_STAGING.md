# C03e-QW — Production-Durable Post-Auth Fallible Verifier-Time Requester/Rendezvous Scheduling-Aware Lifecycle Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_LIFECYCLE_SELECTION`

Selected future boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_LIFECYCLE_SOURCE_MATERIALIZATION`

## 1. Exact predecessor

This selection branches only from evidence-closed C03e-QV:

- branch: `phase-152-c03e-qv-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-serial-lifecycle-source-materialization`
- exact head: `17aec7939db3f6138c5eacb3acb86731205673e5`
- exact tree: `08fc19e665c8cfdefabebf39779f03b1ed862a52`
- PR: #586, preserved draft/open/unmerged
- canonical QV audit Drive ID: `100dOiSDUqIbUcHQiyXMn5V0zUVLpr816`
- canonical QV audit bytes: `22537`
- canonical QV audit SHA-256: `e4f8dc0e5ae7bd21b9a073e29fae817cb75c9c3a34bdedb185eff25bb289ec18`

QV materially provides the fallible requester/rendezvous serial lifecycle but explicitly leaves scheduling-aware propagation as a separate future gate.

## 2. Fresh source finding

At exact QV head, the existing scheduling-aware requester sibling remains:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`

in:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

That existing scheduling-aware sibling still requires:

`T: FnMut() -> u64 + Send`

and calls the historical infallible production-durable cancellation-aware ingress worker.

The same exact file now also contains the QV fallible requester lifecycle surfaces:

- `RequesterRendezvousFallibleVerifierTimePostTerminalResponseSerialLifecycleError`
- `RequesterRendezvousFallibleVerifierTimePostTerminalResponseSerialLifecycleWorkerStop`
- `run_fallible_verifier_time_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

The existing scheduling-terminal carrier remains independently suitable for reuse:

`RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome`

It already owns exactly two orthogonal terminal channels:

1. `scheduling_result: Result<ExpectedDeviceSchedulingAuthorityGrant, ExpectedDeviceSchedulingAuthorityDerivationError>`
2. `acknowledgement_result: Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`

No verifier-time type appears in that carrier, so no new scheduling-terminal carrier is selected.

## 3. Selected immediate source ceiling

Future C03e-QX may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact QV source guard for that path:

`cdda84902c529ad1fe9c6f4b0ec4028318cc2767`

No parent-module change is selected because QV already materialized the narrow parent re-export needed to name the exact QT typed ingress error from this sibling module.

The following QV guards must remain byte-stable in QX:

- parent authenticated runtime: `08deec2f12095738c9e71fdb133914733e68263e`
- QT child ingress implementation: `8b28d22d80ec59e707b2e6b6a44a87fe207231af`

If correctness or compilation requires a second Rust path, parent visibility widening, QT child mutation, higher-owner propagation, persistent-worker mutation, producer/executor/endpoint mutation, or Cargo/workflow widening: STOP and return to selection.

## 4. Selected future stop type

QX may add one production-specific fallible scheduling-aware stop type:

`RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop`

with exactly these semantic variants:

- `Cancelled`
- `Failed(RequesterRendezvousFallibleVerifierTimePostTerminalResponseSerialLifecycleError)`
- `SchedulingTerminal(RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome)`

The existing historical `RequesterRendezvousProductionDurableSchedulingWorkerStop` remains unchanged.

The new `Failed` channel must preserve the exact QV nested typed provenance:

- ingress channel -> exact QT `VerifierTime(PrwaVerifierSourceError)` or exact QT `Ingress(AuthenticatedRemoteSessionPostAuthIngressTransactionError)`
- requester-response channel -> exact existing `Frame(RequesterRendezvousDrAcknowledgementWireError)` or `ResponseIo(RequesterRendezvousDrAcknowledgementResponseIoError)`

No scheduling derivation error is folded into `Failed`; scheduling derivation remains inside the existing orthogonal scheduling terminal carrier.

## 5. Selected future worker

QX may add exactly one dormant sibling:

`run_fallible_verifier_time_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`

Selected generic verifier-time bound:

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send`

All other authority lanes remain explicit and distinct:

- production durable capability authority only for pre-handoff ingress;
- shared-current authority only for requester DR and scheduling derivation freshness;
- shared requester/rendezvous authority for requester registration/scheduling derivation;
- requester-aware policy source remains the requester policy source;
- dispatcher custody remains caller-owned and mutable;
- one caller-owned cancellation future remains pinned across serial cycles.

No aggregate authority object is selected.

## 6. Selected fallible scheduling-aware lifecycle law

The future sibling must preserve the existing NW scheduling-aware law while replacing only the infallible pre-handoff verifier-time lane with the exact QV/QT fallible lane.

Required behavior:

1. Pin the caller cancellation future once and retain it across serial cycles.
2. For each pre-handoff cycle, create only one temporary polling adapter over the retained cancellation future.
3. Call the exact QT fallible production-durable cancellation-aware ingress worker with a mutable reborrow of the caller-owned fallible verifier-time source.
4. Perform no direct verifier-time sample in the scheduling-aware sibling.
5. Perform no direct stream accept/read in the scheduling-aware sibling.
6. QT `Ok(Some(handoff))` transfers the exact requester handoff by value.
7. QT `Ok(None)` returns `Cancelled`.
8. Exact QT error returns `Failed(Ingress(exact error))`; no flattening, stringification, reconstruction, retry, default, cache or fabricated success.
9. Before consuming the handoff, retain only the same requester `SessionId` and target `DeviceId` operation selectors already used by the historical scheduling-aware sibling.
10. Run `continue_requester_rendezvous_retained_custody_through_dr(...)` exactly once for the handoff.
11. If DR result is `Err`, do not invoke scheduling derivation.
12. On DR failure, complete the existing rejected requester acknowledgement exactly once.
13. Exact acknowledgement composition failure on that DR-failure path returns `Failed(RequesterResponse(exact error))`.
14. After successful rejected acknowledgement, poll retained cancellation exactly once; if ready return `Cancelled`, otherwise begin the next QT ingress cycle.
15. If DR result is `Ok`, invoke existing `derive_expected_device_scheduling_authority(...)` exactly once before requester acknowledgement framing/I/O.
16. Retain the exact resulting grant or typed derivation error by value without clone/copy/reconstruction/remint/replay.
17. Complete requester acknowledgement exactly once and retain its exact success/failure independently from scheduling derivation disposition.
18. Return `SchedulingTerminal(RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome)` immediately after those two terminal channels exist.
19. Do not poll cancellation after successful DR once scheduling derivation has been attempted and terminal scheduling/acknowledgement custody exists.
20. Do not start another ingress cycle after terminal scheduling custody exists.

This preserves the existing scheduling-aware semantic distinction:

- pre-scheduling cancellation or lifecycle failure remains worker stop;
- scheduling derivation success/failure is terminal custody, not lifecycle failure;
- acknowledgement success/failure is orthogonal terminal custody after successful DR.

## 7. Explicitly preserved historical surfaces

QX must leave unchanged:

- `RequesterRendezvousPostTerminalResponseSerialLifecycleError`
- `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`
- `RequesterRendezvousProductionDurableSchedulingWorkerStop`
- `RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome`
- `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`
- QV `run_fallible_verifier_time_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`
- historical `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`
- exact QT fallible ingress worker and its error family.

Focused same-file tests may be added only if needed to prove exact stop/custody mapping. They may not introduce runtime/network activation.

## 8. Explicit later gates

QW/QX do not select or materialize:

- persistent recoverable scheduling-worker propagation of the new fallible scheduling stop;
- repeated real-admission collection propagation;
- cooperative scheduling producer propagation;
- executor endpoint-lifecycle propagation;
- higher endpoint-owner propagation;
- concrete QN producer specialization/reintegration;
- QJ dispatcher capture;
- QL channel construction/split;
- receiver-to-QF wiring;
- higher receipt observation policy;
- expected-device admission request construction or send;
- target admission `SessionId` generation;
- expected-device PRWM authentication request-ID allocation;
- any new timing acquisition outside the existing fallible verifier-time source;
- process/executable caller migration;
- listener/readiness/network/runtime activation;
- task spawn/join or new queue/channel;
- peer-close expansion;
- retry/reconnect;
- fallback/default/cached verifier time;
- authentication/trust/RBAC mutation;
- database/schema/control-plane mutation;
- Cargo/lockfile/workflow/Android/packaging/service/repository-config mutation;
- merge, deployment, ready-for-review transition, PR close, branch deletion, reset/rebase/squash/force/history rewrite, or destructive evidence cleanup.

## 9. Selection validation and closure law

C03e-QW itself is documentation-only.

Its final validation must bind only to the exact final QW head. Expected path-filtered workflow skips must be recorded as `SKIPPED`, never PASS. No Android PASS may be inherited if no Android workflow is registered for the docs-only head.

Immutable Drive evidence must be frozen and raw-readback verified before external closure metadata is recorded.

Keep the QW PR draft/open/unmerged.

After QW evidence closure: STOP. Do not create or materialize QX inside QW closure.
