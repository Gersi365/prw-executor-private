# C03e-TU — configured-remote systemd input custody selection staging

Status: **SELECTION STAGING**

## 1. Purpose

C03e-TU selects only the systemd/user-service custody topology and serialization law for the six non-secret `PRW_REMOTE_*` process inputs required by the already-materialized `configured_remote` executable lane.

Canonical selected law:

`GLOBAL_VENDOR_UNIT_BYTE_STABLE / TT_MODE_DROPIN_30_REMAINS_SEPARATE / MANAGED_PER_USER_CONFIGURED_REMOTE_INPUT_DROPIN_40 / EXACTLY_SIX_PRW_REMOTE_ENVIRONMENT_ASSIGNMENTS / NO_ENVIRONMENTFILE / NO_SYSTEMD_CREDENTIAL_FOR_NON_SECRET_REMOTE_INPUTS / FULL_ASSIGNMENT_DOUBLE_QUOTED / BACKSLASH_DOUBLE_QUOTE_AND_PERCENT_ESCAPED / PRINTABLE_UNICODE_ONLY / EXACT_VALUE_ROUND_TRIP / EXISTING_AGENT_PARSERS_REMAIN_SEMANTIC_AUTHORITY / NO_DEFAULT_NORMALIZATION_OR_INFERENCE / LOCAL_ONLY_REQUIRES_PRW_MANAGED_REMOTE_INPUT_DROPIN_ABSENT / CONFIGURED_REMOTE_REQUIRES_MODE_AND_COMPLETE_REMOTE_INPUT_BUNDLE_STAGED_TOGETHER / USER_OWNED_0600_MANAGED_LEAVES / SYMLINK_AND_FOREIGN_CONTENT_REJECTED / ATOMIC_MANAGED_FILE_TRANSACTION_BEFORE_RELOAD_OR_RESTART / FILE_STATE_ROLLBACK_ONLY / NO_DAEMON_RELOAD_RESTART_ENABLEMENT_LINGER_DEPLOYMENT_OR_SERVICE_MUTATION / NO_SOURCE_MUTATION`

TU performs no service/systemd mutation, drop-in creation, environment mutation, daemon reload, service restart, deployment, Rust/source mutation, merge, or host/network mutation.

## 2. Authoritative predecessor

C03e-TT PR `#658` is the exact predecessor.

Exact TT identity:

- branch: `phase-152-c03e-tt-systemd-execution-mode-injection-selection`
- head: `45b5842cf2445d3e903361415fa391a560e6ecec`
- tree: `02b67147833bc26a49b9f197a887ca4fc8f262e1`
- parent/base: C03e-TS `33006fefed790a114a54d72bcc84448423c61fda`
- TT contract blob: `32bc6677b3c8d4578365f9d2faef00b3b854e4da`
- status: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`
- PR remains draft/open/unmerged.

TT is authoritative for the execution-mode custody layer and selects:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/30-agent-execution-mode.conf`

with exactly one `PRW_AGENT_EXECUTION_MODE` assignment for either `local_only` or `configured_remote`.

TU does not modify or reinterpret that selection.

## 3. Preserved service and identity custody

The global package-owned vendor unit remains byte-stable:

`packaging/systemd/prw-agent.service`

TT-audited blob:

`ac11306669cb5df92eb3a0cbbf026971fca05630`

Existing device-identity custody remains separate:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`

TU does not modify the vendor unit, the identity drop-in, `ExecStart=`, restart policy, enablement, linger state, service dependencies, or package ownership.

## 4. Exact configured-remote input family

The complete configured-remote process-input family is exactly:

1. `PRW_REMOTE_BIND_ADDR`
2. `PRW_REMOTE_PEER_DEVICE_ID`
3. `PRW_REMOTE_MAX_ACTIVE_WORKERS`
4. `PRW_REMOTE_APPLICATION_LEASE_SECONDS`
5. `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`
6. `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`

No seventh `PRW_REMOTE_*` variable, alias, compatibility name, inferred value, file source, command-line flag, or manager-global environment source is selected.

## 5. Current Agent parser authority

Current configured-remote source authority remains in the existing Agent source. TU adds no parser and weakens no validator.

The current Linux configured-remote source blob inherited through TT is:

`crates/prw-agent/src/linux_bootstrap.rs`

blob:

`7c739d804deb59090f3ce7eb6491f79f956197ee`

The existing loaders read fixed process-environment names and fail closed on missing, encoding-invalid, malformed, zero, out-of-range, or policy-invalid values according to each source's existing type and validation law.

TU selects deployment custody only. The Agent parser remains semantic authority after systemd decoding.

## 6. Bind-address source law preserved

`PRW_REMOTE_BIND_ADDR` remains parsed as the existing exact `SocketAddr` source.

Existing source behavior includes:

- missing or empty fails closed;
- non-Unicode fails closed;
- malformed `SocketAddr` fails closed;
- unspecified addresses fail closed;
- multicast addresses fail closed;
- IPv4 limited broadcast fails closed;
- port `0` remains valid pre-bind under the existing source law;
- no DNS lookup, interface discovery, route inspection, public-address discovery, fallback, or normalization is introduced by TU.

The future writer must preflight the exact supplied semantic value against this existing law before any managed-file replacement.

## 7. Peer-device source law preserved

`PRW_REMOTE_PEER_DEVICE_ID` remains governed by the existing `DeviceId` source contract.

Current tests establish that missing, empty, and whitespace-only values fail closed, while an otherwise valid non-empty identifier is preserved exactly, including leading/trailing spaces where the existing `DeviceId` contract permits them.

TU therefore selects no trimming, case folding, Unicode normalization, aliasing, slugification, or identifier rewriting.

## 8. Numeric and policy source law preserved

The remaining four values stay under their existing Agent validators:

- `PRW_REMOTE_MAX_ACTIVE_WORKERS` -> existing non-zero target-`usize` parser;
- `PRW_REMOTE_APPLICATION_LEASE_SECONDS` -> existing application-lease parser plus `RemoteSessionApplicationLeasePolicy` validation;
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS` -> existing bounded record-count source;
- `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS` -> existing bounded record-count source.

TU does not add defaults, substitute historical values, trim input, clamp values, saturate, convert units, or infer one value from another.

The future writer must validate the exact semantic text using behavior equivalent to the existing Agent source law and must serialize that same semantic text without normalization.

## 9. Selected managed artifact topology

TU selects one distinct managed per-user systemd user-service drop-in for the complete configured-remote bundle:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/40-configured-remote-inputs.conf`

The ordering is intentional:

- `20-device-identity-credential.conf` -> identity credential custody;
- `30-agent-execution-mode.conf` -> execution-mode custody;
- `40-configured-remote-inputs.conf` -> configured-remote non-secret input custody.

The three artifacts remain semantically separate.

No combined `20/30/40` file and no mutation of an earlier drop-in is selected.

## 10. Exact drop-in semantic surface

`40-configured-remote-inputs.conf` contains exactly one `[Service]` section and exactly six `Environment=` assignments, one for each fixed configured-remote variable.

Deterministic assignment order is exactly:

```ini
[Service]
Environment="PRW_REMOTE_BIND_ADDR=<encoded-bind-address>"
Environment="PRW_REMOTE_PEER_DEVICE_ID=<encoded-peer-device-id>"
Environment="PRW_REMOTE_MAX_ACTIVE_WORKERS=<encoded-max-active-workers>"
Environment="PRW_REMOTE_APPLICATION_LEASE_SECONDS=<encoded-application-lease-seconds>"
Environment="PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS=<encoded-requester-rendezvous-max-records>"
Environment="PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS=<encoded-expected-device-scheduling-consumption-max-records>"
```

`<encoded-...>` denotes only the TU-selected systemd serialization of an already-preflighted exact semantic value. It is not a new value grammar.

No comments, blank alternate assignments, reset assignments, duplicate variable assignments, `UnsetEnvironment=`, `PassEnvironment=`, or `EnvironmentFile=` directive is selected inside this managed drop-in.

## 11. Why one complete bundle drop-in is selected

The six variables form one configured-remote activation prerequisite set.

A single bundle artifact provides one inspectable completeness boundary and prevents a later activation transaction from treating six independently present files as six independently authoritative configuration states.

This does not make the six values semantically interchangeable. Each value retains its own Agent validator and failure classification.

## 12. No `EnvironmentFile=`

TU preserves the TT-selected no-`EnvironmentFile=` service boundary.

No separate dotenv-style parser, path ownership, precedence layer, comment grammar, line-continuation grammar, or additional mutable file authority is introduced.

The selected bundle uses only systemd `Environment=` assignments in the dedicated managed drop-in.

## 13. No systemd credentials for the six values

The six configured-remote values are selected as non-secret process configuration.

TU does not place them in `LoadCredential=`, `LoadCredentialEncrypted=`, credential directories, private-key custody, or the existing device-identity credential drop-in.

If a future product requirement makes any new value secret, that requires a new selection rather than silently widening this non-secret bundle.

## 14. Selected systemd serialization law

Each complete `NAME=value` assignment is emitted as one double-quoted `Environment=` item.

The future writer must begin from an already validated Unicode semantic value and construct the quoted payload deterministically.

Within the value portion:

1. literal backslash `\` is encoded as `\\`;
2. literal double quote `"` is encoded as `\"`;
3. literal percent `%` is encoded as `%%` so systemd specifier expansion yields one literal percent;
4. printable UTF-8 characters other than the three cases above are retained byte-for-byte in UTF-8;
5. `$` is retained literally; no shell-variable expansion layer is selected;
6. non-printable/control characters are not representable in this selected custody and fail deployment preflight;
7. NUL is always invalid for a process environment value and fails preflight;
8. no C escape is accepted as caller-provided syntax: escaping is produced only by the trusted writer from the semantic value.

The required invariant is:

`systemd_decode(encode(value)) == value`

before any file replacement may be considered valid.

## 15. Why percent is escaped

Systemd supports `%` specifiers in applicable unit settings, including environment-setting use cases such as credential-directory references.

TU does not allow a configured remote value to become a unit-specifier authority.

Every literal percent from the semantic value is therefore serialized as `%%`, the systemd spelling for one literal percent.

No `%h`, `%u`, `%d`, `%n`, or other specifier may be intentionally introduced from configured-remote input data.

## 16. Printable-Unicode deployment boundary

The current Agent identifier domain can be broader than the systemd `Environment=` printable-value boundary.

TU does not silently alter the Agent's type contract. Instead it selects a narrower deployment-custody precondition:

- the value must first satisfy the existing Agent semantic parser;
- the value must also be representable as printable Unicode through the selected systemd `Environment=` grammar;
- otherwise deployment preflight fails closed before filesystem mutation.

This is a deployment representation constraint, not a Rust parser change.

## 17. No manager-global environment authority

The PRW-managed configured-remote state is authoritative only through `40-configured-remote-inputs.conf`.

A future writer must not intentionally rely on `systemctl --user set-environment`, shell login state, desktop-session imported variables, PAM environment, inherited manager state, or another unmanaged drop-in as the configured-remote source.

Because user service managers may inherit environment from other sources, deployment preflight must detect conflicting effective custody rather than treating it as a valid PRW-managed configuration.

TU does not authorize mutation of manager-global environment state.

## 18. Local-only stale-input law

For a PRW-managed `local_only` configuration, `40-configured-remote-inputs.conf` must be absent.

The local-only lane does not require the six configured-remote values, and retaining a PRW-managed remote-input bundle while declaring `local_only` creates stale dual-state custody.

A later transition from `configured_remote` to `local_only` must therefore stage:

1. exact `local_only` content for `30-agent-execution-mode.conf`;
2. removal of the PRW-managed `40-configured-remote-inputs.conf`;
3. complete file-state preflight;
4. only then, under separately authorized activation, any daemon reload/restart boundary.

TU authorizes no such real-host removal; it selects only the future transaction law.

Unmanaged foreign environment state is never silently deleted by this selection. A conflict must fail closed for later explicit handling.

## 19. Configured-remote completeness law

For PRW-managed `configured_remote`, both artifacts are required as one logical configuration transaction:

- `30-agent-execution-mode.conf` with exact `configured_remote` mode;
- `40-configured-remote-inputs.conf` with exactly six complete validated assignments.

The future writer must not install the mode first, reload/restart, then discover or provision remote inputs.

Likewise it must not install the remote bundle and leave mode absent or `local_only` as an intended final state.

The desired pair must be staged and validated completely before any service-manager activation boundary.

## 20. Filesystem custody

Both PRW-managed configuration leaves selected by TT/TU use conservative mode `0600`.

A future writer must:

- resolve the intended user's XDG config root through an explicitly validated boundary;
- operate as or for the explicitly intended user;
- create/use the expected `systemd/user/prw-agent.service.d` directory without following attacker-controlled leaf symlinks;
- reject symlink traversal at managed leaves;
- reject non-regular managed leaves;
- reject foreign/unmanaged conflicting content rather than overwrite it;
- stage complete new bytes before replacement;
- atomically replace only PRW-managed leaves;
- preserve `20-device-identity-credential.conf` byte-for-byte;
- preserve the global vendor unit byte-for-byte;
- preserve unrelated drop-ins byte-for-byte;
- preserve enablement and linger policy unless separately authorized.

TU does not select directory ownership changes outside the narrowly required managed path.

## 21. Conflict policy

The later writer must distinguish at least:

- managed file absent;
- managed file present and exactly recognized as PRW-owned current/previous content;
- conflicting file present at the selected leaf;
- symlink/non-regular object present;
- duplicate/conflicting assignment supplied by another PRW-managed artifact;
- effective environment conflict from an unsupported external source.

Only the first two classes may enter a normal PRW-managed replacement transaction.

All conflict classes fail closed without destructive cleanup.

TU selects no takeover, forced overwrite, recursive cleanup, or best-effort reconciliation of foreign content.

## 22. Atomic managed-file transaction

The selected configuration transaction is file-state atomic at the PRW-managed boundary.

Before replacement, the future writer must have:

- exact intended mode;
- all six exact semantic remote values when mode is `configured_remote`;
- successful existing-Agent-equivalent validation of every required value;
- successful selected systemd encode/decode round-trip;
- resolved intended-user/XDG path;
- successful conflict/symlink/non-regular-object preflight;
- staged complete target bytes for every file that will exist after the transaction;
- captured the exact prior PRW-managed file state needed for bounded rollback.

Only after all preflight passes may managed leaves be replaced/removed.

No daemon reload or service restart belongs to this TU transaction.

## 23. Selected rollback scope

TU selects rollback only for the managed configuration-file state changed by the future writer.

The rollback set is bounded to:

- `30-agent-execution-mode.conf` if that transaction changes it;
- `40-configured-remote-inputs.conf` if that transaction creates, replaces, or removes it.

Rollback must restore the exact pre-transaction PRW-managed bytes/existence state if the file mutation transaction itself fails before a separately authorized service-manager activation boundary.

Rollback must not modify:

- the vendor unit;
- `20-device-identity-credential.conf`;
- unrelated drop-ins;
- user-manager global environment;
- enablement or linger;
- network/firewall state;
- package state.

Service-state rollback after a future daemon reload/restart is not selected by TU and requires the later activation-transaction selection.

## 24. Preflight before daemon reload/restart

A later activation checkpoint must require a fresh post-write readback before service-manager mutation.

For configured remote, readback must prove:

- mode file exists with exact selected `configured_remote` assignment;
- remote bundle exists with exactly six assignments;
- decoded values equal the intended semantic inputs;
- the resulting unit configuration parses successfully on the target systemd implementation;
- no managed-file conflict or missing input remains.

For local only, readback must prove:

- mode file exists with exact selected `local_only` assignment;
- PRW-managed remote bundle is absent.

Only a separately authorized checkpoint may decide the exact `daemon-reload`, restart/start, health/readiness, and post-activation rollback transaction.

## 25. Existing restart policy remains non-authority

The package-owned unit retains `Restart=on-failure`.

TU does not reinterpret automatic restart as configuration retry, delayed provisioning, rollback, or convergence.

Invalid configuration must be caught by preflight rather than intentionally entering a restart loop.

## 26. No network or listener mutation

TU adds no:

- socket unit;
- bind operation;
- listener start;
- firewall change;
- route change;
- DNS change;
- TUN/TAP;
- privileged helper;
- network-online dependency;
- readiness publication;
- remote connection attempt.

`PRW_REMOTE_BIND_ADDR` remains only a configured semantic value in this checkpoint.

## 27. Future writer acceptance requirements

A separately authorized writer/source-materialization checkpoint must at minimum test:

1. exact path selection for `30-` and `40-` leaves;
2. exact six-variable order and no seventh assignment;
3. backslash round-trip;
4. double-quote round-trip;
5. percent round-trip without specifier expansion;
6. spaces and printable Unicode round-trip;
7. `$` remains literal;
8. non-printable input fails before mutation;
9. every semantic value is rejected if the existing Agent parser would reject it;
10. `local_only` desired state removes only the PRW-managed `40-` leaf;
11. `configured_remote` requires complete `30-` + `40-` desired state;
12. symlink/non-regular/foreign-content conflicts fail closed;
13. file mode `0600` and intended-user ownership are enforced;
14. interrupted file transaction restores exact prior managed-file state;
15. vendor unit and identity drop-in remain byte-identical;
16. no daemon reload/restart occurs inside writer-only materialization tests.

These are future acceptance requirements, not tests claimed executed by TU.

## 28. Source-materialization ceiling for TU

Current Rust/source/service/deployment ceiling for TU is **ZERO**.

TU changes only its documentation contract.

It does not create the `40-` drop-in, a writer, a serializer implementation, a provisioning API, a systemd command invocation, or an activation transaction.

## 29. Immediate successor boundary

After TU is evidence-closed, the next separately approved checkpoint may select the concrete managed-file writer/source-materialization surface for the already-selected TT/TU laws.

That successor must freshly audit existing provisioning ownership before selecting any code path and must keep service-manager activation separately gated.

TU itself does not authorize that successor.

## 30. Explicit STOP

Keep the TU PR draft/open/unmerged after evidence closure.

Do not perform:

- Rust/source materialization;
- packaging/systemd source mutation;
- actual `30-` or `40-` drop-in creation/removal/replacement;
- user-manager environment mutation;
- systemd credential mutation;
- `EnvironmentFile=` creation;
- daemon reload;
- service start/restart/stop;
- enable/disable;
- linger mutation;
- deployment;
- host/network mutation;
- repository configuration mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

STOP after TU evidence closure.
