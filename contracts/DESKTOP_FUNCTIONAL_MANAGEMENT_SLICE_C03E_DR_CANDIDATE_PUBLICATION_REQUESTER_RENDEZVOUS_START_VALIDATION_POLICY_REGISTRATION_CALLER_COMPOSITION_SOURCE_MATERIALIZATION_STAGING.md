# Desktop Functional Management Slice C03e-DR — Candidate Publication Requester Rendezvous Start Validation / Policy / Registration Caller Composition Source Materialization Staging

## Status

`STAGING / SOURCE MATERIALIZATION`

C03e-DR materializes only the C03e-DQ-selected Agent-internal synchronous composition across already-closed DI current-registry validation, DP requester-aware policy-source resolution, DK dedicated policy admission, and DN private requester/rendezvous registration mutation.

It does not activate a runtime caller.

## Exact predecessor

Closed C03e-DQ:

- head: `fff800558b0ec0b49a7bfa7f5c444cb697b9657d`
- tree: `ee212cd5c272bedf22e38f169e98bfe460772c15`
- PR #240: `Status: CLOSED`, draft/open/unmerged
- rolling Drive evidence: `1028864` bytes / `c2c95a4f5ea6655f8638709fa9081b360601c9615620975eb659bf777f5c1258`

## Exact source anchors audited at predecessor

- parent start-intent module: `a0a68b096783ff4d50503ca5501bdc300718fd45`
- DI registry validation: `1c021bc95a3d674722bfd70559156fa75e07e578`
- DP requester-aware policy source: `123e8a773c2d3caa95958f1eb6275d95fdd59d6e`
- DK policy admission: `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`
- DN runtime owner: `68ba74e82cf703664b7ee090a10fc1c6cce1609d`
- in-memory requester/rendezvous provider: `d01cfbc37433f6099e216397b9bf243aa55c53bc`
- Agent Cargo manifest: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root Cargo.lock: `eeacde7ee776d35088f746a6d09f823f3391b82b`

## Materialized source surface

C03e-DR changes only:

1. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`
   - registers one new sibling composition child module under the already-private start-intent namespace.

2. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
   - materializes the selected composition function and bounded four-stage error.

3. this C03e-DR contract.

No `lib.rs`, manifest, lockfile, provider, DI, DP, DK, DN, wire, listener, runtime-bootstrap, or networking source is changed.

## Materialized composition function

The source materializes the selected effective Agent-internal function:

```rust
pub fn validate_authorize_and_register_requester_rendezvous_start<
    S: RequesterRendezvousStartPolicySource + ?Sized,
>(
    registry: &WorkspaceDeviceRegistry,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    intent: RequesterRendezvousStartIntent,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

Lexical `pub` remains capped by the existing private parent start-intent module. No crate-root re-export is added.

## Exact execution order

The function materializes exactly the DQ-selected order:

1. call existing DI `validate_current_requester_rendezvous_start_intent` by value;
2. call DP `evaluator_for_requester` only with `validated.requester_session()`;
3. move the unchanged validated carrier by value into existing DK `policy_authorize_requester_rendezvous_start`;
4. move the exact DK carrier by value into existing DN `register_policy_authorized_requester_rendezvous_start`.

Every `?` / `map_err` short-circuits before the next stage.

There is no speculative evaluation or mutation before prior-stage success.

## Ownership and provenance

The source preserves the closed ownership chain:

`RequesterRendezvousStartIntent`
→ DI by-value validation
→ `RegistryValidatedRequesterRendezvousStart`
→ temporary requester borrow for DP source resolution
→ DK by-value admission
→ `PolicyAuthorizedRequesterRendezvousStart`
→ DN by-value registration.

The composition source adds no:

- `into_parts`;
- raw field access;
- validated-carrier clone;
- policy-authorized-carrier clone;
- arbitrary carrier constructor;
- raw provider getter;
- mutable provider reference;
- direct DI-to-DN path;
- raw requester-session/target registration path.

The only base identity clones remain encapsulated by the already-closed DN registration boundary.

## Materialized fail-closed error

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum RequesterRendezvousStartCompositionError {
    RegistryValidation(RequesterRendezvousStartRegistryValidationError),
    PolicySource(RequesterRendezvousStartPolicySourceError),
    PolicyAuthorization(RequesterRendezvousStartPolicyAuthorizationError),
    Registration(RequesterRendezvousLifecycleError),
}
```

`Display` identifies the stage. `Error::source` preserves the exact nested error for each variant.

No wire/status/HTTP/PRWC/PRWM mapping is materialized.

## Fail-closed behavior

- DI failure prevents policy-source resolution, policy evaluation, and registration.
- DP source failure prevents DK admission and DN registration.
- DK denial prevents DN registration.
- DN registration failure is propagated once without retry, replacement, overwrite, or fallback.

No rollback layer is added because DI/DP/DK perform no requester/rendezvous provider mutation.

## Currentness semantics

DI remains point-in-time registry validation. The synchronous composition does not convert it into a lease, TTL, perpetual currentness, target transport readiness, live-owner authority, publisher authority, candidate-publication authority, or network reachability.

No second registry validation is added after policy evaluation.

## Policy-source semantics

DP remains an abstract requester-aware source. C03e-DR does not materialize:

- a concrete policy store;
- policy cache/map/schema;
- persistence;
- global/default evaluator;
- fallback evaluator;
- requester-to-policy storage;
- refresh/watch logic.

The exact DI-held authenticated requester session remains the only policy-selection identity input.

## Tests materialized

The composition module adds only production-safe tests that do not fabricate authenticated cryptographic/session authority:

- compile-time composition function signature shape with a signature-only policy source/evaluator;
- distinct composition error variants and nested `Error::source` provenance.

The signature-only evaluator returns `Deny` and is not used to execute the production composition with fabricated authenticated session state.

No synthetic authenticated session fixture is introduced.

## Explicitly absent

C03e-DR does not add or authorize:

- runtime caller wiring;
- concrete requester-aware policy-source implementation;
- policy persistence/cache/store;
- new provider mutator;
- provider exposure;
- retry/idempotency/replacement;
- retirement/cancellation/removal/TTL cleanup;
- target transport readiness;
- live-owner acquisition;
- candidate publication;
- reachability-owner mutation;
- wire/PRWC/PRWM command or response changes;
- listener/main/bootstrap activation;
- background task/thread;
- production networking;
- readiness publication;
- deployment/restart/recovery;
- merge.

## Verification gate

Before closure C03e-DR must prove:

- exact DQ merge base and exact three-path final diff;
- source anchors outside the authorized paths remain unchanged;
- no manifest/lock diff;
- canonical Rust locked graph, rustfmt, Clippy, workspace tests, and build PASS;
- Android native + application PASS if triggered by the source paths;
- immutable Drive audit raw byte-exact readback;
- two guarded reads of the closed-DQ rolling predecessor;
- append-only rolling evidence with exact predecessor prefix and one DR closure/classification/target-gate marker;
- PR remains draft/open/unmerged.

## Target gate

C03e-DR target gate: selected validation → requester-aware policy resolution → dedicated policy authorization → private registration composition is source-materialized but remains uncalled; runtime caller activation remains separately gated.

Any successor must begin with a fresh exact-head read-only audit.