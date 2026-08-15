# Private Remote Workspace Linux Agent Socket-Path Preparation Contract

Version: `0.1.0`

Status: Phase 066 implementation boundary — bind/listen runtime not activated

## Scope

Phase 066 implements only the locked stale-`agent.sock` classification and unlink transaction from Phase 064.

The production entry point requires both:

- a Phase 063 `ValidatedPrwRuntimeDirectory`; and
- a live Phase 065 `AgentInstanceLock` guard.

It does not create, bind, listen on, accept from, or connect to a Unix socket.

## Fixed object

Basename: `agent.sock`

Required stale-candidate mode: `0600`

## Initial classification

The fixed basename is inspected relative to the validated PRW runtime-directory descriptor with final-component no-follow metadata semantics.

Results:

- pathname absent: return `AlreadyAbsent`; no mutation;
- metadata lookup failure other than absence: fail closed;
- existing object continues only if it is a Unix socket, owned by the effective Agent UID, with exact permission/special-mode bits `0600`;
- symlink, non-socket object, wrong owner, or wrong mode: fail closed without unlink or repair.

No chmod/chown repair is permitted for an existing `agent.sock` candidate.

## Identity recheck before unlink

For a trusted-shape stale candidate, the implementation retains its descriptor-relative metadata snapshot.

Immediately before unlink it performs a second no-follow descriptor-relative metadata lookup and requires the second snapshot to match the first for at least:

- filesystem device identifier;
- inode identifier;
- object type;
- owner UID;
- exact mode.

Absence, metadata failure, or any mismatch at this recheck fails closed without unlinking a replacement object.

## Unlink and verification

Only an unchanged trusted-shape candidate may be removed with descriptor-relative `unlinkat` while the supplied `AgentInstanceLock` guard remains alive.

After unlink, a final no-follow descriptor-relative metadata lookup must report `ENOENT`.

- absence -> return `StaleSocketRemoved`;
- object still present -> fail closed;
- another metadata error -> fail closed.

## Authority boundary

The lock guard is a mandatory API parameter. The Phase 066 function cannot be invoked through its intended typed entry point without an already-acquired Phase 065 lifecycle authority.

This layer does not re-probe or reacquire the advisory lock; the owned guard's lifetime is the authority token.

## Connect-probe policy

There is no connect probe. Phase 064 already locked the exclusive persistent instance lock as the conforming-Agent single-instance authority.

## Test boundary

Tests may create temporary pathname Unix sockets using `UnixListener`, files, symlinks, and directories on the CI runner. Those are test scaffolding only and are removed after each test.

Tests cover at least:

- missing `agent.sock` -> `AlreadyAbsent`;
- same-UID stale socket at exact `0600` -> removed and verified absent;
- symlink -> fail closed and target unchanged;
- regular file -> fail closed and unchanged;
- stale socket with wrong mode -> fail closed and remains;
- explicit wrong-owner metadata classification -> fail closed;
- distinct socket identities do not compare as the same stale candidate;
- Phase 065 lock contention still prevents a second conforming lifecycle authority.

## Forbidden interpretation

Phase 066 does not authorize or implement:

- creating/binding/listening/accepting/connecting an Agent listener;
- unlinking an untrusted or changed object;
- chmod/chown repair of `agent.sock`;
- systemd/service activation;
- TCP or abstract-socket fallback;
- process-wide cwd/umask mutation;
- principal/policy changes;
- network/DNS/TUN/database/deployment changes.
