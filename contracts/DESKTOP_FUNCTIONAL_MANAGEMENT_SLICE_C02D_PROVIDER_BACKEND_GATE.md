# Phase 152 Slice C02d — Provider Backend and Cleanup Recovery Gate

Status: `DESIGN_LOCK / FORWARDING_BOUNDS_LOCKED / NO_OS_IO / NO_RUNTIME_ACTIVATION`

Stacked predecessor: `6141ecf08976e26c99329592d52cbd8d736b0edf` (`phase-152-c02c-authority-foundation`).

## Purpose

C02c staged the complete crate-internal management path through authenticated admission, real Agent-owned authority, typed provider dispatch, deterministic local response encoding, and explicit lifecycle quiescence. It deliberately did not select concrete terminal or forwarding backends and did not wire management into production runtime.

C02d prepares the provider layer for later concrete Linux adapters. The first staged source slice added provider-neutral cleanup recovery so retained failed backend handles can be explicitly drained. The second staged slice added pure Agent-owned provider policy seams. The current design step additionally locks finite forwarding connection, timeout, buffer, half-close, cancellation, and join ordering by reusing existing Phase 140 transport bounds instead of introducing wider ad hoc limits.

C02d still does not authorize PTY creation, child-process spawning, TCP listeners, target connections, byte pumps, runtime wiring, production terminal executable/environment selection, production forwarding allowlists, service activation, deployment, or C03.

## Provider-neutral cleanup recovery

### Current blocker resolved by staged source

The existing `TerminalBroker` and `PortForwardBroker` correctly retain a record in `Failed` state when `backend.close()` fails. Before C02d there was no typed operation that retried cleanup for the retained backend handle.

Because C02c `LocalManagementProviderLifecycle::try_finish` reports clean completion only when the terminal and forwarding brokers are empty, a retained failed record needed a provider-neutral path back to quiescence.

### Locked retry semantics

C02d adds one explicit cleanup-retry operation to each broker.

The operation must:

1. accept only an existing record whose lifecycle state is exactly `Failed`;
2. reject unknown records with the existing unknown-session classification before backend mutation;
3. reject `Open`, `Active`, `Opening`, `Closing`, or any other non-`Failed` state before backend mutation;
4. require the retained backend handle to still exist;
5. call only the existing typed `backend.close(&mut handle)` operation;
6. on retry failure, keep the record in `Failed` state with its handle retained and return the existing backend failure classification;
7. on retry success, clear the backend handle, mark the record `Closed`, remove it from the broker, and return the terminal `Closed` record;
8. never transition a failed record back to `Open` or `Active`;
9. never resume application I/O after failure;
10. preserve the immutable PRW principal/session identity and terminal profile/forward specification carried by the record.

The cleanup retry is recovery of teardown authority only. It is not a retry of the user operation that originally failed.

### Required staged tests

Source tests prove the intended state machine, but remain unexecuted while the build gate is closed:

- first close failure retains a `Failed` record and handle;
- a later cleanup retry invokes backend close again;
- fail-once/then-success cleanup returns a `Closed` record and removes it;
- repeated cleanup failure retains `Failed` state;
- cleanup retry on an open/active record fails before backend close;
- cleanup retry on an unknown identifier fails before backend close;
- successful cleanup does not permit later I/O/reuse under the same tracked record.

## Pure provider-policy source seam

C02d includes a crate-internal, OS-I/O-free Agent module at:

`crates/prw-agent/src/local_commands/management_provider_backend_policy.rs`

The module is registered only as a `pub(crate)` pre-runtime module and is not called by the server loop, bootstrap, `main.rs`, or any production runtime path.

### Terminal template identifier seam

The module defines `LinuxTerminalLaunchTemplateId` with exactly two provider-owned identifiers corresponding to the already-typed terminal profiles:

- `PosixInteractiveShell`;
- `BashInteractiveShell`.

`LinuxTerminalLaunchTemplateId::for_profile` accepts only `TerminalProfile` and performs a total fixed mapping. The type carries no executable path, argument vector, environment, working directory, shell fragment, command string, or request-controlled text.

This seam deliberately stops before OS launch materialization. Exact executable paths, fixed argument templates, environment values, and working-directory policy remain a later explicit review gate.

### Forwarding egress policy seam

The module defines:

- `ForwardingEgressDecision::{Allow, Deny}`;
- a pure `ForwardingEgressPolicy` boundary accepting only an already-validated `TcpForwardSpec`;
- `DenyAllForwardingEgressPolicy` as the fail-closed pre-production implementation.

The policy boundary receives no raw request bytes, DNS name, arbitrary bind address, socket-option bag, firewall instruction, or transport object. The validated `TcpForwardSpec` already preserves the Phase 134 loopback-only bind and explicit-IP target domain.

A production allowlist/CIDR/port policy is not selected by C02d. Real forwarding connect remains blocked until such an Agent-owned policy is explicitly reviewed and wired before the concrete backend connection step.

## Terminal backend design lock

The Phase 133 `TerminalBackend` boundary remains authoritative. A future Linux PTY adapter must preserve all existing type constraints.

### Executable selection

Only the existing named profiles are representable:

- `TerminalProfile::PosixShell`;
- `TerminalProfile::BashShell`.

A concrete backend may map the provider-owned C02d template identifiers only to provider-owned, audited fixed executable and argument templates. Request bytes must never supply:

- executable paths;
- arbitrary argument vectors;
- shell fragments or command strings;
- environment-variable bags;
- working-directory paths;
- startup scripts;
- privilege instructions.

Repository review found no existing `/bin/sh`, `/bin/bash`, or environment-clearing production precedent to reuse. C02d therefore does not invent terminal executable/environment values. Exact terminal materialization remains blocked pending a separate explicit lock.

### OS identity boundary

A PRW `TerminalPrincipal` is authorization/audit identity, not a Linux account mapping.

The first concrete Linux terminal backend must run under the Agent process's already-existing effective OS credentials. C02d does not authorize:

- `setuid`/`setgid` identity switching;
- `sudo`, `su`, `pkexec`, PAM login, or other privilege elevation;
- creating or selecting a local account from PRW workspace/user/device identifiers;
- supplementary-group mutation;
- Linux capability elevation;
- namespace/container privilege changes.

If per-user OS-account execution is ever required, it needs a separate explicit security and deployment gate.

### Environment and working directory

A concrete backend must use an Agent-owned fixed/minimal environment policy and trusted working-directory policy. Neither may be derived from terminal request bytes.

No production environment/cwd values are chosen by C02d.

### PTY/process lifecycle

A real terminal handle must own enough state to perform bounded explicit close and process reaping. The provider must not detach child processes or PTY worker threads.

A successful backend close must eventually establish provider-specific terminal cleanup evidence, including child-process reaping where a child was spawned. Close failure must retain sufficient handle state for the provider-neutral cleanup retry path.

Any future helper thread must use scoped/joinable ownership or an equally explicit no-detach structure. C02d reuses the Agent's existing lifecycle principle—explicit cancellation plus observable join—not the existing UnixStream-specific cancellation type itself.

## Forwarding backend design lock

The Phase 134 forwarding domain remains authoritative.

### Bind boundary

A concrete backend may bind only the address implied by the validated named `LoopbackFamily`:

- IPv4 loopback semantics (`127.0.0.1`);
- IPv6 loopback semantics (`::1`).

It must not accept or synthesize wildcard, public, LAN, interface-specific, or request-selected bind addresses.

### Target boundary

Targets remain explicit validated IP address + non-zero TCP port. No hostname or resolver input is introduced. C02d does not add DNS.

### Agent-owned egress policy

Capability admission alone is insufficient to authorize every syntactically valid IP/port target. Before a concrete backend connects, the Agent-owned `ForwardingEgressPolicy` seam must decide whether the exact validated `TcpForwardSpec` is permitted.

The policy remains:

- assembled outside request decoding;
- default-deny before production selection;
- independent from raw request bytes;
- independent from workspace role metadata alone;
- unable to widen the loopback bind boundary;
- explicitly reviewed before production activation.

C02d stages the policy boundary and deny-all implementation but does not choose the production allowlist/CIDR/port policy.

### Privileged local ports

The typed domain allows any non-zero bind port, but a production backend must not assume privilege to bind low ports. Under the current unprivileged gate, an OS permission failure is a normal fail-closed backend failure.

No Linux capability, setuid helper, privileged service, firewall, or system setting may be added to make such binds succeed in C02d.

### Locked forwarding connection/pump bounds

C02d now locks finite values by reusing the already-reviewed Phase 140 remote-transport profile:

- maximum simultaneous accepted connections for one forwarding session: `32`;
- maximum simultaneous forwarding connections across one Agent provider lifecycle: `32`;
- target connect timeout: `5 seconds`;
- inactivity/idle timeout: `30 seconds`;
- per-direction copy buffer: `65,536 bytes` (`64 KiB`).

The aggregate cap equals the per-forward cap, so one forward may consume the full provider connection budget but no combination of forwards may exceed 32 active connections. No per-forward multiplier can expand the aggregate surface.

These values mirror the existing Phase 140 `MAX_REMOTE_BIDI_STREAMS = 32`, `OPERATION_TIMEOUT = 5s`, `IDLE_TIMEOUT = 30s`, and 64 KiB control/stream bound. C02d copies the reviewed design values into a pure Agent policy module; it does not add a dependency on `prw-remote-transport` and performs no network I/O.

### Half-close behavior

The locked policy is `PropagateEofAndDrainPeer`:

1. EOF in one direction is propagated as write-half completion to the peer side;
2. the opposite direction continues draining;
3. draining ends on opposite EOF, explicit cancellation, or the 30-second idle timeout;
4. half-close never creates a detached background worker.

### Listener/connection cancellation and join order

A future concrete forwarding close must follow this order:

1. `StopAccepting` — make the listener unable to admit new loopback connections;
2. `CancelActiveConnections` — cancel/close every currently owned connection so blocked pumps can terminate;
3. `JoinWorkers` — join every owned connection/pump worker;
4. only then may `PortForwardBackend::close` report success.

If any required close/join step cannot establish cleanup, backend close must fail and retain sufficient handle state for `retry_failed_close`.

No forwarding worker may be detached or treated as cleaned merely because an owner object was dropped.

## No ambient privilege or network mutation

C02d does not authorize any provider API accepting or performing:

- firewall rules;
- route mutation;
- DNS configuration;
- interface selection;
- arbitrary socket-option bags;
- TUN/TAP creation;
- public/reverse forwarding;
- UDP or SOCKS;
- service-manager changes;
- privilege escalation.

## Relationship to C02c lifecycle

C02c `LocalManagementProviderLifecycle` remains the Agent-owned aggregate owner. C02d cleanup recovery exists so retained provider failure state can be explicitly drained before `try_finish` reports quiescence.

The C02c rule remains unchanged: dropping active/failed provider state is not cleanup evidence.

The new pure policy/bounds seam does not alter C02c admission, authority, response encoding, or lifecycle ownership.

## Staged C02d source slices

C02d currently stages only:

1. terminal failed-close recovery;
2. forwarding failed-close recovery;
3. source tests for those state transitions;
4. provider-owned terminal template identifiers derived only from `TerminalProfile`;
5. a pure forwarding egress policy interface and deny-all pre-production implementation;
6. finite forwarding connection/timeout/buffer bounds derived from Phase 140 precedent;
7. explicit forwarding half-close policy and close-stage ordering;
8. source tests for those pure policy/bounds seams;
9. manual-only validation specification;
10. corresponding contract/audit evidence.

No staged C02d source performs PTY/process/socket/thread I/O or connects the new seams to production runtime.

## Validation classification

The build gate remains closed.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`;
- formatter/linter/tests: `NOT_RUN / BUILD_GATE_CLOSED`;
- manual validation workflow: `STAGED / WORKFLOW_DISPATCH_ONLY / NOT_RUN`;
- PTY/process execution: `NOT_RUN / NOT_AUTHORIZED`;
- TCP listener/connect/pump execution: `NOT_RUN / NOT_AUTHORIZED`;
- production terminal executable/environment/cwd selection: `NOT_AUTHORIZED`;
- production forwarding egress allowlist selection: `NOT_AUTHORIZED`;
- runtime wiring: `NOT_AUTHORIZED`;
- deployment/privileged changes: `NOT_AUTHORIZED`;
- C03: `NOT_AUTHORIZED`.

## Next reviewed step

Forwarding lifecycle numerics are now locked, but concrete OS adapter source remains blocked.

The next safe C02d design work is narrower:

1. lock exact fixed terminal executable/argument templates;
2. lock fixed/minimal terminal environment and trusted working-directory policy;
3. lock production forwarding egress policy shape and assembly ownership without selecting request-controlled targets;
4. separately authorize and run build validation.

Only after terminal materialization and production egress policy are explicitly locked, and build validation is separately authorized, should Linux PTY or TCP adapter implementation begin.