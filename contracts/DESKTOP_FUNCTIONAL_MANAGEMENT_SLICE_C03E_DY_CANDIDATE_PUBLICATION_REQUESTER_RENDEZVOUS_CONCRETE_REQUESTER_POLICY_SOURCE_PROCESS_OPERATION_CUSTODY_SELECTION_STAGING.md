# Private Remote Workspace — Phase 152 C03e-DY Candidate Publication Requester/Rendezvous Concrete Requester Policy Source Process-Operation Custody Selection

Status: `STAGING_SELECTION`

Gate: `C03E_DY_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CONCRETE_REQUESTER_POLICY_SOURCE_PROCESS_OPERATION_CUSTODY_SELECTED`

## Purpose

C03e-DY selects only the production-process lifetime custody seam for the already-materialized C03e-DX `BoundedRequesterRendezvousStartPolicySource`.

This checkpoint does not select where policy bindings come from, does not populate a policy source from environment/registry/roles/persistence, does not materialize Rust source, does not invoke the C03e-DV requester/rendezvous start helper, and does not activate wire handling, bootstrap execution, networking, deployment, restart, recovery, or merge.

The sole question is: after a caller has already constructed one exact typed bounded requester-policy source, which existing Agent ownership boundary should take it by value so later separately-gated composition can borrow it for the lifetime of one remote process operation?

## Exact predecessor

C03e-DY is rooted only at durably closed C03e-DX:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-dx-candidate-publication-requester-rendezvous-concrete-requester-policy-backing-source-materialization-staging`
- predecessor head: `a86884988ce6fca2a351a6a125388d4ae68f34d9`
- predecessor tree: `ec0b462a16122060c3a02b35e4f8cae7cc6bcd8e`
- DX contract blob: `223648a6e31d48dcc20768b28ebf7d9c52502ecd`
- DX source blob: `f7377011a3ab2034c14d9018a5c0f268f6660ffa`
- authoritative DX audit Drive ID: `1n9nZ_FxsTJGdyKY0Q_sYpsI2k1lCA3wc`
- closed-DX rolling evidence: `1065168` bytes / SHA-256 `512acbda60c886398bf47c09d596037e38c0e0656aa4a9971c3cc0c42b5e3388`

Any later custody materialization is invalid if it is not an exact descendant of this closed checkpoint or if it widens requester policy authority beyond the C03e-DW/DX model.

## Fresh exact-head topology findings

### Concrete source is already its own immutable backing owner

C03e-DX materializes `BoundedRequesterRendezvousStartPolicySource` as a non-clone-required, bounded, one-shot-populated value.

It owns its typed requester policy bindings internally and exposes no live insert/update/remove/replace operation.

Therefore C03e-DY does not select a redundant wrapper merely to own the same map. Process custody can transfer the concrete source itself by value.

### Existing production-process operation inputs are a by-value custody boundary

`LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` already owns typed process-operation dependencies by value, including:

- `SharedCurrentCapabilityAuthority<P>`;
- `SessionAuthenticationService`;
- expected-admission receiver and typed lifecycle callbacks;
- bind address and worker bound.

Its constructor consumes those typed values without performing credential reads, provider I/O, endpoint bind, authentication, authorization, task spawn, readiness publication, or process-lifecycle mutation.

The owner is intentionally non-cloneable.

### Existing operation factory consumes the input owner by value

`linux_agent_remote_process_operation(...)` consumes one `LinuxAgentRemoteProcessOperationInputs` and returns one `FnOnce(...) + Send + 'static` operation closure.

The existing implementation destructures the input owner and captures its long-lived authority/session dependencies for the remote operation lifetime.

This is the narrowest existing process-level custody seam for an already-typed requester-policy source.

### Existing authenticated-session owner must not become global policy storage

`AuthenticatedRemoteSessionRuntimeOwner` owns one connected authenticated remote application session and its bound capability lifetime.

Its C03e-DV helper accepts the requester-aware policy source by shared borrow. That shape is deliberate: requester policy is process/requester-principal authority, not per-session mutable storage.

C03e-DY therefore rejects copying or moving the concrete policy source into each authenticated-session owner.

### Requester/rendezvous runtime owner is a separate mutation authority

`CandidatePublicationRequesterRendezvousRuntimeOwner` owns the private requester/rendezvous provider and its post-DK registration mutation boundary.

It does not own requester policy configuration.

C03e-DY therefore does not fold policy source custody into that provider owner and does not add the provider owner to production inputs in this checkpoint.

## Selected custody seam

C03e-DY selects the existing `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` as the future by-value process-operation custody boundary for exactly one already-constructed:

```text
BoundedRequesterRendezvousStartPolicySource
```

A later source-materialization checkpoint may add one private field semantically equivalent to:

```text
requester_rendezvous_start_policy_source:
    BoundedRequesterRendezvousStartPolicySource
```

and may extend the existing input constructor with one corresponding by-value parameter.

Exact Rust field ordering and import spelling may adjust only for formatting/linting/module visibility. The selected ownership semantics may not widen.

## Construction must remain already-typed injection only

The input constructor must receive a fully constructed `BoundedRequesterRendezvousStartPolicySource` by value.

C03e-DY explicitly does not permit the input constructor to receive raw policy-binding tuples and internally construct policy authority.

It also must not read or derive requester policy from:

- environment variables;
- command-line strings;
- filesystem/config files;
- `WorkspaceDeviceRegistry` membership roles;
- the principal-agnostic `SharedCurrentCapabilityAuthority<P>` policy;
- `SessionAuthenticationService` contents;
- transport certificates;
- socket/IP endpoints;
- target identity;
- candidate-publication traffic;
- provider records;
- a process-global default decision.

Population provenance remains separately gated.

## No default or implicit empty authority

The future production custody field must be explicit in the selected constructor path.

C03e-DY does not select silently constructing `BoundedRequesterRendezvousStartPolicySource::default()` inside `LinuxAgentRemoteProcessOperationInputs::new` or inside `linux_agent_remote_process_operation`.

An empty source may be a valid explicitly constructed fail-closed value, but selecting or constructing that value must be visible at the separately gated population/assembly boundary rather than silently fabricated by custody code.

## Operation-lifetime capture

The existing operation factory consumes the inputs by value. C03e-DY selects that the requester-policy source move with the same ownership transfer.

A later custody materialization may destructure the source from the input owner and retain it in the returned one-shot operation closure.

Until a separately gated caller-invocation checkpoint exists, the captured source must remain unused for authorization execution.

C03e-DY does not authorize passing it to C03e-DV yet.

## Borrowing posture for future use

When later separately gated composition needs requester/rendezvous policy resolution, it may borrow the process-owned source as:

```text
&BoundedRequesterRendezvousStartPolicySource
```

The source must not be cloned per request or per authenticated session.

The existing C03e-DP lifetime shape remains authoritative: the resolved evaluator borrow is tied to the source borrow, while the requester identity input is the exact authenticated session.

## Synchronization posture

C03e-DY selects no new synchronization primitive.

The C03e-DX concrete source is immutable after one-shot construction, so shared borrowing requires no mutation lock in the currently selected model.

C03e-DY does not add the source to `SharedCurrentCapabilityAuthority<P>`, because that type currently couples current registry state with one principal-agnostic `PolicyEvaluator` value and does not encode requester-aware policy selection.

A later policy refresh/update model, if ever selected, must be a separate authority/lifecycle checkpoint and may not retroactively reinterpret this immutable source as a live mutable store.

## Separation from requester/rendezvous provider custody

Policy source custody and requester/rendezvous provider custody remain distinct.

C03e-DY does not select or materialize production custody for `CandidatePublicationRequesterRendezvousRuntimeOwner`.

That owner is mutable post-policy registration authority, while the DX source is immutable policy configuration authority. Combining them in one checkpoint would collapse two different lifecycle and authority boundaries.

A later checkpoint may separately select where the requester/rendezvous runtime owner lives if source topology still requires it.

## Separation from DV caller invocation

The C03e-DV helper remains uncalled.

C03e-DY does not add a target producer, does not alter the capability-request loop, and does not introduce a dispatcher/wire command that invokes:

```text
AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)
```

Holding a source for process lifetime is not authorization execution.

## Identity invariants

C03e-DY preserves the existing identity model:

- `AuthenticatedDeviceSession` is authenticated application-session identity;
- logical requester `DeviceId` is the requester-policy lookup key;
- stored `WorkspaceId + UserId` must match the same authenticated session before evaluator return;
- target `DeviceId` remains DI target-validation input, not requester policy identity;
- `TransportIdentity` remains lower transport certificate identity only;
- `SessionId` remains authentication/session correlation only;
- endpoint/IP/candidate/request IDs remain non-authoritative for requester policy selection.

Process custody does not alter these semantics.

## Error posture

C03e-DY adds no request-time error and no new authorization classification.

Existing boundaries remain:

- concrete source construction: `Capacity` / `DuplicateDevicePolicyBinding`;
- source resolution: `Unavailable` / `Indeterminate`;
- DK: policy denial;
- DN: requester/rendezvous provider lifecycle failure.

Future production assembly failure caused by inability to obtain a concrete source is not selected here because population provenance is still separately gated.

## Expected future source-materialization scope

If a fresh exact-head audit confirms this selection, the next custody source checkpoint should be limited to:

1. `crates/prw-agent/src/linux_bootstrap.rs` — import the existing concrete source, add one private by-value field to `LinuxAgentRemoteProcessOperationInputs`, extend its existing typed constructor, destructure/capture that source in `linux_agent_remote_process_operation`, and keep it deliberately unused by authorization execution;
2. one source-materialization contract.

No new Cargo dependency is expected because `prw-agent` already owns the concrete source and `linux_bootstrap.rs` is in the same crate.

No parent-module registration is expected.

Any lint accommodation must remain local and must not create a getter, global/static source, fallback, or invocation path.

## Explicitly not selected or materialized

C03e-DY does not select or materialize:

- source construction from environment/config/CLI;
- a production list of requester policy bindings;
- role-to-policy derivation;
- policy persistence/schema/serialization;
- live insert/update/remove/replace/refresh;
- policy watch/reload infrastructure;
- combined registry/requester-policy locking;
- a process-global default evaluator;
- requester-policy source cloning;
- per-session policy source ownership;
- requester/rendezvous runtime-owner production custody;
- provider capacity selection for production;
- C03e-DV invocation;
- target-device producer;
- new capability request/response command;
- wire opcode/frame/parser/dispatcher changes;
- PRWC/PRWM mapping;
- listener/accept-loop changes;
- Agent `main.rs` activation changes;
- readiness changes;
- STUN/ICE/TURN/relay activation;
- production networking changes;
- deployment/restart/recovery;
- merge.

## Validation requirement

C03e-DY is documentation-only selection.

Its exact final head must pass canonical Rust validation if the repository's documentation path triggers it. Any automatically triggered ancillary workflows must reach terminal state and must not be promoted from SKIPPED to PASS.

Android validation is not expected for a one-file documentation-only diff; if it does trigger, it must reach terminal PASS before durable closure.

## Safe successor

After durable C03e-DY closure, perform a fresh exact-head audit before materializing the selected process-operation custody field.

Even after custody source materialization, policy population provenance, requester/rendezvous runtime-owner custody, C03e-DV invocation, wire/runtime activation, networking, deployment, and merge remain separately gated.