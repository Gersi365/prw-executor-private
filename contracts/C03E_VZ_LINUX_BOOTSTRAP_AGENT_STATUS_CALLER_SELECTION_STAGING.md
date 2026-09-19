# C03e-VZ — Linux Bootstrap AgentStatus Caller Selection

Status: `SELECTION — SOURCE MATERIALIZATION NOT AUTHORIZED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`LINUX_BOOTSTRAP_AGENT_STATUS_CALLER_SELECTION`

## Exact predecessor authority

Evidence-closed C03e-VY is the exact predecessor:

- branch:
  `phase-152-c03e-vy-signal-aware-agent-status-production-caller-source-materialization`
- exact final head:
  `9df82e78d30612832f58229820e5c7589720983e`
- exact final tree:
  `042cc66647ab6e71cbbfdfd7af1e91bd099f9fac`
- PR #715 remains draft/open/unmerged
- canonical VY evidence ID:
  `1h51Uy88s7vCwAGSb1GSlZ2x8sl5Q8DSA`

VY materialized the dormant AgentStatus-only signal-aware no-companion and companion
entry points while preserving all existing public signal-aware entry points as legacy.

VY explicitly stopped before Linux bootstrap caller selection.

## Fresh exact-source audit

The exact VY final head was read directly before this selection.

Relevant exact blobs:

- `crates/prw-agent/src/linux_bootstrap.rs`
  `66b3a17d703a9c58f38b9ef66e7397be81b1837c`
- `crates/prw-agent/src/main.rs`
  `85ef70bb776d74cba2ba87d9f75e8f7eb08e2fb7`
- `crates/prw-agent/src/linux_signal_aware_runtime.rs`
  `f1350a7bfa01931cbc42a7c049139b057b31a645`
- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
  `1325efac03ed1f90d8e84369a4b1374c539a2e79`
- `crates/prw-agent/tests/phase_102_binary_bootstrap.rs`
  `d8f7551fad83f7da917da26b218280eeea3b59ae`

## Current executable local-only lane

Current `main.rs` local-only routing is:

`main.rs`
-> execution mode `LocalOnly`
-> `linux_bootstrap::run()`
-> `with_initial_runtime_inputs(...)`
-> `run_signal_aware_linux_production_runtime_from_env(...)`
-> VY `LegacyReadOnly`.

The existing bootstrap `run()` function is therefore the exact production source caller for the
local-only lane.

## Current executable configured-remote lane

Current `main.rs` configured-remote routing begins:

`main.rs`
-> execution mode `ConfiguredRemote`
-> `linux_bootstrap::run_with_configured_production_remote_companion()`
-> existing configured-production higher-owner executable custody
-> configured application-lease companion
-> `crate::linux_bootstrap::run_with_production_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion_with_pre_aj_timing_and_application_lease_policy(...)`
-> `with_initial_runtime_inputs(...)`
-> `run_with_remote_process_companion_inputs(...)`
-> `run_signal_aware_linux_production_runtime_from_env_with_companion(...)`
-> VY `LegacyReadOnly`.

The configured remote transport/process authority remains separate from the local signal-aware
worker selection. The selected change concerns only the local IPC worker used while the remote
process companion is owned.

## Why the shared companion helper must not be globally replaced

`run_with_remote_process_companion_inputs(...)` is reused by the public generic injected-remote
companion facade and multiple dormant historical/bootstrap compositions.

Replacing that shared helper globally would silently widen AgentStatus selection into caller
surfaces that are not required by the current executable configured-production route.

VZ therefore rejects global replacement of the shared legacy helper.

The existing generic:

- `run_with_remote_process_companion(...)`;
- `run_with_remote_process_companion_inputs(...)`;

must remain legacy in the immediate source successor.

## Historical change-control precedent

Phase 152 already used a strict facade/materialization/activation split:

- C03e-TM selected a one-path `linux_bootstrap.rs` public facade while keeping `main.rs`
  activation separately gated;
- C03e-TN materialized only that bootstrap facade;
- C03e-TO separately selected the `main.rs` activation;
- later execution-mode work likewise separated the dormant bootstrap source from executable
  lane activation.

VZ preserves the same narrow-authority principle: mutate only the caller layer that owns the
selected decision, and do not expand unrelated call surfaces for convenience.

## Closed future source ceiling

The immediate source-materialization successor is selected to exactly one Rust path:

`crates/prw-agent/src/linux_bootstrap.rs`

No second Rust path is selected.

In particular, the successor SHALL NOT modify:

- `main.rs`;
- `linux_signal_aware_runtime.rs`;
- `linux_runtime_orchestration.rs`;
- `linux_production_runtime_loop.rs`;
- `linux_production_lifecycle.rs`;
- VU/VW worker/spawn source;
- configured-production higher-owner custody source;
- Phase 102 integration test source;
- Cargo manifests or `Cargo.lock`;
- workflows;
- Android source;
- packaging source;
- systemd source/configuration;
- desktop command-3 dispatch.

## Selected local-only source shape

The future one-path materialization must import and use the existing VY function:

`run_signal_aware_linux_production_runtime_from_env_with_agent_status_management(...)`

The existing public:

`linux_bootstrap::run()`

must preserve its current signature, runtime-input creation, report mapping and startup-error
mapping, but replace exactly the legacy signal-aware call with exactly one AgentStatus-only
signal-aware call.

No retry, dual invocation, runtime option or legacy fallback is selected.

## Selected configured-remote source shape

The future one-path materialization must import and use the existing VY function:

`run_signal_aware_linux_production_runtime_from_env_with_agent_status_management_and_companion(...)`

It may add one private dedicated helper in `linux_bootstrap.rs`, conceptually:

`run_with_agent_status_management_remote_process_companion_inputs(...)`

The helper must preserve the existing companion contract exactly:

- same `LocalLinuxProductionRuntimeInputs`;
- same caller-supplied remote operation;
- same `RemoteSessionProcessLifecycleOwner::spawn(...)`;
- same supervisor shutdown publisher;
- same companion finalizer;
- same `finalize_remote_process_companion(...)`;
- same local report mapping;
- same remote finalization evidence.

The only selected semantic difference is that this dedicated helper invokes the VY AgentStatus
companion signal-aware entry point instead of the legacy companion entry point.

Exactly the current configured-production active bootstrap composition:

`run_with_production_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion_with_pre_aj_timing_and_application_lease_policy(...)`

may switch from the shared legacy helper to the dedicated AgentStatus helper.

No other dormant/generic bootstrap companion composition is selected for change.

## Existing runtime inputs remain authoritative

`with_initial_runtime_inputs(...)` remains unchanged.

The selected source successor must reuse exactly:

- `initial_runtime_config()`;
- `BoundedLocalReadPolicy::allow_local_reads()`;
- `LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)`;
- the existing bounded private-DNS snapshot.

No new environment variable, feature flag, process mode, configuration field or caller-provided
management policy is selected.

## Main and execution-mode law remain unchanged

`main.rs` remains byte-identical in the immediate source successor.

It continues to:

- load device identity first;
- read `PRW_AGENT_EXECUTION_MODE` exactly once;
- call `linux_bootstrap::run()` for `local_only`;
- call `run_with_configured_production_remote_companion()` for `configured_remote`;
- preserve all existing terminal/startup event shapes;
- preserve current exit-success law.

No new execution mode is introduced.

The bootstrap source successor changes the local IPC worker selected behind those existing public
facades; it does not change executable mode selection itself.

## Security ceiling

Bootstrap caller selection is not management authority.

The fixed VU/VW law remains:

- `AgentStatusRead = Allow`;
- every other represented management capability = `Deny`;
- commands 1/2 retain the bounded legacy read evaluator;
- same-UID authenticated admission remains below the selected bootstrap caller.

The source successor adds no:

- caller-supplied management policy;
- filesystem authority;
- management provider lifecycle;
- terminal backend;
- forwarding backend;
- transfer provider;
- credential/auth authority;
- remote capability authority;
- network policy.

Configured-remote authority, requester/rendezvous authority, application-lease authority and
remote transport custody remain exactly where they are.

## Validation obligations for the source successor

The exact-final-head successor must prove:

- exactly one changed Rust path: `linux_bootstrap.rs`;
- `main.rs` remains exact blob `85ef70bb776d74cba2ba87d9f75e8f7eb08e2fb7`;
- VY signal-aware source remains exact blob `f1350a7bfa01931cbc42a7c049139b057b31a645`;
- `run()` calls the VY AgentStatus no-companion entry point exactly once;
- `run()` no longer calls the legacy no-companion signal-aware entry point;
- the current configured-production active composition calls one dedicated AgentStatus companion
  helper exactly once;
- that helper calls the VY AgentStatus companion entry point exactly once;
- generic `run_with_remote_process_companion(...)` and
  `run_with_remote_process_companion_inputs(...)` remain legacy;
- no new environment/configuration source is introduced;
- Phase 102 binary bootstrap integration tests remain unchanged and passing, thereby re-proving
  local-only command-1 compatibility, SIGTERM/SIGINT cleanup, second-instance exclusion,
  execution-mode failure and runtime-root failure over the newly selected bootstrap source path;
- VY final-head tests remain the authority for command-3 AgentStatus success, correlated denial of
  non-AgentStatus management, and signal-aware lifecycle semantics;
- configured-production existing unit/integration validation remains passing;
- repository-required exact-head Rust/Android/package workflows remain authoritative;
- `SKIPPED` is not PASS.

A production command-3 request is explicitly not part of this source successor.

## Production activation boundary

The future bootstrap source materialization changes repository source behavior but does not install
or execute that source on PowerCode.

After source materialization, installed production remains unchanged until a separately authorized
build/install/restart or equivalent deployment transaction.

That later activation gate must remain distinct from:

- source materialization;
- package CI validation;
- `main.rs` mutation;
- systemd configuration mutation;
- desktop command-3 dispatch;
- a production command-3 probe.

No installed-runtime success claim may be inferred from source/CI evidence alone.

## Selection classification

`LINUX_BOOTSTRAP_AGENT_STATUS_CALLER_SELECTED / DOCUMENTATION_ONLY / EXACT_ONE_RUST_PATH_FUTURE_SOURCE_CEILING / LOCAL_ONLY_RUN_SELECTS_VY_AGENT_STATUS_ENTRY / CONFIGURED_REMOTE_ACTIVE_COMPOSITION_SELECTS_DEDICATED_AGENT_STATUS_COMPANION_HELPER / SHARED_GENERIC_REMOTE_COMPANION_HELPER_REMAINS_LEGACY / DORMANT_GENERIC_BOOTSTRAP_COMPOSITIONS_REMAIN_LEGACY / MAIN_RS_UNCHANGED / EXECUTION_MODE_SOURCE_UNCHANGED / EXISTING_RUNTIME_INPUTS_REUSED / FIXED_INTERNAL_MANAGEMENT_POLICY_PRESERVED / NO_GENERIC_MANAGEMENT_PROVIDER / NO_FILESYSTEM_AUTHORITY / NO_TERMINAL_BACKEND / NO_FORWARDING_BACKEND / NO_CONFIG_SOURCE_ADDITION / NO_INSTALLED_AGENT_MUTATION / NO_DESKTOP_COMMAND3_DISPATCH / NO_RUNTIME_ACTIVATION / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_LINUX_BOOTSTRAP_AGENT_STATUS_CALLER_SELECTION_AND_BEFORE_ONE_PATH_LINUX_BOOTSTRAP_SOURCE_MATERIALIZATION`

`NO_RACE_FREE_CLAIM`
