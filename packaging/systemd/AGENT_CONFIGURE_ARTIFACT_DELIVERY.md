# PRW Agent Configure Retained Artifact Delivery

Status: `C03E_VG_RETAINED_CHECKSUM_BOUND_AGENT_CONFIGURE_ARTIFACT_DELIVERY_CONTRACT`

This contract defines a retained, checksum-bound delivery surface for `prw-agent-configure`. It does not authorize production-host transfer, production staging, real-root installation, managed configuration writes, user-systemd manager reload, Agent activation, credential operations, enrollment, or networking mutation.

## Authoritative predecessor

C03e-VG follows evidence-closed C03e-VF at exact head:

`ce4291330969cb6a213db9efe2ea8ff725084935`

C03e-VF proved that the production destination is filesystem-ready and that the validated artifact identity is known, but no retained evidence-controlled delivery object exists. VF therefore stopped before production transfer, staging or installation.

## Exact build target

Rust package:

`prw-agent-systemd-orchestration`

Binary target:

`prw-agent-configure`

Locked build command:

```text
cargo build --locked -p prw-agent-systemd-orchestration --bin prw-agent-configure
```

Expected build output:

`target/debug/prw-agent-configure`

C03e-VG produces a fresh retained build at the exact VG source head. It does not claim byte reproducibility against the earlier ephemeral C03e-VE build. The retained C03e-VG binary SHA-256 becomes the delivery identity for any later separately authorized production-transfer/install checkpoint.

## Delivery bundle layout

The retained GitHub Actions artifact contains exactly three regular files at the artifact root:

1. `prw-agent-configure`
2. `SHA256SUMS`
3. `ARTIFACT-MANIFEST.txt`

No symlink, directory payload, service unit, credential, user-systemd drop-in, enrollment material, signing key, or network configuration is part of the delivery bundle.

## Binary checksum binding

`SHA256SUMS` contains the SHA-256 of the retained `prw-agent-configure` bytes using the exact filename above.

`ARTIFACT-MANIFEST.txt` binds:

- format version;
- repository identity;
- exact checked-out Git commit SHA;
- Rust package and binary target;
- locked build command and output path;
- retained binary filename and SHA-256;
- future locked production destination;
- explicit `NOT_AUTHORIZED` markers for production transfer, production staging, real-root installation, managed configuration write, daemon reload and Agent activation.

The preparation script verifies `sha256sum -c SHA256SUMS` before upload.

## Exact-head checkout requirement

For `pull_request` runs, the workflow explicitly checks out the pull request head SHA instead of the default merge-ref. For `workflow_dispatch`, the selected workflow commit is used.

The workflow records `git rev-parse HEAD` and passes that exact value into the manifest.

## GitHub Actions retention surface

Permanent workflow source:

`.github/workflows/phase-152-c03e-vg-agent-configure-artifact-delivery.yml`

Permanent bundle preparation source:

`scripts/prepare-phase-152-c03e-vg-agent-configure-artifact.sh`

The workflow uses pinned `actions/upload-artifact` with:

- deterministic artifact name containing the exact source commit SHA;
- `if-no-files-found: error`;
- explicit 90-day retention;
- compression level `0`;
- overwrite disabled;
- hidden files excluded.

The upload action exposes artifact ID, artifact URL and archive-level SHA-256 artifact digest. The archive digest is distinct from the retained binary SHA-256 and both identities must be evidence-bound.

## Validation requirements

Before upload, the workflow must:

1. install protobuf tooling used by the validated packaging flow;
2. verify the locked Cargo dependency graph;
3. run `scripts/validate-phase-152-c03e-ve-agent-configure-packaging.sh` so the exact retained build output passes the permanent create-only package transaction proof in the same job;
4. prepare the three-file delivery bundle;
5. verify exact three-file regular non-symlink layout;
6. verify `SHA256SUMS`;
7. verify retained binary bytes equal `target/debug/prw-agent-configure`;
8. verify manifest source Git SHA equals the exact checked-out head;
9. verify all hard-boundary `NOT_AUTHORIZED` markers;
10. upload exactly the prepared bundle;
11. expose artifact ID, URL, archive digest and retained binary SHA-256 in the workflow log.

After success, evidence closure must query the GitHub Actions artifact API, download the retained archive, and independently verify bundle layout, checksums, manifest source identity and archive identity before classifying C03e-VG closed.

## Consumption boundary

A later production-transfer/install checkpoint may consume only an evidence-closed C03e-VG artifact whose:

- artifact ID and source workflow run are recorded;
- artifact has not expired or been deleted;
- downloaded archive digest matches the recorded provider digest where independently reproducible/exposed;
- bundle layout is exact;
- `SHA256SUMS` passes;
- binary SHA-256 equals the C03e-VG recorded retained binary SHA-256;
- manifest source Git SHA equals the evidence-closed C03e-VG head.

The later install checkpoint remains responsible for fresh production destination type, ownership, absence, no-replace publication and postconditions.

## Hard prohibitions

C03e-VG must not:

- transfer artifact bytes to PowerCode;
- stage artifact bytes on PowerCode;
- execute the real-root installer;
- execute `prw-agent-configure`;
- execute `write` or `reconfigure-active`;
- create, replace or remove managed `20`, `30`, or `40` user-systemd drop-ins;
- call production `systemctl`, `loginctl`, or `daemon-reload`;
- start, restart, stop, enable or disable the Agent;
- read/decrypt/replace private credential material;
- mutate enrollment, revocation, network, DNS, forwarding or relay state;
- create a GitHub Release or claim signed-release provenance;
- merge, close or mark the PR ready without separate authorization.

## Success boundary

C03e-VG may be classified complete only when:

- exact-head dedicated artifact-delivery CI succeeds;
- standard exact-head Rust validation succeeds;
- a retained Actions artifact exists for the exact final head;
- the artifact is independently downloaded and checksum-verified;
- artifact ID, binary SHA-256, archive digest, retention metadata and exact Git identity are evidence-bound;
- immutable Drive audit evidence is published and verified;
- post-publication GitHub closure binding is recorded.

Even after closure:

`RETAINED_AGENT_CONFIGURE_ARTIFACT_READY / NO_PRODUCTION_TRANSFER / NO_PRODUCTION_STAGING / NO_REAL_ROOT_INSTALL / NO_MANAGED_CONFIGURATION_WRITE / NO_DAEMON_RELOAD / AGENT_NOT_ACTIVATED`
