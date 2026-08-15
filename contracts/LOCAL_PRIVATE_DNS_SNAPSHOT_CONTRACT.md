# Local Private-DNS Snapshot Contract

Status: Phase 026 locked baseline

## Purpose

Define a bounded, read-only local IPC projection for the existing `PrivateDnsConfig` domain type used by the `GetPrivateDnsConfig` command. Phase 026 introduces bounds only; it does not define byte serialization or mutate DNS/network state.

## Source-domain separation

`prw-network::PrivateDnsConfig` remains the flexible network-domain configuration type. Phase 026 MUST NOT change its fields or impose IPC limits directly on that domain model.

The local Agent creates a separate `LocalPrivateDnsSnapshot` only when the current config satisfies all Phase 026 IPC bounds.

## Product safety bounds

The following are PRW local-IPC product caps. They are not claims about normative DNS wire-protocol limits.

- maximum resolver entries: 16
- maximum split-domain entries: 64
- resolver string: 1 through 128 UTF-8 bytes
- split-domain string: 1 through 253 UTF-8 bytes

Empty resolver or split-domain strings are invalid for this IPC projection.

## Snapshot fields

A valid snapshot preserves exactly:

- `enabled: bool`
- `device_naming: bool`
- resolver strings in source order
- split-domain strings in source order

The projection MUST NOT silently normalize, sort, deduplicate, parse, replace, or invent values.

## Validation ordering

The projection MUST reject:

1. resolver count above 16;
2. split-domain count above 64;
3. any empty resolver string;
4. any resolver string above 128 UTF-8 bytes;
5. any empty split-domain string;
6. any split-domain string above 253 UTF-8 bytes.

The source `PrivateDnsConfig` MUST remain unchanged on both success and failure.

## Explicit non-goals

Phase 026 does not:

- parse resolver strings as IPv4/IPv6;
- validate DNS naming syntax or IDNA rules;
- normalize case, trailing dots, or resolver representations;
- define wire byte encoding;
- enable or disable private DNS;
- write resolver configuration to the operating system;
- alter routing or name resolution;
- add a dependency.

## Security and runtime boundary

Phase 026 adds no socket runtime, command dispatcher, filesystem/network/DNS mutation, account authentication, privileged-helper call, private-key operation, systemd activation, database, or deployment.

## Explicit deferrals

Still deferred:

- fixed byte representation for the bounded snapshot;
- resolver-address semantic validation;
- split-domain normalization/validation policy;
- successful `GetPrivateDnsConfig` payload/frame composition;
- command-specific error body schema;
- live runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- timeout/cancellation policy;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
