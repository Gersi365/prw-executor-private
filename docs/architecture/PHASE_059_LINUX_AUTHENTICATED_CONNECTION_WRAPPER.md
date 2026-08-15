# Phase 059 — Linux Authenticated Connection Wrapper

## Objective

Create a type-level boundary that associates an already-connected Linux stream with the Phase 058 same-effective-UID authorization proof before any application-protocol access is exposed inside the Agent crate.

## Construction

`AuthenticatedLocalLinuxConnection::try_new(stream)` owns the incoming stream value immediately, performs kernel-backed authorization through Phase 058, and returns the authenticated wrapper only on success.

On authorization failure, construction returns a typed error containing both the original stream and the bounded authorization failure. This preserves explicit caller ownership for rejection/close handling without protocol reads.

## Stored state

The wrapper stores:

- the exact owned stream instance that was authorized;
- the immutable `AuthorizedLocalLinuxPeer` token derived from that stream.

No caller may supply or replace the token independently.

## Access boundary

The Linux module remains crate-internal. Mutable stream access exists only through the successfully constructed wrapper. This establishes a future composition point where Phase 052 processing can require authenticated ownership rather than accepting an arbitrary raw stream.

## Ordering test

An anonymous Unix socket pair test writes sentinel bytes from one endpoint before wrapper construction. The other endpoint is then authorized and wrapped. Reading after construction must recover the sentinel bytes exactly, proving that `SO_PEERCRED` authorization did not consume application bytes.

## Runtime boundary

No filesystem socket, listener, bind/accept/connect lifecycle, XDG pathname mutation, stale-path cleanup, command policy evaluation, application loop integration, service activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
