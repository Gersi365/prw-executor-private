# Phase 152 C03e-NK — Production requester/rendezvous expected-device scheduling authority consumption-state ownership selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NK_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_CONSUMPTION_STATE_OWNERSHIP_SELECTED`

## 1. Purpose

C03e-NJ closed the authority-scope question for the requester-driven expected-device scheduling decision.

NJ selected:

`REQUESTER_RENDEZVOUS_START_BOUNDED_SCHEDULING_CONSEQUENCE_SELECTED / DISTINCT_DERIVED_ONE_SHOT_SCHEDULING_AUTHORITY_GRANT_MANDATORY / FRESH_CURRENT_REQUESTER_TARGET_POLICY_AND_PROVIDER_STATE_REQUIRED / DIRECT_CAPABILITY_ALLOW_OR_CANDIDATE_PUBLICATION_GRANT_REUSE_FORBIDDEN / SOURCE_MATERIALIZATION_BLOCKED`

NJ also selected the next blocking prerequisite:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_CONSUMPTION_STATE_OWNERSHIP_SELECTION`

C03e-NK audits only that prerequisite.

The bounded question is:

> Where must scheduling-specific one-shot consumption state live, what exact operation identity must it linearize, and under which existing synchronization/lock ordering must one authority commitment become terminal without mutating candidate-publication provider lifecycle?

C03e-NK selects ownership, identity, linearization, creation timing, drop semantics, and post-commit failure disposition only.

It does not materialize a Rust state owner, state representation, capacity, error enum, authority grant carrier, scheduling event/request, sender/channel, SessionId source, authentication PRWM request-ID source, timing source, caller, runtime activation, deployment, or merge.

## 2. Exact predecessor

Canonical repository:

`Gersi365/prw-executor-private`

Repository ID:

`1334911207`

Predecessor branch:

`phase-152-c03e-nj-production-requester-rendezvous-expected-device-scheduling-authority-scope-selection`

Exact predecessor head:

`fa909155ca738ec47e2b3de48cd6fde41131faca`

Exact predecessor tree:

`ce6c969b6c2ee16f96da163a7ba86df46a6bbea1`

Exact predecessor contract blob:

`082ce54940941251889df6486055bc1fb58e6be6`

Predecessor PR:

`#498`

Predecessor closure:

`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_SCOPE_SELECTION`

## 3. Selection

C03e-NK selects:

`SHARED_REQUESTER_RENDEZVOUS_AUTHORITY_COMPOSITE_STATE_SELECTED_AS_SCHEDULING_CONSUMPTION_OWNER / REQUESTER_SESSION_ID_PLUS_TARGET_DEVICE_ID_SELECTED_AS_OPERATION_KEY / EXISTING_REQUESTER_MUTEX_SELECTED_AS_SOLE_LINEARIZATION_PRIMITIVE / LAZY_TERMINAL_COMMIT_TOMBSTONE_SELECTED / CANDIDATE_PUBLICATION_PROVIDER_LIFECYCLE_UNCHANGED / SOURCE_MATERIALIZATION_BLOCKED`

Meaning:

- scheduling-specific one-shot state belongs to the Agent-owned `SharedRequesterRendezvousAuthority` ownership domain;
- the future shared state must be a sibling of the existing candidate-publication runtime owner, not a field inside the candidate-publication provider record;
- the exact scheduling-operation key is the authenticated requester `SessionId` plus the exact logical target `DeviceId`;
- no request ID, transport identity, endpoint, candidate, worker ID, timing value, or configured peer participates in the key;
- the existing requester/rendezvous Tokio mutex is the only selected scheduling-state mutation/linearization primitive;
- no second scheduling mutex is selected;
- no stored `Eligible` authority state is created at requester/rendezvous registration;
- terminal scheduling state is created lazily only at the authority-mint commit point after all NJ-selected fresh gates succeed;
- once committed, the key remains terminal for remint for the lifetime of this process-local shared owner;
- candidate-publication provider current/retired/remove semantics remain unchanged;
- source materialization remains blocked.

## 4. Exact source audit

C03e-NK re-audited exact NJ head `fa909155ca738ec47e2b3de48cd6fde41131faca`.

Relevant exact source blobs include:

- `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`
  - blob `d550ec8d5aa18ed6885ebed42c52ee742498e9d2`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
  - blob `082a70af239972a82318f3e17cb3fd8cb45d9e95`
- `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`
  - blob `d01cfbc37433f6099e216397b9bf243aa55c53bc`
- `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs`
  - blob `260024b7aca2aea6109dc72e778bcda3dcca8038`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
  - blob `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`
- `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
  - blob `60307fff4dd0fd573192ba6e6fab9dedd3321dda`
- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - blob `de66532f18ebbca30ac6bd6b9da4983ded4b8bbe`
- `crates/prw-policy/src/lib.rs`
  - blob `3056b53e81c4429314d9f890dcf2bf3e80d433b8`

The exact NJ tree contains no already-materialized scheduling-authority consumption-state owner or scheduling-specific one-shot ledger.

## 5. Existing shared requester authority is the correct ownership domain

`SharedRequesterRendezvousAuthority` already owns the synchronization boundary around exactly one process-local `CandidatePublicationRequesterRendezvousRuntimeOwner`.

Its clones share one outer `Arc` and one Tokio `Mutex`.

Existing operations under that mutex include:

- requester/rendezvous start registration;
- current requester/rendezvous grant selection for candidate publication;
- exact candidate-publication post-commit requester record cleanup.

This is the narrowest existing Agent ownership domain that already serializes lifecycle mutations and authority reads involving the same requester/target relationship.

C03e-NK therefore selects this shared requester authority as the future scheduling-consumption owner.

## 6. Selected future composite ownership shape

The current source shape is conceptually:

`Arc<Mutex<CandidatePublicationRequesterRendezvousRuntimeOwner>>`

C03e-NK selects that a future source-materialization checkpoint may replace only the mutex payload with a private composite shared state conceptually containing:

1. the unchanged existing candidate-publication requester/rendezvous runtime owner; and
2. one separate scheduling-specific consumption ledger.

The scheduling ledger is a sibling state component.

It is not selected as:

- a field inside `InMemoryRequesterRendezvousAuthorityProvider`;
- a new provider lifecycle variant;
- a candidate-publication grant field;
- a global process singleton outside requester authority;
- a remote-session worker collection field;
- a transport/runtime field;
- a channel-owned field;
- a durable-registry field.

Exact Rust representation remains separately gated.

## 7. Why the candidate-publication provider is not modified

`InMemoryRequesterRendezvousAuthorityProvider` has an established candidate-publication lifecycle:

`Current -> Retired -> removed`

Its exact records retain:

- authenticated requester session;
- expected publisher/target `DeviceId`;
- candidate-publication lifecycle state.

Its current authorization is non-consuming and repeatable while the record remains current.

NJ already established that this provider lifecycle cannot enforce scheduling one-shot semantics.

Adding scheduling `Eligible`, `Committed`, or `Consumed` states to the same provider record would mix two operation domains and could change candidate-publication cleanup behavior.

C03e-NK therefore keeps scheduling consumption state outside the provider.

## 8. Exact scheduling-operation identity key

C03e-NK selects the exact scheduling-consumption key as:

`(authenticated requester SessionId, exact target DeviceId)`

The requester `SessionId` comes only from the exact server-held authenticated requester session.

The target `DeviceId` is the exact logical target proven by the requester/rendezvous validation chain.

The key is an internal scheduling-consumption identity only.

It is not by itself authorization.

Fresh NJ-selected requester/target currentness, provider state, and policy checks remain mandatory before a first commit.

## 9. Why requester DeviceId alone is insufficient

Requester logical `DeviceId` alone does not distinguish two authenticated requester sessions.

Scheduling one-shot semantics are scoped by NJ to the exact authenticated requester operation, not to every future session from the same requester device.

Therefore the requester component of the key must be the authenticated requester `SessionId`, not only requester `DeviceId`.

Requester `DeviceId`, `WorkspaceId`, and `UserId` remain authoritative principal dimensions revalidated through the authenticated session and policy source, but they are not selected as duplicate-key replacements for `SessionId`.

## 10. Why target DeviceId is required

One requester session may nominate different logical targets.

One committed scheduling consequence for target A must not consume the separately authorized opportunity for target B.

Therefore target `DeviceId` is part of the exact key.

Transport identity, endpoint, candidate, IP, port, or reachability path must never substitute for the target logical `DeviceId`.

## 11. Request/correlation IDs are forbidden as operation identity

The existing requester/rendezvous runtime source explicitly keeps outer requester request IDs as correlation only.

C03e-NK therefore rejects using:

- requester/rendezvous wire request ID;
- PRWM authentication request ID;
- PRWC candidate-publication request ID;
- any generic bridge request ID;

as scheduling-consumption identity.

Correlation does not become authority because duplicate classification needs a key.

## 12. No new operation-generation identifier is selected

Exact source contains no separately authoritative requester/rendezvous operation-generation ID.

C03e-NK does not invent one.

Consequently two requester/rendezvous starts that use the same authenticated requester `SessionId` and the same target `DeviceId` share the same scheduling-consumption key for this selected boundary.

After one scheduling commitment for that key, a later re-registration of the same session/target pair may still be meaningful to the existing candidate-publication provider, but it cannot mint a second scheduling authority under C03e-NK.

If product semantics later require more than one scheduling authority for the same requester session/target pair, a new explicit operation-instance identity design must be separately gated. It must not reuse a wire request ID by implication.

## 13. Existing requester mutex is the selected linearization primitive

C03e-NK selects the existing Tokio mutex owned by `SharedRequesterRendezvousAuthority` as the sole scheduling-consumption mutation lock.

A second mutex for scheduling state is rejected because it would introduce a new lock ordering problem between:

- requester provider state;
- scheduling consumption state;
- shared-current registry/policy state.

Keeping both requester runtime owner and scheduling ledger under the same requester mutex permits one atomic requester-domain critical section.

## 14. Existing lock ordering is preserved

Existing requester registration already selects the order:

1. acquire requester/rendezvous mutex;
2. acquire/read `SharedCurrentCapabilityAuthority` while requester custody remains held;
3. perform the bounded synchronous authority operation;
4. release shared-current read;
5. release requester mutex;
6. only then perform response I/O or other external work.

C03e-NK selects the same ordering for future scheduling-authority mint.

No lock inversion is authorized.

No current-authority lock may be acquired first and followed by requester mutex acquisition in the same scheduling path.

## 15. No separate scheduling mutex

No `Arc<Mutex<SchedulingState>>` sibling is selected.

No nested scheduling mutex is selected.

No lock-free atomic/counter is selected.

No global static lock is selected.

The future composite shared requester state must be protected by the one existing requester mutex.

This keeps provider-current classification and scheduling duplicate classification serializable under one requester-domain lock.

## 16. Lazy creation is selected

Scheduling consumption state is not created during requester/rendezvous registration.

Registration continues to establish only the existing requester/rendezvous relationship.

Absence of a scheduling ledger entry means only:

`NO_SCHEDULING_COMMIT_HAS_YET_BEEN_RECORDED_FOR_THIS_KEY`

It does not mean:

- requester is current;
- target is current;
- policy allows;
- provider relationship is current;
- scheduling is authorized.

This avoids caching `Eligible` authority from an earlier point-in-time validation.

## 17. Fresh-gate order before first commit

For a key with no terminal scheduling entry, future scheduling derivation must remain fail-closed and perform the NJ-selected fresh gates before state commitment.

C03e-NK selects the following ownership/locking envelope:

1. acquire the shared requester mutex;
2. classify whether the exact key already has terminal scheduling state;
3. if terminal, reject as duplicate/consumed without remint;
4. if absent, acquire one coherent shared-current authority read under the requester mutex;
5. re-establish exact requester/target current-registry eligibility;
6. re-resolve and reauthorize exact `Capability::RequesterRendezvousStart` policy for the authenticated requester;
7. establish exact current requester/rendezvous provider uniqueness without reusing candidate-publication scheduling authority;
8. only after all checks succeed, commit terminal scheduling state for the key while both requester custody and current-authority read remain lexical;
9. create/return one future owned scheduling grant only after that commit;
10. release shared-current read;
11. release requester mutex;
12. perform no channel send, network I/O, peer acceptance, response write, or runtime work while either authority lock is held.

Exact function decomposition and Rust carrier remain separately gated.

## 18. Terminal commit is the one-shot linearization point

The scheduling one-shot linearization point is the first successful insertion/transition of the exact operation key into terminal scheduling-consumption state.

Before that point:

- no scheduling authority exists;
- cancellation/failure creates no tombstone;
- fresh checks may fail closed with zero scheduling decisions.

At that point:

- the key becomes permanently non-remintable for this owner lifetime;
- exactly one owned scheduling authority may be produced by that transition;
- concurrent contenders serialized behind the same requester mutex must observe the terminal key and reject.

No later receiver-side worker state participates in this producer-side linearization.

## 19. Stored `Eligible` state is rejected

C03e-NK does not select a stored `Eligible` state.

Eligibility is transient evidence evaluated at the authority-mint attempt.

Persisting or retaining an `Eligible` marker from requester/rendezvous registration could outlive:

- requester session currentness;
- requester membership currentness;
- target lifecycle/currentness;
- workspace relationship;
- policy decision;
- requester/rendezvous provider currentness.

Therefore no persistent or process-local eligibility snapshot is selected.

## 20. Terminal tombstone semantics

C03e-NK selects one terminal scheduling-consumption fact per committed key.

The exact Rust enum/set/map representation is not yet selected.

Semantically, the terminal fact means:

`A scheduling authority was already committed for this requester-session/target key; do not mint another.`

It does not mean:

- downstream expected request was sent;
- channel accepted it;
- AJ admission succeeded;
- authentication succeeded;
- a worker became active;
- candidate publication succeeded.

The tombstone records authority consumption, not execution success.

## 21. No post-commit rollback

Once terminal scheduling state is committed, later failures do not erase it.

Specifically, no rollback/remint is authorized after:

- scheduling grant drop;
- future scheduling-event composition failure;
- expected-request construction failure;
- channel full/closed failure;
- receiver rejection;
- receiver-side `DuplicateActiveDevice`;
- real-admission registry/transport failure;
- authentication failure;
- worker failure/completion;
- candidate/reachability changes;
- process shutdown initiation.

This preserves NJ's no-retry/no-replay authority law.

## 22. Grant consumption does not require a second ledger transition

The future owned scheduling grant is separately required by NJ to be one-shot/non-reusable.

C03e-NK does not select a second mutable transition from `Committed` to `Consumed` after the grant leaves the requester authority critical section.

The terminal ledger fact already blocks remint.

Whether the owned grant is later consumed into a scheduling event or simply dropped cannot reopen the key.

This avoids a second cross-component mutation path after authority leaves requester custody.

## 23. Duplicate/concurrent classification

For the exact selected key:

- the first contender that passes all fresh gates and commits the terminal fact wins;
- every contender serialized afterward rejects as already committed/consumed;
- no duplicate authority is coalesced;
- no duplicate authority is queued;
- no duplicate authority replaces the first grant;
- no duplicate authority becomes an automatic retry.

Exact error type/name remains separately gated with representation selection.

## 24. Provider cleanup racing scheduling derivation

The existing requester mutex serializes candidate-provider cleanup and scheduling derivation.

Two important orderings remain deterministic.

If candidate-publication cleanup acquires requester custody first and removes the provider record, a later first scheduling derivation cannot establish the NJ-required current provider relationship and fails with zero scheduling commit.

If scheduling derivation commits first, a later candidate-publication cleanup may retire/remove the provider record normally, but it does not erase the scheduling tombstone.

No cross-operation rollback is selected.

## 25. Candidate-publication lifecycle remains unchanged

Existing provider operations remain exactly:

- `register_current`;
- `authorize_current_for_publisher`;
- `retire`;
- `remove_retired`.

C03e-NK selects no change to their semantics.

Candidate-publication committed cleanup still retires and removes the exact provider record only after definite durable publication commit.

Scheduling state is not removed by that cleanup.

Candidate-publication cleanup failure does not create or reset scheduling authority.

## 26. Existing candidate-publication grant remains separate

`AuthorizedRequesterRendezvous` remains one-shot operation evidence for one candidate-publication attempt only.

C03e-NK does not store that grant in scheduling state.

C03e-NK does not use its non-Clone property as a substitute for scheduling duplicate state.

A future scheduling-specific grant carrier remains separately gated.

## 27. Provider currentness check must not become candidate-grant reuse

The future scheduling mint requires exact provider currentness/uniqueness evidence under requester custody.

However C03e-NK does not authorize repurposing `AuthorizedRequesterRendezvous` as the scheduling grant.

A later source-design checkpoint may need a narrow non-authorizing provider/current-record validation seam or another exact provenance-preserving internal method.

That method must not change provider lifecycle or create candidate-publication authority side effects.

## 28. Shared-current authority remains separate

`SharedCurrentCapabilityAuthority` remains the coherent current registry/policy authority.

Its existing `RwLock` is not replaced by the requester mutex.

The requester mutex owns requester-domain serialization; the shared-current read owns coherent current registry/policy evidence.

C03e-NK preserves both domains and their established nesting order.

## 29. Policy source remains separately authoritative

Requester/rendezvous policy resolution remains bound to the exact authenticated requester.

C03e-NK does not cache policy `Allow` in the scheduling ledger.

C03e-NK does not add policy state to the requester mutex payload.

Fresh policy evaluation remains required for a first scheduling commit.

After terminal scheduling consumption exists, duplicate rejection does not reopen or reevaluate authority for remint.

## 30. Shutdown/drop semantics

The selected scheduling ledger is process-local and owned by the same shared requester authority lifetime.

When the final shared requester authority owner is dropped, scheduling consumption state is dropped in memory.

C03e-NK selects no:

- persistence;
- disk snapshot;
- database row;
- etcd record;
- restart restoration;
- crash recovery;
- timer cleanup;
- TTL expiry;
- background compaction task.

Process restart does not itself authorize retry/replay. A future operation must still satisfy its own authenticated/current authority path.

## 31. Tombstones are not removed on candidate cleanup

Because the selected key is requester `SessionId` plus target `DeviceId`, removing the candidate-publication provider record does not erase scheduling consumption history for that same key.

This prevents a later same-session/same-target provider re-registration from silently creating a second scheduling authority.

This is a deliberate fail-closed consequence of not inventing an operation-generation identifier.

A later design that needs repeat scheduling within one requester session must introduce and separately authorize a stronger operation-instance identity; it may not infer one from request correlation.

## 32. Capacity remains separately gated

NJ requires bounded scheduling authority state.

C03e-NK selects ownership but does not select the exact finite representation or maximum entry count.

Capacity exhaustion must ultimately fail closed before terminal commit and before grant creation.

C03e-NK explicitly does not reinterpret the existing:

`PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`

configuration as scheduling-consumption-history capacity.

That existing value bounds candidate-publication requester/rendezvous records; reusing it for lifetime scheduling tombstones would change its established semantic.

Exact scheduling-state capacity is selected in the next prerequisite.

## 33. No timing/TTL authority

No wall-clock time is part of the scheduling-consumption key.

No timestamp is required for terminal authority consumption.

No expiry is selected.

Verifier time, challenge validity, admission timing, candidate freshness, reachability freshness, and application lease timing remain separate operation domains.

## 34. No channel/sender coupling

Scheduling consumption state does not live in:

- `mpsc::Sender`;
- `mpsc::Receiver`;
- channel capacity;
- send permit;
- sender clone count;
- channel closed state.

A terminal authority commit occurs before any future send attempt.

Channel failure cannot roll it back.

Sender/channel ownership remains downstream and separately gated.

## 35. No worker-state coupling

Scheduling consumption state does not live in the repeated-admission active worker map.

Receiver-side `DuplicateActiveDevice` remains a later independent safety check.

Worker insertion/removal/completion cannot grant, reset, or retry producer scheduling authority.

## 36. No transport/reachability coupling

Scheduling consumption state is not keyed or invalidated by:

- `TransportIdentity`;
- endpoint/IP/port;
- candidate set;
- reachability success/failure;
- relay state;
- configured peer identity.

Logical requester session and target `DeviceId` remain the selected operation identity.

AJ later performs independent current transport/authentication checks.

## 37. No correlation/session-admission field coupling

The scheduling-consumption ledger does not allocate or store:

- future remote `SessionId` for AJ;
- authentication PRWM request ID;
- verifier time source;
- dispatcher;
- admission timing ranges.

Those values remain later request-composition prerequisites.

Requester `SessionId` is used only as part of the authenticated requester operation key; it is not the future target-side admission SessionId.

## 38. No new capability or policy branch

C03e-NK materializes no new capability.

`Capability::RequesterRendezvousStart` remains the exact policy capability selected by NJ for fresh reauthorization at scheduling mint.

`DeviceManage` remains unrelated.

No policy evaluator implementation changes.

## 39. Failure precedence at terminal duplicate

For a key already present in terminal scheduling state, C03e-NK selects terminal duplicate/consumed rejection without attempting a second mint.

A later representation/error-selection checkpoint must choose the exact bounded error classification.

This terminal classification is not evidence that current registry/provider/policy state still permits the operation; it only proves that this owner already consumed the scheduling opportunity for the key.

No remint follows even if later authority state would otherwise be current.

## 40. Failure before commit leaves no state

For a key not yet terminal, failures before the commit point leave no scheduling tombstone.

Examples include:

- requester mutex acquisition cancellation before critical-section execution;
- current requester/target validation failure;
- policy-source unavailable/indeterminate;
- policy denial;
- requester/rendezvous provider missing/stale/retired/ambiguous;
- future bounded scheduling-ledger capacity exhaustion;
- explicit shutdown/cancellation observed before commit.

Zero scheduling decisions remain valid.

## 41. No retry policy is inferred from absent state

An absent key after a pre-commit failure does not itself authorize automatic retry.

It means only that no terminal scheduling consumption has been recorded.

Any later new mint attempt must be separately invoked by the selected operation lifecycle and repeat all required gates.

C03e-NK adds no timer, loop, reconnect, backoff, replacement, or replay mechanism.

## 42. Existing requester registration remains unchanged

`validate_authorize_and_register_requester_rendezvous_start(...)` continues to execute its established four stages:

1. current-registry validation;
2. requester policy-source resolution;
3. dedicated requester/rendezvous-start policy authorization;
4. provider registration.

C03e-NK does not add scheduling state mutation to that function.

The selected scheduling state remains lazy and belongs to a later explicit authority-derivation call after registration.

## 43. Existing source lacks the selected state carrier

The exact NJ source has no:

- scheduling-consumption identity type;
- scheduling terminal set/map;
- scheduling-state capacity;
- scheduling duplicate error;
- scheduling-grant type;
- scheduling-authority mint method.

Therefore C03e-NK remains documentation-only and source materialization is blocked.

## 44. Selected next blocking prerequisite

C03e-NK selects:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_STATE_REPRESENTATION_CAPACITY_AND_FAILURE_SELECTION`

The next checkpoint must remain documentation-only.

It must select the exact bounded process-local representation for the terminal scheduling-consumption ledger under the owner/lock semantics selected here.

It must determine at least:

1. exact private operation-key Rust shape for requester `SessionId` + target `DeviceId`;
2. set/map/record representation;
3. finite capacity and capacity source;
4. duplicate/already-consumed failure classification;
5. capacity-exhaustion failure classification;
6. constructor invariants;
7. lookup/commit atomic API shape under the existing requester mutex;
8. whether the terminal entry stores only identity or any additional non-authorizing metadata;
9. proof that no eligibility/policy/currentness snapshot is retained;
10. proof that candidate-publication provider state remains byte/semantic independent;
11. exact future source path/materialization seam ceiling, without materializing it yet.

The successor must not yet create:

- a Rust scheduling state type;
- a scheduling authority grant type;
- a scheduling mint method;
- a new capability;
- policy-source changes;
- expected-request construction;
- sender/channel construction;
- SessionId generation for target admission;
- authentication PRWM request-ID allocation/reuse;
- verifier/admission timing;
- runtime caller activation;
- listener/startup/readiness wiring;
- retry/reconnect behavior;
- deployment.

## 45. Validation requirement

Closure of C03e-NK requires exact-final-head validation tied to the immutable NK commit.

Expected path-filtered workflows must be recorded exactly.

`SKIPPED` must not be represented as PASS.

No Android PASS may be claimed unless an exact-NK-head Android workflow actually runs and succeeds.

## 46. Durable evidence requirement

Before closure, one immutable raw text audit must be published under canonical Google Drive evidence parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

Required procedure:

1. exact-title pre-upload search returns zero canonical matches;
2. freeze local audit bytes only after exact-final-head validation;
3. compute exact byte length and SHA-256;
4. upload without conversion as `text/plain`;
5. read metadata before any move;
6. move only from the verified source parent to the canonical evidence parent if required;
7. raw Drive readback/download;
8. recompute exact byte length and SHA-256 from Drive bytes;
9. require exact frozen/readback match;
10. exact-title post-upload search returns exactly one canonical artifact;
11. re-read branch and PR state before closure metadata update;
12. final branch/PR readback after closure metadata.

## 47. Explicit exclusions

C03e-NK performs or authorizes none of:

- Rust/source/runtime materialization;
- scheduling-state representation materialization;
- scheduling grant carrier creation;
- new capability creation;
- requester policy mutation;
- candidate provider lifecycle mutation;
- expected-device scheduling event construction;
- `RemoteSessionExpectedDeviceAdmissionRequest` construction;
- expected-request sender/channel creation;
- sender clone/send behavior;
- target-side `SessionId` generation;
- authentication PRWM request-ID allocation/reuse;
- verifier-time construction;
- admission timing policy;
- dispatcher caller migration;
- runtime/listener activation;
- candidate/reachability-triggered scheduling;
- retry/reconnect/rebind/rebootstrap/replay;
- persistence/database/etcd state;
- timer/TTL/background cleanup;
- `main.rs` wiring;
- systemd/service/package mutation;
- credentials/certificate/private-key/trust/RBAC mutation;
- repository configuration mutation;
- deployment/restart/recovery activation;
- merge;
- PR close;
- ready-for-review conversion;
- branch deletion;
- force update/rebase/squash/history rewrite.

## 48. STOP boundary

After selection, exact-final-head validation, immutable evidence publication, closure metadata update and final readback, STOP.

Do not proceed into `PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_STATE_REPRESENTATION_CAPACITY_AND_FAILURE_SELECTION` in the same checkpoint.
