# Phase 152 C03e-FP — Shared Requester/Rendezvous Authority Synchronization and Borrowed FL Adaptation Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FP materializes only the first source seam selected by closed C03e-FO:

- one cloneable Agent-owned Tokio async-mutex wrapper around the exact existing process-local requester/rendezvous runtime owner;
- exact requester-authority lock -> current registry/policy read ordering for the existing DI -> DP -> DK -> DN requester-start composition;
- release of requester-authority synchronization before FD/FH acknowledgement framing and response I/O;
- adaptation of the existing FB/FJ/FL/FN borrowed requester path to consume the shared authority handle rather than one direct mutable process-local owner.

FP remains non-spawned and non-production. It does not materialize recoverable spawned-session custody, persistent FL collection integration, peer close/reuse policy, requester-record cleanup, candidate/reachability continuation, target dialing, listener activation, deployment, restart/recovery, or merge.

## 2. Exact predecessor

FP is based on exact closed C03e-FO:

- branch: `phase-152-c03e-fo-requester-rendezvous-authority-sync-owned-persistent-fl-custody-selection-staging`
- head: `c496422abdd41746d5602a445e27b7b8934c8961`
- tree: `5309027713d91446b6350fedfb7166e7cc6f56f4`
- PR: `#291`, `Status: CLOSED`, draft/open/unmerged
- gate: `C03E_FO_REQUESTER_RENDEZVOUS_AUTHORITY_SYNCHRONIZATION_OWNED_PERSISTENT_FL_CUSTODY_SELECTED`

## 3. Exact audited predecessor source guards

The source audit began from these exact FO-head blobs:

1. process-local requester/rendezvous runtime owner
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
   - `68ba74e82cf703664b7ee090a10fc1c6cce1609d`

2. FB/FJ/FL requester lifecycle
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - `2a4bcbf48965b8ef5fa3202b3bb3ef46b3f96f31`

3. FN borrowed executor seam
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - `a8359a88cbe924ad5d75eb9121e6d5b1bc0a8ee8`

4. remote-session runtime parent
   - `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`

5. shared-current registry/policy authority
   - `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
   - `50356b47d3c5304b67edd424e9286beb028ace16`

6. requester-start DR composition
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
   - `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`

7. requester-aware immutable policy source
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`
   - `f7377011a3ab2034c14d9018a5c0f268f6660ffa`

8. underlying bounded in-memory requester authority provider
   - `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`
   - `d01cfbc37433f6099e216397b9bf243aa55c53bc`

## 4. Materialized shared authority type

FP adds one Agent-internal type:

`SharedRequesterRendezvousAuthority`

Its state is conceptually and concretely:

`Arc<tokio::sync::Mutex<CandidatePublicationRequesterRendezvousRuntimeOwner>>`

The wrapper consumes exactly one existing runtime owner by value.

Clone semantics duplicate only the outer `Arc` handle. They do not clone:

- the runtime owner;
- the provider;
- provider records;
- registration state;
- policy state;
- registry state;
- requester identity;
- transport identity;
- request correlation.

Therefore all clone handles address one shared process-local requester/rendezvous authority allocation.

## 5. No raw synchronization/provider exposure

The new wrapper exposes no:

- raw provider reference;
- raw `Mutex` reference;
- raw `MutexGuard`;
- raw mutable runtime-owner reference;
- provider snapshot;
- reusable registration token.

Its operation-facing source seam performs exactly one requester-start composition under the selected lock ordering.

## 6. Exact lock ordering

The materialized order is:

1. acquire requester/rendezvous Tokio mutex;
2. while that guard is retained, call exact existing `SharedCurrentCapabilityAuthority::with_current_authority(...)`;
3. inside the synchronous current-authority operation, execute exact existing DI -> DP -> DK -> DN requester-start composition once;
4. current registry/policy read guard releases when `with_current_authority(...)` returns;
5. requester/rendezvous mutex guard releases before the shared-authority method returns to FB.

No reverse current-authority -> requester-authority nested acquisition is introduced by FP.

## 7. DR composition law

FP does not reimplement requester authorization.

Under the shared requester guard it still calls exactly:

`validate_authorize_and_register_requester_rendezvous_start(...)`

That preserves the existing stage order and exact failure taxonomy:

- DI registry validation;
- DP requester-aware policy source resolution;
- DK dedicated requester/rendezvous-start policy authorization;
- DN exact private provider registration mutation.

No fallback evaluator or alternate registration path is added.

## 8. FB adaptation

The existing retained-custody FB continuation now borrows:

`&SharedRequesterRendezvousAuthority`

instead of:

`&mut CandidatePublicationRequesterRendezvousRuntimeOwner`.

FB still owns the exact requester handoff transaction separately from the DR authority operation.

The exact requester transaction therefore survives DR success or failure unchanged.

## 9. Guard release before response I/O

FB awaits the new shared-authority DR operation to terminal completion before constructing the retained continuation.

The requester-authority guard is therefore gone before FH begins.

FP does not hold requester/rendezvous synchronization during:

- FD semantic projection;
- FD acknowledgement framing;
- FF same-stream send;
- QUIC send-direction finish;
- any later response I/O.

This preserves FO's bounded critical-section law.

## 10. FL cancellation law preserved

The existing FL worker cancellation boundaries remain unchanged.

Before requester handoff:

- ingress is polled first;
- cancellation is polled second;
- cancellation wins only while ingress remains pending.

After requester handoff:

- cancellation remains deferred across the exact FB DR critical section, including waiting for the requester-authority mutex;
- cancellation remains deferred across exact FH terminal acknowledgement response composition;
- exact requester-response failure still wins over cancellation that became ready during the critical section.

After FH success:

- cancellation is checked before the next EX cycle/verifier sample/stream accept.

No new cancellation point is inserted while requester authority synchronization is pending.

## 11. FJ serial lifecycle adaptation

The isolated requester-aware serial lifecycle now borrows the shared requester authority handle and otherwise preserves exact sequencing:

EX ingress -> FB synchronized DR -> FH terminal response -> next EX only after FH success.

No second EX transaction overlaps the synchronized requester transaction.

## 12. FN borrowed executor adaptation

The existing FN executor method keeps its non-spawned synchronous `Runtime::block_on(...)` shape.

Its requester-authority input changes from a mutable process-local runtime-owner borrow to:

`&SharedRequesterRendezvousAuthority`.

The authenticated-session owner remains a mutable caller-owned borrow.

The executor still:

- creates no task;
- creates no channel/queue;
- creates no cancellation controller;
- stores no join handle;
- mutates no persistent collection;
- preserves the exact FL worker stop unchanged.

## 13. Shared-current authority remains separate

FP does not merge requester/rendezvous authority with `SharedCurrentCapabilityAuthority`.

The former serializes requester/rendezvous provider mutation/current-record authority.

The latter remains the existing shared-current registry/policy authority.

They remain separate allocations, types, and authority domains with explicit nested ordering only for the exact requester-start DR operation.

## 14. Requester-aware policy source remains immutable

FP does not add synchronization or mutation to the bounded requester-aware policy source.

It remains borrowed read-only under its existing `Sync` requirement.

No policy snapshot is copied into the requester authority wrapper.

## 15. Provider semantics unchanged

The exact existing bounded in-memory provider remains one process-local state.

FP does not change:

- capacity;
- duplicate detection;
- current/retired lifecycle representation;
- publisher authorization behavior;
- retirement;
- retired-record removal;
- error taxonomy.

Synchronization only serializes access selected by FO.

## 16. No record cleanup

FP does not retire, remove, rollback, TTL-expire, reset, or otherwise clean requester records on:

- worker cancellation;
- ingress failure;
- response framing failure;
- response I/O failure;
- executor return.

Requester-authority cleanup remains separately gated.

## 17. No peer disposition

FP performs no authenticated-peer close for:

- FL `Cancelled`;
- FL `Failed(Ingress(...))`;
- FL `Failed(RequesterResponse(Frame(...)))`;
- FL `Failed(RequesterResponse(ResponseIo(...)))`.

Capability-only close code 3/code 4 are not widened.

No mixed-family close code/reason is introduced.

## 18. No spawned/persistent activation

FO selected future recoverable spawned-session custody, but FP does not materialize it.

FP does not:

- create `Arc<Mutex<Option<AuthenticatedRemoteSessionRuntimeOwner>>>` session custody;
- spawn FL;
- change task join records;
- change persistent admission/completion types;
- replace capability-only persistent workers;
- wire repeated real admission to FL;
- alter shutdown drain behavior.

## 19. Identity law

FP preserves:

- authenticated PRW application-session lineage as requester logical identity;
- logical `DeviceId` as device identity;
- dynamic IP/port as transient endpoint data only;
- `TransportIdentity` as lower transport evidence only;
- PRWM `request_id` as correlation only.

`Arc` allocation identity, mutex identity, lock ordering, task identity, stream metadata, endpoint tuples, cancellation timing, or request ordering do not become logical identity.

## 20. Candidate/reachability boundary remains closed

FP does not authorize:

- candidate query/selection;
- reachability evaluation;
- endpoint resolution;
- relay/direct-path selection;
- target transport establishment;
- port-forward activation;
- terminal activation;
- remote-session establishment;
- rendezvous completion claim.

Requester `Accepted` remains accepted-for-continuation only.

## 21. Runtime/deployment boundary remains closed

FP does not:

- wire the borrowed FN seam into production lifecycle;
- activate spawned FL;
- activate persistent FL;
- bind/listen for new production traffic;
- publish readiness;
- alter process lifecycle/main;
- change Android behavior;
- widen dependencies/workflows;
- package;
- deploy;
- restart/recover;
- merge.

## 22. Materialized source scope target

The intended FP source scope is limited to:

1. new shared requester/rendezvous authority module;
2. remote-session runtime parent registration/re-export;
3. FB/FJ/FL requester lifecycle adaptation to the shared handle;
4. FN borrowed executor input adaptation;
5. this FP contract.

No bridge, transport, provider, policy, registry, Cargo, lockfile, workflow, Android, listener, packaging, or deployment source is intended to change.

## 23. Source patch provenance

Because the existing executor source is large, exact literal replacements for the three pre-existing Rust paths were generated on the noncanonical helper branch:

`phase-152-c03e-fp-source-patch-helper-noncanonical`

The helper uses exact occurrence-count assertions before mutation.

Only the resulting Rust blob SHAs are imported into canonical FP source scope.

The helper workflow/path and helper branch history are noncanonical evidence and do not belong to the canonical FP tree.

The helper branch is not deleted because deletion is destructive and was not separately authorized.

## 24. Validation requirement

Closure requires exact-final-head:

- PRW Rust Validation FULL PASS;
- Android validation only if an exact-head Android workflow is actually triggered;
- expected auxiliary skips recorded;
- strict FO...FP compare proving only the selected paths changed;
- immutable Drive audit with raw byte-exact readback;
- PR semantic `Status: CLOSED`, remaining draft/open/unmerged.

## 25. Canonical closure target

`CLOSED_SHARED_REQUESTER_RENDEZVOUS_AUTHORITY_SYNCHRONIZATION_BORROWED_FL_ADAPTATION_SOURCE_MATERIALIZATION`

## 26. Canonical gate target

`C03E_FP_SHARED_REQUESTER_RENDEZVOUS_AUTHORITY_SYNCHRONIZATION_BORROWED_FL_ADAPTATION_SOURCE_MATERIALIZED`

## 27. Next separately gated checkpoint

After FP validates, the next conservative seam is expected to be:

**C03e-FQ — recoverable spawned authenticated-session custody and owned FL worker source materialization**.

FQ may materialize only the FO-selected supervisor-retained recoverable session-owner cell and one bounded spawned FL worker custody seam using the already-materialized shared requester authority.

Persistent collection substitution, peer close/reuse policy, requester-record cleanup, candidate/reachability continuation, target dialing, deployment, restart/recovery, and merge remain later gates.
