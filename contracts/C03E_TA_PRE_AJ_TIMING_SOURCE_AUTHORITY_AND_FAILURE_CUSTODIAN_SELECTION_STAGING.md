# C03e-TA — Pre-AJ Timing Source Authority and Failure Custodian Selection

Status: `SELECTION — VALIDATION PENDING`

Date: `2026-09-15`

Boundary:
`PRE_AJ_TIMING_SOURCE_AUTHORITY_AND_FAILURE_CUSTODIAN_SELECTION`

Selection result:
`SERVER_LOCAL_VERIFIER_WALL_CLOCK_SELECTED_AS_PRE_AJ_CLOCK_AUTHORITY / FRESH_SAMPLE_PER_ELIGIBLE_REQUEST / CHALLENGE_WINDOW_USES_LOCKED_300_SECOND_LIFETIME_WITH_CHECKED_ARITHMETIC / PRE_AJ_AUTHENTICATION_NOW_FIELD_IS_COMPATIBILITY_ONLY_AFTER_SZ / APPLICATION_LEASE_REMAINS_SEPARATELY_VERIFIER_OWNED_AND_POLICY_UNSELECTED / COMPLETE_F_PROVIDER_REMAINS_BLOCKED_PENDING_APPLICATION_LEASE_POLICY / TERMINAL_EXACT_REQUEST_TIMING_FAILURE_CUSTODIAN_SELECTED / NO_RETRY_OR_REQUEUE / NO_SOURCE_MATERIALIZATION / NO_RUNTIME_ACTIVATION`

## 1. Scope and authoritative predecessor

This checkpoint is documentation-only. It selects only the concrete underlying pre-AJ clock/challenge authority and the concrete terminal disposition law for the already-materialized C03e-SX timing-failure custodian seam.

Authoritative predecessor is evidence-closed C03e-SZ:

- PR: `#639`;
- branch: `phase-152-c03e-sz-proof-submission-verifier-time-freshness-source-materialization`;
- exact head: `501e4b2fa2b32985b4a72924a829b53f56dac6fa`;
- exact tree: `5c22c0b63ba5f3177d3ec4c41a1b2fa46e313042`;
- status binding: `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- PR remains draft/open/unmerged.

C03e-SZ fixed fresh proof-submission verifier time while explicitly leaving concrete pre-AJ timing provider/custodian, application-lease production policy, and executable activation unselected.

C03e-TA does not reopen or weaken that closure.

## 2. Existing fallible timing seam retained unchanged

The evidence-closed C03e-SX source already materializes the result-valued timing seam:

`F: FnMut(&DeviceId) -> Result<RemoteSessionRealAdmissionTiming, RemoteSessionAdmissionTimingSourceError<Cause>>`

and the separate failure custodian:

`K: FnMut(RemoteSessionAdmissionTimingFailure<D, T, RemoteSessionAdmissionTimingSourceError<Cause>>)`.

The existing law remains authoritative:

- duplicate-active-device preflight invokes rejection custody and does not invoke F or K;
- an eligible vacant request invokes F once;
- F success keeps the exact request plus returned timing for AJ;
- F failure transfers the exact timing error plus the untouched original request to K exactly once;
- timing failure invokes no AJ, transport/session operation, worker creation, rejection callback, admission-failure callback, completion callback, receipt synthesis, retry or requeue;
- after normal K return the supervisor continues only with independent later work;
- the failed request is never sampled again or put back into the channel.

C03e-TA changes none of these source interfaces or propagation semantics.

## 3. Exact current timing shape

`RemoteSessionRealAdmissionTiming` currently owns exactly:

1. `challenge_validity_unix_seconds: Range<u64>`;
2. `authentication_now_unix_seconds: u64`;
3. `application_lease_unix_seconds: Range<u64>`.

The selected C03e-SZ fresh admission path destructures the second field as `_authentication_now_unix_seconds` and does not use it for proof verification. Proof-submission verifier time instead comes from the exact request-retained fallible verifier-time provider immediately before `submit_proof(...)`.

Therefore after C03e-SZ the second field is compatibility state for the retained timing bundle, not proof-time authority.

C03e-TA does not remove or repurpose that field and does not change `RemoteSessionRealAdmissionTiming`.

## 4. Selected pre-AJ clock authority

C03e-TA selects the existing server-local verifier wall-clock authority as the underlying wall-clock authority for pre-AJ challenge-window construction:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds()`

Exact current signature:

`fn current_prwa_verifier_unix_seconds() -> Result<u64, PrwaVerifierSourceError>`.

The implementation observes `SystemTime::now()` and converts through `duration_since(UNIX_EPOCH)`; a non-representable/pre-epoch clock fails closed as `PrwaVerifierSourceError::VerifierTime`.

This selection reuses the audited underlying clock authority only. It does not merge pre-AJ timing-failure custody with proof-submission freshness custody.

Each domain retains its own sampling point and failure owner:

- pre-AJ sample failure belongs to the existing F -> K timing-failure path;
- proof-submission sample failure remains inside the C03e-SZ authentication transaction cleanup path and is not routed to K.

The two samples may observe the same underlying server wall clock at different times. No sample is reused as authority for the other phase.

## 5. Fresh pre-AJ sampling law

A future complete production F adapter, once separately authorized, must acquire a fresh pre-AJ wall-clock sample exactly once for each eligible vacant request after duplicate-active-device preflight and immediately when the existing fallible timing seam invokes F.

It must not use:

- process-start time;
- a cached prior sample;
- request/correlation identifiers;
- `DeviceId` bytes;
- transport identity;
- endpoint/IP/port state;
- reachability or candidate state;
- scheduling grant identity;
- previous callback outcomes;
- fallback Unix epoch/zero;
- retry-until-success;
- saturation or clamping;
- panic/`unwrap`/`expect` as failure policy.

`DeviceId` remains selector/correlation only and is not timing authority.

Clock acquisition failure must be represented through the existing `RemoteSessionAdmissionTimingSourceError::Acquisition(...)` domain and transferred through K under the existing SX law. C03e-TA selects no error flattening into rejection, AJ failure, worker completion or process success.

## 6. Selected challenge-validity construction

The challenge lifetime remains the already-locked PRWA/session-authentication maximum:

`300` seconds.

A future provider implementation must construct the pre-AJ challenge window from the fresh sample using checked arithmetic:

`issued_at = fresh_pre_aj_now`

`expires_at = issued_at.checked_add(300)`

`challenge_validity_unix_seconds = issued_at..expires_at`

Checked-add overflow fails closed through the existing timing-source failure domain; it must not wrap, saturate, clamp or fabricate a range.

The existing session-authentication validator remains authoritative and still rejects zero, reversed, or greater-than-300-second challenge lifetimes.

C03e-TA does not widen the 300-second maximum.

## 7. Compatibility authentication-now field after SZ

For any future adapter that must still construct the existing three-field `RemoteSessionRealAdmissionTiming`, C03e-TA selects the fresh pre-AJ sample itself as the compatibility value for `authentication_now_unix_seconds`.

That assignment has no proof-verification authority after C03e-SZ. The fresh SZ proof-time provider remains authoritative immediately before proof submission.

No later implementation may use the compatibility field to bypass, replace, cache or precompute the C03e-SZ fresh proof-time sample.

## 8. Application lease remains a separate gate

C03e-TA does not select a production application-lease duration, issue-time anchor, expiry-time policy, configuration source, environment variable, credential, file, service property, per-device override, refresh rule or fallback.

Existing C03e-I/C03e-L authority remains unchanged:

- application lease is separately verifier-owned;
- it is independent from the authentication challenge window;
- it must not be silently derived from challenge issue/expiry, proof-time alone, QUIC handshake time or a hidden owner-constructor clock read;
- `RemoteSessionLease::new(...)` remains authoritative for interval validation;
- the existing maximum remains `MAX_REMOTE_SESSION_LEASE_SECONDS = 3_600` seconds.

The current F interface returns the complete three-field timing bundle. Therefore selecting only clock/challenge authority is not sufficient to materialize a complete production F callback while application-lease provenance/policy remains unselected.

C03e-TA explicitly records:

`COMPLETE_PRODUCTION_F_PROVIDER_NOT_YET_AUTHORIZED / APPLICATION_LEASE_POLICY_AND_PER_ADMISSION_PROVENANCE_REQUIRED_FIRST`.

No fixed `0..3600`, `now..now+3600`, challenge-derived lease, process-start lease, static absolute range, test range, or other implicit lease default is selected.

## 9. Concrete timing-failure custodian selection

C03e-TA selects a terminal fail-closed disposition for the existing production K seam.

At the outer production boundary, K must:

1. receive exactly one `RemoteSessionAdmissionTimingFailure` by value;
2. take ownership of the exact bounded timing error and exact untouched original request;
3. perform no retry, requeue, sender clone, replacement request, alternate timing sample, AJ call, transport/session call, scheduling replay, fabricated receipt, rejection callback, admission-failure callback or completion callback;
4. terminally consume/release the timing error and original request under the existing K ownership transfer;
5. return unit so the already-selected supervisor continuation law can resume independent work.

This terminal disposition is safe at this seam because F failure occurs before AJ construction and before the request has created or acquired a peer/session/worker effect that would require rollback. K is not proof of cleanup for unrelated resources; rather, the selected seam owns no such external effect yet.

The selected custodian may be generic over `Cause` while its behavior is concrete. It must not inspect `DeviceId` to derive authority and must not expose dispatcher/verifier/request internals to logging or telemetry as a side effect of disposal.

No log sink, metrics sink, persistence record, dead-letter queue, retry queue, process exit or restart is selected.

## 10. Concrete custodian source placement for a later materialization

If/when separately authorized for source materialization, the narrow preferred location for the terminal production custodian is the existing higher-owner composition path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Current exact SZ blob:
`b387b49bb179a052a9eac7b29d4b00bbb31884ca`.

That path is where the dormant configured-production higher-owner seam already accepts and forwards `on_timing_failure: K` after successful population. A future helper there may provide the exact terminal-disposition callback without changing the lower six/seven-path timing propagation contract.

C03e-TA does not authorize that Rust write now.

## 11. Provider source-materialization remains blocked

A complete concrete F provider cannot be materialized by C03e-TA because F must return the application lease range together with challenge timing, while the production application-lease policy/provenance remains deliberately unselected.

The next lease-policy selection must decide, at minimum:

- the authority that chooses the lease lifetime or exact interval;
- the per-admission issue-time anchor;
- how that anchor relates to successful authentication rather than merely pre-AJ sampling;
- the configuration/provenance source, if configurable;
- fail-closed acquisition/validation behavior;
- how its error composes with the already-selected generic `Cause` lane without weakening K custody;
- whether the resulting interval is constructed before AJ or supplied/constructed at a later post-auth seam while preserving existing APIs or through a separately selected minimal interface evolution.

Until that is selected, no complete F closure/function is authoritative.

This is a deliberate fail-closed boundary, not an invitation to invent a one-hour default.

## 12. Audited source anchors

C03e-TA is grounded in the exact C03e-SZ tree and preserves these source anchors:

- `crates/prw-session/src/prwa_verifier_source.rs`
  - blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`;
  - audited fallible server-local wall clock and locked challenge lifetime.
- `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs`
  - blob `364f27e2bf71fb009192a6efbf15dc4fc0e8e1fb`;
  - existing bounded timing-source error and intact-request failure carrier.
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`
  - blob `17bb04bef8436574b7bc1778f8fa436cc51fc94e`;
  - SZ fresh proof-submission verifier-time transaction.
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
  - blob `08deec2f12095738c9e71fdb133914733e68263e`;
  - separately verifier-owned post-auth lease binding composition.
- `crates/prw-agent/src/linux_bootstrap.rs`
  - blob `0a41a8ea58cc47757c24cf314035452f9100c1e5`;
  - dormant fallible timing forwarding seam, no concrete F/K.
- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
  - blob `b387b49bb179a052a9eac7b29d4b00bbb31884ca`;
  - configured-production higher-owner F/K injection boundary.

C03e-TA changes none of these source paths.

## 13. Validation requirements

This docs-only checkpoint is valid only if final topology proves:

- direct exact C03e-SZ -> C03e-TA ancestry;
- merge base is exact SZ head `501e4b2fa2b32985b4a72924a829b53f56dac6fa`;
- ahead `1`, behind `0`;
- exactly one changed path: this contract;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/package/service/deployment/repository-config mutation;
- all audited source-anchor blobs above remain byte-identical to SZ;
- integrated `main` remains untouched by this checkpoint.

Any PASS claim must bind only to the exact final TA head. `SKIPPED` is not PASS. A docs-only head does not inherit any Android PASS from SZ; Android is claimed only if an Android workflow actually registers and succeeds for the exact TA head.

## 14. Explicit non-actions / STOP

C03e-TA performs no Rust/source/runtime/API mutation. It does not:

- materialize a complete F provider;
- materialize the selected K custodian;
- select or materialize application-lease production policy;
- use 3,600 seconds as a production default;
- derive application lease from challenge/proof timing;
- change `RemoteSessionRealAdmissionTiming`;
- change F/K callback signatures;
- alter C03e-SZ proof-time freshness or cleanup custody;
- add retry/requeue/fallback/cache/clamp/saturation/panic policy;
- add logging, metrics, persistence or dead-letter behavior;
- modify `main.rs`;
- activate an executable caller, endpoint, listener, readiness, networking or remote companion;
- mutate Cargo manifests, lockfiles, workflows, Android source, package/service/systemd files or repository configuration;
- mutate integrated `main`;
- merge;
- deploy;
- restart;
- close or mark any PR ready;
- delete branches;
- reset, rebase, squash, force-update or rewrite history.

After this documentation-only selection checkpoint: `STOP` before any application-lease policy selection or Rust/source materialization.
