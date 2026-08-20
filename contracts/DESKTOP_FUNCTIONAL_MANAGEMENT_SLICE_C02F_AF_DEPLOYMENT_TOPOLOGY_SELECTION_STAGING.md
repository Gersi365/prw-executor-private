# Phase 152 C02f-AF — Deployment Topology Selection Contract

Status: `ARCHITECTURE_SELECTION_STAGED / THREE_VOTER_SELECTED / THREE_FAILURE_DOMAINS / SINGLE_REGION / PLATFORM_NEUTRAL / STABLE_MEMBER_FQDN_CONSTRAINT / DOCS_ONLY / NO_PRODUCTION_ENDPOINT / NO_TLS_AUTH_RBAC / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative predecessor validation head: `30cfb1cf04c070e37e15e1b2c9a187dbfdbafc77`
Predecessor PR: `#50` (`open / draft / unmerged`)
Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This contract advances the first post-AE production-architecture selection boundary identified by the post-AE readiness audit: deployment topology, failure domains, and endpoint-naming constraints.

It is deliberately platform-neutral. It does not choose a cloud/provider, geographic region, concrete DNS zone, certificate authority, credentials, runtime bootstrap owner, or production endpoint values.

## Selected topology

The selected direction for initial production architecture is:

1. exactly three voting etcd members for the initial authority cluster;
2. quorum size two, tolerating loss of one voter / one independent failure domain while preserving authority availability;
3. one voter per independent low-latency availability-zone-equivalent failure domain;
4. all three voters in one region / one low-latency metro-scale consensus locality;
5. no cross-region voting in the initial topology;
6. no one-voter or two-voter production topology;
7. five voters remain a separate later SLO-driven escalation only if continuous authority through two simultaneous independent voter failures is explicitly required;
8. majority loss remains fail closed and transitions to disaster-recovery authority, never to a local fallback authority.

## Endpoint naming constraints

The production endpoint identity model is constrained as follows without selecting concrete names yet:

- each etcd member has one stable member FQDN used as its network identity;
- client and peer traffic remain distinct endpoint roles, even when they share the same member FQDN and differ only by port;
- advertised client and peer endpoints must be remotely reachable from their intended callers/peers;
- `localhost`, loopback-only identities, and wildcard identities such as `0.0.0.0` are prohibited as advertised production endpoints;
- authoritative production configuration must not depend on ephemeral pod/container IPs as member identity;
- concrete DNS zone, member labels, ports, HTTPS scheme, certificate SANs, service-discovery mechanism, and load-balancing exposure remain deferred until platform and TLS/PKI selection.

## Failure-domain constraints

A claimed three-domain topology is valid only when the three voters do not share a single failure boundary that would defeat quorum tolerance. The later platform binding must demonstrate independent power/host/rack-or-zone-equivalent placement and low-latency inter-member networking.

Fast durable storage, predictable fsync latency, resource isolation, and reliable low-latency peer networking remain mandatory operational prerequisites for the advertised quorum tolerance to be meaningful.

## etcd v3.7 guidance alignment

The reviewed official etcd v3.7 guidance supports this selection boundary:

- odd membership is recommended and a three-member cluster has majority two / failure tolerance one;
- a five-member cluster raises failure tolerance to two at additional replication cost;
- cross-region / cross-datacenter voting increases consensus latency and may require timeout tuning;
- low-latency, reliable networking and fast storage are important to cluster stability;
- advertised client/peer addresses must be reachable and production should not advertise `localhost` or `0.0.0.0`.

These checks support the selected three-voter / three-low-latency-failure-domain / single-region direction and the endpoint-naming constraints above.

## Explicitly deferred

This contract does not select or authorize:

- cloud/provider or bare-metal platform;
- concrete region or availability zones;
- concrete DNS zone/member FQDN values;
- concrete client/peer ports or production endpoint values;
- TLS feature activation, private CA, mTLS identities, certificate SANs, rotation, secret storage, etcd auth/RBAC, or credentials;
- recovery epoch/high-water layout or immutable ledger provider;
- first-production bootstrap/runtime ownership;
- R1-R4 stale-side-effect fencing implementation;
- production deployment, merge, retargeting, or runtime activation.

## Next dependency

After this topology selection is validated and frozen, the next dependent architecture review is TLS / PKI / etcd authentication + RBAC, because certificate identities, SANs, credential custody and endpoint scheme depend on the selected endpoint/failure-domain model.

## Authorization boundary

`C02F_AF_TOPOLOGY_SELECTION_ONLY / NO_TLS_SELECTION / NO_RECOVERY_SELECTION / NO_RUNTIME_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Any TLS/PKI/auth/RBAC selection, recovery/high-water selection, concrete platform/region binding, endpoint materialization, runtime activation, deployment, retargeting, or merge requires separate explicit authorization.
