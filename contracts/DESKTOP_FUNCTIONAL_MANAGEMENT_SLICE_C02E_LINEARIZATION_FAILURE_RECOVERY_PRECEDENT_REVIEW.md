# Phase 152 C02e — Linearization and Failure-Recovery Precedent Review

Status: `PASS_STATIC_PRECEDENT_REVIEW / EXCLUSIVE_OWNER_REFERENCE_MODEL_SUPPORTED / VALIDATE_THEN_COMMIT / PRECOMMIT_FAILURE_NONDESTRUCTIVE / POSTCOMMIT_RECOVERY_FORWARD_ONLY / PRODUCTION_SYNC_PRIMITIVE_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO`

Review base head: `2cf79c83213d192c4bd62a82ef6cceb751111699`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The C02e one-shot reachability composition checkpoint requires candidate-publication freshness, plan refresh, traversal invalidation and observation admission to behave as one logically ordered lifecycle transition.

This review determines what repository precedent exists for:

- expected-current compare-and-commit;
- validation before mutation;
- single-use state consumption;
- pre-commit resource staging and cleanup;
- post-commit failure handling;
- source/disposable linearization without prematurely selecting production synchronization infrastructure.

## Precedent 1 — registry expected-current compare-and-rotate

`WorkspaceDeviceRegistry::rotate_transport_identity(...)` owns the current device transport binding through `&mut self`.

Its ordering is explicit:

1. reject an unchanged replacement;
2. locate the current registered device;
3. reject revoked/unbound state;
4. read the exact current `TransportIdentity`;
5. reject when current state differs from caller-supplied `expected_current`;
6. assign the replacement only after every check succeeds.

A stale expected value therefore cannot mutate the registry.

This is an in-memory compare-and-commit model whose logical linearization point is the final state assignment under exclusive mutable ownership.

## Precedent 2 — session-auth single-use compare/verify/consume

Phase 128 locks verifier-owned freshness and requires equivalent atomic compare-and-consume semantics for a future durable implementation.

`SessionAuthenticationService::submit_proof(...)` demonstrates the in-memory source model:

1. reject an already completed session;
2. obtain the exact pending challenge state;
3. verify replay/time/session/nonce/signature semantics;
4. construct the authenticated identity only after verification succeeds;
5. insert the completed authenticated state;
6. remove the pending challenge only after success.

Invalid proof does not consume the pending challenge. A later correct proof may still succeed.

The contract explicitly states that a future durable service must preserve equivalent atomic compare-and-consume behavior rather than copying the in-memory storage mechanism literally.

## Precedent 3 — transactional candidate refresh

`PeerConnectivityPlan::refresh_candidates(...)` owns its candidate vector, observations and candidate-ID high-water mark through `&mut self`.

It:

1. validates the complete proposed candidate vector;
2. rejects capacity, rebinding, retired-ID reuse and duplicates before mutation;
3. computes the next high-water mark;
4. constructs the complete refreshed `Unknown` observation vector;
5. assigns refreshed candidates and high-water state only after validation succeeds.

The documented invariant is that every error preserves the complete previous candidate/observation/high-water state.

This is a direct validate/stage/commit precedent for C02e plan state.

## Precedent 4 — one-shot scheduler staged acquisition and final registration

Phase 084 `schedule_one_authenticated_worker(...)` composes multiple fallible ownership steps:

1. acquire bounded worker capacity;
2. perform one authenticated accept;
3. clone cancellation authority;
4. compose the authenticated session;
5. spawn the scoped worker;
6. finally register the worker handle plus cancellation authority.

Every failure before registry insertion releases/drops owned capacity/connection state instead of leaving a half-registered worker.

The final registry insertion is the ownership commit. Later worker completion/failure is handled through registered lifecycle accounting and reaping rather than pretending the pre-registration state still exists.

## Derived repository pattern

These precedents establish a consistent source-level pattern:

### 1. One owner serializes one in-memory transition

For source/disposable validation, an aggregate authority may own the relevant mutable state and expose one `&mut self` transition. Exclusive mutable ownership is sufficient to make operations linearly ordered inside that reference model.

This does **not** select a production mutex, database transaction, actor, task, channel or distributed lock.

### 2. Validate expected-current state before mutation

Caller-presented expected state is evidence to compare against verifier/owner current state, never authority by itself.

Stale expected state must fail before current state changes.

### 3. Stage all rejectable/fallible preconditions before commit where possible

Validation, capacity/resource acquisition and derivation that can fail without changing the accepted current state should happen before the commit boundary.

Failure before commit must leave the authoritative current state intact or release staged resources automatically/explicitly.

### 4. Make the accepted commit small and ownership-complete

The commit should consist of already validated/infallible state moves whenever possible.

After the commit, every later operation must observe the new ownership/lifecycle state; no stale previous authority may remain admissible.

### 5. Post-commit failures recover forward, not by resurrecting stale authority

Once a transition has legitimately committed, later failure is handled inside the new lifecycle state.

The scheduler does not make an already registered worker "unaccepted" if that worker later terminates; the registry/reaper handles the new state.

For C02e, the analogous rule is that replacement traversal failure after an accepted candidate publication does not reactivate the pre-refresh traversal lifecycle.

### 6. Durable/parallel implementations must preserve equivalent semantics

The source model's `&mut self` exclusivity is a reference semantics, not a production synchronization mandate.

If later state is split across threads/processes/services/durable stores, the selected runtime must provide an equivalent linearizable compare-and-commit boundary or fail closed.

## C02e reference-model consequence

The repository precedent is sufficient to authorize a **source/disposable exclusive-owner reference model** for reachability composition, provided it does not claim production placement or synchronization.

Such a reference model may prove that one mutable owner can serialize:

- candidate-publication expected-current freshness admission (through an abstract/test authority until representation is reviewed);
- candidate-plan refresh;
- prior traversal invalidation;
- current traversal replacement ownership;
- observation admission from only the currently owned traversal lifecycle.

The reference model must not expose stale `CandidateReachabilityUpdate` values for later application outside the owner boundary. If an observation is polled from the current traversal session, application/currentness checking must remain within the same serialized owner operation or carry separately reviewed lifecycle evidence.

## Important representation boundary

This checkpoint does **not** authorize inventing the candidate-publication freshness value itself.

A reference harness may use an abstract test-only compare-and-consume authority solely to prove ordering, but it must not designate that test representation as the production counter/nonce/token/wire field.

Likewise, the source reference model must not create a production traversal generation identifier merely to make tests convenient.

## Test-only placement remains separately reviewed

Although the exclusive-owner reference semantics are now supported, the current production crate graph still has no owner with dependencies on all required domains.

Therefore this checkpoint does not add a Cargo edge or choose a crate.

The next safe seam is to review whether an **integration-test-only dependency/harness** can compose current `prw-connectivity`, Phase 141 traversal and C02e admission semantics without changing the production dependency graph or runtime architecture.

## Security invariants

A later reference implementation must prove:

- stale expected publication state cannot mutate plan/traversal state;
- candidate validation failure does not consume freshness;
- successful refresh invalidates the old traversal before any later observation application;
- an observation racing refresh is ordered entirely before or entirely after the refresh boundary;
- old traversal state is never resurrected after commit;
- transport rotation remains a replacement-plan boundary;
- no production synchronization or wire semantics are inferred from the test model.

## Validation boundary

Static source/design precedent review only.

No source code, Cargo manifest, lockfile, build, formatting, lint, test, workflow, network I/O, traversal activation, Agent/bootstrap state, deployment, signing, privileged mutation, PR, or merge is modified or executed by this checkpoint.
