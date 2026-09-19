# C03e-VX — Signal-Aware AgentStatus Production-Caller Selection Seam Selection

Status: `SELECTION — SOURCE MATERIALIZATION NOT AUTHORIZED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`SIGNAL_AWARE_AGENT_STATUS_PRODUCTION_CALLER_SELECTION_SEAM_SELECTION`

## Exact predecessor authority

Evidence-closed C03e-VW is the exact predecessor:

- branch:
  `phase-152-c03e-vw-agent-status-only-runtime-worker-selection-source-materialization`
- exact final head:
  `c6e6d5e873b6b81c6a26637439ff3e815b5a7942`
- exact final tree:
  `3464cb30abbd7039471ae0171a827a84c515b834`
- PR #713 remains draft/open/unmerged
- canonical VW evidence ID:
  `1OQC8_K6KsVcu5r7A5CEm_-MxApqxIMie`

VW materialized the narrow AgentStatus-only runtime worker-selection seam while keeping
the existing runtime-context constructor as the legacy default.

VW explicitly stopped before production caller selection or Linux bootstrap wiring.

## Fresh exact-source audit

The exact VW head was read directly before this selection.

Relevant exact blobs:

- `crates/prw-agent/src/linux_runtime_orchestration.rs`
  `bb0af75417306722c1967c7b0715bbe4dc871529`
- `crates/prw-agent/src/linux_signal_aware_runtime.rs`
  `0bf61b8bdd7cec4950fedc8678b1d1c087212e45`
- `crates/prw-agent/src/linux_production_runtime_loop.rs`
  `d4e5791908f45f53b35e892c9218e4241b93052f`
- `crates/prw-agent/src/linux_production_lifecycle.rs`
  `c8a81b7c57dcfc0c9c28d7c37d43f6f3f5a4c5a4`
- `crates/prw-agent/src/linux_bootstrap.rs`
  `66b3a17d703a9c58f38b9ef66e7397be81b1837c`
- `crates/prw-agent/src/main.rs`
  `85ef70bb776d74cba2ba87d9f75e8f7eb08e2fb7`

## Current production-local call chain

The installed Linux bootstrap source currently constructs fixed
`LocalLinuxProductionRuntimeInputs` and delegates to the signal-aware runtime.

Local-only lane:

`main.rs`
-> `linux_bootstrap::run()`
-> `run_signal_aware_linux_production_runtime_from_env(...)`
-> `run_signal_aware_linux_production_runtime_from_env_with_companion(...)`
-> `run_signal_aware_linux_production_runtime_loop(...)`
-> `LocalLinuxRuntimeSchedulerContext::new(...)`
-> legacy worker selection.

Configured-remote lane preserves the same local signal-aware runtime through
`run_signal_aware_linux_production_runtime_from_env_with_companion(...)` while owning
the separately composed remote process companion.

The signal-aware runtime therefore is the narrow shared production-local caller immediately
above the VW scheduler-selection seam and below Linux bootstrap policy/activation.

## Why linux_production_runtime_loop.rs is not selected

`linux_production_runtime_loop.rs` also constructs
`LocalLinuxRuntimeSchedulerContext::new(...)`, but the current Linux bootstrap source
delegates to the signal-aware runtime, not that older non-signal-aware production-loop entry.

Changing both callers would widen the immediate source surface without establishing any
additional authority needed by the active bootstrap call chain.

The non-signal-aware production loop therefore remains legacy and unchanged.

## Closed future source ceiling

The next source-materialization checkpoint is selected to exactly one Rust path:

`crates/prw-agent/src/linux_signal_aware_runtime.rs`

No second Rust path is selected.

## Selected source shape

The future materialization SHALL remain additive and dormant from production bootstrap.

### Preserve existing public behavior

The existing public entry points SHALL preserve legacy worker selection:

- `run_signal_aware_linux_production_runtime_loop(...)`
- `run_signal_aware_linux_production_runtime_from_env(...)`
- `run_signal_aware_linux_production_runtime_from_env_with_companion(...)`

No existing bootstrap caller changes in that checkpoint.

### Private bounded selection helper

The signal-aware runtime may add one private two-state local-worker selection value with
exactly:

- legacy read-only;
- AgentStatus-only management.

This local production-caller selection is only a projection onto the already-materialized
VW scheduler constructors. It carries no management authority.

A private signal-aware runtime-loop helper may select:

- `LocalLinuxRuntimeSchedulerContext::new(...)` for legacy;
- `LocalLinuxRuntimeSchedulerContext::new_with_agent_status_management(...)` for
  AgentStatus-only.

The selection must occur exactly where the signal-aware runtime currently constructs the
scheduler context, before entering the unchanged scoped registry/readiness/scheduling loop.

### Explicit dormant AgentStatus siblings

The one-path future materialization may add crate-internal AgentStatus-only siblings for
the signal-aware runtime composition needed by a later bootstrap checkpoint.

The selected sibling surface may cover both existing lifecycle shapes:

1. no-companion signal-aware runtime;
2. companion-owned signal-aware runtime.

The existing public legacy functions remain the default and continue delegating to legacy
selection.

The AgentStatus-only siblings must not read environment configuration to decide worker
selection and must not change process execution mode.

## No new authority propagation

The future one-path source materialization must reuse the exact existing
`LocalLinuxProductionRuntimeInputs` values:

- bounded legacy read policy for commands 1/2;
- status snapshot;
- private-DNS snapshot;
- runtime config;
- worker capacity and wake supplied by the existing lifecycle.

It must add no:

- caller-supplied management policy;
- provider lifecycle;
- filesystem authority;
- terminal backend;
- forwarding backend;
- transfer provider;
- credential/auth authority;
- network policy;
- new environment variable;
- new production configuration field.

## Lifecycle semantics that must remain unchanged

The future signal-aware caller seam must preserve:

- one existing SIGTERM/SIGINT signal source;
- current signal-mask ownership and exact restoration;
- existing lifecycle assembly and cleanup;
- current listener/socket custody;
- same capacity owner;
- same scheduler control/wake;
- same worker registry;
- same readiness precedence;
- same scheduling attempt budget;
- same fail-stop classification;
- same cancellation and final join;
- companion startup/finalization ordering;
- cleanup before exact signal-mask restoration.

Worker selection must not become signal ownership or lifecycle authority.

## Explicitly excluded from the next source checkpoint

The next source materialization SHALL NOT change:

- `crates/prw-agent/src/linux_runtime_orchestration.rs`;
- `crates/prw-agent/src/linux_production_runtime_loop.rs`;
- `crates/prw-agent/src/linux_production_lifecycle.rs`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/main.rs`;
- VU/VW worker or spawn modules;
- listener/accept/authentication modules;
- systemd unit or drop-ins;
- Agent execution-mode parsing;
- configured-remote production inputs;
- remote endpoint/process companion authority;
- management providers/backends;
- Cargo manifests or `Cargo.lock`;
- workflows;
- Android source;
- packaging source;
- desktop command-3 dispatch.

## Production activation boundary

After the selected one-path source materialization, the existing Linux bootstrap source
must still call the legacy signal-aware entry points.

Therefore installed production behavior remains legacy until a separately gated
`linux_bootstrap.rs` checkpoint explicitly chooses the dormant AgentStatus signal-aware
entry point.

That later bootstrap gate must remain distinct from:

- `main.rs` mutation;
- installed Agent replacement;
- systemd restart/reload;
- production command-3 request;
- desktop command-3 dispatch.

## Selected validation obligations

The future one-path source materialization must prove on its exact final head:

- existing public signal-aware runtime entry points still select legacy worker behavior;
- explicit AgentStatus-only signal-aware sibling selects the VW AgentStatus constructor;
- command-3 AgentStatus succeeds through the signal-aware AgentStatus-selected path in a
  test root;
- legacy command 1 still succeeds through that AgentStatus-selected signal-aware path;
- a non-AgentStatus represented management request remains denied by the fixed VU policy;
- signal-aware programmatic shutdown semantics remain unchanged;
- signal termination and mask-restoration tests remain unchanged/passing;
- companion start/finalize ordering remains unchanged;
- cleanup still precedes signal-mask restoration;
- no bootstrap production caller references the AgentStatus-only sibling.

Repository-required exact-head checks remain authoritative.
`SKIPPED` is not PASS.

## Security law

Production caller selection is not management authority.

VU/VW law remains:

- `AgentStatusRead = Allow`;
- every other represented management capability = `Deny`;
- commands 1/2 retain the bounded legacy read evaluator;
- same-UID authenticated admission remains below the selected caller seam.

## Selection classification

`SIGNAL_AWARE_AGENT_STATUS_PRODUCTION_CALLER_SEAM_SELECTED / DOCUMENTATION_ONLY / ONE_PATH_FUTURE_SOURCE_CEILING / SIGNAL_AWARE_RUNTIME_IS_ACTIVE_BOOTSTRAP_LOCAL_CALLER / NON_SIGNAL_AWARE_PRODUCTION_LOOP_REMAINS_LEGACY / EXISTING_PUBLIC_SIGNAL_AWARE_ENTRYPOINTS_REMAIN_LEGACY / PRIVATE_AGENT_STATUS_SIGNAL_AWARE_SIBLINGS_SELECTED / NO_NEW_ENV_OR_CONFIG_SELECTION / EXISTING_RUNTIME_INPUTS_REUSED / SIGNAL_MASK_OWNERSHIP_PRESERVED / LIFECYCLE_CLEANUP_PRESERVED / COMPANION_ORDERING_PRESERVED / FIXED_INTERNAL_MANAGEMENT_POLICY_PRESERVED / NO_GENERIC_MANAGEMENT_PROVIDER / NO_FILESYSTEM_AUTHORITY / NO_TERMINAL_BACKEND / NO_FORWARDING_BACKEND / NO_BOOTSTRAP_MUTATION / NO_MAIN_MUTATION / NO_INSTALLED_AGENT_MUTATION / NO_DESKTOP_COMMAND3_DISPATCH / NO_RUNTIME_ACTIVATION / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_SIGNAL_AWARE_AGENT_STATUS_PRODUCTION_CALLER_SELECTION_SEAM_SELECTION_AND_BEFORE_ONE_PATH_SIGNAL_AWARE_SOURCE_MATERIALIZATION`

`NO_RACE_FREE_CLAIM`
