# C03e-MO — Production Durable Reachability Requester/Rendezvous Explicit-Capacity Combined Population Selection

## Status

`SELECTION — VALIDATION_PENDING`

## Gate

`C03E_MO_PRODUCTION_DURABLE_REACHABILITY_REQUESTER_RENDEZVOUS_EXPLICIT_CAPACITY_COMBINED_POPULATION_SELECTED`

## 1. Exact predecessor authority

Evidence-closed predecessor:

`C03e-MN — Production Durable Reachability Requester/Rendezvous Custody Join Source Materialization`

Exact predecessor branch:

`phase-152-c03e-mn-production-durable-reachability-requester-rendezvous-custody-join-source-materialization`

Exact predecessor head:

`cf1e86ba6592d71a1649d5abd18baded04dcbdea`

Exact predecessor tree:

`4248a827b5a2acdb7144bd710766d508d81167c7`

Exact predecessor production higher-owner source blob:

`c23169dca2136de51f5ff9834f79fc6a56487cab`

C03e-MN is evidence-closed. Its exact-final-head Rust and Android validations are successful, and its immutable Drive audit is recorded at file ID `1Ej4mCHkawVIZgM_iyBEkfBj8aQuk2hoX` with `13393` bytes and SHA-256 `a0d95499c6094e3dbed0a059b52488c4b0257568866d96aa76f48956b6aee741`.

C03e-MN materializes only the dormant by-value custody join selected by C03e-MM. It does not populate the C03e-MJ requester-policy carrier, choose provider capacity provenance, construct a provider/runtime owner from production configuration, add an executable caller, or activate runtime behavior.

## 2. Selection question

After C03e-MN, the exact source graph already contains three independently validated dormant seams:

1. C03e-MJ can populate one fail-closed requester-policy carrier from existing production sources;
2. C03e-ML can construct one requester/rendezvous runtime owner from one explicit caller-owned `usize max_records`;
3. C03e-MN can join those two already-populated custody halves into the existing final durable requester/rendezvous owner.

The smallest next production-population seam is therefore one dormant wrapper that composes those exact three existing helpers without selecting any concrete source for `max_records`.

C03e-MO selects only that composition contract and its bounded two-source error surface.

It does not select an executable caller, environment/configuration key, default provider capacity, startup mapping, callback values, positive requester policy, current registry hydration, operation invocation, listener/readiness publication, or runtime/network activation.

## 3. Existing C03e-MJ production population authority

Exact source path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Existing helper:

`linux_agent_production_durable_reachability_requester_policy_remote_process_operation_inputs_from_production_sources(...)`

It consumes the already-selected production population inputs:

- `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- admission-timing callback `F`;
- completion callback `C`;
- rejection callback `R`;
- repeated-admission-failure callback `E`.

It returns:

`Result<LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs<...>, LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError>`

Its successful value retains by value:

1. the exact fail-closed pre-requester durable production owner; and
2. one empty `BoundedRequesterRendezvousStartPolicySource`.

C03e-MO selects reuse of this helper exactly once and does not duplicate any of its underlying production-source population.

## 4. Existing C03e-ML explicit-capacity runtime authority

Existing helper:

`linux_agent_production_requester_rendezvous_runtime_owner_from_explicit_nonzero_capacity(max_records)`

Exact input:

`usize max_records`

Exact result:

`Result<CandidatePublicationRequesterRendezvousRuntimeOwner, RequesterRendezvousLifecycleError>`

Zero capacity remains invalid through the existing `InMemoryRequesterRendezvousAuthorityProvider::new(max_records)` constructor.

The helper constructs exactly one empty bounded in-memory provider and moves it by value into exactly one existing runtime owner.

C03e-MO selects no default, minimum clamp, fallback, environment variable, worker-limit alias, channel-capacity alias, hard-coded constant or inferred capacity.

The numeric capacity remains explicit caller-owned provenance.

## 5. Existing C03e-MN custody join authority

Existing helper:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_existing_custody(...)`

It consumes by value:

1. one C03e-MJ requester-policy carrier; and
2. one already-populated `CandidatePublicationRequesterRendezvousRuntimeOwner`.

It destructures existing private custody exactly once, calls the existing inner requester/rendezvous owner constructor exactly once, then calls the existing outer durable higher-owner constructor exactly once.

It is infallible and performs ownership composition only.

C03e-MO selects reuse of this helper exactly once.

## 6. Selected combined population boundary

C03e-MO selects one future crate-private dormant async helper with this semantic input boundary:

1. `expected_requests` with the exact existing MJ receiver type;
2. `admission_timing` with the exact existing MJ callback type;
3. `on_completion` with the exact existing MJ callback type;
4. `on_rejection` with the exact existing MJ callback type;
5. `on_admission_failure` with the exact existing MJ callback type;
6. one explicit caller-owned `usize max_records`.

The selected result is the existing exact final owner:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<ProductionRemoteCapabilityDenyAllPolicy, D, T, F, C, R, E>`

No new owner type is selected.

Conceptual helper shape:

```text
async fn linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_production_sources_with_explicit_nonzero_capacity(
    expected_requests,
    admission_timing,
    on_completion,
    on_rejection,
    on_admission_failure,
    max_records,
) -> Result<
    LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
        ProductionRemoteCapabilityDenyAllPolicy,
        ...
    >,
    LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessInputPopulationError,
>
```

Mechanical naming and formatting may adjust only as required by Rust while preserving the exact semantic boundary.

## 7. Selected construction order

The future helper must preserve this exact staged order:

```text
existing production population inputs
 -> C03e-MJ requester-policy population helper exactly once
 -> if failure: return bounded production-source error and STOP

successful C03e-MJ requester-policy carrier
 + exact caller-owned max_records
 -> C03e-ML explicit-capacity runtime-owner helper exactly once
 -> if failure: return bounded requester/rendezvous runtime-construction error and STOP

successful C03e-MJ carrier
 + successful C03e-ML runtime owner
 -> C03e-MN custody join exactly once
 -> exact final dormant durable requester/rendezvous owner
```

The provider/runtime construction therefore occurs only after the existing production-source population has succeeded.

If C03e-ML fails, the already-produced MJ carrier is dropped normally; no partial outer owner is returned and no rollback/retry path is selected.

The final MN join remains infallible.

## 8. Selected bounded composite error

The combined helper has exactly two fallible upstream stages, so C03e-MO selects one new crate-private bounded composite error type whose semantic variants are exactly:

1. existing `LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError` from the C03e-MJ stage; and
2. existing `RequesterRendezvousLifecycleError` from the C03e-ML stage.

Conceptual type name:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessInputPopulationError`

Conceptual variants:

```text
ProductionSources(LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError)
RequesterRendezvousRuntime(RequesterRendezvousLifecycleError)
```

Mechanical variant naming may adjust only for clarity while preserving the exact two-source classification.

The source successor may implement the ordinary bounded error plumbing required by Rust:

- `Display`;
- `std::error::Error` with the exact underlying source preserved;
- `From<LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError>`;
- `From<RequesterRendezvousLifecycleError>`.

No third error variant, opaque boxed error, string-only erasure, retry classification, recovery variant or startup mapping is selected.

## 9. Error precedence law

Error precedence follows the selected construction order.

If the C03e-MJ population stage fails, C03e-ML is not called and no requester/rendezvous provider/runtime owner is constructed.

Only after C03e-MJ succeeds may the explicit-capacity C03e-ML stage run.

If C03e-ML rejects zero capacity or otherwise returns its existing lifecycle error, C03e-MN is not called.

The existing underlying errors remain inspectable through the composite source chain.

No error is swallowed, translated into success, defaulted, retried or replaced with a synthetic fallback.

## 10. Explicit-capacity provenance law

`max_records` remains an explicit `usize` argument supplied by a future caller.

C03e-MO does not select where that caller obtains the value.

In particular, C03e-MO does not select:

- an environment variable;
- a systemd credential;
- a command-line argument;
- a config file;
- a database/control-plane value;
- the production worker limit;
- expected-request channel capacity;
- a device/session count;
- a hard-coded literal;
- a default constant;
- a computed heuristic.

A separately gated future checkpoint must select concrete capacity provenance before executable population can be wired.

## 11. Requester-policy authority remains fail closed

The C03e-MJ population stage still constructs exactly one empty `BoundedRequesterRendezvousStartPolicySource` through the already-materialized path.

The combined helper must not:

- add requester bindings;
- call `evaluator_for_requester(...)`;
- substitute an allow-bearing policy;
- interpret an empty policy source as authorization.

Ownership retention is not requester authorization.

## 12. Requester/rendezvous runtime remains empty and uninvoked

The C03e-ML helper constructs one empty bounded provider/runtime owner from explicit non-zero capacity.

The combined helper must not call:

- requester/rendezvous registration;
- current-grant selection;
- committed lifecycle cleanup.

It must not inspect provider contents or expose provider custody.

An empty runtime owner remains empty after C03e-MN joins it into the final owner.

## 13. Current capability authority remains fail closed

The C03e-MJ lineage still fixes current capability authority to:

`ProductionRemoteCapabilityDenyAllPolicy`

C03e-MO does not generalize the combined helper over an arbitrary current policy type and does not replace the deny-all baseline with an allow-bearing policy.

Current capability authority, requester/rendezvous start policy, requester/rendezvous runtime authority, session authentication authority and durable capability authority remain distinct lanes.

## 14. Identity and authorization invariants

C03e-MO preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

The combined helper derives no identity from:

- IP address or port;
- provider capacity;
- provider record index;
- runtime owner;
- policy-source map;
- worker limit;
- PRWM request ID;
- callback state.

PRWM `request_id` remains correlation only.

Typed population/custody is not authentication and is not authorization.

## 15. No new production provenance besides explicit capacity argument

C03e-MO does not select or alter:

- expected-request producer/sender/channel capacity;
- dispatcher provenance;
- verifier current-time source;
- admission challenge validity/current-time/application-lease policy;
- completion callback policy;
- rejection callback policy;
- repeated-admission-failure callback policy;
- positive requester-policy bindings;
- allow-bearing current capability policy;
- current registry hydration;
- durable-to-current synchronization.

The future combined helper accepts the same already-selected MJ inputs and one explicit capacity argument only.

## 16. No executable caller or runtime activation

C03e-MO does not call or select a caller for:

- the future combined population helper;
- `linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation(...)`;
- the production durable capability projection operation;
- `run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_with_production_durable_capability_projection(...)`;
- `run_with_remote_process_companion(...)`;
- `run()`;
- `main.rs`.

No listener bind, readiness publication, endpoint drive, candidate publication, traversal, dialing, reconnect, rebind, service startup or production networking is activated.

## 17. First source-materialization successor ceiling

A later separately executed source checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

It may add only:

1. one minimum crate-private bounded composite error for the two exact upstream failure types selected here; and
2. one minimum crate-private dormant async combined population helper implementing the exact staged composition selected here.

It may add narrowly scoped `dead_code`, `type_complexity` or `future_not_send` acknowledgements only if required by the existing staging/CI pattern.

It must not add a second source path, new public API, new owner type, third error source, provider getter, policy getter, concrete capacity source, caller wiring, startup error mapping or runtime behavior.

If compilation requires any such scope broadening, the source successor must stop rather than broaden this gate.

## 18. Source-successor construction obligations

The future source materialization must:

1. accept the exact existing MJ production-population inputs unchanged;
2. accept one explicit caller-owned `usize max_records` unchanged;
3. call the existing C03e-MJ requester-policy population helper exactly once;
4. return immediately on its existing population error through the new bounded composite error;
5. only after MJ success, call the existing C03e-ML runtime-owner helper exactly once;
6. return immediately on its existing lifecycle error through the new bounded composite error;
7. call the existing C03e-MN custody join exactly once after both preceding stages succeed;
8. return the exact existing final durable requester/rendezvous owner;
9. perform no provider/policy/runtime operation beyond C03e-ML construction;
10. add no invocation site.

## 19. Validation authority

Only the exact final C03e-MO head may serve as validation authority.

Because MO is documentation-only, Rust validation is sufficient if that is the only workflow triggered by the changed path.

If Android validation triggers, its terminal result must be recorded; `SKIPPED` is not PASS.

A successful workflow tied to a superseded head cannot validate a later head.

## 20. Durable evidence discipline

Canonical audit filename:

`C03E_MO_PRODUCTION_DURABLE_REACHABILITY_REQUESTER_RENDEZVOUS_EXPLICIT_CAPACITY_COMBINED_POPULATION_SELECTION_AUDIT_2026-09-06.md`

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

## 21. Explicit non-selection

C03e-MO does not select or authorize:

- Rust/source mutation in MO itself;
- concrete provider-capacity provenance;
- provider registration or mutation;
- requester-policy evaluation or positive bindings;
- current requester/rendezvous grant selection;
- cleanup/TTL/timer/persistence;
- a new owner type;
- any third error category;
- expected-request/timing/callback production provenance changes;
- allow-bearing current capability policy;
- current registry hydration or durable/current synchronization;
- executable caller population;
- operation invocation;
- `run()` or `main.rs` mutation;
- listener/readiness/network activation;
- candidate publication/traversal/dialing/retry/reconnect/rebind/rebootstrap;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- repository visibility/configuration mutation;
- merge, ready-for-review conversion, PR close, branch deletion, force update, rebase, squash, history rewrite or destructive cleanup;
- deployment, restart, recovery or production cutover.

## 22. STOP boundary

After verified C03e-MO evidence publication: **STOP**.

C03e-MO selects only one future dormant combined population wrapper that composes C03e-MJ production requester-policy population, C03e-ML explicit-capacity requester/rendezvous runtime construction and C03e-MN custody join, with one bounded two-source error surface.

It does not pre-authorize the source materialization, concrete capacity provenance, executable caller wiring, startup error mapping or runtime activation.
