# C03e-SL — production admission timing provenance/source selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_REAL_ADMISSION_TIMING_PROVENANCE_SOURCE_SELECTION`

Selection result:
`CALLER_SUPPLIED_ADMISSION_TIMING_INTERFACE_PRESERVED / DEVICE_ID_IS_SELECTOR_NOT_TIMING_AUTHORITY / REAL_ADMISSION_TIMING_REQUIRES_EXPLICIT_REVIEWED_PRODUCTION_PROVENANCE / VERIFIER_TIME_REUSE_OR_INFALLIBLE_ADAPTER_NOT_SELECTED / CURRENT_INFALLIBLE_CALLBACK_CANNOT_FAIL_CLOSED_OVER_AUDITED_FALLIBLE_WALL_CLOCK / CONCRETE_TIMING_SOURCE_AND_RUST_MATERIALIZATION_BLOCKED`

This checkpoint is documentation-only. It narrows only the production provenance/source requirements for the existing `admission_timing` hook retained by the evidence-closed corrective C03e-SK boundary. It does not select concrete durations, a wall-clock adapter, an executable caller, a configuration source, callback implementations, a Rust source path, a public/error-shape change, or runtime activation.

## Exact predecessor authority

Authoritative predecessor is the evidence-closed corrective C03e-SK replacement:

- predecessor branch: `phase-152-c03e-sk-admission-timing-callback-provenance-selection-corrective-replacement`;
- predecessor head: `5e0ad1f3dece52a6465ed464d94d97202bf99223`;
- predecessor tree: `8c14b760d3c620c0392351fff791aed0bc22ad04`;
- predecessor contract: `contracts/C03E_SK_ADMISSION_TIMING_CALLBACK_PROVENANCE_SELECTION_STAGING.md`;
- predecessor contract blob: `13aa65285498ec60f68e00ca2a574dc18e0c8c15`;
- predecessor PR: `#628`, draft/open/unmerged;
- predecessor status binding: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- predecessor exact topology: one commit, one documentation path, `+169/-0`;
- predecessor Rust Validation #1862 — run `34836659125`, job `103951917630`: `SUCCESS`;
- C02f-AD #1110 — run `34836659194`: `SKIPPED`;
- C02f-AE #1101 — run `34836659183`: `SKIPPED`;
- no Android workflow registered for the exact docs-only corrective SK head; no Android PASS is inherited or claimed;
- `SKIPPED` is not PASS.

Immutable corrective-SK evidence:

- title: `C03E_SK_ADMISSION_TIMING_CALLBACK_PROVENANCE_SELECTION_CORRECTIVE_REPLACEMENT_AUDIT_2026-09-14.md`;
- Drive ID: `1ao3dpyhCSVUGzT4_ofotawuYgrcPXBRk`;
- canonical parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- MIME: `text/markdown`;
- bytes: `11466`;
- SHA-256: `381f538073fd7e4f4d9cf8d01166a0b9700677c0c9fc7328d596e47e48749145`;
- final LF: `true`;
- current revision: `0Bz5eMiLa5v9xRUtyT0syb0UyMXJLVXFoTXYwVlZZaWYzRG04PQ`;
- previous revision: `null`.

The earlier un-evidenced SK attempt on PR #627 remains non-authoritative and untouched. C03e-SL is rooted only in the corrective closed SK head above.

Integrated `main` was freshly re-read immediately before this contract write and remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

The forward-only README recovery history on `main` carries no C03e-SL authority.

## Fresh successor guards

Before this contract write:

- no branch matched `phase-152-c03e-sl`;
- no all-state PR matched `C03e-SL` in the title;
- no canonical Drive evidence result matched `C03E_SL`;
- the new C03e-SL branch was created directly at exact corrective-SK head `5e0ad1f3dece52a6465ed464d94d97202bf99223`;
- immediate branch readback confirmed the branch still pointed to exact corrective-SK head and tree before this file write;
- this exact contract path did not exist before this write.

No source/runtime path was changed while establishing these guards.

## Exact current source finding

### Existing higher-owner hook remains caller supplied

The dormant C03e-SJ higher-owner composition retained unchanged through corrective SK accepts exactly:

`admission_timing: F`

with bound:

`F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`.

Corrective SK explicitly preserved this as a caller-supplied policy hook and selected no concrete implementation, environment variable, systemd credential, configuration file, default, wall clock or executable owner.

C03e-SL preserves that interface exactly. It does not add an argument, alter the callback result type, move the callback to a new owner, sample timing in the higher-owner wrapper, or select an executable invocation site.

### Exact real-admission timing shape

Exact current source defines `RemoteSessionRealAdmissionTiming` with exactly three owned Unix-second inputs:

1. `challenge_validity_unix_seconds: Range<u64>`;
2. `authentication_now_unix_seconds: u64`;
3. `application_lease_unix_seconds: Range<u64>`.

Its constructor is:

`RemoteSessionRealAdmissionTiming::new(challenge_validity_unix_seconds, authentication_now_unix_seconds, application_lease_unix_seconds)`.

The existing admission path invokes the caller-supplied timing hook using the exact expected logical `DeviceId`, then retains the returned timing beside the exact request for the subsequent admission path.

An audited exact-current test helper constructs a synthetic timing bundle with fixed values. Test construction is not production provenance and is not promoted into a production source by C03e-SL.

### Exact audited wall-clock precedent is fallible

The existing PRWA verifier source is a separate server-local authority. Exact current source exposes:

`current_prwa_verifier_unix_seconds() -> Result<u64, PrwaVerifierSourceError>`.

Its implementation observes `SystemTime::now()`, converts through `duration_since(UNIX_EPOCH)`, and maps a non-representable/pre-epoch clock into `PrwaVerifierSourceError::VerifierTime`. Existing verifier challenge-expiry construction also uses checked arithmetic and returns `PrwaVerifierSourceError::ExpiryOverflow` on overflow.

Therefore the audited server-local wall-clock precedent is deliberately fail-closed and fallible.

C03e-SL does not reinterpret this as an infallible real-admission timing source.

## Selected production timing provenance law

### Device identity is selector/correlation only

The `&DeviceId` supplied to `admission_timing` identifies the exact expected logical device for which one timing decision is being requested. It is not itself timing authority.

No timing value may be derived from:

- the bytes/text of `DeviceId`;
- transport identity;
- endpoint/IP/port state;
- reachability state;
- candidate state;
- configured-peer state;
- durable capability authority;
- requester/rendezvous authority;
- scheduling grant identity;
- request/correlation identifiers;
- callback outcome history.

C03e-SL does not turn any identity, authority, transport or correlation value into a clock or lease policy.

### One explicit reviewed timing domain is required

A future production real-admission timing implementation must establish explicit reviewed provenance for all three `RemoteSessionRealAdmissionTiming` components. They must not be independently fabricated by unrelated defaults or hidden fallbacks.

The future design must make clear:

- which authority supplies the challenge-validity window;
- which authority supplies the exact authentication-now Unix-second value;
- which authority supplies the application-lease window;
- when each value is acquired;
- how range ordering and arithmetic are validated;
- how acquisition/conversion/arithmetic failure is represented before a timing bundle is accepted.

C03e-SL does not select concrete duration values, environment-variable names, configuration keys, files, credentials, service properties or persistence records for those inputs.

### Freshness and caching constraints

A future production source must not silently use a process-start frozen `authentication_now_unix_seconds` value for later independent admissions.

C03e-SL selects no cache, refresh loop, timer task, background sampler or retry loop. Any such lifecycle behavior would require a separate boundary.

### Fail-closed arithmetic and conversion

A future source must not create apparently valid timing by:

- defaulting a failed clock observation;
- substituting Unix epoch/zero;
- saturating overflow;
- unchecked wrapping arithmetic;
- clamping an invalid source into an accepted window;
- panic/`expect`/`unwrap` as a production failure policy;
- retrying until a different answer appears;
- reusing a stale previously successful sample after current acquisition failure.

Any future range construction must preserve explicit bounded ordering and checked arithmetic at the selected source boundary. C03e-SL does not materialize that logic.

## Verifier-time remains a separate custody domain

C03e-SL does not reuse the existing fallible PRWA verifier-time source as real-admission timing.

The existing SJ/SK verifier-time relationship remains independently fixed as:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

The real-admission `admission_timing` hook remains separately typed as infallible:

`FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming`.

No conversion from the fallible verifier-time function into the infallible admission-timing hook is selected. In particular, C03e-SL does not select:

- `unwrap`/`expect`;
- default time;
- fallback time;
- saturation/clamp;
- retry;
- cached prior value;
- error flattening;
- panic-to-process-exit semantics;
- synthetic timing on verifier failure.

Verifier-time and real-admission timing remain distinct custody domains even if a later design proves they share an underlying clock authority.

## Current interface/failure-custody mismatch

The current `admission_timing` callback returns `RemoteSessionRealAdmissionTiming` directly and exposes no error lane.

The audited server-local wall-clock precedent is fallible. No exact-current audited production source proves that challenge-validity, authentication-now and application-lease acquisition can satisfy the selected fail-closed provenance law through the current infallible callback without defaulting, panic, hidden fallback or unselected failure handling.

Therefore C03e-SL records:

`CONCRETE_PRODUCTION_ADMISSION_TIMING_SOURCE_NOT_PROVEN / CURRENT_INFALLIBLE_CALLBACK_FAILURE_CUSTODY_UNRESOLVED / SOURCE_MATERIALIZATION_BLOCKED`.

This is a fail-closed selection result, not authorization to alter the callback signature.

## Rust/source ceiling

C03e-SL authorizes no Rust/source materialization.

Future source ceiling:
`UNSELECTED — source-interface/failure-custody selection required first`.

No Rust path is selected for a future C03e-SM by this checkpoint. The alphabetical token alone creates no authority.

A later documentation-only gate must first decide the minimum compatible production source/failure-custody model. At minimum it must prove one of the following without silently weakening failure semantics:

1. an already-existing infallible production timing authority whose acquisition and policy validation are completed earlier at a fail-closed startup/configuration boundary; or
2. a separately authorized callback/error-interface evolution that can propagate timing-source failure without panic/default/fallback.

If neither is proved, source materialization remains blocked.

## Callback and executable boundaries remain unchanged

The other corrective-SK hooks remain caller supplied and unselected:

- `on_completion`;
- `on_rejection`;
- `on_admission_failure`.

C03e-SL does not select concrete sinks, logging, telemetry, persistence, retry, requeue, dead-letter handling, state repair, process restart or callback remapping.

Executable ownership remains unselected:

- `run()` remains unchanged and unselected for this boundary;
- `main.rs` remains unchanged and unselected;
- no service/systemd/package caller is selected;
- no listener/readiness/network behavior is activated;
- no production Agent process is replaced or restarted.

The dormant SJ higher-owner wrapper remains dormant.

## Preserved channel, dispatcher and authority laws

C03e-SL does not modify or reinterpret:

- the single capacity-one expected-request channel;
- sole-sender/no-clone custody;
- single receiver transfer;
- existing dispatcher provenance;
- existing fallible verifier-time provider type;
- durable capability authority custody;
- requester/rendezvous authority custody;
- bounded authority-free completion observation.

No second channel, alternate queue, sender clone, new dispatcher, new authority carrier or new observation surface is selected.

## Validation requirements for this selection checkpoint

C03e-SL is valid only if final GitHub topology proves:

- direct exact corrective-SK -> SL ancestry;
- ahead `1`, behind `0`;
- merge base exact corrective-SK head `5e0ad1f3dece52a6465ed464d94d97202bf99223`;
- exactly one changed path, this contract under `contracts/`;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/package/service/deployment/repository-config mutation;
- exact higher-owner blob `cccefed1192243a16b67c971ca6ba8fe13ae2c82` remains unchanged;
- exact PRWA verifier-source blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b` remains unchanged;
- exact Linux bootstrap blob `b5d236d3c0264ea8870e90af96c76294b50d0fc1` remains unchanged;
- exact `main.rs` blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1` remains unchanged.

Any PASS claim must bind only to the exact final SL head. No corrective-SK, SJ or historical run may be inherited as SL validation authority. `SKIPPED` is not PASS. A docs-only SL head does not require an Android PASS unless an Android workflow actually registers for that exact head.

Draft PR creation and exact-head CI observation are separately gated operational steps after contract materialization. Immutable Drive evidence publication and post-publication GitHub closure binding remain separately gated after exact-head validation.

## Explicit non-actions / STOP

This C03e-SL contract write performs no Rust/source/runtime/API mutation. It does not:

- materialize a production timing provider;
- select concrete challenge/lease durations;
- select environment/config/service/credential inputs;
- change `RemoteSessionRealAdmissionTiming`;
- change the `admission_timing` callback signature;
- reuse or adapt verifier time;
- add an error/result type;
- add panic/default/fallback/cache/retry/clamp/saturation behavior;
- modify callback types or implementations;
- modify `production_durable_capability_higher_owner_custody.rs`;
- modify `linux_bootstrap.rs`;
- modify `main.rs`;
- modify `prw-session`;
- construct/clone/replace channel endpoints;
- activate an executable caller, listener, readiness or network path;
- mutate Cargo/lockfile/workflows/Android/package/service/systemd/repository configuration;
- mutate integrated `main`;
- modify, close or delete predecessor PRs/branches;
- publish Drive evidence;
- merge;
- deploy;
- restart;
- alter authentication/database/control-plane state;
- convert any PR ready-for-review;
- close any PR;
- delete any branch;
- clean accidental refs;
- reset/rebase/squash/force-update;
- rewrite history.

After this docs-only contract materialization: `STOP` before any Rust/source/timing-provider/callback/error-interface/executable/runtime mutation. Exact-head validation and draft PR creation require the separately authorized continuation already granted for this selection checkpoint; immutable evidence publication remains a later separate authorization boundary.
