# C03e-VB Production Device-Identity Provisioning Execution Transaction

Status: `PRODUCTION_IDENTITY_PROVISIONED — ENCRYPTED_CREDENTIAL_AND_FIXED_20_DROPIN_CREATED — POST_EXECUTION_VERIFIED — EVIDENCE_PENDING — SERVICE_NOT_ACTIVATED — ENROLLMENT_NOT_PERFORMED — NETWORKING_NOT_MUTATED`

Boundary: `PRODUCTION_DEVICE_IDENTITY_PROVISIONING_EXECUTION_TRANSACTION`

## Authority

Authoritative predecessor is evidence-closed C03e-VA / PR #691 at exact head `c91f9b5940269ecad89782590bdc82ca6ad96bae`.

C03e-VA closed with `READY_FOR_SEPARATELY_AUTHORIZED_PROVISIONER_EXECUTION` and `STOP_BEFORE_PROVISIONER_EXECUTION_AND_PRODUCTION_IDENTITY_CREATION`.

The user explicitly authorized C03e-VB by saying `Vazhdo` after the proposed exact checkpoint:
- fresh immediate binary/source/target reproof;
- one production execution of the installed provisioner with zero arguments as `gersi365`;
- post-execution metadata and fixed-drop-in verification;
- no private-key decryption;
- no service activation;
- no enrollment;
- no networking mutation.

## Fresh duplicate and predecessor guards

Before the production transaction:
- no existing C03e-VB branch was found;
- no existing C03e-VB PR was found;
- no canonical Drive audit with the target C03e-VB title was found;
- PR #691 remained open/draft/unmerged/mergeable at exact VA head.

## Immediate pre-execution guards

The transaction used the production host `PowerCode` as user `gersi365`.

Exact installed provisioner:
`/usr/lib/private-remote-workspace/prw-device-identity-provision`

Immediate guards required and passed in the same shell before execution:
- caller username exactly `gersi365`;
- caller uid exactly `1000`;
- `HOME=/home/gersi365`;
- provisioner regular non-symlink file;
- provisioner mode/uid/gid/size exactly `0755 / 0 / 0 / 55826168`;
- provisioner SHA-256 exactly `1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`;
- source-defined encrypted credential target absent and not a symlink;
- source-defined `20-device-identity-credential.conf` target absent and not a symlink;
- `30-local-only.conf` absent and not a symlink;
- `40-production-network.conf` absent and not a symlink;
- credential temp-residue count zero;
- drop-in temp-residue count zero;
- existing target ancestor components were non-symlink;
- `/usr/bin/systemd-creds` regular non-symlink file;
- `/usr/bin/systemd-creds` SHA-256 exactly `f5c5f7edac86be6fba6409c52d76cd3159e95ac5d7a45bf7c715b3e65a1e1702`;
- `/etc/machine-id` present, non-empty and 32-hex formatted without recording the value;
- exact provisioner process count zero.

`XDG_STATE_HOME` and `XDG_CONFIG_HOME` were explicitly unset for the execution and `HOME` was explicitly set to `/home/gersi365`, preserving the source-defined production targets established by C03e-VA.

The immediate guard emitted:
`VB_PRECHECK=PASS`

## Production provisioner execution

The installed production provisioner was executed exactly once with zero arguments:

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

No sudo invocation was used for provisioning.

The process completed with exit code `0`.

The provisioner emitted bounded non-secret success output:

`prw-device-identity-provision event=provisioned public_spki_sha256=b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d encrypted_credential_path=/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

Public SPKI SHA-256:
`b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d`

No private key bytes were printed, copied, exported, read back or decrypted by this checkpoint.

## Post-execution encrypted credential verification

Canonical encrypted credential path:
`/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

Fresh post-execution metadata:
- type: regular file;
- mode: `0600`;
- uid/gid: `1000:1000`;
- size: `762` bytes;
- device: `66306`;
- inode: `6181707`.

Credential parent directory:
`/home/gersi365/.local/state/private-remote-workspace/credentials`

Fresh directory metadata:
- type: directory;
- mode: `0700`;
- uid/gid: `1000:1000`;
- device: `66306`;
- inode: `6181601`.

The encrypted credential contents were deliberately not read, hashed, decrypted, exported or copied into evidence.

## Post-execution fixed credential drop-in verification

Canonical `20` drop-in path:
`/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`

Fresh metadata:
- type: regular file;
- mode: `0600`;
- uid/gid: `1000:1000`;
- size: `170` bytes;
- device: `66306`;
- inode: `6321554`.

Drop-in directory metadata:
- type: directory;
- mode: `0700`;
- uid/gid: `1000:1000`;
- device: `66306`;
- inode: `6321550`.

The on-disk `20` drop-in was compared without modification to the exact expected non-secret payload:

`[Service]`
`LoadCredentialEncrypted=prw.device-identity.private-key.v1:/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

Exact payload comparison: `PASS`.

Drop-in SHA-256:
`42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`

## Negative post-execution guards

Still absent:
- `/home/gersi365/.config/systemd/user/prw-agent.service.d/30-local-only.conf`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d/40-production-network.conf`.

Post-execution residue counts:
- credential temp residues: `0`;
- drop-in temp residues: `0`.

Exact provisioner process count after completion: `0`.

Installed provisioner SHA-256 remained:
`1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`

Fresh user-systemd read-only state via the existing `/run/user/1000/bus` remained:
- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `Result=exit-code`;
- `NRestarts=6`;
- `DropInPaths=` empty;
- `Linger=yes`.

The empty manager-visible `DropInPaths` is preserved as an observed fact. C03e-VB did not invoke daemon-reload or any mutating systemctl operation.

No service start/restart/enable/disable mutation occurred.
No linger mutation occurred.
No enrollment mutation occurred.
No network/DNS/forwarding/relay mutation occurred.
No `30` or `40` drop-in was created.

Post-execution guard emitted:
`VB_POSTCHECK=PASS`

## Security boundary

C03e-VB proves only that the source-defined creation-only provisioner completed successfully and persisted the encrypted credential plus exact fixed `20` binding under the verified production paths.

C03e-VB does not claim:
- plaintext/private-key recovery;
- independent decryption of the persisted credential;
- enrollment completion;
- Agent credential load into a running process;
- service activation;
- remote-network activation;
- network reachability.

The encrypted credential remains opaque to this audit.

## Git scope

This checkpoint is docs/evidence only in Git.
No Rust source, Cargo graph, provisioner binary, installer, workflow, service unit, enrollment source or network source is changed by the C03e-VB Git lineage.

## Evidence target

Canonical audit filename:
`C03E_VB_PRODUCTION_DEVICE_IDENTITY_PROVISIONING_EXECUTION_TRANSACTION_AUDIT_2026-09-18.md`

Canonical evidence folder:
`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The audit must preserve:
- exact Git head/tree/base;
- exact CI results;
- public SPKI SHA-256;
- credential and drop-in metadata only;
- exact non-secret `20` payload verification;
- negative `30/40`, service, enrollment and networking guards;
- explicit statement that encrypted credential contents were not read/decrypted/exported.

## Classification and STOP

Technical classification before evidence closure:

`PRODUCTION_IDENTITY_PROVISIONED / ENCRYPTED_CREDENTIAL_CREATED_AND_METADATA_VERIFIED / FIXED_20_DROPIN_CREATED_AND_EXACT_PAYLOAD_VERIFIED / 30_40_ABSENT / SERVICE_NOT_ACTIVATED / ENROLLMENT_NOT_PERFORMED / NETWORKING_NOT_MUTATED / EVIDENCE_PENDING`

STOP:
`STOP_BEFORE_USER_SYSTEMD_MANAGER_RELOAD_OR_AGENT_ACTIVATION_AND_BEFORE_ENROLLMENT_OR_NETWORKING`

Any daemon-reload, service start/restart, credential-load runtime validation, enrollment, revocation, production network activation, relay, forwarding or DNS mutation remains separately gated.

`NO_RACE_FREE_CLAIM`
