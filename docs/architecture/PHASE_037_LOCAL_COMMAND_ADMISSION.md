# Phase 037 — Local Command Policy Admission

## Objective

Prevent the pure successful responder from becoming an implicit authorization policy by requiring an explicit typed policy-admission token.

## Composition boundary

The intended provider-neutral ordering is:

`decoded Request -> authenticated-principal-bound policy context -> capability evaluation -> LocalPolicyAdmittedRequest -> successful responder`

Phase 037 implements only the capability-evaluation/token portion. Authentication and principal-to-policy-context binding remain future runtime prerequisites.

## Exact capability mapping

- `GetAgentStatus` requires `Capability::AgentStatusRead`.
- `GetPrivateDnsConfig` requires `Capability::PrivateDnsConfigRead`.

The mapping is deliberately command-specific. Local status/private-DNS reads are not filesystem reads and are not mapped to `FilesRead`.

## Token construction

`LocalPolicyAdmittedRequest` has no public raw constructor. The current module creates it only when the supplied `PolicyEvaluator` returns `Decision::Allow` for the exact command capability.

The successful read-only responder now requires this token, removing its previous unchecked `LocalAgentRequestEnvelope` entry point.

## Denial handling

Phase 037 returns a typed `Denied` admission error. It deliberately does not yet construct or write an `Unauthorized` response frame; that translation belongs to the next pure protocol-composition layer.

## Validation model

Tests prove:

- exact status and private-DNS capability mapping;
- an evaluator allowing status does not also allow private-DNS read;
- deny-all cannot produce an admission token;
- admitted token preserves request ID and command;
- existing successful responder tests obtain a policy-admitted token before response construction.

## Explicit deferrals

Still deferred:

- principal authentication;
- `SO_PEERCRED` acquisition/enforcement;
- user/device identity binding;
- policy persistence/loading;
- deny-to-terminal-error response composition;
- live socket request loop;
- runtime state acquisition.
