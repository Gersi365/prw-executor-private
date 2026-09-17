# C03e-UF — Current Agent deployment provenance and package reconciliation selection

## 1. Status and boundary

`SELECTION — REAL HOST PACKAGE MUTATION NOT INCLUDED`

Boundary:

`CURRENT_AGENT_DEPLOYMENT_PROVENANCE_PACKAGE_RECONCILIATION_SELECTION`

C03e-UF is a docs-only selection checkpoint. It selects the exact build-provenance law and the narrow privileged, non-activating package-file reconciliation mechanism required before the UE bootstrap sequence may proceed.

UF itself performs no root-owned package replacement, no privileged helper execution, no `systemctl` or `loginctl` mutation, no daemon reload, no service start/stop/restart/try-restart, no enablement/linger mutation, no identity creation, no 20/30/40 mutation, and no deployment.

## 2. Authoritative predecessor

The authoritative predecessor is evidence-closed C03e-UE / PR #669.

Exact predecessor identity:

- branch: `phase-152-c03e-ue-host-baseline-bootstrap-reconciliation-selection`;
- head: `10714024a4df71bd3b5d0232bb0c6b6d7c9fb71f`;
- tree: `cd0218229280c470e8995b3341f2461afd660523`;
- parent/base: evidence-closed C03e-UD `270aa5bf7d14a1f86d51b9a1ed6415e3ec1c63c5`.

UE selected a staged baseline bootstrap and explicitly placed current-Agent deployment provenance / privileged non-activating package reconciliation before identity integration, managed local-only configuration and first start.

UE did not select a final release build profile, package signature mechanism or privileged replacement implementation. UF closes that selection gap without executing it.

## 3. Repository and main state

Canonical repository:

`Gersi365/prw-executor-private`

Repository stable ID:

`1334911207`

Default branch was refreshed before UF materialization and remained:

- `main` head: `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- `main` tree: `63b8e59ca53797fdea6b95432e16f35eaf473604`.

UF does not mutate `main`.

## 4. Current real-host package state

Read-only refresh on PowerCode immediately before UF selection observed:

- installed Agent path: `/usr/lib/private-remote-workspace/prw-agent`;
- installed Agent: regular file;
- installed Agent owner: root;
- installed Agent mode: `0755`;
- installed Agent bytes: `2865776`;
- installed Agent SHA-256: `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`;
- vendor unit path: `/usr/lib/systemd/user/prw-agent.service`;
- vendor unit: regular file;
- vendor unit owner: root;
- vendor unit mode: `0644`;
- vendor unit bytes: `332`;
- vendor unit SHA-256: `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

The installed vendor unit is byte-identical to the repository vendor-unit source at the exact UE predecessor.

Current user-service state remained:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `UnitFileState=enabled`.

Noninteractive sudo remained unavailable.

UF treats this state as read-only selection evidence, not as authorization to repair, start, stop, reload, disable, enable, or replace anything.

## 5. Historical installed-Agent provenance

The installed Agent hash `4dbb114e...` is historical Phase 126 evidence-bound material.

Phase 126 A04/A05 bound that hash to the identity-aware candidate from source commit `1e0b5eec93538bcbf120fa800cf85227eeb32410`.

That historical provenance does not establish equivalence with the current UE source tree. Later source checkpoints added material Agent/configuration/control-plane behavior.

UF therefore classifies the installed Agent as:

`KNOWN_PRIOR_PRW_MANAGED_PAYLOAD / CURRENT_SOURCE_PROVENANCE_STALE`

This classification is sufficient to permit a separately authorized bounded upgrade transaction after exact old/new identities are locked. It is not a current-runtime readiness claim.

## 6. Phase 107 package transaction retained

UF retains the Phase 107 non-activating package-file transaction law.

Fixed package destinations remain exactly:

- Agent: `/usr/lib/private-remote-workspace/prw-agent`;
- vendor unit: `/usr/lib/systemd/user/prw-agent.service`.

Expected final metadata remain:

- Agent: root-owned regular file, mode `0755`;
- vendor unit: root-owned regular file, mode `0644`;
- Agent parent: root-owned system directory, mode `0755`.

Unknown or foreign files at these destinations must never be silently overwritten.

The package transaction must never call `systemctl`, `loginctl`, daemon reload, enablement, start, restart, try-restart, linger mutation, network activation or identity/configuration mutation.

## 7. Phase 109 rollback anomaly retained as a design warning

Phase 109 demonstrated a real-host privileged package install whose visible terminal marker claimed rollback while read-only reconciliation proved the exact new files remained installed.

UF does not reinterpret that event as successful rollback.

The lesson retained by UF is:

- a privileged transaction's own terminal classification is not sufficient evidence of final package state;
- every privileged package mutation requires independent exact post-readback;
- rollback success may be claimed only when the final destination bytes/metadata independently prove restoration;
- ambiguous rollback disposition must stop later stages.

UF therefore rejects a helper design that reports `rolled_back` without exact final-state proof.

## 8. Historical Phase 126 helper is not reusable

The old Phase 126 A05 helper remains present in user-owned historical staging and has its historical SHA-256:

`a75bcc1d44f788e34b12303d4446a8c31de5cb60858d56b59956ea990905ea64`

That helper is hard-coded for the old installed/candidate hash transition and historical stage path.

UF classifies it:

`HISTORICAL_EVIDENCE_ONLY / STALE_FOR_UF / MUST_NOT_EXECUTE`

Its existence is not current deployment authority.

UF does not select copying, patching, parameterizing or executing that shell helper.

## 9. Current source identity used for provenance study

The current source-bearing authority for UF candidate construction is exact evidence-closed UE:

- commit: `10714024a4df71bd3b5d0232bb0c6b6d7c9fb71f`;
- tree: `cd0218229280c470e8995b3341f2461afd660523`;
- `Cargo.lock` SHA-256: `e2d650e7a60663b651f8dfa3013eda729d1b02e763d2c3d8cf1823f43121e909`;
- vendor unit SHA-256: `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- vendor unit bytes: `332`.

Observed toolchain/environment identity:

- `rustc 1.97.1 (8bab26f4f 2026-07-14)`;
- `cargo 1.97.1 (c980f4866 2026-06-30)`;
- host target: `x86_64-unknown-linux-gnu`;
- LLVM: `22.1.6`;
- GNU ld: Ubuntu binutils `2.46`;
- C compiler: Ubuntu GCC `15.2.0-16ubuntu1`;
- observed kernel: Linux `7.0.0-31-generic` x86_64;
- `RUSTFLAGS`: unset;
- `CARGO_ENCODED_RUSTFLAGS`: unset;
- `RUSTC_WRAPPER`: unset;
- `RUSTC_WORKSPACE_WRAPPER`: unset;
- `SOURCE_DATE_EPOCH`: unset;
- `CARGO_BUILD_TARGET`: unset.

The Agent crate has no crate-local `build.rs`.

## 10. Initial repeat-build experiment and failure

UF first tested two clean release builds from exact UE using independent target directories.

Exact semantic command in each target:

`cargo build --locked --release -p prw-agent --bin prw-agent`

Build A:

- target directory: `/tmp/prw-c03e-uf-provenance.1x4349/target-a`;
- result: PASS;
- bytes: `11068312`;
- SHA-256: `461443fc36fbda345baf31d67be67de39398ca84a4654b2b6d9a1d612584eb0a`.

Build B:

- target directory: `/tmp/prw-c03e-uf-provenance.1x4349/target-b`;
- result: PASS;
- bytes: `11068312`;
- SHA-256: `9e8f7892600eb6e9ce8eedf207532332cce7e3b4530cfdf78b7ff00c3626ca6f`.

Result:

`INDEPENDENT_DIFFERENT_TARGET_PATHS_BYTE_IDENTICAL=NO`

This is a provenance-model finding, not a functional code-failure claim.

## 11. Provenance-model drift cause observed

The two binaries embed different absolute generated-source paths from `etcd-client 0.19.0`.

Observed examples:

- A contains `/tmp/prw-c03e-uf-provenance.1x4349/target-a/release/build/etcd-client-a29d23b127d7efe3/out/etcdserverpb.rs`;
- B contains `/tmp/prw-c03e-uf-provenance.1x4349/target-b/release/build/etcd-client-a29d23b127d7efe3/out/etcdserverpb.rs`.

The binaries differ in more than the GNU build-id; differing sections include `.rela.dyn`, `.rodata` and `.text`.

Therefore the historical Phase 126 law `independent clean target directories imply byte-identical Agent output` is no longer valid for this exact dependency graph without additional path normalization.

UF makes no path-independent reproducibility claim.

## 12. Canonical path-bound repeat-build experiment

UF then selected one exact absolute target path for the provenance experiment:

`/tmp/prw-c03e-uf-canonical-target`

Exact command:

`CARGO_TARGET_DIR=/tmp/prw-c03e-uf-canonical-target cargo build --locked --release -p prw-agent --bin prw-agent`

Build C:

- target directory absent/clean before build;
- exact UE source;
- result: PASS;
- bytes: `11068384`;
- SHA-256: `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

The complete target directory was then removed before the second build.

Build D:

- recreated from zero at the exact same absolute target path;
- same exact UE source;
- same exact command;
- result: PASS;
- bytes: `11068384`;
- SHA-256: `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

Result:

`CANONICAL_FIXED_TARGET_PATH_REPEAT_BUILD_BYTE_IDENTICAL=YES`

Tracked repository source remained clean.

## 13. Selected candidate provenance law

UF selects the exact candidate Agent identity:

- source head: `10714024a4df71bd3b5d0232bb0c6b6d7c9fb71f`;
- source tree: `cd0218229280c470e8995b3341f2461afd660523`;
- `Cargo.lock` SHA-256: `e2d650e7a60663b651f8dfa3013eda729d1b02e763d2c3d8cf1823f43121e909`;
- build profile: Cargo `release`;
- package/bin: `-p prw-agent --bin prw-agent`;
- locked dependencies: required;
- canonical absolute target path: `/tmp/prw-c03e-uf-canonical-target`;
- exact candidate bytes: `11068384`;
- exact candidate SHA-256: `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- repeat clean rebuild at the same absolute target path: byte-identical PASS.

This is explicitly:

`PATH_BOUND_REPRODUCIBLE_CANDIDATE`

It is not:

`PATH_INDEPENDENT_REPRODUCIBLE_RELEASE`

A later release-engineering checkpoint may normalize build paths or provide signed distribution artifacts, but that work is not required to prove this one bounded host reconciliation candidate.

## 14. Candidate-build limitations

UF does not claim that the candidate hash is reproduced when any of the following materially changes:

- absolute Cargo target directory;
- source head/tree;
- Cargo.lock;
- Rust/Cargo toolchain;
- target triple;
- linker/compiler behavior;
- dependency graph;
- build profile;
- build flags or wrappers.

UF also does not claim bit reproducibility across a different operating-system image or future toolchain installation merely because version strings match.

The selected candidate hash is exact evidence for the observed fixed-path build law only.

## 15. Vendor unit disposition

The current installed vendor unit is already exact:

`24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`

and matches the exact UE repository source.

UF therefore selects:

`VENDOR_UNIT_VERIFICATION_ONLY / NO_UNIT_REWRITE`

A future package reconciliation must fail closed if the vendor unit ceases to be the exact expected root-owned regular `0644` file before privileged Agent replacement.

It must not rewrite the unit merely to refresh timestamps or ownership that already match.

If the unit is absent, foreign, symlinked, has wrong bytes/mode/owner, or cannot be proven exact, the Agent-only upgrade must STOP and return to selection rather than widening itself into pair repair.

## 16. Selected package reconciliation shape

For the observed host state UF selects:

`AGENT_ONLY_INACTIVE_ROOT_OWNED_UPGRADE / VENDOR_UNIT_VERIFY_ONLY`

The fixed old Agent identity is:

`4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`

The fixed new Agent identity is:

`9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`

The fixed vendor-unit identity is:

`24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`

No other old/new hash pair is authorized by UF.

## 17. No live upgrade

The future package mutation must prove immediately before privileged replacement that `prw-agent.service` is not active/running and has `MainPID=0`.

The current observed `failed/failed + MainPID=0` state satisfies only the read-only selection context.

If the service becomes active, activating, reloading, deactivating, or otherwise running before execution, the package transaction must STOP.

The package reconciliation transaction is not authorized to stop, kill, restart or otherwise quiesce a running Agent.

No live-upgrade semantics are selected.

## 18. Privilege boundary

Noninteractive sudo is not available and UF preserves that boundary.

UF prohibits:

- sudoers mutation;
- broad `NOPASSWD` grants;
- stored sudo passwords;
- runner-as-root conversion;
- desktop privileged shell pass-through;
- arbitrary root command execution;
- automatic privilege escalation from the Agent.

A real privileged package reconciliation remains an explicitly initiated authenticated interactive privilege step unless a later separately selected narrower platform mechanism replaces it.

## 19. Dedicated privileged reconciler selected

UF selects a dedicated Rust implementation for the privileged package-file transaction.

It must be a fixed-purpose PRW package reconciler, not a general shell runner, not a free-form `cp`/`install` wrapper, and not a systemd orchestration command.

The reconciler's only production mutation authority is the fixed Agent destination:

`/usr/lib/private-remote-workspace/prw-agent`

The vendor unit is read/verified only.

It must not accept an arbitrary destination path, arbitrary executable name, arbitrary command, arbitrary environment map, arbitrary ownership/mode, or arbitrary expected hashes.

The old/new/unit hashes above must be compile-time/source-bound constants in the UF-derived source materialization.

## 20. Source-materialization ceiling

The immediate source successor after evidence-closed UF should be one separately approved source checkpoint materializing the dedicated reconciler.

Expected source ceiling:

1. `Cargo.toml` only as needed to register the new workspace crate;
2. `Cargo.lock` only if Cargo dependency resolution actually changes it;
3. `crates/prw-agent-package-reconciliation/Cargo.toml`;
4. `crates/prw-agent-package-reconciliation/src/lib.rs`;
5. `crates/prw-agent-package-reconciliation/src/main.rs`.

No Agent runtime, systemd orchestration, configuration, identity, desktop, Android, network/control-plane, database/auth, packaging vendor-unit, workflow or unrelated path is expected merely to materialize the reconciler.

If implementation archaeology proves another path is necessary, the source checkpoint must STOP and return to selection before widening scope.

## 21. Reconciler dependencies and unsafe boundary

The implementation should reuse already locked Rust dependencies where sufficient, including descriptor-oriented filesystem operations and SHA-256 hashing.

The workspace `unsafe_code = "forbid"` law remains authoritative.

Current `rustix 1.1.4` exposes the primitives needed for a bounded Linux implementation, including `openat2`/resolve flags and `renameat_with`/`RenameFlags::EXCHANGE`.

UF does not authorize raw unsafe syscalls merely because the transaction is privileged.

## 22. Invocation surface

The reconciler must expose one semantic operation only:

`reconcile-current-agent`

The exact binary name should be frozen by the source-materialization checkpoint, with the intended default:

`prw-agent-package-reconcile`

It must not accept arbitrary subcommands.

It may accept only the minimum candidate-stage locator required by the source design. Any such locator must be treated solely as an input source locator; it must never control the production destination or expected hashes.

The candidate stage must be validated component-by-component and opened using descriptor-relative no-symlink custody before candidate bytes are trusted.

## 23. Candidate stage custody

Before privileged execution, a separately authorized non-privileged preparation checkpoint must stage the exact frozen candidate and a transaction manifest under intended-user state custody.

The stage must be:

- owned by the intended unprivileged user;
- mode `0700`;
- non-symlink directory components;
- exact fixed transaction-specific layout;
- not group/other writable.

The candidate file must be:

- regular;
- non-symlink;
- intended-user owned;
- not group/other writable;
- exact `11068384` bytes;
- exact SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

The privileged reconciler must open and hash the candidate through its validated descriptor custody rather than trusting a pathname hash followed by a second pathname read.

## 24. No candidate TOCTOU promotion claim from path-only checking

UF rejects this unsafe shape:

1. `sha256sum <candidate-path>`;
2. later `cp <candidate-path> <root-destination>`.

The selected reconciler must bind validation to the bytes actually copied, using an opened regular-file descriptor and exact metadata/byte validation around that descriptor.

A pathname may identify where to begin custody, but the promoted bytes must be sourced from the already validated/opened object.

## 25. Destination preflight

Before creating any privileged staging file, the reconciler must prove at least:

- effective UID is root;
- the fixed Agent parent is the expected root-owned non-symlink directory;
- the fixed Agent destination is a root-owned non-symlink regular file;
- Agent mode is exactly `0755`;
- Agent bytes hash exactly to the selected old hash `4dbb114e...`;
- fixed vendor unit is root-owned non-symlink regular file mode `0644`;
- vendor-unit bytes hash exactly to `24f646dc...`;
- service is not active/running;
- `MainPID=0` is independently established by the non-privileged execution preflight immediately before the interactive privileged handoff.

The privileged reconciler itself must not obtain non-file authority to mutate systemd state.

If any fixed package identity cannot be proven, fail before destination replacement.

## 26. Root-owned candidate staging

The reconciler must create a private temporary sibling of the fixed Agent destination on the same filesystem.

Required properties before promotion:

- fresh name created without following symlinks;
- exclusive creation;
- root-owned;
- final intended mode `0755` before promotion;
- bytes copied only from the already validated candidate descriptor;
- exact length readback;
- exact new SHA-256 readback;
- file synchronization before promotion.

The reconciler must never execute the candidate binary during package reconciliation.

## 27. Atomic exchange selected

UF selects same-directory atomic exchange as the preferred upgrade commit primitive when the source-materialization tests prove target-kernel support:

`renameat2(..., RENAME_EXCHANGE)` through safe `rustix` APIs.

Immediately before exchange:

- fixed destination must still prove the exact old PRW-managed identity;
- root-owned candidate sibling must still prove the exact new identity.

The exchange atomically places the new file at the fixed Agent destination while preserving the exact old destination bytes at the private sibling name.

That old sibling becomes the transaction rollback object.

This avoids a window in which the destination is absent and retains exact old bytes for bounded exchange-back rollback.

## 28. Fallback if atomic exchange is unavailable

UF does not silently authorize a weaker replacement primitive merely because `RENAME_EXCHANGE` is unavailable.

If the target kernel/filesystem rejects the selected exchange primitive or its semantics cannot be proved in disposable validation, the source materialization/execution checkpoint must STOP and return to selection.

No shell `mv -f` fallback is selected by UF.

## 29. Post-exchange verification

After the exchange, before success classification, the reconciler must prove:

- fixed Agent destination remains regular, non-symlink and root-owned;
- mode exactly `0755`;
- exact bytes/length of the selected candidate;
- SHA-256 exactly `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- rollback sibling remains regular, root-owned and contains exact old bytes/hash `4dbb114e...`;
- vendor unit remains byte/metadata identical to its preflight identity;
- relevant directory synchronization completes.

Only after these checks pass may the old rollback sibling be removed and the parent directory synchronized again.

## 30. Bounded rollback law

If a failure occurs before exchange, no production Agent replacement has occurred and the candidate sibling may be removed when exact transaction ownership is provable.

If a failure occurs after exchange but before successful finalization, the reconciler may attempt exactly one exchange-back only when it can prove:

- destination is still exactly the selected new candidate;
- rollback sibling is still exactly the selected old Agent;
- both are the exact transaction-owned objects expected by this invocation.

After exchange-back, it must independently prove the destination has exact old bytes/mode/owner before claiming rollback success.

If exchange-back cannot be safely attempted, fails, or yields ambiguous state, the reconciler must preserve available transaction evidence and return bounded terminal failure.

No second replacement/retry loop is selected.

## 31. Independent post-privilege reconciliation required

Regardless of the reconciler's own exit/result classification, a separately authorized unprivileged read-only reconciliation must follow the interactive privileged operation before any later bootstrap stage.

That reconciliation must independently read:

- installed Agent bytes/hash/mode/owner;
- vendor-unit bytes/hash/mode/owner;
- user-service loaded/active/sub/MainPID state;
- 20/30/40 state remains unchanged;
- no unexpected package sibling/staging residue when success was claimed.

Deployment success is accepted only from the reconciled final state, not from helper stdout alone.

## 32. Success classification

A successful real package reconciliation may prove only:

`CURRENT_AGENT_PAYLOAD_RECONCILED / EXACT_ROOT_OWNED_BYTES / VENDOR_UNIT_UNCHANGED / SERVICE_NOT_ACTIVATED`

It does not prove:

- daemon reload;
- first start;
- local IPC Ready;
- device identity;
- canonical 20/30/40;
- configured-remote readiness;
- network/control-plane readiness;
- authentication/database readiness.

## 33. No service-manager calls in the reconciler

The privileged reconciler must contain no production execution path that invokes:

- `systemctl`;
- `loginctl`;
- `service`;
- shell command interpreters;
- arbitrary subprocesses supplied by request data.

Service-state eligibility is established read-only outside the privileged mutation and re-read after the transaction.

The privileged helper owns package bytes only.

## 34. No identity/configuration ownership

The reconciler must never read, create, delete or modify:

- encrypted device identity credential;
- `20-device-identity-credential.conf`;
- `30-agent-execution-mode.conf`;
- `40-configured-remote-inputs.conf`;
- any external systemd fragment;
- Agent XDG state/config data.

Those remain independently gated owners/checkpoints.

## 35. No enablement or linger ownership

The current unit is observed enabled and the host historically has linger enabled.

UF does not make either state part of package reconciliation ownership.

The privileged reconciler may not create/remove `default.target.wants` links, enable/disable the unit, or mutate linger.

If future product behavior requires such change, it belongs to a separate selection checkpoint.

## 36. Candidate staging is not deployment

Building or staging the candidate under `/tmp` or intended-user state does not install it.

The UF provenance builds performed only disposable/user-owned filesystem writes outside the fixed package destinations.

No candidate produced during UF was copied into `/usr`.

No historical staged helper was executed.

## 37. Canonical selection summary

UF selects:

`EXACT_UE_SOURCE / LOCKED_CARGO_GRAPH / RELEASE_PROFILE / PATH_BOUND_CANONICAL_TARGET_REPRODUCIBILITY / EXACT_AGENT_SHA_9DB768C1 / EXACT_VENDOR_UNIT_VERIFY_ONLY / KNOWN_PRIOR_PRW_AGENT_SHA_4DBB114E / INACTIVE_NOT_RUNNING_ONLY / DEDICATED_RUST_PRIVILEGED_RECONCILER / FIXED_AGENT_DESTINATION_ONLY / COMPILE_TIME_OLD_NEW_UNIT_HASHES / DESCRIPTOR_BOUND_CANDIDATE_CUSTODY / ROOT_SAME_FILESYSTEM_STAGING / RENAME_EXCHANGE_COMMIT / ONE_EXCHANGE_BACK_ROLLBACK / INDEPENDENT_POST_PRIVILEGE_READBACK / INTERACTIVE_PRIVILEGE_BOUNDARY / NO_UNIT_REWRITE / NO_SYSTEMD_MANAGER_CALL / NO_IDENTITY_OR_20_30_40_MUTATION / NO_ENABLEMENT_OR_LINGER_MUTATION / NO_AUTOMATIC_RETRY / NO_PATH_INDEPENDENT_REPRODUCIBILITY_CLAIM / NO_RACE_FREE_CLAIM`

## 38. Why no path-independent reproducibility claim

The exact current dependency graph embeds an absolute Cargo target-generated source path into the Agent binary.

UF has proven repeatability at one fixed canonical target path after complete clean rebuilds.

UF has also proven non-equality across different target paths.

Therefore stating merely `reproducible build` without the path qualifier would be false for this checkpoint.

All future evidence that cites the selected candidate hash must carry the exact canonical build-path/build-command/source/toolchain qualifiers.

## 39. Source successor validation requirements

The dedicated reconciler source-materialization checkpoint should require at least:

- exact predecessor UF head/tree;
- exact allowed-path delta only;
- compile with workspace unsafe-forbid policy;
- unit tests for fixed hashes/destinations and rejection of arbitrary widening;
- disposable same-filesystem tests for successful exchange;
- injected pre-exchange failure proving no destination mutation;
- injected post-exchange failure proving one bounded exchange-back;
- test that rollback success is claimed only after exact old destination readback;
- symlink/nonregular/ownership/mode/hash drift rejection tests;
- vendor-unit drift rejection;
- no manager/process calls;
- no identity/20/30/40 mutation;
- exact-head full repository CI;
- immutable evidence closure.

No real root-owned host package mutation is part of the source-materialization checkpoint.

## 40. Real deployment preflight requirements

After the reconciler source is evidence-closed, a separately approved read-only deployment preflight should freeze:

- exact reconciler source/build identity;
- exact candidate Agent staged bytes/hash;
- exact old installed Agent readback;
- exact vendor-unit readback;
- package parent metadata;
- intended-user stage custody;
- service not-active/not-running proof;
- `MainPID=0`;
- noninteractive privilege remains unavailable;
- exact interactive invocation that will be handed to the user;
- exact expected success/failure classifications.

The preflight itself remains non-mutating.

## 41. Real deployment mutation remains separately gated

UF does not authorize the future interactive privileged execution merely by selecting its mechanism.

A later explicit user authorization is required after source materialization and read-only preflight evidence.

That authorization may cover exactly one frozen old→new Agent reconciliation transaction and nothing else.

No deployment retry is implied by a failed/ambiguous first invocation.

## 42. Post-deployment successor order

Only after the package reconciliation is independently evidence-closed may UE's staged bootstrap sequence continue.

The next conceptual dependency remains identity integration source materialization for canonical `20-device-identity-credential.conf`, followed by separately gated real first identity provisioning/integration, local-only 30/40 materialization, and later first-start orchestration.

No later step is authorized by UF closure.

## 43. Evidence requirements for UF itself

UF is docs-only.

Its own closure should require:

- exact one-contract-path delta from evidence-closed UE;
- exact predecessor/head/tree/parent proof;
- exact contract bytes/blob/SHA-256;
- `git diff --check`;
- exact-head repository CI registered for the docs-only head;
- `SKIPPED` checks represented as skipped, never PASS;
- immutable raw Markdown publication in the canonical Drive evidence folder;
- byte-identical raw readback;
- exact-title singleton proof;
- one-revision lineage with previous revision `null`;
- metadata-only PR closure binding;
- PR retained open, draft and unmerged.

## 44. Explicit non-actions / STOP

C03e-UF performs no:

- Agent/runtime source modification;
- privileged reconciler source materialization;
- root-owned package-file replacement;
- historical A05 helper execution;
- `sudo` privileged transaction;
- vendor-unit rewrite;
- identity generation or encrypted credential persistence;
- 20/30/40 mutation;
- systemd target verify as part of a mutating transaction;
- daemon reload;
- service start, stop, restart or try-restart;
- enable/disable mutation;
- linger mutation;
- configured-remote activation;
- network/listener mutation;
- database/auth/control-plane mutation;
- external-fragment mutation;
- main-branch mutation;
- repository configuration mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset, rebase, squash, force-push or history rewrite;
- destructive canonical evidence cleanup.

STOP after evidence closure of this selection.

The immediate source-materialization successor remains separately gated.