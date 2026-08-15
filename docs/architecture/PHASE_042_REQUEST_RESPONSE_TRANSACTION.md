# Phase 042 — One-Request Policy / Guarded-Response Transaction

## Objective

Join the Phase 040 generic Request processor and the Phase 041 guarded response writer without activating a Unix socket runtime.

## Sequence

`response-write-state precheck -> generic Read -> Request decode -> policy gate -> terminal frame -> guarded generic Write`

The response-write precheck occurs first so a connection instance already known to be write-poisoned does not consume another Request that it cannot safely answer.

## Safety properties

- already-poisoned response state performs zero input reads, zero policy calls, and zero output writes;
- invalid Request input performs no policy evaluation and no response write;
- allowed valid Request writes the existing success response;
- denied valid Request writes the existing correlated Unauthorized response;
- response write failure poisons the response-write state;
- no implicit flush occurs.

## Validation model

Tests use memory streams and deterministic synthetic readers/writers only. No socket is created.

## Explicit deferrals

Still deferred:

- read-side connection poisoning/discard policy after malformed or truncated input;
- actual socket connection lifecycle;
- peer authentication / `SO_PEERCRED` enforcement;
- live host snapshot acquisition;
- multi-request loop and concurrency;
- timeout/cancellation/retry.
