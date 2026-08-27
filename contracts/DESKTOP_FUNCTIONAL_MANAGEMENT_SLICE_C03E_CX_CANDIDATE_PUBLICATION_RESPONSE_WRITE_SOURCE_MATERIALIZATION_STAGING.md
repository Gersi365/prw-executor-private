# Phase 152 C03e-CX — Candidate Publication Response Write Source Materialization — STAGING

Target gate: `C03E_CX_CANDIDATE_PUBLICATION_RESPONSE_WRITE_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-CW is the exact durable predecessor:

- branch: `phase-152-c03e-cw-candidate-publication-response-write-composition-selection-staging`
- head: `0ab759c66b282fd5f32812bddc7182d68712ef6d`
- tree: `fa06d83075ffb6afedbab10e0d04cccd112226f6`
- PR #220: `Status: CLOSED`, draft/open/unmerged

No mutation outside this contract and `crates/prw-remote-bridge/src/prwc_connection_authentication.rs` is authorized by C03e-CX.

## Purpose

Materialize the CW-selected one-shot candidate-publication terminal result write seam on the existing authenticated PRWC connection without creating a command loop, exposing the raw stream, selecting provider/runtime ownership, changing request-ID authority, or activating networking/runtime behavior.

C03e-CX is source composition only. It joins already-existing pieces:

1. the authenticated PRWC connection that privately owns `ControlTlsServerStream`;
2. the existing candidate-publication decoded `CandidatePublicationControlFrame`;
3. the existing semantic execution result `Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>`;
4. the C03e-CV pure `encode_candidate_publication_execution_result_frame` helper; and
5. the existing bounded `ControlTlsServerStream::write_frame` operation.

## Authorized source paths

Exactly two paths may differ from C03e-CW:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CX_CANDIDATE_PUBLICATION_RESPONSE_WRITE_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-remote-bridge/src/prwc_connection_authentication.rs`

No new Rust module is authorized. No `root.rs`, Cargo manifest, lockfile, workflow, database, runtime/listener, networking, deployment or unrelated path may change.

## Materialized public operation

`AuthenticatedPrwcConnection` may gain one public operation equivalent to:

```text
write_candidate_publication_result(
    &mut self,
    command: &CandidatePublicationControlFrame,
    result: Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>,
) -> Result<(), AuthenticatedPrwcCandidatePublicationResultWriteError>
```

The exact Rust name may follow existing module naming conventions, but the semantic boundary must remain exactly this one-shot write composition.

## Error surface

The source must distinguish at least:

- pure result-frame composition failure before stream I/O;
- actual bounded frame write failure; and
- candidate-publication I/O already terminal before composition/write.

The error surface must retain underlying typed causes where existing code already exposes them.

## Required ordering

For every call:

1. inspect candidate-publication I/O terminal state first;
2. if already terminal, fail immediately before result projection/frame composition and before stream I/O;
3. compose the terminal frame using C03e-CV `encode_candidate_publication_execution_result_frame`;
4. if composition fails, return the pure composition error without changing candidate-publication I/O state;
5. invoke the private bounded server-stream `write_frame` exactly once;
6. if that write fails, mark candidate-publication I/O terminal before returning the write failure;
7. if that write succeeds, return success while leaving candidate-publication I/O state ready.

No later stage may run after an earlier failure.

## Shared terminal state

The same private candidate-publication terminal state used by the existing one-shot receive seam must govern result writes.

Therefore:

- a prior receive frame/decode failure blocks later result writes before I/O;
- a result write failure blocks later candidate-publication reads before I/O;
- a result write failure blocks later result writes before composition and I/O;
- a successful write does not terminalize the state by itself.

Renaming the private state representation is not required. No public connection-state API is selected.

## Request-ID custody

C03e-CX must not allocate, register, retire, reserve, abandon, or otherwise mutate `PrwcRequestIdLifecycle` for candidate-publication result writes.

The only correlation used is the exact peer-originated `command.request_id()` already consumed by the C03e-CV pure framing helper.

## No semantic re-execution

The write operation receives an already-completed semantic execution result. It must not:

- call `execute_authenticated_candidate_publication`;
- consult requester/rendezvous authority;
- consult or mutate the reachability owner;
- revalidate registry state;
- issue freshness;
- inspect candidates for admission;
- reinterpret internal execution errors beyond the existing C03e-CV generic projection.

## Required focused tests

The source materialization must include focused tests proving at least:

1. successful composition writes exactly one frame and preserves the exact decoded Command request ID;
2. successful write leaves candidate-publication I/O ready;
3. already-terminal state blocks composition and write before any stream I/O;
4. pure composition failure performs zero writes and leaves state ready;
5. actual write failure performs exactly one write attempt, returns the typed write failure and terminalizes candidate-publication I/O;
6. after write failure, a later result write is blocked before composition/I/O;
7. after write failure, a later candidate-publication read is blocked before read I/O;
8. the write seam accepts only an already-completed semantic execution result and does not own semantic execution;
9. no test or implementation introduces a retry/loop.

Tests may use private fake I/O traits/functions in this source file. No production raw-stream accessor is authorized.

## Explicit non-goals

C03e-CX does not select or materialize:

- malformed-command response mapping before a valid decoded candidate-publication Command exists;
- fallback Error emission after an ambiguous write failure;
- retry/reconnect behavior;
- command receive/write loops;
- connection keepalive or close policy;
- public raw-stream exposure;
- runtime/process ownership of `InMemoryRequesterRendezvousAuthorityProvider`;
- mutex/actor/shared singleton/global provider state;
- provider registration ingress;
- reachability-owner lookup/routing;
- listener/accept-loop wiring;
- database/schema/persistence changes;
- TTL/clock/background cleanup;
- credentials/bootstrap changes;
- production networking;
- deployment;
- merge.

## Validation gates

Closure requires:

- exact CW parent lineage;
- final compare limited to the two authorized paths;
- canonical Rust validation FULL PASS;
- Android validation PASS if automatically triggered by the source path;
- root and Android native `Cargo.lock` blobs byte-stable;
- no pending/failing automatically-triggered exact-final-head workflows;
- immutable Drive audit raw-readback exact;
- rolling Drive predecessor guard + append-only prefix proof;
- PR remains draft/open/unmerged.

Any source correction must stay inside the authorized source path and be strictly diagnostic-driven.

## Safe successor

After durable C03e-CX closure, perform a fresh exact-head prerequisite audit. Runtime/process ownership of the bounded in-memory requester/rendezvous provider remains a separate gate and must not be conflated with connection command-loop/listener activation.
