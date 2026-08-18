# Phase 152 C02e — Test-Only Candidate Freshness Authority Reference

Status: `SOURCE_SPEC_STAGED / TEST_ONLY_OPAQUE_FRESHNESS_STATES / ADMISSION_BEFORE_FRESHNESS / ACTUAL_PLAN_REFRESH_VALIDATION / STAGED_CLONE_BEFORE_COMMIT / SAME_PEER_SESSION_CONTINUITY / REQUESTER_INDEPENDENT / UNAVAILABLE_STATE_FAIL_CLOSED / PRODUCTION_REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED / NOT_EXECUTED`

Base C02e head: `15d3add736d94432388e83106ee677d45d1eb456`

Ordering corrective head: `bbdb27d167fa28944cc110aa1fdc3832917b5d7d`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The freshness-authority placement checkpoint established that candidate-publication freshness belongs to the upper reachability composition lifecycle and must persist across ordinary session renewal for the same current `PeerConnectivityIdentity`.

This checkpoint stages a test-only source reference for those semantics without selecting a production freshness value or persistence/runtime mechanism.

## Test-only representation boundary

`crates/prw-remote-bridge/tests/reachability_freshness_authority_reference.rs` uses the local enum:

- `Initial`;
- `AfterFirstCommit`;
- `AfterSecondCommit`.

These names exist only to create distinct comparable states in an integration-test reference model.

They are explicitly **not** a production generation sequence, wire encoding, counter width, initial numeric value, nonce, timestamp, replay-window definition, persistent schema, or restart protocol.

No public/private production Rust type is added for candidate freshness.

## Exclusive reference owner

The test-only `FreshnessReachabilityReference` owns:

- one actual `PeerConnectivityPlan`;
- one optional test-local current freshness state;
- one optional test-local current traversal lifecycle marker.

Exclusive `&mut self` operations provide the source/disposable serialization model already authorized by the linearization precedent review.

## Admission-only semantic seam

Static review of the first staged version found that freshness was compared before requester/publisher/workspace/target admission, which contradicted the locked C02e ordering.

The private source-only semantic adapter was therefore minimally decomposed with:

`validate_authenticated_publication_admission(...)`

This helper performs the exact existing non-mutating order:

1. requester session registry-current;
2. publisher session registry-current;
3. same current workspace;
4. exact publisher/publication/target-plan peer identity;
5. target `TransportIdentity` registry-current.

`refresh_from_authenticated_publication(...)` now calls this helper and then performs the same existing transactional candidate refresh. No production module export or wire/runtime behavior was added.

The upper test reference calls this admission helper directly so freshness comparison occurs at the correct seam.

## Admission / compare / stage / commit ordering

The corrected staged publication operation follows this sequence:

1. execute actual C02e requester/publisher/workspace/target/transport admission against the authoritative current plan without mutation;
2. require verifier freshness state to be available;
3. compare caller-presented expected test freshness against owner-current test freshness;
4. determine the next state internally from the test authority rather than accepting caller-selected replacement state;
5. clone the authoritative current `PeerConnectivityPlan` into non-authoritative staging state;
6. apply the actual `PeerConnectivityPlan::refresh_candidates(...)` rules to the staged clone using the authenticated publication candidate vector;
7. on candidate error, discard staging and preserve authoritative plan/freshness/traversal state;
8. when staging succeeds and no fallible work remains, replace the authoritative plan, advance test freshness, and invalidate the current traversal marker inside the same exclusive-owner operation.

This preserves the locked C02e order:

`identity/workspace/target currentness -> publication freshness -> candidate validation -> accepted commit`

The staged clone is not a second authoritative plan model. It is transaction scratch state used to prove validate-before-commit behavior, analogous to repository precedents that prepare a complete next state before assignment.

## Stale freshness error ordering

A stale/duplicate expected freshness is visible only after current requester/publisher/workspace/target/transport admission succeeds.

An unauthorized, stale-session, cross-workspace, retargeted, revoked, or stale-transport caller therefore receives the corresponding admission failure rather than learning publication-freshness state first.

After admission succeeds, stale freshness fails before candidate-plan staging or authoritative mutation.

## Candidate validation failure

A candidate validation failure occurs only after current admission and matching expected freshness.

Because validation runs against a private plan clone, failure does not mutate authoritative plan state, does not advance freshness, and does not invalidate the current traversal lifecycle.

## Same-peer session renewal case

The fixture retains the target signer/binding and creates a second authenticated `SessionId` for the same enrolled target after one successful publication commit.

The renewed session may publish for the same unchanged `DeviceId + TransportIdentity`, but the freshness owner continues from `AfterFirstCommit`; it does not reset to the test `Initial` state.

This stages the locked rule that authentication renewal is provenance renewal, not candidate-publication replay-baseline reset.

## Requester-independence case

The fixture contains two distinct authenticated requesters in the same workspace.

After requester A commits the target publication from the initial freshness state, requester B cannot consume the same stale expected freshness as if it had a separate replay namespace.

The owner-current publisher freshness remains shared for that target peer lifecycle.

## Unavailable-state / restart analogue

The reference owner can be constructed with no current freshness state.

After current identity/workspace/target admission succeeds, publication commit fails `FreshnessUnavailable` before candidate-plan staging or mutation rather than assuming/resetting to the test initial baseline.

This is a source-level analogue for the locked production rule that restart/failover without recoverable current freshness must fail closed.

It does not select durable storage or recovery mechanics.

## Staged cases

The test source stages:

1. successful publication advances freshness, resets plan observation state through actual plan refresh, and invalidates prior traversal;
2. duplicate/stale expected freshness is rejected after current admission and before plan mutation;
3. candidate-ID rebinding failure does not consume current freshness;
4. same-peer new `SessionId` continues the existing freshness lifecycle;
5. a second requester does not obtain an independent freshness namespace;
6. unavailable verifier state fails closed without implicit baseline reset.

## Security boundaries

The reference must not be interpreted as:

- choosing a production freshness enum or counter;
- choosing a production initial value;
- selecting `prw-remote-bridge` as production freshness owner;
- making plan cloning the production transaction architecture;
- selecting a mutex, database, actor or channel;
- selecting persistence/restart mechanics;
- providing a wire field;
- replacing actual Phase 141 traversal currentness requirements.

## Mutation surface

The staged reference/corrective surface is limited to:

- this contract;
- `crates/prw-remote-bridge/tests/reachability_freshness_authority_reference.rs`;
- the unexported source semantic adapter `crates/prw-remote-bridge/src/candidate_reachability.rs` only to extract the admission-only helper and make the existing refresh path reuse it;
- static audit evidence.

No Cargo manifest, lockfile, production module export, registry/session implementation, Phase 141 source, C02d source, runtime/network/deployment state or immutable authority changes.

## Validation state

Source is staged only. No rustfmt, compiler, Clippy, test or build evidence exists until the build gate is separately opened.

## Next safe seam

Complete static source review of the corrected ordering, then review the still-open **replacement peer freshness initialization / re-baselining handshake** requirements without choosing the concrete value representation.
