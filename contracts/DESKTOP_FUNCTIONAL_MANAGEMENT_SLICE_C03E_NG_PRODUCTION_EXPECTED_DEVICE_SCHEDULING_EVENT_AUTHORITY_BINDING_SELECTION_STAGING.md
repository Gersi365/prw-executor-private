# Phase 152 C03e-NG — Production expected-device scheduling event authority binding selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NG_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_AUTHORITY_BINDING_SELECTED`

## 1. Purpose

C03e-NF proved that the exact production repository contains no already-authoritative event source whose established contract may directly produce repeated-admission `expected_device_id` scheduling intent.

NF narrowed the next prerequisite to:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_AUTHORITY_BINDING`

C03e-NG audits only that boundary.

This checkpoint asks one bounded question:

> Does the exact post-NF production source already bind any authoritative operation or lifecycle decision to the right to produce one bounded pre-authentication expected-device scheduling event for one exact logical `DeviceId`?

C03e-NG does not create a decision owner, scheduling event, sender, channel, request, or runtime activation.

## 2. Exact predecessor

Canonical repository:

`Gersi365/prw-executor-private`

Exact predecessor branch:

`phase-152-c03e-nf-production-expected-device-scheduling-event-source-provenance-selection`

Exact predecessor head:

`9f92e512b91e4eee59b1a973b3e1478ae64eb2bd`

Exact predecessor tree:

`8456fd3624959fb34b77b18a02e9947e177b30d8`

C03e-NF selection:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_SOURCE_PROVEN / REQUESTER_RENDEZVOUS_AUTHORITY_REMAINS_CANDIDATE_PUBLICATION_SCOPED / REPEATED_ADMISSION_EXPECTED_DEVICE_REMAINS_INJECTED_PREFLIGHT_EVIDENCE / SOURCE_MATERIALIZATION_BLOCKED`

## 3. Selection result

C03e-NG selects:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_AUTHORITY_BINDING_PROVEN / NO_EXISTING_PRODUCTION_LIFECYCLE_OWNS_EXPECTED_DEVICE_SCHEDULING_DECISION / EXISTING_AUTHORITIES_REMAIN_OPERATION_SCOPED / SOURCE_MATERIALIZATION_BLOCKED`

The exact post-NF repository has components that own local lifecycle, reachability authority, requester/rendezvous authorization, candidate-publication authority, remote-session admission consumption, and post-authenticated worker identity.

None of those established contracts grants one existing operation or lifecycle the authority to decide that one exact logical `DeviceId` shall enter one pre-authentication expected-device admission attempt.

Therefore C03e-NG authorizes no Rust source materialization.

## 4. Authority-binding law

A valid production expected-device scheduling authority binding must establish all of the following without inference from shared data types:

1. one exact production decision owner or lifecycle owns the decision to request one admission attempt;
2. that owner acts under an already-established authenticated principal/session and policy authority, or an explicitly selected non-user lifecycle authority whose scope includes the scheduling decision;
3. the exact logical `DeviceId` originates from an authoritative carrier within that operation, not from transport or configuration convenience;
4. the binding expressly permits producing one pre-authentication expected-device scheduling event;
5. current registry/revocation state is checked at the selected commit point or is carried by a separately selected consumable currentness proof;
6. the authority is bounded to one event or another explicitly selected bounded cardinality;
7. cancellation, revocation, authority replacement, stale currentness, or ambiguous ownership fails closed;
8. duplicate/concurrent scheduling cannot create a second authority path around the receiver-side duplicate guard;
9. sender/channel custody, if later materialized, can only transmit events produced under this exact binding;
10. no request/correlation ID, endpoint, transport identity, candidate, configured peer identity, or authenticated result is silently promoted into pre-auth scheduling authority.

C03e-NG finds no existing source satisfying this complete binding law.

## 5. Local one-shot scheduler is not the owner

Exact source:

`crates/prw-agent/src/linux_one_shot_scheduler.rs`

Exact NF blob:

`ff7b890133c942b9239fc5262c64a64803a6177f`

This module is a local Unix-socket authenticated-session scheduling transaction from the much earlier local Agent lane.

Its own module contract states that it performs at most one local accept attempt, is not an accept loop, and is not wired into Agent bootstrap.

It schedules local scoped workers after local listener acceptance. It does not carry a remote logical target `DeviceId`, does not own requester/rendezvous policy, and does not produce `RemoteSessionExpectedDeviceAdmissionRequest`.

It therefore cannot be promoted into the C03e expected-device scheduling decision owner merely because its name contains `scheduler`.

## 6. Production-local lifecycle is not the owner

Exact source:

`crates/prw-agent/src/linux_production_lifecycle.rs`

Exact NF blob:

`c8a81b7c57dcfc0c9c28d7c37d43f6f3f5a4c5a4`

This lifecycle assembles local XDG/runtime-directory/lock/listener/wake/capacity/scheduler-control resources below one callback boundary.

Its own contract states that it does not run a readiness loop, process OS signals, wire `main.rs`, or activate a service manager.

The callback receives local listener, wake, worker-capacity and local scheduler-control references. It receives no authoritative remote logical target, authenticated requester/rendezvous operation, remote-session scheduling authority, or expected-device event producer.

Local process lifecycle ownership is therefore not remote expected-device scheduling authority.

## 7. Reachability authority is not the scheduling decision owner

Exact source:

`crates/prw-agent/src/reachability_authority_admission.rs`

Exact NF blob:

`f347027b6fe77c47ca3a647c2969a81fc7305565`

The Agent-owned reachability authority admission seam proves successful construction and custody of one reachability live-owner authority.

Its contract expressly says the path is not wired into `main.rs`, runtime readiness, remote transport, or background/retry lifecycle and exposes authority only for a future separately gated reachability operation consumer.

The reachability admission token contains no expected-device scheduling decision, no exact scheduling cardinality, no authenticated requester/target operation binding, and no `RemoteSessionExpectedDeviceAdmissionRequest` producer.

Reachability authority may be a prerequisite for future network operations, but it does not by itself authorize scheduling one logical device for admission.

## 8. Requester target nomination is not the binding

Exact source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`

Exact NF blob:

`5f616f20699d1c7069f5aa8973200a0359c19cde`

`RequesterRendezvousTargetIntent` is explicitly non-authoritative.

It retains one caller-nominated logical target `DeviceId`, carries no requester identity, and confers no authorization or current-registration fact.

`RequesterRendezvousStartIntent` combines an already-authenticated requester session with the nominated target, but possession remains non-authoritative until separate validation and policy stages succeed.

Raw nomination therefore cannot be the authority binding for expected-device admission scheduling.

## 9. Registry validation is not the binding

Exact source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`

Exact NF blob:

`1c021bc95a3d674722bfd70559156fa75e07e578`

`RegistryValidatedRequesterRendezvousStart` proves only point-in-time current-registry eligibility.

Its source contract explicitly excludes policy authorization, requester/rendezvous provider registration authority, transport readiness, live-owner authority, candidate-publication authority, and lease/currentness guarantees.

Point-in-time eligibility is necessary evidence for its own operation but is not an instruction to begin a remote admission attempt.

The registry remains an authority source for current device state; it is not silently reclassified as a scheduling decision owner.

## 10. Requester/rendezvous policy admission is not the binding

Exact source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`

Exact NF blob:

`b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`

The dedicated policy stage evaluates exactly `Capability::RequesterRendezvousStart`.

Its owned success carrier is explicitly not requester/rendezvous provider registration authority, transport readiness, live-owner authority, candidate-publication authority, a lease/currentness guarantee, or network reachability.

The evaluator must already be bound by the caller to the same authenticated requester principal; the function does not perform that binding.

Nothing in this source extends the requester/rendezvous-start capability into authority to enqueue a remote-session expected-device admission request.

Doing so would be a new cross-operation authority rule, not reuse of an existing binding.

## 11. Candidate-publication and requester/rendezvous provider authority remain operation-scoped

NF already established that provider registration and later candidate-publication authorization are bounded to requester/rendezvous and candidate-publication semantics.

C03e-NG preserves that scope.

A requester/rendezvous provider record, an authorized candidate-publication attempt, publication completion, a freshness token, endpoint state, relay state, or reachability observation may not be interpreted as permission to schedule a pre-authentication remote session unless a separately selected authority binding explicitly says so.

No such cross-operation binding exists in the exact post-NF source.

## 12. Remote expected-device request is a consumer carrier, not authority

Exact source:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Exact NF blob:

`ef370ca500f118bc067097ddb8f5c37ab597b214`

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>` owns:

- `expected_device_id: DeviceId`;
- `session_id: SessionId`;
- `authentication_request_id: u64`;
- dispatcher custody;
- verifier-time provider custody.

Its constructor accepts all of those values from the caller.

Its accessor contract states that `expected_device_id` is the pre-authentication logical `DeviceId` used only for scheduling the AJ attempt.

This describes how the receiver consumes already-produced scheduling intent. It does not identify who was authorized to create that intent.

Possession of the carrier is therefore not proof of authority provenance.

## 13. Receiver-side duplicate rejection is not producer authorization

The repeated real-admission supervisor rejects an expected-device request with `DuplicateActiveDevice` when one active authenticated worker already owns the same logical `DeviceId`.

That is a receiver-side safety rule.

It does not establish that any caller capable of constructing or sending a request was authorized to schedule it.

A later producer-side decision owner must classify duplicate/concurrent scheduling before enqueueing without weakening this receiver-side defense.

## 14. Linux production remote composition remains receiver-only

Exact source:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact NF blob:

`709fdbd749fdd63886701fdcc86f32b07110d19e`

`LinuxAgentRemoteProcessOperationInputs<...>` retains:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

as an injected production input.

The exact source imports Tokio `mpsc`, but production composition does not materialize an authoritative expected-request sender/channel owner.

Observed `mpsc::channel::<TestExpectedRequest>(...)` construction in this file is test-only scaffolding for dormant composition/signature tests and is not production authority provenance.

The receiver side therefore cannot be used backward to infer a sender-side decision owner.

## 15. Configured production peer identity is not the binding

The existing `PRW_REMOTE_PEER_DEVICE_ID` configuration path is explicitly process peer intent whose current same-device transport authority remains separately registry-derived.

It is not a stream of bounded admission decisions.

One configured peer cannot be treated as authority for arbitrary one-shot, repeated, reconnect, retry, replacement, or replay scheduling.

Configuration is not authorization.

## 16. Post-authenticated identity is too late to originate pre-auth intent

The repeated-admission lane deliberately distinguishes:

- pre-auth `expected_device_id`, used only to schedule/verify the AJ attempt;
- post-auth `session_owner.logical_device_id()`, authoritative for active-worker identity after successful authentication.

Post-authenticated identity is authoritative for the authenticated session that actually exists.

It cannot be used retroactively as the source that authorized the pre-auth scheduling attempt that had to exist before authentication began.

## 17. Correlation values remain non-authoritative

C03e-NG does not promote or bind authority through:

- `SessionId`;
- authentication PRWM `request_id`;
- candidate-publication request IDs;
- verifier timestamps;
- endpoint identifiers;
- transport identities;
- task IDs;
- worker IDs;
- channel identity or channel capacity.

Those values remain session/correlation/timing/transport/custody data under their existing contracts.

None is a principal or scheduling authorization decision by implication.

## 18. No authenticated principal/policy pair currently authorizes this cross-operation decision

The exact repository has authenticated-session custody and dedicated capability evaluation for protected operations.

However, C03e-NG finds no existing policy capability whose established meaning is:

> this authenticated principal may schedule one pre-authentication expected-device admission attempt for this exact target logical `DeviceId`.

`RequesterRendezvousStart` is narrower and operation-specific.

Existing remote capability authorization occurs after authenticated remote-session establishment and therefore cannot originate the same session's pre-auth scheduling intent.

No generic policy result may be widened by interpretation to fill this missing authority contract.

## 19. Fail-closed authority conclusion

The exact repository contains several legitimate authorities, but they are not interchangeable:

- local Agent lifecycle authority;
- reachability live-owner authority;
- authenticated requester identity;
- requester/rendezvous current-registry eligibility;
- requester/rendezvous policy authorization;
- requester/rendezvous provider custody;
- candidate-publication authorization;
- current registry authority;
- current transport identity;
- remote-session authentication authority;
- post-authenticated capability authorization.

None currently owns the missing decision to schedule one pre-auth expected-device admission attempt.

Shared custody of a `DeviceId` does not create such authority.

## 20. Selected blocking boundary

C03e-NG selects the next missing prerequisite as:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_OWNER_LIFECYCLE`

A later separately gated documentation-only checkpoint may audit/select only the production decision owner/lifecycle whose existing or explicitly selected responsibility is to decide whether one exact logical `DeviceId` should receive one bounded expected-device admission scheduling event.

That successor must determine, before any event/source/sender materialization:

1. whether the decision is requester-driven, lifecycle-driven, control-plane-driven, or another already-authoritative production operation;
2. the exact authenticated principal/session or non-user lifecycle authority under which the decision is made;
3. the exact policy/capability rule, if a principal-driven decision is selected;
4. the authoritative source of the target logical `DeviceId` within that operation;
5. the current-registry/revocation commit point;
6. whether success authorizes exactly one consumable scheduling decision or another explicitly bounded cardinality;
7. cancellation/invalidation semantics when authority changes before enqueue;
8. duplicate/concurrent decision classification;
9. the proof that the selected owner does not broaden requester/rendezvous, candidate-publication, reachability, transport, or post-auth authorities.

The successor must not yet create the event carrier, sender/channel, `SessionId`, authentication request ID, timing provider, dispatcher caller, listener/runtime activation, or retry/reconnect semantics.

## 21. Exact source guards

C03e-NG was audited against these exact NF source blobs:

- `crates/prw-agent/src/linux_bootstrap.rs` — `709fdbd749fdd63886701fdcc86f32b07110d19e`;
- `crates/prw-agent/src/linux_one_shot_scheduler.rs` — `ff7b890133c942b9239fc5262c64a64803a6177f`;
- `crates/prw-agent/src/linux_production_lifecycle.rs` — `c8a81b7c57dcfc0c9c28d7c37d43f6f3f5a4c5a4`;
- `crates/prw-agent/src/reachability_authority_admission.rs` — `f347027b6fe77c47ca3a647c2969a81fc7305565`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs` — `5f616f20699d1c7069f5aa8973200a0359c19cde`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs` — `1c021bc95a3d674722bfd70559156fa75e07e578`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs` — `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` — `ef370ca500f118bc067097ddb8f5c37ab597b214`.

Any stale-write mismatch in these or in the predecessor branch head invalidates this selection and requires a fresh audit.

## 22. Exact NG scope

C03e-NG is documentation-only.

Permitted repository delta:

- exactly this one contract path.

Expected delta classes:

- zero Rust source changes;
- zero Kotlin/Android changes;
- zero Cargo manifest/lockfile changes;
- zero workflow changes;
- zero systemd/package/security/configuration changes;
- zero runtime/listener/network activation changes.

## 23. Explicit exclusions

C03e-NG does not perform or authorize:

- Rust source materialization;
- a new scheduling decision owner implementation;
- a new policy capability;
- policy redesign;
- requester/rendezvous semantic widening;
- provider redesign;
- candidate-publication semantic widening;
- reachability authority semantic widening;
- a scheduling-event carrier;
- expected-request channel construction;
- sender ownership, sender cloning, or send behavior;
- `SessionId` generation;
- authentication PRWM request-ID allocation/reuse;
- verifier-time construction;
- admission timing policy;
- dispatcher caller migration;
- worker/repeated-admission behavior change;
- peer or transport selection/re-selection;
- candidate/reachability mutation;
- direct dialing;
- retry/reconnect/rebind/rebootstrap/replacement/replay;
- listener/readiness activation;
- `run()` or `main.rs` invocation changes;
- systemd/service/package mutation;
- credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- repository visibility/configuration mutation;
- deployment/restart/recovery activation;
- merge;
- PR close or ready-for-review conversion;
- branch deletion;
- history rewrite or force update;
- destructive cleanup of historical branches/evidence.

## 24. Identity and authorization invariants

Continue to preserve:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Logical device identity is not a static IP.

Transport-key rotation does not redefine logical device identity.

A `TransportIdentity` is transport evidence only.

An endpoint/IP/port is transient reachability only.

A PRWM `request_id` is correlation only.

A successful decode, registry lookup, provider lookup, reachability bootstrap, candidate publication, transport accept, or correlation match is not silently upgraded into scheduling authorization.

Receiving devices remain enforcement points for protected remote operations.

## 25. Validation and closure requirements

C03e-NG may be closed only after all of the following are verified on the exact final NG head:

1. exact NF -> NG topology is ahead-only from exact NF head;
2. exactly one changed path exists and it is this contract;
3. no Rust/runtime/manifest/lockfile/workflow/Android/packaging/security path changed;
4. canonical Rust validation for the exact final NG head completes successfully;
5. path-filtered skipped workflows are recorded as `SKIPPED`, never PASS;
6. no Android PASS is claimed unless an exact-head Android workflow actually runs and succeeds;
7. immutable Google Drive audit evidence is published and byte-exact readback verified;
8. post-evidence branch/PR reads confirm the exact final head is unchanged;
9. the PR remains draft/open/unmerged.

## 26. STOP boundary

After C03e-NG durable closure, STOP.

Do not create a scheduling decision owner, authority binding, event carrier, sender, or channel until the separately gated `PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_OWNER_LIFECYCLE` checkpoint has been explicitly selected from fresh exact-head evidence.
