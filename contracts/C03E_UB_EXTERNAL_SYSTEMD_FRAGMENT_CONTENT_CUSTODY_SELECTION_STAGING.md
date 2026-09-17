# C03e-UB — External systemd fragment content custody selection

## 1. Status and boundary

`SELECTION — SOURCE MATERIALIZATION NOT INCLUDED`

Boundary:

`EXTERNAL_SYSTEMD_FRAGMENT_CONTENT_CUSTODY_SELECTION`

C03e-UB is a docs-only selection checkpoint. It resolves the residual same-path external systemd fragment content race left explicit by evidence-closed C03e-UA. It does not execute the managed writer, `systemd-analyze`, `systemctl daemon-reload`, restart, try-restart, enablement, linger mutation, deployment, or any real service activation.

## 2. Authoritative predecessor

The authoritative predecessor is evidence-closed C03e-UA / PR #665.

Exact predecessor source identity:

- branch: `phase-152-c03e-ua-loaded-dropin-topology-source-materialization`;
- head: `f135092a3aa6a91a45055dfdab36dee4db5564ba`;
- tree: `2ed13e59632afa9ad6d1a5513fe71ef617ca8f7f`;
- parent: C03e-TZ `003fce47315e5f41461db4f1b752bb40a3b45277`.

UA materialized the exact loaded `DropInPaths` topology law and explicitly retained:

`NO_FOREIGN_CONTENT_STABILITY_CLAIM / NO_RACE_FREE_CLAIM`

UA therefore proves path topology but does not prove byte, inode, metadata, or semantic stability of an external unit/drop-in file that changes while retaining the same path.

## 3. Problem statement

The current active reconfiguration pipeline has two distinct custody layers:

1. the managed writer owns only PRW-managed `30-agent-execution-mode.conf` and `40-configured-remote-inputs.conf`;
2. the orchestration layer proves loaded path topology before and after manager/process transitions.

The writer already performs a read-only external-fragment conflict inspection before staging managed files. UA additionally proves that foreign loaded drop-in paths do not change across the selected manager/process gates.

Neither mechanism proves that bytes at an unchanged external path stayed unchanged throughout the transaction.

A same-path mutation can therefore preserve the exact `DropInPaths` vector while changing effective service semantics.

This applies not only to foreign drop-ins but also to the main `prw-agent.service` fragment itself.

## 4. Read-only archaeology findings

The repository has one packaged vendor unit source:

`packaging/systemd/prw-agent.service`

Its repository SHA-256 at the UB selection boundary is:

`24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`

A read-only current-host probe observed:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- empty `DropInPaths`;
- `UnitFileState=enabled`.

The installed main unit was a regular root-owned mode-`0644` file and its SHA-256 matched the repository vendor unit at observation time.

This observation is not activation authority and does not change the existing rule that `reconfigure-active` requires an already active/running/Ready Agent.

## 5. Decision summary

UB selects:

`OPAQUE_EXTERNAL_FRAGMENT_SNAPSHOT_CUSTODY / SANITIZED_UNIT_PATH_DISCOVERY / EXACT_EXTERNAL_FRAGMENT_INVENTORY / MAIN_UNIT_INCLUDED / ALL_NON_30_40_DROPINS_INCLUDED / FINAL_COMPONENT_NOFOLLOW / REGULAR_FILE_ONLY / EXACT_METADATA_IDENTITY / EXACT_BYTES / BASELINE_CAPTURE_BEFORE_MANAGED_WRITE / PRE_VERIFY_REPROOF / PRE_RELOAD_REPROOF / POST_RELOAD_REPROOF / PRE_ROLLBACK_REPROOF / POST_RESTART_REPROOF / NO_EXTERNAL_FILE_MUTATION / NO_DESCRIPTOR_HELD_MANAGER_BINDING_CLAIM / NO_ADVISORY_LOCK_AUTHORITY / NO_RACE_FREE_CLAIM`

The selected mechanism is exact in-process snapshot-and-reproof custody.

UB does not select advisory locks, descriptor-held foreign-file authority, foreign-file takeover, or mutation of any external fragment.

## 6. Why exact snapshot-and-reproof is selected

A second semantic custody pass alone is insufficient because an external file may change to a different but still syntactically valid and non-conflicting service configuration.

Descriptor-held custody alone is insufficient because `systemd` does not consume PRW's open file descriptor during `daemon-reload`; the manager reopens pathnames independently. A path can therefore be atomically replaced while PRW still holds an older inode open.

Advisory locks are insufficient because systemd and unrelated external writers are not required to participate in PRW's lock protocol.

Exact snapshot-and-reproof is selected because it can:

- detect same-path byte changes;
- detect inode replacement;
- detect selected metadata changes;
- detect additions/removals in the external candidate fragment inventory;
- reuse the existing sanitized unit-search-path authority;
- remain read-only with respect to external configuration;
- preserve the existing managed `30/40` ownership boundary.

It narrows and detects the race but does not physically prevent every mutation between a final reproof and systemd's independent pathname reopen.

Therefore `NO_RACE_FREE_CLAIM` remains authoritative.

## 7. Ownership boundary

The future custody implementation belongs in `prw-agent-configuration`, not in `prw-agent` and not in desktop code.

Rationale:

- `prw-agent-configuration` already owns sanitized user-unit search-path discovery;
- it already owns external-fragment conflict inspection;
- it already owns managed systemd filesystem custody and NOFOLLOW policy;
- keeping snapshot capture there avoids duplicating systemd fragment parsing/custody rules in the orchestration crate.

The orchestration crate owns only transaction sequencing and failure disposition around the opaque custody token.

## 8. External fragment inventory definition

For one intended-user context, the custody snapshot must first run the existing sanitized unit-path discovery equivalent to:

`/usr/bin/systemd-analyze --user unit-paths`

under an environment cleared and rebuilt only from the selected explicit HOME/XDG locale context.

The ordered unit-search-path list itself is part of the snapshot and must match exactly on every reproof.

For each discovered unit path, the external target inventory includes:

1. `prw-agent.service`, if present;
2. every direct entry under `prw-agent.service.d`, if that directory is present;
3. except only the exact PRW-managed `30-agent-execution-mode.conf` and `40-configured-remote-inputs.conf` leaves in the intended managed directory.

The inventory therefore includes:

- the effective or shadowed main unit candidate files;
- `20-device-identity-credential.conf` when present;
- every other non-managed target drop-in candidate visible in the selected search paths.

UB intentionally uses a conservative candidate inventory rather than attempting to reimplement systemd unit/drop-in precedence.

Any candidate addition, removal, rename, type change, or search-path change causes reproof failure.

## 9. Managed leaves excluded from snapshot custody

Only the two exact leaves owned by the managed writer are excluded:

- `30-agent-execution-mode.conf`;
- `40-configured-remote-inputs.conf`.

They are excluded only when they are the exact leaves inside the intended managed `prw-agent.service.d` directory.

No basename-only exclusion is selected.

`20-device-identity-credential.conf` remains external to the 30/40 writer and is included in the custody snapshot when present.

## 10. External file shape required for active reconfiguration

Every inventoried external fragment must be captured fail-closed as a regular file.

The final path component must not be a symbolic link.

The implementation must open the final component read-only with `O_CLOEXEC | O_NOFOLLOW` and prove the opened descriptor is a regular file.

A symlink, directory, device, FIFO, socket, unreadable file, vanished path, or ambiguous metadata state fails custody.

UB does not create, chmod, chown, rename, unlink, repair, or replace any external fragment.

## 11. Exact per-file snapshot identity

For each external fragment, the opaque snapshot records at least:

- exact path identity;
- device number;
- inode number;
- file mode bits;
- UID;
- GID;
- byte length;
- mtime seconds/nanoseconds;
- ctime seconds/nanoseconds;
- exact file bytes.

Capture must verify that the descriptor identity and the path's final resolved file identity still agree after the read.

If identity or metadata changes while capture is in progress, capture fails closed instead of accepting an unstable snapshot.

The exact bytes are retained only inside the opaque in-process custody object.

## 12. Snapshot bounds

The snapshot is intentionally bounded.

UB selects these implementation-local ceilings for the PRW target unit:

- at most `128` external fragment files;
- at most `1,048,576` bytes per external fragment;
- at most `8,388,608` total external-fragment bytes.

Exceeding a ceiling fails closed before managed mutation.

These ceilings are not systemd format claims. They are PRW transaction-memory bounds for this narrow administrative action.

No external dependency or persistent cache is selected.

## 13. Snapshot secrecy and lifetime

The custody snapshot is opaque outside `prw-agent-configuration`.

It must not implement a content-bearing `Debug` representation, serialization, persistence, or logging surface.

External fragment bytes may contain sensitive configuration. Raw bytes, paths, UID/GID values, metadata fingerprints, or content excerpts must not be emitted through stable user-facing diagnostics.

The snapshot exists only in-process for one administrative transaction.

No database, journal, lock file, hidden drop-in, or recovery sidecar is selected.

## 14. Baseline capture point

For `reconfigure-active`, the orchestration sequence becomes:

1. capture baseline `UnitStatus` and require active/running state;
2. prove baseline local IPC Ready using the existing selected protocol;
3. capture the opaque external-fragment custody snapshot;
4. only then invoke the recoverable managed writer.

Snapshot capture failure stops before managed-file mutation.

The write-only action remains outside active service-manager orchestration and does not require this long-lived transaction snapshot.

## 15. Reproof semantics

A reproof is successful only if all of the following remain exact:

- ordered sanitized unit-search-path list;
- external candidate path inventory;
- per-file regular/non-symlink shape;
- device/inode identity;
- mode/UID/GID;
- length and selected timestamps;
- exact bytes.

Reproof may recapture into a short-lived comparison value internally, but it must not replace the original baseline custody token.

A content change that later returns to the exact original bytes but has changed captured identity/metadata still fails closed when the selected metadata differs.

## 16. Required forward reproof gates

The future active lane must reprove the same baseline external custody at these gates:

1. immediately after successful recoverable managed write and before target-systemd verification;
2. after successful target-systemd verification and immediately before `daemon-reload`;
3. immediately after successful `daemon-reload` and loaded-topology proof, before forward `try-restart`;
4. after forward restart/readiness and the UA loaded-topology reproof, before classifying success.

No reproof is satisfied by `DropInPaths` equality alone.

No reproof may silently refresh the baseline snapshot.

## 17. Failure before daemon reload

If external custody differs at either pre-reload reproof gate:

1. stop forward progress;
2. consume the recoverable managed-writer token to restore exact prior managed `30/40` state;
3. verify that restore through the existing writer custody;
4. do not call `daemon-reload`;
5. do not restart, try-restart, stop, start, enable, disable, or alter linger.

The previously running manager/process state was never reloaded and therefore remains the active baseline.

The bounded orchestration result is external-configuration drift, not target-verify failure and not successful rollback activation.

## 18. Failure after daemon reload but before forward restart

If the post-reload external reproof fails before `try-restart`:

1. do not attempt forward restart;
2. restore exact prior managed `30/40` file state using the recovery token;
3. execute at most one rollback `daemon-reload` so the manager no longer retains the failed desired managed state;
4. do not execute the ordinary rollback `restart` selected by TX;
5. return terminal external-configuration drift requiring explicit reconciliation.

Reason: PRW has no authority to restore the changed external fragment bytes. Restarting automatically would start the service under an external configuration that is no longer the captured baseline.

The already-running process has not yet been intentionally restarted by the forward lane and should not be mutated further by UB's drift path.

If managed restore or the one rollback `daemon-reload` fails, the existing bounded rollback-failure class remains terminal.

## 19. Failure after a restart attempt

After any forward `try-restart` attempt, automatic rollback restart is permitted only if external custody can first be re-proved equal to the original baseline snapshot.

If external custody has drifted after a restart attempt or at the final success gate:

- success must not be claimed;
- PRW must not overwrite, restore, or repair the external files;
- PRW must not perform another automatic restart, stop, start, enablement change, or linger change;
- PRW must not claim exact rollback to the pretransaction service configuration;
- the transaction terminates in a bounded external-drift/manual-reconciliation state.

This is an explicit exception to the generic TX post-reload rollback restart law, because that law assumes the non-managed configuration required for baseline restoration is still available unchanged.

## 20. Pre-rollback reproof for other failures

For a non-custody failure after `daemon-reload`—for example post-reload topology failure, `try-restart` failure, or readiness failure—the orchestrator must reprove external custody before executing the existing ordinary rollback restart.

If reproof succeeds, existing TX/TY/UA rollback semantics remain authoritative.

If reproof fails, the transaction switches to the external-drift disposition in sections 18 or 19 according to whether a restart attempt has occurred.

Thus ordinary rollback restart is never executed under a known-changed external configuration.

## 21. Bounded error surface

UB selects one new stable bounded orchestration classification:

`ExternalConfigurationDrift`

Its display text must remain fixed and path/content free, for example:

`external_configuration_drift`

Internal code may retain stage information for sequencing/tests, but stable diagnostics must not expose raw fragment paths, bytes, environment values, device IDs, credentials, or metadata.

Baseline snapshot acquisition failure before mutation remains a baseline/custody failure and must not be misreported as successful drift recovery.

## 22. Existing loaded-topology law remains required

The UA loaded-topology invariant remains fully authoritative and is not replaced by byte custody.

Success still requires:

- valid absolute/unique `DropInPaths`;
- exact foreign ordered subsequence stability;
- exact desired managed `30/40` membership;
- unchanged `UnitFileState`;
- post-reload proof;
- post-restart proof.

External-fragment snapshot custody is an additional independent proof layer.

## 23. Existing target-systemd verify remains required

UB does not replace or weaken the selected target verify command:

`/usr/bin/systemd-analyze --user --recursive-errors=no --man=no --generators=no verify prw-agent.service`

The external snapshot is re-proved before and after this parse gate as selected above.

A verify failure with unchanged external custody follows the existing pre-reload managed-file rollback law.

## 24. No foreign semantic takeover

Snapshot capture may reuse the existing read-only `fragment_text_conflicts(...)` semantics to ensure baseline external fragments do not claim selected PRW environment custody.

Exact-byte reproof is intentionally stricter than a second semantic-only pass: any external content change fails, even when the new text would still be semantically non-conflicting.

UB does not interpret, rewrite, normalize, canonicalize, or merge foreign fragment semantics.

## 25. Why descriptor-held manager binding is rejected

UB explicitly rejects the claim:

`holding the baseline file descriptor makes daemon-reload consume the same inode`.

That claim is false for the selected process boundary. systemd reopens unit paths independently.

Holding descriptors across the transaction would therefore add FD pressure and apparent authority without binding manager reads to those descriptors.

Descriptors may be used transiently for safe capture/reproof, but they are not retained as a manager-binding mechanism.

## 26. Why advisory locks are rejected

UB selects no flock/fcntl lock on external fragments.

Such locks are cooperative and do not force package managers, editors, provisioning tools, or systemd itself to participate.

The unprivileged same-user orchestrator also cannot establish a universal write-exclusion authority over the root-owned vendor unit.

No lock-based race-free claim is selected.

## 27. Remaining TOCTOU limitation

Even exact reproof cannot make pathname-based external configuration physically immutable between the final user-space check and systemd's subsequent independent open/read.

A mutation can theoretically occur in that interval.

Post-boundary reproof detects many such cases after the fact, but it does not make the transaction mathematically race-free.

UB therefore retains:

`NO_RACE_FREE_CLAIM`

and forbids wording that says external systemd fragments were locked, frozen, transactionally committed with the manager, or guaranteed immutable.

## 28. Source-materialization successor ceiling

The immediate source successor for this selection is constrained to a maximum net source path set of exactly:

1. `crates/prw-agent-configuration/src/linux_systemd.rs`;
2. `crates/prw-agent-systemd-orchestration/src/lib.rs`.

No Cargo manifest or lockfile change is selected or expected.

No `prw-agent`, desktop, Android, packaging, vendor-unit, identity-provisioning, network, control-plane, or service-source widening is selected.

If implementation genuinely requires another repository path, the successor must stop and return to selection rather than silently widening scope.

## 29. Configuration-crate source successor requirements

The future `linux_systemd.rs` successor should minimally:

1. define one opaque external-fragment custody token;
2. reuse sanitized unit-path discovery;
3. inventory main unit candidates and non-30/40 target drop-ins deterministically;
4. enforce the selected count/byte ceilings;
5. capture external files with final-component NOFOLLOW and regular-file checks;
6. retain exact metadata identity and exact bytes;
7. fail capture on unstable descriptor/path identity;
8. provide read-only reproof against the original token;
9. never log or serialize token content;
10. leave the existing managed writer ownership and recovery token semantics unchanged.

## 30. Orchestration-crate source successor requirements

The future orchestration successor should minimally:

1. capture external custody after baseline active/running/Ready proof and before managed write;
2. reprove after managed write and before target verify;
3. reprove after target verify and before daemon reload;
4. reprove after daemon reload/topology and before try-restart;
5. reprove before any ordinary rollback restart after manager mutation;
6. reprove after post-restart Ready/topology before success;
7. implement the selected external-drift failure disposition;
8. add the single bounded `ExternalConfigurationDrift` stable class;
9. preserve all existing timeout, local IPC, try-restart, and non-drift rollback semantics.

## 31. Required focused configuration tests

The source successor must include isolated tests proving at least:

1. vendor/main unit candidate is included in snapshot custody;
2. non-managed drop-ins, including a `20-` identity-style leaf, are included;
3. exact managed `30/40` leaves in the intended managed directory are excluded;
4. same-path byte change fails reproof;
5. same-path inode replacement with equal bytes fails reproof;
6. mode/UID/GID or selected metadata drift fails reproof;
7. external candidate addition fails reproof;
8. external candidate removal fails reproof;
9. unit-search-path sequence drift fails reproof;
10. symlink/nonregular external fragment fails closed;
11. per-file size ceiling fails closed;
12. file-count/total-byte ceiling fails closed;
13. unchanged exact snapshot reproves successfully;
14. snapshot/reproof never mutates external files.

No test may mutate the real host systemd tree.

## 32. Required focused orchestration tests

The source successor must include fake-backend tests proving at least:

1. baseline custody capture occurs before managed writer mutation;
2. post-write pre-verify drift restores managed files without daemon reload;
3. post-verify pre-reload drift restores managed files without daemon reload;
4. post-reload drift prevents forward try-restart;
5. post-reload drift restores managed files and performs at most one rollback daemon reload without ordinary restart;
6. unchanged custody allows the existing forward try-restart lane;
7. final post-restart custody reproof is required before success;
8. external drift after restart attempt suppresses automatic rollback restart;
9. a non-custody post-reload failure with unchanged custody still uses the existing rollback restart law;
10. a non-custody post-reload failure with drift switches to external-drift disposition;
11. stable error text contains no path/content data;
12. successful path retains both UA topology gates and external custody gates.

All orchestration tests must use the injectable fake backend. They must not call real `systemctl`, `systemd-analyze`, the real writer against host configuration, or the real PRW service.

## 33. Validation expectations

The source successor must run, after all edits:

- focused `prw-agent-configuration` tests;
- focused `prw-agent-systemd-orchestration` tests;
- targeted Clippy with `-D warnings`;
- `cargo fmt --all -- --check`;
- locked workspace metadata validation;
- full-workspace Clippy with `-D warnings`;
- full-workspace tests;
- full-workspace build;
- `git diff --check`.

Exact-head GitHub CI must be recorded only after the final source head is frozen.

`SKIPPED` checks remain `SKIPPED`, never PASS.

## 34. Evidence expectations

UB itself is docs-only and must close using the established evidence pattern:

- exact predecessor/head/tree/parent;
- exact one-contract-path delta;
- exact-head CI;
- immutable raw Markdown publication to the canonical Drive evidence folder;
- byte-identical raw readback;
- exact-title singleton proof;
- one-revision lineage with previous revision `null`;
- metadata-only PR closure binding;
- draft/open/unmerged PR retained.

## 35. Existing laws retained unchanged

UB does not alter:

- exact seven Agent configuration variables and validators;
- intended-user HOME/XDG/runtime-root custody;
- `SYSTEMD_UNIT_PATH` environment clearing;
- managed writer ownership of only 30/40;
- recoverable managed-file token semantics;
- target-systemd warning-fail parse gate;
- forward `try-restart` law;
- ordinary `restart` only where rollback remains valid;
- 5-second readiness deadline and 2-second IPC timeout;
- local IPC correlation/protocol/Ready proof;
- no first-start authority;
- no enablement/linger authority;
- no remote-readiness claim;
- no external fragment mutation/takeover.

## 36. Explicit non-actions / STOP

C03e-UB performs no:

- Rust/Cargo/source materialization;
- real external-fragment snapshot execution as an activation gate;
- real `prw-agent-configure` action;
- real managed writer execution;
- real M30/M40 creation, replacement, or removal;
- external unit/drop-in modification;
- daemon reload;
- start, stop, restart, or try-restart;
- enable/disable mutation;
- linger mutation;
- package/deployment mutation;
- listener/network mutation;
- repository configuration mutation;
- merge, ready conversion, PR close, branch deletion, reset, rebase, squash, force-push, or history rewrite.

Real activation remains separately gated after any future source successor is independently validated and evidence-closed.

## 37. Selected successor statement

The immediate successor to C03e-UB, if approved separately, is a two-path source-materialization checkpoint implementing only the external-fragment snapshot/reproof custody selected here.

It must not perform a real host activation as part of source materialization.
