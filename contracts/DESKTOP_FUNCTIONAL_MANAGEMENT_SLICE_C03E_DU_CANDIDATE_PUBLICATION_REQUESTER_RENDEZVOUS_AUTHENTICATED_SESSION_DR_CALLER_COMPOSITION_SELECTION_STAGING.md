# Private Remote Workspace — Phase 152 C03e-DU Authenticated-Session DR Caller Composition Selection

Status: `STAGING_SELECTION`

Gate: `C03E_DU_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHENTICATED_SESSION_DR_CALLER_COMPOSITION_SELECTED`

## Purpose

C03e-DU selects only the narrow future Agent-internal source composition that may connect the
already-closed C03e-DT authenticated-session-derived `RequesterRendezvousStartIntent` construction
helper to the already-closed C03e-DR validation -> requester-aware policy -> dedicated policy
admission -> private provider-registration composition.

C03e-DU is documentation-only. It does not materialize the caller, create a concrete requester-aware
policy source, change current-registry custody, wire a command, start a listener or task, activate the
remote process companion, publish readiness, perform networking, deploy, restart, recover, or merge.

## Exact predecessor

The sole predecessor is durably closed C03e-DT:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch:
  `phase-152-c03e-dt-candidate-publication-requester-rendezvous-authenticated-session-start-intent-construction-source-materialization-staging`
- predecessor head: `d13f130154c5cf443df998340bb4215983921b98`
- predecessor tree: `aabb1d7db399c9148937bafb79493da607a96b71`
- DT source blob: `fa77e79d4cf26498bf65954a28af3795a44eb203`
- DT contract blob: `5de49c13ab351fce669a4f047ddfe579d8bd5ed8`
- authoritative DT audit Drive ID: `1fnPn0dxGok6kOHhGtbuNUGGDfwLOX-WE`
- closed DT rolling evidence: `1044506` bytes /
  `de940ca894aad816cd53a44613ca910e9f26f3ad1f62f0175bcb8ee33e22c33c`

Any later materialization must remain an exact descendant of this closed head.

## Fresh read-only source-topology audit

### DT authenticated-session intent construction

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
at blob `fa77e79d4cf26498bf65954a28af3795a44eb203` now contains the uncalled DT helper:

```text
requester_rendezvous_start_intent(
    &self,
    target_device_id: DeviceId,
) -> RequesterRendezvousStartIntent
```

Requester identity is derived only from the exact `AuthenticatedDeviceSession` retained through the
private `BoundRemoteSession`; the caller cannot supply requester session identity independently.

The helper is synchronous and infallible. The target is moved by value and remains unvalidated intent.

### DR composition

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
at blob `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090` contains the existing uncalled DR composition:

```text
validate_authorize_and_register_requester_rendezvous_start(
    registry: &WorkspaceDeviceRegistry,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    intent: RequesterRendezvousStartIntent,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

Its internal order remains:

1. DI current-registry validation;
2. DP requester-aware policy-source resolution using the exact DI-held authenticated requester;
3. DK exact `Capability::RequesterRendezvousStart` admission;
4. DN one private provider registration mutation.

DR already preserves distinct fail-closed errors for registry validation, policy source, policy
authorization, and registration.

### Requester-aware policy-source boundary remains abstract

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`
at blob `123e8a773c2d3caa95958f1eb6275d95fdd59d6e` remains an interface only.

It requires evaluator resolution from the exact authenticated requester and forbids
process-global/default/substitute evaluator fallback.

No concrete requester-aware policy store/cache/map/schema is selected or materialized on the closed
DT head.

### Provider runtime custody remains private

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
at blob `68ba74e82cf703664b7ee090a10fc1c6cce1609d` retains the concrete in-memory provider privately.

Its registration mutation accepts only the by-value DK policy-authorized carrier. No raw provider
getter, provider extraction, retirement forwarding, publisher authorization forwarding, or generic
mutation API exists.

### Shared-current capability authority is not requester-policy proof

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
at blob `50356b47d3c5304b67edd424e9286beb028ace16` owns a current registry plus one
principal-agnostic `PolicyEvaluator` under a Tokio `RwLock`.

C03e-DU does not select that principal-agnostic evaluator as requester-bound policy proof and does not
change `SharedCurrentCapabilityAuthority` visibility or locking semantics.

A later runtime integration must still prove that the `WorkspaceDeviceRegistry` supplied to the DU/DR
path is the authoritative current registry at the operation point. DU does not select that later
integration mechanism.

### Production bootstrap remains outside scope

The Agent binary and Linux bootstrap remain unchanged from the closed lineage. The DT helper and DR
composition have no production caller. No remote companion, candidate-rendezvous runtime owner, or
concrete requester-aware policy source is wired into production bootstrap by DU.

## Selected future composition

C03e-DU selects one future free Agent-internal composition in the existing
`candidate_publication_requester_rendezvous_start_intent_composition` module.

Preferred semantic shape:

```text
fn validate_authorize_and_register_requester_rendezvous_start_for_authenticated_session<
    S: RequesterRendezvousStartPolicySource + ?Sized,
>(
    requester_owner: &AuthenticatedRemoteSessionRuntimeOwner,
    target_device_id: DeviceId,
    registry: &WorkspaceDeviceRegistry,
    policy_source: &S,
    rendezvous_runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

Exact lexical visibility and argument ordering may be adjusted only as required by existing Rust
module privacy and canonical lint/format rules. The semantic inputs and authority boundaries may not
be widened.

The selected operation is synchronous.

## Selected execution order

The future composition must perform exactly:

1. borrow the exact `AuthenticatedRemoteSessionRuntimeOwner`;
2. invoke only the existing DT helper with the by-value nominated logical target `DeviceId`;
3. receive exactly one existing `RequesterRendezvousStartIntent`;
4. pass that intent by value, unchanged, into the existing DR composition together with the exact
   supplied registry, requester-aware policy source, and private rendezvous runtime owner;
5. return DR's existing `Result<(), RequesterRendezvousStartCompositionError>` unchanged.

No intermediate identity decomposition or new authorization carrier is selected.

## Requester identity authority

The caller composition must not accept a raw `AuthenticatedDeviceSession` parameter.

Requester identity must remain:

```text
AuthenticatedRemoteSessionRuntimeOwner
  -> DT helper
  -> exact retained AuthenticatedDeviceSession
  -> RequesterRendezvousStartIntent
  -> DI validation
  -> DP requester-aware policy resolution
```

The future caller may not reconstruct requester identity from `DeviceId`, `SessionId`, `UserId`,
`WorkspaceId`, public identity, `TransportIdentity`, endpoint state, request ID, publisher state, or
candidate state.

## Target identity authority

The caller supplies one logical target `DeviceId` by value only.

That target remains nomination intent until existing DI current-registry validation proves exact
current target eligibility and same-workspace constraints.

Target identity does not become requester identity, transport identity, policy proof, or provider
registration authority before DI/DK succeed.

## Registry boundary

C03e-DU retains the existing DR `&WorkspaceDeviceRegistry` input unchanged.

DU does not:

- clone registry state;
- create a registry cache or snapshot owner;
- widen registry mutation access;
- select `SharedCurrentCapabilityAuthority` as the final runtime acquisition mechanism;
- claim that an arbitrary registry reference is current merely because it has the right Rust type.

The future runtime caller remains responsible for supplying the authoritative current registry at the
operation point. Selecting that acquisition/custody seam is separate.

## Policy-source boundary

C03e-DU retains the existing DP `RequesterRendezvousStartPolicySource` abstraction unchanged.

The future composition does not:

- implement the trait;
- select a policy store;
- use the principal-agnostic policy stored in `SharedCurrentCapabilityAuthority` as a substitute;
- fabricate a requester-bound wrapper from arbitrary `(session, evaluator)` inputs;
- fall back to a default/global evaluator.

Policy-source failure remains DR's existing fail-closed `PolicySource` class.

## Provider boundary

The future caller receives only `&mut CandidatePublicationRequesterRendezvousRuntimeOwner`.

It does not receive or expose the raw provider.

Only DR -> DN may reach the one existing private registration mutation after DI/DP/DK success.

No retry, replacement, idempotency, retirement, removal, publisher authorization, TTL or cleanup
operation is selected.

## Error boundary

DT construction is infallible, so DU selects no new error enum or error class.

The future caller must return the existing DR
`RequesterRendezvousStartCompositionError` unchanged.

The four existing fail-closed stage classes remain:

- registry validation;
- requester-aware policy source;
- dedicated policy authorization;
- provider registration.

No error is translated into success, retry, fallback, or substitute authorization.

## Concurrency and lifetime

C03e-DU selects a synchronous source composition only.

It selects no:

- future;
- `async fn`;
- task;
- thread;
- channel;
- mutex;
- new `RwLock`;
- queue;
- retry loop;
- detached work;
- background lifecycle.

No reference to the authenticated session, registry, evaluator, or provider escapes the call.

## API and dependency constraints

C03e-DU selects:

- no dependency additions;
- no Cargo manifest changes;
- no lockfile changes;
- no crate-root re-export;
- no public external API;
- no new owner type;
- no new provider type;
- no feature or toolchain change.

The existing private parent module continues to cap effective visibility of the DR/DU composition.

## Explicitly absent from C03e-DU

C03e-DU does not select or materialize:

- Rust source changes;
- concrete requester-aware policy backing;
- policy persistence/loading/mutation;
- global/default/fallback policy substitution;
- current-registry acquisition from `SharedCurrentCapabilityAuthority`;
- shared-authority visibility changes;
- runtime caller activation;
- wire command/opcode/frame/parser/dispatcher changes;
- PRWC/PRWM changes;
- requester/rendezvous provider construction changes;
- provider retirement/removal/publisher authorization changes;
- candidate-publication execution;
- candidate publication or reachability mutation;
- listener/task/thread activation;
- remote process companion activation;
- Agent `main.rs` wiring;
- readiness publication;
- production networking;
- STUN/ICE/TURN/relay activation;
- persistence/database/distributed coordination;
- systemd/packaging/firewall/NAT/route/DNS/TUN/TAP host mutation;
- deployment/restart/recovery;
- merge.

## Validation expectation

Because C03e-DU is documentation-only, canonical exact-final-head Rust validation must complete under
repository workflow policy. Android may legitimately not trigger for a contract-only diff and must not
be reported as PASS unless it actually runs and succeeds.

No exact-final-head workflow may remain pending or failing before durable closure.

## Closure evidence requirements

Durable closure requires:

1. exact final branch/head/tree read;
2. exact DT -> DU compare;
3. proof that the compare contains only this contract path;
4. canonical exact-final-head CI verdict;
5. fresh raw guarded read of the closed DT rolling predecessor;
6. immutable DU audit upload to the existing Drive project folder;
7. byte-exact immutable audit readback;
8. append-only rolling evidence update preserving every DT predecessor byte;
9. byte-exact rolling readback;
10. PR body transition to `Status: CLOSED` only after durable Drive evidence.

The PR remains draft/open/unmerged by project convention.

## Target gate

C03e-DU target gate:

> A future Agent-internal synchronous caller composition is selected to invoke only the existing DT
> authenticated-session-derived intent helper and then the existing DR validation/policy/registration
> composition. Source materialization remains separately gated. Concrete requester-aware policy
> backing, authoritative runtime registry acquisition, wire/runtime/listener/network activation,
> deployment and merge remain separately gated.
