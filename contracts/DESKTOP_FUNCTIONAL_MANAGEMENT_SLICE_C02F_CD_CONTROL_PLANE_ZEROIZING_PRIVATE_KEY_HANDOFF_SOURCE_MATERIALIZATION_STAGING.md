# Phase 152 C02f-CD — Control-Plane Zeroizing Private-Key Handoff Source Materialization Staging

Status: `MATERIALIZING / ZEROIZING_PRIVATE_KEY_BY_VALUE_CONSTRUCTOR / EXISTING_VEC_CONSTRUCTOR_DELEGATES_AFTER_ZEROIZING_WRAP / NO_CUSTODY_CRATE / NO_CREDENTIAL_READ / NO_REAL_VALUES / NO_PROVIDER_BOOTSTRAP_INVOCATION / NO_RUNTIME_ACTIVATION / NO_RECOVERY / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Exact prerequisite

C02f-CD derives only from canonical C02f-CC:

- branch: `phase-152-c02f-cc-reachability-custody-zeroizing-handoff-schema-selection-staging`;
- head: `01042429f62c0165619a6f1d55753f643ddfc6e4`;
- tree: `4fcc86a6b19f1f66d1143ba37ca8750ece94c221`;
- gate: `C02F_CC_REACHABILITY_CUSTODY_ZEROIZING_HANDOFF_AND_CREDENTIAL_SCHEMA_SELECTED`.

## Scope

CD materializes only the control-plane seam required before reachability custody source may exist.

`ReachabilityEtcdClientIdentityMaterial` must expose a narrow constructor that accepts the role private key as `Zeroizing<Vec<u8>>` by value and retains that zeroizing owner directly.

The selected constructor is:

```rust
pub fn new_with_zeroizing_private_key(
    certificate_pem: impl Into<Vec<u8>>,
    private_key_pem: Zeroizing<Vec<u8>>,
) -> Result<Self, ReachabilityEtcdClientIdentityMaterialError>
```

The existing `new(...)` constructor remains for current tests/compatibility but must wrap its private-key input in `Zeroizing<Vec<u8>>` before delegating to the new constructor. This also ensures the private-key buffer is already zeroizing if certificate validation fails after delegation begins.

## Required invariants

After CD:

1. future custody can move a `Zeroizing<Vec<u8>>` directly into control-plane retained storage;
2. the zeroizing constructor does not clone or unwrap private-key plaintext;
3. the existing Vec-based constructor wraps before delegation;
4. private-key whitespace validation still occurs on the zeroizing buffer;
5. certificate validation semantics are unchanged;
6. no private-key accessor, Clone, Debug, Display, serialization, or generic secret API is added;
7. provider TLS composition continues to borrow the retained key only as a slice;
8. no provider network I/O occurs during either constructor;
9. no dependency or lockfile change is required because exact `zeroize = "=1.9.0"` is already present from CB.

## Validation

The final CD gate requires:

- exact CC ancestry;
- final net diff limited to this contract and `bootstrap.rs`;
- no temporary helper workflow in the final tree;
- canonical Rust validation full pass;
- Android/AD/AE verdicts reported exactly as triggered/skipped/not-triggered;
- Drive audit/readback and append-only rolling status preservation;
- PR draft/open/unmerged.

## Explicit exclusions

CD does not authorize or materialize:

- `prw-reachability-custody` crate;
- `$CREDENTIALS_DIRECTORY` reads;
- any systemd credential filename lookup;
- any real endpoint, CA, certificate, or private key;
- service-unit/drop-in edits;
- encrypted credential provisioning;
- `bootstrap_reachability_live_owner_preparation(...)` invocation;
- etcd connection/auth/RBAC/membership mutation;
- Agent startup/readiness/runtime task wiring;
- authority acquisition/currentness/release activation;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 activation;
- deployment;
- merge.

## Gate

`C02F_CD_CONTROL_PLANE_ZEROIZING_PRIVATE_KEY_HANDOFF_SOURCE_MATERIALIZED`

This gate means only that the zeroizing custody-to-control-plane private-key handoff seam exists and is validated.