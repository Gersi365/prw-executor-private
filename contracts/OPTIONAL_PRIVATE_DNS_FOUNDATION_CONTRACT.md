# Private Remote Workspace Optional Private DNS Foundation Contract

Version: `0.1.0`

Status: Phase 137 implementation lock

## Purpose

Phase 137 establishes typed, bounded and reversible configuration primitives for optional PRW private DNS.

It does **not** modify `/etc/resolv.conf`, call `resolvectl`, install a resolver, open DNS sockets, publish DNS records, alter NetworkManager/systemd-resolved, mutate split-DNS routing or activate production DNS.

## Non-dependency rule

Basic PRW connectivity must not depend on private DNS.

`prw-connectivity` remains independent from `prw-private-dns` and continues to select explicit-IP candidates with no hostname/resolver dependency.

Disabling private DNS must not disable or invalidate direct/relay connectivity state.

## Mode

`PrivateDnsMode` is:

- `Disabled`;
- `Enabled`.

Default configuration is `Disabled`.

A disabled configuration may retain valid resolver/domain/device-naming settings so the feature can be toggled without destroying user configuration. Retained settings have no effect until an external audited integration layer applies an enabled configuration.

## Device naming

Optional device naming is represented by:

- `device_naming: bool`;
- one canonical `DeviceDnsLabel` per future record;
- one configured device-domain suffix when device naming is enabled.

Phase 137 config requires a device-domain suffix when `device_naming` is true.

`DeviceDnsLabel` rules:

- 1 through 63 bytes;
- lowercase ASCII letters, digits and hyphen only;
- first and last character must be alphanumeric;
- uppercase, underscore, whitespace, dot and non-ASCII are rejected.

Phase 137 does not create device records or allocate network addresses.

## Domain suffix validation

`DnsDomainSuffix` is canonical lower-case ASCII without a trailing dot.

Bounds/rules:

- total length 1 through 253 bytes;
- one or more dot-separated labels;
- each label 1 through 63 bytes;
- labels use lowercase ASCII letters, digits and hyphen;
- label first/last character must be alphanumeric;
- empty labels, leading/trailing dot, uppercase, underscore, whitespace and non-ASCII are rejected.

## Custom resolver endpoints

One resolver endpoint contains:

- explicit `IpAddr`;
- explicit non-zero port.

No resolver hostname is accepted.

Rejected addresses:

- unspecified;
- multicast;
- IPv4 limited broadcast.

Loopback resolver addresses are allowed for local resolver implementations.

Initial bound: at most 4 resolver endpoints.

Duplicate resolver endpoints are rejected.

## Split DNS

Initial bound: at most 16 split-domain suffixes.

Duplicate split-domain suffixes are rejected.

A configuration containing split-domain suffixes requires at least one explicit resolver endpoint. This avoids a configuration that claims split routing without a destination resolver.

## Configuration boundary

`PrivateDnsConfig` stores only validated values:

- mode;
- device naming flag;
- optional device-domain suffix;
- bounded resolver endpoints;
- bounded split-domain suffixes.

The config object has no method that mutates the operating system, performs DNS queries, opens sockets or changes connectivity candidates.

## Required tests

Tests must prove at least:

- default mode is `Disabled`;
- invalid/uppercase/empty/oversized device labels rejected;
- valid canonical device labels accepted;
- invalid domain syntax/length rejected;
- valid canonical split/device suffixes accepted;
- resolver zero port/invalid address classes rejected;
- resolver count bound of 4;
- split-domain count bound of 16;
- duplicate resolver/split-domain values rejected;
- device naming requires a device-domain suffix;
- split domains require at least one resolver;
- disabled mode can retain valid settings while remaining inactive;
- enabled mode reports active without performing any OS mutation;
- `prw-connectivity` has no dependency on `prw-private-dns`;
- no API accepts a resolver hostname, shell command or OS resolver mutation instruction.

## Explicitly deferred

- DNS query/response implementation;
- device-name record catalog;
- private address to name publication;
- DNSSEC policy;
- search-domain behavior;
- systemd-resolved/NetworkManager integration;
- Android private-DNS integration;
- OS split-DNS mutation;
- production rollback/restore transaction;
- production activation.

## Production boundary

Phase 137 source/disposable work may proceed under the user's authorization through Phase 137.

No PowerCode resolver setting, OS DNS state, DNS listener, DNS query path or user-impacting production networking state is authorized by this contract.

Completion of Phase 137 ends the currently authorized source/disposable roadmap. Any subsequent phase or production activation requires a new explicit scope/authorization as applicable.
