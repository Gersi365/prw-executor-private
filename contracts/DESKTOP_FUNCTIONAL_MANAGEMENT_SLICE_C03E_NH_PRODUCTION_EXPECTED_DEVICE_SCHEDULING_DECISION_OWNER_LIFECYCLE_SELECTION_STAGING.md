# Phase 152 C03e-NH — Production expected-device scheduling decision owner/lifecycle selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NH_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_OWNER_LIFECYCLE_SELECTED`

## 1. Purpose

C03e-NG closed the production expected-device scheduling event authority-binding audit fail-closed.

NG established that the exact production repository contains no already-proven authority binding granting any existing lifecycle or operation the right to produce one bounded pre-authentication expected-device scheduling event for one exact logical `DeviceId`.

NG selected the next blocking prerequisite:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_OWNER_LIFECYCLE`

C03e-NH audits only that prerequisite.

This checkpoint asks one bounded question:

> Does the exact post-NG production source already establish a production operation or lifecycle whose existing responsibility and authority semantics make it the owner of the decision that one exact logical `DeviceId` should receive one bounded pre-authentication expected-device admission scheduling attempt?

C03e-NH does not create such an owner. It does not create a scheduling event, request, sender, channel, policy capability, lifecycle callback, runtime task, retry loop, or deployment path.

## 2. Exact predecessor

Canonical repository:

`Gersi365/prw-executor-private`

Repository ID:

`1334911207`

Predecessor branch:

`phase-152-c03e-ng-production-expected-device-scheduling-event-authority-binding-selection`

Exact predecessor head:

`8d22d9644ad785bdda80a77dfa0df068b708ca4f`

Exact predecessor tree:

`b58fcc9851a33def33e724f2541acef8096154b3`

Predecessor closed selection:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_AUTHORITY_BINDING_PROVEN / NO_EXISTING_PRODUCTION_LIFECYCLE_OWNS_EXPECTED_DEVICE_SCHEDULING_DECISION / EXISTING_AUTHORITIES_REMAIN_OPERATION_SCOPED / SOURCE_MATERIALIZATION_BLOCKED`

C03e-NH is rooted exactly at that head.

## 3. Authority question

The decision being audited is narrower than remote-session authentication and narrower than requester/rendezvous start.

The audited decision is:

1. choose one exact logical target `DeviceId`;
2. decide that one bounded pre-authentication real-admission attempt may be scheduled for that target;
3. establish the authority under which that decision is made;
4. bind current registry/revocation semantics to the decision;
5. bound the decision's cardinality, lifetime, cancellation and concurrency semantics;
6. only after those properties are proven may a future separately gated event/request producer be considered.

The owner of this decision cannot be inferred from possession of values that are merely consumed by later stages.

## 4. Exact source surfaces audited

C03e-NH re-audits the exact NG head and the following production-relevant surfaces:

- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`;
- `crates/prw-policy/src/lib.rs`.

No source outside the exact predecessor head is treated as authority evidence for this selection.

## 5. Production remote-process composition is a consumer boundary

`LinuxAgentRemoteProcessOperationInputs` owns an injected:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<...>>`

The production composition helpers continue to accept that receiver from their caller and pass it into the repeated-admission runtime.

That proves receiver-side custody only.

It does not prove:

- who creates the production channel;
- who owns the production sender;
- who chooses one target `DeviceId`;
- which authenticated principal or lifecycle authorizes that choice;
- which operation causes one request to be enqueued;
- how that operation is cancelled or invalidated;
- whether the operation may schedule once or repeatedly.

Observed `mpsc::channel` construction in `linux_bootstrap.rs` remains test scaffolding. No production channel/sender decision owner is established by the exact source.

Therefore the Linux production remote-process composition is not selected as the expected-device scheduling decision owner.

## 6. Real admission is consumer-side validation, not decision ownership

`real_remote_admission_transaction.rs` accepts an already-selected `intended_device_id`.

The transaction then performs the existing bounded real-admission sequence, including fresh registry/transport resolution and authenticated identity checking.

That transaction is authoritative for the admission checks it performs.

It is not authoritative for the earlier decision that the admission attempt should exist.

The fact that it can reject a stale, mismatched or unauthenticated peer does not retroactively authorize the caller that chose the intended `DeviceId`.

Therefore real admission remains a consumer/validator of scheduling intent, not the producer-side decision owner.

## 7. Repeated-admission supervisor is consumer-side scheduling machinery

`RemoteSessionExpectedDeviceAdmissionRequest` carries:

- one pre-authentication expected logical `DeviceId`;
- one `SessionId`;
- one authentication request identifier;
- one dispatcher;
- one verifier-time provider.

Its `expected_device_id` is explicitly the logical device used for scheduling the admission attempt.

The repeated supervisor can reject a request when an active authenticated worker already owns the same logical `DeviceId`.

That duplicate rule is a receiver-side safety property.

It does not prove that the producer was authorized to schedule the request.

The supervisor does not discover the business/lifecycle reason why a new expected-device request should exist.

Therefore repeated admission remains consumer-side machinery and is not selected as the decision owner.

## 8. Requester target intent is not the decision owner

`RequesterRendezvousTargetIntent` is explicitly non-authoritative caller-nominated target intent.

It carries no requester identity and grants no current-registration fact or authorization.

`RequesterRendezvousStartIntent` combines an already-authenticated requester session with a nominated target, but construction still performs ownership composition only.

The target remains intent until later validation and authorization.

Possession of a nominated target cannot itself authorize a pre-authentication admission scheduling decision.

Therefore requester target intent is not selected as the owner.

## 9. Registry validation is eligibility evidence, not lifecycle ownership

`RegistryValidatedRequesterRendezvousStart` proves point-in-time current-registry eligibility for the requester/target pair.

Its source explicitly excludes policy authorization, provider registration authority, transport readiness, live-owner authority, candidate-publication authority and perpetual currentness.

Registry eligibility is a required safety condition for some operations.

It does not, by itself, establish the operation that owns a new expected-device scheduling decision.

Therefore registry validation is not selected as the owner.

## 10. Requester/rendezvous policy is operation-scoped

The existing dedicated policy stage evaluates exactly:

`Capability::RequesterRendezvousStart`

That capability authorizes the requester/rendezvous-start operation under its existing contract.

C03e-NH finds no source contract stating that this same decision also authorizes production creation of a repeated-admission expected-device scheduling event.

Treating the requester/rendezvous policy grant as a remote-session scheduling grant would widen an existing capability across operation boundaries.

That widening is prohibited.

Therefore `RequesterRendezvousStart` is not selected as the expected-device scheduling decision authority.

## 11. Requester/rendezvous runtime custody is provider-state ownership only

`CandidatePublicationRequesterRendezvousRuntimeOwner` owns bounded in-memory requester/rendezvous provider state.

The runtime can consume a policy-authorized requester/rendezvous start and register the requester-session/target-device relationship in the provider.

That registration is a legitimate provider mutation under the existing requester/rendezvous operation.

It does not independently define:

- expected-device admission scheduling authority;
- remote-session creation authority;
- a one-shot admission lifecycle;
- a production sender/channel owner;
- a fresh admission request ID;
- an admission `SessionId`;
- an admission timing source.

Provider custody does not become a cross-operation scheduling decision owner by implication.

Therefore requester/rendezvous runtime custody is not selected as the owner.

## 12. Requester/rendezvous retained continuation remains post-authenticated and operation-scoped

The retained-custody continuation begins from an authenticated requester-side remote-session operation and continues the requester/rendezvous flow using the existing operation-specific authorities.

That continuation may lawfully perform the requester/rendezvous behavior already selected for it.

C03e-NH finds no exact source contract granting it a second independent authority to originate a new pre-authentication remote-session admission for the target.

Using post-authenticated requester custody to infer pre-auth target scheduling authority would cross operation and session-lifecycle boundaries.

Therefore the retained requester/rendezvous continuation is not selected as the expected-device scheduling decision owner.

## 13. Candidate publication and reachability do not own the decision

Candidate publication proves or mutates candidate-publication state under its own established authority.

Reachability authority owns the reachability operation selected for it.

Neither domain establishes why a new remote-session admission attempt should be scheduled.

A candidate, endpoint, reachability success, live-owner token, bound port or transport result can be useful only after the relevant operation has already been authorized.

None may become a scheduling trigger or decision authority by existence alone.

Therefore neither candidate publication nor reachability is selected as the owner.

## 14. Configured peer identity is process intent only

`PRW_REMOTE_PEER_DEVICE_ID` is an explicit production configuration source for one logical peer intent.

The source contract already distinguishes configuration validity from identity/authentication/authorization/current transport authority.

A configured logical peer may constrain which peer the process intends to interact with.

It does not establish why or when a new expected-device admission should be scheduled, nor which principal/lifecycle is authorized to request that action.

Therefore configured peer identity is not selected as the decision owner.

## 15. Transport identity and endpoint state are not owner authority

`TransportIdentity`, IP address, port, endpoint, route, candidate and reachability values are transport/reachability evidence only.

They are downstream of logical identity and authority selection.

They cannot define the logical scheduling decision owner.

Transport-key rotation must not redefine logical device identity or implicitly trigger a new scheduling authority path.

Therefore transport/reachability state is not selected as the owner.

## 16. Post-authenticated logical identity is authoritative too late

After successful authentication, the authenticated session owner is authoritative for the logical device identity attached to that authenticated session.

That identity can safely key the retained active-worker collection and receiver-side duplicate handling.

It exists after the pre-authentication scheduling decision has already been made.

It cannot retroactively establish who was authorized to create that scheduling decision.

Therefore post-authenticated identity is not selected as the decision owner.

## 17. Correlation, session and timing values are not authority

The following values remain non-authoritative for this decision:

- PRWM `request_id`;
- authentication request identifier;
- `SessionId`;
- verifier timestamp/provider;
- admission timing ranges;
- channel identity;
- channel capacity;
- worker/task identifiers;
- completion correlation.

These values may be required for bounded execution and correlation after an authorized operation has been selected.

They do not authorize creation of that operation.

## 18. Generic policy capabilities are not widened

The exact `Capability` enum contains existing operation capabilities, including `DeviceManage` and `RequesterRendezvousStart`.

C03e-NH finds no exact source binding from `DeviceManage` to expected-device remote admission scheduling.

A semantically broad capability name is not sufficient evidence to infer a new production operation.

Likewise, the presence of `RequesterRendezvousStart` does not authorize a different pre-auth remote-session scheduling operation.

No existing capability is widened to fill this missing decision-owner boundary.

## 19. No control-plane decision owner is proven

The exact audited production source does not expose a control-plane event, durable command, queue item or authenticated control-plane operation whose existing semantics are:

> schedule one bounded pre-authentication admission attempt for this exact logical `DeviceId`.

No such control-plane event source may be invented from registry state, configuration, candidate state, endpoint state or generic device-management permission.

Therefore no control-plane owner is selected.

## 20. No autonomous lifecycle decision owner is proven

The exact audited production lifecycle does not establish an autonomous rule such as:

- schedule when process starts;
- schedule when a peer is configured;
- schedule when reachability becomes available;
- schedule when a candidate is published;
- schedule when capacity becomes free;
- schedule when a worker completes;
- schedule on a timer;
- schedule on retry/recovery.

Any such rule would be new behavior and could create repeated or unauthorized admission attempts.

Therefore no autonomous lifecycle owner is selected.

## 21. No requester-driven scheduling owner is proven

The strongest requester-side evidence is an authenticated requester/rendezvous operation plus current-registry validation and dedicated requester/rendezvous policy authorization.

That chain is authoritative for the requester/rendezvous operation it defines.

It is not an established cross-operation grant for pre-auth remote-session scheduling.

Therefore no requester-driven scheduling owner is selected from the existing source.

## 22. Decision currentness/revocation commit point remains unselected

Because no decision owner is proven, C03e-NH cannot lawfully select a producer-side currentness/revocation commit point.

The existing real-admission transaction performs fresh registry/transport checks as part of admission validation.

Those checks remain mandatory for their existing purpose.

They do not replace the need for a future selected decision owner to prove the appropriate current-registry/revocation semantics at or immediately before producing a scheduling event.

No lease, cache or stale validated carrier is promoted into perpetual scheduling authority.

## 23. Cardinality remains unselected at the producer boundary

No existing production owner establishes whether the decision may be:

- exactly once per authenticated requester operation;
- exactly once per target state transition;
- exactly once per explicit control-plane command;
- repeatedly attempted under a bounded retry policy;
- cancelled before enqueue;
- consumed exactly once after enqueue.

Receiver-side worker capacity and duplicate-active-device rejection do not answer these producer-side cardinality questions.

Therefore producer cardinality remains blocked.

## 24. Cancellation and invalidation remain unselected at the producer boundary

No existing owner establishes what must invalidate an unconsumed scheduling decision.

A future decision-origin selection must explicitly address at least:

- requester session invalidation, if requester-driven;
- target revocation;
- target membership suspension/removal;
- workspace relationship change;
- policy denial/change;
- process shutdown;
- operation cancellation;
- replacement/supersession;
- expiry, if any bounded lifetime is selected.

C03e-NH does not invent these semantics.

## 25. Duplicate/concurrent producer decisions remain unselected

The repeated-admission supervisor already rejects one request when an active authenticated worker owns the same logical `DeviceId`.

That is insufficient to classify concurrent producer decisions that occur before an active worker exists.

A future decision origin must define whether duplicate/concurrent decisions are:

- rejected before enqueue;
- coalesced;
- serialized;
- independently valid under distinct explicit authority;
- superseded/cancelled.

No such producer-side classification is proven by the exact source.

## 26. Closed C03e-NH selection

C03e-NH selects:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_OWNER_LIFECYCLE_PROVEN / REQUESTER_RENDEZVOUS_AND_PROVIDER_LIFECYCLES_REMAIN_OPERATION_SCOPED / REAL_ADMISSION_AND_REPEATED_SUPERVISOR_REMAIN_CONSUMER_SIDE / SOURCE_MATERIALIZATION_BLOCKED`

Meaning:

1. no existing production lifecycle or operation is proven to own the expected-device scheduling decision;
2. no existing authenticated principal/policy pair is silently promoted into that role;
3. no autonomous lifecycle trigger is inferred;
4. no control-plane trigger is inferred;
5. no requester/rendezvous, candidate-publication, reachability, configured-peer, transport or post-auth authority is widened;
6. no sender/channel/event/request materialization is authorized.

## 27. Selected next blocking prerequisite

C03e-NH selects the next separately gated prerequisite:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_ORIGIN_OPERATION_SELECTION`

The next checkpoint must remain documentation-only.

It must select which production operation class is intended to originate the scheduling decision before any authority carrier or runtime implementation is considered.

The next checkpoint must compare at least these possibilities without presuming any is valid:

- explicit authenticated requester-driven operation;
- explicit authenticated control-plane/device-management operation;
- explicit non-user lifecycle operation with independently proven authority;
- another already-authoritative production operation discovered from fresh exact-head evidence;
- if none is compatible, an explicit new operation boundary that requires a later separately gated policy/authority design.

The selection must identify:

1. the exact operation semantic;
2. its initiating principal/session or non-user authority;
3. its target logical `DeviceId` provenance;
4. whether an existing policy capability exactly matches that semantic;
5. current-registry/revocation validation placement;
6. one-shot/cardinality rules;
7. cancellation/invalidation rules;
8. duplicate/concurrent decision classification;
9. evidence that the operation does not widen any existing authority domain.

## 28. STOP boundary

After C03e-NH selection, do not materialize:

- a decision-owner Rust type;
- a new policy capability;
- a policy evaluator branch;
- a scheduling decision carrier;
- a scheduling event carrier;
- `RemoteSessionExpectedDeviceAdmissionRequest` production construction;
- a production `mpsc` channel;
- sender custody or sender cloning;
- send/try_send behavior;
- `SessionId` generation;
- authentication PRWM request-ID allocation/reuse;
- verifier-time construction;
- admission timing policy;
- dispatcher construction/migration for this purpose;
- a background task;
- timer/retry/reconnect loop;
- candidate/reachability-triggered scheduling;
- listener/readiness activation;
- `main.rs` wiring;
- systemd/service/package changes;
- deployment/restart/recovery activation.

Those remain separately gated after `PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_ORIGIN_OPERATION_SELECTION` is selected from fresh evidence.

## 29. Repository and evidence guardrails

C03e-NH must remain one documentation-only commit on top of exact C03e-NG.

Allowed path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_NH_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_OWNER_LIFECYCLE_SELECTION_STAGING.md`

No Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging/security mutation is allowed in this checkpoint.

The PR must remain draft/open/unmerged after closure.

Exact-final-head CI must be observed independently for C03e-NH.

Path-filtered `SKIPPED` workflows must be recorded as `SKIPPED`, never as PASS.

Durable evidence must be published as immutable raw text under the canonical Drive evidence parent and verified by byte-exact raw readback and SHA-256.

## 30. Identity and authorization invariants

C03e-NH preserves:

`logical DeviceId/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Logical `DeviceId` is not IP address.

`TransportIdentity` is transport evidence only.

Transport-key rotation does not redefine logical device identity.

Endpoint/IP/port is transient reachability only.

PRWM `request_id` is correlation only.

Requester target nomination is intent only until the separately required authority chain succeeds.

Registry eligibility is not policy authorization.

Provider registration is not remote-session scheduling authority.

Candidate publication is not remote-session scheduling authority.

Reachability is not remote-session scheduling authority.

Successful authentication cannot retroactively authorize the pre-auth scheduling decision.

Receiving devices continue to enforce authorization for protected operations.

## 31. Closure criterion

C03e-NH may be classified closed only after all of the following are true:

- the exact predecessor is still NG head `8d22d9644ad785bdda80a77dfa0df068b708ca4f`;
- exactly one NH contract path differs;
- exact topology is one commit ahead and zero behind;
- exact-final-head CI reaches terminal classification;
- durable Drive evidence is published only after terminal validation;
- raw Drive readback matches frozen local bytes and SHA-256;
- exact-title Drive uniqueness is verified;
- final branch and PR state are re-read;
- PR remains draft/open/unmerged;
- no source materialization has occurred.

Until those checks complete, status remains staged rather than closed.
