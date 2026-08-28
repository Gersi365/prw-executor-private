# Private Remote Workspace — Phase 152 C03e-EB Requester/Rendezvous Concrete Requester Policy Source Crate-Private Process-Operation Custody Source Materialization

Status: `STAGED_SOURCE_MATERIALIZATION`

Gate: `C03E_EB_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CONCRETE_REQUESTER_POLICY_SOURCE_CRATE_PRIVATE_PROCESS_OPERATION_CUSTODY_SOURCE_MATERIALIZED`

## Purpose

C03e-EB materializes only the compiler-safe crate-private custody boundary selected and durably closed by C03e-EA.

The source change may retain one already-constructed `BoundedRequesterRendezvousStartPolicySource` for one remote-process-operation lifetime without modifying the existing public remote-process input constructor and without making requester-policy authority public.

C03e-EB does not populate requester policy, does not add requester/rendezvous provider production custody, does not invoke C03e-DV, and does not activate wire/runtime/network/bootstrap production flow.

## Exact predecessor

C03e-EB is rooted only at durably closed C03e-EA:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-ea-candidate-publication-requester-rendezvous-concrete-requester-policy-source-crate-private-process-operation-custody-corrective-selection-staging`
- predecessor head: `168b0f1fa071c5a51b36ad9db145fd4bd5ee40c4`
- predecessor tree: `e7adcb84830009c1c6e369ab389b6b7924892d24`
- predecessor contract blob: `268b2f2da94fbcd68a0fc0ff473b5ad1fa805a59`
- predecessor immutable audit Drive ID: `1JSE9XQDHEV7D1VaXbUeJwFmw8PtNoOQp`
- predecessor rolling evidence: `1077082` bytes / SHA-256 `e589b23f43671d64cd08909d52581c1e12def1a742e9c8462e54bea0d0cae44f`

Any source materialization is invalid if the branch is not an exact descendant of this closed checkpoint.

## Fresh exact-head source anchors

At the closed C03e-EA head:

- `crates/prw-agent/src/linux_bootstrap.rs` blob: `8d569a432fa5d8706cc1458a771f40dedd501f72`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs` blob: `f7377011a3ab2034c14d9018a5c0f268f6660ffa`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs` blob: `4a5495b5f01e732ad458cd6603f50dc76ad0688f`
- `crates/prw-agent/src/lib.rs` blob: `58b37553c2f089e0f5f4a911be2f40893e18173c`

`candidate_publication_requester_rendezvous_start_intent` remains exported as `pub(crate)`, while `linux_bootstrap` remains public. Therefore the C03e-EA wrapper/factory must remain crate-private.

## Allowed source target

Only this existing Rust source file may change:

`crates/prw-agent/src/linux_bootstrap.rs`

A focused test in that same file may be added or adapted.

No other Rust source file, Cargo manifest, lockfile, `main.rs`, wire/parser/dispatcher surface, requester/rendezvous provider module, or networking runtime is in scope.

## Required import

C03e-EB may import only the already-materialized concrete requester-aware source type from the existing crate-private requester/rendezvous policy-source module:

`BoundedRequesterRendezvousStartPolicySource`

No raw policy binding type is required by production custody code.

## Required crate-private ownership wrapper

C03e-EB may materialize a crate-private non-cloneable wrapper semantically equivalent to:

```text
pub(crate) struct LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
> {
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
}
```

The wrapper must own both values by value. It must not expose getters that leak raw policy authority.

Exact type name and documentation wording may change only for formatter/lint/readability compatibility.

## Required crate-private constructor

The wrapper constructor must be crate-private and accept only:

1. one already-constructed complete `LinuxAgentRemoteProcessOperationInputs<...>` value; and
2. one already-constructed `BoundedRequesterRendezvousStartPolicySource` value.

It must store both unchanged.

The existing public `LinuxAgentRemoteProcessOperationInputs::new(...)` signature must remain source-semantically unchanged.

The constructor must not accept raw requester-policy bindings and must not derive policy from environment, CLI, files, registry roles, shared-current principal-agnostic policy, session state, transport identity, endpoints, target identity, candidate/request traffic, provider state, or global defaults.

## Required crate-private delegation adapter

C03e-EB may materialize one crate-private factory semantically equivalent to:

```text
pub(crate) fn linux_agent_requester_rendezvous_remote_process_operation<...>(
    inputs: LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<...>,
) -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static
```

The adapter must:

1. consume the wrapper by value;
2. separate the unchanged public remote-process inputs from the concrete requester-policy source;
3. call existing `linux_agent_remote_process_operation(remote_process_inputs)` exactly once;
4. return one outer `FnOnce` closure that captures both the concrete requester-policy source and the delegated operation by value;
5. keep the requester-policy source alive for the delegated operation lifetime;
6. invoke only the delegated existing operation when called;
7. not use the requester-policy source for authorization execution in C03e-EB.

A leading-underscore ownership binding is permitted solely to retain the source without fabricating invocation.

## No duplicated lifecycle graph

The new adapter must delegate to the existing public operation factory. It must not duplicate or alter:

- executor construction;
- reachability authority bootstrap;
- remote endpoint bind/start;
- shutdown-controller publication;
- remote admission lifecycle driving;
- worker/readiness management;
- process companion ownership/finalization.

## Public API invariants

C03e-EB must preserve unchanged:

- `pub mod linux_bootstrap`;
- public `LinuxAgentRemoteProcessOperationInputs<...>`;
- public `LinuxAgentRemoteProcessOperationInputs::new(...)` signature;
- public `linux_agent_remote_process_operation(...)` signature;
- effective crate-private visibility of requester/rendezvous policy-source authority.

No authority-facing type or module becomes newly public.

## Compiler and lint posture

The repository uses Rust edition 2024 and workspace Clippy `all`, `pedantic`, and `nursery`, with canonical validation promoted to `-D warnings`.

C03e-EB may add narrow `dead_code` allowances with explicit reasons because the crate-private custody seam is intentionally materialized before separately gated production assembly.

No lint suppression may hide public/private authority widening, identity substitution, or unsafe code.

## Focused proof

A focused side-effect-free construction test may:

- construct the existing public remote-process inputs exactly as the current construction-shape test does;
- construct an explicit empty fail-closed `BoundedRequesterRendezvousStartPolicySource` only for test ownership proof;
- wrap the two owned values in the new crate-private wrapper;
- construct the new adapter operation;
- prove the returned operation satisfies `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` without invoking it.

The existing public factory construction test must remain valid.

## No requester-policy population

Production custody code receives a fully constructed source. C03e-EB selects and materializes no population provenance and no implicit default source.

No production call to `BoundedRequesterRendezvousStartPolicySource::default()` is allowed.

## No requester/rendezvous invocation

C03e-EB must not call:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

and must not add a logical target producer, requester/rendezvous provider production owner, command mapping, dispatcher branch, worker-loop branch, or any other path that reaches C03e-DV.

## Authority separation

C03e-EB must preserve:

- current registry authority from `SharedCurrentCapabilityAuthority::with_current_authority(...)` as separate from requester-aware policy;
- principal-agnostic shared-current `P` as non-substitutable for requester-aware policy;
- requester-aware policy source as its own immutable authority input;
- requester/rendezvous provider authority as separately mutable post-policy state.

No duplicate or fallback authority source is introduced.

## Identity invariants

- `AuthenticatedDeviceSession` remains authenticated application-session identity.
- logical requester `DeviceId` remains requester-policy lookup identity.
- exact authenticated `WorkspaceId + UserId` remain dimensions checked by the bounded requester source.
- `TransportIdentity` remains lower transport certificate identity only.
- endpoint/IP/candidate/request IDs and target identity do not substitute for logical authorization identity.

## Synchronization and lifecycle

C03e-EB adds no lock, lease, TTL, watch, refresh, persistence, distributed coordination, retry, fallback, replacement, or live mutation API.

The new wrapper is only operation-lifetime custody. It creates no perpetual-currentness or reusable authorization guarantee.

## Validation requirements

Because C03e-EB changes Rust source, canonical closure requires exact-final-head:

- PRW Rust Validation FULL PASS, including locked dependency graph, rustfmt, Clippy, workspace tests, and workspace build;
- PRW Android Validation FULL PASS if the source-changing diff triggers the Android workflow;
- no exact-final-head pending or failing workflow;
- stable dependency anchors unchanged.

Any corrective source commit must remain type/formatter/lint-only and may not widen authority semantics.

If compilation requires public requester-policy exposure, modification of the existing public input constructor, policy population, provider custody, C03e-DV invocation, lifecycle duplication, or runtime/network activation, C03e-EB must stop rather than widen scope.

## Explicitly still gated

C03e-EB does not select or activate:

- requester-policy production population provenance;
- live requester-policy lifecycle;
- requester/rendezvous provider production custody;
- C03e-DV invocation;
- logical target production;
- PRWC/PRWM mapping;
- wire/parser/dispatcher handling;
- bootstrap/main production assembly;
- listener/readiness/network activation;
- persistence/distributed coordination;
- deployment;
- restart/recovery;
- merge.
