# PRW Phase 152 C02d Provider Backend and Cleanup Recovery Audit

Status: `EGRESS_POLICY_SHAPE_AND_FORWARDING_BOUNDS_LOCKED / CLEANUP_SEAMS_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_OS_IO`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- stacked_predecessor: `6141ecf08976e26c99329592d52cbd8d736b0edf`
- predecessor_branch: `phase-152-c02c-authority-foundation`
- branch: `phase-152-c02d-provider-backend-design`
- staged_head_before_audit_refresh: `2232dc078bab5afc679d0be47b4d8443e77dc54a`

## Exact stacked scope before this audit refresh

- relation: `ahead 17 / behind 0`
- merge base: exact C02c head `6141ecf08976e26c99329592d52cbd8d736b0edf`
- changed files: `7`
- additions: `1093`
- deletions: `1`

Changed paths remain restricted to:

- `.github/workflows/phase-152-c02d-provider-cleanup-validation.yml`
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02D_PROVIDER_BACKEND_GATE.md`
- `crates/prw-agent/src/local_commands.rs`
- `crates/prw-agent/src/local_commands/management_provider_backend_policy.rs`
- `crates/prw-forwarding/src/lib.rs`
- `crates/prw-terminal/src/lib.rs`
- `logs/audits/phase-152-c02d-provider-backend-design/C02D_PROVIDER_BACKEND_AUDIT.md`

No root Cargo/lock, Agent Cargo, `main.rs`, Linux bootstrap/runtime, policy crate, app, deployment, signing, systemd-credential, or privileged-system source is modified.

## Provider-neutral cleanup recovery

`TerminalBroker::retry_failed_close` and `PortForwardBroker::retry_failed_close` remain teardown-only recovery paths.

Both require an exact retained `Failed` record and retained handle, call only existing typed backend close, preserve failed state/handle on retry failure, and remove only after close success. Failed state never returns to `Open`/`Active`, and no user I/O is replayed.

This is the only path from retained provider failure back to C02c lifecycle quiescence. `Drop` and record deletion are not cleanup evidence.

Staged cleanup tests remain `NOT_RUN / BUILD_GATE_CLOSED`.

## Terminal provider policy state

The Agent-only `LinuxTerminalLaunchTemplateId` seam still maps only:

- `TerminalProfile::PosixShell -> PosixInteractiveShell`;
- `TerminalProfile::BashShell -> BashInteractiveShell`.

It carries no executable path, argv, environment, cwd, command text, shell fragment, startup script, privilege instruction, or request-controlled string.

Repository search found no production `/bin/sh`, `/bin/bash`, or `env_clear` precedent to reuse. Exact terminal executable/argument/environment/cwd values remain deliberately unselected rather than guessed.

Classification: `TERMINAL_MATERIALIZATION=BLOCKED_NO_REPO_PRECEDENT / NO_OS_SOURCE`.

## Forwarding egress policy shape now locked

The pure Agent-owned forwarding seam contains:

- `ForwardingEgressDecision::{Allow, Deny}`;
- `ForwardingEgressPolicy(TcpForwardSpec)`;
- `DenyAllForwardingEgressPolicy`;
- `ExactForwardingEgressPolicy`;
- `MAX_FORWARD_EGRESS_TARGETS = 32`.

`ExactForwardingEgressPolicy` is a bounded allowlist of exact typed `ForwardTarget` values only.

Locked properties:

1. Agent-owned crate-internal assembly only;
2. input values are already-validated explicit IP + non-zero TCP port targets;
3. maximum 32 configured input targets;
4. duplicate exact targets are collapsed;
5. evaluation compares `spec.target()` only, so loopback bind port does not alter target authorization;
6. no hostname or DNS representation;
7. no CIDR/subnet representation;
8. no port-range or wildcard representation;
9. no arbitrary bind address, firewall, route, interface, socket-option, or privilege representation;
10. an empty target set is valid and behaves as deny-all.

Source tests stage exact-target allow/deny, same-target/different-bind behavior, IP/port mismatch denial, deduplication, and over-bound rejection. They remain unexecuted.

No actual production target IP or port is selected. Real forwarding connect therefore remains blocked by deployment policy values even though policy **shape** is no longer open.

Classification: `FORWARDING_EGRESS_SHAPE=LOCKED_EXACT_TARGET_ALLOWLIST_MAX_32 / VALUES_NOT_SELECTED`.

## Forwarding finite bounds locked from Phase 140 precedent

The Agent policy module reuses existing Phase 140 design values:

- per-forward active connections: `32`;
- aggregate provider active connections: `32`;
- exact configured egress targets: `32`;
- connect timeout: `5 seconds`;
- idle timeout: `30 seconds`;
- per-direction copy buffer: `65,536 bytes`.

The source precedent is `prw-remote-transport` Phase 140:

- `MAX_REMOTE_BIDI_STREAMS = 32`;
- `OPERATION_TIMEOUT = 5 seconds`;
- `IDLE_TIMEOUT = 30 seconds`;
- 64 KiB control/stream bound.

No dependency on `prw-remote-transport` was added. This is design-value reuse only.

## Half-close and teardown order

Locked half-close policy: `PropagateEofAndDrainPeer`.

EOF is propagated to the peer write half; the opposite direction drains until its EOF, explicit cancellation, or 30-second idle expiry. No half-close creates detached work.

Locked forwarding close order:

1. `StopAccepting`;
2. `CancelActiveConnections`;
3. `JoinWorkers`;
4. only then backend close may report success.

Cleanup/join failure must retain sufficient handle state for provider-neutral retry.

## Static OS-I/O exclusion

The production provider-policy module imports typed domain values and `Duration` only. It does not call process, PTY, socket, thread, filesystem, resolver, firewall, or runtime APIs.

`std::net::{IpAddr, Ipv4Addr}` appears only under `#[cfg(test)]` for typed address fixtures and performs no network operation.

The module remains crate-internal and is not referenced by Agent main/bootstrap/runtime paths.

## Manual-only validation specification

`.github/workflows/phase-152-c02d-provider-cleanup-validation.yml` remains `workflow_dispatch` only and has not been dispatched.

Static invariants now include:

- exact C02c lineage and seven-path allowlist;
- forbidden Cargo/main/Linux-runtime/policy/app paths;
- cleanup APIs/tests;
- terminal template ID seam;
- deny-all and exact-target egress policy types;
- `MAX_FORWARD_EGRESS_TARGETS = 32`;
- exact `32 / 32 / 5s / 30s / 64KiB` forwarding bounds;
- half-close policy;
- `StopAccepting -> CancelActiveConnections -> JoinWorkers` ordering.

Future separately authorized toolchain validation stays scoped to provider crates plus `prw-agent --lib`; Agent binary/runtime execution is not part of the specification.

The workflow is not evidence until dispatched under explicit build authorization.

## Validation classification

No Rust toolchain or OS provider operation was executed.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`
- formatter/Clippy/tests: `NOT_RUN / BUILD_GATE_CLOSED`
- manual workflow: `STAGED / WORKFLOW_DISPATCH_ONLY / NOT_RUN`
- PTY/process I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- TCP bind/listen/connect/pump I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- terminal executable/argv/env/cwd: `NOT_SELECTED / BLOCKED_NO_REPO_PRECEDENT`
- forwarding finite lifecycle bounds: `LOCKED_FROM_PHASE140_PRECEDENT`
- forwarding egress policy shape: `LOCKED_EXACT_TARGET_ALLOWLIST_MAX_32`
- forwarding production target values: `NOT_SELECTED`
- runtime wiring: `NOT_AUTHORIZED`
- signing/systemd credentials: `NOT_AUTHORIZED`
- deployment/privileged changes: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Remaining C02d blockers

Concrete OS adapter source remains blocked by:

1. exact fixed terminal executable and argument templates;
2. fixed/minimal terminal environment and trusted working-directory policy;
3. actual reviewed production forwarding target values/configuration;
4. separately authorized build validation.

Forwarding policy shape, target-count bound, connection counts, timeouts, copy-buffer size, half-close behavior, cancellation, and worker join ordering are no longer open design questions.

No PTY child creation, TCP socket implementation, production runtime wiring, deployment, privileged changes, or C03 is authorized by this audit.