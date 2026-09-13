# C03e-SC — fallible handoff receipt higher-observation projection source-seam selection

Status: `SELECTION — VALIDATION/EVIDENCE PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_HANDOFF_RECEIPT_HIGHER_OBSERVATION_PROJECTION_SOURCE_SEAM_SELECTION`

Selected future boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_HANDOFF_RECEIPT_HIGHER_OBSERVATION_PROJECTION_SOURCE_MATERIALIZATION`

This checkpoint is documentation-only. It selects one future dormant source boundary and does not materialize Rust/source/runtime behavior.

## Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-SB:

- predecessor head: `6dd28de3755ff3dd23bae8b50991d4205591a7df`;
- predecessor tree: `0b4c1746b0262e77d5334d68c5c96aaaea1cf857`;
- predecessor PR: `#618`, draft/open/unmerged;
- exact `linux_bootstrap.rs` blob: `316d2dcc01dc9298f85d21f0f2ca5e8cbab4b00f`;
- exact endpoint-owner child blob: `eac44ccdf999a1270b056eb88330ee50b6ff97af`;
- exact parent `remote_session_capability_runtime.rs` blob: `b79f6dfb33d9c29a229a581061cb3e30dd5a8d77`;
- exact higher-owner custody blob: `093cff1e4643f995f0cdc5e337ecfc3bbc2ec582`.

Integrated `main` was freshly re-read immediately before this selection write and remained:

- head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

Fresh branch and all-state PR searches returned no existing `C03e-SC` successor before mutation.

## Exact-current source finding

The exact SB endpoint child already contains the private fallible expected-device producer stack:

- private `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt`;
- private `RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceiptOutcome`;
- private verifier-time function-pointer alias;
- private C03e-RZ endpoint-owner specialization
  `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_expected_device_admission_producer(...)`;
- private RX live producer;
- private RT shutdown-suppression mapper;
- exact generic RN producer forwarding below RZ.

RZ receives a caller-supplied observer whose exact bound is the private raw receipt:

`O: FnMut(RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffReceipt)`.

The raw receipt is intentionally neither `Copy` nor `Clone`. It owns requester `DeviceId` correlation plus one private terminal outcome. Its ineligible arm preserves exact fallible requester scheduling-worker completion provenance by value. Its eligible-terminal arm preserves exact requester acknowledgement result plus one private bounded handoff disposition.

The raw receipt must remain boundary-private. Making RZ directly `pub(crate)` would expose private receipt and verifier-time source details through a higher interface and is therefore not selected.

## Existing projection precedent

The same exact child already proves the accepted visibility pattern for higher observation:

- raw terminal payloads remain private;
- a bounded projection enum is `pub(crate)` in the child;
- a crate-private parent re-export makes only the bounded family available to higher callers;
- an adapter performs the projection locally and forwards only the bounded projection.

Existing `RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection` and `RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection` are precedent only. C03e-SC does not mutate or repurpose either family.

## Selected future source ceiling

A separately gated C03e-SD materialization may change at most these two Rust paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`
   - required predecessor blob: `eac44ccdf999a1270b056eb88330ee50b6ff97af`;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - required predecessor blob: `b79f6dfb33d9c29a229a581061cb3e30dd5a8d77`.

The parent path may be used only for a crate-private re-export of the newly bounded projection family. No higher-owner source path is selected for mutation in C03e-SD.

If correct source materialization requires a third Rust path, higher-owner caller mutation, channel integration, visibility widening of raw receipt types, or runtime behavior, C03e-SD must STOP and return to selection.

## Selected bounded observation law

The future child may add only authority-free bounded projection types sufficient to represent higher-observable receipt semantics without carrying raw private payloads.

The selected observation keeps requester correlation separate from outcome:

`FnMut(DeviceId, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection)`

The projected `DeviceId` is the exact requester correlation already stored in the private receipt. It is not target expected-device identity, transport identity, scheduling identity or authorization authority.

The bounded outcome must distinguish only terminal semantic families already present in exact-current source. The selected family is conceptually:

- `Cancelled`;
- `VerifierTimeFailure`;
- `IngressFailure`;
- `RequesterResponseFailure`;
- `SchedulingDerivationFailure { acknowledgement: ... }`;
- `AbnormalTaskCompletion`;
- `EligibleTerminal { acknowledgement: ..., disposition: ... }`.

Acknowledgement observation is bounded to exactly:

- `Succeeded`;
- `Failed`.

Eligible handoff disposition observation is bounded to exactly the four existing private zero-data terminal states:

- `Enqueued`;
- `ConstructionFailed`;
- `ChannelClosed`;
- `SuppressedOnShutdown`.

No raw acknowledgement error payload is selected for higher exposure. No raw ingress, verifier-time, requester-response, scheduling-derivation or join error payload is selected for higher exposure.

The future projection may use rustfmt-/Clippy-compatible names different from these conceptual names only if the semantic partition above remains exact.

## Projection mapping law

A future C03e-SD adapter must consume one private receipt locally and project it synchronously exactly once.

For an ineligible receipt, the mapper must preserve the following semantic distinctions without exposing raw payloads:

1. requester worker cancellation -> `Cancelled`;
2. fallible verifier-time source failure -> `VerifierTimeFailure`;
3. other fallible post-auth ingress failure -> `IngressFailure`;
4. requester terminal-response failure -> `RequesterResponseFailure`;
5. scheduling-terminal derivation failure -> `SchedulingDerivationFailure`, preserving only bounded acknowledgement success/failure;
6. abnormal spawned-worker completion -> `AbnormalTaskCompletion`.

The exact-current RR classifier is the authority that only `SchedulingTerminal + Ok(grant)` is eligible for continuation. Therefore an ineligible scheduling-terminal receipt is expected to contain scheduling derivation failure, not an eligible grant. C03e-SD must prove this invariant from exact source before materialization; it must not invent a panic, grant-remint, replay, refund or rollback policy merely to handle an unproven impossible state.

For an eligible-terminal receipt, the mapper must preserve only:

- bounded requester acknowledgement success/failure;
- exact bounded handoff disposition family.

The mapper must not return, retain, reconstruct or re-export any authority-bearing payload.

## Selected future adapter law

A future same-child crate-visible wrapper may adapt the private RZ specialization for bounded higher observation, but only if an exact-head C03e-SD audit proves it can do so within the two-path ceiling.

The wrapper must:

- invoke the existing private RZ specialization exactly once;
- keep raw receipt observation entirely inside the endpoint child;
- synchronously map each raw receipt exactly once into requester `DeviceId` plus the bounded projection;
- invoke the caller-supplied bounded observer exactly once per raw receipt;
- forward all non-observation endpoint/producer inputs unchanged;
- preserve the existing caller-borrowed dispatcher factory;
- preserve the existing caller-borrowed typed sender;
- preserve the existing expected-request receiver transfer semantics;
- preserve the exact RT shutdown-suppression mapper through RZ;
- return the existing `Result<(), RemoteSessionPersistentCollectionConfigError>` unchanged.

The wrapper signature must not expose the private raw receipt type, private receipt outcome, private verifier-time source alias, raw requester lifecycle stop/error families, raw acknowledgement error or scheduling grant. If the function-pointer verifier-time type must appear at the higher interface, it may be spelled using its existing public underlying type rather than widening the private alias.

## Authority and identity preservation

C03e-SC preserves these exact separations:

- requester `DeviceId` remains requester correlation only;
- target expected `DeviceId` remains sourced only from the already-consumed eligible scheduling grant during request construction;
- requester scheduling `SessionId` remains distinct from target admission `SessionId`;
- target admission `SessionId` and expected-device authentication request ID remain independently sourced by their existing selected generators;
- verifier-time provenance remains the existing fallible PRWA verifier source;
- capability authority remains separate from identity, transport and scheduling authority;
- queue acceptance is not authentication, admission, authorization, endpoint success or readiness.

The observation projection owns no scheduling grant, request, dispatcher, sender, receiver, endpoint, transport, capability authority, verifier-time provider, retry token, continuation, stream or task handle.

## Existing channel and producer laws remain unchanged

Exact SB higher-owner source already contains dormant capacity-one channel custody:

- exactly one Tokio MPSC channel;
- capacity exactly `1`;
- exactly one retained sender and one receiver;
- no sender clone;
- consuming `into_parts(self)` ownership split.

C03e-SC does not construct, split or transfer that channel.

The existing RX producer remains the sole selected live send composition:

- at most one request;
- exactly one borrowed `sender.send(request).await` for constructed custody;
- full channel means ordinary asynchronous backpressure;
- receiver closure maps to `ChannelClosed`;
- receiver closure is not supervisor shutdown.

No `try_send`, `blocking_send`, callback `block_on`, timeout escape, detached producer task, alternate queue, retry queue, spare sender or second producer future is selected.

## Separately gated after C03e-SC

C03e-SC does not select or authorize:

- source materialization of this projection itself;
- mutation of `production_durable_capability_higher_owner_custody.rs`;
- channel construction or `into_parts()` invocation in a production caller;
- sender/receiver ownership transfer;
- dispatcher-factory invocation by a higher owner;
- actual RZ invocation from a higher owner;
- higher-owner/requester integration;
- process/runtime caller migration;
- executable invocation;
- listener/readiness/network activation;
- target dialing;
- status refresh or host-state inference;
- database/schema/control-plane mutation;
- authentication cutover;
- dependency/manifest/lockfile/workflow/Android-source mutation;
- package/service/systemd mutation;
- repository configuration mutation;
- merge, deployment, restart or recovery activation.

## Validation and evidence requirements for this selection checkpoint

C03e-SC is valid only if final GitHub topology proves:

- direct exact SB -> SC ancestry;
- ahead `1`, behind `0`;
- merge base exact SB head;
- exactly one changed documentation path, this contract;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/packaging/deployment changes.

PASS claims must bind only to the exact final SC head. `SKIPPED` is not PASS. No SB or historical workflow result may be inherited as SC authority.

Immutable evidence must be published exactly once into the canonical PRW Drive evidence folder after exact-final-head validation. The immutable audit remains frozen after publication; the PR body becomes the post-publication closure binding.

## Explicit non-actions / STOP

This selection checkpoint performs no Rust/source/runtime mutation. It does not create projection Rust types, widen raw receipt visibility, invoke RZ, construct/split a channel, clone/move a sender, transfer a receiver, construct a dispatcher, generate identifiers, sample verifier time, construct/send a request, interpret a raw receipt at runtime, spawn a task, activate a listener, publish readiness, deploy, merge, convert the PR ready-for-review, close the PR, delete a branch, rewrite history or mutate repository configuration.

Keep the C03e-SC PR draft/open/unmerged.

After evidence closure: `STOP`. A fresh exact-head audit is required before any C03e-SD source materialization.