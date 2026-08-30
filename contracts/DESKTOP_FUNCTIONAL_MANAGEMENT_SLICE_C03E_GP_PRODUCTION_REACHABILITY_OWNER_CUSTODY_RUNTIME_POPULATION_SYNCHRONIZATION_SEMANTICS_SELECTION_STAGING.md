# Phase 152 — C03e-GP Production Reachability Owner Custody Runtime Population / Synchronization Semantics Selection Staging

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-GP selects the narrow production-runtime population and synchronization semantics that must hold before the existing per-peer `ProductionReachabilityOwnerCustodyMap` may become an authoritative input to candidate-publication orchestration.

This checkpoint is selection-only. It does not populate the map, activate candidate-publication handoff, add a watcher, select a distributed backend, alter transport runtime behavior, start a worker, bind a listener, dial, publish readiness, deploy, restart, merge or change repository visibility.

## 2. Exact predecessor

The exact predecessor is canonically CLOSED C03e-GO.

- GO branch: `phase-152-c03e-go-candidate-publication-current-mesh-same-stream-terminal-response-custody-source-materialization-staging`
- GO final head: `308b198ba3ba92e2fb7c50cd1bc06b49008eed8b`
- GO final tree: `4cc01a0c02aa1e10f66128b6a921968316398bba`
- GO PR: `#317`
- GO remains draft/open/unmerged.

GP begins from the exact GO final head and does not amend GO.

## 3. Fresh exact-GO source facts

The selection in this contract is grounded in the exact GO source state, not in an assumed future architecture.

### 3.1 Existing custody map

`crates/prw-agent/src/production_reachability_owner_custody.rs` already defines `ProductionReachabilityOwnerCustodyMap`.

The current source establishes that:

- the map is constructed from already-recovered `ProductionReachabilityOwnerCustody` values;
- each custody contributes an exact `PeerConnectivityIdentity` key;
- duplicate exact peer identities fail closed during construction;
- exact lookup returns the custody associated with that exact peer;
- mutable lookup is the route to mutable production-owner custody;
- the map does not use IP addresses, ports or current reachable endpoints as owner-authority identity.

### 3.2 Candidate execution requires mutable production owner authority

`crates/prw-remote-bridge/src/candidate_publication_execution.rs` requires a mutable `ProductionReachabilityOwner` for candidate-publication semantic execution.

Therefore an Agent-level candidate handoff cannot be considered production-authoritative merely because ingress, semantic execution, result framing and same-stream response custody have been materialized separately. The higher runtime must first possess exact per-peer mutable owner custody.

### 3.3 Current Linux Agent bootstrap does not populate this map

At exact GO head:

- `crates/prw-agent/src/main.rs` performs startup/preflight and delegates to `linux_bootstrap::run()`;
- `crates/prw-agent/src/linux_bootstrap.rs` does not wire `ProductionReachabilityOwner` or `ProductionReachabilityOwnerCustodyMap` into the production runtime;
- related custody/bootstrap modules remain staged rather than proving an active production owner-custody population path.

Consequently GP must select the missing population/synchronization law before any Agent candidate-handoff activation is considered.

## 4. Selected authoritative key law

The authoritative map key is exactly `PeerConnectivityIdentity`.

The runtime must not key, alias, merge, select or authorize production owner custody by:

- IP address;
- socket address;
- port;
- DNS answer;
- relay endpoint;
- current candidate endpoint;
- current transport connection address;
- PRWM/current-Mesh `request_id`;
- historical PRWC request correlation;
- freshness token;
- display name.

Endpoint material remains transient reachability data.

Canonical identity remains:

`logical device/session identity -> registry/discovery -> current reachable endpoint/candidates -> authenticated transport`

Not:

`device identity = static IP`

and not:

`owner authority = endpoint identity`.

## 5. Selected population input law

Production population consumes only already-recovered `ProductionReachabilityOwnerCustody` values whose existing recovery and ownership invariants have already succeeded.

GP does not select a new recovery mechanism and does not permit runtime population to manufacture owner authority from incomplete transport or endpoint observations.

Population input therefore means:

`already-recovered production owner custodies -> validate exact peer uniqueness -> construct complete owner-custody map -> expose complete map to authorized higher runtime`

It does not mean:

`live endpoint observation -> infer owner -> insert authority`.

## 6. Selected fail-closed uniqueness law

At most one authoritative mutable production owner custody may exist in one populated map for one exact `PeerConnectivityIdentity`.

If population input contains duplicate exact peer identities, population fails closed.

GP does not select:

- last-write-wins replacement;
- first-write-wins suppression;
- silent deduplication;
- endpoint-preference arbitration;
- freshness-token arbitration between duplicate owner custodies;
- merge of mutable owner authority;
- fallback to a different peer key.

Any future replacement or transfer law must be selected explicitly by a later checkpoint and must preserve single mutable authority.

## 7. Selected atomic exposure law

The runtime must construct and validate the complete intended startup/recovery owner-custody snapshot before that map is exposed as authoritative input to candidate-publication execution.

A population failure must not expose a partially authoritative map to candidate execution.

The selected sequence is:

1. obtain the intended already-recovered custody set;
2. validate exact `PeerConnectivityIdentity` uniqueness while constructing the map;
3. fail the population operation if any required construction invariant fails;
4. only after successful complete construction, publish/expose that completed map to the owning runtime;
5. only then may exact-peer lookup be considered eligible for later candidate-handoff orchestration.

This is an in-process authority exposure law. GP does not claim or require a database transaction, distributed transaction, consensus protocol or external atomic-swap mechanism.

## 8. Selected startup/recovery snapshot law

The narrowest currently-supported production population model is a startup/recovery snapshot of already-recovered custodies.

GP therefore selects:

- one complete startup/recovery population boundary;
- exact-peer immutable key membership for that populated snapshot unless a later explicitly-authorized synchronization mechanism updates it;
- mutable access to the owner custody behind an exact key as required by existing execution seams;
- fail-closed startup/recovery population if the intended custody snapshot cannot be constructed consistently.

GP does not infer that the current Agent bootstrap already supplies this snapshot. That source-materialization remains future work.

## 9. Live synchronization mechanism explicitly unselected

The repository evidence at exact GO does not establish a production-authoritative live synchronization mechanism for `ProductionReachabilityOwnerCustodyMap`.

Therefore GP intentionally does **not** select or fabricate:

- filesystem watcher;
- etcd watch;
- Spanner change stream;
- pub/sub subscription;
- polling interval;
- lease-renewal loop;
- background reconciliation worker;
- hot-reload channel;
- actor mailbox;
- lock-free map replacement scheme;
- process-to-process owner transfer protocol;
- cross-host consensus protocol.

A later checkpoint may select live synchronization only from concrete source/platform evidence and must preserve the exact-peer, single-mutable-owner and fail-closed laws selected here.

Until such a mechanism is selected and materialized, the production-authoritative interpretation is the successfully constructed startup/recovery snapshot only.

## 10. No silent live mutation law

Because live synchronization is unselected, unrelated runtime paths must not gain authority to insert, delete, replace or re-key `ProductionReachabilityOwnerCustodyMap` entries opportunistically.

In particular, candidate publication itself must not mutate owner-map membership merely because:

- a request was authenticated;
- a request carries a matching `request_id`;
- a peer presented a new endpoint;
- durable reachability commit succeeded;
- a current-Mesh response was sent successfully;
- transport reconnect occurred.

Candidate publication may mutate the reachability state owned by an already-authoritative `ProductionReachabilityOwner`; it does not create the owner authority that permits that mutation.

## 11. Selected lookup gate for later candidate handoff

A later Agent candidate-publication handoff may proceed toward semantic execution only after exact authenticated peer context resolves to exactly one mutable production owner custody under the populated map.

The conceptual gate is:

`authenticated peer context -> exact PeerConnectivityIdentity -> authoritative custody-map lookup -> mutable ProductionReachabilityOwner -> existing candidate semantic execution`

No endpoint-address fallback is authorized.

No missing-owner fallback is authorized.

No duplicate-owner arbitration is authorized.

## 12. Missing owner / population failure classification

Failure to construct the production owner-custody snapshot, or failure to find required exact owner custody, is an internal owner-authority/runtime-custody failure.

It must remain distinct from:

- candidate semantic `Rejected` result;
- durable commit result;
- requester cleanup disposition;
- GM current-Mesh terminal-result frame construction error;
- GO same-stream terminal-response I/O error;
- transport authentication failure;
- request-correlation mismatch.

GP does not select a peer-visible wire encoding for internal owner-custody failure and does not authorize disclosure of internal owner-custody details.

## 13. Mutable-authority fencing law

The purpose of the custody map is not merely to locate data. It gates mutable production owner authority.

Any later population/synchronization materialization must preserve:

- one mutable authoritative custody per exact peer key within the active map state;
- no aliasing by endpoint;
- no simultaneous silent replacement of active mutable authority;
- no cloning/reconstruction of owner authority from request metadata;
- no bypass around existing `ProductionReachabilityOwner` ownership/fencing semantics.

The exact mechanism for a future live owner transfer is outside GP.

## 14. Recovery and population remain separate phases

GP keeps these concepts distinct:

1. recovery establishes a valid `ProductionReachabilityOwnerCustody` under existing recovery rules;
2. population validates the set of already-recovered custodies and installs the complete exact-peer map for runtime lookup;
3. candidate execution consumes mutable owner authority obtained through that map;
4. any future live synchronization changes map authority only under a separately selected mechanism.

Population must not weaken recovery checks, and candidate execution must not substitute for population.

## 15. No candidate-publication orchestration activation

Even though C03e-GK, GM and GO already materialize semantic execution, terminal frame composition and same-stream terminal-response custody seams, GP does not wire them together from Agent ingress.

`CandidatePublicationHandoffNotSelected` remains unchanged unless a later checkpoint explicitly replaces it after all required runtime custody prerequisites are materialized and validated.

GP does not:

- invoke candidate semantic execution from production ingress;
- compose candidate terminal frames from Agent ingress;
- send candidate terminal responses from Agent ingress;
- select repeated-ingress worker behavior;
- select candidate cancellation behavior;
- select peer-close behavior;
- replay candidate semantics after response failure.

## 16. No new runtime concurrency law

GP does not select a new mutex, RwLock, actor, channel, concurrent map, task topology or synchronization primitive.

The word "synchronization" in this checkpoint means authority-state synchronization semantics: what must remain true if owner-custody membership ever changes. It does not pre-authorize a concrete threading or distributed-system implementation.

The currently selected executable boundary remains startup/recovery snapshot population with no live synchronization mechanism.

## 17. Dynamic-network invariant

GP preserves the project-wide dynamic-network invariant.

A device/peer may move between networks or change IP addresses without changing its logical identity or owner-authority key.

Therefore:

- `PeerConnectivityIdentity` is the custody-map key;
- current endpoint candidates may change;
- authenticated transport may reconnect;
- IP/port changes do not create, transfer or revoke owner custody by themselves.

## 18. Source-materialization target implied by this selection

After canonical GP closure, a fresh exact-final-head audit may authorize a narrow successor that materializes startup/recovery owner-custody population in Agent runtime.

Such a successor should, if source evidence still supports it:

- consume already-recovered production owner custodies;
- construct one `ProductionReachabilityOwnerCustodyMap` fail-closed;
- expose it only after complete construction;
- preserve exact peer-key exclusivity;
- avoid adding a live synchronization mechanism unless separately selected;
- remain distinct from candidate-handoff orchestration if that is still the narrowest safe boundary.

This paragraph does not pre-authorize a particular file change or runtime activation. A fresh audit is mandatory.

## 19. Explicit non-activation / non-selection boundary

GP does not authorize or select:

- concrete owner-custody recovery backend changes;
- production map mutation code;
- live watcher/reconciler/poller;
- distributed lease protocol;
- ownership transfer protocol;
- Agent candidate handoff;
- ingress worker/task activation;
- cancellation policy;
- traversal activation;
- listener binding;
- readiness publication;
- production dialing;
- service deployment;
- restart/recovery action;
- merge;
- branch deletion;
- repository visibility mutation.

## 20. Repository visibility observation

Repository metadata has been observed reporting the repository as public even though the project posture expects private operation.

GP does not change repository visibility. Any visibility mutation requires separate explicit authorization.

## 21. GP change ceiling

This checkpoint is docs-only semantics selection.

Authorized changed path ceiling: exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GP_PRODUCTION_REACHABILITY_OWNER_CUSTODY_RUNTIME_POPULATION_SYNCHRONIZATION_SEMANTICS_SELECTION_STAGING.md`

No Rust, Cargo manifest/lockfile, workflow, Android/Kotlin/Gradle, runtime, Agent implementation, transport, deployment, configuration or unrelated contract path is authorized to change in GP.

## 22. Validation law

Only the exact final GP head may provide closure evidence.

If the GP contract requires a correction, every validation run on the superseded head is superseded for closure purposes.

No manual workflow dispatch is authorized.

Path-filtered workflows must be recorded with their actual conclusions; skipped is not PASS.

## 23. Closure law

GP may become canonically CLOSED only after all of the following hold:

1. exact predecessor is the GO final head;
2. exact GO -> GP compare is ahead-only with no behind commits;
3. changed-path set is exactly the one authorized GP contract path;
4. exact final contract blob is recorded;
5. required automatic CI on the exact final GP head reaches terminal acceptable conclusions;
6. an immutable GP audit is created from the exact final state;
7. exact local audit bytes and SHA-256 are recorded;
8. the audit is uploaded directly to the canonical `Private Remote Workspace` Drive folder;
9. the exact uploaded object is raw-fetched;
10. raw readback bytes and SHA-256 equal the local audit exactly;
11. PR/head state is re-read immediately before closure metadata mutation;
12. PR body is updated to `Status: CLOSED` while the PR remains draft/open/unmerged;
13. PR and branch are independently re-read after closure.

## 24. Successor rule

Canonical GP closure selects semantics only. It does not authorize immediate Agent candidate-handoff orchestration.

After closure, perform a fresh exact-final-head audit and choose the narrowest remaining source boundary.

The expected candidate is startup/recovery production owner-custody map population materialization, unless fresh source evidence shows a different prerequisite.

Agent candidate-handoff orchestration remains gated until production owner-custody runtime population itself is materialized and validated.