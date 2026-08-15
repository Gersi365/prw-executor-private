# Phase 049 — Boundary-Aware Request/Response Transaction

## Objective

Carry the Phase 048 clean-EOF distinction through the existing guarded response writer without adding a socket runtime.

## Data flow

`Read` → response-state precheck → Phase 048 boundary policy processor → either clean EOF or terminal response frame → Phase 041 guarded `Write`.

## Outcomes

- `CleanEof`: stop normally without policy-side response output.
- `ResponseWritten`: one terminal policy-gated response was emitted successfully.

## Safety ordering

A pre-existing `WritePoisoned` state rejects before the input reader is touched. This preserves the rule that an already unusable connection instance is not consumed further.

A clean EOF does not mutate response-write state. Request/framing/decode failures do not write a response and do not poison response-write state. Only an actual guarded response-write failure can transition the response state to `WritePoisoned`.

## Runtime boundary

No socket, file descriptor, peer credential, filesystem pathname, inbound poisoning integration, multi-request loop, timer, concurrent task, authentication, service activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
