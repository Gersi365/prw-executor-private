# Phase 026 — Bounded Private-DNS IPC Snapshot

## Objective

Prepare the second read-only local command, `GetPrivateDnsConfig`, for a future wire codec without letting unbounded `Vec<String>` fields flow directly into the IPC protocol.

## Separate projection

The existing `prw-network::PrivateDnsConfig` remains unchanged and flexible. Phase 026 adds a local-Agent IPC projection that is created only after explicit bounds validation.

This separation prevents IPC constraints from silently becoming configuration-domain or DNS-runtime constraints.

## Locked product caps

The Phase 026 local IPC snapshot admits:

- at most 16 resolver strings;
- at most 64 split-domain strings;
- resolver strings from 1 to 128 UTF-8 bytes;
- split-domain strings from 1 to 253 UTF-8 bytes.

These are deliberate PRW IPC resource bounds, not assertions about DNS protocol maxima.

## Preservation behavior

On success, the snapshot copies the two booleans and both string lists exactly in source order. It does not normalize, parse, sort, deduplicate, or infer configuration.

This is important because DNS syntax, resolver-address semantics, IDNA handling, and normalization policy have not yet been locked.

## Validation behavior

Count bounds are checked before per-entry bounds. Empty strings and oversized entries are rejected. The source configuration is borrowed and never mutated.

## Tests

Focused tests cover:

- default disabled config projection;
- exact preservation of flags/list order;
- exact list-count bounds;
- above-count rejection;
- exact string-byte bounds;
- empty and overlong resolver/domain rejection.

## Runtime boundary

Phase 026 performs no DNS parsing, name resolution, OS DNS integration, networking, socket I/O, command dispatch, dependency addition, or service activation.

## Next bounded step

After validation, a byte codec can be designed around the now-bounded snapshot. Because list counts are <=64 and each entry is <=253 bytes, the codec can use explicit length prefixes with a deterministic worst-case payload size well below the global 1 MiB local IPC limit.
