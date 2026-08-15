# Local Terminal Response Frame Contract

Status: Phase 020 locked baseline

## Purpose

Define the invariant between an already validated local IPC frame's outer message kind and the common terminal response-status prefix. This phase does not create a transport runtime or a command dispatcher.

## Authoritative prerequisite invariants

Phase 020 operates only on `LocalIpcFrame`, whose existing constructors already require:

- a supported local IPC protocol version;
- a non-zero typed request identifier;
- a globally bounded payload;
- an exact match between the frame header's declared payload length and the actual in-memory payload length.

Phase 020 MUST reuse those invariants and MUST NOT duplicate their validation logic.

## Terminal response kind/status mapping

Exactly one outer message kind is valid for each terminal response status:

- `LocalAgentResponseStatus::Ok` -> `LocalIpcMessageKind::Response`
- every current non-`Ok` response status -> `LocalIpcMessageKind::Error`

Therefore:

- `Response` carrying a non-`Ok` status is invalid;
- `Error` carrying `Ok` is invalid;
- `Request` is never a terminal response and is invalid at this boundary.

## Status decoding

The response status MUST be decoded through the existing Phase 016 common response-status prefix codec. Phase 020 MUST NOT define a second status-code mapping.

Missing or unknown status prefixes fail closed.

Any command-specific body after the status prefix remains opaque to this validator.

## Request correlation

A validated terminal response exposes the existing frame header's typed, non-zero request ID unchanged. This phase does not create a new request ID and does not perform outstanding-request lookup; the Phase 013 request tracker remains the separate correlation-state component.

## Validation ordering

The Phase 020 validator MUST:

1. reject outer `Request` frames before interpreting the payload as a response;
2. decode the common response-status prefix;
3. derive the only valid terminal message kind for that status;
4. reject a kind/status mismatch;
5. return the existing request ID and decoded status only after all checks pass.

## Security and runtime boundary

Phase 020 adds no:

- socket bind/listen/accept/connect operation;
- live command dispatch or execution;
- outstanding-request mutation;
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

- combining terminal-frame validation with the Phase 013 outstanding-request tracker;
- command-specific error body schema;
- bounded private-DNS response body and codec;
- live runtime status collection;
- runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- timeout/cancellation policy;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
