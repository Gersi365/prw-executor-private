# Private Remote Workspace Linux Listening Agent Socket Contract

Version: `0.1.0`

Status: Phase 068 implementation boundary — accept loop/bootstrap activation not authorized

## Scope

Phase 068 implements only the type-level transition from a Phase 067 `BoundAgentSocket` to a `ListeningAgentSocket`.

It does not bind a new pathname, accept a connection, read application bytes, or activate the Agent bootstrap/service.

## Input state

The constructor consumes a validated Phase 067 `BoundAgentSocket`.

Therefore its preconditions already include:

- validated XDG runtime root;
- validated same-UID `0700` PRW runtime directory;
- held Phase 065 instance lock;
- Phase 066 stale-path preparation;
- descriptor-anchored bind;
- same-UID socket filesystem identity;
- exact socket mode `0600`.

A raw file descriptor or raw pathname cannot be supplied through the intended typed entry point.

## Backlog

The backlog is supplied explicitly by the caller as `NonZeroU16`.

The implementation converts this value losslessly to the `i32` backlog required by `rustix::net::listen`.

Phase 068 defines no implicit/default backlog and no dynamic backlog tuning policy.

## Listen transition

The constructor calls `rustix::net::listen` exactly once on the owned Phase 067 socket descriptor.

- success consumes the bound state and returns `ListeningAgentSocket`;
- failure returns a typed `ListeningAgentSocketTransitionFailure` that retains the original `BoundAgentSocket` and a bounded `ListenFailed` error.

Returning the bound object on failure preserves the caller's ability to invoke its existing identity-guarded cleanup path.

## Listening object

`ListeningAgentSocket` owns the Phase 067 `BoundAgentSocket` and therefore transitively retains:

- the socket descriptor;
- the validated PRW runtime-directory borrow;
- the live Phase 065 instance-lock borrow;
- the validated socket filesystem identity.

It exposes descriptor borrowing for a later explicit accept phase and delegates explicit cleanup to Phase 067.

There is no reverse `listening -> bound` conversion because a successful `listen` changes kernel socket state and cannot be represented honestly as the earlier type.

## Test boundary

Temporary runner tests may connect a local `std::os::unix::net::UnixStream` to prove that a successfully transitioned socket accepts kernel connection establishment into its listen backlog. They do not call `accept`.

Tests cover at least:

- bound -> listening transition with caller-supplied backlog;
- validated filesystem identity is unchanged by `listen`;
- a local client can connect only after the socket has entered listening state;
- explicit cleanup closes the listening descriptor and removes the exact validated pathname through Phase 067 cleanup;
- the Phase 065 instance lock remains held while the listening object exists.

## Forbidden interpretation

Phase 068 does not authorize or implement:

- `accept`/`accept4`;
- an accept loop or worker scheduling;
- application-protocol reads/writes;
- SO_PEERCRED handling at accept time (already designed for the next phase);
- nonblocking/readiness policy changes;
- Agent bootstrap or systemd/service activation;
- network/DNS/TUN/database/deployment changes.
