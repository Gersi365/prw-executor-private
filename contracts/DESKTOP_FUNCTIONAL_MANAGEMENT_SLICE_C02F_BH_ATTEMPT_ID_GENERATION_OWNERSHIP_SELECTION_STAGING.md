# Phase 152 C02f-BH — Attempt-ID Generation Ownership Selection Staging

## Status

Documentation-only selection checkpoint after validated C02f-BG.

C02f-BH selects trusted generation ownership and operational domain separation for the two 32-byte attempt identifiers required by the already-materialized normal acquisition path. It does not add Rust source, generate any production identifier, perform provider I/O, allocate a fence, initialize first-owner state, issue a recovery epoch, construct a runtime, activate R1-R4 effects, deploy, or merge anything.

## Exact prerequisite

The exact validated prerequisite is C02f-BG:

- head `f8072591e3952c59453e26324a33d10d30164e6f`;
- tree `b94d0618411379e70b1fb7bf48669bfc229e0672`;
- gate `C02F_BG_ACQUISITION_ASYNC_AUTHORITY_ASSEMBLY_READINESS_COMPLETE`.

C02f-BG remains authoritative for the broader acquisition-readiness finding. C02f-BH closes only the attempt-ID generation ownership gap identified there.

## Existing typed domains

The two existing attempt-ID domains remain distinct and unchanged:

1. C02f-AJ `SequenceAllocationAttemptId([u8; 32])` identifies one logical fence-sequence reservation/allocation attempt;
2. C02f-Z `AuthorityAttemptId([u8; 32])` identifies one logical live-owner mutation attempt.

Both existing constructors reject the all-zero byte string. C02f-AR already requires the live-owner authority-attempt identifier to be separate from the sequence-allocation attempt identifier, and C02f-AB rejects reuse of the predecessor live-owner authority-attempt identifier.

C02f-BH does not merge these types, add a common serialized wire type, or make either identifier caller-selectable.

## Existing production cryptographic-randomness precedent

The repository already has a production precedent for server-owned cryptographic randomness:

- `prw-enrollment` directly depends on `aws-lc-rs = 1.18.0` with the repository-selected feature profile;
- enrollment challenge creation uses `aws_lc_rs::rand::{SecureRandom, SystemRandom}`;
- one fresh fixed-width server-owned byte array is filled through `SystemRandom::new().fill(...)`;
- provider randomness failure becomes a bounded fail-closed service error;
- request input does not provide or influence the generated nonce bytes.

C02f-BH selects the same cryptographic-provider family for future production attempt-ID generation rather than introducing `rand`, UUID generation, timestamps, counters, hashes of request fields, process IDs, host IDs, network identities, or request-supplied entropy.

This checkpoint does not yet add `aws-lc-rs` to `prw-control-plane`; dependency and source materialization remain a later checkpoint.

## Selected ownership boundary

Future production generation belongs to the narrow `prw-control-plane` acquisition-preparation boundary that will compose the already-private fence-sequence and live-owner planning machinery.

The bridge-facing semantic `acquire(peer)` caller supplies only `PeerConnectivityIdentity`. The bridge must not generate, accept, deserialize, reconstruct, derive, cache, persist, or select either attempt ID.

The future control-plane acquisition-preparation implementation must own generation internally before constructing the corresponding typed plans.

No public API may be added that accepts raw 32-byte attempt-ID input for production acquisition preparation.

Test-only deterministic entropy may be injected behind private/test-only seams, but no such seam may become a production request surface.

## Sequence-allocation attempt generation

For one new logical normal-path sequence allocation:

1. first obtain the exact authoritative initialized PRWF head through the already-selected provider path;
2. then obtain one fresh 32-byte cryptographic random draw from the selected production random provider;
3. pass those bytes only to `SequenceAllocationAttemptId::new`;
4. if randomness generation fails or the typed constructor rejects the bytes, fail closed before `plan_allocation` and before any allocation mutation;
5. once accepted, use that exact typed attempt ID to build one AJ allocation plan;
6. retain the same plan and therefore the same attempt ID through the entire AQ bounded submission/re-observation/reissue state machine.

A provider-level reissue of the same logical retained allocation must never generate a replacement sequence-allocation attempt ID.

A distinct later logical allocation operation must perform a new independent random draw and must not intentionally reuse a prior attempt ID.

## Live-owner authority-attempt generation

For one new logical live-owner acquisition mutation after an AQ allocation has resolved `Committed`:

1. retain the exact committed allocation evidence;
2. obtain one new independent 32-byte cryptographic random draw from the selected production random provider;
3. pass those bytes only to `AuthorityAttemptId::new`;
4. if randomness generation fails or the typed constructor rejects the bytes, fail closed before AR live-owner planning and before any live-owner mutation;
5. use that exact typed ID in the one AR Current successor and resulting C02f-AB live-owner transaction plan;
6. retain the same successor/plan and therefore the same authority-attempt ID throughout AE bounded reconciliation and any one permitted exact reissue.

A provider-level reissue of the same logical retained live-owner mutation must never generate a replacement authority-attempt ID.

A distinct later logical live-owner mutation must perform a new independent random draw. Existing C02f-AB predecessor comparison remains authoritative for rejecting direct predecessor attempt-ID reuse.

## Operational domain separation

C02f-BH selects operational and typed domain separation, not a KDF-based labeled derivation scheme.

The two attempt IDs must be generated by separate cryptographic-random fill operations into separate 32-byte buffers and immediately converted through their distinct typed constructors.

The following are prohibited:

- deriving `AuthorityAttemptId` from `SequenceAllocationAttemptId`;
- deriving `SequenceAllocationAttemptId` from `AuthorityAttemptId`;
- copying or reinterpreting one typed identifier as the other;
- splitting one random byte buffer between the two domains;
- using one generated 32-byte value intentionally for both domains;
- deriving either ID from peer identity, fence, epoch, sequence, etcd revision, Spanner state, time, PID/UID/GID, process start data, endpoint, credentials, request bytes, or any other deterministic acquisition input.

A coincidental equality between two independent 256-bit random draws is not treated as semantic cross-domain reuse. No cross-domain equality cache or global uniqueness database is selected. The safety requirement is independent cryptographic generation plus typed/provenance separation, while each existing protocol retains its own exact-state/attempt checks.

## Randomness failure and zero handling

Each logical ID generation uses one provider fill operation for one complete 32-byte destination.

If the cryptographic provider reports failure, generation fails closed and no mutation plan may be created from partially filled or fallback bytes.

If a complete successful fill produces the prohibited all-zero value, the existing typed constructor rejects it and the operation fails closed. C02f-BH does not select an unbounded retry loop, deterministic fallback, timestamp fallback, counter fallback, or request fallback for this case.

A later source checkpoint may expose one bounded internal generation error type, but it must not convert generation failure into contention, successful acquisition, or a fabricated identifier.

## Freshness semantics

`fresh` in C02f-BH means a new independent cryptographic random draw for each new logical attempt domain instance.

It does not mean that a caller can assert freshness, that wall-clock time proves freshness, or that the process maintains a universal history of every identifier ever generated.

Protocol-level retained-plan identity remains authoritative during reconciliation:

- same logical sequence allocation reissue => same retained sequence-allocation attempt ID;
- new logical sequence allocation => new random draw;
- same logical live-owner mutation reissue => same retained authority-attempt ID;
- new logical live-owner mutation => new random draw.

## Error mapping boundary

Attempt-ID generation failure occurs before the corresponding deterministic mutation plan is admitted.

A later control-plane preparation facade must surface generation failure as a fail-closed preparation error. When eventually mapped to the public semantic async-authority boundary, it must become `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous` unless a later contract selects a more specific non-success semantic error.

Generation failure must never map to `Granted`, `Contended`, `Current`, `Released`, `NotCurrent`, retry authorization, or local fallback ownership.

## Recovery attempt-ID separation

The existing recovery-epoch domain has its own `RecoveryEpochAttemptId` and remains outside C02f-BH.

Neither normal sequence allocation nor live-owner acquisition may reuse or derive from a recovery-epoch attempt ID. Normal acquisition must not generate or issue a recovery epoch as a fallback.

## Dependency and source-materialization boundary

C02f-BH is documentation only.

A later source checkpoint may add the already-used repository cryptographic provider dependency to the narrow crate that owns production generation and may materialize private helpers/facades implementing this contract.

That later source checkpoint must preserve:

- exact 32-byte width;
- independent provider fills;
- existing typed constructors;
- fail-closed provider/zero handling;
- no request-controlled raw attempt-ID input;
- no public broadening of private sequence/live-owner provider modules;
- exact retained ID continuity across AQ/AE reconciliation.

C02f-BH itself makes no Cargo or lockfile change.

## Relationship to remaining acquisition blockers

Closing C02f-BH does not make full `acquire(peer)` source assembly ready by itself.

The following BG blockers remain after this selection:

1. first-owner / absent-live-owner-record bootstrap semantics;
2. narrow control-plane acquisition-handoff preparation facade selection/materialization;
3. surrounding lifecycle proof that normal acquisition is operating against an initialized PRWF head for the current valid recovery epoch;
4. later source materialization of the already-selected AW `AS handoff -> AE -> AV` execution composition;
5. complete async-authority owner/runtime assembly and R1-R4 effect-side stale-fence enforcement remain later boundaries.

## Explicitly not authorized / not activated

C02f-BH does not:

- add or modify Rust source;
- add or modify Cargo manifests or `Cargo.lock`;
- generate any production random bytes or attempt ID;
- accept attempt IDs from requests;
- allocate or reissue a production fence sequence;
- perform etcd Get/Txn/re-observation;
- plan or execute a live-owner acquisition mutation;
- create first-owner live-owner state;
- initialize PRWF state;
- issue or contact a recovery-epoch provider;
- contact Spanner;
- construct endpoints, clients, TLS/auth/RBAC/credentials or runtimes;
- activate Agent integration;
- activate R1-R4 stale-effect rejection;
- deploy or merge.

## Exact source scope

C02f-BH adds exactly this one documentation contract.

No Rust source, workflow, Cargo manifest, lockfile, Android, Agent, runtime, provider implementation, credential, networking, deployment or merge file is selected.

## Validation gate

C02f-BH is valid only if canonical repository validation remains green on the exact final BH head and an exact BG -> BH compare proves one documentation-only addition with BG as the exact merge base.

Expected gate after validation:

`C02F_BH_ATTEMPT_ID_GENERATION_OWNERSHIP_SELECTED`

Passing this gate authorizes only the selection recorded here. It does not authorize source generation, first-owner semantics, full acquisition preparation, runtime activation, R1-R4 activation, deployment, or merge.
