# Local Server Connection-State Contract

## Status

Phase 044 aggregates the two already-validated server-side protocol safety states for one future local IPC connection instance.

## Aggregate state

`LocalServerConnectionState` owns:

- one Phase 043 inbound Request state;
- one Phase 041 terminal-response write state.

A connection instance is usable only while both component states are healthy.

## Unusable reasons

The aggregate exposes one of three unusable reasons:

- `InboundReadPoisoned`;
- `ResponseWritePoisoned`;
- `BothPoisoned`.

## Processing rule

Before consuming another Request, the aggregate state is checked.

If either component is already poisoned:

- zero Request bytes are consumed;
- zero policy evaluations occur;
- zero response bytes are written.

When both are healthy, processing delegates to the Phase 043 inbound guard, which preserves the existing ownership of state transitions.

## Transition ownership

- Request read/decode failure poisons only the inbound state.
- Terminal-response write failure poisons only the response-write state.
- A successful transaction leaves both healthy.

The aggregate does not introduce reset transitions. Once either component is poisoned, the connection instance is not reusable.

## Runtime boundary

Phase 044 owns no socket/file descriptor and performs no bind/listen/accept/connect/close, authentication, peer-credential lookup, host-state acquisition, DNS/network mutation, database work, task/thread creation, systemd activation, deployment, or private-key operation.
