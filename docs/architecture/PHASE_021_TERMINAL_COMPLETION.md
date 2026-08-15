# Phase 021 — Transactional Terminal Completion

## Objective

Compose two already validated in-memory components:

- Phase 020 terminal-response frame validation; and
- Phase 013 bounded outstanding-request tracking.

The result is an exactly-once terminal completion operation that remains independent of any socket or async runtime.

## Transaction order

The implementation first validates the terminal frame. Only after that succeeds does it call `LocalRequestTracker::complete()` for the frame's existing request ID.

This ordering matters: a malformed or kind/status-inconsistent frame cannot consume request state.

## Known ID semantics

A valid terminal response for an ID that is not currently outstanding fails as `UnknownRequestId`. No other request is removed.

A successful completion removes exactly one known ID. Replaying the same response then fails until that ID is explicitly registered again under the existing Phase 013 rules.

## Error structure

The composition keeps Phase 020 validation errors separate from Phase 013 tracker errors. It does not flatten all failures into a generic protocol error.

## Tests

Focused tests prove:

- a valid `Response + Ok` completes exactly one request and leaves another untouched;
- a valid `Error + non-Ok` also completes its known request;
- an invalid terminal frame leaves the registered request intact;
- an unknown response ID leaves unrelated tracker state intact;
- replay after completion is rejected.

## Runtime boundary

Phase 021 performs no socket I/O, timeout scheduling, cancellation, command dispatch, filesystem mutation, dependency addition, service activation, or remote networking.

## Next bounded step

The next safe protocol/state step is to define timeout/cancellation state semantics independently of wall-clock scheduling: typed reasons and terminal tracker transitions can be modeled in memory before any timer/task runtime is selected.
