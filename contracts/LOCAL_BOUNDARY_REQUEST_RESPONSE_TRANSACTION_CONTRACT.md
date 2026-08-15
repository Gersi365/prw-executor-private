# Private Remote Workspace Boundary-Aware Request/Response Transaction Contract

Version: `0.1.0`

Status: Phase 049 provider-neutral guarded response-write composition

## Scope

Phase 049 composes the Phase 048 boundary-aware policy processor with the existing Phase 041 guarded terminal-response writer over caller-supplied `std::io::Read` and `std::io::Write` objects.

It owns no socket or transport.

## Ordering

The ordering is strict:

1. reject an already `WritePoisoned` response-write state before consuming input;
2. perform the Phase 048 boundary-aware Request/policy processing;
3. if the outcome is `CleanEof`, return a clean stop outcome without response I/O;
4. if the outcome is `Response(frame)`, validate/write it through the existing guarded response writer;
5. any generic response-write failure transitions the response-write state to `WritePoisoned` through Phase 041.

## Successful outcomes

- `CleanEof`: no Request existed at the next frame boundary and no response bytes were written;
- `ResponseWritten`: one complete valid Request produced one policy-gated terminal response that was written successfully.

## Failure behavior

- an already poisoned response-write state fails before input read, policy evaluation, or output;
- frame/Request/policy-response processing failures return before response writing and do not poison the response-write state;
- guarded response-write failures preserve the Phase 041 error taxonomy and poisoning semantics.

## Authentication boundary

The policy evaluator remains caller-supplied and is assumed to represent an already authenticated policy context. Phase 049 does not authenticate a principal.

## Forbidden interpretation

Phase 049 does not authorize or implement:

- Unix socket bind/listen/accept/connect/close;
- peer credentials or authentication;
- XDG runtime-path mutation;
- inbound read-poisoning integration;
- multi-request loops;
- timers, concurrency, cancellation, or task/thread creation;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
