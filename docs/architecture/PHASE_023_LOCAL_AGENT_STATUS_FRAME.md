# Phase 023 — Complete GetAgentStatus Response Frame

## Objective

Compose the previously validated status-body and generic terminal-frame layers into one complete typed in-memory `GetAgentStatus` response frame.

## Encode path

`LocalAgentStatusSnapshot`
→ Phase 018 five-byte body
→ Phase 022 terminal builder with `Ok`
→ outer `Response` frame with current protocol version and the supplied request ID.

The resulting payload is byte-identical to the Phase 019 seven-byte success payload. This proves that the generic Phase 022 builder does not introduce a second status representation.

## Decode path

The decoder first applies the Phase 020 terminal-frame invariant. A valid terminal `Error` is still rejected because this module is specifically the successful `GetAgentStatus` path. For `Response + Ok`, only the fixed two-byte common status prefix is skipped and the remaining body is decoded by Phase 018.

The result preserves the original frame request ID and typed snapshot.

## Separation from completion state

Phase 023 does not remove the request ID from the Phase 013 tracker. Phase 021 already owns exactly-once tracker completion. A later composition step may combine successful status decoding with that transition without changing either component.

## Tests

Focused tests cover:

- exact payload equality with Phase 019;
- outer `Response` kind and request ID preservation;
- all four runtime-state round trips through a complete frame;
- valid terminal error rejection on the success-specific decoder;
- malformed status body rejection after terminal validation.

## Runtime boundary

All work remains in memory. No stream read/write, Unix socket, peer credential, async task, live state collection, command execution, dependency addition, or service activation occurs.

## Next bounded step

The next safe step is a byte-level round trip using the existing generic Phase 011 reader and Phase 012 writer with memory buffers only, proving that the complete status response survives the actual 24-byte frame-header wire codec plus seven-byte payload.
