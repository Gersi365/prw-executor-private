# C03e-VV — AgentStatus-Only Runtime Worker-Selection Source Seam Selection

Status: `SELECTION — SOURCE MATERIALIZATION NOT AUTHORIZED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`AGENT_STATUS_ONLY_RUNTIME_WORKER_SELECTION_SOURCE_SEAM_SELECTION`

## Exact predecessor authority

Evidence-closed C03e-VU is the exact predecessor:

- branch:
  `phase-152-c03e-vu-agent-status-only-management-runtime-ownership-source-materialization`
- exact final head:
  `d23d50a94e9409cc84f9781f4d525a31e2ce9857`
- exact final tree:
  `cdc08a5eb342bc1c5fb2db77a27739ed6ff21ae5`
- PR #711 remains draft/open/unmerged
- canonical VU evidence ID:
  `1iTk-t03rv703qNqYq-yGyWhYbElKwZzF`

VU materialized a dormant crate-private finite worker that preserves legacy commands 1/2
and admits command 3 only through the fixed internal AgentStatus management policy.

VU explicitly stopped before listener/spawn, scheduler, production worker selection,
Linux bootstrap, Agent main, installed Agent, or desktop command-3 dispatch wiring.

## Selection objective

Select the smallest additive source seam that can make the existing runtime scheduler
capable of choosing the VU AgentStatus-only worker while preserving the currently active
legacy worker path as the exact default.

This checkpoint selects source shape only.

It does not materialize Rust source, change runtime behavior, activate command 3,
modify production configuration, or alter the installed Agent.

## Fresh exact-source audit

The VU branch was read directly before this selection.

Relevant exact blobs:

- `crates/prw-agent/src/linux_session_worker.rs`
  `ca5852b14649d918876bf2498cb54a2093cae715`
- `crates/prw-agent/src/linux_session_worker/management_agent_status.rs`
  `f5ff8c2752d996466f59d08d4e914edfac0e3811`
- `crates/prw-agent/src/linux_session_worker_thread.rs`
  `7e005d63d94e9f78f7021b2e5c7fc733e9fa2926`
- `crates/prw-agent/src/linux_runtime_orchestration.rs`
  `c2dd6b205941c758294d8eb8da5bb92c68de94c7`
- `crates/prw-agent/src/linux_production_runtime_loop.rs`
  `d4e5791908f45f53b35e892c9218e4241b93052f`
- `crates/prw-agent/src/linux_signal_aware_runtime.rs`
  `0bf61b8bdd7cec4950fedc8678b1d1c087212e45`
- `crates/prw-agent/src/linux_bootstrap.rs`
  `66b3a17d703a9c58f38b9ef66e7397be81b1837c`

The VU narrow worker remains in a private child module and currently returns a private,
coarse AgentStatus-management worker error.

The existing scoped-thread adapter always invokes the legacy
`run_authenticated_session_worker(...)`.

The existing runtime scheduling context owns only:

- worker capacity;
- bounded legacy read policy;
- Agent status snapshot;
- private-DNS snapshot;
- existing finite worker config;
- completion-wake notifier.

The existing production runtime input bundle already owns exactly those values and does
not own a management provider, filesystem authority, terminal backend, forwarding backend,
or caller-supplied management policy.

Therefore no new production management authority needs to be propagated merely to select
the fixed AgentStatus-only worker.

## Existing production call chain

Current production-local worker routing is:

`linux_bootstrap`
-> signal-aware production runtime
-> `LocalLinuxRuntimeSchedulerContext::new(...)`
-> bounded runtime scheduling
-> `schedule_one_authenticated_runtime_worker(...)`
-> `spawn_authenticated_session_worker_with_completion_wake(...)`
-> legacy `run_authenticated_session_worker(...)`

The VU AgentStatus-only worker is not referenced by that chain.

## Selected source shape

The future source materialization SHALL remain additive.

### 1. Shared bounded worker-result envelope

`linux_session_worker.rs` may add exactly one bounded
AgentStatus-management processing variant to the existing
`LocalLinuxSessionWorkerError` envelope.

A crate-private parent-level adapter may call the already-materialized private VU worker
and convert its private coarse processing failure into that new shared error variant.

The VU child implementation itself does not need to become directly visible to sibling
modules and does not need to change.

This preserves the existing scoped-worker registry/completion result type.

### 2. Additive scoped spawn adapter

`linux_session_worker_thread.rs` may add a sibling completion-wake spawn function that:

- consumes one already-authenticated session;
- consumes one already-acquired worker permit;
- borrows the existing bounded legacy read evaluator for commands 1/2;
- carries the existing Agent status and private-DNS snapshots;
- carries the existing worker configuration;
- installs the same completion-wake guard;
- invokes only the parent-level AgentStatus-management worker adapter;
- returns the existing `LocalLinuxScopedWorkerResult` type.

The existing legacy spawn functions remain unchanged.

### 3. Explicit runtime worker selection at the existing spawn seam

`linux_runtime_orchestration.rs` may add one bounded internal worker-selection value with
exactly two states:

- legacy read-only worker;
- AgentStatus-only management worker.

The existing `LocalLinuxRuntimeSchedulerContext::new(...)` SHALL preserve the legacy
worker selection exactly.

A separate explicit AgentStatus-only constructor may select the narrow worker without
accepting a caller-supplied management policy or provider authority.

`schedule_one_authenticated_runtime_worker(...)` may branch only at the final scoped
worker spawn step, after the existing:

- capacity acquisition;
- same-UID authenticated accept;
- cancellation clone;
- authenticated-session composition.

Every other scheduling, registry, cancellation, completion, readiness, and shutdown
semantic remains unchanged.

## Immediate future source ceiling

The next source-materialization checkpoint is selected to exactly three Rust paths:

1. `crates/prw-agent/src/linux_session_worker.rs`
2. `crates/prw-agent/src/linux_session_worker_thread.rs`
3. `crates/prw-agent/src/linux_runtime_orchestration.rs`

No fourth Rust path is selected.

In particular, the existing VU child worker path remains unchanged:

`crates/prw-agent/src/linux_session_worker/management_agent_status.rs`

## Explicitly excluded from the next source checkpoint

The next source materialization SHALL NOT change:

- `crates/prw-agent/src/linux_production_runtime_loop.rs`;
- `crates/prw-agent/src/linux_signal_aware_runtime.rs`;
- `crates/prw-agent/src/linux_production_lifecycle.rs`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/main.rs`;
- worker registry/completion/cancellation modules;
- listener/accept/authentication modules;
- management provider lifecycle;
- management Linux backends;
- filesystem authority;
- terminal backend;
- forwarding backend;
- Cargo manifests or `Cargo.lock`;
- workflows;
- Android source;
- systemd units or production configuration;
- desktop command-3 dispatch.

## Higher-owner boundary after the selected source materialization

After the three-path worker-selection seam is materialized and exact-head validated,
production callers will still select the legacy constructor.

A later separately gated checkpoint must decide whether and how the signal-aware production
runtime selects the AgentStatus-only scheduler constructor.

That later gate must remain distinct from:

- Linux bootstrap wiring;
- installed Agent replacement/restart;
- desktop command-3 socket dispatch;
- production command-3 request execution.

## Security law

The selected worker choice is not management authority.

The AgentStatus-only lane retains VU's fixed internal management policy:

- `AgentStatusRead = Allow`;
- every other represented management capability = `Deny`.

The selection value cannot carry:

- a management policy;
- filesystem authority;
- provider lifecycle;
- terminal backend;
- forwarding backend;
- transfer provider;
- privilege escalation;
- network policy.

Legacy commands 1/2 retain the existing bounded read evaluator.

Same-UID authenticated local admission remains prior to worker selection.

## Selected validation obligations

The future three-path source materialization must prove on its exact final head:

- existing legacy runtime context constructor still selects the legacy worker;
- new explicit AgentStatus-only context constructor selects only the narrow worker;
- command 3 AgentStatus succeeds through the selected narrow spawn path;
- a non-AgentStatus represented command remains denied by the fixed VU policy;
- legacy command 1 remains functional through the narrow selection;
- worker permit RAII remains intact;
- completion wake remains after worker-owned state release;
- existing registry/completion classification remains usable unchanged;
- no production caller selects the new narrow constructor.

Repository-required Rust/Android/package checks remain authoritative only for the exact
final head. `SKIPPED` is not PASS.

## Selection classification

`AGENT_STATUS_ONLY_RUNTIME_WORKER_SELECTION_SEAM_SELECTED / THREE_PATH_FUTURE_SOURCE_CEILING / EXISTING_RUNTIME_CONTEXT_CONSTRUCTOR_REMAINS_LEGACY / NEW_EXPLICIT_AGENT_STATUS_ONLY_CONTEXT_CONSTRUCTOR_SELECTED / WORKER_SELECTION_BRANCH_ONLY_AT_FINAL_SCOPED_SPAWN / SHARED_EXISTING_WORKER_RESULT_REGISTRY_COMPLETION_ENVELOPE_PRESERVED / VU_CHILD_WORKER_REMAINS_PRIVATE_AND_UNCHANGED / FIXED_INTERNAL_MANAGEMENT_POLICY_PRESERVED / NO_GENERIC_MANAGEMENT_PROVIDER / NO_FILESYSTEM_AUTHORITY / NO_TERMINAL_BACKEND / NO_FORWARDING_BACKEND / NO_PRODUCTION_CALLER_MIGRATION / NO_BOOTSTRAP_WIRING / NO_DESKTOP_COMMAND3_DISPATCH / NO_RUNTIME_ACTIVATION / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_AGENT_STATUS_ONLY_RUNTIME_WORKER_SELECTION_SOURCE_SEAM_SELECTION_AND_BEFORE_THREE_PATH_SOURCE_MATERIALIZATION`

`NO_RACE_FREE_CLAIM`
