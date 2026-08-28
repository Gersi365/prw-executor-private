# Private Remote Workspace — Phase 152 C03e-DU Authenticated-Session Current-Authority Caller Composition Selection

Status: `STAGING_SELECTION`

Gate: `C03E_DU_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHENTICATED_SESSION_CURRENT_AUTHORITY_CALLER_COMPOSITION_SELECTED`

## Purpose

C03e-DU selects only the narrow future Agent-internal caller-composition seam that may connect the
already-closed C03e-DT authenticated-session-derived requester/rendezvous start-intent helper to the
already-closed C03e-DR validation -> requester-aware policy -> dedicated authorization -> private
registration composition while sourcing current registry state from the existing
`SharedCurrentCapabilityAuthority`.

This checkpoint is documentation-only. It does not materialize the selected caller, create a concrete
requester-aware policy store/source, expose a wire operation, activate a listener or remote companion,
wire the Agent binary, publish readiness, perform production networking, deploy, restart, recover, or
merge.

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

Any DU mutation is invalid if it is not an exact descendant of this head or if its final scope is not
strictly documentation-only.

## Fresh read-only source-topology audit

The exact closed DT head was re-read before DU mutation.

### C03e-DT authenticated-session intent source

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
at blob `fa77e79d4cf26498bf65954a28af3795a44eb203` now contains the closed DT helper:

```text
pub(crate) fn requester_rendezvous_start_intent(
    &self,
    target_device_id: DeviceId,
) -> RequesterRendezvousStartIntent
```

It derives requester identity only from:

```text
self
  -> private RemoteSessionCapabilityRuntimeOwner
  -> retained BoundRemoteSession
  -> BoundRemoteSession::session()
  -> exact AuthenticatedDeviceSession
```

The exact authenticated session is cloned only for owned intent custody. The target `DeviceId` is
consumed by value and remains non-authoritative intent.

DT remains uncalled on the closed predecessor.

### Existing shared-current authority

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
at blob `50356b47d3c5304b67edd424e9286beb028ace16` owns one coherent current state:

- `WorkspaceDeviceRegistry`;
- one principal-agnostic `PolicyEvaluator` value `P`;
- both retained beneath a Tokio `RwLock` shared through one `Arc`.

Its existing `with_current_authority(...)` method:

- acquires one async read guard;
- passes borrowed `&WorkspaceDeviceRegistry` and `&P` into one synchronous closure;
- does not expose the lock guard;
- requires the closure result not to borrow the registry/policy values;
- releases the guard when the synchronous closure returns.

This is already the selected current-registry authority for remote-session protected operations.

### Existing principal-agnostic policy is not requester-bound policy proof

`prw-policy::PolicyEvaluator` is principal-agnostic. The `P` stored in
`SharedCurrentCapabilityAuthority<P>` is already used by the general bound-session capability path,
but C03e-DO/DP explicitly require requester/rendezvous-start policy to be resolved from the exact
authenticated requester through `RequesterRendezvousStartPolicySource`.

Therefore DU explicitly rejects using the `P` value yielded by `with_current_authority(...)` as the
requester-aware policy source merely because it shares custody with the registry.

The future DU caller must ignore that `P` value for requester/rendezvous-start policy admission.

### Existing requester-aware policy-source boundary

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`
at blob `123e8a773c2d3caa95958f1eb6275d95fdd59d6e` requires:

```text
RequesterRendezvousStartPolicySource::evaluator_for_requester(
    &self,
    &AuthenticatedDeviceSession,
)
```

The evaluator borrow is source-owned and source resolution fails closed with bounded `Unavailable` or
`Indeterminate` classifications. No process-global/default/substitute evaluator fallback is allowed.

No concrete requester-aware source/store/cache/map/schema exists on the closed DT head.

### Existing C03e-DR composition

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
at blob `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090` owns the already-selected synchronous order:

1. DI validates the by-value intent against current registry state;
2. DP resolves policy from the exact DI-held authenticated requester session;
3. DK consumes the unchanged DI provenance and evaluates exactly
   `Capability::RequesterRendezvousStart`;
4. DN consumes the exact DK provenance and performs one private provider registration mutation.

It returns one bounded `RequesterRendezvousStartCompositionError` preserving four distinct failure
classes and short-circuits before later stages.

DR remains uncalled on the closed DT head.

### Existing requester/rendezvous runtime owner

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
at blob `68ba74e82cf703664b7ee090a10fc1c6cce1609d` privately retains the concrete in-memory provider.

Its selected registration method is crate-internal and accepts only the exact consumed DK
`PolicyAuthorizedRequesterRendezvousStart` provenance. The raw provider remains inaccessible.

### Existing production bootstrap remains outside this path

`crates/prw-agent/src/main.rs` remains blob
`db6b8028c6df100a961a0fb5818347bea2fdc5c1` and still invokes only
`prw_agent::linux_bootstrap::run()`.

`crates/prw-agent/src/linux_bootstrap.rs` remains blob
`8d569a432fa5d8706cc1458a771f40dedd501f72`. Its injected remote-process operation owns
`SharedCurrentCapabilityAuthority<P>` for the existing remote-session path but no
`CandidatePublicationRequesterRendezvousRuntimeOwner` and no concrete
`RequesterRendezvousStartPolicySource`.

DU selects no change to either bootstrap file.

## Selected future caller seam

DU selects one future crate-internal async operation on the existing
`AuthenticatedRemoteSessionRuntimeOwner`.

Preferred semantic shape:

```text
async fn register_requester_rendezvous_start<P, S>(
    &self,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    target_device_id: DeviceId,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

where `S` implements the existing `RequesterRendezvousStartPolicySource` and `P` remains the existing
principal-agnostic current-capability policy type. Exact Rust visibility/bounds may be adjusted only as
required by module privacy, `Send`/`Sync`, and canonical linting while preserving this authority shape.

The selected caller itself remains uncalled after future source materialization until another gate
explicitly activates a runtime/wire producer.

## Selected execution order

The future caller must perform exactly this order:

1. consume the nominated target `DeviceId` into the existing DT helper, deriving one
   `RequesterRendezvousStartIntent` only from the authenticated session retained by `self`;
2. await exactly one existing `SharedCurrentCapabilityAuthority::with_current_authority(...)` read;
3. inside that one synchronous closure, use only the yielded `&WorkspaceDeviceRegistry` as current
   registry authority;
4. deliberately ignore the yielded principal-agnostic `&P` for requester/rendezvous-start policy;
5. delegate exactly once to existing C03e-DR using:
   - that current registry borrow;
   - the separately supplied requester-aware `policy_source`;
   - the supplied mutable requester/rendezvous runtime owner;
   - the exact DT-produced intent;
6. return the existing DR result unchanged.

No second registry lookup/composition path is selected.

## Current-registry authority rule

DU explicitly rejects a separate caller-supplied raw `&WorkspaceDeviceRegistry` parameter.

Reason:

- the established remote-session runtime already owns current registry state inside
  `SharedCurrentCapabilityAuthority`;
- accepting an unrelated registry borrow would permit divergence from the current registry used by
  the remote-session authority graph;
- currentness must therefore come from the same shared-current owner.

The selected DU operation acquires one read guard only through the existing bounded
`with_current_authority(...)` seam.

## Lock-scope rule

The existing authority read guard may remain held only for the one synchronous DR call inside the
existing synchronous closure.

DU selects no `await`, network I/O, dispatcher execution, cancellation wait, task lifecycle work,
blocking storage operation, or external process interaction while that read guard is held.

The guard is released immediately when the synchronous DR call returns.

This temporary lock scope does not create a lease, TTL, cached currentness token, perpetual
currentness guarantee, or authority that survives the call. DI provenance remains a bounded
point-in-time validation fact after the call returns.

## Requester-aware policy rule

The policy source passed to DU must remain the sole source used by DP.

DU explicitly rejects:

- substituting the `P` value stored in `SharedCurrentCapabilityAuthority`;
- process-global policy;
- default policy;
- fallback policy;
- another requester's evaluator;
- constructing a requester-bound wrapper from arbitrary `(requester, evaluator)` inputs;
- caching a prior evaluator decision as reusable registration authority.

Concrete backing for `RequesterRendezvousStartPolicySource` remains separately gated.

## Identity rules preserved

- `AuthenticatedDeviceSession` remains logical requester identity.
- `DeviceId` remains logical target identity.
- `TransportIdentity` remains lower-transport certificate identity only.
- `SessionId` remains authentication/session correlation only.
- request IDs remain wire/message correlation only.
- endpoint/candidate/publisher/live-owner state cannot substitute for requester identity.

The caller receives no raw requester identity parameter.

## Error semantics

DU selects no new semantic error class.

The future caller returns the existing `RequesterRendezvousStartCompositionError` unchanged:

- registry-validation failure;
- requester-aware policy-source failure;
- dedicated policy-authorization failure;
- provider-registration failure.

Tokio read-lock acquisition has no selected application-level failure mapping. No retry, fallback,
replacement, suppression, translation, or fabricated success is selected.

## Mutation and ownership semantics

The future caller:

- borrows the authenticated runtime owner immutably;
- consumes one target `DeviceId` by value;
- creates exactly one non-authoritative DT intent;
- borrows the shared-current authority;
- borrows the requester-aware policy source;
- mutably borrows the existing requester/rendezvous runtime owner for exactly one DR registration
  attempt;
- returns no provider reference, lock guard, registry reference, evaluator reference, provenance
  carrier, or reusable authorization token.

No `Clone`/`Copy` widening of DI/DK provenance is selected.

## Concurrency and runtime boundary

The future caller is async only because it awaits the existing shared-current read guard.

DU selects no:

- task spawn;
- thread spawn;
- channel;
- background worker;
- persistent queue;
- retry loop;
- cancellation source;
- readiness publication;
- remote listener activation;
- process-companion activation.

The existing Tokio runtime topology is not changed by this selection.

## No production activation

DU is not a wire/runtime activation checkpoint.

It does not select:

- a new PRWC/PRWM command/opcode/frame;
- remote capability dispatcher routing;
- a producer of the nominated target `DeviceId`;
- concrete requester-aware policy backing;
- construction of `CandidatePublicationRequesterRendezvousRuntimeOwner` in bootstrap;
- addition of that owner or policy source to `LinuxAgentRemoteProcessOperationInputs`;
- `run_with_remote_process_companion(...)` invocation from `main.rs`;
- listener/readiness/network activation.

A future source-materialization checkpoint may only materialize this selected helper and focused tests.
Actual runtime/wire/bootstrap activation remains separately gated after that.

## Dependency and API constraints

DU selects:

- no dependency additions;
- no Cargo manifest changes;
- no lockfile changes;
- no feature changes;
- no crate-root public re-export;
- no external crate API;
- no raw provider/registry/session/lock access surface.

Closed-DT dependency anchors remain:

- Agent Cargo: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Explicitly absent from C03e-DU

C03e-DU does not materialize or activate:

- Rust source changes;
- concrete requester-aware policy source/store/cache/map/schema;
- policy persistence/loading/mutation;
- fallback/default/global policy substitution;
- raw current-registry input or extraction;
- raw lock-guard exposure;
- provider construction/extraction;
- provider retirement/removal/publisher authorization changes;
- candidate-publication execution;
- candidate construction/publication;
- reachability-authority mutation;
- wire command/opcode/frame/parser/dispatcher changes;
- PRWC/PRWM changes;
- remote capability dispatcher changes;
- listener/task/thread/process-companion activation;
- Agent `main.rs` wiring;
- readiness publication;
- production networking;
- STUN/ICE/TURN/relay activation;
- persistence/database/distributed coordination;
- systemd/packaging/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## Validation expectation

Because DU is documentation-only, canonical validation must be evaluated on the exact final head.

Rust workflow must reach terminal success according to repository CI policy. Android validation may
legitimately not trigger for a documentation-only one-file diff; a non-trigger must not be reported as
PASS.

No exact-final-head workflow may remain pending or failing before durable closure.

## Closure evidence requirements

Durable DU closure requires:

1. exact final branch/head/tree read;
2. exact DT -> DU compare;
3. proof that the compare contains only this contract path;
4. canonical exact-final-head CI verdict;
5. fresh guarded raw read of the closed DT rolling predecessor;
6. immutable DU audit upload to the existing Drive project folder;
7. byte-exact immutable audit readback;
8. append-only rolling evidence update preserving every DT predecessor byte;
9. byte-exact rolling readback;
10. PR body transition to `Status: CLOSED` only after durable Drive evidence.

The PR remains draft/open/unmerged by project convention.

## Target gate

C03e-DU target gate:

> The future authenticated-session requester/rendezvous caller is selected to derive its intent only
> through closed DT, obtain current registry only through `SharedCurrentCapabilityAuthority`, ignore
> that owner's principal-agnostic policy for requester/rendezvous authorization, and delegate once to
> closed DR using the separately supplied requester-aware policy source and private runtime owner.
> Source materialization remains separately gated. Concrete requester-aware policy backing,
> wire/runtime/bootstrap/network activation, deployment and merge remain separately gated.
