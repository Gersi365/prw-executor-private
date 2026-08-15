# Private Remote Workspace Linux Agent Authenticated Session Bridge Contract

Version: `0.1.0`

Status: Phase 071 implementation boundary — composition only; no runtime loop/bootstrap activation

## Scope

Phase 071 composes the already validated Phase 070 authenticated one-shot accept outcome with the already validated Phase 060 authenticated application-session type.

It does not perform a new accept operation, retrieve credentials again, bind policy, read/write application bytes, process Requests, create a loop, or activate the Agent bootstrap/service.

## Input boundary

The bridge consumes only a Phase 070 `AuthenticatedAgentAcceptOutcome`.

Therefore an authenticated connection variant already proves that:

- a Phase 068 listener entered listening state;
- Phase 070 readiness preparation verified `O_NONBLOCK` before accept;
- the connected descriptor was accepted with close-on-exec semantics;
- Linux `SO_PEERCRED` was obtained;
- peer UID matched the Agent effective UID;
- no PRW application-protocol byte was consumed before authentication.

The bridge has no raw file-descriptor, raw pathname, or unauthenticated-stream entry point.

## Outcome mapping

The mapping is exact:

- Phase 070 `NoConnectionReady` -> Phase 071 `NoConnectionReady`;
- Phase 070 `Authenticated(connection)` -> Phase 071 `AuthenticatedSession(AuthenticatedLocalLinuxSession::new(connection))`.

No error is added because Phase 060 session construction is an in-memory ownership/state composition with no fallible I/O step.

## Session construction

Creating the Phase 060 session:

- preserves the same authenticated connected stream ownership;
- preserves the immutable kernel peer-credential authorization metadata;
- initializes one fresh `LocalServerConnectionState` through the existing Phase 060 constructor;
- performs no application-protocol read or write;
- does not select or manufacture a `PolicyEvaluator`;
- does not process a Request budget.

## Authentication and policy separation

Same-UID transport authentication remains distinct from principal/capability policy binding.

Phase 071 must not infer a principal, workspace, role, or capability set from Unix UID equality.

The future caller must still supply the policy evaluator and snapshots explicitly when invoking Phase 060 bounded Request processing.

## Test requirements

Tests must prove at least:

- no-ready maps to no-ready without side effects;
- an already authenticated connection maps to a Phase 060 session with the same peer UID;
- the new session begins with usable/fresh aggregate connection state;
- sentinel application bytes written before authentication/session composition remain unread through the bridge and are recoverable only after explicitly extracting/accessing the authenticated stream;
- bridge construction invokes no Request processing or policy evaluation.

## Deferred runtime orchestration

Phase 071 does not implement:

- an accept loop;
- poll/epoll/async readiness;
- connection scheduling or concurrency;
- per-connection worker/task creation;
- request-budget selection;
- snapshot refresh policy;
- policy evaluator selection/binding;
- shutdown/cancellation orchestration;
- Agent bootstrap or systemd/service activation.

## Forbidden interpretation

Phase 071 does not authorize or implement:

- application Request processing in the bridge;
- shell/terminal/file runtime activation;
- TCP or abstract-socket fallback;
- principal/policy binding changes;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
