# Phase 152 C03e-EO — Requester Rendezvous Decoded Target → Agent Target-Intent Adaptation Source Materialization

Status: `READY_FOR_SOURCE_VALIDATION`

## 1. Purpose

C03e-EO source-materializes exactly the pure Agent-side adaptation selected and durably closed by C03e-EN.

The implementation accepts one already-decoded logical `DeviceId` by value and returns the existing crate-private `RequesterRendezvousTargetIntent` by calling its existing constructor exactly once.

This checkpoint does not add stream I/O, invoke decoding from I/O, invoke C03e-EJ from I/O, invoke C03e-DV, or activate requester/rendezvous execution.

## 2. Exact predecessor

- predecessor checkpoint: `C03e-EN`
- predecessor branch: `phase-152-c03e-en-requester-rendezvous-decoded-target-agent-target-intent-adaptation-selection-staging`
- exact predecessor head: `9fb3bbf07d71ac9490278b287bed9bb0749bba44`
- exact predecessor tree: `9f33789261973914fcb4cc34345c65226b4d2b5f`
- predecessor closure: `CLOSED_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SELECTION`
- predecessor target gate: `C03E_EN_REQUESTER_RENDEZVOUS_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SELECTED`

C03e-EN remains untouched.

## 3. Materialized source boundary

Source location:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

C03e-EO adds one crate-private pure helper with the selected semantic shape:

```rust
pub(crate) const fn adapt_decoded_requester_rendezvous_target_device_id(
    target_device_id: DeviceId,
) -> RequesterRendezvousTargetIntent {
    RequesterRendezvousTargetIntent::new(target_device_id)
}
```

The helper may carry only documentation and lint-scoped dead-code staging metadata in addition to this behavior.

## 4. Exact ownership semantics

The helper:

- consumes exactly one existing `DeviceId` by value;
- performs no clone;
- performs no string conversion;
- performs no normalization;
- performs no registry lookup;
- performs no policy evaluation;
- performs no provider mutation;
- performs no I/O;
- returns exactly one existing `RequesterRendezvousTargetIntent`.

## 5. Identity invariants

- the input `DeviceId` remains target nomination only;
- requester identity is absent from this helper;
- requester identity remains sourced only from the retained authenticated application session at the existing C03e-EH/EJ boundary;
- transport identity, request correlation, endpoints, candidates, registry rows, policy/provider state, caches, and defaults cannot substitute for requester or target identity.

## 6. Request correlation invariants

C03e-EO does not accept or return a PRWM `request_id`.

The outer request ID remains transaction correlation owned by a separately gated future transaction boundary. It is not encoded into `RequesterRendezvousTargetIntent` and is not interpreted as identity or authority.

## 7. Crate-boundary invariants

C03e-EO preserves:

- bridge ownership of PRWZ wire encode/decode;
- Agent ownership of `RequesterRendezvousTargetIntent`;
- no bridge → Agent dependency;
- no Agent type exposure from bridge;
- no public visibility widening;
- no manifest or lockfile change;
- no dependency upgrade.

## 8. Test boundary

The same Agent source file may add a narrow unit test proving that the helper preserves the exact logical target value through `RequesterRendezvousTargetIntent::target_device_id()`.

The test must not add I/O, mocks of network behavior, provider execution, policy execution, or runtime activation.

## 9. Authorized changed paths

C03e-EO authorizes exactly two changed paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs`.

No other repository path is authorized to change.

## 10. Explicit exclusions

C03e-EO does not materialize or activate:

- `RequesterRendezvousTargetWireRequest` decoding invocation from any stream;
- requester-specific stream acceptance/read/write;
- a combined wire-to-EJ transaction;
- C03e-EJ invocation from I/O;
- C03e-DV invocation;
- registry/workspace/requester-policy/provider execution;
- success/error response protocol;
- request-ID lifecycle/replay/idempotency semantics;
- candidate response wire format;
- direct Internet dialing;
- relay dialing;
- SSH/traffic dialing;
- queue/retry/reconnect;
- TTL/revocation reinterpretation;
- generic `BridgeCommand` behavior;
- worker/admission/public API widening;
- bootstrap/main/network activation;
- deployment;
- restart/recovery;
- merge.

## 11. Validation contract

Required exact-head canonical validation:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --all-targets
```

Android must be classified only from actual exact-head workflow behavior. No Android PASS may be inferred from absence or `SKIPPED`.

## 12. Closure criteria

C03e-EO may close only if the exact final head proves:

1. exact C03e-EN merge base;
2. ahead only and zero behind;
3. exactly the two authorized changed paths;
4. no manifest/lock/dependency/configuration drift;
5. helper semantics exactly match C03e-EN selection;
6. target-preservation unit test is source-materialized;
7. canonical Rust validation FULL PASS;
8. Android classified exactly from actual exact-head workflow status;
9. no exact-final-head required workflow pending or failing;
10. immutable Drive audit raw readback byte-exact;
11. rolling predecessor exact post-EN before append;
12. exact EN prefix preserved after append;
13. EO closure/classification/target-gate markers each exactly once;
14. PR remains draft/open/unmerged.

## 13. Closure classification

If all criteria pass:

`CLOSED_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SOURCE_MATERIALIZATION`

## 14. Successor boundary

After durable C03e-EO closure, a successor may separately select whether and where one requester/rendezvous transaction can combine existing bridge decode output, this Agent target-intent adaptation, existing C03e-EJ authenticated-session adaptation, request correlation, and later authority execution.

That future selection must explicitly decide stream ownership and error/response semantics before any I/O activation. C03e-EO itself authorizes none of those behaviors.

## 15. Target gate

C03e-EO targets exactly:

`C03E_EO_REQUESTER_RENDEZVOUS_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SOURCE_MATERIALIZED`

Passing this gate means only that the selected pure Agent adaptation exists in source and validates. It does not mean any remote requester/rendezvous transaction executes.