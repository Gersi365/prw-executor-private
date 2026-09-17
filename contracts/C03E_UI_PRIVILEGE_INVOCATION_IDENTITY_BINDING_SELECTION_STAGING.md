# C03e-UI — Privilege invocation identity binding selection

## 1. Status and boundary

`SELECTION — SUDO_RS_INJECTED_CALLER_IDENTITY — SOURCE CORRECTION NOT INCLUDED`

Boundary:

`PRIVILEGE_INVOCATION_IDENTITY_BINDING_SELECTION`

C03e-UI is a docs-only selection checkpoint. It resolves the privilege-invocation identity blocker frozen by evidence-closed C03e-UH without correcting Rust source, creating a deployment stage, authenticating sudo, mutating sudoers, or performing any package/systemd mutation.

The selected mechanism is intentionally narrow: when the fixed-purpose reconciler is launched through the host's authenticated sudo-rs boundary as root, source should recover the authenticated unprivileged caller identity from sudo-rs-injected `SUDO_UID` / `SUDO_GID` / `SUDO_USER`, not from `getuid()` after sudo has switched the command to root.

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UH / PR #672.

Exact predecessor identity:

- branch: `phase-152-c03e-uh-package-reconciliation-deployment-preflight`;
- head: `5e13b7a6d016a4bc903e4d8e8bbbcce75a55afdb`;
- tree: `d548a33209c1fda7aa34fb47f28d7415ecd715c9`;
- status: `PREFLIGHT — VALIDATED — EVIDENCE_RECORDED — CLOSED — DEPLOYMENT BLOCKED`;
- retained open, draft and unmerged.

UH proved the package/service baseline but stopped because current UG source requires effective UID `0`, derives stage owner from `getuid()`, and rejects stage-owner UID `0`. It therefore could not freeze an exact interactive sudo command under the then-assumed UID semantics.

UI does not reopen or bypass that stop. It selects a source correction for a later separately authorized checkpoint.

## 3. Repository and main state

Canonical repository:

`Gersi365/prw-executor-private`

Repository stable ID:

`1334911207`

`main` remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

UI does not mutate `main`.

## 4. Exact host sudo implementation discovery

Read-only host inspection on PowerCode established that the active sudo entry point is not the classic sudo binary assumed conservatively in UH.

Observed:

- `/usr/bin/sudo` resolves to `/usr/lib/cargo/bin/sudo`;
- the resolved target is root-owned set-user-ID mode `4755`;
- target size `1090848` bytes;
- installed packages include classic `sudo 1.9.17p2-1ubuntu3`, `sudo-common 1.2ubuntu`, and `sudo-rs 0.2.13-0ubuntu1.2`;
- Ubuntu 26.04 uses sudo-rs as the enabled sudo implementation by default, selectable through the distribution alternatives mechanism;
- the host's local `sudo(8)` manual identifies itself as sudo-rs `0.2.13`.

The ordinary unprivileged PRW session had no ambient `SUDO_UID`, `SUDO_GID`, `SUDO_USER`, or `SUDO_COMMAND` variables.

Current intended operator account remained:

- name `gersi365`;
- UID `1000`;
- primary GID `1000`;
- primary group `gersi365`;
- home `/home/gersi365`;
- shell `/bin/bash`.

No authenticated sudo command was executed merely to discover these facts.

## 5. Exact sudo-rs upstream identity used for semantic archaeology

The upstream sudo-rs `v0.2.13` annotated tag resolves to:

- tag object `6c4841efa09ca5774e116a8bc256c362b9130db3`;
- release commit `965cd7b99b04faf55819606178a5e8233cfd8b9e`;
- tag date `2026-03-10`;
- verified tag signature reported by GitHub.

Ubuntu's installed package is `0.2.13-0ubuntu1.2`. Ubuntu package metadata identifies the source as `rust-sudo-rs 0.2.13-0ubuntu1.2`; Ubuntu's September 2026 security notice for `.1.2` describes a sudoedit TOCTOU correction. UI does not infer a binary-level equivalence claim beyond the host/package/version observations, but the selected direct-command environment law is also explicit in the upstream v0.2.13 source and in sudo-rs compatibility documentation.

## 6. sudo-rs current-user law

In exact upstream v0.2.13 source, `Context::from_run_opts` resolves:

`current_user = CurrentUser::resolve()`

before target-command execution.

`CurrentUser::resolve()` resolves `User::real()`.

`User::real()` resolves the process real UID through `getuid()` and looks up that UID as the current user.

Therefore sudo-rs's `context.current_user` represents the invoking user of sudo-rs before the target identity switch.

For the intended host invocation, that invoking identity is the authenticated operator, currently UID/GID `1000:1000`.

## 7. sudo-rs injected environment law

Exact upstream v0.2.13 `src/sudo/env/environment.rs` constructs the target command environment after filtering the invoking environment.

The `add_extra_env` step explicitly inserts:

- `SUDO_COMMAND` from the resolved requested command;
- `SUDO_UID` from `context.current_user.uid`;
- `SUDO_GID` from `context.current_user.gid`;
- `SUDO_USER` from `context.current_user.name`;
- `SUDO_HOME` from `context.current_user.home`.

Critically, these are inserted by sudo-rs itself after the ordinary environment-preservation/filtering step. The compatibility documentation also states that `SUDO_COMMAND`, `SUDO_GID`, `SUDO_UID`, and `SUDO_USER` cannot be preserved from the invoking user's environment because sudo sets them itself.

Therefore, for a command successfully launched by sudo-rs, the selected `SUDO_*` values are not merely caller-preserved arbitrary environment values; they are sudo-rs outputs derived from its resolved invoking-user context.

## 8. sudo-rs target-root execution law

Exact upstream v0.2.13 source sets the command's target user using `setgid` and `setuid` before exec.

The source comment records that these calls set real, effective and saved GID/UID to the target values.

For the default root target, the child command therefore runs with root real/effective identity even though the injected `SUDO_UID` / `SUDO_GID` / `SUDO_USER` retain the original sudo caller identity.

This directly explains the UH mismatch:

- reconciler `getuid()` under ordinary sudo-rs target-root execution is expected to be `0`;
- sudo-rs's injected `SUDO_UID` retains the authenticated invoking UID, e.g. `1000`.

UI therefore rejects the earlier assumption that real UID preservation through `stay_setuid` is required for this host mechanism.

## 9. Selected identity authority

UI selects:

`SUDO_RS_INJECTED_CALLER_IDENTITY`

with the following source-level authority split:

1. `geteuid()==0` remains mandatory for privileged package mutation.
2. `getuid()==0` should also be required for this sudo-target-root invocation shape, so the source does not silently reopen the earlier setuid-helper execution model.
3. `SUDO_UID` is the authoritative intended stage-owner UID carried across the sudo identity switch.
4. `SUDO_GID` is a mandatory companion value and should bind the private stage directory's GID as an additional consistency check.
5. `SUDO_USER` is mandatory and must be non-empty and non-`root`, but it is provenance/consistency metadata only; it must not be used to construct paths, select destinations, select hashes, or widen authority.
6. `SUDO_COMMAND`, `USER`, `LOGNAME`, `HOME`, `SUDO_HOME`, arbitrary environment variables, current working directory and caller-provided executable names are not identity authority.

The source correction must fail closed if the selected sudo-rs identity tuple is absent, malformed or inconsistent with stage custody.

## 10. Exact parsing law

The successor source should read only these fixed environment keys for invoking identity:

- `SUDO_UID`;
- `SUDO_GID`;
- `SUDO_USER`.

`SUDO_UID` and `SUDO_GID` should be accepted only as canonical ASCII unsigned decimal strings:

- digits `0` through `9` only;
- no sign;
- no whitespace;
- no separator;
- no prefix/suffix;
- no overflow beyond the platform UID/GID representation;
- no leading zeros for a non-zero value.

For this mechanism both selected caller UID and GID must be non-zero.

`SUDO_USER` must be present, non-empty and not equal to `root`. Source should not require UTF-8 merely to obtain numeric ownership authority; if implementation chooses an `OsString`/`OsStr` representation, no lossy conversion is required.

No fallback is selected if any field fails validation.

## 11. Stage-custody binding selected

The existing UG stage custody remains authoritative and is narrowed as follows.

The stage directory must remain:

- absolute and normalized;
- component-wise no-symlink;
- descriptor-opened with the selected `openat2` resolve law;
- exact mode `0700`;
- owned by parsed `SUDO_UID`;
- group-owned by parsed `SUDO_GID`;
- not group/other writable.

The fixed candidate must remain:

- regular and non-symlink;
- opened descriptor-relative beneath the validated stage;
- owned by the exact stage/SUDO UID;
- not group/other writable;
- exact selected Agent length/hash;
- identity-stable around hashing.

The fixed manifest must remain:

- regular and non-symlink;
- same stage/SUDO UID owner;
- exact mode `0600`;
- bounded length;
- exact deterministic bytes.

UI does not weaken any candidate, manifest or package-destination custody rule.

## 12. Why no passwd-database lookup is selected in the reconciler

UI does not require the reconciler to add a user-database dependency or unsafe libc account lookup solely to re-resolve `SUDO_USER`.

Reason:

- sudo-rs v0.2.13 already resolves its real invoking UID through its `User` account representation before it injects `SUDO_UID`, `SUDO_GID` and `SUDO_USER`;
- the reconciler needs numeric ownership authority, for which `SUDO_UID`/`SUDO_GID` map directly to file metadata;
- adding an account lookup would expand source/dependency surface without strengthening the boundary against a malicious root process;
- a malicious root process can synthesize environment and mutate the package destination directly regardless.

`SUDO_USER` is therefore retained as mandatory provenance metadata, while numeric stage ownership is enforced from the injected UID/GID pair.

## 13. Threat-model statement

The selected SUDO_* tuple is not a cryptographic identity token and is not a security boundary against already-privileged malicious root.

It is selected to preserve the identity authenticated and resolved by sudo-rs across sudo-rs's deliberate target-root UID switch.

A root process can synthesize these environment variables, but root already possesses direct authority over the fixed package destination. PRW's relevant security property here is that unprivileged/request-controlled data cannot select another package destination, expected hash, arbitrary executable, or arbitrary privileged action.

UI makes no claim that SUDO_* can constrain a hostile root administrator.

## 14. No arbitrary environment authority

The successor must not expose a generic environment map or accept a caller-selected variable name.

Only the exact three fixed SUDO_* keys above may influence invoking-user custody.

They may influence only:

- expected stage UID;
- expected stage GID;
- mandatory provenance-presence validation.

They must never influence:

- production Agent destination;
- vendor-unit destination;
- old/new/unit hashes;
- candidate filename;
- manifest filename;
- semantic action;
- root staging filename family;
- systemd manager action;
- command execution;
- network/database/auth behavior.

## 15. Alternative: retain `getuid()` stage binding

Rejected.

Under the discovered sudo-rs target-root law, the reconciler child has real UID root. Retaining current `getuid()` stage ownership therefore reproduces UH's deterministic `stage_custody_invalid` blocker for a private user-owned stage.

No further preflight can repair that mismatch without changing source or changing the privilege mechanism.

## 16. Alternative: sudoers `stay_setuid`

Not selected.

Reasons:

- the discovered sudo-rs mechanism already carries invoking identity explicitly through SUDO_*;
- UF/UH prohibit modifying sudoers merely to make package reconciliation work;
- depending on a non-default policy option would create host-policy coupling and a larger configuration authority surface;
- it would preserve the earlier `getuid()` design rather than using the explicit sudo-provided caller identity channel.

UI therefore requires no `stay_setuid` configuration and no sudoers mutation.

## 17. Alternative: explicit UID/GID CLI arguments

Rejected.

A CLI such as:

`reconcile-current-agent --caller-uid 1000 --caller-gid 1000 <stage>`

would widen the caller-controlled invocation surface and make identity a free-form input rather than a value injected by the privilege boundary.

No arbitrary UID, GID or username argument is selected.

## 18. Alternative: `SUDO_USER` alone

Rejected as sole authority.

A name string does not map directly to filesystem ownership metadata and would require a separate account lookup to obtain UID/GID. Numeric sudo-rs-injected UID/GID already exist and are the narrower file-custody primitives.

`SUDO_USER` remains mandatory provenance metadata, not sole authority.

## 19. Alternative: set-user-ID PRW helper

Not selected.

Installing a persistent PRW setuid-root helper would introduce a new privileged executable installation/maintenance surface and would require a separate root-owned installation transaction before the package transaction it is meant to authorize.

UI preserves the already selected explicitly initiated interactive sudo boundary instead.

## 20. Alternative: parent-process or `/proc` identity inference

Rejected.

Parent PID/process topology is implementation-dependent, especially when sudo uses PTYs, monitors or intermediate processes. Process ancestry is unnecessary because sudo-rs already exports the resolved caller identity directly.

No `/proc/<ppid>` executable-name, PID, terminal or process-tree heuristic is selected as identity authority.

## 21. Alternative: login UID/session identity

Not selected.

Kernel audit/login session identifiers are not the same semantic primitive as the user whose credentials sudo-rs authenticated for this command and can be absent, inherited across session transitions, or unsuitable outside one login mechanism.

UI binds to sudo-rs's own authenticated caller context instead.

## 22. Exact source-successor ceiling

The immediate separately authorized source successor should correct only the package reconciler's invoking-user resolution.

Expected path ceiling:

`crates/prw-agent-package-reconciliation/src/lib.rs`

No Cargo dependency change is expected because:

- `std::env::var_os` is sufficient to read fixed variables;
- existing `rustix` process access already supplies real/effective UID checks;
- existing Unix metadata supplies UID/GID stage checks;
- no passwd-database lookup is selected.

If implementation archaeology proves another repository path or dependency is required, the source checkpoint must STOP and return to selection before widening scope.

## 23. Suggested source shape

A successor may materialize a small internal value such as:

`InvokingUserIdentity { uid, gid, user }`

Production resolution should:

1. prove real UID root;
2. prove effective UID root;
3. read exact fixed `SUDO_UID`, `SUDO_GID`, `SUDO_USER` keys;
4. parse UID/GID under the canonical numeric law;
5. reject UID/GID zero;
6. reject missing/empty/root `SUDO_USER`;
7. pass the resolved UID/GID into stage validation;
8. require stage metadata UID/GID to match;
9. keep all package destinations/hashes compile-time fixed.

The parser should be structured as a pure function over supplied `OsStr` values where possible so tests need not mutate process-global environment in parallel.

## 24. Bounded failure classification

UI selects one new bounded pre-package failure classification for absent/malformed sudo caller identity:

`invoking_user_identity_invalid`

It should occur before stage candidate, manifest, package-parent, installed-Agent, vendor-unit or root staging mutation.

Existing `stage_custody_invalid` remains appropriate when the sudo identity itself is valid but the selected stage metadata/path custody does not match it.

This distinction preserves diagnostic precision without exposing paths, environment values or usernames in production error output.

## 25. Required successor tests

The source-correction checkpoint should add focused tests proving at least:

- exact valid `SUDO_UID=1000`, `SUDO_GID=1000`, non-root user parses;
- missing UID rejected;
- missing GID rejected;
- missing user rejected;
- empty user rejected;
- `root` user marker rejected;
- UID zero rejected;
- GID zero rejected;
- signed/whitespace/non-decimal/overflow numeric values rejected;
- non-canonical leading-zero numeric values rejected;
- stage owner UID mismatch rejected;
- stage GID mismatch rejected;
- valid stage with selected UID/GID accepted in disposable tests;
- arbitrary CLI UID/GID arguments remain rejected;
- old/new/unit hashes and fixed destinations remain unchanged;
- no process execution/systemd/identity/20/30/40 authority is introduced.

Full exact-head repository CI and immutable evidence closure remain required.

## 26. Current host invocation implication

After a future evidence-closed source correction and a fresh read-only deployment preflight, the intended privilege shape may again be considered as an ordinary authenticated sudo-rs direct command:

`sudo -- <exact-reconciler-path> reconcile-current-agent <exact-private-stage-path>`

UI does not freeze or authorize an executable path, stage path or real command yet.

The later preflight must first prove:

- exact corrected reconciler binary identity;
- exact private stage and candidate/manifest custody;
- active sudo implementation/version still matches the selected semantic assumptions or is re-verified;
- installed old Agent/vendor unit still exact;
- service still non-running and `MainPID=0` immediately before handoff;
- no root staging residue;
- no unexpected SUDO_* ambient state in the unprivileged preflight context.

## 27. sudo implementation drift law

The selected source mechanism intentionally depends on standard sudo-style injected variables and is concretely evidenced against current host sudo-rs `0.2.13-0ubuntu1.2` plus exact upstream v0.2.13 source.

A later real deployment preflight must re-read the active `/usr/bin/sudo` implementation and version.

If the host switches to a different privilege implementation that does not inject compatible `SUDO_UID` / `SUDO_GID` / `SUDO_USER` semantics, deployment must STOP rather than silently falling back to `getuid()`, CLI UID arguments or weaker stage custody.

## 28. No environment-spoof claim against root

A direct root shell could manually set SUDO_* values. UI does not claim otherwise.

This does not invalidate the selected boundary for the intended flow because:

- the authenticated sudo-rs process itself overwrites these variables for its target command;
- the source requires root execution before trusting them;
- a malicious root process already has stronger direct mutation authority than this helper;
- PRW does not use these variables to widen package destination/action authority.

No statement in later evidence should describe the tuple as tamper-proof after arbitrary root compromise.

## 29. Executable custody remains separately preflighted

UI selects caller-identity binding only.

It does not by itself prove that a future reconciler executable path is safe for root execution. A later deployment preflight must freeze the exact reconciler bytes/hash/path/owner/mode and state the local-admin trust assumption for the interactive handoff.

No request-controlled executable or remote-supplied executable is authorized by UI.

If later executable-custody archaeology reveals a source or privilege-mechanism widening is required, execution must STOP and return to selection.

## 30. No race-free claim

SUDO_* binding resolves who sudo-rs authenticated; it does not make the overall deployment transaction race-free.

The service remains enabled and linger remains enabled in current evidence. A fresh non-running/MainPID read is still required immediately before future privilege handoff, and UG's source still makes no systemd manager call.

Candidate/stage descriptor custody and package-file exact-identity checks remain the transaction's file-level race mitigations.

## 31. Canonical selection

UI selects:

`INTERACTIVE_SUDO_RS / TARGET_ROOT_REAL_AND_EFFECTIVE_UID / SUDO_UID_STAGE_OWNER_AUTHORITY / SUDO_GID_STAGE_GROUP_CROSSCHECK / SUDO_USER_REQUIRED_PROVENANCE / FIXED_ENV_KEYS_ONLY / CANONICAL_NUMERIC_PARSE / FAIL_CLOSED_ON_MISSING_OR_MISMATCH / NO_PASSWD_LOOKUP / NO_EXPLICIT_UID_ARG / NO_STAY_SETUID / NO_SUDOERS_MUTATION / NO_SETUID_PRW_HELPER / NO_PROCESS_ANCESTRY_HEURISTIC / NO_ARBITRARY_ENV_AUTHORITY / NO_ROOT_ADVERSARY_CLAIM / NO_RACE_FREE_CLAIM`

This selection resolves the specific UH source-design blocker without authorizing source correction or deployment.

## 32. Exact-head validation expectations

UI is docs-only. Its own closure should require:

- exact predecessor UH head/tree/base proof;
- exact one-contract-path delta;
- contract blob/bytes/SHA-256/final-LF identity;
- `git diff --check`;
- exact-head repository CI registration;
- `SKIPPED` workflows represented as skipped, never PASS;
- immutable raw Markdown audit in the canonical Drive evidence folder;
- byte-identical raw readback;
- exact-title singleton proof;
- one-revision lineage with previous revision `null`;
- metadata-only PR closure binding;
- PR retained open, draft and unmerged.

## 33. Explicit non-actions / STOP

C03e-UI performs no:

- package reconciler Rust/source correction;
- Cargo dependency mutation;
- intended-user deployment-stage creation;
- Agent candidate chmod/copy/move;
- reconciler deployment staging;
- authenticated sudo execution;
- sudoers or alternatives mutation;
- NOPASSWD/stored-password configuration;
- root-owned package replacement;
- vendor-unit rewrite;
- root sibling creation;
- systemd manager mutation;
- service start/stop/restart/try-restart;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation;
- main mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite;
- destructive evidence cleanup.

STOP after evidence closure of this selection.

The next safe boundary is a separately authorized source-correction checkpoint, expected to modify only `crates/prw-agent-package-reconciliation/src/lib.rs` to replace post-sudo `getuid()` stage-owner derivation with the exact selected sudo-rs injected identity law and focused tests. No real deployment is implied by that source correction.