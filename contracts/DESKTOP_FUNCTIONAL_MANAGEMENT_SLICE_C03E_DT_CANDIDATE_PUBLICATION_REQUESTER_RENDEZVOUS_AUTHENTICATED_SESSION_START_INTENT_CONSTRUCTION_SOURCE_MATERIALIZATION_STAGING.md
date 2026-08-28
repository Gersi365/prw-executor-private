# Private Remote Workspace — Phase 152 C03e-DT Authenticated-Session Requester/Rendezvous Start-Intent Construction Source Materialization

Status: `STAGING_SOURCE_MATERIALIZATION`

Gate: `C03E_DT_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHENTICATED_SESSION_START_INTENT_CONSTRUCTION_SOURCE_MATERIALIZED`

## Purpose

C03e-DT materializes only the C03e-DS-selected Agent-internal helper that constructs the existing
`RequesterRendezvousStartIntent` from one already-authenticated remote-session runtime owner and one
caller-nominated logical target `DeviceId`.

This checkpoint does not invoke the C03e-DR validation/policy/registration composition, create a
concrete requester-aware policy source, mutate the requester/rendezvous provider, expose a wire
operation, start a listener, activate the remote process companion, publish readiness, perform
production networking, deploy, restart, recover, or merge.

## Exact predecessor

The sole predecessor is durably closed C03e-DS:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- branch:
  `phase-152-c03e-ds-candidate-publication-requester-rendezvous-authenticated-session-start-intent-construction-selection-staging`
- head: `1bc20166495283a9ac64d306a83b6a9b7a6ed363`
- tree: `399f2e29c9c6fc273c44f3388d199b5836984ecb`
- contract blob: `c9a8d9f9f6dae630802e2eb113201f8b3a71e753`
- authoritative immutable audit Drive ID: `1TRKFoIbotLkGkpAY7y1ml2zaTmudR5LV`
- closed DS rolling evidence: `1038864` bytes /
  `fc4fc201491710a44c116a50a7c6e8426412ce5068697aa099659d0ca44c8508`

C03e-DT must remain an exact descendant of that closed head.

## Fresh source-topology audit

Before source mutation, exact closed DS source was re-read.

### Authenticated remote-session owner

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

Pre-DT blob:
`6dab5f083b4835db969d9c5cf5e4673616f7bdf4`

The existing `AuthenticatedRemoteSessionRuntimeOwner` owns:

- the already-authenticated remote peer;
- one private `RemoteSessionCapabilityRuntimeOwner`;
- through that owner, the retained `BoundRemoteSession`.

The same source already derives logical device identity from:

```text
self.capability_owner.bound_session.session().device_id()
```

Therefore the child module already has direct private access to the exact authenticated application
session without adding a raw getter or decomposing either owner.

### Existing typed intent

The existing crate-internal `RequesterRendezvousStartIntent` owns:

- one `AuthenticatedDeviceSession`;
- one requester-nominated target `DeviceId`.

Its constructor performs ownership composition only and grants no registry, policy, provider,
reachability, transport-readiness, lease, or networking authority.

### Identity boundary

C03e-DT preserves:

- `AuthenticatedDeviceSession` as logical requester identity;
- `DeviceId` as logical device identity;
- `TransportIdentity` as lower-transport certificate identity only;
- `SessionId` as session/authentication correlation only;
- request IDs as message correlation only.

Transport identity, endpoint identity, candidate identity, publisher identity, request ID, session ID,
or live-owner state cannot substitute for the authenticated requester session.

## Materialized helper

C03e-DT adds one operation on the existing `AuthenticatedRemoteSessionRuntimeOwner` with semantic
shape:

```text
pub(crate) fn requester_rendezvous_start_intent(
    &self,
    target_device_id: DeviceId,
) -> RequesterRendezvousStartIntent
```

The source is materialized in the existing authenticated-owner module. No new module or owner type is
created.

## Exact requester provenance

The implementation derives requester identity only through:

```text
AuthenticatedRemoteSessionRuntimeOwner
  -> private RemoteSessionCapabilityRuntimeOwner
  -> private retained BoundRemoteSession
  -> BoundRemoteSession::session()
  -> exact AuthenticatedDeviceSession
```

It clones that exact authenticated session only inside this operation-specific helper because the
existing intent requires owned session custody.

The clone is ownership/storage adaptation only. It does not:

- authenticate again;
- extend or renew a lease;
- establish current-registry eligibility;
- evaluate policy;
- create provider-registration authority;
- cache currentness;
- create a reusable authorization token.

No caller-supplied raw authenticated session parameter exists.

## Exact target provenance

The helper consumes one caller-nominated logical target `DeviceId` by value.

The target remains non-authoritative intent until the existing DI current-registry validation path
succeeds later.

C03e-DT does not validate target enrollment, target current membership, exact target preservation,
same-workspace relationship, or requester currentness.

## Output authority boundary

The sole result is the existing `RequesterRendezvousStartIntent`.

Possession proves only:

- requester identity was sourced from the already-authenticated runtime owner;
- one logical target was nominated.

Possession does not prove:

- current-registry eligibility;
- same-workspace relationship;
- requester policy authorization;
- requester/rendezvous provider registration;
- candidate-publication authority;
- reachability;
- transport readiness;
- live-owner/freshness state;
- lease/TTL currentness;
- networking success.

## Visibility

The helper is `pub(crate)` only.

This permits a later separately gated Agent-internal caller without exposing the helper as external
crate API. No crate-root re-export is added.

A scoped `dead_code` allowance records that source is intentionally materialized before caller
activation. The allowance does not alter runtime behavior or authority.

## Test surface

C03e-DT adds a focused compile-time signature test proving that the helper requires:

- `&AuthenticatedRemoteSessionRuntimeOwner`;
- one by-value `DeviceId`;
- output `RequesterRendezvousStartIntent`.

The test does not create a network peer, perform I/O, evaluate policy, mutate a provider, or activate a
runtime.

## No raw identity widening

C03e-DT adds no:

- full-session getter;
- raw `BoundRemoteSession` getter;
- raw `RemoteSessionCapabilityRuntimeOwner` getter;
- `into_parts` decomposition;
- generic identity extraction;
- constructor from raw `DeviceId`/`SessionId`/`UserId`/`WorkspaceId`/public identity/
  `TransportIdentity` components.

The typed intent is built directly inside the authenticated-owner boundary.

## No authorization or provider widening

The helper does not:

- call DI registry validation;
- resolve DP requester-aware policy;
- call DK policy admission;
- call DN provider registration;
- invoke the full DR composition;
- read or mutate requester/rendezvous provider state;
- select fallback/default/global policy;
- cache a policy decision;
- create a registration-ready/provider-ready token.

## Concurrency and lifetime

The helper is synchronous and bounded to an immutable borrow of the authenticated runtime owner.

It creates no:

- future;
- task;
- thread;
- channel;
- mutex;
- `RwLock`;
- queue;
- retry;
- async retention;
- background lifecycle.

No borrowed reference into `BoundRemoteSession` survives the helper return.

## Source scope

Initial DT source commit:

- commit: `119132f57e996efdbd71198a23fa857e6e2c5036`
- source blob: `fa77e79d4cf26498bf65954a28af3795a44eb203`
- predecessor -> source-commit compare: exact DS merge base, ahead `1`, behind `0`
- exactly one changed source path
- source diff: `+44/-1`

The one deletion is import restructuring required to include the focused test dependencies; it does
not remove behavior.

Final DT scope after this contract must remain limited to:

1. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
2. this DT staging contract.

No manifest, lockfile, workflow, provider, policy-source, DI/DP/DK/DN/DR, wire, listener, bootstrap,
main, networking, deployment, database, authentication-cutover, or host-mutation path is authorized.

## Dependency and lock guard

Pre-DT exact anchors:

- `crates/prw-agent/Cargo.toml`:
  `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`:
  `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `Cargo.lock` remains the closed lineage anchor:
  `cce9ca06190a196661ab38d54a747893e26af95f`

C03e-DT selects no dependency, feature, toolchain, or lock change.

## Explicitly absent

C03e-DT does not materialize:

- concrete requester-aware policy source/store/cache/map/schema;
- policy persistence/loading/mutation;
- default/fallback/global policy substitution;
- C03e-DR composition caller activation;
- requester/rendezvous provider construction or registration changes;
- provider retirement/removal/publisher authorization changes;
- candidate-publication execution;
- candidate construction/publication;
- reachability-authority mutation;
- wire command/opcode/frame/parser/dispatcher changes;
- PRWC/PRWM changes;
- remote capability dispatcher changes;
- listener/task/thread/runtime activation;
- remote process companion activation;
- Agent `main.rs` wiring;
- readiness publication;
- production networking;
- STUN/ICE/TURN/relay activation;
- persistence/database/distributed coordination;
- systemd/packaging/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## Validation requirement

Durable DT closure requires canonical exact-final-head validation.

For a source-changing checkpoint, Rust validation must reach terminal success for:

- locked dependency graph;
- rustfmt;
- Clippy;
- workspace tests;
- workspace build.

Android validation, if triggered by repository workflow policy, must reach a terminal successful
verdict before closure. A non-trigger must not be reported as PASS.

No exact-final-head workflow may remain pending or failing.

## Durable evidence requirement

Closure requires:

1. final branch/head/tree read;
2. exact DS -> DT compare;
3. exact final changed-path proof;
4. canonical final-head workflow verdicts;
5. fresh guarded raw read of closed DS rolling evidence;
6. immutable DT audit upload to the existing Drive project folder;
7. byte-exact immutable audit readback;
8. append-only rolling evidence update preserving the complete DS predecessor prefix;
9. byte-exact rolling readback;
10. PR body transition to `Status: CLOSED` only after durable evidence.

The PR remains draft/open/unmerged by project convention.

## Target gate

C03e-DT target gate:

> The C03e-DS-selected authenticated-session-derived requester/rendezvous start-intent helper is
> source-materialized on the existing authenticated remote-session owner. The helper remains uncalled.
> Concrete requester-aware policy backing, C03e-DR composition caller activation,
> wire/runtime/listener/network activation, deployment and merge remain separately gated.
