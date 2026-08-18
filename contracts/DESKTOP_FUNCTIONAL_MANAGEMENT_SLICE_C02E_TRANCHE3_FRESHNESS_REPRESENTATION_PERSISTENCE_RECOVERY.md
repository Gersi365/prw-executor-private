# Phase 152 C02e — Tranche 3 Candidate-Publication Freshness Representation / Persistence / Recovery

Status: `DESIGN_LOCK / PRODUCTION_REPRESENTATION_SELECTED / OPAQUE_256_BIT_VERIFIER_TOKEN / ROTATE_ON_ACCEPTED_COMMIT / DURABLE_LIFECYCLE_STATE_REQUIRED / RETIRED_TOMBSTONE_REQUIRED / AUTHENTICATED_CURRENT_TOKEN_RESYNC_ALLOWED / STATE_LOSS_RECOVERY_REQUIRED / SAME_IDENTITY_REBASELINE_FORBIDDEN / WIRE_KIND_UNSELECTED / UPPER_OWNER_UNSELECTED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Tranche 2 closeout base head:

`f1f3d58bf3128da377e4e8838648bcf52992ca3a`

Frozen predecessor C02d:

`857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This checkpoint selects the production **candidate-publication freshness value representation** and the persistence/recovery semantics required to preserve C02e replay safety across accepted publications, process restart, failover, response loss and transport-identity lifecycle changes.

It does not select the production upper reachability owner, persistence product/database, synchronization primitive, wire message kind, control-plane codec, runtime task, socket, traversal adapter, deployment path or PR/merge.

## Prior locked authority

This checkpoint preserves the earlier C02e locks:

- candidate-publication freshness is verifier-owned upper-reachability coordination state;
- its scope is the exact current publisher peer lifecycle (`DeviceId + TransportIdentity`), not an endpoint, requester, or `SessionId`;
- ordinary session renewal/reconnect for the same current peer does not reset freshness;
- authenticated requester/publisher/current registry/workspace admission precedes freshness comparison;
- stale/duplicate freshness fails before candidate mutation;
- complete candidate validation precedes authoritative commit;
- accepted plan refresh, freshness advance and prior traversal invalidation are one logical successful transition;
- failed freshness or candidate validation is non-consuming;
- missing freshness for an existing lifecycle is recovery-required, not a new lifecycle;
- caller-selected initial baselines and automatic same-identity re-baselining are forbidden.

## Selected production freshness representation

The production freshness value is an **opaque 256-bit verifier-issued token** represented as exactly 32 bytes.

Normative properties:

1. token width is exactly 32 bytes;
2. the all-zero 32-byte value is invalid and must never be installed as current/bootstrap freshness;
3. the bytes have no integer, counter, timestamp, duration, request-ID, candidate-ID, endpoint, session-ID or clock meaning;
4. the verifier/upper authority is the only authority that may generate token material installed into authoritative freshness state;
5. token generation must use an approved cryptographically secure entropy source;
6. a replacement token installed after a successful publication must differ from the expected/current token it replaces;
7. token equality is scoped by the exact peer lifecycle record and never authorizes another `DeviceId` or `TransportIdentity`;
8. the token is replay-ordering state, not a substitute for session authentication, registry validation, authorization or transport identity.

The provider-neutral dormant source representation is:

`crates/prw-remote-bridge/src/candidate_publication_freshness.rs`

That source remains unexported/unwired in this tranche and performs no I/O.

## Why opaque token, not counter or timestamp

A counter would require a normative initial numeric value, width/overflow behavior and durable increment/recovery semantics. A timestamp would require trusted-clock, skew/window and replay-window policy. Neither is determined by existing repository precedent.

An opaque verifier-issued token directly implements the already locked compare-and-advance shape without importing clock semantics or caller-selected numeric baselines:

`expected exact current token + exact current authenticated peer + valid complete candidates`

`-> accepted candidate plan + verifier-issued replacement token + prior traversal stale`

A replay of an older authentic publication carries an already-consumed expected token and therefore cannot satisfy the current comparison.

## Durable logical record

Persistence is logically keyed to one exact `PeerConnectivityIdentity`:

`DeviceId + TransportIdentity`

The durable semantic record has exactly one of these lifecycle dispositions:

### `NewLifecycleEligible(token)`

An authority-created peer lifecycle that is explicitly eligible for first-publication bootstrap and already has a verifier-issued non-zero 32-byte bootstrap token.

Storage absence must never be interpreted as this state.

### `Established(token)`

An existing peer lifecycle with exact recoverable current verifier freshness.

Ordinary session renewal/reconnect continues using this record.

### `RecoveryRequired`

The peer lifecycle exists/currentness must be conservatively assumed, but exact verifier freshness cannot be proven or recovered.

Candidate publication fails closed. No default token, zero token, fresh bootstrap token or caller claim may repair this state.

### `Retired`

A historical peer lifecycle tombstone. It carries no current token and is not bootstrap-eligible.

The tombstone prevents a historical exact `DeviceId + TransportIdentity` from silently becoming a fresh replay namespace if the same transport-identity bytes later become registry-current again.

## Transport-identity reuse rule

The current Phase 130 registry rejects an unchanged replacement, but it does not itself prove that a replacement `TransportIdentity` value has never been used previously by the same device.

Therefore freshness persistence must not equate:

`registry says this transport value is current`

with:

`this exact peer identity is historically new`.

If a durable `Retired` record already exists for the exact `DeviceId + TransportIdentity`, that historical identity is **not** automatically eligible for `NewLifecycleEligible` bootstrap.

Until a separately reviewed identity-incarnation/re-baseline mechanism exists, publication under that reused exact peer identity fails closed rather than resetting replay protection.

This Tranche 3 rule intentionally avoids changing the Phase 130 registry contract.

## Bootstrap transaction

For an authoritatively new peer lifecycle:

1. current registry identity and transport identity are revalidated;
2. upper authority proves explicit new-lifecycle eligibility; absence alone is insufficient;
3. verifier generates a non-zero 32-byte bootstrap token;
4. `NewLifecycleEligible(token)` is durably installed for the exact peer lifecycle before it is presented for publication use;
5. requester or publisher cannot choose or replace the bootstrap token;
6. if durable installation cannot be proven, publication remains unavailable.

The wire/delivery mechanism for giving the exact current token to the authenticated publisher remains unselected.

## Accepted publication transaction

For `NewLifecycleEligible(expected)` or `Established(expected)`:

1. authenticate/revalidate publisher and requester using existing C02e ordering;
2. revalidate exact registry-current `DeviceId + TransportIdentity`;
3. compare the publication's expected freshness with the exact durable current token;
4. reject stale/duplicate mismatch before authoritative candidate mutation;
5. validate the complete candidate vector on staged/non-authoritative state;
6. verifier generates a distinct non-zero replacement token;
7. one linearizable durable commit publishes all authoritative effects together:
   - refreshed current `PeerConnectivityPlan`;
   - freshness lifecycle becomes `Established(replacement)`;
   - prior traversal lifecycle/current queued observations become stale according to the existing C02e contract;
8. acknowledgment/current-token delivery occurs only after the durable commit is known successful.

If any validation fails or the commit aborts, the old plan/freshness/traversal authority remains current.

Two publications racing from the same expected token cannot both commit.

## Persistence atomicity requirement

The persistence backend remains unselected, but its semantics are not optional.

A conforming implementation must provide an equivalent linearizable compare-and-commit boundary over the current expected freshness and all authoritative reachability effects. Examples may include a serializable transaction or compare-and-swap guarded transaction, but this checkpoint does not select one.

A design that durably advances freshness separately from plan/traversal state without a proven atomic composition is non-conforming.

A design that commits plan state and acknowledges success before durable freshness advance is non-conforming.

## Crash and restart semantics

### Crash before commit

The old durable plan/token remains authoritative. The publisher may retry using the same expected token after ordinary authentication/currentness checks.

### Crash after commit but before acknowledgment

The replacement token and refreshed plan are already authoritative. Replay/retry with the old token fails.

This is not state loss. The exact current token may be recovered/reissued through the authenticated resynchronization rule below.

### Restart/failover with complete durable record

The successor authority restores the exact current lifecycle state/token and continues without freshness reset.

### Restart/failover with missing, corrupt, ambiguous or non-authoritative freshness

The exact peer lifecycle becomes `RecoveryRequired` for publication purposes and fails closed.

No process-local default, counter reset, token regeneration, empty row or missing-row interpretation may create `NewLifecycleEligible` for an existing lifecycle.

## Authenticated current-token resynchronization

Tranche 3 selects a recovery semantic for **lost delivery**, not for lost authority state.

When durable state is intact but the legitimate publisher no longer possesses the current verifier token (for example commit succeeded and the response was lost), the authority may reissue the **same exact current token without changing freshness state** only after:

1. authenticating a current publisher session;
2. revalidating the session against current registry membership/device identity;
3. revalidating the exact current `TransportIdentity`;
4. confirming the durable record is `NewLifecycleEligible(token)` or `Established(token)` for that exact peer;
5. confirming the record is neither `RecoveryRequired` nor `Retired`.

Resynchronization is read/re-delivery of current verifier state. It is not a re-baseline, token rotation, replay reset, requester-specific namespace or automatic recovery from missing durable state.

The wire message/endpoint used for resynchronization remains unselected and must be separately reviewed before runtime activation.

## Transport rotation and retirement

When current transport identity changes from old to replacement:

- old peer plan/freshness/traversal state becomes stale;
- the old freshness record becomes `Retired` as part of the authoritative lifecycle transition or an equivalent linearizable composition;
- the replacement exact peer identity may become `NewLifecycleEligible` only if authority can prove it is historically bootstrap-eligible;
- a historical `Retired` record for the replacement key blocks automatic bootstrap;
- no old token or token continuity authorizes the replacement identity.

The concrete cross-authority transaction between registry transport rotation and future upper reachability storage remains part of the still-unselected production composition owner. Until that integration is selected, this is a required semantic boundary, not activated runtime behavior.

## Same-identity state-loss recovery

Automatic same-identity re-baselining remains forbidden.

`RecoveryRequired` cannot transition to `NewLifecycleEligible` or `Established(new_token)` merely because an authenticated publisher asks, reconnects, restarts or loses local token material.

A future same-identity re-baseline would require a separate security contract proving prior publication capabilities are invalidated and old/new baselines cannot both commit.

No such protocol is selected here.

## Persistence-retention rule

A conforming durable authority must retain enough lifecycle history/tombstone information to ensure that storage compaction or deletion cannot make a previously used exact peer identity appear historically new.

This checkpoint does not require a particular table, database or infinite physical row retention. It requires equivalent semantic memory of retired identities for as long as the surrounding identity system could make those exact identities current again.

## Deliberately unselected

Tranche 3 does not select:

- production upper reachability owner crate/module;
- database/storage product;
- database key byte encoding or record serialization format;
- replication/consensus technology;
- mutex/lock/actor/task primitive;
- control-plane message kind or field allocation;
- text/base64/hex wire encoding;
- endpoint/API for token delivery or resynchronization;
- runtime bootstrap wiring;
- socket/network/STUN/ICE/TURN/QUIC activation;
- production traversal owner;
- deployment, signing, system mutation, PR or merge.

## Validation boundary

Executable validation for this tranche may compile/lint/test the dormant representation and the existing workspace. It may not activate production networking or select the still-open owner/wire/runtime surfaces.

## Exit condition

Tranche 3 closes when:

1. the exact 32-byte opaque token representation is present as dormant provider-neutral source;
2. executable tests prove non-zero exact-width token and disjoint lifecycle states;
3. persistence/recovery/retirement/resynchronization semantics are recorded;
4. Cargo manifests and lockfile remain unchanged;
5. focused and workspace validation pass;
6. temporary validation harnesses are removed after authoritative evidence capture.
