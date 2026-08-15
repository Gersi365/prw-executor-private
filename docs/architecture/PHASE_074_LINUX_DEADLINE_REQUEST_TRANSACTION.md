# Phase 074 — Linux Deadline-Aware One-Request Transaction

Status: implementation payload awaiting CI validation

## Purpose

Compose Phase 073 absolute-deadline I/O with the existing authenticated Phase 060 session and Phase 051 aggregate state without duplicating Request framing, policy evaluation, response construction, or poisoning semantics.

## Ordering

For one authenticated Request attempt:

1. verify/use the existing aggregate connection state through Phase 051;
2. start one absolute read deadline immediately before boundary frame acquisition;
3. read/decode the Request and build the policy-gated terminal response through the existing pipeline;
4. start the independent write deadline lazily on the first non-empty terminal-response write;
5. preserve the existing inbound/read poisoning and response/write poisoning semantics.

Clean EOF produces no response and therefore never starts the deferred write deadline.

## Scope

Phase 074 does not:

- accept a new connection;
- spawn a worker/thread/task;
- loop over multiple Requests;
- choose production deadline values;
- resume a connection after Request-budget exhaustion;
- activate Agent bootstrap or systemd;
- change authentication or policy semantics.

## Validation target

CI must prove:

- a valid authenticated Request completes with independent read/write budgets;
- an idle authenticated peer reaches the read deadline and leaves the aggregate state unusable for inbound-read reason;
- clean EOF remains a normal stop and does not attempt to start an otherwise unrepresentable deferred write deadline;
- existing response bytes decode correctly;
- locked metadata, rustfmt, Clippy `-D warnings`, tests, and build remain green.
