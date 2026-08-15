# Phase 031 — Complete Local Read-Only Request Frame

## Objective

Compose the existing two-byte read-only command payload with the validated local IPC frame layer so a request can be represented as one complete in-memory object before any transport runtime exists.

## Encode path

`LocalAgentCommand`
→ Phase 015 two-byte command codec
→ bounded `LocalIpcPayload`
→ current-version `LocalIpcFrameHeader` with outer kind `Request`
→ `LocalIpcFrame`.

The supplied request ID is preserved. The caller cannot choose `Response` or `Error` through this builder.

## Exact wire size

Both currently admitted commands use exactly two payload bytes. Combined with the fixed 24-byte frame header, the future wire representation is exactly 26 bytes.

Stable command payloads remain:

- `GetAgentStatus`: `00 01`
- `GetPrivateDnsConfig`: `00 02`

## Decode path

The decoder rejects any non-`Request` frame before command interpretation, then delegates the exact payload to the Phase 015 decoder. This prevents response/error frames from being confused with requests solely because their payload bytes resemble a command ID.

## Reuse boundary

Phase 031 deliberately does not duplicate:

- command identifiers;
- command payload length validation;
- frame-version validation;
- request-ID validation;
- payload bounds;
- header/payload length coupling.

Those remain owned by previously validated components.

## Runtime boundary

All work remains in memory. No `Read`/`Write` stream, request tracker, Unix socket, command dispatcher, peer credential, task/thread/timer, dependency, or service activation is introduced.

## Next bounded step

After validation, the fixed 26-byte Request frame can be composed with the generic Phase 011/012 frame reader/writer for memory-stream round trips. Outbound request registration versus write ordering remains a separate state-transition decision and will not be silently coupled to framing.
