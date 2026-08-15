# Phase 044 — Aggregate Server Connection Processing State

## Objective

Provide one pure in-memory usability boundary for the server side of a future local IPC connection before any socket runtime is activated.

## Composition

`LocalServerConnectionState` combines:

- Phase 043 `LocalInboundRequestState`;
- Phase 041 `LocalTerminalResponseWriteState`.

The connection is usable only when both directions remain healthy.

## Processing sequence

`aggregate usability precheck -> Phase 043 guarded transaction`

The precheck is intentionally ahead of input consumption. A connection already known to be unusable cannot consume another Request that it may be unable to parse or answer safely.

## Failure ownership

The aggregate does not duplicate lower-level transition logic:

- malformed/truncated/invalid Request -> inbound state owns poisoning;
- response write failure -> response-write state owns poisoning;
- success -> both remain healthy.

The aggregate only reports the resulting connection-level unusable reason.

## Recovery model

No state-reset method exists. A future runtime must dispose of an unusable connection instance and establish a fresh authenticated connection state rather than mutating the old protocol stream back to healthy.

## Validation model

Memory/synthetic streams prove healthy success, inbound poisoning, response-write poisoning, and zero-read/policy/write behavior on a later call after the aggregate is unusable.

## Explicit deferrals

Still deferred:

- actual socket close/discard/reconnect;
- peer authentication / `SO_PEERCRED` enforcement;
- safe stale-socket pathname lifecycle;
- clean EOF / multi-request loop semantics;
- live snapshot acquisition;
- concurrency, timeout, cancellation, retry.
