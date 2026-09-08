# Phase 152 C03e-NI — Production expected-device scheduling decision origin operation selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NI_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_ORIGIN_OPERATION_SELECTED`

## 1. Purpose

C03e-NH closed the expected-device scheduling decision owner/lifecycle audit fail-closed.

NH established that the exact production repository does not already prove a production lifecycle or operation that owns the producer-side decision to schedule one bounded pre-authentication expected-device admission attempt.

NH selected the next blocking prerequisite:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_ORIGIN_OPERATION_SELECTION`

C03e-NI audits only that prerequisite.

This checkpoint asks one bounded question:

> Which already-represented production operation class is the correct semantic origin for the decision that one exact logical target `DeviceId` should receive one bounded pre-authentication expected-device admission scheduling attempt, without treating selection of that origin as scheduling authorization itself?

C03e-NI selects an operation class only. It does not authorize the scheduling side effect, create a new capability, widen an existing capability, mint an authority grant, create an event/request/channel/sender, allocate session or correlation identifiers, activate a caller, or deploy anything.

## 2. Exact predecessor

Canonical repository:

`Gersi365/prw-executor-private`

Repository ID:

`1334911207`

Predecessor branch:

`phase-152-c03e-nh-production-expected-device-scheduling-decision-owner-lifecycle-selection`

Exact predecessor head:

`7d54c3a7d41a29de09dbd073fb1c0b8bf65e9543`

Exact predecessor tree:

`e6a7453e8185853b51158b287732bb1e1baefa90`

NH selection:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_DECISION_OWNER_LIFECYCLE_PROVEN / REQUESTER_RENDEZVOUS_AND_PROVIDER_LIFECYCLES_REMAIN_OPERATION_SCOPED / REAL_ADMISSION_AND_REPEATED_SUPERVISOR_REMAIN_CONSUMER_SIDE / SOURCE_MATERIALIZATION_BLOCKED`

## 3. Selection

C03e-NI selects:

`AUTHENTICATED_REQUESTER_RENDEZVOUS_START_SELECTED_AS_EXPECTED_DEVICE_SCHEDULING_DECISION_ORIGIN_OPERATION_CLASS / TARGET_DEVICE_ID_PROVEN_BY_REQUESTER_RENDEZVOUS_CURRENT_REGISTRY_CHAIN / EXISTING_REQUESTER_RENDEZVOUS_START_CAPABILITY_DOES_NOT_YET_AUTHORIZE_ADMISSION_SCHEDULING / SOURCE_MATERIALIZATION_BLOCKED`

Meaning:

- the semantic origin operation class is the authenticated requester-driven requester/rendezvous-start operation;
- the selected target logical `DeviceId` is the exact target carried through the requester/rendezvous current-registry validation chain;
- this selection identifies causal operation ownership only;
- the existing `Capability::RequesterRendezvousStart` authorization remains scoped to the requester/rendezvous-start operation already defined by exact source;
- C03e-NI does not treat that capability as authorization to create an expected-device admission request;
- no scheduling producer, event, authority carrier or runtime caller is materialized;
- source materialization remains blocked.

## 4. Origin selection is not authority selection

C03e-NI explicitly separates two concepts.

The **origin operation class** answers:

> Which authenticated operation expresses the user/server intent that can causally require one target-side expected-device admission attempt?

The **scheduling authority scope** answers:

> What exact policy/authority rule permits that operation to cause one bounded pre-auth expected-device admission scheduling decision for the target?

C03e-NI answers only the first question.

The second remains blocked and is selected as the next prerequisite.

## 5. Exact source audit

C03e-NI re-audited the exact NH head and relies on these exact source blobs:

- `crates/prw-policy/src/lib.rs`
  - blob `3056b53e81c4429314d9f890dcf2bf3e80d433b8`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`
  - blob `5f616f20699d1c7069f5aa8973200a0359c19cde`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`
  - blob `1c021bc95a3d674722bfd70559156fa75e07e578`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`
  - blob `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`
  - blob `f7377011a3ab2034c14d9018a5c0f268f6660ffa`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
  - blob `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
  - blob `082a70af239972a82318f3e17cb3fd8cb45d9e95`
- `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs`
  - blob `260024b7aca2aea6109dc72e778bcda3dcca8038`
- `crates/prw-remote-bridge/src/lib.rs`
  - blob `ad6833cc4e71a372810b260f157126a3df6645e5`
- `contracts/END_TO_END_AUTHENTICATED_CAPABILITY_BRIDGE_CONTRACT.md`
  - blob `1466aa61549b7512b39a847af4f61e4cd047204b`
- `crates/prw-agent/src/linux_bootstrap.rs`
  - blob `709fdbd749fdd63886701fdcc86f32b07110d19e`
- `apps/android/app/src/main/kotlin/com/privateworkspace/prw/DeviceManagement.kt`
  - blob `028b7dc2d4234adac772f9eb2e7d6a2ee02353c3`

## 6. Why requester/rendezvous start is the selected semantic origin

`Capability::RequesterRendezvousStart` is defined as:

> Begin requester-side rendezvous toward one registry-validated logical target.

That is the only represented operation capability whose exact semantic already combines:

- a requester-side initiation act;
- one explicit logical target;
- a rendezvous/connection-establishment purpose.

The target is not derived from IP, endpoint, transport state, configured peer state, candidate state, or worker state.

The target begins as requester-nominated logical intent and is then preserved through the current-registry validation chain.

No other existing operation class has this exact connection-initiation semantic.

## 7. Authenticated requester provenance

`RequesterRendezvousStartIntent` packages:

- one already-authenticated server-held requester session; and
- one nominated target `DeviceId`.

Construction alone is non-authoritative.

The exact source explicitly states that construction performs ownership composition only and does not perform registry validation, policy authorization, provider mutation, transport authority or I/O.

This makes the requester session the correct principal provenance for the operation without prematurely treating possession of the start-intent carrier as scheduling authority.

## 8. Exact target DeviceId provenance

`validate_current_requester_rendezvous_start_intent` validates the requester/target pair in a fixed fail-closed order:

1. requester authenticated-session currentness;
2. exact target lookup;
3. target device lifecycle;
4. target membership lifecycle;
5. same-workspace equality;
6. exact target identity preservation.

The resulting `RegistryValidatedRequesterRendezvousStart` retains exactly:

- the server-held authenticated requester session; and
- the exact logical target `DeviceId` that survived those checks.

This is the strongest existing pre-provider target provenance for the requester/rendezvous operation.

It remains point-in-time registry eligibility, not perpetual currentness and not scheduling authority.

## 9. Dedicated requester policy binding

The requester/rendezvous policy source resolves an evaluator using the exact authenticated requester dimensions.

Its concrete dedicated policy allows only `Capability::RequesterRendezvousStart` when configured to allow; every other capability is denied.

The policy admission stage evaluates exactly `Capability::RequesterRendezvousStart` once.

This proves that the requester/rendezvous operation already has a dedicated principal-bound policy boundary.

It does not prove that the same capability authorizes expected-device admission scheduling.

## 10. Requester/rendezvous composition confirms operation identity

`validate_authorize_and_register_requester_rendezvous_start` executes the established requester operation in fixed fail-closed order:

1. current-registry validation;
2. requester-bound policy-source resolution;
3. exact requester/rendezvous-start capability authorization;
4. requester/rendezvous provider registration.

The composition explicitly introduces no fallback evaluator, direct provider bypass, retry or second registration path.

This is the exact existing operation chain that C03e-NI selects as the semantic origin class.

C03e-NI does not add a fifth scheduling stage to that function.

## 11. Server-side requester/rendezvous state preserves the selected target relationship

The requester/rendezvous runtime owner consumes the policy-authorized requester/rendezvous start and registers:

- the authenticated requester session; and
- the validated target `DeviceId`.

Later, candidate publication can authorize one current server-side requester/rendezvous selection for the authenticated publisher device.

The resulting `AuthorizedRequesterRendezvous` carries:

- the authenticated requester session; and
- the expected publisher logical `DeviceId`.

The one-shot grant is operation evidence for one candidate-publication attempt only.

This server-side chain demonstrates that requester/rendezvous start is already the operation that establishes the requester-to-target rendezvous relationship.

It still does not create expected-device admission scheduling authority.

## 12. Why `DeviceManage` is not selected

`Capability::DeviceManage` exists as a represented policy-domain value, but representation does not imply implementation or authorization.

The exact policy source states that the capability model is intentionally narrow and that a capability is not implemented merely because it is represented.

The exact authenticated capability bridge exposes 18 typed operations and does not expose `DeviceManage`.

The Phase 143 bridge contract explicitly states that `DeviceManage` is not remotely exposed by the initial bridge.

The bounded local management policy also denies `DeviceManage`.

Therefore `DeviceManage` supplies neither:

- an executable production operation;
- a typed request semantic for connection initiation;
- a target rendezvous operation;
- scheduling authority.

A broad name such as “manage a device” cannot be promoted into expected-device scheduling authority by implication.

## 13. Android device-management UI is not the origin operation

The Android `DeviceManagementController` owns local presentation state and pending revocation intent.

Its `requestRevocation` method only marks one enrolled device as pending revocation in local controller state.

It does not create an authenticated server-side device-management command, rendezvous start, expected-device scheduling decision, remote admission request, channel send, or production authority grant.

Therefore Android device-management intent is not selected as the scheduling origin operation.

## 14. Generic authenticated capability bridge is not the origin

The existing Phase 143 `BridgeCommand` registry exposes status, file, terminal and forwarding operations only.

It does not expose requester/rendezvous start, device management, expected-device scheduling, remote-session admission creation or a generic arbitrary command.

The capability bridge authorizes operations only after a remote session already exists and has passed application-session, registry, transport and policy gates.

That timing is also too late to originate the same pre-auth expected-device admission scheduling decision.

Therefore the generic remote capability bridge is not selected as the origin operation.

## 15. Production remote-process lifecycle is not the origin

`linux_bootstrap.rs` retains requester/rendezvous policy/runtime custody alongside the remote-process lifecycle, but the remote-process path still consumes an injected expected-request receiver.

The requester/rendezvous custody joins remain dormant/separately gated composition seams and do not create a scheduling producer.

There is no production rule in exact source saying to schedule an expected-device admission attempt on:

- process start;
- listener bind;
- configured peer presence;
- reachability success;
- candidate publication;
- worker capacity;
- worker completion;
- retry timer;
- reconnect/recovery.

Therefore no autonomous lifecycle operation is selected.

## 16. Configured peer is not an operation

`PRW_REMOTE_PEER_DEVICE_ID` is process configuration/peer intent.

A configured target value does not identify an authenticated initiating principal, operation instance, authorization decision, cardinality rule, cancellation rule or current-registry commit point.

Configuration presence is not an operation and is not selected as the origin.

## 17. Candidate publication is downstream, not the origin

Candidate publication consumes an already-established current requester/rendezvous authority relationship for the authenticated publisher.

Its one-shot grant authorizes one publication attempt.

Candidate publication therefore confirms downstream rendezvous execution, but does not replace the requester operation that established the relationship.

Publication success, candidate presence or reachability success is not promoted into admission scheduling authority.

## 18. Real admission and repeated admission remain downstream consumers

Real admission consumes an already-selected intended target logical identity and validates current registry/transport/authenticated identity state.

Repeated admission consumes `RemoteSessionExpectedDeviceAdmissionRequest` values from an injected receiver and enforces receiver-side worker safety.

Neither runtime defines the initiating user/server operation that requested the connection attempt.

Neither is selected as the origin operation.

## 19. Selected operation class remains dormant until later gates

Selecting requester/rendezvous start as the semantic origin does not claim that a production wire caller or executable invocation is already active.

Exact source still contains separately gated/dormant caller and custody seams.

C03e-NI authorizes no caller migration or runtime activation.

A later source-materialization checkpoint must still prove the exact production caller and ordering after authority scope, event provenance, channel ownership and request construction prerequisites are closed.

## 20. Scheduling authority remains unproven

The existing requester/rendezvous-start capability authorizes the existing operation only.

C03e-NI does not reinterpret policy success as permission to:

- create one `RemoteSessionExpectedDeviceAdmissionRequest`;
- schedule a new pre-auth remote-session admission;
- create or send on a production expected-request channel;
- allocate a `SessionId`;
- allocate/reuse an authentication PRWM request identifier;
- select verifier/admission timing;
- bypass a later current-registry/revocation check;
- retry/replay/reconnect automatically.

Doing any of those would be a new authority consequence and remains blocked.

## 21. Producer currentness remains separately gated

The requester/rendezvous registry-validation carrier is point-in-time evidence only.

A future scheduling authority design must select the exact producer-side currentness/revocation commit point.

It must decide whether scheduling requires a fresh current-registry revalidation after requester/rendezvous registration and immediately before scheduling authorization/event creation.

C03e-NI does not cache the earlier validation as perpetual authority.

## 22. Producer cardinality remains separately gated

The existing requester/rendezvous provider has bounded lifecycle and one-shot candidate-publication grants, but those semantics do not automatically define expected-device scheduling cardinality.

A future authority scope must state whether one authorized requester/rendezvous start permits:

- zero or one scheduling decision;
- exactly one decision after a specified commit point;
- a separately authorized bounded retry family;
- no retries.

C03e-NI selects none of those behaviors.

## 23. Cancellation and invalidation remain separately gated

The future scheduling authority scope must explicitly define invalidation on relevant changes, including as applicable:

- requester session invalidation;
- requester membership suspension/removal;
- target revocation;
- target membership suspension/removal;
- workspace relationship change;
- policy denial/change;
- requester/rendezvous retirement/removal;
- explicit cancellation;
- process shutdown;
- supersession;
- selected expiry.

C03e-NI invents no cancellation policy.

## 24. Duplicate/concurrent scheduling remains separately gated

Receiver-side `DuplicateActiveDevice` does not classify concurrent producer decisions before an active worker exists.

A future authority scope must explicitly select producer-side behavior such as reject, coalesce, serialize or another bounded rule.

No such behavior is inferred from requester/rendezvous provider capacity or one-shot candidate-publication grants.

## 25. No authority-domain widening

C03e-NI does not widen:

- requester/rendezvous-start authorization;
- candidate-publication authority;
- reachability authority;
- transport authority;
- registry eligibility evidence;
- `DeviceManage`;
- post-authenticated remote capability authority;
- configured-peer intent;
- receiver-side duplicate protection.

Selecting an operation class is not a grant.

## 26. Selected next blocking prerequisite

C03e-NI selects:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_SCOPE_SELECTION`

The next checkpoint must remain documentation-only.

It must determine the exact authority contract by which one authenticated requester/rendezvous-start operation may, or may not, authorize one bounded expected-device scheduling decision for its exact registry-validated target.

That checkpoint must explicitly compare at least:

1. a dedicated new scheduling capability evaluated for the same authenticated requester;
2. an explicit, separately approved semantic extension of `RequesterRendezvousStart` limited to one bounded scheduling consequence;
3. a derived one-shot scheduling authority grant minted only after exact requester/target currentness and policy gates;
4. fail-closed rejection if no authority form can preserve operation separation without widening existing grants.

It must select:

- exact policy/capability semantics;
- exact principal binding;
- exact target `DeviceId` provenance;
- exact current-registry/revocation commit point;
- one-shot/cardinality semantics;
- cancellation/invalidation semantics;
- duplicate/concurrent producer classification;
- authority lifetime/consumption semantics;
- proof that candidate-publication, reachability, transport, configured-peer, generic device-management and post-authenticated capability authority remain separate.

The next checkpoint must not yet create:

- a Rust capability variant;
- a policy-source implementation;
- a scheduling authority carrier;
- a scheduling event carrier;
- `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- an `mpsc` sender/channel;
- `SessionId` generation;
- authentication PRWM request-ID allocation/reuse;
- verifier/admission timing;
- runtime caller activation;
- retry/reconnect behavior;
- deployment.

## 27. Validation requirement

Closure of C03e-NI requires exact-final-head validation tied to the immutable NI commit.

Expected path-filtered workflow conclusions must be recorded exactly; `SKIPPED` must not be represented as PASS.

No Android PASS may be claimed unless an exact-NI-head Android workflow actually runs and succeeds.

## 28. Durable evidence requirement

Before closure, one immutable text audit must be published under the canonical Google Drive evidence parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

Required publication procedure:

1. exact-title pre-upload search returns zero canonical matches;
2. freeze local audit bytes;
3. compute exact byte length and SHA-256;
4. upload as raw `text/plain` without conversion;
5. read metadata before any move;
6. move only from the verified source parent to the canonical parent if required;
7. raw Drive readback;
8. recompute exact byte length and SHA-256 from readback;
9. require exact local/readback match;
10. exact-title post-upload search returns exactly one canonical artifact;
11. re-read final branch and PR state before closure metadata update.

## 29. Explicit exclusions

C03e-NI performs or authorizes none of:

- Rust/source/runtime materialization;
- new capability creation;
- existing capability widening;
- scheduling authority implementation;
- scheduling decision/event carrier creation;
- production expected-request construction;
- production channel/sender creation;
- sender clone/send behavior;
- `SessionId` generation;
- authentication PRWM request-ID allocation/reuse;
- verifier-time construction;
- admission timing policy;
- dispatcher migration;
- caller migration;
- listener/readiness activation;
- candidate/reachability-triggered scheduling;
- retry/reconnect/rebind/rebootstrap/replay;
- main.rs wiring;
- systemd/service/package mutation;
- credentials/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- repository visibility/configuration mutation;
- deployment/restart/recovery activation;
- merge;
- PR close;
- ready-for-review conversion;
- branch deletion;
- history rewrite/force update.

## 30. STOP boundary

After selection, exact-final-head validation, immutable evidence publication, closure metadata update and final readback, STOP.

Do not proceed into `PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_SCOPE_SELECTION` in the same checkpoint.
