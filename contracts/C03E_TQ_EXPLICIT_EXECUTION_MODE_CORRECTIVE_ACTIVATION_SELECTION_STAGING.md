# C03e-TQ Explicit Execution Mode Corrective Activation Selection — STAGING

**Status:** `SELECTION — VALIDATION PENDING`
**Date:** 2026-09-16
**Repository:** `Gersi365/prw-executor-private`

## 1. Corrective purpose

C03e-TQ is a docs-only corrective selection after the blocked/restored C03e-TP activation attempt.

TP proved that the C03e-TO one-path `main.rs` activation ceiling is insufficient: direct configured-production activation makes the historical Phase 102 executable fixture fail before local listener readiness because the fixture intentionally owns only `XDG_RUNTIME_DIR` plus device identity and does not own configured-production remote inputs.

TQ selects an explicit executable mode authority that preserves fail-closed behavior without hidden defaults, fallback, dual invocation, fabricated remote configuration, or weakening the already-selected TN bounded diagnostic surfaces.

Canonical corrective law:

`EXPLICIT_NON_SECRET_PROCESS_EXECUTION_MODE / PRW_AGENT_EXECUTION_MODE / EXACT_LOCAL_ONLY_OR_CONFIGURED_REMOTE / STRICT_ASCII_EXACT_NO_TRIM_CASEFOLD_ALIAS_DEFAULT_OR_FALLBACK / IDENTITY_PREFLIGHT_AND_PUBLIC_SPKI_EVENT_PRECEDE_MODE_READ / MODE_SOURCE_FAILURE_IS_BOUNDED_EXECUTION_MODE_START_FAILURE / LOCAL_ONLY_CALLS_LEGACY_RUN_EXACTLY_ONCE_WITH_HISTORICAL_TERMINAL_EVENT / LOCAL_ONLY_READS_NO_CONFIGURED_REMOTE_SOURCES / CONFIGURED_REMOTE_CALLS_TN_FACADE_EXACTLY_ONCE_WITH_ZERO_LOCAL_ONLY_FALLBACK / CONFIGURED_REMOTE_RETAINS_TO_COMBINED_LOCAL_REMOTE_SUCCESS_AND_REMOTE_COMPANION_FIELD / PHASE_102_FIXTURE_EXPLICITLY_SELECTS_LOCAL_ONLY / PHASE_125_IDENTITY_FAILURE_PRECEDENCE_UNCHANGED / DORMANT_MODE_SOURCE_FIRST / TWO_PATH_ACTIVATION_AND_BINARY_FIXTURE_SECOND / NO_SERVICE_OR_DEPLOYMENT_ACTIVATION`

## 2. Controlling authority

Authoritative predecessor remains evidence-closed C03e-TO:

- PR `#654`
- head `93421f06254ce5406ce08f3e739003ac5e95af57`
- tree `eb145f1b7af029710474d9f9c59cfd6972e0c77b`
- status `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`
- immutable TO audit Drive ID `1ymYeGStMjWgxflgIEHlhgkIkVun760jQ`
- TO audit is exactly one Drive revision with `previousRevisionId = null`

C03e-TP blocked audit is corrective evidence, not a source predecessor:

- Drive ID `1_6HudT_oKCT-hAYoMfLbh7TeOXF2C7Hs`
- bytes `7358`
- SHA-256 `008f36d7fef382ad282c4d28d360103a27fb36c84bab9502b00b6cac6d813fa6`
- exactly one Drive revision with `previousRevisionId = null`
- no TP source commit, remote branch or PR exists

Integrated `main` remains outside this branch lineage at:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`

## 3. TP blocker incorporated as selection evidence

The restored TP candidate changed only `main.rs` and matched the TO callsite law, but full workspace tests failed in:

`phase_102_binary_bootstrap::standalone_binary_bootstrap_contract_is_proven_sequentially`

Observed failure:

`Agent exited before listener readiness: status=exit status: 1, connect_error=No such file or directory (os error 2)`

The main library unit suite passed `666/666`; the Phase 125 identity-preflight integration test also passed. The blocker is therefore the new executable configured-production prerequisite, not a general compile/type failure.

## 4. Existing Phase 102 fixture contract

`crates/prw-agent/tests/phase_102_binary_bootstrap.rs` spawns the real `prw-agent` binary with only:

- `XDG_RUNTIME_DIR`
- `CREDENTIALS_DIRECTORY`

Its success paths wait for the existing local Unix listener, exercise local status, deliver SIGTERM/SIGINT and verify bounded cleanup/terminal behavior.

It does not currently own configured-production remote inputs.

## 5. Existing configured-production prerequisites

The exact TO/TN source tree contains fixed configured-production inputs including:

- `PRW_REMOTE_BIND_ADDR`
- `PRW_REMOTE_PEER_DEVICE_ID`
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`
- `PRW_REMOTE_APPLICATION_LEASE_SECONDS`
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`
- `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`

TQ does not select fabricated values for these sources in the Phase 102 fixture.

In particular, the test must not invent a production bind address, endpoint authority, peer identity, registry state, scheduling authority or network reachability merely to keep a local lifecycle regression test green.

## 6. Selected executable mode source

Selected fixed non-secret process environment source:

`PRW_AGENT_EXECUTION_MODE`

The source is process configuration, not identity, authorization, request correlation or transport identity.

It is read exactly once per Linux executable invocation after successful device-identity preflight and after the existing public-SPKI event, but before either local-only or configured-remote bootstrap is entered.

## 7. Selected exact values

Exactly two values are valid:

- `local_only`
- `configured_remote`

Parsing law:

- exact ASCII bytes only;
- no trimming;
- no case folding;
- no aliases;
- no prefix/suffix matching;
- no empty value;
- no default;
- no inference from presence/absence of any `PRW_REMOTE_*` variable;
- no fallback from configured-remote failure to local-only.

Missing, non-Unicode or any other value is an explicit process-configuration failure.

## 8. Selected public mode type

Selected public enum in `linux_bootstrap.rs`:

`LinuxAgentExecutionMode`

Variants:

- `LocalOnly`
- `ConfiguredRemote`

The enum grants only executable composition selection. It grants no network, scheduling, capability, identity, endpoint or retry authority by itself.

## 9. Selected mode source error

Selected public bounded error:

`LinuxAgentExecutionModeSourceError`

Variants:

- `Missing`
- `NonUnicode`
- `InvalidValue`

Display/Debug must not include the raw environment value.

`std::error::Error::source()` returns `None` for all variants.

## 10. Selected loader

Selected public loader:

`load_linux_agent_execution_mode_from_env()`

It performs exactly one `std::env::var_os("PRW_AGENT_EXECUTION_MODE")` read and delegates to one pure parser.

Selected private parser semantic shape:

`parse_linux_agent_execution_mode_value(Option<OsString>) -> Result<LinuxAgentExecutionMode, LinuxAgentExecutionModeSourceError>`

The parser is directly unit-testable without mutating process-global environment state.

## 11. Bounded executable diagnostic for mode-source failure

After successful identity preflight/public-SPKI event, mode-source failure maps to exactly:

`prw-agent event=startup_failure kind=execution_mode exit=failure signal_mask_restore=not_applicable`

Then the process returns `ExitCode::FAILURE`.

No raw value, parser class, environment name duplication, Debug payload or lower source text is emitted.

The existing identity failure remains higher precedence because mode acquisition occurs only after identity succeeds.

## 12. Selected local-only executable lane

When mode is `LocalOnly`, the executable calls exactly once:

`prw_agent::linux_bootstrap::run()`

Local-only mode must:

- perform zero TN configured-production facade calls;
- perform zero `PRW_REMOTE_*` source reads through the executable composition;
- construct no higher configured-remote Tokio driver;
- preserve the historical local terminal event shape exactly, without adding `remote_companion`;
- preserve current local counters, cleanup and signal-mask evidence;
- preserve current success/exit law `report.is_success()`;
- preserve existing startup-failure projection from `LinuxAgentBootstrapStartFailure`;
- perform no retry or configured-remote fallback.

`local_only` is an explicit selected operating mode, not a fallback.

## 13. Selected configured-remote executable lane

When mode is `ConfiguredRemote`, the executable calls exactly once:

`prw_agent::linux_bootstrap::run_with_configured_production_remote_companion()`

Configured-remote mode must:

- perform zero legacy local-only `run()` calls;
- perform no retry;
- perform no fallback to `LocalOnly` on configuration/runtime/bootstrap failure;
- preserve TO identity ordering;
- preserve TN bounded configured-production startup-failure projection;
- preserve TO combined success law `local.is_success() && remote.is_success()`;
- preserve all existing local terminal fields/counters/cleanup/signal-mask evidence;
- append exactly one final `remote_companion=<remote.token()>` field;
- return failure for every remote finalization state except TN `shutdown_requested_joined`.

## 14. Mode selection is not runtime fallback

The executable branches once on an explicit validated process configuration value before entering either lane.

The selected lane is then terminal for that invocation.

No failure inside `ConfiguredRemote` may cause a second call to `run()`, a second mode read, a second facade call, or any local-only continuation.

## 15. Identity precedence

Linux executable ordering is selected as:

1. load device identity credential;
2. on identity failure emit existing exact `device_identity` startup-failure event and exit failure;
3. emit existing public-SPKI SHA-256 event;
4. acquire/validate `PRW_AGENT_EXECUTION_MODE` exactly once;
5. on mode failure emit bounded `execution_mode` startup-failure event and exit failure;
6. enter exactly one selected lane.

This preserves the Phase 125 fail-before-runtime identity contract.

## 16. Non-Linux behavior

The current non-Linux `unsupported_platform` startup-failure path remains unchanged.

Non-Linux execution does not read `PRW_AGENT_EXECUTION_MODE`.

## 17. Phase 102 fixture migration

The successful Phase 102 real-binary fixture explicitly sets:

`PRW_AGENT_EXECUTION_MODE=local_only`

for each helper that expects historical local listener readiness.

This is not a production default. It is explicit test ownership of the lane that the test actually proves.

The fixture must not set fabricated `PRW_REMOTE_*` values.

All historical Phase 102 SIGTERM/SIGINT/status/cleanup assertions remain valid in `local_only` mode.

## 18. Additional Phase 102 mode-failure proof

Within the same existing test file, the future activation checkpoint should add a bounded proof that a process with:

- valid device identity;
- valid `XDG_RUNTIME_DIR`;
- missing `PRW_AGENT_EXECUTION_MODE`

fails with exact `kind=execution_mode`, `signal_mask_restore=not_applicable`, and creates no local Agent socket.

A malformed-value case may be added in the same file if needed to pin the strict parser-to-executable projection, but no additional integration-test path is selected.

## 19. Phase 125 fixture remains unchanged

`crates/prw-agent/tests/phase125_device_identity_bootstrap.rs` remains unchanged.

Because identity acquisition precedes mode acquisition, its existing no-credential case still emits only:

`kind=device_identity`

although it does not set `PRW_AGENT_EXECUTION_MODE`.

This is an explicit precedence invariant.

## 20. First corrective source successor — dormant mode source

The immediate source successor after TQ, if separately authorized, is limited to exactly one Rust path:

`crates/prw-agent/src/linux_bootstrap.rs`

It may materialize only:

- `PRW_AGENT_EXECUTION_MODE_ENV` constant;
- `LinuxAgentExecutionMode`;
- `LinuxAgentExecutionModeSourceError`;
- pure strict parser;
- public one-read loader;
- bounded unit tests for exact valid/missing/non-Unicode/invalid values and raw-value-free diagnostics.

It must not modify `main.rs`, integration tests, service files, higher-owner source, Cargo/lock/workflows or runtime behavior.

This first corrective source successor remains dormant and performs no executable activation.

## 21. Second corrective source successor — executable activation plus fixture

Only after the dormant mode source is evidence-closed may a later separately authorized activation checkpoint modify exactly two paths:

1. `crates/prw-agent/src/main.rs`
2. `crates/prw-agent/tests/phase_102_binary_bootstrap.rs`

It may materialize only the mode read/branch, local-only existing path, configured-remote TO path, bounded mode failure diagnostic, explicit Phase 102 `local_only` selection and bounded missing-mode integration proof.

If compilation or tests require a third path, that checkpoint must STOP rather than widen scope.

## 22. Existing packaging is not silently changed

Current `packaging/systemd/prw-agent.service` supplies no `PRW_AGENT_EXECUTION_MODE` and no configured-production `PRW_REMOTE_*` values.

TQ does not modify that unit.

Therefore no real service deployment/activation is authorized after source activation alone.

Before any real-host configured-remote deployment, a separate selection must decide how the user service receives the explicit execution mode and required configured-production inputs. No hidden environment synthesis is selected here.

## 23. Service restart behavior is not used as fallback

The existing unit has `Restart=on-failure`. TQ does not use that behavior as configuration retry or mode fallback.

A missing/invalid mode is a deterministic process-configuration failure. Real deployment remains separately gated so a known-invalid unit is not intentionally activated into a restart loop.

## 24. TO law retained versus superseded

TQ retains TO for the `ConfiguredRemote` lane:

- identity/SPKI ordering;
- one TN facade call;
- no retry/fallback;
- local+remote combined success;
- final `remote_companion` field;
- bounded startup-failure tokens.

TQ supersedes only TO's assumption that the Linux executable has a single unconditional configured-remote lane.

The explicit mode authority is the corrective fact introduced by TP regression evidence.

## 25. Security invariants preserved

Execution mode is not device identity, peer identity, endpoint identity, request identity, scheduling authority or capability authority.

`ConfiguredRemote` does not grant authorization by itself; all existing registry/policy/authentication/capability checks remain controlling.

`LocalOnly` does not fabricate remote success and exposes no remote listener/endpoint custody.

No static IP is promoted to identity.

## 26. Validation expectations for dormant mode-source materialization

The first source successor must prove:

- exact env constant;
- exact two valid values;
- missing/non-Unicode/empty/whitespace/case-variant/unknown values rejected;
- no trim/default/fallback;
- exactly one env read in the public loader;
- raw value absent from Display/Debug;
- one changed Rust path;
- full local fmt/metadata/Clippy/tests/build;
- exact-final-head canonical CI.

## 27. Validation expectations for later activation materialization

The later two-path activation checkpoint must prove:

- identity failure still wins without mode;
- missing mode with valid identity fails as `execution_mode` before socket creation;
- Phase 102 success fixtures explicitly select `local_only` and remain green;
- local-only invokes exactly one legacy `run()` and zero configured facade calls;
- configured-remote invokes exactly one TN facade and zero legacy `run()` calls;
- no configured-remote failure fallback;
- configured terminal field/order and combined exit law match TO;
- exactly two changed paths;
- full local and exact-final-head canonical CI.

## 28. Explicit non-actions

TQ performs no Rust/source mutation, environment mutation, real service configuration, service start/restart, listener/network activation, configured remote startup, deployment, merge, ready conversion, PR close, branch deletion, history rewrite, force push, dependency/workflow change, repository configuration mutation or destructive evidence cleanup.

## 29. Current STOP

TQ selects the corrective boundary only.

STOP before dormant execution-mode source materialization.
STOP again before the later two-path executable activation checkpoint.
STOP before any systemd/service/deployment configuration or runtime activation.
