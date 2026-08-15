# Local Agent Status Decode-Before-Completion Contract

Status: Phase 025 locked baseline

## Purpose

Define the command-specific state-transition ordering for a successful `GetAgentStatus` response read from a generic byte stream. A response MUST be fully acquired and command-specifically decoded before its request ID may be removed from the connection-local outstanding-request tracker.

## Required operation order

Processing MUST occur in this exact order:

1. acquire one complete validated frame through the Phase 011/024 generic read path;
2. validate the terminal frame and fully decode the Phase 023 `GetAgentStatus` response, including its fixed five-byte status body;
3. obtain the typed request ID only from that fully decoded result;
4. complete exactly that ID through `LocalRequestTracker::complete()`;
5. return the decoded status result only after tracker completion succeeds.

## State-preservation rule

Tracker state MUST remain unchanged when any failure occurs before step 4, including:

- truncated header;
- invalid header;
- truncated payload;
- terminal kind/status mismatch;
- valid terminal non-success response;
- malformed Agent status body;
- unknown runtime-state identifier;
- unsupported embedded protocol version.

A fully decoded response whose request ID is not currently outstanding MUST fail as `UnknownRequestId` and MUST NOT alter any unrelated outstanding request.

## Relationship to Phase 021

Phase 021 remains the generic terminal-response completion primitive and is valid for protocol paths where terminal validation is sufficient before completion.

Phase 025 intentionally uses a stricter command-specific ordering for `GetAgentStatus`: successful command-body decoding is an additional prerequisite before tracker mutation. Phase 025 does not weaken or rewrite Phase 021.

## Exactly-once behavior

After one fully valid known response is successfully decoded and completed, replay of the same response without re-registering its request ID MUST fail as `UnknownRequestId`.

## Error separation

The Phase 025 composition MUST keep these categories distinct:

- stream/frame acquisition or command-specific decode failure; and
- outstanding-request tracker transition failure.

## Runtime boundary

Phase 025 adds no:

- Unix socket creation, bind, listen, accept, or connect;
- peer-credential access;
- async runtime/task/thread policy;
- timer or cancellation scheduling;
- command dispatch or execution;
- live Agent state collection;
- new dependency;
- filesystem/network/DNS mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic private-key operation;
- systemd activation;
- database or deployment.

## Explicit deferrals

Still deferred:

- timeout/cancellation and late-response semantics;
- command-specific error body schema;
- bounded private-DNS response body and codec;
- live runtime status collection;
- runtime command dispatch;
- Unix socket runtime and SO_PEERCRED enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
