# Phase 152 C03e-JG — Production Peer Logical Device Source Gap Selection

Status: **SELECTION ONLY — SOURCE MATERIALIZATION BLOCKED**

Target gate:

`C03E_JG_PRODUCTION_PEER_LOGICAL_DEVICE_SOURCE_GAP_SELECTED`

## 1. Exact predecessor

C03e-JG starts only from the exact closed C03e-JF head:

`2cfb0bccbbe9791be260ff9e1caefaafb26adb02`

C03e-JF materialized one narrow operation-specific durable-registry runtime-custody lookup:

`DeviceId -> authoritative current same-device TransportIdentity -> PeerConnectivityIdentity`

The JF method accepts exactly one already-typed logical `DeviceId`, calls the private durable semantic store's current-transport lookup exactly once, propagates the existing provider-neutral durable-registry error unchanged, and constructs `PeerConnectivityIdentity::new(device_id, current_transport)` only after success.

JF deliberately did **not** select where the production executable obtains the logical `DeviceId` supplied to that lookup.

## 2. Scope of this checkpoint

C03e-JG selects only the remaining production logical-peer-device source gap.

It does not select a concrete source, materialize Rust/Kotlin/Cargo/lockfile/workflow/systemd/runtime source, call the durable registry, bootstrap reachability, populate a production peer field, invoke requester/rendezvous behavior, activate startup/readiness/networking, provision security/provider state, deploy, merge, close a pull request, mark a pull request ready, or delete a branch.

## 3. Retained C03e-IN peer provenance law

Closed C03e-IN remains authoritative for production `PeerConnectivityIdentity` provenance.

A production peer must represent one authoritative current same-device binding:

- one logical `DeviceId`; and
- that exact same device's current `TransportIdentity`.

The following remain non-authoritative substitutes:

- bind address, IP or port;
- connectivity candidates/endpoints/paths;
- DNS/public-IP observation;
- PRWM `request_id`;
- `SessionId`;
- expected-device scheduling intent **alone**;
- requester identity **alone**;
- requester/rendezvous target `DeviceId` **alone**;
- transport identity **alone**;
- environment literals;
- disposable/test identities.

Unknown, revoked, unbound, stale, mismatched, ambiguous or unavailable current authority must fail closed with no fabricated fallback peer.

C03e-JF now satisfies the previously missing current same-device transport lookup half of this law **when an exact logical `DeviceId` has already been selected**.

C03e-JF does not itself make any candidate logical-device source authoritative.

## 4. Existing production process-operation peer slot

The current Agent source retains the C03e-IG production input owner:

`LinuxAgentProductionReachabilityRemoteProcessOperationInputs<...>`

It contains exactly:

- `peer: PeerConnectivityIdentity`; and
- existing `LinuxAgentRemoteProcessOperationInputs<...>`.

The production process operation consumes that exact peer before constructing the reachability/bootstrap/endpoint lifecycle.

Therefore the remaining executable population problem is not a missing `PeerConnectivityIdentity` type or a missing current-transport lookup. It is the missing pre-bootstrap source of the logical `DeviceId` that JF must resolve.

## 5. Reachability bootstrap ordering proves the source must be pre-bootstrap

The current production reachability bootstrap has the exact shape equivalent to:

`bootstrap_production_reachability(config, peer: &PeerConnectivityIdentity)`

The supplied peer is used during authoritative durable-owner recovery before the production live authority and endpoint lifecycle are composed.

Consequently, the logical `DeviceId` used to construct that peer must be available and authoritative **before**:

- reachability durable-owner recovery;
- production endpoint bind;
- repeated remote-session admission;
- requester/rendezvous runtime handling.

A source that exists only after those stages would form an invalid temporal/custody dependency for this process bootstrap peer.

## 6. Requester/rendezvous raw target intent is not selected as the process peer source

The current crate-private `RequesterRendezvousTargetIntent` and `RequesterRendezvousStartIntent` carry a requester-nominated logical target `DeviceId`.

Their source contract explicitly classifies that target as unvalidated intent until separately gated registry/workspace/policy validation succeeds.

Raw target intent therefore cannot populate the production process peer.

Doing so would incorrectly convert requester-supplied intent into production bootstrap identity authority.

## 7. Registry-validated requester/rendezvous target remains operation-local point-in-time provenance

The current `RegistryValidatedRequesterRendezvousStart` owns the exact target `DeviceId` only after the requester and target pass the existing point-in-time requester/rendezvous registry validation chain.

That carrier explicitly proves only requester/rendezvous registry eligibility. It is not:

- transport readiness;
- production reachability bootstrap provenance by itself;
- a lease/currentness guarantee;
- a process-lifetime peer selection;
- provider registration authority.

The current validation implementation remains part of requester/rendezvous operation composition and is not proven as a pre-bootstrap executable process-peer producer.

C03e-JG therefore does not silently reuse this carrier as startup identity authority.

## 8. Policy-authorized requester/rendezvous target remains operation-local authority

`PolicyAuthorizedRequesterRendezvousStart` privately retains the exact registry-validated requester/rendezvous carrier after the dedicated requester-rendezvous-start policy gate.

That type explicitly remains non-authoritative for network reachability and current transport readiness.

The current validation-policy-registration composition is a requester/rendezvous operation seam and explicitly has no selected runtime caller.

C03e-JG finds no exact evidence that a policy-authorized requester/rendezvous target exists before production reachability durable-owner recovery or endpoint startup.

It therefore cannot be selected as the process bootstrap peer source in this checkpoint.

## 9. Expected-device admission stream is not a singular pre-bootstrap peer source

Current `LinuxAgentRemoteProcessOperationInputs` contains:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<...>>`

Each `RemoteSessionExpectedDeviceAdmissionRequest` owns an `expected_device_id: DeviceId`, but the receiver is consumed by the repeated real-admission endpoint lifecycle after endpoint startup.

The stream is therefore:

- post-bind admission input;
- potentially multi-item;
- admission scheduling/correlation authority for individual expected remote sessions;
- not a selected singular process-lifetime peer identity source.

C03e-IN already states that expected-device scheduling intent alone is not production peer provenance.

Combining one later admission item's `DeviceId` with JF's current-transport lookup could create a valid current same-device **peer value at that later point**, but the exact source does not establish that this item is the logical peer that must already exist for earlier reachability durable-owner recovery.

C03e-JG therefore rejects implicit first-item, next-item, cached-item or arbitrary expected-request selection as bootstrap authority.

## 10. No environment/configuration logical-device authority is selected

The existing production bind-address environment value remains reachability configuration only.

C03e-JG finds no selected fixed environment field, command-line field, systemd credential, file, DNS value, IP address, certificate identity, transport identity, request ID, session ID, or test fixture that is authorized to provide the production logical peer `DeviceId`.

No new environment/configuration identity field is selected here.

## 11. No inference from local requester identity

Authenticated requester/session identity is not automatically the remote production peer identity.

The requester/rendezvous model deliberately keeps requester identity and target logical `DeviceId` distinct.

C03e-JG does not derive the production peer from:

- the local authenticated requester device;
- the currently accepted remote session device;
- candidate publisher identity;
- session worker key;
- repeated-admission expected device without an explicit higher-level peer-selection decision.

## 12. No inference from transport certificate identity

A `TransportIdentity` alone is not a logical peer `DeviceId`.

C03e-JF intentionally accepts the logical device first and reads the current transport for that exact device.

C03e-JG does not invert that direction by searching the registry for a device from a transport identity, scanning records, using a reverse index, or accepting certificate identity as logical-device authority.

## 13. Closed gap selection

C03e-JG selects the following exact conclusion:

> The current post-JF source still lacks a selected authoritative pre-bootstrap production logical `DeviceId` source for the single `PeerConnectivityIdentity` required by the production reachability process-operation input owner.

The durable current-device/current-transport authority itself now exists, and JF can resolve an explicitly supplied logical device into a fail-closed current same-device peer. The remaining gap is strictly **which logical device the production process is intended to bootstrap reachability for, and where that decision comes from before provider/durable-owner recovery begins**.

## 14. Source materialization remains blocked

C03e-JG authorizes **no Rust source-materialization successor for production peer-field population**.

In particular it does not authorize a helper that simply accepts:

- raw requester target intent;
- first/next expected-device request;
- authenticated requester identity;
- environment `DeviceId`;
- transport identity;
- bind address/IP/port;
- a test fixture;

and passes that value to JF.

Such a helper would encode an unselected authority decision.

## 15. Required next prerequisite

The next separately gated checkpoint must select the production logical peer-device source and ownership lifecycle before peer-field population can be materialized.

That selection must prove at least:

1. the logical `DeviceId` is available before production reachability durable-owner recovery;
2. the source is authoritative for the intended process-operation peer, not merely correlation or requester intent;
3. whether the process operation owns exactly one peer for its lifetime or may transition between peers;
4. how any relationship to requester/rendezvous target intent or expected-device admission is established without circular startup dependency;
5. whether the source is local configuration, server-owned lifecycle state, an already-authorized typed carrier, or another existing authority;
6. failure behavior when no logical peer is selected;
7. whether peer selection may change while a reachability durable owner is active;
8. how selection changes interact with JF current transport rotation/revocation/currentness;
9. dependency-safe handoff into `prw-agent` without broad provider/registry exposure;
10. the first exact source-materialization ceiling and focused tests.

## 16. Explicit non-authorization

C03e-JG does not authorize or perform:

- Rust/Kotlin/Cargo/Cargo.lock source materialization;
- a new logical peer `DeviceId` configuration field;
- environment/systemd credential identity selection;
- registry Get/Txn/Put;
- provider connection/bootstrap;
- peer-field population;
- requester/rendezvous registration or policy evaluation;
- expected-request consumption;
- reachability durable-owner recovery;
- endpoint bind;
- candidate publication;
- NAT traversal;
- peer dialing;
- `run_with_production_reachability_requester_rendezvous_remote_process_companion(...)` invocation;
- `run()` or `main.rs` mutation;
- readiness/listener changes;
- credential/certificate/trust/RBAC provisioning;
- production registry population/migration;
- systemd/package changes;
- deployment/restart;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review transition, branch deletion, or history rewrite.

## 17. Closure meaning

C03e-JG closes only:

`PRODUCTION_PEER_LOGICAL_DEVICE_SOURCE_GAP_SELECTED`

It records that the current durable-registry chain has solved the current same-device transport authority gap but has **not** selected the pre-bootstrap logical peer-device decision source.

Production peer input population remains blocked until that distinct source/lifecycle authority is selected.