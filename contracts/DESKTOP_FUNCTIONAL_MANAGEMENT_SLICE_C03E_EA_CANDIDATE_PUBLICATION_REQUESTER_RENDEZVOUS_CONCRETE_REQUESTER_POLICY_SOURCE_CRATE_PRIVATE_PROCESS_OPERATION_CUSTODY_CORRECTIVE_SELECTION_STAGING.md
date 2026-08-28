# Private Remote Workspace — Phase 152 C03e-EA Requester/Rendezvous Concrete Requester Policy Source Crate-Private Process-Operation Custody Corrective Selection

Status: `STAGING_SELECTION`

Gate: `C03E_EA_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CONCRETE_REQUESTER_POLICY_SOURCE_CRATE_PRIVATE_PROCESS_OPERATION_CUSTODY_CORRECTIVE_SELECTED`

## Purpose

C03e-EA corrects only the Rust API visibility boundary blocked by durably closed C03e-DZ.

The C03e-DY semantic intent remains authoritative: one already-constructed `BoundedRequesterRendezvousStartPolicySource` should be retained for one remote-process-operation lifetime without deriving policy inside custody code and without invoking C03e-DV.

C03e-EA selects a crate-private ownership adapter rather than changing the existing public `LinuxAgentRemoteProcessOperationInputs` constructor or making requester-policy authority types public.

This checkpoint is selection-only. It materializes no Rust source and activates no requester/rendezvous operation.

## Exact predecessor

C03e-EA is rooted only at durably closed C03e-DZ:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-dz-candidate-publication-requester-rendezvous-concrete-requester-policy-source-process-operation-custody-source-materialization-staging`
- predecessor head: `57d33660a0205cd6043c6a2af96130ba32bfb312`
- predecessor tree: `1484d2da3cdc82177b747c0568643b7053bfe6c9`
- predecessor final contract blob: `5f5f156e0a186da0fee6d3f2031fd88439fc863c`
- predecessor immutable audit Drive ID: `1JUgrcjptlNYz9UUDmrT566JszZC7TWyd`
- predecessor rolling evidence: `1074019` bytes / SHA-256 `e858d3602a9024bc286301d93e8130b2d6ee673e211831579dcd321abb24bbe0`

Any future source materialization is invalid if it is not an exact descendant of this closed checkpoint.

## C03e-DZ blocker carried forward

The requester-policy source is effectively crate-private because its parent module is exported as:

```text
pub(crate) mod candidate_publication_requester_rendezvous_start_intent;
```

The previously selected direct custody constructor is public under public `linux_bootstrap`:

```text
pub const fn LinuxAgentRemoteProcessOperationInputs::new(...)
```

Directly adding `BoundedRequesterRendezvousStartPolicySource` to that public constructor would expose an effectively crate-private type through a public interface. Canonical PRW Clippy runs with `-D warnings`, so that `private_interfaces` mismatch is a hard blocker.

C03e-EA must solve only that visibility-boundary problem.

## Fresh topology findings

### Existing public inputs can remain an opaque owned value

`LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` already owns the currently public remote-process inputs by value. Its fields are private to `linux_bootstrap`, while the type and constructor are public.

A corrective wrapper does not need to expose or duplicate those fields. It can own one complete `LinuxAgentRemoteProcessOperationInputs<...>` value unchanged.

### Existing public operation factory can remain unchanged

`linux_agent_remote_process_operation(...)` already consumes the public inputs value by value and returns the existing one-shot remote-process operation closure.

A crate-private adapter factory can delegate to this existing public factory rather than reimplementing the remote-process lifecycle graph.

### The requester-policy source is suitable for by-value internal custody

C03e-DX already proved `BoundedRequesterRendezvousStartPolicySource` is a bounded owned immutable concrete source after one-shot construction and is `Send + Sync` in its selected implementation.

No live mutation lock, clone-per-session behavior, or public exposure is needed merely to retain it for process-operation lifetime.

### No production call site currently requires widening

The current `linux_bootstrap.rs` topology has no production requester/rendezvous invocation path. The existing construction-shape use of `LinuxAgentRemoteProcessOperationInputs::new(...)` is confined to the already-validated operation-factory test surface.

Therefore the corrective adapter may be materialized crate-internally before any separately gated production assembly/population/invocation checkpoint.

## Selected corrective ownership boundary

C03e-EA selects a new crate-private wrapper/adapter in the existing `linux_bootstrap` module with semantics equivalent to:

```text
pub(crate) struct LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
> {
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
}
```

Exact Rust type name may adjust only for readability/linting. The selected visibility is crate-private, not public.

The wrapper is an API visibility boundary and lifetime custody adapter. It is not a second requester-policy backing store.

## Selected wrapper constructor

A future source-materialization checkpoint may add a crate-private constructor semantically equivalent to:

```text
pub(crate) const fn new(
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
) -> Self
```

The constructor receives only already-typed owned values and stores them unchanged.

It must not accept raw policy-binding tuples and must not construct or derive requester policy.

The existing public `LinuxAgentRemoteProcessOperationInputs::new(...)` signature remains byte/source-semantically unchanged by this selection.

## Selected crate-private factory delegation

A future source-materialization checkpoint may add a crate-private factory semantically equivalent to:

```text
pub(crate) fn linux_agent_requester_rendezvous_remote_process_operation<...>(
    inputs: LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<...>,
) -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

The factory must:

1. consume the crate-private wrapper by value;
2. separate the unchanged public `remote_process_inputs` value from the concrete requester-policy source;
3. call the existing `linux_agent_remote_process_operation(remote_process_inputs)` exactly once;
4. return one outer one-shot closure that captures the requester-policy source by value together with the existing operation closure;
5. keep the source alive for the duration of the delegated operation invocation;
6. not use the source for authorization execution in this checkpoint.

A leading-underscore local ownership binding is permitted solely to prove/carry lifetime custody without pretending the source has been invoked.

## No duplicated lifecycle graph

The corrective factory must delegate to the existing public remote-process operation factory. It must not duplicate:

- executor construction;
- authority bootstrap;
- endpoint startup;
- controller publication;
- lifecycle driving;
- shutdown ownership;
- readiness or worker management.

This keeps the corrective seam limited to API visibility and lifetime custody.

## Public API invariants

C03e-EA explicitly preserves:

- `pub mod linux_bootstrap` as currently exported;
- the public `LinuxAgentRemoteProcessOperationInputs<...>` type;
- the existing public `LinuxAgentRemoteProcessOperationInputs::new(...)` constructor signature;
- the public `linux_agent_remote_process_operation(...)` factory signature;
- effective crate-private visibility of requester/rendezvous policy source types.

No authority-facing item becomes newly public.

## Population provenance remains separate

The future wrapper constructor must receive a fully constructed `BoundedRequesterRendezvousStartPolicySource` by value.

C03e-EA does not select where policy bindings come from and does not permit custody code to derive policy from:

- environment variables;
- command-line values;
- files or persistence;
- registry membership/roles;
- `SharedCurrentCapabilityAuthority<P>`'s principal-agnostic policy;
- `SessionAuthenticationService` state;
- transport certificates or `TransportIdentity`;
- socket/IP endpoints;
- target `DeviceId`;
- candidate/request IDs or traffic;
- requester/rendezvous provider records;
- process-global defaults.

No implicit empty/default requester-policy source is selected.

## No requester/rendezvous invocation

The future corrective custody materialization must not call:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

It must not add a target producer, requester/rendezvous provider production owner, command mapping, wire/parser/dispatcher branch, or worker-loop invocation.

Holding the source by value for a process-operation lifetime is not authorization execution.

## Provider custody remains separate

`CandidatePublicationRequesterRendezvousRuntimeOwner` remains a separate mutable post-policy authority boundary.

C03e-EA does not add it to the corrective wrapper and does not combine immutable requester-policy configuration custody with mutable requester/rendezvous provider custody.

## Synchronization posture

C03e-EA adds no synchronization primitive. The C03e-DX concrete source remains immutable after one-shot construction.

No lock, watch, refresh, update/remove surface, lease, TTL, persistence, or distributed coordination is selected.

## Identity invariants

C03e-EA preserves:

- `AuthenticatedDeviceSession` as authenticated application-session identity;
- logical requester `DeviceId` as requester-policy lookup identity;
- exact `WorkspaceId + UserId` requester dimensions checked by the C03e-DX source;
- `TransportIdentity` as lower transport certificate identity only;
- no endpoint/IP/candidate/request/target identity substitution for requester policy;
- current registry authority as separate from requester-aware policy source custody.

## Source-materialization gate

A successor source-materialization checkpoint may modify only the narrow Rust source needed to implement this crate-private adapter and its compile-time/side-effect-free ownership tests, provided fresh exact-head audit finds no contradiction.

Likely source target:

`crates/prw-agent/src/linux_bootstrap.rs`

The existing closed-DZ/DY blob is:

`8d569a432fa5d8706cc1458a771f40dedd501f72`

No manifest/lock changes should be required.

If compiler/lint requirements demand public exposure of requester-policy authority, changing the existing public constructor API, policy population, provider custody, or C03e-DV invocation, materialization must stop rather than widen scope.

## Explicitly still gated

C03e-EA does not materialize Rust source and does not select or activate:

- requester-policy population provenance;
- live requester-policy lifecycle or persistence;
- requester/rendezvous provider production custody;
- C03e-DV invocation;
- target production;
- PRWC/PRWM mapping;
- wire/parser/dispatcher handling;
- bootstrap/main production assembly;
- listener/readiness/network activation;
- process-companion activation;
- distributed coordination;
- deployment, restart, recovery, or merge.
