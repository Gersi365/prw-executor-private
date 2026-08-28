# Phase 152 C03e-DO — Candidate Publication Requester/Rendezvous Start Requester-Bound Policy Source Composition Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DO_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_REQUESTER_BOUND_POLICY_SOURCE_COMPOSITION_SELECTED`

## Exact predecessor

C03e-DO is rooted only at durably closed C03e-DN:

- branch: `phase-152-c03e-dn-candidate-publication-requester-rendezvous-start-provider-registration-mutation-source-materialization-staging`
- head: `20ee8f414dd290535a1911a30be04619ad1a97f2`
- tree: `2791ed9cdb90b923d5e2dec5240e86d95bf1c2c5`
- PR #237: `Status: CLOSED`, draft/open/unmerged
- closed-DN rolling evidence: `1017179` bytes / SHA-256 `4195d6971b412d7bbe7994f4123b9d1c7ba703033e8cca0774ff22ef54109a05`

This checkpoint is selection-only. It does not materialize a source trait, a concrete policy store, caller wiring, a provider call, a wire command, a listener, networking, deployment, or merge.

## Why a separate requester-bound policy-source boundary is required

C03e-DJ/DK deliberately separated requester/rendezvous-start policy admission from current-registry validation and provider mutation. The already-materialized DK admission function accepts one `RegistryValidatedRequesterRendezvousStart` and one borrowed `PolicyEvaluator`, evaluates exactly `Capability::RequesterRendezvousStart`, and only `Decision::Allow` creates `PolicyAuthorizedRequesterRendezvousStart`.

The DK contract is explicit that its evaluator must already have been selected and bound by the caller to the same authenticated requester principal represented by the registry-validated carrier. DK itself does not perform that binding.

The current `prw-policy::PolicyEvaluator` interface is principal-agnostic:

```rust
pub trait PolicyEvaluator {
    fn evaluate(&self, capability: Capability) -> Decision;
}
```

The interface carries no `AuthenticatedDeviceSession`, `SessionId`, `WorkspaceId`, `UserId`, or `DeviceId`. Therefore an arbitrary `&E where E: PolicyEvaluator` is not, by type alone, proof that the evaluator belongs to the requester represented by the DK input.

The existing remote capability bridge is useful precedent but does not close this gap for the requester/rendezvous path. `CapabilityBridge::new(registry, policy)` semantically describes `policy` as an already-selected principal policy, but its type still accepts only `&P`. Likewise `SharedCurrentCapabilityAuthority<P>` owns one coherent registry/policy state and supports current per-operation reads, but it is constructed independently of any particular authenticated requester and therefore is not itself same-requester policy-binding proof.

C03e-DO consequently rejects direct composition of:

`validate_current_requester_rendezvous_start_intent(...)`
→ arbitrary/global `&PolicyEvaluator`
→ `policy_authorize_requester_rendezvous_start(...)`
→ `register_policy_authorized_requester_rendezvous_start(...)`.

Doing so would convert an undocumented caller assertion into provider mutation authority.

## Current exact source anchors

The read-only prerequisite audit is anchored at exact closed-DN source:

- `crates/prw-policy/src/lib.rs`: `3745024b5b222fcb36244222fad3c9c05a59cece`
- `crates/prw-session/src/lib.rs`: `0b0b6624df93ebcf3efae632d94dfc337ee67761`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`: `1c021bc95a3d674722bfd70559156fa75e07e578`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`: `b0db3f0ee8e8f5144f128faeff6fc98fa01ca1a8`
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`: `68ba74e82cf703664b7ee090a10fc1c6cce1609d`
- `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`: `50356b47d3c5304b67edd424e9286beb028ace16`
- `crates/prw-remote-bridge/src/lib.rs`: `7b1c5c62339983da6ae2556f73510d7582ec0c5b`

No existing `PolicySource`, `evaluate_for`, requester-aware evaluator selector, or equivalent typed same-requester policy-source surface was found in the audited source.

## Selected boundary

C03e-DO selects one future **Agent-internal requester-aware policy source** between DI registry validation and DK policy admission.

The source is not a second policy engine. It is a selection boundary whose sole job is to resolve the already-existing `PolicyEvaluator` that is authoritative for the exact authenticated requester principal represented by the DI carrier.

A future source shape should remain operation-specific and crate-internal. The selected semantic shape is equivalent to:

```rust
trait RequesterRendezvousStartPolicySource {
    type Evaluator: PolicyEvaluator + ?Sized;
    type Error;

    fn evaluator_for_requester<'a>(
        &'a self,
        requester: &AuthenticatedDeviceSession,
    ) -> Result<&'a Self::Evaluator, Self::Error>;
}
```

This is a semantic shape, not source materialization in C03e-DO. Exact naming, visibility syntax, error enum layout, generic bounds, module placement, and concrete source implementation remain for the next source checkpoint after an exact-head audit.

The key lifetime property is intentional: the returned evaluator borrow is tied to the policy source, not to the borrowed requester session. This permits the caller to borrow `validated.requester_session()` only for policy selection, end that borrow, then move the exact non-Clone-required `RegistryValidatedRequesterRendezvousStart` by value into the existing DK admission function without cloning or decomposing the provenance carrier.

## Required future composition order

A later composition may proceed only in this order:

1. consume one DD `RequesterRendezvousStartIntent` into DI `validate_current_requester_rendezvous_start_intent(...)` against current registry state;
2. on DI success, borrow only `validated.requester_session()` for requester-aware policy-source selection;
3. require successful source resolution of the evaluator authoritative for that exact authenticated requester principal;
4. end the requester-session selection borrow;
5. move the exact same `RegistryValidatedRequesterRendezvousStart` by value into existing DK `policy_authorize_requester_rendezvous_start(...)` with the resolved evaluator;
6. only on DK `Allow`, move the exact resulting `PolicyAuthorizedRequesterRendezvousStart` by value into existing DN `register_policy_authorized_requester_rendezvous_start(...)`;
7. propagate the existing bounded provider lifecycle result without retry or replacement.

No later step is entered after a failure in an earlier step.

## Requester identity used for policy selection

The policy-source selector receives the exact server-held `AuthenticatedDeviceSession` that survived DI current-registry validation.

That session already carries:

- `SessionId`;
- `WorkspaceId`;
- `UserId`;
- logical requester `DeviceId`;
- canonical authenticated public identity.

The policy source may use the identity dimensions its authoritative policy model requires, but it must resolve policy from this exact authenticated requester session. It must not substitute:

- target `DeviceId`;
- `TransportIdentity`;
- endpoint/address;
- candidate identity;
- candidate-publication request ID;
- PRWC/PRWM request ID;
- publisher traffic;
- provider record;
- raw caller-supplied workspace/user/device values.

Target identity remains a DI registry-validation concern. Policy-source selection is requester-principal selection, not target discovery or transport routing.

## Fail-closed source resolution

Policy-source resolution must fail closed when the correct requester evaluator cannot be determined.

The later source checkpoint must preserve at least these semantic outcomes:

- exact requester policy resolved;
- requester policy unavailable;
- requester policy indeterminate/ambiguous.

Unavailable or indeterminate resolution must not:

- fall back to a process-global evaluator;
- fall back to `BoundedLocalReadPolicy` or `BoundedLocalManagementPolicy`;
- substitute another requester's evaluator;
- default to `Decision::Allow`;
- retry against alternate policy stores;
- create a DK policy-authorized carrier;
- call the DN provider mutation;
- create a retry token, lease, cache entry, or side effect.

Whether unavailable and indeterminate remain one bounded source error or two distinct variants is left to the source-materialization audit, provided both fail before DK admission.

## No fabricated bound-policy token

C03e-DO explicitly rejects a wrapper that can be constructed merely from:

```text
(&AuthenticatedDeviceSession, &PolicyEvaluator)
```

or equivalent raw arguments and then labels the result `RequesterBound...`.

Such a constructor would not validate that the evaluator actually came from the authoritative policy source for that requester and would manufacture provenance rather than preserve it.

If a later implementation introduces a typed bound-policy handle, its construction must remain private to an authoritative requester-aware policy source after successful resolution. C03e-DO does not require such a handle because the minimal source interface can return a borrow tied to the source and the caller can immediately invoke DK.

## Existing DK authority remains authoritative

C03e-DO does not replace or duplicate DK admission.

The resolved evaluator must still be passed to:

```rust
policy_authorize_requester_rendezvous_start(
    validated,
    evaluator,
)
```

DK remains the only selected constructor path for `PolicyAuthorizedRequesterRendezvousStart`, and it must continue to evaluate exactly:

```rust
Capability::RequesterRendezvousStart
```

exactly once for that admission attempt.

The requester-aware policy source must not create `PolicyAuthorizedRequesterRendezvousStart` directly, must not expose or reconstruct its private nested field, and must not turn policy selection into provider authority.

## Existing DN provider mutation remains authoritative

C03e-DO does not change DN.

Only the DK output may reach:

```rust
CandidatePublicationRequesterRendezvousRuntimeOwner::
    register_policy_authorized_requester_rendezvous_start(...)
```

The DN owner remains the only selected mutation owner. Its provider remains private. The operation remains crate-internal, consumes the DK carrier by value, derives only the exact authenticated requester session plus DI-validated target logical `DeviceId` inside the mutation boundary, and makes exactly one `register_current` attempt.

No policy source may call the provider directly or expose a raw identity-registration overload.

## Current-registry and current-policy coherence

DI already proves requester and target current-registry eligibility only at the point of validation. C03e-DO does not convert that result into a lease or perpetual currentness.

A future caller composition must preserve current policy selection after DI success. It must not cache an evaluator result across unrelated requester sessions or silently reuse an evaluator selected for an earlier principal.

C03e-DO does not yet select synchronization, lock ownership, transaction semantics, a combined registry/policy lock, policy refresh notifications, or multi-worker sharing. The existing `SharedCurrentCapabilityAuthority<P>` is architectural evidence that coherent current registry/policy operation scopes are possible, but it is not automatically reused because it does not encode requester-aware policy selection.

Whether a later concrete policy source is stored beside current registry state, behind a shared lock, in a principal-indexed map, or behind another bounded authority remains separately gated.

## Error preservation

The future caller composition must keep the existing semantic failure boundaries distinguishable:

- DI registry-validation failure;
- requester-aware policy-source resolution failure;
- DK policy denial;
- DN provider lifecycle failure.

C03e-DO does not select wire/status/Error-frame mappings for any of these outcomes. It does not collapse source-unavailable into DK `Denied`, because absence/indeterminacy of authoritative policy is not evidence that the evaluator made a deny decision.

No failure path may fabricate provider success.

## Visibility and ownership posture

The future requester-aware policy-source surface should be Agent-internal and as narrow as possible.

Selected ownership rules:

- source itself may be shared or borrowed, but C03e-DO selects no concrete synchronization primitive;
- source resolution borrows the DI requester's authenticated session only;
- returned evaluator is borrowed from the authoritative source and is not cloned by the composition;
- DI carrier is not cloned and is not decomposed;
- DK carrier is not cloned and is not decomposed;
- no raw provider reference is returned;
- no evaluator is stored inside requester/rendezvous provider records;
- no policy decision is cached as reusable provider-registration authority.

## Relationship to existing remote capability bridge

The current remote capability bridge validates the current authenticated session and current transport binding before evaluating the already-supplied policy. That order remains useful precedent for fail-closed authorization.

C03e-DO deliberately strengthens requester/rendezvous composition relative to the existing bridge's untyped principal-policy selection seam: requester-aware policy selection must be explicit before DK admission rather than represented only by a comment on an arbitrary `&P`.

This checkpoint does not modify the existing bridge, `SharedCurrentCapabilityAuthority`, remote session runtime, production remote endpoint, or local Linux command policy paths.

## Explicitly not selected or materialized

C03e-DO does not select or materialize:

- a concrete policy database/store;
- policy schema or serialization;
- a process-global allow policy;
- a principal-policy cache;
- a registry+policy combined lock for this requester/rendezvous path;
- policy refresh/watch infrastructure;
- evaluator cloning;
- DI/DK `into_parts` or raw provenance extraction;
- target-specific policy semantics beyond the existing dedicated capability;
- provider mutation changes;
- provider getter/extraction/conversion;
- retry/idempotency/replacement registration;
- retirement/cancellation/removal/TTL cleanup;
- candidate-publication execution;
- reachability-owner lookup or mutation;
- wire opcode/frame/parser/dispatcher changes;
- PRWC/PRWM mapping;
- listener/accept loop;
- Agent `main.rs` or production bootstrap wiring;
- runtime task/thread activation;
- persistence/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Expected next bounded checkpoint

After durable C03e-DO closure, perform a fresh exact-head audit and materialize only the minimal requester-aware policy-source interface plus focused compile-time/fail-closed tests if the audited source topology permits it without introducing a concrete policy store or caller/provider wiring.

A concrete policy-source implementation and the validation → requester-policy-selection → DK admission → DN registration caller composition remain separately gated unless the next audit proves they can be materialized without inventing policy authority or widening runtime behavior.

C03e-DO therefore closes only architecture selection for the missing same-requester policy-source seam.