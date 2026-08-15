# Local Read-Only Responder Contract

## Status

Phase 036 composes the two currently admitted local read-only command envelopes with their already-validated successful response-frame builders.

## Inputs

The responder receives:

- one already-decoded `LocalAgentRequestEnvelope`;
- one caller-supplied `LocalAgentStatusSnapshot`;
- one caller-supplied bounded `LocalPrivateDnsSnapshot`.

The responder does not read live host state itself.

## Command mapping

- `GetAgentStatus` delegates to the existing Phase 023 successful status-frame builder.
- `GetPrivateDnsConfig` delegates to the existing Phase 028 successful private-DNS-frame builder.

No command code, response codec, or terminal-frame construction logic is duplicated.

## Correlation

The response frame must preserve the request ID from the input request envelope exactly.

## Admission boundary

Phase 036 does not authenticate, authorize, or policy-check the request. A future runtime admission layer must perform those checks before treating this pure responder as eligible to produce a successful response.

## Failure boundary

Any defensive lower-level response-build failure remains command-specific and typed. Phase 036 does not convert such a failure into a fabricated successful response.

## Runtime boundary

Phase 036 performs no socket I/O, host reads, DNS mutation, system calls, thread/task creation, timeouts, tracker mutation, retry, or deployment action.
