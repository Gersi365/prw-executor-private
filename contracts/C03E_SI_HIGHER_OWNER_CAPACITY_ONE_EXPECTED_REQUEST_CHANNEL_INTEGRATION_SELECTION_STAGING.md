# C03e-SI — higher-owner capacity-one expected-request channel integration selection

Status: `SELECTION — VALIDATION/EVIDENCE PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_HIGHER_OBSERVATION_RUNTIME_INPUT_AWARE_LINUX_OPERATION_HIGHER_OWNER_CAPACITY_ONE_EXPECTED_REQUEST_CHANNEL_INTEGRATION_SELECTION`

Selected future boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_HIGHER_OBSERVATION_RUNTIME_INPUT_AWARE_LINUX_OPERATION_HIGHER_OWNER_CAPACITY_ONE_EXPECTED_REQUEST_CHANNEL_INTEGRATION_SOURCE_MATERIALIZATION`

This checkpoint is documentation-only. It selects one future dormant higher-owner/channel integration boundary and does not materialize Rust/source/runtime behavior.

## Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-SH:

- predecessor branch: `phase-152-c03e-sh-two-path-runtime-input-corrective-source`;
- predecessor head: `fd1915ae1b95d4580d9f871288f3400346aee042`;
- predecessor tree: `1e7126e4d09d2fe7cb393f3492588fddcb72750e`;
- predecessor PR: `#624`, draft/open/unmerged/mergeable;
- predecessor status binding: `SOURCE MATERIALIZED — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- immutable SH evidence Drive ID: `1fNMrJ0xwVSn9JPGPvYxIB6fSIP8hsmjU`;
- SH `linux_bootstrap.rs` blob: `b5d236d3c0264ea8870e90af96c76294b50d0fc1`;
- SH production-reachability wrapper blob: `260d53f5c46b912a6b3a592ca58a44e82d8f6d8e`;
- exact higher-owner custody blob retained unchanged through SH: `093cff1e4643f995f0cdc5e337ecfc3bbc2ec582`;
- SD child endpoint lifecycle guard blob: `881846753f51bdf94cff32ff0f9649dbcf50a80f`;
- SD parent runtime guard blob: `2b6a0fd693f2ddacec608018a4db811cc36ef4c0`.

Exact SH validation authority remains bound only to SH head `fd1915ae1b95d4580d9f871288f3400346aee042`: Rust Validation #1856 and Android Validation #1804 succeeded; C02f-AD #1106 and C02f-AE #1097 were `SKIPPED`, not PASS.

Integrated `main` was freshly re-read immediately before this SI contract write and remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

The recovered `main` tree is the same semantic source tree recorded by SH after the forward-only README recovery. These recovery-history commits carry no C03e-SI authority.

Fresh guards before this contract write confirmed:

- the SI branch existed at exact SH head/tree with zero additional commits;
- this exact SI contract path did not exist;
- no `phase-152-c03e-sj-*` branch existed;
- no all-state PR with `C03e-SJ` in the title existed;
- the canonical Drive evidence parent contained no exact-title collision for `C03E_SI_HIGHER_OWNER_CAPACITY_ONE_EXPECTED_REQUEST_CHANNEL_INTEGRATION_SELECTION_AUDIT_2026-09-14.md`.

## Exact-current source finding

Exact SH source now contains both sides of the previously separate ownership boundary. The missing dependency is no longer a lower endpoint or Linux-bootstrap seam; it is the narrow higher-owner composition that creates/splits the already-materialized capacity-one channel, moves the receiver through existing configured production population, and transfers the sole sender plus exact populated higher-owner custody into the existing SH runtime-input-aware Linux seam.

### Existing QL capacity-one channel custody

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs` already contains private:

`LinuxAgentProductionExpectedDeviceAdmissionChannel<D, T>`.

That custody:

- owns exactly one `mpsc::Sender<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- owns exactly one `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- constructs exactly one Tokio MPSC pair through `mpsc::channel(1)`;
- retains the exact returned sender and receiver without cloning either endpoint;
- exposes only a consuming `into_parts(self)` ownership decomposition;
- performs no request construction, send, receipt mapping, task spawn, companion invocation or runtime activation.

This existing QL law remains authoritative. C03e-SI does not select a second channel abstraction or any sender duplication.

### Existing configured receiver-population path

The same higher-owner source already contains:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_configured_production_sources(...)`.

That helper already accepts the typed expected-request receiver by value and moves it through the existing configured production population chain. On success it returns one exact:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`

containing:

- the populated production reachability/requester-rendezvous typed aggregate; and
- the exact retained `Arc<ProductionDurableCapabilityAuthority>`.

The higher-owner source owns the private fields of that aggregate and may therefore consume/destructure the exact successful value without widening visibility or adding a second custody representation.

Population already has a bounded existing error family:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationError`.

No new population error type is selected.

### Existing higher-owner configured-population companion error

The same file already contains:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationCompanionError`.

It already composes exactly:

- configured production population failure; and
- `LinuxAgentBootstrapStartFailure` after population success.

It already has the required `From` conversions. The selected future composition should reuse this existing bounded error family rather than introduce a new error surface.

### Existing SH Linux seam

Exact SH `linux_bootstrap.rs` now contains crate-private:

`run_with_production_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion(...)`.

Its exact higher interface accepts:

1. one populated `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>` aggregate by value;
2. one exact `Arc<ProductionDurableCapabilityAuthority>` by value; and
3. one typed expected-request sender by value.

For this lane the dispatcher type is the existing crate-private:

`LinuxAgentProductionRemoteCapabilityDispatcher`.

The verifier-time type remains the exact existing function-pointer type:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

The SH seam derives the existing status-only dispatcher factory from the exact immutable runtime-input bundle, moves the exact receiver already embedded in the typed input aggregate into the bounded endpoint lifecycle, borrows the one exact sender for producer enqueue, and preserves the existing bootstrap/bind/controller/drive/finalization order.

It does not construct/split/clone the channel and is not invoked by `run()` or the Agent executable.

## Selected future source ceiling

A separately gated C03e-SJ source-materialization checkpoint may change exactly one Rust path and no other:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Required predecessor blob:

`093cff1e4643f995f0cdc5e337ecfc3bbc2ec582`.

Guard-only paths that must remain unchanged during SJ include at minimum:

- `crates/prw-agent/src/linux_bootstrap.rs` at blob `b5d236d3c0264ea8870e90af96c76294b50d0fc1`;
- `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs` at blob `260d53f5c46b912a6b3a592ca58a44e82d8f6d8e`;
- SD child endpoint lifecycle at blob `881846753f51bdf94cff32ff0f9649dbcf50a80f`;
- SD parent runtime module at blob `2b6a0fd693f2ddacec608018a4db811cc36ef4c0`.

No Cargo/lockfile/workflow/Android/package/service/deployment/repository-config path is selected.

If correct SJ materialization requires any second Rust path, visibility widening outside already crate-visible SH types, `run()`/`main.rs` mutation, dependency/workflow change, or new runtime authority, SJ must STOP and return to fresh selection.

## Selected future higher-owner composition law

C03e-SJ may add only one dormant crate-private higher-owner wrapper, plus strictly necessary same-file import adjustments and focused same-file tests if practical. Exact helper naming may vary for rustfmt/Clippy ergonomics, but semantics must remain within this section.

The future wrapper may execute the following ownership composition only when some later separately gated caller invokes it:

1. construct exactly one existing `LinuxAgentProductionExpectedDeviceAdmissionChannel<LinuxAgentProductionRemoteCapabilityDispatcher, fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>>` through its existing `new()`;
2. consume that custody exactly once through existing `into_parts()`;
3. retain the exact returned sender without cloning it;
4. move the exact returned receiver by value into `linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_configured_production_sources(...)` exactly once;
5. await that configured population exactly once;
6. on population success, consume/destructure the exact returned higher-owner aggregate into the exact production reachability/requester-rendezvous inputs and exact retained durable-authority `Arc`;
7. pass those exact two values plus the exact sole sender by value into the existing SH Linux seam exactly once;
8. return the SH bootstrap result through the already-existing configured-population companion error family.

No endpoint/channel/request/authority value may be recreated after ownership transfer.

## Exact channel law

The future SJ composition must preserve QL exactly:

- one bounded Tokio MPSC channel;
- capacity exactly `1`;
- one sender and one receiver only;
- zero `Sender::clone()` calls;
- zero spare sender;
- zero second receiver;
- zero second channel;
- zero unbounded channel;
- zero alternate/dynamic capacity.

The sole sender is enqueue authority only. It is not identity, authentication, admission, capability, scheduling, acknowledgement or readiness authority.

The receiver moves exactly once into existing configured production population. It must not be borrowed for a second consumer, recreated, wrapped in a second queue, or retained after that move.

Full-channel behavior remains normal asynchronous Tokio backpressure through the already-existing lower producer law. SJ must not introduce `try_send`, `blocking_send`, reserve/try-reserve, timeout escape, busy loop, callback `block_on`, detached buffering, retry/requeue or alternate queue behavior.

## Exact dispatcher and verifier-time law

C03e-SJ must not construct a dispatcher in the higher owner and must not expose the private dispatcher source.

The exact dispatcher factory remains derived only inside the existing SH Linux seam from the exact immutable `LocalLinuxProductionRuntimeInputs<'_>` status snapshot.

The exact verifier-time type remains:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

SJ must not sample verifier time, cache a verifier-time value, default/retry a verifier-time failure, flatten that failure, or substitute an alternate time source.

## Exact observation and authority law

The future SJ wrapper may accept only the callback/input types already required to instantiate the configured population chain and SH bounded higher-observation seam.

The completion observer for this lane remains the bounded authority-free projection:

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection`.

No raw handoff receipt, raw request, sender/receiver, dispatcher, scheduling grant, durable capability authority, requester/rendezvous authority, endpoint, transport, retry token, continuation or task handle may be exposed through the observer.

Requester `DeviceId` remains correlation only. Target expected-device identity remains sourced only from the existing scheduling-grant/request-construction lineage. Logical device identity remains distinct from IP/transport identity and capability authority.

## Existing error and short-circuit ordering

The future SJ wrapper should reuse:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationCompanionError`.

Required ordering:

1. channel creation/decomposition is infallible and introduces no error variant;
2. configured production population runs once with the exact receiver;
3. any configured population failure returns immediately before SH invocation;
4. only after successful population may the exact returned higher-owner custody be consumed into the SH seam;
5. SH `LinuxAgentBootstrapStartFailure` maps through the existing `Bootstrap` branch of the existing composite error.

No retry, fallback, alternate population, synthetic owner, replacement channel, second companion or recovery path is selected.

## Dormancy and invocation boundary

C03e-SJ selects source composition only. It does not select an executable invocation site.

The new wrapper must remain dormant after materialization:

- `run()` remains unchanged;
- `main.rs` remains unchanged;
- no systemd/service/package caller is added;
- no listener/readiness/network behavior is activated merely by adding the helper;
- no production Agent process is replaced or restarted.

Actual higher-owner invocation from an executable/runtime owner remains a later separately gated checkpoint after SJ evidence closure.

## Separately gated after C03e-SI

C03e-SI does not select or authorize:

- C03e-SJ source materialization inside this checkpoint;
- mutation of `linux_bootstrap.rs` or endpoint lifecycle sources;
- sender clone or alternate queue ownership;
- direct higher-owner dispatcher construction;
- raw receipt/authority visibility widening;
- `run()` or `main.rs` caller wiring;
- executable/process caller migration;
- listener/readiness/network activation;
- public/LAN bind expansion;
- TUN/TAP, route, firewall or NAT mutation;
- production STUN/ICE or relay deployment;
- resolver/private-DNS mutation;
- production Agent replacement/restart;
- production transport credential provisioning;
- database/schema/control-plane mutation;
- authentication cutover;
- Cargo/manifest/lockfile/workflow/Android-source mutation;
- package/service/systemd mutation;
- repository configuration/visibility mutation;
- merge, deployment, restart or recovery activation;
- cleanup of `__no_op_invalid__`, `tmp-do-not-use`, or other refs;
- Phase 153 or Phase 154 work.

## Validation and evidence requirements for this selection checkpoint

C03e-SI is valid only if final GitHub topology proves:

- direct exact SH -> SI ancestry;
- ahead `1`, behind `0`;
- merge base exact SH head `fd1915ae1b95d4580d9f871288f3400346aee042`;
- exactly one changed documentation path, this contract;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/packaging/service/deployment/repository-config mutation.

PASS claims must bind only to the exact final SI head. `SKIPPED` is not PASS. No SH or historical workflow result may be inherited as SI authority. A docs-only SI head does not require an Android PASS unless an Android workflow actually registers for that exact head; no absent or historical Android run may be represented as PASS.

Immutable evidence, when separately authorized after exact-final-head validation, must be published exactly once into the canonical PRW Drive evidence folder. Before upload there must be a fresh exact-title zero-collision search in the canonical parent. After upload there must be metadata/readback verification, exact byte/hash/final-LF verification where supported, singleton exact-title post-search and one-revision/no-predecessor verification.

The immutable audit must remain frozen after publication. Its internal status may remain `SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`; a future draft PR body may become the verified post-publication closure binding.

## Explicit non-actions / STOP

This C03e-SI contract write performs no Rust/source/runtime mutation. It does not construct or split the channel at runtime, clone/move a sender into live runtime, transfer a receiver into live runtime, invoke configured production population, invoke the SH Linux seam, derive a dispatcher factory at runtime, sample verifier time, construct/send an expected-device request, spawn a task, activate a listener, publish new readiness, change network state, mutate `main`, modify repository configuration, create/modify a PR, publish Drive evidence, merge, deploy, convert ready-for-review, close a PR, delete a branch, clean accidental refs, reset/rebase/squash/force-update, or rewrite history.

After this docs-only contract materialization: `STOP`. Exact-head validation, draft PR creation and immutable evidence publication remain separately gated actions. A fresh exact-head audit is required before any C03e-SJ source materialization.
