# Phase 058 — Linux Same-UID Peer Authorization

## Objective

Turn the Phase 057 read-only Linux identity adapter into an explicit typed authorization boundary, without starting any socket listener or reading application protocol bytes.

## Data flow

Existing connected FD → Phase 057 `SO_PEERCRED` lookup → Agent effective UID → exact UID equality check → `AuthorizedLocalLinuxPeer` token or bounded failure.

## Token semantics

The authorization token is capability-like: it cannot be directly constructed by normal production callers and is returned only by the verified same-effective-UID path. It retains immutable PRW-local peer PID/UID/GID metadata.

The token is not itself a policy evaluator and does not grant command capabilities. It proves only the locked local transport precondition that the kernel-reported peer UID matches the Agent's effective UID.

## Failure behavior

- `SO_PEERCRED` failure → fail closed;
- peer UID mismatch → fail closed;
- neither failure reads or writes application bytes.

## Test strategy

An anonymous `UnixStream::pair()` validates the real same-process/same-UID success path. A read-only non-socket FD validates that peer-credential lookup failures remain bounded and do not accidentally authorize.

The UID equality predicate is also tested directly for match and mismatch values so the fail-closed comparison rule is deterministic without requiring privileged UID switching in CI.

## Runtime boundary

No filesystem socket pathname, listener, bind/accept/connect lifecycle, XDG mutation, stale-path cleanup, application protocol I/O, systemd activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
