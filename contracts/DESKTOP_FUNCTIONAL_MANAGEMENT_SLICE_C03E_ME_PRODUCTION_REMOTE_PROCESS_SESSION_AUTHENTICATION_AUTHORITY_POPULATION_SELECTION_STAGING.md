# C03e-ME — Production Remote-Process Session-Authentication Authority Population Selection

## Status

`SELECTION — VALIDATION_PENDING`

## Gate

`C03E_ME_PRODUCTION_REMOTE_PROCESS_SESSION_AUTHENTICATION_AUTHORITY_POPULATION_SELECTED`

## Exact predecessor authority

Evidence-closed predecessor:

`C03e-MD — Production durable-reachability same-custody owner population source materialization`

Exact predecessor branch:

`phase-152-c03e-md-production-durable-reachability-same-custody-owner-population-source-materialization`

Exact predecessor head:

`2d6041fe3e15538ff22e7e555b2dc5d98b39204f`

Exact predecessor tree:

`c26c07488493310ba389dd17070fd9c028993dfb`

Exact predecessor higher-owner blob:

`b73f1d4ee8451ba7822b271b66d7a1a19a4d1f3e`

C03e-MD materialized exactly one dormant pre-requester same-custody owner population helper and left concrete production provenance for current capability authority, session authentication, expected-request producer/channel, admission timing, callbacks and requester/rendezvous separately gated.

## Selection question

Select the smallest independent concrete production provenance boundary that can be resolved from exact inherited source without choosing authorization policy, request-channel semantics, timing policy, callback policy, requester/rendezvous state, executable caller wiring or runtime activation.

## Fresh exact-source audit

### Session-authentication authority source

Exact source:

`crates/prw-session/src/lib.rs`

Exact C03e-MD blob:

`0b0b6624df93ebcf3efae632d94dfc337ee67761`

Existing type:

`SessionAuthenticationService`

is an in-memory single-use device-session authentication authority holding private pending and authenticated maps.

Existing constructor:

`SessionAuthenticationService::new()`

creates an empty fail-closed service and returns `Self` directly. Construction performs no credential read, network I/O, registry mutation, challenge generation, authentication, authorization, persistence, task spawn or runtime activation.

Fresh challenge randomness is generated only later by `begin_session(...)` through AWS-LC `SystemRandom`; randomness failure remains the existing bounded `SessionServiceError::ChallengeRandomness` path. The constructor itself is infallible.

A successful session establishes authenticated session identity only; capability authorization remains separate.

### Remote process ownership and lifetime

Exact source:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact C03e-MD blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

Existing `LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>` owns exactly one `SessionAuthenticationService` by value beside the current capability authority, expected-request receiver, timing input and callbacks.

The existing remote process operation destructures that owner into one mutable `session_authentication` value and passes `&mut session_authentication` through the repeated real remote-admission endpoint lifecycle.

No clone or independent per-worker/per-session authentication service is selected by that ownership graph.

### Endpoint repeated-admission custody

Exact source:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact C03e-MD blob:

`b1afdeba4b0bc59399ff4e4dbf480479f4fb2cfe`

Existing repeated lifecycle input includes:

`session_authentication: &mut SessionAuthenticationService`

so the exact service retained by the remote process operation is reused mutably across the repeated admission lifecycle instead of creating a fresh authority for every admission.

### Current capability authority is not this selection

Exact source:

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`

Exact C03e-MD blob:

`60307fff4dd0fd573192ba6e6fab9dedd3321dda`

`SharedCurrentCapabilityAuthority<P>` combines one `WorkspaceDeviceRegistry` and one caller-selected policy value under shared current-state custody. Selecting its production population would necessarily select or separately source registry and policy provenance. C03e-ME therefore does not infer an empty registry, allow policy, deny policy, management policy or any other capability policy.

### Admission timing remains separately unresolved

Existing `RemoteSessionRealAdmissionTiming` carries challenge-validity range, authentication current time and application-lease range. C03e-ME does not select those values, their clock source or their policy.

### Expected-request producer/channel remains separately unresolved

The existing remote process input accepts an already-created:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D,T>>`

C03e-ME does not select the sender/producer, channel capacity, producer lifecycle, dispatcher source or verifier-time source. Existing `mpsc::channel(...)` observations in `linux_bootstrap.rs` are test construction and are not production provenance.

### Requester/rendezvous remains separately unresolved

Exact policy source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

Exact C03e-MD blob:

`f7377011a3ab2034c14d9018a5c0f268f6660ffa`

Its bounded in-memory source explicitly does not wire production custody or population.

Exact runtime owner:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

Exact C03e-MD blob:

`082a70af239972a82318f3e17cb3fd8cb45d9e95`

Its constructor consumes an already-configured in-memory provider; provider capacity and lifecycle state remain caller-owned decisions.

C03e-ME does not select requester/rendezvous policy bindings, provider type/capacity/population, registration strategy or final requester/rendezvous join.

## Selected production provenance boundary

C03e-ME selects exactly one production remote-process session-authentication authority population rule:

1. for one future construction of one dormant production remote-process input owner, construct exactly one fresh `SessionAuthenticationService` by calling existing `SessionAuthenticationService::new()` exactly once;
2. move that exact service by value into the existing C03e-MD production durable-reachability population path exactly once;
3. retain the existing operation-level ownership law so the same service is later mutably reused across repeated admissions if and only if a separately gated caller eventually invokes the operation;
4. do not construct a second service for the same remote-process input owner;
5. do not construct a new service per admission, per worker or per authenticated session;
6. do not clone, `Arc`-wrap, expose, split or persist the service;
7. add no service refresh/watch/retry/fallback/recovery semantics;
8. leave all session challenge timing and lease timing inputs caller-supplied and separately gated.

The selected constructor is intentionally the existing empty fail-closed constructor. C03e-ME does not add or invent cryptography, randomness, credentials, key material, identity proof rules or authentication policy.

## Immediate later source-materialization ceiling

A later separately executed source checkpoint may materialize this selection in exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

The additive source shape is ceilinged to one crate-private dormant composition wrapper around the existing C03e-MD population helper.

Conceptually, that wrapper may accept the same still-unresolved typed inputs as the MD helper except `SessionAuthenticationService`, construct exactly one `SessionAuthenticationService::new()`, and pass that exact value by value into the existing MD helper exactly once.

Mechanical helper spelling may adjust only for Rust naming/formatting while preserving this semantic boundary.

Because `SessionAuthenticationService::new()` is infallible, the later wrapper must not invent a new population error variant merely for construction. It should preserve the existing C03e-MD `LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError` unchanged unless fresh source evidence proves a distinct fallible operation is required.

The later source checkpoint must remain dormant and add no invocation site.

## Explicit non-selection

C03e-ME does not select or authorize:

- concrete `SharedCurrentCapabilityAuthority<P>` production population;
- `WorkspaceDeviceRegistry` production population;
- capability policy type or allow/deny decision;
- session challenge validity range;
- authentication clock/current-time source;
- application lease range;
- expected-request sender/producer;
- expected-request channel capacity;
- dispatcher production source;
- verifier-time production source;
- completion callback policy;
- rejection callback policy;
- admission-failure callback policy;
- requester/rendezvous policy-source production bindings;
- requester/rendezvous provider type/capacity/population;
- requester/rendezvous registration strategy;
- final requester/rendezvous higher-owner join;
- LX companion invocation;
- mutation of `linux_bootstrap.rs`;
- mutation of `prw-session` authentication algorithms or cryptography;
- `main.rs` or `run()` mutation;
- executable caller/input assembly;
- endpoint bind/listener/readiness/publication changes;
- runtime/network activation;
- auth/authorization/trust weakening;
- identity/address/correlation conflation;
- manifest/lockfile/workflow/Android/packaging/systemd/credential/certificate/trust/RBAC/repository-configuration changes;
- merge or ready-for-review conversion;
- deploy/restart/recovery;
- PR close;
- branch deletion;
- force update/rebase/squash/history rewrite;
- destructive cleanup.

## Identity and authority invariants

Remain distinct:

- `SharedCurrentCapabilityAuthority<P>`;
- `SessionAuthenticationService`;
- authenticated `DeviceId` / `AuthenticatedDeviceSession`;
- `ProductionDurableRegistryRuntimeCustody`;
- raw `ProductionDurableCapabilityAuthority`;
- requester/rendezvous authority;
- current `PeerConnectivityIdentity`;
- reachability/socket address;
- PRWM `request_id` correlation.

Session authentication establishes authenticated session identity; it does not itself grant capability authorization, requester/rendezvous authority or transport authority.

## Validation and closure requirement

C03e-ME is documentation-only selection. Exact-final-head validation is still required before closure. Skipped workflows are not PASS and superseded-head workflows are not closure evidence.

Closure also requires immutable canonical Google Drive evidence with exact-title uniqueness and raw-byte/hash readback verification.

## STOP boundary

After verified C03e-ME closure: STOP.

C03e-ME selects only the session-authentication population boundary above. It does not pre-authorize its later source materialization or any other unresolved production provenance domain.
