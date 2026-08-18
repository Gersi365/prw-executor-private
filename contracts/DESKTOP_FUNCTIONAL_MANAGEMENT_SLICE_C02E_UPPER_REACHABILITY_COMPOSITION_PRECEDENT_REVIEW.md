# Phase 152 C02e — Upper Reachability Composition Precedent Review

Status: `PASS_STATIC_PRECEDENT_REVIEW / UPPER_ORCHESTRATION_PATTERN_CONFIRMED / CONCRETE_OWNER_UNSELECTED / ONE_SHOT_COMPOSITION_SEAM_AUTHORIZED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Review base head: `904b7c63c07fae1bde409459d89a65b12b498ddc`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The preceding C02e ownership review proved that the current crate graph has no existing component that owns both mutable `PeerConnectivityPlan` lifecycle and Phase 141 `IceConnectivitySession` lifecycle.

This review asks the narrower architectural question required to continue safely: **how has this repository historically introduced an upper composition owner when lower state machines already existed but runtime ownership was not yet selected?**

The goal is to reuse that repository pattern instead of assigning traversal ownership to `prw-agent`, `prw-remote-bridge`, `prw-nat-traversal`, or a new crate without authority.

## Authoritative repository precedent

The local Linux Agent progression provides a directly relevant composition pattern.

### Phase 051 — aggregate without a second state model

`LOCAL_BOUNDARY_SERVER_CONNECTION_STATE_CONTRACT.md` extends the existing connection state through composition and explicitly says it does not create a second or parallel connection-state model. Policy remains caller-supplied and runtime concerns remain outside the state machine.

### Phase 052 — bounded caller-owned resumption

The bounded server connection loop adds a caller-supplied work quantum. The same reader/writer/state may be resumed by the caller, while socket/runtime/thread/timer ownership remains deferred.

The lower processing state therefore exposes bounded operations without choosing the long-running owner.

### Phase 071 — composition bridge before orchestration

`LINUX_AGENT_AUTHENTICATED_SESSION_BRIDGE_CONTRACT.md` composes an already authenticated accept outcome into a fresh application session. It deliberately performs no accept loop, scheduling, concurrency, shutdown orchestration, policy binding, bootstrap activation, or application processing.

This separates **state composition** from **runtime orchestration**.

### Phase 072 — runtime obligations locked before activation

`LINUX_AGENT_CONNECTION_PROCESSING_BOUNDS_CONTRACT.md` locks absolute request deadlines, finite request count, bounded concurrency, initial worker model, and the requirement for a separate shutdown lifecycle before Agent bootstrap activation.

It does not activate the scheduler or production runtime merely because the responsibilities are now known.

### Phase 077/080/081 — ownership pieces materialized independently

The worker lifecycle sequence then locks and implements separate ownership primitives before scheduling:

- Phase 077: scoped worker lifetime and mandatory result accounting;
- Phase 080: registry ownership of scoped worker handles and explicit reap/join behavior;
- Phase 081: cancellation authority associated with the exact registered worker before the first accept-and-spawn scheduler is activated.

The lower worker/session state is not mutated to become its own scheduler or cancellation owner.

### Phase 085 — finite scheduler composition before an outer runtime loop

`LINUX_AGENT_BOUNDED_SCHEDULING_CYCLE_CONTRACT.md` composes already reviewed capacity, registry, cancellation and one-shot scheduler behavior behind a caller-bounded attempt budget.

It explicitly does not choose or implement the long-running outer wait loop.

### Phase 092 — upper finite orchestrator after prerequisite seams exist

`crates/prw-agent/src/linux_runtime_orchestration.rs` finally composes readiness, scheduling control, worker capacity, worker registry, completion wake and finite scheduling into a dedicated upper orchestration layer.

The source documentation explicitly states that Phase 092 preserves the lower Phase 084/086 APIs and still contains no production outer loop or bootstrap activation.

## Repository pattern derived

The repeated pattern is:

1. keep lower protocol/state objects authoritative for their own local invariants;
2. do not make a lower object own unrelated lifecycle state merely because a composition gap exists;
3. lock the cross-object lifecycle semantics first;
4. materialize bounded ownership/accounting/cancellation primitives separately where needed;
5. compose them through a one-shot or caller-bounded upper transition;
6. only after that transition is validated may a long-running/runtime owner be selected and activated in a separately reviewed phase.

This pattern is stronger than selecting a crate based only on current dependency proximity.

## C02e mapping

The current dynamic-reachability components map onto that precedent as follows.

### Lower authoritative state

`PeerConnectivityPlan` owns:

- immutable `PeerConnectivityIdentity` for one plan lifetime;
- bounded candidate vector validation;
- plan-lifetime `CandidateId` non-rebinding/high-water freshness;
- transactional candidate refresh;
- reachability observations and deterministic path selection.

It must not become a traversal-session scheduler, publication freshness verifier, registry authority, or runtime owner.

### Lower traversal protocol state

Phase 141 `IceConnectivitySession` owns:

- one bounded Sans-I/O traversal session;
- its local/remote candidate correlation;
- STUN/ICE protocol progress;
- selected-pair correlation to an existing `CandidateId`.

It must not become PRW identity authority, candidate-publication freshness authority, plan-replacement policy, or the owner of the whole reachability lifecycle.

### Admission/provenance state

The C02e source-only candidate semantic adapter owns authenticated publication derivation/current registry admission semantics but remains unexported and intentionally has no candidate-publication freshness representation or traversal lifecycle authority.

### Missing upper composition responsibility

A future upper owner must coordinate, as one controlled lifecycle boundary:

`accepted authenticated candidate publication`

`-> verifier-owned publication freshness compare-and-advance`

`-> transactional PeerConnectivityPlan candidate refresh`

`-> stale old traversal-session/queued-observation invalidation on successful refresh`

`-> replacement traversal-session establishment from the refreshed current candidate state when traversal continues`

`-> admission of reachability observations only from the current traversal lifecycle`

No current crate owns this complete chain.

## Locked ownership rules

C02e therefore locks the following composition rules without naming the final crate:

1. **Upper ownership, not lower-state self-ownership.** The composition boundary must live above both plan state and traversal protocol state.
2. **No hidden second plan model.** The owner must retain `PeerConnectivityPlan` as the authoritative connectivity plan rather than shadowing candidate/observation state in a second model.
3. **No hidden second ICE authority.** The owner may hold/replace a Phase 141 traversal session but must not duplicate ICE protocol state or selected-pair logic.
4. **Successful refresh is a lifecycle commit.** Publication freshness advance, plan refresh success, and old-traversal invalidation belong to one logical accepted transition. A successful plan refresh cannot leave the old traversal session admissible.
5. **Failed refresh is non-destructive.** If identity, workspace, publication freshness, or candidate validation fails, the current plan and current traversal lifecycle remain unchanged; freshness state does not advance.
6. **Observation admission is owner-gated.** `CandidateId` existence alone is insufficient after a refresh. The upper owner must reject observations attributable to a stale traversal lifecycle before delegating current observations to `PeerConnectivityPlan::set_observation(...)`.
7. **Transport rotation is stronger than endpoint refresh.** A stale `TransportIdentity` invalidates the old plan and traversal lifecycle; replacement-plan construction remains required.
8. **No runtime inference.** These rules do not select a thread/task model, channel, lock, async runtime, queue, socket adapter, ICE restart API, persistence backend, or long-running loop.

## Concrete owner remains deliberately unselected

Current evidence is insufficient to choose among:

- a future crate-internal module in an existing crate;
- a new dedicated composition crate;
- a later Agent runtime integration layer;
- another upper control-plane/runtime component not yet materialized.

Choosing one now would create an architecture/dependency decision that the repository precedent says should follow, not precede, the bounded composition seam.

## Next safe seam

The next safe C02e step is a **design-only one-shot reachability composition transition** analogous to the repository's earlier one-shot/bounded orchestration stages.

That transition may lock:

- exact preconditions and ordering;
- success/failure state effects;
- which state remains current after every failure class;
- when prior traversal state becomes stale;
- what evidence a caller must receive;
- explicit prohibition on network/runtime activation.

It must remain representation-neutral for candidate-publication freshness and ownership-neutral for the final runtime crate.

Only after that one-shot transition is separately reviewed should source/API placement or a new dependency edge be considered.

## Validation boundary

Static repository/design review only.

No Rust source, Cargo manifest, lockfile, runtime wiring, socket behavior, STUN/ICE/TURN activation, QUIC activity, Agent/bootstrap state, deployment, signing, privileged mutation, PR, or merge is authorized or performed by this checkpoint.
