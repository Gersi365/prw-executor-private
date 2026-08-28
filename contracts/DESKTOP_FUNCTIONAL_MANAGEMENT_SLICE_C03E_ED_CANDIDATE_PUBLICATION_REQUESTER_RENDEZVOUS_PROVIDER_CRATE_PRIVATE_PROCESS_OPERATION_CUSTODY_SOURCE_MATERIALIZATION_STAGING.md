# Private Remote Workspace — Phase 152 C03e-ED Requester/Rendezvous Provider Crate-Private Process-Operation Custody Source Materialization

Status: `STAGED_SOURCE_MATERIALIZATION`

Gate: `C03E_ED_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_PROVIDER_CRATE_PRIVATE_PROCESS_OPERATION_CUSTODY_SOURCE_MATERIALIZED`

## Purpose

C03e-ED materializes only the custody boundary selected and durably closed by C03e-EC: the existing crate-private requester/rendezvous remote-process wrapper may retain one already-constructed `CandidatePublicationRequesterRendezvousRuntimeOwner` by value alongside the already-retained concrete requester-aware policy source and the unchanged existing remote-process inputs.

This checkpoint is ownership-only. It does not construct or populate the underlying requester/rendezvous provider, select provider capacity, expose provider internals, invoke C03e-DV, select or derive a rendezvous target `DeviceId`, modify worker/admission/lifecycle signatures, or activate wire/runtime/network/bootstrap production flow.

## Exact predecessor

C03e-ED is rooted only at durably closed C03e-EC:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-ec-candidate-publication-requester-rendezvous-provider-crate-private-process-operation-custody-selection-staging`
- predecessor head: `f272b1bdbf00d379e277e63c5f09b8c0484b277c`
- predecessor tree: `6fe5dab8d165405aed710685676d700311e962c1`
- predecessor contract blob: `393c779c83ce9f07282cc8e3c16a90c1f769faff`
- predecessor immutable audit Drive ID: `1Oxy-Z-lsnBRCGLFv6acIjbKUAkIIzo9Q`
- predecessor rolling evidence: `1081648` bytes / SHA-256 `3be9b47c6923aaa920c6a04ed9719ca8e50b5d30c804a62651865eaa995d2ac1`

Any source materialization is invalid if the branch is not an exact descendant of this closed checkpoint.

## Fresh exact-head source anchors

At the closed C03e-EC head:

- `crates/prw-agent/src/linux_bootstrap.rs` blob: `2fb4d1d63633a15e2cc7711f9e89e0e53fb1983e`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs` blob: `68ba74e82cf703664b7ee090a10fc1c6cce1609d`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` blob: `db90d55be95dcec1e8e9d1e6be15b1ed11121642`
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs` blob: `812b56e9b948a41f2f746eb406ba24567efbd528`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` blob: `47c41735de3c153cde8794b46479e09da7cfba18`
- `crates/prw-agent/src/lib.rs` blob: `58b37553c2f089e0f5f4a911be2f40893e18173c`

## Allowed source target

Only this existing Rust source file may change:

`crates/prw-agent/src/linux_bootstrap.rs`

No other Rust source file, Cargo manifest, lockfile, `main.rs`, workflow, wire/parser/dispatcher module, requester/rendezvous provider implementation, executor/admission lifecycle, or networking runtime is in scope.

## Required import

C03e-ED may import only the already-materialized runtime-owner type:

`crate::candidate_publication_requester_rendezvous_runtime::CandidatePublicationRequesterRendezvousRuntimeOwner`

No raw provider type is required by the process-operation wrapper.

## Required wrapper extension

The existing crate-private non-cloneable wrapper may be extended semantically to:

```text
pub(crate) struct LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
> {
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
}
```

The exact formatting may vary only for rustfmt/lint/readability compatibility.

The wrapper must remain crate-private and non-cloneable. It must expose no getter for the policy source, runtime owner, raw provider, mutable provider reference, or reusable authority token.

## Required constructor extension

The existing crate-private constructor may be extended to consume exactly three already-constructed typed values:

1. one complete `LinuxAgentRemoteProcessOperationInputs<...>`;
2. one `BoundedRequesterRendezvousStartPolicySource`;
3. one `CandidatePublicationRequesterRendezvousRuntimeOwner`.

All three values must be stored by value unchanged.

The constructor must not accept or derive:

- raw requester-policy bindings;
- raw `InMemoryRequesterRendezvousAuthorityProvider` construction parameters;
- provider capacity;
- registry-derived/default policy or provider state;
- session-derived policy population;
- transport identity;
- endpoint/IP/candidate/request identifiers;
- rendezvous target identity;
- synchronization handles;
- task/runtime handles.

The existing public `LinuxAgentRemoteProcessOperationInputs::new(...)` signature and body must remain source-semantically unchanged.

## Required custody-only adapter behavior

The existing crate-private `linux_agent_requester_rendezvous_remote_process_operation(...)` adapter may be modified only to:

1. destructure the added runtime owner by value together with the existing fields;
2. call existing `linux_agent_remote_process_operation(remote_process_inputs)` exactly once;
3. return one outer `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` closure capturing the delegated operation, requester-policy source, and requester/rendezvous runtime owner by value;
4. on invocation, explicitly drop the requester/rendezvous runtime owner and requester-policy source without using either for authorization or mutation;
5. invoke the delegated existing operation exactly once.

The explicit drops are ownership/lifetime proof only. They do not constitute production requester/rendezvous activation.

No lifecycle graph may be copied, reordered, replaced, or duplicated.

## Existing public operation remains unchanged

C03e-ED must preserve unchanged:

- public `LinuxAgentRemoteProcessOperationInputs<...>`;
- public `LinuxAgentRemoteProcessOperationInputs::new(...)`;
- public `linux_agent_remote_process_operation(...)` signature and body;
- public `run_with_remote_process_companion(...)` signature and behavior;
- remote executor construction;
- reachability-authority bootstrap;
- endpoint bind/start;
- shutdown-controller publication;
- repeated real-admission endpoint lifecycle;
- worker collection/cancellation/finalization behavior.

## C03e-DV remains uncalled

C03e-ED must not call:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

No logical target producer is added.

The repeated-admission `expected_device_id` remains the logical device being admitted/authenticated and MUST NOT substitute for a requester/rendezvous target.

## Provider construction remains separately gated

`CandidatePublicationRequesterRendezvousRuntimeOwner` already owns one configured `InMemoryRequesterRendezvousAuthorityProvider` and exposes the narrow crate-internal post-policy registration mutation selected by earlier checkpoints.

C03e-ED consumes only an already-constructed runtime owner. It does not choose:

- provider capacity;
- provider construction location;
- provider default/population;
- provider retirement/removal behavior;
- publisher authorization invocation;
- persistence or refresh;
- lock/atomic/channel topology.

## Authority separation

C03e-ED preserves:

- current registry authority from `SharedCurrentCapabilityAuthority::with_current_authority(...)`;
- principal-agnostic shared-current `P` as non-substitutable for requester-aware policy;
- requester-aware policy source as separate immutable policy authority;
- requester/rendezvous runtime owner as separate mutable post-policy provider state;
- authenticated application session as requester identity;
- caller-supplied logical target `DeviceId` as a distinct future input to C03e-DV.

No fallback or duplicate authority source is introduced.

## Identity invariants

- `AuthenticatedDeviceSession` remains authenticated application-session identity.
- logical requester `DeviceId` remains requester-policy lookup identity.
- exact authenticated `WorkspaceId + UserId` remain policy-source match dimensions.
- `TransportIdentity` remains lower-transport certificate identity only.
- endpoint/IP/candidate/request IDs do not substitute for logical authorization identity.
- AJ/repeated-admission `expected_device_id` is not a rendezvous target.

## Synchronization and lifecycle

C03e-ED adds no lock, atomic, channel, TTL, lease, watch, refresh, cache, persistence, distributed coordination, retry, fallback, replacement, readiness state, task, thread, runtime, or network activation.

The newly retained runtime owner is custody only. No request-time use is selected.

## Compiler and lint posture

The repository uses Rust edition 2024 and canonical Clippy with warnings denied. Existing narrow `dead_code` allowances on the crate-private staged wrapper/factory may be retained or wording-adjusted only as required for the staged source seam.

No lint suppression may hide authority widening, identity substitution, public API widening, unsafe code, or unused side effects.

If Clippy rejects a no-effect binding, explicit `drop(...)` is the selected compiler-safe ownership proof.

## Validation requirements

Because C03e-ED changes Rust source, canonical closure requires exact-final-head:

- PRW Rust Validation FULL PASS, including locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- PRW Android Validation FULL PASS if the source-changing diff triggers Android validation;
- no exact-final-head pending or failing workflow;
- dependency anchors unchanged.

Any corrective source commit must remain formatter/type/lint-only and may not widen authority or runtime semantics.

If compilation requires public requester/rendezvous authority exposure, modification of existing public process inputs/factory, provider construction/population, target production, C03e-DV invocation, lifecycle duplication, synchronization, or runtime/network activation, C03e-ED must stop rather than widen scope.

## Explicitly still gated

C03e-ED does not select or activate:

- provider construction/capacity;
- requester-policy production population;
- live policy/provider currentness;
- synchronization primitives;
- C03e-DV invocation;
- logical target production;
- PRWC/PRWM mapping;
- parser/frame/dispatcher handling;
- worker/admission/lifecycle signature widening;
- bootstrap/main production assembly;
- listener/readiness/network activation;
- persistence/distributed coordination;
- deployment;
- restart/recovery;
- merge.
