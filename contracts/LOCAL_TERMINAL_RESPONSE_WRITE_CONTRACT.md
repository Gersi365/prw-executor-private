# Local Terminal Response Write Contract

## Status

Phase 041 locks a response-side generic `Write` guard with its own connection-instance write safety state.

## State model

`LocalTerminalResponseWriteState` has exactly two states:

- `Healthy`;
- `WritePoisoned`.

`Healthy -> WritePoisoned` is the only transition. `WritePoisoned` is absorbing for that state object.

This state is intentionally separate from the Request-side send state introduced by Phase 034.

## Required write ordering

For one in-memory frame:

1. reject an already-poisoned state before validation or I/O;
2. validate the existing terminal Response/Error invariant;
3. only after successful validation, delegate the complete frame to the existing generic writer;
4. if generic writing fails, transition to `WritePoisoned` before returning the typed write error;
5. do not flush implicitly.

## Invalid frame behavior

A frame that is not a valid terminal response:

- causes zero writer I/O;
- leaves a healthy response-write state healthy;
- returns the existing typed terminal-response validation failure.

## Write failure behavior

Any generic frame-write failure poisons the response-write state. The state is poisoned conservatively because the writer may have accepted a prefix of the frame and the stream may no longer be frame-aligned.

Later writes on the same state object are rejected before writer I/O.

## Recovery rule

No same-instance reset transition is provided. Future runtime recovery requires controlled disposal/re-establishment of the affected connection instance.

## Runtime boundary

Phase 041 does not create, bind, accept, connect, close, or reset a socket. It performs no authentication, policy evaluation, host-state acquisition, tracker mutation, DNS/network mutation, task/thread creation, database access, systemd activation, deployment, or private-key operation.
