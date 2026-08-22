# Phase 152 C02f-CB — Control-Plane Role Private-Key Zeroization Source Hardening Staging

Status: `MATERIALIZING / CONTROL_PLANE_SECRET_MEMORY_HARDENING_ONLY / ROLE_PRIVATE_KEY_ZEROIZE_ON_DROP / VALIDATION_FAILURE_ZEROIZATION / NO_CUSTODY_LOADER / NO_SYSTEMD_CREDENTIAL_VALUES / NO_ENDPOINT_VALUES / NO_PROVIDER_BOOTSTRAP_INVOCATION / NO_AGENT_MAIN_WIRING / NO_RUNTIME_ACTIVATION / NO_RECOVERY / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Exact prerequisite

C02f-CB derives only from canonical C02f-CA:

- branch: `phase-152-c02f-ca-reachability-bootstrap-custody-secret-memory-selection-staging`;
- head: `5030c011a229cefa7ba6a1b4b14242e5ce507275`;
- tree: `9356f534dbbaeec711753ea4c299bf69eec9912a`;
- gate: `C02F_CA_REACHABILITY_BOOTSTRAP_CUSTODY_AND_SECRET_MEMORY_SELECTED`.

CA selected systemd service credentials as the future Ubuntu reachability-bootstrap custody substrate, but explicitly forbids live private-key credential loading until PRW-owned role private-key memory is hardened.

## Scope

CB closes only that prerequisite in the existing C02f-BX control-plane bootstrap types.

The current source retains each role private key in a normal `Vec<u8>`. CB changes PRW ownership so the private-key plaintext is wrapped in zeroizing memory before structural validation and remains zeroizing for the rest of its PRW-owned lifetime.

CB does not materialize the CA-selected custody adapter and does not load any real secret.

## Required source shape

`ReachabilityEtcdClientIdentityMaterial` keeps the role certificate separately and changes only private-key storage conceptually from:

```rust
private_key_pem: Vec<u8>
```

to:

```rust
private_key_pem: Zeroizing<Vec<u8>>
```

The constructor must convert and wrap the private-key input before the whitespace/emptiness check:

```rust
let private_key_pem = Zeroizing::new(private_key_pem.into());
```

This order is mandatory so rejected private-key input is also zeroized when the local value drops.

The existing role-reuse guard must compare private-key byte slices without adding Clone, Debug, display, serialization or public accessors.

The existing provider TLS composition may borrow the zeroizing buffer as a slice. CB does not redefine how the external etcd/rustls dependency internally owns identity bytes after that call.

## Dependency boundary

`prw-control-plane` may add the already-resolved exact dependency:

`zeroize = "=1.9.0"`

No new package version is selected.

If Cargo proves a root `Cargo.lock` package-edge correction is required, only the deterministic `prw-control-plane -> zeroize` edge is authorized. No version/checksum drift is authorized.

## Security invariants

After CB:

1. PRW-owned role private-key plaintext is zeroized on drop;
2. empty/whitespace-only constructor failure paths also drop a zeroizing private-key buffer;
3. role private keys remain distinct and exact-byte reuse remains rejected;
4. certificate handling remains unchanged;
5. no private-key accessor is added;
6. no `Clone`, `Debug`, display or serialization implementation is added to the identity material;
7. bounded public errors contain no secret bytes;
8. no provider network I/O is added by construction;
9. no secret source or fallback path is introduced.

## Explicit exclusions

CB does not authorize or materialize:

- a reachability custody crate/module;
- systemd credential names or encrypted blob paths;
- `CREDENTIALS_DIRECTORY` reads;
- real endpoint values;
- real trust/certificate/private-key bytes;
- `bootstrap_reachability_live_owner_preparation(...)` invocation;
- etcd connection/auth/RBAC/membership mutation;
- `prw-agent/src/main.rs` changes;
- startup/readiness/runtime task changes;
- authority acquisition/currentness/release execution;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 activation;
- deployment;
- merge.

## Validation requirements

The CB gate may be claimed only after:

1. exact CA ancestry is verified;
2. CA -> CB compare contains only the CB contract, the control-plane manifest/source changes and any Cargo-proven deterministic lock edge;
3. any temporary corrective/materialization workflow is absent from the final tree;
4. exact-head Rust validation reaches terminal success for locked graph, formatting, Clippy, tests and build;
5. Android/AD/AE workflow verdicts are reported exactly as triggered/skipped/not-triggered;
6. Drive audit is written and read back;
7. rolling status is appended with its prior prefix byte-identical;
8. the CB PR remains draft/open/unmerged.

## Gate

`C02F_CB_CONTROL_PLANE_ROLE_PRIVATE_KEY_ZEROIZATION_HARDENED`

This gate means only that PRW-owned control-plane role private-key memory has the CA-required zeroization invariant. It does not mean custody, provider bootstrap, authority runtime or production activation is live.