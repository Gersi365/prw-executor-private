# C03e-TE Production Repeated-Admission Caller and Higher-Owner Propagation Selection — Staging

## Status

`SELECTION — SOURCE MATERIALIZATION NOT AUTHORIZED BY THIS FILE`

This document selects the next narrow production caller-migration and higher-owner propagation shape after evidence-closed C03e-TD. It is documentation only. It does not materialize Rust source, bind an executable application-lease configuration source, activate a listener/runtime, merge, deploy, restart, convert a PR to ready, or close a PR.

## Authoritative predecessor

C03e-TD is the exact predecessor.

- PR: `#643`
- branch: `phase-152-c03e-td-core-timing-interface-error-transaction-source-materialization`
- head: `9f92674cb0448c0295900fb17bed57eeb70a29a5`
- tree: `3cb50f4ed3ef46846f40cca33f5734eb5a743a2c`
- direct parent of the future TE selection commit: exact TD head above
- TD state at selection recovery: draft / open / unmerged / evidence-closed
- `main` remains outside this line of mutation

C03e-TD materialized the core production timing/lease transaction shape but deliberately stopped before the repeated-admission caller and higher-owner propagation chain.

## Selected law

`NEW_SIBLING_PRODUCTION_REPEATED_ADMISSION_LANE / GENERIC_PRIVATE_REQUEST_PREPARATION_SUCCESS_CARRIER / F_RETURNS_REMOTE_SESSION_PRODUCTION_PRE_AJ_TIMING / PRE_AJ_F_TO_K_CUSTODY_UNCHANGED / VALIDATED_REMOTE_SESSION_APPLICATION_LEASE_POLICY_PROPAGATED_BY_VALUE_FROM_HIGHER_OWNER / NO_RAW_LEASE_LIFETIME_OR_DEFAULT_IN_PROPAGATION_CHAIN / TD_FRESH_PROOF_THEN_POST_AUTH_LEASE_TRANSACTION_SELECTED / POST_AUTH_LEASE_FAILURE_REMAINS_ADMISSION_E_NOT_K / SAME_REQUEST_OWNED_VERIFIER_PROVIDER_RETAINED_FOR_WORKER_AFTER_SUCCESS / EXISTING_COMPATIBILITY_LANES_PRESERVED / FOUR_PATH_PROPAGATION_CEILING / NO_CONFIG_SOURCE_BINDING / NO_RUNTIME_ACTIVATION`

## Current exact source anchors at C03e-TD

The following exact TD source state motivates the selection.

### TD core transaction

`crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`

- blob: `76bb47248dcac1fd2f58787bb38cc92166b2ecf3`
- selected production sibling already exists:
  `admit_expected_remote_device_session_with_fresh_verifier_time_and_application_lease_policy`
- it accepts:
  - expected logical `DeviceId`
  - `SessionId`
  - challenge validity only
  - authentication request ID
  - the exact request-owned mutable fallible verifier-time provider
  - a validated `RemoteSessionApplicationLeasePolicy`
- it accepts no compatibility authentication-now value and no prebuilt absolute application-lease range
- it uses the retained provider for fresh proof time through the existing authentication transaction
- only after authentication success, it samples the same provider again for application-lease issue time
- it uses checked expiry arithmetic
- lease-source failure returns `ApplicationLeaseVerifierTime`
- expiry overflow returns `ApplicationLeaseExpiryOverflow`
- either lease-preparation failure closes the same authenticated peer with the existing binding-stage code-2 diagnostic

### Current production repeated-admission collection

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

- blob: `5e7926a3dc187de4a6aa6f27b0b77ef4208488d2`
- private `prepare_expected_request_with_timing_result` is still hard-wired to successful `RemoteSessionRealAdmissionTiming`
- current fallible production F shape is still:
  `FnMut(&DeviceId) -> Result<RemoteSessionRealAdmissionTiming, TimingError>`
- the current fallible production body still destructures:
  - challenge-validity range
  - compatibility authentication-now field
  - preconstructed absolute application-lease range
- it still calls historical `admit_expected_remote_device_session_with_fresh_verifier_time`
- K currently owns only timing-source failure before admission, together with the untouched request

### Current endpoint forwarding

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

- blob: `efb43b5d5825e8a5cd225b91a8584b03dcc648f9`
- current higher-observation fallible-admission-timing lane forwards `RemoteSessionRealAdmissionTiming`
- K remains distinct from `on_admission_failure`

### Current Linux composition

`crates/prw-agent/src/linux_bootstrap.rs`

- blob: `0a41a8ea58cc47757c24cf314035452f9100c1e5`
- current fallible-admission-timing Linux composition carries `RemoteSessionRealAdmissionTiming` through the generic production input aggregate
- it forwards K separately into the endpoint lifecycle

### Current higher-owner custody

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

- blob: `b387b49bb179a052a9eac7b29d4b00bbb31884ca`
- current configured-production-source higher-owner sibling accepts an F returning `RemoteSessionRealAdmissionTiming`
- K is forwarded only after existing configured production-source population succeeds
- no application-lease policy parameter is currently propagated through this chain

## Selection 1 — preserve old lanes; add one new sibling production lane

The historical repeated-admission, endpoint, Linux and higher-owner APIs remain compatibility surfaces.

TE selects adding a new sibling production lane rather than rewriting the existing fallible-admission-timing lane in place.

The old `RemoteSessionRealAdmissionTiming`-based overloads remain callable and retain their current behavior.

The new sibling lane alone uses:

`RemoteSessionProductionPreAjTiming`

plus a separately supplied validated:

`RemoteSessionApplicationLeasePolicy`.

No historical API is reinterpreted to give old fields new meaning.

## Selection 2 — genericize only the private request-preparation success carrier

The private helper:

`prepare_expected_request_with_timing_result`

may be behavior-preservingly generalized over a success carrier type `Timing`.

Selected semantic shape:

```text
fn prepare_expected_request_with_timing_result<D, T, V, F, R, Timing, TimingError, K>(...)
    -> Option<(RemoteSessionExpectedDeviceAdmissionRequest<D, T>, Timing)>
where
    F: FnMut(&DeviceId) -> Result<Timing, TimingError>,
    ...
```

This genericization must not change:

- duplicate-active-device preflight ordering
- the rule that duplicate rejection occurs before F invocation
- exact request ownership
- exact DeviceId selector/correlation role
- K invocation only on F error
- K receipt of the exact error plus untouched request
- no retry/requeue on F error
- no AJ/session/transport/scheduling/callback fabrication on F error

No public generic timing abstraction is selected.

## Selection 3 — exact new F shape

For the new production sibling lane, F is selected as:

```text
FnMut(&DeviceId)
    -> Result<
        RemoteSessionProductionPreAjTiming,
        RemoteSessionAdmissionTimingSourceError<Cause>
    >
```

This F source owns only pre-AJ challenge timing.

For each eligible vacant request:

1. duplicate-active-device preflight completes first
2. F is invoked exactly once
3. F success yields only `RemoteSessionProductionPreAjTiming`
4. the timing value is consumed into only `challenge_validity_unix_seconds`
5. no proof time is sampled by F
6. no application-lease issue time is sampled by F
7. no application-lease lifetime is selected by F
8. no absolute application-lease interval is built by F

DeviceId remains selector/correlation input only and does not authorize or define lease duration.

## Selection 4 — K remains the exact pre-AJ timing-failure custodian

TE does not redesign K.

Selected K shape remains the existing typed family:

```text
RemoteSessionAdmissionTimingFailure<
    D,
    T,
    RemoteSessionAdmissionTimingSourceError<Cause>
>
```

On F error:

- the exact timing-source error is retained
- the exact request remains untouched and recoverable by K
- the request-owned dispatcher remains inside the request
- the request-owned verifier provider remains inside the request
- no peer is accepted
- no challenge is sent
- no proof is requested
- no application lease is sampled or constructed
- no admission failure E callback is invoked
- no retry/requeue is selected

`InvalidApplicationLease` and `ArithmeticOverflow` variants that may exist on historical timing error families are compatibility artifacts and are not repurposed for post-auth lease failure.

## Selection 5 — validated application-lease policy is a separate higher-owner input

The new propagation chain accepts exactly one already-validated:

`RemoteSessionApplicationLeasePolicy`

by value.

The typed value represents the TB/TD policy:

- positive whole seconds
- `1..=MAX_REMOTE_SESSION_LEASE_SECONDS`
- current maximum `3600`
- no implicit default
- no fallback
- no remote-selected duration

The propagation chain must not accept any of these in place of the typed policy:

- raw `u64` lifetime seconds
- `Option<u64>`
- environment variable text
- CLI text
- static fallback duration
- DeviceId-derived duration
- transport/IP/endpoint-derived duration
- request-ID-derived duration
- scheduling-derived duration
- requester/rendezvous-derived duration
- capability-authority-derived duration

TE selects no raw configuration source and no policy-construction site.

A future separately authorized configuration checkpoint must decide how a trusted process owner obtains raw lifetime input and calls `RemoteSessionApplicationLeasePolicy::new`.

## Selection 6 — policy validation precedes all request-specific admission work

The new sibling higher-owner chain receives the already-validated policy before it enters the repeated-admission lifecycle.

Therefore invalid raw policy cannot reach:

- request duplicate preflight
- F
- peer acceptance
- challenge preparation
- proof submission
- post-auth lease-time sampling
- binding
- worker spawn

TE does not add a policy-validation failure callback to the per-request K or E lanes.

Raw policy validation remains a future process/configuration-owner concern.

## Selection 7 — exact repeated-admission call order

For each eligible request in the new lane, the selected order is:

1. active DeviceId duplicate preflight
2. one F invocation for `RemoteSessionProductionPreAjTiming`
3. split exact request ownership into:
   - expected DeviceId
   - SessionId
   - authentication request ID
   - dispatcher
   - mutable request-owned fallible verifier provider
4. consume pre-AJ timing into challenge-validity range only
5. invoke TD sibling:
   `admit_expected_remote_device_session_with_fresh_verifier_time_and_application_lease_policy`
6. TD transaction resolves current transport authority and accepts the exact peer
7. current-registry challenge preparation
8. fresh authentication/proof using the exact request-owned verifier provider
9. only after authentication success, TD samples the same provider again for lease issue time
10. checked expiry construction using the propagated validated policy
11. existing bound-session composition
12. on success, the mutable borrow ends and the same provider object is moved with dispatcher/session owner into existing worker admission custody

No provider clone, replacement, hidden global clock, cached verifier time, challenge-derived lease time, or pre-AJ lease range is selected.

## Selection 8 — post-auth lease failure remains E, never K

The existing `on_admission_failure` callback E remains the owner of every error returned by the TD admission transaction.

Therefore:

- `ApplicationLeaseVerifierTime(PrwaVerifierSourceError)` -> E
- `ApplicationLeaseExpiryOverflow` -> E
- structural bind/lease rejection -> existing `Binding(RemoteBridgeError)` -> E

These are not F errors and are not sent to K.

No post-auth lease failure reconstructs an untouched pre-auth request.

No post-auth lease failure requeues, retries, re-authenticates, invents pending-session abort, or invents authenticated-session deletion.

## Selection 9 — worker/provider custody after success is unchanged

Successful TD admission borrows the request-owned verifier provider only for the admission future.

After that future completes successfully and is dropped:

- the same provider object remains owned by the repeated-admission caller
- it is moved unchanged into the existing `RemoteSessionWorkerAdmission`
- worker verifier-time semantics remain unchanged
- no fresh provider is constructed for the worker
- no provider clone is selected

Dispatcher custody is likewise preserved.

## Selection 10 — scheduling, producer and shutdown semantics are unchanged

The new sibling collection lane must preserve the existing cooperative producer/scheduling semantics:

- current active-worker bound validation
- ready completion reaping
- request-source-open state
- producer receipt handling
- shutdown ordering
- suppression callback behavior
- receipt observation behavior
- worker cancellation/drain behavior
- in-flight admission drain behavior
- post-shutdown success orderly-close behavior
- active DeviceId uniqueness

TE selects no scheduling redesign.

## Selection 11 — endpoint lifecycle propagation sibling

A new crate-internal endpoint-lifecycle sibling is selected.

It preserves the existing higher-observation fallible-admission-timing boundary but adds:

- `application_lease_policy: RemoteSessionApplicationLeasePolicy`
- F success type `RemoteSessionProductionPreAjTiming`

It forwards unchanged:

- durable capability authority
- requester/rendezvous policy/authority
- session authentication custody
- expected-request receiver
- dispatcher factory
- expected-request sender
- observation projection
- rejection callback R
- admission failure E
- timing failure K

It performs no lease validation, time sampling, raw configuration read, or runtime activation.

## Selection 12 — Linux composition propagation sibling

A new crate-internal Linux composition sibling is selected beside the existing historical fallible-admission-timing function.

Its inputs retain the existing generic production input aggregate. The aggregate may continue to carry F generically; TE does not require a new broad aggregate solely for the timing carrier.

The Linux sibling additionally receives the already-validated `RemoteSessionApplicationLeasePolicy` by value and forwards it into the new endpoint-lifecycle sibling.

It must not:

- read raw lease configuration
- call `RemoteSessionApplicationLeasePolicy::new`
- create a default lease policy
- sample verifier time
- construct a lease range
- alter dispatcher factory semantics
- alter remote-process companion/runtime composition
- activate any new listener/readiness behavior

## Selection 13 — higher-owner propagation sibling

A new crate-private higher-owner sibling is selected beside:

`run_with_production_durable_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion_from_configured_production_sources_with_fallible_admission_timing`

The new sibling accepts:

- F returning `RemoteSessionProductionPreAjTiming` under the existing `RemoteSessionAdmissionTimingSourceError<Cause>` family
- the already-validated `RemoteSessionApplicationLeasePolicy`
- existing completion/rejection/admission-failure callbacks
- existing timing-failure K

It may continue to reuse the existing configured-production-source population path for the unrelated already-selected production inputs.

The application-lease policy itself is **not** populated by that existing configured-source helper in TE.

After existing population succeeds, the higher owner forwards the exact typed policy by value to the new Linux composition sibling.

K remains outside configured population exactly as in the current lineage: configured population failure short-circuits before K is forwarded into the runtime composition.

## Selection 14 — no new aggregate solely for the lease policy

TE does not select widening the existing Linux production input aggregates merely to store `RemoteSessionApplicationLeasePolicy`.

Preferred propagation is an explicit typed function parameter through the new sibling chain.

Rationale:

- keeps ownership visible
- prevents accidental coupling to existing configured-source population
- avoids implying a raw config source has been selected
- avoids silently changing compatibility aggregates
- preserves narrow source scope

A future checkpoint may revisit aggregation only if compile-time ownership requires it and must justify that separately.

## Selection 15 — no executable configuration source in TE

Explicitly not selected:

- environment variable name for application lease lifetime
- systemd credential name
- CLI option
- config file field
- hard-coded default
- fallback to 3600 seconds
- fallback to challenge lifetime
- remote negotiation
- per-device override
- per-request override
- lease renewal/refresh/sliding policy

Any such source or policy is a separate gate after the typed propagation lane exists.

## Selection 16 — no production activation in TE

TE performs no source materialization and selects no immediate activation.

Even after a future source-materialization checkpoint, these remain separately gated:

- executable raw lease configuration source
- concrete producer for fresh challenge timing if not already separately materialized
- `main.rs` wiring
- listener/readiness activation
- network exposure change
- service-manager activation
- deployment/restart
- merge

## Selected future source-materialization ceiling

If separately authorized, the first propagation source checkpoint should be limited to exactly these four existing Rust paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`
3. `crates/prw-agent/src/linux_bootstrap.rs`
4. `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

The first source checkpoint should not modify:

- TD core transaction files
- `remote_session_capability_runtime.rs`
- `RemoteSessionExpectedDeviceAdmissionRequest`
- application lease raw config sources
- `main.rs`
- workflow files
- Cargo manifests/lockfile unless an unforeseen compile requirement is separately reviewed
- deployment/service files

## Selected future source tests

A future separately authorized 4-path source checkpoint should prove at least:

1. historical `RemoteSessionRealAdmissionTiming` lanes still compile unchanged
2. private request preparation accepts both old and new success carriers without changing rejection/K semantics
3. duplicate-active-device rejection occurs before new F invocation
4. F failure reaches K with exact error + untouched request and does not reach E
5. new F success carries challenge timing only
6. new collection sibling accepts a validated `RemoteSessionApplicationLeasePolicy`, not raw seconds
7. new collection sibling invokes the TD fresh-proof + post-auth-lease transaction
8. post-auth lease verifier failure remains E and never K
9. post-auth lease overflow remains E and never K
10. structural binding errors remain E as `Binding`
11. the same request-owned verifier provider object is retained for worker custody after successful admission
12. endpoint/Linux/higher-owner siblings forward the exact typed policy without defaulting or reconstructing it
13. existing configured production-source population does not acquire responsibility for the lease policy
14. scheduling/producer/shutdown callbacks preserve current ordering
15. no source path outside the selected four-path ceiling changes

## Explicit non-selection / STOP

C03e-TE does not authorize:

- Rust source materialization
- raw application-lease config binding
- application-lease defaulting
- K source redesign
- request type redesign
- TD transaction redesign
- production listener/readiness/runtime/network activation
- `main.rs` wiring
- dependency upgrades
- architecture redesign
- merge
- deploy
- restart
- ready conversion
- PR close
- branch deletion
- history rewrite / force push
- repository configuration changes
- destructive evidence cleanup

The next checkpoint after TE, if authorized, is source materialization of only the selected four-path sibling propagation lane.