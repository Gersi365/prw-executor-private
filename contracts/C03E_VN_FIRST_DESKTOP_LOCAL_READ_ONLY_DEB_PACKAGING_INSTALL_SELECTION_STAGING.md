# C03e-VN — First Desktop Local Read-Only Debian Packaging / Install Selection

Status: `SELECTION STAGING — NO BUILD — NO INSTALL — NO LAUNCH`

Boundary:
`FIRST_DESKTOP_LOCAL_READ_ONLY_DEB_PACKAGING_INSTALL_SELECTION`

Date: 2026-09-18

Repository: `Gersi365/prw-executor-private`

## 1. Exact predecessor and sequencing decision

This checkpoint is based on evidence-closed C03e-VM:

- predecessor branch: `phase-152-c03e-vm-configured-remote-candidate-bundle-selection-preflight`;
- exact predecessor head: `9d988a8557d5bf9d0f309477e2750c3321fead52`;
- exact predecessor tree: `82664760768745ef29b95ff1aa18f04031ae43ad`;
- predecessor PR: #703, draft/open/unmerged;
- canonical VM Drive evidence ID: `1p7y4DF74kby1eWWvQwYmhIjwJoyCh26G`.

C03e-VM remains authoritative for configured-remote values. Its six-value blocker is not
resolved, overridden, inferred, or bypassed here.

The user subsequently clarified the product sequencing need: there is no installed PC
application yet, so configured-remote value selection is premature as the immediate
user-facing next step. C03e-VN therefore selects an orthogonal local desktop packaging
boundary while preserving the VM configured-remote STOP unchanged.

## 2. Fresh exact-source desktop facts

At the exact VM head:

- `apps/desktop/Cargo.toml` blob:
  `28c8c628651b92c5e62ed0ee97fb059b6037918e`;
- package name: `prw-desktop`;
- workspace version: `0.1.0`;
- GTK Rust binding: `gtk4 = 0.11.3`, feature floor `v4_14`;
- libadwaita Rust binding: `libadwaita = 0.9.2`, feature floor `v1_5`;
- `apps/desktop/README.md` blob:
  `78cdbf538f9d5edb36804860549a9ba4aba319f6`;
- `apps/desktop/src/main.rs` blob:
  `7a16d10a50b87a8ec328c38bc0ddeed3f70f1d51`;
- application ID: `io.patchmirror.prw.desktop`.

The current desktop application is a native Rust/GTK4/libadwaita process. Its active
runtime behavior is intentionally narrow:

- user-launched only;
- reads the local Agent through
  `$XDG_RUNTIME_DIR/private-remote-workspace/agent.sock`;
- uses the existing same-user Unix-socket security boundary;
- reads only Agent status and private-DNS summary;
- performs no TCP networking;
- performs no D-Bus fallback;
- performs no shell-command fallback;
- does not install, start, stop, restart, or replace the Agent;
- does not activate terminal/file/transfer/forwarding operations;
- does not activate production remote networking.

The existing Phase 152 management and command-3 source remains dormant and is not part
of this packaging selection.

## 3. Fresh target-host compatibility observations

Read-only inspection of the authorized PowerCode host on 2026-09-18 observed:

- OS: Ubuntu 26.04;
- architecture: `amd64`;
- `libgtk-4-1` version `4.22.4+ds-0ubuntu0.1` installed;
- `libadwaita-1-0` version `1.9.1-0ubuntu0.1` installed;
- `libgtk-4-dev` version `4.22.4+ds-0ubuntu0.1` installed;
- `libadwaita-1-dev` version `1.9.1-0ubuntu0.1` installed;
- `pkg-config` version `2.5.1-4` installed;
- GNOME Shell `50.1-0ubuntu1.2` installed;
- `ubuntu-desktop-minimal` installed;
- `/usr/share/applications` exists;
- `/home/gersi365/.local/share/applications` exists;
- no installed `prw-desktop` executable or PRW desktop launcher was identified by the
  preceding read-only desktop audit.

These observations establish compatibility context only. They do not authorize install,
package-manager, launcher, desktop-session, or Agent mutation.

## 4. Selected first distributable format

The first desktop distribution unit is selected as one native Debian binary package:

`private-remote-workspace-desktop_<version>-<revision>_amd64.deb`

For the current workspace version, the first candidate package identity is:

- Debian package: `private-remote-workspace-desktop`;
- upstream version source: exact workspace package version `0.1.0`;
- first Debian revision: `1`;
- candidate Debian version: `0.1.0-1`;
- architecture: `amd64`.

This first package is intentionally Ubuntu/Debian-family and amd64 only. Multi-arch,
Flatpak, AppImage, Snap, Windows, macOS, mobile distribution, store publication, signing,
auto-update, and repository publication remain separate future boundaries.

## 5. Selected build environment

The immediate packaging successor must build the package on a disposable CI runner, not
on the production PowerCode filesystem.

Selected build baseline:

- GitHub Actions disposable Linux runner;
- Ubuntu 24.04 baseline for the first amd64 package;
- repository-locked Rust toolchain;
- `cargo build --locked --release -p prw-desktop`;
- native build prerequisites limited to the already-required GTK/libadwaita toolchain
  plus standard Debian packaging validation tools.

The Ubuntu 24.04 build floor is selected so the package is built against the lower
GTK/libadwaita ABI floor already required by the crate and is expected to remain usable
on the observed newer Ubuntu 26.04 target. Exact binary dependency validation remains
mandatory before any install.

No new generic packaging framework or Rust packaging dependency is selected. In
particular, C03e-VN does not select `cargo-deb`, Flatpak tooling, AppImage tooling,
Snapcraft, or an updater framework.

## 6. Selected package payload

The first package payload is deliberately minimal and contains no service or background
component.

Selected installed payload:

1. executable:
   `/usr/lib/private-remote-workspace/prw-desktop`
   - owner after package installation: `root:root`;
   - mode: `0755`;
   - regular file;
   - no setuid bit;
   - no setgid bit;
   - no file capabilities.

2. desktop launcher:
   `/usr/share/applications/io.patchmirror.prw.desktop.desktop`
   - owner after package installation: `root:root`;
   - mode: `0644`;
   - regular file.

The first package intentionally contains no:

- systemd service or timer;
- autostart entry;
- D-Bus service file;
- polkit policy;
- sudoers entry;
- shell wrapper;
- symlink into `/usr/bin`;
- credential;
- private key;
- Agent configuration;
- `30-agent-execution-mode.conf`;
- `40-configured-remote-inputs.conf`;
- network configuration;
- firewall rule;
- DNS configuration;
- updater;
- package repository configuration;
- telemetry or analytics component.

## 7. Selected desktop-entry semantics

The future launcher must remain a direct exact-path launch boundary.

Selected semantic fields:

- `Type=Application`;
- `Name=Private Remote Workspace`;
- `Exec=/usr/lib/private-remote-workspace/prw-desktop`;
- `TryExec=/usr/lib/private-remote-workspace/prw-desktop`;
- `Terminal=false`;
- `StartupNotify=true`;
- application identity remains `io.patchmirror.prw.desktop`.

The launcher must not invoke a shell, `env`, `sh -c`, command substitution, a helper
script, a privileged wrapper, or an alternate executable path.

No icon asset is selected for the first package. A product icon is a separately reviewable
UI asset and must not be fabricated merely to satisfy packaging.

No autostart behavior is selected. The application starts only from an explicit user
launch or an explicitly executed test launch in a separately authorized checkpoint.

## 8. Runtime dependency law

The package must not bundle GTK, libadwaita, glibc, or unrelated system libraries.

Before package construction closes, the exact release ELF must be inspected and the
package dependency set must be derived from its actual dynamic-library requirements using
standard Debian tooling. The final `Depends` field must not be guessed manually.

At minimum, compatibility validation must prove that the resulting binary's GTK and
libadwaita requirements do not exceed the selected feature floors:

- GTK 4.14 or compatible newer runtime;
- libadwaita 1.5 or compatible newer runtime.

The observed PowerCode runtime is newer than both floors, but installation readiness is
not established until the exact built binary and exact package metadata are validated.

## 9. Package-construction constraints

The immediate source-materialization successor may add only the minimal packaging inputs
needed to produce and validate this one package.

Selected implementation principles:

- standard Debian tooling only;
- no maintainer scripts unless a concrete validation defect proves one indispensable and
  a new selection explicitly authorizes it;
- no `preinst`, `postinst`, `prerm`, or `postrm` in the selected first package;
- deterministic package name/version/architecture;
- exact source commit recorded in build evidence;
- exact release binary SHA-256 recorded before packaging;
- exact `.deb` SHA-256 recorded after packaging;
- `dpkg-deb --info` and `dpkg-deb --contents` inspected;
- desktop entry validated with `desktop-file-validate`;
- extracted package tree checked for exact path, owner-intent, and mode;
- no unexpected payload path admitted.

The packaging implementation must fail closed if the release binary is absent, if the
desktop-entry validation fails, if package contents differ from the selected ceiling, or
if dynamic dependencies cannot be mapped to explicit Debian package dependencies.

## 10. Artifact custody

The first package artifact is a build artifact, not an installed production object.

The packaging successor must retain:

- exact source head;
- exact source tree;
- exact desktop source blobs;
- exact build workflow/run/job identity;
- exact release binary size and SHA-256;
- exact Debian package filename, size and SHA-256;
- exact package metadata and payload listing;
- validation result.

A successful build artifact does not authorize installation.

## 11. Future install transaction selected in principle, not authorized

A later, separately authorized install transaction may consume only an exact validated
`.deb` artifact whose hash is bound by evidence.

That future install checkpoint must:

1. revalidate the exact package SHA-256 before installation;
2. inspect package metadata and payload again before mutation;
3. prove no existing conflicting payload path;
4. install through the system package manager as one bounded transaction;
5. verify installed binary exact bytes/hash against the package payload;
6. verify launcher exact bytes and mode;
7. verify no service/autostart/systemd/Agent/network state was created or changed;
8. stop before first application launch unless launch is separately authorized.

C03e-VN does not authorize any of those install actions.

## 12. Rollback / uninstall selection

The package manager owns the installed desktop payload.

A future uninstall must be bounded to removal of the selected package-owned files and
must not:

- stop or remove the headless PRW Agent;
- remove Agent configuration;
- remove Agent credentials;
- alter configured-remote values;
- alter networking;
- delete user data not owned by the package.

No cleanup script is selected at this stage.

## 13. Relationship to Agent and configured-remote state

The desktop package is not the Agent.

Installing or launching the desktop client must not imply any of:

- Agent replacement;
- Agent restart;
- Agent reconfiguration;
- command-3 activation;
- management provider activation;
- terminal/file/transfer/forwarding authority;
- remote listener activation;
- configured-remote activation;
- selection of any `PRW_REMOTE_*` value.

The existing Agent must remain authoritative and may remain in
`PRW_AGENT_EXECUTION_MODE=local_only`.

C03e-VM's blocker remains:

`EXPLICIT_CONFIGURED_REMOTE_SIX_VALUE_SELECTION`

and is intentionally deferred until a real remote-client/peer path is ready for that
decision.

## 14. Immediate next source ceiling

After C03e-VN selection closure, the next separately gated checkpoint may materialize only
the selected Debian desktop packaging inputs and disposable-CI package build/validation
path.

It may produce a candidate `.deb` artifact in CI.

It must not:

- install the package on PowerCode;
- write `/usr/lib/private-remote-workspace/prw-desktop` on PowerCode;
- write `/usr/share/applications/io.patchmirror.prw.desktop.desktop` on PowerCode;
- launch `prw-desktop` on PowerCode;
- mutate Agent files or service state;
- write managed `30` or `40`;
- activate command-3 management dispatch;
- select or invent configured-remote values;
- activate TCP/UDP remote networking;
- merge any PR;
- mark any draft PR ready;
- close or delete predecessor branches.

## 15. Selected closure classification

If exact-head CI and immutable evidence publication succeed, C03e-VN closes as:

`FIRST_DESKTOP_LOCAL_READ_ONLY_DEB_PACKAGING_INSTALL_SELECTION / NATIVE_DEB_AMD64_SELECTED / DISPOSABLE_UBUNTU_24_04_BUILD_SELECTED / EXACT_PATH_LAUNCHER_SELECTED / NO_AUTOSTART / NO_MAINTAINER_SCRIPTS / AGENT_LOCAL_ONLY_PRESERVED / CONFIGURED_REMOTE_DEFERRED / NO_BUILD_IN_THIS_CHECKPOINT / NO_INSTALL / NO_LAUNCH / NO_PRODUCTION_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_BEFORE_DESKTOP_DEB_PACKAGING_SOURCE_MATERIALIZATION_AND_BEFORE_ANY_PACKAGE_BUILD_INSTALL_OR_LAUNCH`

C03e-VN is selection-only. It authorizes no source materialization beyond this staging
contract, no build artifact, no host/package-manager mutation, no application launch, no
Agent mutation, no remote configuration, and no networking mutation.

`NO_RACE_FREE_CLAIM`
