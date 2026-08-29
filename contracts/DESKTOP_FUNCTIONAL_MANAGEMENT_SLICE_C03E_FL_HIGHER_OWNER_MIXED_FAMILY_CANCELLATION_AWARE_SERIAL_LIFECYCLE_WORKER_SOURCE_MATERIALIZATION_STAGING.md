# Phase 152 C03e-FL — Higher-Owner Mixed-Family Cancellation-Aware Serial Lifecycle Worker Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FL materializes only the C03e-FK-selected isolated Agent-owned cancellation-aware higher-owner worker around the existing requester-aware mixed-family serial lifecycle.

FL must preserve all existing FJ/FB/FH/EX semantics. It does not integrate a persistent production worker, close a live authenticated peer, select candidate/reachability/endpoint/relay state, dial target traffic, activate port-forward/terminal/session behavior, alter Android behavior, widen dependencies/workflows, deploy, restart, recover, or merge.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fk-higher-owner-mixed-family-serial-lifecycle-failure-cancellation-disposition-selection-staging`
- exact head: `2e3961f6c3c51419135734c89b176f73317a2b41`
- exact tree: `74f6a4307df484703fd82cb0f4d9b3f502823b75`
- FK contract blob: `113039adda984caeb19e195fa02303577663adf3`
- FJ/FB/FH Agent source blob: `2bab48c68b63f1e5b2058c40fc3539e7841d5a32`

FL must remain an exact descendant of that head.

## 3. Authorized source scope

FL may change only:

1. this contract; and
2. `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`.

No parent-module export, bridge, transport, dependency, lockfile, Android, workflow, runtime integration, listener, packaging, deployment, restart, recovery, or merge change is authorized absent a new concrete contradiction.

## 4. Materialized worker stop law

FL materializes one Agent-local worker stop family:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

with exactly:

- `Cancelled`
- `Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError)`

The existing nested FJ error remains unchanged:

- `Ingress(AuthenticatedRemoteSessionPostAuthIngressTransactionError)`
- `RequesterResponse(RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError)`

and FH remains unchanged internally:

- `Frame(RequesterRendezvousDrAcknowledgementWireError)`
- `ResponseIo(RequesterRendezvousDrAcknowledgementResponseIoError)`

FL adds no clean `Completed` class because current FJ has no clean terminal completion.

## 5. Materialized cancellation-aware worker

FL materializes:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`

The worker owns one caller-supplied cancellation future for its full invocation. It does not hand that cancellation future by value to the existing EX worker helper because FK requires cancellation to remain available after requester handoff.

The worker reuses existing seams:

- `AuthenticatedRemoteSessionRuntimeOwner::run_repeated_post_auth_control_stream_ingress(...)`
- `continue_requester_rendezvous_retained_custody_through_dr(...)`
- `complete_requester_rendezvous_terminal_dr_acknowledgement_response(...)`

No second acceptor or alternate protocol path is introduced.

## 6. Pre-handoff cancellation race

For each serial ingress cycle, FL pins exactly one existing EX repeated-ingress future and polls:

1. ingress first;
2. caller cancellation second.

Therefore:

- already-ready requester handoff wins over simultaneous cancellation;
- already-ready ingress failure wins over simultaneous cancellation;
- cancellation wins only while ingress remains pending;
- cancellation return drops the in-flight ingress future first;
- the exclusive mutable session-owner borrow is released before the worker returns.

This preserves the existing EX ingress-first precedence.

## 7. Requester critical-section cancellation deferral

Once exact EX requester handoff exists, FL stops polling caller cancellation until the requester transaction reaches one terminal FH outcome.

The exact critical section is:

1. consume exact handoff through FB once;
2. allow exact shared-current DR validation/authorization/requester-registration composition to resolve;
3. consume exact retained continuation through FH once;
4. observe exact FH `Ok(())`, `Frame(...)`, or `ResponseIo(...)`.

Cancellation cannot preempt this sequence.

This prevents local cancellation from introducing response abandonment after requester registration may already have committed.

Existing bounded transport/runtime timeout behavior remains authoritative.

## 8. Failure precedence

If exact FH returns `Frame(...)`:

- FL returns `Failed(RequesterResponse(Frame(...)))`;
- pending cancellation does not relabel or suppress that failure;
- no next EX cycle starts;
- no response retry or fallback is attempted.

If exact FH returns `ResponseIo(...)`:

- FL returns `Failed(RequesterResponse(ResponseIo(...)))`;
- pending cancellation does not relabel or suppress that failure;
- no next EX cycle starts;
- no retry/resend/replacement stream occurs.

If pre-handoff EX returns ingress failure:

- FL returns `Failed(Ingress(...))`;
- pending cancellation does not relabel or suppress that already-ready failure.

## 9. Post-FH-success cancellation boundary

After exact FH `Ok(())`, FL polls caller cancellation once before the next loop iteration may begin.

If cancellation became ready during FB/FH:

- FL returns `Cancelled`;
- no next verifier-time sample occurs;
- no next control stream is accepted;
- no next frame is received.

If cancellation remains pending:

- the next serial EX ingress cycle may begin;
- existing EX continues to sample fresh verifier time and apply current authority.

## 10. Transaction-complete rejected acknowledgement law

An exact DR semantic `Err(RequesterRendezvousStartCompositionError)` remains a valid FD generic `Rejected` acknowledgement.

If that rejection is framed and sent successfully through FH:

- the requester transaction is complete;
- FL applies the same post-FH-success cancellation boundary;
- if not cancelled, serial mixed-family ingress may resume.

DR rejection is not FL worker failure.

## 11. Peer lifecycle law

FL performs no automatic whole-peer close for:

- `Cancelled`;
- `Failed(Ingress(...))`;
- `Failed(RequesterResponse(Frame(...)))`;
- `Failed(RequesterResponse(ResponseIo(...)))`.

FL does not reuse capability-only code 3 or code 4 and does not invent a mixed-family close code/reason.

The outer authenticated session owner remains responsible for any later peer disposition after this borrowed worker returns.

## 12. No retry/replay law

FL introduces no:

- ingress retry;
- response retry/resend;
- replacement stream;
- duplicate acknowledgement;
- DR rerun;
- requester-registration retry;
- capability re-dispatch;
- consumed requester transaction reuse;
- request ID reuse as authority.

## 13. Identity and correlation law

FL preserves:

- authenticated PRW application-session lineage as requester logical identity;
- exact nominated `DeviceId` as target logical identity;
- dynamic IP/port as transient endpoint data only;
- `TransportIdentity` as lower transport evidence only;
- PRWM `request_id` as correlation only.

Cancellation timing and error classes do not become identity.

## 14. Candidate/reachability boundary remains closed

FL does not authorize:

- candidate query/selection;
- reachability evaluation;
- endpoint resolution;
- relay selection;
- direct path selection;
- target transport establishment;
- port-forward activation;
- terminal activation;
- remote-session establishment;
- rendezvous completion.

Requester `Accepted` remains accepted-for-continuation only.

## 15. Runtime/integration boundary remains closed

FL does not:

- spawn a task;
- create a channel or queue;
- register persistent worker ownership;
- replace the existing production capability-only worker;
- integrate admission/listener lifecycle;
- publish readiness;
- alter process lifecycle control;
- alter Android behavior;
- widen dependencies/workflows;
- package/deploy/restart/recover;
- merge.

## 16. Required validation

Closure requires exact-final-head PRW Rust Validation full PASS.

If Android validation triggers for the exact final head, its terminal verdict must be recorded. If it does not trigger, no Android PASS claim may be made.

Any formatting-only corrective commit must remain within the authorized Rust path and be treated as mechanical only.

## 17. Canonical closure target

`CLOSED_HIGHER_OWNER_MIXED_FAMILY_CANCELLATION_AWARE_SERIAL_LIFECYCLE_WORKER_SOURCE_MATERIALIZATION`

## 18. Canonical gate target

`C03E_FL_HIGHER_OWNER_MIXED_FAMILY_CANCELLATION_AWARE_SERIAL_LIFECYCLE_WORKER_SOURCE_MATERIALIZED`

## 19. Next separately gated seam

After FL closure, the next checkpoint should audit/select only the higher-owner peer-retirement/reuse disposition after the isolated worker returns `Cancelled` or `Failed(...)`.

Persistent runtime/listener integration, candidate/reachability continuation, target dialing, deployment, and merge remain separately gated.
