# Phase 024 — Local Agent Status Stream Composition

## Objective

Compose the complete Phase 023 successful `GetAgentStatus` response frame with the already validated Phase 011 generic frame reader and Phase 012 generic frame writer.

The phase remains transport-agnostic: it uses only `std::io::Read` and `std::io::Write` and does not create or configure a socket.

## Exact wire size

A successful status response consists of:

- 24-byte local IPC frame header; and
- 7-byte successful `GetAgentStatus` payload.

Total wire size is therefore exactly 31 bytes.

## Write path

The write helper:

1. builds the complete status response frame through Phase 023;
2. passes that validated frame to Phase 012 `write_frame()`;
3. preserves frame-build and stream-write failures as distinct typed errors;
4. does not flush the writer implicitly.

No second frame/header/payload encoder is introduced.

## Read path

The read helper:

1. acquires exactly one complete frame through Phase 011 `read_frame()`;
2. therefore inherits header-first validation before payload allocation;
3. passes the validated frame to the Phase 023 successful status-frame decoder;
4. preserves generic frame-read failures separately from command-specific status decode failures.

Bytes following the first complete frame remain unread.

## In-memory validation

Phase 024 tests use only `Vec<u8>` and `std::io::Cursor`.

Focused coverage includes:

- exact 31-byte wire length;
- all four Agent runtime states round-tripping through write/read/decode;
- exact one-frame consumption with trailing bytes preserved;
- truncated header preservation as `TruncatedHeader`;
- truncated payload preservation as `TruncatedPayload`.

## Runtime boundary

Phase 024 does not:

- create, bind, listen, accept, or connect a Unix socket;
- access peer credentials;
- choose an async runtime;
- start tasks or threads;
- schedule timeouts or cancellation;
- dispatch or execute Agent commands;
- mutate outstanding-request state;
- collect live Agent state;
- add a dependency;
- activate systemd or deploy infrastructure.

## Next bounded step

After validation, the next safe composition point is to decide and prove the ordering between command-specific status decoding and outstanding-request completion. A malformed command-specific body must not accidentally consume request state; this can be modeled and tested entirely in memory before any socket runtime is activated.
