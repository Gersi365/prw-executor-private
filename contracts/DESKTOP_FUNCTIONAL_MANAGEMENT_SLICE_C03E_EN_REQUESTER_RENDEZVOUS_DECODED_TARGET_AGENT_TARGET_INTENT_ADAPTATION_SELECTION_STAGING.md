# Phase 152 C03e-EN — Requester Rendezvous Decoded Target → Agent Target-Intent Adaptation Selection

Status: `READY_FOR_SELECTION_VALIDATION`

## 1. Purpose

C03e-EN selects exactly one narrow Agent-only semantic adaptation boundary after closed C03e-EM.

The selected boundary converts one already-decoded caller-nominated logical `DeviceId` into the existing crate-private `RequesterRendezvousTargetIntent` without adding stream I/O, requester authorization, registry/policy/provider execution, or rendezvous dialing.

This checkpoint is docs-only. It does not materialize source behavior.

## 2. Exact predecessor

Authoritative predecessor:

- checkpoint: `C03e-EM`;
- branch: `phase-152-c03e-em-requester-rendezvous-prwm-target-request-codec-source-materialization-staging`;
- exact predecessor head: `dafe62c471fd1396cdf695e2dd4a14a8e8a0f9cd`;
- exact predecessor tree: `6d430c6a5360ed0db183c01b7ac0adc02729b91d`;
- predecessor closure: `CLOSED_SOURCE_MATERIALIZATION`;
- predecessor target gate: `C03E_EM_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_CODEC_SOURCE_MATERIALIZED`.

C03e-EM remains untouched.

## 3. Exact topology facts at predecessor head

### 3.1 Bridge-owned decoded request

`crates/prw-remote-bridge/src/requester_rendezvous_target_request_wire.rs` owns the pure PRWZ decoder and exposes:

- `RequesterRendezvousTargetWireRequest::request_id() -> u64`;
- `RequesterRendezvousTargetWireRequest::target_device_id() -> &DeviceId`;
- `RequesterRendezvousTargetWireRequest::into_target_device_id() -> DeviceId`.

The wire type intentionally contains no requester/session/transport identity.

### 3.2 Existing Agent target intent

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs` already owns:

- `RequesterRendezvousTargetIntent`;
- `RequesterRendezvousTargetIntent::new(DeviceId)`;
- `RequesterRendezvousTargetIntent::into_target_device_id()`.

`RequesterRendezvousTargetIntent` is nomination only. Possession is not authorization or current-registration proof.

### 3.3 Existing post-auth caller seam

`crates/prw-agent/src/remote_session_capability_runtime.rs` already owns the C03e-EJ crate-private seam:

`adapt_post_auth_requester_rendezvous_target_intent(&AuthenticatedRemoteSessionRuntimeOwner, RequesterRendezvousTargetIntent) -> RequesterRendezvousStartIntent`.

That seam derives requester identity only from the exact retained authenticated application session through the existing authenticated-session adapter.

### 3.4 Existing current-authority composition remains separate

`AuthenticatedRemoteSessionRuntimeOwner::register_requester_rendezvous_start(...)`, materialized by C03e-DV, already performs the separately bounded current-registry/requester-policy/provider composition.

C03e-EN does not invoke, widen, replace, or reinterpret that helper.

## 4. Selected adaptation boundary

C03e-EN selects an Agent-owned pure by-value adaptation with this semantic shape:

```text
already-decoded logical DeviceId
        |
        | exact by-value ownership transfer only
        v
RequesterRendezvousTargetIntent::new(DeviceId)
```

The adaptation must preserve the exact logical target value byte-for-byte/domain-value-for-domain-value and must not derive any additional authority from it.

## 5. Selected ownership location

Any later source materialization selected from this contract must remain inside `prw-agent` and adjacent to the existing C03e-EJ post-auth target-intent caller seam.

Preferred source location:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

No bridge source mutation is required for the adaptation itself because C03e-EM already exposes ownership transfer through `into_target_device_id()` and the Agent target-intent constructor already exists.

## 6. Selected source-level shape for a later materialization checkpoint

The later implementation, if separately authorized by a successor source-materialization checkpoint, should be no broader than one crate-private pure helper equivalent to:

```rust
pub(crate) fn adapt_decoded_requester_rendezvous_target_device_id(
    target_device_id: DeviceId,
) -> RequesterRendezvousTargetIntent {
    RequesterRendezvousTargetIntent::new(target_device_id)
}
```

The exact function name may be normalized during the source-materialization checkpoint, but the semantic boundary may not broaden.

## 7. Request-ID ownership

The outer PRWM `request_id` remains wire-transaction correlation owned outside this adaptation.

C03e-EN explicitly does not:

- embed request ID into `RequesterRendezvousTargetIntent`;
- reinterpret request ID as requester identity;
- reinterpret request ID as target identity;
- allocate request IDs;
- define replay/idempotency semantics;
- define success/error response semantics.

A later stream/transaction checkpoint must preserve request correlation separately if it is selected.

## 8. Identity invariants

Requester identity remains only the exact authenticated application session retained by the existing Agent session owner.

Target identity remains only the caller-nominated logical `DeviceId` decoded by C03e-EM and transferred unchanged into `RequesterRendezvousTargetIntent`.

Transport identity, endpoint candidates, publisher identity, session/request correlation, registry rows, cached state, provider state, and defaults must not substitute for either identity.

## 9. Authority invariants

The selected adaptation proves only typing and ownership transfer.

It does not prove:

- requester authorization;
- requester currentness;
- target current registration;
- workspace relationship;
- requester-aware policy admission;
- provider registration;
- target reachability;
- transport eligibility;
- successful rendezvous.

Those remain separately gated by existing or future checkpoints.

## 10. Crate-boundary invariants

C03e-EN preserves:

- bridge ownership of PRWZ encode/decode;
- Agent ownership of authenticated-session identity and `RequesterRendezvousTargetIntent`;
- no bridge → Agent dependency;
- no Agent type exposure from bridge;
- no widening of Agent visibility beyond crate-internal use;
- no manifest or lockfile changes.

## 11. Explicitly allowed changes in C03e-EN

C03e-EN itself may change only:

1. this contract document;
2. PR metadata;
3. durable audit/rolling evidence outside repository source after exact-head validation.

No Rust/Kotlin/Gradle/source/configuration file is authorized to change in C03e-EN.

## 12. Explicit exclusions

C03e-EN does not select or materialize:

- the adaptation source helper itself;
- `MeshControlStream` acceptance/read/write;
- requester-specific stream I/O;
- decoding invocation from I/O;
- C03e-EJ invocation from I/O;
- C03e-DV invocation;
- registry/policy/provider execution;
- success/error response protocol;
- request-ID lifecycle/replay/idempotency;
- rendezvous candidate response encoding;
- direct Internet dialing;
- relay dialing;
- SSH/traffic dialing;
- queue/retry/reconnect behavior;
- TTL/revocation/policy reinterpretation;
- generic `BridgeCommand` redesign;
- worker/admission/public API widening;
- bootstrap/main/network activation;
- dependency upgrades;
- deployment;
- restart/recovery;
- merge.

## 13. Validation contract

C03e-EN must remain an exact one-file docs-only delta from C03e-EM.

Required validation:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --all-targets
```

Android is classified only from actual exact-head workflow behavior; no PASS may be inferred from non-triggering or SKIPPED status.

## 14. Closure criteria

C03e-EN may close only on one exact final head proving:

1. exact C03e-EM merge base;
2. ahead only and zero behind;
3. exactly one changed path: this contract;
4. zero source/manifest/lock drift;
5. canonical Rust exact-head validation FULL PASS;
6. Android classified exactly from actual workflow status;
7. no exact-final-head required workflow pending or failing;
8. immutable Drive audit raw-readback byte-exact;
9. rolling predecessor is exact post-EM before append;
10. exact EM rolling prefix preserved after append;
11. EN closure/classification/target-gate markers each exactly once;
12. PR remains draft/open/unmerged.

## 15. Closure classification

If all criteria pass, C03e-EN closes as:

`CLOSED_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SELECTION`

## 16. Successor boundary

Only after durable C03e-EN closure may a successor source-materialization checkpoint add the one selected Agent-only pure adaptation helper.

That successor still must not introduce stream I/O, invoke C03e-EJ from I/O, invoke C03e-DV, or define a response protocol.

## 17. Target gate

C03e-EN targets exactly:

`C03E_EN_REQUESTER_RENDEZVOUS_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SELECTED`

Passing this gate means only that the pure Agent-side adaptation boundary has been selected. It does not mean the adaptation executes from any remote transaction.