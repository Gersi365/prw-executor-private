# Local Inbound Request-State Contract

## Status

Phase 043 locks a provider-neutral inbound-processing safety state for one future local IPC connection instance.

## States

- `Healthy`: another Request may be consumed.
- `ReadPoisoned`: a Request read/decode failure occurred and no later Request may be consumed from that state object.

`Healthy -> ReadPoisoned` is the only transition. `ReadPoisoned` is absorbing.

## Poison trigger

A Phase 042 failure classified as Request acquisition/decoding failure transitions inbound state to `ReadPoisoned` before the error is returned.

This includes malformed/truncated/non-Request/unknown-command input handled by the existing Request read/decode path.

## Non-trigger failures

Policy-response construction failure or response-write failure does not by itself classify the inbound framing state as poisoned. Response write ambiguity remains represented by the separate Phase 041 response-write state.

## Already-poisoned behavior

When inbound state is already `ReadPoisoned`:

- zero input reads occur;
- zero policy evaluations occur;
- zero response writes occur.

## Recovery rule

No same-instance reset exists. Future runtime code must discard the affected connection instance rather than resume Request consumption on the same logical input stream.

## Runtime boundary

Phase 043 owns no socket/file descriptor and performs no bind/listen/accept/connect/close, authentication, peer-credential lookup, host-state acquisition, DNS/network mutation, database work, task/thread creation, systemd activation, deployment, or private-key operation.
