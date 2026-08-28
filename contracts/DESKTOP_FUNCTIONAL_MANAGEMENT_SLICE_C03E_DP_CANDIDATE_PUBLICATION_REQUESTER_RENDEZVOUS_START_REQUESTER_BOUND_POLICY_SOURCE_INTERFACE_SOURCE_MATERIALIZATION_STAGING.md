# Phase 152 C03e-DP — Candidate Publication Requester/Rendezvous Start Requester-Bound Policy Source Interface Source Materialization — STAGING

## Status

`STAGED SOURCE MATERIALIZATION`

## Target gate

`C03E_DP_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_REQUESTER_BOUND_POLICY_SOURCE_INTERFACE_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-DP is rooted only at durably closed C03e-DO:

- branch: `phase-152-c03e-do-candidate-publication-requester-rendezvous-start-requester-bound-policy-source-composition-selection-staging`
- head: `f57431a697aba410b42ed7738f03286209703690`
- tree: `9348389d51750e77d6d32e71319b2271c3af5817`
- PR #238 remains `Status: CLOSED`, draft/open/unmerged
- closed-DO rolling evidence: `1021480` bytes / SHA-256 `e62e6b94cdefc66fb593c9ad6da2beb3d737c5f56dbb98de5095ec4a68309dfe`

## Materialized source boundary

C03e-DP materializes only the C03e-DO-selected Agent-internal requester-aware policy-source interface.

New source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

The parent crate-internal start-intent module registers this source as a crate-internal child module. No `lib.rs` export is added because the parent itself already caps visibility at crate scope.

The effective interface is:

```rust
pub(crate) trait RequesterRendezvousStartPolicySource {
    type Evaluator: PolicyEvaluator + ?Sized;

    fn evaluator_for_requester<'a>(
        &'a self,
        requester: &AuthenticatedDeviceSession,
    ) -> Result<&'a Self::Evaluator, RequesterRendezvousStartPolicySourceError>;
}
```

This source interface does not evaluate `Capability::RequesterRendezvousStart`; existing C03e-DK remains the only policy-admission gate.

## Stable fail-closed source failures

C03e-DP materializes exactly two bounded source-resolution failures:

- `Unavailable`: no authoritative requester policy is currently available;
- `Indeterminate`: authoritative requester policy cannot be resolved deterministically.

These are source-resolution failures, not policy `Decision::Deny` outcomes. They are intentionally distinct from the existing DK `RequesterRendezvousStartPolicyAuthorizationError::Denied`.

No fallback evaluator is selected or materialized.

## Requester identity input

The interface accepts only a borrowed `AuthenticatedDeviceSession` as requester identity input.

A later caller must supply the exact server-held authenticated requester session retained by the DI `RegistryValidatedRequesterRendezvousStart` carrier after current-registry validation.

The interface accepts no raw:

- workspace ID;
- user ID;
- device ID;
- session ID;
- public-key bytes;
- target identity;
- transport identity;
- endpoint;
- candidate ID;
- request ID;
- publisher traffic;
- provider record.

Therefore DP does not create an alternate raw identity-selection lane.

## Lifetime and ownership preservation

The evaluator borrow is tied to the policy-source borrow, not the requester-session borrow.

This is required so a future caller may:

1. hold an owned DI `RegistryValidatedRequesterRendezvousStart`;
2. borrow `validated.requester_session()` only while resolving the evaluator;
3. end that requester borrow;
4. move the unchanged DI carrier by value into existing DK with the source-owned evaluator borrow.

DP adds no `Clone`, `Copy`, `into_parts`, raw-field extraction, or reconstructed provenance surface to DI or DK carriers.

A compile-time signature test uses independent higher-ranked source/requester lifetimes to lock this property without constructing or fabricating an authenticated session fixture.

## Test-only implementation is not production authority

The source file contains one `#[cfg(test)]` signature-only implementation solely to type-check the selected lifetime shape. It is not compiled as production policy authority, performs no runtime composition, and is never invoked with an authenticated session.

The production source remains only a trait plus bounded error type. No concrete policy store or evaluator mapping is materialized in DP.

## Existing DK and DN boundaries remain unchanged

C03e-DP does not modify:

- `candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`;
- `candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`;
- `candidate_publication_requester_rendezvous_runtime.rs`;
- `prw-policy`;
- `prw-session`;
- requester/rendezvous provider implementation.

Existing DK still consumes the DI carrier by value and evaluates exactly `Capability::RequesterRendezvousStart`.

Existing DN still consumes only `PolicyAuthorizedRequesterRendezvousStart` by value and performs the one selected private-provider `register_current` attempt.

DP cannot create either carrier and cannot call the provider.

## No fabricated bound-policy token

No `RequesterBound...` wrapper is added. No constructor accepts arbitrary `(AuthenticatedDeviceSession, PolicyEvaluator)` inputs. The only production surface is the requester-aware source-resolution trait.

If a later concrete implementation cannot prove authoritative requester mapping, it must return the bounded source error rather than fabricate binding provenance.

## Expected exact scope

C03e-DP is bounded to exactly three paths:

1. new requester-aware policy-source interface source;
2. existing start-intent parent module registration only;
3. this source-materialization contract.

No manifest or lockfile change is expected because `prw-agent` already depends on both `prw-policy` and `prw-session`.

## Explicitly absent

C03e-DP does not materialize:

- a concrete requester policy source/store/map/cache;
- policy persistence/schema/serialization;
- process-global/default policy fallback;
- policy refresh/watch infrastructure;
- registry+policy synchronization topology;
- DI -> source -> DK -> DN caller composition;
- provider mutation changes or raw provider access;
- evaluator cloning;
- provenance decomposition;
- retry/idempotency/replacement registration;
- retirement/cancellation/removal/TTL cleanup;
- candidate-publication execution;
- reachability-owner lookup/mutation;
- wire opcode/frame/parser/dispatcher changes;
- PRWC/PRWM mapping;
- listener/accept-loop or Agent `main.rs` wiring;
- runtime task/thread activation;
- persistence/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Validation requirement

The exact final DP head must pass canonical Rust validation, including locked dependency graph, rustfmt, Clippy, workspace tests, and workspace build. Because Agent Rust source changes may trigger Android/native validation, any automatically triggered Android workflow must reach terminal PASS before durable closure.

Any formatter/Clippy correction must remain inside the exact DP three-path boundary and may not widen authority semantics.

## Safe successor

After durable C03e-DP closure, perform a fresh exact-head audit before selecting or materializing a concrete requester-aware policy source implementation.

The next checkpoint must not automatically combine concrete policy-source storage with DI -> source -> DK -> DN caller composition, wire exposure, runtime/listener activation, networking, deployment, or merge.