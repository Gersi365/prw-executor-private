# C02e Tranche 3 Freshness Representation / Persistence / Recovery Audit

Status: `STAGED_FOR_EXECUTABLE_VALIDATION`

Base head:

`f1f3d58bf3128da377e4e8838648bcf52992ca3a`

Frozen predecessor C02d:

`857583b25ed1206317641a93fd8f927819c954d8`

## Scope

This audit records the Tranche 3 decision surface for production candidate-publication freshness representation and durable recovery semantics while preserving the closed production runtime/network/composition boundary.

Reviewed prior C02e authority includes:

- candidate-publication freshness/replay checkpoint;
- freshness authority placement;
- bootstrap/re-baseline lifecycle;
- test-only freshness authority reference;
- current Phase 130 transport-identity rotation behavior;
- Tranche 2 actual Phase141 integration closeout.

## Representation decision

Selected representation:

`CandidatePublicationFreshnessToken = opaque non-zero [u8; 32]`

Properties:

- 256 bits / exactly 32 bytes;
- all-zero invalid;
- no numeric/counter/timestamp semantics;
- verifier-issued only for authoritative installation;
- replacement token must differ from current token;
- token is replay-ordering state, not authentication authority.

Dormant source:

`crates/prw-remote-bridge/src/candidate_publication_freshness.rs`

Executable representation tests:

`crates/prw-remote-bridge/tests/candidate_publication_freshness_representation.rs`

## Durable lifecycle decision

Selected logical lifecycle dispositions:

1. `NewLifecycleEligible(token)`;
2. `Established(token)`;
3. `RecoveryRequired`;
4. `Retired`.

Storage absence is never equivalent to `NewLifecycleEligible`.

`RecoveryRequired` contains no current token and fails closed for publication.

`Retired` is a durable semantic tombstone preventing historical exact peer identity from being silently treated as a fresh replay namespace.

## Registry precedent finding

Current Phase 130 `rotate_transport_identity` rejects only an unchanged replacement and stale expected-current state. It does not itself retain a used-transport history that proves a replacement transport value is globally/newly unused for that device.

Therefore Tranche 3 does not infer bootstrap eligibility from registry currentness alone. If the exact `DeviceId + TransportIdentity` has historical retired freshness state, automatic bootstrap is forbidden.

No Phase 130 registry source is modified in this tranche.

## Atomic persistence decision

A successful candidate publication must behave as one linearizable compare/validate/commit operation over:

- exact expected durable freshness token;
- current authenticated peer identity;
- complete validated candidate plan;
- replacement verifier-issued freshness token;
- prior traversal invalidation.

Failed freshness comparison, failed candidate validation or aborted durable commit is non-consuming.

The persistence technology remains unselected.

## Recovery decision

### Durable state intact, token delivery lost

Authenticated current-token resynchronization is allowed as read/re-delivery of the same current verifier token after exact current publisher session/registry/transport validation.

This operation does not rotate freshness and is not a re-baseline.

### Durable state unavailable/corrupt/ambiguous

Publication enters/remains `RecoveryRequired` and fails closed.

No default, zero, random replacement or missing-row bootstrap is allowed for the existing identity.

### Same-identity re-baseline

Still forbidden and remains a separate future security review.

## Runtime boundary

This tranche does not select or activate:

- production upper reachability owner;
- persistence backend;
- runtime synchronization/task model;
- wire message/field allocation;
- socket/network/STUN/ICE/TURN/QUIC I/O;
- production traversal owner;
- deployment/signing/system mutation;
- PR/merge.

## Validation plan

The Tranche 3 validator must prove:

1. Cargo manifests and `Cargo.lock` are byte-stable;
2. `cargo fmt --all -- --check` passes;
3. focused Tranche 3 representation test passes under `--locked`;
4. focused `prw-remote-bridge` Clippy with all targets/features and warnings denied passes;
5. full workspace Clippy/tests/build pass under `--locked`;
6. tracked Cargo `target/` noise is normalized before final drift evaluation;
7. final tracked diff after normalization is empty;
8. no production network/runtime activation occurs;
9. authoritative evidence is committed and the temporary validator removes itself on PASS.

Final status remains pending executable validation.
