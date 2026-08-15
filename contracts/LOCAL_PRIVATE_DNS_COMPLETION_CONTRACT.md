# Local Private-DNS Decode-Before-Completion Contract

Status: Phase 030 locked baseline

## Purpose

Define the state-transition ordering for a successful streamed `GetPrivateDnsConfig` response. The bounded private-DNS body MUST be fully decoded before the corresponding request ID may be removed from the connection-local outstanding-request tracker.

## Required operation order

Processing MUST occur in this order:

1. acquire one complete validated frame through the Phase 029 stream read path;
2. validate terminal `Response + Ok` semantics;
3. fully decode and validate the bounded Phase 027 private-DNS body;
4. obtain the typed request ID only from the fully decoded result;
5. complete exactly that ID through `LocalRequestTracker::complete()`;
6. return the decoded private-DNS result only after completion succeeds.

## State-preservation rule

Tracker state MUST remain unchanged when any failure occurs before step 5, including:

- truncated/invalid generic frame data;
- terminal kind/status mismatch;
- valid terminal non-success response;
- reserved private-DNS flag bits;
- excessive counts;
- invalid/truncated entry lengths;
- invalid UTF-8;
- trailing command-body bytes;
- any defensive bounded snapshot invariant failure.

A fully decoded response whose request ID is not outstanding MUST fail as `UnknownRequestId` and MUST NOT alter any unrelated outstanding request.

## Exactly-once behavior

After one valid known response is successfully decoded and completed, replay without explicit re-registration of the request ID MUST fail as `UnknownRequestId`.

## Relationship to Phase 025

Phase 030 applies the same decode-before-completion safety rule already validated for `GetAgentStatus`, but uses the independent bounded private-DNS command decoder. Neither command path weakens the generic Phase 021 terminal-completion primitive.

## Error separation

The Phase 030 composition MUST distinguish:

- stream/frame acquisition or command-specific decode failures; and
- request-tracker transition failures.

## Runtime boundary

Phase 030 adds no Unix socket, peer-credential access, DNS mutation, command dispatcher, task/thread/timer runtime, dependency, privileged-helper invocation, authentication, private-key operation, systemd activation, database, or deployment.

## Explicit deferrals

Still deferred:

- resolver-address semantic validation;
- split-domain normalization/validation policy;
- command-specific error body schema;
- timeout/cancellation and late-response policy;
- live command dispatch;
- Unix socket runtime and peer-credential enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
