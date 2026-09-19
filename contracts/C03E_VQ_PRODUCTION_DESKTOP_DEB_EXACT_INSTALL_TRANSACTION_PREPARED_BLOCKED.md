# C03e-VQ — Production Desktop Debian Exact Installation Transaction

Status: `INSTALL_PREPARED — BLOCKED_ON_LOCAL_ROOT_AUTHENTICATION — NO_INSTALL_EXECUTED — NO_LAUNCH`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## 1. Exact predecessor

Evidence-closed C03e-VP is the exact predecessor:

- branch: `phase-152-c03e-vp-production-desktop-deb-artifact-transfer-staging`;
- exact head: `5419c78e61ae464478a12eaf8bcf5a9700b0a8f5`;
- exact tree: `03065dc0ac013cc457df00c9d35985de20382f3d`;
- PR #706: draft/open/unmerged;
- canonical completion evidence ID:
  `1B3WvFq_OMuR4swFwCH_jNzym0BRdCb1_`.

C03e-VP closed after exact artifact transfer/staging and before package installation or
desktop launch.

## 2. Exact staged candidate authority

Only this exact staged file is eligible for installation:

`/home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vp/staging/643f9bc942e94dbaad54050461f104117e08c2c3/private-remote-workspace-desktop_0.1.0-1_amd64.deb`

Required staged state:

- regular file;
- uid/gid `1000:1000`;
- mode `0600`;
- size `418670` bytes;
- SHA-256:
  `2f29a60187a00101e641499aea68b88abc6ca740bae39d9fc574eb31e5110c71`.

The enclosing stage must remain:

`/home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vp/staging/643f9bc942e94dbaad54050461f104117e08c2c3`

with directory mode `0700` and uid/gid `1000:1000`.

## 3. Exact package metadata

Fresh read-only preflight on PowerCode proved:

- Package: `private-remote-workspace-desktop`;
- Version: `0.1.0-1`;
- Architecture: `amd64`;
- Depends:
  `libadwaita-1-0 (>= 1.0.1), libc6 (>= 2.38), libgcc-s1 (>= 4.2), libglib2.0-0t64 (>= 2.54.0), libgtk-4-1 (>= 4.0.0)`.

The package data payload contains exactly:

1. `/usr/lib/private-remote-workspace/prw-desktop`
   - root:root
   - mode `0755`
   - exact packaged size `1373600` bytes
   - exact packaged binary SHA-256:
     `1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`.

2. `/usr/share/applications/io.patchmirror.prw.desktop.desktop`
   - root:root
   - mode `0644`
   - exact packaged size `202` bytes.

The package control archive contains only the regular `control` file. No `preinst`,
`postinst`, `prerm`, `postrm`, triggers, or other maintainer script is present.

## 4. Fresh install preflight

At `2026-09-19T08:38:55+02:00` on PowerCode:

- hostname: `PowerCode`;
- user: `gersi365`;
- uid/gid: `1000:1000`;
- exact stage: directory `0700`, uid/gid `1000:1000`;
- exact .deb: regular `0600`, uid/gid `1000:1000`, size `418670`;
- exact .deb SHA-256 matched authority;
- package was not installed;
- binary destination was absent and unowned by dpkg;
- launcher destination was absent and unowned by dpkg;
- Agent was `active`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- managed `40-configured-remote-inputs.conf` was absent;
- exact installed desktop executable process count was `0`.

All package dependencies had already been proven installed above the package minimums
during C03e-VO/C03e-VP readiness work.

## 5. Exact installation command

The only selected package-manager mutation is:

`/usr/bin/dpkg --install /home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vp/staging/643f9bc942e94dbaad54050461f104117e08c2c3/private-remote-workspace-desktop_0.1.0-1_amd64.deb`

It must execute with root authority obtained through normal local sudo authentication.

No `apt`, `apt-get`, `gdebi`, shell installer, curl pipe, package rebuild, alternate
.deb, or alternate install root is authorized.

## 6. Required pre-mutation revalidation

Immediately before privileged dpkg execution, re-prove:

- exact host/user/uid/gid;
- exact stage owner/mode;
- exact .deb type/owner/mode/size/hash;
- exact package metadata;
- control archive contains no maintainer script;
- package not already installed;
- both final payload paths absent and unowned;
- Agent active/local_only;
- managed 40 absent;
- exact desktop executable process count 0.

Any mismatch blocks installation.

## 7. Required post-install verification

After successful dpkg execution, prove:

- dpkg package status exactly `install ok installed`;
- installed version exactly `0.1.0-1`;
- installed architecture exactly `amd64`;
- `/usr/lib/private-remote-workspace/prw-desktop`:
  - regular non-symlink file;
  - root:root;
  - mode `0755`;
  - size `1373600`;
  - SHA-256
    `1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`;
- `/usr/share/applications/io.patchmirror.prw.desktop.desktop`:
  - regular non-symlink file;
  - root:root;
  - mode `0644`;
  - exact bytes equal the packaged launcher;
- dpkg owns both exact installed paths;
- Agent remains active/local_only;
- managed 40 remains absent;
- stage remains unchanged and checksum-valid;
- exact desktop executable process count remains `0`.

## 8. Launch remains forbidden

C03e-VQ installs the package only.

It must not:

- execute `prw-desktop`;
- open the desktop launcher;
- use `gtk-launch`;
- use `gio launch`;
- trigger autostart;
- create a service/timer/autostart unit;
- restart/reload/start/stop/reconfigure the Agent;
- write managed 30 or 40;
- activate command-3 management;
- select configured-remote values;
- activate TCP/UDP listeners;
- change DNS/firewall/routes;
- alter credentials/private keys.

First desktop launch is a separate later checkpoint.

## 9. Local root-authentication blocker

Fresh preflight executed:

`sudo -n true`

and returned:

`SUDO_NONINTERACTIVE=FAIL`

Therefore the connected session does not possess non-interactive root authority.

The user must not provide a sudo password in chat, in a GitHub secret, in Drive, in a
script, or through any assistant-visible channel.

No attempt may be made to bypass sudo policy, scrape credentials, change sudoers, weaken
authentication, install a privilege helper, or reuse an unrelated credential.

The install transaction remains blocked until the user performs the one local privileged
dpkg invocation or otherwise supplies a normal locally authenticated root action without
disclosing credentials to the assistant.

## 10. Prepared classification

`PRODUCTION_DESKTOP_DEB_EXACT_INSTALL_PREPARED / EXACT_STAGED_PACKAGE_REVALIDATED / PACKAGE_METADATA_AND_NO_MAINTAINER_SCRIPTS_VERIFIED / INSTALL_DESTINATIONS_ABSENT_AND_UNOWNED / AGENT_ACTIVE_LOCAL_ONLY_PRESERVED / BLOCKED_ON_LOCAL_ROOT_AUTHENTICATION / NO_INSTALL_EXECUTED / NO_LAUNCH / NO_AGENT_MUTATION / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_BLOCKED_ON_LOCAL_ROOT_AUTHENTICATION_BEFORE_EXACT_DPKG_INSTALL`

After local root authentication completes the exact selected dpkg installation, the next
continuation must perform read-only post-install verification and stop before first
desktop launch.

`NO_RACE_FREE_CLAIM`
