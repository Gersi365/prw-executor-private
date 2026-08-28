# Desktop Functional Management Slice C03e-DQ — Candidate Publication Requester Rendezvous Start Validation / Policy / Registration Caller Composition Selection Staging

## Status

`STAGING / SELECTION ONLY`

C03e-DQ selects the smallest future Agent-internal composition boundary that may connect the already-closed requester/rendezvous start prerequisites without activating any runtime caller.

Exact predecessor is closed C03e-DP:

- head: `58418696b51675055188012f235bae8896fc73c7`
- tree: `a7f1057d01c037e93e19af10b07ea00a2f440ee6`
- PR #239: `Status: CLOSED`, draft/open/unmerged
- rolling Drive evidence: `1025749` bytes / `7c0102f88c2d06c05829bf45897f6ac21d50361a59423c679d09a2bd1fe1f037`

This checkpoint changes documentation only. It does not materialize the composition source selected below.

## Existing closed prerequisites

The selected future composition must use the existing closed boundaries exactly as they are rather than reproducing their logic.

### DI — current-registry validation

Existing input:

`RequesterRendezvousStartIntent`

Existing consuming validator:

```rust
validate_current_requester_rendezvous_start_intent(
    registry: &WorkspaceDeviceRegistry,
    intent: RequesterRendezvousStartIntent,
) -> Result<
    RegistryValidatedRequesterRendezvousStart,
    RequesterRendezvousStartRegistryValidationError,
>
```

DI proves only point-in-time registry eligibility for the exact authenticated requester session and exact nominated logical target `DeviceId`.

### DP — requester-aware policy source

Existing source interface:

```rust
pub trait RequesterRendezvousStartPolicySource {
    type Evaluator: PolicyEvaluator + ?Sized;

    fn evaluator_for_requester<'a>(
        &'a self,
        requester: &AuthenticatedDeviceSession,
    ) -> Result<&'a Self::Evaluator, RequesterRendezvousStartPolicySourceError>;
}
```

The effective visibility is capped by the private requester/rendezvous start-intent parent module. The source must resolve policy for the exact authenticated requester and fail closed rather than substitute a global/default/other-requester evaluator.

### DK — dedicated policy admission

Existing consuming admission:

```rust
policy_authorize_requester_rendezvous_start(
    registry_validated: RegistryValidatedRequesterRendezvousStart,
    evaluator: &E,
) -> Result<
    PolicyAuthorizedRequesterRendezvousStart,
    RequesterRendezvousStartPolicyAuthorizationError,
>
```

It evaluates exactly `Capability::RequesterRendezvousStart`. Only `Decision::Allow` creates the typed policy-authorized carrier.

### DN — private provider-registration mutation owner

Existing crate-internal consuming method:

```rust
CandidatePublicationRequesterRendezvousRuntimeOwner::
    register_policy_authorized_requester_rendezvous_start(
        &mut self,
        authorized: PolicyAuthorizedRequesterRendezvousStart,
    ) -> Result<(), RequesterRendezvousLifecycleError>
```

The provider remains private and raw provider access is not exposed.

## Selected future composition boundary

C03e-DQ selects one new sibling child module under the existing private requester/rendezvous start-intent namespace rather than adding orchestration to DI, DP, DK, DN, `lib.rs`, or the provider.

Preferred future source path:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`

The parent start-intent module may register this child with the same path/module pattern already used for registry validation, policy admission, and policy source.

No crate-root re-export is selected.

## Selected future function shape

The future source checkpoint should materialize an effective Agent-internal function equivalent to:

```rust
fn validate_authorize_and_register_requester_rendezvous_start<
    S: RequesterRendezvousStartPolicySource + ?Sized,
>(
    registry: &WorkspaceDeviceRegistry,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    intent: RequesterRendezvousStartIntent,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

Lexical visibility may be `pub` if required by the private parent-module visibility cap and Clippy; effective visibility must remain Agent-internal. The source checkpoint must not add a crate-root re-export.

## Mandatory execution order

The future function must execute exactly these semantic stages in this order:

1. **DI current-registry validation**
   - consume the raw `RequesterRendezvousStartIntent`;
   - produce `RegistryValidatedRequesterRendezvousStart` only on full DI success.

2. **DP requester-aware policy resolution**
   - call the supplied `RequesterRendezvousStartPolicySource` only with `validated.requester_session()`;
   - obtain the evaluator authoritative for that exact authenticated requester;
   - no global/default/substitute evaluator fallback.

3. **DK dedicated policy authorization**
   - move the unchanged `RegistryValidatedRequesterRendezvousStart` by value into the existing DK admission function;
   - evaluate exactly `Capability::RequesterRendezvousStart`;
   - only `Allow` may produce `PolicyAuthorizedRequesterRendezvousStart`.

4. **DN private registration mutation**
   - move the exact `PolicyAuthorizedRequesterRendezvousStart` by value into the existing DN runtime-owner method;
   - perform only the existing single `register_current` mutation encapsulated by DN.

No stage may be reordered, skipped, retried, substituted, or invoked speculatively.

## Borrow and ownership selection

The future composition must preserve the existing ownership chain rather than introduce decomposition APIs.

Required ownership sequence:

`RequesterRendezvousStartIntent`
→ by-value DI validation
→ `RegistryValidatedRequesterRendezvousStart`
→ temporary borrowed requester only for DP source resolution
→ by-value DK admission
→ `PolicyAuthorizedRequesterRendezvousStart`
→ by-value DN registration.

The evaluator borrow returned by DP is source-owned, not requester-owned. This is the reason the composition may end the temporary requester borrow and then move the unchanged DI carrier into DK.

Forbidden shortcuts:

- no `into_parts` on DI or DK carriers;
- no public/raw-field extraction;
- no clone of DI/DK provenance carriers;
- no arbitrary constructor for validated or policy-authorized carriers;
- no provider getter or mutable provider reference;
- no direct DI-to-DN bypass;
- no raw `(AuthenticatedDeviceSession, DeviceId)` registration composition outside DN.

DN remains the only selected boundary where base requester session and target `DeviceId` may be cloned to satisfy provider-owned storage.

## Selected fail-closed composition error

The future composition should preserve phase-specific failure provenance with a bounded error equivalent to:

```rust
#[non_exhaustive]
enum RequesterRendezvousStartCompositionError {
    RegistryValidation(RequesterRendezvousStartRegistryValidationError),
    PolicySource(RequesterRendezvousStartPolicySourceError),
    PolicyAuthorization(RequesterRendezvousStartPolicyAuthorizationError),
    Registration(RequesterRendezvousLifecycleError),
}
```

The source checkpoint may implement `Display`, `Error::source`, and explicit conversions or `map_err` plumbing as needed, but it must preserve these four semantic classes distinctly.

No wire/status/HTTP/PRWC/PRWM error mapping is selected here.

## Short-circuit and mutation semantics

Mandatory fail-closed behavior:

- registry-validation failure: policy source, policy evaluation and registration are not called;
- policy-source failure: DK admission and registration are not called;
- policy denial: registration is not called;
- registration failure: propagate the existing `RequesterRendezvousLifecycleError` without retry/fallback.

The existing DN/provider contract already treats duplicate identity and capacity exhaustion as fail-before-mutation outcomes. DQ selects no rollback layer, because no earlier stage performs requester/rendezvous provider mutation.

No automatic retry, idempotency token, replacement, overwrite, second registration attempt, or fallback registration path is selected.

## Currentness semantics

DI remains point-in-time current-registry validation only.

The future synchronous composition does not convert DI validation into:

- a lease;
- TTL authorization;
- perpetual registry currentness;
- target transport readiness;
- live-owner authority;
- publisher authority;
- candidate-publication authority.

DQ selects no second registry validation after policy admission. Any later requirement for renewed currentness, TTL/lease semantics, or concurrent-registry coordination must be justified in a separate checkpoint rather than silently added to the composition.

## Policy-source semantics retained

The supplied `RequesterRendezvousStartPolicySource` is an abstract authoritative source boundary only.

DQ does not select or implement:

- concrete policy store ownership;
- cache/map/schema layout;
- persistence;
- global/default evaluator;
- evaluator fallback;
- requester-to-policy table;
- policy refresh/watch logic;
- principal reconstruction from `DeviceId`, `TransportIdentity`, endpoint, `SessionId`, candidate/request IDs, or publisher traffic.

The exact authenticated requester session remains the policy-selection input.

## Identity separation retained

- `DeviceId` / authenticated PRW session identity are logical identity.
- `TransportIdentity` is lower-transport certificate identity only.
- `CandidateId` is plan-scoped correlation only.
- `ConnectivityEndpoint` is transient endpoint/configuration only.
- `SessionId` is auth/session correlation only.
- PRWC/PRWM request IDs are message correlation only.

None of those correlation/transport values may substitute for the authenticated requester session used by DP policy selection.

## Runtime and provider boundaries

DQ does not select a runtime caller.

The future composition function may exist as dead/staged Agent-internal source and remain uncalled.

Explicitly not selected:

- main/bootstrap wiring;
- listener registration;
- wire request handler;
- PRWC/PRWM command mapping;
- background task/thread;
- synchronization owner;
- production network path;
- readiness publication;
- provider exposure;
- provider capacity construction/configuration changes;
- provider retirement/remove/publisher-authorization calls;
- candidate-publication execution;
- reachability-owner mutation.

## Minimal future source surface

If a fresh successor audit confirms topology remains unchanged, the expected source-materialization checkpoint should need only:

1. parent start-intent module registration;
2. one new composition source module;
3. one successor contract.

No changes are expected in:

- `prw-policy`;
- `prw-registry`;
- `prw-session`;
- `prw-remote-bridge` provider source;
- DI registry-validation source;
- DP policy-source source;
- DK policy-admission source;
- DN runtime-owner source;
- manifests/lockfiles;
- wire/runtime/listener/networking code.

Any need to widen this source surface must trigger a fresh audit rather than opportunistic scope expansion.

## Verification requirements for a future source checkpoint

A source-materialization successor must prove at minimum:

- exact predecessor head/tree and exact diff scope;
- no manifest/lock drift;
- compile-time function/error signature shape;
- strict stage ordering through focused fakes/spies where production-safe fixtures permit;
- policy source receives the exact DI requester session;
- source failure and policy denial prevent DN registration;
- only policy allow reaches DN registration;
- no direct provider import/call from the composition module except through the DN runtime owner type/error surface;
- canonical Rust validation FULL PASS;
- Android validation terminal PASS if triggered by path filters;
- immutable Drive audit plus append-only rolling proof before PR closure.

Tests must not fabricate authenticated cryptographic/session authority merely to exercise production mutation. Signature- and collaborator-order tests are preferred when authoritative session fixtures are unavailable.

## Explicit exclusions

C03e-DQ does not materialize or authorize:

- composition source;
- concrete requester-aware policy-source implementation;
- policy store/cache/map/schema/persistence;
- caller wiring;
- raw provider access;
- additional provider mutators;
- retry/idempotency/replacement;
- retirement/cancellation/removal/TTL cleanup;
- target transport readiness;
- live-owner acquisition;
- candidate publication;
- reachability-owner mutation;
- wire/PRWC/PRWM changes;
- listener/runtime/main/bootstrap activation;
- production networking;
- deployment/restart/recovery;
- merge.

## Target gate

C03e-DQ target gate: validation-policy-registration composition semantics selected; source materialization and caller activation remain separately gated.

A successor must begin with a fresh exact-head read-only audit before materializing the selected composition source.