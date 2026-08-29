# Phase 152 C03e-FW — Recovered Requester-Aware Worker Completion Peer-Disposition Source Materialization

Status: VALIDATING

## 1. Purpose

C03e-FW materializes only the C03e-FV-selected peer-disposition semantics for one exact C03e-FU recovered requester-aware worker completion.

FW adds no requester-record cleanup, no peer reuse, no worker restart, no candidate/reachability continuation, no dialing, no listener/bootstrap/readiness activation, no deployment, no restart/recovery, and no merge.

## 2. Exact predecessor

Canonical predecessor checkpoint:

`C03E_FV_RECOVERED_REQUESTER_AWARE_WORKER_COMPLETION_PEER_DISPOSITION_REQUESTER_CLEANUP_SEMANTICS_SELECTED`

Exact predecessor branch:

`phase-152-c03e-fv-recovered-requester-aware-worker-completion-peer-disposition-requester-cleanup-semantics-selection-staging`

Exact predecessor head:

`fc285d5a0ef84345eea90e43fdec36c2d7d07ffd`

Exact predecessor tree:

`8a6c9b83c3a2b6c9e5d556011f323465797141e2`

FV remains frozen.

## 3. Materialization scope

FW may change only:

1. this contract;
2. the existing authenticated-session descendant module that already owns requester/rendezvous peer operations, to add one dedicated consuming requester-aware terminal-failure close seam;
3. the existing recoverable requester-aware worker custody module, to add one higher-owner completion disposition classifier/consumer and focused tests.

The FU repeated-admission scheduler, active map, AJ path, FS owner recovery, cancellation-all-then-drain law, requester/rendezvous authority provider, Android source, manifests, lockfiles, workflows, packaging, and Agent binary remain byte-stable.

## 4. Verified close-code namespace

Current source already owns these fixed close diagnostics:

- code `1`: logical-session authentication transaction failure;
- code `2`: post-authentication binding failure;
- code `3`: capability-session termination failure;
- code `4`: orderly capability-session shutdown;
- code `5`: post-accept real-admission preparation failure.

FW selects the first unused contiguous code:

`6`

for requester-aware terminal failure.

The fixed non-secret reason is:

`remote requester-aware session terminated`

The diagnostic contains no user/workspace/device/session/requester/target identity, policy result, raw error, panic payload, network address, or task identity.

## 5. Dedicated terminal-failure close seam

FW materializes a consuming method on `AuthenticatedRemoteSessionRuntimeOwner` with the normative ownership shape:

`close_for_requester_aware_terminal_failure(self)`

The seam:

- consumes the exact recovered owner by value;
- closes the exact retained authenticated peer once;
- uses fixed code `6` and the fixed non-secret requester-aware termination reason;
- exposes no raw peer;
- performs no requester cleanup;
- performs no session deletion;
- performs no retry/reconnect;
- performs no candidate/reachability work.

It is distinct from existing code-3 capability failure and existing code-4 orderly shutdown.

## 6. Completion disposition classifier

FW materializes one narrow internal classification with exactly two peer-disposition branches:

- `OrderlyShutdown`;
- `TerminalFailure`.

The exact original completion result is not rewritten or flattened. Classification exists only to select the peer-close side effect.

Mapping is exact:

- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled)` -> `OrderlyShutdown`;
- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(_))` -> `TerminalFailure`;
- `Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)` -> `TerminalFailure`.

## 7. Higher-owner completion consumer

FW materializes one consuming helper over exact FU completion custody:

`RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`

The helper:

1. consumes the completion;
2. recovers exact `DeviceId`, exact session owner, and exact FL/join result through the existing `into_parts()` boundary;
3. classifies the peer-disposition branch from the exact result;
4. consumes the exact owner through:
   - existing `close_for_orderly_shutdown(self)` for cancellation;
   - new `close_for_requester_aware_terminal_failure(self)` for typed FL failure or abnormal join;
5. returns only authenticated `DeviceId` plus the exact unchanged FL/join result after owner disposition.

The helper returns no peer, owner, restart token, requester cleanup authority, or reachability state.

## 8. Cancellation branch

Cancellation remains the FU repeated-supervisor orderly shutdown path.

FW reuses existing code-4 closure exactly and does not duplicate or alter that diagnostic.

No new cancellation code is introduced.

## 9. Typed FL failure branch

Exact typed FL failures remain unchanged:

- ingress failure;
- requester-response framing failure;
- requester-response I/O failure.

All typed FL failures select new code-6 terminal peer disposition.

No typed failure is reclassified as cancellation or abnormal join.

## 10. Abnormal join branch

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion` selects the same code-6 fail-stop peer disposition.

The join classification remains unchanged in the returned result.

No panic payload, Tokio task identity, retry, reconnect, or replacement worker is introduced.

## 11. No clean-success reuse

Exact FL has no ordinary clean-success terminal variant.

FW materializes no peer-reuse branch.

## 12. Requester cleanup remains absent

FW does not call or expose requester/rendezvous provider lifecycle operations.

No `retire`, `remove_retired`, publisher-wide cleanup, wildcard cleanup, provider reset, or compensation rollback is added.

Exact requester cleanup remains separately gated because FU completion does not carry `(requester SessionId, expected publisher DeviceId)`.

## 13. FU scheduler remains byte-stable

FW does not change:

- repeated expected-device request polling;
- duplicate preflight;
- one in-flight AJ;
- authenticated `DeviceId` active-map key;
- exact FL worker spawn;
- ready-completion-first ordering;
- FS owner recovery;
- shutdown cancellation-all-then-drain;
- post-shutdown AJ drain;
- never-inserted post-shutdown AJ-success orderly close.

The new disposition helper is a higher-owner consumption seam only.

## 14. Focused tests

FW must add focused compile/branch tests proving:

- the new terminal-failure close method is consuming by value;
- fixed code is `6` and reason is the exact non-secret constant;
- cancellation classifies to orderly shutdown;
- typed FL failure classifies to terminal failure;
- abnormal join classifies to terminal failure;
- the completion-disposition helper consumes exact FU completion and returns only `DeviceId` plus unchanged result shape.

No network integration test or runtime activation is required by this checkpoint.

## 15. Validation

Before closure, exact final FW head must pass canonical PRW Rust validation:

- checkout;
- native prerequisites;
- exact toolchain;
- locked dependency graph;
- rustfmt;
- Clippy;
- workspace tests;
- workspace build.

Android validation must be reported only if an exact-head Android workflow actually triggers. No Android verdict may be inherited.

## 16. Immutable evidence

Closure requires one immutable Drive audit under canonical parent:

`0AD5eMiLa5v9xUk9PVA`

with local byte count/SHA-256 and raw Drive byte-exact readback.

PR must remain draft/open/unmerged.

## 17. Intended closure

After exact-final-head CI and immutable Drive readback, FW closes as:

`CLOSED_RECOVERED_REQUESTER_AWARE_WORKER_COMPLETION_PEER_DISPOSITION_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_FW_RECOVERED_REQUESTER_AWARE_WORKER_COMPLETION_PEER_DISPOSITION_SOURCE_MATERIALIZED`

## 18. Deferred boundaries

FW explicitly defers:

- requester-record retirement/removal;
- requester cleanup receipt/token design;
- requester cleanup timing/idempotence/error policy;
- provider persistence/TTL;
- peer reuse;
- worker restart;
- reconnect/re-admission automation;
- candidate publication;
- candidate/reachability selection;
- target dialing/forwarding;
- listener/bootstrap/readiness activation;
- deployment;
- restart/recovery;
- merge.
