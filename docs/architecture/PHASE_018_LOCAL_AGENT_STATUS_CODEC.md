# Phase 018 — Local Agent Status Body Codec

## Objective

Turn the Phase 017 minimal typed Agent status snapshot into a deterministic fixed-width byte body without introducing a general serializer or any runtime socket behavior.

## Design correction before implementation

The initial candidate considered one-byte protocol-version components. Repository inspection showed that `LocalIpcProtocolVersion` already stores both major and minor as `u16`. Phase 018 therefore preserves that established width instead of narrowing or truncating it.

The locked body is five bytes:

```
+0      runtime state   u8
+1..2   protocol major  u16 BE
+3..4   protocol minor  u16 BE
```

Current `Ready` / protocol `1.0` bytes:

```
02 00 01 00 00
```

## Why fixed width

The status body contains only two bounded concepts: one lifecycle state and one protocol-version tuple. A fixed-width representation provides deterministic parsing, no allocation, no text normalization, no schema dependency, and a clear fail-closed length rule.

## Decoder behavior

The decoder validates in this order:

1. exact body length;
2. known runtime-state identifier;
3. big-endian protocol major/minor extraction;
4. exact current-version support.

Only then is a typed `LocalAgentStatusSnapshot` returned.

Trailing bytes are rejected rather than ignored. Unknown states and unsupported protocol versions are distinct failures.

## Compatibility policy

Phase 018 intentionally accepts only the exact currently supported protocol version. A later compatibility policy may widen this behavior, but that must be an explicit protocol decision rather than an accidental decoder side effect.

## Runtime boundary

This phase remains pure in-memory codec work. It does not:

- collect live Agent state;
- dispatch `GetAgentStatus`;
- prepend the Phase 016 response-status prefix;
- construct an outer IPC frame;
- open a Unix socket;
- read peer credentials;
- add a dependency;
- activate a service.

## Next bounded step

After validation, the narrow next step is to compose the Phase 016 response-status prefix with the Phase 018 status body into a typed successful `GetAgentStatus` response payload. That composition can remain pure memory code and does not require runtime activation.
