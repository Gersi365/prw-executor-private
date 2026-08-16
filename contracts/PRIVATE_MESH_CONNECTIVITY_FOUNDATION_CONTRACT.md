# Private Remote Workspace Private Mesh Connectivity Foundation Contract

Version: `0.1.0`

Status: Phase 135 implementation lock

## Purpose

Phase 135 establishes the provider-neutral candidate, observation and deterministic path-selection model for private remote connectivity.

It does **not** create sockets, perform STUN/ICE traffic, configure WireGuard/TUN, assign operating-system addresses, mutate routes/firewalls, open listeners, contact a relay, alter DNS or activate production networking.

## Product path order

For a peer that has usable observed candidates, PRW selects in this fixed order:

1. `LocalDirect`;
2. `InternetDirect`;
3. `Relay`;
4. `Offline` when no candidate is currently observed reachable.

A reachable direct candidate must not be displaced by a reachable relay candidate.

Phase 136 owns the relay transport foundation. Phase 135 may represent relay candidates so selection/fallback semantics are architecture-complete, but it does not relay bytes.

## Identity separation

Logical device identity and transport/network identity are distinct.

A `PeerConnectivityIdentity` contains:

- one logical `DeviceId`;
- one opaque `TransportIdentity` represented as exactly 32 bytes.

The all-zero transport identity is rejected. Phase 135 neither derives nor signs this identifier and introduces no cryptographic primitive.

Changing transport identity must not imply changing the logical device identifier.

## Endpoint model

A candidate endpoint consists only of:

- explicit `IpAddr`;
- explicit non-zero port.

No hostname or resolver input exists in the Phase 135 selector. Basic path selection therefore has no DNS dependency.

Rejected endpoint addresses:

- unspecified;
- multicast;
- IPv4 limited broadcast.

Loopback endpoints are permitted for disposable/local validation. Endpoint scope classification is owned by discovery/provider logic; the selector does not infer topology from textual hostnames.

## Candidate model

Each `ConnectivityCandidate` has:

- non-zero broker-scoped `CandidateId`;
- one `ConnectivityPathKind` (`LocalDirect`, `InternetDirect`, `Relay`);
- one validated explicit endpoint.

A peer plan accepts at most 16 candidates.

Duplicate candidate identifiers are rejected. Duplicate exact `(path kind, endpoint)` candidates are rejected.

The plan stores no private keys, user credentials or application payload.

## Observations

Phase 135 does not perform network probes. It accepts typed provider observations:

- `Unknown`;
- `Reachable`;
- `Unreachable`.

Only `Reachable` candidates are selectable.

Observation updates require an existing candidate identifier. Unknown identifiers fail closed.

## Deterministic selection

Selection is deterministic and independent of candidate insertion order.

Within the same path kind, the reachable candidate with the numerically lowest `CandidateId` wins.

The returned `SelectedConnectivityPath` contains the selected candidate identity/kind/endpoint or `Offline`.

No selection result is itself an authorization grant for terminal, files or port forwarding.

## Required tests

Tests must prove at least:

- zero `CandidateId` rejected;
- all-zero `TransportIdentity` rejected;
- invalid endpoint classes/zero port rejected;
- candidate count bound of 16;
- duplicate candidate IDs rejected;
- duplicate exact kind+endpoint rejected;
- unknown observation target rejected;
- unknown/unreachable candidates are not selected;
- reachable `LocalDirect` beats reachable `InternetDirect` and `Relay`;
- reachable `InternetDirect` beats reachable `Relay` when local is unavailable;
- relay is selected only when no direct candidate is reachable;
- no reachable candidate produces `Offline`;
- insertion order does not change selection;
- same-kind tie breaks by lowest `CandidateId`;
- logical `DeviceId` and transport identity remain independently observable;
- no API accepts a DNS name, raw command, TUN/interface instruction, firewall rule or private key.

## Explicitly deferred

- real host/network candidate discovery;
- STUN requests;
- ICE connectivity checks;
- NAT traversal packets;
- UDP/TCP dialing;
- WireGuard/userspace tunnel;
- TUN interface creation;
- private address assignment;
- operating-system route/firewall mutation;
- production endpoint publication;
- keepalive/path-health scheduler;
- migration/failover timing;
- relay byte transport (Phase 136);
- optional private DNS integration (Phase 137).

## Production boundary

Phase 135 source/disposable work may proceed under the user's authorization through Phase 137.

No PowerCode production private networking, route, firewall, socket, tunnel, endpoint publication or user-impacting connectivity change is authorized by this contract.
