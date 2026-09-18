# PRW Device-Identity Provisioner Retained Artifact Delivery

Status: `C03E_UX_RETAINED_CHECKSUM_BOUND_PROVISIONER_ARTIFACT_DELIVERY_CONTRACT`

This contract defines a retained, checksum-bound delivery surface for the already-validated `prw-device-identity-provision` binary. It does not authorize production-host transfer, real-root installation, provisioner execution, identity provisioning, systemd mutation, or networking mutation.

## Authoritative predecessor

C03e-UX follows evidence-closed C03e-UW at exact head:

`5ff7ad3e2b962a7fbdb21fe3ef25ad629b97ee24`

C03e-UW proved that the production destination and create-only installer mechanics are read-only ready, while production installation remains blocked because the exact validated provisioner bytes were not retained in a canonical delivery surface.

## Exact build target

Rust package:

`prw-device-identity-provisioning`

Binary target:

`prw-device-identity-provision`

Locked build command remains:

```text
cargo build --locked -p prw-device-identity-provisioning --bin prw-device-identity-provision
```

Expected build output:

`target/debug/prw-device-identity-provision`

C03e-UX does not change build mode or claim reproducibility against an older non-retained build.

## Delivery bundle layout

The retained GitHub Actions artifact contains exactly three regular files at the artifact root:

1. `prw-device-identity-provision`
2. `SHA256SUMS`
3. `ARTIFACT-MANIFEST.txt`

No symlink, directory payload, service unit, credential, drop-in, enrollment material, signing key, or network configuration is part of the delivery bundle.

## Binary checksum binding

`SHA256SUMS` contains the SHA-256 of the retained `prw-device-identity-provision` bytes using the exact filename above.

`ARTIFACT-MANIFEST.txt` binds at minimum:

- format version;
- repository identity;
- exact checked-out Git commit SHA;
- Rust package and binary target;
- locked build command;
- retained binary filename;
- retained binary SHA-256;
- future locked production destination;
- explicit statements that production installation and provisioner execution remain unauthorized.

The preparation script must verify `sha256sum -c SHA256SUMS` before the bundle is uploaded.

## Exact-head checkout requirement

For `pull_request` runs, the delivery workflow must explicitly check out the pull request head SHA rather than relying on the default merge-ref checkout. For `workflow_dispatch`, the selected workflow commit is used.

The workflow records `git rev-parse HEAD` and passes that exact value to the bundle manifest.

## GitHub Actions retention surface

Permanent workflow source:

`.github/workflows/phase-152-c03e-ux-provisioner-artifact-delivery.yml`

Permanent bundle preparation source:

`scripts/prepare-phase-152-c03e-ux-provisioner-artifact.sh`

The workflow uses the current major `actions/upload-artifact@v7` surface with:

- deterministic artifact name containing the exact source commit SHA;
- `if-no-files-found: error`;
- explicit 90-day retention;
- compression level `0` so the binary is not needlessly recompressed;
- overwrite disabled.

The upload action exposes:

- artifact ID;
- artifact URL;
- SHA-256 artifact digest for the uploaded artifact archive.

The artifact archive digest is distinct from the provisioner binary SHA-256. Both identities must be recorded during C03e-UX evidence closure.

## Validation requirements

Before upload, the workflow must:

1. install the same protobuf build prerequisite used by the validated packaging flow;
2. verify the locked Cargo dependency graph;
3. run the permanent C03e-UV disposable packaging validator so the exact retained binary is the same build output that passed the create-only installer mechanics proof in that job;
4. prepare the three-file delivery bundle;
5. verify the bundle contains exactly the expected three regular non-symlink files;
6. verify `SHA256SUMS` against the retained binary;
7. verify retained binary bytes equal the built `target/debug/prw-device-identity-provision` bytes;
8. verify the manifest exact source Git SHA equals the checked-out head;
9. upload exactly that prepared bundle;
10. expose artifact ID, URL, and archive digest in the workflow log.

After the workflow succeeds, evidence closure must query the GitHub Actions artifact API, download the retained artifact, and independently verify its contents and checksums before classifying C03e-UX closed.

## Consumption boundary

A later production-install checkpoint may consume only an evidence-closed C03e-UX artifact whose:

- artifact ID and source workflow run are recorded;
- artifact has not expired or been deleted;
- downloaded archive digest matches the recorded artifact digest where exposed by the provider;
- bundle layout is exact;
- `SHA256SUMS` passes;
- binary SHA-256 equals the C03e-UX recorded binary SHA-256;
- manifest source Git SHA equals the evidence-closed C03e-UX head.

The later installer remains responsible for final production destination type, mode, ownership, absence, atomic no-replace publication and postconditions.

## Hard prohibitions

C03e-UX must not:

- transfer the artifact to the production host;
- stage bytes on the production host;
- execute the real-root installer;
- execute `prw-device-identity-provision`;
- generate a production device identity;
- call production `systemd-creds`;
- write or delete the production credential;
- write or delete PRW `20/30/40` drop-ins;
- call `systemctl` or `loginctl` on the production host;
- change linger, service state, enrollment or networking;
- create a GitHub Release or claim signed-release provenance;
- merge, close, or mark the PR ready without separate authorization.

## Success boundary

C03e-UX may be classified complete only when:

- exact-head dedicated artifact-delivery CI succeeds;
- standard exact-head Rust validation succeeds;
- a retained Actions artifact exists for the exact final head;
- the artifact is independently downloaded and checksum-verified;
- artifact ID, binary SHA-256, artifact digest, retention/expiry metadata and exact Git identity are evidence-bound;
- immutable Drive audit evidence is published and verified;
- post-publication GitHub closure binding is recorded.

Even after closure:

`RETAINED_ARTIFACT_READY / NO_PRODUCTION_HOST_TRANSFER / NO_REAL_ROOT_INSTALL / NO_PROVISIONER_EXECUTION / NO_IDENTITY_PROVISIONING`
