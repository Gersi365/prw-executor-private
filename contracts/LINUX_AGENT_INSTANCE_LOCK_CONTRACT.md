# Private Remote Workspace Linux Agent Instance-Lock Contract

Version: `0.1.0`

Status: Phase 065 implementation boundary — listener runtime not activated

## Scope

Phase 065 implements only the `agent.lock` single-instance authority locked by Phase 064.

The production adapter accepts a Phase 063 `ValidatedPrwRuntimeDirectory`. It does not accept a raw directory pathname and it does not inspect or mutate `agent.sock`.

## Fixed lock object

Basename: `agent.lock`

Required mode: `0600`

The file is persistent and is not unlinked when the lock guard is dropped.

## Open and validation

The lock file is opened/created relative to the validated PRW runtime-directory descriptor with:

- read/write access;
- create-if-missing semantics;
- final-component no-follow semantics;
- close-on-exec semantics;
- nonblocking open semantics so an unexpected special object cannot stall startup.

Creation requests mode `0600`.

Descriptor metadata must prove:

- regular-file type;
- owning UID equal to the effective Agent UID.

Only after those checks may a same-UID lock file be normalized with descriptor-based chmod to exact `0600`, followed by complete descriptor revalidation.

Wrong owner or wrong object type must never be repaired.

## Lock acquisition

The adapter requests `FlockOperation::NonBlockingLockExclusive` on the validated lock-file descriptor.

Results:

- success returns an owned `AgentInstanceLock` guard that retains the descriptor and therefore the lock;
- `EWOULDBLOCK`/`EAGAIN` becomes `AlreadyRunning`;
- any other flock error becomes a bounded lock-acquisition failure.

No retry loop or arbitrary wait timeout is introduced.

## Lifetime

Dropping the guard closes its owned descriptor. Linux then releases the advisory lock when the final descriptor referencing that locked open-file description is closed.

The filesystem `agent.lock` inode remains for reuse by later Agent instances.

## Test boundary

Tests use only temporary runner directories/files. They prove at least:

- absent lock file creation and `0600` validation;
- second concurrent conforming acquisition returns `AlreadyRunning`;
- acquisition succeeds after the first guard is dropped;
- pre-existing same-UID wrong-mode lock file is normalized to `0600`;
- final-component symlink fails closed without changing its target;
- explicit wrong-owner metadata classification fails before mode normalization.

## Forbidden interpretation

Phase 065 does not authorize or implement:

- inspection/unlink/bind/listen/accept/connect of `agent.sock`;
- systemd/service activation;
- a blocking lock wait loop;
- process-wide cwd/umask mutation;
- principal/policy changes;
- network/DNS/TUN/database/deployment changes.
