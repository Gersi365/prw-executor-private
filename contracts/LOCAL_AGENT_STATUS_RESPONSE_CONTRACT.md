# Local Agent Status Success Payload Contract

Status: Phase 019 locked baseline

## Purpose

Compose the common Phase 016 response-status prefix with the Phase 018 fixed-width Agent status body for the successful `GetAgentStatus` path. This remains pure in-memory protocol work and does not activate command dispatch or a socket runtime.

## Exact successful payload

A successful `GetAgentStatus` command payload is exactly 7 bytes:

```
bytes 0..1  response_status     u16, big-endian
byte 2      runtime_state       u8
bytes 3..4  protocol_major      u16, big-endian
bytes 5..6  protocol_minor      u16, big-endian
```

For this success-specific codec, bytes 0..1 MUST encode `LocalAgentResponseStatus::Ok` (`00 00`).

Current `Ready` / protocol `1.0` success bytes:

```
00 00 02 00 01 00 00
```

## Composition rule

Phase 019 MUST reuse:

- the Phase 016 response-status codec for bytes 0..1; and
- the Phase 018 status-body codec for bytes 2..6.

It MUST NOT create a second independent mapping for either component.

## Decoder requirements

The success decoder MUST fail closed when:

1. the common response-status prefix is missing;
2. the response-status identifier is unknown;
3. the response status is known but is not `Ok`;
4. the remaining body is not exactly five bytes;
5. the runtime-state identifier is unknown; or
6. the embedded local IPC protocol version is unsupported.

A known non-success status is not a successful status payload and MUST NOT be interpreted as if it carried a status snapshot.

## Scope boundary

Phase 019 does not define the outer IPC frame kind, request correlation, error-response body schema, live status collection, command dispatch, or socket runtime.

No dependency is added.

## Security / runtime exclusions

Phase 019 adds no:

- socket bind/listen/accept/connect operation;
- command execution;
- filesystem mutation;
- account authentication;
- privileged-helper invocation;
- DNS/network mutation;
- cryptographic private-key operation;
- systemd activation;
- database or deployment.

## Explicit deferrals

Still deferred:

- outer `Response`/`Error` frame-kind and response-status invariant;
- request-ID correlation at the composed frame level;
- error-response body schema;
- bounded private-DNS response body and codec;
- live runtime status collection;
- runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- timeout/cancellation policy;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
