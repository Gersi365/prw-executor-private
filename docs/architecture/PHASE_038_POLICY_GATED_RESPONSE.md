# Phase 038 — Policy-Gated Terminal Response Composition

## Objective

Close the pure protocol gap between Phase 037 policy admission and terminal response construction without introducing live transport or authentication implementation.

## Composition

`build_policy_gated_read_only_response()` receives:

- one already-decoded `LocalAgentRequestEnvelope`;
- one caller-supplied `PolicyEvaluator` context;
- caller-supplied status/private-DNS snapshots.

It evaluates the exact Phase 037 capability. `Allow` enters the token-gated successful responder. `Deny` builds a correlated `Unauthorized` terminal error through the existing Phase 022 terminal builder.

## Safety properties

- denied requests never receive an admission token;
- denied requests never enter the successful responder;
- request correlation is preserved on both success and denial;
- the terminal builder remains the sole authority for Response/Error kind derivation;
- a capability for one read command cannot authorize the other command;
- no alternate status/error serializer is introduced.

## Authentication caveat

This layer is only policy-gated, not authenticated. A live runtime must authenticate a peer first and pass the policy evaluator appropriate to that authenticated context. Phase 038 cannot infer or establish identity.

## Validation model

Tests prove both allowed success paths, deny-all -> correlated Unauthorized Error, and cross-capability denial.

## Explicit deferrals

Still deferred:

- local peer authentication / `SO_PEERCRED` enforcement;
- principal-to-policy binding implementation;
- raw stream request processing;
- malformed-request error response policy;
- live host snapshot acquisition;
- socket response writing;
- concurrency, timeout, cancellation, and retry.
