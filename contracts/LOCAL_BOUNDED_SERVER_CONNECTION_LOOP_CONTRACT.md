# Private Remote Workspace Bounded Server Connection Loop Contract

Version: `0.1.0`

Status: Phase 052 provider-neutral bounded connection-loop composition

## Scope

Phase 052 repeatedly invokes the Phase 051 boundary-aware aggregate server connection entry point over caller-supplied generic `std::io::Read` and `std::io::Write` objects.

It owns no socket or transport.

## Caller-supplied budget

The caller provides a non-zero `usize` Request budget for one loop invocation.

No hard-coded Request count, timeout, or lifetime policy is introduced by Phase 052.

## Stop outcomes

The loop returns one of two successful stop outcomes:

- `CleanEof { responses_written }`: orderly EOF was reached before another frame began;
- `BudgetExhausted { responses_written }`: exactly the caller-supplied number of Requests produced successful terminal responses.

## No over-read on budget exhaustion

When the budget is exhausted, the function returns immediately after the last permitted response write. It must not probe or consume any byte of a following frame.

The caller may invoke the function again with a new non-zero budget to resume processing the same generic stream and aggregate state.

## Failure behavior

Any Phase 051 aggregate processing failure returns immediately with the existing typed error. Component state transitions already performed by lower layers remain authoritative.

A budget stop does not poison inbound or response-write state.

## Authentication boundary

The supplied policy evaluator is still assumed to represent an already authenticated policy context. Phase 052 does not authenticate a principal.

## Forbidden interpretation

Phase 052 does not authorize or implement:

- Unix socket bind/listen/accept/connect/close;
- `SO_PEERCRED` retrieval or principal authentication;
- XDG runtime-path mutation;
- an unbounded loop;
- wall-clock timeout policy;
- concurrency, cancellation, or task/thread creation;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
