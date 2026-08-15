# Phase 064 — Linux Agent Socket Lifecycle Security

Status: decision locked; runtime not activated

## Objective

Close the stale-socket/single-instance/bind-ordering design blocker left by Phases 014, 053, and 061 without yet creating a real filesystem-backed listener.

## Locked startup sequence

```text
ValidatedXdgRuntimeRoot
      |
      v
ValidatedPrwRuntimeDirectory
      |
      v
open/create agent.lock relative to PRW dir
      |
      +-- no-follow
      +-- regular file
      +-- owner == effective UID
      +-- normalize/revalidate 0600 only after owner proof
      v
nonblocking exclusive flock
      |
      +-- busy -> ALREADY_RUNNING; touch nothing else
      v
statat(agent.sock, SYMLINK_NOFOLLOW)
      |
      +-- missing -> bind preparation
      |
      +-- socket + same UID + 0600
      |       |
      |       +-- record dev/inode
      |       +-- re-stat exact identity
      |       +-- unlinkat
      |       +-- confirm absent
      |
      +-- anything else -> fail closed
      v
AF_UNIX SOCK_STREAM + CLOEXEC
      |
      v
bind /proc/self/fd/<prw-dir-fd>/agent.sock
      |
      v
descriptor-relative mode normalization to 0600
      |
      v
statat no-follow post-bind validation
      |
      +-- socket
      +-- same UID
      +-- exact 0600
      +-- capture dev/inode
      v
listen
```

## Why an instance lock replaces connect probing

The lifecycle contract requires every conforming Agent to hold a nonblocking exclusive `flock` on persistent `agent.lock` for its full listener lifetime.

Therefore a second conforming process that cannot acquire the lock stops before examining or mutating the socket pathname. A process that successfully acquires the lock knows no conforming live Agent holds the lifecycle authority; it can then apply strict stale-file classification to a residual socket node.

This avoids creating a probe connection before startup classification is complete.

## Persistent lock-file policy

`agent.lock` remains on disk after orderly shutdown. Only the advisory lock state is transient.

Leaving the inode persistent avoids an unlink/recreate race around the lock authority itself.

## Descriptor anchoring

Linux has no pathname Unix-socket `bindat`. The locked adapter strategy uses the already-open Phase 063 PRW-directory descriptor through `/proc/self/fd/<fd>/agent.sock` rather than changing current working directory or falling back to an ambient re-resolved XDG path.

If the `/proc/self/fd` anchor is unavailable, startup fails closed.

All metadata inspection, stale unlink, mode normalization, and cleanup decisions remain descriptor-relative to the validated PRW directory.

## Post-bind permission strategy

The process does not change its global umask.

The parent PRW directory is already same-UID `0700`. After bind, the socket filesystem node is normalized to `0600` and validated before `listen` is allowed.

## Socket identity token

Post-bind validation records at least:

- filesystem device identifier;
- inode identifier;
- socket type;
- effective-UID owner;
- mode `0600`.

Cleanup requires a fresh no-follow stat matching this identity. A changed object is not unlinked.

## Accepted peer sequence

```text
accept
  -> Phase 059 SO_PEERCRED + same-UID authentication
  -> Phase 060 authenticated application session
  -> caller-bounded command processing
```

No application Request byte is read before same-UID transport authentication succeeds.

## Shutdown sequence

```text
stop accepts
  -> close listener
  -> re-stat agent.sock
  -> unlink only exact recorded socket identity
  -> confirm absence when possible
  -> close/release agent.lock last
```

An unclean exit releases the advisory lock when descriptors close; a residual pathname socket is handled as stale only after the next process acquires the lock.

## Deferred implementation

Phase 064 adds no runtime code. The next implementation work should be staged rather than activating the full listener in one step:

1. instance-lock adapter and tests;
2. stale `agent.sock` classifier/unlink transaction and tests;
3. bind/post-bind validation object using temporary runner directories only;
4. accepted-stream composition with existing Phase 059/060 types;
5. only after those pass, a separate runtime/listener activation review.
