# C03e-OC — Production requester/rendezvous expected-device admission producer owner source-seam decomposition selection

Status: `SELECTION_STAGING`

Boundary:
`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_PRODUCER_OWNER_SOURCE_SEAM_DECOMPOSITION_SELECTION`

Predecessor authority:
- exact closed C03e-OB head: `3ec6110f0faf63ad9f7172d3f117bb7449cdec64`;
- exact closed C03e-OB tree: `62d3eeff9f5732517253e86c87c469ea5e504a42`;
- predecessor PR: `#516`, draft/open/unmerged and explicitly CLOSED;
- this checkpoint is documentation-only.

## 1. Purpose

C03e-OB proved that a complete production `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` producer is not yet materialized. It selected the identifier, correlation, dispatcher, verifier-time and sender/channel laws that any future producer must satisfy, but intentionally stopped before choosing the exact source decomposition.

C03e-OC audits the exact current C03e-OB source graph and selects the smallest source seams that may be materialized later without activating the remote runtime, constructing or sending an expected-device admission request, or collapsing unresolved custody domains into one synthetic factory.

This checkpoint does not implement any source seam.

## 2. Fresh authority and concurrency result

Immediately before this branch was created:
- C03e-OB PR `#516` was re-read as draft/open/unmerged with head `3ec6110f0faf63ad9f7172d3f117bb7449cdec64`;
- the C03e-OB branch was re-read at the same head and tree `62d3eeff9f5732517253e86c87c469ea5e504a42`;
- recent PR chronology still had `#516` as the newest project PR;
- no `phase-152-c03e-oc-*` branch existed;
- no matching C03e-OC producer-owner/source-seam PR existed.

Therefore the `C03e-OC` token is assigned only after direct current absence/concurrency checks. It is not inferred solely from alphabetical naming.

## 3. Exact-current source observations

### 3.1 Executable assembly remains dormant

Exact C03e-OB `crates/prw-agent/src/main.rs` calls only `linux_bootstrap::run()`.

The public `linux_bootstrap::run()` preserves the fixed local Linux Agent profile and does not invoke the dormant remote companion path. Existing `run_with_remote_process_companion(...)` and production/requester-rendezvous companion helpers remain caller-supplied or crate-private dormant seams.

Therefore this selection must not wire `main.rs`, replace `run()`, start a remote companion, bind an endpoint, publish readiness, or alter executable exit policy.

### 3.2 Existing process operation owns the expected-request receiver only

Exact C03e-OB `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` in `crates/prw-agent/src/linux_bootstrap.rs` owns:
- bind address;
- max active workers;
- current-capability authority;
- `SessionAuthenticationService`;
- `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- admission timing;
- completion/rejection/admission-failure callbacks.

Its production population helpers accept and move that receiver. They do not create or retain a matching sender.

`RemoteSessionProcessLifecycleOwner` separately owns only the remote OS-thread and shutdown-controller handoff/finalization. It does not own expected-request sender custody or request-construction inputs.

`production_durable_capability_higher_owner_custody.rs` likewise accepts an already-existing expected-request receiver from its caller and explicitly does not create an expected-request channel.

No exact-current source owner has therefore been proved to own both the receiver lifecycle and a production sender.

### 3.3 Scheduling-aware terminal custody exists below the endpoint owner

Exact C03e-OB deep executor source already exposes:
`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_scheduling(...)`.

Its completion callback receives:
- requester-side authenticated `DeviceId`; and
- exact `Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The method preserves the NY-selected peer/session-owner disposition law and closes/drains the endpoint exactly as the historical durable lifecycle does.

The requester-side callback `DeviceId` remains requester identity and MUST NOT be used as target expected-device identity.

### 3.4 Endpoint lifecycle owner does not yet propagate scheduling-aware completion

Exact C03e-OB `RemoteSessionEndpointLifecycleRuntime` exposes historical, production-durable, requester-aware and bounded completion-projection drives, but no scheduling-aware wrapper.

The existing scheduling-aware executor method has visibility sufficient for a sibling method inside `remote_session_endpoint_lifecycle_runtime.rs` to delegate to it without changing the deep executor source.

The exact scheduling terminal types and one-shot grant remain private within `remote_session_capability_runtime`:
- `RequesterRendezvousProductionDurableSchedulingWorkerStop` is `pub(super)`;
- `ExpectedDeviceSchedulingAuthorityGrant` is `pub(super)`;
- the grant is intentionally neither `Copy` nor `Clone` and owns requester `SessionId` plus target `DeviceId` only.

This privacy is useful authority containment and MUST NOT be widened merely to let `linux_bootstrap.rs` inspect the grant.

### 3.5 Target admission SessionId generation is not an existing core/session primitive

Exact C03e-OB `prw_core::SessionId` is a typed non-empty string wrapper. `SessionId::new(...)` validates; it does not generate freshness or randomness.

Exact `SessionAuthenticationService::begin_session(...)` accepts a caller-supplied `SessionId`, generates only the authentication challenge nonce, and rejects duplicate session IDs already present in pending/authenticated state.

Therefore challenge-nonce generation MUST NOT be treated as target admission SessionId provenance.

C03e-OB remains authoritative that target admission SessionId material is fresh server-local CSPRNG material: exact 32 random bytes -> 64 lowercase hexadecimal ASCII -> typed `SessionId`, with `SessionAuthenticationService` retaining duplicate-session authority.

### 3.6 Authentication request ID remains caller-produced correlation

Exact expected-device real admission takes `authentication_request_id: u64` from the caller and passes it into the existing registry-bound authentication transaction.

Exact authentication transaction consumes that ID for Challenge -> Proof correlation; it does not allocate it.

No exact-current expected-device PRWM request-ID owner is materialized. PRWC correlation values or historical owner concepts MUST NOT be substituted.

The C03e-OB law remains authoritative: one fresh independent non-zero CSPRNG `u64` per expected-device authentication transaction, with no retry/remint after a terminal generation/collision failure for a consumed scheduling grant.

### 3.7 Existing direct CSPRNG dependency means no Cargo widening is required for future ID-source materialization

Exact C03e-OB `crates/prw-agent/Cargo.toml` already directly depends on pinned `aws-lc-rs = 1.18.0` with the selected feature set.

Therefore a later Agent-local CSPRNG source need not add a Cargo dependency or mutate the lockfile solely to obtain cryptographic randomness.

This observation does not select an implementation in C03e-OC.

### 3.8 NB dispatcher implementation exists, but production expected-request construction does not

Exact C03e-OB `LinuxAgentProductionRemoteCapabilityDispatcher` in `linux_bootstrap.rs` is an owned status-only dispatcher over `LocalAgentStatusSnapshot` and satisfies the existing `CapabilityDispatcher` shape.

Its constructor is side-effect-free. Exact current production remote-process population remains generic over caller-supplied `D`; no production expected-request owner currently constructs and transfers this dispatcher into a request.

Provider-backed command families remain unsupported under the current NB adapter. A future expected-request producer MUST NOT silently widen dispatcher/provider semantics.

### 3.9 Request-carried verifier time is structurally incompatible with fail-closed SystemTime failure today

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>` retains `T: FnMut() -> u64`.

The request-carried verifier-time callback is later invoked as an infallible `u64` source. By contrast, a server-local `SystemTime::now().duration_since(UNIX_EPOCH)` conversion is fallible.

No exact-current expected-device production adapter has been proved that can both:
- sample `SystemTime` at each verifier-time call; and
- surface clock conversion failure fail-closed;
without panic, default/zero substitution, saturation, frozen startup time, hidden process abort, or widening an interface.

C03e-OB explicitly forbids treating the current fallible PRWA clock as directly shape-compatible with this `T`.

Therefore verifier-time source materialization remains an independent interface-selection problem. It MUST NOT be hidden inside a supposedly complete producer factory.

### 3.10 Direct send from the current scheduling callback is not yet a proved backpressure-safe seam

The scheduling-aware completion callback is synchronous `FnMut(...) -> ()` and is invoked while the repeated-admission supervisor is being driven on the same private executor.

A future finite-capacity expected-request sender cannot be assumed safe here:
- awaited `mpsc::Sender::send(...)` is not available through the current synchronous callback without adding a new async/runtime handoff;
- `blocking_send(...)` or nested runtime driving on the same runtime is not selected;
- `try_send(...)` would create an explicit `Full` failure that the current callback return type cannot propagate;
- silently dropping, retrying or reminting after `Full`/`Closed` is forbidden.

Accordingly channel creation, sender custody, exact capacity and producer failure propagation MUST remain later until a separately selected synchronous-completion-to-producer result/backpressure seam exists.

No channel-capacity alias to worker count, requester-record capacity or scheduling-consumption capacity is selected by C03e-OC.

## 4. Closed decomposition selection

A single standalone expected-admission producer factory is **not currently proven**.

The source graph MUST be decomposed into separately gated stages in this order:

### Stage A — scheduling-aware endpoint propagation

Materialize only a dormant scheduling-aware sibling on `RemoteSessionEndpointLifecycleRuntime` that:
- consumes the existing endpoint owner exactly once;
- delegates exactly once to the existing deep scheduling-aware durable endpoint lifecycle;
- forwards authority, capability authority, requester policy, requester/rendezvous authority, session authentication, expected-request receiver, admission timing, rejection and admission-failure callbacks unchanged;
- forwards exact scheduling terminal custody only within the existing `remote_session_capability_runtime` privacy boundary;
- preserves supervisor-shutdown, endpoint close and idle-drain semantics;
- performs no grant extraction, request construction, send, channel creation, ID generation, dispatcher construction, timing-source construction, task spawn or runtime activation.

**Smallest immediate future Rust source ceiling:**
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

No other Rust source path is authorized in this first source successor.

The new scheduling-aware sibling MUST remain module-internal enough that private scheduling grant/terminal types are not widened to crate/public visibility merely for convenience.

### Stage B — higher-owner boundary-safe scheduling projection/producer-result seam

After Stage A is independently validated and closed, a new documentation gate must select how a higher owner receives construction eligibility without exposing or reminting the private one-shot scheduling grant.

That gate must also select how producer failure is represented from the currently synchronous completion boundary.

It must not construct/send the request yet unless all later dependencies are independently selected.

### Stage C — expected-request channel owner and backpressure/shutdown law

Only after Stage B proves a result/failure seam may channel creation be selected/materialized.

That future gate must establish:
- one explicit finite non-zero channel capacity source, not an undocumented alias;
- exact channel creation point;
- receiver movement into the existing remote-process input owner;
- one non-cloneable sender owner or an explicitly justified bounded sender-owner set;
- exact `Full`/`Closed` behavior;
- no background retry loop;
- no send retry/remint after a consumed scheduling grant;
- sender drop/receiver close/shutdown relation;
- cancellation-safe termination;
- no hidden task/runtime activation.

C03e-OC intentionally does **not** select capacity or create a channel because the current synchronous callback cannot yet fail closed on sender backpressure.

### Stage D — target admission SessionId and authentication request-ID source owner

A later bounded source stage may materialize independent Agent-local CSPRNG primitives using the already-present direct `aws-lc-rs` dependency.

Required laws remain:
- target admission SessionId: 32 independent random bytes -> 64 lowercase hex ASCII -> `SessionId`;
- auth request ID: independent fresh non-zero random `u64`;
- no shared raw random sample between the two outputs;
- no requester SessionId reuse;
- no PRWC/PRWA/candidate/request correlation reuse;
- no counter/time/hash/default/sentinel fallback;
- no unbounded collision retry;
- failure/collision is terminal for that consumed scheduling grant unless a later gate explicitly selects a bounded retry law without reminting the grant.

No Cargo/lockfile change is selected for this stage.

### Stage E — NB dispatcher construction custody

A later stage must select the exact production-owned `LocalAgentStatusSnapshot` source/lifetime used to construct `LinuxAgentProductionRemoteCapabilityDispatcher` and move that owned dispatcher into exactly one expected request.

No provider-family widening is permitted.

### Stage F — verifier-time interface compatibility

Before any complete request factory is authorized, a documentation gate must resolve the current `FnMut() -> u64` versus fallible call-time `SystemTime` incompatibility.

That gate must reject:
- panic/`expect` as a production clock-failure policy;
- zero/default/saturation;
- startup-frozen time;
- silently ignoring pre-epoch/clock-conversion failure;
- hidden process abort;
- unrelated PRWA context reuse.

If the only correct fail-closed solution requires changing an interface, that interface change must be selected explicitly and separately. C03e-OC does not widen `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` or any downstream verifier interface.

### Stage G — request construction and send

Only after Stages A-F are independently selected/materialized/validated may a later gate authorize:
- consumption of one construction-eligible scheduling grant exactly once;
- target `DeviceId` derivation only from that consumed grant;
- construction of one existing `RemoteSessionExpectedDeviceAdmissionRequest<D, T>`;
- one bounded send under the selected channel law.

Requester callback `DeviceId` remains forbidden as target identity.
Requester scheduling `SessionId` remains forbidden as target admission `SessionId`.
Requester acknowledgement disposition remains orthogonal and cannot revoke/remint the grant.

## 5. Selected classification

`SINGLE_COMPLETE_EXPECTED_ADMISSION_PRODUCER_FACTORY_NOT_CURRENTLY_PROVEN /
SOURCE_GRAPH_MUST_DECOMPOSE_INTO_SEPARATELY_GATED_STAGES /
SCHEDULING_AWARE_ENDPOINT_PROPAGATION_IS_THE_FIRST_SOURCE_PREREQUISITE /
FIRST_SOURCE_CEILING_IS_REMOTE_SESSION_ENDPOINT_LIFECYCLE_RUNTIME_RS_ONLY /
PRIVATE_SCHEDULING_TERMINAL_AND_GRANT_VISIBILITY_MUST_NOT_BE_WIDENED_FOR_CONVENIENCE /
CURRENT_SYNCHRONOUS_SCHEDULING_CALLBACK_CANNOT_YET_PROPAGATE_FINITE_CHANNEL_BACKPRESSURE_FAILURE /
EXPECTED_REQUEST_CHANNEL_CREATION_CAPACITY_SENDER_CUSTODY_AND_SHUTDOWN_REMAIN_LATER /
TARGET_ADMISSION_SESSION_ID_AND_AUTH_REQUEST_ID_REQUIRE_INDEPENDENT_AGENT_LOCAL_CSPRNG_SOURCE_OWNERS /
EXISTING_DIRECT_AWS_LC_RS_DEPENDENCY_MEANS_NO_CARGO_WIDENING_IS_REQUIRED_FOR_ID_RANDOMNESS /
NB_STATUS_ONLY_DISPATCHER_IS_REUSABLE_ONLY_AFTER_EXACT_OWNED_CONSTRUCTION_CUSTODY_IS_SELECTED /
CALL_TIME_SYSTEMTIME_FAIL_CLOSED_PROVIDER_IS_NOT_CURRENTLY_COMPATIBLE_WITH_INFALLIBLE_REQUEST_T /
VERIFIER_TIME_INTERFACE_COMPATIBILITY_REQUIRES_A_SEPARATE_SELECTION /
REQUEST_CONSTRUCTION_AND_SEND_REMAIN_BLOCKED /
NO_RUNTIME_ACTIVATION_SELECTED`

## 6. Immediate future source boundary

The only source successor authorized by this selection is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_SCHEDULING_AWARE_ENDPOINT_LIFECYCLE_PROPAGATION_SOURCE_MATERIALIZATION`

That future source checkpoint must:
1. start from the exact closed C03e-OC head only after a fresh branch/PR absence audit;
2. modify exactly `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`;
3. add only one dormant scheduling-aware endpoint-lifecycle sibling delegating to the existing deep scheduling-aware executor method;
4. preserve private scheduling terminal/grant visibility;
5. preserve all existing historical endpoint methods unchanged;
6. preserve exact shutdown/close/idle-drain semantics;
7. add only bounded tests needed to prove the source shape/forwarding law;
8. make zero changes to `linux_bootstrap.rs`, `main.rs`, higher-owner custody, expected-request channel, request construction, IDs, dispatcher, verifier time, Cargo/lockfile/workflows/Android/packaging/deployment;
9. remain dormant/uninvoked by executable production assembly;
10. STOP after exact-head validation and durable evidence.

A successor token is not selected here solely from alphabetical naming.

## 7. Explicit non-actions

C03e-OC performs no Rust/source/runtime mutation.

It performs no:
- expected-device request construction;
- expected-device request send;
- expected-request channel creation;
- sender retention or clone;
- backpressure implementation;
- target admission SessionId generation;
- authentication request-ID generation;
- NB dispatcher production construction;
- verifier-time provider construction;
- timing-interface widening;
- scheduling grant extraction, clone, copy, reconstruction, remint or replay;
- requester ACK retry/resend;
- scheduling-consumption rollback;
- requester cleanup;
- candidate/reachability continuation;
- target dial;
- listener/bootstrap/readiness/runtime activation;
- `main.rs` wiring;
- Cargo/lockfile/workflow/Android mutation;
- deployment;
- merge;
- branch deletion;
- force push/history rewrite;
- repository configuration/ruleset/permission mutation.

## 8. Validation and evidence requirements

Before closure, this docs-only checkpoint must prove:
- exact predecessor/head ancestry;
- one changed documentation path only;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/packaging/deployment delta;
- exact-head workflow outcomes, with `SKIPPED != PASS`;
- immutable `text/plain` audit evidence in the canonical Drive evidence folder;
- exact evidence byte count and SHA-256 after raw Drive readback;
- exactly one post-publication exact-title evidence match;
- final PR remains draft/open/unmerged;
- no concurrent successor appeared before closure.

After closure: `STOP`.
