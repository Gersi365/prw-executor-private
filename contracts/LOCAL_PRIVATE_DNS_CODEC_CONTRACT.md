# Local Private-DNS Snapshot Codec Contract

Status: Phase 027 locked baseline

## Purpose

Define a deterministic bounded binary representation for the Phase 026 `LocalPrivateDnsSnapshot`. This codec is local-IPC serialization only; it does not parse DNS semantics or mutate resolver state.

## Fixed header

Every encoded snapshot starts with exactly three bytes:

```
byte 0  flags
byte 1  resolver_count
byte 2  split_domain_count
```

### Flags

- bit 0 (`0x01`): `enabled`
- bit 1 (`0x02`): `device_naming`
- bits 2..7: reserved and MUST be zero

A decoder MUST reject any payload with a reserved flag bit set.

### Counts

Counts are unsigned one-byte values, but the stricter Phase 026 bounds remain authoritative:

- resolver_count <= 16
- split_domain_count <= 64

Counts above those bounds MUST be rejected before list allocation or entry decoding.

## Entry encoding

Entries are encoded in source order. Each resolver is emitted first, followed by each split domain.

Each entry uses:

```
length  u16 big-endian
value   exactly `length` UTF-8 bytes
```

The length MUST be non-zero and within the list-specific Phase 026 bound before any entry allocation:

- resolver entry <= 128 UTF-8 bytes
- split-domain entry <= 253 UTF-8 bytes

Declared bytes MUST be fully present and valid UTF-8.

## Maximum encoded length

With all Phase 026 bounds saturated:

- fixed header: 3 bytes
- 16 resolvers × (2-byte length + 128-byte value) = 2,080 bytes
- 64 split domains × (2-byte length + 253-byte value) = 16,320 bytes

Maximum encoded snapshot length is therefore exactly **18,403 bytes**.

This is well below the global 1 MiB local IPC payload limit.

## Decoder ordering

The decoder MUST:

1. require the three-byte fixed header;
2. reject reserved flag bits;
3. reject counts above Phase 026 bounds;
4. for each declared entry, require a complete two-byte length;
5. reject zero or over-bound length before allocating the string;
6. require all declared bytes;
7. validate UTF-8 before allocating an owned string;
8. reject trailing bytes after the declared lists;
9. perform a defensive final `LocalPrivateDnsSnapshot` invariant validation.

## Preservation rule

Encode/decode round trips MUST preserve:

- both booleans;
- list counts;
- exact UTF-8 string contents;
- resolver order;
- split-domain order.

The codec MUST NOT normalize, parse, sort, deduplicate, or infer DNS values.

## Runtime boundary

Phase 027 adds no socket runtime, command dispatch, DNS/network mutation, resolver lookup, account authentication, privileged-helper call, private-key operation, dependency, systemd activation, database, or deployment.

## Explicit deferrals

Still deferred:

- successful `GetPrivateDnsConfig` response-status payload composition;
- complete response frame and stream composition;
- resolver-address semantic parsing;
- split-domain normalization/validation policy;
- command-specific error body schema;
- runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- timeout/cancellation policy;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
