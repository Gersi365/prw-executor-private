# Phase 152 C03e-GT — Production Reachability Freshness-Token Source Materialization

Status: `STAGING`

Target gate:
`C03E_GT_PRODUCTION_REACHABILITY_FRESHNESS_TOKEN_SOURCE_MATERIALIZED`

## Purpose

C03e-GT source-materializes only the C03e-GS-selected concrete production freshness-token source
behind the existing provider-neutral `CandidatePublicationFreshnessTokenSource` trait.

This checkpoint does not construct a durable-store provider, bootstrap a new reachability lifecycle,
populate the production owner map, activate candidate handoff/current-Mesh response execution, open a
listener, start traversal/dialing, deploy, restart, merge, or alter repository visibility.

## Exact predecessor

GT is rooted directly at canonically closed C03e-GS:

- GS PR `#321`;
- exact GS head `091fb06a5bd20afc10b77495b9784a45995a245a`;
- exact GS tree `dfa56ebb484c08002e81de01bdbad5ab376236c0`;
- GS gate `C03E_GS_PRODUCTION_REACHABILITY_FRESHNESS_TOKEN_GENERATION_ORDER_SEMANTICS_SELECTED`;
- GS remains intentionally draft/open/unmerged.

No intermediate branch, merge result, `main`, or reconstructed local state is a permitted GT base.

## Fresh exact-GS-head source audit

The exact GS source establishes:

1. `prw-remote-bridge::reachability_owner::CandidatePublicationFreshnessTokenSource` is already the
   sole provider-neutral production freshness issuance port.
2. `CandidatePublicationFreshnessToken` already fixes the exact 32-byte non-zero representation.
3. `ProductionReachabilityOwner` already fixes issuance location/order and exact-current repetition
   handling; GT must not duplicate or move those semantics.
4. Agent production-owner custody remains generic over `T: CandidatePublicationFreshnessTokenSource`,
   so the concrete source belongs on the Agent composition side.
5. `crates/prw-agent/Cargo.toml` already pins `aws-lc-rs = "=1.18.0"`, feature set `alloc, non-fips`,
   but only as a dev-dependency. Production compilation therefore requires promotion of that exact
   existing dependency declaration into `[dependencies]`; no version or feature change is selected.
6. No existing C03e-GT branch/PR or concrete Agent-owned production freshness-token source exists.

## Authorized final path scope

GT authorizes only these four paths unless compiler evidence proves an unavoidable contradiction:

1. `crates/prw-agent/Cargo.toml` — promote the already-pinned `aws-lc-rs` dependency from dev-only to
   production dependency without changing version/features;
2. `crates/prw-agent/src/lib.rs` — register exactly one dormant crate-internal source module;
3. `crates/prw-agent/src/production_reachability_freshness_token_source.rs` — concrete stateless
   `SystemRandom` implementation plus focused tests;
4. this GT source-materialization contract.

No `Cargo.lock` or Android-native lockfile change is expected because the exact package/version is
already present in the locked workspace through the existing Agent dev-dependency. If canonical
`--locked` validation proves otherwise, GT must stop and re-audit rather than silently editing a
lockfile or broadening scope.

## Materialized law

### Concrete provider

`ProductionReachabilityFreshnessTokenSource` implements the existing
`CandidatePublicationFreshnessTokenSource` trait using:

`aws_lc_rs::rand::{SecureRandom, SystemRandom}`.

The source is Agent-owned and stateless.

### Exact issuance

One `issue_token()` call:

1. allocates one fresh local `[u8; CANDIDATE_PUBLICATION_FRESHNESS_TOKEN_BYTES]` buffer;
2. performs exactly one `SystemRandom::new().fill(&mut bytes)` call;
3. maps provider failure to existing `FreshnessTokenSourceError::Unavailable`;
4. delegates exact non-zero representation validation to
   `CandidatePublicationFreshnessToken::new(bytes)`;
5. maps all-zero rejection to existing `FreshnessTokenSourceError::Unavailable`;
6. returns the exact typed token on success.

There is no retry loop, second fill, fallback generator, token pool, counter, UUID, time input,
request/candidate/endpoint input, identity-derived seed, or persistence interaction.

### Existing owner law remains authoritative

GT does not pass the current token into the concrete source and does not compare replacement
freshness inside the source.

Existing `ProductionReachabilityOwner::commit_candidate_publication(...)` remains responsible for:

- exact current publisher freshness validation;
- one issuance after staged candidate-plan validation;
- `ReplacementFreshnessUnchanged` when one issued token equals exact current state;
- no second issuance after exact-current repetition;
- durable compare-and-commit;
- local freshness/plan install only after definite durable success.

Recovery, authenticated freshness resynchronization, and retirement remain zero-generation paths.

## Focused validation expectations

The new module carries focused tests proving:

- a deterministic non-zero fill is invoked exactly once and yields exact bytes;
- provider failure is invoked once and fails closed;
- all-zero provider output is invoked once and fails closed;
- the production `SystemRandom` implementation returns a typed non-zero token under validation.

The private test seam accepts `FnOnce`, structurally preventing an internal retry loop from calling a
single injected fill provider more than once.

## Explicit exclusions

GT does not select or materialize:

- new-lifecycle/bootstrap freshness callsite/order;
- concrete durable reachability provider, schema, serialization, credentials, replication, or CAS
  implementation;
- production owner recovery/population/synchronization;
- candidate handoff or current-Mesh candidate execution activation;
- requester/rendezvous changes;
- response I/O or worker/cancellation integration;
- traversal, STUN/ICE/TURN/relay activation;
- listener/readiness/runtime bootstrap;
- production networking, firewall/route/DNS/TUN/TAP/systemd mutation;
- deployment, restart/recovery operation, merge, branch deletion, or visibility mutation.

## Validation gate

GT may close only when all of the following hold on one exact final head:

1. exact GS merge base, ahead-only lineage;
2. final changed-path set remains within the authorized scope or a separately documented compiler
   contradiction is explicitly resolved;
3. `cargo metadata --locked` / canonical locked dependency graph succeeds without unauthorized lock
   mutation;
4. canonical rustfmt succeeds;
5. canonical Clippy with warnings denied succeeds;
6. workspace tests succeed;
7. workspace build succeeds;
8. Android validation, if automatically triggered, succeeds on the same final head;
9. no failing or pending automatically triggered workflow remains;
10. immutable Drive audit is stored in the canonical Private Remote Workspace folder and verified by
    raw byte/hash readback.

Successful closure classification:
`CLOSED_PRODUCTION_REACHABILITY_FRESHNESS_TOKEN_SOURCE_MATERIALIZATION`.

Until that closure, this checkpoint remains `STAGING`, draft, open, and unmerged.
