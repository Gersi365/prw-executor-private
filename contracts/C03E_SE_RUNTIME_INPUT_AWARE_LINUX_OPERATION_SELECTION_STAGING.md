# C03e-SE — runtime-input-aware Linux operation source-seam selection

Status: `SELECTION — VALIDATION/EVIDENCE PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_HIGHER_OBSERVATION_RUNTIME_INPUT_AWARE_LINUX_OPERATION_SEAM_SELECTION`

Selected future boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_HIGHER_OBSERVATION_RUNTIME_INPUT_AWARE_LINUX_OPERATION_SOURCE_MATERIALIZATION`

This checkpoint is documentation-only. It selects one future dormant Linux source boundary and does not materialize Rust/source/runtime behavior.

## Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-SD:

- predecessor head: `e78aaf1fcea2b957ec95ece39efedd394c8de1ad`;
- predecessor tree: `22cee8bf42b877df19d54ce822b6daf695270331`;
- predecessor PR: `#620`, draft/open/unmerged;
- predecessor status binding: `SOURCE MATERIALIZED — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- immutable SD evidence Drive ID: `1zxRNnxdfp76uv1bDzWdf3RcPlDSWquPZ`;
- exact `linux_bootstrap.rs` blob: `316d2dcc01dc9298f85d21f0f2ca5e8cbab4b00f`;
- exact endpoint-owner child blob: `881846753f51bdf94cff32ff0f9649dbcf50a80f`;
- exact parent `remote_session_capability_runtime.rs` blob: `2b6a0fd693f2ddacec608018a4db811cc36ef4c0`;
- exact higher-owner custody blob: `093cff1e4643f995f0cdc5e337ecfc3bbc2ec582`.

Integrated `main` was freshly re-read immediately before this selection write and remained:

- head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

Fresh branch and all-state PR searches returned no existing `C03e-SE` or `C03e-SF` successor before mutation. The canonical Drive folder also returned no exact-title collision for the intended SE audit filename.

## Exact-current source finding

Exact SD source now contains all lower dependencies needed for a one-file Linux composition seam.

### Existing status-only dispatcher factory

`crates/prw-agent/src/linux_bootstrap.rs` already contains private:

`linux_agent_production_remote_capability_dispatcher_factory_from_runtime_inputs(...)`

The helper:

- accepts one existing `LocalLinuxProductionRuntimeInputs<'_>` bundle;
- calls existing `LinuxAgentProductionRemoteCapabilityDispatcherSource::from_runtime_inputs(...)` exactly once;
- captures only the immutable production status snapshot through that existing source;
- returns a producer-owned factory behavior equivalent to `FnMut() -> LinuxAgentProductionRemoteCapabilityDispatcher`;
- calls existing `source.new_dispatcher()` once per factory invocation;
- carries a Rust 2024 `use<>` return bound so the returned factory does not retain the runtime-input lifetime.

The factory performs no status refresh, host query, readiness inference, synchronization, shared mutable state, retry, fallback or alternate dispatcher provenance.

### Existing requester/rendezvous durable Linux operation precedent

The same exact `linux_bootstrap.rs` already contains:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(...)`.

That operation proves the accepted Linux ownership pattern for this layer:

- consume one existing `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>` aggregate;
- wrap the retained bounded requester policy source exactly once in `Arc`;
- retain the already-preconstructed `SharedRequesterRendezvousAuthority` unchanged;
- accept one explicit `Arc<ProductionDurableCapabilityAuthority>`;
- preserve one exact executor through production reachability bootstrap and endpoint bind;
- preserve shutdown-controller publication ordering;
- delegate endpoint lifecycle once;
- avoid executable caller wiring, readiness policy invention, retry or second runtime/teardown.

C03e-SE does not mutate this historical operation and does not reuse its historical infallible callback projection as the new observation family.

### Existing SD bounded producer-observation adapter

The exact SD endpoint child exposes crate-visible:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_expected_device_admission_producer_with_higher_observation_projection(...)`.

Its higher interface already accepts:

- current capability authority by shared reference;
- exact `Arc<ProductionDurableCapabilityAuthority>`;
- requester policy source by `Arc`;
- shared requester/rendezvous authority by reference;
- mutable session-authentication custody;
- one typed expected-request receiver;
- one caller-borrowed dispatcher factory;
- one caller-borrowed typed sender;
- one bounded higher observer `FnMut(DeviceId, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection)`;
- existing admission timing, rejection and admission-failure callbacks.

It keeps raw fallible handoff receipts private, maps them synchronously once, forwards only bounded authority-free observation, creates no channel, clones no sender and adds no higher-owner invocation site.

The parent module already re-exports the bounded SD observation family crate-privately, so `linux_bootstrap.rs` can name the bounded projection without widening the raw receipt or child-private types.

### Existing runtime-input companion seam

The exact Linux bootstrap already has private:

`with_initial_runtime_inputs(...)`

and:

`run_with_remote_process_companion_inputs(inputs, operation)`.

The existing public `run_with_remote_process_companion(operation)` obtains runtime inputs only after the remote operation has already been supplied. Therefore it cannot by itself derive the SB status-only dispatcher factory from the exact runtime-input bundle before the operation is built.

The private `run_with_remote_process_companion_inputs(...)` accepts the exact runtime-input bundle and one already-built remote operation. This is the minimal existing same-file seam that permits one future sibling to:

1. observe the exact immutable runtime-input bundle;
2. derive the existing SB dispatcher factory once;
3. build one remote operation that owns that factory;
4. pass the same runtime-input bundle into the unchanged local signal-aware companion runner.

`LocalLinuxProductionRuntimeInputs<'_>` is an immutable copyable bundle. The selected future seam must not mint a second status snapshot merely to solve operation construction ordering.

## Selected future source ceiling

A separately gated C03e-SF source-materialization checkpoint may change exactly one Rust path:

`crates/prw-agent/src/linux_bootstrap.rs`

Required predecessor blob:

`316d2dcc01dc9298f85d21f0f2ca5e8cbab4b00f`.

No endpoint child, parent runtime module, higher-owner custody, lower executor/producer source, Cargo/lockfile, workflow, Android, packaging, service or executable path is selected for mutation.

If correct SF materialization requires any second Rust path, raw receipt visibility widening, higher-owner source mutation, channel-custody mutation, `run()`/`main.rs` mutation, dependency/workflow change or new runtime authority, C03e-SF must STOP and return to selection.

## Selected future Linux composition law

C03e-SF may add only a dormant same-file Linux operation/companion composition sufficient to bind the already-existing dependencies. Exact helper names may vary for rustfmt/Clippy ergonomics, but semantics must remain within this section.

The minimal future composition may consist of one operation factory plus one same-file companion wrapper only where both are strictly required to preserve runtime-input ordering. No new public surface is selected.

### Runtime-input acquisition and dispatcher provenance

The future companion wrapper must:

- reuse existing `with_initial_runtime_inputs(...)` exactly once;
- receive the exact `LocalLinuxProductionRuntimeInputs<'_>` produced there;
- invoke existing `linux_agent_production_remote_capability_dispatcher_factory_from_runtime_inputs(...)` exactly once on that exact bundle;
- retain the returned dispatcher factory only inside the one remote operation;
- pass the same runtime-input bundle unchanged into existing `run_with_remote_process_companion_inputs(...)` exactly once.

It must not construct another `LocalAgentStatusSnapshot`, re-read host/runtime status, refresh readiness, create shared mutable dispatcher state or expose the dispatcher source outside `linux_bootstrap.rs`.

### Existing typed input custody

The future operation must consume one existing production reachability/requester-rendezvous typed aggregate by value and preserve its existing custody split:

- production reachability inputs;
- bounded requester/rendezvous start policy source;
- preconstructed `SharedRequesterRendezvousAuthority`.

The requester policy source may be wrapped exactly once in `Arc`, matching the existing durable projection operation precedent. The shared requester/rendezvous authority must be retained unchanged. The existing explicit `Arc<ProductionDurableCapabilityAuthority>` must be forwarded unchanged.

No requester-policy lookup, requester provider registration, durable authorization decision, scheduling grant derivation or provider refresh is selected at operation-factory construction time.

### Expected-request channel ownership

The future operation may accept exactly one already-existing typed expected-request sender by value from a future caller, retain that exact sender without cloning it, and borrow it only for the SD endpoint invocation.

The exact receiver remains the receiver already carried inside the typed remote-process inputs and must move exactly once into the SD endpoint adapter.

C03e-SF must not:

- construct the QL capacity-one channel;
- invoke QL `into_parts()` itself unless the exact channel custody has already been supplied inside the same one-file Linux boundary without higher-owner mutation;
- clone the sender;
- create a spare sender;
- create a second channel;
- replace capacity `1` with any alternate capacity;
- use `try_send`, `blocking_send`, reserve/try-reserve, timeout escape, callback `block_on`, retry/requeue or an unbounded channel.

The preferred selected boundary is stricter: channel construction and ownership split remain higher-owner work for a later separately gated checkpoint; SF accepts already-separated sender/receiver custody through typed inputs plus the one exact sender.

### Bounded observation callback

The future sibling must use only the SD bounded observation family:

`FnMut(DeviceId, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection)`.

The existing generic completion-callback slot in the typed remote-process input aggregate may be instantiated with this bounded observer for the new sibling and moved exactly once into the SD adapter. No second callback store, callback fan-out, raw receipt observer or historical completion aggregate reconstruction is selected.

Requester `DeviceId` remains requester correlation only. The bounded projection remains authority-free and owns no scheduling grant, request, dispatcher, sender, receiver, endpoint, transport, capability authority, verifier-time provider, retry token, continuation, stream or task handle.

### Exact fallible verifier-time type

The new Linux seam must bind expected-request verifier time using the exact existing public underlying function-pointer type:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

It must not sample verifier time during operation construction, default or retry verifier time, cache a sampled value, flatten verifier-time failure or widen the private child alias.

### Runtime composition ordering

If the dormant operation is later invoked by a separately gated caller, it must preserve the existing Linux production order:

1. create exactly one `RemoteSessionExecutorRuntime`;
2. bootstrap production reachability custody for the already-selected peer;
3. bind the endpoint from existing systemd credentials;
4. publish the existing shutdown controller;
5. invoke the SD higher-observation endpoint adapter exactly once;
6. let the existing lower lifecycle own close/wait-idle/finalization semantics.

The future sibling may not create a second executor, second endpoint, second teardown path, second remote companion, detached producer task or alternate shutdown controller.

## Identity and authority preservation

C03e-SE preserves all existing separations:

- PRW logical device identity is distinct from IP/transport identity;
- requester `DeviceId` is requester correlation only;
- target expected `DeviceId` remains sourced only from the already-selected scheduling-grant/request-construction lineage;
- requester scheduling `SessionId` remains distinct from target admission `SessionId`;
- authentication request ID remains independently sourced;
- capability authority remains distinct from identity, transport and scheduling authority;
- status snapshot provenance is observation only and cannot mint authorization;
- queue acceptance is not authentication, admission, authorization, endpoint success or readiness.

No static IP or endpoint value may become logical identity or capability authority through this composition.

## Higher-owner boundary remains separate

Exact SD higher-owner custody remains guard-only at blob:

`093cff1e4643f995f0cdc5e337ecfc3bbc2ec582`.

C03e-SE does not select mutation of `production_durable_capability_higher_owner_custody.rs` for SF.

A later checkpoint must separately decide how the existing QL capacity-one channel custody is constructed/split, how the sole sender and receiver are transferred into the selected SF Linux seam, and where the higher-owner caller invokes that seam.

That later selection must preserve:

- exactly one capacity-one channel;
- no sender clone;
- receiver move exactly once;
- no duplicate durable/requester authority bootstrap;
- no alternate dispatcher provenance;
- no executable activation unless separately authorized.

## Separately gated after C03e-SE

C03e-SE does not select or authorize:

- C03e-SF source materialization inside this checkpoint;
- mutation of `production_durable_capability_higher_owner_custody.rs`;
- QL channel construction or split in a production caller;
- higher-owner sender/receiver integration;
- higher-owner invocation of the future SF seam;
- process executable caller migration;
- `run()` or `main.rs` mutation;
- listener/readiness/network activation;
- public/LAN bind expansion;
- TUN/TAP, route, firewall or NAT mutation;
- production STUN/ICE or relay deployment;
- system resolver/private-DNS mutation;
- production Agent replacement/restart;
- production transport credential provisioning;
- target dialing;
- database/schema/control-plane mutation;
- authentication cutover;
- dependency/manifest/lockfile/workflow/Android-source mutation;
- package/service/systemd mutation;
- repository configuration mutation;
- merge, deployment, restart or recovery activation;
- Phase 153 or Phase 154 work.

## Validation and evidence requirements for this selection checkpoint

C03e-SE is valid only if final GitHub topology proves:

- direct exact SD -> SE ancestry;
- ahead `1`, behind `0`;
- merge base exact SD head;
- exactly one changed documentation path, this contract;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/packaging/deployment/repository-config mutation.

PASS claims must bind only to the exact final SE head. `SKIPPED` is not PASS. No SD or historical workflow result may be inherited as SE authority. A docs-only SE head does not require an Android PASS unless an Android workflow actually registers for that exact head; no absent or historical Android run may be represented as PASS.

Immutable evidence must be published exactly once into the canonical PRW Drive evidence folder after exact-final-head validation. Before upload there must be a fresh exact-title zero-collision search in the canonical parent. After upload there must be metadata verification, complete raw/readback byte identity where supported, exact byte/hash/final-LF verification, singleton exact-title post-search and one-revision/no-predecessor verification.

The immutable audit must remain frozen after publication. Its internal status may remain `SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`; the draft PR body becomes the verified post-publication closure binding.

## Explicit non-actions / STOP

This selection checkpoint performs no Rust/source/runtime mutation. It does not construct or split a channel, clone/move a sender into live runtime, transfer a receiver into live runtime, invoke the dispatcher factory at runtime, invoke the SD endpoint adapter, sample verifier time, construct/send an expected-device request, interpret a raw receipt outside the endpoint child, spawn a remote producer task, activate a listener, publish new readiness, change network state, deploy, merge, convert the PR ready-for-review, close the PR, delete a branch, rewrite history or mutate repository configuration.

Keep the C03e-SE PR draft/open/unmerged.

After evidence closure: `STOP`. A fresh exact-head audit is required before any C03e-SF source materialization.
