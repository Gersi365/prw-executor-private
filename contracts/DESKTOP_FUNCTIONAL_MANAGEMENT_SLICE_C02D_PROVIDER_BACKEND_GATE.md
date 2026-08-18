# Phase 152 Slice C02d — Provider Backend and Cleanup Recovery Gate

Status: `DESIGN_LOCK / PROVIDER_CLEANUP_RECOVERY / NO_OS_IO / NO_RUNTIME_ACTIVATION`

Stacked predecessor: `6141ecf08976e26c99329592d52cbd8d736b0edf` (`phase-152-c02c-authority-foundation`).

## Purpose

C02c staged the complete crate-internal management path through authenticated admission, real Agent-owned authority, typed provider dispatch, deterministic local response encoding, and explicit lifecycle quiescence. It deliberately did not select concrete terminal or forwarding backends and did not wire management into production runtime.

C02d prepares the provider layer for later concrete Linux adapters. The first implementation slice is provider-neutral cleanup recovery: a backend close failure must not make the owning management lifecycle permanently non-quiescent when the backend handle still exists and a later cleanup retry can succeed.

C02d does not authorize PTY creation, child-process spawning, TCP listeners, target connections, byte pumps, runtime wiring, policy selection, service activation, deployment, or C03.

## Provider-neutral cleanup recovery

### Current blocker

The existing `TerminalBroker` and `PortForwardBroker` correctly retain a record in `Failed` state when `backend.close()` fails. This prevents silent reuse, but there is no typed operation that retries cleanup for the retained backend handle.

Because C02c `LocalManagementProviderLifecycle::try_finish` reports clean completion only when the terminal and forwarding brokers are empty, a retained failed record currently has no provider-neutral path back to quiescence.

### Locked retry semantics

C02d may add one explicit cleanup-retry operation to each broker.

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

Source tests should prove, without being executed while the build gate is closed:

- first close failure retains a `Failed` record and handle;
- a later cleanup retry invokes backend close again;
- fail-once/then-success cleanup returns a `Closed` record and removes it;
- repeated cleanup failure retains `Failed` state;
- cleanup retry on an open/active record fails before backend close;
- cleanup retry on an unknown identifier fails before backend close;
- successful cleanup does not permit later I/O/reuse under the same tracked record.

## Terminal backend design lock

The Phase 133 `TerminalBackend` boundary remains authoritative. A future Linux PTY adapter must preserve all existing type constraints.

### Executable selection

Only the existing named profiles are representable:

- `TerminalProfile::PosixShell`;
- `TerminalProfile::BashShell`.

A concrete backend may map those profiles only to provider-owned, audited fixed executable and argument templates. Request bytes must never supply:

- executable paths;
- arbitrary argument vectors;
- shell fragments or command strings;
- environment-variable bags;
- working-directory paths;
- startup scripts;
- privilege instructions.

The exact executable/template choices remain a later implementation review.

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

Capability admission alone is insufficient to authorize every syntactically valid IP/port target. Before a concrete backend connects, a later Agent-owned forwarding-target policy must decide whether the exact validated target is permitted.

That policy must be:

- assembled outside request decoding;
- default-deny;
- independent from request bytes;
- independent from workspace role metadata alone;
- unable to widen the loopback bind boundary;
- explicitly reviewed before production activation.

C02d does not choose the production allowlist/CIDR/port policy yet.

### Privileged local ports

The typed domain allows any non-zero bind port, but a production backend must not assume privilege to bind low ports. Under the current unprivileged gate, an OS permission failure is a normal fail-closed backend failure.

No Linux capability, setuid helper, privileged service, firewall, or system setting may be added to make such binds succeed in C02d.

### Connection/pump lifecycle

Before real forwarding OS I/O is implemented, the concrete design must lock finite bounds for:

- simultaneously accepted connections per forward;
- aggregate forwarding connections per Agent/provider lifecycle;
- connect timeout;
- read/write or idle deadlines;
- bounded buffer sizes;
- half-close behavior;
- listener cancellation;
- active connection cancellation;
- worker/pump join and teardown ordering.

A forwarding handle must not detach listener or byte-pump workers. Successful close must cancel/close owned sockets and join owned workers before reporting provider close success.

Close failure must retain sufficient handle state for later cleanup retry.

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

## First C02d implementation slice

Only provider-neutral cleanup recovery is authorized in this design branch before build validation:

- terminal failed-close retry;
- forwarding failed-close retry;
- source tests for those state transitions;
- corresponding contract/audit evidence.

This slice performs no OS I/O beyond whatever a later caller-supplied backend implementation of the already-existing trait would do. The repository currently contains only test/spies for these backend traits; C02d does not add a concrete OS backend in the first slice.

## Validation classification

The build gate remains closed.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`;
- formatter/linter/tests: `NOT_RUN / BUILD_GATE_CLOSED`;
- PTY/process execution: `NOT_RUN / NOT_AUTHORIZED`;
- TCP listener/connect/pump execution: `NOT_RUN / NOT_AUTHORIZED`;
- production policy selection: `NOT_AUTHORIZED`;
- runtime wiring: `NOT_AUTHORIZED`;
- deployment/privileged changes: `NOT_AUTHORIZED`;
- C03: `NOT_AUTHORIZED`.

## Next reviewed step

After provider-neutral cleanup recovery is staged and statically audited, C02d may continue with concrete backend implementation design. Actual Linux PTY or TCP adapter source should not begin until its fixed executable/environment policy, forwarding target policy, worker bounds, cancellation, join, and teardown semantics are explicitly locked.
