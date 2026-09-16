# C03e-TM Public Executable Facade / Main Ownership / Exit Diagnostic Selection

**Status:** selection staging only — no source materialization or runtime activation

## 1. Boundary

C03e-TM selects the narrow public executable-facing facade above the evidence-closed C03e-TL synchronous driver, the exact future `main.rs` ownership/callsite replacement, and the bounded process exit/diagnostic policy.

Canonical selected law:

`NARROW_PUBLIC_LINUX_BOOTSTRAP_FACADE / TL_HIGHER_OWNER_DRIVER_REMAINS_CRATE_PRIVATE / PUBLIC_REMOTE_START_FAILURE_PROJECTION / REMOTE_RUNTIME_REMOTE_APPLICATION_LEASE_REMOTE_CONFIGURATION_OR_EXISTING_BOOTSTRAP_KIND / PRE_BOOTSTRAP_REMOTE_FAILURE_SIGNAL_MASK_NOT_APPLICABLE / EXISTING_BOOTSTRAP_KIND_AND_MASK_RESTORE_PRESERVED / REMOTE_FINALIZATION_SUCCESS_ONLY_SHUTDOWN_REQUESTED_AND_JOINED / PROCESS_EXIT_SUCCESS_REQUIRES_LOCAL_SUCCESS_AND_REMOTE_SUCCESS / BOUNDED_REMOTE_COMPANION_TOKEN_APPENDED_TO_TERMINAL_EVENT / DEVICE_IDENTITY_PREFLIGHT_AND_FINGERPRINT_EVENT_UNCHANGED / NO_RAW_ERROR_CONFIG_VALUE_OR_PRIVATE_SOURCE_LOGGING / FIRST_FACADE_MATERIALIZATION_ONE_PATH / MAIN_RS_ACTIVATION_SEPARATELY_GATED / NO_SOURCE_MUTATION / NO_RUNTIME_ACTIVATION`

TM itself performs no Rust/source/runtime mutation.

## 2. Authoritative predecessor

C03e-TL is the direct predecessor:

- PR `#651`
- branch `phase-152-c03e-tl-concrete-c-r-disposition-async-runtime-executable-custody-source-materialization`
- head `034887d36de6c0f38daa2baf89e3ed0497d75a4c`
- tree `3ce9b522e7e81dd7ca6ab80a3c4b049b56ca9ee6`
- state recovered before TM: draft / open / unmerged / evidence-closed
- immutable TL Drive audit ID `1HiNVkYmutQzQFQ5NQ07VkDqK-rSggvA7`
- TL Drive revision count rechecked before TM: exactly `1`
- current revision `0Bz5eMiLa5v9xcHlEbUJNak1vQkxTWWNZMVVDYjB4SUdkdHJzPQ`
- previous revision `null`

Integrated `main` remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`

## 3. Exact-current source anchors

At exact TL head:

- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
  - blob `1325efac03ed1f90d8e84369a4b1374c539a2e79`
  - owns the crate-private TL selected executable-custody driver
  - TL driver returns `LinuxAgentBootstrapWithRemoteReport` or `LinuxAgentProductionConfiguredApplicationLeaseSelectedExecutableCustodyError`
- `crates/prw-agent/src/linux_bootstrap.rs`
  - blob `50a8498697b4767d19f0b1668a664e257a7af306`
  - explicitly identifies itself as the narrow public Linux Agent binary-bootstrap facade
  - already exposes `LinuxAgentBootstrapWithRemoteReport`, `LinuxAgentBootstrapStartKind`, `LinuxAgentBootstrapStartFailure`, `LinuxAgentBootstrapSignalMaskRestore`, and `run_with_remote_process_companion`
- `crates/prw-agent/src/main.rs`
  - blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1`
  - performs device-identity credential preflight, emits the public SPKI SHA-256 event, then calls only `prw_agent::linux_bootstrap::run()`
- `crates/prw-agent/src/lib.rs`
  - blob `53e6b9c33d1a3be644fb6645289f6854cc096eee`
  - already exports `linux_bootstrap` as `pub mod`
  - retains `production_durable_capability_higher_owner_custody` as `pub(crate)`

Therefore no `lib.rs` visibility widening is selected.

## 4. Existing TL private error topology

The TL driver error is exactly:

`LinuxAgentProductionConfiguredApplicationLeaseSelectedExecutableCustodyError`

with:

- `RuntimeConstruction`
- `Companion(LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredApplicationLeaseCompanionError)`

The configured application-lease companion error is exactly:

- `ApplicationLeasePolicySource(...)`
- `Companion(LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationCompanionError)`

The configured-population companion error is exactly:

- `ConfiguredPopulation(...)`
- `Bootstrap(LinuxAgentBootstrapStartFailure)`

TM does not make any of those higher-owner error types public.

## 5. Selected public facade location

The public executable-facing facade is selected to live in:

`crates/prw-agent/src/linux_bootstrap.rs`

Selected public function name:

`run_with_configured_production_remote_companion`

Selected semantic result:

```text
Result<
    LinuxAgentBootstrapWithRemoteReport,
    LinuxAgentConfiguredProductionRemoteStartFailure,
>
```

The facade must invoke the existing TL synchronous driver exactly once and perform only bounded error projection around that call.

The facade must not construct another Tokio runtime, call another `block_on`, create another expected-request channel, sample timing, reload configuration, retry, fall back, spawn a task/thread, or reconstruct lower custody.

## 6. Selected public bounded failure kind

Selected public enum:

`LinuxAgentConfiguredProductionRemoteStartKind`

Selected variants:

- `RemoteRuntime`
- `RemoteApplicationLease`
- `RemoteConfiguration`
- `Bootstrap(LinuxAgentBootstrapStartKind)`

Selected token mapping:

- `RemoteRuntime` -> `"remote_runtime"`
- `RemoteApplicationLease` -> `"remote_application_lease"`
- `RemoteConfiguration` -> `"remote_configuration"`
- `Bootstrap(kind)` -> exact existing `kind.token()`

No raw source/configuration value, private type name, transport coordinate, request/session identifier, verifier timestamp, or inner error text is exposed by the token surface.

## 7. Selected public bounded start failure

Selected public struct:

`LinuxAgentConfiguredProductionRemoteStartFailure`

It contains only:

- `LinuxAgentConfiguredProductionRemoteStartKind`
- `LinuxAgentBootstrapSignalMaskRestore`

Selected public accessors:

- `kind()`
- `signal_mask_restore()`

Its `Display` surface, if materialized, must remain generic and bounded. Its public `Error::source()` must not expose the crate-private higher-owner chain.

## 8. Exact TL-to-public failure projection

The selected mapper is exhaustive over the current TL hierarchy:

1. TL `RuntimeConstruction`
   - public kind: `RemoteRuntime`
   - signal-mask restore: `NotApplicable`

2. TL `Companion(ApplicationLeasePolicySource(_))`
   - public kind: `RemoteApplicationLease`
   - signal-mask restore: `NotApplicable`

3. TL `Companion(Companion(ConfiguredPopulation(_)))`
   - public kind: `RemoteConfiguration`
   - signal-mask restore: `NotApplicable`

4. TL `Companion(Companion(Bootstrap(failure)))`
   - public kind: `Bootstrap(failure.kind())`
   - signal-mask restore: exact `failure.signal_mask_restore()`

The first three classes occur before the existing Linux signal-aware bootstrap owns a changed signal mask, so `NotApplicable` is selected rather than fabricated rollback evidence.

Existing bootstrap failures retain their existing kind and rollback evidence exactly.

## 9. Selected remote finalization success law

`LinuxAgentRemoteProcessCompanionFinalization` remains secondary bounded evidence, but executable success must now account for it when the configured production remote lane is selected.

Selected remote success is exactly:

```text
Finalized {
    controller: ShutdownRequested,
    thread: Joined,
}
```

All other remote finalization states are executable failure:

- `SpawnFailed`
- `Finalized { controller: ShutdownRequested, thread: Panicked }`
- `Finalized { controller: UnavailableBeforeEndpointStartup, thread: Joined }`
- `Finalized { controller: UnavailableBeforeEndpointStartup, thread: Panicked }`

No remote failure is converted to local success merely because the local bootstrap report is normal.

## 10. Selected bounded remote finalization token

The existing public `LinuxAgentRemoteProcessCompanionFinalization` is selected to gain bounded pure methods:

- `token()`
- `is_success()`

Selected tokens:

- `SpawnFailed` -> `"spawn_failed"`
- `Finalized { ShutdownRequested, Joined }` -> `"shutdown_requested_joined"`
- `Finalized { ShutdownRequested, Panicked }` -> `"shutdown_requested_panicked"`
- `Finalized { UnavailableBeforeEndpointStartup, Joined }` -> `"unavailable_before_endpoint_startup_joined"`
- `Finalized { UnavailableBeforeEndpointStartup, Panicked }` -> `"unavailable_before_endpoint_startup_panicked"`

`is_success()` is true only for `shutdown_requested_joined`.

No panic payload, thread identity, endpoint, request, peer, configuration or lower error is exposed.

## 11. Selected `main.rs` ownership/callsite replacement

A later separately authorized activation checkpoint must preserve the existing Linux executable order:

1. load the Ubuntu enrollment signer from the systemd credential;
2. on identity-source failure, emit the existing exact `device_identity` startup-failure event and return `ExitCode::FAILURE`;
3. emit the existing `device_identity_loaded public_spki_sha256=...` event unchanged;
4. call `prw_agent::linux_bootstrap::run_with_configured_production_remote_companion()` exactly once;
5. do not call legacy `linux_bootstrap::run()` on that path;
6. do not invoke both facades;
7. do not retry or fall back to local-only operation if the configured remote facade fails.

The process main thread remains the owner of the synchronous facade call.

## 12. Selected successful terminal handling

On facade success:

- extract `local = report.local()` exactly once;
- extract `remote = report.remote()` exactly once;
- use existing local counters and terminal/cleanup/signal-mask tokens unchanged;
- compute executable success as:

```text
local.is_success() && remote.is_success()
```

The existing terminal stderr event is preserved and extended only by one bounded field appended at the end:

`remote_companion=<remote.token()>`

No existing field is renamed or reordered before the appended field.

Exit policy:

- combined success -> `ExitCode::SUCCESS`
- any local or remote non-success -> `ExitCode::FAILURE`

## 13. Selected startup-failure handling

On public configured-production-remote facade failure, `main.rs` retains the existing startup-failure event shape:

```text
prw-agent event=startup_failure kind={} exit=failure signal_mask_restore={}
```

The values are only:

- `failure.kind().token()`
- `failure.signal_mask_restore().token()`

No `Display`, `Debug`, `source()`, raw inner error, environment value, endpoint, identity, request/session ID, or verifier time is logged.

All facade failures return `ExitCode::FAILURE`.

## 14. Existing non-Linux and identity behavior remains unchanged

TM selects no change to:

- non-Linux `unsupported_platform` failure event;
- identity credential acquisition;
- identity public-SPKI fingerprint encoding;
- the `device_identity_loaded` stdout event;
- device identity as distinct from transport/endpoint/request identity.

Remote configuration remains configuration, not identity or authorization.

## 15. No `lib.rs` or higher-owner visibility widening

No `lib.rs` change is selected because `linux_bootstrap` is already public.

The TL driver and its private higher-owner error chain remain in:

`production_durable_capability_higher_owner_custody.rs`

with crate-private visibility.

TM explicitly rejects making the whole higher-owner module public or exposing private population/application-lease error types through the public binary API.

## 16. Immediate dormant source successor

If separately authorized, the first TM successor is limited to exactly one Rust path:

`crates/prw-agent/src/linux_bootstrap.rs`

It may materialize only:

- the public configured-production-remote start kind;
- the public bounded start failure;
- the private exhaustive TL-error mapper;
- `LinuxAgentRemoteProcessCompanionFinalization::token()`;
- `LinuxAgentRemoteProcessCompanionFinalization::is_success()`;
- `run_with_configured_production_remote_companion()`;
- bounded pure/type tests.

It must not alter `main.rs` or invoke the new facade from executable code.

This first successor therefore remains dormant and is not runtime activation.

## 17. Separately gated executable activation successor

Only after the dormant facade materialization is evidence-closed may a separate activation checkpoint modify exactly:

`crates/prw-agent/src/main.rs`

That checkpoint may only:

- replace the existing `linux_bootstrap::run()` call with the selected public configured-production-remote facade;
- project `LinuxAgentBootstrapWithRemoteReport` to existing local terminal fields plus the selected appended `remote_companion` token;
- apply the selected combined local+remote exit policy;
- apply the selected bounded startup-failure token projection.

That `main.rs` change is executable/runtime activation and requires explicit authorization at that later gate.

## 18. Validation expectations for dormant facade materialization

The first source successor should prove:

- exact public facade signature;
- exhaustive mapping of all current TL error variants;
- pre-bootstrap remote failures map to `NotApplicable` signal-mask restore;
- nested existing bootstrap failure preserves exact kind and signal-mask restore;
- public error source does not expose private inner errors;
- exact five remote finalization tokens;
- `is_success()` true only for shutdown-requested + joined;
- exactly one call to the TL synchronous driver;
- zero additional Tokio Builder / `block_on` / spawn / thread creation;
- zero `main.rs`, `lib.rs`, higher-owner, Cargo/lock/workflow/service/deployment mutation.

## 19. Validation expectations for later `main.rs` activation

The later activation checkpoint should prove:

- identity preflight event and public-SPKI event remain byte-for-byte unchanged;
- exactly one configured-production-remote facade invocation;
- zero legacy `linux_bootstrap::run()` invocation in Linux main;
- combined exit success requires both local and remote success;
- terminal event preserves all existing fields and appends exactly one `remote_companion` field;
- startup failure logs only bounded kind and signal-mask tokens;
- non-Linux behavior is unchanged;
- no fallback to local-only bootstrap;
- exact-final-head Rust and Android CI succeed before evidence closure.

## 20. Explicit non-actions

TM performs no Rust/source materialization, no public facade implementation, no `main.rs` mutation, no executable invocation change, no runtime/listener/readiness/network activation, no service-manager change, no deploy/restart, no dependency/Cargo/lock/workflow change, no repository configuration change, no merge, no ready conversion, no PR close, no branch deletion, no reset/rebase/squash/history rewrite/force push, and no destructive evidence cleanup.

## 21. STOP

C03e-TM is selection only. The first follow-on source checkpoint must stop after dormant one-path `linux_bootstrap.rs` facade materialization and evidence closure. `main.rs` activation remains a separate explicit authorization boundary.
