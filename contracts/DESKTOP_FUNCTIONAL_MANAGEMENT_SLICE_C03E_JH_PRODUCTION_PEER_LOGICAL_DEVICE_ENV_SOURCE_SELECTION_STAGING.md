# Phase 152 C03e-JH — Production Peer Logical Device Environment Source Selection

Status: **SELECTION ONLY — SOURCE MATERIALIZATION NOT YET AUTHORIZED**

Target gate:

`C03E_JH_PRODUCTION_PEER_LOGICAL_DEVICE_ENV_SOURCE_SELECTED`

## 1. Exact predecessor

C03e-JH starts only from exact closed C03e-JG head:

`7ff89ee4016019ed5528c2254e5eddfdd62b5144`

C03e-JG selected the remaining production peer gap: post-JF source can resolve an explicitly supplied logical `DeviceId` into one authoritative current same-device `PeerConnectivityIdentity`, but no pre-bootstrap source yet decided which logical `DeviceId` is the production process peer.

C03e-JH selects that missing source and lifecycle only. It does not materialize it.

## 2. Selected source

C03e-JH selects one fixed, non-secret Agent process configuration environment variable:

`PRW_REMOTE_PEER_DEVICE_ID`

The value represents only the operator/deployment-selected **logical peer intent for this production process lifetime**.

It is not, by itself:

- current registry authority;
- current transport identity;
- proof that the device exists;
- proof that the device is enrolled;
- proof that the device is still eligible;
- proof of workspace/policy authorization;
- reachability observation;
- a connectivity candidate;
- a certificate identity.

Full production peer provenance is obtained only when this typed logical device selection is resolved through the existing C03e-JF durable-registry current same-device lookup.

## 3. Why environment configuration is selected

The current production process already has a fixed non-secret Agent-owned environment configuration precedent:

`PRW_REMOTE_BIND_ADDR`

Current `linux_bootstrap.rs` reads that exact variable once through `std::env::var_os`, validates it fail-closed, and retains a stable bounded source error without echoing the configured value.

The logical peer selector is also non-secret process intent, not credential/private-key material.

Selecting an Agent-owned environment source avoids:

- adding `prw-core` to the systemd credential-custody crate only for one non-secret semantic identifier;
- mixing semantic process-peer selection into provider TLS/bootstrap configuration;
- adding another provider/security credential;
- inventing a new cross-crate configuration service;
- deriving identity from IP/certificate/request/runtime events.

## 4. Fixed source ownership

`prw-agent` owns acquisition and parsing of `PRW_REMOTE_PEER_DEVICE_ID`.

No other crate gains authority to reinterpret the environment variable.

The source remains process configuration only.

`prw-registry` remains the semantic authority for current registered-device and current transport state.

`prw-control-plane` remains provider mechanics/bootstrap owner.

## 5. Exact parse law

The future source loader must use:

`std::env::var_os(PRW_REMOTE_PEER_DEVICE_ID_ENV)`

or an exactly equivalent fixed-name acquisition boundary.

Selected constant name:

`PRW_REMOTE_PEER_DEVICE_ID_ENV`

Selected constant value:

`"PRW_REMOTE_PEER_DEVICE_ID"`

The value must be converted to Unicode without substitution.

On successful Unicode conversion, the exact string is passed directly to:

`DeviceId::new(value)`

The existing `DeviceId` law is retained:

- empty or whitespace-only value is rejected;
- any non-empty value is preserved exactly;
- no trimming, lowercasing, case-folding, delimiter parsing, path interpretation, hostname interpretation, or identifier normalization is added.

## 6. Selected source error boundary

The first source materialization may expose one bounded error equivalent to:

`LinuxAgentRemotePeerDeviceSourceError`

with stable classes equivalent to:

- `Missing`
- `NonUnicode`
- `InvalidIdentifier`

The display/error surface must not echo the configured identifier value.

`InvalidIdentifier` maps the existing `DeviceId::new` empty/whitespace rejection without widening identifier rules.

No raw `VarError`, `OsString`, or configured value is required to escape the source boundary.

## 7. One-read process-lifetime law

The selected logical peer `DeviceId` is read exactly once for one production process-operation assembly/bootstrap transaction.

It is not polled, watched, refreshed, or re-read during an active reachability owner lifecycle.

Changing the process environment after acquisition does not re-key an already-recovered production reachability owner.

A different configured peer requires a new explicit process/bootstrap lifecycle; C03e-JH does not materialize or automate that lifecycle transition.

## 8. Fixed-peer lifetime law

One production reachability process-operation instance owns exactly one selected logical peer for the lifetime of the recovered reachability owner used by that operation.

The selected source does not rotate between peers while the same production owner remains active.

This matches the current recovery API, which recovers one exact owner for one exact `PeerConnectivityIdentity` lifecycle.

The existing multi-peer custody-map seam over already-recovered owners does not change this one-process-operation selection law and is not activated by C03e-JH.

## 9. JF remains authoritative for current transport

After successful source parsing, the typed logical `DeviceId` must later be passed to the existing JF operation:

`ProductionDurableRegistryRuntimeCustody::peer_connectivity_identity(device_id)`

That operation:

- performs the authoritative current same-device transport lookup;
- fails closed for unavailable/invalid/non-participating/unbound authority;
- returns a `PeerConnectivityIdentity` only after current authority succeeds.

Therefore the environment value does not become full peer provenance by itself.

This preserves C03e-IN's rule that an environment literal alone is not production peer identity authority.

## 10. Currentness and transport rotation

A successful JF resolution creates a point-in-time current peer lifecycle consisting of:

- the selected process logical `DeviceId`; and
- the then-current durable-registry `TransportIdentity` for that same device.

If the device's transport identity later rotates, the already-recovered reachability owner remains keyed to its original exact peer lifecycle.

C03e-JH selects no hidden re-key, refresh, retry, migration, owner replacement, or automatic rebootstrap.

A later explicit lifecycle checkpoint is required if production must react automatically to transport rotation while running.

## 11. Revocation/removal/unbound failure

The process selector does not override registry state.

If JF cannot establish a valid current peer for the configured logical `DeviceId`, peer construction fails closed and production reachability bootstrap must not proceed with a fabricated or stale peer.

No fallback is selected to:

- a previous transport;
- another device;
- requester/rendezvous target;
- expected-device stream;
- local requester identity;
- environment transport identity;
- test fixture.

## 12. No requester/rendezvous coupling

`PRW_REMOTE_PEER_DEVICE_ID` is process bootstrap configuration, not requester/rendezvous intent.

Requester/rendezvous target `DeviceId` may later equal or differ from the configured process peer according to separately selected operation semantics.

C03e-JH does not infer equality and does not overwrite one source with the other.

Requester/rendezvous validation/policy remains operation-local and occurs after the process bootstrap boundary.

## 13. No expected-admission coupling

The repeated `expected_requests` stream remains post-bind admission scheduling input.

Its `expected_device_id` values do not select or mutate `PRW_REMOTE_PEER_DEVICE_ID`.

The configured process peer is not selected from first/next/last expected request.

No stream item re-keys the active reachability owner.

## 14. No secret/security semantics

The logical peer `DeviceId` is not secret credential material.

C03e-JH therefore does not place it into:

- systemd `LoadCredential`;
- registry provider mTLS identity;
- private-key custody;
- certificate trust material;
- provider RBAC configuration.

This checkpoint creates no secret and changes no security configuration.

## 15. No implicit local-device meaning

The selected environment variable means the logical **remote/process peer** required by the existing production reachability process-operation input owner.

It is not the local Agent's own device identity and must not be populated from the local device certificate or authenticated requester session.

The name `PRW_REMOTE_PEER_DEVICE_ID` is selected to preserve this distinction.

## 16. First source-materialization ceiling

After C03e-JH closure, the next separately gated source checkpoint may change only:

`crates/prw-agent/src/linux_bootstrap.rs`

Allowed first source delta:

- `PRW_REMOTE_PEER_DEVICE_ID_ENV` constant;
- bounded `LinuxAgentRemotePeerDeviceSourceError`;
- pure/internal parser taking `Option<OsString>` or equivalent testable input;
- fixed-name `load_linux_agent_remote_peer_device_id_from_env()` returning `DeviceId`;
- focused tests for constant/signature and missing/non-Unicode/empty/whitespace/exact-preservation behavior.

The source checkpoint must not yet:

- call JF;
- construct `PeerConnectivityIdentity`;
- populate `LinuxAgentProductionReachabilityRemoteProcessOperationInputs`;
- call reachability bootstrap;
- invoke the executable assembly wrapper;
- modify `run()` or `main.rs`.

## 17. Expected focused tests

The first source checkpoint should prove without registry/provider/runtime I/O:

- exact constant value `PRW_REMOTE_PEER_DEVICE_ID`;
- missing variable -> bounded missing error;
- non-Unicode -> bounded non-Unicode error where test platform permits construction;
- empty string -> invalid identifier;
- whitespace-only -> invalid identifier;
- ordinary non-empty value -> exact `DeviceId` preservation;
- a non-empty value with leading/trailing spaces is preserved exactly if accepted by existing `DeviceId::new`, rather than silently trimmed;
- loader return type is exactly `DeviceId`.

## 18. Later peer-population prerequisite

After source materialization, a separate checkpoint must select and materialize the join:

1. load exact process logical `DeviceId` once;
2. resolve it through JF current durable registry authority once;
3. construct/populate the existing production process-operation peer owner;
4. propagate source/registry failure without fallback.

That later join remains distinct from runtime activation and executable wrapper invocation.

## 19. Explicit non-authorization

C03e-JH does not authorize or perform:

- Rust source materialization itself;
- environment variable mutation;
- service-unit environment configuration;
- JF registry lookup;
- provider/registry I/O;
- peer construction/population;
- reachability durable recovery;
- requester/rendezvous invocation;
- expected-request consumption;
- endpoint bind/readiness;
- traversal/dialing;
- runtime/startup wiring;
- credential/security/RBAC changes;
- production registry data mutation;
- deployment/restart;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review transition, branch deletion, or history rewrite.

## 20. Closure meaning

C03e-JH closes only:

`PRODUCTION_PEER_LOGICAL_DEVICE_ENV_SOURCE_SELECTED`

It selects `PRW_REMOTE_PEER_DEVICE_ID` as one fixed non-secret process-lifetime logical peer intent source, while preserving JF durable-registry lookup as the authority that converts that intent into one current same-device production peer.

No source is materialized or activated by this selection checkpoint.