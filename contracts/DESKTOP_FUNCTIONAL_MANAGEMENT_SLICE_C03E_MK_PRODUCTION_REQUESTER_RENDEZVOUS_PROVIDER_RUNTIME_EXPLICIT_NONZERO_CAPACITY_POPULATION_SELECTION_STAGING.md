# C03e-MK — Production Requester/Rendezvous Provider Runtime Explicit Non-Zero Capacity Population Selection

## Status

`SELECTION — VALIDATION_PENDING`

## Gate

`C03E_MK_PRODUCTION_REQUESTER_RENDEZVOUS_PROVIDER_RUNTIME_EXPLICIT_NONZERO_CAPACITY_POPULATION_SELECTED`

## 1. Exact predecessor authority

Evidence-closed predecessor:

`C03e-MJ — Production Requester/Rendezvous Start Policy-Source Fail-Closed Empty Population Source Materialization`

Exact predecessor branch:

`phase-152-c03e-mj-production-requester-rendezvous-start-policy-source-fail-closed-empty-population-source-materialization`

Exact predecessor head:

`7f754af9d13fd9e959859e0f82e2469967fe6b59`

Exact predecessor tree:

`e009de599db29fc206d38b44bd227eace567d3aa`

Exact predecessor production higher-owner blob:

`8a6d8be2f2ef5fbba42695fe75888e65218164e7`

C03e-MJ is evidence-closed and materializes only one private intermediate production owner retaining the existing C03e-MH pre-requester durable owner beside exactly one empty fail-closed `BoundedRequesterRendezvousStartPolicySource` plus one dormant population wrapper. Provider/runtime custody, provider capacity, provider registration, final requester/rendezvous join, expected-request ingress, timing, callbacks and executable caller wiring remain separately gated.

## 2. Selection question

C03e-MK asks for the smallest independently resolvable requester/rendezvous provider-runtime production population boundary after C03e-MJ without inventing a production capacity constant, consuming the MJ requester-policy carrier, registering authority, constructing the final requester/rendezvous aggregate, or activating runtime behavior.

Fresh exact-source audit shows that provider construction and runtime-owner custody can be composed deterministically when one capacity is supplied explicitly by the future caller.

The exact numeric production capacity itself is not independently resolvable from current source authority and therefore remains caller-owned.

## 3. Exact provider authority

Exact source:

`crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`

Exact C03e-MJ blob:

`d01cfbc37433f6099e216397b9bf243aa55c53bc`

Existing provider:

`InMemoryRequesterRendezvousAuthorityProvider`

is explicitly documented as caller-owned, bounded and process-local.

Its state contains exactly:

- caller-supplied `max_records: usize`;
- process-local requester/rendezvous records.

The provider retains no:

- persistence;
- clock or TTL;
- synchronization primitive;
- transport identity;
- candidate-publication payload;
- request ID;
- runtime handle.

## 4. Explicit non-zero capacity law

Existing constructor:

`InMemoryRequesterRendezvousAuthorityProvider::new(max_records)`

creates an empty provider with one explicit finite non-zero record bound.

The constructor rejects exactly zero capacity with existing:

`RequesterRendezvousLifecycleError::InvalidCapacity`

and performs no fallback, clamp or substitution.

Capacity exhaustion during later registration is separately represented by existing:

`RequesterRendezvousLifecycleError::CapacityExhausted`.

C03e-MK therefore selects only the existing constructor validation law. It does not select any concrete numeric capacity.

## 5. Exact runtime-owner authority

Exact source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

Exact C03e-MJ blob:

`082a70af239972a82318f3e17cb3fd8cb45d9e95`

Existing owner:

`CandidatePublicationRequesterRendezvousRuntimeOwner`

owns exactly one already-configured `InMemoryRequesterRendezvousAuthorityProvider` by value.

Existing constructor:

`CandidatePublicationRequesterRendezvousRuntimeOwner::new(provider)`

performs lifetime-custody transfer only.

Its source contract explicitly states that provider capacity and lifecycle state are established by the caller before owner construction.

The constructor performs no registration, authorization, I/O, task creation or readiness publication.

## 6. No production capacity inference from tests

Existing tests instantiate provider capacities including `1` and `2` only to exercise constructor, lifecycle and capacity behavior.

C03e-MK does not promote any test value to production provenance.

No `1`, `2`, registry maximum, worker limit, expected-request channel capacity or other observed bound is selected as requester/rendezvous provider capacity.

The future caller must supply one explicit `usize` value.

Zero remains invalid through the existing provider constructor.

## 7. Why provider/runtime population is independently selectable now

C03e-MJ already separates requester-policy-source custody from provider/runtime custody.

Provider/runtime population requires only:

1. one explicit caller-owned capacity value;
2. the existing provider constructor;
3. the existing runtime-owner by-value constructor.

It does not require:

- an authenticated requester;
- a requester policy binding;
- requester-policy evaluation;
- a target publisher `DeviceId`;
- provider registration;
- expected-request ingress;
- admission timing;
- callback policy;
- durable capability authority;
- final Linux production aggregate construction.

Therefore provider/runtime population can be selected as its own dormant production boundary without prematurely resolving later authority or execution seams.

## 8. Empty provider is custody, not authorization

A newly constructed provider contains zero requester/rendezvous records.

Constructing that empty provider and transferring it into `CandidatePublicationRequesterRendezvousRuntimeOwner` does not authenticate a requester and does not grant requester/rendezvous or candidate-publication authority.

Existing authorization lookup against an empty provider remains fail-closed through existing `RequesterRendezvousAuthorityError::Missing` semantics.

C03e-MK selects no lookup call and creates no grant during population.

## 9. MJ requester-policy source remains distinct

Exact requester-policy source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

Exact C03e-MJ blob:

`f7377011a3ab2034c14d9018a5c0f268f6660ffa`

MJ retains exactly one empty `BoundedRequesterRendezvousStartPolicySource` in its private intermediate production carrier.

C03e-MK does not consume, clone, inspect, evaluate, populate or join that source.

Policy-source availability and provider/runtime custody remain separate authority lanes.

## 10. Provider registration remains separately gated

Existing runtime-owner mutation:

`register_policy_authorized_requester_rendezvous_start(...)`

accepts only an already policy-authorized provenance carrier and mutates the private provider.

C03e-MK does not call it.

No requester/rendezvous record is created by the selected provider/runtime population boundary.

No registration strategy, requester identity, target publisher identity, duplicate-handling policy beyond existing provider semantics, cleanup trigger or lifecycle schedule is selected.

## 11. Cleanup and current-grant selection remain separately gated

Existing runtime owner exposes narrow crate-internal operations for:

- current grant selection for an exact publisher;
- retire-then-remove cleanup after committed publication.

C03e-MK does not call either operation.

No current grant is selected and no lifecycle record exists to retire or remove during provider/runtime population.

No TTL, timer, cleanup task or automatic eviction policy is introduced.

## 12. No final requester/rendezvous join

C03e-MK does not construct:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`

or:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`.

It does not consume the C03e-MJ intermediate requester-policy carrier.

It does not move the raw durable capability authority into the existing final higher-owner `Arc` custody.

That join remains separately gated after both requester-policy and provider/runtime provenance are independently available.

## 13. Remaining production provenance is untouched

C03e-MK does not select:

- expected-request sender/producer;
- expected-request channel capacity;
- dispatcher source;
- verifier current-time source;
- challenge-validity timing;
- authentication current time;
- application lease;
- completion callback policy;
- rejection callback policy;
- repeated-admission-failure callback policy;
- allow-bearing current capability policy;
- current-registry hydration;
- durable-to-current synchronization.

No synthetic test value is promoted to production authority.

## 14. Selected production provider/runtime population law

C03e-MK selects exactly this future production rule:

1. accept exactly one caller-supplied `usize max_records` by value;
2. call `InMemoryRequesterRendezvousAuthorityProvider::new(max_records)` exactly once;
3. if construction fails, return the existing `RequesterRendezvousLifecycleError` unchanged;
4. do not clamp zero, substitute a default, retry with another value, or derive capacity from any unrelated constant;
5. on provider construction success, move that exact provider by value into `CandidatePublicationRequesterRendezvousRuntimeOwner::new(provider)` exactly once;
6. return the exact runtime owner;
7. perform no provider registration;
8. perform no requester-policy lookup/evaluation;
9. perform no current-grant selection;
10. perform no cleanup;
11. consume no C03e-MJ requester-policy carrier;
12. construct no final requester/rendezvous aggregate;
13. add no invocation site and activate no runtime/listener/readiness/network behavior.

## 15. Error law

The selected composition requires no new error type.

The future population helper must preserve existing:

`RequesterRendezvousLifecycleError`

unchanged.

In particular:

- zero capacity remains `InvalidCapacity`;
- no fallback capacity is attempted;
- no partial runtime owner is returned on failure.

Runtime-owner construction itself is infallible after one valid provider exists.

## 16. Immediate later source-materialization ceiling

A later separately executed source checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

That source checkpoint may add only the minimum crate-private dormant provider/runtime population composition selected here.

Conceptual helper shape:

`linux_agent_production_requester_rendezvous_runtime_owner_from_explicit_nonzero_capacity(max_records: usize) -> Result<CandidatePublicationRequesterRendezvousRuntimeOwner, RequesterRendezvousLifecycleError>`

Mechanical naming may adjust only for Rust naming/formatting while preserving this exact semantic boundary.

The helper must only:

1. construct one provider from the explicit capacity;
2. propagate its existing construction error unchanged;
3. transfer the exact provider by value into one existing runtime owner;
4. return that owner.

It must not add a generic public constructor or raw provider getter to `candidate_publication_requester_rendezvous_runtime.rs`.

## 17. Why the production higher-owner path is selected

The production durable higher-owner module already contains the dormant production population sequence and the private C03e-MJ intermediate custody.

Placing the future composition there keeps this new production provenance adapter crate-private and avoids widening the generic requester/rendezvous runtime-owner API.

C03e-MK itself remains documentation-only and makes zero Rust/source/runtime changes.

## 18. Security and authority invariants

Remain distinct:

- fail-closed `SharedCurrentCapabilityAuthority<ProductionRemoteCapabilityDenyAllPolicy>`;
- current `WorkspaceDeviceRegistry`;
- `SessionAuthenticationService`;
- authenticated `DeviceId` / `AuthenticatedDeviceSession`;
- requester/rendezvous start policy source;
- requester/rendezvous provider/runtime authority;
- raw `ProductionDurableCapabilityAuthority`;
- `ProductionDurableRegistryRuntimeCustody`;
- current `PeerConnectivityIdentity`;
- reachability/socket address;
- PRWM `request_id` correlation.

Provider capacity is a bounded resource configuration, not identity, authentication or authorization.

An empty provider grants nothing.

An empty requester-policy source grants nothing.

Authentication does not itself grant requester/rendezvous or capability authorization.

## 19. Explicit non-selection

C03e-MK does not select or authorize:

- Rust/source mutation in MK itself;
- any concrete provider capacity value;
- provider registration/population with authority records;
- positive requester-policy bindings;
- allow-bearing requester policy;
- requester-policy evaluation;
- current requester/rendezvous grant selection;
- cleanup/TTL/timer/persistence semantics;
- final requester/rendezvous aggregate construction;
- MJ carrier consumption;
- LX companion invocation;
- expected-request producer/channel/capacity;
- dispatcher/verifier-time production source;
- admission timing/clock/lease values;
- callback policy;
- allow-bearing current capability policy;
- current registry hydration;
- durable-to-current registry synchronization;
- executable caller/input assembly;
- `linux_bootstrap.rs`, `main.rs` or `run()` mutation;
- listener/readiness/network activation;
- authentication/authorization/trust weakening;
- identity/address/correlation conflation;
- manifest/lockfile/workflow/Android/packaging/systemd/credential/certificate/trust/RBAC/repository-configuration changes;
- merge;
- ready-for-review conversion;
- deploy;
- restart/recovery;
- PR close;
- branch deletion;
- force update;
- rebase;
- squash;
- history rewrite;
- destructive cleanup.

## 20. Validation authority

Only the exact final C03e-MK head may serve as validation authority.

Skipped workflows are not PASS.

A workflow result from a superseded head is not closure evidence.

## 21. Durable evidence discipline

Canonical audit filename:

`C03E_MK_PRODUCTION_REQUESTER_RENDEZVOUS_PROVIDER_RUNTIME_EXPLICIT_NONZERO_CAPACITY_POPULATION_SELECTION_AUDIT_2026-09-06.md`

Canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

Closure requires:

- exact-title pre-upload uniqueness;
- immutable upload;
- raw byte readback;
- exact byte-count and SHA-256 verification;
- exact-title post-upload uniqueness;
- PR closure metadata update;
- post-publication branch/main/successor guard.

## 22. STOP boundary

After verified C03e-MK evidence publication: **STOP**.

C03e-MK selects only one explicit-capacity provider/runtime population composition that returns an empty runtime owner and preserves caller-owned capacity provenance.

It does not pre-authorize the source materialization, provider registration, final requester/rendezvous join, executable caller wiring or runtime activation.
