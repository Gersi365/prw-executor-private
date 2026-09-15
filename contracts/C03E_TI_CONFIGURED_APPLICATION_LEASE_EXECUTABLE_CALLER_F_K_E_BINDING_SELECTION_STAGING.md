# C03e-TI — configured application-lease executable-caller F/K/E binding selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`CONFIGURED_APPLICATION_LEASE_EXECUTABLE_CALLER_F_K_E_BINDING_SELECTION`

Selection result:
`TH_CONFIGURED_LEASE_WRAPPER_SELECTED_AS_CALLER_READY_SEAM / PRE_AJ_F_USES_FRESH_SERVER_VERIFIER_WALL_CLOCK / LOCKED_300_SECOND_CHALLENGE_WINDOW_WITH_CHECKED_ARITHMETIC / TIMING_CAUSE_IS_PRWA_VERIFIER_SOURCE_ERROR / CLOCK_FAILURE_MAPS_TO_ACQUISITION / EXPIRY_OVERFLOW_MAPS_TO_ARITHMETIC_OVERFLOW / K_TERMINALLY_CONSUMES_EXACT_TIMING_ERROR_AND_UNTOUCHED_REQUEST / E_TERMINALLY_CONSUMES_REAL_ADMISSION_OBSERVATION_AFTER_LOWER_FAILURE_CUSTODY / C_AND_R_REMAIN_CALLER_SUPPLIED_UNCHANGED / NO_LOGGING_RETRY_REQUEUE_RESTART_OR_FALLBACK / ONE_PATH_FUTURE_SOURCE_CEILING / NO_MAIN_OR_RUNTIME_ACTIVATION`

## 1. Scope and authoritative predecessor

C03e-TI is documentation-only. It selects the smallest caller-ready composition immediately after evidence-closed C03e-TH. It does not materialize Rust, invoke the TH wrapper, modify `linux_bootstrap::run()`, modify `main.rs`, activate any remote companion, listener, readiness, network or service path, merge, deploy or restart.

Authoritative predecessor is C03e-TH PR #647:

- branch: `phase-152-c03e-th-production-application-lease-config-source-custody-materialization`;
- head: `4f7767277cb4f6cf41bddd2d46e1a81dfb8aa77a`;
- tree: `9aba992efbd251a5f532e044c28a4d2418ce7311`;
- state: draft / open / unmerged / evidence-closed;
- immutable Drive audit ID: `1_s9unW1jb7KonvkuE57Dw3f1iunImcgH`;
- immutable audit revision count: exactly `1`;
- current revision: `0Bz5eMiLa5v9xRXlpcDdEL09mNVMzR0hXUDhpWmtDYkpsVDZZPQ`;
- previous revision: `null`.

Integrated `main` remains outside the branch line:

- head: `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree: `63b8e59ca53797fdea6b95432e16f35eaf473604`.

## 2. Exact-current source anchors

All findings are pinned to exact C03e-TH head.

- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
  - blob `764d7a004bcd2ea1647a503f4fd8ea1c096f468b`;
  - owns the TH configured application-lease wrapper and existing TF sibling.
- `crates/prw-agent/src/main.rs`
  - blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1`;
  - still performs identity preflight and calls only `prw_agent::linux_bootstrap::run()`.
- `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs`
  - blob `364f27e2bf71fb009192a6efbf15dc4fc0e8e1fb`;
  - owns `RemoteSessionAdmissionTimingSourceError` and exact intact-request K carrier.
- `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`
  - blob `76bb47248dcac1fd2f58787bb38cc92166b2ecf3`;
  - owns `RemoteSessionRealAdmissionError` and TD post-auth lease failure classes.
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
  - blob `8df82e8240374eb697460aebed80f9f2e4e46482`;
  - owns `RemoteSessionProductionPreAjTiming` and rejection/request compatibility surfaces.
- `crates/prw-session/src/prwa_verifier_source.rs`
  - blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`;
  - exposes the selected verifier wall clock and locked challenge lifetime.

## 3. Exact TH wrapper being selected

TH materializes the crate-private async wrapper:

`run_with_production_durable_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion_from_configured_production_sources_with_pre_aj_timing_and_configured_application_lease_policy`

It accepts caller-supplied `F`, `C`, `R`, `E`, `K`, loads and validates `PRW_REMOTE_APPLICATION_LEASE_SECONDS` exactly once, then delegates to the TF typed-policy sibling.

Its exact new production timing bound is:

```text
F: FnMut(&DeviceId)
    -> Result<
        RemoteSessionProductionPreAjTiming,
        RemoteSessionAdmissionTimingSourceError<Cause>
    >
```

The exact request-owned verifier provider beneath this boundary remains:

```text
fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>
```

TI does not change the TH wrapper signature.

## 4. Why F can now be concrete

C03e-TA selected the server-local PRWA verifier wall clock as the pre-AJ challenge timing authority and the locked challenge lifetime of 300 seconds. Later C03e-TB/TG/TH independently selected and materialized application-lease policy provenance, so F no longer needs to manufacture or carry an application lease.

The current production timing carrier contains only challenge validity:

`RemoteSessionProductionPreAjTiming { challenge_validity_unix_seconds }`.

Therefore the previously blocked complete production pre-AJ F can now be selected without inventing application-lease timing or proof-submission timing.

## 5. Selected concrete F provider

Selected future helper name:

`production_remote_session_pre_aj_timing_from_verifier_clock`

Selected exact semantic signature:

```text
fn production_remote_session_pre_aj_timing_from_verifier_clock(
    expected_device_id: &DeviceId,
) -> Result<
    RemoteSessionProductionPreAjTiming,
    RemoteSessionAdmissionTimingSourceError<PrwaVerifierSourceError>,
>
```

`expected_device_id` remains correlation/selector input only. The helper must not derive time, lifetime, identity, authorization, endpoint, transport or policy from it.

Selected order per eligible request:

1. invoke `prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds()` exactly once;
2. map source failure to `RemoteSessionAdmissionTimingSourceError::Acquisition(exact_error)`;
3. set `issued_at = fresh_sample`;
4. compute `expires_at = issued_at.checked_add(PRWA_VERIFIER_CHALLENGE_LIFETIME_SECONDS)`;
5. map checked-add overflow to `RemoteSessionAdmissionTimingSourceError::ArithmeticOverflow`;
6. construct exactly `RemoteSessionProductionPreAjTiming::new(issued_at..expires_at)`;
7. return it with no second clock read.

The exact challenge lifetime constant remains:

`prw_session::prwa_verifier_source::PRWA_VERIFIER_CHALLENGE_LIFETIME_SECONDS`

which remains equal to the locked session-authentication maximum of 300 seconds.

No `Policy`, `InvalidChallengeValidity` or `InvalidApplicationLease` error is fabricated by this helper. With a successful fresh `u64` sample and checked addition of the locked positive 300-second lifetime, the returned range is the exact selected challenge window.

## 6. Freshness and timing-domain separation

The F sample is pre-AJ challenge issue timing only.

It is not reused for:

- proof-submission verifier time;
- post-auth application-lease issue time;
- worker request-loop verifier time;
- process start time;
- scheduling time;
- remote-provided time.

The request-owned verifier provider still performs its separate fresh proof-time sample inside the SZ/TD authentication transaction and remains available for the distinct post-auth lease issue-time sample and later worker custody.

No cache, retry, fallback, stale reuse, saturation, wrap, clamp or hidden second clock is selected.

## 7. Selected K terminal custodian

C03e-TA already selected K as a terminal exact-request timing-failure custodian. TI now selects the concrete caller-side binding shape.

Selected future helper name:

`dispose_production_remote_session_admission_timing_failure`

Selected exact semantic payload:

```text
RemoteSessionAdmissionTimingFailure<
    LinuxAgentProductionRemoteCapabilityDispatcher,
    fn() -> Result<u64, PrwaVerifierSourceError>,
    RemoteSessionAdmissionTimingSourceError<PrwaVerifierSourceError>,
>
```

The helper must:

1. receive the failure by value exactly once;
2. consume it through `into_parts()` or an equivalently explicit ownership-consuming operation;
3. terminally release the exact timing error and exact untouched request;
4. return unit;
5. perform no retry, requeue, alternate timing sample, sender clone, replacement request, AJ/transport/session operation, scheduling replay, callback redirection, process exit or restart.

It must not log or expose dispatcher, verifier provider, request, DeviceId, SessionId, request ID or source-cause internals.

After normal K return, the existing supervisor continuation law remains authoritative for independent later work. The failed request is never resampled or requeued.

## 8. Selected E terminal admission-failure disposition

Selected future helper name:

`dispose_production_remote_session_real_admission_failure`

Selected exact semantic signature:

```text
fn dispose_production_remote_session_real_admission_failure(
    expected_device_id: DeviceId,
    error: RemoteSessionRealAdmissionError,
)
```

The helper receives only bounded correlation plus the already-returned real-admission error. It terminally consumes/releases both values and returns unit.

It must not:

- retry admission;
- requeue or reconstruct a request;
- resample verifier time;
- reopen or replace a peer/session;
- fabricate pending-session abort or authenticated-session deletion;
- restart the process;
- convert failure into success;
- log source-error internals or identity/correlation values.

Any stage-specific peer/session/authentication/binding cleanup required by a real admission failure remains owned by the existing lower transaction that produced `RemoteSessionRealAdmissionError`. E is not a second cleanup authority.

`expected_device_id` is correlation only and is not authority.

## 9. C and R remain explicitly caller-supplied

C03e-SK selected the provenance law for completion and rejection hooks but intentionally did not select concrete production sinks.

TI does not silently invent them.

The caller-ready adapter therefore continues to accept:

```text
C: FnMut(
    DeviceId,
    RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection,
) + Send + 'static
```

and:

```text
R: FnMut(
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest<
        LinuxAgentProductionRemoteCapabilityDispatcher,
        fn() -> Result<u64, PrwaVerifierSourceError>,
    >,
) + Send + 'static
```

unchanged.

TI selects no logging, telemetry, persistence, retry, requeue, dead-letter or process-exit semantics for C or R. A later executable activation must either provide already-authoritative concrete C/R dispositions or return to a separate selection checkpoint before `main.rs` wiring.

## 10. Selected caller-ready adapter

Selected future helper name:

`run_with_production_durable_reachability_requester_rendezvous_configured_application_lease_companion_with_selected_timing_and_failure_custody`

It remains crate-private and async.

Its only generic caller-supplied policy hooks are C and R. It does not accept F, E or K from its caller.

Selected semantic shape:

```text
async fn ...<C, R>(
    on_completion: C,
    on_rejection: R,
) -> Result<
    LinuxAgentBootstrapWithRemoteReport,
    LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredApplicationLeaseCompanionError,
>
```

with C/R bounds exactly as section 9.

It must invoke the TH wrapper exactly once with:

1. `production_remote_session_pre_aj_timing_from_verifier_clock` as F;
2. caller-supplied C unchanged;
3. caller-supplied R unchanged;
4. `dispose_production_remote_session_real_admission_failure` as E;
5. `dispose_production_remote_session_admission_timing_failure` as K.

It returns the exact TH wrapper result unchanged.

## 11. TH/TG configuration failure remains outside K and E

The TH wrapper loads and validates application-lease configuration before it constructs the TF expected-request channel or enters configured population.

Therefore:

- `LinuxAgentRemoteApplicationLeasePolicySourceError` remains process-configuration failure;
- configured application-lease source failure invokes neither K nor E;
- the new caller-ready adapter must not catch or remap that failure into K/E;
- no retry/default/fallback is selected.

The exact TH wrapper error remains the adapter result error unchanged.

## 12. Existing lower error domains remain unchanged

Pre-AJ F failures:
- go only to K with the exact intact request.

Real admission failures after request decomposition/AJ entry:
- go only to E under existing lower transaction semantics.

Post-proof verifier-time failure:
- remains authentication transaction cleanup, then real-admission `Authentication` failure to E.

Post-auth application-lease verifier-time failure:
- remains `ApplicationLeaseVerifierTime` to E after existing binding-stage close diagnostic.

Post-auth checked lease expiry overflow:
- remains `ApplicationLeaseExpiryOverflow` to E after existing binding-stage close diagnostic.

Structural binding failure:
- remains `Binding` to E.

None of these are rerouted to K.

## 13. No async-runtime or executable activation selection

The selected adapter remains async because the existing TH wrapper is async.

TI does not select how a synchronous process entrypoint will drive that future. In particular it does not select:

- a new Tokio runtime;
- `block_on`;
- an existing local runtime as an async executor for higher-owner population;
- thread spawn solely to drive the wrapper;
- nesting inside the remote companion thread;
- `main.rs` invocation;
- `linux_bootstrap::run()` replacement.

Async execution ownership remains a separate gate. This avoids introducing a second runtime or deadlock/reentrancy risk without an explicit review.

## 14. First source-materialization ceiling

If separately authorized, the immediate TI source successor is limited to exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

It may add only:

1. the selected concrete F helper;
2. the selected K terminal custodian helper;
3. the selected E terminal disposition helper;
4. the selected C/R-parameterized caller-ready async adapter;
5. bounded pure/type tests for exact function signatures, F success/failure/overflow construction through injected/pure helper seams if needed, and K/E ownership consumption without I/O.

No second Rust path is selected.

The source successor must stop if implementation requires `linux_bootstrap.rs`, `main.rs`, a public API, Cargo/lock changes, workflow changes, a runtime builder, service/package changes, a new configuration source, logging/telemetry/persistence, lower propagation mutation or any other path.

## 15. Future source validation expectations

A separately authorized source successor should prove:

- exactly one changed Rust path;
- F cause type is exact `PrwaVerifierSourceError`;
- F obtains one fresh verifier-clock sample per invocation;
- F uses exact locked challenge lifetime constant;
- source error -> `Acquisition(exact_error)`;
- checked expiry overflow -> `ArithmeticOverflow`;
- F returns challenge-only `RemoteSessionProductionPreAjTiming`;
- DeviceId is not used as timing/identity authority;
- K consumes exact error/request once and returns unit;
- K has no log/retry/requeue/restart side effect;
- E consumes DeviceId/error once and returns unit;
- E does not duplicate lower cleanup;
- adapter accepts only C/R policy hooks;
- adapter calls TH wrapper exactly once with selected F/E/K;
- C/R are forwarded unchanged;
- TH error is returned unchanged;
- no `main.rs` or `linux_bootstrap::run()` reachability is added.

## 16. Identity and authorization invariants

TI preserves:

- logical DeviceId distinct from transport/endpoint identity;
- request IDs as correlation only;
- DeviceId as timing-source selector/correlation only;
- application-lease policy as process-owned validated configuration, not remote-selected;
- proof and lease issue-time samples as verifier-owned fresh observations;
- existing capability authorization and requester/rendezvous authority separation;
- no endpoint/IP/transport coordinate promoted into logical identity or authorization.

## 17. Explicit non-actions / STOP

TI performs no Rust/source/runtime mutation. It does not:

- materialize F/K/E helpers;
- materialize the caller-ready adapter;
- choose concrete C or R sinks;
- invoke TH;
- modify `linux_bootstrap.rs` or `main.rs`;
- select an async runtime driver;
- activate a listener/readiness/network path;
- change application-lease configuration source or policy;
- change verifier-time provider semantics;
- change K/E signatures or lower propagation;
- add logging/telemetry/persistence/retry/requeue/restart/fallback;
- mutate Cargo/lockfiles/workflows/Android/package/service/systemd/repository configuration;
- merge, ready-convert, close PRs, delete branches, rewrite history, deploy, restart or destructively clean evidence.

After TI exact-head validation, immutable evidence publication and post-publication GitHub closure binding: STOP before source materialization.
