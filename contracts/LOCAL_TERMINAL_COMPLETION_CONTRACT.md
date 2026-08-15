# Local Terminal Completion Contract

Status: Phase 021 locked baseline

## Purpose

Compose the validated Phase 020 terminal-response frame invariant with the Phase 013 connection-local outstanding-request tracker. This phase defines exactly-once completion state semantics without activating any socket runtime.

## Required operation order

Terminal response processing MUST occur in this order:

1. validate the frame through the Phase 020 terminal-response validator;
2. obtain the existing typed request ID only from the validated terminal response;
3. complete that request ID in the connection-local Phase 013 tracker;
4. return the validated terminal response only after tracker completion succeeds.

A terminal frame that fails Phase 020 validation MUST NOT mutate tracker state.

## Known-request requirement

A valid terminal response whose request ID is not currently outstanding MUST fail closed as a request-tracker error. It MUST NOT remove or alter another outstanding request.

## Exactly-once completion

Once a known request ID is successfully completed, it is removed from the outstanding set. Replaying the same terminal response without a new registration of that ID MUST fail as `UnknownRequestId`.

The Phase 013 rule permitting an ID to be registered again after successful completion remains unchanged.

## State mutation boundary

The only Phase 021 mutation is removal of exactly one known request ID through the existing `LocalRequestTracker::complete()` operation.

Phase 021 MUST NOT:

- register a request implicitly;
- clear the tracker on failure;
- consume a request for an invalid terminal frame;
- consume a different request when the supplied ID is unknown;
- alter response status or frame metadata.

## Error preservation

Phase 021 distinguishes:

- terminal-frame validation failures from Phase 020; and
- request-tracker transition failures from Phase 013.

These error categories remain typed and fail closed.

## Security and runtime boundary

Phase 021 adds no:

- socket bind/listen/accept/connect operation;
- timeout or cancellation scheduler;
- task/thread runtime;
- command dispatch or execution;
- filesystem mutation;
- serialization dependency;
- account authentication;
- privileged-helper invocation;
- DNS/network mutation;
- cryptographic private-key operation;
- systemd activation;
- database or deployment.

## Explicit deferrals

Still deferred:

- timeout/cancellation and late-response policy;
- command-specific error body schema;
- bounded private-DNS response body and codec;
- live runtime status collection;
- runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
