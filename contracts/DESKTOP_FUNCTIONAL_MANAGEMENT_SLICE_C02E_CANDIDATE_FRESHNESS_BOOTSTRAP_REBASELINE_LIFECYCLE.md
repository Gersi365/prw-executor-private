# Phase 152 C02e — Candidate Freshness Bootstrap / Re-Baseline Lifecycle

Status: `DESIGN_LOCK / NEW_PEER_LIFECYCLE_BOOTSTRAP_VERIFIER_OWNED / EXISTING_LIFECYCLE_STATE_LOSS_IS_RECOVERY_REQUIRED_NOT_NEW / INVALID_FIRST_PUBLICATION_NONCONSUMING / SUCCESSFUL_FIRST_PUBLICATION_SINGLE_COMMIT / SAME_PEER_AUTOMATIC_REBASELINE_FORBIDDEN / REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO`

Review base head: `a10e784ff6a241608a18f6fc073c90698b703189`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

C02e now locks where candidate-publication freshness is owned and that ordinary session renewal must not reset it. This checkpoint resolves the next lifecycle question without selecting a concrete token representation:

**when may a freshness lifecycle be initialized, and how must the system distinguish a legitimately new peer lifecycle from an existing lifecycle whose verifier state is unavailable?**

This distinction is required to prevent restart/state-loss from becoming a replay reset primitive.

## Relevant repository precedent

Phase 128 session authentication provides a narrow but useful ownership precedent:

- the verifier/service creates fresh challenge state itself using provider randomness;
- the caller does not choose the verifier nonce/baseline;
- challenge state is bound to an exact authenticated identity/session context;
- invalid proof does not consume the pending challenge;
- successful proof consumes the state exactly once;
- future durable implementations must preserve equivalent atomic semantics rather than copy the in-memory storage details.

C02e reuses only this **verifier-owned initialization / non-consuming failure / single successful transition shape**. It does not reuse the Phase 128 nonce type, size, lifetime, canonical message or session-ID scope.

## Freshness lifecycle states — semantic distinction

A future upper freshness authority must distinguish at least these semantic conditions even if the concrete implementation uses different names/types.

### 1. Legitimately new peer lifecycle

The exact current `PeerConnectivityIdentity` has been authoritatively created as a new connectivity/publication lifecycle and no prior candidate-publication freshness state exists for that identity.

Examples include:

- initial establishment of candidate publication state for a newly current transport identity;
- a replacement peer lifecycle after an authorized `TransportIdentity` rotation.

Only this condition may enter the new-lifecycle bootstrap path.

### 2. Current peer lifecycle with established freshness

The exact peer identity already has current verifier-owned publication freshness state. Ordinary expected-current compare-and-advance applies.

Session renewal, reconnect and requester changes remain within this condition when `DeviceId + TransportIdentity` is unchanged.

### 3. Existing peer lifecycle with verifier state unavailable

The system knows or must conservatively assume that the exact peer lifecycle existed and may have accepted candidate publications, but the verifier cannot recover/prove its current freshness state.

Examples may include unrecovered durable-state loss, failover without authoritative state, or corrupted/missing freshness storage for an otherwise current peer identity.

This condition is **not** equivalent to a legitimately new peer lifecycle.

Publication acceptance must fail closed.

## New-lifecycle bootstrap ownership

For a legitimately new peer lifecycle, bootstrap state must be established by the verifier/upper owner, not selected by the publisher/requester.

At minimum:

1. registry-current publisher identity and exact current `TransportIdentity` are revalidated;
2. the owner confirms that this exact peer lifecycle is authoritatively eligible for initialization rather than an existing lifecycle with lost state;
3. the verifier establishes fresh bootstrap state according to the later reviewed representation;
4. bootstrap state is bound to the exact peer lifecycle and cannot authorize another `DeviceId`/`TransportIdentity`;
5. the publisher may receive whatever challenge/expected-state material the later protocol requires, but cannot choose the verifier baseline;
6. requester identity does not create separate bootstrap state for the same target peer.

The exact bootstrap state representation, delivery channel and cryptographic binding remain unselected.

## First accepted publication

The first candidate publication under a new freshness lifecycle must preserve the same locked ordering as later updates:

1. current requester/publisher/workspace/target/transport admission;
2. exact bootstrap/freshness proof comparison against verifier-owned current bootstrap state;
3. complete candidate-plan validation;
4. one logical successful transition that makes the candidate state current and advances/consumes bootstrap freshness into ordinary current freshness state;
5. stale prior traversal state is not made authoritative.

If candidate validation fails, bootstrap freshness must not be consumed merely because the publisher presented the correct current bootstrap proof.

If two first publications race from the same bootstrap state, at most one may commit.

After successful first commit, replay of the bootstrap state must fail.

## Session renewal during bootstrap

Ordinary authenticated session renewal for the same peer identity must not create a second independent bootstrap namespace.

A later protocol may choose to bind an individual bootstrap proof to a current authenticated session, but verifier ownership must ensure there is still only one current peer-lifecycle bootstrap/freshness authority.

If verifier policy intentionally supersedes bootstrap material after session change, the old material must be invalidated as part of one verifier-controlled transition; the publisher cannot reset bootstrap by requesting a new session.

Exact supersession mechanics remain unselected.

## Transport rotation

An authorized `TransportIdentity` rotation is an authoritative identity-lifecycle transition:

- old peer identity/freshness/plan/traversal state becomes stale;
- the replacement `DeviceId + new TransportIdentity` is a distinct peer lifecycle;
- verifier-owned bootstrap may be established for that replacement lifecycle;
- no old freshness value, candidate ID, endpoint, session ID or traversal credential may initialize/authorize the replacement identity.

This checkpoint does not require that every freshness recovery rotate transport identity; it only recognizes transport rotation as an existing authoritative way to create a genuinely new peer identity lifecycle.

## Existing-lifecycle state loss / restart / failover

Missing freshness state for an existing current peer identity must not automatically call the new-lifecycle bootstrap path.

Otherwise a process restart, database loss, failover race or deliberate state deletion could reset replay protection while old authentic publications remain replayable.

Therefore:

- the future authority must retain/recover enough authoritative lifecycle metadata to distinguish **new lifecycle** from **existing lifecycle state unavailable**;
- when current freshness cannot be proven for an existing lifecycle, candidate publication acceptance fails closed;
- the owner must not accept a caller claim that this is the "first" publication;
- no default/zero/empty/first token may be assumed.

The exact durable metadata/storage/replication mechanism remains unselected.

## Re-baselining an existing peer identity

Automatic re-baselining for the same current `PeerConnectivityIdentity` is forbidden.

Any future recovery protocol that deliberately establishes a new freshness baseline while keeping the same peer identity would need a separately reviewed security contract proving that:

- all prior publication capabilities/proofs are invalidated or made non-replayable;
- the re-baseline is verifier-authorized rather than publisher-selected;
- concurrent old/new baselines cannot both commit;
- current registry identity remains exact;
- the operation does not weaken transport-rotation or candidate/traversal lifecycle rules.

No repository precedent currently determines such a same-identity re-baselining protocol, so it remains unselected.

## Safe failure posture

Until a valid new-lifecycle bootstrap or separately reviewed recovery/re-baseline is available:

`freshness unavailable for existing peer -> candidate publication unavailable/fail closed`

This does not require deleting the logical device or changing its `DeviceId`.

Other already-authoritative connectivity state may remain inspectable, but no new candidate publication may become current without verifier freshness authority.

## Relationship to test-only freshness reference

The existing test-only owner currently models:

- established current freshness; and
- unavailable freshness.

It intentionally does not yet model a production/bootstrap challenge representation.

A later test-only lifecycle-state extension may distinguish `new lifecycle eligible for verifier bootstrap` from `existing lifecycle freshness unavailable` using local non-normative markers, but must not promote those markers into production protocol values.

## Security invariants

C02e must not:

- infer "new peer lifecycle" merely from missing freshness storage;
- allow publisher/requester to choose the initial replay baseline;
- reset freshness because a process/session/control connection restarted;
- create parallel bootstrap namespaces per requester/session for one peer lifecycle;
- consume bootstrap freshness after rejected candidate validation;
- allow two first publications from one bootstrap state to commit;
- reuse old transport identity freshness for a replacement transport identity;
- automatically re-baseline the same peer identity after state loss;
- invent concrete nonce/counter/timestamp/wire/persistence values in this checkpoint.

## Deliberately unselected

- bootstrap value type/size/encoding;
- bootstrap lifetime/expiry/retry count;
- cryptographic proof/canonical message;
- wire delivery field/message kind;
- durable lifecycle metadata schema;
- failover consensus/replication;
- same-identity recovery/re-baselining protocol;
- concrete upper-owner crate/module;
- production synchronization primitive.

## Next safe seam

Review whether the current source/design chain has enough lifecycle state to represent **new vs established vs recovery-required** without a production freshness value, and whether a test-only lifecycle-state reference can prove that missing state never aliases new lifecycle.

No production source/runtime or Cargo change is authorized by this checkpoint.
