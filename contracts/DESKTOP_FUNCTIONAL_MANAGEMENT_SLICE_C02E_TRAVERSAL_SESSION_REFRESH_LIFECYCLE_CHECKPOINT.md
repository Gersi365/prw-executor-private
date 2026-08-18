# Phase 152 C02e — Traversal Session / Candidate Refresh Lifecycle Checkpoint

Status: `DESIGN_LOCK / OLD_TRAVERSAL_SESSION_INVALID_AFTER_REFRESH / RETAINED_CANDIDATE_OLD_OBSERVATION_REJECT_REQUIRED / EXACT_RUNTIME_CORRELATION_UNSELECTED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Review base head: `4b250701b53dc0cf690f4e87e751a28ed0ddab1b`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This checkpoint closes a narrow lifecycle gap found after the C02e source-only integration review.

Candidate-ID lifetime freshness prevents an old numeric identifier from being rebound or reused for a different candidate. That is necessary but not sufficient to prevent stale Phase 141 observation state after a full candidate refresh.

## Concrete gap

Phase 141 `IceConnectivitySession` owns its own configured `remote_candidates` correlation set and produces `CandidateReachabilityUpdate` values containing only:

- `CandidateId`;
- `ReachabilityObservation`.

`CandidateReachabilityUpdate::apply(...)` delegates to `PeerConnectivityPlan::set_observation(...)` and therefore checks only whether that candidate ID currently exists in the target plan.

C02e `PeerConnectivityPlan::refresh_candidates(...)` intentionally resets **all** observations to `Unknown` after successful refresh.

For a removed candidate, a late update from the old ICE session already fails because the ID is absent and cannot be reused within the same plan lifetime.

However, if a candidate is retained exactly across refresh with the same ID/path/endpoint, a late `CandidateReachabilityUpdate` produced by the **pre-refresh traversal session** can still find that ID and set the refreshed plan observation to `Reachable`.

That would repopulate post-refresh reachability state using an observation whose traversal-session lifecycle predates the refresh that intentionally cleared observations.

## Locked lifecycle rule

Every successful full candidate refresh creates a new traversal-observation lifecycle boundary.

After `PeerConnectivityPlan::refresh_candidates(...)` succeeds:

1. every Phase 141 `IceConnectivitySession` created for the preceding candidate-state lifecycle is stale;
2. every queued/unapplied `CandidateReachabilityUpdate` produced by such a stale traversal session is stale;
3. stale traversal-session updates must not be applied to the refreshed plan, including updates for candidates retained exactly across the refresh;
4. a caller that continues traversal must establish a replacement traversal session from the refreshed current candidate state and current authenticated coordination metadata before new reachability observations may become authoritative;
5. only observations attributable to the current traversal-session lifecycle may update the current refreshed plan.

This is a session-lifecycle rule, not a change to PRW logical device identity.

## Why CandidateId freshness alone is insufficient

Candidate-ID lifetime freshness solves endpoint aliasing:

- removed IDs cannot return;
- old ID cannot become a new endpoint;
- late updates for removed IDs fail closed.

It does not identify **which traversal session** produced an observation for an exactly retained candidate.

Therefore `CandidateId` must remain candidate correlation only. It must not be treated as traversal-session freshness authority.

## Relationship to candidate-publication freshness

Candidate-publication replay freshness and traversal-observation freshness are separate boundaries.

Publication freshness answers:

`is this candidate-set update newer/current relative to previously accepted candidate publications?`

Traversal-session currentness answers:

`was this reachability observation produced by the traversal lifecycle established for the current accepted candidate state?`

Both are required before a future runtime can safely drive dynamic reachability.

A replay-safe candidate publication does not make an older ICE observation current, and a current ICE observation does not authenticate or order candidate publications.

## Transport rotation

Transport rotation remains a stronger lifecycle reset:

- old `TransportIdentity` makes the old connectivity plan stale;
- replacement plan is required for the new current transport identity;
- all traversal sessions and observations associated with the old plan/transport identity are stale by definition.

No traversal state from the old transport may authorize or establish reachability for the replacement plan merely because endpoint or candidate numbers match.

## Phase 141 source relationship

No Phase 141 source mutation is required or authorized by this checkpoint.

The existing `IceConnectivitySession` already represents bounded session-local protocol/correlation state and is started only as one traversal session. It is not an identity authority and does not own C02e plan replacement policy.

The missing enforcement belongs to the later composition/runtime boundary that owns both:

- current `PeerConnectivityPlan` lifecycle;
- current traversal-session lifecycle.

That owner must ensure stale traversal sessions/queued observations cannot cross a successful candidate refresh.

## Deliberately unselected enforcement mechanism

C02e does **not** select:

- a traversal generation integer;
- a traversal-session ID wire field;
- a nonce or timestamp;
- a lock/channel/task cancellation implementation;
- async runtime ownership;
- queue-drain semantics;
- a production ICE restart API;
- control-plane signaling layout.

A future source/runtime implementation may choose a bounded mechanism only after the owning composition boundary is reviewed. Until then, production traversal integration remains fail-closed/unwired.

## Security invariants

C02e must not:

- apply an old traversal-session observation after a successful candidate refresh;
- allow an exactly retained `CandidateId` to serve as proof that an observation belongs to the current traversal lifecycle;
- preserve pre-refresh selected-pair reachability as current post-refresh evidence;
- mutate Phase 141 into an identity or publication authority;
- infer traversal freshness from IP/port;
- weaken candidate-publication freshness or transport-rotation replacement-plan rules;
- activate sockets, STUN/ICE/TURN runtime, QUIC runtime, Agent/bootstrap or deployment.

## Relationship to prior integration review

The preceding source-only integration review remains valid for identity, provenance, registry ordering, candidate transactionality, candidate-ID lifetime freshness and the mandatory publication-freshness gate.

This checkpoint supplements its Phase 141 conclusion with the narrower retained-candidate case: candidate presence alone is not sufficient currentness evidence after a plan refresh. The later composition boundary must additionally enforce traversal-session lifecycle currentness.

## Validation boundary

Static design/source inspection only. No build, `cargo fmt`, Clippy, tests, workflow dispatch, real network I/O or production mutation is performed.

## Next safe seam

Continue C02e with static ownership review for the future composition point that must atomically coordinate:

`accepted candidate publication -> candidate plan refresh -> traversal-session invalidation/replacement -> current observation admission`

Do not choose runtime/concurrency primitives until an existing authoritative owner/precedent can be reused safely.
