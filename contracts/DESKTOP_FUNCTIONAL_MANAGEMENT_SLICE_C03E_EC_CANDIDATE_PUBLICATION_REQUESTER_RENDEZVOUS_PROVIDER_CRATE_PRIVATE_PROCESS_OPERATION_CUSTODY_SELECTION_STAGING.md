# Private Remote Workspace — Phase 152 C03e-EC Requester/Rendezvous Provider Crate-Private Process-Operation Custody Selection

Status: `STAGED_SELECTION`

Gate: `C03E_EC_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_PROVIDER_CRATE_PRIVATE_PROCESS_OPERATION_CUSTODY_SELECTED`

## Purpose

C03e-EC selects only the next narrow ownership boundary after durably closed C03e-EB: one already-constructed `CandidatePublicationRequesterRendezvousRuntimeOwner` may be retained by value for the same crate-private remote-process-operation lifetime that already retains the concrete requester-aware policy source.

This checkpoint selects custody only. It does not construct or populate the underlying requester/rendezvous provider, does not select provider capacity, does not expose provider internals, does not invoke C03e-DV, does not select a rendezvous target `DeviceId`, and does not activate any wire/runtime/network/bootstrap production flow.

## Exact predecessor

C03e-EC is rooted only at durably closed C03e-EB:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-eb-candidate-publication-requester-rendezvous-concrete-requester-policy-source-crate-private-process-operation-custody-source-materialization-staging`
- predecessor head: `f25915ca3f271931b5ac584472c798c0444a453e`
- predecessor tree: `0def3369be7f98197a618ed573eb22c1a5536933`
- predecessor contract blob: `f86c50d46d2b169143193d74343fbaa618d63b32`
- predecessor immutable audit Drive ID: `10gzuTYUTA30h8Z5_GtTLqZUbtL8xPdWg`
- predecessor rolling evidence: `1080059` bytes / SHA-256 `71303de37ecc6c12a07c4e812c43dfa7d1c488590abe701f5240842bb95995e5`

Any later source materialization is invalid if it is not an exact descendant of this closed predecessor.

## Fresh exact-head topology anchors

At the closed C03e-EB head:

- `crates/prw-agent/src/linux_bootstrap.rs` blob: `2fb4d1d63633a15e2cc7711f9e89e0e53fb1983e`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs` blob: `68ba74e82cf703664b7ee090a10fc1c6cce1609d`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` blob: `db90d55be95dcec1e8e9d1e6be15b1ed11121642`
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs` blob: `812b56e9b948a41f2f746eb406ba24567efbd528`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` blob: `47c41735de3c153cde8794b46479e09da7cfba18`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs` blob: `999fb2d2deed48e4c3ffee5af17d2b521642eff8`
- `crates/prw-agent/src/lib.rs` blob: `58b37553c2f089e0f5f4a911be2f40893e18173c`
- lower provider source `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs` is the existing bounded process-local provider implementation.

## Fresh topology finding: authenticated requester availability

The existing AJ admission transaction `admit_expected_remote_device_session(...)` returns `AuthenticatedRemoteSessionRuntimeOwner` only after:

1. current-registry expected transport resolution;
2. exact lower-transport peer acceptance;
3. a second current-registry challenge-preparation read;
4. logical-session authentication; and
5. post-authentication bound-session composition.

Therefore C03e-DV may derive requester identity only from the resulting authenticated-session owner. C03e-EC does not move requester/rendezvous authorization before AJ success.

## Fresh topology finding: no rendezvous target exists in the worker admission seam

The repeated real-admission supervisor consumes `RemoteSessionExpectedDeviceAdmissionRequest<D, T>`, whose `expected_device_id` selects the logical device being admitted and authenticated.

After AJ succeeds, the supervisor checks:

- `session_owner.logical_device_id()`; and
- equality of that authenticated logical device with the pre-authentication `expected_device_id`.

It then creates `RemoteSessionWorkerAdmission<D, T>` from only:

- the authenticated session owner;
- dispatcher `D`; and
- verifier-time provider `T`.

No second logical `DeviceId` representing a requester/rendezvous target is present in this seam.

Consequently the AJ `expected_device_id` MUST NOT be reused as the C03e-DV target. Doing so would collapse the admitted requester identity into target identity and fabricate authority not selected by any existing contract.

C03e-EC therefore selects no C03e-DV invocation and no target producer.

## Existing C03e-DV seam remains unchanged and uncalled

The existing crate-private authenticated-session helper remains semantically:

```text
AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(
    &self,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    target_device_id: DeviceId,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

Its authority separation remains:

- requester identity from the exact retained authenticated application session;
- current registry from one `SharedCurrentCapabilityAuthority::with_current_authority(...)` read;
- principal-agnostic shared-current policy deliberately ignored for requester/rendezvous policy;
- separately supplied requester-aware policy source as DP authority;
- separately supplied mutable requester/rendezvous runtime owner as post-policy provider state;
- caller-supplied logical target `DeviceId` as a distinct input.

C03e-EC does not call, wrap, retry, alter, or expose this helper.

## Selected process-operation custody boundary

A future source-materialization checkpoint may extend the existing C03e-EB crate-private ownership wrapper so that it owns, by value, all three already-constructed process-lifetime values:

```text
pub(crate) struct LinuxAgentRequesterRendezvousRemoteProcessOperationInputs<
    P, D, T, F, C, R, E,
> {
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    requester_rendezvous_start_policy_source: BoundedRequesterRendezvousStartPolicySource,
    requester_rendezvous_runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner,
}
```

This is a custody selection only. Exact source spelling may vary only for formatter/lint/readability compatibility.

The wrapper remains crate-private and non-cloneable. No raw provider getter, runtime-owner getter, policy-source getter, mutable provider reference, or reusable authority token may be added.

## Selected constructor shape

A future source-materialization checkpoint may extend the existing crate-private wrapper constructor to consume only:

1. one already-constructed complete `LinuxAgentRemoteProcessOperationInputs<...>`;
2. one already-constructed `BoundedRequesterRendezvousStartPolicySource`; and
3. one already-constructed `CandidatePublicationRequesterRendezvousRuntimeOwner`.

All three values are transferred by value and stored unchanged.

The constructor must not accept:

- raw requester-policy bindings;
- raw `InMemoryRequesterRendezvousAuthorityProvider` construction parameters;
- provider capacity values;
- registry-derived defaults;
- process-global/default requester policy;
- session-derived policy population;
- transport identity;
- endpoint/IP/candidate/request identifiers;
- rendezvous target identity;
- synchronization handles;
- task/runtime handles.

The existing public `LinuxAgentRemoteProcessOperationInputs::new(...)` signature remains unchanged.

## Selected delegation-adapter custody behavior

A future source-materialization checkpoint may adapt the existing crate-private C03e-EB delegation factory so its returned `FnOnce` captures by value:

- the existing delegated public remote-process operation;
- the concrete requester-aware policy source; and
- the requester/rendezvous runtime owner.

For this custody-only gate, invoking the outer closure may explicitly drop both requester/rendezvous authority values before delegating exactly once to the existing public operation.

This explicit drop is permitted only as a no-authorization custody proof. It must not be interpreted as production requester/rendezvous activation.

No lifecycle graph may be copied or replaced.

## Why the runtime owner, not the raw provider, is selected

`CandidatePublicationRequesterRendezvousRuntimeOwner` already owns exactly one configured `InMemoryRequesterRendezvousAuthorityProvider` and exposes only the crate-internal post-policy registration mutation required by DN.

The raw provider additionally exposes lifecycle operations such as retirement/removal and publisher authorization. C03e-EC does not widen those raw operations into `linux_bootstrap` custody.

The process-operation wrapper therefore owns the higher-level runtime owner, not a raw provider field or getter.

## Provider construction and capacity remain separately gated

`InMemoryRequesterRendezvousAuthorityProvider::new(max_records)` requires one explicit finite non-zero capacity and fails on zero capacity.

C03e-EC deliberately does not select:

- a production capacity value;
- a capacity derivation rule;
- environment/CLI/config provenance;
- a default provider;
- lazy provider construction;
- replacement provider construction;
- recovery/replay behavior.

A future production assembly must supply an already-constructed runtime owner from separately selected provenance.

## No synchronization selection

The requester/rendezvous runtime owner contains mutable provider state. Existing remote-session worker execution uses spawned tasks inside a private current-thread Tokio runtime and can retain multiple active authenticated workers.

C03e-EC does not select how mutable requester/rendezvous provider state would be shared or serialized across future worker-triggered C03e-DV calls.

In particular, C03e-EC adds no:

- `Arc`;
- mutex/RwLock;
- async lock;
- channel/actor;
- task-local owner;
- per-worker provider clone;
- global singleton;
- unsafe interior mutability.

Any future invocation topology requiring shared mutable access must be separately selected and must preserve fail-closed authority semantics.

## Public API invariants

C03e-EC preserves unchanged:

- `pub mod linux_bootstrap`;
- public `LinuxAgentRemoteProcessOperationInputs<...>`;
- public `LinuxAgentRemoteProcessOperationInputs::new(...)`;
- public `linux_agent_remote_process_operation(...)`;
- public endpoint/executor lifecycle signatures;
- existing AJ admission transaction signature;
- existing worker-admission shape;
- effective crate-private requester-policy authority;
- private raw provider field inside `CandidatePublicationRequesterRendezvousRuntimeOwner`.

No authority-facing public API is widened.

## Identity invariants

- `AuthenticatedDeviceSession` remains authenticated application-session identity.
- requester logical `DeviceId` comes only from the authenticated application session retained by `AuthenticatedRemoteSessionRuntimeOwner`.
- the AJ `expected_device_id` is the logical identity expected to authenticate; it is not a rendezvous target grant.
- the future C03e-DV target remains a distinct logical `DeviceId` input with separately selected provenance.
- `TransportIdentity` remains lower transport certificate identity only.
- endpoint/IP/candidate/request IDs do not substitute for requester or target logical identity.

## Authority separation

C03e-EC preserves four distinct concepts:

1. current registry authority — `SharedCurrentCapabilityAuthority` registry state;
2. principal-agnostic capability policy — the existing shared-current `P` used by normal capability authorization;
3. requester-aware start policy — `BoundedRequesterRendezvousStartPolicySource`;
4. requester/rendezvous provider state — `CandidatePublicationRequesterRendezvousRuntimeOwner`.

The shared-current `P` must not substitute for requester-aware policy. The provider must not authorize a start before DI/DP/DK. Custody does not itself confer authorization.

## No duplicated lifecycle graph

Any future source materialization must continue to delegate exactly once to the existing public remote-process operation and must not duplicate or alter:

- executor creation;
- reachability-authority bootstrap;
- endpoint bind;
- shutdown-controller publication;
- AJ admission;
- authenticated worker construction;
- persistent worker collection;
- endpoint teardown;
- process companion finalization.

## Compiler/lint gate for future source materialization

A future source checkpoint may add only narrow explicit `dead_code` allowances while the custody seam remains unactivated.

If compilation requires any of the following, materialization must stop rather than widen C03e-EC:

- public requester-policy exposure;
- public requester/rendezvous runtime-owner exposure through the process constructor;
- modification of the existing public process-input constructor;
- raw provider exposure;
- provider capacity/default selection;
- new synchronization semantics;
- C03e-DV invocation;
- target derivation;
- worker/lifecycle signature widening;
- networking/bootstrap activation.

The `FnOnce + Send + 'static` return type may compiler-prove by-value custody. If the runtime owner is not `Send` under the exact current type topology, this gate must stop for corrective selection rather than introducing unsafe or broader synchronization.

## Validation requirements

C03e-EC is documentation-only selection. Closure requires exact-final-head:

- exact C03e-EB merge base;
- exactly one documentation path changed;
- no Rust source, manifests, lockfiles, `main.rs`, wire/parser/dispatcher, or runtime files changed;
- canonical workflows that trigger for the exact final head must reach terminal non-failing states;
- no unreported Android PASS when Android does not trigger;
- dependency anchors remain unchanged.

## Explicitly still gated

C03e-EC does not select or activate:

- provider production construction or capacity provenance;
- requester-policy production population provenance;
- live requester-policy mutation/lifecycle;
- synchronization/shared mutable provider access;
- C03e-DV invocation;
- logical rendezvous target production;
- PRWC/PRWM command/response mapping;
- wire/parser/dispatcher handling;
- bootstrap/main production assembly;
- listener/readiness/network activation;
- persistence/distributed coordination;
- deployment;
- restart/recovery;
- merge.

## Successor gate

If C03e-EC closes without contradiction, the next eligible gate is a separately reviewed source-materialization checkpoint for this custody selection only.

That successor must begin from the exact closed C03e-EC head, re-read the exact source topology, and stop if compiler/type/lint behavior requires authority widening beyond this selection.
