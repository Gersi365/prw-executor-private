# Phase 057 — Linux Local Peer Identity Adapter

## Objective

Introduce a non-activating Linux-only adapter for effective Agent UID and kernel-reported Unix peer credentials using the exact rustix dependency baseline from Phases 054–056.

## API boundary

The adapter owns two read-only operations:

- read the Agent process effective UID;
- read `SO_PEERCRED` for an already-existing socket FD.

It translates rustix types into a small PRW-local credentials record so higher layers do not need to depend on rustix-specific types.

## Security ordering

This phase does not read application bytes and does not produce an authorization result. A future adapter must compare the peer UID to the effective Agent UID before the generic local Request pipeline is invoked.

## Error model

A failed peer-credential lookup maps to a bounded local error enum rather than exposing raw errno text to protocol layers.

## Test strategy

Linux unit tests use an anonymous `UnixStream::pair()` only as a pre-connected FD source. This exercises real Linux `SO_PEERCRED` behavior without creating a filesystem socket path, listener, accept loop, or XDG mutation.

## Runtime boundary

No filesystem socket, bind/listen/accept/connect, runtime-directory mutation, stale-path cleanup, protocol I/O, service activation, network/DNS/TUN mutation, database work, private-key operation, or deployment is introduced.
