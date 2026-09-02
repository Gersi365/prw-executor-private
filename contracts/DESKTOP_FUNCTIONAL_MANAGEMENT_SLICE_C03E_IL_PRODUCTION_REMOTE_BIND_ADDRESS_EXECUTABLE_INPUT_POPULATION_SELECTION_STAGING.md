# C03e-IL — Production Remote Bind-Address Executable Input Population Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_IL_PRODUCTION_REMOTE_BIND_ADDRESS_EXECUTABLE_INPUT_POPULATION_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_REMOTE_BIND_ADDRESS_EXECUTABLE_INPUT_POPULATION_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-IK head:

`e40a7b203927c46c0ead8cbf78b46f28740ff6e0`

Exact predecessor tree:

`c4ad86477b9c99e84a9b56e34610951231855f3f`

Exact predecessor `crates/prw-agent/src/linux_bootstrap.rs` blob:

`db146a724ad7cd21e97b6da9ce6d527d24901c4b`

Exact predecessor `crates/prw-agent/src/main.rs` blob:

`db6b8028c6df100a961a0fb5818347bea2fdc5c1`

IK gate:
`C03E_IK_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_EXECUTABLE_ASSEMBLY_SOURCE_MATERIALIZED`

IK closure:
`CLOSED_PRODUCTION_REACHABILITY_REQUESTER_RENDEZVOUS_PROCESS_COMPANION_EXECUTABLE_ASSEMBLY_SOURCE_MATERIALIZATION`

IK immutable Drive audit:
`1vDQIDdXel67gM7q-PZryTUX3zZ7dxAJ5`

IK materializes the IJ-selected crate-private executable assembly wrapper but leaves all concrete production input population and executable invocation separately gated.

## 2. Why IL is the next bounded seam

At the exact IK head, the executable assembly wrapper now accepts one already-constructed:

```text
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
>
```

and delegates it to the existing II operation and existing process-companion runner.

The wrapper is deliberately dormant and cannot manufacture any of its production inputs.

The exact source already contains one concrete production input source with bounded semantics:

```text
load_linux_agent_remote_bind_addr_from_env()
 -> Result<SocketAddr, LinuxAgentRemoteBindAddressSourceError>
```

No equivalently selected executable-production provenance is present in `linux_bootstrap.rs` for the remaining remote-operation fields or for requester/rendezvous custody.

IL therefore selects only the smallest next input-population boundary: consume already-typed non-bind remote-operation values, load exactly one production remote bind address through the existing bounded loader, and construct the existing `LinuxAgentRemoteProcessOperationInputs<...>` owner.

IL does not select full production input assembly and does not select any executable caller.

## 3. Existing production bind-address source remains authoritative

The exact IK source exposes:

```text
pub fn load_linux_agent_remote_bind_addr_from_env()
 -> Result<SocketAddr, LinuxAgentRemoteBindAddressSourceError>
```

The loader reads only the fixed process configuration name:

```text
PRW_REMOTE_BIND_ADDR
```

Its established law remains unchanged:

- parse directly as `SocketAddr`;
- no DNS lookup;
- no interface enumeration;
- no route inspection;
- no public-address discovery;
- no socket bind;
- no fallback;
- reject absent/empty, non-Unicode, malformed, unspecified, multicast and IPv4 limited-broadcast values;
- allow port `0` as pre-bind configuration;
- return only the existing bounded `LinuxAgentRemoteBindAddressSourceError` classification.

IL does not modify this loader or broaden its source semantics.

## 4. Bind address remains reachability, never identity

IL preserves the established identity invariant:

```text
logical peer/device identity
 -> registry/discovery
 -> transient endpoint candidates
 -> authenticated transport
```

The production remote bind address is only a transient local reachability coordinate.

It is not and must never become:

- `DeviceId`;
- `PeerConnectivityIdentity`;
- `TransportIdentity`;
- requester identity;
- requester/rendezvous target identity;
- authenticated-session identity;
- registry identity;
- authorization identity;
- candidate publisher identity;
- PRWM `request_id` identity.

PRWM `request_id` remains correlation only.

## 5. Existing remote-process input owner remains authoritative

The exact IK source already contains:

```text
LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>
```

with exact fields:

```text
bind_addr: SocketAddr
max_active_workers: NonZeroUsize
capability_authority: SharedCurrentCapabilityAuthority<P>
session_authentication: SessionAuthenticationService
expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>
admission_timing: F
on_completion: C
on_rejection: R
on_admission_failure: E
```

and the existing constructor:

```text
LinuxAgentRemoteProcessOperationInputs::new(...)
```

IL selects no field addition, removal, type change or public API widening to this owner.

## 6. Selected future helper

IL selects one future crate-private helper in:

`crates/prw-agent/src/linux_bootstrap.rs`

with the sole responsibility of populating the existing `bind_addr` field from the existing production loader while consuming all remaining remote-operation values exactly as already typed.

The selected semantic shape is:

```text
linux_agent_remote_process_operation_inputs_from_production_bind_addr(
    max_active_workers,
    capability_authority,
    session_authentication,
    expected_requests,
    admission_timing,
    on_completion,
    on_rejection,
    on_admission_failure,
)
 -> Result<LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
           LinuxAgentRemoteBindAddressSourceError>
```

The exact generic field types must remain those already required by `LinuxAgentRemoteProcessOperationInputs::new(...)`.

No new public type, public function, error envelope or configuration source is selected.

## 7. Exact population law

When a later separately gated caller invokes the selected helper, it must perform exactly:

```text
load_linux_agent_remote_bind_addr_from_env()
 -> bind_addr
 -> LinuxAgentRemoteProcessOperationInputs::new(
        bind_addr,
        already_typed_remaining_inputs...
    )
```

The helper must:

1. call `load_linux_agent_remote_bind_addr_from_env()` exactly once;
2. fail immediately with the exact existing `LinuxAgentRemoteBindAddressSourceError` on loader failure;
3. construct `LinuxAgentRemoteProcessOperationInputs::new(...)` exactly once on success;
4. move all remaining typed values without replacement, cloning, defaulting or mutation;
5. return the constructed owner without invoking any remote operation or runtime.

## 8. No fallback or alternate source

The selected helper must not:

- synthesize `0.0.0.0`, `::`, loopback, a fixed production IP or any other default;
- read another environment variable;
- read a file, socket, registry, DNS resolver or network interface;
- perform hostname resolution;
- discover public or private addresses;
- retry with another port;
- inspect route state;
- derive an address from peer identity;
- derive identity from the address;
- silently replace an invalid address.

Failure remains fail-closed through the existing loader error.

## 9. All non-bind production provenance remains deferred

IL intentionally does not select executable-production sources for:

- `PeerConnectivityIdentity`;
- `max_active_workers`;
- `SharedCurrentCapabilityAuthority<P>`;
- `SessionAuthenticationService` population/state;
- `expected_requests` producer or channel lifecycle;
- admission timing;
- completion callback;
- rejection callback;
- repeated-admission-failure callback;
- requester/rendezvous start-policy population;
- requester/rendezvous runtime-provider construction/population;
- provider capacity;
- provider lifecycle provenance.

These values remain caller-supplied typed custody at the selected IL helper boundary.

A later checkpoint must select each executable-production provenance before a real full owner may be assembled.

## 10. No peer-identity population

IL does not select construction of:

```text
PeerConnectivityIdentity
```

and does not connect the existing executable device-identity signer preflight in `main.rs` to connectivity peer identity.

No transport fingerprint, certificate fingerprint, signer fingerprint, endpoint address or bind address may be promoted into logical peer identity by IL.

Any production `PeerConnectivityIdentity` source requires a separate audited selection.

## 11. No capability/session authority population

IL does not select a production registry, capability policy or session-authentication population strategy.

The helper receives the exact already-typed:

```text
SharedCurrentCapabilityAuthority<P>
SessionAuthenticationService
```

by value.

It performs no registry lookup, policy evaluation, session challenge, signer verification, authenticated-session creation or authority mutation.

## 12. No expected-request producer selection

IL does not select how:

```text
mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>
```

is produced or fed.

It selects no dispatcher construction, target intent, requester/rendezvous DR execution, expected-device admission request creation, queue capacity, producer task, producer thread or channel shutdown policy.

## 13. No requester/rendezvous activation or provider population

IL does not modify or invoke:

```text
BoundedRequesterRendezvousStartPolicySource
CandidatePublicationRequesterRendezvousRuntimeOwner
```

and does not select their production construction.

Specifically not selected:

- requester-aware policy evaluation;
- requester/rendezvous registry validation;
- provider registration;
- current-grant selection;
- provider mutation;
- provider cleanup;
- target-intent ingestion;
- candidate publication;
- NAT traversal;
- peer dialing.

## 14. Existing production/reachability aggregate owners remain frozen

IL selects no mutation to:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs<...>
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>
```

or their constructors.

The future helper returns only the existing lower-level:

```text
LinuxAgentRemoteProcessOperationInputs<...>
```

A later separately gated checkpoint may decide how an already-populated lower-level owner is joined with a production `PeerConnectivityIdentity` and requester/rendezvous custody.

## 15. Existing operation factories remain frozen

IL selects no mutation to:

```text
linux_agent_remote_process_operation(...)
linux_agent_production_reachability_remote_process_operation(...)
linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)
```

The future helper must not invoke any of them.

## 16. IK executable assembly wrapper remains dormant

IL selects no invocation or mutation of:

```text
run_with_production_reachability_requester_rendezvous_remote_process_companion(...)
```

The wrapper remains source-present but unreachable from the production executable path.

IL does not construct the full input owner required by that wrapper.

## 17. `run()` and `main.rs` remain frozen

At the exact IK head:

- `run()` still enters only the existing local signal-aware runtime;
- `main.rs` still performs the existing device-identity signer preflight and calls `linux_bootstrap::run()`.

IL preserves both paths exactly.

The first source successor after IL must not modify `run()` or `main.rs` and must not make the IL helper or IK wrapper reachable from service startup.

## 18. Side-effect boundary

Definition of the future helper is dormant source materialization.

When the helper is not invoked, it performs no I/O.

If a later separately gated caller invokes it, the only side effect selected by IL is the existing process-environment read performed by `load_linux_agent_remote_bind_addr_from_env()`.

The helper itself must perform no:

- socket creation/bind/listen/connect;
- systemd credential read;
- provider I/O;
- registry I/O;
- DNS/network discovery;
- task/thread spawn;
- readiness publication;
- candidate publication;
- traversal/dialing;
- durable-owner mutation.

## 19. Failure law

IL adds no new failure classification.

The exact failure law is:

```text
load_linux_agent_remote_bind_addr_from_env() failure
 -> return exact LinuxAgentRemoteBindAddressSourceError unchanged
 -> do not construct LinuxAgentRemoteProcessOperationInputs
 -> no fallback
 -> no retry
 -> no remote/runtime side effect
```

There is no conversion into `LinuxAgentBootstrapStartFailure` in this checkpoint.

Executable exit policy for this error remains separately gated.

## 20. First source-successor ceiling

The first source-materialization successor after IL is authorized to modify exactly one source file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized source change is limited to:

1. one crate-private helper named exactly `linux_agent_remote_process_operation_inputs_from_production_bind_addr`;
2. parameters corresponding exactly to all existing `LinuxAgentRemoteProcessOperationInputs` fields except `bind_addr`;
3. one exact call to existing `load_linux_agent_remote_bind_addr_from_env()`;
4. one exact successful call to existing `LinuxAgentRemoteProcessOperationInputs::new(...)`;
5. unchanged return of `LinuxAgentRemoteBindAddressSourceError` on loader failure;
6. no new type unless compilation proves a type alias is strictly necessary and still remains inside this one file; a new owner struct is not selected;
7. bounded compile/source-shape evidence only; no production environment mutation is required for tests.

If implementation requires a second source path, new public API, new configuration source, environment fallback, new error envelope, peer-identity construction, provider construction or executable invocation, the successor must stop and select a separate checkpoint.

## 21. Test and validation obligations

The first source successor must prove:

1. the helper compiles with the exact existing generic field types;
2. the success type is exactly `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>`;
3. the failure type is exactly `LinuxAgentRemoteBindAddressSourceError`;
4. source inspection shows exactly one production bind loader call and one existing input-owner constructor call;
5. `run()`, `main.rs`, the IK wrapper and all operation factories remain unchanged;
6. no second source path changes;
7. workspace validation remains green.

Tests must not introduce process-global environment mutation solely to exercise the helper. The existing pure parser tests and loader contract remain authoritative for bind-source semantics.

## 22. Repository/runtime invariants

IL does not authorize:

- repository visibility/configuration change;
- merge;
- branch deletion;
- deployment;
- restart;
- systemd unit/package mutation;
- `LoadCredential=` mutation;
- credential/certificate/trust/RBAC mutation;
- listener/readiness activation;
- requester/rendezvous activation;
- candidate publication;
- NAT traversal;
- peer dialing;
- full production input assembly;
- executable caller activation.

The IL PR must remain draft/open/unmerged after closure.

## 23. Stop condition after IL

After IL closes, the next authorized step is only the source-materialization successor within the exact ceiling in Section 20.

After that source successor closes, work must stop again and select the next still-missing production provenance seam from the exact resulting source state.

No assumption is made that peer identity, capability/session authority, expected-request production, requester/rendezvous provider population or executable activation is automatically next.