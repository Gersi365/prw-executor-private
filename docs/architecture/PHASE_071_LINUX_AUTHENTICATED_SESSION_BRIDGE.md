# Phase 071 — Linux Authenticated Accept-to-Session Bridge

Status: implementation candidate; not wired to Agent bootstrap

## Purpose

Phase 071 connects two already validated type boundaries without adding transport or application runtime behavior:

```text
Phase 070 AuthenticatedAgentAcceptOutcome
                |
                v
Phase 071 pure composition
                |
                v
Phase 060 AuthenticatedLocalLinuxSession<UnixStream>
```

## Exact mapping

- `NoConnectionReady` remains `NoConnectionReady`.
- `Authenticated(connection)` becomes `AuthenticatedSession(AuthenticatedLocalLinuxSession::new(connection))`.

The bridge is infallible and does no I/O.

## Security ordering preserved

The input authenticated connection has already passed Linux `SO_PEERCRED` exact same-effective-UID authorization in Phase 070/059.

Phase 071 does not read application bytes, re-authenticate, bind a policy evaluator, or process Requests. The Phase 060 session simply owns the authenticated connection and a fresh aggregate protocol state.

## Deferred runtime work

No accept loop, readiness engine, scheduler, concurrency policy, request-budget policy, snapshot refresh, policy binding, bootstrap wiring, or service activation is introduced here.
