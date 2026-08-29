# Phase 152 C03e-FN — Borrowed Executor Drive Seam for FL Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FN materializes only the C03e-FM-selected first borrowed executor integration seam for the already-materialized C03e-FL requester-aware cancellation-aware serial lifecycle worker.

FN adds no spawned task, join handle, cancellation controller, channel, queue, persistent collection entry, peer close, peer drop, worker restart, session reuse, requester-authority synchronization, candidate/reachability continuation, target dialing, production listener activation, deployment, restart/recovery, or merge.

## 2. Exact predecessor

FN is based exactly on closed C03e-FM:

- predecessor branch: `phase-152-c03e-fm-production-higher-owner-integration-custody-peer-disposition-selection-staging`
- predecessor head: `b6327635e491bd0263992038c13e1969f0032854`
- predecessor tree: `e46a52e1072dd528dbe5a1381e32912c8dfe3bb7`
- predecessor gate: `C03E_FM_PRODUCTION_HIGHER_OWNER_INTEGRATION_CUSTODY_PEER_DISPOSITION_SELECTED`

FN must remain a strict descendant of that exact head.

## 3. Exact source guards before FN mutation

The following FM-head blobs are authoritative guards:

- FL worker / FB / FH source:
  `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
  `2a4bcbf48965b8ef5fa3202b3bb3ef46b3f96f31`
- executor runtime:
  `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
  `47c41735de3c153cde8794b46479e09da7cfba18`
- authenticated-session runtime:
  `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
  `083bf83fd1827f6175c9eb62ff93b40147fa9271`
- requester/rendezvous runtime owner:
  `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
  `68ba74e82cf703664b7ee090a10fc1c6cce1609d`
- parent remote-session runtime module:
  `crates/prw-agent/src/remote_session_capability_runtime.rs`
  `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`

## 4. Materialized executor seam shape

FN adds one Agent-internal borrowed executor method on the existing `RemoteSessionExecutorRuntime`.

Conceptually:

```text
drive_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(
    &mut executor,
    &mut authenticated_session_owner,
    &shared_current_authority,
    &requester_policy_source,
    &mut requester_rendezvous_runtime_owner,
    verifier_time_source,
    &mut dispatcher,
    cancellation,
) -> exact FL worker stop
```

The exact Rust signature remains bounded to the existing generic authorities and traits already used by FL.

## 5. Executor custody law

The existing private current-thread Tokio runtime remains the sole executor used by this seam.

FN performs exactly one domain-specific `Runtime::block_on(...)` through the already-owned private executor runtime.

It does not:

- construct another runtime;
- expose a raw runtime or handle;
- expose generic `block_on`;
- spawn a task;
- construct a join handle;
- construct a cancellation controller;
- insert work into a persistent collection.

## 6. Exact FL delegation law

The borrowed executor seam delegates exactly once to:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`

It does not reconstruct FL semantics locally.

Therefore FL remains the sole authority for:

- ingress-first cancellation ordering before requester handoff;
- cancellation deferral through exact FB + FH requester critical section;
- requester-response failure precedence over pending cancellation;
- post-FH cancellation observation before the next ingress cycle;
- exact `Cancelled` versus `Failed(existing FJ error)` classification.

## 7. Borrowed owner law

The executor method borrows, rather than consumes:

- exact `AuthenticatedRemoteSessionRuntimeOwner` by `&mut`;
- exact `CandidatePublicationRequesterRendezvousRuntimeOwner` by `&mut`;
- mutable dispatcher by `&mut`.

The shared-current authority and requester policy source remain borrowed read authority inputs.

The verifier-time provider and caller cancellation future are passed into exact FL according to its existing ownership contract.

After the executor drive returns, both mutable owners return to the caller automatically with their exact post-FL state.

## 8. Exact stop return law

FN returns the exact existing:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

unchanged.

No wrapper error, success type, join error, capability-only worker stop, integer code, boolean, string, or suppression layer is added.

`Cancelled` remains `Cancelled`.

`Failed(Ingress(...))` remains exact FL failure.

`Failed(RequesterResponse(Frame(...)))` remains exact FL failure.

`Failed(RequesterResponse(ResponseIo(...)))` remains exact FL failure.

## 9. Retained-stopped peer law

FN performs no peer disposition.

After any normal exact FL stop:

- the authenticated-session owner remains borrowed caller custody;
- no code-3 close occurs;
- no code-4 close occurs;
- no new mixed-family close occurs;
- no intentional owner drop occurs;
- no restart/reuse/next ingress occurs in FN.

This preserves FM retained-stopped custody exactly.

## 10. Existing capability executor paths remain unchanged

FN does not alter semantics of:

- `drive_capability_request_worker`;
- `drive_spawned_capability_request_worker`;
- `drive_supervised_capability_request_worker`;
- `drive_persistent_remote_worker_collection`;
- repeated real-admission supervisor paths;
- endpoint-lifecycle paths.

Capability-only code-3/code-4 behavior remains capability-only.

## 11. Requester-authority concurrency barrier remains closed

FN borrows one exact mutable process-local requester/rendezvous runtime owner for the whole FL drive.

It does not select or materialize:

- `Arc`;
- mutex;
- RwLock;
- actor;
- channel;
- provider clone;
- sharding;
- concurrent FL workers.

Persistent multi-worker FL integration remains separately gated.

## 12. Identity and correlation law

FN preserves all prior identity boundaries:

- authenticated PRW session lineage remains requester logical identity;
- logical `DeviceId` remains device identity;
- dynamic IP/port remains transient endpoint data only;
- `TransportIdentity` remains lower transport evidence only;
- PRWM `request_id` remains correlation only.

Executor identity, Tokio runtime identity, task identity, cancellation timing, failure category, stream metadata, or request ordering does not become logical identity.

## 13. Candidate/reachability boundary remains closed

FN authorizes no candidate query/selection, reachability evaluation, endpoint resolution, relay/direct-path selection, target dialing, port-forward activation, terminal activation, remote-session establishment, or rendezvous completion claim.

Requester DR accepted-for-continuation semantics remain unchanged.

## 14. Runtime activation boundary remains closed

FN is source materialization only.

No existing production caller is changed to invoke the new method.

No listener, process-lifecycle, readiness, service-manager, packaging, host networking, deployment, restart/recovery, or merge behavior is changed.

## 15. Allowed implementation scope

FN may change only:

1. this FN contract;
2. the existing executor source needed to add the borrowed drive seam.

No FL worker source change is authorized unless compiler evidence demonstrates an unavoidable visibility contradiction. No such contradiction was found in the pre-mutation audit.

## 16. Validation gate

FN may close only after:

- exact FM merge-base verification;
- exact final changed-path cardinality audit;
- exact-final-head Rust validation FULL PASS;
- Android validation only if actually triggered, with no unsupported PASS claim otherwise;
- immutable Drive audit upload;
- raw byte-exact Drive readback;
- PR semantic closure while remaining draft/open/unmerged.

## 17. Explicit non-goals

No peer close.
No peer drop policy.
No restart/reuse.
No spawned worker.
No persistent collection mutation.
No requester-authority synchronization.
No candidate/reachability continuation.
No target dial.
No listener activation.
No dependency/workflow widening.
No Android source mutation.
No packaging.
No deployment.
No restart/recovery.
No merge.

## 18. Target gate

On successful source materialization and exact-head validation:

`C03E_FN_BORROWED_EXECUTOR_DRIVE_FL_SOURCE_MATERIALIZED`