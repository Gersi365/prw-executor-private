# C03e-IH — Production Reachability Requester/Rendezvous Process-Companion Custody Join Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_IH_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_CUSTODY_JOIN_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_CUSTODY_JOIN_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-IG head:

`c0bef6d4431178dcbade9add820b72fde35c58fb`

Exact predecessor tree:

`bff1cd5ad8c636a0d06a835bc2efdfb76f810f90`

Exact predecessor `crates/prw-agent/src/linux_bootstrap.rs` blob:

`e6719fc41031a6150b48e541b6b767906208d49a`

IG gate:
`C03E_IG_PRODUCTION_REACHABILITY_PROCESS_COMPANION_PUBLISH_DRIVE_SOURCE_MATERIALIZED`

IG closure:
`CLOSED_PRODUCTION_REACHABILITY_PROCESS_COMPANION_PUBLISH_DRIVE_SOURCE_MATERIALIZATION`

IG immutable Drive audit:
`1LYZKCgtMrrMBKOKltY8HQQ2o0lATYQea`

IG materializes a dormant crate-private production reachability process operation. It does not join requester/rendezvous custody and does not activate an executable caller.

## 2. Why IH is the next bounded seam

The closed IF contract explicitly kept requester/rendezvous custody separate from the production reachability sibling and required any later join to be selected separately after the production process operation existed.

IG now satisfies that prerequisite.

IH therefore selects only the smallest ownership join between:

1. the already-materialized production reachability process-operation input owner; and
2. the already-materialized requester/rendezvous policy-source and provider-runtime custody values.

This checkpoint selects no requester/rendezvous authorization execution, no provider mutation, no production input population, and no executable activation.

## 3. Existing production process-operation evidence

At the exact IG head, `linux_bootstrap.rs` already contains the crate-private non-cloneable owner:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    peer: PeerConnectivityIdentity,
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
}
```

and the crate-private factory:

```text
linux_agent_production_reachability_remote_process_operation(inputs)
 -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

Factory construction is side-effect free. The returned operation performs production credential/provider/bootstrap/bind/lifecycle work only if a later caller invokes it.

The exact invocation order remains:

```text
one RemoteSessionExecutorRuntime
 -> production reachability bootstrap from existing systemd credential custody seam
 -> same-executor production endpoint startup
 -> exact shutdown-controller publication
 -> production endpoint lifecycle drive with durable production-owner custody retained
```

IH does not change that order.

## 4. Existing requester/rendezvous process-operation custody evidence

At the exact IG head, `linux_bootstrap.rs` also retains the earlier crate-private owner:

```text
LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E> {
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
}
```

This wrapper currently preserves requester/rendezvous authority values by value for operation lifetime and delegates to the existing public two-role remote process operation without invoking requester/rendezvous behavior.

IH does not mutate this existing wrapper or its factory.

## 5. Existing requester-policy source evidence

Exact source path:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

Exact IG blob:

`f7377011a3ab2034c14d9018a5c0f268f6660ffa`

`BoundedRequesterRendezvousStartPolicySource` is an already-populated bounded requester-principal-indexed policy source. Its request-time authority lookup requires the exact authenticated requester dimensions and fails closed when policy is unavailable or indeterminate.

IH selects no policy population, default policy, refresh, persistence, watch, synchronization, registry derivation or environment/config source.

## 6. Existing requester/rendezvous runtime-owner evidence

Exact source path:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

Exact IG blob:

`082a70af239972a82318f3e17cb3fd8cb45d9e95`

`CandidatePublicationRequesterRendezvousRuntimeOwner` already owns exactly one configured process-local requester/rendezvous authority provider by value. Its narrow crate-internal methods cover already-selected registration, current-grant selection and committed lifecycle cleanup seams.

IH selects none of those methods for invocation.

## 7. Selected production requester/rendezvous custody owner

IH selects one new crate-private non-cloneable carrier in `linux_bootstrap.rs`:

```text
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
> {
    production_inputs:
        LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source:
        BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_runtime_owner:
        CandidatePublicationRequesterRendezvousRuntimeOwner,
}
```

The new owner consumes exactly these three already-typed values by value.

It owns no raw requester policy bindings, registry, provider client, credential, certificate, key, endpoint string, runtime handle, synchronization primitive or task handle.

Construction performs ownership composition only.

## 8. Selected custody-join factory

IH selects one new crate-private factory:

```text
linux_agent_production_reachability_requester_rendezvous_remote_process_operation(inputs)
 -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

Factory construction must:

1. destructure only the selected typed owner;
2. construct the existing IG production operation exactly once from `production_inputs`;
3. retain the requester-policy source and requester/rendezvous runtime owner by value in the returned one-shot closure.

Factory construction performs no I/O and invokes no policy/provider/runtime behavior.

## 9. Selected invocation custody law

When a future separately gated caller invokes the returned join operation, IH selects exactly this custody-only sequence:

```text
retain requester_rendezvous_runtime_owner
retain requester_rendezvous_start_policy_source
retain existing IG production operation
 -> explicitly release requester/rendezvous custody values without invoking them
 -> delegate exactly once to the existing IG production operation
```

The requester/rendezvous values are retained through operation construction and until operation invocation. IH does not reinterpret that lifetime retention as requester/rendezvous activation.

No requester/rendezvous method is called by the selected join.

## 10. Why the join remains dormant

The selected join intentionally mirrors the earlier requester/rendezvous process-operation custody precedent: ownership is made compatible with the process-companion operation lifetime without yet choosing the downstream callsite that executes requester/rendezvous start authority.

IH therefore does not:

- call `RequesterRendezvousStartPolicySource::evaluator_for_requester(...)`;
- call requester/rendezvous registry validation;
- invoke the requester-aware policy admission seam;
- register a requester/rendezvous start;
- select a current requester/rendezvous grant;
- mutate or clean up requester/rendezvous provider state;
- create or consume `RequesterRendezvousTargetIntent`;
- invoke the separately materialized requester/rendezvous DR path;
- alter candidate-publication authorization.

A later separately selected checkpoint must choose any actual requester/rendezvous execution path.

## 11. Production operation remains authoritative

The new join must delegate to the exact IG production factory, not duplicate its composition logic and not fall back to the existing public two-role factory.

Therefore the production operation continues to own:

```text
PeerConnectivityIdentity
+ LinuxAgentRemoteProcessOperationInputs
```

and continues to preserve:

```text
one executor
 -> production bootstrap
 -> same-executor production endpoint bind
 -> shutdown-controller publication
 -> production endpoint drive
```

No requester/rendezvous custody value may replace or alter the production peer identity, bind address, admission request identity, capability authority, session authentication service or endpoint lifecycle inputs.

## 12. Existing public two-role path remains frozen

IH selects no mutation to:

```text
LinuxAgentRemoteProcessOperationInputs
linux_agent_remote_process_operation(...)
LinuxAgentRequesterRendezvousRemoteProcessOperationInputs
linux_agent_requester_rendezvous_remote_process_operation(...)
```

Existing public/two-role callers remain source-compatible and behaviorally unchanged.

The new production/requester join remains crate-private.

## 13. Identity invariants

IH preserves the established identity law:

- `PeerConnectivityIdentity` is logical connectivity identity, not a fixed IP address;
- dynamic IP/port is transient reachability only;
- requester identity comes only from authenticated application-session custody in the existing requester/rendezvous chain;
- requester/rendezvous target identity remains explicit caller-nominated logical `DeviceId` until its existing validation/authorization chain establishes authority;
- repeated-admission `expected_device_id` is not rendezvous target identity;
- candidate publisher identity is not rendezvous target identity;
- `SessionId`, transport identity, request IDs, PRWM `request_id`, endpoint addresses and candidate addresses are not substitute logical device identity;
- PRWM `request_id` remains correlation only.

The custody join derives no identity from any reachability endpoint.

## 14. Authority separation

IH preserves the following separate authorities:

1. existing generic capability authority in `LinuxAgentRemoteProcessOperationInputs`;
2. session authentication service;
3. production reachability live-owner authority and durable snapshot custody;
4. requester-aware start-policy source;
5. private requester/rendezvous runtime provider custody.

The new join does not merge these into one generic authority object and exposes no raw provider or policy backing.

## 15. Failure and publication law

Because the selected join delegates unchanged to the IG production operation, all existing IG failure and controller-publication laws remain authoritative.

Before IG production operation invocation, releasing requester/rendezvous custody values cannot create fallback behavior.

Production executor/bootstrap/endpoint startup failures still suppress controller publication and lifecycle drive exactly as IG selected.

`ReceiverGoneShutdownRequested` still enters the production endpoint drive once so orderly shutdown can be observed while durable production owner custody remains retained.

IH selects no new retry, fallback, replacement runtime, re-bootstrap, rebind or two-role downgrade.

## 16. No executable activation

IH explicitly does not select a caller from:

- `run_with_remote_process_companion(...)`;
- `run()`;
- `main.rs`;
- signal-aware readiness construction;
- systemd service startup.

At the exact IG head, `run()` still enters only the existing local signal-aware runtime, and `main.rs` still performs device-identity custody preflight then calls `linux_bootstrap::run()`.

IH leaves both unchanged.

## 17. No production input/source assembly

IH does not select construction of any real production process input.

Still separately gated:

- loading the remote bind address for executable production use;
- selecting/constructing the exact production `PeerConnectivityIdentity` source;
- constructing the generic capability authority;
- constructing the session authentication service;
- producing expected-device admission requests;
- constructing dispatcher/timing/callback values;
- populating the requester-aware policy source;
- constructing the requester/rendezvous provider runtime owner;
- choosing requester/rendezvous provider capacity/lifecycle population provenance.

## 18. First source-successor ceiling

The first source-materialization successor after IH is authorized to modify exactly one file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized changes are only:

1. one crate-private production/requester-rendezvous custody owner with the exact three typed owned fields selected above;
2. one crate-private side-effect-free join factory returning the existing one-shot process-operation shape;
3. exact delegation to the existing IG production operation without duplicating production composition;
4. explicit non-invoking retention/release of requester-policy source and requester/rendezvous runtime owner;
5. bounded no-I/O type/lifetime/delegation tests.

If compilation requires any second source path, public API widening, policy/provider invocation or lifecycle behavior change, the successor must stop and select a separate extension checkpoint rather than silently widening scope.

## 19. Test obligations for first source successor

Tests must perform no real credential read, provider connection, endpoint bind, listener activation, policy evaluation, registry lookup, requester/rendezvous provider mutation or network I/O.

Required evidence:

1. the new carrier consumes the exact IG production input owner plus the exact requester-policy source and requester/rendezvous runtime owner by value;
2. the new factory returns exact `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` shape;
3. factory construction performs no I/O;
4. requester/rendezvous custody values are retained in the returned operation lifetime and are not invoked;
5. exact delegation occurs once to the existing IG production operation;
6. existing IG production operation remains callable unchanged;
7. existing requester-only custody wrapper remains callable unchanged;
8. existing public two-role factory remains callable unchanged;
9. no generic authority/provider/runtime escape is added;
10. workspace validation remains green.

## 20. Explicitly deferred execution/activation

IH does not select:

- requester/rendezvous policy evaluation from the production operation;
- requester/rendezvous provider mutation from the production operation;
- target-intent production or wire ingress;
- requester/rendezvous DR execution from the production operation;
- candidate publication activation;
- NAT traversal or peer dialing;
- production input population;
- `run_with_remote_process_companion(...)` invocation from `run()`;
- `run()` mutation;
- `main.rs` mutation;
- readiness/listener publication semantics;
- systemd `LoadCredential=` wiring;
- credential/certificate/trust/RBAC provisioning;
- service restart or deployment.

Every such boundary remains separately gated.

## 21. Explicit non-authorization

IH authorizes no:

- Rust source mutation in IH itself;
- requester/rendezvous authorization execution;
- provider mutation;
- registry mutation;
- production runtime activation;
- public API widening;
- replacement/removal of the existing two-role operation;
- generic runtime/future-driving API;
- new task/thread/background runtime;
- second Tokio runtime;
- retry/fallback/re-bootstrap/rebind;
- readiness publication;
- candidate publication/traversal activation;
- peer dialing;
- durable-owner mutation;
- credential/certificate/trust/RBAC creation or mutation;
- systemd unit/package mutation;
- deployment/restart;
- repository visibility/configuration change;
- merge;
- branch deletion.

The IH PR must remain draft/open/unmerged after closure.