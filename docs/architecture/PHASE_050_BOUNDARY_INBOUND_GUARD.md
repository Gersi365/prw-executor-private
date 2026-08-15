# Phase 050 — Boundary-Aware Inbound Guard

## Objective

Carry the Phase 043 inbound `ReadPoisoned` safety model across the Phase 049 clean-EOF-aware transaction without treating orderly EOF as a protocol failure.

## Data flow

Inbound-state precheck → Phase 049 boundary-aware request/response transaction → preserve clean EOF or classify transaction failure.

## State rules

- `CleanEof`: inbound state stays healthy; response-write state stays unchanged.
- successful response: both states remain healthy.
- framing/Request decode failure: inbound state becomes `ReadPoisoned`; response state remains unchanged.
- response-write failure: inbound state stays healthy; response state is poisoned by the existing guarded writer.
- pre-existing inbound poison: reject before input read, policy evaluation, or response write.

## Why the error match is structural

Phase 049 already preserves request-processing errors as `Processing(LocalRequestProcessorError::Request(...))`. Phase 050 matches exactly that branch rather than duplicating frame-error knowledge. This keeps state ownership aligned with the existing error taxonomy.

## Runtime boundary

No socket, file descriptor, peer credential, filesystem pathname, multi-request loop, timer, task/thread, authentication, systemd activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
