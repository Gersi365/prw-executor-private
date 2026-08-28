# Private Remote Workspace — Phase 152 C03e-DX Candidate Publication Requester/Rendezvous Concrete Requester Policy Backing Source Materialization

Status: `STAGED_SOURCE_MATERIALIZATION`

Gate: `C03E_DX_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CONCRETE_REQUESTER_POLICY_BACKING_SOURCE_MATERIALIZED`

## Purpose

C03e-DX materializes only the concrete bounded requester-aware policy backing selected and durably closed in C03e-DW.

This checkpoint does not wire the concrete source into production custody, call the C03e-DV requester/rendezvous start helper, choose a target producer, alter wire/parser/dispatcher behavior, change bootstrap/main ownership, activate a listener or process companion, publish readiness, activate networking, persist policy, deploy, restart, recover, or merge.

## Exact predecessor

C03e-DX is rooted only at durably closed C03e-DW:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-dw-candidate-publication-requester-rendezvous-concrete-requester-policy-backing-selection-staging`
- predecessor head: `ec80a86ab151db21488d37eba4e662f336e9d198`
- predecessor tree: `720c1bef20d4a9e0e8186937828bca5e30e24b62`
- predecessor PR: #247, `Status: CLOSED`, draft/open/unmerged
- predecessor contract blob: `86b8d27259fd6359cc3c89fda289687e5535a207`
- authoritative DW audit Drive ID: `1eFmlER12BmhDFFPNSimncbde4FV9ZDSi`
- closed-DW rolling evidence: `1059940` bytes / SHA-256 `35be13a296ec4ba6ad2578308ebc8abc4b7572eabd3a38fa8236468023fcfcfa`

Any materialization that is not an exact descendant of this closed checkpoint, or that widens the selected authority semantics, is invalid for C03e-DX.

## Fresh exact-head topology

The existing C03e-DP module remains:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

It already owns the Agent-internal requester-aware source trait:

```rust
pub trait RequesterRendezvousStartPolicySource {
    type Evaluator: PolicyEvaluator + ?Sized;

    fn evaluator_for_requester<'a>(
        &'a self,
        requester: &AuthenticatedDeviceSession,
    ) -> Result<&'a Self::Evaluator, RequesterRendezvousStartPolicySourceError>;
}
```

The module is already registered below the crate-internal requester/rendezvous start-intent parent module. Therefore C03e-DX requires no parent-module registration change and no crate-root export.

`AuthenticatedDeviceSession` already exposes borrowed authenticated logical identity dimensions:

- `workspace_id()`;
- `user_id()`;
- `device_id()`;

and also exposes `session_id()` plus canonical public identity. C03e-DX uses only the workspace/user/logical-device dimensions selected by C03e-DW for requester-policy lookup.

`WorkspaceId`, `UserId`, and `DeviceId` are strongly typed `Eq + Hash` identifiers.

`prw_registry::MAX_REGISTERED_DEVICES` remains `4096` and is the selected upper bound for the concrete source.

The existing `PolicyEvaluator` interface remains principal-agnostic and evaluates one `Capability` into one `Decision`. Existing bounded local read/management policies always deny `Capability::RequesterRendezvousStart`, so C03e-DX must not reuse either as the requester/rendezvous evaluator.

## Exact source scope

C03e-DX is bounded to exactly two changed paths:

1. this contract;
2. the existing requester-aware policy-source Rust module.

No Cargo manifest or lockfile change is selected. `prw-agent` already depends on `prw-core`, `prw-policy`, `prw-registry`, and `prw-session`.

## Materialized dedicated evaluator

C03e-DX materializes one operation-specific evaluator with semantics equivalent to:

```text
RequesterRendezvousStartPolicy {
    requester_rendezvous_start: Decision,
}
```

Its `PolicyEvaluator` implementation must behave exactly as follows:

- `Capability::RequesterRendezvousStart` -> configured `Decision`;
- every other capability -> `Decision::Deny`.

A wildcard/future capability is therefore denied fail-closed rather than implicitly granted.

The evaluator is policy configuration only. Constructing it does not authenticate a requester, validate registry state, create DK authorization provenance, mutate provider state, or activate runtime behavior.

No generic `allow_all` policy is introduced.

## Materialized backing binding

C03e-DX materializes one configuration binding carrying exactly:

- logical requester `DeviceId`;
- requester `WorkspaceId`;
- requester `UserId`;
- the dedicated requester/rendezvous-start evaluator.

The device ID is construction-time key material for the backing map. It is not accepted as a separate request-time lookup input.

The binding does not store:

- `SessionId`;
- `TransportIdentity`;
- canonical public-key material;
- endpoint/IP state;
- candidate identity;
- request IDs;
- publisher/provider state;
- target `DeviceId`.

Canonical authenticated public identity remains a C03e-DI current-registry validation concern and is not duplicated into policy authority.

## Materialized bounded source

C03e-DX materializes one bounded in-memory source with semantics equivalent to:

```text
BoundedRequesterRendezvousStartPolicySource {
    policies_by_device: HashMap<DeviceId, RequesterRendezvousStartPolicyBinding>,
}
```

The source implements the existing C03e-DP `RequesterRendezvousStartPolicySource` trait with evaluator type `RequesterRendezvousStartPolicy`.

The request-time trait surface remains unchanged and accepts only:

`&AuthenticatedDeviceSession`

No public/crate-visible request-time overload accepts raw workspace/user/device identity dimensions.

## Construction-only population

C03e-DX selects one-shot source construction from owned bindings rather than a public mutable insert/update/remove surface.

The materialized constructor must:

1. begin with an empty bounded map;
2. process each owned binding once;
3. key it by its logical requester `DeviceId`;
4. reject a duplicate logical device binding before any overwrite;
5. reject capacity beyond `MAX_REGISTERED_DEVICES`;
6. return a fully constructed source or a bounded construction error.

The source exposes no runtime replace/update/remove operation in C03e-DX.

This preserves later gating for policy refresh, mutation, reload, persistence, watches, distributed coordination, or live administrative update semantics.

## Stable construction failures

C03e-DX materializes a bounded backing-construction error that distinguishes at least:

- capacity exceeded;
- duplicate logical device policy binding.

These are construction failures and must remain distinct from request-time C03e-DP source failures:

- `Unavailable`;
- `Indeterminate`.

Construction failure creates no partially returned authoritative source and no fallback evaluator.

## Exact request-time resolution

For each call to existing `evaluator_for_requester(...)`, the concrete source must:

1. read the exact logical requester `DeviceId` from the supplied authenticated session;
2. look up only that device key;
3. if absent, return existing `RequesterRendezvousStartPolicySourceError::Unavailable`;
4. if present, compare the entry's stored `WorkspaceId` with the exact authenticated session workspace;
5. compare the entry's stored `UserId` with the exact authenticated session user;
6. if either comparison differs, return existing `RequesterRendezvousStartPolicySourceError::Indeterminate`;
7. only on exact device + workspace + user match, return a borrow of that entry's dedicated evaluator.

No request-time lookup may:

- search a second device entry after mismatch;
- fall back to another requester;
- consult a process-global evaluator;
- consult `BoundedLocalReadPolicy`;
- consult `BoundedLocalManagementPolicy`;
- default to `Decision::Allow`;
- translate source absence into a DK policy decision;
- create provider authority.

## Private implementation helper boundary

The implementation may use a private module-internal helper that receives already-extracted logical device/workspace/user references solely to centralize/test exact-match resolution logic.

Such a helper:

- must remain private to the module implementation;
- is not a request-time authority surface;
- must be called by the trait implementation only with dimensions read from the supplied `AuthenticatedDeviceSession`;
- must not be exported through the parent module or crate root;
- must not accept transport, endpoint, target, candidate, request, publisher, or provider identity.

This does not create the raw caller-supplied identity lane rejected by C03e-DO/DW because no external or crate-visible caller can invoke it as an authorization selector.

## Existing DK authority remains unchanged

The concrete source does not itself authorize requester/rendezvous start.

Existing C03e-DK remains the only selected policy admission gate and must still evaluate exactly:

`Capability::RequesterRendezvousStart`

using the evaluator returned by the source for the exact requester.

The concrete source may not construct `PolicyAuthorizedRequesterRendezvousStart`, expose its private fields, or call the DN provider.

## Existing DV caller remains unchanged and uncalled

C03e-DV already materialized one crate-internal async helper that receives a separately supplied `RequesterRendezvousStartPolicySource`.

C03e-DX does not modify or invoke that helper.

It does not place the new source inside `LinuxAgentRemoteProcessOperationInputs`, bootstrap state, a global/static, a shared lock, a task, a listener, or network runtime.

Concrete custody and population provenance remain separately gated.

## Synchronization and lifetime posture

The concrete source is immutable after successful one-shot construction in C03e-DX.

A shared borrowed source can therefore satisfy later read-only requester-policy selection without introducing an additional lock in this checkpoint.

C03e-DX does not select:

- live mutation synchronization;
- `RwLock`/`Mutex` ownership;
- policy refresh notifications;
- snapshots/leases/TTL;
- distributed policy coherence.

The evaluator returned by `evaluator_for_requester` remains borrowed from the source, preserving the C03e-DP lifetime contract.

## Required validation tests

The exact source checkpoint should lock at least these properties:

- dedicated evaluator returns configured decision for `RequesterRendezvousStart`;
- dedicated evaluator denies every other currently represented capability;
- exact device + workspace + user lookup resolves the configured evaluator;
- absent device is `Unavailable`;
- workspace mismatch is `Indeterminate`;
- user mismatch is `Indeterminate`;
- duplicate logical device binding is rejected;
- capacity overflow beyond `MAX_REGISTERED_DEVICES` is rejected;
- concrete source implements existing `RequesterRendezvousStartPolicySource` with the dedicated evaluator type;
- concrete source/evaluator are `Send + Sync` for later borrowed async caller compatibility without adding synchronization here;
- existing source-lifetime signature test remains valid.

Tests may exercise a private implementation helper for exact logical-dimension matching. They must not add a production raw request-time selector.

## Explicitly absent

C03e-DX does not materialize or select:

- a process-global/default policy;
- fallback to existing local policies;
- requester-policy persistence/schema/serialization;
- live policy update/remove/replace;
- policy watches/reload;
- policy custody in production runtime inputs;
- concrete bootstrap population;
- user/role-to-policy derivation;
- target policy selection;
- requester/rendezvous provider changes;
- DV caller invocation;
- target producer;
- wire opcode/frame/parser/dispatcher changes;
- PRWC/PRWM mapping;
- listener/accept-loop/process-companion activation;
- readiness publication;
- runtime task/thread activation;
- networking;
- deployment/restart/recovery;
- merge.

## Validation requirement

Because C03e-DX changes Agent Rust source, the exact final head must pass canonical PRW Rust Validation including:

- locked dependency graph;
- rustfmt;
- Clippy;
- workspace tests;
- workspace build.

Any automatically triggered Android/native validation must also reach terminal PASS before durable closure.

C02f-AD/C02f-AE workflows may be terminal `SKIPPED`; `SKIPPED` must not be reported as PASS.

No exact-final-head workflow may remain pending or failing at durable closure.

## Safe successor

After durable C03e-DX closure, perform a fresh exact-head topology audit before selecting any production custody/population seam.

Do not automatically combine source materialization with production runtime ownership, caller invocation, target production, wire exposure, bootstrap/listener/network activation, persistence, deployment, restart/recovery, or merge.
