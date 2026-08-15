# Phase 029 — Local Private-DNS Stream Composition

## Objective

Compose the complete Phase 028 successful `GetPrivateDnsConfig` frame with the already validated generic Phase 011/012 frame reader/writer while remaining independent of any concrete socket transport.

## Write path

The writer accepts a generic `std::io::Write`, a typed request ID, and a bounded private-DNS snapshot. It builds the complete frame through Phase 028 and delegates exact header/payload output to Phase 012.

No flush occurs implicitly.

## Read path

The reader accepts a generic `std::io::Read`, acquires exactly one frame through Phase 011, and then delegates all private-DNS command validation to Phase 028.

This retains the important Phase 011 property that the 24-byte header is fully validated before payload allocation.

## Wire sizes

A default snapshot uses 29 bytes on the stream: 24-byte header + 5-byte payload.

A maximally bounded Phase 026 snapshot uses exactly 18,429 bytes: 24-byte header + 18,405-byte payload.

## Memory-only validation

Tests use `Vec<u8>` and `std::io::Cursor` only. They prove:

- exact default wire length;
- bounded UTF-8/list/flag round trip;
- exact maximum wire bound;
- exactly one frame is consumed and trailing bytes remain unread;
- truncated payload remains a generic `TruncatedPayload` read failure.

## Runtime boundary

No Unix socket, peer credentials, DNS configuration, command dispatcher, task/thread runtime, timer, dependency, or service activation is introduced.

## Next bounded step

After validation, the streamed private-DNS path should adopt the same decode-before-completion rule already locked for `GetAgentStatus`: command-specific decoding must succeed before its outstanding request ID is removed from connection-local tracker state.
