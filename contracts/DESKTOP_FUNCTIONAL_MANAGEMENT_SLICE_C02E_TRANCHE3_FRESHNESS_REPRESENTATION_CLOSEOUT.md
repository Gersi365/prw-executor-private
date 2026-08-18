# Desktop Functional Management Slice C02e — Tranche 3 Freshness Representation Closeout

Status: `TRANCHE3_FRESHNESS_REPRESENTATION_PERSISTENCE_RECOVERY_PASS`

## Authority

This checkpoint closes the Phase 152 C02e Tranche 3 selection and executable validation of candidate-publication freshness representation plus persistence/recovery semantics.

Frozen predecessor C02d remains:

`857583b25ed1206317641a93fd8f927819c954d8`

Tranche 2 closeout base remains:

`f1f3d58bf3128da377e4e8838648bcf52992ca3a`

The exact Tranche 3 validated head was:

`db25639dace7531336f0f00b2f245642441d983e`

The authoritative PASS evidence was committed by GitHub Actions as child commit:

`17d6520eef8946319b64c7e512b3ec5b45e79408`

Authoritative report:

`logs/audits/phase-152-c02e-dynamic-reachability-design/C02E_TRANCHE3_FRESHNESS_VALIDATION_db25639dace7531336f0f00b2f245642441d983e.txt`

Report blob:

`9446d6c987d84ba46bbb3395375077f13355e516`

## Selected representation

C02e now selects one production candidate-publication freshness value representation:

**opaque verifier-issued non-zero 256-bit token / exactly 32 bytes.**

The dormant provider-neutral source is:

`crates/prw-remote-bridge/src/candidate_publication_freshness.rs`

Locked representation properties are:

- exact width: 32 bytes;
- all-zero value invalid;
- no numeric, monotonic-counter, timestamp, duration, request-ID, session-ID, candidate-ID, endpoint or clock semantics;
- authoritative token installation is verifier-owned;
- token generation for production installation requires a cryptographically secure verifier-owned entropy source;
- successful compare-and-commit installs a distinct replacement token;
- token equality is scoped by exact `DeviceId + TransportIdentity` peer lifecycle;
- the token is replay-ordering state, not authentication or authorization authority.

The source remains dormant/unexported in this tranche. It selects no RNG implementation, store, codec, task or network adapter.

## Durable lifecycle closeout

The production logical durability model is now locked to four disjoint lifecycle states:

1. `NewLifecycleEligible(token)` — explicitly authorized historically-new peer lifecycle with verifier-issued bootstrap token;
2. `Established(token)` — existing peer lifecycle with exact recoverable current freshness;
3. `RecoveryRequired` — existing lifecycle whose exact current verifier freshness cannot be authoritatively recovered;
4. `Retired` — historical peer lifecycle tombstone that is not bootstrap-eligible.

Storage absence is never equivalent to `NewLifecycleEligible`.

`RecoveryRequired` fails closed for candidate publication and carries no current token.

`Retired` prevents a historical exact `DeviceId + TransportIdentity` from silently becoming a fresh replay namespace if those exact transport-identity bytes become current again.

## Persistence / atomicity closeout

The persistence product and synchronization primitive remain unselected, but required semantics are now locked.

A successful publication must provide one linearizable compare/validate/commit boundary over:

`exact current authenticated peer + expected durable freshness token + complete valid candidate vector`

`-> refreshed current plan + distinct verifier-issued replacement token + previous traversal lifecycle stale`

A stale token, invalid candidate vector or aborted commit is non-consuming.

Acknowledgment/current-token delivery may occur only after the durable commit is known successful.

No conforming implementation may persist freshness separately from candidate/traversal authority in a way that permits one side to become authoritative without the other.

## Crash / recovery semantics

The following behavior is locked:

- crash before commit: old plan/token remains authoritative and ordinary retry with the same current token may proceed after normal authentication/currentness checks;
- crash after durable commit but before response: replacement token/plan is authoritative and retry with the old token fails;
- restart/failover with complete authoritative durability: restore exact current lifecycle/token, without reset;
- missing, corrupt, ambiguous or non-authoritative freshness for an existing lifecycle: `RecoveryRequired`, fail closed;
- no zero/default/random-on-restart/missing-row bootstrap is permitted for an existing lifecycle;
- automatic same-identity re-baselining remains forbidden.

## Authenticated token resynchronization

Tranche 3 also locks a narrow lost-delivery recovery semantic.

When durable freshness remains authoritative but the legitimate publisher no longer possesses its current token, the authority may re-deliver the **same exact current token without advancing or resetting freshness** only after current authenticated publisher-session, registry and exact transport-identity revalidation.

This is current-state resynchronization, not re-baselining.

No wire message kind, endpoint, codec or runtime implementation is selected by this rule.

## Transport rotation / historical reuse

The current registry compare-and-rotate contract does not itself prove that replacement transport-identity bytes have never appeared previously for the same device.

Therefore:

- old peer freshness becomes retired when that peer lifecycle is superseded;
- a durable `Retired` record for an exact `DeviceId + TransportIdentity` blocks automatic new-lifecycle bootstrap if those exact bytes later appear current again;
- registry currentness alone is insufficient evidence of historical freshness-lifecycle novelty;
- no Phase 130 registry source is changed by this tranche.

A future identity-incarnation or same-identity re-baseline mechanism, if desired, requires separate security review.

## Executed validation result

The exact validated head passed:

1. locked Cargo metadata;
2. full workspace rustfmt check;
3. focused candidate-publication freshness representation test;
4. focused `prw-remote-bridge` Clippy with all targets/features and warnings denied;
5. full locked workspace Clippy;
6. full locked workspace tests;
7. full locked workspace build;
8. tracked Cargo target-cache normalization;
9. source/manifest/lock hash stability checks;
10. final tracked-diff zero check.

Authoritative final markers are:

- `LOCKED_METADATA_RC=0`;
- `FORMAT_RC=0`;
- `FOCUSED_TEST_RC=0`;
- `FOCUSED_CLIPPY_RC=0`;
- `WORKSPACE_CLIPPY_RC=0`;
- `WORKSPACE_TESTS_RC=0`;
- `WORKSPACE_BUILD_RC=0`;
- `PRE_NORMALIZE_DRIFT_RC=0`;
- `TARGET_RESTORE_RC=0`;
- `HASH_DRIFT_RC=0`;
- `FINAL_DRIFT_RC=0`;
- `FIRST_FAILURE=NONE`;
- `STATUS=PASS`.

`Cargo.lock` remained byte-stable at SHA-256:

`becbd46de66354591afd3a4d755a9b4ba06f9c9c15045069b85e04a99525423a`

No Cargo manifest or dependency graph mutation was made by Tranche 3.

## Failure-history classification

The first executable Tranche 3 run stopped only at deterministic rustfmt formatting. Locked metadata and all hash/drift checks available at that point were clean. The reported formatter changes were applied mechanically without altering the representation or persistence/recovery decision.

The later exact validated head passed all focused and full-workspace gates and supersedes the formatter-only failure as the closeout result. Historical evidence is retained rather than rewritten.

## Temporary harness cleanup

The temporary Tranche 3 validator removed itself in the authoritative PASS evidence commit. Its absence from the final closeout tree is evidence-harness cleanup only and does not alter the validated source, contract or Cargo state.

## Boundaries that remain closed

This closeout does **not** select or activate:

- the concrete production upper reachability composition owner crate/module;
- a persistence/database product;
- database schema byte encoding or serialization;
- distributed replication/consensus implementation;
- synchronization, mutex, actor, async task, cancellation or queue primitive;
- control-plane freshness message kind or wire field;
- token wire encoding or token delivery/resynchronization endpoint;
- production normal dependency from `prw-remote-bridge` to `prw-nat-traversal`;
- production traversal owner/runtime integration;
- real STUN/ICE/TURN, QUIC, TCP/UDP, forwarding or other network I/O;
- deployment, signing, privileged host mutation, PR or merge;
- automatic same-identity freshness re-baselining.

Result: **TRANCHE 3 CLOSED / PASS.**
