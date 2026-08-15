# Phase 030 — Private-DNS Decode Before Request Completion

## Objective

Apply the same safe ordering already validated for `GetAgentStatus` to streamed `GetPrivateDnsConfig` responses: command-specific bounded decoding must succeed before request-tracker state is consumed.

## Ordering

```
stream read
→ generic frame validation
→ terminal Response+Ok validation
→ bounded private-DNS body decoding
→ request-ID completion
```

This prevents a structurally terminal frame with malformed DNS snapshot bytes from consuming an outstanding request.

## Reuse

The implementation delegates acquisition and complete command-specific decoding to Phase 029, then calls the existing Phase 013 `LocalRequestTracker::complete()` transition. It does not create a second tracker or private-DNS decoder.

## Failure preservation

Tests prove that tracker state remains intact after:

- malformed private-DNS body;
- truncated stream;
- a fully decoded response with an unknown request ID.

A fully valid known response removes exactly its own ID, leaving other outstanding requests untouched. Replay after successful completion is rejected.

## Runtime boundary

All validation uses in-memory `Cursor`/`Vec` state. No Unix socket, DNS mutation, peer credentials, async runtime, timer, command dispatcher, dependency, or service activation is introduced.

## Next bounded step

With both read-only command response paths now covered through wire decode and safe tracker completion, the next protocol gap is the **request side**: a complete `Request` frame builder/decoder for the existing two-byte command payload can be added and validated entirely in memory before runtime transport activation.
