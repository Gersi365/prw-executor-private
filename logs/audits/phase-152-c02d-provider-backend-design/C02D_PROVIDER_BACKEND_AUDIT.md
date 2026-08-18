# PRW Phase 152 C02d Provider Backend and Cleanup Recovery Audit

Status: `FORWARDING_BOUNDS_LOCKED / CLEANUP_AND_POLICY_SEAMS_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_OS_IO`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- stacked_predecessor: `6141ecf08976e26c99329592d52cbd8d736b0edf`
- predecessor_branch: `phase-152-c02c-authority-foundation`
- branch: `phase-152-c02d-provider-backend-design`
- staged_head_before_audit_refresh: `5b392ba4ea6866b6d84e857abad85733fc49469e`

## Exact stacked scope before this audit refresh

- relation: `ahead 13 / behind 0`
- merge base: exact C02c head `6141ecf08976e26c99329592d52cbd8d736b0edf`
- changed files: `7`
- additions: `1124`
- deletions: `1`

Changed paths:

- `.github/workflows/phase-152-c02d-provider-cleanup-validation.yml`
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02D_PROVIDER_BACKEND_GATE.md`
- `crates/prw-agent/src/local_commands.rs`
- `crates/prw-agent/src/local_commands/management_provider_backend_policy.rs`
- `crates/prw-forwarding/src/lib.rs`
- `crates/prw-terminal/src/lib.rs`
- `logs/audits/phase-152-c02d-provider-backend-design/C02D_PROVIDER_BACKEND_AUDIT.md`

No root Cargo/lock, Agent Cargo, `main.rs`, Linux bootstrap/runtime, policy crate, app, deployment, signing, systemd-credential, or privileged-system source is modified by the C02d delta.

## Provider-neutral cleanup recovery

`TerminalBroker::retry_failed_close` and `PortForwardBroker::retry_failed_close` are staged as teardown-only recovery.

For both brokers:

- only exact retained `Failed` state is accepted;
- unknown/non-failed records fail before backend close;
- the retained backend handle is required;
- retry invokes only existing typed `backend.close(&mut handle)`;
- retry failure preserves `Failed` + handle;
- retry success clears the handle, marks `Closed`, removes the record, and returns the immutable closed record;
- no failed record can return to `Open`/`Active` or resume application I/O.

This closes the C02c quiescence gap without treating `Drop` or record deletion as cleanup evidence.

Source tests for fail-then-success, repeated failure, state rejection, and removal are staged but `NOT_RUN / BUILD_GATE_CLOSED`.

## Pure Agent-owned provider policy seam

`crates/prw-agent/src/local_commands/management_provider_backend_policy.rs` is registered only as a crate-internal pre-runtime module.

### Terminal template identity

`LinuxTerminalLaunchTemplateId` contains only:

- `PosixInteractiveShell`;
- `BashInteractiveShell`.

`for_profile(TerminalProfile)` performs a total typed mapping. No executable path, argv, environment, cwd, command text, shell fragment, startup script, privilege instruction, or request-controlled string is carried by this type.

Repository search found no existing production `/bin/sh`, `/bin/bash`, or `env_clear` precedent to reuse. Exact terminal executable/argument/environment/cwd materialization therefore remains intentionally unselected rather than invented.

Classification: `TERMINAL_MATERIALIZATION=BLOCKED_NO_REPO_PRECEDENT / NO_OS_SOURCE`.

### Forwarding egress seam

The module defines:

- `ForwardingEgressDecision::{Allow, Deny}`;
- pure `ForwardingEgressPolicy(TcpForwardSpec)`;
- `DenyAllForwardingEgressPolicy`.

The production module receives only already-validated forwarding specs and has no socket/process/thread/filesystem/runtime/resolver/firewall API. `std::net` appears only under `#[cfg(test)]` for typed fixture construction.

Production egress allowlist/CIDR/port selection remains unselected; real connect therefore remains blocked.

## Forwarding finite bounds locked from existing repo precedent

C02d does not invent a new concurrency/timeout/buffer profile. It reuses values already locked by the Phase 140 `prw-remote-transport` foundation:

- `MAX_REMOTE_BIDI_STREAMS = 32`;
- `OPERATION_TIMEOUT = 5 seconds`;
- `IDLE_TIMEOUT = 30 seconds`;
- `MAX_CONTROL_PAYLOAD_BYTES = 65,536` and a 65,536-byte stream receive window.

C02d stages corresponding pure Agent policy constants:

- `MAX_FORWARD_CONNECTIONS_PER_SESSION = 32`;
- `MAX_FORWARD_CONNECTIONS_AGGREGATE = 32`;
- `FORWARD_CONNECT_TIMEOUT = 5 seconds`;
- `FORWARD_IDLE_TIMEOUT = 30 seconds`;
- `FORWARD_COPY_BUFFER_BYTES = 65,536`.

The aggregate cap equals the per-forward cap, so one forward can consume the provider budget but multiple forwards cannot multiply the aggregate concurrency beyond 32.

No new Cargo dependency on `prw-remote-transport` was added; the values are design precedent only.

## Forwarding half-close and teardown order

Locked half-close policy: `PropagateEofAndDrainPeer`.

Semantics:

1. EOF in one direction becomes peer write-half completion;
2. the opposite direction keeps draining;
3. draining ends on opposite EOF, explicit cancellation, or the 30-second idle timeout;
4. no half-closed connection creates detached work.

Locked provider close stages:

1. `StopAccepting`;
2. `CancelActiveConnections`;
3. `JoinWorkers`;
4. only then may backend close report success.

If required cleanup/join evidence cannot be established, close must fail with enough handle state retained for provider-neutral cleanup retry.

## Manual-only validation specification

`.github/workflows/phase-152-c02d-provider-cleanup-validation.yml` remains `workflow_dispatch` only and has not been dispatched.

It now statically asserts:

- exact stacked lineage and seven-path allowlist;
- root Cargo/lock, Agent Cargo/main/Linux runtime, policy crate, and app exclusions;
- terminal/forwarding cleanup APIs and staged tests;
- provider-policy module registration;
- terminal template ID and deny-all egress seams;
- exact forwarding `32 / 32 / 5s / 30s / 64KiB` constants;
- half-close policy;
- `StopAccepting -> CancelActiveConnections -> JoinWorkers` close ordering.

Future authorized toolchain validation is limited to:

- `cargo fmt --all -- --check`;
- `prw-terminal` + `prw-forwarding` Clippy/tests/build;
- `prw-agent --lib` Clippy/build;
- Agent library test filter `management_provider_backend_policy`;
- final clean diff.

The workflow remains a specification, not execution evidence.

## Validation classification

No Rust toolchain or OS provider operation was executed.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`
- formatter/Clippy/tests: `NOT_RUN / BUILD_GATE_CLOSED`
- manual workflow: `STAGED / WORKFLOW_DISPATCH_ONLY / NOT_RUN`
- PTY/process I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- TCP bind/listen/connect/pump I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- terminal executable/argv/env/cwd: `NOT_SELECTED / BLOCKED_NO_REPO_PRECEDENT`
- forwarding finite lifecycle bounds: `LOCKED_FROM_PHASE140_PRECEDENT`
- forwarding production egress policy: `DENY_ALL_PRE_PRODUCTION / PRODUCTION_POLICY_NOT_SELECTED`
- runtime wiring: `NOT_AUTHORIZED`
- signing/systemd credentials: `NOT_AUTHORIZED`
- deployment/privileged changes: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Remaining C02d blockers

Concrete OS adapter source remains blocked by only the policy/materialization gates not yet resolved:

1. exact fixed terminal executable and argument templates;
2. fixed/minimal terminal environment and trusted working-directory policy;
3. production forwarding egress policy shape/allowlist assembly ownership;
4. separately authorized build validation.

Forwarding connection counts, timeouts, copy-buffer size, half-close behavior, cancellation, and worker join ordering are no longer open design questions.

No PTY child creation, TCP socket implementation, production runtime wiring, deployment, privileged changes, or C03 is authorized by this audit.