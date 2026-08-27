# Phase 152 C03e-CO — Candidate Publication Post-Authentication PRWC Command Receive Seam Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Target gate:
`C03E_CO_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_RECEIVE_SEAM_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-CN is the authoritative predecessor:
- branch: `phase-152-c03e-cn-candidate-publication-post-auth-prwc-command-execution-seam-selection-staging`
- head: `199f57d7bdc1347eb497f00bcd62b0f9f5d83ecf`
- tree: `29cad0002aa7f2984f9f4af8e9bde2a209c2a393`
- gate: `C03E_CN_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_EXECUTION_SEAM_SELECTED`
- PR #211: body `Status: CLOSED`, draft/open/unmerged.

C03e-CO materializes only the CN-selected bridge-owned receive seam. It does not alter CN authority selection, C03e-CK authentication semantics, C03e-CJ locally-originated request-ID custody, C03e-CM requester/rendezvous carrier semantics, or existing candidate/reachability authorities.

## 2. Materialized source responsibility

`AuthenticatedPrwcConnection` remains sole owner of its private `ControlTlsServerStream` and now exposes one narrow method equivalent to:

```text
receive_candidate_publication_command(
    &mut self
) -> Result<CandidatePublicationControlFrame, AuthenticatedPrwcCommandReceiveError>
```

The method:
- performs exactly one existing bounded `ControlTlsServerStream::read_frame()` per non-terminal call;
- passes that frame directly to `decode_candidate_publication_control_frame(...)`;
- returns the existing typed `CandidatePublicationControlFrame` on success;
- never exposes the raw stream, socket, rustls state or listener;
- never performs a response write, retry, reconnect, frame loop, accept loop or background task;
- never calls requester/rendezvous, publication, reachability-owner or durable-store authority.

## 3. Private fail-closed receive state

The authenticated connection now owns a private candidate-publication receive state with exactly two meanings:
- `Ready` — one bounded receive attempt may be made;
- `Terminal` — no further read is permitted through this seam.

Selected/materialized transition semantics:
- successful strict candidate-publication decode leaves the state `Ready` for a later separately gated sequential receive;
- any `ControlFrameError` terminalizes the receive state before returning an error;
- any `CandidatePublicationControlFrameError` terminalizes the receive state before returning an error;
- once terminal, later calls return `AuthenticatedPrwcCommandReceiveError::Terminal` without another frame read.

This state is protocol-safety state only. It is not identity, requester authority, freshness authority, request correlation or a durable lifecycle token.

## 4. Receive error surface

C03e-CO materializes `AuthenticatedPrwcCommandReceiveError` with three fail-closed classes:
- `Frame(ControlFrameError)`;
- `Command(CandidatePublicationControlFrameError)`;
- `Terminal`.

No provider, registry, requester/rendezvous, reachability-owner, publication freshness or durable commit errors are added to this receive surface.

## 5. Inbound request-ID custody remains unchanged

Peer-originated candidate-publication PRWC `request_id` remains outer correlation carried by `CandidatePublicationControlFrame`.

C03e-CO does not:
- allocate a replacement locally-originated request ID;
- insert the inbound ID into `PrwcRequestIdLifecycle`;
- complete that ID through `PrwcRequestIdLifecycle`;
- treat the ID as requester, publisher, replay, freshness or rendezvous authority.

C03e-CJ remains exclusively the lifecycle for locally-originated PRWC request IDs.

## 6. Authenticated publisher authority remains separate

The returned candidate-publication frame contains structural/correlation data only. Publisher logical authority remains the existing `AuthenticatedDeviceSession` held by `AuthenticatedPrwcConnection`.

C03e-CO does not derive publisher identity from PRWP payload, request ID, candidate data, transport bytes or `TransportIdentity`.

## 7. Focused deterministic source validation

Before candidate attachment, an isolated probe was used only to obtain canonical rustfmt/Clippy/test-authored source bytes. The probe is not part of C03e-CO lineage.

Authoritative successful probe evidence:
- recognized existing corrective-workflow carrier run: `33084744121`;
- job: `98561216111`;
- source generation: SUCCESS;
- canonical formatting + exact one-source-path guard: SUCCESS;
- focused Clippy: SUCCESS;
- focused tests: SUCCESS;
- bot source-only commit: `c57c0c63a204af5fd59550939d37b8c0211b35d6`;
- canonical source blob: `1af3aa2851e87e3a4f7990c98e105e62141d8db1`.

Two earlier newly-created probe workflow files failed at workflow startup with zero jobs. They produced no source commit and are not source/compile/test failures. None of those workflow files are authorized in the C03e-CO candidate tree.

## 8. Focused tests materialized inline

The source materialization includes focused inline tests proving at least:
- one successful candidate-publication Command consumes exactly one frame and preserves the peer-originated request ID;
- frame-read failure terminalizes the receive side and a later call performs no additional read;
- strict candidate-publication decode failure terminalizes the receive side and a later call performs no additional read.

No new test file is required.

## 9. Exact source/diff boundary

The exact CN -> C03e-CO candidate diff is authorized to contain only these two paths:

1. `crates/prw-remote-bridge/src/prwc_connection_authentication.rs`
2. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CO_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_RECEIVE_SEAM_SOURCE_MATERIALIZATION_STAGING.md`

Any third path blocks C03e-CO attachment/closure and requires a fresh audit before mutation.

In particular C03e-CO authorizes no change to:
- `crates/prw-remote-bridge/src/root.rs`;
- any `Cargo.toml`;
- root or Android `Cargo.lock`;
- `candidate_publication_control_frame.rs`;
- requester/rendezvous provider/carrier source;
- candidate reachability or reachability-owner source;
- workflow files;
- Agent/Desktop/Android application source;
- runtime/network/deployment configuration.

## 10. Dependency/lock invariants

No dependency is added or moved.

The following predecessor lock blobs must remain byte-stable:
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`.

No lock regeneration is selected.

## 11. Explicitly excluded behavior

C03e-CO does not:
- expose or transfer the raw authenticated stream;
- write candidate-publication success/failure responses;
- define Response versus Error envelopes;
- execute `publish_current_candidates(...)`;
- call `RequesterRendezvousAuthorityProvider`;
- create or consume an `AuthorizedRequesterRendezvous`;
- validate publication freshness;
- commit reachability candidates;
- instantiate a concrete requester/rendezvous provider;
- bind/listen/accept connections;
- start a frame loop, retry loop, async task or product runtime;
- activate networking, deployment or merge.

## 12. Canonical validation requirement

Probe validation is pre-attachment evidence only and does not replace canonical exact-head repository validation.

C03e-CO may close only after:
- exact CN predecessor remains unchanged;
- CN -> CO compare is ahead-only with exact CN merge base and exactly the two authorized paths;
- canonical Rust validation on exact final CO head reaches terminal SUCCESS for locked graph, formatting, Clippy, tests and workspace build;
- canonical Android validation, if automatically triggered by the source delta, reaches terminal SUCCESS for native adapter and Android application;
- skipped workflows are recorded as SKIPPED, never PASS;
- root and Android lock blobs remain byte-stable;
- PR remains draft/open/unmerged;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive predecessor is race-checked and preserved byte-for-byte as prefix;
- PR body changes `STAGED -> CLOSED` only after durable Drive closure.

## 13. Safe successor rule

After durable C03e-CO closure, no execution/provider/runtime checkpoint is implied automatically.

A fresh read-only audit must choose the next prerequisite between:
- provider-neutral candidate-publication execution composition using the already-materialized C03e-CM authority port; and
- concrete requester/rendezvous provider lifecycle/representation materialization required before production runtime activation.

Any execution composition must continue to preserve authenticated publisher identity, one-shot requester/rendezvous authority, publication freshness and existing durable candidate compare-and-commit ordering. No successor may jump directly to listener/runtime activation, production networking, deployment or merge.

## 14. Completion meaning

Closure means only that the CN-selected post-authenticated one-frame candidate-publication Command receive seam exists as validated Rust source while keeping the raw stream private and fail-closed.

It does not mean candidate publication is authorized or executed, requester/rendezvous state has a concrete provider, a response is written, reachability is mutated, a listener is active, runtime is wired, or anything is deployed.

Target gate:
`C03E_CO_CANDIDATE_PUBLICATION_POST_AUTH_PRWC_COMMAND_RECEIVE_SEAM_SOURCE_MATERIALIZED`
