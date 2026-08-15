# Phase 040 — Generic Request Read / Policy Response Processor

## Objective

Compose the validated generic Request reader with the crate-internal policy response pipeline without yet writing a response to any transport.

## Processing sequence

`generic Read -> one complete frame -> Request decode -> policy evaluation -> terminal response frame in memory`

Request decoding is a hard prerequisite for policy evaluation.

## Malformed input boundary

Malformed/truncated/non-Request/unknown-command input returns the existing typed Request-read/decode error. The policy evaluator is not called and this layer does not guess at an error response.

This avoids treating untrusted or insufficiently validated frame metadata as safe response correlation.

## Valid input behavior

A valid current command reaches the Phase 038 policy gate exactly once. The returned frame is either the existing command-specific successful response or the existing Unauthorized terminal error.

## Validation model

Memory streams prove:

- allowed valid Request -> successful correlated response;
- denied valid Request -> correlated Unauthorized response;
- unknown command -> policy evaluator call count remains zero;
- truncated Request -> policy evaluator call count remains zero.

## Explicit deferrals

Still deferred:

- response write and response-side write poisoning;
- malformed-request reply policy;
- peer authentication / `SO_PEERCRED` enforcement;
- live snapshot acquisition;
- socket accept/read/write loop;
- concurrency, timeout, cancellation, and retry.
