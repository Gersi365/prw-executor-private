# Phase 152 C03e-EI — Candidate-Publication Requester/Rendezvous Post-Authentication Target-Intent Caller Ingress Selection

Status: `STAGING_SELECTION`

Target gate: `C03E_EI_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_AUTH_TARGET_INTENT_CALLER_INGRESS_SELECTED`

## 1. Purpose

C03e-EI selects, but does not source-materialize, the narrow caller boundary that may consume one already-typed `RequesterRendezvousTargetIntent` only after an exact `AuthenticatedRemoteSessionRuntimeOwner` exists and delegate that typed target to the already-materialized C03e-EH authenticated-session adapter.

This checkpoint does not select byte-level wire ingress, generic `BridgeCommand` integration, requester/rendezvous authorization, provider mutation, C03e-DV invocation, worker-lifecycle widening, bootstrap activation, networking, deployment, or merge.

The selected boundary exists only to give a future separately gated requester-specific control transaction one explicit Agent-internal caller seam that remains outside the principal-agnostic capability lane.

## 2. Exact predecessor

C03e-EI is rooted exactly at durably closed C03e-EH:

- predecessor branch: `phase-152-c03e-eh-candidate-publication-requester-rendezvous-target-intent-authenticated-session-adaptation-source-materialization-staging`
- predecessor head: `012854a068514a727ba5327c92724e725d0c5697`
- predecessor tree: `ba1560bde55ebe9e4c0e087832658b6247898eb6`
- predecessor PR: `#258`, draft/open/unmerged, `Status: CLOSED`
- predecessor closure classification: `SOURCE_MATERIALIZATION_FULL_PASS`
- predecessor target gate: `C03E_EH_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_AUTHENTICATED_SESSION_ADAPTATION_SOURCE_MATERIALIZED`
- post-EH rolling evidence: `1097038` bytes
- post-EH rolling SHA-256: `36a785693d27e49b66e7a95a90bd8aba7ceb95dcf16755b59646178f6b38e6bd`

No earlier checkpoint is reopened.

## 3. Fresh exact-head topology

### 3.1 C03e-EH typed adaptation already exists

At the exact C03e-EH head, `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` has blob:

`4bf8c5589a5523e4bd5ea97e960b1f7f921d5e7d`

It contains the crate-private C03e-EH helper with semantics equivalent to:

```rust
pub(crate) fn requester_rendezvous_start_intent_from_target_intent(
    &self,
    target_intent: RequesterRendezvousTargetIntent,
) -> RequesterRendezvousStartIntent {
    self.requester_rendezvous_start_intent(target_intent.into_target_device_id())
}
```

Therefore C03e-EI does not select another requester source, another target source, or another start-intent representation.

### 3.2 Existing remote worker admission is generic capability custody only

At the exact C03e-EH head, `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` has blob:

`47c41735de3c153cde8794b46479e09da7cfba18`

Its existing public `RemoteSessionWorkerAdmission<D, T>` owns exactly:

```text
AuthenticatedRemoteSessionRuntimeOwner
+ dispatcher D
+ verifier-time provider T
```

It contains no requester/rendezvous target intent and no requester-aware policy or provider authority.

The persistent worker consumes that admission and enters the existing generic capability request worker. C03e-EI therefore does not widen `RemoteSessionWorkerAdmission`, `into_parts()`, persistent-worker collection signatures, or worker-spawn signatures merely to carry requester/rendezvous target intent.

### 3.3 Existing authenticated worker is the principal-agnostic capability lane

The existing `AuthenticatedRemoteSessionRuntimeOwner::process_one_capability_request(...)` accepts one control stream, receives one generic capability frame, authorizes it through `CapabilityBridge`, dispatches an already-authorized generic request, and sends one generic capability response.

The existing serial loop and worker are built around that transaction.

C03e-EI does not add requester/rendezvous target intent to that transaction, does not add a `BridgeCommand` variant, and does not treat generic policy `P` as requester-aware rendezvous policy.

### 3.4 Process-operation requester/rendezvous custody is not target ingress

`crates/prw-agent/src/linux_bootstrap.rs` already contains the crate-private requester/rendezvous process-operation wrapper selected/materialized by C03e-EA/EB/EC/ED. It retains one already-constructed requester-aware policy source and one already-constructed requester/rendezvous runtime owner beside the existing public remote-process inputs.

That wrapper is process-operation custody, not a target producer and not an authenticated-session caller. C03e-EI does not reinterpret it as target ingress and does not activate its retained authorities.

## 4. Missing boundary

After C03e-EH, all of the following typed facts exist separately:

1. exact authenticated requester ownership in `AuthenticatedRemoteSessionRuntimeOwner`;
2. exact caller-nominated target ownership in `RequesterRendezvousTargetIntent`;
3. exact side-effect-free EH adaptation from those two facts to `RequesterRendezvousStartIntent`;
4. separately retained requester-aware policy source and requester/rendezvous runtime owner in process-operation custody;
5. source-materialized C03e-DV authorization/provider composition, still deliberately uncalled.

What does not yet exist is one dedicated requester-specific caller seam for a future control transaction to hand an already-decoded typed target intent to C03e-EH without entering the generic capability bridge or changing worker admission identity semantics.

## 5. Selected C03e-EI boundary

C03e-EI selects a future crate-private, side-effect-free, one-shot Agent-internal caller seam with semantics equivalent to:

```rust
pub(crate) fn adapt_post_auth_requester_rendezvous_target_intent(
    session_owner: &AuthenticatedRemoteSessionRuntimeOwner,
    target_intent: RequesterRendezvousTargetIntent,
) -> RequesterRendezvousStartIntent {
    session_owner.requester_rendezvous_start_intent_from_target_intent(target_intent)
}
```

The exact function/module spelling may change only for readability or lint compliance. The semantic boundary is fixed.

The selected seam may be materialized in a dedicated crate-private requester/rendezvous control-ingress module under the existing Agent remote-session runtime surface, so future wire/control work has one narrow caller target without modifying the generic `BridgeCommand` lane.

## 6. Required ownership semantics

A future source materialization of the C03e-EI selection must:

1. borrow one already-authenticated `AuthenticatedRemoteSessionRuntimeOwner`;
2. consume exactly one already-constructed `RequesterRendezvousTargetIntent` by value;
3. call the existing C03e-EH adaptation exactly once;
4. return only the existing `RequesterRendezvousStartIntent`;
5. perform no I/O, stream acceptance, decoding, registry read, policy evaluation, provider mutation, synchronization, task spawn, cancellation, readiness change, or lifecycle transition;
6. remain crate-private;
7. add no requester identity argument and no alternate target identity argument.

No second envelope is required unless source-level visibility/lint mechanics prove one necessary. The already-typed `RequesterRendezvousTargetIntent` remains the sole target carrier.

## 7. Requester identity invariant

Requester authorization identity remains exactly the logical `DeviceId` and workspace/user binding retained by the exact authenticated application session owned by `AuthenticatedRemoteSessionRuntimeOwner`.

The selected EI caller seam must not accept or derive requester identity from:

- the target `DeviceId`;
- `TransportIdentity`;
- `SessionId`;
- request/correlation IDs;
- control-stream identity;
- remote/local endpoint or IP address;
- candidate address;
- candidate-publication publisher identity;
- repeated-admission `expected_device_id` as a separate requester assertion;
- registry role or enumeration;
- policy output;
- provider state;
- environment/configuration/default state.

Requester identity is not a field of `RequesterRendezvousTargetIntent`.

## 8. Target identity invariant

The logical rendezvous target remains exactly the explicit caller-nominated `DeviceId` already owned by the consumed `RequesterRendezvousTargetIntent`.

The selected EI caller seam must not infer, normalize, replace, default, cache, or cross-fill that target from:

- the requester's authenticated logical `DeviceId`;
- repeated-admission `expected_device_id`;
- authenticated candidate-publication publisher identity;
- `TransportIdentity`;
- `SessionId`;
- request/correlation IDs;
- endpoints/IP addresses;
- candidate state;
- registry roles or device enumeration;
- policy source;
- requester/rendezvous provider state;
- previous requests;
- process globals;
- environment/CLI/files/configuration.

Possession of target intent remains nomination only, not authorization or registration proof.

## 9. Why `RemoteSessionWorkerAdmission` is not selected

`RemoteSessionWorkerAdmission<D, T>` is already a public ownership shape for one authenticated generic-capability worker candidate.

Adding requester/rendezvous target intent to it would:

- widen a public worker/admission signature;
- bind a one-shot requester target nomination to worker admission lifetime without an explicit control transaction;
- risk confusing the admitted/requester `DeviceId` with the separately nominated target;
- alter a closed generic worker topology before requester-specific control semantics are selected.

C03e-EI therefore rejects worker-admission widening.

## 10. Why generic `BridgeCommand` is not selected

Closed C03e-DW/DX/DV deliberately keep requester/rendezvous policy separate from the principal-agnostic generic capability evaluator `P`.

The generic bridge is therefore not a valid authority boundary for requester/rendezvous start merely because it already has frame parsing and dispatch infrastructure.

C03e-EI does not add or select:

- a `BridgeCommand::RequesterRendezvousStart` variant;
- generic `required_capability()` mapping for requester/rendezvous start;
- requester-aware policy through generic `PolicyEvaluator`;
- authorized-request dispatcher routing for requester/rendezvous target intent.

## 11. Relationship to wire/control

C03e-EI selects only the Agent-internal post-authentication caller seam that a future dedicated requester-specific control transaction may call after producing one valid typed `RequesterRendezvousTargetIntent`.

C03e-EI does not select or materialize:

- a new magic value or opcode;
- `DeviceId` byte encoding;
- target-intent frame shape;
- parser/decoder;
- control-stream discriminator or multiplexing rule;
- stream acceptance policy;
- request/response correlation semantics;
- success/error response frame;
- malformed-target classification;
- retry behavior;
- peer-close behavior.

Those remain separately gated.

## 12. Relationship to C03e-DV

C03e-DV remains source-materialized and deliberately uncalled.

The EI-selected caller seam ends after producing the existing `RequesterRendezvousStartIntent` through C03e-EH. It does not borrow:

- `SharedCurrentCapabilityAuthority<P>`;
- `BoundedRequesterRendezvousStartPolicySource`;
- `CandidatePublicationRequesterRendezvousRuntimeOwner`.

It does not invoke registry validation, requester-aware policy evaluation, or provider registration.

A later separately gated caller-composition checkpoint must explicitly select any connection from requester-specific control ingress to C03e-DV.

## 13. Relationship to process-operation custody

C03e-EB/ED custody remains unchanged and inactive for requester/rendezvous execution.

C03e-EI does not:

- move policy source or runtime owner into per-session worker admission;
- clone them per session;
- make the mutable runtime owner process-global;
- add synchronization around the runtime owner;
- construct provider or policy backing;
- alter the existing public remote-process inputs or factory;
- change bootstrap/main assembly.

## 14. Expected source-materialization scope

Only after C03e-EI closes durably, a successor may source-materialize the selected one-shot caller seam.

Expected narrow source scope is:

- one dedicated crate-private Agent runtime/control-ingress module or an equivalently private local seam;
- one module registration if a new private module is used;
- one function borrowing the authenticated owner, consuming typed target intent, and delegating exactly once to C03e-EH;
- side-effect-free signature/ownership tests if required.

No manifest or lockfile change is expected.

The successor must not simultaneously add byte-level wire handling or invoke C03e-DV.

## 15. Explicit exclusions

C03e-EI does not select or materialize:

- C03e-DV invocation;
- requester-aware registry/policy/provider execution;
- requester-policy population or refresh;
- provider construction/capacity/persistence;
- generic `BridgeCommand` requester/rendezvous behavior;
- public `RemoteSessionWorkerAdmission` changes;
- worker collection/channel/signature widening;
- target wire encoding or protocol extension;
- candidate-publication command changes;
- target derivation from publication/admission/transport/session state;
- public process-input widening;
- bootstrap/main assembly;
- listener/readiness/network activation;
- STUN/ICE/TURN behavior;
- deployment;
- restart/recovery;
- merge.

## 16. Dependency anchors

The following exact C03e-EH anchors must remain byte-stable for C03e-EI:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

C03e-EI selects no dependency mutation.

## 17. Closure criteria

C03e-EI may close only if one exact final head proves:

1. exact C03e-EH merge base;
2. ahead only and zero behind;
3. exactly one docs-only changed path: this contract;
4. C03e-EH authenticated-session runtime remains byte-stable;
5. existing target-intent source remains byte-stable;
6. remote-session executor/worker admission source remains byte-stable;
7. Linux requester/rendezvous process-operation custody source remains byte-stable;
8. dependency anchors remain byte-stable;
9. canonical exact-head required CI is terminal clean;
10. SKIPPED workflows remain classified as SKIPPED;
11. immutable Drive audit readback is byte-exact;
12. rolling predecessor is exact post-EH before append;
13. rolling predecessor prefix remains byte-exact after append;
14. EI closure/classification/target-gate markers occur exactly once;
15. PR remains draft/open/unmerged.

## 18. Target gate

C03e-EI targets exactly:

`C03E_EI_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_AUTH_TARGET_INTENT_CALLER_INGRESS_SELECTED`

Passing this gate means only that one dedicated Agent-internal post-authentication caller seam has been selected for later source materialization. It does not mean wire ingress exists, requester/rendezvous authorization runs, provider state mutates, C03e-DV is invoked, or networking/deployment is activated.
