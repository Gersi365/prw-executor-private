# Private Remote Workspace Boundary-Aware Server Connection-State Contract

Version: `0.1.0`

Status: Phase 051 provider-neutral aggregate boundary composition

## Scope

Phase 051 extends the existing Phase 044 `LocalServerConnectionState` with a boundary-aware processing entry point backed by the Phase 050 inbound guard.

It does not create a second or parallel connection-state model.

## Aggregate precheck

Before any boundary read, the existing aggregate state is checked through its current unusable-reason classification.

If either protocol direction is already poisoned:

- zero input bytes are consumed;
- zero policy evaluations occur;
- zero response bytes are written;
- the existing `InboundRead`, `ResponseWrite`, or `Both` reason is returned.

## Successful outcomes

- `CleanEof`: orderly peer close before any new-frame byte; aggregate state remains healthy;
- `ResponseWritten`: one complete Request passed through boundary read, Request decode, policy gate, and guarded response write successfully; aggregate state remains healthy.

## Failure state ownership

- framing or Request-decoding failure transitions inbound state to `ReadPoisoned` through Phase 050, making aggregate reason `InboundRead`;
- response-write failure transitions response state to `WritePoisoned`, making aggregate reason `ResponseWrite`;
- response construction failures do not falsely poison inbound framing state;
- clean EOF never makes aggregate state unusable.

## Repeated invocation

Repeated calls may process consecutive frames from the same generic `Read`/`Write` pair and eventually return `CleanEof`. Phase 051 does not itself implement an internal loop.

## Authentication boundary

The policy evaluator remains caller-supplied and is assumed to represent an already authenticated policy context. Phase 051 does not perform peer authentication.

## Forbidden interpretation

Phase 051 does not authorize or implement:

- Unix socket bind/listen/accept/connect/close;
- `SO_PEERCRED` retrieval or principal authentication;
- XDG runtime-path mutation;
- an internal multi-request loop;
- timers, concurrency, cancellation, or task/thread creation;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
