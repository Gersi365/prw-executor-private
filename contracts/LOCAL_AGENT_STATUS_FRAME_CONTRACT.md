# Complete Local Agent Status Response Frame Contract

Status: Phase 023 locked baseline

## Purpose

Compose the validated Phase 018 Agent-status body codec with the generic Phase 022 terminal-response frame builder to create and decode a complete successful `GetAgentStatus` in-memory response frame.

## Build inputs

The successful status-frame builder accepts only:

- a typed non-zero `LocalIpcRequestId`; and
- a typed `LocalAgentStatusSnapshot`.

The caller does not provide message kind, response status, payload length, or protocol version.

## Build composition

The builder MUST:

1. encode the snapshot using the Phase 018 five-byte status-body codec;
2. call the Phase 022 terminal-response builder with status `Ok`;
3. allow Phase 022 to prepend the Phase 016 status prefix and derive outer kind `Response`;
4. rely on existing payload/header/frame constructors for lower-level invariants.

The resulting frame payload is exactly the seven-byte Phase 019 successful status payload.

For `Ready` on protocol `1.0`:

```
00 00 02 00 01 00 00
```

The header payload length is exactly `7`.

## Decode ordering

The successful status-frame decoder MUST:

1. validate the complete frame through the Phase 020 terminal-response validator;
2. reject a valid terminal frame whose status is non-`Ok`;
3. remove only the already validated two-byte common status prefix from interpretation;
4. decode the remaining bytes through the Phase 018 status-body decoder;
5. return the existing request ID and typed status snapshot only after all checks pass.

## Error separation

The decoder preserves separate categories for:

- terminal-frame invariant failure;
- valid terminal non-success status; and
- invalid command-specific status body.

## Scope boundary

Phase 023 does not mutate the Phase 013 request tracker. Completion remains the separate Phase 021 state transition.

Phase 023 also adds no socket runtime, stream I/O, command dispatch, live state collection, dependency, authentication, network mutation, privileged-helper invocation, crypto operation, systemd activation, database, or deployment.

## Explicit deferrals

Still deferred:

- byte-level writer/reader round-trip of the complete status frame;
- composition with Phase 021 outstanding-request completion;
- command-specific error body schema;
- bounded private-DNS response body and codec;
- timeout/cancellation policy;
- live runtime status collection;
- runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
