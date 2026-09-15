# C03e-SY — proof-submission verifier-time freshness corrective selection

## Status

`CORRECTIVE SELECTION — VALIDATION PENDING`

## Boundary

`PROOF_SUBMISSION_VERIFIER_TIME_FRESHNESS_CORRECTIVE_SELECTION`

## Purpose

C03e-SY is documentation-only. It reviews the first concrete production-provider/executable integration gate after evidence-closed C03e-SX and records a blocking freshness conflict before any provider, timing-failure custodian, executable caller, listener, service, deployment, or runtime activation is allowed.

C03e-SY changes no Rust/source/runtime/configuration file. It does not activate the C03e-SX fallible admission-timing lane.

## Exact predecessor

Repository: `Gersi365/prw-executor-private`

Authoritative predecessor: evidence-closed C03e-SX PR #637.

Exact predecessor head:

`e23fc33d0a660e3ebefc85908c0e59c9f6a7f615`

Exact predecessor tree:

`18fe21b085577a129a4b20458641f91d2673f37a`

C03e-SN remains the sole authority for the pre-AJ fallible admission-timing source/error/custody semantics materialized by SX.

## Pinned exact-current findings

The review pins these exact SX blobs:

- `crates/prw-session/src/prwa_verifier_source.rs` — `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`;
- `crates/prw-control-plane/src/session_auth.rs` — `1dbd06d8d9741844e4d8bbb235d27431921a1650`;
- `crates/prw-agent/src/remote_session_authentication_transaction.rs` — `4fa704a679db603c1a1e9a5d5c40f24e2ec72ff9`;
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs` — `812b56e9b948a41f2f746eb406ba24567efbd528`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` — `1539b6b9a08bf18883d7a16022f15f7c240eaf08`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs` — `ca54b314c1f36524e5ebb26750fadc2532750442`;
- `crates/prw-agent/src/main.rs` — `db6b8028c6df100a961a0fb5818347bea2fdc5c1`.

No source drift is inferred from an older branch or local working tree; all findings above were inspected at exact SX head.

## Existing proof-time authority

C03e-CE previously selected the PRWA verifier wall-clock authority and requires a fresh observation for `SessionAuthenticationService::submit_proof(...)`. Its contract explicitly states that proof-submission `now_unix_seconds` must be obtained from the same verifier wall-clock authority and that the challenge issue timestamp must not be reused as proof-verification `now`.

Exact current source exposes:

`current_prwa_verifier_unix_seconds() -> Result<u64, PrwaVerifierSourceError>`

The function observes `SystemTime::now()`, converts through `duration_since(UNIX_EPOCH)`, and fails closed when the wall clock is not representable.

The expected-device admission request already retains a verifier-time provider `T`. In the production specialization selected before SX, `T` is the existing fallible PRWA verifier-time function pointer.

## Blocking freshness conflict discovered after SX

`RemoteSessionRealAdmissionTiming` currently owns three values:

1. `challenge_validity_unix_seconds: Range<u64>`;
2. `authentication_now_unix_seconds: u64`;
3. `application_lease_unix_seconds: Range<u64>`.

The C03e-SX fallible preparation lane invokes the admission-timing source before AJ construction and before network acceptance. It then decomposes the timing bundle and passes the precomputed `authentication_now_unix_seconds` into `admit_expected_remote_device_session(...)`.

`admit_expected_remote_device_session(...)` performs current-registry resolution, lower-transport acceptance, challenge preparation, challenge send, proof receive/correlation and only then reaches `complete_registry_bound_session_authentication(...)`.

`complete_registry_bound_session_authentication(...)` currently receives the precomputed `now_unix_seconds` and passes it to `SessionAuthenticationService::submit_proof(...)` after the proof frame has been received and validated for request/session correlation.

Therefore the current pre-AJ `authentication_now_unix_seconds` can be older than the actual proof-submission instant by the complete transport-accept plus challenge/proof round-trip duration.

C03e-SM explicitly warned that sample-before-AJ placement does not prove freshness after a long transport-accept wait and that later resampling requires a separate transaction/timing review.

Activating a concrete production timing provider without correcting this would violate the earlier C03e-CE proof-time freshness authority. C03e-SY therefore blocks provider/executable activation at this boundary.

## Corrective semantic selection

For the later corrected fallible expected-device admission path, proof verification MUST obtain a fresh verifier-time observation after the proof message has been received and its request/session correlation has succeeded, and immediately before the one `SessionAuthenticationService::submit_proof(...)` invocation.

The fresh observation MUST come from the exact verifier-time provider retained by the same expected-device request. It must not come from:

- the pre-AJ `authentication_now_unix_seconds` field;
- challenge issue time or expiry time;
- process-start time;
- connection acceptance time;
- QUIC/TLS handshake metadata;
- request payload time;
- system-clock access hidden inside `SessionAuthenticationService`;
- a cached/stale successful value;
- a retry/fallback/default source.

The existing request-owned verifier-time provider is borrowed for the proof-time observation. Successful authentication must return that same provider instance to the existing worker-admission custody; it is not cloned, replaced, reconstructed, or consumed by proof verification.

## Fresh-read invocation law

The corrected path selects exactly this ordering for a proof candidate that reaches semantic verification:

1. accept the already-selected lower-transport peer under current authority;
2. prepare the existing challenge;
3. send the exact challenge;
4. receive one proof candidate;
5. verify exact PRWM request-id correlation;
6. verify exact logical `SessionId` correlation;
7. construct the typed proof;
8. invoke the retained verifier-time source exactly once for this proof submission;
9. on verifier-time success, invoke `SessionAuthenticationService::submit_proof(...)` exactly once with that fresh value;
10. on authentication success, preserve the same verifier-time provider for later worker request-loop verification.

Malformed transport/wire/correlation failures before step 8 do not sample verifier time. No hidden retry is selected.

## Fresh-read failure custody

A verifier-time source failure at step 8 occurs after AJ has started and after a pending challenge exists. It is therefore NOT a C03e-SN pre-AJ admission-timing-source failure and MUST NOT be sent to the C03e-SX timing-failure custodian `K`.

The failure belongs to the existing logical-session authentication transaction cleanup domain.

The later implementation must retain the exact `PrwaVerifierSourceError` cause under a bounded authentication-primary failure classification, then execute the same terminal cleanup law used by other authentication-transaction failures:

- call `abort_pending_session(...)` exactly once for the prepared session;
- retain any cleanup error beside the primary failure;
- close the same accepted peer exactly once with the existing bounded authentication-failure diagnostic;
- return through the existing real-admission authentication-failure phase;
- create no authenticated session owner or worker;
- perform no retry, fallback, requeue, replacement session, or synthetic success.

Exact future Rust symbol names and compatibility layout are deferred to the next source-layout selection. C03e-SY selects the failure domain and custody law, not an implementation signature.

## Pre-AJ timing lane after correction

C03e-SN/SX pre-AJ failure semantics remain unchanged for acquisition/policy/range failures that occur before AJ construction. Duplicate-active-device preflight still performs zero admission-timing source calls. A vacant pre-AJ timing failure still transfers the exact intact request to `K` once and creates no AJ/network/session/worker effect.

The pre-AJ `authentication_now_unix_seconds` value must not remain authoritative for proof submission in the corrected fallible path.

C03e-SY does not silently delete or rename that field, because existing infallible APIs and historical callers remain compatibility surfaces. Exact representation/migration is a later source-layout decision.

## Challenge and application-lease policy findings

The existing Phase 128 challenge domain accepts lifetimes from 1 through `MAX_SESSION_AUTH_CHALLENGE_LIFETIME_SECONDS = 300` seconds. C03e-CE separately selected an exact fixed PRWA verifier challenge lifetime of 300 seconds, and `PRWA_VERIFIER_CHALLENGE_LIFETIME_SECONDS` materializes that decision.

The remote application-session lease domain has only a maximum bound: `MAX_REMOTE_SESSION_LEASE_SECONDS = 3_600`. Existing C03e-I authority explicitly keeps the application-session lease interval separate from the authentication challenge and requires a separately verifier-owned lease interval. No reviewed predecessor selects 3,600 seconds as a production default or exact lease duration.

Accordingly C03e-SY does NOT invent an application-lease duration, environment key, configuration key, service property, persistence field, or fallback default. A concrete admission-timing provider remains blocked until that policy is separately selected or an already-authoritative exact policy is identified.

## Timing-failure terminal custodian finding

C03e-SX materializes the caller-supplied pre-AJ failure custodian `K`, but no exact-current production caller supplies a concrete terminal custodian. Logging alone, implicit Drop, status-only observation, retry/requeue, or unbounded in-memory retention is not selected as disposition.

C03e-SY does not invent that sink. Concrete terminal disposition of the intact pre-AJ failed request remains a separately reviewed executable-owner decision.

## Executable activation finding

`crates/prw-agent/src/main.rs` still invokes only `prw_agent::linux_bootstrap::run()`.

The production remote-process companion and the C03e-SX fallible-admission-timing companions remain dormant source seams. No exact-current executable caller activates them.

C03e-SY does not change `run()`, `main.rs`, listener/readiness order, systemd configuration, credentials, network binding, service lifecycle, deployment, or runtime activation.

## Corrective selection result

`FRESH_PROOF_SUBMISSION_VERIFIER_TIME_REQUIRED / PRE_AJ_AUTHENTICATION_NOW_NOT_AUTHORITATIVE_FOR_PROOF / REQUEST_RETAINED_VERIFIER_TIME_PROVIDER_SELECTED_AS_FRESH_SOURCE / FRESH_SOURCE_FAILURE_BELONGS_TO_AUTHENTICATION_TRANSACTION_CLEANUP_DOMAIN / PRE_AJ_TIMING_FAILURE_K_UNCHANGED / APPLICATION_LEASE_EXACT_POLICY_STILL_UNSELECTED / CONCRETE_PRE_AJ_FAILURE_CUSTODIAN_STILL_UNSELECTED / EXECUTABLE_REMOTE_COMPANION_ACTIVATION_STILL_UNSELECTED / NO_SOURCE_MUTATION / NO_RUNTIME_ACTIVATION`

## Current mutation ceiling

C03e-SY current Rust/source/runtime ceiling: `ZERO`.

The only authorized repository mutation for this checkpoint is this new documentation contract.

No existing contract is replaced or rewritten.

## Validation obligations

The docs-only checkpoint must prove:

- direct ancestry from exact evidence-closed SX head;
- exactly one added contract path and zero other changed paths;
- all pinned exact-current source blobs above remain unchanged;
- the C03e-CE fresh-proof-time rule and C03e-SM freshness warning are represented without weakening either;
- the fresh read is placed after proof correlation and before `submit_proof`;
- proof-time source failure uses authentication cleanup custody, not pre-AJ `K`;
- the request-owned verifier-time provider remains available for worker custody after successful authentication;
- no exact application-lease duration is fabricated from the 3,600-second maximum;
- no concrete sink/executable/runtime activation is implied;
- exact-final-head repository-required CI is recorded without inheriting predecessor PASS.

## Next gate

After C03e-SY validation and immutable evidence closure, the recommended next checkpoint is a documentation-only source-layout/compatibility selection for the fresh proof-submission verifier-time correction.

That later gate must select exact symbols, error representation, compatibility strategy, changed paths and focused tests before any Rust mutation. It must determine how the corrected fallible path stops using pre-AJ `authentication_now_unix_seconds` as proof authority while preserving existing infallible compatibility and the same request-owned verifier-time provider for post-auth worker custody.

Provider policy materialization, concrete pre-AJ failure sink, executable caller wiring and runtime activation remain later independent gates.

## STOP

After this docs-only contract and its validation/evidence workflow: STOP.

Do not merge, convert ready-for-review, close the PR, delete branches, rewrite history, deploy, restart services, activate provider/sink/executable/runtime behavior, alter credentials/configuration, or expand into Rust/source materialization.
