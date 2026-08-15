# Phase 052 — Bounded Server Connection Loop

## Objective

Add a provider-neutral bounded loop around the Phase 051 clean-EOF-aware aggregate connection entry point without selecting a socket runtime or imposing a product-level timeout/request-count policy.

## Work quantum

Each invocation receives a caller-supplied `NonZeroUsize` Request budget. The loop processes at most that many successful Request/response transactions.

## Stop behavior

- Clean EOF before the next frame begins returns `CleanEof` with the number of responses written during this invocation.
- Exhausting the work budget returns `BudgetExhausted` with the exact budget count.
- Budget exhaustion performs no extra boundary probe, so a following frame remains completely unread.

## Resumption

A caller may invoke the bounded loop again on the same reader, writer, and `LocalServerConnectionState`. This provides a future runtime with an explicit scheduling/work-quantum primitive without introducing threads, timers, or concurrency here.

## Error behavior

Any Phase 051 error is returned immediately. Existing inbound/write poisoning semantics remain authoritative; the loop adds no new poison state.

## Runtime boundary

No socket, file descriptor, peer credential, filesystem pathname, timeout, concurrent task, authentication, systemd activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
