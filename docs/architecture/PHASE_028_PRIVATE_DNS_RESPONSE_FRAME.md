# Phase 028 — Complete GetPrivateDnsConfig Response Frame

## Objective

Compose the validated bounded private-DNS snapshot path into a complete successful local IPC response frame without adding a new framing or serialization mechanism.

## Encode path

`LocalPrivateDnsSnapshot`
→ Phase 027 fallible bounded binary codec
→ Phase 022 terminal-response builder with `Ok`
→ outer `Response` frame with current IPC protocol version and supplied request ID.

The generic terminal builder remains responsible for the two-byte status prefix, outer kind, header payload length, and frame coupling.

## Maximum size

The Phase 027 body maximum is 18,403 bytes. Adding the two-byte `Ok` prefix yields a maximum command payload of 18,405 bytes. Adding the fixed 24-byte frame header yields a maximum wire size of 18,429 bytes.

This is intentionally much smaller than the global 1 MiB local IPC limit.

## Decode path

The decoder first applies the Phase 020 terminal-response invariant. A valid `Error` frame is rejected because this module is the successful command path. For `Response + Ok`, it strips only the validated common status prefix and delegates all command-body validation to Phase 027.

The result preserves the original request ID and returns the bounded private-DNS snapshot.

## Default stable representation

A default disabled snapshot with empty lists produces payload bytes:

```
00 00 00 00 00
```

The frame outer kind is `Response` and payload length is five.

## Error layering

The implementation preserves:

- encode error vs terminal-frame build error;
- terminal-frame decode error vs non-success status vs private-DNS body decode error.

No error is flattened into text.

## Tests

Focused tests prove:

- locked maximum body/payload/wire sizes;
- stable default payload;
- request-ID preservation;
- UTF-8/list/flag round trip through a complete frame;
- maximum snapshot fits the success payload bound;
- terminal error rejection;
- malformed private-DNS body rejection after terminal validation.

## Runtime boundary

All work remains in memory. No DNS configuration, socket/stream I/O, peer credentials, command execution, dependency addition, tracker mutation, or service activation occurs.

## Next bounded step

After validation, this complete response frame can be composed with the already validated generic Phase 011/012 frame reader/writer for a memory-stream round trip, following the same pattern used by the `GetAgentStatus` path.
