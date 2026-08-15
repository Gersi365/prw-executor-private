# Local Agent Status Body Codec Contract

Status: Phase 018 locked baseline

## Purpose

Define the exact byte representation of the minimal read-only Agent status snapshot introduced by Phase 017. This contract does not activate command dispatch or a local socket runtime.

## Exact body length

A successful `GetAgentStatus` command-specific response body is exactly 5 bytes.

```
byte 0      runtime_state        u8
bytes 1..2  protocol_major       u16, big-endian
bytes 3..4  protocol_minor       u16, big-endian
```

No padding, optional field, length prefix, text encoding, timestamp, hostname, PID, path, build metadata, DNS configuration, address, or extension byte is present in Phase 018.

## Runtime-state mapping

The first byte uses the Phase 017 mapping:

- `1` = `Starting`
- `2` = `Ready`
- `3` = `Degraded`
- `4` = `Stopping`

Every other value is invalid and MUST be rejected.

## Protocol-version encoding

`protocol_major` and `protocol_minor` preserve the existing `LocalIpcProtocolVersion` domain width. Both are unsigned 16-bit values encoded in network byte order.

The Phase 018 decoder accepts only the exact protocol version currently supported by the Agent build. A body naming another major or minor version MUST fail closed as `UnsupportedProtocolVersion`.

For the current version `1.0`, a `Ready` body is therefore:

```
02 00 01 00 00
```

## Decoder requirements

The decoder MUST:

1. reject any body whose length is not exactly five bytes;
2. reject an unknown runtime-state identifier;
3. parse major/minor as big-endian `u16` values;
4. reject any protocol version not exactly supported by the active build;
5. return a typed `LocalAgentStatusSnapshot` only after all checks pass.

The decoder MUST NOT ignore trailing bytes or silently truncate protocol-version fields.

## Encoder requirements

The encoder MUST emit exactly five bytes from a typed status snapshot and MUST preserve the existing `u16` major/minor representation.

## Security boundary

Phase 018 adds no:

- socket bind/listen/accept/connect operation;
- command dispatcher;
- filesystem mutation;
- serialization dependency;
- account authentication;
- privileged-helper invocation;
- DNS/network mutation;
- cryptographic private-key operation;
- systemd activation;
- database or deployment.

## Explicit deferrals

Still deferred:

- integration of this body with the response-status prefix and outer frame;
- private-DNS response-body schema and codec;
- live runtime status collection;
- command dispatch;
- Unix socket runtime and peer-credential enforcement;
- timeout/cancellation policy;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
