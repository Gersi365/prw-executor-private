# Local Connection Send-State Contract

## Status

Phase 034 locks a provider-neutral in-memory send-state for one future local IPC connection instance.

## States

Exactly two states exist:

- `Healthy`: no ambiguous Request write failure has occurred;
- `WritePoisoned`: a Request write failed after Phase 033 registration and the stream may no longer be frame-aligned.

`Healthy -> WritePoisoned` is the only state transition. `WritePoisoned` is absorbing for the lifetime of that state object.

## Send admission

A Request send is admitted only while the state is `Healthy`.

When the state is already `WritePoisoned`:

- no Request frame is built;
- no request ID is registered;
- no writer I/O occurs;
- the attempt returns a typed `WritePoisoned` error.

## Failure classification

Phase 033 build and registration failures leave a healthy send-state healthy because no ambiguous write has occurred.

Any Phase 033 generic frame-write failure transitions the send-state to `WritePoisoned` before the error is returned. This applies conservatively even when a particular test writer accepted zero bytes because the generic write-error taxonomy does not encode a reliable committed-byte count for a real transport.

## Recovery rule

There is no reset transition on the same send-state object. A future runtime must discard the poisoned connection instance and construct a new connection/send-state after controlled teardown and re-establishment.

## Runtime boundary

Phase 034 owns no socket, file descriptor, task, thread, timer, timeout, reconnect loop, or teardown operation. It does not alter the outstanding-request tracker beyond delegating a newly admitted send to the Phase 033 transaction.
