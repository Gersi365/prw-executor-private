# Phase 152 C03e-OA — Production requester/rendezvous expected-device admission-request construction dependency provenance and custody selection

Status: `STAGING — SELECTION ONLY — SOURCE MATERIALIZATION BLOCKED`

Gate reserved for closure:
`C03E_OA_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CONSTRUCTION_DEPENDENCY_PROVENANCE_CUSTODY_SELECTED`

Closure reserved for exact validated/evidence-recorded head only:
`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REQUEST_CONSTRUCTION_DEPENDENCY_PROVENANCE_CUSTODY_SELECTION`

## 1. Exact predecessor authority

This checkpoint is documentation-only and is rooted at the exact durably closed C03e-NZ head:

`8ea0ef22788aa8a80e4ab7e0cd7bd8909fd64ba7`

Exact C03e-NZ tree:

`e5f893c8e6d09a2104e4e3a5a12bfada450e6c98`

C03e-NZ selected construction eligibility and input-authority law for the existing receiver-compatible:

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>`

C03e-OA does not reinterpret, widen, or source-materialize that request type.

## 2. Question selected here

The only question closed by this checkpoint is:

> Which dependencies required by the existing expected-device admission request have production provenance/custody already proved at exact C03e-NZ, which dependencies remain unresolved, and what ownership laws must any later coherent producer preserve before source materialization may be authorized?

This checkpoint does not construct or send a request.

## 3. Exact existing request shape

The exact C03e-NZ source audit confirms that the existing constructor requires exactly:

1. target expected logical `DeviceId`;
2. target admission `SessionId`;
3. authentication request ID as `u64`;
4. owned capability dispatcher `D`;
5. owned verifier-time provider `T`.

The request transfers dispatcher and verifier-time custody by value into the receiver-side admission path.

No sixth constructor input is invented here.

## 4. Target DeviceId provenance — resolved

C03e-NZ already selected the only construction-eligible scheduling case:

`SchedulingTerminal` with scheduling derivation `Ok(ExpectedDeviceSchedulingAuthorityGrant)`.

For that case, the target expected logical `DeviceId` MUST come only from the consumed scheduling grant.

The following are not target substitutes:

- scheduling-aware endpoint callback `DeviceId`, which identifies the authenticated requester worker;
- requester `SessionId` retained by the scheduling grant;
- transport identity;
- endpoint/IP/port/candidate/reachability state;
- configured peer identity;
- request/correlation IDs;
- policy or registry snapshots;
- default/test values.

No synthetic target and no target reconstruction are selected.

## 5. Scheduling grant custody

The one-shot scheduling grant remains non-`Copy` and non-`Clone`.

Any future coherent producer must consume its target identity exactly once by value or by an ownership-preserving projection that cannot duplicate/remint the grant.

The grant must not be:

- cloned;
- copied;
- reconstructed;
- reminted;
- replayed;
- hidden in global/static state;
- converted into a reusable bearer token;
- replaced by requester callback identity.

Requester acknowledgement disposition remains orthogonal to the already-derived scheduling result. Acknowledgement failure does not revoke, remint, reconstruct, or create a replacement scheduling grant.

## 6. Dispatcher provenance — concrete dormant implementation exists, caller custody still unmaterialized

The exact C03e-NZ ancestry contains the C03e-NB materialized crate-private dispatcher:

`LinuxAgentProductionRemoteCapabilityDispatcher`

in:

`crates/prw-agent/src/linux_bootstrap.rs`

Its exact current source remains a status-only `CapabilityDispatcher` implementation:

- it owns one `LocalAgentStatusSnapshot`;
- `AgentStatus` projects the existing bounded five-byte status body;
- file, transfer, terminal, and forwarding command families fail closed as unsupported;
- its constructor remains explicitly dormant pending separately gated production caller composition.

C03e-OA therefore selects the following law:

`NB_STATUS_ONLY_DISPATCHER_IS_THE_ONLY_CURRENTLY_PROVEN_CONCRETE_REMOTE_DISPATCHER_IMPLEMENTATION / ITS_PRODUCTION_EXPECTED_REQUEST_CALLER_CONSTRUCTION_AND_TRANSFER_CUSTODY_REMAIN_UNMATERIALIZED`

A later producer must not fabricate a broader dispatcher or synthesize success for unsupported provider families merely to complete expected-request construction.

No provider-family construction is selected here.

No dispatcher source mutation is authorized here.

## 7. Target admission SessionId provenance — unresolved

No exact current production source audited for this lane proves lawful production of the target admission `SessionId` required by `RemoteSessionExpectedDeviceAdmissionRequest`.

Historical verifier/session checkpoints establish typed `SessionId` semantics in other bounded lanes, but those semantics are not silently transferred as a production generator for this expected-device admission lane.

The requester `SessionId` carried by `ExpectedDeviceSchedulingAuthorityGrant` is scheduling provenance and MUST NOT be reused as the target admission `SessionId`.

Test literals, fixture IDs, configured strings, requester/session correlation from other lanes, candidate-publication identifiers, and transport identity are not production `SessionId` provenance.

Selected state:

`TARGET_ADMISSION_SESSION_ID_PRODUCTION_CUSTODY_NOT_PROVEN`

## 8. Authentication request-ID provenance — unresolved

No exact current production source audited for this lane proves an authentication PRWM request-ID allocator/custody owner for the expected-device admission request.

Historical Phase-129 PRWC request IDs are connection/message correlation and are not silently reusable as this authentication PRWM request-ID producer.

Historical authentication protocol selections that require one non-zero transaction request ID do not by themselves prove this expected-device producer owns a compatible allocator.

Candidate-publication PRWC correlation IDs are also not compatible authority merely because they are `u64` values.

No literal, increment invented at the call site, timestamp-derived value, SessionId-derived value, hash-derived value, or cross-lane correlation reuse is selected.

Selected state:

`AUTHENTICATION_PRWM_REQUEST_ID_PRODUCTION_CUSTODY_NOT_PROVEN`

## 9. Verifier-time provenance — historical law exists, this producer custody remains unresolved

Historical authenticated-session/challenge boundaries establish that challenge validity/admission verification time is verifier-owned authority rather than requester-controlled input.

That historical law is preserved.

However, C03e-OA does not silently promote any historical clock/time helper into the expected-request producer without an exact current producer-custody selection.

The expected-request `T` remains owned input transferred into the receiver-side admission transaction.

No requester timestamp, scheduling timestamp, acknowledgement timestamp, wall-clock literal, monotonic counter, candidate freshness token, or endpoint observation is verifier-time authority.

Selected state:

`EXPECTED_REQUEST_VERIFIER_TIME_PRODUCTION_CUSTODY_NOT_YET_PROVEN_FOR_THIS_PRODUCER`

## 10. Expected-request sender/channel ownership — unresolved

C03e-NC, C03e-ND, and C03e-NE previously proved the production expected-request composition remained receiver-side and did not prove a real production owner for:

- channel creation;
- sender custody;
- capacity;
- sender clone policy;
- backpressure;
- closed-channel behavior;
- sender-drop/shutdown ordering.

C03e-NZ through C03e-NY solved scheduling authority/result custody, not expected-request sender ownership.

C03e-OA finds no exact current evidence that those channel-lifecycle questions have since been materialized for this request lane.

Selected state:

`EXPECTED_REQUEST_SENDER_CHANNEL_PRODUCTION_OWNER_NOT_PROVEN`

A later checkpoint must not introduce a second/parallel admission queue merely to bypass this unresolved owner. Any future channel/source selection must be reconciled with the existing receiver-side executor/supervisor lifecycle and shutdown law before source mutation.

## 11. Coherent producer law

A complete expected-request producer is NOT proved merely because individual compatible-looking primitives exist.

A future coherent producer must possess, in one auditable ownership composition:

1. one construction-eligible consumed scheduling grant target;
2. one independently produced target admission `SessionId`;
3. one independently produced authentication PRWM request ID;
4. one allowed concrete dispatcher instance with explicit caller custody;
5. one verifier-owned time provider;
6. one selected expected-request sender whose lifecycle is paired with the existing receiver-side executor lifecycle.

All six dependencies must refer to the same one expected admission attempt without identity substitution or cross-lane correlation reuse.

The producer must construct at most one request from one consumed scheduling grant.

## 12. Dependency ordering preserved from BG and later checkpoints

The historical BG ordering remains an authority constraint, not an implementation claim.

For this lane, source materialization must preserve at least:

1. current registry/policy/scheduling authority prerequisites already selected upstream;
2. concrete allowed dispatcher custody;
3. target admission `SessionId` production custody;
4. authentication PRWM request-ID production custody;
5. verifier-owned time custody;
6. target `DeviceId` from the exact consumed scheduling grant;
7. selected sender/channel lifecycle;
8. only then complete request construction and send.

A future design may combine construction operations in one bounded factory only if it preserves these authority separations and exact ownership provenance.

## 13. Failure law before request construction

If any required dependency is unavailable, invalid, exhausted, closed, or not authoritatively selectable, request construction must fail closed before send.

No missing dependency may be replaced with:

- requester identity;
- a zero/default ID;
- reused correlation from another protocol lane;
- test fixture data;
- synthetic time;
- a broader fake dispatcher;
- an unbounded queue;
- a replacement scheduling grant.

No partial request may be sent.

## 14. Post-scheduling failure does not remint authority

The scheduling-consumption tombstone/one-shot law remains terminal producer-side authority consumption.

Therefore later inability to produce SessionId, request ID, timing, dispatcher, or sender custody does not erase scheduling consumption and does not authorize remint/retry/replay of the grant.

This checkpoint selects no automatic retry policy.

## 15. Request construction remains blocked

Because at least the following production dependencies remain unproved:

- target admission `SessionId` producer/custody;
- authentication PRWM request-ID producer/custody;
- verifier-time producer custody for this expected-request lane;
- production expected-request sender/channel owner;
- concrete NB dispatcher caller composition into this producer;

C03e-OA MUST NOT authorize a Rust request-construction/send successor.

Classification:

`TARGET_DEVICE_PROVEN_BY_CONSUMED_SCHEDULING_GRANT / NB_STATUS_ONLY_DISPATCHER_IMPLEMENTATION_PROVEN_BUT_CALLER_CUSTODY_UNMATERIALIZED / TARGET_ADMISSION_SESSION_ID_PRODUCER_UNRESOLVED / AUTHENTICATION_PRWM_REQUEST_ID_PRODUCER_UNRESOLVED / VERIFIER_TIME_PRODUCER_CUSTODY_UNRESOLVED_FOR_THIS_LANE / EXPECTED_REQUEST_SENDER_CHANNEL_OWNER_UNRESOLVED / COHERENT_EXPECTED_REQUEST_PRODUCER_NOT_YET_PROVEN / SOURCE_MATERIALIZATION_BLOCKED`

## 16. Smallest next blocking prerequisite

After independent C03e-OA closure, the next separately gated documentation boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_IDENTIFIER_TIMING_AND_SENDER_PRODUCTION_CUSTODY_SELECTION`

That future gate must select, at minimum:

1. exact target admission `SessionId` producer and uniqueness/lifetime law;
2. exact authentication PRWM request-ID producer and non-zero/correlation/uniqueness law;
3. exact verifier-owned time provider construction/custody for this lane;
4. exact expected-request sender/channel owner, capacity, clone policy, backpressure and shutdown law;
5. exact composition point where the existing NB dispatcher is constructed/moved into the request;
6. proof that no requester `SessionId`, PRWC request ID, candidate-publication request ID, transport identity, timestamp, or test value substitutes for these inputs;
7. the smallest future source ceiling, if and only if all required provenance is proved.

That future checkpoint remains documentation-only unless it independently selects and closes a source-materialization boundary afterward.

No successor token is inferred here solely from naming.

## 17. Immediate source ceiling

For C03e-OA itself:

`ZERO_RUST_SOURCE_PATHS`

Only this one documentation contract may differ from exact C03e-NZ.

Any Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging change blocks closure.

## 18. Validation plan

Closure requires exact-head readback proving:

- branch is rooted directly at exact C03e-NZ head;
- exact C03e-NZ merge base;
- ahead 1 / behind 0 unless a forward-only documentation correction is required and explicitly preserved;
- only this contract differs;
- no source/runtime/dependency/workflow mutation;
- PR remains draft/open/unmerged;
- exact-head Rust validation reaches terminal success;
- every skipped workflow is recorded as `SKIPPED`, never PASS;
- no Android PASS is claimed unless an exact-head Android run actually exists and succeeds.

## 19. Durable evidence plan

After exact-head validation:

1. freeze one immutable raw text audit;
2. exact-title search in canonical Drive evidence parent must return zero;
3. upload raw source-type-preserving `text/plain` to canonical parent;
4. verify title, MIME, parent and exact byte size;
5. raw-download/readback the artifact;
6. recompute SHA-256 and require byte/hash equality;
7. exact-title post-search must return exactly one artifact with the same Drive ID;
8. re-read branch and PR before closure metadata mutation;
9. update PR body only to `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
10. keep PR draft/open/unmerged;
11. STOP.

Canonical evidence parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

## 20. Explicit non-actions

C03e-OA authorizes none of the following:

- Rust source mutation;
- expected-device admission request construction;
- expected-request send;
- channel creation;
- sender clone;
- target admission `SessionId` generation/selection;
- authentication PRWM request-ID allocation/reuse;
- verifier-time construction for this lane;
- NB dispatcher caller wiring/materialization;
- broader provider construction;
- requester acknowledgement retry/resend;
- scheduling grant clone/copy/reconstruction/remint/replay;
- scheduling-consumption deletion/rollback;
- requester cleanup;
- candidate/reachability continuation;
- target dial;
- listener/bootstrap/readiness activation;
- runtime activation;
- Cargo/lockfile/workflow/Android mutation;
- deployment;
- merge;
- branch deletion;
- force push/history rewrite;
- repository configuration/ruleset/permission change.

## 21. Closure rule

C03e-OA may close only if its exact final head validates and durable evidence proves that the repository delta is documentation-only and that the dependency classification above is preserved exactly.

C03e-OA closure does not authorize the next source mutation.

After closure:

`STOP`
