# Desktop Functional Management Slice — C03e-QE Production Durable Reachability Fallible Verifier-Time Production-Source Population Companion Composition Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_REACHABILITY_FALLIBLE_VERIFIER_TIME_PRODUCTION_SOURCE_POPULATION_COMPANION_COMPOSITION_SELECTION`

Gate:

`C03E_QE_PRODUCTION_DURABLE_REACHABILITY_FALLIBLE_VERIFIER_TIME_PRODUCTION_SOURCE_POPULATION_COMPANION_COMPOSITION_SELECTED`

## 1. Exact predecessor authority

This selection is based only on the evidence-closed C03e-QD source-materialization checkpoint.

Exact predecessor branch:

`phase-152-c03e-qd-production-durable-reachability-fallible-verifier-time-higher-owner-companion-assembly-source-materialization`

Exact predecessor head:

`5affc165db2a063f96114136a71fe5912935c469`

Exact predecessor tree:

`3821bd3f7a7bea011fbd40747aabed6cc80bdce2`

Exact predecessor source blob for:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

is:

`72a3341b112c49e2d314eed0ba3986f0a4053166`

C03e-QD PR #569 is intentionally draft/open/unmerged and evidence-closed.

Canonical immutable QD audit:

`C03E_QD_PRODUCTION_DURABLE_REACHABILITY_FALLIBLE_VERIFIER_TIME_HIGHER_OWNER_COMPANION_ASSEMBLY_SOURCE_MATERIALIZATION_AUDIT_2026-09-11.md`

Drive ID:

`1lFFwqK6fDRLP-avzSCc8UGDXkATSNuBM`

Frozen/readback bytes:

`11300`

SHA-256:

`90c04d7e77f015fc35b90252f8aba727a9b5fc8640d6f13a740c18e204b895d3`

## 2. Fresh live-state audit before selection

Immediately before this selection branch was created:

- QD still resolved exactly to head `5affc165db2a063f96114136a71fe5912935c469` and tree `3821bd3f7a7bea011fbd40747aabed6cc80bdce2`;
- `main` still resolved to `7c993fa93977a0bb84e0d030874eee7fd0cae77f`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`;
- no `phase-152-c03e-qe-*` successor branch existed;
- PR #569 remained the latest PR in the active lineage and remained draft/open/unmerged;
- the selected future composition helper did not exist in the exact QD source.

A redundant repeated create-branch request after the QE branch already existed was rejected with `Reference already exists`. That request was a no-op: it did not move the branch, alter source, rewrite history, or change `main`.

## 3. Exact source finding

The exact QD higher-owner source already contains two independently proven ingredients required for the next bounded composition.

First, the existing C03e-MH population wrapper:

`linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_fail_closed_current_capability_authority(...)`

returns exactly:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<ProductionRemoteCapabilityDenyAllPolicy, D, T, F, C, R, E>`.

That wrapper already composes the existing production worker-limit/bind population, one fresh process-local `SessionAuthenticationService`, one fail-closed empty `WorkspaceDeviceRegistry` plus `ProductionRemoteCapabilityDenyAllPolicy`, one fixed logical peer source, and one same-custody durable-registry peer plus raw durable-capability authority lineage. It accepts the expected-request receiver, admission-timing provider and callbacks from its caller unchanged.

Second, C03e-QD materialized exactly:

`run_with_production_durable_reachability_remote_process_companion_with_fallible_verifier_time_completion_projection(...)`

which consumes the same higher-owner input aggregate by value and preserves the fallible verifier-time generic/callback families.

Therefore the next unresolved boundary does not require requester/rendezvous joining, a new provider, a new channel, or an executable caller. A one-file dormant composition can connect these two already-existing layers while keeping all still-unresolved provenance caller-supplied.

## 4. Selected future source boundary

The immediate future source-materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

No second source path is selected.

The future checkpoint may add one crate-private async helper with the selected name:

`run_with_production_durable_reachability_remote_process_companion_with_fallible_verifier_time_completion_projection_from_production_sources(...)`

The future helper must accept only the caller-owned surfaces already required by the existing population and QD companion layers:

- `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- `F` admission timing;
- `C` completion callback;
- `R` rejection callback;
- `E` repeated-admission-failure callback.

It must not accept or construct a requester/rendezvous owner, request sender, production dispatcher source, concrete verifier-time source, executable/process callback policy, readiness publisher, listener, or process signal surface.

## 5. Selected exact composition order

The future helper must perform exactly two stages in this order.

### Stage 1 — production-source population

Invoke exactly once and await exactly once:

`linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_fail_closed_current_capability_authority(...)`

The exact caller-supplied expected-request receiver, admission timing and callbacks must be forwarded unchanged.

Population failure must short-circuit before QD companion assembly.

### Stage 2 — QD higher-owner companion assembly

Only after successful population, move the exact returned higher-owner input aggregate by value into exactly one call to:

`run_with_production_durable_reachability_remote_process_companion_with_fallible_verifier_time_completion_projection(...)`

The QD companion result must be preserved except for wrapping in the selected bounded outer error.

No alternate population path, fallback, retry, reconstruction, clone, cache, synthetic owner, degraded authority, second runtime, second companion, or second teardown path is selected.

## 6. Selected generic and callback law

The future composition must preserve the exact QD fallible-verifier-time bounds:

- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`;
- `F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`;
- `C: FnMut(DeviceId, RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection) + Send + 'static`;
- `R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>) + Send + 'static`;
- `E: FnMut(RemoteSessionRepeatedAdmissionFailure) + Send + 'static`.

The current capability authority remains the existing fail-closed `ProductionRemoteCapabilityDenyAllPolicy` lane selected by C03e-MG/MH. This checkpoint does not select any allow-bearing current policy or registry hydration.

The exact raw durable capability authority remains same-custody with the selected peer through the existing C03e-LZ/MH population chain and remains retained through the complete QD lower operation.

## 7. Selected bounded outer error

The future materialization may add one crate-private bounded two-stage outer error named:

`LinuxAgentProductionDurableReachabilityFallibleVerifierTimePopulationCompanionError`

with exactly these semantic lanes:

1. `Population(LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError)`
2. `Bootstrap(LinuxAgentBootstrapStartFailure)`

The error must preserve each exact underlying value.

`Display` may provide bounded zero-data stage classification only.

`std::error::Error::source()` must return the population error for the population lane. The bootstrap lane must not require mutation or trait widening of `LinuxAgentBootstrapStartFailure`; if that existing type still does not implement `std::error::Error`, `source()` for the bootstrap lane must remain `None`, matching the established C03e-MV precedent.

Only narrowly necessary `From` plumbing for these two exact lanes is selected.

## 8. Explicitly caller-supplied / unresolved provenance

This selection intentionally leaves all of the following outside the future source checkpoint:

- construction or ownership of the expected-request channel;
- construction/send of any `RemoteSessionExpectedDeviceAdmissionRequest`;
- production provenance for `D` dispatcher values carried by expected requests;
- installation or selection of a concrete `T` verifier-time provider;
- verifier-time sampling outside the already-existing lower lifecycle;
- requester/rendezvous policy/provider/runtime joining;
- expected-device scheduling-authority derivation;
- durable authorization invocation;
- admission-timing production policy;
- completion/rejection/failure callback policy;
- callback logging, metrics, process-exit or restart semantics;
- executable caller population;
- `run()` or `main.rs` invocation.

The future helper is composition only. It does not resolve these provenance lanes.

## 9. Identity and custody invariants preserved

This selection does not alter established identity or custody laws:

- configured peer `DeviceId` remains logical peer intent only;
- current same-device `PeerConnectivityIdentity` remains durable-registry-derived transport authority;
- authenticated owner-derived `DeviceId` remains post-auth worker authority;
- expected pre-auth `DeviceId` remains scheduling intent;
- request IDs remain correlation only unless separately authorized;
- current capability authority and durable capability authority remain distinct lanes;
- requester/rendezvous authority remains distinct and is not joined here;
- no authority is cloned, reminted, reconstructed, cached, downgraded, or silently substituted.

## 10. Source ceiling and STOP conditions

The future source checkpoint must STOP and reselect instead of widening if correctness requires any of the following:

- a second Rust/source path;
- `linux_bootstrap.rs` mutation;
- `remote_session_capability_runtime*` mutation;
- `prw-session` mutation;
- manifest or lockfile mutation;
- workflow mutation;
- Android source mutation;
- requester/rendezvous join;
- expected-request sender/channel creation;
- concrete dispatcher or verifier-time provider population;
- callback remapping;
- executable invocation;
- listener/readiness/process-signal activation;
- service/systemd/package changes;
- credential/certificate/private-key/trust/RBAC changes;
- database/schema/control-plane changes;
- authentication cutover;
- repository visibility/configuration changes;
- deployment, restart or recovery activation.

## 11. Selection checkpoint exclusions

C03e-QE itself is documentation-only. It performs no Rust or runtime source mutation and does not materialize the future helper.

It does not merge any PR, convert any PR ready-for-review, close any PR, delete any branch, rewrite history, force-update refs, deploy, restart services, activate listeners, change readiness, mutate repository configuration, or alter `main`.

## 12. Immediate successor ceiling

If and only if exact-final-head QE validation and immutable evidence closure succeed, a separately gated source checkpoint may materialize only the one-file composition selected above.

That future checkpoint must:

- re-read exact QE head and exact predecessor source blob before mutation;
- verify the target helper/error are absent;
- modify only `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`;
- preserve the exact two-stage population-then-QD order;
- validate formatting, Clippy, tests and workspace build on its exact final head;
- preserve Android regression validation when the Rust source path triggers it;
- publish immutable evidence only after exact-final-head validation is terminal.

It must not simultaneously construct expected requests, select verifier-time provenance, join requester/rendezvous semantics, wire an executable caller, activate runtime behavior, merge, deploy, or perform destructive cleanup.

After C03e-QE closure: `STOP`.
