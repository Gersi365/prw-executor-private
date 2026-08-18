# Phase 152 Slice C02d — Provider Backend and Cleanup Recovery Gate

Status: `DESIGN_LOCK / EGRESS_POLICY_SHAPE_AND_FORWARDING_BOUNDS_LOCKED / NO_OS_IO / NO_RUNTIME_ACTIVATION`

Stacked predecessor: `6141ecf08976e26c99329592d52cbd8d736b0edf` (`phase-152-c02c-authority-foundation`).

## Purpose

C02c staged the complete crate-internal management path through authenticated admission, real Agent-owned authority, typed provider dispatch, deterministic local response encoding, and explicit lifecycle quiescence. It deliberately did not select concrete terminal or forwarding backends and did not wire management into production runtime.

C02d prepares the provider layer for later concrete Linux adapters. Staged source now covers provider-neutral cleanup recovery, pure Agent-owned provider policy seams, finite forwarding connection/timeout/buffer/half-close/join bounds, and a bounded exact-target forwarding egress policy shape. All of this remains OS-I/O-free and runtime-unwired.

C02d still does not authorize PTY creation, child-process spawning, TCP listeners, target connections, byte pumps, runtime wiring, production terminal executable/environment selection, production forwarding target values, service activation, deployment, or C03.

## Provider-neutral cleanup recovery

`TerminalBroker::retry_failed_close` and `PortForwardBroker::retry_failed_close` are teardown-only recovery paths.

For each broker, retry:

1. accepts only an existing exact `Failed` record;
2. rejects unknown/non-failed records before backend mutation;
3. requires the retained backend handle;
4. calls only the existing typed `backend.close(&mut handle)` operation;
5. preserves `Failed` + handle when close fails again;
6. clears the handle, marks `Closed`, removes the record, and returns the immutable closed record only after backend close succeeds;
7. never transitions failed state back to `Open`/`Active`;
8. never resumes application I/O after failure.

This provides a real path back to C02c lifecycle quiescence without treating `Drop`, record deletion, or state reassignment as cleanup evidence.

Source tests for these transitions remain authored but `NOT_RUN / BUILD_GATE_CLOSED`.

## Pure provider-policy source seam

C02d includes the crate-internal module:

`crates/prw-agent/src/local_commands/management_provider_backend_policy.rs`

It is registered only as a `pub(crate)` pre-runtime module and is not called by `main.rs`, bootstrap, server loop, production runtime, or deployment source.

### Terminal template identifier seam

`LinuxTerminalLaunchTemplateId` contains exactly:

- `PosixInteractiveShell`;
- `BashInteractiveShell`.

`for_profile(TerminalProfile)` performs a total mapping from the already-typed terminal profile. The type carries no executable path, argv, environment, cwd, command text, shell fragment, startup script, privilege instruction, or request-controlled string.

Repository review found no existing production `/bin/sh`, `/bin/bash`, or `env_clear` precedent. C02d therefore does not invent terminal executable/environment values. Exact terminal materialization remains a separate blocker.

## Forwarding egress policy

### Fail-closed base policy

`ForwardingEgressPolicy` accepts only an already-validated `TcpForwardSpec`. `DenyAllForwardingEgressPolicy` is the default pre-production policy and always denies.

The policy boundary receives no raw request bytes, DNS name, arbitrary bind address, socket-option bag, firewall instruction, or transport object.

### Locked production policy shape

C02d now locks the only production egress policy shape permitted by this slice: a bounded allowlist of exact typed `ForwardTarget` values.

`ExactForwardingEgressPolicy`:

- is assembled only by Agent-owned crate-internal code outside request decoding;
- stores only typed explicit IP address + non-zero TCP port targets;
- allows at most `32` configured input targets;
- deduplicates repeated exact targets;
- compares only `spec.target()` so local loopback bind-port choice cannot widen or narrow target authorization;
- cannot represent hostnames;
- cannot represent DNS names;
- cannot represent CIDRs or subnet ranges;
- cannot represent port ranges;
- cannot represent wildcard targets;
- cannot mutate bind semantics;
- cannot carry firewall, route, interface, socket-option, or privilege instructions.

An empty exact-target policy is valid and behaves as deny-all.

The bound of 32 matches the forwarding aggregate connection ceiling and existing Phase 140 concurrency precedent. The policy cannot grow without bound in memory.

C02d locks this **shape and assembly ownership only**. No production IP address or port value is selected by this branch. Real forwarding connect remains blocked until reviewed deployment configuration provides the exact allowed target set.

## Terminal backend design lock

The Phase 133 `TerminalBackend` boundary remains authoritative.

Only the existing named profiles are representable:

- `TerminalProfile::PosixShell`;
- `TerminalProfile::BashShell`.

A future concrete backend may map the provider-owned C02d template IDs only to audited fixed executable/argument templates. Request bytes must never supply executable paths, argv, shell fragments, command strings, environment bags, cwd paths, startup scripts, or privilege instructions.

PRW `TerminalPrincipal` remains authorization/audit identity, not a Linux account mapping. The first concrete provider must use the Agent process's existing effective OS credentials; no `setuid`/`setgid`, sudo/su/pkexec, PAM account selection, supplementary-group mutation, Linux capability elevation, or namespace privilege change is authorized.

A concrete terminal backend must also use an Agent-owned fixed/minimal environment and trusted working-directory policy, neither derived from request bytes. Exact values remain unselected.

A real terminal handle must own enough state for explicit close and child-process reaping. No child or PTY worker may detach. Backend close success must require provider-specific cleanup/reap evidence; failure must retain enough handle state for `retry_failed_close`.

## Forwarding backend design lock

### Bind and target domain

The Phase 134 domain remains authoritative:

- bind is only named IPv4/IPv6 loopback semantics;
- no request-selected arbitrary bind address exists;
- target is explicit validated IP address + non-zero TCP port;
- no hostname or resolver input exists;
- no DNS is introduced.

Low local ports remain ordinary fail-closed OS permission cases. No capabilities, setuid helper, privileged service, firewall, or system setting may be added to force such binds to succeed.

### Locked finite forwarding bounds

C02d reuses the already-reviewed Phase 140 transport profile:

- maximum simultaneous accepted connections for one forwarding session: `32`;
- maximum simultaneous forwarding connections across one Agent provider lifecycle: `32`;
- maximum exact egress targets in one Agent-owned policy: `32`;
- target connect timeout: `5 seconds`;
- inactivity/idle timeout: `30 seconds`;
- per-direction copy buffer: `65,536 bytes` (`64 KiB`).

The source precedent is Phase 140's `MAX_REMOTE_BIDI_STREAMS = 32`, `OPERATION_TIMEOUT = 5s`, `IDLE_TIMEOUT = 30s`, and 64 KiB control/stream receive bound. C02d copies these reviewed values into a pure Agent policy seam without adding a dependency on `prw-remote-transport`.

### Half-close behavior

Locked policy: `PropagateEofAndDrainPeer`.

1. EOF in one direction becomes peer write-half completion;
2. the opposite direction continues draining;
3. draining ends on opposite EOF, explicit cancellation, or the 30-second idle timeout;
4. half-close creates no detached worker.

### Close ordering

A future concrete forwarding close must execute:

1. `StopAccepting`;
2. `CancelActiveConnections`;
3. `JoinWorkers`;
4. only then may `PortForwardBackend::close` report success.

If cleanup/join evidence cannot be established, close fails and retains sufficient handle state for provider-neutral cleanup retry.

## No ambient privilege or network mutation

C02d does not authorize provider APIs accepting or performing firewall rules, route mutation, DNS configuration, interface selection, arbitrary socket-option bags, TUN/TAP creation, public/reverse forwarding, UDP, SOCKS, service-manager changes, or privilege escalation.

## Relationship to C02c lifecycle

C02c `LocalManagementProviderLifecycle` remains the Agent-owned aggregate owner. C02d does not alter C02c admission, authority, response encoding, or lifecycle ownership.

Cleanup recovery allows retained provider failures to be explicitly drained before `try_finish` reports quiescence. The rule remains: dropping active/failed provider state is not cleanup evidence.

## Staged C02d source slices

C02d currently stages only:

1. terminal failed-close recovery;
2. forwarding failed-close recovery;
3. cleanup state-machine tests;
4. provider-owned terminal template IDs derived only from `TerminalProfile`;
5. pure forwarding egress policy interface + deny-all default;
6. bounded exact-target egress policy shape with max 32 targets;
7. forwarding connection/timeout/buffer bounds derived from Phase 140 precedent;
8. explicit forwarding half-close behavior and close-stage ordering;
9. pure-source tests for policy/bounds semantics;
10. manual-only validation specification;
11. corresponding contract/audit evidence.

No staged C02d source performs PTY/process/socket/thread I/O or connects these seams to production runtime.

## Validation classification

The build gate remains closed.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`;
- formatter/linter/tests: `NOT_RUN / BUILD_GATE_CLOSED`;
- manual validation workflow: `STAGED / WORKFLOW_DISPATCH_ONLY / NOT_RUN`;
- PTY/process execution: `NOT_RUN / NOT_AUTHORIZED`;
- TCP listener/connect/pump execution: `NOT_RUN / NOT_AUTHORIZED`;
- production terminal executable/environment/cwd selection: `NOT_AUTHORIZED`;
- production forwarding target values: `NOT_SELECTED`;
- egress policy shape: `LOCKED_EXACT_TARGET_ALLOWLIST_MAX_32`;
- runtime wiring: `NOT_AUTHORIZED`;
- deployment/privileged changes: `NOT_AUTHORIZED`;
- C03: `NOT_AUTHORIZED`.

## Remaining C02d blockers

Concrete OS adapter source remains blocked by:

1. exact fixed terminal executable and argument templates;
2. fixed/minimal terminal environment and trusted working-directory policy;
3. actual reviewed production forwarding target values/configuration;
4. separately authorized build validation.

Forwarding policy shape, connection counts, timeouts, copy-buffer size, half-close behavior, cancellation, and worker join ordering are no longer open design questions.

Only after terminal materialization, deployment egress values, and build validation are explicitly resolved should Linux PTY or TCP adapter implementation begin.