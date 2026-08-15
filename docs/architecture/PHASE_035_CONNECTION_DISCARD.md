# Phase 035 — Connection Discard / Outstanding Request Disposition

## Objective

Preserve explicit correlation for every outstanding Request when a future local IPC connection is discarded, without activating a transport or choosing retry behavior.

## Disposition model

`discard_local_connection_request_state()` captures:

- whether the connection send-state was write-poisoned;
- every still-outstanding request ID in deterministic registration order.

It then leaves the old `LocalRequestTracker` empty.

## Why abandoned is distinct from completed

A connection can disappear without a validated terminal response. In particular, a write-poisoned Request may have been partially or completely observed by the peer even though the local writer returned an error. Treating such IDs as completed would fabricate a terminal outcome; silently dropping them would lose caller correlation.

Phase 035 therefore surfaces them as abandoned only.

## Why there is no automatic retry

Automatic retry is deliberately absent. The protocol must not assume idempotence merely because today's two commands are read-only. Future command classes may have different semantics, and write ambiguity can make peer observation uncertain.

## Validation model

Tests prove:

- healthy discard returns outstanding IDs in registration order;
- poisoned discard records poison context and returns the ambiguous request ID;
- empty discard is stable;
- abandonment empties the old tracker without creating a completion record.

## Explicit deferrals

Still deferred:

- typed caller-visible failure result for abandoned requests;
- retry/idempotency policy;
- actual connection close/reconnect;
- cancellation frames and timeout handling;
- late-response handling across replaced connections;
- connection generation/epoch identity;
- Unix socket runtime and peer credential enforcement.
