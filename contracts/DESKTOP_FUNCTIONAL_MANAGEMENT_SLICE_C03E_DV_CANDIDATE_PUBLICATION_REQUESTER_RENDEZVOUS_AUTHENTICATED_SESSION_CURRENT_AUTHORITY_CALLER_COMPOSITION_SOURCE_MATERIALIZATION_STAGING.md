# Private Remote Workspace — Phase 152 C03e-DV Authenticated-Session Current-Authority Caller Composition Source Materialization

Status: `STAGING_SOURCE_MATERIALIZATION`

Gate: `C03E_DV_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHENTICATED_SESSION_CURRENT_AUTHORITY_CALLER_COMPOSITION_SOURCE_MATERIALIZED`

## Purpose

C03e-DV materializes only the C03e-DU-selected Agent-internal async caller-composition seam on the existing `AuthenticatedRemoteSessionRuntimeOwner`.

The source change may connect the already-closed C03e-DT authenticated-session-derived `RequesterRendezvousStartIntent` helper to the already-closed C03e-DR synchronous validation -> requester-aware policy -> dedicated authorization -> private registration composition while obtaining current registry state only through the existing `SharedCurrentCapabilityAuthority::with_current_authority(...)` operation.

C03e-DV does not materialize a concrete requester-aware policy source/store, add a wire command, activate a listener or remote companion, widen bootstrap inputs, publish readiness, perform production networking, deploy, restart, recover, or merge.

## Exact predecessor

The sole predecessor is durably closed C03e-DU:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch: `phase-152-c03e-du-candidate-publication-requester-rendezvous-authenticated-session-current-authority-caller-composition-selection-staging`
- predecessor head: `6b1f1558d7e9463b5101f4395906bfceb496081a`
- predecessor tree: `9278ad4059d004f93fb8ac1b2cb6a70d64379e45`
- DU contract blob: `bbdfd8ed07aca056e69b5183535e955d73abf8f2`
- DU authenticated-session runtime source blob: `fa77e79d4cf26498bf65954a28af3795a44eb203`
- authoritative DU audit Drive ID: `1FFVav5nI2ZRZV2mW8ZooOWi35k6XK7Ga`
- closed DU rolling evidence: `1050209` bytes / `d57ffe2e98412b7c3a950f6c11b183eb8df0f2841d1c4fa24d27dfd4ee65cba5`

Any DV mutation is invalid if it is not an exact descendant of this head or if it widens authority beyond the C03e-DU selection.

## Fresh read-only topology guard

Before DV mutation the exact closed-DU source topology was re-read.

### Existing DT authenticated-session start-intent helper

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

Closed-DU blob: `fa77e79d4cf26498bf65954a28af3795a44eb203`

The existing crate-internal helper:

```text
requester_rendezvous_start_intent(&self, target_device_id: DeviceId)
    -> RequesterRendezvousStartIntent
```

derives requester identity only from the exact authenticated application session retained by the existing `BoundRemoteSession`. The target `DeviceId` is consumed by value and remains non-authoritative intent.

### Existing current-authority seam

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`

Closed-DU blob: `50356b47d3c5304b67edd424e9286beb028ace16`

`SharedCurrentCapabilityAuthority<P>` retains one current `WorkspaceDeviceRegistry` plus one principal-agnostic `PolicyEvaluator` value beneath one Tokio `RwLock`.

Its existing `with_current_authority(...)`:

- awaits one read guard;
- invokes one synchronous `FnOnce` with borrowed `&WorkspaceDeviceRegistry` and `&P`;
- requires the closure result to be `Send` and unable to borrow those authority values;
- releases the guard when the synchronous closure returns;
- does not expose the raw lock guard.

### Existing requester-aware policy source

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

Closed-DU blob: `123e8a773c2d3caa95958f1eb6275d95fdd59d6e`

`RequesterRendezvousStartPolicySource` resolves policy from the exact authenticated requester session and fails closed with the existing bounded source errors. No process-global/default/substitute evaluator is allowed.

No concrete requester-aware policy source/store/cache/map/schema exists at the DV predecessor.

### Existing DR composition

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`

Closed-DU blob: `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`

The existing synchronous composition performs exactly:

1. DI current-registry validation;
2. DP requester-aware policy-source resolution from the exact validated requester session;
3. DK dedicated `Capability::RequesterRendezvousStart` authorization;
4. DN private requester/rendezvous provider registration.

It returns `RequesterRendezvousStartCompositionError` unchanged and performs no retry, fallback, replacement, or fabricated success.

### Existing private requester/rendezvous runtime owner

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

Closed-DU blob: `68ba74e82cf703664b7ee090a10fc1c6cce1609d`

The concrete provider remains private. Registration is crate-internal and accepts only the exact DK provenance carrier.

## Materialized source seam

C03e-DV permits one new crate-internal async method on `AuthenticatedRemoteSessionRuntimeOwner` with the following semantic shape:

```text
async fn register_requester_rendezvous_start<P, S>(
    &self,
    authority: &SharedCurrentCapabilityAuthority<P>,
    policy_source: &S,
    runtime_owner: &mut CandidatePublicationRequesterRendezvousRuntimeOwner,
    target_device_id: DeviceId,
) -> Result<(), RequesterRendezvousStartCompositionError>
```

Required generic authority constraints are limited to:

- `P: PolicyEvaluator + Send + Sync`, matching the existing shared-current authority implementation;
- `S: RequesterRendezvousStartPolicySource + Sync + ?Sized`, only so the borrowed requester-aware policy source may cross the existing `Send` synchronous closure boundary.

No semantic authority may be added through generic bounds.

## Required execution order

The materialized method must perform exactly:

1. consume `target_device_id` through the existing C03e-DT helper;
2. await exactly one `SharedCurrentCapabilityAuthority::with_current_authority(...)` invocation;
3. inside that synchronous closure use only the yielded current `WorkspaceDeviceRegistry`;
4. deliberately ignore the yielded principal-agnostic current-capability policy value for requester/rendezvous authorization;
5. delegate exactly once to `validate_authorize_and_register_requester_rendezvous_start(...)` with:
   - the current registry borrow;
   - the separately supplied requester-aware policy source;
   - the mutable private requester/rendezvous runtime owner;
   - the exact DT-produced intent;
6. return the existing DR `Result<(), RequesterRendezvousStartCompositionError>` unchanged.

No second registry lookup, no alternative composition path, and no raw authority extraction is allowed.

## Lock-scope rule

The shared-current read guard may span only the one synchronous DR call.

No `await`, network I/O, dispatcher execution, cancellation wait, task lifecycle operation, blocking storage operation, external process interaction, or provider background operation may occur while that guard is held.

The guard is released when the synchronous closure returns.

This creates no lease, TTL, reusable currentness token, perpetual-currentness guarantee, or currentness authority surviving the call.

## Requester-aware policy exclusion rule

The principal-agnostic `P` value yielded by `with_current_authority(...)` is not requester-bound policy proof.

C03e-DV explicitly rejects:

- using `P` for requester/rendezvous policy merely because it shares custody with the registry;
- process-global policy;
- default policy;
- fallback policy;
- another requester's evaluator;
- arbitrary requester/evaluator pairing;
- cached policy decision as reusable registration authority.

Concrete backing for `RequesterRendezvousStartPolicySource` remains separately gated.

## Identity rules preserved

- `AuthenticatedDeviceSession` remains logical requester identity.
- `DeviceId` remains logical target identity.
- `TransportIdentity` remains lower-transport certificate identity only.
- `SessionId` remains authentication/session correlation only.
- request IDs remain wire/message correlation only.
- endpoint, candidate, publisher, and live-owner state do not become requester identity.

The materialized method receives no raw requester identity parameter.

## Error semantics

C03e-DV introduces no new semantic error class.

The method returns `RequesterRendezvousStartCompositionError` unchanged, preserving:

- registry-validation failure;
- requester-aware policy-source failure;
- dedicated policy-authorization failure;
- provider-registration failure.

No retry, fallback, replacement, suppression, translation, or fabricated success is allowed.

## Ownership and visibility

The method:

- borrows the authenticated remote-session owner immutably;
- borrows the shared-current authority;
- borrows the requester-aware policy source;
- mutably borrows the existing requester/rendezvous runtime owner for exactly one DR attempt;
- consumes the nominated target `DeviceId` by value;
- returns no raw registry reference, lock guard, requester session, evaluator reference, provider reference, provenance carrier, or reusable authorization token.

The method remains crate-internal and remains uncalled by production/runtime/wire code after DV materialization.

## Source scope

The expected source mutation is limited to:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

plus this C03e-DV contract.

No parent-module registration is expected because the method is added inside an already compiled module.

## Dependency and lock guard

C03e-DV permits no dependency, feature, manifest, toolchain, or lockfile changes.

Closed-DU anchors must remain unchanged:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Explicitly absent

C03e-DV does not materialize or activate:

- concrete requester-aware policy source/store/cache/map/schema;
- policy persistence/loading/mutation;
- global/default/fallback/substitute policy;
- raw registry input or extraction;
- raw lock-guard exposure;
- raw requester/session/provider access;
- provider construction, retirement, publisher authorization, or lifecycle widening;
- candidate-publication execution;
- candidate construction/publication;
- reachability-authority mutation;
- wire command/opcode/frame/parser/dispatcher changes;
- PRWC/PRWM changes;
- remote capability dispatcher routing;
- target `DeviceId` producer wiring;
- `LinuxAgentRemoteProcessOperationInputs` widening;
- Agent `main.rs` wiring;
- listener/task/thread/process-companion activation;
- readiness publication;
- production networking;
- STUN/ICE/TURN/relay activation;
- persistence/database/distributed coordination;
- systemd/packaging/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## Validation expectation

Because DV changes Rust source, canonical closure requires exact-final-head validation of both:

- PRW Rust Validation: locked graph, rustfmt, Clippy, workspace tests, workspace build;
- Android validation, including native adapter and Android application terminal verdict if triggered by the source-changing diff.

Ancillary workflow skips remain skips and must not be promoted to PASS.

No exact-final-head workflow may remain pending or failing at closure.

## Closure gate

C03e-DV may close only if:

1. branch lineage is an exact descendant of closed DU;
2. the final diff contains only this contract plus the narrowly selected authenticated-session runtime source mutation;
3. the source method preserves exact DU authority, identity, lock-scope, error, and ownership constraints;
4. dependency/lock anchors are unchanged;
5. canonical exact-final-head CI is fully terminal and successful for the source-changing checkpoint;
6. immutable and rolling Drive evidence are written and read back consistently;
7. the PR remains draft/open/unmerged;
8. concrete requester-aware policy backing, wire/runtime/bootstrap/network activation, deployment, and merge remain separately gated.
