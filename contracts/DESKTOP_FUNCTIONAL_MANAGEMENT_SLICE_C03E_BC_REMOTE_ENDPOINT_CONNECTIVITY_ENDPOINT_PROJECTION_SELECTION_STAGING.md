# Phase 152 C03e-BC — Remote Endpoint Connectivity-Endpoint Projection Selection

Status: STAGED SELECTION ONLY

Gate target:
`C03E_BC_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SELECTED`

## 1. Exact predecessor

Closed C03e-BB:
- branch: `phase-152-c03e-bb-remote-endpoint-bound-address-observation-source-materialization-staging`;
- head: `3ac2413583ccd9ec378b59ec61b23c6a13237b0e`;
- tree: `3aae115f77db010baaa9e780250c79a2e2f2b4bf`;
- gate: `C03E_BB_REMOTE_ENDPOINT_BOUND_ADDRESS_OBSERVATION_SOURCE_MATERIALIZED`.

BB exposes only the exact read-only local `SocketAddr` observed from the already-bound remote endpoint through `RemoteSessionEndpointLifecycleRuntime::bound_addr(&self)` and maps lower observation failure to the bounded Agent error `RemoteSessionEndpointBoundAddressError::Unavailable`.

BB does not create a connectivity endpoint, connectivity candidate, path class, candidate identifier, priority, publication, provider mutation, readiness claim or production networking policy.

## 2. Audit basis on the exact BB tree

The first missing semantic seam after BB is before candidate publication/provider mutation.

Exact inherited source establishes all of the following:

1. `prw-agent` does not directly depend on `prw-connectivity` in its manifest.
2. `prw-remote-bridge` already depends on `prw-connectivity` and publicly exposes the existing dynamic-reachability modules through its production crate root.
3. `prw-connectivity::ConnectivityEndpoint::new(address, port)` is the existing strict endpoint constructor. It rejects zero ports, unspecified addresses, multicast addresses and IPv4 limited broadcast.
4. `ConnectivityCandidate` requires already-typed `CandidateId`, `ConnectivityPathKind` and `ConnectivityEndpoint` values.
5. `candidate_reachability::AuthenticatedCandidatePublication` already requires a `Vec<ConnectivityCandidate>`; it accepts neither `SocketAddr` nor a raw endpoint projection input.
6. `candidate_reachability` contains no existing `SocketAddr -> ConnectivityEndpoint` helper and does not call `ConnectivityEndpoint::new` for callers.
7. Existing reachability-owner/publication machinery is downstream of already-formed authenticated candidate publications and therefore is not the first missing seam after BB.

Consequently, provider publication, candidate-envelope creation and Agent dependency widening are not selected by this checkpoint.

## 3. Selected semantic boundary

C03e-BC selects exactly one provider-neutral remote-bridge semantic adapter that projects one already-observed `SocketAddr` into the existing validated `ConnectivityEndpoint` domain type.

The selected semantic operation is equivalent to:

```text
observed SocketAddr
    -> address = observed.ip()
    -> port = observed.port()
    -> ConnectivityEndpoint::new(address, port)
    -> exact successful ConnectivityEndpoint OR existing ConnectivityError
```

The input must represent an observation from the already-bound endpoint boundary selected/materialized by BA/BB. The adapter itself does not bind or inspect a socket and cannot establish that provenance on its own.

The exact existing `ConnectivityEndpoint::new` validation remains authoritative. BC does not duplicate, relax, bypass or reinterpret endpoint validation.

## 4. Ownership and layering selection

The projection seam is selected as `prw-remote-bridge`-owned, adjacent to the existing candidate-reachability semantics.

This preserves current layering:
- Agent does not gain a direct `prw-connectivity` manifest dependency merely for endpoint projection;
- `prw-remote-bridge` already owns the bridge between authenticated remote reachability semantics and `prw-connectivity` domain values;
- no crate-root widening is required because `candidate_reachability` is already a public module;
- no provider implementation, etcd wire format or network-runtime ownership is moved.

The future materialized seam may expose the existing `ConnectivityEndpoint`/`ConnectivityError` types through its public function signature because those types remain owned by `prw-connectivity`; BC does not define replacement wrapper types or a second endpoint-validation taxonomy.

## 5. Failure semantics

Projection fails exactly when the existing `ConnectivityEndpoint::new` constructor rejects the observed address/port.

Selected failure behavior:
- port `0` remains invalid;
- unspecified IPv4/IPv6 remains invalid;
- multicast IPv4/IPv6 remains invalid;
- IPv4 limited broadcast remains invalid;
- successful values preserve the exact IP address and port;
- no invalid observed address is rewritten to loopback, a host interface, a discovered public address or any fallback value.

An observed wildcard bind address therefore does not become externally publishable merely because an endpoint is bound. A separately gated bind-source/interface/discovery decision is required if production configuration must yield an explicit publishable address.

BC adds no retry, interface enumeration, resolver lookup, route inspection, NAT discovery, STUN, ICE, TURN, relay substitution or provider lookup.

## 6. Explicit non-selection of candidate semantics

BC does not select or materialize creation of `ConnectivityCandidate`.

In particular it does not choose:
- a `CandidateId` or candidate-ID allocator/custodian;
- `ConnectivityPathKind::LocalDirect`;
- `ConnectivityPathKind::InternetDirect`;
- `ConnectivityPathKind::Relay`;
- path-class inference from an IP address;
- priority/ranking semantics;
- candidate-set replacement semantics;
- removed-ID non-reuse/high-water custody;
- stale endpoint replacement policy.

A `SocketAddr` or `ConnectivityEndpoint` alone is insufficient evidence for any of those decisions.

## 7. Explicit non-selection of publication/currentness

BC does not construct `AuthenticatedCandidatePublication` and does not call any publication/refresh/provider owner.

It therefore does not select:
- publisher/requester identity provenance;
- transport-identity currentness;
- publication freshness tokens;
- replay/fencing semantics;
- candidate-vector transactional refresh;
- durable provider CAS;
- etcd/provider mutation;
- discovery advertisement;
- externally routable reachability claims.

The existing candidate-publication and reachability-owner layers remain byte-stable unless a later separately gated checkpoint authorizes their use or identifies a concrete defect.

## 8. Identity and authority invariants

Unchanged:
- `DeviceId` / authenticated PRW session identity is logical identity;
- `TransportIdentity` is lower-transport certificate identity only;
- `SocketAddr` and `ConnectivityEndpoint` are transient network endpoint/configuration state only;
- `SessionId` is authentication correlation only;
- runtime/task/thread/controller/channel/lock/endpoint/candidate identifiers are not PRW identity or authentication evidence;
- reachability-authority possession is an admission prerequisite, not user identity;
- endpoint bind or endpoint projection success is not authentication, authorization, readiness, currentness, public routability or publication evidence;
- protected requests remain dependent on existing fresh-current registry/transport/policy evaluation.

## 9. Activation and production exclusions

C03e-BC does not authorize or materialize:
- Rust/source implementation of the selected projection;
- Agent `main.rs` or executable remote-lane invocation;
- production bind-address source or wildcard/interface policy;
- direct Agent dependency on `prw-connectivity`;
- connectivity candidate creation;
- candidate publication/provider mutation;
- NAT/STUN/ICE/TURN/relay discovery or activation;
- expected-device producer/discovery;
- concrete production capability dispatcher;
- registry persistence/population/watch;
- policy loading/mutation;
- account authentication/enrollment mutation;
- readiness/status changes;
- remote failure -> local fail-stop or process-exit policy;
- retry/backoff/reconnect/rebootstrap/rebind/replacement;
- second Tokio runtime or generic runtime/handle exposure;
- systemd unit/drop-in mutation;
- host firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF/R1-R4 activation;
- merge.

## 10. Selected future C03e-BD materialization scope

If BC closes without contradiction, the immediately following source-materialization checkpoint is bounded to exactly:

1. `crates/prw-remote-bridge/src/candidate_reachability.rs`;
2. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BD_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SOURCE_MATERIALIZATION_STAGING.md`.

No manifest, crate root, Agent source, provider owner, candidate-freshness module or connectivity-core source mutation is selected for BD.

Focused BD tests must be pure/non-networking and prove:
- exact IPv4 address/port pass-through;
- exact IPv6 address/port pass-through;
- existing zero-port rejection;
- existing unspecified-address rejection;
- existing multicast rejection;
- existing IPv4 limited-broadcast rejection;
- no fallback/rewrite/retry/secondary operation;
- no candidate/publication/provider side effect.

The implementation should delegate directly to the existing `ConnectivityEndpoint::new` constructor rather than reproduce its validation logic.

## 11. Closure condition

C03e-BC can close only after:
- the exact BB predecessor remains unchanged;
- the BC diff remains docs-only and contains exactly this selection contract;
- canonical validation for the exact final BC head reaches its applicable terminal verdict;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive evidence is freshly guarded and appended with exact predecessor-prefix preservation;
- PR body moves from `Status: STAGED` to `Status: CLOSED` only after Drive verification;
- the PR remains draft/open/unmerged;
- final GitHub race checks remain clean.

Gate target remains:

`C03E_BC_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SELECTED`
