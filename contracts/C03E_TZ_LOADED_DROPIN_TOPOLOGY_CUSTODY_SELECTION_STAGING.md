# C03e-TZ Loaded systemd drop-in topology custody selection staging

## Status

`SELECTION — SOURCE MATERIALIZATION NOT AUTHORIZED IN TZ`

Boundary:
`LOADED_SYSTEMD_DROPIN_TOPOLOGY_CUSTODY_SELECTION`

C03e-TZ is documentation-only. It selects the exact loaded-`DropInPaths` topology invariant for the already source-materialized C03e-TY `reconfigure-active` lane. It performs no Rust/Cargo/source materialization and no real host or service-manager mutation.

## 1. Authority and predecessor

Authoritative predecessor is evidence-closed C03e-TY / PR #663.

- exact TY final head: `cadeb32f3e2229f183fb97f502889a7800d0053e`;
- exact TY final tree: `dd4e1d63c4dc9a0b37221e6910ef71b993f75ade`;
- exact TY parent: `a58978b9a18d006c64666e3882efd3f88c693b26`;
- exact TX base below TY: `33475fb028c0373d35bb994b07b128575d08d203`.

TY materialized the same-user one-shot orchestration lane selected by TX, including `write`, `reconfigure-active`, recoverable managed-file custody, target-systemd verification, daemon reload separation, forward `try-restart`, bounded local readiness proof and rollback.

TY intentionally did not add a stronger whole-loaded-drop-in invariant because TX had not selected one.

## 2. Selection result

C03e-TZ selects:

`EXACT_BASELINE_FOREIGN_DROPIN_SEQUENCE_INVARIANT / EXACT_MANAGED_30_40_MEMBERSHIP_DELTA / FAIL_CLOSED_ABSOLUTE_UNIQUE_DROPIN_PATHS / POST_RELOAD_REPROOF / POST_RESTART_REPROOF / EXACT_BASELINE_SEQUENCE_ROLLBACK / NO_FOREIGN_CONTENT_OWNERSHIP / NO_RACE_FREE_CLAIM / ONE_SOURCE_PATH_SUCCESSOR_CEILING / NO_REAL_ACTIVATION_IN_TZ`

## 3. Problem closed by this selection

Exact TY `post_reload_matches(...)` currently proves only:

- `LoadState=loaded`;
- unchanged `UnitFileState`;
- intended managed `30-` is present;
- configured-remote managed `40-` is present, or local-only managed `40-` is absent.

It does not reject an unrelated loaded drop-in path that appears, disappears or changes position between the baseline status capture and the post-`daemon-reload` status capture.

Therefore a path-topology race after writer preflight can survive the current forward gate.

Rollback is already stronger: after restored files, reload, restart and readiness, exact TY requires `restored.drop_in_paths == baseline.drop_in_paths`.

TZ closes the asymmetry for the forward path without granting ownership over unrelated drop-ins.

## 4. Managed path identities

For one validated same-user context, define the managed directory exactly as:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d`

Define:

- `M30 = <managed-directory>/30-agent-execution-mode.conf`;
- `M40 = <managed-directory>/40-configured-remote-inputs.conf`.

Only exact path equality with `M30` or `M40` is classified as PRW-managed for this topology invariant.

A same-basename file in another unit search directory is foreign and must remain in foreign topology custody.

## 5. Valid `DropInPaths` representation

Every parsed `DropInPaths` status used by `reconfigure-active` must fail closed unless:

1. every decoded path is absolute;
2. no exact decoded path occurs more than once;
3. decoding succeeds under the existing bounded systemd-path decoder;
4. the complete systemd status payload still contains exactly the selected seven unique properties.

No canonicalization, symlink resolution, filesystem ownership inference or path alias folding is selected for `DropInPaths` comparison.

Comparisons use the exact decoded `PathBuf` identities emitted by the trusted fixed `systemctl --user show` query.

## 6. Baseline foreign sequence

After the existing active/running and local IPC Ready preconditions succeed and before file mutation, the caller already holds baseline `UnitStatus`.

TZ defines:

`baseline_foreign = baseline.drop_in_paths with every exact M30/M40 entry removed, preserving the remaining order exactly.`

Because section 5 rejects duplicate exact paths, this filtering is unambiguous.

The baseline sequence is an observation/custody invariant only. The orchestrator does not open, rewrite, delete, chmod, rename or claim ownership over any foreign path.

## 7. Forward managed delta

After successful recoverable file write, target-systemd verification and successful user-manager `daemon-reload`, the loaded managed membership must be exactly:

### `local_only`

- `M30`: present exactly once;
- `M40`: absent.

### `configured_remote`

- `M30`: present exactly once;
- `M40`: present exactly once.

No other managed-membership interpretation, alias or fallback is selected.

## 8. Exact foreign topology invariant after daemon reload

Let:

`post_foreign = post_reload.drop_in_paths with every exact M30/M40 entry removed, preserving remaining order exactly.`

The post-reload gate succeeds only if:

- `LoadState == loaded`;
- `UnitFileState == baseline.UnitFileState`;
- section 5 valid-path rules pass;
- section 7 exact managed membership passes;
- `post_foreign == baseline_foreign` as an exact ordered sequence.

Therefore all of the following fail closed:

- a new foreign loaded drop-in path;
- disappearance of a baseline foreign loaded drop-in path;
- substitution of one foreign path for another;
- reordering of baseline foreign loaded paths;
- duplicate loaded paths;
- relative loaded paths.

No forward restart is attempted after such failure.

## 9. Why foreign sequence equality is selected instead of constructing one full expected vector

TZ does not synthesize systemd's placement order for newly present `M30`/`M40` relative to foreign paths.

Systemd remains the authority for its loaded ordering. PRW only asserts:

1. exact foreign ordered subsequence stability; and
2. exact desired managed membership.

This avoids reimplementing systemd drop-in precedence while still detecting every foreign path-topology change visible through `DropInPaths`.

## 10. Post-restart reproof

The bounded readiness helper already returns the final `UnitStatus` after forward `try-restart` and local IPC Ready proof.

TZ requires the caller to apply the same section 5, section 7 and section 8 topology law to that returned status before classifying the transaction as successful.

Thus successful forward completion requires the selected loaded topology invariant at both:

1. the post-`daemon-reload`, pre-restart gate; and
2. the post-`try-restart`, Ready gate.

A topology mismatch at the second gate is a post-restart failure and enters the already-selected rollback path.

## 11. Rollback loaded topology

Rollback retains the stronger exact baseline restoration rule already present in TY.

After exact managed-file restore, one rollback `daemon-reload`, one rollback `restart`, and bounded local readiness proof, the returned status must satisfy:

- existing active/running/readiness requirements;
- captured `UnitFileState` unchanged;
- valid section 5 `DropInPaths` representation;
- `restored.drop_in_paths == baseline.drop_in_paths` as an exact ordered sequence.

This is not weakened to foreign-only comparison.

Any mismatch is terminal rollback failure. No further retry, cleanup or takeover is selected.

## 12. Error-surface selection

No new public stable orchestration error class is required by TZ.

- invalid baseline `DropInPaths` remains baseline systemd failure;
- post-reload topology mismatch remains `PostReloadState`;
- post-restart topology mismatch remains post-restart readiness/transaction failure and invokes rollback;
- rollback topology mismatch remains `Rollback`.

Raw foreign paths must not be added to stable error strings or logs by this checkpoint.

## 13. Residual same-path content race

`DropInPaths` proves loaded path topology, not byte identity of foreign unit/drop-in files.

A foreign file can theoretically change contents while retaining the same path. Whole-sequence path equality alone cannot prove that its bytes, inode or semantic directives remained unchanged across the transaction.

The TW/TY writer performs read-only competing-environment custody inspection before managed-file staging, but TZ does not convert that preflight into persistent foreign-file ownership, locking or content fingerprint custody.

Accordingly TZ explicitly selects:

`NO_FOREIGN_CONTENT_STABILITY_CLAIM`

and explicitly rejects the phrase `race-free activation` for this boundary.

Any decision to require foreign-file byte/inode fingerprints, a second read-only semantic custody pass, descriptor-held foreign custody, or another stronger concurrency mechanism requires a separate selection checkpoint.

## 14. What TZ does prove

After a future source successor implements this selection, a successful active reconfiguration may claim only that:

- the same-user active/ready precondition was satisfied;
- managed desired files passed existing writer custody and exact-byte verification;
- target-systemd verification passed;
- foreign loaded drop-in path topology was unchanged from baseline at both selected forward gates;
- managed loaded `30/40` membership matched the desired mode at both forward gates;
- `UnitFileState` remained unchanged;
- bounded local Agent readiness passed.

It still does not prove remote reachability, peer authentication, control-plane health, deployment completion or foreign-file byte stability.

## 15. Source-materialization successor ceiling

A future source-materialization checkpoint for this selection is constrained to a maximum net source path set of:

`crates/prw-agent-systemd-orchestration/src/lib.rs`

No Cargo manifest or lockfile change is selected or expected.

No `prw-agent`, `prw-agent-configuration`, desktop, Android, vendor-unit, packaging or service-source widening is selected.

If implementation genuinely requires another source path, the successor must stop and return to selection rather than silently widening scope.

## 16. Selected implementation shape

The future one-path source successor should minimally:

1. harden `parse_drop_in_paths(...)` with absolute-path and duplicate rejection;
2. derive the two exact managed paths from the already validated `UserContext`;
3. derive the ordered baseline foreign sequence by filtering exact managed identities;
4. replace presence-only `post_reload_matches(...)` logic with the selected managed-membership plus foreign-sequence invariant;
5. reapply the same invariant to the final `UnitStatus` returned after forward `wait_ready(...)`;
6. retain exact full baseline ordered equality on rollback.

No new CLI argument, environment variable, persistent state, lock file or external dependency is selected.

## 17. Required successor tests

The future source successor must include focused tests, in the existing inline test module, for at least:

1. local-only accepts unchanged foreign sequence plus exact `M30` and no `M40`;
2. configured-remote accepts unchanged foreign sequence plus exact `M30` and `M40`;
3. post-reload foreign addition fails before forward restart;
4. post-reload foreign removal fails before forward restart;
5. post-reload foreign substitution fails before forward restart;
6. post-reload foreign reordering fails before forward restart;
7. duplicate exact loaded path fails closed;
8. relative loaded path fails closed;
9. wrong managed membership fails closed for each mode;
10. post-restart foreign topology drift invokes rollback;
11. successful post-restart path proves the invariant twice;
12. rollback still requires exact full ordered baseline equality.

Tests must use the injectable fake backend or isolated parsing helpers only. They must not execute real `daemon-reload`, restart, writer mutation or PRW service activation.

## 18. Existing laws retained unchanged

TZ does not alter:

- exact seven configuration variables and validators;
- intended-user HOME/XDG/runtime-root custody;
- `SYSTEMD_UNIT_PATH` environment clearing;
- managed writer 30/40 ownership boundary;
- recoverable token semantics;
- target-systemd verify command;
- forward `try-restart` law;
- rollback-only ordinary `restart` law;
- 5-second readiness deadline and 2-second IPC timeout;
- local IPC correlation/protocol/Ready proof;
- no enablement or linger authority;
- no first-start authority;
- no remote readiness claim.

## 19. Real-host observation is not activation authority

Read-only archaeology on the current host observed `prw-agent.service` as:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `UnitFileState=enabled`;
- empty current `DropInPaths`.

This is only a current read-only observation. Under existing TX/TY law, `reconfigure-active` would reject this state before file mutation because the service is not active/running/Ready.

TZ selects no repair, start or recovery action for it.

## 20. Validation and evidence expectations

TZ itself is one docs-only contract file.

Validation must prove:

- exact parent is evidence-closed TY final head;
- exact net delta is one contract path only;
- repository whitespace/diff integrity is clean;
- exact-head CI is recorded accurately, including `SKIPPED` as `SKIPPED`, never PASS;
- immutable canonical Drive evidence is published only after final exact-head CI;
- evidence raw-byte readback, singleton canonical name/parent and revision lineage are verified before PR closure binding.

## 21. Explicit non-actions / STOP

C03e-TZ performs no:

- Rust/Cargo/source materialization;
- `prw-agent-configure` execution;
- real managed writer execution;
- real `30-`/`40-` creation, replacement or removal;
- `daemon-reload`;
- start, stop, restart or try-restart;
- enable/disable mutation;
- linger mutation;
- foreign drop-in mutation or takeover;
- deployment or package mutation;
- listener/network mutation;
- repository configuration mutation;
- merge, ready conversion, PR close, branch deletion or history rewrite.

Real activation remains separately gated.

## 22. Selection closure

If exact-head validation and immutable evidence closure succeed, C03e-TZ closes only the loaded drop-in topology selection.

The immediate source successor may materialize the one-path law in `crates/prw-agent-systemd-orchestration/src/lib.rs` and nothing else.

Disposition of the residual same-path foreign-content race remains separate and must not be implied by TZ closure.
