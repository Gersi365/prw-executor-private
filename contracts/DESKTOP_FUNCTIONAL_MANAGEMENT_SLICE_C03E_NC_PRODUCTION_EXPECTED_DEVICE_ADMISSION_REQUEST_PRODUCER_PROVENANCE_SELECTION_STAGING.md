# C03e-NC — Production Expected-Device Admission Request Producer Provenance Selection

Status: `SELECTION / STAGING`
Date: `2026-09-08`

Gate on successful closure:
`C03E_NC_PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_PRODUCER_PROVENANCE_SELECTED`

## 1. Decision and scope

C03e-NC performs the documentation-only fresh exact-head producer-provenance assessment required by closed C03e-NB before any production construction or caller propagation of the newly materialized status-only remote capability dispatcher.

Selection result:

`NO_PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_PRODUCER_PROVEN / STATUS_ONLY_DISPATCHER_CALLER_PROPAGATION_BLOCKED_ON_EXPECTED_REQUEST_PRODUCTION_SEAM / RUNTIME_ACTIVATION_DEFERRED`

C03e-NB materialized the dormant crate-private `LinuxAgentProductionRemoteCapabilityDispatcher` in `crates/prw-agent/src/linux_bootstrap.rs`. The adapter is validated and owns only the existing typed production status snapshot, but NB intentionally did not construct it at an executable caller or inject it into a production admission request.

The exact NB source audit shows that the remote admission/runtime chain receives `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` through already-supplied receiver/custody inputs. No audited production source constructs the request, creates its channel/sender, or proves ownership of all request fields required to insert the NB dispatcher.

NC therefore does not invent such a producer. It records the exact missing prerequisite and keeps production dispatcher caller propagation blocked until a later separately bounded provenance checkpoint proves or selects a real construction source.

NC changes documentation only. It does not change Rust/source, Cargo/lockfile, workflows, provider construction, expected-request wiring, channels, authentication, policy, registry semantics, runtime startup, listener activation, `run()`, `main.rs`, deployment, merge or production state.

## 2. Exact predecessor

Repository: `Gersi365/prw-executor-private`

Predecessor: `C03e-NB`

Branch:
`phase-152-c03e-nb-production-remote-status-only-capability-dispatcher-materialization`

Exact NB head:
`285b156c766ee2ab458ae6e6594e62b50526ef77`

Exact NB tree:
`ba5c3cb828246dcd2193568977bc02fa14dea3e1`

Exact NB changed source path:
`crates/prw-agent/src/linux_bootstrap.rs`

Exact NB final target blob:
`709fdbd749fdd63886701fdcc86f32b07110d19e`

NB PR: `#490`

NB closure:
`MATERIALIZED — VALIDATED — EVIDENCE_RECORDED — CLOSED`

NB explicitly left runtime caller migration, provider construction, runtime activation and deployment at zero and required a fresh exact-head audit before selecting production adapter construction/caller propagation.

## 3. Materialized dispatcher boundary retained from NB

The exact NB `linux_bootstrap.rs` contains the dormant crate-private:

`LinuxAgentProductionRemoteCapabilityDispatcher`

It owns exactly one:

`LocalAgentStatusSnapshot`

Its `CapabilityDispatcher` implementation receives only an already-authorized `AuthorizedCapabilityRequest` and applies the NB-selected command projection:

- `BridgeCommand::AgentStatus` returns `encode_status_snapshot(self.status_snapshot).to_vec()`;
- file commands fail closed with `UnsupportedProviderFamily`;
- transfer commands fail closed with `UnsupportedProviderFamily`;
- terminal commands fail closed with `UnsupportedProviderFamily`;
- forwarding commands fail closed with `UnsupportedProviderFamily`.

The adapter owns no request identifier, session identifier, registry, policy evaluator, TLS material, timing source, channel, provider, endpoint or runtime lifecycle authority.

NC does not modify this adapter.

## 4. Exact expected-device admission request ownership shape

The existing remote-session executor runtime defines:

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>`

The request is not merely a dispatcher carrier. It owns the complete expected admission package required by the authenticated worker path, including expected device/transport and admission material, request/session correlation and timing inputs, plus the generic dispatcher `D`.

The worker/runtime chain obtains this request from an `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>` and moves its dispatcher into downstream authenticated capability processing only after the existing admission/authentication path succeeds.

Therefore constructing a production `LinuxAgentProductionRemoteCapabilityDispatcher` is insufficient by itself. A production caller must already own or legitimately derive every other request field and the sending side of the expected-request channel.

NC does not infer those authorities from the existence of the dispatcher, endpoint, requester/rendezvous runtime, durable registry, or local production runtime inputs.

## 5. Production endpoint lifecycle is a consumer, not a producer

Exact NB source:
`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

The production endpoint drive accepts:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

as an explicit input.

Both the ordinary production endpoint drive and the durable-capability requester-aware projection forward that receiver into the existing lower remote-session endpoint lifecycle.

This source therefore proves only receiver consumption/propagation. It does not prove:

- creation of the expected-request channel;
- ownership of its sender;
- construction of `RemoteSessionExpectedDeviceAdmissionRequest`;
- generation of admission request IDs/session IDs;
- selection of verifier timing;
- production dispatcher construction at the sender;
- a production upstream caller authorized to assemble those values.

Test-only channel creation is not production provenance.

## 6. Production durable higher-owner custody is also a consumer

Exact NB source:
`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

The durable higher-owner layer composes already-existing durable registry, capability authority, reachability/provider/executor and remote-session higher-owner custody values.

The expected-request receiver is supplied into that ownership graph; the production facade does not establish a new sending side or construct expected admission requests.

Observed `mpsc::channel()` construction in this source belongs to tests/synthetic fixtures, not the production composition path.

Consequently this layer cannot be treated as the missing production producer merely because it is the highest currently audited custody owner.

## 7. Durable registry bootstrap does not create admission requests

Exact NB source:
`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`
Blob:
`6ad990bf3b8e6536351e06d3b939370ed73c887e`

This source performs bounded production durable-registry bootstrap and can derive:

- the semantic durable registry store;
- the dormant production durable capability authority;
- one current same-device `PeerConnectivityIdentity` from authoritative registry state.

It explicitly performs no requester/rendezvous population, caller creation, runtime task creation or endpoint/runtime activation.

A current peer identity is necessary evidence for remote admission, but it is not a complete `RemoteSessionExpectedDeviceAdmissionRequest` and does not grant authority to manufacture the remaining request fields.

## 8. Durable registry runtime custody does not create admission requests

Exact NB source:
`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`
Blob:
`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

This custody can:

- resolve one current `PeerConnectivityIdentity` for a supplied logical `DeviceId`;
- retain the production durable capability authority;
- authorize one already-read post-auth capability transaction through the existing durable bridge.

It does not read an expected-request channel, create an expected request, select an application-session admission package, construct the NB dispatcher, or own an upstream admission producer.

A successful durable-registry lookup or authorization result is not a substitute for expected-request construction provenance.

## 9. Reachability production layers do not create admission requests

Exact NB source:
`crates/prw-agent/src/production_reachability_owner_composition.rs`
Blob:
`6a338b43995ecc069383e8aee63d7b53a35bc6ff`

This source recovers production reachability-owner custody from an already-narrowed durable executor and one exact peer. It owns no authenticated expected-request producer.

Exact NB source:
`crates/prw-agent/src/production_reachability_runtime_custody.rs`
Blob:
`ffcddc0253de2b5430be798061ddad8e920a07ac`

This source retains live reachability authority and recovered durable owner custody, and may bind an endpoint only when separately invoked. It owns no expected-request sender or request constructor.

Exact NB source:
`crates/prw-agent/src/production_reachability_custody_bootstrap.rs`
Blob:
`ba1e9bb318a4d64206eb745ccb33a00d587f87a3`

This source joins systemd credential custody to reachability bootstrap for one logical peer identity. It does not wire startup/readiness, activate requester/rendezvous behavior, or construct authenticated expected requests.

Reachability authority therefore cannot be promoted into expected-request producer authority without a separately proven composition law.

## 10. Requester/rendezvous sources do not create authenticated capability admission requests

Exact NB sources audited:

- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`.

These sources retain requester/rendezvous authority/runtime ownership and package bounded start intent. They do not establish the production sender for `RemoteSessionExpectedDeviceAdmissionRequest`, do not construct the complete authenticated admission request, and do not own the NB dispatcher.

Requester/rendezvous candidate authority is not interchangeable with authenticated capability admission authority.

NC therefore rejects using requester/rendezvous start intent as an implicit expected-request constructor.

## 11. Remote-session runtime is downstream consumption

Exact NB sources audited beneath:

`crates/prw-agent/src/remote_session_capability_runtime/`

including the executor runtime and recoverable/repeated requester-aware admission worker layers.

The typed expected request is received, validated/admitted, and eventually consumed by the authenticated worker path. The generic bound remains:

`D: CapabilityDispatcher + Send + 'static`.

This proves that the NB dispatcher has the correct downstream ownership shape. It does not prove where production code creates the upstream request.

Synthetic test constructors, test dispatchers, test channels and fixture timing functions are explicitly not production provenance.

## 12. Production status snapshot is available but is not the missing producer

The existing production Linux bootstrap already constructs:

`LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)`

and NB proved that the snapshot can be owned by `LinuxAgentProductionRemoteCapabilityDispatcher` without borrowed authority.

That closes dispatcher construction from a status-value perspective only.

It does not establish:

- which production caller should create the expected admission request;
- when that request should be created;
- how expected device/transport identity is selected at that boundary;
- who owns sender lifetime and channel closure semantics;
- how request/session identifiers are produced;
- how admission timing/timestamp authority is selected;
- how TLS/application-session admission material is populated;
- whether one dispatcher is constructed per request, per expected device, or another lifecycle.

NC refuses to answer these questions by convenience.

## 13. Producer-provenance matrix

| Audited production layer | Proven output/custody | Expected-request producer proven? |
|---|---|---|
| Linux production bootstrap | typed local runtime/status inputs; remote operation composition | No |
| Durable registry bootstrap | semantic store, current peer, durable capability authority | No |
| Durable registry runtime custody | current peer lookup and post-auth authorization | No |
| Reachability owner composition | recovered production reachability owner custody | No |
| Reachability runtime custody | joint authority/owner custody; endpoint bind seam | No |
| Production endpoint lifecycle | consumes/forwards expected-request receiver | No |
| Durable capability higher-owner custody | retains already-supplied expected-request receiver/custody | No |
| Requester/rendezvous runtime/start intent | requester/candidate authority and bounded intent | No |
| Remote-session admission workers | consume typed expected requests | No |
| Test/synthetic helpers | construct channels/requests for tests | Not production provenance |

The matrix establishes a missing source boundary, not permission to create one arbitrarily.

## 14. Rejected implicit producer choices

NC explicitly rejects selecting any of the following as a production producer without separate provenance:

- `main.rs` merely because it is an executable root;
- `run()` merely because it is a process-level function;
- Linux local runtime inputs merely because they contain the status snapshot;
- endpoint lifecycle merely because it consumes the receiver;
- durable higher-owner custody merely because it retains the receiver;
- requester/rendezvous start intent merely because it precedes remote activity;
- current durable-registry peer lookup merely because it supplies current identity;
- listener accept or authenticated transport success as request-generation authority;
- a new `mpsc::channel()` introduced for convenience;
- a globally stored sender;
- a test/synthetic request constructor promoted to production;
- request IDs, session IDs or timestamps invented from constants/counters without an existing authority law;
- permissive default TLS/session material;
- one status dispatcher reused through an unproven shared mutable/global lifetime;
- construction that bypasses the existing Phase 143 authorization path.

## 15. Security and authority invariants

The canonical remote authorization order remains unchanged:

valid remote transport/request framing -> authenticated session/admission -> current registry and transport validation -> typed capability authorization -> dispatcher.

The expected-device request is upstream admission material. Its construction must not itself grant a remote capability or bypass later current-state authorization.

The following values are not independent grants:

- logical `DeviceId`;
- transport identity alone;
- requester/rendezvous ownership;
- candidate publication state;
- endpoint bind success;
- socket accept success;
- local UID;
- status snapshot;
- dispatcher existence;
- request/session identifier;
- durable registry object existence.

NC adds no alternate identity model, policy path, role inference, fallback authority or generic command surface.

## 16. Exact missing prerequisite

The exact missing prerequisite after NB is:

`PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_PRODUCER_PROVENANCE`

A later separately bounded checkpoint must identify or select a real production source that can legitimately own/derive the complete inputs required to construct and send `RemoteSessionExpectedDeviceAdmissionRequest<D, T>`.

That checkpoint must prove at least:

1. the authoritative upstream caller/lifecycle boundary;
2. sender/channel ownership and shutdown semantics;
3. expected logical device provenance;
4. current transport/TLS admission-material provenance;
5. application-session/request correlation provenance;
6. verifier-owned timing provenance;
7. how the NB status-only dispatcher is constructed by value at that boundary;
8. that the request still flows through the unchanged authenticated admission and Phase 143 authorization chain;
9. no test/synthetic/fallback producer is promoted to production;
10. no listener/runtime activation occurs merely to prove construction provenance.

If no existing source owns those values together, that later checkpoint must record the precise missing composition prerequisite rather than materializing a new runtime producer opportunistically.

## 17. Immediate successor ceiling after NC closure

NC does not authorize expected-request producer Rust materialization.

After NC closure, the next separately gated checkpoint may perform only a bounded producer-provenance/source-seam selection answering the prerequisite in section 16.

It may not yet:

- migrate production callers;
- create a new channel in executable startup;
- inject `LinuxAgentProductionRemoteCapabilityDispatcher` into a live request;
- activate a listener;
- start remote admission workers;
- alter systemd/package/deployment state;
- merge any PR;
- widen public APIs merely to bridge ownership;
- change authentication, registry, policy or cryptographic semantics.

A source mutation is authorized only after an exact predecessor explicitly selects its path and semantic ceiling.

## 18. Exact NC repository scope

Only this new documentation path may differ from exact NB:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_NC_PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_PRODUCER_PROVENANCE_SELECTION_STAGING.md`

No Rust/source, Cargo/lockfile, workflow, package/systemd, registry/provider implementation, endpoint, listener, authentication, policy, networking, `main.rs`, deployment or production-state path belongs to NC.

Runtime activation remains `0%`.

## 19. Validation and durable closure

NC closes only after:

- exact NB predecessor remains `285b156c766ee2ab458ae6e6594e62b50526ef77`;
- NB -> NC is exactly one added documentation path with no source deletion or mutation;
- exact-final-head Rust validation reaches terminal success;
- every automatically triggered workflow is reported with its actual terminal result; `SKIPPED` is not `PASS`;
- any Android workflow actually triggered for the exact head is reported by its real terminal conclusion;
- one immutable Markdown audit is written to canonical Drive evidence parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT` with exact-title presearch zero, raw readback byte/hash equality, title/parent/MIME verification and postsearch exactly one;
- only after evidence acceptance may the PR body record `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- the PR remains draft/open/unmerged;
- final GitHub/Drive race checks confirm NB, NC, `main` and successor state.

No runtime activation or production caller migration is authorized by NC closure.

## 20. Closure meaning

C03e-NC closes only the fresh post-NB expected-device admission request producer-provenance question.

It records:

`NO_PRODUCTION_EXPECTED_DEVICE_ADMISSION_REQUEST_PRODUCER_PROVEN / STATUS_ONLY_DISPATCHER_CALLER_PROPAGATION_BLOCKED_ON_EXPECTED_REQUEST_PRODUCTION_SEAM / RUNTIME_ACTIVATION_DEFERRED`

The dormant NB dispatcher remains validated but unused by a production expected-request producer.

The next step is provenance/source-seam selection for that missing producer, not runtime activation and not ad hoc caller wiring.
