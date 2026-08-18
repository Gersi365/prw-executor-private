# PRW Phase 152 C02d Provider Backend and Cleanup Recovery Audit

Status: `CLEANUP_RECOVERY_AND_PURE_POLICY_SEAMS_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_OS_IO`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- stacked_predecessor: `6141ecf08976e26c99329592d52cbd8d736b0edf`
- predecessor_branch: `phase-152-c02c-authority-foundation`
- branch: `phase-152-c02d-provider-backend-design`
- staged_head_before_audit_refresh: `92bd1ef6d4324467b387376b38b761451772b6b5`

## Scope

C02d remains stacked directly on the C02c implementation-staged checkpoint. It now contains two bounded source slices before any concrete PTY or TCP backend can be reviewed:

1. provider-neutral retryable teardown for retained failed terminal/forwarding handles;
2. a pure Agent-owned provider policy seam for terminal template identifiers and forwarding egress decisions.

Exact stacked compare immediately before this audit refresh:

- relation: `ahead 9 / behind 0`
- merge base: exact C02c head `6141ecf08976e26c99329592d52cbd8d736b0edf`
- changed files: `7`
- additions: `983`
- deletions: `1`

C02d-specific changed paths before this audit refresh:

- `.github/workflows/phase-152-c02d-provider-cleanup-validation.yml`
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02D_PROVIDER_BACKEND_GATE.md`
- `crates/prw-agent/src/local_commands.rs`
- `crates/prw-agent/src/local_commands/management_provider_backend_policy.rs`
- `crates/prw-forwarding/src/lib.rs`
- `crates/prw-terminal/src/lib.rs`
- `logs/audits/phase-152-c02d-provider-backend-design/C02D_PROVIDER_BACKEND_AUDIT.md`

The C02d delta does not modify:

- root `Cargo.toml`;
- root `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- Agent `main.rs`;
- Agent Linux bootstrap/runtime source;
- `crates/prw-policy`;
- desktop or Android source;
- service-manager/deployment files;
- runtime signing or systemd credential loading.

## Terminal cleanup recovery

`TerminalBroker::retry_failed_close` is staged as teardown-only recovery.

Locked behavior:

1. removes the requested broker entry temporarily;
2. returns `UnknownSession` if no record exists;
3. requires retained state to be exactly `TerminalState::Failed`;
4. reinserts a non-failed record unchanged and returns `InvalidState` before backend close;
5. requires the failed record to retain a backend handle;
6. invokes only existing `TerminalBackend::close(&mut handle)`;
7. on backend close failure, retains `Failed` + handle and returns `Backend`;
8. on success, clears the handle, marks `Closed`, removes the broker record, and returns the closed immutable session record.

The retry path never transitions `Failed` back to `Open`, never replays terminal input/output/resize, and never changes immutable principal/profile/geometry state.

Staged source tests cover fail-then-success recovery, repeated failure retention, open/unknown rejection before backend close, and terminal removal after successful cleanup. They are authored but **not executed** while the build gate is closed.

## Forwarding cleanup recovery

`PortForwardBroker::retry_failed_close` mirrors the same teardown-only semantics.

Unknown/non-failed records fail before backend close; repeated backend failure retains the failed record and handle; success clears the handle, marks `Closed`, removes the record, and returns the immutable closed forwarding record.

The retry path does not reopen a listener, reconnect a target, resume pumps, or mutate forwarding principal/specification state.

Staged source tests cover fail-then-success recovery, repeated cleanup failure retention, active/unknown rejection, and removal after successful retry. They are authored but **not executed**.

## C02c quiescence blocker closure

C02c `LocalManagementProviderLifecycle::try_finish` reports clean completion only when terminal and forwarding brokers are empty.

Before C02d, backend close failure retained a failed record but exposed no provider-neutral retry path. C02d now provides an explicit route from retained `Failed` state to terminal `Closed` + broker removal only through another real backend close attempt.

No cleanup is inferred from `Drop`, record deletion, or state reassignment without backend close success.

## Pure Agent-owned provider policy seam

New source:

`crates/prw-agent/src/local_commands/management_provider_backend_policy.rs`

The module is registered in `local_commands.rs` only as a `pub(crate)` module with an explicit pre-runtime `dead_code` allowance. It is not referenced by `main.rs`, bootstrap, server loop, production runtime, or deployment source.

### Terminal template IDs

`LinuxTerminalLaunchTemplateId` contains exactly:

- `PosixInteractiveShell`;
- `BashInteractiveShell`.

`for_profile(TerminalProfile)` is a total mapping from the already-typed provider-neutral profile to a provider-owned template identifier.

The production type carries no:

- executable path;
- argv vector;
- raw command or shell fragment;
- environment bag;
- working directory;
- startup script;
- privilege instruction;
- request-controlled string.

No exact executable/argument/environment/cwd values are selected by this slice.

### Forwarding egress policy

The new module defines:

- `ForwardingEgressDecision::{Allow, Deny}`;
- `ForwardingEgressPolicy`, accepting only an already-validated `TcpForwardSpec`;
- `DenyAllForwardingEgressPolicy`, which always returns `Deny`.

The production module imports only `prw_forwarding::TcpForwardSpec` and `prw_terminal::TerminalProfile`. It has no process, PTY, socket, thread, filesystem, runtime, resolver, firewall, or privilege API.

`std::net::{IpAddr, Ipv4Addr}` is used only inside `#[cfg(test)]` to construct typed `TcpForwardSpec` fixtures. No network operation is performed by those source tests.

Staged tests prove the intended pure semantics:

- each terminal profile maps to only its provider-owned template ID;
- default forwarding egress is deny-all;
- a fixture policy can allow one exact validated specification while denying different bind/target specifications.

These tests remain **NOT_RUN / BUILD_GATE_CLOSED**.

## Concrete backend design remains blocked

The contract now explicitly separates the staged pure policy seam from concrete OS adapter materialization.

### Terminal blockers still open

Before PTY/process adapter source begins, a later reviewed design must lock:

- exact fixed executable for each template ID;
- exact provider-owned argument template;
- fixed/minimal environment policy and values;
- trusted working-directory policy;
- child/PTY ownership and reaping evidence;
- cancellation/join/teardown ordering.

PRW principal identity remains authorization/audit identity only and must not select a Linux account or trigger `setuid`/`setgid`, PAM, sudo/su/pkexec, Linux capability elevation, or related privilege changes.

### Forwarding blockers still open

Before listener/connect/pump source begins, a later reviewed design must lock:

- production egress allowlist/CIDR/port policy shape and assembly ownership;
- per-forward simultaneous connection bound;
- aggregate provider connection bound;
- connect timeout;
- read/write or idle deadline;
- bounded per-direction buffer size;
- half-close semantics;
- listener cancellation;
- active connection cancellation;
- worker/pump join ordering;
- close-success cleanup evidence.

Loopback-only bind and explicit-IP/no-DNS target constraints remain unchanged.

## Manual-only validation specification

`.github/workflows/phase-152-c02d-provider-cleanup-validation.yml` remains **workflow_dispatch-only** and has not been dispatched.

The updated future validation specification requires:

1. exact branch and caller-supplied 40-character expected head;
2. exact C02c stacked lineage;
3. an explicit seven-path C02d allowlist;
4. hard bans on root Cargo/lock, Agent Cargo, `main.rs`, Linux bootstrap/runtime, policy crate, and app source;
5. cleanup recovery and pure-policy source invariant greps;
6. locked metadata/lockfile determinism;
7. `cargo fmt --all -- --check`;
8. Clippy for terminal/forwarding plus `prw-agent --lib`, all with `-D warnings`;
9. provider tests plus only the Agent library tests matching `management_provider_backend_policy`;
10. provider build plus `prw-agent --lib` build;
11. final clean source diff.

The workflow is a staged specification only. It is **not build/test evidence** while the build gate remains closed.

## Validation classification

No Rust toolchain command was executed for this C02d slice.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`
- formatter/Clippy/tests: `NOT_RUN / BUILD_GATE_CLOSED`
- manual validation workflow: `STAGED / WORKFLOW_DISPATCH_ONLY / NOT_RUN`
- concrete PTY/process I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- concrete TCP listener/connect/pump I/O: `NOT_IMPLEMENTED / NOT_AUTHORIZED`
- production terminal executable/environment/cwd policy: `NOT_SELECTED / NOT_AUTHORIZED`
- production forwarding egress policy: `DENY_ALL_PRE_PRODUCTION / PRODUCTION_POLICY_NOT_SELECTED`
- runtime wiring: `NOT_AUTHORIZED`
- runtime signing/systemd credential loading: `NOT_AUTHORIZED`
- deployment/privileged changes: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Next C02d work

Cleanup recovery and the pure provider policy seams are source-staged and statically bounded.

Concrete OS adapter source remains blocked. The next safe C02d work is design/evidence only: lock exact terminal materialization policy and finite forwarding worker/connection lifecycle values, then update the manual validation specification accordingly.

Actual PTY child creation, TCP listener/connect/pump implementation, production runtime wiring, production policy selection, deployment, privileged changes, or C03 remain closed.