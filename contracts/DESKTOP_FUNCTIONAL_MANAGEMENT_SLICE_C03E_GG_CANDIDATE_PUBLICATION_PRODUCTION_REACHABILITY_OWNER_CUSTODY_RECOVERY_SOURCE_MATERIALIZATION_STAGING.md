# Phase 152 C03e-GG — Candidate Publication Production Reachability Owner Custody/Recovery Source Materialization

Status: VALIDATING

Target gate:
`C03E_GG_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-GF is the authoritative predecessor:

- branch: `phase-152-c03e-gf-candidate-publication-production-reachability-owner-custody-recovery-semantics-selection-staging`;
- head: `51c4fa40e8f384bb7bddfbe4a300feb72e788244`;
- tree: `4830990ba08c71544c6a3c12a5df12e165cd3895`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SEMANTICS_SELECTION`;
- gate: `C03E_GF_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SEMANTICS_SELECTED`;
- PR #308: `Status: CLOSED`, draft/open/unmerged.

GF remains frozen. GG starts exactly from the GF commit and does not amend a closed predecessor.

## 2. Source-topology finding

GF selected one Agent-owned custody boundary around the existing production:

`ProductionReachabilityOwner<S,T>`

where:

- `S: ReachabilityDurableStore`;
- `T: CandidatePublicationFreshnessTokenSource`.

Existing production recovery requires an exact:

`&PeerConnectivityIdentity`

from `prw-connectivity`.

At exact GF head, `prw-agent` does not directly depend on `prw-connectivity`. GG therefore adds the direct workspace path dependency rather than relying on a transitive dependency or introducing a bridge re-export solely to bypass Cargo dependency ownership.

`prw-connectivity` already exists in the workspace and already exists as a package in `Cargo.lock`. Exact locked-graph validation determines whether the `prw-agent` dependency list in `Cargo.lock` must also receive the corresponding generated edge. GG does not pre-emptively rewrite unrelated lock content.

## 3. Materialized Agent owner

GG adds crate-internal:

`ProductionReachabilityOwnerCustody<S,T>`

The custody wrapper owns exactly one:

`ProductionReachabilityOwner<S,T>`

by value.

It does not implement `Clone` and stores no second plan, freshness record, durable store, token source, traversal session or peer snapshot outside the existing production owner.

## 4. Authoritative construction

GG materializes:

`ProductionReachabilityOwnerCustody::recover(store, token_source, &peer)`

Construction delegates exactly once to existing:

`ProductionReachabilityOwner::recover(store, token_source, peer)`

No alternative constructor exists in GG.

Consequences remain exactly the production-owner law:

- missing durable state -> `ReachabilityOwnerError::DurableStateMissing`;
- ambiguous/unavailable durable load -> exact `ReachabilityOwnerError::Persistence(...)`;
- peer mismatch -> exact snapshot mismatch classification;
- retired/recovery-required durable lifecycle is not silently made Current;
- no default plan/freshness/token is fabricated;
- no new-lifecycle bootstrap state is created.

## 5. One-time recovery law

Initial recovery is custody construction, not per-command behavior.

After successful construction, ordinary bounded mutable operations use the retained owner. GG performs no hidden second `load_current`, no per-operation `recover`, no background reload and no retry loop.

If the retained owner later reaches `RecoveryRequired`, existing explicit `reload_from_store()` remains the only production-owner reload law. GG does not invoke it automatically.

## 6. Bounded mutable composition seam

GG adds one crate-internal closure seam:

`with_owner_mut(...)`

It provides lexical exclusive mutable access to the retained production owner only for one caller-supplied synchronous operation.

The closure is higher-ranked over the mutable owner borrow so a reference tied to the owner borrow cannot be returned as the generic result. GG exposes no raw owner getter, raw store getter, raw token-source getter, mutex guard or clone.

The seam itself performs no candidate execution or mutation. A later separately gated adapter must select the actual operation supplied to it.

## 7. No async/mutex invention

`ProductionReachabilityOwner` already serializes in-process mutation through `&mut self`, while its durable compare-and-commit seam is the existing cross-owner/process arbiter.

GG therefore does not wrap the owner in:

- `Arc`;
- `Mutex`;
- Tokio mutex;
- task mailbox;
- background worker.

No concurrency primitive is introduced without a separately proven requirement.

## 8. Direct dependency boundary

GG adds exactly one direct workspace dependency to `crates/prw-agent/Cargo.toml`:

`prw-connectivity = { path = "../prw-connectivity" }`

This dependency exists solely because Agent now names the exact `PeerConnectivityIdentity` required by the GF-selected recovery constructor.

GG does not add a new external crate or version.

## 9. Lockfile rule

`Cargo.lock` already contains the workspace `prw-connectivity` package.

GG requires exact-head locked dependency validation before closure.

- If Cargo accepts the direct dependency with the existing lockfile, `Cargo.lock` MUST remain byte-stable.
- If Cargo reports that the lockfile must be updated, the only authorized lockfile change is the generated addition of `"prw-connectivity"` to the `prw-agent` package dependency list, with no package/version/checksum churn.

Any broader lockfile change is out of scope and blocks closure.

## 10. Focused source tests

GG source tests use a bounded in-memory counting durable-store test double and counting token source to prove construction behavior.

The successful recovery test proves:

- exact requested peer is passed to the durable store;
- `load_current` occurs exactly once;
- recovered mode is existing `Current` for an established snapshot;
- repeated bounded custody access does not reload;
- no durable compare-and-commit occurs during construction/access observation;
- no freshness token is issued during construction/access observation.

Failure tests prove:

- missing durable state remains exactly `DurableStateMissing`;
- ambiguous durable load remains exact nested persistence error;
- no durable commit/token issue follows either recovery failure.

## 11. Live-owner authority remains distinct

GG does not modify:

- `ReachabilityLiveOwnerComposedAsyncAuthority`;
- `ReachabilityAuthorityRuntimeOwner`;
- live-owner lease/fence currentness;
- reachability authority bootstrap/custody.

No conversion, equivalence or shared mutation domain between live-owner authority and `ProductionReachabilityOwner<S,T>` is created.

## 12. Candidate ingress remains dormant

GE's typed current-Mesh candidate family remains stopped at:

`CandidatePublicationHandoffNotSelected`

GG does not remove or bypass this barrier.

It does not connect `PostAuthCandidatePublicationTransaction` to the production owner.

## 13. FY/GA/GC remain dormant

GG does not invoke:

- `execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`;
- `project_candidate_publication_terminal_result(...)`;
- `compose_candidate_publication_terminal_result_frame(...)`.

Requester/rendezvous selection, durable candidate commit, post-commit requester cleanup, semantic projection and terminal result framing remain separately gated.

## 14. No response write

GG adds no current-Mesh candidate response send/finish API.

It does not select:

- Accepted/Rejected write custody;
- local send error classification;
- retry/re-encoding;
- peer close policy;
- repeated-loop continuation after candidate response.

## 15. No traversal/runtime activation

GG does not call or activate:

- `provision_current_traversal(...)`;
- `poll_and_apply_current_reachability(...)`;
- traversal factories;
- socket creation;
- listener bind/accept;
- readiness publication;
- target selection/dialing;
- deployment;
- process restart/recovery.

Recovery construction restores only the existing durable production owner state as defined by `ProductionReachabilityOwner::recover`.

## 16. Authorized path set

Before any CI-required lock correction, GG is authorized to change exactly four paths:

1. this GG contract;
2. `crates/prw-agent/Cargo.toml`;
3. `crates/prw-agent/src/lib.rs`;
4. `crates/prw-agent/src/production_reachability_owner_custody.rs`.

A fifth path, root `Cargo.lock`, is conditionally authorized only if exact locked-graph validation proves it required, and then only for the single existing-package dependency edge described above.

No other path is authorized.

## 17. Validation requirements

Canonical closure requires exact-final-head:

- exact GF merge base;
- ahead-only branch state;
- only authorized paths;
- locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS with warnings denied;
- workspace tests PASS;
- workspace build PASS;
- Android validation PASS if the exact final head triggers Android workflow;
- path-filtered AD/AE recorded accurately as PASS/skipped/failure, never inferred.

Any correction creates a new candidate head and supersedes prior validation evidence.

## 18. Immutable evidence

After exact-final-head CI is terminal green:

1. freeze GG source;
2. record final commit/tree/compare/path/blob evidence;
3. create immutable GG audit locally;
4. compute exact local bytes and SHA-256;
5. upload directly to canonical Drive folder `Private Remote Workspace` / `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch the exact Drive object;
7. recompute bytes/SHA and require equality;
8. update the GG PR to `Status: CLOSED` while keeping it draft/open/unmerged;
9. independently re-read PR and branch state.

No My Drive root upload is permitted.

## 19. Explicit non-goals

GG does not:

- materialize current-Mesh candidate execution adaptation;
- map an authenticated session to a production owner;
- create a peer-to-owner collection/registry;
- choose multi-peer lookup cardinality;
- mutate requester/rendezvous authority;
- commit candidate publication;
- project or write a terminal result;
- activate traversal;
- bind/accept listener;
- publish readiness;
- dial;
- deploy;
- restart/recover a process;
- merge;
- delete branches.

## 20. Closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_GG_CANDIDATE_PUBLICATION_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RECOVERY_SOURCE_MATERIALIZED`

Canonical source law:

**Agent owns one non-cloneable production reachability owner recovered exactly once from authoritative durable state for one exact peer lifecycle; construction preserves existing typed recovery failures, bounded mutable custody cannot leak a raw owner reference, and GG performs no candidate execution, response I/O or runtime activation.**

## 21. Successor rule

GG does not pre-authorize an execution adapter.

After canonical GG closure, perform a fresh exact-final-head audit before selecting the next prerequisite. In particular, source materialization may expose a peer-to-production-owner lookup/cardinality requirement that must be selected before the GE current-Mesh candidate handoff can replace `CandidatePublicationHandoffNotSelected`.
