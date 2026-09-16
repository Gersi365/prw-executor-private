# C03e-TO Main Configured Production Remote Activation Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`MAIN_CONFIGURED_PRODUCTION_REMOTE_ACTIVATION_CALLSITE_EXIT_DIAGNOSTIC_SELECTION`

## 1. Exact predecessor

Authoritative predecessor is evidence-closed C03e-TN:

- PR `#653`;
- exact head `b5cd2021b7a8f0e755aa6aed81fb4c2752231938`;
- exact tree `f87fa5ed8c7b4a4f23d622d7aa8f1e8e13976886`;
- TN public-facade source blob `1b7ca3136d4a0d11f343cc4acd030275f6ff08af`;
- current `main.rs` blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1`.

C03e-TO is documentation-only. It selects the exact executable activation callsite and bounded terminal/startup projection but performs no source mutation or runtime activation.

## 2. Selected law

`IDENTITY_PREFLIGHT_UNCHANGED / PUBLIC_SPKI_EVENT_UNCHANGED / EXACT_ONE_CONFIGURED_PRODUCTION_REMOTE_FACADE_CALL / LEGACY_LOCAL_ONLY_RUN_NOT_CALLED / NO_RETRY_OR_LOCAL_ONLY_FALLBACK / LOCAL_REPORT_FIELDS_COUNTERS_AND_SIGNAL_EVIDENCE_PRESERVED / REMOTE_COMPANION_TOKEN_APPENDED_LAST / PROCESS_SUCCESS_REQUIRES_LOCAL_AND_REMOTE_SUCCESS / CONFIGURED_REMOTE_START_FAILURE_USES_BOUNDED_KIND_AND_SIGNAL_MASK_ONLY / NON_LINUX_PATH_UNCHANGED / ONE_PATH_FUTURE_ACTIVATION_CEILING_MAIN_RS / NO_SOURCE_MUTATION / NO_RUNTIME_ACTIVATION`

## 3. Existing Linux executable order remains authoritative

The future activation must preserve the current Linux executable order before the bootstrap call:

1. call `load_ubuntu_enrollment_signer_from_systemd_credential()` exactly once;
2. on identity-source failure, emit the existing exact bounded event:
   `prw-agent event=startup_failure kind=device_identity exit=failure signal_mask_restore=not_applicable`;
3. return `ExitCode::FAILURE` on that identity-source failure;
4. after identity success, emit the existing exact public-SPKI event:
   `prw-agent event=device_identity_loaded public_spki_sha256={...}`;
5. only then enter the selected configured-production remote facade.

No remote configuration/runtime construction may occur before successful identity preflight and the existing fingerprint event.

## 4. Exact future callsite replacement

A separately authorized source successor must replace exactly the current Linux call:

`prw_agent::linux_bootstrap::run()`

with exactly one call to:

`prw_agent::linux_bootstrap::run_with_configured_production_remote_companion()`

The future Linux path must contain zero calls to legacy `linux_bootstrap::run()` after this replacement.

The executable must not invoke both facades, retry the configured facade, fall back to local-only bootstrap, or perform a second configured-facade invocation.

## 5. Successful facade result decomposition

On:

`Ok(report: LinuxAgentBootstrapWithRemoteReport)`

the future main callsite must obtain exactly:

- the existing local `LinuxAgentBootstrapReport` through `report.local()`;
- the existing bounded `LinuxAgentRemoteProcessCompanionFinalization` through `report.remote()`.

The local report remains the sole source for all existing terminal fields, counters, cleanup token and signal-mask restore token.

The remote finalization value contributes only:

- `remote.is_success()` to the combined process-success decision;
- `remote.token()` to the one selected terminal field.

No remote value may replace or reinterpret local terminal/counter/cleanup/signal evidence.

## 6. Combined success law

Future executable success is exactly:

`local.is_success() && remote.is_success()`

Therefore `ExitCode::SUCCESS` is returned only when:

1. the existing local Linux bootstrap report is successful; and
2. TN remote finalization is exactly `ShutdownRequested + Joined`.

Any other local or remote terminal state returns `ExitCode::FAILURE`.

A locally successful bootstrap must not mask remote spawn failure, early unavailability, or remote companion panic.

A successful remote finalization must not mask local runtime/cleanup/signal failure.

## 7. Existing local terminal projection is preserved

The future main callsite must preserve the existing terminal event fields and their existing order:

- `terminal`
- `exit`
- `readiness_steps`
- `listener_armed_steps`
- `runtime_wakes`
- `wait_interruptions`
- `scheduling_attempts`
- `workers_registered`
- `worker_completions`
- `peer_rejections`
- `cleanup`
- `signal_mask_restore`

Their values remain derived exactly as today from the local report and counters.

No existing local token, counter, field name or ordering is selected for change.

## 8. Selected bounded remote terminal field

The future terminal event appends exactly one new final field after existing `signal_mask_restore`:

`remote_companion={}`

Its value is exactly:

`remote.token()`

No raw remote error, panic payload, endpoint, peer, request/session identifier, environment value, thread identity, timing value, or private source detail may be emitted.

## 9. Selected terminal event shape

Semantically, the future event is the current exact event plus one final field:

`prw-agent event=terminal terminal={} exit={} readiness_steps={} listener_armed_steps={} runtime_wakes={} wait_interruptions={} scheduling_attempts={} workers_registered={} worker_completions={} peer_rejections={} cleanup={} signal_mask_restore={} remote_companion={}`

The `exit` token is computed from the combined local+remote success law.

The `terminal`, counters, cleanup and signal-mask tokens remain local-report projections unchanged.

## 10. Configured-production startup failure policy

On:

`Err(failure: LinuxAgentConfiguredProductionRemoteStartFailure)`

the future executable must preserve the existing startup-failure event shape exactly:

`prw-agent event=startup_failure kind={} exit=failure signal_mask_restore={}`

The two values are exactly:

- `failure.kind().token()`;
- `failure.signal_mask_restore().token()`.

The process returns `ExitCode::FAILURE`.

No `Display`, `Debug`, `Error::source()`, raw environment/configuration value, endpoint, request/session ID, verifier time, private type name or inner error text may be logged.

## 11. Public failure tokens remain TN authority

The future executable does not invent new configured-production start tokens.

TN remains authoritative for:

- `remote_runtime`;
- `remote_application_lease`;
- `remote_configuration`;
- existing bootstrap-kind token passthrough.

TN remains authoritative for `signal_mask_restore=not_applicable` on failures occurring before existing signal-aware Linux bootstrap custody and exact nested bootstrap signal-mask evidence otherwise.

## 12. Identity and correlation invariants

Activation does not change identity semantics.

- the device enrollment signer remains device-local identity custody;
- public-SPKI SHA-256 remains the existing bounded startup fingerprint event;
- logical `DeviceId` is not replaced by an endpoint, IP address, request ID or transport identity;
- request IDs remain correlation only;
- remote configuration values are configuration, not identity or authorization;
- no new identity source is selected.

## 13. Runtime ownership effect of the later activation

A future `main.rs` replacement is an actual executable/runtime activation boundary.

Calling the TN public facade causes the already-materialized lower chain to own its selected behavior, including:

- the TL caller-owned current-thread Tokio runtime;
- configured application-lease source loading;
- configured production population and expected-request channel custody;
- existing Linux bootstrap execution;
- existing remote process companion lifecycle.

TO does not redesign or duplicate those lower owners.

TO does not claim that the future activation is behavior-free; it explicitly selects only the executable callsite and bounded diagnostic/exit projection over already-materialized behavior.

## 14. No fallback or retry

Future activation must not add:

- retry of runtime construction;
- retry of configuration acquisition;
- retry of configured population;
- retry of bootstrap;
- retry of remote finalization;
- fallback to `linux_bootstrap::run()`;
- local-only fallback after remote failure;
- process restart;
- service-manager restart policy;
- alternate diagnostics path.

Existing lower-level retry/cleanup semantics remain exactly where already owned; TO creates none.

## 15. Non-Linux executable behavior

The existing non-Linux main path remains unchanged.

It must continue to emit its existing `unsupported_platform` startup failure and return failure exactly as currently implemented.

No configured-production facade invocation is selected for non-Linux targets by TO.

## 16. Future source-materialization ceiling

If separately authorized after TO evidence closure, the activation source successor may modify exactly one Rust path:

`crates/prw-agent/src/main.rs`

Required predecessor blob:

`db6b8028c6df100a961a0fb5818347bea2fdc5c1`

Guard-only TN facade blob:

`crates/prw-agent/src/linux_bootstrap.rs`
`1b7ca3136d4a0d11f343cc4acd030275f6ff08af`

The TN facade path is not selected for modification.

## 17. Allowed future one-path mutation

The future `main.rs` checkpoint may change only:

1. the Linux bootstrap call from legacy `run()` to the TN facade exactly once;
2. successful-result handling to derive local and remote bounded reports;
3. success calculation to require both local and remote success;
4. the terminal event by appending exactly one final `remote_companion` field;
5. configured-production startup failure handling through existing bounded `kind()` and `signal_mask_restore()` accessors.

No other main behavior is selected for change.

## 18. Forbidden widening in the activation successor

The future activation checkpoint must STOP rather than widen scope if it requires changes to:

- `linux_bootstrap.rs`;
- `lib.rs`;
- higher-owner source;
- any remote-session runtime source;
- Cargo manifests or lockfiles;
- workflow files;
- Android source;
- service/systemd/package/deployment files;
- repository configuration.

It must also STOP if compilation requires a second source/test path rather than silently widening the one-path ceiling.

## 19. Validation expectations for future activation

The future exact-head source checkpoint must prove at minimum:

- rustfmt success;
- locked dependency graph success;
- Clippy with warnings denied success;
- full workspace tests success;
- full workspace build success;
- Android validation if triggered;
- exact one-path final diff;
- exactly one configured facade call;
- zero legacy local-only `run()` call on the Linux main path;
- exact bounded startup failure projection;
- remote terminal field appended last;
- combined local+remote process success law;
- no raw/private diagnostic leakage.

## 20. Explicit non-actions in C03e-TO

C03e-TO performs no:

- Rust/source mutation;
- `main.rs` change;
- executable invocation change;
- Tokio runtime construction;
- configured environment read;
- expected-request channel creation;
- listener/readiness/network activation;
- service-manager mutation;
- deployment/restart;
- database/schema/auth/control-plane mutation;
- dependency/Cargo/lock/workflow change;
- repository configuration change;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/history rewrite/force push;
- destructive evidence cleanup.

## 21. STOP

TO is selection-only.

The selected one-path `main.rs` activation successor is executable/runtime activation and requires separate explicit authorization after TO exact-head validation and immutable evidence closure.
