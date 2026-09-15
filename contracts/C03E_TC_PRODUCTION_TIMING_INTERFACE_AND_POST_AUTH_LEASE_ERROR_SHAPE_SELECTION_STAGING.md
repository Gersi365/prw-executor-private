# C03e-TC — Production Timing Interface and Post-Auth Lease Error Shape Selection

Status: `SELECTION — VALIDATION PENDING`

Date: `2026-09-15`

Boundary:
`PRODUCTION_TIMING_INTERFACE_AND_POST_AUTH_LEASE_ERROR_SHAPE_SELECTION`

Selection result:
`PRODUCTION_PRE_AJ_F_RETURNS_CHALLENGE_TIMING_ONLY / REQUEST_OWNED_FALLIBLE_VERIFIER_PROVIDER_REUSED_FOR_DISTINCT_PROOF_AND_POST_AUTH_LEASE_SAMPLES / EXPLICIT_VALIDATED_APPLICATION_LEASE_POLICY_SEPARATE_FROM_F / PRE_AJ_F_TO_K_ERROR_FAMILY_REUSED_UNCHANGED / POST_AUTH_LEASE_FAILURE_EXTENDS_REAL_ADMISSION_ERROR_NOT_F_TO_K / BINDING_STAGE_CLOSE_DIAGNOSTIC_REUSED_THROUGH_NARROW_INTERNAL_HELPER / EXISTING_REMOTE_SESSION_REAL_ADMISSION_TIMING_AND_EXISTING_ADMISSION_APIS_PRESERVED_AS_COMPATIBILITY_SURFACES / NEW_SIBLING_PRODUCTION_TRANSACTION_SELECTED / NO_SOURCE_MATERIALIZATION / NO_RUNTIME_ACTIVATION`

## 1. Scope and authoritative predecessor

This checkpoint is documentation-only.

Authoritative predecessor is evidence-closed C03e-TB:

- PR: `#641`;
- branch: `phase-152-c03e-tb-application-lease-policy-post-auth-provenance-selection`;
- exact head: `778962bf602b126ed0dc888f89855e406d5cb26e`;
- exact tree: `007bca29fd119f99a83a383114623a7d77b29952`;
- status binding: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- PR remains draft/open/unmerged.

C03e-TB selected:

- one explicit process-owned bounded application-lease lifetime policy;
- exact bounds `1..=3_600` whole seconds;
- no implicit default;
- one distinct fresh verifier-time sample only after authentication succeeds;
- checked post-auth lease expiry construction;
- post-auth lease-source failure as a binding-stage admission failure rather than F -> K timing failure;
- the fact that the existing pre-AJ full timing bundle is not the final production interface.

C03e-TC closes only the minimal interface and error-shape decision required to represent that law without placeholder lease values or compatibility-semantic corruption.

It does not materialize Rust source, migrate a higher-owner caller, select an executable configuration source, activate a runtime, wire `main.rs`, bind/listen, publish readiness, deploy, restart or merge.

## 2. Exact current production-shape mismatch

Exact current source defines:

`RemoteSessionRealAdmissionTiming`

with exactly:

1. `challenge_validity_unix_seconds: Range<u64>`;
2. `authentication_now_unix_seconds: u64`;
3. `application_lease_unix_seconds: Range<u64>`.

The current fallible pre-AJ seam therefore has shape:

`F: FnMut(&DeviceId) -> Result<RemoteSessionRealAdmissionTiming, TimingError>`.

That shape requires a fully formed absolute application-lease range before AJ begins.

C03e-TB instead requires the application-lease issue time to be sampled only after authentication succeeds.

Therefore the existing three-field timing carrier cannot be used as the final production F output without constructing a fake, stale, static, challenge-derived, pre-AJ or otherwise non-authoritative lease range.

C03e-TC forbids such placeholder adaptation.

## 3. Existing request-owned verifier provider is sufficient

`RemoteSessionExpectedDeviceAdmissionRequest<D, T>` already owns:

- expected logical `DeviceId`;
- `SessionId`;
- authentication request ID;
- dispatcher `D`;
- verifier-time provider `T`.

The C03e-SZ fresh transaction already accepts that provider by mutable reference:

`verifier_time_unix_seconds: &mut T`

with current bound:

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

C03e-TC therefore selects **no new clock authority and no second provider object**.

The exact same request-owned provider instance is retained across the fresh admission transaction and is invoked at distinct custody points:

1. once at the C03e-SZ proof-submission freshness point;
2. once again only after authentication succeeds for application-lease issue time;
3. after successful admission, the same provider object remains available to move into the existing worker admission for later per-request capability verifier-time use.

No provider clone, replacement provider, cached value, process-global substitute or hidden wall-clock read is selected.

## 4. Selected production pre-AJ timing carrier

C03e-TC selects a new narrow production-only timing carrier with semantic name:

`RemoteSessionProductionPreAjTiming`

Its exact payload is only:

`challenge_validity_unix_seconds: Range<u64>`.

It does **not** contain:

- `authentication_now_unix_seconds`;
- application-lease issue time;
- application-lease expiry time;
- application-lease absolute range;
- lease lifetime policy;
- proof-submission verifier time;
- transport time;
- worker/request-loop time.

The selected type is Agent-internal production composition state rather than a new wire/public protocol object.

A later source checkpoint may provide only the minimal ownership helpers required to construct and consume this one range.

## 5. Selected production F shape

The selected production fallible pre-AJ timing callback becomes a sibling production shape:

`F: FnMut(&DeviceId) -> Result<RemoteSessionProductionPreAjTiming, RemoteSessionAdmissionTimingSourceError<Cause>>`.

`DeviceId` remains selector/correlation context only and is not timing authority.

The provider law from C03e-TA remains unchanged:

- sample the existing server-local verifier wall clock once when F is invoked for one eligible vacant request;
- construct challenge issue time from that sample;
- construct challenge expiry with checked addition of the locked 300-second challenge lifetime;
- return only that exact challenge range.

No proof-time or lease-time sample is taken by F.

## 6. Existing pre-AJ error family and K remain unchanged

C03e-TC does not create a new K error taxonomy.

The existing:

`RemoteSessionAdmissionTimingSourceError<Cause>`

remains the pre-AJ timing-source error family for the production F -> K seam.

The existing:

`RemoteSessionAdmissionTimingFailure<D, T, TimingError>`

remains the exact carrier of pre-AJ timing failure plus the untouched original request.

The selected K law from C03e-TA remains unchanged.

The production pre-AJ path may use only the variants relevant to challenge acquisition/construction. Existing application-lease-related variants remain compatibility surface and are not repurposed as post-auth lease failures.

In particular, post-auth lease verifier-time failure or lease-expiry overflow must **not** be wrapped in `RemoteSessionAdmissionTimingSourceError` and must never reach K.

## 7. Selected private request-preparation adaptation

The current private helper:

`prepare_expected_request_with_timing_result(...)`

hardcodes `RemoteSessionRealAdmissionTiming` as its success carrier.

C03e-TC selects a behavior-preserving private genericization over the success timing carrier so the same exact duplicate-device and K ownership law can serve both:

- historical `RemoteSessionRealAdmissionTiming` callers;
- the new `RemoteSessionProductionPreAjTiming` production sibling.

The genericization must not change:

- duplicate-active-device rejection ordering;
- whether F is called;
- exact F call count;
- exact request ownership;
- K call count;
- retry/requeue behavior;
- callback ordering;
- request replacement behavior.

This is internal type-shape reuse only, not runtime behavior selection.

## 8. Selected application-lease policy carrier

C03e-TC selects a narrow Agent-internal typed policy value:

`RemoteSessionApplicationLeasePolicy`

with one private semantic field:

`lifetime_seconds: u64`.

Construction is valid only when:

`1 <= lifetime_seconds <= prw_remote_bridge::MAX_REMOTE_SESSION_LEASE_SECONDS`.

Current maximum remains `3_600` seconds.

The selected pure validation error semantic name is:

`RemoteSessionApplicationLeasePolicyError`

with bounded invalid-lifetime classification.

The policy type has no implicit `Default` and no fallback value.

It is created before real admission use and passed separately from F.

C03e-TC does not select the executable source of the raw duration. Environment variable, CLI, systemd credential, config file, database, Android setting, registry field and remote protocol remain separately gated.

## 9. Policy is separate from timing authority

`RemoteSessionApplicationLeasePolicy` selects only lease **duration policy**.

It does not contain or mint:

- issue time;
- expiry time;
- verifier-time provider;
- `DeviceId` authority;
- transport identity;
- request identity;
- capability authority;
- scheduling authority;
- reachability state.

A valid policy value alone cannot construct an absolute lease.

The absolute interval still requires the C03e-TB-selected fresh post-auth verifier-time sample.

## 10. Selected sibling fresh production admission transaction

C03e-TC selects a new sibling production transaction semantic surface:

`admit_expected_remote_device_session_with_fresh_verifier_time_and_application_lease_policy(...)`.

Its relevant timing inputs are:

- `challenge_validity_unix_seconds: Range<u64>` from `RemoteSessionProductionPreAjTiming`;
- `authentication_request_id: u64` from the exact request;
- `verifier_time_unix_seconds: &mut T` from the exact request-owned provider;
- `application_lease_policy: RemoteSessionApplicationLeasePolicy` as separate trusted process-owned policy.

It does not accept:

- `authentication_now_unix_seconds`;
- preconstructed application-lease absolute range.

Existing registry, expected-transport, accepted-peer, challenge-preparation and fresh authentication semantics remain unchanged.

## 11. Selected exact sampling order inside the sibling transaction

The sibling production transaction must preserve this exact order:

1. resolve expected current transport identity through current authority;
2. accept the exact authenticated lower-transport peer;
3. re-read current authority for challenge preparation;
4. begin the exact registry-bound session challenge;
5. run existing fresh authentication transaction;
6. acquire the C03e-SZ proof-submission sample through the exact request-owned provider;
7. successfully complete `submit_proof(...)` and obtain the authenticated logical session;
8. only after that success, invoke the **same provider instance** exactly once more for application-lease issue time;
9. read the already-validated `RemoteSessionApplicationLeasePolicy` lifetime;
10. compute lease expiry with checked addition;
11. call existing post-auth binding composition with the exact resulting range;
12. on success, return the existing `AuthenticatedRemoteSessionRuntimeOwner`.

No lease sample is acquired before authentication success.

No proof sample is reused as lease issue time.

No lease sample is reused as later capability request time.

## 12. Selected post-auth lease failure representation

C03e-TC selects extending the existing public bounded admission-phase error family:

`RemoteSessionRealAdmissionError`.

It adds two semantically exact post-auth application-lease classifications:

- `ApplicationLeaseVerifierTime(PrwaVerifierSourceError)`;
- `ApplicationLeaseExpiryOverflow`.

The verifier-time variant preserves the exact underlying `PrwaVerifierSourceError` as its source.

The overflow variant has no underlying source.

No raw `SystemTimeError`, wall-clock value, request data, device identity, provider internals or policy value is exposed in the bounded display/debug classification.

## 13. Why direct admission-error variants are selected

C03e-TC intentionally does not add another public nested error enum for this two-case phase.

The existing admission transaction already owns the phase taxonomy:

- `Registry`;
- `Accept`;
- `Challenge`;
- `Authentication`;
- `Binding`.

Adding the two exact lease-preparation variants directly is the smaller API change while preserving the distinction required by C03e-TB.

`ApplicationLeaseVerifierTime` is not `Authentication`.

`ApplicationLeaseExpiryOverflow` is not `Binding(RemoteBridgeError)`.

Actual rejection by `RemoteSessionLease::new(...)` after a successfully constructed interval remains the existing `Binding(RemoteBridgeError)` lane.

## 14. Selected post-auth failure cleanup helper

The exact binding-stage close diagnostic already exists in:

`authenticated_remote_session_runtime.rs`:

- close code: `2`;
- reason: `remote session binding failed`.

C03e-TC selects one narrow Agent-internal helper semantic surface to reuse that exact diagnostic for lease-preparation failure before `compose_authenticated_remote_session(...)` can be called.

Selected semantic helper name:

`close_authenticated_remote_session_binding_failure(...)`.

It accepts only the already-authenticated peer reference and performs exactly the existing code-2 close.

It does not:

- construct an admission error;
- abort a pending challenge;
- delete an authenticated session;
- retry;
- sample time;
- select policy;
- construct a lease;
- publish readiness;
- perform any other cleanup.

This avoids duplicating or widening the diagnostic while keeping post-auth peer cleanup explicit at the transaction boundary that still owns the peer.

## 15. Selected failure behavior after authentication success

If the second provider call for lease issue time fails:

- call `close_authenticated_remote_session_binding_failure(...)` exactly once;
- return `RemoteSessionRealAdmissionError::ApplicationLeaseVerifierTime(exact_error)`;
- construct no lease range;
- call no binding composition;
- construct no capability/session runtime owner;
- spawn no worker;
- perform no retry/resample/fallback/re-authentication.

If checked lease expiry addition overflows:

- call the same close helper exactly once;
- return `RemoteSessionRealAdmissionError::ApplicationLeaseExpiryOverflow`;
- call no binding composition;
- construct no worker/session owner;
- perform no retry/fallback/clamping/saturation/wrapping.

Pending-session abort is forbidden because authentication already succeeded.

Authenticated-session deletion is not invented because no authoritative deletion API exists.

## 16. Binding error lane remains unchanged

If post-auth lease time sampling and expiry construction succeed, the exact range is passed to existing:

`compose_authenticated_remote_session(...)`.

That function remains authoritative for `BoundRemoteSession::new(...)` and `RemoteSessionLease::new(...)` structural validation.

If the existing binding composition rejects the range or other binding input:

- its existing code-2 close remains authoritative;
- the resulting `RemoteBridgeError` remains mapped through existing `RemoteSessionRealAdmissionError::Binding`.

C03e-TC does not flatten binding validation failure into the new lease-preparation variants.

## 17. Existing public/historical surfaces remain compatibility surfaces

C03e-TC does not authorize mutation or removal of:

- `RemoteSessionRealAdmissionTiming`;
- its constructor/accessors/`into_parts` behavior;
- `admit_expected_remote_device_session(...)`;
- `admit_expected_remote_device_session_with_fresh_verifier_time(...)`;
- historical infallible collection overloads;
- historical fallible timing overloads;
- existing tests that exercise those surfaces.

Those APIs may remain available for compatibility while the new production sibling is introduced separately.

No existing caller is silently switched by this selection checkpoint.

## 18. Existing `authentication_now_unix_seconds` compatibility field is not migrated

The existing compatibility field:

`authentication_now_unix_seconds`

remains untouched on `RemoteSessionRealAdmissionTiming`.

It is not copied into `RemoteSessionProductionPreAjTiming` because C03e-SZ made proof verifier time request-owned and freshly sampled immediately before proof submission.

The new production sibling therefore has no compatibility authentication-now parameter.

C03e-TC does not remove the old field from historical surfaces.

## 19. Existing lease-related pre-AJ error variants are not repurposed

`RemoteSessionAdmissionTimingSourceError<Cause>` currently includes legacy/general structural variants such as:

- `InvalidApplicationLease`;
- `ArithmeticOverflow`.

C03e-TC does not delete them.

It also does not use them to represent the C03e-TB-selected post-auth lease acquisition/expiry failure.

Their existence on compatibility surfaces is not evidence that application lease belongs in pre-AJ F.

## 20. Production repeated-admission sibling shape

A later source checkpoint may add a sibling production repeated-admission path that consumes:

- `RemoteSessionApplicationLeasePolicy` once as trusted process-owned configuration;
- fallible production F returning `RemoteSessionProductionPreAjTiming`;
- unchanged K for pre-AJ timing failures;
- request-owned fallible verifier provider `T`;
- existing completion/rejection/admission-failure callbacks.

For each eligible request:

- F provides only challenge timing;
- the request-owned provider remains inside the exact request until the request is destructured for admission;
- the sibling fresh transaction performs proof and post-auth lease samples;
- successful admission returns the same provider object to existing worker custody.

No new provider channel, provider registry, provider clone or process-global clock owner is selected.

## 21. Shutdown and supervisor semantics remain unchanged

C03e-TC does not alter the existing supervisor race or drain law.

If shutdown wins while one sibling production admission is in flight, existing in-flight admission drain/cleanup ordering remains authoritative.

The new post-auth lease sample is simply part of that one admission future.

No detached lease task, timeout task, worker or retry loop is selected.

## 22. Pure policy validation occurs before admission use

`RemoteSessionApplicationLeasePolicy` must be validated before the repeated-admission path can use it.

An invalid raw lifetime must fail before any request-specific:

- F invocation;
- peer acceptance;
- challenge;
- proof;
- lease sampling;
- binding;
- worker spawn.

C03e-TC does not select which executable/configuration population layer obtains the raw value.

A later configuration checkpoint must preserve this fail-closed pre-use validation.

## 23. Identity and authority boundaries remain unchanged

The interface split does not merge authority domains.

- `DeviceId` remains logical identity/correlation context, not time authority.
- `TransportIdentity` remains lower-transport identity.
- request ID remains correlation only.
- lease lifetime policy remains configuration/policy, not clock authority.
- verifier provider remains time authority, not scheduling/capability authority.
- lease validity remains application-session lifetime, not capability grant.
- current registry, current transport binding, current lease validity, request decoding, policy and dispatcher admission remain re-evaluated per later capability request.

No IP address becomes logical identity.

## 24. Exact audited current anchors

C03e-TC is grounded in exact C03e-TB head `778962bf602b126ed0dc888f89855e406d5cb26e` and preserves these exact source/contract anchors:

- `contracts/C03E_TB_APPLICATION_LEASE_POLICY_AND_POST_AUTH_PROVENANCE_SELECTION_STAGING.md`
  - blob `6a455a48d90e723f30e9be9865a30b643d6f391f`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
  - blob `1539b6b9a08bf18883d7a16022f15f7c240eaf08`;
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`
  - blob `17bb04bef8436574b7bc1778f8fa436cc51fc94e`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
  - blob `08deec2f12095738c9e71fdb133914733e68263e`;
- `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs`
  - blob `364f27e2bf71fb009192a6efbf15dc4fc0e8e1fb`;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - blob `44d2285dbb035c9a6f692d897af8283e95fbafb2`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`
  - blob `5e7926a3dc187de4a6aa6f27b0b77ef4208488d2`;
- `crates/prw-session/src/prwa_verifier_source.rs`
  - blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`;
- `crates/prw-remote-bridge/src/lib.rs`
  - blob `ad6833cc4e71a372810b260f157126a3df6645e5`;
- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
  - blob `b387b49bb179a052a9eac7b29d4b00bbb31884ca`;
- `crates/prw-agent/src/linux_bootstrap.rs`
  - blob `0a41a8ea58cc47757c24cf314035452f9100c1e5`.

C03e-TC changes none of these source paths.

## 25. Selected future source-materialization ceiling

C03e-TC itself authorizes no source mutation.

If separately authorized, the first narrow core source-materialization checkpoint should be constrained to the minimal interface/error transaction layer and should not yet migrate higher-owner/bootstrap callers.

Preferred first-source ceiling:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - add `RemoteSessionProductionPreAjTiming` only;
2. `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`
   - add `RemoteSessionApplicationLeasePolicy` + validation error;
   - add the two selected admission-error variants;
   - add the sibling fresh production transaction;
3. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
   - add only the narrow code-2 close helper required by the sibling transaction;
4. `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - expose only the narrow crate/public symbols required by the new core surfaces.

A subsequent separately gated caller-integration checkpoint may adapt the production collection/helper and only later higher-owner/bootstrap custody.

This staged ceiling avoids mixing interface correctness with runtime activation or broad caller migration.

## 26. Source checkpoint must preserve old APIs

Any later source materialization under this selection must preserve old APIs byte-semantically unless an explicit compatibility-only visibility/import edit is required.

In particular it must not silently rewrite old functions to new post-auth lease semantics while retaining their old signatures.

New semantics must enter through new sibling surfaces.

## 27. Validation requirements for this docs-only checkpoint

C03e-TC is valid only if final topology proves:

- direct exact C03e-TB -> C03e-TC ancestry;
- merge base exact TB head `778962bf602b126ed0dc888f89855e406d5cb26e`;
- ahead `1`, behind `0`;
- exactly one changed path: this contract;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android/package/service/deployment/repository-config mutation;
- all audited source anchors above remain byte-identical to TB;
- integrated `main` remains untouched.

Any PASS claim must bind only to the exact final TC head.

`SKIPPED` is not PASS.

A docs-only TC head does not inherit Android PASS from any predecessor. Android is claimed only if a workflow actually registers and succeeds for the exact TC head.

## 28. Explicit non-actions / STOP

C03e-TC performs no Rust/source/runtime/API mutation.

It does not:

- add `RemoteSessionProductionPreAjTiming` in Rust;
- add `RemoteSessionApplicationLeasePolicy` in Rust;
- add any new error variant in Rust;
- add the sibling production transaction in Rust;
- add the binding-failure close helper in Rust;
- genericize the private request-preparation helper in Rust;
- migrate a production collection caller;
- migrate higher-owner custody;
- change `linux_bootstrap.rs`;
- select an environment variable or other executable lease configuration source;
- materialize the C03e-TA concrete F provider;
- materialize K;
- remove or mutate `RemoteSessionRealAdmissionTiming`;
- remove or mutate existing admission functions;
- add retry/requeue/fallback/cache/clamp/saturation behavior;
- add lease renewal/refresh/extension;
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

After this documentation-only selection checkpoint: `STOP` before source materialization.

The next separately gated checkpoint may materialize only the selected narrow core timing-interface/error transaction shape, without production caller migration or runtime activation.
