# Phase 152 C03e-NJ — Production requester/rendezvous expected-device scheduling authority scope selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NJ_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_SCOPE_SELECTED`

## 1. Purpose

Closed C03e-NI selected the authenticated requester-driven requester/rendezvous-start operation as the semantic origin operation class for one bounded expected-device scheduling decision while explicitly leaving scheduling authorization unproven.

NI selected the next blocking prerequisite:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_SCOPE_SELECTION`

C03e-NJ audits only that prerequisite.

This checkpoint asks one bounded question:

> What exact authority contract may permit one authenticated requester/rendezvous-start operation to derive one bounded pre-authentication expected-device scheduling decision for its exact registry-validated logical target without turning a raw policy `Allow`, a provider record, candidate-publication authority, reachability authority, transport state, or post-authenticated capability authority into a reusable scheduler grant?

C03e-NJ selects authority semantics only.

It does not create a Rust capability variant, policy implementation, scheduling-authority carrier, one-shot state owner, event/request carrier, sender/channel, `SessionId`, authentication PRWM request identifier, timing source, runtime caller, retry path, listener activation, or deployment.

## 2. Exact predecessor

Canonical repository:

`Gersi365/prw-executor-private`

Repository ID:

`1334911207`

Predecessor branch:

`phase-152-c03e-ni-production-expected-device-scheduling-decision-origin-operation-selection`

Exact predecessor head:

`0cff9617e0eee2fcacd00f51ff8fdeb2d6031dc9`

Exact predecessor tree:

`bfe8290951da85871e334fd310e377cd211a1664`

NI selection:

`AUTHENTICATED_REQUESTER_RENDEZVOUS_START_SELECTED_AS_EXPECTED_DEVICE_SCHEDULING_DECISION_ORIGIN_OPERATION_CLASS / TARGET_DEVICE_ID_PROVEN_BY_REQUESTER_RENDEZVOUS_CURRENT_REGISTRY_CHAIN / EXISTING_REQUESTER_RENDEZVOUS_START_CAPABILITY_DOES_NOT_YET_AUTHORIZE_ADMISSION_SCHEDULING / SOURCE_MATERIALIZATION_BLOCKED`

## 3. C03e-NJ closed selection

C03e-NJ selects:

`REQUESTER_RENDEZVOUS_START_BOUNDED_SCHEDULING_CONSEQUENCE_SELECTED / DISTINCT_DERIVED_ONE_SHOT_SCHEDULING_AUTHORITY_GRANT_MANDATORY / FRESH_CURRENT_REQUESTER_TARGET_POLICY_AND_PROVIDER_STATE_REQUIRED / DIRECT_CAPABILITY_ALLOW_OR_CANDIDATE_PUBLICATION_GRANT_REUSE_FORBIDDEN / SOURCE_MATERIALIZATION_BLOCKED`

Meaning:

- no new independently grantable user-facing scheduling capability is selected;
- `Capability::RequesterRendezvousStart` receives one explicit, separately gated semantic extension: a successful requester/rendezvous-start operation may authorize derivation of at most one bounded expected-device scheduling authority for the exact operation target;
- raw `Decision::Allow` for `RequesterRendezvousStart` is never sufficient input to a scheduler;
- the scheduling authority must be represented by a distinct owned one-shot grant derived only after fresh requester/target currentness, exact requester policy reauthorization, and exact current requester/rendezvous provider-state validation;
- candidate-publication `AuthorizedRequesterRendezvous` remains candidate-publication scoped and must not be reused as the scheduling grant;
- provider registration success remains insufficient by itself;
- no scheduling source/runtime materialization is authorized by C03e-NJ.

## 4. Why authority scope is separate from operation policy

The exact `Capability` model defines `RequesterRendezvousStart` as:

`Begin requester-side rendezvous toward one registry-validated logical target.`

That capability already names the operation selected by NI.

However the existing policy source and admission path establish only that the authenticated requester may perform the requester/rendezvous-start operation.

Existing source does not state that the resulting `Decision::Allow` object, evaluator, registry-validated carrier, or policy-authorized carrier may be copied, retained, replayed, or handed directly to the repeated-admission scheduler.

C03e-NJ therefore distinguishes:

- **operation policy** — whether this authenticated requester may begin requester/rendezvous toward this logical target; and
- **scheduling authority** — one consumable server-side authority to cause at most one bounded expected-device scheduling consequence of that exact operation.

The former is existing policy semantics.

The latter is selected here as a distinct derived authority form.

## 5. Exact source audit

C03e-NJ re-audits the exact NI head, including these exact source blobs:

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
- `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`
  - blob `d01cfbc37433f6099e216397b9bf243aa55c53bc`
- `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`
  - blob `d550ec8d5aa18ed6885ebed42c52ee742498e9d2`
- `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
  - blob `60307fff4dd0fd573192ba6e6fab9dedd3321dda`
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`
  - blob `812b56e9b948a41f2f746eb406ba24567efbd528`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
  - blob `ef370ca500f118bc067097ddb8f5c37ab597b214`
- predecessor NI contract
  - blob `3720de5697367ea29155071f5d2f816723696e16`

## 6. Alternative A — dedicated new scheduling capability

C03e-NJ considered a new independently represented capability such as a dedicated requester expected-device scheduling permission.

This option is not selected.

Reasons:

1. exact source already has a dedicated operation capability whose semantic is to begin requester-side rendezvous toward one registry-validated logical target;
2. expected-device scheduling is selected by NI only as one bounded internal consequence of that same operation, not as a separately exposed user operation;
3. a second independently grantable capability would create a two-policy partial state in which requester/rendezvous start could be allowed while its required bounded scheduling consequence is separately denied, or vice versa;
4. no separate scheduling wire command, client operation, target-management operation, or current policy-source backing exists;
5. introducing a new capability would require a new policy-model mutation before evidence demonstrates that a distinct user-facing permission boundary is necessary.

C03e-NJ therefore does not select a new `Capability` variant.

This does not mean scheduling is authorized by raw `RequesterRendezvousStart` policy success.

## 7. Alternative B — direct semantic widening of `RequesterRendezvousStart`

C03e-NJ considered treating successful `RequesterRendezvousStart` policy authorization itself as direct scheduler authority.

This option is rejected.

A raw `Decision::Allow` or `PolicyAuthorizedRequesterRendezvousStart` does not carry:

- one-shot consumption state;
- current requester/rendezvous provider uniqueness at scheduling time;
- a fresh current-registry commit point after provider registration;
- producer duplicate/concurrency classification;
- scheduling consumption/disposition state;
- protection against replay by a later caller.

Passing raw policy success directly to expected-request construction would therefore widen the existing policy carrier into a reusable cross-operation grant.

C03e-NJ forbids that shape.

## 8. Alternative C — explicit bounded semantic extension plus derived one-shot grant

This option is selected.

The separately approved semantic extension is narrow:

> One successfully authorized requester/rendezvous-start operation may cause derivation of at most one server-side expected-device scheduling authority for the exact same logical target, provided all selected fresh authority gates still succeed at the scheduling-authority commit point.

The extension authorizes only **derivation**.

It does not authorize direct request construction, channel send, session-ID generation, authentication request-ID generation, timing selection, retry, replay, reconnect, or capability execution.

A distinct derived one-shot scheduling grant is mandatory between policy authorization and any future scheduling event/request construction.

## 9. Alternative D — global fail-closed rejection

C03e-NJ retains fail-closed behavior for every missing or ambiguous gate, but does not select permanent rejection of the scheduling feature.

The combination selected in sections 8–18 preserves authority separation without adding a new independently grantable capability:

- exact authenticated requester principal;
- exact target provenance;
- current requester/rendezvous provider state;
- fresh current registry;
- exact requester policy reauthorization;
- distinct one-shot grant;
- explicit producer duplicate behavior;
- no direct reuse of other authority domains.

Therefore a bounded authority contract can be selected while source materialization remains separately gated.

## 10. Exact principal binding

The scheduling authority principal is the exact already-authenticated requester application session belonging to the selected requester/rendezvous operation.

It must retain the existing authenticated dimensions:

- requester `SessionId`;
- requester logical `DeviceId`;
- requester `WorkspaceId`;
- requester `UserId`.

No raw `DeviceId`, PRWM request identifier, candidate-publication request identifier, transport identity, endpoint, IP address, task ID, worker ID, or channel identity may substitute for the authenticated requester principal.

The derived scheduling authority may carry only the minimum requester provenance needed to prove identity and one-shot ownership; its exact Rust representation remains separately gated.

## 11. Exact target provenance

The scheduling target is the exact logical target `DeviceId` selected by closed C03e-NI.

That value originates as requester-nominated target intent but becomes eligible only through the existing requester/rendezvous current-registry validation chain.

C03e-NJ does not permit constructing scheduling authority from:

- `PRW_REMOTE_PEER_DEVICE_ID` process configuration;
- candidate endpoint data;
- reachability state;
- current `TransportIdentity`;
- IP/port/path state;
- a post-authenticated remote-session owner;
- the `expected_device_id` field of a request that has already been constructed.

The future derived authority must preserve the exact target logical identity without normalization, reinterpretation, endpoint substitution, or transport-key substitution.

## 12. Current requester/rendezvous provider-state requirement

The current in-memory requester/rendezvous provider stores records keyed by exact requester session plus expected publisher/target logical device.

Exact source permits multiple distinct requester records for one target and classifies more than one current matching record as `Ambiguous` when current publisher authority is selected.

Exact source also classifies:

- no matching record as `Missing`;
- retired-only matching authority as `StaleOrRetired`;
- more than one current match as `Ambiguous`.

C03e-NJ selects the same fail-closed uniqueness law for scheduling-authority derivation:

- exactly one current requester/rendezvous relationship for the scheduling target must be deterministically attributable to the exact authenticated requester operation;
- missing, retired/stale, ambiguous, unavailable, or indeterminate provider authority fails before a scheduling grant can exist.

Provider registration success alone is not sufficient.

## 13. Candidate-publication grant must not be reused

`AuthorizedRequesterRendezvous` is explicitly scoped to one candidate-publication attempt.

The current provider authorization call is non-consuming and exact source proves that repeated calls can return multiple fresh candidate-publication grants while the provider record remains `Current`.

Therefore that type cannot be repurposed as scheduling authority.

Doing so would violate both:

- its exact candidate-publication scope; and
- the required one-shot scheduling cardinality.

A future scheduling-authority composition may use the underlying exact current provider relationship only through a separately selected scheduling-specific provenance/consumption seam.

It must not reinterpret or rename `AuthorizedRequesterRendezvous` into the scheduling grant.

## 14. Fresh current-registry commit point

The earlier requester/rendezvous start registry validation is point-in-time evidence only.

C03e-NJ selects a fresh scheduling-authority currentness commit point after requester/rendezvous registration and immediately before one scheduling authority is minted.

At that commit point the exact current registry must again establish, under one coherent current-authority read:

1. requester authenticated-session currentness;
2. exact requester logical identity preservation;
3. target existence;
4. target `DeviceLifecycle::Enrolled`;
5. target active membership;
6. requester and target same-workspace relationship;
7. exact target logical identity preservation;
8. no target revocation or requester currentness failure visible at that read.

Failure at any step produces no scheduling grant.

The existing AJ transaction will still perform its own later fresh current registry/transport reads during actual admission.

The producer-side check selected here does not replace AJ currentness.

## 15. Fresh requester policy reauthorization

C03e-NJ selects a second exact requester policy evaluation at scheduling-authority mint time.

The policy source must resolve the evaluator from the same authenticated requester dimensions.

The exact capability evaluated remains:

`Capability::RequesterRendezvousStart`

No new capability is selected.

`Decision::Deny`, unavailable policy, indeterminate policy, principal mismatch, or fallback-evaluator pressure fails before a scheduling grant exists.

The scheduling consequence is therefore tied to a currently valid requester/rendezvous-start policy decision rather than only to historical registration success.

The current bounded requester policy source is immutable after one-shot population, but the fresh evaluation rule is retained so future policy-source evolution cannot silently convert historical policy success into a perpetual scheduling lease.

## 16. Authority-lock ordering

The existing shared requester/rendezvous authority already establishes the lock order:

1. acquire requester/rendezvous authority custody;
2. while that custody remains held, acquire one shared-current registry/policy read;
3. execute the bounded synchronous authority composition;
4. release current-authority read;
5. release requester/rendezvous custody before response I/O.

C03e-NJ selects this existing ordering as the required ordering for future scheduling-authority derivation.

The future seam must not invert the locks.

It must keep provider-currentness selection, fresh registry checks, requester policy reauthorization, and one-shot mint-state decision within one bounded authority critical section or an equivalently proven linearization point.

No network I/O, channel send, remote peer accept, durable reachability commit, or response I/O may occur while those authority locks are held.

## 17. One-shot cardinality

One exact authenticated requester/rendezvous-start operation may authorize:

`ZERO_OR_ONE_EXPECTED_DEVICE_SCHEDULING_DECISION`

Zero is valid when:

- any fresh authority gate fails;
- the operation is cancelled before scheduling commitment;
- duplicate/concurrent scheduling state rejects the attempt;
- shutdown occurs before commitment;
- the future scheduling sink is unavailable before commitment.

At most one is valid after successful authority commitment.

There is no implicit retry family.

A second scheduling authority may not be minted for the same exact requester-session/target operation identity merely because the first downstream attempt failed.

A new attempt requires a new separately authenticated and authorized requester/rendezvous-start operation unless a later checkpoint explicitly selects a different bounded retry law.

## 18. One-shot authority state is mandatory

The existing requester/rendezvous provider record cannot by itself enforce the selected scheduling cardinality:

- `authorize_current_for_publisher(...)` is non-consuming;
- provider records remain `Current` until candidate-publication cleanup retires/removes them;
- retiring the existing record merely to prevent a second scheduling grant would mutate candidate-publication lifecycle semantics and is not authorized here.

Therefore a scheduling-specific one-shot state owner is required.

That future state must distinguish at least:

- not yet minted/eligible;
- one scheduling authority committed or consumed;
- terminally unavailable/ineligible as required by the selected lifecycle.

C03e-NJ selects the semantic requirement only and does not select storage representation, map key type, mutex ownership, persistence, TTL, or source location.

## 19. Duplicate and concurrent producer classification

C03e-NJ selects fail-closed producer behavior.

For the same exact requester-session/target operation identity:

- concurrent mint attempts must serialize at one authority linearization point;
- exactly one may commit;
- every later/concurrent duplicate is rejected;
- duplicates are not coalesced because coalescing could erase principal/operation provenance;
- duplicates are not queued as retries;
- duplicates do not replace an existing committed authority.

For one target with more than one current requester/rendezvous relationship where a unique operation cannot be established, authority derivation fails as ambiguous before mint.

Receiver-side `DuplicateActiveDevice` remains a separate downstream defense and does not replace producer one-shot enforcement.

## 20. Authority lifetime

The selected scheduling authority is intentionally short-lived and operation-bound.

No independent wall-clock TTL is selected by C03e-NJ.

The authority must not become a durable bearer token, cache entry transferable across operations, reconnect ticket, or replayable capability.

Its intended lifetime is:

1. fresh authority gates succeed;
2. one scheduling-specific one-shot state transition commits;
3. one distinct owned scheduling grant is produced;
4. that grant is moved exactly once into the future scheduling-event/request-construction boundary;
5. the grant is consumed/dropped and cannot be recovered for reuse.

If a later design requires queue residency, expiry, persistence, or crash recovery, that requires another explicit gate.

## 21. Cancellation and invalidation

Before authority commitment, all of the following fail closed or cancel derivation as applicable:

- requester authenticated-session invalidation;
- requester device revocation;
- requester membership suspension/removal;
- target revocation;
- target membership suspension/removal;
- requester/target workspace mismatch;
- requester policy denial/unavailability/indeterminacy;
- requester/rendezvous record missing, retired, stale, ambiguous, unavailable, or indeterminate;
- explicit operation cancellation;
- process/supervisor shutdown.

After one-shot authority commitment, no automatic remint is authorized.

A downstream scheduling-event construction/send failure consumes or terminally abandons that operation's authority according to the future one-shot state contract; it does not silently restore eligibility.

C03e-NJ does not select a rollback-to-eligible state.

## 22. No retry/replay/reconnect authority

The selected semantic extension authorizes no:

- automatic retry after channel-full/closed failure;
- reconnect loop;
- replay after process restart;
- remint after request rejection;
- remint after AJ authentication failure;
- remint after worker completion;
- replacement after receiver `DuplicateActiveDevice`;
- timer-driven requeue;
- candidate/reachability-triggered repeated scheduling.

Any future retry family requires a distinct bounded policy and lifecycle checkpoint.

## 23. Real admission remains independently fail-closed

`admit_expected_remote_device_session(...)` receives an already-selected `expected_device_id` and then independently:

- resolves current expected transport identity from current registry state;
- accepts only the matching lower-transport peer;
- performs a second fresh current-authority read before challenge preparation;
- authenticates the logical session;
- composes the post-authenticated runtime owner.

Therefore the derived scheduling grant does not grant transport identity, bypass registry currentness, bypass authentication, or grant post-authenticated capabilities.

The scheduling authority permits only one bounded pre-auth attempt to enter that existing fail-closed transaction.

## 24. Post-authenticated capability authority remains separate

After successful admission, protected remote capability execution still requires its existing authenticated-session/current-registry/transport/policy chain.

C03e-NJ does not grant:

- `AgentStatusRead`;
- terminal capability;
- file capability;
- forwarding capability;
- `DeviceManage`;
- `PolicyManage`;
- any provider-specific side effect.

Expected-device scheduling authority is pre-authentication attempt authority only.

## 25. Candidate publication remains separate

Candidate-publication authority continues to use its own one-shot `AuthorizedRequesterRendezvous` semantics and fresh currentness/reachability commit ordering.

C03e-NJ does not:

- treat publication success as scheduling permission;
- treat candidate presence as scheduling permission;
- reuse candidate-publication grant type;
- retire/remove the requester/rendezvous record as a scheduling side effect;
- change candidate-publication cleanup ordering.

The two operation consequences may share underlying requester/target provenance but remain distinct authority domains.

## 26. Reachability and transport remain separate

`TransportIdentity`, endpoint/IP/port, candidate data, path selection, reachability state, relay state, bind address, and current peer transport evidence do not become scheduling authorization.

They may be consumed later by their existing dedicated runtime/authority seams.

The scheduling grant carries logical authority only.

AJ continues to resolve current transport identity from the logical `DeviceId` at admission time.

## 27. Configured peer remains non-authoritative

`PRW_REMOTE_PEER_DEVICE_ID` remains process peer intent/configuration only.

It cannot create, refresh, duplicate, replace, or recover a scheduling grant.

A configured value matching the target does not bypass authenticated requester provenance, requester policy, provider uniqueness, or fresh registry validation.

## 28. `DeviceManage` remains unrelated

`Capability::DeviceManage` remains represented but not remotely exposed by the existing initial authenticated bridge.

C03e-NJ does not use it as a target-side scheduling permission and does not create an Android/server device-management scheduling path.

Local Android pending-revocation intent remains presentation/client state and cannot authorize admission scheduling.

## 29. Correlation/session/timing values remain non-authoritative

C03e-NJ selects no production source for:

- expected-request `SessionId`;
- authentication PRWM `request_id`;
- verifier time;
- challenge validity;
- authentication-now time;
- application-lease range;
- dispatcher construction.

Those values remain later composition prerequisites.

They do not identify the requester operation and cannot create scheduling authority.

## 30. Request object remains downstream

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>` contains:

- `expected_device_id`;
- `session_id`;
- `authentication_request_id`;
- dispatcher;
- verifier-time provider.

Its `expected_device_id` accessor is explicitly pre-authentication logical scheduling input.

C03e-NJ does not permit construction of this request from raw target identity alone.

A future request constructor must consume an already-authoritative one-shot scheduling consequence plus separately selected sources for all remaining request fields.

## 31. Sender/channel remains downstream

C03e-NJ does not select:

- `mpsc::channel(...)` creation;
- channel capacity;
- sender owner;
- sender clone policy;
- send ordering;
- backpressure behavior;
- closed-channel behavior;
- sender-drop shutdown ordering.

The sender may attach only after one-shot authority ownership and later event/request composition are separately closed.

## 32. Selected next blocking prerequisite

C03e-NJ selects:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_CONSUMPTION_STATE_OWNERSHIP_SELECTION`

The next checkpoint must remain documentation-only.

It must select where and how the mandatory scheduling-specific one-shot state is owned and linearized without mutating the existing candidate-publication provider lifecycle.

It must determine at minimum:

1. exact scheduling-operation identity key;
2. owner/custody location for eligible/committed/consumed state;
3. synchronization primitive or proof that an existing primitive safely owns the transition;
4. lock ordering relative to `SharedRequesterRendezvousAuthority` and `SharedCurrentCapabilityAuthority`;
5. atomic duplicate/concurrent classification;
6. whether state is created during requester/rendezvous registration or lazily at scheduling-authority derivation;
7. shutdown/drop semantics;
8. whether any post-commit scheduling failure can alter the state;
9. proof that candidate-publication record retirement/removal remains unchanged;
10. proof that no timer, persistence, retry or replay is introduced implicitly.

The successor must not yet create:

- a Rust scheduling-authority carrier;
- a Rust scheduling one-shot state owner;
- a new capability variant;
- policy-source mutation;
- scheduling event/request construction;
- expected-request sender/channel;
- `SessionId` source;
- authentication PRWM request-ID source;
- admission timing source;
- runtime/listener activation;
- deployment.

## 33. Validation requirement

Closure of C03e-NJ requires exact-final-head validation tied to the immutable NJ commit.

Expected path-filtered workflow conclusions must be recorded exactly; `SKIPPED` must not be represented as PASS.

No Android PASS may be claimed unless an exact-NJ-head Android workflow actually runs and succeeds.

## 34. Durable evidence requirement

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

## 35. Explicit exclusions

C03e-NJ performs or authorizes none of:

- Rust/source/runtime materialization;
- new capability creation;
- direct raw `RequesterRendezvousStart` policy-to-scheduler wiring;
- candidate-publication grant reuse;
- provider lifecycle reinterpretation;
- scheduling-authority carrier creation;
- scheduling one-shot state implementation;
- scheduling event/request carrier creation;
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

## 36. STOP boundary

After selection, exact-final-head validation, immutable evidence publication, closure metadata update, and final branch/PR readback, STOP.

Do not proceed into `PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_CONSUMPTION_STATE_OWNERSHIP_SELECTION` in the same checkpoint.
