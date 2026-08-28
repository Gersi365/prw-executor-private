# Phase 152 C03e-EG — Candidate-Publication Requester/Rendezvous Target-Intent Authenticated-Session Adaptation Selection

Status: `STAGING_SELECTION`

Target gate: `C03E_EG_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_AUTHENTICATED_SESSION_ADAPTATION_SELECTED`

## 1. Purpose

C03e-EG selects, but does not source-materialize, the narrow authenticated-session adaptation boundary that turns one already-typed caller-nominated `RequesterRendezvousTargetIntent` into the existing `RequesterRendezvousStartIntent` while preserving the authenticated requester identity retained by `AuthenticatedRemoteSessionRuntimeOwner`.

This checkpoint exists because durably closed C03e-EF materialized the dedicated target-intent carrier but deliberately did not connect it to the authenticated-session runtime. C03e-EG closes only that design gap. It does not invoke requester authorization, registry validation, requester/rendezvous provider mutation, wire handling, bootstrap assembly, or networking.

## 2. Exact predecessor

Durably closed predecessor: C03e-EF.

- predecessor branch: `phase-152-c03e-ef-candidate-publication-requester-rendezvous-target-intent-carrier-source-materialization-staging`
- predecessor head: `d4f2a2e84f342e45146269210ae3952491a7cac4`
- predecessor tree: `391734374b0f00768a90db4815cda4885bb6bbee`
- predecessor PR: `#256`, draft/open/unmerged, `Status: CLOSED`
- predecessor closure classification: `SOURCE_MATERIALIZATION_FULL_PASS`
- predecessor target gate: `C03E_EF_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_CARRIER_SOURCE_MATERIALIZED`

C03e-EG must remain rooted exactly at that predecessor. No earlier checkpoint is reopened.

## 3. Exact topology observed at the C03e-EF head

### 3.1 Dedicated target-intent carrier now exists

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs` contains the C03e-EF carrier:

```rust
pub struct RequesterRendezvousTargetIntent {
    target_device_id: DeviceId,
}
```

with typed construction/access semantics for exactly one logical target `DeviceId`:

```rust
pub const fn new(target_device_id: DeviceId) -> Self
pub const fn target_device_id(&self) -> &DeviceId
pub fn into_target_device_id(self) -> DeviceId
```

The carrier contains no requester identity, no authenticated session, no policy material, and no provider state.

Its containing module remains effective crate-private:

```rust
pub(crate) mod candidate_publication_requester_rendezvous_start_intent;
```

Therefore C03e-EG must not widen any authority-facing public API merely to perform the adaptation.

### 3.2 Existing start-intent type already carries both identities

The same source module already contains:

```rust
pub struct RequesterRendezvousStartIntent {
    requester_session: AuthenticatedDeviceSession,
    target_device_id: DeviceId,
}
```

That type is the already-selected typed boundary used by downstream requester/rendezvous validation and authorization composition.

### 3.3 Existing authenticated-session runtime owns the requester identity

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` retains the authenticated application session through the existing runtime ownership chain. Its `logical_device_id()` derives only from the retained authenticated application session.

The existing C03e-DT helper is:

```rust
pub(crate) fn requester_rendezvous_start_intent(
    &self,
    target_device_id: DeviceId,
) -> RequesterRendezvousStartIntent {
    RequesterRendezvousStartIntent::new(
        self.capability_owner.bound_session.session().clone(),
        target_device_id,
    )
}
```

This helper already establishes the correct identity split:

- requester identity comes only from the exact retained authenticated `AuthenticatedDeviceSession`;
- target identity comes only from the explicit caller-nominated logical `DeviceId` argument.

### 3.4 Existing C03e-DV composition remains deliberately uncalled

The runtime also contains the source-materialized C03e-DV method:

```rust
pub(crate) async fn register_requester_rendezvous_start<...>(
    &self,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    target_device_id: DeviceId,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

It currently constructs the start intent from a raw target `DeviceId`, then performs one current-authority scope and delegates to the closed requester-aware validation/authorization/provider composition.

C03e-EG does not select invocation of this method and does not alter its signature.

### 3.5 Missing boundary

At the exact C03e-EF head, the authenticated-session runtime does not import or consume `RequesterRendezvousTargetIntent`. The newly materialized target carrier therefore has no selected authenticated-session adaptation boundary yet.

The missing step is not another target source and not another requester source. It is a typed adapter that consumes the target carrier while sourcing requester identity exclusively from the already-authenticated runtime owner.

## 4. Selected C03e-EG boundary

C03e-EG selects a future crate-private helper on `AuthenticatedRemoteSessionRuntimeOwner` with semantics equivalent to:

```rust
pub(crate) fn requester_rendezvous_start_intent_from_target_intent(
    &self,
    target_intent: RequesterRendezvousTargetIntent,
) -> RequesterRendezvousStartIntent {
    self.requester_rendezvous_start_intent(
        target_intent.into_target_device_id(),
    )
}
```

The exact helper name may change only for local readability or lint compliance. The semantic boundary is fixed by this contract.

## 5. Required adaptation semantics

The future source materialization selected by C03e-EG must satisfy all of the following.

1. **Consume the typed target intent by value.**
   - Input is one already-constructed `RequesterRendezvousTargetIntent`.
   - No raw target reconstruction source is introduced in the adapter.

2. **Preserve requester identity provenance.**
   - Requester identity must come only from the exact authenticated application session retained by `AuthenticatedRemoteSessionRuntimeOwner`.
   - The adapter must not accept requester `DeviceId`, `AuthenticatedDeviceSession`, principal, role, transport identity, endpoint, request ID, or any equivalent alternate requester source as an argument.

3. **Preserve target identity provenance.**
   - Target identity must come only from `target_intent.into_target_device_id()` or an equivalent consuming extraction from the exact carrier.
   - The target must not be inferred, replaced, normalized, or cross-filled from requester/session/transport/provider state.

4. **Reuse the existing C03e-DT construction boundary.**
   - Preferred implementation delegates exactly once to the existing `requester_rendezvous_start_intent(DeviceId)` helper.
   - This avoids duplicating authenticated-session extraction and keeps one canonical requester-identity construction path.

5. **Return only the existing typed start intent.**
   - Output is `RequesterRendezvousStartIntent`.
   - No registry result, authorization result, provider registration result, wire response, or lifecycle state is produced at this checkpoint.

6. **Remain crate-private.**
   - No requester/rendezvous authority-facing type or helper is made externally public.
   - Existing public APIs remain unchanged.

7. **Remain side-effect free.**
   - No I/O.
   - No network action.
   - No lock/current-authority acquisition.
   - No registry lookup.
   - No policy evaluation.
   - No provider mutation.
   - No lifecycle or readiness mutation.

## 6. Identity invariants

C03e-EG preserves the project-wide identity model without reinterpretation.

### Requester

The logical requester is the `DeviceId` carried by the retained authenticated application session owned by `AuthenticatedRemoteSessionRuntimeOwner`.

The requester is **not**:

- the target `DeviceId`;
- a `TransportIdentity`;
- a `SessionId`;
- an endpoint or IP address;
- a candidate address;
- a request/correlation ID;
- repeated-admission `expected_device_id`;
- a publisher identity;
- a role-derived or policy-derived identity.

### Target

The logical target is exactly the caller-nominated `DeviceId` already encapsulated by `RequesterRendezvousTargetIntent`.

The target is **not** derived from:

- requester logical identity;
- retained authenticated session identity;
- repeated-admission `expected_device_id`;
- PRWC publisher identity;
- `TransportIdentity`;
- `SessionId`;
- request/correlation IDs;
- endpoints, IPs, or candidate addresses;
- capability roles;
- registry contents;
- requester policy source;
- provider/runtime-owner state;
- cache, process global, environment, CLI, file, or default value.

## 7. Explicit non-selection

C03e-EG does **not** select or authorize any of the following:

- source materialization of the adapter in this checkpoint;
- modification or replacement of `RequesterRendezvousTargetIntent`;
- modification or replacement of `RequesterRendezvousStartIntent`;
- modification of the existing raw-`DeviceId` C03e-DT helper;
- invocation or signature change of C03e-DV `register_requester_rendezvous_start(...)`;
- registry validation;
- requester-aware policy evaluation;
- provider registration or mutation;
- policy-source population, persistence, refresh, defaulting, or process-global authority;
- provider construction/capacity changes;
- generic principal-agnostic policy `P` as requester-specific authority;
- `BridgeCommand` changes;
- PRWP/PRWC/PRWM opcode, parser, frame, request, response, or protocol changes;
- target encoding on the wire;
- derivation of target from candidate-publication payloads;
- dispatcher routing;
- target producer selection outside this already-typed ingress carrier;
- public `LinuxAgentRemoteProcessOperationInputs` changes;
- public worker/lifecycle/process-input widening;
- bootstrap/main assembly;
- listener/readiness/network activation;
- deployment, restart, recovery, or merge.

## 8. Expected future source-materialization scope

A later checkpoint, only after C03e-EG is durably closed, may source-materialize the selected helper in `authenticated_remote_session_runtime.rs`.

The expected narrow source delta is:

- import `RequesterRendezvousTargetIntent` alongside the existing requester/rendezvous intent types;
- add one crate-private helper consuming the typed target carrier;
- delegate to the existing C03e-DT helper with the carrier's exact logical target `DeviceId`;
- add side-effect-free unit/ownership tests if needed for canonical validation.

That future checkpoint must not simultaneously invoke C03e-DV or expand into registry/policy/provider/wire/bootstrap activation.

## 9. Validation contract for C03e-EG itself

C03e-EG is a docs-only selection checkpoint.

Before durable closure it must prove:

- exact predecessor head/tree remains C03e-EF;
- exact merge base is the C03e-EF head;
- no behind commits;
- changed path set contains only this contract;
- authenticated-session runtime source remains byte-stable;
- target-intent source remains byte-stable;
- dependency anchors remain unchanged;
- canonical exact-head CI has no pending/failing required validation;
- any SKIPPED workflow remains classified as SKIPPED, never PASS;
- PR remains draft/open/unmerged.

## 10. Dependency anchors to preserve

At the closed C03e-EF head:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

C03e-EG selects no manifest or lockfile mutation.

## 11. Successor boundary

If C03e-EG closes durably with exact-head validation and evidence readback, the next checkpoint may select/materialize only the corresponding source adapter from typed target intent to authenticated-session start intent.

No successor may treat this selection as authorization to invoke C03e-DV, mutate requester/rendezvous provider state, add wire ingress, activate networking, deploy, restart, recover, or merge.
