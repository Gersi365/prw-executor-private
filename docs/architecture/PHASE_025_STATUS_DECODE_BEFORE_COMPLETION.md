# Phase 025 — Decode Before Outstanding-Request Completion

## Objective

Lock the safest command-specific ordering for streamed `GetAgentStatus` responses: tracker state is consumed only after the response is fully valid at both the generic frame layer and the command-specific status-body layer.

## Ordering rationale

Phase 021 proves that a generic valid terminal response can complete a known request exactly once. `GetAgentStatus` has an additional five-byte body contract, however. Completing the request immediately after terminal validation would allow a malformed command-specific body to consume the outstanding request.

Phase 025 therefore imposes the stricter sequence:

```
stream read
→ generic frame validation
→ terminal Response+Ok validation
→ five-byte status-body decoding
→ request-ID completion
```

## Failure preservation

No request-tracker mutation occurs for truncated streams or malformed command-specific bodies. A fully decoded response for an unknown ID also leaves unrelated outstanding IDs intact.

## Relationship to existing phases

The implementation reuses:

- Phase 024 stream reader for acquisition and complete status decoding;
- Phase 013 `LocalRequestTracker::complete()` for the actual state transition.

Phase 021 remains available as the generic terminal-completion composition; it is not modified or duplicated.

## Focused validation

Tests prove:

- a fully valid known response completes only its own request;
- a malformed status body leaves its registered request outstanding;
- a truncated payload leaves its registered request outstanding;
- a fully decoded unknown ID leaves unrelated state untouched;
- replay after successful completion fails exactly once semantics.

## Runtime boundary

All tests use in-memory streams and state. Phase 025 does not open sockets, add dependencies, start timers/tasks/threads, dispatch commands, collect live status, or activate services.

## Next bounded step

With the successful status path now covered from typed snapshot through wire bytes and safe request completion, the next provider-neutral gap is the read-only `GetPrivateDnsConfig` response schema. Its existing domain type contains variable-length strings and lists, so bounds must be locked before any byte codec is introduced.
