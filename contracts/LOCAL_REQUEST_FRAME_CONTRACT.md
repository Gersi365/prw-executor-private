# Local Read-Only Request Frame Contract

Status: Phase 031 locked baseline

## Purpose

Define the complete in-memory local IPC Request frame for the two currently admitted read-only Agent commands. This phase composes the Phase 015 two-byte command codec with the existing validated frame constructors and does not activate stream or socket runtime behavior.

## Inputs

The Request frame builder accepts only:

- a typed non-zero `LocalIpcRequestId`; and
- a typed `LocalAgentCommand`.

The caller does not supply outer message kind, payload length, or protocol version.

## Build rules

The builder MUST:

1. encode the command through the existing Phase 015 command encoder;
2. use the resulting exact two-byte payload;
3. set outer message kind to `LocalIpcMessageKind::Request`;
4. use the current `LocalIpcProtocolVersion`;
5. preserve the supplied typed request ID;
6. use the existing `LocalIpcPayload`, `LocalIpcFrameHeader`, and `LocalIpcFrame` constructors.

No second command-code mapping may be introduced.

## Exact size

Current command payload length: **2 bytes**.

With the existing fixed 24-byte local IPC frame header, every current read-only Request frame has exact wire length:

**26 bytes**.

## Stable command bytes

- `GetAgentStatus` -> `00 01`
- `GetPrivateDnsConfig` -> `00 02`

## Decode ordering

The decoder MUST:

1. require outer message kind `Request` before interpreting command bytes;
2. preserve the existing frame request ID;
3. delegate the payload to the Phase 015 command decoder;
4. reject payloads that are not exactly two bytes or contain an unknown command identifier.

A `Response` or `Error` frame MUST NOT be interpreted as a request even if its payload happens to contain valid command bytes.

## Lower-level invariants

Phase 031 operates on `LocalIpcFrame`, so the existing validated frame layer remains authoritative for supported protocol version, non-zero request ID, bounded payload, and declared/actual payload-length equality.

## Runtime boundary

Phase 031 adds no:

- stream I/O;
- Unix socket creation, bind, listen, accept, or connect;
- request-tracker mutation;
- command dispatch or execution;
- filesystem/network/DNS mutation;
- new dependency;
- peer-credential access;
- privileged-helper invocation;
- account authentication;
- cryptographic private-key operation;
- systemd activation;
- database or deployment.

## Explicit deferrals

Still deferred:

- generic stream read/write composition for Request frames;
- outbound request registration/write ordering;
- command-specific error body schema;
- timeout/cancellation and late-response policy;
- live command dispatch;
- Unix socket runtime and peer-credential enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
