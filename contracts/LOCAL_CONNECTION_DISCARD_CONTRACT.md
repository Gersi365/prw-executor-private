# Local Connection Discard Contract

## Status

Phase 035 locks provider-neutral disposition of outstanding local Requests when one future connection instance is discarded.

## Abandonment semantics

Connection discard is not terminal-response completion.

All IDs still present in the bounded per-connection request tracker are removed and returned in original registration order as `abandoned` IDs.

The caller can therefore associate an explicit future failure/disposition with every outstanding Request rather than silently losing correlation state.

## Write-poison context

The discard disposition also records whether the connection send-state was `WritePoisoned`.

A Request whose write failure poisoned the connection remains outstanding under Phase 033/034 and is therefore included in the abandoned-ID set when the connection state is discarded.

## Retry rule

Phase 035 performs no retry and authorizes no automatic retry. A partially written Request may have been observed by the peer, so retry policy must remain a separate explicit decision even though the currently implemented commands are read-only.

## Tracker postcondition

After discard disposition is captured:

- the old tracker is empty;
- none of the abandoned IDs are classified as completed by this operation;
- a terminal completion attempted against the emptied old tracker is rejected as an unknown ID.

## Runtime boundary

Phase 035 does not close a socket, send a cancellation frame, reconnect, schedule retry, invoke callbacks, create tasks, mutate networking/DNS, or perform user-facing notification. It only transforms bounded in-memory correlation state.
