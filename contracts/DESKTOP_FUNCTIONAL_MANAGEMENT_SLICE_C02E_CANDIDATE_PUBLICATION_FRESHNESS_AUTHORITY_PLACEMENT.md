# Phase 152 C02e — Candidate Publication Freshness Authority Placement

Status: `DESIGN_LOCK / UPPER_REACHABILITY_AUTHORITY_REQUIRED / PEER_LIFECYCLE_SCOPED / SESSION_RENEWAL_MUST_NOT_RESET / REQUESTER_INDEPENDENT / TRANSPORT_ROTATION_NEW_LIFECYCLE / RESTART_FAIL_CLOSED / REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO`

Review base head: `02e8d41eb33caf1b7cbe8f53045516b1dd242619`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

C02e already requires verifier-owned candidate-publication freshness and a logical atomic transition with candidate-plan refresh. The exact freshness representation remains intentionally unselected.

This checkpoint resolves the separate **state-ownership and lifecycle-placement** question: which domain may own publication freshness, what identity scope it follows, and which lifecycle events must or must not reset it.

It does not select a counter, nonce, timestamp, database, wire field or synchronization primitive.

## Existing authorities reviewed

### Phase 128 session-authentication challenge state

`SessionAuthChallengeState` is explicitly the provider-neutral device-session authentication domain.

It is bound to:

- one `SessionId`;
- one exact server challenge nonce;
- issue/expiry time;
- one immutable enrolled identity snapshot;
- single-use proof-consumption state.

Its contract is a short-lived cryptographic authentication challenge. It is not an ongoing candidate-publication ordering stream.

Reusing `SessionAuthNonce`, `SessionAuthChallengeState`, the 300-second challenge window, or challenge `SessionId` as candidate-publication freshness would therefore conflate two distinct security domains and would silently select a representation that C02e has not reviewed.

### Phase 128 `SessionAuthenticationService`

The session service owns pending/completed authentication transactions keyed by `SessionId`.

Its successful transition creates authenticated session identity and consumes the pending authentication challenge. It deliberately does not grant networking/candidate authority.

Candidate-publication freshness must not be stored there merely because both domains use replay protection.

### Phase 130 workspace/device registry

`WorkspaceDeviceRegistry` owns current workspace membership, immutable device binding/current device lifecycle, and the separately rotatable current `TransportIdentity`.

It correctly acts as current identity authority used before candidate admission.

It does **not** model reachability publication history, candidate vectors, traversal sessions, or per-publication ordering. Adding candidate freshness into this registry would couple transient reachability coordination state into the identity registry rather than preserving the existing domain separation.

### C02e semantic publication adapter

`AuthenticatedCandidatePublication` is a bounded semantic snapshot derived from a registry-current authenticated publisher. It intentionally carries no freshness field and remains unexported.

The publication value itself must not become the owner of mutable verifier state.

## Locked owner placement

Candidate-publication freshness belongs to the **upper reachability composition authority** that owns, or is transactionally coupled to, the accepted candidate-state lifecycle.

The future architecture may implement that authority as:

- state directly held by the upper composition owner; or
- a dedicated verifier subauthority participating in the same linearizable commit boundary.

The choice between those implementation shapes remains open.

What is locked is that freshness is **not** owned independently by the requester, the publisher's session-authentication challenge service, an endpoint, Phase 141 ICE state, or the identity registry.

## Freshness identity scope

The publication freshness lifecycle is publisher/target reachability state.

At admission time it must be bound to the exact registry-current authenticated publisher identity and its current `PeerConnectivityIdentity`:

`authenticated publisher registry tuple + DeviceId + current TransportIdentity`

The connectivity discriminator is therefore the exact current publisher `PeerConnectivityIdentity`, not an IP address/port and not a requester identity.

### Requester independence

Freshness must not be partitioned by requester.

If two same-workspace requesters consume candidate state for the same target peer, they must not each obtain an independent replay namespace that allows the same stale target publication to become current twice.

Requester/session/workspace admission remains required, but requester identity is authorization context, not the publisher publication-ordering key.

### Publisher session renewal independence

A new authenticated `SessionId` for the same registry-current logical device and unchanged `TransportIdentity` does **not** by itself create a new candidate-publication freshness lifecycle.

Session renewal, reconnect, control-transport reconnection or application-session reauthentication must not silently reset publication freshness while the exact same `PeerConnectivityIdentity` remains current.

Otherwise an older authentic candidate publication from the same peer lifecycle could become admissible again merely because authentication was renewed.

A future wire protocol may bind each individual publication to a current authenticated session as provenance, but the verifier freshness state must survive ordinary session replacement for the same peer lifecycle.

## Transport-identity rotation

`TransportIdentity` rotation remains the explicit replacement boundary.

When registry current state rotates from old transport identity to new transport identity:

- the old `PeerConnectivityIdentity` is stale;
- the old plan is stale;
- the old traversal lifecycle is stale;
- the old publication freshness lifecycle is stale and cannot authorize the new transport identity;
- the replacement peer lifecycle requires verifier-controlled freshness initialization under the new exact identity.

No numeric/token continuity from the old lifecycle may itself authorize the new one.

The exact initialization representation/protocol for the replacement lifecycle remains unselected.

## First-publication initialization

A future freshness authority must distinguish verifier-owned initialization from caller authority.

When no current freshness state exists for a legitimate new peer lifecycle, the verifier/owner must establish whatever initial state the separately reviewed representation requires.

The publisher must not be able to choose an arbitrary initial value and thereby define the verifier's replay baseline.

This checkpoint does not choose whether initialization uses a verifier challenge, opaque token, generation, durable row, or another mechanism.

## Restart / failover rule

Service restart, process replacement or multi-node failover must not silently behave as "freshness reset to initial" for an already existing peer lifecycle.

If the future production owner cannot prove/recover the exact current publication freshness state after restart/failover, candidate-publication acceptance must fail closed until a separately reviewed recovery/re-baselining procedure safely re-establishes verifier authority.

The exact durable storage, replication, failover and recovery mechanism remains unselected.

This rule preserves replay safety without prematurely selecting persistence architecture.

## Relationship to one-shot composition commit

The upper freshness authority must participate in the same logical accepted transition already locked by C02e:

`current authenticated publisher/requester/peer`

`+ exact expected current publication freshness`

`+ complete valid candidate vector`

`-> refreshed current PeerConnectivityPlan`

`+ advanced verifier freshness`

`+ previous traversal lifecycle stale`

If freshness comparison fails, authoritative plan/traversal state remains unchanged.

If candidate validation fails, freshness remains unchanged.

If commit succeeds, later session renewal for the same peer continues from the advanced freshness state rather than resetting it.

## Separation from traversal freshness

Candidate-publication freshness and traversal-observation lifecycle currentness remain distinct:

- publication freshness orders accepted candidate sets for one peer lifecycle;
- traversal currentness decides whether a reachability observation came from the traversal lifecycle associated with the currently accepted candidate state.

The upper owner coordinates both, but one token/state must not be assumed to serve both purposes unless a later contract explicitly proves that design.

## Security invariants

C02e must not:

- key publication replay protection only by `SessionId`;
- reset publication freshness on ordinary authenticated session renewal/reconnect for the same current peer identity;
- create a separate replay namespace per requester;
- store candidate publication history in the identity registry without a separately reviewed architecture change;
- reuse Phase 128 nonce/challenge/time-window fields as candidate freshness by analogy;
- let Phase 141 ICE/session state become candidate-publication freshness authority;
- let an old transport identity's freshness authorize a replacement transport identity;
- silently reset current freshness after restart/failover;
- accept caller-selected initial freshness state;
- weaken the existing atomic refresh/freshness/traversal-invalidating commit semantics.

## Deliberately unselected

This checkpoint still does not choose:

- freshness value type/width;
- initial value;
- random nonce size/encoding;
- timestamp/window;
- replay-window model;
- persistence schema;
- recovery/re-baselining protocol;
- database/transaction primitive;
- synchronization/runtime primitive;
- wire field/message kind;
- concrete crate/module for the final upper owner.

## Next safe seam

A test-only generic freshness-authority reference may now be staged to prove the locked ownership/lifecycle semantics without selecting a production representation.

Such a reference must:

- use test-local opaque freshness values only;
- stage/validate plan mutation on non-authoritative state before freshness commit;
- commit plan + freshness + traversal invalidation together under exclusive reference ownership;
- prove stale/duplicate expected state fails before authoritative mutation;
- prove failed candidate validation does not consume freshness;
- prove same-peer session renewal does not reset the authority;
- remain unexported and not infer production persistence/wire/runtime design.

## Validation boundary

Static design review only. No production source API, Cargo manifest, lockfile, build/test workflow, network I/O, traversal activation, Agent/bootstrap, deployment, signing, privileged state, PR or merge is modified or executed by this checkpoint.
