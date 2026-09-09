# Desktop Functional Management Slice C03e-NR

## Production requester/rendezvous expected-device scheduling-authority derivation composition selection — STAGING

Status: `SELECTION / DOCUMENTATION_ONLY / SOURCE_MATERIALIZATION_BLOCKED`

Gate candidate:

`C03E_NR_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_DERIVATION_COMPOSITION_SELECTED`

Predecessor authority:

- exact closed C03e-NQ head: `afde37ed5d0c32a5bcc5333fdda3ec7dd5fb083d`;
- exact closed C03e-NQ tree: `93be064fbc0579dbbed4d535571b0107d5004a03`;
- C03e-NQ PR: `#505`, intentionally draft/open/unmerged;
- C03e-NQ durable evidence Drive ID: `1jjXMceJkA4X662gNVlgkitdavmC_GWby`;
- C03e-NQ final Rust validation: #1671 `SUCCESS`;
- C03e-NQ final Android validation: #1585 `SUCCESS`.

This checkpoint is documentation-only. It selects one bounded dormant composition for deriving exactly one expected-device scheduling authority after an already-successful requester/rendezvous registration. It does not materialize Rust source, invoke the composition from the requester lifecycle, alter requester acknowledgement semantics, construct an expected-device admission request, create a sender/channel, allocate a request ID or admission `SessionId`, choose timing, activate a listener/runtime, deploy, merge, or mutate production/control-plane state.

---

## 1. Closed predecessor invariants

C03e-NI selected authenticated requester/rendezvous start as the semantic origin operation class for the expected-device scheduling decision. Target provenance remains the exact requester-rendezvous current-registry chain.

C03e-NJ selected a bounded internal scheduling consequence of `RequesterRendezvousStart`, but raw `Decision::Allow`, a policy-authorized requester-start carrier, and candidate-publication `AuthorizedRequesterRendezvous` are not themselves scheduling authority. A distinct derived one-shot scheduling authority is mandatory.

C03e-NK selected:

- scheduling-specific terminal consumption as process-local sibling state under `SharedRequesterRendezvousAuthority`;
- exact operation key = requester `SessionId` + target logical `DeviceId`;
- the existing requester mutex as the sole scheduling-state linearization primitive;
- no rollback/remint after terminal consumption;
- candidate-publication provider lifecycle independently preserved.

C03e-NL selected:

- bounded terminal `Vec<Key>` state;
- explicit finite capacity;
- duplicate-before-capacity ordering;
- `InvalidCapacity`, `AlreadyConsumed`, and `CapacityExhausted` representation failures;
- successful insertion as the terminal-consumption linearization point;
- no eviction, TTL, persistence, retry, replay, compaction, restart restore, or timer semantics.

C03e-NN materialized the dedicated scheduling-consumption capacity source.

C03e-NO materialized the dormant private bounded ledger representation.

C03e-NP selected one shared-mutex composite owner and one atomic three-path owner/population integration seam.

C03e-NQ materialized that seam. The final shared requester authority owns the requester/rendezvous runtime owner and scheduling-consumption ledger under one shared Tokio mutex/`Arc`, while configured production population constructs that owner exactly once from separate requester/rendezvous and scheduling-consumption capacities.

C03e-NQ explicitly did **not** add a scheduling-authority derivation method or any call to `commit_if_absent` from a production scheduling path.

---

## 2. Exact C03e-NQ source facts relevant to derivation

### 2.1 Shared requester authority

Exact NQ blob:

`d277f2c9255d8435c9a7100dfac7814f64da8916`

Current shared state is one `Arc<Mutex<SharedRequesterRendezvousState>>` containing:

- `CandidatePublicationRequesterRendezvousRuntimeOwner`;
- `ExpectedDeviceSchedulingConsumptionLedger`.

`SharedRequesterRendezvousAuthority::validate_authorize_and_register_requester_rendezvous_start(...)` currently:

1. acquires the requester/shared-owner mutex;
2. while that guard is held, acquires one shared-current registry/policy read;
3. runs the existing synchronous DI -> DP -> DK -> DN requester-start composition exactly once;
4. releases current-authority read and requester mutex before response I/O.

It returns only the existing requester-start composition result.

The scheduling ledger remains unreachable from production methods. Its private successful insertion remains the selected terminal-consumption linearization point.

### 2.2 Existing current-registry validator

Exact blob:

`1c021bc95a3d674722bfd70559156fa75e07e578`

`validate_current_requester_rendezvous_start_intent(...)` performs, in fixed order:

1. authenticated requester-session currentness;
2. exact target lookup;
3. target device lifecycle;
4. target membership lifecycle;
5. same-workspace equality;
6. exact target-identity preservation.

Successful return proves only point-in-time current registry eligibility. It is not policy or scheduling authority.

### 2.3 Existing requester-aware policy source and admission

Policy source blob:

`f7377011a3ab2034c14d9018a5c0f268f6660ffa`

Policy admission blob:

`b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`

The requester-aware source resolves policy only from the exact authenticated requester dimensions. `Unavailable` and `Indeterminate` fail closed; there is no fallback evaluator.

The dedicated policy admission evaluates exactly `Capability::RequesterRendezvousStart`. Every other capability remains denied by the concrete requester-start policy. Policy success is still only an operation gate and is not scheduling authority.

### 2.4 Existing registration composition

Exact blob:

`8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`

The existing synchronous DR composition order is:

1. current-registry validation;
2. requester-aware policy-source resolution;
3. exact requester-start policy authorization;
4. requester/rendezvous registration mutation.

It returns `Result<(), RequesterRendezvousStartCompositionError>` and intentionally has no post-registration scheduling result channel.

Re-running this composition merely to derive scheduling authority is rejected because a successful first registration may cause the second registration to fail as `RecordAlreadyExists`.

### 2.5 Existing provider uniqueness evidence

Concrete provider blob:

`d01cfbc37433f6099e216397b9bf243aa55c53bc`

Provider-neutral authority blob:

`260024b7aca2aea6109dc72e778bcda3dcca8038`

Runtime-owner blob:

`082a70af239972a82318f3e17cb3fd8cb45d9e95`

For one exact target/publisher `DeviceId`, current requester/rendezvous provider authorization is fail-closed:

- more than one current match -> `Ambiguous`;
- one current match -> one owned `AuthorizedRequesterRendezvous`;
- retired-only match -> `StaleOrRetired`;
- no match -> `Missing`;
- provider-neutral port also reserves `UnavailableOrIndeterminate`.

The concrete provider selection is non-consuming and repeatable. Candidate-publication lifecycle is not retired or removed by selection itself.

`AuthorizedRequesterRendezvous` is candidate-publication operation authority only. It is neither `Copy` nor `Clone` and must not become or escape as expected-device scheduling authority.

### 2.6 Existing requester acknowledgement semantics

`requester_rendezvous_retained_custody_dr_continuation.rs` retains exactly:

- the original requester transaction; and
- the existing `Result<(), RequesterRendezvousStartCompositionError>`.

That result is encoded into the existing requester/rendezvous DR acknowledgement. Existing registration success/failure therefore already has a stable peer-visible semantic channel.

A new scheduling-derivation failure that occurs after registration mutation must not silently be translated into a requester-start registration failure.

---

## 3. Design alternatives considered

### A. Re-run the existing full DR composition after registration

Rejected.

It would repeat registration and can transform a successful first operation into `RecordAlreadyExists`. It also duplicates a mutation path merely to obtain fresh validation/policy evidence.

### B. Extend the existing DR registration result so scheduling derivation failure becomes DR failure

Rejected.

Scheduling derivation necessarily occurs after successful provider registration because fresh provider uniqueness is a selected mint prerequisite. A post-registration scheduling failure therefore happens after a real side effect already committed. Rewriting that state as `RequesterRendezvousStartCompositionError` would misrepresent registration truth and could encourage a retry that collides with the existing record.

### C. Commit scheduling consumption before provider registration

Rejected.

NJ/NK require current provider state to be uniquely attributable to the exact operation before mint. Pre-registration consumption would commit authority without that prerequisite.

### D. Use receiver-side `DuplicateActiveDevice` as producer one-shot state

Rejected.

Receiver duplicate protection is downstream admission defense and does not classify producer-side scheduling authority consumption.

### E. Add a second scheduling mutex or separate scheduling owner

Rejected.

NK and NQ already selected one shared requester mutex/composite owner as the sole scheduling-state linearization domain.

### F. Add a new provider/public bridge query before the existing provider API is proven insufficient

Not selected.

The existing provider authorization port already proves exactly-one current requester/rendezvous state with the required Missing/Stale/Ambiguous/Unavailable classifications. Creating a second provider query surface is unnecessary for the first bounded composition.

### G. Use the existing candidate-publication grant as scheduling authority

Rejected.

`AuthorizedRequesterRendezvous` remains candidate-publication authority only.

### H. Use the existing provider selection only as an ephemeral uniqueness witness, then discard its candidate-publication grant before scheduling reauthorization/commit

**Selected.**

The existing grant may exist only inside the requester mutex long enough to obtain the provider-held exact authenticated requester session and exact target selected by the unique-current provider check. It must then be dropped before fresh registry/policy reauthorization and before terminal scheduling consumption.

No candidate-publication authority bit, capability, lifecycle action, grant object, or grant lifetime crosses into the scheduling grant.

---

## 4. Selected post-registration derivation boundary

The first derivation method is **post-registration only**.

It does not perform requester/rendezvous registration and does not replace or widen:

`validate_authorize_and_register_requester_rendezvous_start(...)`.

The future dormant method accepts only non-authorizing exact operation selectors:

- requester `SessionId`;
- target logical `DeviceId`;
- existing `SharedCurrentCapabilityAuthority`;
- existing requester-aware policy source.

The requester `SessionId` and target `DeviceId` inputs are lookup/match selectors only. Possession does not authorize scheduling.

The future caller must invoke this derivation only after the existing DR registration result is `Ok(())`. Caller integration is **not** selected or materialized by NR.

---

## 5. Selected lock and execution order

The future dormant derivation method must preserve this exact order:

1. acquire the existing requester/shared-owner mutex;
2. under that mutex, perform the existing unique-current requester/rendezvous provider selection for the exact target `DeviceId`;
3. verify the selected provider relationship matches the exact input requester `SessionId` and exact target `DeviceId`;
4. copy only the exact provider-held authenticated requester session and target needed for a fresh non-authorizing start-intent carrier;
5. drop the candidate-publication `AuthorizedRequesterRendezvous` **before** scheduling currentness/policy reauthorization;
6. while the requester mutex remains held, acquire one fresh shared-current authority read;
7. re-run current-registry validation on the provider-held exact requester session + target;
8. resolve the requester-aware policy source again for that freshly validated requester;
9. evaluate exactly `Capability::RequesterRendezvousStart` again;
10. derive the exact terminal-consumption key only from the revalidated/re-authorized requester `SessionId` + target `DeviceId`;
11. drop policy-authorized requester-start provenance as a gate result rather than treating it as scheduling authority;
12. call the scheduling ledger terminal commit while both the requester mutex and current-authority read remain held;
13. after successful terminal insertion, construct the distinct one-shot scheduling authority grant **infallibly**;
14. return from the current-authority closure, releasing the current read;
15. release the requester/shared-owner mutex before any caller I/O, channel send, request construction, dispatcher action or downstream execution.

This preserves the existing requester-mutex-first / current-authority-second lock order.

Because the current-authority read remains held across fresh registry validation, fresh policy reauthorization, terminal ledger insertion and grant construction, current registry/policy authority cannot change between successful reauthorization and terminal consumption.

No network/channel/response I/O is authorized while either authority lock is held.

---

## 6. Selected exact provider relationship rule

The unique-current provider result must match both:

- input requester `SessionId`; and
- input target `DeviceId`.

A unique provider record for the target that belongs to a different requester session is not the selected operation and must fail closed as an operation-identity mismatch.

The candidate-publication grant itself must:

- remain local to the derivation method;
- never be returned;
- never be stored;
- never be converted into the scheduling grant;
- never be used to authorize candidate publication from this path;
- be dropped before fresh scheduling registry/policy checks.

The scheduling path reuses only the provider's already-proven unique-current classification and exact server-held relationship facts, not candidate-publication authority.

Candidate-publication record lifecycle remains unchanged. Derivation performs no retire/remove operation.

---

## 7. Selected fresh registry and policy reauthorization

After exact provider uniqueness and operation-key match, derivation must create one fresh non-authorizing requester/rendezvous start-intent from the exact provider-held authenticated requester session and target.

That fresh intent must pass the existing registry validator again.

The exact requester-aware policy source must then be resolved again from the freshly validated requester dimensions.

The existing policy admission must evaluate exactly:

`Capability::RequesterRendezvousStart`

again.

No new `Capability` variant is selected.

The existing principal-agnostic shared-current capability evaluator is not silently substituted for the dedicated requester-aware policy source.

Policy success is a prerequisite gate only. The `PolicyAuthorizedRequesterRendezvousStart` carrier is not scheduling authority and must not be returned as such.

---

## 8. Selected terminal-consumption commit semantics

Terminal scheduling-consumption identity remains exactly:

`requester SessionId + target DeviceId`.

Commit ordering remains:

1. duplicate check;
2. capacity check;
3. insertion.

Selected derivation failure classification after construction:

- `AlreadyConsumed` -> no new grant;
- `CapacityExhausted` -> no new grant;
- constructor-only `InvalidCapacity` is not a normal derivation outcome because a shared authority cannot exist unless ledger construction already succeeded.

If an impossible ledger-construction classification is nevertheless observed through internal representation plumbing, the future composition must fail closed as an explicit invariant-state failure; it must not panic, default, reset or rematerialize the ledger.

Successful insertion remains the irreversible producer-side scheduling-authority consumption point.

No rollback to eligible is permitted after successful insertion.

---

## 9. Selected derived scheduling grant

A distinct private/crate-internal one-shot scheduling authority grant is selected.

Semantic contents are exactly:

- requester `SessionId` that formed the terminal-consumption key;
- exact target logical `DeviceId` that formed the terminal-consumption key.

The grant must be:

- neither `Copy` nor `Clone`;
- constructible only inside the successful post-commit derivation path;
- infallibly constructed after ledger insertion;
- non-reusable;
- free of endpoint/IP/transport identity;
- free of candidate/reachability payload;
- free of PRWM request ID;
- free of admission `SessionId`;
- free of timing values;
- free of dispatcher/channel/sender custody.

The requester `SessionId` is authority provenance, not an admission-session identifier.

The target `DeviceId` is logical scheduling scope, not transport endpoint authority.

The grant is not yet an expected-device admission request and does not send anything.

---

## 10. Selected derivation error surface

The future dormant derivation composition may introduce one bounded error classification preserving these stages:

1. provider unique-current selection failure:
   - `Missing`;
   - `StaleOrRetired`;
   - `Ambiguous`;
   - `UnavailableOrIndeterminate`;
2. exact operation identity mismatch;
3. current-registry validation failure;
4. requester-aware policy-source failure;
5. requester-start policy denial;
6. terminal consumption `AlreadyConsumed`;
7. terminal consumption `CapacityExhausted`;
8. explicit impossible consumption-state invariant classification if required to avoid panic/default on an unreachable constructor-only state.

Errors must remain fail-closed and non-authorizing.

No error may expose credentials, transport identity, endpoint data, configured environment contents, request payloads, private keys or policy contents.

No error authorizes retry, remint, ledger reset, record replacement, provider cleanup, candidate publication, or receiver admission.

---

## 11. Registration and acknowledgement semantics remain separate

NR selects **no change** to the existing DR registration result:

`Result<(), RequesterRendezvousStartCompositionError>`.

NR selects **no change** to existing requester/rendezvous acknowledgement framing or response I/O.

A future caller-integration checkpoint must preserve:

- existing registration success/failure truth independently; and
- scheduling derivation success/failure as a separate internal channel.

A scheduling derivation failure after registration success must not retroactively relabel the registration as failed.

A derivation failure does not roll back or remove the current requester/rendezvous record.

A derivation success does not retire or remove the requester/rendezvous record.

The exact peer-visible policy for any future scheduling-derivation failure remains separately gated.

---

## 12. Post-commit downstream failure rule

After successful terminal ledger insertion and one-shot grant creation:

- dropping the grant does not reopen eligibility;
- future request-construction failure does not reopen eligibility;
- future channel-send failure does not reopen eligibility;
- receiver rejection does not reopen eligibility;
- candidate-publication failure does not reopen eligibility;
- process shutdown does not create remint/replay authority;
- no automatic retry/reconnect/remint is implied.

Any future downstream behavior must consume the one-shot grant under a separately reviewed gate.

---

## 13. First source-successor path ceiling

The first separately gated source successor may modify exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`

Allowed changes are limited to dormant composition:

- one private/crate-internal one-shot scheduling authority grant carrier;
- one bounded derivation error classification;
- one post-registration dormant derivation method on `SharedRequesterRendezvousAuthority`;
- exact provider unique-current witness confinement and operation-key match;
- fresh current-registry validation using existing validator;
- fresh requester-aware policy-source resolution using existing source;
- exact requester-start policy reauthorization using existing admission;
- narrow internal ledger commit visibility/wrapper required to commit exact requester-session + target identity without exposing the raw ledger;
- infallible post-commit grant construction;
- focused tests for exact order/fail-closed/key/cardinality semantics.

The successor must not modify:

- requester/rendezvous retained-custody continuation;
- acknowledgement wire/response mapping;
- provider/bridge API;
- candidate-publication runtime owner;
- current-authority owner;
- Linux bootstrap;
- higher-owner custody;
- expected-device admission request types;
- sender/channel wiring;
- runtime/listener/executable activation.

If the dormant composition proves it requires a second source path, the source successor must STOP and return to a fresh documentation gate.

---

## 14. Explicitly forbidden shortcuts

Not selected or authorized:

- new policy `Capability` variant;
- using raw `Decision::Allow` as scheduling authority;
- returning or storing `AuthorizedRequesterRendezvous` as scheduling authority;
- candidate-publication lifecycle mutation for scheduling;
- treating provider target lookup alone as requester identity;
- skipping exact requester-session match;
- skipping fresh registry revalidation;
- skipping fresh requester-aware policy-source resolution;
- skipping fresh exact `RequesterRendezvousStart` policy evaluation;
- committing before provider uniqueness;
- committing before fresh registry/policy gates;
- constructing a fallible grant after terminal commit;
- rollback/remint after commit;
- ledger reset/eviction/TTL/cleanup;
- second mutex;
- provider lifecycle reuse as scheduling consumption;
- receiver `DuplicateActiveDevice` as producer consumption;
- request/correlation ID as operation identity;
- endpoint/IP/transport identity as scheduling identity;
- SessionId allocation for expected-device admission;
- authentication PRWM request-ID allocation;
- verifier/admission timing selection;
- expected-device request construction;
- expected-device channel/sender creation;
- dispatcher execution;
- requester acknowledgement behavior change;
- runtime/listener/main/run activation;
- persistence/database/control-plane mutation;
- deployment/restart;
- merge/PR close/ready-for-review conversion;
- branch deletion/force update/history rewrite.

---

## 15. Closed selection

If validated and evidence-closed, C03e-NR selects:

`POST_REGISTRATION_DORMANT_SCHEDULING_AUTHORITY_DERIVATION_SELECTED / REQUESTER_MUTEX_THEN_CURRENT_AUTHORITY_READ_ORDER_PRESERVED / EXISTING_PROVIDER_UNIQUE_CURRENT_SELECTION_USED_ONLY_AS_EPHEMERAL_RELATIONSHIP_WITNESS / CANDIDATE_PUBLICATION_GRANT_AUTHORITY_NOT_REUSED / EXACT_REQUESTER_SESSION_AND_TARGET_OPERATION_KEY_MATCH_REQUIRED / FRESH_REGISTRY_AND_REQUESTER_POLICY_REAUTHORIZATION_REQUIRED / TERMINAL_CONSUMPTION_COMMIT_PRECEDES_INFALLIBLE_ONE_SHOT_GRANT_CONSTRUCTION / REQUESTER_DR_RESULT_AND_ACKNOWLEDGEMENT_SEMANTICS_UNCHANGED / ONE_FILE_SOURCE_MATERIALIZATION_SEAM_SELECTED / SOURCE_MATERIALIZATION_BLOCKED`

Meaning:

- derivation is a separate post-registration internal operation;
- current provider uniqueness must first resolve exactly one relationship for the target and match the original requester-session/target operation key;
- candidate-publication grant authority never becomes scheduling authority and is dropped before scheduling reauthorization/commit;
- current registry and requester policy are explicitly rechecked after provider state is proven;
- terminal ledger insertion is the irreversible authority-consumption point;
- the scheduling grant is a distinct non-cloneable post-commit carrier;
- existing DR result/acknowledgement semantics remain untouched;
- first source materialization remains dormant and one-file only.

---

## 16. Validation requirements for C03e-NR

C03e-NR itself is documentation-only.

Exact-final-head validation must bind only to the final NR head.

At minimum:

- locked dependency graph: PASS;
- formatting: PASS;
- Clippy: PASS;
- workspace tests: PASS;
- workspace build: PASS.

Path-filtered workflows must be recorded exactly as observed. `SKIPPED` is not `PASS`.

No Android PASS may be claimed unless an exact-NR-head Android workflow actually runs and succeeds.

Durable closure requires immutable canonical Drive evidence with raw byte-count and SHA-256 readback verification.

---

## 17. Next blocking boundary after NR

After durable NR closure, the next separately gated source boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_DERIVATION_COMPOSITION_SOURCE_MATERIALIZATION`

Likely branch name, subject to a fresh successor check:

`phase-152-c03e-ns-production-requester-rendezvous-expected-device-scheduling-authority-derivation-composition-source-materialization`

That source successor is bounded to the one exact Rust path in Section 13.

Even after that dormant composition materializes, a fresh documentation gate is still required before caller/result-custody integration into the requester lifecycle, acknowledgement/peer-visible policy, expected-device request construction, channel send or runtime activation.

---

## STOP

C03e-NR selects only derivation composition semantics.

No Rust/source/runtime materialization is authorized in this checkpoint.