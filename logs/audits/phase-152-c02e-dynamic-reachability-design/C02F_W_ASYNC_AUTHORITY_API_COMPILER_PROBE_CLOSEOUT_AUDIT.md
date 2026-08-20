# Phase 152 C02f-W — Async Authority API Compiler Probe Closeout Audit

Status: `COMPILER_PROBE_COMPLETE / IMPL_FUTURE_SEND_MECHANICALLY_VALIDATED / STATIC_DISPATCH_BORROWING_VALIDATED / PR42_CLOSED_UNMERGED / NO_NEW_ARCHITECTURE_SELECTED / NO_RUNTIME_ACTIVATION / DOCS_ONLY`

Date: 2026-08-20

## 1. Scope

C02f-W closes the compiler-validation question opened by C02f-U and refined by C02f-V:

- whether `etcd-client = 0.19.0` can be used behind an explicit Rust return bound of `impl Future + Send` for the selected KV operations needed by the future authority adapter;
- whether a live-owner authority port represented with statically-dispatched `impl Future + Send` methods can borrow `&mut self`, peer identity, and authority grants without boxing or `async-trait`;
- whether those forms compile under the repository's canonical Rust toolchain.

This checkpoint does **not** select the production authority API, does **not** mutate production Rust source, does **not** select schema/CAS/TLS/topology/recovery values, and does **not** activate any network or runtime behavior.

## 2. Authoritative base

Active branch at probe start and immediately before every mutation:

- branch: `phase-152-c02e-dynamic-reachability-design`
- base/head: `2edd56f33e267e0322e7094a147b77c0c2fd318e`
- tree: `e4f399234bce23f777263e9dea17b6fa40159864`
- predecessor checkpoint: C02f-V

The production branch remained byte-stable throughout the disposable probe.

## 3. Disposable probe branch

Probe branch:

- `probe/phase-152-c02f-w-async-authority-api`
- final probe head: `70fe00b5a102829cc908626c522598eb776ec5a0`
- base merge point: `2edd56f33e267e0322e7094a147b77c0c2fd318e`
- relation to base: 6 commits ahead / 0 behind

Final probe diff versus production base contained exactly three added files:

1. `.github/workflows/phase-152-c02f-w-async-authority-probe.yml`
   - 51 additions
2. `crates/prw-control-plane/tests/c02f_w_etcd_future_send_probe.rs`
   - 20 additions
3. `crates/prw-remote-bridge/tests/c02f_w_async_authority_port_probe.rs`
   - 98 additions

Total disposable probe diff:

- 3 files
- 169 additions
- 0 deletions

No probe file was merged into the production branch.

## 4. Pull request containment

Disposable PR:

- PR: #42
- title: `Probe C02f-W async authority API representation`
- base: `phase-152-c02e-dynamic-reachability-design`
- final head: `70fe00b5a102829cc908626c522598eb776ec5a0`
- state: CLOSED
- merged: FALSE
- changed files: 3
- additions: 169
- deletions: 0

The PR existed only to obtain compiler/CI evidence and was closed unmerged after the targeted probe succeeded.

## 5. Probe 1 — etcd-client future `Send` bound

The control-plane probe used the already materialized dependency:

```text
etcd-client = { version = "=0.19.0", default-features = false }
```

It asserted that:

- `etcd_client::Client` is `Send`;
- a wrapper around `Client::get(...)` can expose:
  `impl Future<Output = Result<GetResponse, Error>> + Send + '_`;
- a wrapper around `Client::txn(...)` can expose:
  `impl Future<Output = Result<TxnResponse, Error>> + Send + '_`.

The wrappers contain no endpoint connection and are never polled against a real etcd server. The probe validates the Rust type/future boundary only.

### Result

PASS under the canonical toolchain.

This is mechanical evidence that the selected etcd client operations required by the future authority adapter do not force a non-`Send` future representation at this boundary.

## 6. Probe 2 — static-dispatch authority port

The remote-bridge probe represented a candidate authority port with methods equivalent to:

- acquire: borrowed `&mut self` + borrowed exact peer identity -> `impl Future + Send`;
- currentness: borrowed `&mut self` + borrowed grant -> `impl Future + Send`;
- release: borrowed `&mut self` + borrowed grant -> `impl Future + Send`.

A concrete reference implementation returned ready futures and the test imposed an explicit generic `Future + Send` assertion on all three calls.

### Result

PASS under the canonical toolchain.

This is mechanical evidence that static dispatch plus borrowing is viable without requiring:

- `async-trait` as an API design choice;
- boxed futures as an API design choice;
- dynamic trait-object dispatch.

It does **not** prove or select object safety / `dyn` dispatch.

## 7. Validation evidence

Targeted workflow:

- name: `Phase 152 C02f-W Async Authority Probe`
- successful run number: #3
- run ID: `32332896113`
- job ID: `96316795351`
- conclusion: SUCCESS

Successful steps:

1. checkout — PASS
2. install protobuf compiler — PASS
3. record Rust/protoc toolchain — PASS
4. `cargo metadata --locked --no-deps` — PASS
5. `cargo fmt --all -- --check` — PASS
6. Clippy control-plane probe with `-D warnings` — PASS
7. Clippy remote-bridge probe with `-D warnings` — PASS
8. control-plane probe test — PASS
9. remote-bridge probe test — PASS

Toolchain observed by the probe:

- rustc 1.97.1
- cargo 1.97.1
- rustfmt 1.9.0-stable
- clippy 0.1.97
- protoc/libprotoc 3.21.12

Earlier disposable attempts were non-semantic failures only:

- initial canonical PR validation stopped at rustfmt before compiler validation;
- targeted run #1 reached Clippy but failed only on `clippy::elidable-lifetime-names` in probe syntax, while the explicit `+ Send` bound itself was accepted;
- targeted run #2 was interrupted/cancelled during package-install infrastructure and produced no Rust verdict;
- final formatting/lifetime cleanup retained the same `+ Send` semantics and produced successful run #3.

## 8. Mechanical conclusions

C02f-W establishes the following compiler facts:

1. `etcd-client 0.19.0` is compatible with a statically-dispatched `impl Future + Send` wrapper for the probed `get` and `txn` operations.
2. A borrowed `&mut self` live-owner authority port can expose `impl Future + Send` methods without boxing.
3. The existing peer/grant semantic types can participate in that borrowed async boundary.
4. No dependency on `async-trait` is mechanically required by the current static-dispatch use case.
5. No dynamic-dispatch requirement has been demonstrated by current source usage or by this probe.

## 9. What this checkpoint does not select

The following remain explicitly unselected and require architecture approval before production mutation:

- whether the production authority trait/API is changed to the probed `impl Future + Send` form;
- exact receiver/concurrency model around the authority client (`&mut self`, interior mutability, cloning, serialization, or another proven model);
- exact etcd key/value schema and binary framing;
- exact CAS guard (`value`, `mod_revision`, or another proven transaction guard);
- indeterminate mutation reconciliation algorithm;
- TLS feature/profile, CA roots, mTLS identities, RBAC credentials/scopes, or endpoints;
- cluster voter count, topology, storage, heartbeat/election settings;
- recovery epoch bit split/scope and external immutable epoch ledger provider;
- R1-R4 sink-side stale-fence enforcement implementation;
- runtime/executor ownership and network activation.

## 10. Architecture boundary preserved

Inherited locked invariants remain unchanged:

- authenticated `DeviceId` / PRW session identity is logical identity;
- `TransportIdentity` is independently rotatable transport identity;
- exact live-owner namespace is `DeviceId + TransportIdentity`;
- IP/port/NAT/relay endpoint locations are not identity;
- shared control-plane authority placement T3 remains locked;
- etcd v3.7 remains the selected authority provider;
- `etcd-client 0.19.0` remains exact-pinned with default features disabled;
- fence remains PRW-owned `ReachabilityLiveOwnerFence(NonZeroU128)`;
- stale/member-local reads cannot prove authority currentness;
- authority ambiguity/unavailability fails closed;
- stale release cannot clear a newer owner;
- R1-R4 still require effect-boundary stale-fence rejection;
- clocks/leases/TTL/watch are not primary safety authority.

## 11. Production mutation status

C02f-W itself is documentation-only.

No production Rust source, Cargo manifest, lockfile, workflow, runtime configuration, TLS material, endpoint configuration, deployment state, credential, firewall, route, or network behavior is changed by this checkpoint.

The latest canonical full-workspace executable validation of merged production source remains the previously closed C02f-M validation unless a newer independent production validation is recorded elsewhere. The C02f-W targeted probe is evidence for the candidate API representation only.

## 12. Next gate

The useful non-repetitive readiness work for the async API shape is complete.

A production source mutation now requires an explicit selection/authorization for the relevant architecture group. The probed candidate available for such a selection is:

```text
static-dispatch live-owner authority port
with explicit `impl Future<...> + Send`
without mandatory boxing or `async-trait`
```

C02f-W does not itself make that selection.
