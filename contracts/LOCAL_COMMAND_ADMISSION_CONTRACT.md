# Local Command Admission Contract

## Status

Phase 037 locks an explicit policy-admission boundary between decoded local Requests and successful read-only response construction.

## Capabilities

The two currently admitted local read-only commands require distinct capabilities:

- `GetAgentStatus` -> `AgentStatusRead`;
- `GetPrivateDnsConfig` -> `PrivateDnsConfigRead`.

Neither command is mapped to the generic filesystem `FilesRead` capability.

## Admission token

A raw `LocalAgentRequestEnvelope` is not sufficient to enter the successful responder path.

`policy_admit_local_request()` evaluates the command's exact required capability through a caller-supplied `PolicyEvaluator` and returns a `LocalPolicyAdmittedRequest` only on `Decision::Allow`.

The successful read-only responder accepts the admitted token, not an unchecked raw request envelope.

## Denial

`Decision::Deny` produces a typed admission error and no admission token. Phase 037 does not itself encode/write the `Unauthorized` terminal response; that remains a later composition step.

## Authentication boundary

Policy admission is not authentication.

The current `PolicyEvaluator` interface is principal-agnostic. A future runtime must first authenticate the local peer/principal and then select or construct the evaluator bound to that authenticated identity/context before calling the Phase 037 admission function.

Possessing a decoded Request or calling this module does not authenticate a user/device/process.

## Runtime boundary

Phase 037 performs no peer-credential lookup, cryptographic authentication, socket I/O, host-state acquisition, DNS/network mutation, database lookup, systemd activation, retry, task/thread creation, or deployment action.
