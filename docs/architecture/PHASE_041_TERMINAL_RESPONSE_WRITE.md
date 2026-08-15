# Phase 041 — Guarded Terminal Response Write

## Objective

Add response-side write ambiguity protection before any live transport is activated.

## Composition

`write_terminal_response_guarded()` performs:

`response-write state -> terminal-frame validation -> existing generic frame writer`

The state is independent from the Request-side Phase 034 send state so client Request-send semantics and server terminal-response-write semantics remain explicit.

## Safety properties

- already-poisoned state rejects before I/O;
- invalid terminal frame rejects before I/O and does not poison;
- successful write preserves Healthy state;
- any header/payload write failure changes the response-write state to WritePoisoned before return;
- poisoned state is absorbing and later writes perform no I/O;
- no implicit flush occurs.

## Validation model

Tests use `Vec<u8>` and deterministic synthetic writers only. They cover success, invalid Request-kind frame, header-write failure, payload-write failure after 24 bytes, and later-write rejection.

## Explicit deferrals

Still deferred:

- actual response socket write;
- transport teardown/reconnect;
- unified connection object combining read/request-send/response-write states;
- peer authentication and `SO_PEERCRED`;
- response-write integration with the Phase 040 processor;
- concurrency, timeout, cancellation, and retry.
