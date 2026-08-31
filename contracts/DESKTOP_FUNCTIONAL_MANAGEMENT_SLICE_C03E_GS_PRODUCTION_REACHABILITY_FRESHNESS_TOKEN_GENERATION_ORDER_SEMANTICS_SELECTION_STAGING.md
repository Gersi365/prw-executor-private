# Phase 152 C03e-GS — Production Reachability Freshness-Token Generation/Order Semantics Selection

Status: `STAGING`

Target gate:
`C03E_GS_PRODUCTION_REACHABILITY_FRESHNESS_TOKEN_GENERATION_ORDER_SEMANTICS_SELECTED`

## Purpose

C03e-GS selects exactly one missing production semantic seam after canonically closed C03e-GR:
the concrete production mechanism and ordering contract for verifier-owned candidate-publication
freshness-token issuance used by `ProductionReachabilityOwner`.

This checkpoint is documentation-only. It does not materialize a token source, change a Cargo
manifest or lockfile, construct a production durable store, populate production owner custody,
activate candidate handoff, open network listeners, or perform deployment/runtime mutation.

## Exact predecessor

C03e-GS is rooted directly at the exact canonically closed C03e-GR head:

- repository: `Gersi365/prw-executor-private`;
- predecessor PR: `#320` — `Phase 152 C03e-GR: materialize awaitable production reachability durable execution path`;
- exact GR head: `a20705874416544215e93f12e916b21957eee542`;
- exact GR tree: `c12e33f34b466b47871d4180e4974c3819a867e6`;
- GR remains draft/open/unmerged while canonically `Status: CLOSED`;
- GR explicitly left `production freshness-token source construction` unmaterialized.

No intermediate branch, merge result, `main`, or reconstructed local state is a permitted GS base.

## Fresh read-only audit findings

The exact GR source and historical checkpoint record establish all of the following before GS
selection.

### Existing token representation is already authoritative

`crates/prw-remote-bridge/src/candidate_publication_freshness.rs` at exact GR head already fixes:

- `CANDIDATE_PUBLICATION_FRESHNESS_TOKEN_BYTES = 32`;
- `CandidatePublicationFreshnessToken` as an opaque verifier-issued 32-byte value;
- all-zero as invalid and reserved so missing/default state cannot alias a valid token;
- current freshness as replay/currentness state, not an authentication credential;
- production generation from a cryptographically secure verifier-owned entropy source;
- no numeric, timestamp, request, endpoint, or publisher meaning assigned to token bytes.

Exact audited blob:
`fd7c2f095999b6a6479be79c562637fe5f46634c`.

GS therefore does not redesign token size, representation, lifecycle, or authority meaning.

### Existing production owner already fixes the issuance location and commit order

`crates/prw-remote-bridge/src/reachability_owner.rs` at exact GR head already defines:

- `CandidatePublicationFreshnessTokenSource::issue_token(&mut self)`;
- the requirement for one non-zero opaque token;
- fail-closed `FreshnessTokenSourceError::Unavailable`;
- prohibition on derivation from publisher input, clocks, request IDs, candidate IDs, or endpoints;
- rejection of a generated token equal to the exact current token as
  `ReachabilityOwnerError::ReplacementFreshnessUnchanged`;
- no durable or local state mutation before successful durable compare-and-commit.

The existing `commit_candidate_publication(...)` order is already explicit and must remain:

1. require the owner to be `Current`;
2. validate authenticated publisher identity/workspace/transport admission;
3. obtain the exact current verifier token;
4. require the publisher-presented freshness to equal that exact current token;
5. clone and completely validate the replacement candidate plan;
6. call `token_source.issue_token()`;
7. reject an exact-current replacement token;
8. stage the replacement freshness record and complete durable snapshot;
9. await the exact-current durable compare-and-commit;
10. only after definite durable `Committed`, install local plan/freshness, invalidate any old traversal,
    and return `ReachabilityCommitOutcome`.

Exact audited GR blob:
`fb7543361ea3a144ae9275284b41bf0ef63df2ad`.

GS therefore selects no alternative ordering and introduces no second rotation path.

### Recovery/resynchronization already have durable-state authority

The exact GR owner recovery path is:
`ProductionReachabilityOwner::recover(store, token_source, peer)`.

Recovery loads the existing authoritative durable snapshot and never calls `issue_token()`.
Missing durable state remains `DurableStateMissing`; ambiguous/unavailable durable state remains
fail closed. Recovery is not a freshness rebaseline.

GR also materialized authenticated freshness resynchronization through authoritative durable load
after current identity validation. Resynchronization does not mint a replacement token.

Retirement likewise compares using the exact current token and writes a durable tombstone; it does
not mint a replacement freshness token.

### Historical contracts left the concrete production source deliberately gated

The historical lineage does not contain a conflicting concrete reachability-freshness provider
selection.

Most importantly:

- C03e-GQ PR `#319` explicitly leaves `concrete freshness-token source` separately gated while
  selecting only the awaitable durable-store execution prerequisite;
- C03e-GR PR `#320` explicitly closes its source materialization with `no production
  freshness-token source construction`;
- C03e-GF/GG retain the token source as a generic `T` passed into Agent-owned production owner
  recovery/custody rather than choosing a concrete production implementation.

Earlier candidate-publication checkpoints also repeatedly preserved freshness authority as a
separate concern rather than deriving it from CandidateId, request IDs, SessionId, endpoints,
transport identity, or publisher data.

### Existing repository CSPRNG precedent is reusable

The repository already uses the exact production entropy provider selected elsewhere:

`aws_lc_rs::rand::{SecureRandom, SystemRandom}`.

Existing precedents include:

- C02f-BL PR `#83`, which materialized production 32-byte authority/attempt-ID generation with
  `SystemRandom::fill`, exactly one provider fill per logical generation call, fail-closed provider
  errors, no hidden retry, no deterministic fallback, and no identity/clock-derived bytes;
- C03e-CE PR `#202`, which selected the same provider for one fresh 32-byte verifier `SessionId`;
- exact GR `crates/prw-session/src/prwa_verifier_source.rs`, which materializes one
  `SystemRandom::new().fill(&mut [u8; 32])` per verifier-session context with no collision retry;
- exact GR `crates/prw-session/Cargo.toml`, which binds `aws-lc-rs = "=1.18.0"` with
  `default-features = false` and features `alloc`, `non-fips`.

The exact GR Agent manifest already carries the same `aws-lc-rs` version/features as a
`dev-dependency`; GS does not change its dependency class.

These precedents allow GS to choose an existing audited library/provider rather than inventing a
new cryptographic primitive or entropy design.

## Selected canonical law

### 1. Concrete production entropy provider

The production implementation of the existing
`CandidatePublicationFreshnessTokenSource` contract shall use:

`aws_lc_rs::rand::SystemRandom`

through the existing `SecureRandom::fill` API.

The production source is verifier/Agent owned. Publisher, requester, remote client, control-plane
payload, database row, candidate set, endpoint, request ID, `SessionId`, `DeviceId`,
`TransportIdentity`, wall clock, monotonic clock, process ID, thread ID, counter, UUID, or durable
revision is not entropy authority for this token.

No proprietary PRW cryptographic primitive is selected.

### 2. Production source ownership

The concrete source belongs on the Agent-side production composition boundary that constructs and
retains `ProductionReachabilityOwnerCustody<S, T>`.

This preserves the existing layering:

- `prw-remote-bridge` owns the provider-neutral token-source trait and reachability semantics;
- `prw-agent` owns production composition/custody and therefore the concrete production source;
- publisher/client inputs never construct or inject production freshness tokens;
- the durable-store provider remains a separate production dependency and authority.

GS does not require `prw-remote-bridge` to depend directly on `aws-lc-rs` and does not move the
provider-neutral trait out of `prw-remote-bridge`.

### 3. Exact issuance shape

One call to production `issue_token()` shall:

1. create one fresh local `[u8; CANDIDATE_PUBLICATION_FRESHNESS_TOKEN_BYTES]` buffer;
2. invoke `SystemRandom::new().fill(&mut bytes)` exactly once;
3. on provider failure, return `FreshnessTokenSourceError::Unavailable` with no retry or fallback;
4. pass the filled bytes through the existing `CandidatePublicationFreshnessToken::new(bytes)`
   representation constructor;
5. on all-zero rejection, fail closed as token-source unavailability for the production trait
   boundary;
6. otherwise return exactly that typed token.

There is no second fill inside one logical `issue_token()` call.

There is no deterministic fallback.

There is no retry loop for provider failure, all-zero output, or token collision/repetition.

### 4. Exact-current repetition remains an owner-level failure

The concrete token source does not receive the current token and does not perform comparison against
it.

The existing owner remains the authority for the exact-current comparison immediately after one
successful issuance.

If the newly issued token equals `expected_current`:

- `commit_candidate_publication(...)` returns existing
  `ReachabilityOwnerError::ReplacementFreshnessUnchanged`;
- no second token is generated;
- durable compare-and-commit is not called;
- local plan, freshness, traversal, and owner mode remain unchanged.

This preserves a single generation attempt per candidate-publication commit attempt and avoids a
hidden retry policy.

### 5. Candidate-publication generation/order remains exactly the GR order

For one candidate-publication commit attempt, replacement freshness is generated only after:

- current-owner enforcement;
- authenticated publication admission;
- exact presented-current-freshness equality;
- complete staged candidate-plan validation.

Generation occurs before:

- staged freshness/snapshot finalization;
- durable compare-and-commit;
- local current-state installation;
- traversal invalidation;
- terminal success projection.

Exactly one production `issue_token()` call is permitted along this successful pre-CAS path.

No token is pre-generated before publication validation and no pool/batch of future tokens is
selected.

### 6. Durable CAS determines whether generated freshness becomes current

A generated replacement token has no currentness authority merely because generation succeeded.

It becomes current only as part of the exact complete replacement durable snapshot after
`ReachabilityDurableStore::compare_and_commit(expected_current, replacement)` returns definite
`Committed`.

Before that commit:

- it is staged only;
- it authorizes no publication;
- it is not externally current;
- it must not replace local current freshness.

On `StaleExpected` or ambiguous persistence failure, existing recovery-required behavior remains
authoritative. No new token is generated to hide or retry that result.

### 7. Recovery performs zero generation

`ProductionReachabilityOwner::recover(...)` must retain exactly zero calls to production
`issue_token()`.

Recovery restores only existing durable current freshness. It must not:

- create a replacement token when storage is missing;
- rotate merely because a process restarted;
- turn missing/ambiguous storage into a new lifecycle;
- retry recovery by minting a new token.

The token-source object may be constructed and retained as part of recovered owner custody, but it
remains unused until a later admissible candidate-publication commit reaches the existing issuance
point.

### 8. Authenticated freshness resynchronization performs zero generation

A freshness resynchronization request returns/observes the authoritative durable current freshness
under the already-materialized identity/currentness checks.

It must not call production `issue_token()`, rotate freshness, create a new durable snapshot, or
change the current token merely because a peer needed resynchronization.

### 9. Retirement performs zero generation

Retiring a non-current exact peer lifecycle uses the existing current token as the compare key and
persists the existing retired lifecycle tombstone semantics.

Retirement must not issue a replacement token before or after the tombstone commit.

### 10. Bootstrap remains separately gated

GS does not activate or materialize an authoritative new-lifecycle bootstrap path.

`CandidatePublicationFreshnessLifecycle::NewLifecycleEligible(token)` remains an existing typed
representation whose creation must be authorized by the separate lifecycle/bootstrap authority.

If a later separately gated production bootstrap path needs to mint a bootstrap freshness token, it
must not invent a second entropy mechanism; it must preserve the same 32-byte, non-zero,
verifier-owned CSPRNG law unless a future explicit contract supersedes GS. The bootstrap callsite,
transaction order, durable creation semantics, and lifecycle authorization remain outside GS.

### 11. Source is stateless entropy custody, not durable freshness authority

The production token-source implementation owns no persistent sequence, cache, token pool, last
value, database handle, clock, thread, task, channel, executor, retry budget, or lifecycle map.

The durable freshness record remains authoritative for currentness.

The source is only a stateless mechanism for obtaining one candidate replacement value when the
existing owner reaches the selected issuance point.

### 12. No new externally visible error taxonomy

GS selects no new peer-visible error or protocol status.

Production provider failure, all-zero rejection, and other inability to produce a valid token are
collapsed through the existing provider-neutral `FreshnessTokenSourceError::Unavailable` boundary.

Exact-current equality remains the already-existing
`ReachabilityOwnerError::ReplacementFreshnessUnchanged` classification.

Existing candidate-publication terminal-result projection remains unchanged and does not disclose
provider internals.

## Security and identity invariants preserved

- Freshness is verifier-owned replay/currentness state, not authentication.
- `DeviceId` remains logical device identity.
- `TransportIdentity` remains lower-transport certificate identity.
- `SessionId` and request IDs remain correlation/authentication-session state in their existing
  domains, not freshness entropy.
- `CandidateId` remains plan-scoped candidate identity/correlation, not freshness entropy.
- endpoint/address/path state is transient reachability data, not freshness entropy.
- randomness provider success is not authorization.
- generated token possession is not device/user identity or capability authority.
- private cryptographic material is not uploaded or derived by GS.

## Explicitly rejected alternatives

GS rejects all of the following production freshness mechanisms:

- timestamps or clock-derived freshness;
- monotonic counters or database sequences;
- UUID-based freshness;
- hashing publisher/request/candidate/endpoint data;
- derivation from DeviceId, TransportIdentity, SessionId, request ID, CandidateId, IP/port, or
  workspace/user identity;
- deterministic PRNG seeds;
- publisher-provided replacement freshness;
- requester-provided replacement freshness;
- database-generated random/default values as the source authority;
- reusable token pools or pre-generated batches;
- hidden retry-until-different behavior;
- hidden retry on provider failure or all-zero output;
- generation during recovery, resynchronization, or retirement;
- a second production reachability-owner model or second rotation path.

## Documentation-only changed-path bound

C03e-GS itself may change exactly one repository path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GS_PRODUCTION_REACHABILITY_FRESHNESS_TOKEN_GENERATION_ORDER_SEMANTICS_SELECTION_STAGING.md`

Any Rust, Kotlin, manifest, lockfile, workflow, Agent executable, bridge, transport, provider,
database, configuration, deployment, or second contract path is outside GS and blocks closure.

## Validation requirements

Because GS is docs-only, canonical exact-head validation requires:

- exact GR merge base and one-path compare proof;
- PRW Rust Validation on exact GS head, including locked dependency graph, rustfmt, Clippy,
  workspace tests, and workspace build;
- path-filtered workflow outcomes recorded exactly as triggered/skipped;
- Android must not be reported as PASS unless an Android workflow actually triggers for exact GS
  head;
- no superseded or predecessor CI may be reused as GS exact-head PASS evidence.

## Future source-materialization boundary

After canonical GS closure, any source successor must begin with a fresh exact-GS-head audit.

The expected narrow materialization is an Agent-owned implementation of the existing
`CandidatePublicationFreshnessTokenSource` using `SystemRandom` and the exact selected one-fill /
no-retry semantics, plus only compiler/Cargo-required registration/dependency paths proven by that
fresh audit.

The future source successor must not automatically include:

- concrete durable-store provider/schema/credentials;
- production owner-map population;
- Agent candidate-handoff activation;
- new-lifecycle bootstrap materialization;
- current-Mesh response activation;
- traversal/dialing;
- listener/readiness/runtime bootstrap;
- deployment/restart/recovery actions;
- merge or branch deletion.

If compiler or locked-graph evidence requires a broader path set than the fresh source audit
predicts, that contradiction must stop source closure and be handled by a separately reviewed
scope correction rather than silently broadening GS.

## Closure condition

C03e-GS may be classified `PASS` only when all of the following are true on one exact final GS
head:

1. direct ancestry from exact GR head is proven;
2. exactly one docs-only contract path differs from GR;
3. the selected `SystemRandom` / one-fill / no-retry / existing-owner-order law is byte-stable in the
   final contract;
4. exact-head canonical Rust validation is successful;
5. all automatically triggered workflow states are accounted for with no pending/failing exact-head
   workflow;
6. immutable project evidence records the exact head/tree/compare/CI result and raw byte/hash
   verification;
7. no source/runtime/network/provider/database/deployment/merge mutation has occurred.

Until those conditions are met, the target gate is not claimed.
