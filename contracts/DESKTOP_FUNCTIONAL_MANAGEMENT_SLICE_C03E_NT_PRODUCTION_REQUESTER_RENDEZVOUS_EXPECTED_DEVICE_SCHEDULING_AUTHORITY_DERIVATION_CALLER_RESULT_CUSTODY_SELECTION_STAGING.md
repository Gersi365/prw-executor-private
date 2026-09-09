# C03e-NT — Production requester/rendezvous expected-device scheduling-authority derivation caller/result-custody selection

Status: **STAGING — SELECTION ONLY — SOURCE MATERIALIZATION BLOCKED**

## 1. Gate

`C03E_NT_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_DERIVATION_CALLER_RESULT_CUSTODY_SELECTED`

Closure token reserved for validated evidence closure:

`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_DERIVATION_CALLER_RESULT_CUSTODY_SELECTION`

## 2. Authoritative predecessor

Exact predecessor: C03e-NS.

Exact C03e-NS head:

`fdd12cda928ebcb0603793d2eb1ba5b88cb10da1`

Exact C03e-NS tree:

`06c628dc912daa6eb58ed28c8b8596697bcd7c60`

Exact C03e-NS target blob:

`e6d030a33f291ec78e2ddaf83bc9888711e3c4bb`

C03e-NS materialized only the dormant one-file scheduling-authority derivation composition inside:

`crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`

It materialized:

- one internal non-`Copy`, non-`Clone` `ExpectedDeviceSchedulingAuthorityGrant` containing only requester `SessionId` + target `DeviceId`;
- bounded `ExpectedDeviceSchedulingAuthorityDerivationError` classifications;
- one dormant post-registration `derive_expected_device_scheduling_authority(...)` method;
- exact provider-current relationship witness confinement;
- exact requester-session/target identity match;
- fresh current-registry validation;
- fresh requester-aware `Capability::RequesterRendezvousStart` policy reauthorization;
- terminal scheduling-consumption insertion before one-shot grant construction;
- exact requester-mutex-first/current-authority-second ordering.

C03e-NS did **not** select or materialize caller/result-custody integration, requester acknowledgement changes, peer-visible scheduling failure mapping, expected-device request construction, sender/channel custody, admission SessionId, PRWM request ID, timing, dispatcher/listener activation, retry/remint, deployment, or merge.

## 3. Selection question

C03e-NT selects the first dormant caller/result-custody boundary for the already-materialized C03e-NS scheduling derivation method.

The selection must answer all of the following without activating request construction or transport/runtime behavior:

1. which existing requester lifecycle may first call `derive_expected_device_scheduling_authority(...)`;
2. whether derivation occurs before or after requester DR acknowledgement composition/I/O;
3. how scheduling derivation success/failure is preserved independently from the already-completed requester/rendezvous DR result;
4. how a non-cloneable grant survives acknowledgement framing or response-I/O failure without duplication or loss;
5. whether derivation failure changes the requester acknowledgement;
6. whether cancellation may override a post-registration scheduling result;
7. whether the existing serial requester ingress loop continues after a scheduling derivation result exists;
8. which result owner receives the one-shot grant next;
9. which exact source path may later materialize the selected seam;
10. what remains separately gated after caller/result-custody source materialization.

## 4. Exact current caller surface

The exact C03e-NS requester continuation source is:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact C03e-NS blob:

`a8cb82f4eda44a207ba889bacd60c3f24c1901e7`

It currently owns the retained requester/rendezvous DR lifecycle.

### 4.1 Existing retained continuation

`RequesterRendezvousRetainedCustodyDrContinuation` currently retains exactly:

- one `PostAuthRequesterRendezvousTransaction` by value; and
- one `Result<(), RequesterRendezvousStartCompositionError>`.

The bridge transaction survives DR success or failure.

The current continuation does not retain a scheduling derivation result or one-shot scheduling grant.

### 4.2 Existing DR continuation

`continue_requester_rendezvous_retained_custody_through_dr(...)` currently:

1. destructures the exact requester transaction and `RequesterRendezvousStartIntent` from the existing custody handoff;
2. calls `SharedRequesterRendezvousAuthority::validate_authorize_and_register_requester_rendezvous_start(...)` once;
3. stores only the exact DR result beside the retained requester transaction;
4. performs no response I/O.

### 4.3 Existing acknowledgement boundary

`complete_requester_rendezvous_terminal_dr_acknowledgement_response(...)` currently:

1. encodes the exact requester acknowledgement only from the retained requester transaction plus exact DR result;
2. consumes the exact requester transaction into the existing same-stream response send/finish surface;
3. returns only `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`;
4. returns no retry-capable transaction or continuation.

A semantic DR error is encoded as the existing valid generic rejected acknowledgement; it is not itself an acknowledgement-composition failure.

### 4.4 Existing serial requester lifecycle

The historical serial lifecycle continues to the next ingress cycle after one successful terminal requester acknowledgement.

The cancellation-aware worker deliberately does not poll cancellation during DR or terminal acknowledgement because registration may already have committed.

### 4.5 Latest dormant production-durable worker

The latest production-shaped dormant requester worker in the same source path is:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

It already receives:

- one authenticated session runtime owner;
- production durable capability authority;
- requester DR current authority;
- requester-aware policy source;
- shared requester/rendezvous authority;
- verifier time source;
- capability dispatcher;
- cancellation future.

It currently:

1. runs the production-durable repeated post-auth ingress worker;
2. receives one exact requester handoff;
3. executes retained DR continuation;
4. completes terminal DR acknowledgement;
5. only after successful acknowledgement checks cancellation;
6. otherwise starts another ingress cycle.

The function remains dormant and is annotated as awaiting separately gated higher-owner ownership propagation.

## 5. Critical authority-separation finding

Requester acknowledgement framing and response I/O are transport/result-delivery mechanics.

They are **not** prerequisites for scheduling authority.

Therefore scheduling-authority derivation must not be conditioned on successful requester acknowledgement framing or same-stream response delivery after requester/rendezvous registration has succeeded.

If acknowledgement I/O were placed before derivation, a transport failure could suppress scheduling despite all actual scheduling prerequisites being true:

- requester/rendezvous registration already succeeded;
- exact provider relationship exists;
- fresh registry currentness can be proven;
- fresh requester policy can be reauthorized;
- terminal scheduling-consumption capacity remains available.

That would make response transport an accidental scheduling-authority gate, which is not selected.

## 6. Selected first caller lifecycle

The first caller selected for future materialization is **only**:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_capability(...)`

Rationale:

- it is the latest production-shaped requester lifecycle already carrying both requester DR authority and production durable capability ingress authority;
- it is still dormant;
- it already owns the exact post-handoff DR/acknowledgement sequence;
- it can receive and transfer a one-shot scheduling result without activating request construction;
- it avoids retroactively changing historical requester lifecycle variants.

The following historical functions are not selected as first scheduling-derivation callers and should retain their existing semantics in the first source successor:

- `run_requester_rendezvous_post_terminal_response_serial_lifecycle(...)`;
- `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`.

## 7. Selected ordering relative to DR and acknowledgement

For the selected production-durable worker, the future ordering is:

1. receive one exact requester/rendezvous handoff;
2. preserve exact requester `SessionId` and target `DeviceId` as non-authorizing selectors before consuming the start intent;
3. run the existing requester/rendezvous DR registration composition exactly once;
4. if DR fails, do **not** invoke scheduling derivation;
5. if DR succeeds, immediately invoke the C03e-NS dormant scheduling derivation method;
6. retain the scheduling derivation result as an orthogonal internal result channel;
7. only after the derivation result exists, compose the requester DR acknowledgement from the unchanged DR result only;
8. attempt the existing same-stream acknowledgement send/finish exactly once;
9. preserve acknowledgement disposition independently from the scheduling derivation result;
10. if a scheduling derivation result exists, return terminal caller/result custody upward before any cancellation poll or next ingress cycle;
11. only when DR failed and its acknowledgement completed successfully may the historical cancellation check / next-ingress behavior continue unchanged.

Thus:

`DR success -> scheduling derivation -> ACK framing/send`

not:

`DR success -> ACK framing/send -> scheduling derivation`.

## 8. Why derivation precedes acknowledgement I/O

The selected order preserves authority separation:

- scheduling derivation depends only on requester/provider/current-registry/policy/terminal-consumption authority;
- acknowledgement delivery does not mint, revoke, validate or invalidate scheduling authority;
- response-I/O failure cannot silently suppress a scheduling decision that was otherwise authorized;
- acknowledgement transport cannot reopen or reset terminal scheduling-consumption state;
- the scheduler result remains valid independently from peer response delivery.

Both requester and current-authority locks are already released when the C03e-NS derivation method returns, so acknowledgement composition/I/O remains outside authority locks.

## 9. Selected derivation condition

Scheduling derivation is attempted **only** when the exact requester/rendezvous DR result is `Ok(())`.

If DR returns any `RequesterRendezvousStartCompositionError`:

- scheduling derivation is not called;
- no scheduling ledger insertion occurs;
- no one-shot scheduling grant is constructed;
- the exact existing rejected DR acknowledgement is composed/sent as today;
- after successful rejected acknowledgement the historical serial lifecycle may continue normally.

A DR failure is not translated into a scheduling derivation error.

## 10. Selected orthogonal result channels

After DR success, future caller integration must preserve two independent terminal channels:

### 10.1 Scheduling channel

Exactly one:

`Result<ExpectedDeviceSchedulingAuthorityGrant, ExpectedDeviceSchedulingAuthorityDerivationError>`

Properties:

- success owns the unique non-Clone one-shot grant by value;
- failure preserves the exact bounded derivation classification;
- neither success nor failure is encoded into requester DR acknowledgement framing;
- no channel value is duplicated, cloned, reminted or reconstructed.

### 10.2 Requester acknowledgement channel

Exactly one:

`Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`

Properties:

- framing uses only the existing exact requester transaction + existing DR result;
- scheduling derivation success/failure does not alter Accepted/Rejected semantics;
- response-I/O failure does not erase scheduling outcome custody;
- acknowledgement failure does not authorize retry/resend/replacement stream.

The future composition must retain both channels together after DR success.

## 11. Selected terminal custody shape

The source successor should introduce one non-`Copy`, non-`Clone` terminal scheduling caller-result carrier for the production-durable worker.

Semantic shape:

`RequesterRendezvousSchedulingAuthorityCallerTerminalOutcome`

containing:

- one scheduling derivation result by value; and
- one requester acknowledgement disposition by value.

The exact Rust name may be refined in source materialization if needed for local naming consistency, but the semantic shape is fixed by this selection.

The carrier owns no raw requester transaction after acknowledgement completion.

It owns no stream, sender, channel, dispatcher, endpoint, timing, PRWM request ID, admission SessionId or expected-device request.

## 12. Grant custody law

If scheduling derivation succeeds:

- the one-shot `ExpectedDeviceSchedulingAuthorityGrant` must be moved exactly once into the terminal caller-result carrier;
- acknowledgement framing success or failure must not drop the grant;
- acknowledgement response-I/O success or failure must not drop the grant;
- cancellation must not override or replace the grant after it exists;
- the serial requester worker must stop and transfer the grant upward;
- no next requester ingress cycle may begin while the one-shot grant is still owned by that worker;
- no grant clone, copy, reconstruction or re-derivation is allowed.

The next separately gated higher-owner checkpoint will decide whether and how that grant may later be consumed by expected-device request construction.

## 13. Derivation failure custody law

If requester DR succeeded but scheduling derivation fails:

- requester/rendezvous registration remains successful;
- provider state is not rolled back;
- requester acknowledgement remains the existing success acknowledgement;
- the derivation error remains internal and typed;
- no second peer-visible scheduling error frame is emitted;
- the selected production-durable requester worker stops after acknowledgement disposition is known;
- the terminal caller-result carrier preserves both the derivation failure and acknowledgement disposition;
- no next requester ingress cycle begins on that worker;
- no retry/remint/ledger reset/provider replacement is authorized.

This preserves truthful separation between:

1. successful requester/rendezvous registration; and
2. failed scheduling-authority derivation.

## 14. Acknowledgement failure after scheduling result exists

If DR succeeded and scheduling derivation has already produced either success or failure, a later acknowledgement framing or response-I/O failure does **not** replace that scheduling result.

The terminal caller-result carrier must preserve both facts.

Examples:

### 14.1 Grant + ACK success

Preserve:

- scheduling grant by value;
- acknowledgement `Ok(())`.

### 14.2 Grant + ACK failure

Preserve:

- scheduling grant by value;
- exact acknowledgement framing or response-I/O error.

The grant is not dropped and terminal consumption is not reopened.

### 14.3 Derivation failure + ACK success

Preserve:

- exact derivation error;
- acknowledgement `Ok(())`.

### 14.4 Derivation failure + ACK failure

Preserve:

- exact derivation error;
- exact acknowledgement error.

Neither channel overwrites the other.

## 15. Cancellation ordering

Existing requester lifecycle intentionally avoids cancellation polling after requester handoff while DR/acknowledgement may commit or complete terminal state.

C03e-NT extends that non-cancellable critical lifecycle segment only semantically to include the new post-DR scheduling derivation and terminal result preservation.

Selected rule:

- no cancellation poll between successful DR registration and scheduling derivation;
- no cancellation poll between scheduling derivation result creation and requester acknowledgement completion attempt;
- once a scheduling result exists, the worker returns that terminal result upward before checking cancellation;
- cancellation cannot erase a grant, replace a derivation error, or force a next ingress cycle;
- if DR failed and no scheduling derivation was attempted, the historical post-ack cancellation check remains unchanged.

## 16. Serial lifecycle termination rule

The selected production-durable requester worker becomes terminal for one scheduling-result-bearing transaction.

After DR success:

- derivation is attempted exactly once;
- acknowledgement is attempted exactly once;
- the scheduling + acknowledgement terminal result is returned upward exactly once;
- the worker does not loop to another ingress cycle.

After DR failure:

- no derivation result exists;
- existing acknowledgement semantics remain unchanged;
- successful rejected acknowledgement may return to the historical cancellation/next-ingress loop.

This prevents silent loss of a non-cloneable scheduling grant or scheduling derivation error.

## 17. Historical lifecycle preservation

The first source successor must not require scheduling derivation in the older historical requester lifecycle variants.

Existing behavior of:

- `run_requester_rendezvous_post_terminal_response_serial_lifecycle(...)`; and
- `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`

must remain behaviorally unchanged.

If source implementation would require altering those historical variants' scheduling semantics, the successor must STOP and return to a documentation gate.

## 18. Provider and candidate-publication preservation

Caller integration does not change requester/rendezvous provider lifecycle.

Derivation success does not retire or remove the current requester/rendezvous record.

Derivation failure does not retire or remove it.

Acknowledgement failure does not retire or remove it.

The candidate-publication `AuthorizedRequesterRendezvous` witness remains confined inside the C03e-NS derivation method and is never exposed to the caller-result carrier.

No candidate-publication grant is returned, stored or reused as scheduling authority.

## 19. Peer-visible semantics

Requester DR acknowledgement remains sourced exclusively by:

`Result<(), RequesterRendezvousStartCompositionError>`

Scheduling derivation result does not change requester acknowledgement framing.

C03e-NT selects **no** new peer-visible scheduling failure mapping.

There is:

- no second response frame;
- no scheduling-specific rejection frame;
- no requester close code;
- no stream retry;
- no resend;
- no replacement stream.

Any future peer-visible scheduling failure policy requires a separate documentation gate.

## 20. Exact first source-successor ceiling

Next separately gated source boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_DERIVATION_CALLER_RESULT_CUSTODY_SOURCE_MATERIALIZATION`

Likely successor branch after a fresh authority check:

`phase-152-c03e-nu-production-requester-rendezvous-expected-device-scheduling-authority-derivation-caller-result-custody-source-materialization`

Exact source path ceiling:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

No second path is authorized.

Allowed source successor changes in that one file only:

- import the already-materialized C03e-NS scheduling grant/error types through existing sibling-module visibility;
- add a production-durable-only DR+derivation continuation/result-custody seam;
- preserve exact requester SessionId + target DeviceId selectors before consuming the start intent;
- invoke scheduling derivation only after DR success;
- invoke derivation before requester acknowledgement framing/I/O;
- add a non-cloneable scheduling + acknowledgement terminal result carrier;
- adapt only the latest production-durable requester worker to return terminal scheduling result custody;
- preserve historical requester lifecycle functions unchanged;
- preserve existing rejected-DR acknowledgement loop semantics;
- focused tests proving result-channel orthogonality and non-dropping grant custody.

If source materialization requires any second file, module export change, runtime activation path, expected-device request construction or sender/channel path, it must STOP and return to a documentation gate.

## 21. Explicitly forbidden in the next source successor

Not authorized:

- changes to `shared_requester_rendezvous_authority.rs`;
- changes to `remote_session_capability_runtime.rs` exports;
- changes to `linux_bootstrap.rs`;
- changes to higher-owner population/custody source;
- new channel or queue;
- expected-device admission request construction;
- expected-device sender;
- admission SessionId generation/source;
- PRWM request-ID generation/source;
- admission timing source;
- dispatcher/listener/runtime activation;
- task spawn solely for scheduling;
- automatic retry/remint/reconnect/replay;
- provider rollback;
- ledger rollback/reset;
- candidate-publication grant reuse;
- requester acknowledgement rewrite;
- peer close;
- deployment;
- merge;
- branch deletion/force update/history rewrite.

## 22. Next boundary after caller/result-custody source materialization

Even after the one-file caller/result-custody source successor closes, expected-device request materialization remains blocked.

A fresh documentation checkpoint must next select the consumption boundary for the one-shot scheduling grant, including at minimum:

- which higher owner receives the grant-bearing terminal result;
- whether requester acknowledgement failure affects later request-construction permission;
- exact expected-device admission SessionId authority/source;
- exact PRWM authentication request-ID authority/source;
- exact admission timing authority/source;
- exact request-construction owner;
- exact channel/sender custody;
- whether request construction occurs synchronously or through an existing worker owner;
- proof target logical DeviceId remains logical identity and not endpoint authority;
- proof fresh registry/transport/authentication checks remain receiver-side independent requirements;
- no retry/remint on downstream failure.

No choice in C03e-NT authorizes those future steps.

## 23. Closed selection

Upon exact-head validation and durable evidence publication, C03e-NT closes with:

`PRODUCTION_DURABLE_REQUESTER_WORKER_SELECTED_AS_FIRST_DERIVATION_CALLER / DERIVATION_AFTER_DR_SUCCESS_BEFORE_ACKNOWLEDGEMENT_COMPOSITION_AND_IO_SELECTED / DR_FAILURE_SKIPS_DERIVATION / SCHEDULING_DERIVATION_AND_REQUESTER_ACKNOWLEDGEMENT_RESULTS_PRESERVED_AS_ORTHOGONAL_TERMINAL_CUSTODY / NON_CLONE_GRANT_MUST_SURVIVE_ACKNOWLEDGEMENT_FAILURE / DERIVATION_FAILURE_DOES_NOT_REWRITE_SUCCESSFUL_DR_ACKNOWLEDGEMENT / CANCELLATION_CANNOT_OVERRIDE_POST_REGISTRATION_SCHEDULING_TERMINAL_RESULT / PRODUCTION_DURABLE_WORKER_STOPS_AND_TRANSFERS_SCHEDULING_RESULT_UPWARD / HISTORICAL_REQUESTER_LIFECYCLES_REMAIN_UNCHANGED / ONE_FILE_CALLER_RESULT_CUSTODY_MATERIALIZATION_SEAM_SELECTED / REQUEST_CONSTRUCTION_AND_SEND_REMAIN_BLOCKED / SOURCE_MATERIALIZATION_BLOCKED`

## STOP

C03e-NT is documentation-only.

No Rust/source/runtime mutation is authorized by this contract.

The next source checkpoint may begin only after this selection is exact-head validated, durably evidenced, and closed.