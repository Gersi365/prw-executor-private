# Phase 152 C03e-EZ — Requester/Rendezvous Response-Stream Custody Source Materialization (Staging)

Status: MATERIALIZING_STAGING

## 1. Purpose

C03e-EZ materializes only the C03e-EY-selected requester/rendezvous response-stream custody repair through the existing C03e-ET one-read family ingress, C03e-EV one-transaction Agent seam, and C03e-EX repeated-loop handoff barrier.

The checkpoint preserves one exact authenticated control stream by value together with the already strict decoded requester/rendezvous request and the existing session-derived non-authoritative `RequesterRendezvousStartIntent`.

C03e-EZ does not invoke C03e-DV, execute requester/provider authority, define requester response semantics, construct or write a response frame, resume the EX loop after requester handoff, dial traffic, activate runtime/listener/bootstrap behavior, deploy, restart, recover, or merge.

## 2. Exact predecessor authority

Canonical predecessor:

- repository: `Gersi365/prw-executor-private`
- checkpoint: C03e-EY
- branch: `phase-152-c03e-ey-requester-rendezvous-handoff-continuation-response-stream-custody-selection-staging`
- head: `071cf615f59c19957e450e6d2fc2a7f7aaec2f19`
- tree: `267b027c62bf9b0539c3e280c4489e09eb5f3c6d`
- contract blob: `d6197b354cacd2794a28e6de8a08f00b297ebaf3`
- closure: `CLOSED_REQUESTER_RENDEZVOUS_HANDOFF_CONTINUATION_RESPONSE_STREAM_CUSTODY_SELECTION`
- gate: `C03E_EY_REQUESTER_RENDEZVOUS_HANDOFF_CONTINUATION_RESPONSE_STREAM_CUSTODY_SELECTED`

C03e-EZ must remain an exact descendant of that predecessor.

## 3. Materialized ET requester custody

`prw-remote-bridge::post_auth_control_stream_ingress` must retain requester/rendezvous stream custody exactly as the capability branch already does.

The requester branch becomes one by-value bridge custody envelope containing:

- the exact strict `RequesterRendezvousTargetWireRequest` decoded from the first bounded PRWM frame;
- the exact same `MeshControlStream` from which that frame was read.

The envelope performs no second read, response write, retry, close, provider call, target resolution, dialing, or identity derivation.

The selected law remains:

`accepted stream -> exactly one ET read -> strict requester decode + exact same stream custody`

## 4. Materialized EV requester handoff

`AuthenticatedRemoteSessionRuntimeOwner::process_one_post_auth_control_stream_ingress(...)` continues to own the only accept for one isolated transaction.

For requester/rendezvous traffic it must:

1. receive the ET requester custody envelope by value;
2. read the strict decoded target only from that envelope;
3. adapt that exact logical target through the existing C03e-EO helper;
4. derive requester identity only from the retained authenticated PRW session through the existing C03e-EJ helper;
5. package the resulting `RequesterRendezvousStartIntent` together with the exact ET custody envelope as one Agent handoff value;
6. return the handoff without invoking C03e-DV or any requester/provider authority.

No clone or replacement of the stream is permitted.

## 5. Materialized EX handoff barrier

The C03e-EX repeated loop and cancellation-aware worker retain their existing control law:

- capability success may continue to the next iteration;
- requester/rendezvous returns one typed handoff barrier immediately;
- no new stream is accepted while that requester handoff is outstanding.

C03e-EZ changes only the requester handoff payload so it now carries response-stream custody. It does not resume the loop after a future DV result or response attempt.

## 6. Correlation and identity separation

The strict decoded request remains the sole holder of the outer requester `request_id`.

That value remains transaction correlation only.

Identity authority remains:

- authenticated PRW application session -> requester logical identity;
- strict target `DeviceId` -> nominated logical target identity;
- `TransportIdentity` -> lower transport evidence only;
- IP/port -> transient reachability only.

No request ID, stream handle, transport handle, PID, UID, GID, IP, or port may fabricate PRW logical identity.

## 7. Historical ER isolation

The historical C03e-ER one-shot requester seam remains unchanged and uninvoked.

C03e-EZ must not widen its return contract or activate it. The new response-stream custody handoff is specific to the ET -> EV -> EX deterministic single-owner lineage selected by C03e-EY.

## 8. Continuation remains uninvoked

C03e-EZ does not call any requester-rendezvous registration/execution seam.

In particular it does not:

- invoke C03e-DV;
- invoke C03e-DN directly;
- perform C03e-DI registry validation;
- obtain C03e-DP requester policy;
- authorize requester/rendezvous start;
- mutate requester/rendezvous live provider state.

Those operations remain a separately gated continuation checkpoint.

## 9. Response semantics remain gated

C03e-EZ preserves the exact stream only. It does not define or implement:

- requester success payload bytes;
- requester rejection payload bytes;
- requester response operation tags;
- `Response` versus `Error` outer-kind mapping;
- external error disclosure policy;
- requester result codec;
- response frame construction;
- response frame write;
- stream shutdown after response;
- response write retry/replay/idempotency.

Candidate-publication result codecs remain explicitly out of scope and are not reused by analogy.

## 10. Failure and close ownership

No new peer-close code or reason is introduced.

Existing capability-specific termination/shutdown diagnostics remain capability-specific and are not widened to requester/rendezvous continuation.

No requester failure is converted into fabricated success, retry, replacement stream, replacement session, or peer-close behavior.

## 11. Serialization and backpressure

C03e-EZ preserves strict single-owner serialization:

- one accepted control stream at a time;
- one bounded first read at ET;
- one EV transaction at a time;
- requester handoff terminates EX before another accept;
- no queue;
- no detached provider task;
- no detached response writer;
- no speculative pre-accept.

Backpressure remains the absence of another accept while requester custody is outstanding.

## 12. Expected source surface

The intended source delta is limited to:

1. `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
   - materialize typed requester request + exact same stream custody;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - materialize a crate-private ET/EV/EX requester handoff owner without changing historical ER correlation alias semantics;
3. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
   - thread ET requester custody through EV and EX without invoking continuation;
4. this contract.

No dependency, workflow, Android, packaging, systemd, bootstrap, listener, or `main.rs` mutation is selected.

## 13. Explicit exclusions

C03e-EZ does not materialize or activate:

- C03e-DV invocation;
- requester/provider execution;
- requester response codec/schema/construction/write;
- response close/retry/replay/idempotency;
- candidate selection;
- transport selection;
- reachability resolution;
- relay allocation;
- dialing;
- repeated-loop resume after requester handoff;
- second acceptor or second read;
- task spawning;
- runtime/listener/bootstrap activation;
- Agent `main.rs` activation;
- dependency/workflow changes;
- Android application changes;
- packaging/systemd changes;
- deployment/restart/recovery;
- merge.

## 14. Validation and closure

Closure requires:

- exact C03e-EY merge base;
- changed-path ceiling matching the selected source surface;
- exact-final-head Rust validation FULL PASS;
- Android validation only if triggered/applicable, with no untriggered PASS claim;
- immutable Drive audit with raw byte-exact readback;
- rolling Drive append from exact post-EY predecessor with prefix and suffix verification;
- PR kept draft/open/unmerged.

Closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_RESPONSE_STREAM_CUSTODY_SOURCE_MATERIALIZATION`

Gate marker:

`C03E_EZ_REQUESTER_RENDEZVOUS_RESPONSE_STREAM_CUSTODY_SOURCE_MATERIALIZED`
