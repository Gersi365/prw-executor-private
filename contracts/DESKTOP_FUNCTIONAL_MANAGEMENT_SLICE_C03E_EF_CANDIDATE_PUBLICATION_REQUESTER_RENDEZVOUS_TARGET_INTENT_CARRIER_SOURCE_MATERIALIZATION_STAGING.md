# Private Remote Workspace — Phase 152 C03e-EF Requester/Rendezvous Target-Intent Carrier Source Materialization

Status: `STAGED_SOURCE_MATERIALIZATION`

Gate: `C03E_EF_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_CARRIER_SOURCE_MATERIALIZED`

## Purpose

C03e-EF materializes only the dedicated logical target-intent carrier selected by durably closed C03e-EE.

The checkpoint does not introduce wire encoding, stream handling, generic capability authorization, authenticated-session adaptation, C03e-DV invocation, provider mutation, bootstrap activation, networking, deployment, or merge.

## Exact predecessor

C03e-EF is rooted exactly at closed C03e-EE:

- C03e-EE head: `f5f89d26829f067b52a057d96e8050d293185b24`
- C03e-EE tree: `1c4b2313a4767fe2637811061c26b37b84f2dd47`
- C03e-EE PR: `#255`, draft/open/unmerged, `Status: CLOSED`
- post-EE rolling evidence: `1086412` bytes
- post-EE rolling SHA-256: `180a8d9967f2c4563534beab9b54ae1014233f004fd932c84df06115ec4414d5`

## Source placement

The selected carrier is materialized in:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`

This module is already crate-private from `crates/prw-agent/src/lib.rs` and already imports `prw_core::DeviceId` for requester/rendezvous start intent.

No new module registration or dependency is required.

## Materialized semantic shape

The source MAY use the following exact semantic shape:

```text
RequesterRendezvousTargetIntent {
    target_device_id: DeviceId,
}
```

The materialized value is crate-private and owns exactly one logical target `DeviceId` by value.

It exposes only ownership-composition accessors needed by later separately gated adaptation:

- a constructor consuming one explicit logical `DeviceId`;
- a borrowed target accessor;
- a consuming accessor that returns the same `DeviceId` without reinterpretation.

## Authority invariants

The carrier contains no requester identity.

Requester identity remains obtainable only from the exact retained `AuthenticatedDeviceSession` at the authenticated-session runtime boundary.

The carrier contains no:

- `TransportIdentity`;
- `SessionId`;
- request ID;
- registry principal;
- policy decision;
- endpoint or IP address;
- candidate;
- provider handle;
- freshness value;
- role;
- workspace/user authority claim;
- fallback/default target.

Possession of the carrier is not authorization, current-registration proof, workspace proof, requester-target relationship proof, or provider-registration permission.

## Target invariant

The carried `DeviceId` is exactly the explicit caller-nominated logical rendezvous target.

Construction and access MUST NOT infer or replace it from:

- the requester's authenticated logical `DeviceId`;
- repeated-admission `expected_device_id`;
- candidate-publication publisher identity;
- `TransportIdentity`;
- `SessionId`;
- request correlation;
- endpoint/candidate state;
- registry role or enumeration;
- provider state;
- cached target state;
- environment/configuration defaults.

## Relationship to existing start intent

The existing `RequesterRendezvousStartIntent` remains unchanged in authority semantics:

```text
RequesterRendezvousStartIntent {
    requester_session: AuthenticatedDeviceSession,
    target_device_id: DeviceId,
}
```

C03e-EF does not replace it and does not construct it automatically.

A later separately gated adaptation checkpoint may consume `RequesterRendezvousTargetIntent` together with the exact authenticated-session runtime owner to construct the existing start intent.

## Relationship to C03e-DV

C03e-DV remains source-materialized and uncalled.

C03e-EF does not call:

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`

and does not borrow current registry authority, requester-aware policy source, or mutable requester/rendezvous runtime owner.

## Generic capability lane remains unchanged

C03e-EF does not add a `BridgeCommand` variant and does not modify `CapabilityBridge`.

Requester/rendezvous policy therefore remains outside the principal-agnostic generic capability policy evaluator `P`, as selected by closed DW/DX/DV/EE.

## Wire remains separately gated

C03e-EF materializes an in-memory typed value only.

It does not select or materialize:

- PRWM/PRWC/PRWP magic or opcode;
- byte representation of `DeviceId`;
- parser/decoder;
- request/response frame;
- multiplexing discriminator;
- stream acceptance;
- malformed-target errors;
- retry semantics;
- negative response semantics;
- peer-close behavior.

## Mutability and lifecycle

The carrier owns one immutable target value after construction.

No update, replace, clear, cache, persistence, synchronization, watch, refresh, or background lifecycle API is selected.

The consuming accessor is ownership transfer only and does not mutate external authority.

## Source scope

Expected source mutation is limited to:

1. this contract; and
2. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`.

No manifest or lockfile change is expected.

No source change to authenticated-session runtime, Linux bootstrap, remote bridge, provider implementation, worker lifecycle, candidate publication, or main is selected.

## Explicit exclusions

C03e-EF excludes:

- authenticated-session target adaptation;
- requester identity construction from the carrier;
- registry validation;
- requester-aware policy evaluation;
- provider registration;
- C03e-DV invocation;
- target wire ingress;
- generic `BridgeCommand` integration;
- candidate-publication command changes;
- policy population/default/currentness;
- provider construction/capacity selection;
- public process-input widening;
- worker/admission/lifecycle signature widening;
- bootstrap/main activation;
- listener/readiness/network activation;
- STUN/ICE/TURN behavior;
- deployment;
- restart/recovery;
- merge.

## Dependency expectation

The following anchors must remain byte-stable:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Closure criteria

C03e-EF may close only on one exact final head with:

1. exact C03e-EE merge base;
2. ahead only and zero behind;
3. only the contract plus the selected Agent source path changed;
4. final source diff limited to the dedicated target-intent carrier and ownership-only methods;
5. no authenticated-session adaptation or DV invocation;
6. canonical Rust validation FULL PASS;
7. Android validation assessed exactly as triggered or not triggered, without fabricated PASS;
8. dependency anchors byte-stable;
9. immutable Drive audit raw-readback byte-exact;
10. rolling predecessor exactly post-EE before append;
11. rolling predecessor prefix byte-exact after append;
12. C03e-EF closure/classification/target-gate markers exactly once;
13. PR remains draft/open/unmerged.

## Target gate

C03e-EF targets exactly:

`C03E_EF_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_TARGET_INTENT_CARRIER_SOURCE_MATERIALIZED`

Passing this gate means only that the dedicated in-memory typed carrier exists. It does not mean that production wire can create it, an authenticated session consumes it, C03e-DV runs, or any rendezvous/network/deployment action is activated.
