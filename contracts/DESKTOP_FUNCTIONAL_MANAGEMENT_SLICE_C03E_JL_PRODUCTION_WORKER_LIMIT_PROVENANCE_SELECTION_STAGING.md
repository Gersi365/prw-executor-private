# Phase 152 C03e-JL — Production worker-limit provenance boundary selection

Status: **SELECTION STAGING**

Target gate:
`C03E_JL_PRODUCTION_WORKER_LIMIT_PROVENANCE_BOUNDARY_SELECTED`

Intended closure classification after exact-head validation and immutable audit:
`CLOSED_PRODUCTION_WORKER_LIMIT_PROVENANCE_SELECTION`

## 1. Exact predecessor

C03e-JL is rooted exactly at the closed C03e-JK production-peer executable-input-population source-materialization head:

- branch: `phase-152-c03e-jk-production-peer-executable-input-population-source-materialization`
- head: `4caa280a327ddeb1d81ee160c4c1aee3a4a0b0ba`
- tree: `a61328f9ff5f92b9213ac5fb41f598d2817cc8d5`
- predecessor gate: `C03E_JK_PRODUCTION_PEER_EXECUTABLE_INPUT_POPULATION_SOURCE_MATERIALIZED`
- predecessor closure: `CLOSED_PRODUCTION_PEER_EXECUTABLE_INPUT_POPULATION_SOURCE_MATERIALIZATION`

C03e-JJ requires that after the first production-peer source successor, the next still-missing executable-production provenance seam be selected only after a fresh audit of the exact resulting source state. C03e-JL performs that selection only.

## 2. Exact-source observations that govern this selection

On exact C03e-JK head `4caa280a327ddeb1d81ee160c4c1aee3a4a0b0ba`, `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` retains these already-typed inputs by value:

- `bind_addr: SocketAddr`;
- `max_active_workers: NonZeroUsize`;
- `capability_authority: SharedCurrentCapabilityAuthority<P>`;
- `session_authentication: SessionAuthenticationService`;
- `expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- `admission_timing: F`;
- `on_completion: C`;
- `on_rejection: R`;
- `on_admission_failure: E`.

The exact source also shows:

1. production bind-address population already exists through `load_linux_agent_remote_bind_addr_from_env()` and the existing production-bind input-population helper;
2. production peer population is already materialized by C03e-JK;
3. `SessionAuthenticationService::new()` creates an empty in-memory authority and therefore represents composition/lifecycle custody rather than an external production-value source;
4. `SharedCurrentCapabilityAuthority::new(...)` composes already-supplied registry/policy authority and does not itself provide a narrow production-value source;
5. expected-request and admission-timing values remain caller/runtime-producer inputs and are coupled to later runtime production semantics;
6. requester/rendezvous and rejection-policy surfaces are likewise runtime/authority-coupled rather than the smallest remaining scalar production provenance seam;
7. `LocalLinuxWorkerCapacity::new(NonZeroUsize)` is explicitly caller-bounded in-memory accounting and does not provide a production configuration source;
8. `max_active_workers` therefore remains a single, explicit, strictly-positive scalar caller input with no concrete production source on the exact C03e-JK tree.

Historical C03e-BG provenance selection also required bounded worker-capacity configuration before full expected-request/remote-process input composition, while leaving concrete production configuration/source mechanisms unselected. No later exact-source evidence reviewed for C03e-JL establishes a concrete production source for `max_active_workers`.

## 3. Selected boundary

C03e-JL selects exactly one next executable-production provenance boundary:

> the production provenance boundary that supplies the existing `NonZeroUsize` value consumed as `LinuxAgentRemoteProcessOperationInputs::max_active_workers`.

This is a value-provenance selection, not a worker-runtime redesign.

The selected production source must eventually produce exactly one strictly-positive `NonZeroUsize` worker-limit value before construction of `LinuxAgentRemoteProcessOperationInputs` can be considered fully production-populated.

The selected value has only this authority:

- it bounds concurrent active worker admission where existing runtime code already consumes `max_active_workers`;
- it does not identify a device, peer, session, requester, rendezvous target, transport, endpoint, capability principal, policy principal, or request;
- it does not create worker slots, start workers, bind sockets, authenticate sessions, mutate registry/policy state, or activate the Agent runtime merely by being sourced.

## 4. Concrete source mechanism is intentionally NOT selected

The exact C03e-JK source does not establish a production worker-limit source mechanism. Therefore C03e-JL MUST NOT invent one.

C03e-JL does **not** select any of the following:

- an environment-variable name;
- a command-line flag;
- a systemd credential;
- a systemd environment entry;
- a configuration-file path or key;
- a registry field;
- a database field;
- a network/control-plane lookup;
- a compile-time constant as production policy;
- a hard-coded production value;
- a platform-specific secret/configuration store;
- a default value;
- a fallback value;
- an auto-sized value derived from CPU count, memory, connection count, registry population, or any other ambient process/host signal.

In particular, names such as `PRW_MAX_ACTIVE_WORKERS` are not selected or authorized by this checkpoint merely because an environment source would be mechanically convenient.

A concrete source mechanism requires its own evidence-driven selection checkpoint before source materialization.

## 5. Required semantics for the future concrete source

Although the transport/storage mechanism remains unselected, any later selected production worker-limit source MUST preserve these semantic requirements:

1. The produced value must be representable as `NonZeroUsize` before it enters `LinuxAgentRemoteProcessOperationInputs`.
2. Zero is invalid production provenance and must not be silently converted to one or another positive value.
3. Malformed, absent, unavailable, or out-of-range source data must not silently become a default or fallback unless a later explicit selection contract separately authorizes such semantics.
4. Source acquisition must not fabricate worker authority from active-worker observations.
5. The configured bound and current active-worker accounting remain distinct concepts.
6. `LocalLinuxWorkerCapacity` remains accounting over an already-selected bound; it is not itself production-source authority.
7. Existing `MAX_REGISTERED_DEVICES` or other repository bounds must not be repurposed as the production value unless a later exact-source selection explicitly proves and selects that relationship.
8. No retry, alternate source, cache, stale-value reuse, or dynamic refresh semantics are implied by C03e-JL.
9. Merely sourcing the value must remain dormant with respect to listener activation, task spawn, connection acceptance, authentication, readiness, and process lifecycle.

## 6. Failure boundary

C03e-JL selects only the provenance boundary, not a concrete typed source-error API.

A later concrete-source selection must define a bounded failure surface appropriate to the selected source mechanism. That later contract must decide, from exact source evidence, at least:

- missing-source behavior;
- malformed-source behavior;
- zero-value behavior;
- overflow/out-of-range behavior;
- whether the mechanism can fail for provider/I/O reasons;
- exact error propagation and stable diagnostic requirements.

Until that later selection, no new error type, variant, display string, fallback branch, retry loop, or default is authorized.

## 7. Ordering and custody

The production worker-limit source belongs before construction of the existing `LinuxAgentRemoteProcessOperationInputs` value.

The future composition shape is constrained only to this extent:

```text
selected production worker-limit source
    -> validated strictly-positive value
    -> NonZeroUsize
    -> LinuxAgentRemoteProcessOperationInputs::new(..., max_active_workers, ...)
```

This checkpoint does not select the source-call position relative to unrelated remaining provenance except that:

- bind-address production source remains already materialized and unchanged;
- production peer population remains already materialized and unchanged;
- worker-limit sourcing must not manufacture or substitute for capability/session/expected-request/timing/requester-rendezvous provenance;
- no full production input assembler is selected here.

## 8. Identity and authority invariants

C03e-JL preserves the PRW identity invariant:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

The worker-limit value is scheduling/configuration provenance only.

It MUST NOT be used as or derived into:

- `DeviceId`;
- `PeerConnectivityIdentity`;
- `TransportIdentity`;
- `SessionId`;
- requester identity;
- requester/rendezvous target identity;
- endpoint identity;
- candidate identity;
- capability identity or authorization evidence.

IP addresses remain transient endpoints only. PRWM `request_id` remains correlation only.

## 9. Exact exclusions

C03e-JL does not select, materialize, or authorize:

- Rust source changes;
- a concrete worker-limit source mechanism;
- a concrete worker-limit source name/path/key;
- a production worker-limit value;
- a default/fallback/retry/cache/refresh policy;
- dynamic worker-limit mutation;
- worker-accounting redesign;
- task spawning or cancellation changes;
- persistent-worker collection changes;
- listener/bind activation;
- endpoint lifecycle activation;
- readiness publication;
- process-signal wiring;
- `run()` or `main.rs` integration;
- capability-authority production population;
- session-authentication production population/state restoration;
- expected-request producer/channel lifecycle production;
- admission timing/clock production;
- completion/rejection/admission-failure callback production;
- requester/rendezvous policy-source population;
- requester/rendezvous runtime-provider construction/capacity/lifecycle provenance;
- registry mutation;
- policy mutation;
- candidate publication;
- traversal, dialing, retry, reconnect, rebind, rebootstrap, or replacement;
- environment/service/systemd/package mutation;
- credential/certificate/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment, restart, recovery activation, or merge;
- PR readiness conversion, PR close, branch deletion, or history rewrite.

## 10. Why the other still-injected fields are not selected first

This ordering is narrowness-driven, not alphabet-driven.

### Capability authority

`SharedCurrentCapabilityAuthority` is a composed authority object over supplied registry/policy state. Selecting production population now would require a broader registry/policy ownership and policy-source decision, not one scalar provenance seam.

### Session authentication

`SessionAuthenticationService` is an in-memory authority initialized by `new()` and populated through session lifecycle. Treating its construction as a production source would conflate lifecycle composition with value provenance.

### Expected requests

The expected-request receiver is tied to a producer/channel lifecycle and authoritative pre-handshake scheduling intent. Selecting it requires producer and runtime-lifecycle semantics beyond this scalar boundary.

### Admission timing

Admission timing is supplied through typed runtime timing values/providers. A production clock/timing policy requires a separate authority and sampling-boundary selection.

### Completion/rejection/failure callbacks

These callbacks encode runtime observation/reporting ownership, not a single configuration-value source.

### Requester/rendezvous surfaces

Requester/rendezvous inputs involve policy/provider/lifecycle authority and must not be collapsed into worker-limit configuration.

The worker limit is therefore the smallest still-missing, already-typed executable-production provenance seam on the exact C03e-JK state.

## 11. Successor rule

After C03e-JL is canonically validated and evidence-recorded, **STOP**.

The next checkpoint must begin with a fresh exact-head audit and may select only the concrete production source mechanism for the already-selected `max_active_workers: NonZeroUsize` provenance boundary.

That successor must not assume environment configuration, command-line configuration, systemd credentials, a config file, a constant, or any other transport merely from C03e-JL.

Only after a concrete source mechanism and its exact failure semantics are separately selected may a later source-materialization checkpoint add the minimum source implementation and focused tests.

If the exact resulting source state before that successor exposes a contradictory or already-authoritative worker-limit source, the successor must use that evidence rather than this contract's historical absence finding.

## 12. Validation target for C03e-JL

C03e-JL itself is documentation-only and must close only if all of the following are true on the exact final head:

- exact predecessor remains C03e-JK head `4caa280a327ddeb1d81ee160c4c1aee3a4a0b0ba` as merge base;
- branch is ahead only by the intended documentation commit(s) and is zero behind;
- exactly one changed path exists:
  `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_JL_PRODUCTION_WORKER_LIMIT_PROVENANCE_SELECTION_STAGING.md`;
- there are zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/host changes;
- repository CI required for the exact final docs-only head is observed and recorded without inheriting verdicts from another head;
- skipped workflows are recorded only as skipped, never as PASS;
- immutable Drive evidence is frozen, uploaded, raw-read back, and verified byte-exact before closure metadata is claimed;
- the PR remains draft, open, and unmerged.

Only then may the target gate be considered closed.
