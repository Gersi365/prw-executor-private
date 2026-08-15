# Local Admission Visibility Contract

## Status

Phase 039 hardens the Rust API boundary around local command admission and terminal response construction.

## Crate boundary

The following modules are Agent-internal implementation surfaces and must not be publicly reachable through the `prw-agent` library API:

- `local_commands::admission`;
- `local_commands::responder`;
- `local_commands::policy_response`.

They are exposed only as `pub(crate)` modules.

## Rationale

Phase 037/038 intentionally separate authentication from policy evaluation, but a public library caller could otherwise supply an arbitrary `PolicyEvaluator` and directly invoke the admission/response pipeline outside the future authenticated Agent runtime.

Making the pipeline crate-internal does not itself implement authentication. It narrows the API so only code compiled inside `prw-agent` can compose that pipeline, allowing the future runtime authentication layer to own the external admission boundary.

## Public protocol/domain types

This visibility hardening does not change the existing public wire/domain types, frame codecs, request/response envelopes, or policy capability enum. It only narrows the Agent-side admission and successful-response orchestration modules.

## Runtime boundary

Phase 039 performs no socket I/O, authentication, peer-credential lookup, host-state acquisition, policy persistence, DNS/network mutation, database work, systemd activation, task/thread creation, deployment, or private-key operation.
