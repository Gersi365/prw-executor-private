# C03e-MI — Production Requester/Rendezvous Start Policy-Source Fail-Closed Empty Population Selection

## Status

`SELECTION — VALIDATION_PENDING`

## Gate

`C03E_MI_PRODUCTION_REQUESTER_RENDEZVOUS_START_POLICY_SOURCE_FAIL_CLOSED_EMPTY_POPULATION_SELECTED`

## 1. Exact predecessor authority

Evidence-closed predecessor:

`C03e-MH — Production Remote-Process Current Capability Authority Fail-Closed Population Source Materialization`

Exact predecessor head:

`db31cdae967099ac2a7640620dd9b948f564b652`

Exact predecessor tree:

`2dc708587e21395d92ba41087e8854dc8c461835`

Exact predecessor higher-owner blob:

`655b1c5dfae63d44e597b6b6fd2e870339268c76`

C03e-MH is evidence-closed and materializes only the fail-closed current-capability-authority population wrapper. Expected-request ingress, timing, callbacks, requester/rendezvous production population and executable caller wiring remain separately gated.

## 2. Selection question

C03e-MI asks for the smallest independently resolvable production provenance boundary remaining after C03e-MH.

Fresh exact-source audit rejects immediate production selection of:

- expected-request sender/producer/channel capacity, because one expected request carries logical device/session/correlation/capability plus dispatcher/verifier-time behavior and no concrete production producer is materialized;
- admission timing, because `RemoteSessionRealAdmissionTiming` remains caller-supplied and no production clock/timing source is materialized;
- callbacks, because existing production projection forwards caller-supplied callbacks unchanged and no concrete production callback policy is materialized;
- requester/rendezvous provider capacity/population, because the concrete provider requires one explicit caller-owned finite non-zero capacity and no canonical production capacity source is materialized.

The existing bounded requester/rendezvous start policy source, however, has deterministic fail-closed empty-source semantics independent of provider capacity, request ingress, timing and callbacks.

## 3. Exact requester-policy source authority

Exact source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

Exact C03e-MH blob:

`f7377011a3ab2034c14d9018a5c0f268f6660ffa`

Existing concrete source:

`BoundedRequesterRendezvousStartPolicySource`

is a bounded immutable requester-principal-indexed source.

Its source contract states:

- population is one-shot;
- no live insert/update/remove/replace operation exists;
- no persistence, refresh, watch, lock or distributed-coherence semantics are selected;
- policy lookup receives the exact authenticated requester session;
- lookup must fail closed rather than substitute a process-global/default evaluator or another requester's evaluator.

The source derives `Default`; its only field is an initially empty `HashMap<DeviceId, RequesterRendezvousStartPolicyBinding>`.

## 4. Empty-source fail-closed law

For an empty `BoundedRequesterRendezvousStartPolicySource`, `evaluator_for_requester(...)` cannot resolve any logical requester binding.

The existing lookup path therefore returns:

`RequesterRendezvousStartPolicySourceError::Unavailable`

before policy evaluation or requester/rendezvous provider mutation.

The existing source contract defines that error as:

`No authoritative requester policy is currently available.`

Failure does not select a fallback evaluator.

Therefore an empty source grants no requester/rendezvous-start capability and cannot authorize provider registration.

## 5. This is a new reviewed production selection, not a test inference

Earlier checkpoints deliberately treated `BoundedRequesterRendezvousStartPolicySource::default()` observed in tests as synthetic construction rather than production provenance.

C03e-MI does not retroactively treat those tests as production authority.

Instead, C03e-MI is the first explicit reviewed gate that selects the exact empty-source semantics as a production fail-closed bootstrap baseline.

The selection is grounded in the source-level source contract and exact empty backing semantics, not in test values.

No positive requester binding is inferred from any test.

## 6. Dedicated requester policy remains separate from source availability

Existing:

`RequesterRendezvousStartPolicy`

is a dedicated evaluator for `Capability::RequesterRendezvousStart` and denies every other represented capability.

Existing:

`RequesterRendezvousStartPolicy::deny()`

can construct a deny evaluator, but C03e-MI does not create any requester binding containing that policy.

The selected baseline is stricter:

- zero requester bindings;
- no resolved evaluator;
- lookup returns `Unavailable` for every requester.

C03e-MI therefore does not create a process-global deny evaluator or any logical-principal policy record.

## 7. Requester/rendezvous provider remains independently unresolved

Exact provider source:

`crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`

Exact C03e-MH blob:

`d01cfbc37433f6099e216397b9bf243aa55c53bc`

Existing:

`InMemoryRequesterRendezvousAuthorityProvider::new(max_records)`

requires one explicit finite non-zero `usize` capacity.

The source describes the provider as caller-owned.

It contains no canonical production capacity and selects no production runtime owner, persistence, TTL/clock policy, synchronization primitive, frame loop, listener or production networking.

C03e-MI therefore does not select:

- provider capacity;
- provider construction;
- provider population;
- provider registration lifecycle;
- provider cleanup policy.

## 8. Existing runtime owner does not fill the provider provenance gap

Exact source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

Exact C03e-MH blob:

`082a70af239972a82318f3e17cb3fd8cb45d9e95`

Existing:

`CandidatePublicationRequesterRendezvousRuntimeOwner::new(provider)`

consumes one already-configured `InMemoryRequesterRendezvousAuthorityProvider` by value.

Its source contract explicitly states that provider capacity and lifecycle state are established by the caller before construction.

C03e-MI does not infer a capacity or construct this owner.

## 9. Expected-request ingress remains unresolved

Existing production remote-process inputs retain:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`.

Exact request construction carries at least:

- expected logical `DeviceId`;
- `SessionId`;
- authentication request correlation ID;
- capability;
- request bytes;
- dispatcher `D`;
- verifier current-time function `T`;
- retry bound/configuration;
- remote processing delay range.

The request constructor explicitly derives authentication request correlation from the request correlation value; that correlation remains non-authorizing and is not identity/authentication evidence by itself.

No concrete production expected-request producer is materialized in the audited exact source graph.

Observed `mpsc::channel(..., 1)` construction remains test-only.

C03e-MI selects no sender, producer, capacity, dispatcher or verifier-time source.

## 10. Admission timing remains unresolved

Existing:

`RemoteSessionRealAdmissionTiming`

contains caller-supplied:

- challenge-validity Unix-second range;
- authentication current Unix second;
- application-lease Unix-second range.

Fresh exact-source audit found no production clock/timing source in this lane.

Observed `1..2 / 1 / 1..2` values remain synthetic test data.

C03e-MI selects no timing value, clock, lease or challenge policy.

## 11. Callback policy remains unresolved

Existing production durable projection operations forward completion, rejection and admission-failure callbacks unchanged.

C03e-MI selects no:

- no-op callback;
- logging callback;
- metrics callback;
- retry callback;
- process-exit callback;
- persistence callback.

No test callback is promoted to production provenance.

## 12. Selected production policy-source population law

C03e-MI selects exactly this future production rule:

1. construct exactly one `BoundedRequesterRendezvousStartPolicySource::default()`;
2. treat that exact source as an explicit production fail-closed bootstrap baseline with zero requester bindings;
3. retain that exact source by value beside the already-populated C03e-MH pre-requester durable owner;
4. do not evaluate it during population;
5. do not add any requester binding;
6. do not construct `RequesterRendezvousStartPolicy::deny()` as a binding substitute;
7. do not construct or mutate a requester/rendezvous provider;
8. do not select provider capacity;
9. do not register a requester/rendezvous record;
10. add no invocation site and activate no runtime behavior.

The exact selected policy-source construction is infallible.

## 13. Required intermediate custody law

Because provider provenance is still unresolved, the selected policy source must not be prematurely joined into the existing final requester/rendezvous aggregate.

A future source materialization must retain the exact source in one private non-cloneable intermediate owner beside the existing C03e-MH pre-requester durable owner.

Conceptual carrier shape:

`LinuxAgentProductionDurableReachabilityRequesterPolicyRemoteProcessOperationInputs<...>`

with exactly two private semantic fields:

- existing `LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<...>`;
- exact `BoundedRequesterRendezvousStartPolicySource`.

Mechanical naming may adjust only for Rust naming/formatting while preserving the same ownership boundary.

The carrier must not expose:

- generic getters;
- split/extract APIs;
- `Clone`;
- `Copy`;
- public construction;
- raw policy map access;
- provider access.

## 14. Immediate later source-materialization ceiling

A later separately executed source checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

That checkpoint may add only:

1. the private non-cloneable intermediate requester-policy custody carrier selected above; and
2. one crate-private dormant async wrapper around the existing C03e-MH helper.

The wrapper must:

1. invoke the existing C03e-MH fail-closed current-capability population helper exactly once;
2. return immediately on its existing error;
3. only after MH success, construct exactly one `BoundedRequesterRendezvousStartPolicySource::default()`;
4. move the exact successful MH owner plus exact empty policy source into the new intermediate carrier exactly once;
5. preserve the existing MH population error unchanged;
6. add no invocation site.

No new error variant is selected because empty policy-source construction is infallible.

## 15. Why the policy source is created after MH success

The selected ordering preserves fail-before-next-stage composition:

- existing remote/session/current-capability/peer/durable population must succeed first;
- only then is the new requester-policy source created and retained.

This avoids constructing later-stage authority on a failed earlier population path and keeps ownership lineage mechanically obvious.

## 16. No final requester/rendezvous join yet

C03e-MI does not authorize construction of:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`

or:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`.

Those joins require separately resolved requester/rendezvous runtime-provider custody.

The raw durable capability authority retained by the existing pre-requester owner must remain in its current custody until that later join is separately selected.

## 17. Security and authority invariants

Remain distinct:

- current fail-closed `SharedCurrentCapabilityAuthority<ProductionRemoteCapabilityDenyAllPolicy>`;
- current in-memory `WorkspaceDeviceRegistry`;
- `SessionAuthenticationService`;
- authenticated application-session identity;
- requester/rendezvous start policy source;
- requester/rendezvous provider/runtime authority;
- durable registry runtime custody;
- raw production durable capability authority;
- logical `DeviceId`;
- current `PeerConnectivityIdentity`;
- reachability/socket address;
- PRWM `request_id` correlation.

An empty requester policy source grants nothing.

Authentication does not itself grant requester/rendezvous or capability authorization.

PRWM request correlation is not identity or authorization.

## 18. Explicit non-selection

C03e-MI does not select or authorize:

- Rust/source mutation in MI itself;
- positive requester/rendezvous policy bindings;
- allow-bearing requester/rendezvous policy;
- requester/rendezvous provider capacity;
- requester/rendezvous provider construction/population;
- requester/rendezvous registration;
- provider cleanup/TTL/persistence;
- final requester/rendezvous join;
- expected-request producer/channel/capacity;
- dispatcher/verifier-time production source;
- admission timing/clock/lease values;
- callback policy;
- allow-bearing current capability policy;
- current registry hydration;
- durable-to-current registry synchronization;
- LX companion invocation;
- executable caller/input assembly;
- `linux_bootstrap.rs`, `main.rs` or `run()` mutation;
- listener/readiness/network activation;
- merge, ready-for-review conversion, deploy, restart/recovery, PR close, branch deletion, force update, rebase, squash, history rewrite or destructive cleanup.

## 19. Validation authority

Only the exact final C03e-MI head may serve as validation authority.

Skipped workflows are not PASS.

A workflow result from a superseded head is not closure evidence.

## 20. Durable evidence discipline

Canonical audit filename:

`C03E_MI_PRODUCTION_REQUESTER_RENDEZVOUS_START_POLICY_SOURCE_FAIL_CLOSED_EMPTY_POPULATION_SELECTION_AUDIT_2026-09-06.md`

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

## 21. STOP boundary

After verified C03e-MI evidence publication: **STOP**.

C03e-MI selects only the fail-closed empty requester/rendezvous start policy-source population plus the minimum private intermediate custody required to retain that authority without inventing provider provenance.

It does not pre-authorize the source materialization or any later provider/join/caller boundary.
