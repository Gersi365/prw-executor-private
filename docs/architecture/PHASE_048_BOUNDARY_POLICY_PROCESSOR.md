# Phase 048 — Boundary-Aware Policy Processor

## Objective

Carry the Phase 047 clean-EOF distinction through policy-gated response construction without introducing response I/O or runtime transport.

## Data flow

`std::io::Read` → Phase 047 boundary Request reader → either `CleanEof` or decoded Request → existing Phase 038 policy-gated response builder.

## Outcomes

- `CleanEof`: no Request existed and the policy evaluator is not called.
- `Response(frame)`: one complete decoded Request passed through the existing Allow/Deny policy path and produced one correlated terminal frame in memory.

## Failure isolation

Boundary/framing/Request-decode failures remain `LocalRequestProcessorError::Request`. Defensive response-construction failures remain `LocalRequestProcessorError::Response`.

No malformed or truncated input is converted to `CleanEof`.

## Authentication boundary

The policy evaluator is caller-supplied and must represent an already authenticated policy context in a future runtime. This layer adds no identity authentication.

## Runtime boundary

No socket, file descriptor, response write, peer credential, filesystem path, timer, task/thread, systemd activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
