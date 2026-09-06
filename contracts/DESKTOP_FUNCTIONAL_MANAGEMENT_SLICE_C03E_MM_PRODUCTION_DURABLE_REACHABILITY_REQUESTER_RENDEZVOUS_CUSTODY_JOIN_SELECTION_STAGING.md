# C03e-MM — Production Durable Reachability Requester/Rendezvous Custody Join Selection

## Status

`SELECTION — VALIDATION_PENDING`

## Gate

`C03E_MM_PRODUCTION_DURABLE_REACHABILITY_REQUESTER_RENDEZVOUS_CUSTODY_JOIN_SELECTED`

## 1. Exact predecessor authority

Evidence-closed predecessor:

`C03e-ML — Production Requester/Rendezvous Provider Runtime Explicit Non-Zero Capacity Population Source Materialization`

Exact predecessor branch:

`phase-152-c03e-ml-production-requester-rendezvous-provider-runtime-explicit-nonzero-capacity-population-source-materialization`

Exact predecessor head:

`a6459d60f70600f68c133d66f1c3f56c02472848`

Exact predecessor tree:

`d4948b3b1aedae7ed5d84707defc5f48525fcdea`

Exact predecessor production higher-owner blob:

`70ed722bf6e4dbd175437c32c8e1b6f0b7636dfa`

Exact predecessor `linux_bootstrap.rs` blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

C03e-ML is evidence-closed. Its exact-final-head Rust and Android validations are successful, and its immutable Drive audit is recorded at file ID `1acWb-rkOcFsNxY1GWlGZ7wGzO05YiXM0`.

C03e-ML materializes only one dormant helper that converts an explicit caller-owned non-zero-capacity candidate into one empty `CandidatePublicationRequesterRendezvousRuntimeOwner`. It does not consume the C03e-MJ requester-policy carrier, construct the final requester/rendezvous aggregate, add an executable caller, or activate runtime behavior.

## 2. Selection question

C03e-MM asks for the smallest independently resolvable ownership seam after both required requester/rendezvous custody halves are available:

1. the existing C03e-MJ intermediate production owner retaining the pre-requester durable owner plus one fail-closed empty requester-policy source; and
2. one already-populated requester/rendezvous runtime owner of the exact type materialized by C03e-ML.

The smallest next seam is an infallible by-value custody join into already-existing inner and outer production owner types.

C03e-MM does not select another provenance source. It does not select a concrete provider capacity, expected-request producer, timing source, callback policy, executable caller, startup mapping, or runtime activation.

## 3. Existing C03e-MJ intermediate custody authority

Exact source path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

At the exact C03e-ML head, the private non-cloneable intermediate owner already exists:

```text
LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs<
    D, T, F, C, R, E,
>
```

Its two semantic fields are exactly:

1. `LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<ProductionRemoteCapabilityDenyAllPolicy, ...>`; and
2. `BoundedRequesterRendezvousStartPolicySource`.

The existing C03e-MJ population helper constructs the requester-policy source only after the prior production/durable population succeeds and retains the exact source by value without evaluating it.

The source remains fail closed with zero requester bindings.

## 4. Existing pre-requester durable owner authority

The C03e-MJ carrier retains the already-existing private owner:

```text
LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<
    ProductionRemoteCapabilityDenyAllPolicy,
    D, T, F, C, R, E,
>
```

Its two semantic fields are exactly:

1. `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<ProductionRemoteCapabilityDenyAllPolicy, ...>`; and
2. one raw `ProductionDurableCapabilityAuthority`.

The raw durable authority is intentionally not yet wrapped in the higher-owner `Arc` while this pre-requester owner remains incomplete.

C03e-MM selects no new durable-registry/provider bootstrap, no second durable authority, no clone, and no extraction API.

## 5. Existing requester/rendezvous runtime-owner authority

Exact runtime source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

Exact retained source blob in the current lineage:

`082a70af239972a82318f3e17cb3fd8cb45d9e95`

Existing owner:

`CandidatePublicationRequesterRendezvousRuntimeOwner`

owns exactly one configured in-memory requester/rendezvous authority provider by value.

C03e-ML materializes the production adapter that can construct this owner from one explicit caller-owned `usize max_records` while preserving the existing `RequesterRendezvousLifecycleError` unchanged.

C03e-MM accepts an already-successful runtime owner. It therefore does not call the provider constructor, choose capacity, register authority, select a current grant, clean up lifecycle state, or add an error surface.

## 6. Existing inner production requester/rendezvous owner

Exact source path:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact C03e-ML blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

Existing crate-private non-cloneable owner:

```text
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
>
```

contains exactly:

1. `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, ...>`;
2. `BoundedRequesterRendezvousStartPolicySource`;
3. `CandidatePublicationRequesterRendezvousRuntimeOwner`.

Its existing crate-private constructor:

```text
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(
    production_inputs,
    requester_rendezvous_start_policy_source,
    requester_rendezvous_runtime_owner,
)
```

consumes those already-typed values by value and performs ownership composition only.

C03e-MM selects reuse of this constructor exactly once. It does not duplicate or alter its composition.

## 7. Existing outer durable higher-owner

Exact source path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Existing crate-private owner:

```text
LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
>
```

contains exactly:

1. the existing inner `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, ...>`; and
2. exactly one `Arc<ProductionDurableCapabilityAuthority>`.

Its existing constructor consumes the inner requester/rendezvous owner plus one raw durable capability authority and creates the one outer `Arc` required for durable operation-lifetime custody.

C03e-MM selects reuse of this constructor exactly once. It selects no additional `Arc`, no clone, no second raw durable authority, and no durable-authority getter.

## 8. Selected custody-join law

C03e-MM selects exactly one future crate-private dormant join helper with this semantic input boundary:

1. one already-populated `LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs<...>`; and
2. one already-populated `CandidatePublicationRequesterRendezvousRuntimeOwner`.

The helper must perform only this ownership sequence:

```text
existing C03e-MJ requester-policy carrier
 -> move out exact pre-requester durable owner
 -> move out exact requester-policy source

exact pre-requester durable owner
 -> move out exact production reachability inputs
 -> move out exact raw durable capability authority

exact production reachability inputs
 + exact requester-policy source
 + exact requester/rendezvous runtime owner
 -> existing LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(...)

resulting exact inner requester/rendezvous owner
 + exact raw durable capability authority
 -> existing LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs::new(...)

 -> exact final dormant durable requester/rendezvous owner
```

Every input is consumed by value exactly once.

## 9. Infallible join law

C03e-MM selects no new error type.

The join accepts only already-constructed typed custody values. All fallible production-source population and requester/rendezvous provider construction remain outside this join.

Therefore the future join helper returns the exact outer owner directly and does not return `Result`.

No retry, fallback, partial owner, alternate runtime owner, alternate policy source, synthetic authority or recovery path is selected.

## 10. Exact generic authority law

The C03e-MJ intermediate owner already fixes its current capability authority to:

`ProductionRemoteCapabilityDenyAllPolicy`.

The future join must preserve that exact type into the inner and outer requester/rendezvous production owners.

C03e-MM does not generalize the helper over a new arbitrary policy parameter and does not replace the fail-closed current capability baseline with an allow-bearing policy.

The separate requester/rendezvous start policy source remains an independent authority lane.

## 11. Requester-policy source remains uninvoked

The exact requester-policy source remains:

`BoundedRequesterRendezvousStartPolicySource`

with zero bindings under the currently selected production baseline.

The selected join does not call `evaluator_for_requester(...)`, does not construct a requester binding, does not substitute `RequesterRendezvousStartPolicy::deny()`, and does not interpret empty-source custody as authorization.

Ownership retention is not policy execution.

## 12. Requester/rendezvous runtime owner remains uninvoked

The selected join does not call runtime-owner operations for:

- requester/rendezvous registration;
- current-grant selection;
- committed lifecycle cleanup.

It does not inspect provider contents and does not expose raw provider access.

An empty runtime owner remains empty after the join.

Ownership retention is not requester/rendezvous authorization or candidate-publication activation.

## 13. Durable capability authority remains distinct

The raw `ProductionDurableCapabilityAuthority` retained by the pre-requester owner remains distinct from:

- current fail-closed capability authority;
- requester/rendezvous start-policy authority;
- requester/rendezvous runtime/provider authority;
- session authentication authority;
- logical peer identity;
- endpoint/reachability data;
- PRWM request correlation.

The selected outer constructor only adapts its lifetime custody into the already-existing `Arc` shape. It does not synchronize, refresh, mutate or reinterpret durable authority.

## 14. Identity and authorization invariants

C03e-MM preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

The join derives no identity from IP address, port, provider record, runtime owner, policy-source map, worker limit, request ID or callback.

PRWM `request_id` remains correlation only.

A typed custody join is not authentication and is not authorization.

Receiving enforcement points and capability-oriented authorization remain unchanged.

## 15. No production source population in MM

C03e-MM does not select or perform:

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
- durable-to-current synchronization;
- provider capacity provenance.

Those remain separately gated.

## 16. No executable caller or activation

C03e-MM does not call or select a caller for:

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

It may add only one minimum crate-private dormant join helper implementing the exact by-value composition selected here.

Conceptual helper shape:

```text
linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_existing_custody(
    requester_policy_inputs,
    requester_rendezvous_runtime_owner,
) -> LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    ProductionRemoteCapabilityDenyAllPolicy,
    ...
>
```

Mechanical naming and formatting may adjust only as required by Rust while preserving the exact semantic boundary.

The source successor may add a narrowly scoped `dead_code` acknowledgement if required by the dormant staging pattern.

It must not add a second source path, new public API, new owner type, new error type, provider getter, policy getter, runtime activation, or caller wiring.

## 18. Source-successor construction obligations

The future source materialization must:

1. accept the exact C03e-MJ intermediate owner by value;
2. accept one exact already-populated requester/rendezvous runtime owner by value;
3. destructure the MJ carrier exactly once;
4. destructure the contained pre-requester durable owner exactly once;
5. call the existing inner requester/rendezvous owner constructor exactly once;
6. call the existing outer durable higher-owner constructor exactly once;
7. move the exact raw durable authority into that outer constructor;
8. return the exact resulting outer owner;
9. perform no I/O and invoke no policy/provider/runtime operation;
10. add no invocation site.

If compilation requires any second source path, public API widening, generic extraction interface, error widening, runtime behavior change, or source/provenance selection, the source successor must stop rather than broaden this gate.

## 19. Validation authority

Only the exact final C03e-MM head may serve as validation authority.

Because MM is documentation-only, no Android PASS is required unless Android validation actually triggers.

`SKIPPED` is not PASS.

A successful workflow tied to a superseded head cannot validate a later head.

## 20. Durable evidence discipline

Canonical audit filename:

`C03E_MM_PRODUCTION_DURABLE_REACHABILITY_REQUESTER_RENDEZVOUS_CUSTODY_JOIN_SELECTION_AUDIT_2026-09-06.md`

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

C03e-MM does not select or authorize:

- Rust/source mutation in MM itself;
- a concrete requester/rendezvous provider capacity;
- provider construction in the join;
- provider registration or mutation;
- requester-policy evaluation or positive bindings;
- current requester/rendezvous grant selection;
- cleanup/TTL/timer/persistence;
- a new owner type;
- a new error type;
- expected-request/timing/callback production provenance;
- allow-bearing current capability policy;
- current registry hydration or durable/current synchronization;
- executable input population from production sources;
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

After verified C03e-MM evidence publication: **STOP**.

C03e-MM selects only the infallible by-value custody join between the already-populated C03e-MJ requester-policy carrier and one already-populated C03e-ML requester/rendezvous runtime owner into the already-existing inner and outer production owners.

It does not pre-authorize the source materialization, combined production population, executable caller wiring, startup error mapping or runtime activation.
