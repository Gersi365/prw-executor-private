# Local Policy-Gated Response Contract

## Status

Phase 038 composes typed local command policy admission with correlated terminal response construction.

## Required ordering

For one already-decoded local read-only Request:

1. determine the exact command capability through Phase 037;
2. evaluate the caller-supplied policy context;
3. on `Allow`, obtain `LocalPolicyAdmittedRequest` and delegate to the existing successful responder;
4. on `Deny`, do not construct an admission token and instead build a correlated terminal `Unauthorized` error frame.

## Allowed path

The allowed path reuses the Phase 037 token-gated responder and existing command-specific successful response builders. No new success serialization is introduced.

## Denied path

The denied path uses the existing terminal-response builder with:

- the original request ID;
- status `Unauthorized`;
- outer frame kind derived as `Error` by the existing terminal builder;
- no command-specific error body.

The resulting payload is only the existing two-byte Unauthorized status prefix.

## Capability isolation

Granting one local read capability does not grant the other. A mismatched capability decision therefore follows the denied path.

## Authentication boundary

The evaluator passed to Phase 038 is assumed to have been selected/bound by a future runtime after authenticating the peer/principal. Phase 038 itself performs no authentication and must not be interpreted as proving principal identity.

## Runtime boundary

Phase 038 performs no stream/socket I/O, peer-credential lookup, host-state acquisition, DNS/network mutation, tracker mutation, policy persistence, database access, task/thread creation, systemd activation, deployment, or private-key operation.
