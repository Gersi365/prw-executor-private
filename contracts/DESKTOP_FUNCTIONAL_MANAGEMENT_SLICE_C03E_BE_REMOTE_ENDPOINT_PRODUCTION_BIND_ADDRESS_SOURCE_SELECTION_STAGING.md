# Phase 152 C03e-BE — Remote Endpoint Production Bind-Address Source Selection

Status: STAGED SELECTION

Gate target:
`C03E_BE_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_SELECTED`

## 1. Exact predecessor

Closed C03e-BD:
- branch: `phase-152-c03e-bd-remote-endpoint-connectivity-endpoint-projection-source-materialization-staging`;
- head: `700b1c895c0a2ef8b78a6673d4c01ffb5f762265`;
- tree: `0f4a83e2d3dd141888ce1a9f1307d3b311ba8633`;
- gate: `C03E_BD_REMOTE_ENDPOINT_CONNECTIVITY_ENDPOINT_PROJECTION_SOURCE_MATERIALIZED`.

BD materialized only a pure projection from an already-observed bound `SocketAddr` to the existing validated `ConnectivityEndpoint`. It did not choose where the Agent obtains the production endpoint bind input.

## 2. Audited missing seam

The current Agent remote endpoint constructors already require a typed `SocketAddr` bind input:
- `RemoteSessionEndpointLifecycleRuntime::bind_from_systemd_credentials(..., bind_addr: SocketAddr)`;
- `RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials(..., bind_addr: SocketAddr)`;
- `LinuxAgentRemoteProcessOperationInputs` stores `bind_addr: SocketAddr`.

The existing `linux_agent_remote_process_operation` documentation explicitly leaves the production bind-address source unselected.

The current standalone Agent `main.rs` does not construct or invoke the remote operation, and the current `packaging/systemd/prw-agent.service` supplies no remote bind-address configuration.

No existing repository module was found that owns production host-interface enumeration or automatic bind-address selection. `prw-network` contains domain configuration types only, while reachability custody owns fixed sensitive systemd service credentials rather than ordinary bind configuration.

Therefore a configuration source must be selected before executable remote-lane composition can be selected safely.

## 3. Selected source

BE selects one fixed, non-secret process environment variable as the production remote bind-address configuration source:

`PRW_REMOTE_BIND_ADDR`

The selected value is the complete socket address, including port, and must parse directly as `std::net::SocketAddr`.

Examples of the selected input shape include:
- `192.0.2.10:4433`;
- `[2001:db8::10]:4433`;
- `192.0.2.10:0` when the operator deliberately requests a kernel-selected ephemeral port.

The source is explicit configuration only. It is not identity, authentication, authorization, reachability evidence, publication provenance, readiness evidence or public-routability evidence.

## 4. Selected ownership boundary

The source reader belongs at the Agent process/bootstrap configuration boundary, adjacent to `LinuxAgentRemoteProcessOperationInputs` in `crates/prw-agent/src/linux_bootstrap.rs`.

This preserves the existing layering:
- no direct `prw-agent -> prw-connectivity` dependency is added;
- `prw-connectivity` remains provider-neutral connectivity domain logic;
- `prw-remote-bridge` remains the candidate/publication/reachability composition layer;
- reachability credential custody remains limited to its existing sensitive fixed credential set;
- the transport runtime continues to consume an already-typed `SocketAddr` and does not learn environment-variable policy.

No Cargo manifest or crate-root change is selected.

## 5. Selected parse and validation semantics

A future materialization reads `PRW_REMOTE_BIND_ADDR` as an operating-system string and fails closed when the variable is absent, empty, non-Unicode, or not an exact `SocketAddr`.

No hostname form is accepted and no DNS resolution is performed.

After parsing, the source validation rejects addresses that cannot become a valid BD connectivity endpoint after bind:
- unspecified IPv4 `0.0.0.0`;
- unspecified IPv6 `::`;
- multicast IPv4;
- multicast IPv6;
- IPv4 limited broadcast `255.255.255.255`.

No fallback address or automatic replacement is attempted.

Loopback and other syntactically valid unicast addresses are not rejected solely by this source layer. Their configuration does not imply that they are externally reachable, publishable, `LocalDirect`, or `InternetDirect`.

## 6. Port-zero semantics

Port `0` is deliberately accepted by the bind-address source.

At bind time, port `0` asks the kernel to select an ephemeral local port. The existing lower transport owns that socket bind. After a successful bind, the already-materialized BB observation obtains the exact kernel-selected local `SocketAddr`, and the BD projection validates the observed non-zero port through `ConnectivityEndpoint::new`.

Therefore BE does not apply the `ConnectivityEndpoint` non-zero-port rule prematurely to pre-bind configuration.

A successful parse of `address:0` is configuration validity only. It is not evidence that bind succeeded or that a usable endpoint exists.

## 7. Selected bounded failure surface

The future source reader exposes a stable Agent-bootstrap configuration error with these semantic classes:
- configuration unavailable: the variable is absent or empty;
- configuration encoding invalid: the operating-system value is not Unicode;
- socket address invalid: the value does not parse exactly as `SocketAddr`;
- address not bind-advertisable: the parsed IP is unspecified, multicast, or IPv4 limited broadcast.

The error boundary must not echo the configured value in a way that turns logs into an unintended configuration-disclosure surface.

No bind, credential read, provider call or other side effect occurs before successful source validation.

## 8. Testability selection

The materialization should split environment acquisition from pure validation so tests do not mutate global process environment.

A pure helper accepts an injected optional OS-string value and returns the same typed result/error used by the public environment reader. Focused tests cover:
- missing value;
- empty value;
- malformed socket address;
- valid IPv4;
- valid IPv6;
- port `0` preservation;
- unspecified IPv4/IPv6 rejection;
- multicast IPv4/IPv6 rejection;
- IPv4 limited-broadcast rejection;
- loopback preservation without any reachability claim.

The tests perform no socket bind, DNS lookup, interface enumeration, route inspection or process-environment mutation.

## 9. Candidate semantics remain separately gated

BE does not construct `ConnectivityCandidate`.

A `ConnectivityCandidate` still requires separately authorized values for:
- `CandidateId`;
- `ConnectivityPathKind`;
- the validated `ConnectivityEndpoint`.

BE does not infer `LocalDirect` or `InternetDirect` from the configured IP address, the observed bound address, loopback/private/global address shape, interface naming, route state or bind success.

BE does not allocate or reuse candidate identifiers and does not alter the existing plan-scoped candidate-ID high-water rules.

## 10. Publication and provider semantics remain separately gated

BE does not:
- construct `AuthenticatedCandidatePublication`;
- call `publish_current_candidates`;
- commit candidate publication through `ProductionReachabilityOwner`;
- issue freshness tokens;
- perform durable CAS;
- mutate reachability observations;
- start STUN/ICE/TURN/relay processing;
- advertise an endpoint through discovery or another provider.

The configured bind address is not currentness or reachability evidence.

## 11. Executable activation remains separately gated

BE does not modify Agent `main.rs` and does not invoke the existing remote process operation.

BE also does not select or materialize:
- expected-device request production/discovery;
- a production capability dispatcher;
- registry/policy/timing/session-authentication source assembly;
- remote readiness publication;
- remote failure -> local process-exit policy;
- retry/backoff/reconnect/rebind/rebootstrap/replacement policy.

The existing local Agent lifecycle remains unchanged.

## 12. Packaging and host state remain unchanged

BE does not modify `packaging/systemd/prw-agent.service` and does not select how production packaging will populate `PRW_REMOTE_BIND_ADDR`.

No systemd unit/drop-in, environment file, credential, host interface, firewall, NAT, route, DNS, TUN/TAP, socket activation, deployment, restart or recovery operation is changed.

The eventual packaging source for the environment value remains a separate explicit mutation gate.

## 13. Identity and security invariants

- `DeviceId` / authenticated PRW session identity remains logical identity.
- `TransportIdentity` remains lower-transport certificate identity only.
- `SocketAddr` and `ConnectivityEndpoint` remain transient network endpoint/configuration state only.
- `SessionId` remains authentication correlation only.
- environment-variable presence or parse success is not authentication, authorization, readiness, currentness, reachability or publication evidence.
- protected operations continue to require existing fresh-current registry/transport/policy evaluation.

## 14. Selected BF source-materialization scope

If BE closes, the next source-materialization checkpoint is limited to:
1. `crates/prw-agent/src/linux_bootstrap.rs`;
2. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BF_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_MATERIALIZATION_STAGING.md`.

BF may materialize only the selected constant, bounded error type, pure parser/validator, environment reader and focused non-networking tests.

BF must not wire the resulting value into `main.rs`, invoke the remote lane, construct a candidate, publish reachability, mutate provider state, or modify packaging/systemd.

## 15. Exact BE scope

The intended final BD -> BE diff is one documentation path only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BE_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_SELECTION_STAGING.md`

No source file, manifest, lockfile, workflow, Agent binary, packaging file or host artifact is authorized to change in BE.

## 16. Closure condition

BE can close only after:
- the exact BD predecessor remains unchanged;
- the final BD -> BE diff remains the single selected docs-only path;
- canonical automatically applicable validation for the exact final BE head reaches a terminal successful verdict;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive evidence is freshly guarded, appended in place with exact predecessor-prefix preservation, and raw-readback verified;
- PR body moves from `Status: STAGED` to `Status: CLOSED` only after Drive verification;
- the PR remains draft/open/unmerged;
- final GitHub race checks remain clean.

No merge, deployment, host mutation or executable activation is part of BE closure.

Gate target remains:

`C03E_BE_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_SELECTED`
