# Phase 065 — Linux Agent Instance Lock

Status: implementation boundary

## Objective

Materialize the Phase 064 single-instance authority without touching `agent.sock` or activating a listener.

## Flow

```text
ValidatedPrwRuntimeDirectory
      |
      v
openat(agent.lock, RDWR|CREATE|NOFOLLOW|CLOEXEC|NONBLOCK, 0600)
      |
      v
fstat
      |
      +-- regular file
      +-- owner == effective Agent UID
      v
if mode != 0600
      |
      v
fchmod(fd, 0600)
      |
      v
fstat + complete revalidation
      |
      v
flock(fd, NonBlockingLockExclusive)
      |
      +-- would block -> AlreadyRunning
      +-- other error -> fail closed
      v
AgentInstanceLock { owned fd }
```

## Security properties

- Path resolution is relative to the Phase 063 validated PRW directory descriptor.
- Final-component symlinks are not followed.
- Wrong owner and non-regular-file objects are never repaired.
- Mode repair is descriptor-based and occurs only after same-UID regular-file proof.
- There is no blocking wait or startup timeout policy in this layer.
- The lock guard owns the descriptor; dropping the guard releases the advisory lock through descriptor closure while leaving the persistent lock file in place.

## Deferred work

Phase 065 does not inspect or mutate `agent.sock`. Stale socket classification is a separate later transaction that requires an already-acquired `AgentInstanceLock`.
