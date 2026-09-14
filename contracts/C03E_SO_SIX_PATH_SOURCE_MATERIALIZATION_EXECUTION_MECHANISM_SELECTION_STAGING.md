# C03e-SO — six-path source-materialization execution-mechanism selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`FALLIBLE_ADMISSION_TIMING_SIX_PATH_SOURCE_MATERIALIZATION_EXECUTION_MECHANISM_SELECTION`

Selection result:
`ONE_TRANSIENT_SELF_REMOVING_WORKFLOW_SELECTED / INLINE_GUARDED_SOURCE_TRANSFORM_ONLY / FINAL_NET_TOPOLOGY_REMAINS_EXACTLY_SIX_RUST_PATHS / NO_EXTERNAL_SCRIPT_PATH / NO_RUNTIME_ACTIVATION / SOURCE_MATERIALIZATION_SEPARATELY_GATED`

This checkpoint is documentation-only. It resolves only the execution-control gap encountered after C03e-SN selected a six-path source implementation but did not select a repository-side materialization mechanism. No Rust/source/runtime behavior is materialized by C03e-SO.

## 1. Exact predecessor authority

Authoritative predecessor: evidence-closed C03e-SN PR #631.

- repository: `Gersi365/prw-executor-private`, stable ID `1334911207`;
- predecessor branch: `phase-152-c03e-sn-fallible-admission-timing-source-layout-and-propagation-selection`;
- predecessor head: `c12df54c9be2538449efe438ac3cdb65fa7bbd63`;
- predecessor tree: `681a4b870dadbcf163581e15541f03abbd96635e`;
- predecessor contract: `contracts/C03E_SN_FALLIBLE_ADMISSION_TIMING_SOURCE_LAYOUT_AND_PROPAGATION_SELECTION_STAGING.md`;
- predecessor contract blob: `c081a019cc06012464e5f1cc584dd1814fb6b586`;
- predecessor PR state: draft/open/unmerged;
- predecessor closure: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

Canonical C03e-SN immutable evidence:

- filename: `C03E_SN_FALLIBLE_ADMISSION_TIMING_SOURCE_LAYOUT_AND_PROPAGATION_SELECTION_AUDIT_2026-09-14.md`;
- Drive ID: `1Tvai42NE6hF4MwPdm49sTNq3A_OhN3z_`;
- canonical parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- MIME: `text/markdown`;
- frozen/raw-readback bytes: `13052`;
- SHA-256: `c18d6f26e267ca3715c1314c1a639505aa1652d42b837c3aa011e5fc0101b567`;
- final LF: true;
- current revision: `0Bz5eMiLa5v9xRTE1cTl6ait6T0hMRS9XVThhWHYwbW45Y2wwPQ`;
- previous revision: null.

C03e-SN selected the complete source semantics and source layout. C03e-SO does not alter, reinterpret, broaden or replace any SN type, callback, custody, control-flow, visibility, compatibility, path or test requirement.

## 2. Why a separate execution-mechanism selection is required

C03e-SN pins five existing Rust files plus one new Rust file. The connected repository mutation surface available for this continuation can create whole blobs/files but does not provide a repository patch primitive over existing files. Blind whole-file reconstruction is not an acceptable evidence-preserving substitute for exact source edits.

Historical Phase 152 materialization checkpoints establish a forward-only precedent for a branch-local transient guarded workflow that materializes selected source and removes itself before the final source head. Current change control requires that such workflow mutation be explicitly selected rather than inferred from the six-path source ceiling.

Therefore C03e-SO selects one narrowly bounded transient workflow as an execution mechanism only. The workflow is not part of the final source topology, product runtime, build configuration or deployed artifact.

## 3. Current C03e-SO mutation ceiling

C03e-SO itself may add exactly one documentation path:

`contracts/C03E_SO_SIX_PATH_SOURCE_MATERIALIZATION_EXECUTION_MECHANISM_SELECTION_STAGING.md`

No Rust/source/runtime/workflow/manifest/lockfile/Android/package/service/repository-configuration path may change in SO.

Expected SN -> SO topology:

- one direct commit;
- ahead 1, behind 0;
- merge base exact SN head;
- exactly one added documentation path;
- zero Rust/source/runtime/workflow/configuration mutation.

## 4. Frozen six-path source boundary inherited from SN

The separately gated source successor must still finish with exactly these six Rust paths and no seventh final path:

| ID | Exact source path | Exact SN authority | Selected role inherited unchanged |
| --- | --- | --- | --- |
| P0 | `crates/prw-agent/src/remote_session_capability_runtime.rs` | `2b6a0fd693f2ddacec608018a4db811cc36ef4c0` | private child declaration and crate-only re-exports |
| P1 | `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs` | absent at SN | bounded timing-source error and intact-request failure carrier with inline tests |
| P2 | `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs` | `d50e423d01b4161e244c075d59a85fd83533cf62` | pure fallible preparation, shared result-capable driver internals, compatibility lift, fallible executor siblings and both-path tests |
| P3 | `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs` | `881846753f51bdf94cff32ff0f9649dbcf50a80f` | three forwarding siblings preserving receipt projection and endpoint custody |
| P4 | `crates/prw-agent/src/linux_bootstrap.rs` | `b5d236d3c0264ea8870e90af96c76294b50d0fc1` | one runtime-input-aware Linux forwarding sibling with separate failure custodian |
| P5 | `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs` | `cccefed1192243a16b67c971ca6ba8fe13ae2c82` | one dormant higher-owner sibling preserving channel population and separate custodian passing |

The protected request-definition path remains unchanged:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

with exact SN blob:
`1539b6b9a08bf18883d7a16022f15f7c240eaf08`.

No other source/configuration path is selected.

## 5. Selected transient materializer path

The later source-materialization checkpoint may introduce exactly one transient workflow path in branch history:

`.github/workflows/phase-152-c03e-sp-fallible-admission-timing-six-path-source-materializer.yml`

This path is selected only as a temporary execution mechanism. It must be absent from the final C03e-SP tree and absent from the final predecessor-to-SP net diff.

No external helper script, generated patch file, temporary contract, fixture, manifest, lockfile, build script or second workflow path is selected.

The workflow must be self-contained. Any source-transform program must be embedded inline in this one workflow file and must operate only on P0-P5.

## 6. Selected forward-only lineage

The later C03e-SP source checkpoint may use exactly this execution shape:

1. create the SP branch from the exact evidence-closed SO head after a fresh namespace audit;
2. add only the selected transient workflow file;
3. let that workflow run on the helper-addition push;
4. prove exact predecessor/source guards before editing;
5. materialize only P0-P5;
6. run the selected local source guards and formatting/check steps;
7. remove the transient workflow in the same forward-only source-finalization push or commit sequence;
8. leave a final SP head whose net SO -> SP diff contains only P0-P5;
9. open/retain the SP PR draft/open/unmerged and use only exact-final-head PR CI as closure validation authority.

No reset, rebase, squash, force update, branch rewind or history rewrite is selected. A failed materializer attempt must remain transparent audit history and may be corrected only forward if the correction remains inside this execution-mechanism boundary.

## 7. Mandatory materializer ancestry and identity guards

Before source mutation, the transient workflow must fail closed unless all of the following hold:

- it is running on the exact selected SP branch name;
- the helper-addition commit has the exact evidence-closed SO head as its direct parent;
- the inherited SN source blobs for P0, P2, P3, P4 and P5 match the hashes listed above;
- P1 is absent;
- the protected request-definition path matches `1539b6b9a08bf18883d7a16022f15f7c240eaf08`;
- the exact SN contract remains present with blob `c081a019cc06012464e5f1cc584dd1814fb6b586`;
- the exact SO contract is present at the verified SO blob selected by the SP preflight;
- the working tree is clean before transformation;
- the selected target symbols are absent before insertion where SN requires new symbols;
- no successor or conflicting SP source branch/PR exists outside the branch being materialized.

Any mismatch requires STOP with no source commit.

## 8. Deterministic transform requirements

The inline transform must use exact lexical/structural anchors whose expected occurrence counts are asserted before replacement. It must fail closed on missing or duplicate anchors rather than guessing placement.

The transform may:

- add `mod admission_timing_failure;` and the two crate-private re-exports only in P0;
- create P1 with exactly the SN-selected two types and inline tests;
- add the SN-selected pure helper, shared result-capable internals, Infallible compatibility lift, S5/S6 siblings and inline behavioral tests only in P2;
- add S2/S3/S4 only in P3;
- add S1 only in P4;
- add S0 only in P5.

The transform must not:

- rename or replace historical infallible entry points;
- widen public API;
- change the protected request-definition path;
- add a seventh source/configuration path;
- create a concrete timing provider;
- create a concrete failure sink/custodian implementation;
- create executable caller wiring;
- activate runtime/network/listener/readiness behavior;
- mutate Cargo manifests, lockfiles, dependencies, build scripts or persistent workflows.

## 9. Inherited semantic law that the materializer must preserve

The source transform must implement SN exactly:

- `RemoteSessionAdmissionTimingSourceError<Cause>` remains crate-visible, non-exhaustive, with exactly `Acquisition(Cause)`, `Policy(Cause)`, `InvalidChallengeValidity`, `InvalidApplicationLease`, and `ArithmeticOverflow`;
- its Debug output exposes only bounded variant labels and never renders Cause/request data;
- Error::source returns the retained cause only for Acquisition and Policy;
- `RemoteSessionAdmissionTimingFailure<D, T, TimingError>` remains non-cloneable with private `error` and exact intact request fields and only `new`, `error`, `request`, and `into_parts` operations;
- production timing remains `F: FnMut(&DeviceId) -> Result<RemoteSessionRealAdmissionTiming, RemoteSessionAdmissionTimingSourceError<Cause>>`;
- separate failure custody remains `K: FnMut(RemoteSessionAdmissionTimingFailure<D, T, RemoteSessionAdmissionTimingSourceError<Cause>>)`, with Send + 'static only at the selected production ownership boundaries;
- duplicate preflight occurs before timing acquisition and performs zero F/K calls;
- vacant requests sample F exactly once before request decomposition;
- timing failure keeps the exact request intact, creates no AJ/network/session/worker effect, transfers one carrier to K exactly once, and does not replay/requeue the failed request;
- both idle and pending-producer request arms use the same fallible preparation law;
- an independently pending producer future retains its custody and eventual receipt across another request's timing failure;
- existing infallible APIs delegate through the selected explicit `Infallible` success lift without acquiring a new failure requirement;
- the seven selected forwarding siblings preserve original ordered inputs/outputs and append K separately;
- configured population does not invoke F or K;
- endpoint close/wait_idle and receipt projection remain unchanged.

## 10. Selected local materializer validation

Before the workflow is permitted to push the source-finalization commit, it must at minimum perform:

- `rustfmt`/workspace formatting over the materialized Rust source;
- formatting check;
- `git diff --check`;
- a compile/check of `prw-agent` sufficient to catch source/type errors before push;
- exact target-symbol occurrence guards;
- forbidden-pattern guards for panic/default/fallback/replay/requeue or unauthorized provider/executable wiring where mechanically checkable;
- an exact base-to-worktree path check proving the net candidate consists only of P0-P5 after the helper is removed;
- a final assertion that the transient workflow path is absent from the candidate tree.

These materializer checks are pre-push safeguards only. They are not final checkpoint validation authority.

## 11. Exact-final-head validation authority for the later source checkpoint

Only CI attached to the exact final SP head may support closure.

Required interpretation:

- PRW Rust Validation must complete successfully, including locked dependency graph, formatting, Clippy, tests and workspace build;
- Android Validation is claimed only if it actually runs and succeeds for the exact final head;
- path-filtered `SKIPPED` workflows remain `SKIPPED`, not PASS;
- validation from SO, SN, a helper-only commit, a superseded candidate or a failed materializer attempt is not inherited as SP final-head PASS.

If final-head Rust CI fails, correct forward-only within the selected six source paths when possible. If correction requires any additional persistent source/configuration path or semantic widening, STOP and return to selection.

## 12. Final topology requirement for the later source checkpoint

The final SO -> SP comparison must prove:

- merge base exact evidence-closed SO head;
- forward-only ancestry;
- exactly six final changed paths;
- all six are P0-P5 above;
- P1 is added;
- P0/P2/P3/P4/P5 are modified;
- the transient workflow is absent from the final tree and absent from the final net diff;
- no contract, manifest, dependency, lockfile, build script, persistent workflow, Android source, package, service or repository configuration path changed;
- no executable/runtime activation path changed.

Historical helper commits may appear in ancestry only as explicitly selected transient execution history. They do not widen the final source boundary.

## 13. Evidence requirements for the later source checkpoint

After exact-final-head validation and before source-checkpoint closure:

- freeze one immutable Markdown audit;
- pre-search the canonical Drive parent for exact-title collision and require zero matches;
- upload exactly once;
- verify exact filename, canonical parent and MIME;
- raw-read back the complete bytes;
- verify exact byte count, SHA-256 and final LF;
- verify exact-title canonical uniqueness after upload;
- verify revision lineage and require no evidence rewrite;
- bind the verified evidence metadata in the final SP PR body while keeping that PR draft/open/unmerged.

## 14. Explicit C03e-SO non-actions

C03e-SO does not itself perform or authorize inside this checkpoint:

- Rust/source/runtime materialization;
- creation of the transient materializer workflow during SO;
- concrete timing-provider materialization;
- concrete timing-failure custodian/sink implementation;
- executable caller or `run()`/`main.rs` wiring;
- listener/readiness/network/runtime activation;
- Cargo manifest, lockfile, dependency, build-script, Android-source, package, service, systemd or repository-configuration mutation;
- database/schema/control-plane mutation;
- authentication cutover;
- merge;
- deployment;
- restart/recovery activation;
- ready-for-review conversion;
- PR closure;
- branch deletion;
- reset, rebase, squash, force update or history rewrite;
- destructive evidence cleanup.

C03e-SO selects only the bounded execution mechanism required for the separately gated C03e-SP source materialization. The product/source semantics remain exactly those closed by C03e-SN.

**STOP after C03e-SO evidence closure. Do not create C03e-SP or any transient workflow inside the SO checkpoint.**
