# Phase 027 — Bounded Private-DNS Snapshot Codec

## Objective

Give the bounded Phase 026 `LocalPrivateDnsSnapshot` a deterministic binary representation while preserving the existing provider-neutral and no-runtime-mutation boundary.

## Layout

The codec starts with a three-byte fixed header:

```
flags | resolver_count | split_domain_count
```

Each resolver string follows in source order as `u16 BE length + UTF-8 bytes`, followed by split-domain entries in the same format.

Only flag bits 0 (`enabled`) and 1 (`device_naming`) are defined. All other bits are rejected.

## Bounded allocation strategy

Counts are validated against Phase 026 limits before creating the output vectors. For each entry, the two-byte length is read and checked for non-zero/list-specific maximum before bytes are sliced or an owned `String` is allocated. UTF-8 is validated before ownership conversion.

This keeps malformed count/length fields from controlling unbounded allocation.

## Exact maximum size

The maximally populated Phase 026 snapshot encodes to exactly 18,403 bytes:

- 3 fixed bytes;
- 16 × (2 + 128) resolver bytes;
- 64 × (2 + 253) split-domain bytes.

This remains far below the 1 MiB local IPC payload bound.

## Fail-closed cases

The decoder rejects:

- truncated fixed header;
- reserved flags;
- excessive list counts;
- truncated entry length;
- zero-length entry;
- over-bound entry length;
- truncated entry bytes;
- invalid UTF-8;
- trailing bytes;
- defensive final snapshot-invariant failure.

## Semantic separation

The codec treats resolver and split-domain text as bounded UTF-8. It deliberately does not parse IP addresses, DNS labels, IDNA, case, trailing-dot semantics, or duplicate entries. Those policies remain independent architecture decisions.

## Validation focus

Tests cover stable default bytes, flags/list/UTF-8 round trip, exact maximum encoded length, invalid flags/counts, entry boundary failures, invalid UTF-8, truncation, and trailing bytes.

## Runtime boundary

This phase performs only in-memory encode/decode. It does not configure DNS, open sockets, dispatch commands, access peer credentials, add dependencies, or activate services.

## Next bounded step

After validation, the 18,403-byte maximum body can be composed with the existing two-byte `Ok` response prefix and Phase 022 terminal response builder to produce a complete successful `GetPrivateDnsConfig` response path without any new framing mechanism.
