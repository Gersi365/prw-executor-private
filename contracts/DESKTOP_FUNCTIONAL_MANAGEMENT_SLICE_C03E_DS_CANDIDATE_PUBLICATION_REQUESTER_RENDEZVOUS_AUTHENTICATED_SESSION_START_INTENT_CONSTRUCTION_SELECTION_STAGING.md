# Private Remote Workspace — Phase 152 C03e-DS Authenticated-Session Requester/Rendezvous Start-Intent Construction Selection

Status: `STAGING_SELECTION`

Gate: `C03E_DS_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHENTICATED_SESSION_START_INTENT_CONSTRUCTION_SELECTED`

## Purpose

C03e-DS selects only the narrow future Agent-internal construction seam that may create one
`RequesterRendezvousStartIntent` from an already-authenticated remote-session runtime owner and one
caller-nominated logical target `DeviceId`.

This checkpoint is documentation-only. It does not materialize the seam, invoke the C03e-DR
validation/policy/registration composition, create a concrete requester-aware policy source, expose a
wire operation, start a listener, activate a remote process companion, publish readiness, perform
networking, deploy, restart, recover, or merge.

## Exact predecessor

The sole predecessor is durably closed C03e-DR:

- repository: `Gersi365/prw-executor-private`
- repository ID: `1334911207`
- predecessor branch:
  `phase-152-c03e-dr-candidate-publication-requester-rendezvous-start-validation-policy-registration-caller-composition-source-materialization-staging`
- predecessor head: `17498398b2da854b2946158d89ef428674d5e0a1`
- predecessor tree: `dd2cf46576be32710305c232b862179c80b2a3a4`
- predecessor composition source blob:
  `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`
- predecessor rolling evidence: `1033115` bytes /
  `0eede86a091b48ac6c408464942916d56ff4696b1eb081d3b92d049d2ae566b3`

C03e-DS must remain rooted exactly at that predecessor. A later mutation is invalid if the
predecessor head changes or if the final compare is not an exact descendant with the intended scope.

## Read-only source-topology audit

The exact C03e-DR head establishes the following relevant topology.

### Existing unvalidated intent

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`
at blob `4a5495b5f01e732ad458cd6603f50dc76ad0688f` owns:

- one `AuthenticatedDeviceSession`;
- one requester-nominated logical target `DeviceId`.

`RequesterRendezvousStartIntent::new(...)` performs ownership composition only. It performs no
registry validation, policy evaluation, provider mutation, I/O, synchronization, networking, or
readiness publication.

The intent is neither `Copy` nor `Clone`.

### Existing authenticated remote-session owner

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
retains one already-authenticated live remote application session under
`AuthenticatedRemoteSessionRuntimeOwner`.

That owner already reaches the exact authenticated logical session only through the private retained
`RemoteSessionCapabilityRuntimeOwner` -> `BoundRemoteSession` chain.

Its current public/super-module identity helper exposes only the logical `DeviceId`; it does not
expose the complete `AuthenticatedDeviceSession`.

### Existing bound-session identity source

`crates/prw-remote-bridge/src/remote_session_binding.rs`
at blob `fcaa4960c7ec150d317e8aea197b5e936f3529a4` provides
`BoundRemoteSession::session() -> &AuthenticatedDeviceSession`.

That value is the already-authenticated logical application-session identity carried by the
verifier-owned remote session lease.

`BoundRemoteSession` separately retains `TransportIdentity`. Transport identity is not the logical
requester identity and must not be substituted for the authenticated application session.

### Existing C03e-DR composition

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
at blob `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090` is materialized but uncalled.

It requires:

1. `&WorkspaceDeviceRegistry`;
2. `&RequesterRendezvousStartPolicySource`;
3. `&mut CandidatePublicationRequesterRendezvousRuntimeOwner`;
4. one by-value `RequesterRendezvousStartIntent`.

It then performs only the already-selected DI -> DP -> DK -> DN order.

C03e-DS does not invoke that function.

### Existing requester-aware policy-source boundary

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`
at blob `123e8a773c2d3caa95958f1eb6275d95fdd59d6e` requires policy resolution from the exact
`AuthenticatedDeviceSession`.

It explicitly forbids process-global/default/substitute evaluator fallback.

No concrete requester-aware policy source/store/cache/map/schema is materialized on the C03e-DR
head.

### Existing shared-current capability authority is not the selected DS source

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
at blob `50356b47d3c5304b67edd424e9286beb028ace16` owns current registry plus one
principal-agnostic `PolicyEvaluator` under a Tokio `RwLock`.

That type remains authoritative for the already-existing capability-request path, but it is not
selected by C03e-DS as proof of requester-aware policy binding for requester/rendezvous start.

Reasons:

- the `PolicyEvaluator` interface itself is principal-agnostic;
- C03e-DO/DP already forbid treating an arbitrary/global evaluator as same-requester binding proof;
- `with_current_authority(...)` intentionally confines registry/policy borrows to the lock guard;
- C03e-DP requires any returned requester evaluator borrow to be source-owned;
- C03e-DS therefore must not bypass DP by extracting or substituting the shared-current policy.

No change to `SharedCurrentCapabilityAuthority` is selected.

### Existing Agent binary/bootstrap remains unactivated for this path

`crates/prw-agent/src/main.rs`
at blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1` still calls
`prw_agent::linux_bootstrap::run()`.

It does not invoke `run_with_remote_process_companion(...)` and does not construct candidate
publication requester/rendezvous runtime custody.

`crates/prw-agent/src/linux_bootstrap.rs`
at blob `8d569a432fa5d8706cc1458a771f40dedd501f72` provides injected remote-process composition
surfaces, but its current remote operation inputs contain no
`CandidatePublicationRequesterRendezvousRuntimeOwner` and no concrete
`RequesterRendezvousStartPolicySource`.

C03e-DS selects no change to either file.

## Selected future seam

C03e-DS selects one future crate-internal operation on the existing
`AuthenticatedRemoteSessionRuntimeOwner` whose only purpose is to construct the existing
`RequesterRendezvousStartIntent`.

Preferred semantic shape:

```text
fn requester_rendezvous_start_intent(
    &self,
    target_device_id: DeviceId,
) -> RequesterRendezvousStartIntent
```

The exact method name and visibility syntax may be adjusted only as required by Rust module privacy
and linting, while preserving the authority boundary selected here.

## Selected input provenance

The future seam has exactly two semantic inputs.

### Requester identity

Requester identity must come only from:

```text
self
  -> retained RemoteSessionCapabilityRuntimeOwner
  -> retained BoundRemoteSession
  -> BoundRemoteSession::session()
  -> AuthenticatedDeviceSession
```

The future implementation may clone that exact already-authenticated session value only inside this
operation-specific construction boundary because `RequesterRendezvousStartIntent` owns its session.

The clone is storage/ownership adaptation only. It creates no new authentication fact, capability,
lease, policy decision, registry-currentness claim, registration authority, or reusable token.

No caller-supplied raw `AuthenticatedDeviceSession` parameter is selected.

### Target identity

The future seam consumes one caller-nominated logical target `DeviceId` by value.

That target remains unvalidated intent until the existing C03e-DR composition later reaches DI
current-registry validation.

The nominated target is not transport identity, endpoint identity, candidate identity, request ID,
session ID, live-owner state, or policy authority.

## Selected output

The sole output is the existing `RequesterRendezvousStartIntent`.

Possession of this value proves only:

- the requester session value was derived from the already-authenticated remote-session owner;
- one logical target `DeviceId` was nominated.

It does not prove:

- requester currentness;
- target enrollment/current membership;
- same-workspace relationship;
- requester policy authorization;
- requester/rendezvous provider registration;
- candidate-publication authority;
- reachability;
- live-owner/freshness state;
- transport readiness;
- lease/TTL currentness;
- successful networking.

Those facts remain in their existing separately gated boundaries.

## No raw identity widening

C03e-DS explicitly rejects:

- a new public/full-session getter on `AuthenticatedRemoteSessionRuntimeOwner`;
- a raw `BoundRemoteSession` getter;
- a raw `RemoteSessionCapabilityRuntimeOwner` getter;
- a caller-supplied `AuthenticatedDeviceSession` parameter;
- reconstruction from `DeviceId`, `SessionId`, `UserId`, `WorkspaceId`, public identity, or
  `TransportIdentity`;
- an `into_parts` decomposition of the authenticated runtime owner;
- a new generic identity-extraction API.

The future seam should construct the typed intent directly inside the owner boundary.

## No authorization widening

The future seam must not:

- call DI registry validation;
- resolve DP policy;
- call DK policy admission;
- call DN registration;
- call the full DR composition;
- read or mutate the requester/rendezvous provider;
- introduce a default/fallback evaluator;
- cache a policy decision;
- create a registration-ready or provider-ready token.

The typed intent remains non-authoritative.

## Failure semantics

Construction itself is selected as infallible because it only:

- borrows the existing authenticated session already retained by the owner;
- clones that exact session for owned intent custody;
- moves the nominated target `DeviceId` into the existing intent constructor.

No new fallible lookup, policy source, lock acquisition, I/O, network operation, persistence, or
provider mutation is selected.

If later source materialization discovers a concrete Rust ownership/privacy contradiction to this
shape, that contradiction must be recorded and the checkpoint must stop rather than widening the
API.

## Concurrency and lifetime semantics

C03e-DS selects no task, future, channel, mutex, `RwLock`, thread, async retention, queue, retry, or
background lifecycle.

The future helper is synchronous and bounded to the borrow of the existing
`AuthenticatedRemoteSessionRuntimeOwner`.

It does not retain references into `BoundRemoteSession` after return. The returned intent owns only
the cloned authenticated session value and consumed target `DeviceId`.

## Identity rules preserved

- `AuthenticatedDeviceSession` remains logical requester identity.
- `DeviceId` remains logical device identity.
- `TransportIdentity` remains lower-transport certificate identity only.
- `SessionId` remains authentication/session correlation only.
- request IDs remain message correlation only.
- endpoints/candidates/addresses do not become requester identity or authorization.
- requester identity and target identity remain distinct.

## Dependency and API constraints

C03e-DS selects:

- no dependency additions;
- no Cargo manifest changes;
- no lockfile changes;
- no crate-root public re-export;
- no broader `pub` API than required by the existing crate-internal call topology;
- no feature changes.

The existing child start-intent module remains crate-internal through its private parent registration
in `prw-agent/src/lib.rs`.

## Explicitly absent from C03e-DS

C03e-DS does not select or materialize:

- Rust source changes;
- a concrete requester-aware policy source;
- policy persistence/load/mutation;
- requester policy store/cache/map/schema;
- policy fallback/default/global substitution;
- C03e-DR caller activation;
- requester/rendezvous provider registration;
- provider retirement/removal/publisher authorization changes;
- candidate-publication execution;
- candidate construction/publication;
- reachability-authority mutation;
- wire command/opcode/frame/parser/dispatcher changes;
- PRWC/PRWM changes;
- remote capability dispatcher changes;
- listener/task/thread/runtime activation;
- `run_with_remote_process_companion(...)` activation;
- Agent `main.rs` wiring;
- readiness publication;
- production networking;
- STUN/ICE/TURN/relay activation;
- persistence/database/distributed coordination;
- systemd/packaging/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## Validation expectation

Because C03e-DS is documentation-only, canonical validation must be evaluated on the exact final
head.

Expected gates:

```text
cargo check/fmt/clippy/test/build workflow policy as defined by repository CI
```

Android validation may legitimately not trigger for a one-file contract-only diff. A missing Android
trigger must not be reported as PASS.

No final-head workflow may remain pending or failing before durable closure.

## Closure evidence requirements

Durable closure requires:

1. exact final branch/head/tree read;
2. exact predecessor -> DS compare;
3. proof that the compare contains only this contract path;
4. canonical exact-head CI verdict;
5. fresh raw guarded read of the closed C03e-DR rolling predecessor;
6. immutable C03e-DS audit upload to the existing Drive project folder;
7. byte-exact immutable audit readback;
8. append-only rolling evidence update preserving every predecessor byte;
9. byte-exact rolling readback;
10. PR body transition to `Status: CLOSED` only after durable Drive evidence.

The PR remains draft/open/unmerged by project convention.

## Target gate

C03e-DS target gate:

> The future Agent-internal start-intent construction seam is selected to derive requester identity
> only from the already-authenticated remote-session owner and to consume one nominated logical
> target `DeviceId`. Source materialization of this helper remains separately gated. Concrete
> requester-aware policy backing, DR caller activation, wire/runtime/listener/networking activation,
> deployment and merge remain separately gated.
