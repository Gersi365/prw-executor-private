# Private Remote Workspace
# C03e-UX Production Device-Identity Provisioner Artifact Delivery Staging

Status: `SOURCE/DELIVERY MATERIALIZATION — RETAINED CHECKSUM-BOUND ARTIFACT TARGET — NO PRODUCTION HOST TRANSFER — NO REAL-ROOT INSTALL — NO IDENTITY PROVISIONING`

Audit date: 2026-09-18
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`
Boundary: `PRODUCTION_DEVICE_IDENTITY_PROVISIONER_ARTIFACT_DELIVERY`

## 1. Authority and predecessor

Authoritative predecessor is evidence-closed C03e-UW / PR #687 at exact head:

`5ff7ad3e2b962a7fbdb21fe3ef25ad629b97ee24`

C03e-UW proved:

- production destination `/usr/lib/private-remote-workspace/prw-device-identity-provision` was absent at read-only preflight;
- host destination and create-only installer mechanics were read-only ready;
- the current shell remained unprivileged and real-root installation was not authorized;
- dedicated C03e-UV validation had built exact provisioner bytes with SHA-256 `1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`;
- neither the dedicated UV run nor exact-head Rust run retained an Actions artifact;
- repository releases were absent;
- therefore production install was blocked on retained validated artifact delivery.

C03e-UX is limited to closing that artifact-retention/delivery gap.

## 2. Scope

C03e-UX materializes:

1. a permanent retained-artifact delivery contract;
2. a permanent bundle-preparation script;
3. a dedicated exact-head GitHub Actions artifact-delivery workflow;
4. packaging-boundary documentation for later consumption.

It does not change the Rust provisioner implementation, installer semantics, production host, device identity, service state, enrollment, or networking.

## 3. Exact build identity

Locked build command remains:

`cargo build --locked -p prw-device-identity-provisioning --bin prw-device-identity-provision`

Expected build output remains:

`target/debug/prw-device-identity-provision`

C03e-UX intentionally does not claim that a new retained build must match the historical non-retained UV SHA-256. The retained UX binary receives its own exact checksum and exact Git-source binding.

## 4. Exact-head checkout

The dedicated workflow must not rely on the default pull-request merge-ref checkout.

For pull requests it explicitly checks out:

`${{ github.event.pull_request.head.sha }}`

with fallback to `${{ github.sha }}` for non-PR `workflow_dispatch` runs.

The workflow then verifies:

`git rev-parse HEAD == expected source SHA`

and records that SHA in the artifact manifest.

## 5. Delivery bundle

The retained artifact root must contain exactly:

- `prw-device-identity-provision`;
- `SHA256SUMS`;
- `ARTIFACT-MANIFEST.txt`.

No other regular file, symlink, service unit, credential, drop-in, key material, enrollment payload, or network configuration is permitted.

The bundle preparation script requires an absent output directory and fails closed on an unsafe/missing source artifact or malformed source Git SHA.

## 6. Binary checksum binding

`SHA256SUMS` binds the exact retained binary bytes.

The manifest binds:

- format `PRW_C03E_UX_PROVISIONER_ARTIFACT_V1`;
- repository `Gersi365/prw-executor-private`;
- exact source Git SHA;
- Rust package and binary target;
- locked build command;
- build output path;
- retained filename;
- binary SHA-256;
- future production destination;
- explicit `NOT_AUTHORIZED` statements for production-host transfer, real-root install, provisioner execution and identity provisioning.

The preparation path verifies `sha256sum -c SHA256SUMS` before upload.

## 7. Packaging-mechanics reproof

The exact binary later retained by the UX workflow is first passed through the permanent C03e-UV disposable package validator in the same job.

That reproof preserves:

- locked dependency graph;
- exact target build;
- create-only installer mechanics;
- destination mode `0755` in disposable root;
- source/destination byte equality;
- duplicate create-only rejection;
- destination-symlink rejection;
- parent-chain symlink rejection;
- real-root opt-in and uid-0 guards;
- no provisioner execution.

## 8. Retention surface

Dedicated workflow:

`.github/workflows/phase-152-c03e-ux-provisioner-artifact-delivery.yml`

Artifact name format:

`c03e-ux-prw-device-identity-provision-<exact-source-git-sha>`

Upload surface:

`actions/upload-artifact@v7`

Locked upload behavior:

- missing files are fatal;
- retention is 90 days;
- compression level is 0;
- overwrite is false;
- hidden files are excluded.

The upload action exposes artifact ID, artifact URL and archive SHA-256 digest. Archive digest and retained binary SHA-256 are separate identities and both must be evidence-bound.

## 9. Post-run evidence requirement

C03e-UX cannot close on workflow success alone.

Evidence closure must freshly query the exact-final-head workflow run and:

- identify exactly the intended retained artifact;
- record artifact ID/name/size/creation/expiry metadata exposed by GitHub;
- record provider artifact digest when exposed;
- download the artifact archive;
- inspect the archive layout;
- verify exactly the three intended bundle files;
- verify `SHA256SUMS`;
- verify manifest source SHA equals the exact-final-head C03e-UX commit;
- verify manifest binary SHA equals the independently computed retained binary SHA;
- bind the artifact to exact-final-head CI and immutable Drive audit evidence.

## 10. Security and privilege boundary

Artifact retention does not grant production privilege.

No C03e-UX source or workflow step may:

- contact or mutate the production host;
- use the production Remote Desktop channel;
- execute a real-root installer;
- execute the provisioner;
- generate a P-256 production identity;
- run production `systemd-creds`;
- create, replace or delete the production credential;
- create, replace or delete PRW `20/30/40` drop-ins;
- mutate systemd manager/service state or linger;
- mutate enrollment or networking;
- create a public listener;
- create a GitHub Release or claim signed release provenance.

## 11. Retention limitations

The Actions artifact is intentionally a retained delivery surface, not permanent archival evidence.

The explicit 90-day retention means later production installation must re-check that the evidence-closed artifact still exists and has not expired or been deleted. If it is unavailable, production installation remains blocked until a separately evidence-closed artifact successor is produced.

Immutable audit evidence remains separately stored in the canonical Drive evidence folder.

## 12. Validation expectations

Exact-final-head closure requires:

- dedicated C03e-UX artifact-delivery workflow: `SUCCESS`;
- artifact upload step: `SUCCESS`;
- exact artifact visible through GitHub Actions artifact API;
- artifact download and independent checksum/layout verification: `PASS`;
- standard PRW Rust validation on the same exact head: `SUCCESS`;
- unrelated conditional workflows classified accurately if skipped;
- immutable Drive evidence publication and post-publication GitHub binding.

## 13. STOP

C03e-UX stops after retained artifact and evidence closure.

No artifact may be transferred or staged on the production host under this checkpoint.
No real-root install may be executed.
No provisioner may be executed.
No production device identity may be created.
No credential/drop-in, systemd/service, linger, enrollment or networking mutation is authorized.

After evidence-closed C03e-UX, the next safe checkpoint is a fresh production installation transaction preflight that re-proves artifact availability/checksum and host destination absence before requesting separate authorization for the create-only real-root install.

`NO_RACE_FREE_CLAIM`
