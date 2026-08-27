# Phase 152 C03e-CD — PRWC Pre-Mesh Authentication Pure Codec Lock-Synchronized Source Materialization

Status: `STAGED — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CD_PRWC_PRE_MESH_AUTHENTICATION_PURE_CODEC_LOCK_SYNCHRONIZED_SOURCE_MATERIALIZED`

## 1. Exact predecessor authority

C03e-CD branches only from the exact closed C03e-CC checkpoint:

- branch: `phase-152-c03e-cc-prwc-pre-mesh-authentication-codec-lockfile-scope-reselection-staging`;
- head: `2bd6fa09c506ae6afce599007a58924e672a5c0a`;
- tree: `acf8330478f7cc8b6dcbdb9ebe4b80282843b533`;
- gate: `C03E_CC_PRWC_PRE_MESH_AUTHENTICATION_CODEC_LOCKFILE_SCOPE_RESELECTED`.

Blocked C03e-CB remains preserved as provenance and is not promoted as a branch ancestor of C03e-CD.

## 2. Reused source provenance

The corrected source materialization reuses the exact source blobs already exercised at blocked C03e-CB head `fa670d799bfd8cfe5b380a033d38a5f78cd58f87`:

- `crates/prw-remote-bridge/Cargo.toml` blob `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- `crates/prw-remote-bridge/src/root.rs` blob `8fdc1f30d6be12e55e0cfa0c7624810e60466b99`;
- `crates/prw-remote-bridge/src/control_session_auth_wire.rs` blob `77c6f401ef73c0b2a97645ae8bc83524c769a905`.

At blocked C03e-CB, PRW Rust Validation #1145 / run `33006752983` / job `98302377128` was FULL PASS. Android #842 / run `33006752962` reached the native adapter and failed only because `apps/android/native/Cargo.lock` required synchronization while `--locked` prohibited mutation. That blocker is not treated as a source-code Rust failure.

No source redesign or unrelated source correction is selected here.

## 3. Exact five-path authorization

C03e-CC authorizes C03e-CD to differ from the exact CC predecessor in exactly these five paths and no others:

1. `crates/prw-remote-bridge/Cargo.toml`;
2. `apps/android/native/Cargo.lock`;
3. `crates/prw-remote-bridge/src/root.rs`;
4. `crates/prw-remote-bridge/src/control_session_auth_wire.rs`;
5. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CD_PRWC_PRE_MESH_AUTHENTICATION_PURE_CODEC_LOCK_SYNCHRONIZED_SOURCE_MATERIALIZATION_STAGING.md`.

Any sixth changed path invalidates this checkpoint and requires renewed scope audit before closure.

Root `Cargo.lock` is not authorized to change and must remain byte-identical to predecessor blob `eeacde7ee776d35088f746a6d09f823f3391b82b`.

## 4. Cargo-authored Android native lock synchronization

The Android native lock synchronization was produced in an isolated non-merge probe from the exact blocked-CB source tree.

Probe evidence:

- probe PR: #200, closed without merge;
- probe source tree before lock synchronization: `1c32f3e866b28e953972c1305388b8e943f74e11`;
- successful workflow run: `33052104650`;
- successful job: `98449877843`;
- Cargo-authored bot commit: `7ce94985943a11ba20b9a08ab64bc21f49e6ea04`;
- resulting `apps/android/native/Cargo.lock` blob: `cce9ca06190a196661ab38d54a747893e26af95f`.

The probe guard required exactly one changed path and exactly one semantic lockfile line. The accepted delta is:

```text
+ "prw-core",
```

inside the existing `prw-remote-bridge` dependency list.

The accepted probe commit is exactly +1/-0 in `apps/android/native/Cargo.lock`. It does not add or change package records, package versions, registry sources, checksums, feature resolutions, or unrelated dependency edges.

Earlier probe strategies that produced no required delta or attempted wider registry version refresh were rejected before any lock result was promoted. They are not C03e-CD source inputs.

## 5. PRWA v1.0 protocol invariants

C03e-CD preserves the C03e-CA/C03e-CC protocol selection exactly:

- inner protocol magic/version: `PRWA` v1.0;
- fixed 12-byte inner header;
- message types: Begin=1, Challenge=2, Proof=3, Authenticated=4, Rejected=5;
- outer PRWC pairing remains `Authentication`, `Response`, and `Error` exactly as selected;
- one caller-supplied non-zero BY-managed request ID is used for outer correlation only;
- `DeviceId` and `SessionId` remain bounded typed values;
- authentication nonce is exactly 32 bytes;
- challenge lifetime is greater than zero and at most 300 seconds;
- proof profile remains P-256 / SHA-256 with DER signature encoding;
- encoded proof signature length remains 1..=256 bytes;
- `Rejected` remains generic and does not expose sensitive authentication detail;
- successful decode establishes structural/type validity only.

This codec does not itself authenticate a session or authorize a capability.

## 6. Authority separation

`SessionAuthenticationService` and current registry/session authority remain unchanged and separate from this pure codec.

The codec does not:

- allocate request IDs;
- generate challenges, nonces, or SessionIds;
- verify cryptographic proofs;
- consult registry authority;
- call session-authentication services;
- create authenticated sessions;
- make capability decisions.

Those responsibilities remain with their already-selected authority owners and later explicitly reviewed composition checkpoints.

## 7. Runtime and side-effect boundary

C03e-CD is source materialization only.

It performs no:

- listener or socket activation;
- frame-loop activation;
- network I/O;
- requester or rendezvous provider execution;
- candidate-publication execution;
- Agent, Desktop, or Android application activation;
- system service mutation;
- DNS or routing mutation;
- deployment;
- restart;
- database migration;
- authentication cutover;
- production credential change;
- rebase or merge.

## 8. Required exact-head validation

After the exact five-path commit is attached to the C03e-CD branch, closure requires fresh terminal evidence for that exact head.

Required observations:

1. canonical PRW Rust Validation must reach terminal success for the exact C03e-CD head;
2. canonical Android validation must reach terminal success for the exact C03e-CD head, including the native adapter locked dependency graph and all application stages that the canonical workflow actually triggers;
3. C02f-AD and C02f-AE workflows must be reported according to their actual terminal states; skipped workflows are not PASS;
4. no pending or failing triggered workflow may remain when closure evidence is written;
5. any environment/tooling failure must be distinguished from a source defect before any corrective mutation.

No validation PASS is claimed by this staging contract.

## 9. Closure requirements

Before C03e-CD may be semantically closed:

- fresh GitHub branch/head/tree race-check must match the validated head;
- predecessor-to-head compare must remain exactly five changed paths and no sixth path;
- Android native lock diff must remain exactly the one selected `"prw-core"` edge;
- root `Cargo.lock` must remain byte-stable;
- exact-head canonical workflows must be terminal as required above;
- an immutable C03e-CD audit must be written to Google Drive and read back exactly;
- the rolling `C02E_BRANCH_STATUS.md` must be updated only after a fresh predecessor guard proves no concurrent drift;
- the C03e-CD pull request must remain draft/open/unmerged after semantic closure.

Until those requirements are satisfied, this checkpoint remains `STAGED — PENDING EXACT-HEAD VALIDATION`.
