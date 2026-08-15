# Local Agent Status Stream I/O Contract

Status: Phase 024 locked baseline

## Purpose

Compose the Phase 023 complete successful `GetAgentStatus` response frame with the existing Phase 011 generic frame reader and Phase 012 generic frame writer. This contract remains transport-agnostic and uses only `std::io::Read` / `std::io::Write`.

## Wire length

A successful `GetAgentStatus` frame contains:

- 24-byte local IPC frame header; and
- 7-byte successful status payload.

Therefore its exact wire length is 31 bytes.

## Write path

The Phase 024 writer MUST:

1. build a complete successful status frame through Phase 023;
2. write that validated frame through the Phase 012 generic `write_frame()` path;
3. preserve build and write failures as separate typed categories;
4. perform no implicit flush.

## Read path

The Phase 024 reader MUST:

1. acquire exactly one complete validated frame through Phase 011 `read_frame()`;
2. decode that frame through the Phase 023 successful status-frame decoder;
3. preserve generic frame-read and status-frame-decode failures as separate typed categories;
4. leave bytes following the first complete frame unread.

## Validation ordering

The generic Phase 011 reader remains authoritative for header-first acquisition and payload-length-bounded allocation. Phase 024 MUST NOT allocate command payload storage before the generic header validation has succeeded.

## Error preservation

The composition MUST preserve at least:

- truncated header;
- header I/O failure;
- invalid header;
- unsupported payload length;
- truncated payload;
- payload I/O failure;
- status-frame decode failures.

It MUST NOT collapse these into an untyped generic string error.

## Runtime boundary

Phase 024 adds no:

- Unix socket creation, bind, listen, accept, or connect;
- peer-credential access;
- async runtime or task/thread policy;
- timeout or cancellation scheduling;
- command dispatch or execution;
- outstanding-request mutation;
- filesystem mutation;
- new dependency;
- account authentication;
- privileged-helper invocation;
- DNS/network mutation;
- cryptographic private-key operation;
- systemd activation;
- database or deployment.

## Explicit deferrals

Still deferred:

- composition with Phase 021 outstanding-request completion;
- command-specific error body schema;
- bounded private-DNS response body and codec;
- timeout/cancellation policy;
- live runtime status collection;
- runtime command dispatch;
- actual Unix socket runtime and SO_PEERCRED enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
