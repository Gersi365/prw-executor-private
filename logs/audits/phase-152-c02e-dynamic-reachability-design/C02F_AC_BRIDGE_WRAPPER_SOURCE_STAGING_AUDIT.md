# Phase 152 C02f-AC — Bridge Wrapper / Definitive Provider Result Mapping Source Staging Audit

Status: `SOURCE_STAGING_COMPLETE / STATIC_READBACK_PASS / EXECUTABLE_VALIDATION_NOT_RUN / NO_REAL_ETCD_CONTACT / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Staging branch: `phase-152-c02f-ac-bridge-wrapper-staging`
Authoritative predecessor branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `84112467751c18d123df7ebad315514b0c7fcd34`
Exact implementation source head: `e38189b07837be5156a05d03a5bc6ba25388940d`

## Authorized scope

C02f-Z selected the next implementation tranche after codec and deterministic transaction planning as:

> bridge wrapper that maps definitive provider outcomes into the already-selected async semantic port

This source tranche implements only that bridge/result-mapping boundary. It does not wire a real `etcd-client` Get/Txn call and does not activate an endpoint, socket, runtime, task, TLS profile, Watch, lease, TTL, deployment or production authority.

## Exact source changes

Relative to predecessor head `84112467751c18d123df7ebad315514b0c7fcd34`, the staging branch is ahead and not behind. The source diff is confined to exactly these three paths:

1. `crates/prw-remote-bridge/Cargo.toml`
   - promotes the already-existing `prw-control-plane` workspace dependency from dev-only to normal dependency;
   - does not add a new external crate or change the selected `etcd-client` pin.

2. `crates/prw-remote-bridge/src/reachability_live_owner_provider_bridge.rs`
   - adds an orchestration-side lower port for definitive C02f-AB provider outcomes;
   - adds `ReachabilityLiveOwnerProviderBridge<P>` implementing the existing C02f-Y `ReachabilityLiveOwnerAsyncAuthority`;
   - maps provider failure classifications only to the already-selected public semantic errors;
   - contains runtime-independent scripted tests using `std::future::ready` and direct polling only.

3. `crates/prw-remote-bridge/src/root.rs`
   - exports the new provider bridge module;
   - preserves the explicit no-runtime/no-network activation statement.

No `Cargo.lock`, workflow, Agent, deployment, endpoint, TLS, schema, network, auth or production-runtime file is modified by this tranche.

## Dependency direction preserved

C02f-Z requires that the concrete provider remain owned below the bridge and that `prw-control-plane` must not depend on `prw-remote-bridge`.

The staged dependency direction remains acyclic:

```text
prw-core <- prw-connectivity <- prw-control-plane
                               ^
                               |
                    prw-remote-bridge orchestration
```

`prw-control-plane/Cargo.toml` remains dependent on `etcd-client`, `prw-connectivity` and `prw-core`; no inverse `prw-control-plane -> prw-remote-bridge` edge is introduced.

## Definitive semantic mapping

### Acquisition

- definitive committed transaction + exact requested peer + `Current` successor -> construct `ReachabilityLiveOwnerFence` from the provider-owned non-zero raw fence and return `ReachabilityLiveOwnerAcquisition::Granted`;
- definitive compare failure -> `ReachabilityLiveOwnerAcquisition::Contended`;
- malformed/cross-peer/non-Current successor context -> `UnavailableOrAmbiguous`;
- no compare failure or unresolved provider outcome can construct a grant.

### Currentness

- definitive provider `Current` -> semantic `Current`;
- definitive provider `Stale` -> semantic `Stale`;
- unavailable/ambiguous provider result -> semantic `UnavailableOrAmbiguous`;
- fence exhaustion -> semantic `FenceExhausted`.

### Release

- definitive pre-read `NotCurrent` -> semantic `NotCurrent`;
- committed release transaction with exact peer, exact fence and `Released` successor -> semantic `Released`;
- compare failure whose authoritative observation proves the grant stale -> semantic `NotCurrent`;
- compare failure whose authoritative observation still proves the same grant `Current` -> `UnavailableOrAmbiguous`, because the semantic release enum has no honest result meaning “release definitely did not commit but grant is still current”;
- cross-peer, wrong-fence or non-Released mutation context -> `UnavailableOrAmbiguous`.

This preserves fail-closed semantics and prevents a failed release from being misreported as either successful or stale.

## Async contract preservation

The existing C02f-Y port requires returned futures to be `Send` and uses mutable static dispatch. The wrapper implementation therefore requires:

```text
P: ReachabilityLiveOwnerDefinitiveProviderPort + Send
```

This is necessary because the bridge `async move` future retains its mutable wrapper/provider borrow across provider `.await` points.

No Tokio type, task spawning primitive or runtime handle is added.

## Static validation performed

The following authoritative checks were completed against GitHub:

- repository ownership/transfer resolved to `powercode365-dotcom/prw-executor-private`, repository ID `1334911207`;
- authoritative predecessor branch head reverified exactly as `84112467751c18d123df7ebad315514b0c7fcd34` before mutation;
- C02f-Z contract re-read and exact source order #3 confirmed;
- C02f-Y async semantic trait signatures re-read;
- C02f-AB definitive mutation/currentness/transaction APIs re-read;
- C02f-AA authority-record `peer()`, `lifecycle()` and `fence()` visibility re-read and confirmed public across crate boundaries;
- staging compare confirms exact predecessor merge base, ahead-only relation, and only the three authorized source/dependency paths above;
- staged Cargo/root/module files were read back from the staging branch after mutation;
- current GitHub combined status for the source head contains no registered status checks.

## Executable validation limitation

This ChatGPT execution environment currently exposes `git` but no `cargo`, `rustc` or `rustfmt`, and no authenticated GitHub CLI checkout. The repository workflow supports manual dispatch, but the connected GitHub action surface available in this session does not expose workflow dispatch for a new run. A PR was intentionally not created merely to trigger CI because PR creation is a separate repository action and was not part of this authorized tranche.

Therefore this audit deliberately does **not** claim:

- `cargo fmt --all -- --check` PASS;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` PASS;
- `cargo test --locked --workspace --all-targets` PASS;
- `cargo build --locked --workspace --all-targets` PASS.

Those canonical executable checks remain required before this source staging can be promoted/merged as a validated C02f-AC checkpoint.

## Safety conclusion

The requested bridge wrapper/result mapping has been source-materialized on an isolated staging branch without real etcd contact or runtime/network activation. The source boundary is statically consistent with C02f-Z/C02f-Y/C02f-AB and remains fail closed. Executable Rust validation remains pending due tooling availability, not because an executable source defect has been observed.
