# Phase 152 C03e-BF — Remote Endpoint Production Bind-Address Source Materialization

Status: STAGED MATERIALIZATION

Gate target:
`C03E_BF_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-BE:
- branch: `phase-152-c03e-be-remote-endpoint-production-bind-address-source-selection-staging`;
- head: `1d23875d5b3f27e07ff17b8a8d86034d7a3cec9e`;
- tree: `d9da50552cf9c144fe8106289aaef6a17bed8e17`;
- gate: `C03E_BE_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_SELECTED`.

BE selected one explicit non-secret process configuration source, `PRW_REMOTE_BIND_ADDR`, parsed directly as `std::net::SocketAddr` at the Agent bootstrap/process configuration boundary.

## 2. Materialized source path

BF modifies only:

`crates/prw-agent/src/linux_bootstrap.rs`

and adds this materialization contract.

The exact materialized source commit immediately before this final contract update is:
`27027f9b9a06b7030207e1770f0e89f88e596bec`

The exact `linux_bootstrap.rs` blob at that source commit is:
`8d569a432fa5d8706cc1458a771f40dedd501f72`

No Cargo manifest, lockfile, crate root, `main.rs`, packaging/systemd file or workflow is changed.

## 3. Fixed configuration source

BF materializes:

```rust
pub const PRW_REMOTE_BIND_ADDR_ENV: &str = "PRW_REMOTE_BIND_ADDR";
```

and a public environment reader:

```rust
pub fn load_linux_agent_remote_bind_addr_from_env()
-> Result<SocketAddr, LinuxAgentRemoteBindAddressSourceError>
```

The public reader performs only `std::env::var_os(PRW_REMOTE_BIND_ADDR_ENV)` followed by the selected pure parser/validator.

It does not bind a socket, enumerate interfaces, inspect routes, resolve DNS, discover public addresses, read credentials or call any provider.

## 4. Bounded source error

BF materializes the non-exhaustive public error:

`LinuxAgentRemoteBindAddressSourceError`

with exactly these semantic classes:
- `Unavailable` — absent or empty configuration;
- `EncodingInvalid` — non-Unicode OS value;
- `SocketAddressInvalid` — not an exact `SocketAddr`;
- `AddressNotBindAdvertisable` — parsed IP is unspecified, multicast or IPv4 limited broadcast.

Display strings remain bounded and do not echo the configured value.

## 5. Pure parser/validator

BF materializes a pure injected-value helper:

```rust
fn parse_linux_agent_remote_bind_addr_value(
    value: Option<OsString>,
) -> Result<SocketAddr, LinuxAgentRemoteBindAddressSourceError>
```

The helper:
1. rejects missing/empty values;
2. converts the OS string to Unicode without fallback;
3. parses directly as `SocketAddr`;
4. rejects unspecified IP;
5. rejects multicast IP;
6. rejects IPv4 limited broadcast;
7. otherwise returns the exact parsed `SocketAddr` unchanged.

No hostname syntax, DNS lookup, interface selection, route inference, address rewrite or fallback is introduced.

## 6. Port-zero preservation

BF deliberately does not reject port `0`.

Port `0` remains a valid pre-bind request for kernel-selected ephemeral port assignment. After a successful future bind, the already-closed BB observation returns the exact bound `SocketAddr`, and the already-closed BD projection applies the existing `ConnectivityEndpoint::new` validation to the actual bound endpoint.

Therefore BF does not conflate pre-bind configuration validation with post-bind endpoint validation.

## 7. Loopback and reachability semantics

Loopback and other valid unicast addresses are preserved exactly by this source layer.

Their acceptance means only that the operator supplied a syntactically and semantically eligible bind address for this explicit configuration source. It does not establish:
- external reachability;
- candidate publishability;
- `LocalDirect` classification;
- `InternetDirect` classification;
- readiness;
- authentication or authorization.

## 8. Focused non-networking tests

BF adds tests in the existing `linux_bootstrap.rs` test module for:
- exact public reader signature and fixed configuration name;
- missing value rejection;
- empty value rejection;
- malformed address rejection;
- non-Unicode rejection on Unix;
- exact IPv4 preservation;
- exact IPv6 preservation;
- port `0` preservation;
- loopback preservation;
- unspecified IPv4/IPv6 rejection;
- multicast IPv4/IPv6 rejection;
- IPv4 limited-broadcast rejection.

Tests invoke the pure injected-value parser and do not mutate the process environment.

Tests perform no socket bind, DNS lookup, interface enumeration, route inspection, provider I/O or host networking mutation.

## 9. Existing remote composition remains unchanged

BF does not modify `LinuxAgentRemoteProcessOperationInputs`, `linux_agent_remote_process_operation`, `run_with_remote_process_companion`, endpoint lifecycle behavior or lower transport behavior.

The new reader is not consumed by those paths in BF.

`main.rs` remains byte-stable relative to BE and still invokes only the existing local bootstrap path.

Thus BF creates configuration-source capability without executable activation.

## 10. Dependency layering remains unchanged

BF introduces only `std` types already available to `prw-agent`:
- `OsString`;
- `IpAddr`;
- `Ipv4Addr`;
- `SocketAddr`.

No direct `prw-agent -> prw-connectivity` dependency is added.

No existing reachability-custody responsibility is widened, and no transport module learns process-environment policy.

## 11. Candidate semantics remain separately gated

BF does not construct `ConnectivityCandidate` and does not choose:
- CandidateId custody/allocation;
- plan-scoped high-water handling;
- `ConnectivityPathKind`;
- candidate priority/ranking;
- candidate provenance.

The source does not infer `LocalDirect` or `InternetDirect` from IP shape, loopback/private/global categorization, bind configuration or bind success.

## 12. Publication/provider semantics remain separately gated

BF does not:
- create `AuthenticatedCandidatePublication`;
- call `publish_current_candidates`;
- call `ProductionReachabilityOwner::commit_candidate_publication`;
- issue freshness tokens;
- mutate durable CAS/provider state;
- start candidate traversal;
- activate STUN/ICE/TURN/relay;
- mutate discovery.

## 13. Executable activation remains separately gated

BF does not modify Agent `main.rs` and does not wire the newly materialized reader into process startup.

BF does not select or materialize:
- expected-device producer/discovery;
- production capability dispatcher;
- registry/policy/timing/session-authentication source assembly;
- remote readiness;
- remote failure -> local process-exit policy;
- retry/backoff/reconnect/rebind/rebootstrap/replacement.

## 14. Packaging and host state remain unchanged

BF does not modify `packaging/systemd/prw-agent.service` and does not define how deployment supplies `PRW_REMOTE_BIND_ADDR`.

No systemd unit/drop-in, environment file, service credential, firewall, NAT, route, DNS, TUN/TAP, interface, socket activation, deployment, restart or recovery operation is changed.

## 15. Identity and security invariants

- `DeviceId` / authenticated PRW session identity remains logical identity.
- `TransportIdentity` remains lower-transport certificate identity only.
- `SocketAddr` / `ConnectivityEndpoint` remains transient endpoint/configuration state only.
- `SessionId` remains authentication correlation only.
- the environment variable and successfully parsed address are not identity or authorization material.
- configuration validity is not currentness, readiness, reachability, publication provenance or public-routability evidence.
- protected operations retain fresh-current registry/transport/policy evaluation.

## 16. Exact intended BE -> BF scope

The final BF branch must differ from exact closed BE only in:
1. `crates/prw-agent/src/linux_bootstrap.rs`;
2. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BF_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_MATERIALIZATION_STAGING.md`.

Any additional path blocks closure until explained and explicitly authorized.

## 17. Validation note

The first PR-head validation on `d9d9c98eb46ecf363ac3f4ab9218c399346adbd8` reached the locked dependency graph successfully and then failed only at `cargo fmt --all -- --check`. The exact rustfmt diff was applied verbatim to the source at `27027f9b9a06b7030207e1770f0e89f88e596bec`. That older CI head is obsolete for closure; only validation of the final head after this contract anchor may close BF.

## 18. Closure condition

BF can close only after:
- exact BE predecessor lineage remains unchanged;
- final BE -> BF diff is limited to the two intended paths;
- the new source readback matches the selected semantics;
- canonical Rust validation for the exact final BF head reaches terminal success;
- any automatically triggered Android validation reaches terminal success before closure;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No merge, deployment, systemd/host mutation or executable remote activation is part of BF closure.

Gate target remains:
`C03E_BF_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_MATERIALIZED`
