# C03e-WB — WA Agent Production Activation Transaction Selection

Status: `SELECTION — DOCUMENTATION_ONLY — PRODUCTION_MUTATION_NOT_AUTHORIZED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`WA_AGENT_PRODUCTION_ACTIVATION_TRANSACTION_SELECTION`

## Exact predecessor authority

Evidence-closed C03e-WA is the exact predecessor:

- branch:
  `phase-152-c03e-wa-linux-bootstrap-agent-status-caller-source-materialization`
- exact final head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`
- exact final tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`
- PR #717 remains draft/open/unmerged
- canonical WA evidence ID:
  `1ys9XCrA8XfYaRLG7HY4QT0VUADLtcDAH`

WA changed repository source behavior only and explicitly stopped before any production install,
Agent restart or production command-3 probe.

## Fresh production baseline

Read-only PowerCode inspection at `2026-09-19T13:02:05+02:00` proved:

- service `prw-agent.service` active/running;
- `Result=success`;
- `NRestarts=0`;
- `MainPID=2983`;
- executable:
  `/usr/lib/private-remote-workspace/prw-agent`;
- installed Agent is root-owned mode `0755`;
- installed Agent bytes: `11068384`;
- installed Agent SHA-256:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- vendor unit:
  `/usr/lib/systemd/user/prw-agent.service`;
- manager-visible execution mode:
  `PRW_AGENT_EXECUTION_MODE=local_only`;
- fixed identity credential drop-in `20-device-identity-credential.conf` remains loaded;
- fixed local-only drop-in `30-agent-execution-mode.conf` remains loaded;
- managed `40-configured-remote-inputs.conf` remains absent.

The prior WA closure guard additionally retained exactly one desktop process, unchanged desktop
binary SHA-256
`1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`,
with no desktop TCP/UDP socket activity.

WB performs no host mutation.

## Candidate artifact truth

The exact WA Desktop Debian validation run is:

- workflow `361684968`;
- run `35437908261` / #18;
- exact WA head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- conclusion `success`.

Its sole retained GitHub Actions artifact is:

`c03e-vo-private-remote-workspace-desktop-0.1.0-1-amd64-fe24713c4a72e7e3ad6048f19df5352b8668f552`

- artifact ID `10583232043`;
- archive size `424869` bytes;
- GitHub artifact digest
  `sha256:803fe93f88f5e0997d4f32d10745ed65861ee3109889894220215c61579b2065`.

Direct artifact inspection proves the package is:

`private-remote-workspace-desktop_0.1.0-1_amd64.deb`

with exact package SHA-256:

`a49255621c2c8f0aca759ae49c0353788c20d97dc8b0e3c4cf9c44f6591240a9`

and contains exactly the desktop payload:

- `usr/lib/private-remote-workspace/prw-desktop`;
- `usr/share/applications/io.patchmirror.prw.desktop.desktop`.

It contains no `prw-agent` executable.

Therefore the Desktop Debian CI artifact is explicitly:

`NOT_AN_AGENT_ACTIVATION_ARTIFACT`

and MUST NOT be used as the WA Agent deployment candidate.

Package CI remains validation evidence only.

## Existing Agent reconciliation authority

Current exact WA source still contains the dedicated fixed-purpose package reconciler:

`crates/prw-agent-package-reconciliation/src/lib.rs`

Exact WA blob:

`67916d2af85ba5656fbc3bab3ad54661776488f1`

Its production authority remains compile-time fixed to the prior UF/UQ transaction:

- destination:
  `/usr/lib/private-remote-workspace/prw-agent`;
- vendor unit verify-only:
  `/usr/lib/systemd/user/prw-agent.service`;
- fixed old Agent SHA-256:
  `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`;
- fixed new Agent SHA-256:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- fixed old bytes:
  `2865776`;
- fixed new bytes:
  `11068384`;
- fixed candidate source head:
  `10714024a4df71bd3b5d0232bb0c6b6d7c9fb71f`;
- fixed canonical target:
  `/tmp/prw-c03e-uf-canonical-target`.

The currently installed production Agent is already the reconciler's compiled-in NEW identity.

Therefore the existing reconciler binary/source cannot safely upgrade production to WA without a
separately validated retargeting of its compile-time old/new candidate authority.

WB explicitly rejects reusing the historical reconciler unchanged.

## Historical deployment precedent retained

C03e-UF proved that the Agent binary is not path-independent reproducible because an absolute
generated-source path from `etcd-client 0.19.0` is embedded in the binary.

UF therefore selected path-bound reproducibility and proved two clean release builds were
byte-identical only when performed at one exact absolute Cargo target path.

The retained build shape was:

`CARGO_TARGET_DIR=<canonical-absolute-target> cargo build --locked --release -p prw-agent --bin prw-agent`

C03e-UO selected user-attended same-terminal sudo authentication only:

1. `/usr/bin/sudo`
2. `-u`
3. `root`
4. `--`
5. exact staged `prw-agent-package-reconcile`
6. `reconcile-current-agent`
7. exact private stage directory

No password may be requested, transported, stored or injected by ChatGPT or automation.

C03e-UQ independently proved the resulting root-owned Agent replacement while keeping service
activation separate.

C03e-VL later proved first service activation with a separately gated
`systemctl --user start prw-agent.service` transaction and post-start containment checks.

WB reuses these validated ownership boundaries rather than inventing a new deployment mechanism.

## Selected transaction decomposition

WA production activation is NOT one undifferentiated mutation.

WB selects the following ordered gates.

### Gate 1 — exact WA Agent candidate provenance

A separately authorized build/provenance checkpoint must produce an exact Agent candidate from:

- source head:
  `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- source tree:
  `bcd8ee5d8f1d3e67b68217e77e341c98be5e9e5f`;
- `Cargo.lock` from the same exact head.

It must preserve the UF path-bound reproducibility law.

WB selects the canonical future build path:

`/tmp/prw-c03e-wc-canonical-target`

The candidate-provenance checkpoint must perform two complete clean release builds at that exact
absolute path using:

`CARGO_TARGET_DIR=/tmp/prw-c03e-wc-canonical-target cargo build --locked --release -p prw-agent --bin prw-agent`

Both builds must be byte-identical before one SHA-256/byte-count identity may be selected.

No path-independent reproducibility claim is permitted.

No production file is changed by this gate.

### Gate 2 — fixed-purpose reconciler retargeting

Only after Gate 1 has selected the exact WA candidate bytes may a separately authorized source
checkpoint retarget the existing fixed-purpose package reconciler.

Its selected law is:

- fixed installed-old SHA-256:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- fixed installed-old bytes:
  `11068384`;
- fixed new SHA-256/bytes:
  exactly the Gate-1 selected candidate;
- fixed candidate source head/tree:
  exact WA head/tree;
- fixed Cargo.lock SHA-256:
  exact Gate-1 recorded value;
- fixed canonical target:
  `/tmp/prw-c03e-wc-canonical-target`;
- fixed Agent destination:
  `/usr/lib/private-remote-workspace/prw-agent`;
- fixed vendor unit:
  `/usr/lib/systemd/user/prw-agent.service`;
- vendor unit remains verify-only;
- no arbitrary destination;
- no shell or general command authority;
- no systemd-manager authority;
- no configuration/credential/network authority.

The exact source-path ceiling for that retargeting is not implicitly authorized by WB; it must be
audited and closed by its own selection/source checkpoint.

### Gate 3 — private stage materialization

A separately authorized stage checkpoint must create one private user-owned stage containing only
the exact selected artifacts required by the retargeted reconciler transaction:

- retargeted `prw-agent-package-reconcile`;
- exact WA `candidate-prw-agent`;
- exact frozen deployment manifest.

Stage custody must retain the established no-symlink, owner/mode, exact-hash and exact-child-count
law.

Historical UF/UQ stage bytes MUST NOT be reused as the WA stage.

### Gate 4 — active-service mutation preflight

Immediately before any production mutation, read-only reproof must establish at minimum:

- PR #717 / WA exact source authority unchanged;
- exact staged artifact identities unchanged;
- installed Agent still exact old WA-predecessor identity:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- vendor unit exact identity re-proved;
- service still `active/running`;
- exactly one Agent process;
- `Result=success`;
- no automatic restart loop;
- manager-visible `local_only`;
- fixed `20` and `30` drop-ins loaded;
- managed `40` absent;
- device identity credential binding intact;
- no unexpected external PRW service/environment conflict;
- no queued user-manager job affecting the Agent;
- existing local Unix listener healthy;
- no observed Agent TCP/UDP activity.

Any drift is STOP.

### Gate 5 — explicit service stop

The existing package reconciler explicitly requires the caller to establish a non-running service
preflight and deliberately owns no systemd-manager call.

Because production is currently active, WB rejects hot replacement beneath the running Agent.

The selected mutation is therefore:

`systemctl --user stop prw-agent.service`

not a package hot-swap and not an immediate `restart`.

After stop returns, mandatory read-only proof must establish:

- `MainPID=0`;
- exact Agent process count `0`;
- local Agent Unix socket absent/not listening;
- no queued user-manager job;
- no automatic restart occurred;
- installed Agent still equals the exact expected old identity;
- stage/reconciler/unit identities remain exact.

If stop or post-stop proof fails, the privileged package transaction is not authorized.

### Gate 6 — exact user-attended reconciliation

Only after the post-stop reproof passes may the exact same-terminal sudo model be used.

The selected argv shape is:

1. `/usr/bin/sudo`
2. `-u`
3. `root`
4. `--`
5. exact WA-stage `prw-agent-package-reconcile`
6. `reconcile-current-agent`
7. exact WA private stage directory

The password, if requested by sudo-rs, is entered only by the user into that same local terminal.

No `sudo -S`, askpass, NOPASSWD, sudoers mutation, credential file/environment, synthetic
`SUDO_*`, alternate setuid helper, direct root shell, `su`, `pkexec`, wrapper, pipe or
redirection is selected.

After the command returns, independent readback must prove:

- installed Agent root:root mode `0755`;
- installed Agent exact new Gate-1 bytes/SHA;
- vendor unit unchanged and exact;
- no root reconciliation residue;
- service remains non-running;
- no unexpected production path mutation.

If reconciliation fails or identity is ambiguous, STOP. No automatic retry is selected.

### Gate 7 — production start of the new Agent

Because Gate 5 intentionally stopped the service, successful activation uses:

`systemctl --user start prw-agent.service`

not `restart`.

This start is allowed only after Gate 6 independently proves the exact new installed Agent.

No `daemon-reload` is selected because the vendor unit and fixed `20`/`30` configuration are
not changed by the WA install transaction. A future preflight must still verify
`NeedDaemonReload=no`; drift is STOP.

### Gate 8 — mandatory post-start health

The exact activation checkpoint must prove at minimum:

- service loaded and `active/running`;
- `Result=success`;
- one new non-zero `MainPID`;
- executable resolves to:
  `/usr/lib/private-remote-workspace/prw-agent`;
- `/proc/<MainPID>/exe` and installed path correspond to the exact Gate-1 candidate identity;
- no automatic restart loop; `NRestarts` remains acceptable and no failure restart event occurred;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- fixed `20` and `30` loaded;
- managed `40` absent;
- device identity loaded without exposing private credential bytes;
- runtime directory remains user-owned `0700`;
- `agent.lock` remains regular user-owned `0600`;
- `agent.sock` remains user-owned Unix socket `0600`;
- exactly one Agent Unix listener;
- no Agent TCP listener/connection;
- no Agent UDP listener/connection;
- no unexpected user-manager jobs;
- no new startup failure/main-exit journal event;
- installed desktop process/binary remains outside this transaction.

### Gate 9 — legacy compatibility proof

Before any command-3 production probe, the activated WA Agent must first prove that the already
deployed legacy local read surface still works.

A bounded same-UID command-1 `GetAgentStatus` probe must prove:

- trusted runtime/socket path;
- successful connection;
- request-ID correlation;
- `Ok` terminal status;
- decodable status snapshot.

No command-2 mutation exists; command 2 remains read-only if observed but is not required by WB.

### Gate 10 — dedicated production command-3 AgentStatus probe

Fresh exact WA source proves that a production command-3 probe cannot currently be delegated to
the installed desktop application:

- `apps/desktop/src/ipc.rs` owns live Unix socket I/O but sends only legacy local commands;
- `apps/desktop/src/local_management_ipc.rs` can build canonical command-3 bridge-management
  frames, but explicitly performs no socket I/O;
- desktop `main.rs` does not wire command-3 dispatch.

WB therefore rejects:

- ad-hoc hand-crafted command-3 bytes;
- raw shell socket writes;
- desktop command-3 activation by implication;
- widening desktop management authority merely to run one deployment probe.

Before the production command-3 probe, a separately gated source checkpoint must select and
materialize one bounded **same-UID one-shot AgentStatus probe**.

That probe must reuse existing canonical protocol/runtime components rather than define an
alternate wire format:

- trusted `LocalIpcContract` endpoint derivation and same-UID socket custody checks;
- `BridgeCommand::AgentStatus`;
- canonical PRWC encoding;
- Agent-owned command-3 local framing;
- existing `LocalIpcFrame` write/read primitives;
- existing terminal-response validation;
- exact request-ID correlation;
- terminal status must be `Ok`;
- management success body tag must be Agent status (`1`);
- the following five-byte status snapshot must decode under the existing status codec.

The probe must:

- perform exactly one AgentStatus request;
- have finite read/write timeouts;
- own no terminal/file/forwarding/provider authority;
- own no filesystem root;
- perform no configuration/network mutation;
- not persist credentials;
- not become a long-lived daemon;
- not alter desktop source/runtime authority.

A successful command-3 production probe may be claimed only after:

1. exact WA Agent candidate is installed;
2. the new Agent is active and healthy;
3. legacy command-1 compatibility passes;
4. the dedicated probe exact source/binary identity is validated;
5. request-ID correlated `Ok` AgentStatus response and decoded snapshot are observed.

## Failure and recovery law

WB selects no automatic retry.

At any failed gate:

- STOP;
- preserve exact evidence;
- do not advance to the next mutation.

If the service has been stopped and the package transaction has not produced an independently
proven valid installed state, no automatic start is selected. Recovery must be separately
authorized from the exact observed state.

If package reconciliation independently proves the old Agent remains exact after a pre-exchange
failure, a later recovery checkpoint may explicitly select restoration of the old active service;
WB does not silently authorize that recovery action.

## Security ceiling

This activation transaction changes only the installed Agent executable and, later, service
lifecycle state required to execute that exact candidate.

It does not authorize:

- configured-remote input creation;
- execution-mode transition;
- managed `40` write;
- credential replacement;
- enrollment/revocation mutation;
- desktop command-3 dispatch;
- terminal/file/forwarding authority;
- management provider lifecycle;
- network/DNS/firewall/route mutation;
- control-plane/database mutation;
- vendor-unit rewrite;
- `20`/`30` mutation;
- enablement/linger mutation;
- arbitrary privileged file replacement.

VU/VW management law remains fixed:

- `AgentStatusRead = Allow`;
- every other represented management capability = `Deny`.

## Immediate successor selection

The immediate successor to WB is not a production mutation.

It is a separately authorized candidate-provenance checkpoint, expected as C03e-WC, that must
select exact WA Agent candidate bytes by two clean same-path builds at:

`/tmp/prw-c03e-wc-canonical-target`

and stop before package-reconciler retargeting, stage creation, service stop, sudo execution,
installed-file replacement, Agent start or any production command-3 probe.

## Selection classification

`WA_AGENT_PRODUCTION_ACTIVATION_TRANSACTION_SELECTED / DOCUMENTATION_ONLY / DESKTOP_DEBIAN_ARTIFACT_EXPLICITLY_EXCLUDED_AS_AGENT_CANDIDATE / PATH_BOUND_AGENT_BUILD_PROVENANCE_REQUIRED / EXACT_WA_SOURCE_HEAD_TREE_BOUND / EXISTING_RECONCILER_NOT_REUSABLE_UNCHANGED / RECONCILER_RETARGET_REQUIRED / ACTIVE_SERVICE_MUST_STOP_BEFORE_RECONCILE / USER_ATTENDED_SAME_TERMINAL_SUDO_PRESERVED / ROOT_FIXED_DESTINATION_RECONCILIATION_PRESERVED / START_AFTER_SUCCESSFUL_INSTALL_NOT_HOT_RESTART / POST_START_LOCAL_ONLY_HEALTH_REQUIRED / LEGACY_COMMAND1_PROBE_REQUIRED / COMMAND3_AGENT_STATUS_PROBE_REQUIRES_SEPARATE_ONE_SHOT_SAME_UID_PROBE / DESKTOP_COMMAND3_DISPATCH_NOT_SELECTED / NO_GENERIC_MANAGEMENT_PROVIDER / NO_FILESYSTEM_AUTHORITY_EXPANSION / NO_TERMINAL_AUTHORITY / NO_FORWARDING_AUTHORITY / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_PRODUCTION_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_WA_AGENT_PRODUCTION_ACTIVATION_TRANSACTION_SELECTION_AND_BEFORE_WA_AGENT_CANDIDATE_PROVENANCE_BUILD`

`NO_RACE_FREE_CLAIM`
