# Phase 066 — Linux Agent Socket-Path Preparation

Status: implementation boundary

## Objective

Materialize the Phase 064 stale-path transaction without creating a listener. The operation is available only while a Phase 065 `AgentInstanceLock` guard is held.

## Flow

```text
ValidatedPrwRuntimeDirectory + &AgentInstanceLock
      |
      v
statat(dirfd, agent.sock, SYMLINK_NOFOLLOW)
      |
      +-- ENOENT -> AlreadyAbsent
      +-- other error -> fail closed
      v
trusted-shape validation
      |
      +-- Unix socket
      +-- owner == effective Agent UID
      +-- exact mode 0600
      v
capture first metadata snapshot
      |
      v
statat(dirfd, agent.sock, SYMLINK_NOFOLLOW)
      |
      +-- absent/error/change -> fail closed, no unlink
      v
match dev + ino + type + owner + mode
      |
      v
unlinkat(dirfd, agent.sock)
      |
      v
statat(dirfd, agent.sock, SYMLINK_NOFOLLOW)
      |
      +-- ENOENT -> StaleSocketRemoved
      +-- present/error -> fail closed
```

## Security properties

- Lifecycle authority is explicit in the function type through `&AgentInstanceLock`.
- Every lookup and unlink is descriptor-relative to the Phase 063 validated PRW directory.
- The final component is inspected without following symlinks.
- Existing socket objects are never chmod/chown repaired.
- A candidate must already have trusted shape: socket, same UID, exact `0600`.
- Device/inode/type/owner/mode are rechecked immediately before unlink so a pathname replacement is not silently removed.
- There is no connect probe.

## Test strategy

Temporary CI tests may bind `UnixListener` objects only to manufacture pathname socket nodes for the filesystem transaction. Production Phase 066 code has no bind/listen/connect call.

The tests also include two simultaneously existing socket nodes to prove the identity matcher distinguishes different device/inode identities deterministically.

## Deferred work

The next phase may implement the bind/post-bind validation object using temporary runner directories. Bootstrap/listener activation remains a separate later review.
