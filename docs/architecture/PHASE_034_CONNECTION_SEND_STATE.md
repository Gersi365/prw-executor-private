# Phase 034 — Connection Send-State / Write Poisoning

## Objective

Encode the conservative connection-safety consequence of Phase 033 write ambiguity without activating a live transport.

## State model

`LocalConnectionSendState` begins `Healthy`. A generic Request write failure transitions it to `WritePoisoned`. The poisoned state is absorbing for that connection-state instance.

Build or tracker-registration failures do not poison because the Phase 033 ordering guarantees that no writer I/O begins before those stages succeed.

## Send composition

`send_tracked_local_command_request()` performs:

1. reject immediately if the connection state is already `WritePoisoned`;
2. delegate the admitted send to the Phase 033 build/register/write transaction;
3. on success, remain `Healthy`;
4. on build/register failure, remain `Healthy`;
5. on write failure, transition to `WritePoisoned` before returning the typed nested failure.

## Why there is no reset

The pure state object intentionally has no transition back to `Healthy`. A write failure may leave a real byte stream partially committed. Future runtime recovery therefore requires discarding the affected connection and constructing a fresh connection instance, not mutating the same logical stream back to healthy.

## Validation model

Memory/synthetic writers prove:

- successful send remains healthy;
- duplicate registration does not poison and performs no I/O;
- write failure poisons and retains the registered ID;
- every later send on a poisoned state is rejected before tracker mutation or I/O.

## Explicit deferrals

Still deferred:

- concrete connection object owning a transport;
- actual socket teardown/reconnect;
- outstanding-request disposition when a connection is discarded;
- timeout/cancellation/late-response policy;
- concurrent send serialization;
- Unix socket runtime and peer credential enforcement;
- live command dispatch.
