# Phase 051 — Boundary-Aware Server Connection State

## Objective

Extend the existing Phase 044 aggregate connection state so it can consume the Phase 050 clean-EOF-aware processing path without introducing a second state model.

## Data flow

Aggregate usability precheck → Phase 050 boundary inbound guard → either `CleanEof`, `ResponseWritten`, or a typed failure whose component state has already been updated where appropriate.

## Repeated-call semantics

The new entry point processes exactly one boundary attempt per call. A future runtime may invoke it repeatedly, but Phase 051 itself owns no loop. This keeps lifecycle control explicit and testable.

## State rules

- clean EOF leaves aggregate state usable;
- successful Request/response leaves aggregate state usable;
- framing/Request decode failure makes aggregate reason `InboundRead`;
- response write failure makes aggregate reason `ResponseWrite`;
- any pre-existing aggregate unusable reason rejects before input read, policy evaluation, or output.

## Compatibility

The existing Phase 044 non-boundary entry point remains available and unchanged. Phase 051 adds a separate boundary-aware entry point on the same `LocalServerConnectionState`.

## Runtime boundary

No socket, file descriptor, peer credential, filesystem pathname, internal loop, timer, task/thread, authentication, systemd activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
