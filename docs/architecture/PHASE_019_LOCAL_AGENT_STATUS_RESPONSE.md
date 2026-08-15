# Phase 019 — Successful Agent Status Response Payload

## Objective

Compose two already validated protocol components without introducing a new serializer:

1. Phase 016 common response-status prefix; and
2. Phase 018 fixed-width Agent status body.

The result is the command-specific successful `GetAgentStatus` payload.

## Locked layout

The successful payload is seven bytes:

```
00 00 | SS | MM MM | NN NN
```

Where:

- `00 00` is the `Ok` response status;
- `SS` is the Phase 017 runtime-state identifier;
- `MM MM` is the current protocol major as big-endian `u16`;
- `NN NN` is the current protocol minor as big-endian `u16`.

For `Ready` on protocol `1.0`:

```
00 00 02 00 01 00 00
```

## Reuse rather than duplication

The implementation delegates prefix decoding to the Phase 016 codec and body decoding to the Phase 018 codec. This keeps status identifiers, state identifiers, version width, endian rules, and error checks authoritative in one place each.

## Non-success handling

This module is deliberately success-specific. A known status such as `InvalidRequest`, `Unauthorized`, or `InternalError` is rejected as `NonSuccessStatus`; it is not followed into the status-body decoder. Error payload schemas remain a separate future boundary.

## Runtime boundary

Phase 019 is pure memory composition. It does not:

- create an outer IPC frame;
- assign or validate a request ID;
- dispatch a command;
- collect live Agent state;
- open a Unix socket;
- access peer credentials;
- allocate a general serialization layer;
- add a dependency.

## Next bounded step

After validation, the narrow next protocol step is an outer response-frame invariant that couples message kind, request ID, payload length, and success/error status semantics. This can still be implemented and tested entirely in memory before any socket runtime is activated.
