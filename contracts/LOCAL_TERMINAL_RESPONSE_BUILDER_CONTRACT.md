# Local Terminal Response Frame Builder Contract

Status: Phase 022 locked baseline

## Purpose

Define the encode-side construction boundary for a complete local terminal response frame. The builder MUST create outer message kind and response status together so callers cannot choose an inconsistent pair.

## Inputs

The builder accepts:

- an existing typed, non-zero `LocalIpcRequestId`;
- a typed `LocalAgentResponseStatus`;
- opaque command-specific body bytes.

The caller does not provide an outer message kind.

## Kind derivation

The builder MUST derive the outer message kind through the existing Phase 020 mapping:

- `Ok` -> `Response`
- every current non-`Ok` status -> `Error`

The builder MUST NOT expose an independent kind argument.

## Payload composition

The complete payload is:

```
response_status_prefix || command_specific_body
```

The status prefix MUST be produced by the existing Phase 016 response-status encoder.

The command-specific body remains opaque to Phase 022.

## Bounds

The global local IPC payload limit remains 1,048,576 bytes.

Because every terminal payload reserves exactly two bytes for the response-status prefix, the maximum Phase 022 command-specific body length is:

```
1,048,576 - 2 = 1,048,574 bytes
```

Bodies above that bound MUST be rejected before complete frame construction.

## Lower-level constructors

After composition, Phase 022 MUST use the existing validated constructors:

- `LocalIpcPayload::new()`;
- `LocalIpcFrameHeader::new()` with the current protocol version;
- `LocalIpcFrame::new()`.

Lower-level failures remain typed rather than being silently ignored.

## Round-trip invariant

Every frame successfully produced by the Phase 022 builder MUST pass the Phase 020 terminal-response validator and return the same request ID and response status.

## Security and runtime boundary

Phase 022 adds no:

- socket bind/listen/accept/connect operation;
- command dispatch or execution;
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

- command-specific error body schema;
- complete `GetAgentStatus` frame builder using its five-byte typed body;
- bounded private-DNS response body and codec;
- timeout/cancellation policy;
- live runtime status collection;
- runtime command dispatch;
- Unix socket runtime and peer-credential enforcement;
- privileged-helper protocol;
- crypto-provider selection;
- remote control-plane protocol.
