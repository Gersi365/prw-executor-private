# C03e-SQ — transient materializer workflow-permission correction selection

Status: `CORRECTIVE SELECTION — VALIDATION PENDING`

Boundary:
`FALLIBLE_ADMISSION_TIMING_TRANSIENT_MATERIALIZER_WORKFLOW_PERMISSION_CORRECTION_SELECTION`

Selection result:
`ACTIONS_SELF_REMOVAL_UNAVAILABLE / SOURCE_ONLY_WORKFLOW_PUSH_SELECTED / CONNECTOR_SHA_GUARDED_HELPER_REMOVAL_SELECTED / FINAL_NET_TOPOLOGY_REMAINS_EXACTLY_SIX_RUST_PATHS / FAILED_SP_ATTEMPT_RETAINED_NONCANONICAL / NO_PERMISSION_OR_REPOSITORY_CONFIG_MUTATION`

C03e-SQ is documentation-only. It corrects only the execution mechanism selected by C03e-SO after live execution proved that the repository GitHub Actions token can write ordinary repository content but cannot create, update, or delete workflow files. It does not alter any C03e-SN source semantic, source path, callback, custody, compatibility or test requirement.

## 1. Authoritative predecessor

The authoritative predecessor remains evidence-closed C03e-SO PR #632.

- repository: `Gersi365/prw-executor-private`, stable ID `1334911207`;
- predecessor branch: `phase-152-c03e-so-six-path-source-materialization-execution-mechanism-selection`;
- predecessor head: `ece909105886ffbe31c04e4aea6a0c92a0d4374c`;
- predecessor tree: `4a94a5a7f264f22af64a09ac80897bb2877203b4`;
- predecessor contract: `contracts/C03E_SO_SIX_PATH_SOURCE_MATERIALIZATION_EXECUTION_MECHANISM_SELECTION_STAGING.md`;
- predecessor contract blob: `a5ab2b5839c9b7d88ea99d1317eaf19193393cfc`;
- predecessor status: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- predecessor PR remains draft/open/unmerged.

Canonical C03e-SO immutable evidence remains:

- filename: `C03E_SO_SIX_PATH_SOURCE_MATERIALIZATION_EXECUTION_MECHANISM_SELECTION_AUDIT_2026-09-14.md`;
- Drive ID: `1AOlG_YUGbElCC2cf3xoyVcFXFgz1TuJw`;
- canonical parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- MIME: `text/markdown`;
- frozen/raw-readback bytes: `15842`;
- SHA-256: `9135412c73bcdd2b21ca46c322896e70314ec7adc35e2b5ca86426cb21126fca`;
- revision: `0Bz5eMiLa5v9xVjR1aW9zY2JXZ0dGZGMySW5kWFJPNGE2cTc0PQ`;
- previous revision: null.

## 2. Live correction finding

A C03e-SP execution attempt was created from exact C03e-SO to exercise the SO-selected self-removing workflow mechanism.

Attempt branch:
`phase-152-c03e-sp-fallible-admission-timing-six-path-source-materialization`

Remote attempt head after the observed failures:
`db29ba67af5480287c273276103b8c1404b34557`

Exact SO -> failed-attempt topology at correction selection:

- ahead `3`, behind `0`;
- merge base exact C03e-SO head;
- three forward-only helper/correction commits;
- final net diff contains only `.github/workflows/phase-152-c03e-sp-fallible-admission-timing-six-path-source-materializer.yml`;
- zero P0-P5 Rust/source mutation;
- zero manifest/lockfile/dependency/build-script/runtime/executable mutation.

The attempt is retained as noncanonical audit history. It is not a source-materialization checkpoint and is not a predecessor for the corrected source successor.

### Attempt run 1

Initial helper commit:
`21cf148b65c3495bb9d972f3765e03222bf03bbc`

Materializer run:
`34854935213`, job `104011919429`.

The exact ancestry/blob/path guards completed before transform execution. The run then failed closed during source-transform anchor validation because `impl RemoteSessionExecutorRuntime {` occurred five times instead of the helper's assumed single occurrence. No source commit or source push occurred.

### Attempt run 2

Correction-staging commit:
`3b00918eb9f3fc19878210248c890839ed3ffc9f`

Correction run:
`34855269847`, job `104013085876`.

The run failed inside the helper-correction script because nested Python quoting was invalid. No source commit or source push occurred.

### Attempt run 3

Second correction-script commit:
`db29ba67af5480287c273276103b8c1404b34557`

Correction run:
`34855346242`, job `104013356740`.

The correction logic successfully produced a local forward-only helper correction commit in the ephemeral runner, but GitHub rejected the push with the exact authorization failure:

`refusing to allow a GitHub App to create or update workflow ... without workflows permission`

The workflow token reported `Contents: write` and no workflow-file write permission. The local ephemeral commit never reached the repository. The remote branch therefore remained `db29ba67af5480287c273276103b8c1404b34557` and still contains no Rust/source mutation.

This is an execution-permission limitation, not a source-semantic failure.

## 3. Why C03e-SO requires correction

C03e-SO selected one transient **self-removing** workflow. Live execution proves that the Actions credential cannot satisfy the self-removal step because GitHub treats `.github/workflows/*` changes as requiring workflow-specific permission beyond ordinary contents write.

Changing Actions permissions, repository configuration, installation permissions, secrets, tokens or rulesets merely to satisfy that assumption is not selected and is not permitted by this checkpoint.

Silently replacing self-removal with an external deletion would also change the selected mechanism. C03e-SQ therefore records that change explicitly before any further source materialization.

## 4. Corrected execution mechanism

The corrected later source checkpoint may use exactly one transient helper path:

`.github/workflows/phase-152-c03e-sr-fallible-admission-timing-six-path-source-materializer.yml`

The corrected mechanism has two explicit actors with disjoint mutation roles:

1. **GitHub Actions helper**
   - runs only from the exact selected SR branch;
   - validates exact corrected-selection ancestry and inherited SN source identities;
   - performs the deterministic transform only on P0-P5;
   - runs formatting, `git diff --check`, source/type checks and target-symbol guards;
   - commits and pushes **only P0-P5 source changes**;
   - must not create, update or delete its own workflow file after the helper-addition commit;
   - must not mutate any other path.
2. **Connected GitHub contents API**
   - after successful helper source push and fresh readback, fetches the exact current helper blob SHA;
   - verifies the branch/source candidate and helper identity again;
   - deletes only the exact helper path with the connector `delete_file` primitive and that current blob SHA;
   - creates one normal forward-only helper-removal commit;
   - does not amend, reset, rebase, squash or force-update prior history.

The connector-mediated deletion is part of the selected execution mechanism, not a source change. It is selected because its SHA-guarded file deletion primitive is available and does not require repository permission/configuration mutation.

## 5. Frozen source boundary inherited unchanged from C03e-SN

The corrected source successor still has exactly six final Rust paths:

1. `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - SN blob `2b6a0fd693f2ddacec608018a4db811cc36ef4c0`.
2. `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs`
   - absent at SN/SO/SQ predecessor.
3. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`
   - SN blob `d50e423d01b4161e244c075d59a85fd83533cf62`.
4. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`
   - SN blob `881846753f51bdf94cff32ff0f9649dbcf50a80f`.
5. `crates/prw-agent/src/linux_bootstrap.rs`
   - SN blob `b5d236d3c0264ea8870e90af96c76294b50d0fc1`.
6. `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
   - SN blob `cccefed1192243a16b67c971ca6ba8fe13ae2c82`.

Protected request-definition path remains unchanged:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
with exact blob `1539b6b9a08bf18883d7a16022f15f7c240eaf08`.

No seventh final source/configuration path is selected.

## 6. Source semantics remain C03e-SN authority

C03e-SQ changes no product semantics. The later source transform remains bound to C03e-SN exactly, including:

- bounded crate-visible `RemoteSessionAdmissionTimingSourceError<Cause>` with the five selected variants;
- bounded Debug and Error source behavior;
- non-cloneable intact-request `RemoteSessionAdmissionTimingFailure<D,T,TimingError>`;
- fallible timing callback returning `Result<RemoteSessionRealAdmissionTiming, RemoteSessionAdmissionTimingSourceError<Cause>>`;
- separate caller-owned failure custodian receiving the complete carrier exactly once;
- duplicate-active-device preflight before timing acquisition with zero timing/custodian calls;
- exactly one timing sample for a vacant request before request decomposition;
- no AJ/network/session/worker effect on timing-source failure;
- no forged duplicate/AJ/completion/receipt result;
- no replay/requeue after custody transfer;
- identical fallible preparation law in idle and pending-producer request paths;
- independent pending producer custody and eventual receipt preservation;
- explicit `Infallible` compatibility lift for historical infallible APIs;
- seven exact `_with_fallible_admission_timing` forwarding siblings;
- configured population invokes neither timing source nor failure custodian;
- existing endpoint receipt projection and close/wait-idle law unchanged.

Concrete timing provider, concrete failure sink/custodian implementation and executable/runtime activation remain unselected.

## 7. Corrected source-successor lineage

After C03e-SQ evidence closure, the next canonical source checkpoint is C03e-SR, not the failed/noncanonical SP attempt.

Required C03e-SR execution shape:

1. fresh-audit exact evidence-closed SQ head and SR namespace;
2. create SR from exact SQ head;
3. add only the selected transient SR workflow helper;
4. run helper with exact ancestry/source/anchor guards;
5. helper transforms only P0-P5, validates locally and pushes a source-only commit while leaving the helper file unchanged;
6. re-read the pushed source candidate and require all six selected Rust paths plus the one helper path as the only SQ-to-candidate paths;
7. verify the helper blob SHA from the candidate head;
8. use connected GitHub `delete_file` with that exact SHA to remove only the helper in a separate forward-only commit;
9. verify final SQ -> SR net diff is exactly P0-P5 and helper absent;
10. open/retain SR PR draft/open/unmerged and use only exact-final-head CI for closure authority.

No history rewrite is selected. Failed helper or source-validation attempts remain transparent forward-only history.

## 8. Mandatory helper and connector deletion guards

Before source mutation, helper must fail closed unless:

- exact selected SR branch identity matches;
- helper-addition commit direct parent is exact evidence-closed SQ head;
- P0/P2/P3/P4/P5 match their pinned SN blobs;
- P1 is absent;
- protected request-definition path matches its pinned blob;
- SN, SO and SQ contract identities match their exact closed blobs;
- worktree is clean;
- target symbols are absent where required;
- no conflicting SR branch/PR exists.

Before connector helper deletion, refresh and require:

- branch head equals the exact successful helper-pushed source candidate;
- helper path exists and its blob SHA equals the freshly fetched expected SHA;
- SQ -> candidate diff contains exactly seven paths: P0-P5 plus the one transient helper;
- P1 is added and P0/P2/P3/P4/P5 modified;
- no contract, manifest, dependency, lockfile, build script, second workflow, Android source, package, service, repository configuration or executable/runtime path changed;
- protected request-definition path remains byte-identical.

Any mismatch requires STOP before deletion or further mutation.

## 9. Final topology requirement

Final exact SQ -> SR comparison must prove:

- merge base exact evidence-closed SQ head;
- forward-only ancestry;
- exactly six final changed paths;
- all six are P0-P5;
- P1 added; P0/P2/P3/P4/P5 modified;
- transient helper absent from final tree and final net diff;
- no documentation contract changes inside SR;
- no manifest/dependency/lockfile/build-script/persistent-workflow/Android/package/service/repository-config mutation;
- no executable/runtime activation path mutation.

The connector helper-removal commit may remain in history. Its final net contribution is only removal of the selected transient helper.

## 10. Validation authority

The helper's local formatting/type/source checks are safeguards only.

Only CI registered for exact final SR head after helper removal may support closure:

- PRW Rust Validation must complete successfully through locked dependency graph, formatting, Clippy, tests and workspace build;
- Android is PASS only if an exact-final-head Android workflow actually runs and succeeds;
- `SKIPPED` remains not PASS;
- no validation result from SO, SQ, failed SP, helper-addition head, source candidate before helper removal or a superseded corrective head is inherited.

If a correction can remain within P0-P5 it may proceed forward-only. If correctness requires any additional persistent source/configuration path or semantic widening, STOP and return to selection.

## 11. C03e-SQ current mutation ceiling

C03e-SQ itself may add exactly one documentation path:

`contracts/C03E_SQ_TRANSIENT_MATERIALIZER_WORKFLOW_PERMISSION_CORRECTION_SELECTION_STAGING.md`

Expected SO -> SQ topology:

- one direct commit;
- ahead 1, behind 0;
- merge base exact SO head;
- exactly one added documentation path;
- zero Rust/source/runtime/workflow/configuration mutation.

The failed SP branch is not modified, deleted, reset, rebased, force-updated or reused as the SQ base.

## 12. Explicit non-actions / STOP

C03e-SQ does not:

- materialize Rust/source/runtime behavior;
- create the corrected SR helper;
- alter the failed SP branch;
- grant or change GitHub Actions/workflow permissions;
- mutate repository configuration, rulesets, secrets, tokens or installation permissions;
- implement a concrete timing provider or failure sink;
- wire an executable caller or activate runtime/network/listener/readiness behavior;
- change Cargo manifests, dependencies, lockfiles, build scripts, Android source, packages, services or systemd;
- merge, deploy, restart, mark ready, close PRs, delete branches, rewrite history or destructively clean evidence.

**STOP after C03e-SQ evidence closure. Do not create C03e-SR or the corrected transient helper inside SQ.**
