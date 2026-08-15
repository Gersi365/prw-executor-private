# Phase 036 — Pure Read-Only Command Responder

## Objective

Connect the already-validated request envelope to the already-validated successful response builders without activating runtime command execution.

## Composition

The pure responder receives an already-decoded request plus caller-supplied status/private-DNS snapshots.

For `GetAgentStatus` it delegates to Phase 023. For `GetPrivateDnsConfig` it delegates to Phase 028. Request correlation is inherited directly from the request envelope.

## Why snapshots are supplied

The responder deliberately does not inspect process state, files, DNS configuration, operating-system APIs, or network state. Separating state acquisition from protocol response composition keeps the current phase deterministic and testable and leaves future authorization/admission ordering explicit.

## Validation model

Focused tests decode the produced frames through the existing command-specific decoders and prove:

- status response preserves request ID and exact supplied status snapshot;
- private-DNS response preserves request ID and exact supplied bounded DNS snapshot;
- existing builders/codecs remain the only response serialization authority.

## Explicit deferrals

Still deferred:

- runtime state acquisition;
- peer authentication/authorization;
- policy evaluation at command admission;
- error-response mapping for denied/runtime-failed commands;
- socket read/write loop;
- concurrent request processing;
- cancellation/timeouts/retry.
