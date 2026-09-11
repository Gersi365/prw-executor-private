# C03e-QI — Production Expected-Device Admission Status-Only Dispatcher Snapshot Provenance / Factory Custody Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_SNAPSHOT_PROVENANCE_FACTORY_CUSTODY_SELECTION`

Selected future boundary:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_CUSTODY_SOURCE_MATERIALIZATION`

## 1. Exact predecessor authority

This checkpoint is based only on evidence-closed C03e-QH.

Exact predecessor branch:

`phase-152-c03e-qh-production-expected-device-admission-fallible-verifier-time-request-construction-composition-source-materialization`

Exact predecessor head:

`900f31aa0e40726ddd1f4b8c1638a1e5791a2060`

Exact predecessor tree:

`62ee7284781cd8f9403c86ed0198b3d63ca13446`

Exact predecessor authorized endpoint source blob:

`808d3cae4d31214923ee5380f8ac0f76dd171485`

C03e-QH PR #573 remains draft/open/unmerged and its closure body records:

`SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

Canonical C03e-QH immutable evidence is Drive object:

`1KXFGVUond-Y3_X6-CdXBkEk7Gdlm842p`

under canonical parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`.

The frozen/readback QH evidence stream is `23046` bytes with SHA-256:

`afa0af3e590d2f3d6db02e78a315302afab32b46f811262dd80d79f34b2d595a`.

## 2. Fresh post-QH audit result

Fresh live audit before assigning QI confirmed:

- integrated `main` remained `7c993fa93977a0bb84e0d030874eee7fd0cae77f`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`;
- C03e-QH remained exact head/tree/source blob above;
- PR #573 remained draft/open/unmerged/mergeable;
- no `phase-152-c03e-qi-*` successor branch existed before QI assignment;
- no C03e-QI PR existed;
- recent PR chronology remained headed by C03e-QH #573;
- C03e-QH remained the latest C03e Q-series checkpoint.

The fresh audit therefore permits one separately gated successor selection only. It does not inherit permission to materialize the selected source boundary in this checkpoint.

## 3. Exact QH gap being resolved

C03e-QH materializes a synchronous request-construction helper generic over one exact caller-supplied dispatcher:

`D: CapabilityDispatcher + Send + 'static`.

QH explicitly leaves concrete NB status-only dispatcher production snapshot provenance/construction/transfer separately gated.

The unresolved question for this checkpoint is narrower than channel ownership or producer invocation:

> Which already-existing production Agent status value may lawfully seed the existing dormant NB dispatcher, and what custody shape may retain that value for later per-request dispatcher construction without inventing another status source?

QI selects only that question.

## 4. Existing concrete dispatcher authority

Exact-current `crates/prw-agent/src/linux_bootstrap.rs` already contains the C03e-NB materialized crate-private concrete dispatcher:

`LinuxAgentProductionRemoteCapabilityDispatcher`.

It owns exactly one:

`LocalAgentStatusSnapshot`.

Its existing constructor is:

`LinuxAgentProductionRemoteCapabilityDispatcher::new(status_snapshot: LocalAgentStatusSnapshot)`.

Its existing `CapabilityDispatcher` implementation preserves the C03e-MZ/NB status-only law:

- `BridgeCommand::AgentStatus` returns `encode_status_snapshot(self.status_snapshot).to_vec()`;
- file, transfer, terminal and forwarding command families fail closed with `UnsupportedProviderFamily`;
- the dispatcher does not read host state;
- the dispatcher does not own authentication/session authority;
- the dispatcher does not frame a PRWM response;
- the dispatcher does not construct or mutate providers.

QI does not alter that dispatcher representation or dispatch law.

## 5. Historical MZ/NA/NB authority remains applicable

Historical C03e-MZ selected the status-only adapter design against an already-existing typed production Agent status snapshot.

Historical C03e-NA selected exactly one `linux_bootstrap.rs` dormant materialization seam and deferred runtime wiring.

Historical C03e-NB materialized the current crate-private dispatcher and proved the worker bound:

`CapabilityDispatcher + Send + 'static`.

C03e-NC/ND correctly blocked caller propagation at that time because the complete expected-request producer/source composition was not then proven.

Later O/P/Q-series work has since materialized the scheduling authority, receipt representation/classification, identifier sources, fallible verifier-time propagation, and QH request-construction seam. QI does not rewrite historical NC/ND findings; it resolves only the remaining status-snapshot provenance/factory question now exposed by QH.

## 6. Exact production status snapshot provenance now proven

Fresh exact-QH source inspection proves a concrete production snapshot source already exists in `linux_bootstrap.rs`.

`with_initial_runtime_inputs(...)` constructs the fixed initial local production runtime inputs exactly once with:

`LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)`.

That exact snapshot is stored inside:

`LocalLinuxProductionRuntimeInputs<'_>`.

`LocalLinuxProductionRuntimeInputs<'_>` is `Clone + Copy` and exposes:

`status_snapshot(self) -> LocalAgentStatusSnapshot`.

The same `LocalLinuxProductionRuntimeInputs` value is passed into the existing local signal-aware production runtime / remote-companion facade.

Therefore the authoritative production snapshot provenance selected by QI is:

`with_initial_runtime_inputs(...)`
→ exact `LocalLinuxProductionRuntimeInputs::status_snapshot()`
→ owned dormant dispatcher-source custody.

No second call to `LocalAgentStatusSnapshot::current(...)` is selected for the remote dispatcher path.

## 7. Snapshot semantic law

The selected snapshot is the exact immutable production snapshot already used by the local Agent runtime profile.

QI does not reinterpret it as a dynamically refreshed process-state feed.

Specifically:

- it is not a live host-state reader;
- it is not recomputed for each expected-device request;
- it is not changed from `Ready` to `Stopping` by the remote path;
- it is not sourced from endpoint readiness, remote listener state, reachability state, requester state or worker state;
- it is not derived from a request, peer, DeviceId, SessionId, channel state or verifier time;
- it is not a substitute for authentication, authorization, admission or reachability evidence.

Remote `AgentStatus` therefore may later report only the same immutable snapshot semantics already exposed by the local production runtime input, not a stronger claim of dynamic currentness.

## 8. Selected future factory custody

A later separately gated source materialization may add one dormant private custody concept in `linux_bootstrap.rs`, equivalent in authority to:

`LinuxAgentProductionRemoteCapabilityDispatcherSource`

or an equivalently narrow private name.

It may own exactly:

- one `LocalAgentStatusSnapshot` copied from the existing `LocalLinuxProductionRuntimeInputs::status_snapshot()` provenance.

It must own no:

- expected-device scheduling grant;
- requester DeviceId;
- target expected DeviceId;
- requester or target SessionId;
- authentication request ID;
- verifier-time provider or sampled verifier time;
- expected-request sender or receiver;
- channel;
- retry token;
- endpoint owner;
- requester/rendezvous authority;
- production durable capability authority;
- session-authentication service;
- reachability owner;
- listener/readiness authority;
- host-state handle.

The custody is status data only.

## 9. Selected factory construction law

The future dormant source may expose only narrow private construction equivalent to:

`from_runtime_inputs(inputs: LocalLinuxProductionRuntimeInputs<'_>) -> Self`

or an equally strict helper that is provably called with:

`inputs.status_snapshot()`

from the same `with_initial_runtime_inputs(...)` production input bundle.

The materialized source must not call:

`LocalAgentStatusSnapshot::current(...)`

for dispatcher provenance.

It must not accept an arbitrary `LocalAgentRuntimeState` from the remote producer path.

It must not decode a status frame, read a socket, query the OS, inspect runtime counters or infer readiness.

## 10. Selected per-request dispatcher factory law

The future dormant source may expose a narrow private method equivalent to:

`new_dispatcher(&self) -> LinuxAgentProductionRemoteCapabilityDispatcher`.

That method may only invoke the already-existing constructor:

`LinuxAgentProductionRemoteCapabilityDispatcher::new(self.status_snapshot)`.

Because `LocalAgentStatusSnapshot` is `Copy`, this produces a fresh owned dispatcher without cloning a dispatcher and without introducing shared mutable state.

No `Clone` or `Copy` implementation is required for the dispatcher-source custody itself.

No `Arc`, `Mutex`, `RwLock`, atomic cell, watch channel or dynamic snapshot cache is selected.

## 11. Future transfer law into QH construction

QI selects the later transfer semantics but does not materialize or invoke them.

For each live completion already proven eligible and not shutdown-suppressed by the existing classifier path, a later concrete producer composition may:

1. obtain exactly one fresh concrete NB dispatcher from the retained dispatcher source;
2. move that dispatcher by value into the existing QH request-construction helper;
3. never reuse that dispatcher after move;
4. never use the dispatcher as eligibility, identity, acknowledgement or retry authority.

If QH identifier construction later fails, the just-created dispatcher may be dropped normally together with the failed construction attempt. No dispatcher retry custody is retained.

No dispatcher is required for a completion classified `Ineligible` or mapped to `SuppressedOnShutdown`, because those paths do not enter the existing eligible continuation construction seam.

## 12. Identity / authority separation remains exact

The dispatcher snapshot has no authority over expected-device identity.

The existing laws remain:

- requester callback `DeviceId` is requester correlation only;
- target expected `DeviceId` comes only from consuming the one-shot scheduling grant;
- requester scheduling `SessionId` is not target admission `SessionId`;
- target admission `SessionId` is generated only by the existing QH-selected source;
- expected-device PRWM authentication request ID is generated only by its independent QH-selected source;
- verifier time remains the existing fallible function pointer and is not sampled during construction;
- capability authorization remains owned below request construction by the existing authorized request/dispatcher chain.

A status snapshot cannot mint, replace or validate any of those authorities.

## 13. QH construction order remains unchanged

QI does not modify the QH helper.

The existing exact order remains:

1. target admission SessionId source;
2. authentication request-ID source;
3. bind fallible verifier-time function pointer without sampling;
4. consume scheduling grant exactly once;
5. take target expected DeviceId only from the grant;
6. construct exactly one expected request with the exact caller-supplied dispatcher.

The future concrete producer may create the dispatcher immediately before entering this helper; QI does not authorize reordering inside QH or opening the grant earlier.

## 14. Construction failure remains terminal

If either QH identifier source fails after a concrete dispatcher has been created:

- the sealed scheduling grant remains unopened and is terminally disposed by the existing QH path;
- requester correlation and exact acknowledgement result remain in the existing `ConstructionFailed` receipt;
- the concrete dispatcher is dropped normally;
- no snapshot refresh occurs;
- no second dispatcher is constructed for retry;
- no alternate identifier, grant remint/replay/refund/rollback or second construction attempt is authorized.

## 15. Preserved channel law

QI does not create or select a new channel design.

The eventual expected-request handoff remains exactly:

- one bounded Tokio MPSC channel;
- capacity exactly `1`;
- exactly one higher-owned production sender;
- receiver create-once / move-once;
- enqueue only by `sender.send(request).await`;
- full means asynchronous backpressure;
- no sender clone;
- no `try_send`;
- no `blocking_send`;
- no callback `block_on`;
- no alternate/unbounded/retry queue;
- no second producer future;
- no hidden/detached producer task.

QI materializes none of those operations.

## 16. `Constructed` / `Enqueued` distinction remains exact

A dispatcher being constructed does not mean a request exists.

A QH `Constructed(...)` result means only one local typed request exists.

It does not mean:

- sender accepted the request;
- receiver observed the request;
- authentication succeeded;
- admission succeeded;
- worker insertion succeeded;
- capability authorization succeeded;
- acknowledgement succeeded;
- endpoint/reachability succeeded.

Only a later successful `sender.send(request).await` may map to existing disposition `Enqueued`.

A later failed send due receiver closure may map to existing `ChannelClosed`.

QI does not materialize either mapping.

## 17. Selected future source ceiling

The immediate future source materialization ceiling is exactly one Rust path:

`crates/prw-agent/src/linux_bootstrap.rs`

A future QJ source materialization may add only the dormant dispatcher-source/factory custody selected above plus focused same-file type/shape tests if practical.

It must not modify:

- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`;
- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`;
- `main.rs`;
- Cargo manifests or lockfile;
- workflows;
- Android sources;
- packaging/deployment files.

If correct source materialization requires a second Rust path, QJ must STOP and return to a fresh selection checkpoint.

## 18. Future source materialization exclusions

The selected one-file future materialization must not:

- alter `LinuxAgentProductionRemoteCapabilityDispatcher` dispatch behavior;
- widen the dispatcher or factory to crate-external public API;
- call `LocalAgentStatusSnapshot::current(...)` for the remote dispatcher source;
- introduce a dynamic status provider;
- construct expected requests;
- call the QH helper;
- classify live requester completions;
- inspect/consume/dispose a scheduling grant;
- create expected-request channel/sender/receiver;
- send a request;
- produce `Enqueued` or `ChannelClosed` receipts;
- specialize/invoke the generic cooperative producer path;
- wire the receiver to QF;
- mutate the QF higher owner;
- migrate a process/executable caller;
- invoke `run()` with a real remote operation;
- activate listener/readiness/runtime/network behavior;
- modify service/systemd/package configuration;
- read or mutate credentials, certificates, keys or trust;
- mutate RBAC, DB, schema, control plane or authentication policy;
- modify repository settings/rulesets/permissions;
- merge, deploy, restart or recover production.

## 19. Later separately gated dependencies after factory materialization

Even after the one-file dispatcher-source/factory materialization closes, the following remain separate:

1. actual capacity-one expected-request channel construction and sole sender custody;
2. concrete async producer composition that receives a live eligible continuation, obtains one concrete dispatcher, invokes QH construction and performs `sender.send(request).await`;
3. exact `Enqueued` / `ChannelClosed` receipt composition;
4. specialization/invocation of the existing generic producer path with the concrete receipt;
5. receiver-to-QF production wiring;
6. requester/rendezvous higher-owner join;
7. process/runtime caller migration;
8. executable caller;
9. listener/readiness/process/runtime/network activation;
10. deployment.

No ordering among independent later gates is invented beyond the fact that actual send requires an actual sender/channel and an actually constructed request.

## 20. Documentation-only scope of QI

QI itself changes exactly one contract path and no Rust/source/runtime path.

QI does not materialize the selected factory custody.

QI does not create an expected-request channel, dispatcher instance, expected request, producer closure or executable caller.

QI therefore changes no production behavior.

## 21. Closure target

After exact-final-head CI and immutable evidence publication, QI may be classified:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

only for the exact final QI head.

Keep the QI PR draft/open/unmerged.

STOP after C03e-QI closure.

Do not create the source-materialization successor inside QI closure.