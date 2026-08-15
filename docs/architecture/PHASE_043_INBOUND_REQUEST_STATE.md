# Phase 043 — Inbound Request Processing State

## Objective

Prevent continued Request consumption after an input read/decode failure on one future connection instance.

## State model

`LocalInboundRequestState` begins `Healthy`. A Phase 042 Request-processing failure that originates from Request read/decode changes it to `ReadPoisoned`. The poisoned state is absorbing.

## Failure ownership

Inbound poisoning is limited to Request read/decode failures. Response-side write ambiguity remains represented by the separate Phase 041 `LocalTerminalResponseWriteState`.

This separation avoids falsely classifying a validly consumed Request as an input framing failure merely because its response could not be written.

## Admission ordering

`process_one_with_inbound_guard()` first rejects an already-poisoned inbound state, then delegates to the existing Phase 042 transaction. Therefore a poisoned inbound state performs no input read, policy evaluation, or response write.

## Validation model

Tests prove:

- successful transaction leaves inbound state Healthy;
- unknown command poisons inbound state before return;
- later attempt on poisoned inbound state performs no read/policy/write;
- response write failure poisons only response-write state, not inbound state.

## Explicit deferrals

Still deferred:

- actual connection close/discard implementation;
- unified connection lifecycle object;
- clean EOF versus protocol-error loop semantics;
- malformed-request terminal reply policy;
- peer authentication / `SO_PEERCRED`;
- live socket loop, concurrency, timeout, cancellation, and retry.
