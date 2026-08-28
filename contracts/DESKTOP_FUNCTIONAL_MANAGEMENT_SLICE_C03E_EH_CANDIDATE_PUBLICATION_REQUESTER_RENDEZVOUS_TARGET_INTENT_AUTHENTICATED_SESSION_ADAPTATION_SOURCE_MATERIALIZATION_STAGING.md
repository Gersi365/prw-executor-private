# Phase 152 C03e-EH — Candidate-Publication Requester/Rendezvous Target-Intent Authenticated-Session Adaptation Source Materialization

Status: `STAGING_SOURCE_MATERIALIZATION`

Target gate: `C03E_EH_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_AUTHENTICATED_SESSION_ADAPTATION_SOURCE_MATERIALIZED`

## 1. Purpose

C03e-EH source-materializes only the crate-private authenticated-session adapter selected by durably closed C03e-EG. The adapter consumes one already-typed caller-nominated `RequesterRendezvousTargetIntent`, extracts its exact logical target `DeviceId`, and delegates to the existing C03e-DT authenticated-session start-intent helper so requester identity remains sourced exclusively from the retained authenticated application session.

C03e-EH does not invoke C03e-DV, perform registry validation or requester-aware policy evaluation, mutate requester/rendezvous provider state, add target wire ingress, widen public API, activate networking, deploy, restart, recover, or merge.

## 2. Exact predecessor

Durably closed predecessor: C03e-EG.

- predecessor branch: `phase-152-c03e-eg-candidate-publication-requester-rendezvous-target-intent-authenticated-session-adaptation-selection-staging`
- predecessor PR: #257, draft/open/unmerged, `Status: CLOSED`
- predecessor head: `1b2b149295f80f22149235ce2835ce48778f00c2`
- predecessor tree: `2f7e9aca901ecf6955601ac8221559904b4c77c2`
- predecessor closure classification: `CLOSED_AUTHENTICATED_SESSION_ADAPTATION_SELECTION`
- predecessor gate: `C03E_EG_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_AUTHENTICATED_SESSION_ADAPTATION_SELECTED`

The C03e-EH branch was created directly from that exact predecessor head.

## 3. Source materialized

The only Rust source path modified by the materialization commit is:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

Closed-EG predecessor blob:

`db90d55be95dcec1e8e9d1e6be15b1ed11121642`

C03e-EH materialized blob after the source commit:

`4bf8c5589a5523e4bd5ea97e960b1f7f921d5e7d`

Source commit:

`735464e1e7ae76c35fe1846a24dd7d865dfe5296` — `Phase 152 C03e-EH: materialize authenticated-session target-intent adaptation`

Its exact predecessor compare is one source path, `+39/-2`. The two deleted lines are replaced import forms; no prior runtime behavior is removed.

## 4. Exact adapter

C03e-EH imports the already-existing C03e-EF target carrier alongside the existing start intent:

```rust
RequesterRendezvousStartIntent, RequesterRendezvousTargetIntent,
```

It materializes exactly one new crate-private helper on `AuthenticatedRemoteSessionRuntimeOwner`:

```rust
#[must_use]
#[allow(
    dead_code,
    reason = "C03e-EH materializes typed target-intent authenticated-session adaptation before separately gated caller activation"
)]
pub(crate) fn requester_rendezvous_start_intent_from_target_intent(
    &self,
    target_intent: RequesterRendezvousTargetIntent,
) -> RequesterRendezvousStartIntent {
    self.requester_rendezvous_start_intent(target_intent.into_target_device_id())
}
```

This is the exact C03e-EG-selected composition shape: consume typed target by value, extract only its exact logical `DeviceId`, then delegate to C03e-DT for authenticated requester construction.

## 5. Requester identity provenance

Requester identity is not accepted as an argument to the new helper.

It continues to originate only through the pre-existing C03e-DT helper:

```rust
RequesterRendezvousStartIntent::new(
    self.capability_owner.bound_session.session().clone(),
    target_device_id,
)
```

Therefore the requester remains the exact logical `DeviceId` represented by the retained authenticated application session.

The helper does not derive or substitute requester identity from:

- target `DeviceId`;
- `TransportIdentity`;
- `SessionId`;
- repeated-admission `expected_device_id`;
- publisher identity;
- request/correlation IDs;
- endpoint/IP/candidate metadata;
- roles;
- registry contents;
- principal-agnostic policy state;
- requester policy backing;
- provider state;
- environment, CLI, file, cache, process-global, or default state.

## 6. Target identity provenance

Target identity comes only from consuming the exact `RequesterRendezvousTargetIntent`:

```rust
target_intent.into_target_device_id()
```

No target validation or authority is inferred from possession of that carrier. The target remains unvalidated caller-nominated intent until separately gated registry/workspace/policy composition occurs later.

The target is not derived from:

- requester logical identity;
- authenticated-session identity;
- repeated-admission `expected_device_id`;
- publisher identity;
- `TransportIdentity`;
- `SessionId`;
- request/correlation IDs;
- endpoints/IP/candidate addresses;
- roles;
- registry contents;
- requester policy source;
- provider/runtime state;
- cache, process globals, environment, CLI, files, or defaults.

## 7. Side-effect boundary

The new helper is a pure ownership/composition adapter. It performs no:

- authentication;
- registry lookup;
- current-authority acquisition;
- policy evaluation;
- provider mutation;
- I/O;
- synchronization;
- wire parsing or serialization;
- dispatcher execution;
- task spawn/join/cancellation;
- listener/readiness mutation;
- network action;
- persistence or refresh.

It only consumes one target carrier and delegates once to the existing C03e-DT construction helper.

## 8. Compile-shape test

C03e-EH adds a side-effect-free signature assertion requiring the exact selected method shape:

```rust
fn assert_requester_rendezvous_target_intent_adaptation_signature(
    adaptation: fn(
        &AuthenticatedRemoteSessionRuntimeOwner,
        RequesterRendezvousTargetIntent,
    ) -> RequesterRendezvousStartIntent,
)
```

The test binds:

```rust
AuthenticatedRemoteSessionRuntimeOwner::requester_rendezvous_start_intent_from_target_intent
```

No peer construction, registry mutation, policy evaluation, provider mutation, or network I/O is needed for this ownership/signature proof.

## 9. Existing boundaries left unchanged

C03e-EH does not modify:

- `RequesterRendezvousTargetIntent` source or semantics;
- `RequesterRendezvousStartIntent` source or semantics;
- C03e-DT `requester_rendezvous_start_intent(DeviceId)` signature or body;
- C03e-DV `register_requester_rendezvous_start(...)` signature or body;
- `SharedCurrentCapabilityAuthority<P>`;
- requester-aware policy source/backing;
- requester/rendezvous provider runtime owner;
- public `LinuxAgentRemoteProcessOperationInputs` or worker/lifecycle surfaces;
- `BridgeCommand` or PRWP/PRWC/PRWM protocol surfaces.

C03e-DV remains source-materialized and deliberately uncalled.

## 10. Public API invariants

The new helper is `pub(crate)` and the target/start-intent module remains effective crate-private outside `prw-agent`.

No requester/rendezvous authority-facing item is made externally public. No existing public constructor or operation signature is widened or replaced.

## 11. Validation requirements

Before durable C03e-EH closure, the exact final head must prove:

- exact C03e-EG merge base;
- zero behind commits;
- changed paths limited to this contract and the one authenticated-session runtime source;
- source patch limited to the selected import/helper/signature-test materialization;
- target-intent source remains byte-stable;
- C03e-DV source semantics remain unchanged;
- dependency anchors remain byte-stable;
- canonical Rust validation FULL PASS including locked graph, rustfmt, Clippy, workspace tests and workspace build;
- Android validation must be classified only from the actual workflow result if triggered;
- any SKIPPED workflow remains SKIPPED;
- no exact-final-head pending/failing workflow;
- PR remains draft/open/unmerged.

## 12. Dependency anchors to preserve

From closed C03e-EG:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

No manifest or lockfile change is selected.

## 13. Explicit exclusions

C03e-EH does not authorize or materialize:

- C03e-DV invocation;
- conversion of the new typed adapter into a registration/authorization call;
- registry validation or target eligibility evaluation;
- requester-aware policy execution;
- provider registration/mutation;
- policy population/default/currentness/persistence/live refresh;
- provider construction/capacity changes;
- target derivation from session/transport/publication metadata;
- `BridgeCommand` changes;
- target wire encoding;
- PRWP/PRWC/PRWM opcode/parser/frame/request/response changes;
- dispatcher routing;
- bootstrap/main production assembly;
- process-input/worker/lifecycle public widening;
- listener/readiness/network activation;
- deployment, restart, recovery, or merge.

## 14. Successor boundary

After C03e-EH is durably closed with exact-head CI and Drive evidence, any successor must begin with a fresh topology audit. The existence of this typed adaptation does not itself authorize C03e-DV invocation. A later checkpoint must separately select the next caller/ingress boundary before requester-aware validation/authorization/provider mutation can become reachable.
