# Phase 152 C03e-BD — Remote Endpoint Connectivity-Endpoint Projection Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Gate target:
`C03E_BD_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-BC:
- branch: `phase-152-c03e-bc-remote-endpoint-connectivity-endpoint-projection-selection-staging`;
- head: `874ae08f078ef619ca0cee3890113759e230be92`;
- tree: `5cfc866d74de6693824358969f000456f71cbed1`;
- gate: `C03E_BC_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SELECTED`.

BC selected one provider-neutral `prw-remote-bridge` semantic adapter from an already-observed `SocketAddr` to the existing validated `ConnectivityEndpoint`, with direct delegation to `ConnectivityEndpoint::new` and no candidate/publication/provider semantics.

## 2. Materialized source boundary

BD materializes exactly one public pure projection helper in:

`crates/prw-remote-bridge/src/candidate_reachability.rs`

The helper is:

```text
project_observed_socket_addr_to_connectivity_endpoint(
    observed: SocketAddr,
) -> Result<ConnectivityEndpoint, ConnectivityError>
```

Its implementation performs exactly:

```text
ConnectivityEndpoint::new(observed.ip(), observed.port())
```

No other production operation is performed.

## 3. Validation ownership preserved

BD does not reproduce endpoint validation. The existing `ConnectivityEndpoint::new` constructor remains the sole authority for endpoint-domain validation.

Therefore the materialized helper preserves the existing rejection classes for:
- port `0`;
- unspecified IPv4/IPv6;
- multicast IPv4/IPv6;
- IPv4 limited broadcast.

Successful projection preserves the exact observed IP address and port.

No fallback, rewrite, resolver lookup, interface selection, route inspection or public-address substitution exists in the source.

## 4. Layering preserved

BD changes no manifest or crate root.

`prw-agent` still has no direct `prw-connectivity` dependency.

`prw-remote-bridge` already depends on `prw-connectivity`, and `candidate_reachability` was already a public module before BD. The new helper therefore stays inside the previously selected ownership boundary without dependency widening.

## 5. Focused non-networking proof

BD adds pure focused tests in the same module.

The tests prove:
- the exact selected function-pointer shape;
- exact IPv4 address/port pass-through;
- exact IPv6 address/port pass-through;
- existing zero-port rejection;
- existing unspecified-address rejection for IPv4 and IPv6;
- existing multicast-address rejection for IPv4 and IPv6;
- existing IPv4 limited-broadcast rejection.

The tests perform no socket bind, socket connect, DNS, interface discovery, route lookup, NAT traversal, provider call, publication, credential read, task spawn or runtime activation.

## 6. Candidate semantics remain absent

BD does not construct `ConnectivityCandidate` and does not choose or mutate:
- `CandidateId`;
- candidate-ID allocation/high-water custody;
- `ConnectivityPathKind`;
- priority/ranking;
- candidate-set replacement;
- stale endpoint replacement.

The projection output remains only a validated `ConnectivityEndpoint`.

## 7. Publication/currentness remain absent

BD does not construct `AuthenticatedCandidatePublication` and does not call existing publication/freshness/reachability-owner operations.

No requester/publisher provenance, transport-currentness, freshness token, replay/fencing, provider CAS, etcd mutation or discovery advertisement is added.

## 8. Identity/security invariants unchanged

- `DeviceId` / authenticated PRW session identity remains logical identity.
- `TransportIdentity` remains lower-transport certificate identity only.
- `SocketAddr` and `ConnectivityEndpoint` remain transient network endpoint/configuration state only.
- `SessionId` remains authentication correlation only.
- projection success is not authentication, authorization, readiness, currentness, public routability or publication evidence.
- protected operations continue to rely on existing fresh-current registry/transport/policy evaluation.

## 9. Exact source scope

The intended final BC -> BD scope is exactly two paths:
1. `crates/prw-remote-bridge/src/candidate_reachability.rs`;
2. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BD_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SOURCE_MATERIALIZATION_STAGING.md`.

No Cargo manifest, lockfile, crate root, Agent source, connectivity-core source, provider owner, freshness module, Android source, workflow, readiness/status, systemd, host or deployment file is authorized to change.

## 10. Explicit exclusions

BD does not authorize or materialize:
- Agent consumption/wiring of the projection helper;
- production bind-address source or interface policy;
- direct Agent dependency on `prw-connectivity`;
- connectivity candidate creation;
- candidate-ID/path/priority policy;
- authenticated candidate publication;
- reachability provider mutation;
- NAT/STUN/ICE/TURN/relay activation;
- executable `main.rs` remote-lane invocation;
- expected-device producer/discovery;
- concrete production capability dispatcher;
- readiness/status change;
- remote failure -> local fail-stop/process-exit policy;
- retry/backoff/reconnect/rebootstrap/rebind/replacement;
- second Tokio runtime or generic runtime/handle exposure;
- systemd unit/drop-in mutation;
- host firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF/R1-R4 activation;
- merge.

## 11. Closure condition

BD can close only after:
- the exact BC predecessor remains unchanged;
- the final BC -> BD diff remains within the two authorized paths;
- canonical Rust and any other automatically applicable validation for the exact final BD head reach terminal verdicts;
- any validation failure is classified as source defect versus environment/tooling failure before correction;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive evidence is freshly guarded, appended in place with exact predecessor-prefix preservation, and raw-readback verified;
- PR body moves from `Status: STAGED` to `Status: CLOSED` only after Drive verification;
- the PR remains draft/open/unmerged;
- final GitHub race checks remain clean.

No merge, deployment or host mutation is part of BD closure.

Gate target remains:

`C03E_BD_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SOURCE_MATERIALIZED`
