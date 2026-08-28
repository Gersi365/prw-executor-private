# Phase 152 C03e-EJ — Candidate-Publication Requester/Rendezvous Post-Authentication Target-Intent Caller Ingress Source Materialization

Status: `STAGED_SOURCE_MATERIALIZATION`

Target gate: `C03E_EJ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_AUTH_TARGET_INTENT_CALLER_INGRESS_SOURCE_MATERIALIZED`

## 1. Purpose

C03e-EJ source-materializes only the one-shot crate-private post-authentication caller ingress seam selected by durably closed C03e-EI.

The materialization must borrow one existing `AuthenticatedRemoteSessionRuntimeOwner`, consume one existing `RequesterRendezvousTargetIntent` by value, delegate exactly once to the existing C03e-EH typed authenticated-session adapter, and return only the existing `RequesterRendezvousStartIntent`.

This checkpoint does not add requester/rendezvous wire handling, generic `BridgeCommand` behavior, C03e-DV invocation, registry/policy/provider execution, worker/admission widening, bootstrap activation, networking, deployment, or merge.

## 2. Exact predecessor

Durably closed predecessor C03e-EI:

- branch: `phase-152-c03e-ei-candidate-publication-requester-rendezvous-post-auth-target-intent-caller-ingress-selection-staging`
- head: `8c1992b4fabc0f59d1c84c1bd5c971fee575507d`
- tree: `68327f9ee48618256328502205e56879eb091bb9`
- PR: `#259`, draft/open/unmerged, `Status: CLOSED`
- closure classification: `CLOSED_POST_AUTH_TARGET_INTENT_CALLER_INGRESS_SELECTION`
- target gate: `C03E_EI_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_AUTH_TARGET_INTENT_CALLER_INGRESS_SELECTED`
- post-EI rolling size: `1100685` bytes
- post-EI rolling SHA-256: `43e53b67ca289df9130027094f97782e1316f801e17077842485a69a8a2c47f0`

C03e-EJ must remain rooted exactly at this predecessor.

## 3. Exact source anchors

At the closed C03e-EI head:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`: `a2066d9917c82e44678548e6db5a97113e2015e9`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`: `4bf8c5589a5523e4bd5ea97e960b1f7f921d5e7d`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`: `5f616f20699d1c7069f5aa8973200a0359c19cde`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`: `47c41735de3c153cde8794b46479e09da7cfba18`
- `crates/prw-agent/src/linux_bootstrap.rs`: `b0fb368d95f35fb034b7cb51c76510fdfcbd7613`

Only the remote-session capability runtime module root is selected for Rust source mutation in C03e-EJ.

## 4. Materialized semantic shape

The source materialization may use the exact semantic shape:

```rust
#[must_use]
#[allow(
    dead_code,
    reason = "C03e-EJ materializes requester-specific post-auth target-intent caller ingress before separately gated control/wire activation"
)]
pub(crate) fn adapt_post_auth_requester_rendezvous_target_intent(
    session_owner: &AuthenticatedRemoteSessionRuntimeOwner,
    target_intent: RequesterRendezvousTargetIntent,
) -> RequesterRendezvousStartIntent {
    session_owner.requester_rendezvous_start_intent_from_target_intent(target_intent)
}
```

Exact formatting or the allow-reason text may change only for rustfmt/lint compliance. The ownership and authority semantics may not change.

## 5. Placement

The selected implementation placement is the existing:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

This avoids creating another state-holding type or another module merely to wrap the already-materialized C03e-EH helper.

The module root may import the existing `RequesterRendezvousStartIntent` and `RequesterRendezvousTargetIntent` from the already-effective-crate-private requester/rendezvous intent module.

No new public re-export is selected.

## 6. Requester identity invariant

The caller seam accepts no requester identity argument.

Requester identity remains available only through the exact authenticated application session retained by the borrowed `AuthenticatedRemoteSessionRuntimeOwner` and used by C03e-EH/C03e-DT.

The seam must not derive requester identity from:

- target `DeviceId`;
- `TransportIdentity`;
- `SessionId`;
- request/correlation IDs;
- control-stream identity;
- endpoints/IP/candidates;
- candidate-publication publisher identity;
- repeated-admission `expected_device_id` as a separate assertion;
- registry roles/enumeration;
- policy/provider/default state.

## 7. Target identity invariant

The target remains exactly the logical `DeviceId` already owned by the consumed `RequesterRendezvousTargetIntent`.

The seam must not accept another raw target and must not infer or replace target identity from:

- requester identity;
- repeated-admission `expected_device_id`;
- publisher identity;
- `TransportIdentity`;
- `SessionId`;
- request/correlation IDs;
- endpoint/candidate state;
- registry/policy/provider state;
- cache, process global, environment, CLI, file or configuration defaults.

## 8. Delegation invariant

The source must delegate exactly once to:

`AuthenticatedRemoteSessionRuntimeOwner::requester_rendezvous_start_intent_from_target_intent(...)`

It must not duplicate the C03e-EH target extraction or C03e-DT authenticated-session requester construction.

The return value is exactly the existing `RequesterRendezvousStartIntent`.

## 9. Side-effect boundary

The materialized function performs no:

- network I/O;
- control-stream acceptance;
- frame parsing or serialization;
- current-authority acquisition;
- registry lookup;
- policy evaluation;
- requester/rendezvous provider mutation;
- synchronization;
- task/channel creation;
- cancellation or lifecycle work;
- readiness publication;
- bootstrap activation.

It is an ownership/typing caller seam only.

## 10. Public API boundary

C03e-EJ must not change:

- public `RemoteSessionWorkerAdmission<D, T>`;
- public executor methods;
- public `LinuxAgentRemoteProcessOperationInputs` or constructor;
- public remote-process operation factory;
- generic bridge APIs;
- any authority-facing item to broader visibility.

The new seam must remain `pub(crate)` or narrower.

## 11. Generic capability lane remains unchanged

No change is selected to:

- `BridgeCommand`;
- `CapabilityBridge`;
- `PolicyEvaluator P`;
- `AuthorizedCapabilityRequest`;
- authorized dispatcher;
- generic capability request wire;
- serial capability request loop;
- generic remote-session worker.

Requester/rendezvous authorization remains outside the principal-agnostic capability evaluator.

## 12. C03e-DV remains uncalled

C03e-EJ does not call or change the signature of:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

The materialized seam does not accept or borrow:

- `SharedCurrentCapabilityAuthority<P>`;
- `BoundedRequesterRendezvousStartPolicySource`;
- `CandidatePublicationRequesterRendezvousRuntimeOwner`.

No registry validation, requester-aware policy evaluation, or provider registration is performed.

## 13. Wire/control remains separately gated

C03e-EJ does not add:

- magic/opcode;
- target `DeviceId` wire encoding;
- requester/rendezvous request frame;
- parser/decoder;
- stream discriminator/multiplexing;
- request/response semantics;
- success/error response;
- retry/peer-close behavior.

A future requester-specific control transaction may call the EJ seam only after a separate selection and materialization gate.

## 14. Expected source diff

Expected changed paths are exactly:

1. this C03e-EJ contract; and
2. `crates/prw-agent/src/remote_session_capability_runtime.rs`.

Expected Rust source delta is limited to:

- import of the existing target/start intent types;
- one crate-private one-shot free function;
- optional side-effect-free compile/signature test only if canonical validation requires it.

No manifest or lockfile mutation is expected.

## 15. Dependency anchors

Must remain byte-stable:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## 16. Explicit exclusions

C03e-EJ excludes:

- C03e-DV invocation;
- requester-aware registry/policy/provider execution;
- requester-policy population/default/refresh/persistence;
- provider construction/capacity/persistence;
- `RemoteSessionWorkerAdmission` changes;
- worker collection/channel/signature widening;
- generic `BridgeCommand` requester/rendezvous behavior;
- target wire encoding/opcode/parser/dispatcher/response;
- target derivation or defaulting;
- public process-input widening;
- bootstrap/main assembly;
- listener/readiness/network activation;
- STUN/ICE/TURN behavior;
- deployment;
- restart/recovery;
- merge.

## 17. Closure criteria

C03e-EJ may close only on one exact final head proving:

1. exact C03e-EI merge base;
2. ahead only and zero behind;
3. only the contract and selected runtime module root changed;
4. Rust source diff limited to the selected import/caller seam and any strictly necessary side-effect-free compile test;
5. C03e-EH runtime implementation remains byte-stable;
6. target-intent source remains byte-stable;
7. executor/worker admission and Linux custody remain byte-stable;
8. dependency anchors remain byte-stable;
9. canonical Rust FULL PASS;
10. Android validation classified exactly as triggered or not triggered;
11. no exact-final-head pending/failing workflow;
12. immutable Drive audit raw-readback byte-exact;
13. rolling predecessor exact post-EI before append;
14. exact EI prefix preserved after append;
15. EJ closure/classification/target-gate markers exactly once;
16. PR remains draft/open/unmerged.

## 18. Target gate

C03e-EJ targets exactly:

`C03E_EJ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_AUTH_TARGET_INTENT_CALLER_INGRESS_SOURCE_MATERIALIZED`

Passing this gate means only that the selected crate-private in-memory caller seam exists. It does not mean requester-specific wire ingress exists, C03e-DV executes, provider state mutates, or networking/deployment is active.
