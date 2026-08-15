# Private Remote Workspace Linux Bound Agent Socket Contract

Version: `0.1.0`

Status: Phase 067 implementation boundary — listener/bootstrap activation not authorized

## Scope

Phase 067 implements a bound-but-not-listening Linux filesystem-backed Agent socket object and its post-bind filesystem validation.

The production constructor requires:

- a Phase 063 `ValidatedPrwRuntimeDirectory`; and
- a live Phase 065 `AgentInstanceLock`.

The constructor internally executes the Phase 066 socket-path preparation transaction before socket creation/bind, so stale-path preparation cannot be skipped through this entry point.

Phase 067 does not call `listen`, `accept`, or `connect`, and the Agent bootstrap does not invoke this constructor.

## Descriptor-anchored bind path

Linux provides no pathname Unix-socket `bindat` operation.

The constructor builds the bind address from the retained PRW runtime-directory descriptor as:

`/proc/self/fd/<prw-runtime-directory-fd>/agent.sock`

The PRW runtime-directory descriptor remains alive for the full `BoundAgentSocket` lifetime.

If construction of this Linux descriptor-anchored address or bind fails, the constructor fails closed. There is no fallback to a re-resolved ambient XDG pathname, current-working-directory mutation, TCP, or an abstract Unix socket.

## Socket creation

The socket is created as:

- address family: Unix;
- type: stream;
- close-on-exec flag set;
- default protocol.

No nonblocking/listen policy is introduced in this phase.

## Initial post-bind identity

Immediately after bind, descriptor-relative no-follow metadata lookup of `agent.sock` beneath the validated PRW directory must prove:

- object type is Unix socket;
- owner UID equals the effective Agent UID.

The implementation captures stable filesystem identity fields including device and inode.

Failure before a stable same-UID socket identity is captured fails closed and does not guess at cleanup.

## Mode normalization and revalidation

The socket filesystem entry is normalized to exact `0600` relative to the validated PRW runtime directory.

On Linux the selected `chmodat` operation uses no unsupported no-follow flag. This occurs only after the no-follow initial lookup has proved a same-UID socket inside the already-validated same-UID `0700` PRW directory while the exclusive instance lock is held.

Immediately after normalization, a second descriptor-relative no-follow lookup must prove:

- same device and inode as the initial bound socket;
- Unix-socket type;
- same effective-UID owner;
- exact mode `0600`.

Any identity change or validation failure fails closed.

## Partial-construction cleanup

After stable initial socket identity has been captured, failures during mode normalization/revalidation trigger a best-effort cleanup transaction that:

1. closes the newly created socket descriptor;
2. re-stats `agent.sock` no-follow relative to the validated PRW directory;
3. unlinks only if device/inode/socket-type/effective-UID still match the object created by this bind attempt;
4. verifies absence.

A changed object is never unlinked by this recovery path.

## Returned object

Success returns `BoundAgentSocket<'a>` which:

- owns the bound socket descriptor;
- borrows the validated PRW runtime directory;
- borrows the live `AgentInstanceLock` for its lifetime;
- stores the validated socket filesystem identity;
- records whether Phase 066 found the path already absent or removed a stale socket;
- implements descriptor borrowing for a later explicit listen phase.

It is not a listening socket yet.

## Explicit cleanup

`BoundAgentSocket::cleanup(self)` closes the bound socket descriptor first, then removes `agent.sock` only if a fresh no-follow lookup still matches the recorded device/inode/socket-type/effective-UID/exact-0600 identity. Absence is accepted as already clean; a replacement object is not removed.

Dropping the object without explicit cleanup closes the socket descriptor but may leave a stale pathname node, intentionally matching crash-like recovery semantics handled by Phase 066 on the next locked startup.

## Test boundary

Tests use only temporary runner filesystem directories and do not alter the Agent bootstrap.

They cover at least:

- path already absent -> bind succeeds; node is socket/same-UID/0600; explicit cleanup removes it;
- a trusted stale socket is removed by Phase 066 and replaced by the newly bound object;
- the `/proc/self/fd` bind remains anchored to the validated PRW directory after its ambient pathname is renamed and replaced;
- a wrong-mode stale socket blocks before bind and remains untouched;
- cleanup refuses to unlink a replacement socket with a different filesystem identity.

## Forbidden interpretation

Phase 067 does not authorize or implement:

- `listen`, `accept`, or accepted-peer processing;
- Agent bootstrap/service activation;
- a TCP/abstract-socket/ambient-path fallback;
- process-wide cwd/umask mutation;
- principal/policy changes;
- network/DNS/TUN/database/deployment changes.
