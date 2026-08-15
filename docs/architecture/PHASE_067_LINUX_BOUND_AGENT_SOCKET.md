# Phase 067 — Linux Bound Agent Socket

Status: implementation boundary; not listening; bootstrap unchanged

## Objective

Create a bound filesystem-backed Unix stream socket only after Phase 065 lifecycle authority and Phase 066 stale-path preparation, then validate the resulting socket filesystem object before any future `listen` call is possible.

## Flow

```text
ValidatedPrwRuntimeDirectory + &AgentInstanceLock
      |
      v
Phase 066 prepare_agent_socket_path_for_bind
      |
      v
socket_with(AF_UNIX, STREAM, CLOEXEC)
      |
      v
SocketAddrUnix(/proc/self/fd/<prw-dir-fd>/agent.sock)
      |
      v
bind
      |
      v
statat(agent.sock, SYMLINK_NOFOLLOW)
      |
      +-- socket
      +-- owner == effective Agent UID
      +-- capture dev + ino
      v
chmodat(agent.sock, 0600)
      |
      v
statat(agent.sock, SYMLINK_NOFOLLOW)
      |
      +-- same dev + ino
      +-- socket
      +-- same UID
      +-- exact 0600
      v
BoundAgentSocket
```

## Why mode normalization is followed by identity revalidation

Linux does not implement `AT_SYMLINK_NOFOLLOW` for the selected `fchmodat` behavior. The operation therefore occurs only after the initial no-follow lookup has proven a same-UID socket inside the same-UID `0700` PRW directory while the exclusive lifecycle lock is held.

The subsequent no-follow stat must still match the original device/inode and security shape before the result is accepted.

## Partial-startup cleanup

Once an initial same-UID socket identity has been captured, any later construction failure closes the new socket descriptor and performs only identity-guarded best-effort removal. A changed pathname object is left untouched.

## Explicit bound-socket cleanup

`BoundAgentSocket::cleanup(self)` closes the socket descriptor, rechecks exact validated identity, unlinks only an unchanged node, and verifies absence.

Dropping without cleanup intentionally permits a stale pathname node, which models an unclean process exit and is recoverable by the Phase 066 locked stale classifier.

## Descriptor-anchor test

A test renames the actual validated PRW runtime directory after Phase 063/065 setup, creates a replacement directory at the old ambient pathname, and then binds Phase 067. The new `agent.sock` must appear in the renamed original directory referenced by the retained descriptor, not in the replacement directory.

## Deferred work

Phase 067 does not call `listen`, `accept`, `connect`, or Phase 059/060 connection processing. Those remain separate phases and bootstrap activation remains separately gated.
