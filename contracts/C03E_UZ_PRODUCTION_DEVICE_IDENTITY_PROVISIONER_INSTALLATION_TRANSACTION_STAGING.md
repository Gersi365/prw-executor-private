# C03e-UZ — Production Device-Identity Provisioner Installation Transaction

Status: `TRANSACTION STARTED — RETAINED ARTIFACT REPROVED — PRODUCTION ARTIFACT STAGED AND CHECKSUM-VERIFIED — BLOCKED ON USER-ATTENDED SUDO AUTHENTICATION — REAL-ROOT INSTALL NOT COMPLETED — PROVISIONER NOT EXECUTED`

Boundary: `PRODUCTION_DEVICE_IDENTITY_PROVISIONER_INSTALLATION_TRANSACTION`

Audit date: 2026-09-18

## Authority

Authoritative predecessor is evidence-closed C03e-UY / PR #689 at exact head:

`57189763a42f1ccf78a3f8ecc5483f0d75a2eaec`

C03e-UY classified the production host as read-only preflight-ready for a separately authorized create-only install transaction and left a hard STOP before artifact transfer/staging and real-root install.

The user explicitly authorized the previously proposed C03e-UZ checkpoint. C03e-UZ scope is limited to:

- immediate retained-artifact and production-destination reproof;
- transfer/staging of the exact checksum-bound artifact on the production host;
- host-side ZIP, bundle and binary checksum verification;
- real-root create-only install through the permanent installer if required sudo authentication is available;
- post-install byte/mode/ownership verification;
- no provisioner execution;
- no identity provisioning;
- no credential, PRW `20/30/40`, systemd/service, linger, enrollment or networking mutation.

## Fresh predecessor and duplicate recovery

Before mutation, fresh reads confirmed:

- C03e-UY PR #689 remained open, draft, unmerged and mergeable;
- C03e-UY exact head remained `57189763a42f1ccf78a3f8ecc5483f0d75a2eaec`;
- C03e-UY exact tree remained `bc380d1065e8ebfddecccae2df34b3c41e8cf1d7`;
- no C03e-UZ branch existed before branch creation;
- no C03e-UZ PR existed;
- no canonical Drive evidence collision for C03e-UZ existed.

## Permanent installer identity

Exact predecessor installer source remains:

`packaging/systemd/install-prw-device-identity-provision.sh`

Installer blob SHA:

`a2bf55f790469a08ef035bcdc90446be297a3751`

Locked production destination:

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

The installer remains create-only, requires explicit `--allow-root-filesystem` for `ROOT=/`, requires uid 0 in real-root mode, stages into the locked destination parent, publishes with atomic same-filesystem no-replace hard-link semantics, verifies installed bytes/mode, verifies root ownership for real-root mode, and never executes the provisioner.

## Immediate pre-mutation host guard

Immediately before staging, the production host was freshly checked read-only.

Observed:

- shell user `gersi365`, uid/gid `1000:1000`;
- locked production destination absent;
- installer temp residue count `0`;
- `/usr/lib/private-remote-workspace` remained directory mode `0755`, uid/gid `0:0`, device `66306`, inode `3035841`;
- parent filesystem `/dev/nvme0n1p2`, ext4, mounted `rw,relatime`;
- required command surfaces `curl`, `sha256sum`, `unzip`, `bash`, `cp`, `chmod`, `ln`, `mktemp`, `cmp`, `stat` were present;
- `sudo -n true` returned false, proving no non-interactive cached/NOPASSWD sudo authorization was available in this shell context.

C03e-UZ did not infer or invent a password and did not substitute another privilege-escalation mechanism.

## Fresh retained artifact reproof immediately before transfer

C03e-UZ freshly queried exact C03e-UX workflow run `35331996301` and confirmed final retained artifact:

- artifact ID `10541149499`;
- name `c03e-ux-prw-device-identity-provision-bf8ba5d770f41b02b752cc75ac1f3439a1243c44`;
- source Git head `bf8ba5d770f41b02b752cc75ac1f3439a1243c44`;
- size `55827441` bytes;
- expired `false`;
- expiry `2026-12-17T09:54:49Z`;
- provider archive SHA-256 `1514905144b902bdc6e8059c3fd9d6c96e3954b77b54d8348626cba9f4a5e3d2`.

## Authorized production-host staging mutation

C03e-UZ created/used the controlled user-owned staging hierarchy:

`/home/gersi365/.local/state/private-remote-workspace/staging/c03e-uz-10541149499`

Staging directory metadata after verification:

- regular directory, never observed as symlink;
- mode `0700`;
- uid/gid `1000:1000`;
- device `66306`;
- inode `6181640`.

The exact retained ZIP bytes were transferred to:

`/home/gersi365/.local/state/private-remote-workspace/staging/c03e-uz-10541149499/c03e-uz-authorized-artifact.zip`

Observed ZIP metadata:

- regular file;
- mode `0600`;
- uid/gid `1000:1000`;
- size `55827441` bytes;
- inode `6181646`;
- SHA-256 `1514905144b902bdc6e8059c3fd9d6c96e3954b77b54d8348626cba9f4a5e3d2`.

Host-side ZIP SHA-256 exactly matched the freshly queried provider archive digest before extraction.

Archive layout was required to contain exactly three top-level names:

1. `ARTIFACT-MANIFEST.txt`
2. `SHA256SUMS`
3. `prw-device-identity-provision`

Those exact objects were extracted by explicit entry name into the controlled staging directory.

Staged metadata files:

- `ARTIFACT-MANIFEST.txt`: regular file, mode `0600`, uid/gid `1000:1000`, size `759`, inode `6181611`;
- `SHA256SUMS`: regular file, mode `0600`, uid/gid `1000:1000`, size `96`, inode `6181662`.

Staged binary:

`/home/gersi365/.local/state/private-remote-workspace/staging/c03e-uz-10541149499/prw-device-identity-provision`

Observed:

- regular non-symlink file;
- mode `0755`;
- uid/gid `1000:1000`;
- size `55826168` bytes;
- device `66306`;
- inode `6181663`;
- SHA-256 `1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`.

Host-side `sha256sum -c SHA256SUMS` returned:

`prw-device-identity-provision: OK`

Manifest readback bound:

- `format=PRW_C03E_UX_PROVISIONER_ARTIFACT_V1`;
- `repository=Gersi365/prw-executor-private`;
- `source_git_sha=bf8ba5d770f41b02b752cc75ac1f3439a1243c44`;
- `binary_sha256=1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`;
- `future_production_destination=/usr/lib/private-remote-workspace/prw-device-identity-provision`.

The immutable UX manifest still contains its historical `NOT_AUTHORIZED` markers for production transfer/install/execution/identity provisioning. Those markers are preserved unchanged; they do not themselves grant later authority. C03e-UZ transfer/staging authority came from the explicit successor checkpoint authorization after C03e-UY closure.

## Staging command behavior note

The first staging command successfully completed:

- ZIP transfer;
- ZIP size/digest validation;
- exact three-entry layout check;
- explicit extraction;
- staged binary size/digest validation;
- `sha256sum -c` PASS;
- source SHA and binary SHA manifest checks.

It then exited code `1` only because a final diagnostic grep used the wrong manifest key name `future_destination=`. The immutable manifest uses `future_production_destination=`.

A second read-only staging reproof used the exact manifest key and returned `C03E_UZ_STAGING=PASS`. No staged bytes were rewritten by that correction.

## Immediate post-staging production-destination guard

After staging verification and before any real-root install attempt:

`/usr/lib/private-remote-workspace/prw-device-identity-provision` remained absent.

`sudo -n true` again returned false.

Therefore the real-root installer was not invoked.

## Privilege boundary and blocker

C03e-UZ is blocked at:

`BLOCKED_ON_USER_ATTENDED_SUDO_AUTHENTICATION_BEFORE_REAL_ROOT_INSTALL`

The blocker is not artifact identity, destination safety, filesystem readiness, installer mechanics or checksum validation. Those were freshly re-proved.

The blocker is that the authorized remote shell has no non-interactive sudo authorization. C03e-UZ will not request, receive, store, log or transmit a sudo password through project evidence or chat, and will not substitute a different privilege escalation path merely to bypass the selected privilege boundary.

## Mutation statement

C03e-UZ did mutate the production host only by creating and populating the controlled user-owned staging path described above.

C03e-UZ did not:

- create or alter `/usr/lib/private-remote-workspace/prw-device-identity-provision`;
- invoke the permanent installer in real-root mode;
- invoke `prw-device-identity-provision`;
- generate, rotate, replace or import a production identity key;
- invoke production `systemd-creds`;
- create, replace or delete the encrypted identity credential;
- create, replace or delete PRW `20/30/40` drop-ins;
- invoke `systemctl` or `loginctl`;
- mutate systemd manager or service state;
- mutate linger;
- mutate enrollment or networking;
- alter the existing Agent payload;
- clean up or delete the staged transaction bytes.

## Current classification

`PRODUCTION_ARTIFACT_STAGED_AND_CHECKSUM_VERIFIED / REAL_ROOT_INSTALL_BLOCKED_ON_USER_ATTENDED_SUDO_AUTH / PROVISIONER_NOT_EXECUTED / IDENTITY_NOT_PROVISIONED`

This is not a successful installation claim.

## STOP

`STOP_BEFORE_REAL_ROOT_CREATE_ONLY_INSTALL_PENDING_VALID_SUDO_AUTHENTICATION`

A continuation may only resume from the exact staged artifact after freshly re-verifying:

- staged ZIP and binary SHA-256;
- `SHA256SUMS` PASS;
- manifest source/binary/destination bindings;
- production destination still absent;
- installer temp residue absent;
- sudo authorization available in the same execution context.

If those pass, the next allowed mutation is exactly the already authorized create-only real-root installer invocation followed by final file type, mode, ownership and byte/hash verification.

Even after a successful install, STOP remains before provisioner execution and identity provisioning.

`NO_RACE_FREE_CLAIM`
