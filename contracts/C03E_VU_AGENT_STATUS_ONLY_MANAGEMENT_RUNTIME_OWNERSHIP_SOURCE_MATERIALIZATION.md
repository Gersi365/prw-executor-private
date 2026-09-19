# C03e-VU — AgentStatus-Only Management Runtime Ownership Source Materialization

Status: `SOURCE_MATERIALIZATION — PRODUCTION_ACTIVATION_NOT_AUTHORIZED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Exact predecessor

Evidence-closed C03e-VT is the exact predecessor:

- branch:
  `phase-152-c03e-vt-command3-management-boundary-transaction-source-materialization`
- exact head:
  `1e8a74fde23f61848b181b9c1f30a4bf80b888de`
- exact tree:
  `e542408510a523db2e446919c2d6b5bf71e36637`
- PR #710 remains draft/open/unmerged
- canonical VT evidence ID:
  `1qDN0MCL94BteBzGLaui448laSH8mtqOm`

VT selected `BridgeCommand::AgentStatus` as the first future live management slice and
stopped before worker/bootstrap/desktop runtime wiring.

## Historical runtime audit

Historical Phase-152 PRs #35–#38 proved a lock-late generic management deadline-session
and finite-worker design.

Fresh blob comparison proved VT is byte-identical to the historical deadline-session base
for:

- `linux_authenticated_session.rs`;
- `linux_session_worker.rs`;
- `management_provider_backend_policy.rs`;
- `management_provider_lifecycle.rs`;
- `server_connection_state.rs`.

However the historical generic runtime context also requires
`management_linux_backends.rs`, which is absent in VT and contains real PTY/process and
forwarding socket implementations. Its runtime context accepts a caller-supplied
`BoundedLocalManagementPolicy`.

Therefore importing the historical generic runtime stack would exceed the VT-selected
AgentStatus-only first-slice authority ceiling.

## VU selected design

VU materializes a narrower runtime ownership path instead.

The command-3 AgentStatus path owns:

- one fixed internal `BoundedLocalManagementPolicy`;
- exactly `AgentStatusRead = Allow`;
- every other represented capability = `Deny`;
- authenticated same-UID Linux admission;
- existing deterministic AgentStatus management response encoding;
- existing generic-frame boundary safety;
- existing aggregate inbound/write poisoning state;
- existing absolute read deadline;
- existing deferred response-write deadline;
- existing finite-worker request budget and permit RAII.

The narrow path accepts no caller-supplied management policy.

It owns no:

- production filesystem authority;
- provider lifecycle mutex;
- terminal backend;
- forwarding backend;
- transfer provider construction.

Therefore runtime assembly cannot widen the first command-3 slice by supplying broader
management authority.

## Legacy preservation

Commands 1/2 remain on the existing exact legacy decoder, read policy and responder.

Code 3 is classified after generic frame acquisition and delegated only to the fixed
AgentStatus adapter.

Malformed generic framing and malformed legacy requests preserve existing inbound poison
semantics.

Canonical command-3 admission failures remain correlated terminal errors.

## Worker ownership

The new finite worker sibling:

- consumes the existing authenticated session and worker permit;
- preserves permit RAII;
- preserves the exact existing finite request budget;
- gives each request a fresh absolute read budget;
- preserves deferred response-write budget semantics;
- never returns the connection for reuse;
- has no management policy/provider arguments.

The existing public legacy worker remains unchanged.

## Runtime activation ceiling

VU source remains dormant and crate-private.

VU does not connect the narrow worker to:

- listener accept/spawn;
- scheduler;
- Linux bootstrap;
- Agent `main.rs`;
- installed production Agent;
- desktop command-3 socket dispatch.

A later gate must explicitly select production spawn/bootstrap ownership before any
installed Agent can execute command 3.

## Explicit non-actions

VU performs no:

- production Agent/Desktop build, transfer, install or replacement;
- Agent restart/reload/reconfiguration;
- command-3 request to the production Agent;
- production policy mutation;
- filesystem-root selection;
- filesystem or transfer mutation;
- PTY/process launch;
- forwarding listener/connect;
- configured-remote selection/write;
- network/DNS/firewall/route mutation;
- credential mutation;
- sudo/root action;
- merge/ready/close;
- branch deletion;
- reset/rebase/squash/force update/history rewrite.

## Classification

`AGENT_STATUS_ONLY_MANAGEMENT_RUNTIME_OWNERSHIP_SOURCE_MATERIALIZED / FIXED_INTERNAL_POLICY_AGENT_STATUS_READ_ONLY / CALLER_CANNOT_WIDEN_MANAGEMENT_POLICY / NO_FILESYSTEM_AUTHORITY_REQUIRED / NO_PROVIDER_LIFECYCLE_REQUIRED / NO_TERMINAL_BACKEND / NO_FORWARDING_BACKEND / LEGACY_COMMANDS_1_2_PRESERVED / DEADLINE_AND_WORKER_BOUNDS_PRESERVED / RUNTIME_NOT_BOOTSTRAP_WIRED / DESKTOP_NOT_DISPATCHING_COMMAND3 / PRODUCTION_RUNTIME_UNCHANGED / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_AGENT_STATUS_ONLY_MANAGEMENT_RUNTIME_OWNERSHIP_SOURCE_MATERIALIZATION_AND_BEFORE_PRODUCTION_SPAWN_BOOTSTRAP_OR_DESKTOP_DISPATCH_WIRING`

`NO_RACE_FREE_CLAIM`
